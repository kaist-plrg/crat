use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type re_dfa_t;
    pub type dfa;
    pub type break_point;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn copysign(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    static mut ERRNO_node: *mut NODE;
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
    fn strcoll(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn wcscoll(__s1: *const wchar_t, __s2: *const wchar_t) -> libc::c_int;
    fn wcslen(_: *const libc::c_int) -> libc::c_ulong;
    fn getpid() -> __pid_t;
    static ruletab: [*const libc::c_char; 0];
    static mut NF: libc::c_long;
    static mut NR: libc::c_long;
    static mut FNR: libc::c_long;
    static mut BINMODE: libc::c_int;
    static mut IGNORECASE: bool;
    static mut OFS: *mut libc::c_char;
    static mut ORS: *mut libc::c_char;
    static mut OFMT: *mut libc::c_char;
    static mut CONVFMT: *const libc::c_char;
    static mut TEXTDOMAIN: *mut libc::c_char;
    static mut BINMODE_node: *mut NODE;
    static mut CONVFMT_node: *mut NODE;
    static mut FNR_node: *mut NODE;
    static mut IGNORECASE_node: *mut NODE;
    static mut NF_node: *mut NODE;
    static mut NR_node: *mut NODE;
    static mut OFMT_node: *mut NODE;
    static mut OFS_node: *mut NODE;
    static mut ORS_node: *mut NODE;
    static mut PROCINFO_node: *mut NODE;
    static mut LINT_node: *mut NODE;
    static mut TEXTDOMAIN_node: *mut NODE;
    static mut Nnull_string: *mut NODE;
    static mut fields_arr: *mut *mut NODE;
    static mut sourceline: libc::c_int;
    static mut source: *mut libc::c_char;
    static mut make_number: Option::<unsafe extern "C" fn(libc::c_double) -> *mut NODE>;
    static mut str2number: Option::<unsafe extern "C" fn(*mut NODE) -> *mut NODE>;
    static mut format_val: Option::<
        unsafe extern "C" fn(*const libc::c_char, libc::c_int, *mut NODE) -> *mut NODE,
    >;
    static mut cmp_numbers: Option::<
        unsafe extern "C" fn(*const NODE, *const NODE) -> libc::c_int,
    >;
    static mut nextfree: [block_header; 2];
    static mut field0_valid: bool;
    static mut do_itrace: bool;
    static mut do_flags: do_flag_values;
    static mut exit_val: libc::c_int;
    static mut gawk_mb_cur_max: libc::c_int;
    fn r_unref(tmp: *mut NODE);
    fn make_array() -> *mut NODE;
    fn force_array(symbol: *mut NODE, canfatal: bool) -> *mut NODE;
    fn array_vname(symbol: *const NODE) -> *const libc::c_char;
    fn concat_exp(nargs: libc::c_int, do_subsep: bool) -> *mut NODE;
    fn assoc_list(
        symbol: *mut NODE,
        sort_str: *const libc::c_char,
        sort_ctxt: sort_context_t,
    ) -> *mut *mut NODE;
    fn do_delete(symbol: *mut NODE, nsubs: libc::c_int);
    fn do_delete_loop(symbol: *mut NODE, lhs: *mut *mut NODE);
    fn lookup_builtin(name: *const libc::c_char) -> builtin_func_t;
    fn do_printf(nargs: libc::c_int, redirtype: libc::c_int);
    fn do_print(nargs: libc::c_int, redirtype: libc::c_int);
    fn do_print_rec(args: libc::c_int, redirtype: libc::c_int);
    fn do_match(nargs: libc::c_int) -> *mut NODE;
    fn do_sub(nargs: libc::c_int, flags: libc::c_uint) -> *mut NODE;
    fn call_sub(name: *const libc::c_char, nargs: libc::c_int) -> *mut NODE;
    fn call_match(nargs: libc::c_int) -> *mut NODE;
    fn call_split_func(name: *const libc::c_char, nargs: libc::c_int) -> *mut NODE;
    fn strncasecmpmbs(
        _: *const libc::c_uchar,
        _: *const libc::c_uchar,
        _: size_t,
    ) -> libc::c_int;
    fn r_fatal(mesg: *const libc::c_char, _: ...);
    fn set_loc(file: *const libc::c_char, line: libc::c_int);
    fn more_blocks(id: libc::c_int) -> *mut libc::c_void;
    fn r_dupnode(n: *mut NODE) -> *mut NODE;
    fn in_main_context() -> libc::c_int;
    fn nextfile(curfile_0: *mut *mut IOBUF, skipping: bool) -> libc::c_int;
    fn make_str_node(
        s: *const libc::c_char,
        len: size_t,
        flags: libc::c_int,
    ) -> *mut NODE;
    fn update_PROCINFO_num(subscript: *const libc::c_char, val: libc::c_double);
    fn inrec(iop: *mut IOBUF, errcode: *mut libc::c_int) -> bool;
    fn after_beginfile(curfile_0: *mut *mut IOBUF);
    fn do_getline(intovar: libc::c_int, iop: *mut IOBUF) -> *mut NODE;
    fn do_getline_redir(intovar: libc::c_int, redirtype: redirval) -> *mut NODE;
    fn r_warning(mesg: *const libc::c_char, _: ...);
    fn lookup(name: *const libc::c_char) -> *mut NODE;
    fn do_patsplit(nargs: libc::c_int) -> *mut NODE;
    fn do_split(nargs: libc::c_int) -> *mut NODE;
    fn re_update(t: *mut NODE) -> *mut Regexp;
    fn research(
        rp: *mut Regexp,
        str: *mut libc::c_char,
        start: libc::c_int,
        len: size_t,
        flags: libc::c_int,
    ) -> libc::c_int;
    fn get_field(num: libc::c_long, assign: *mut Func_ptr) -> *mut *mut NODE;
    fn free_api_string_copies();
    fn awk_value_to_node(_: *const awk_value_t) -> *mut NODE;
    static mut lintfunc: Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>;
    fn str2wstr(n: *mut NODE, ptr: *mut *mut size_t) -> *mut NODE;
    fn r_free_wstr(n: *mut NODE);
    fn reset_record();
    fn update_global_values();
    static mut symbol_table: *mut NODE;
    static mut func_table: *mut NODE;
    fn estrdup(str: *const libc::c_char, len: size_t) -> *mut libc::c_char;
    fn close_extensions();
    fn close_io(stdio_problem: *mut bool, got_EPIPE: *mut bool) -> libc::c_int;
    fn set_record(
        buf: *const libc::c_char,
        cnt: size_t,
        _: *const awk_fieldwidth_info_t,
    );
    fn getenv_long(name: *const libc::c_char) -> libc::c_long;
    fn set_RS();
    fn rebuild_record();
    fn update_ext_api();
    fn frame_popped();
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
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
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type wchar_t = libc::c_int;
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
    pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
    pub sub: C2RustUnnamed_1,
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
pub union C2RustUnnamed_1 {
    pub nodep: C2RustUnnamed_3,
    pub val: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
pub struct C2RustUnnamed_3 {
    pub l: C2RustUnnamed_10,
    pub r: C2RustUnnamed_5,
    pub x: C2RustUnnamed_4,
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
pub union C2RustUnnamed_4 {
    pub extra: *mut exp_node,
    pub aptr: Option::<unsafe extern "C" fn() -> ()>,
    pub xl: libc::c_long,
    pub cmnt: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
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
    pub d: C2RustUnnamed_7,
    pub x: C2RustUnnamed_6,
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
pub union C2RustUnnamed_6 {
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
pub union C2RustUnnamed_7 {
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
    pub hs: C2RustUnnamed_9,
    pub hi: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub next: *mut bucket_item,
    pub li: [libc::c_long; 2],
    pub val: [*mut exp_node; 2],
    pub cnt: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub next: *mut bucket_item,
    pub str_0: *mut libc::c_char,
    pub len: size_t,
    pub code: size_t,
    pub name: *mut exp_node,
    pub val: *mut exp_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
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
pub type lintvals = libc::c_uint;
pub const LINT_no_effect: lintvals = 2;
pub const LINT_assign_in_cond: lintvals = 1;
pub const LINT_illegal: lintvals = 0;
pub type redirval = libc::c_uint;
pub const redirect_twoway: redirval = 6;
pub const redirect_input: redirval = 5;
pub const redirect_pipein: redirval = 4;
pub const redirect_pipe: redirval = 3;
pub const redirect_append: redirval = 2;
pub const redirect_output: redirval = 1;
pub const redirect_none: redirval = 0;
pub type INSTRUCTION = exp_instruction;
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
pub type binmode_values = libc::c_uint;
pub const BINMODE_BOTH: binmode_values = 3;
pub const BINMODE_OUTPUT: binmode_values = 2;
pub const BINMODE_INPUT: binmode_values = 1;
pub const TEXT_TRANSLATE: binmode_values = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flagtab {
    pub val: libc::c_int,
    pub name: *const libc::c_char,
}
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
pub type Func_pre_exec = Option::<
    unsafe extern "C" fn(*mut *mut INSTRUCTION) -> libc::c_int,
>;
pub type Func_post_exec = Option::<unsafe extern "C" fn(*mut INSTRUCTION) -> ()>;
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
pub type sort_context_t = libc::c_uint;
pub const ASORTI: sort_context_t = 3;
pub const ASORT: sort_context_t = 2;
pub const SORTED_IN: sort_context_t = 1;
pub type builtin_func_t = Option::<unsafe extern "C" fn(libc::c_int) -> *mut NODE>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct optypetab {
    pub desc: *mut libc::c_char,
    pub operator: *mut libc::c_char,
}
pub type EXEC_STATE = exec_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exec_state {
    pub next: *mut exec_state,
    pub cptr: *mut INSTRUCTION,
    pub rule: libc::c_int,
    pub stack_size: libc::c_long,
    pub source: *const libc::c_char,
}
pub type scalar_cmp_t = libc::c_uint;
pub const SCALAR_GE: scalar_cmp_t = 5;
pub const SCALAR_GT: scalar_cmp_t = 4;
pub const SCALAR_LE: scalar_cmp_t = 3;
pub const SCALAR_LT: scalar_cmp_t = 2;
pub const SCALAR_NEQ: scalar_cmp_t = 1;
pub const SCALAR_EQ: scalar_cmp_t = 0;
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
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
unsafe extern "C" fn TOP_SCALAR() -> *mut NODE {
    let mut t: *mut NODE = (*stack_ptr).rptr;
    if (*t).type_0 as libc::c_uint == Node_var_array as libc::c_int as libc::c_uint {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"./awk.h\0" as *const u8 as *const libc::c_char,
            1896 as libc::c_int,
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
        (*stack_ptr).rptr = t;
    }
    return t;
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
unsafe extern "C" fn POP_SCALAR() -> *mut NODE {
    let fresh0 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    let mut t: *mut NODE = (*fresh0).rptr;
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
unsafe extern "C" fn str_terminate_f(mut n: *mut NODE, mut savep: *mut libc::c_char) {
    *savep = *((*n).sub.val.sp).offset((*n).sub.val.slen as isize);
    *((*n).sub.val.sp).offset((*n).sub.val.slen as isize) = '\0' as i32 as libc::c_char;
}
#[inline]
unsafe extern "C" fn in_array(mut a: *mut NODE, mut s: *mut NODE) -> *mut NODE {
    let mut ret: *mut *mut NODE = 0 as *mut *mut NODE;
    ret = ((*(*a).sub.nodep.l.lp).exists).unwrap()(a, s);
    return if !ret.is_null() { *ret } else { 0 as *mut NODE };
}
#[inline]
unsafe extern "C" fn POP_ARRAY(mut check_for_untyped: bool) -> *mut NODE {
    let fresh1 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    let mut t: *mut NODE = (*fresh1).rptr;
    static mut warned: bool = 0 as libc::c_int != 0;
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0 && !warned && check_for_untyped as libc::c_int != 0
        && ((*t).type_0 as libc::c_uint == Node_var_new as libc::c_int as libc::c_uint
            || (*t).type_0 as libc::c_uint
                == Node_elem_new as libc::c_int as libc::c_uint)
    {
        warned = 1 as libc::c_int != 0;
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"./awk.h\0" as *const u8 as *const libc::c_char,
            1857 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"behavior of `for' loop on untyped variable is not defined by POSIX\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    return if (*t).type_0 as libc::c_uint
        == Node_var_array as libc::c_int as libc::c_uint
    {
        t
    } else {
        force_array(t, 1 as libc::c_int != 0)
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
pub static mut fcall_list: *mut *mut NODE = 0 as *const *mut NODE as *mut *mut NODE;
pub static mut fcall_count: libc::c_long = 0 as libc::c_int as libc::c_long;
pub static mut currule: libc::c_int = 0 as libc::c_int;
pub static mut curfile: *mut IOBUF = 0 as *const IOBUF as *mut IOBUF;
pub static mut exiting: bool = 0 as libc::c_int != 0;
pub static mut interpret: Option::<
    unsafe extern "C" fn(*mut INSTRUCTION) -> libc::c_int,
> = None;
static mut num_exec_hook: libc::c_int = 0 as libc::c_int;
static mut pre_execute: [Func_pre_exec; 10] = [None; 10];
static mut post_execute: Func_post_exec = None;
pub static mut OFSlen: libc::c_int = 0;
pub static mut ORSlen: libc::c_int = 0;
pub static mut OFMTidx: libc::c_int = 0;
pub static mut CONVFMTidx: libc::c_int = 0;
static mut node_Boolean: [*mut NODE; 2] = [0 as *const NODE as *mut NODE; 2];
pub static mut casetable: [libc::c_char; 256] = [
    '\0' as i32 as libc::c_char,
    '\u{1}' as i32 as libc::c_char,
    '\u{2}' as i32 as libc::c_char,
    '\u{3}' as i32 as libc::c_char,
    '\u{4}' as i32 as libc::c_char,
    '\u{5}' as i32 as libc::c_char,
    '\u{6}' as i32 as libc::c_char,
    '\u{7}' as i32 as libc::c_char,
    '\u{8}' as i32 as libc::c_char,
    '\t' as i32 as libc::c_char,
    '\n' as i32 as libc::c_char,
    '\u{b}' as i32 as libc::c_char,
    '\u{c}' as i32 as libc::c_char,
    '\r' as i32 as libc::c_char,
    '\u{e}' as i32 as libc::c_char,
    '\u{f}' as i32 as libc::c_char,
    '\u{10}' as i32 as libc::c_char,
    '\u{11}' as i32 as libc::c_char,
    '\u{12}' as i32 as libc::c_char,
    '\u{13}' as i32 as libc::c_char,
    '\u{14}' as i32 as libc::c_char,
    '\u{15}' as i32 as libc::c_char,
    '\u{16}' as i32 as libc::c_char,
    '\u{17}' as i32 as libc::c_char,
    '\u{18}' as i32 as libc::c_char,
    '\u{19}' as i32 as libc::c_char,
    '\u{1a}' as i32 as libc::c_char,
    '\u{1b}' as i32 as libc::c_char,
    '\u{1c}' as i32 as libc::c_char,
    '\u{1d}' as i32 as libc::c_char,
    '\u{1e}' as i32 as libc::c_char,
    '\u{1f}' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '!' as i32 as libc::c_char,
    '"' as i32 as libc::c_char,
    '#' as i32 as libc::c_char,
    '$' as i32 as libc::c_char,
    '%' as i32 as libc::c_char,
    '&' as i32 as libc::c_char,
    '\'' as i32 as libc::c_char,
    '(' as i32 as libc::c_char,
    ')' as i32 as libc::c_char,
    '*' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    ',' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    ':' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '<' as i32 as libc::c_char,
    '=' as i32 as libc::c_char,
    '>' as i32 as libc::c_char,
    '?' as i32 as libc::c_char,
    '@' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'q' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    '[' as i32 as libc::c_char,
    '\\' as i32 as libc::c_char,
    ']' as i32 as libc::c_char,
    '^' as i32 as libc::c_char,
    '_' as i32 as libc::c_char,
    '`' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'q' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    '{' as i32 as libc::c_char,
    '|' as i32 as libc::c_char,
    '}' as i32 as libc::c_char,
    '~' as i32 as libc::c_char,
    '\u{7f}' as i32 as libc::c_char,
    -128i32 as libc::c_char,
    -127i32 as libc::c_char,
    -126i32 as libc::c_char,
    -125i32 as libc::c_char,
    -124i32 as libc::c_char,
    -123i32 as libc::c_char,
    -122i32 as libc::c_char,
    -121i32 as libc::c_char,
    -120i32 as libc::c_char,
    -119i32 as libc::c_char,
    -118i32 as libc::c_char,
    -117i32 as libc::c_char,
    -116i32 as libc::c_char,
    -115i32 as libc::c_char,
    -114i32 as libc::c_char,
    -113i32 as libc::c_char,
    -112i32 as libc::c_char,
    -111i32 as libc::c_char,
    -110i32 as libc::c_char,
    -109i32 as libc::c_char,
    -108i32 as libc::c_char,
    -107i32 as libc::c_char,
    -106i32 as libc::c_char,
    -105i32 as libc::c_char,
    -104i32 as libc::c_char,
    -103i32 as libc::c_char,
    -102i32 as libc::c_char,
    -101i32 as libc::c_char,
    -100i32 as libc::c_char,
    -99i32 as libc::c_char,
    -98i32 as libc::c_char,
    -97i32 as libc::c_char,
    -96i32 as libc::c_char,
    -95i32 as libc::c_char,
    -94i32 as libc::c_char,
    -93i32 as libc::c_char,
    -92i32 as libc::c_char,
    -91i32 as libc::c_char,
    -90i32 as libc::c_char,
    -89i32 as libc::c_char,
    -88i32 as libc::c_char,
    -87i32 as libc::c_char,
    -86i32 as libc::c_char,
    -85i32 as libc::c_char,
    -84i32 as libc::c_char,
    -83i32 as libc::c_char,
    -82i32 as libc::c_char,
    -81i32 as libc::c_char,
    -80i32 as libc::c_char,
    -79i32 as libc::c_char,
    -78i32 as libc::c_char,
    -77i32 as libc::c_char,
    -76i32 as libc::c_char,
    -75i32 as libc::c_char,
    -74i32 as libc::c_char,
    -73i32 as libc::c_char,
    -72i32 as libc::c_char,
    -71i32 as libc::c_char,
    -70i32 as libc::c_char,
    -69i32 as libc::c_char,
    -68i32 as libc::c_char,
    -67i32 as libc::c_char,
    -66i32 as libc::c_char,
    -65i32 as libc::c_char,
    -32i32 as libc::c_char,
    -31i32 as libc::c_char,
    -30i32 as libc::c_char,
    -29i32 as libc::c_char,
    -28i32 as libc::c_char,
    -27i32 as libc::c_char,
    -26i32 as libc::c_char,
    -25i32 as libc::c_char,
    -24i32 as libc::c_char,
    -23i32 as libc::c_char,
    -22i32 as libc::c_char,
    -21i32 as libc::c_char,
    -20i32 as libc::c_char,
    -19i32 as libc::c_char,
    -18i32 as libc::c_char,
    -17i32 as libc::c_char,
    -16i32 as libc::c_char,
    -15i32 as libc::c_char,
    -14i32 as libc::c_char,
    -13i32 as libc::c_char,
    -12i32 as libc::c_char,
    -11i32 as libc::c_char,
    -10i32 as libc::c_char,
    -41i32 as libc::c_char,
    -8i32 as libc::c_char,
    -7i32 as libc::c_char,
    -6i32 as libc::c_char,
    -5i32 as libc::c_char,
    -4i32 as libc::c_char,
    -3i32 as libc::c_char,
    -2i32 as libc::c_char,
    -33i32 as libc::c_char,
    -32i32 as libc::c_char,
    -31i32 as libc::c_char,
    -30i32 as libc::c_char,
    -29i32 as libc::c_char,
    -28i32 as libc::c_char,
    -27i32 as libc::c_char,
    -26i32 as libc::c_char,
    -25i32 as libc::c_char,
    -24i32 as libc::c_char,
    -23i32 as libc::c_char,
    -22i32 as libc::c_char,
    -21i32 as libc::c_char,
    -20i32 as libc::c_char,
    -19i32 as libc::c_char,
    -18i32 as libc::c_char,
    -17i32 as libc::c_char,
    -16i32 as libc::c_char,
    -15i32 as libc::c_char,
    -14i32 as libc::c_char,
    -13i32 as libc::c_char,
    -12i32 as libc::c_char,
    -11i32 as libc::c_char,
    -10i32 as libc::c_char,
    -9i32 as libc::c_char,
    -8i32 as libc::c_char,
    -7i32 as libc::c_char,
    -6i32 as libc::c_char,
    -5i32 as libc::c_char,
    -4i32 as libc::c_char,
    -3i32 as libc::c_char,
    -2i32 as libc::c_char,
    -1i32 as libc::c_char,
];
pub unsafe extern "C" fn load_casetable() {
    let mut i: libc::c_int = 0;
    static mut loaded: bool = 0 as libc::c_int != 0;
    if loaded as libc::c_int != 0
        || do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0
    {
        return;
    }
    loaded = 1 as libc::c_int != 0;
    i = 0o200 as libc::c_int;
    while i <= 0o377 as libc::c_int {
        if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
            & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
            && *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
                & _ISlower as libc::c_int as libc::c_ushort as libc::c_int != 0
            && i
                != ({
                    let mut __res: libc::c_int = 0;
                    if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = i;
                            __res = if __c < -(128 as libc::c_int)
                                || __c > 255 as libc::c_int
                            {
                                __c
                            } else {
                                *(*__ctype_toupper_loc()).offset(__c as isize)
                            };
                        } else {
                            __res = toupper(i);
                        }
                    } else {
                        __res = *(*__ctype_toupper_loc()).offset(i as isize);
                    }
                    __res
                })
        {
            casetable[i
                as usize] = ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = i;
                        __res = if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = toupper(i);
                    }
                } else {
                    __res = *(*__ctype_toupper_loc()).offset(i as isize);
                }
                __res
            }) as libc::c_char;
        } else {
            casetable[i as usize] = i as libc::c_char;
        }
        i += 1;
        i;
    }
}
static mut nodetypes: [*const libc::c_char; 21] = [
    b"Node_illegal\0" as *const u8 as *const libc::c_char,
    b"Node_val\0" as *const u8 as *const libc::c_char,
    b"Node_regex\0" as *const u8 as *const libc::c_char,
    b"Node_dynregex\0" as *const u8 as *const libc::c_char,
    b"Node_var\0" as *const u8 as *const libc::c_char,
    b"Node_var_array\0" as *const u8 as *const libc::c_char,
    b"Node_var_new\0" as *const u8 as *const libc::c_char,
    b"Node_elem_new\0" as *const u8 as *const libc::c_char,
    b"Node_param_list\0" as *const u8 as *const libc::c_char,
    b"Node_func\0" as *const u8 as *const libc::c_char,
    b"Node_ext_func\0" as *const u8 as *const libc::c_char,
    b"Node_builtin_func\0" as *const u8 as *const libc::c_char,
    b"Node_array_ref\0" as *const u8 as *const libc::c_char,
    b"Node_array_tree\0" as *const u8 as *const libc::c_char,
    b"Node_array_leaf\0" as *const u8 as *const libc::c_char,
    b"Node_dump_array\0" as *const u8 as *const libc::c_char,
    b"Node_arrayfor\0" as *const u8 as *const libc::c_char,
    b"Node_frame\0" as *const u8 as *const libc::c_char,
    b"Node_instruction\0" as *const u8 as *const libc::c_char,
    b"Node_final --- this should never appear\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut optypes: [optypetab; 124] = [
    {
        let mut init = optypetab {
            desc: b"Op_illegal\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_times\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            operator: b" * \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_times_i\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b" * \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_quotient\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b" / \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_quotient_i\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b" / \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_mod\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            operator: b" % \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_mod_i\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            operator: b" % \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_plus\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            operator: b" + \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_plus_i\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b" + \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_minus\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            operator: b" - \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_minus_i\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b" - \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_exp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            operator: b" ^ \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_exp_i\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            operator: b" ^ \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_concat\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_line_range\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_cond_pair\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b", \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_subscript\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b"[]\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_sub_array\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b"[]\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_preincrement\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b"++\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_predecrement\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b"--\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_postincrement\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b"++\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_postdecrement\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b"--\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_unary_minus\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b"-\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_unary_plus\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b"+\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_field_spec\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b"$\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_not\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            operator: b"! \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_assign\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b" = \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_store_var\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b" = \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_store_sub\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b" = \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_store_field\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b" = \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_store_field_exp\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b" = \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_assign_times\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b" *= \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_assign_quotient\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b" /= \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_assign_mod\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b" %= \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_assign_plus\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b" += \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_assign_minus\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b" -= \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_assign_exp\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b" ^= \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_assign_concat\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_and\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            operator: b" && \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_and_final\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_or\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            operator: b" || \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_or_final\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_equal\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            operator: b" == \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_notequal\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b" != \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_less\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            operator: b" < \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_greater\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b" > \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_leq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            operator: b" <= \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_geq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            operator: b" >= \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_match\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            operator: b" ~ \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_match_rec\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_nomatch\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b" !~ \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_rule\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_K_case\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b"case\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_K_default\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b"default\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_K_break\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b"break\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_K_continue\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b"continue\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_K_print\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b"print\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_K_print_rec\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b"print\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_K_printf\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b"printf\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_K_next\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b"next\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_K_exit\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b"exit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_K_return\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b"return\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_K_return_from_eval\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b"return\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_K_delete\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b"delete\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_K_delete_loop\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_K_getline_redir\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b"getline\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_K_getline\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b"getline\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_K_nextfile\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b"nextfile\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_K_namespace\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b"@namespace\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_builtin\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_sub_builtin\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_ext_builtin\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_in_array\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b" in \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_func_call\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_indirect_func_call\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_push\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_push_arg\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_push_arg_untyped\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_push_i\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_push_re\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_push_array\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_push_param\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_push_lhs\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_subscript_lhs\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b"[]\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_field_spec_lhs\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b"$\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_no_op\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_pop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_jmp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_jmp_true\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_jmp_false\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_get_record\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_newfile\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_arrayfor_init\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_arrayfor_incr\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_arrayfor_final\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_var_update\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_var_assign\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_field_assign\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_subscript_assign\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_after_beginfile\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_after_endfile\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_func\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_comment\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_exec_count\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_breakpoint\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_lint\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_lint_plus\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_atexit\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_stop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_token\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_symbol\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_list\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_K_do\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            operator: b"do\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_K_for\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            operator: b"for\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_K_arrayfor\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b"for\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_K_while\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b"while\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_K_switch\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b"switch\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_K_if\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            operator: b"if\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_K_else\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b"else\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_K_function\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: b"function\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_cond_exp\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_parens\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: b"Op_final --- this should never appear\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = optypetab {
            desc: 0 as *const libc::c_char as *mut libc::c_char,
            operator: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
];
pub unsafe extern "C" fn nodetype2str(mut type_0: NODETYPE) -> *const libc::c_char {
    static mut buf: [libc::c_char; 40] = [0; 40];
    if type_0 as libc::c_uint >= Node_illegal as libc::c_int as libc::c_uint
        && type_0 as libc::c_uint <= Node_final as libc::c_int as libc::c_uint
    {
        return nodetypes[type_0 as libc::c_int as usize];
    }
    sprintf(
        buf.as_mut_ptr(),
        dcgettext(
            0 as *const libc::c_char,
            b"unknown nodetype %d\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        type_0 as libc::c_int,
    );
    return buf.as_mut_ptr();
}
pub unsafe extern "C" fn opcode2str(mut op: OPCODE) -> *const libc::c_char {
    if op as libc::c_uint >= Op_illegal as libc::c_int as libc::c_uint
        && (op as libc::c_uint) < Op_final as libc::c_int as libc::c_uint
    {
        return optypes[op as libc::c_int as usize].desc;
    }
    (set_loc
        as unsafe extern "C" fn(
            *const libc::c_char,
            libc::c_int,
        ) -> ())(b"eval.c\0" as *const u8 as *const libc::c_char, 416 as libc::c_int);
    (Some(
        (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ())).unwrap(),
    ))
        .unwrap()(
        dcgettext(
            0 as *const libc::c_char,
            b"unknown opcode %d\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        op as libc::c_int,
    );
    return 0 as *const libc::c_char;
}
pub unsafe extern "C" fn op2str(mut op: OPCODE) -> *const libc::c_char {
    if op as libc::c_uint >= Op_illegal as libc::c_int as libc::c_uint
        && (op as libc::c_uint) < Op_final as libc::c_int as libc::c_uint
    {
        if !(optypes[op as libc::c_int as usize].operator).is_null() {
            return optypes[op as libc::c_int as usize].operator
        } else {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"eval.c\0" as *const u8 as *const libc::c_char,
                429 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"opcode %s not an operator or keyword\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                optypes[op as libc::c_int as usize].desc,
            );
        }
    } else {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"eval.c\0" as *const u8 as *const libc::c_char,
            432 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"unknown opcode %d\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            op as libc::c_int,
        );
    }
    return 0 as *const libc::c_char;
}
pub unsafe extern "C" fn flags2str(mut flagval: libc::c_int) -> *const libc::c_char {
    static mut values: [flagtab; 21] = [
        {
            let mut init = flagtab {
                val: MALLOC as libc::c_int,
                name: b"MALLOC\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: STRING as libc::c_int,
                name: b"STRING\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: STRCUR as libc::c_int,
                name: b"STRCUR\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: NUMCUR as libc::c_int,
                name: b"NUMCUR\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: NUMBER as libc::c_int,
                name: b"NUMBER\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: USER_INPUT as libc::c_int,
                name: b"USER_INPUT\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: BOOLVAL as libc::c_int,
                name: b"BOOL\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: INTLSTR as libc::c_int,
                name: b"INTLSTR\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: NUMINT as libc::c_int,
                name: b"NUMINT\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: INTIND as libc::c_int,
                name: b"INTIND\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: WSTRCUR as libc::c_int,
                name: b"WSTRCUR\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: MPFN as libc::c_int,
                name: b"MPFN\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: MPZN as libc::c_int,
                name: b"MPZN\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: NO_EXT_SET as libc::c_int,
                name: b"NO_EXT_SET\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: NULL_FIELD as libc::c_int,
                name: b"NULL_FIELD\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: ARRAYMAXED as libc::c_int,
                name: b"ARRAYMAXED\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: HALFHAT as libc::c_int,
                name: b"HALFHAT\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: XARRAY as libc::c_int,
                name: b"XARRAY\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: NUMCONSTSTR as libc::c_int,
                name: b"NUMCONSTSTR\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: REGEX as libc::c_int,
                name: b"REGEX\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: 0 as libc::c_int,
                name: 0 as *const libc::c_char,
            };
            init
        },
    ];
    return genflags2str(flagval, values.as_ptr());
}
pub unsafe extern "C" fn genflags2str(
    mut flagval: libc::c_int,
    mut tab: *const flagtab,
) -> *const libc::c_char {
    static mut buffer: [libc::c_char; 8192] = [0; 8192];
    let mut sp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut space_left: libc::c_int = 0;
    let mut space_needed: libc::c_int = 0;
    sp = buffer.as_mut_ptr();
    space_left = 8192 as libc::c_int;
    i = 0 as libc::c_int;
    while !((*tab.offset(i as isize)).name).is_null() {
        if flagval & (*tab.offset(i as isize)).val != 0 as libc::c_int {
            space_needed = (strlen((*tab.offset(i as isize)).name))
                .wrapping_add(
                    (sp != buffer.as_mut_ptr()) as libc::c_int as libc::c_ulong,
                ) as libc::c_int;
            if space_left <= space_needed {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"eval.c\0" as *const u8 as *const libc::c_char,
                    488 as libc::c_int,
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
                        b"buffer overflow in genflags2str\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            if sp != buffer.as_mut_ptr() {
                let fresh2 = sp;
                sp = sp.offset(1);
                *fresh2 = '|' as i32 as libc::c_char;
                space_left -= 1;
                space_left;
            }
            strcpy(sp, (*tab.offset(i as isize)).name);
            space_left = (space_left as libc::c_ulong).wrapping_sub(strlen(sp))
                as libc::c_int as libc::c_int;
            sp = sp.offset(strlen(sp) as isize);
        }
        i += 1;
        i;
    }
    *sp = '\0' as i32 as libc::c_char;
    return buffer.as_mut_ptr();
}
unsafe extern "C" fn posix_compare(mut s1: *mut NODE, mut s2: *mut NODE) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if gawk_mb_cur_max == 1 as libc::c_int {
        let mut save1: libc::c_char = 0;
        let mut save2: libc::c_char = 0;
        let mut p1: *const libc::c_char = 0 as *const libc::c_char;
        let mut p2: *const libc::c_char = 0 as *const libc::c_char;
        save1 = *((*s1).sub.val.sp).offset((*s1).sub.val.slen as isize);
        *((*s1).sub.val.sp)
            .offset((*s1).sub.val.slen as isize) = '\0' as i32 as libc::c_char;
        save2 = *((*s2).sub.val.sp).offset((*s2).sub.val.slen as isize);
        *((*s2).sub.val.sp)
            .offset((*s2).sub.val.slen as isize) = '\0' as i32 as libc::c_char;
        p1 = (*s1).sub.val.sp;
        p2 = (*s2).sub.val.sp;
        loop {
            let mut len: size_t = 0;
            ret = strcoll(p1, p2);
            if ret != 0 as libc::c_int {
                break;
            }
            len = strlen(p1);
            p1 = p1.offset(len.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            p2 = p2.offset(len.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            if p1
                == ((*s1).sub.val.sp)
                    .offset((*s1).sub.val.slen as isize)
                    .offset(1 as libc::c_int as isize) as *const libc::c_char
            {
                if p2
                    != ((*s2).sub.val.sp)
                        .offset((*s2).sub.val.slen as isize)
                        .offset(1 as libc::c_int as isize) as *const libc::c_char
                {
                    ret = -(1 as libc::c_int);
                }
                break;
            } else {
                if !(p2
                    == ((*s2).sub.val.sp)
                        .offset((*s2).sub.val.slen as isize)
                        .offset(1 as libc::c_int as isize) as *const libc::c_char)
                {
                    continue;
                }
                ret = 1 as libc::c_int;
                break;
            }
        }
        *((*s1).sub.val.sp).offset((*s1).sub.val.slen as isize) = save1;
        *((*s2).sub.val.sp).offset((*s2).sub.val.slen as isize) = save2;
    } else {
        let mut p1_0: *const wchar_t = 0 as *const wchar_t;
        let mut p2_0: *const wchar_t = 0 as *const wchar_t;
        str2wstr(s1, 0 as *mut *mut size_t);
        str2wstr(s2, 0 as *mut *mut size_t);
        p1_0 = (*s1).sub.val.wsp;
        p2_0 = (*s2).sub.val.wsp;
        loop {
            let mut len_0: size_t = 0;
            ret = wcscoll(p1_0, p2_0);
            if ret != 0 as libc::c_int {
                break;
            }
            len_0 = wcslen(p1_0);
            p1_0 = p1_0
                .offset(len_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            p2_0 = p2_0
                .offset(len_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            if p1_0
                == ((*s1).sub.val.wsp)
                    .offset((*s1).sub.val.wslen as isize)
                    .offset(1 as libc::c_int as isize) as *const wchar_t
            {
                if p2_0
                    != ((*s2).sub.val.wsp)
                        .offset((*s2).sub.val.wslen as isize)
                        .offset(1 as libc::c_int as isize) as *const wchar_t
                {
                    ret = -(1 as libc::c_int);
                }
                break;
            } else {
                if !(p2_0
                    == ((*s2).sub.val.wsp)
                        .offset((*s2).sub.val.wslen as isize)
                        .offset(1 as libc::c_int as isize) as *const wchar_t)
                {
                    continue;
                }
                ret = 1 as libc::c_int;
                break;
            }
        }
    }
    return ret;
}
pub unsafe extern "C" fn cmp_nodes(
    mut t1: *mut NODE,
    mut t2: *mut NODE,
    mut use_strcmp: bool,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut len1: size_t = 0;
    let mut len2: size_t = 0;
    let mut l: libc::c_int = 0;
    let mut ldiff: libc::c_int = 0;
    if t1 == t2 {
        return 0 as libc::c_int;
    }
    fixtype(t1);
    fixtype(t2);
    if (*t1).flags as libc::c_uint & NUMBER as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
        && (*t2).flags as libc::c_uint & NUMBER as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
    {
        return cmp_numbers.unwrap()(t1, t2);
    }
    force_string_fmt(t1, CONVFMT, CONVFMTidx);
    force_string_fmt(t2, CONVFMT, CONVFMTidx);
    len1 = (*t1).sub.val.slen;
    len2 = (*t2).sub.val.slen;
    ldiff = len1.wrapping_sub(len2) as libc::c_int;
    if len1 == 0 as libc::c_int as libc::c_ulong
        || len2 == 0 as libc::c_int as libc::c_ulong
    {
        return ldiff;
    }
    if do_flags as libc::c_uint & DO_POSIX as libc::c_int as libc::c_uint != 0
        && !use_strcmp
    {
        return posix_compare(t1, t2);
    }
    l = (if ldiff <= 0 as libc::c_int { len1 } else { len2 }) as libc::c_int;
    if IGNORECASE {
        let mut cp1: *const libc::c_uchar = (*t1).sub.val.sp as *const libc::c_uchar;
        let mut cp2: *const libc::c_uchar = (*t2).sub.val.sp as *const libc::c_uchar;
        let mut save1: libc::c_char = *((*t1).sub.val.sp)
            .offset((*t1).sub.val.slen as isize);
        let mut save2: libc::c_char = *((*t2).sub.val.sp)
            .offset((*t2).sub.val.slen as isize);
        if gawk_mb_cur_max > 1 as libc::c_int {
            let ref mut fresh3 = *((*t2).sub.val.sp).offset((*t2).sub.val.slen as isize);
            *fresh3 = '\0' as i32 as libc::c_char;
            *((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize) = *fresh3;
            ret = strncasecmpmbs(cp1, cp2, l as size_t);
            *((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize) = save1;
            *((*t2).sub.val.sp).offset((*t2).sub.val.slen as isize) = save2;
        } else {
            ret = 0 as libc::c_int;
            loop {
                let fresh4 = l;
                l = l - 1;
                if !(fresh4 > 0 as libc::c_int && ret == 0 as libc::c_int) {
                    break;
                }
                ret = casetable[*cp1 as usize] as libc::c_int
                    - casetable[*cp2 as usize] as libc::c_int;
                cp1 = cp1.offset(1);
                cp1;
                cp2 = cp2.offset(1);
                cp2;
            }
        }
    } else {
        ret = memcmp(
            (*t1).sub.val.sp as *const libc::c_void,
            (*t2).sub.val.sp as *const libc::c_void,
            l as libc::c_ulong,
        );
    }
    ret = if ret == 0 as libc::c_int { ldiff } else { ret };
    return ret;
}
unsafe extern "C" fn push_frame(mut f: *mut NODE) {
    static mut max_fcall: libc::c_long = 0;
    fcall_count += 1;
    fcall_count;
    if fcall_list.is_null() {
        max_fcall = 10 as libc::c_int as libc::c_long;
        fcall_list = emalloc_real(
            ((max_fcall + 1 as libc::c_int as libc::c_long) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut NODE>() as libc::c_ulong),
            b"push_frame\0" as *const u8 as *const libc::c_char,
            b"fcall_list\0" as *const u8 as *const libc::c_char,
            b"eval.c\0" as *const u8 as *const libc::c_char,
            654 as libc::c_int,
        ) as *mut *mut NODE;
    } else if fcall_count == max_fcall {
        max_fcall *= 2 as libc::c_int as libc::c_long;
        fcall_list = erealloc_real(
            fcall_list as *mut libc::c_void,
            ((max_fcall + 1 as libc::c_int as libc::c_long) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut NODE>() as libc::c_ulong),
            b"push_frame\0" as *const u8 as *const libc::c_char,
            b"fcall_list\0" as *const u8 as *const libc::c_char,
            b"eval.c\0" as *const u8 as *const libc::c_char,
            657 as libc::c_int,
        ) as *mut *mut NODE;
    }
    if fcall_count > 1 as libc::c_int as libc::c_long {
        memmove(
            fcall_list.offset(2 as libc::c_int as isize) as *mut libc::c_void,
            fcall_list.offset(1 as libc::c_int as isize) as *const libc::c_void,
            ((fcall_count - 1 as libc::c_int as libc::c_long) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut NODE>() as libc::c_ulong),
        );
    }
    let ref mut fresh5 = *fcall_list.offset(1 as libc::c_int as isize);
    *fresh5 = f;
}
unsafe extern "C" fn pop_frame() {
    if fcall_count > 1 as libc::c_int as libc::c_long {
        memmove(
            fcall_list.offset(1 as libc::c_int as isize) as *mut libc::c_void,
            fcall_list.offset(2 as libc::c_int as isize) as *const libc::c_void,
            ((fcall_count - 1 as libc::c_int as libc::c_long) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut NODE>() as libc::c_ulong),
        );
    }
    fcall_count -= 1;
    fcall_count;
    if do_flags as libc::c_uint & DO_DEBUG as libc::c_int as libc::c_uint != 0 {
        frame_popped();
    }
}
pub unsafe extern "C" fn dump_fcall_stack(mut fp: *mut FILE) {
    let mut f: *mut NODE = 0 as *mut NODE;
    let mut func: *mut NODE = 0 as *mut NODE;
    let mut i: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut k: libc::c_long = 0 as libc::c_int as libc::c_long;
    if fcall_count == 0 as libc::c_int as libc::c_long {
        return;
    }
    fprintf(
        fp,
        dcgettext(
            0 as *const libc::c_char,
            b"\n\t# Function Call Stack:\n\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    func = (*frame_ptr).sub.nodep.x.extra;
    let fresh6 = k;
    k = k + 1;
    fprintf(
        fp,
        b"\t# %3ld. %s\n\0" as *const u8 as *const libc::c_char,
        fresh6,
        (*func).sub.nodep.name,
    );
    i = 1 as libc::c_int as libc::c_long;
    while i < fcall_count {
        f = *fcall_list.offset(i as isize);
        func = (*f).sub.nodep.x.extra;
        let fresh7 = k;
        k = k + 1;
        fprintf(
            fp,
            b"\t# %3ld. %s\n\0" as *const u8 as *const libc::c_char,
            fresh7,
            (*func).sub.nodep.name,
        );
        i += 1;
        i;
    }
    fprintf(fp, b"\t# %3ld. -- main --\n\0" as *const u8 as *const libc::c_char, k);
}
pub unsafe extern "C" fn set_IGNORECASE() {
    static mut warned: bool = 0 as libc::c_int != 0;
    if (do_flags as libc::c_uint & DO_LINT_EXTENSIONS as libc::c_int as libc::c_uint != 0
        || do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0)
        && !warned
    {
        warned = 1 as libc::c_int != 0;
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"eval.c\0" as *const u8 as *const libc::c_char,
            716 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"`IGNORECASE' is a gawk extension\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0 {
        IGNORECASE = 0 as libc::c_int != 0;
    } else {
        IGNORECASE = boolval((*IGNORECASE_node).sub.nodep.l.lptr);
    }
    set_RS();
}
pub unsafe extern "C" fn set_BINMODE() {
    let mut current_block: u64;
    static mut warned: bool = 0 as libc::c_int != 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v: *mut NODE = fixtype((*BINMODE_node).sub.nodep.l.lptr);
    if (do_flags as libc::c_uint & DO_LINT_EXTENSIONS as libc::c_int as libc::c_uint != 0
        || do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0)
        && !warned
    {
        warned = 1 as libc::c_int != 0;
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"eval.c\0" as *const u8 as *const libc::c_char,
            737 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"`BINMODE' is a gawk extension\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0 {
        BINMODE = TEXT_TRANSLATE as libc::c_int;
    } else if (*v).flags as libc::c_uint & NUMBER as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        BINMODE = (*v).sub.val.fltnum as libc::c_long as libc::c_int;
        if BINMODE < TEXT_TRANSLATE as libc::c_int {
            BINMODE = TEXT_TRANSLATE as libc::c_int;
        } else if BINMODE > BINMODE_BOTH as libc::c_int {
            BINMODE = BINMODE_BOTH as libc::c_int;
        }
    } else if (*v).flags as libc::c_uint & STRING as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        p = (*v).sub.val.sp;
        match (*v).sub.val.slen {
            1 => {
                match *p.offset(0 as libc::c_int as isize) as libc::c_int {
                    48 | 49 | 50 | 51 => {
                        BINMODE = *p.offset(0 as libc::c_int as isize) as libc::c_int
                            - '0' as i32;
                        current_block = 18377268871191777778;
                    }
                    114 => {
                        BINMODE = BINMODE_INPUT as libc::c_int;
                        current_block = 18377268871191777778;
                    }
                    119 => {
                        BINMODE = BINMODE_OUTPUT as libc::c_int;
                        current_block = 18377268871191777778;
                    }
                    _ => {
                        BINMODE = BINMODE_BOTH as libc::c_int;
                        current_block = 13379492636939256815;
                    }
                }
            }
            2 => {
                match *p.offset(0 as libc::c_int as isize) as libc::c_int {
                    114 => {
                        current_block = 14935584943247102534;
                        match current_block {
                            15018109828435125687 => {
                                BINMODE = BINMODE_BOTH as libc::c_int;
                                if *p.offset(1 as libc::c_int as isize) as libc::c_int
                                    != 'r' as i32
                                {
                                    current_block = 13379492636939256815;
                                } else {
                                    current_block = 18377268871191777778;
                                }
                            }
                            _ => {
                                BINMODE = BINMODE_BOTH as libc::c_int;
                                if *p.offset(1 as libc::c_int as isize) as libc::c_int
                                    != 'w' as i32
                                {
                                    current_block = 13379492636939256815;
                                } else {
                                    current_block = 18377268871191777778;
                                }
                            }
                        }
                    }
                    119 => {
                        current_block = 15018109828435125687;
                        match current_block {
                            15018109828435125687 => {
                                BINMODE = BINMODE_BOTH as libc::c_int;
                                if *p.offset(1 as libc::c_int as isize) as libc::c_int
                                    != 'r' as i32
                                {
                                    current_block = 13379492636939256815;
                                } else {
                                    current_block = 18377268871191777778;
                                }
                            }
                            _ => {
                                BINMODE = BINMODE_BOTH as libc::c_int;
                                if *p.offset(1 as libc::c_int as isize) as libc::c_int
                                    != 'w' as i32
                                {
                                    current_block = 13379492636939256815;
                                } else {
                                    current_block = 18377268871191777778;
                                }
                            }
                        }
                    }
                    _ => {
                        current_block = 18377268871191777778;
                    }
                }
            }
            _ => {
                current_block = 13379492636939256815;
            }
        }
        match current_block {
            18377268871191777778 => {}
            _ => {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"eval.c\0" as *const u8 as *const libc::c_char,
                    794 as libc::c_int,
                );
                (Some(lintfunc.unwrap()))
                    .unwrap()(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"BINMODE value `%s' is invalid, treated as 3\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    p,
                );
            }
        }
    } else {
        BINMODE = 3 as libc::c_int;
    };
}
pub unsafe extern "C" fn set_OFS() {
    static mut first: bool = 1 as libc::c_int != 0;
    let mut new_ofs_len: size_t = 0;
    if first {
        first = 0 as libc::c_int != 0;
    } else if !field0_valid {
        get_field(
            9223372036854775807 as libc::c_long - 1 as libc::c_int as libc::c_long,
            0 as *mut Func_ptr,
        );
        rebuild_record();
    }
    (*OFS_node)
        .sub
        .nodep
        .l
        .lptr = force_string_fmt((*OFS_node).sub.nodep.l.lptr, CONVFMT, CONVFMTidx);
    new_ofs_len = (*(*OFS_node).sub.nodep.l.lptr).sub.val.slen;
    if OFS.is_null() {
        OFS = emalloc_real(
            new_ofs_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
            b"set_OFS\0" as *const u8 as *const libc::c_char,
            b"OFS\0" as *const u8 as *const libc::c_char,
            b"eval.c\0" as *const u8 as *const libc::c_char,
            829 as libc::c_int,
        ) as *mut libc::c_char;
    } else if (OFSlen as libc::c_ulong) < new_ofs_len {
        OFS = erealloc_real(
            OFS as *mut libc::c_void,
            new_ofs_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
            b"set_OFS\0" as *const u8 as *const libc::c_char,
            b"OFS\0" as *const u8 as *const libc::c_char,
            b"eval.c\0" as *const u8 as *const libc::c_char,
            831 as libc::c_int,
        ) as *mut libc::c_char;
    }
    memcpy(
        OFS as *mut libc::c_void,
        (*(*OFS_node).sub.nodep.l.lptr).sub.val.sp as *const libc::c_void,
        (*(*OFS_node).sub.nodep.l.lptr).sub.val.slen,
    );
    OFSlen = new_ofs_len as libc::c_int;
    *OFS.offset(OFSlen as isize) = '\0' as i32 as libc::c_char;
}
pub unsafe extern "C" fn set_ORS() {
    (*ORS_node)
        .sub
        .nodep
        .l
        .lptr = force_string_fmt((*ORS_node).sub.nodep.l.lptr, CONVFMT, CONVFMTidx);
    ORS = (*(*ORS_node).sub.nodep.l.lptr).sub.val.sp;
    ORSlen = (*(*ORS_node).sub.nodep.l.lptr).sub.val.slen as libc::c_int;
}
pub static mut fmt_list: *mut *mut NODE = 0 as *const *mut NODE as *mut *mut NODE;
unsafe extern "C" fn fmt_ok(mut n: *mut NODE) -> libc::c_int {
    let mut tmp: *mut NODE = force_string_fmt(n, CONVFMT, CONVFMTidx);
    let mut p: *const libc::c_char = (*tmp).sub.val.sp;
    static mut float_formats: [libc::c_char; 7] = unsafe {
        *::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"efgEFG\0")
    };
    static mut flags: [libc::c_char; 6] = unsafe {
        *::std::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b" +-#'\0")
    };
    let fresh8 = p;
    p = p.offset(1);
    if *fresh8 as libc::c_int != '%' as i32 {
        return 0 as libc::c_int;
    }
    while *p as libc::c_int != 0
        && !(strchr(flags.as_ptr(), *p as libc::c_int)).is_null()
    {
        p = p.offset(1);
        p;
    }
    while *p as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        p = p.offset(1);
        p;
    }
    if *p as libc::c_int == '\0' as i32
        || *p as libc::c_int != '.' as i32
            && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
                as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                == 0
    {
        return 0 as libc::c_int;
    }
    if *p as libc::c_int == '.' as i32 {
        p = p.offset(1);
        p;
    }
    while *p as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        p = p.offset(1);
        p;
    }
    if *p as libc::c_int == '\0' as i32
        || (strchr(float_formats.as_ptr(), *p as libc::c_int)).is_null()
    {
        return 0 as libc::c_int;
    }
    p = p.offset(1);
    if *p as libc::c_int != '\0' as i32 {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn fmt_index(mut n: *mut NODE) -> libc::c_int {
    let mut ix: libc::c_int = 0 as libc::c_int;
    static mut fmt_num: libc::c_int = 4 as libc::c_int;
    static mut fmt_hiwater: libc::c_int = 0 as libc::c_int;
    let mut save: libc::c_char = 0;
    if fmt_list.is_null() {
        fmt_list = emalloc_real(
            (fmt_num as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut NODE>() as libc::c_ulong),
            b"fmt_index\0" as *const u8 as *const libc::c_char,
            b"fmt_list\0" as *const u8 as *const libc::c_char,
            b"eval.c\0" as *const u8 as *const libc::c_char,
            903 as libc::c_int,
        ) as *mut *mut NODE;
    }
    n = force_string_fmt(n, CONVFMT, CONVFMTidx);
    save = *((*n).sub.val.sp).offset((*n).sub.val.slen as isize);
    *((*n).sub.val.sp).offset((*n).sub.val.slen as isize) = '\0' as i32 as libc::c_char;
    while ix < fmt_hiwater {
        if cmp_nodes(*fmt_list.offset(ix as isize), n, 1 as libc::c_int != 0)
            == 0 as libc::c_int
        {
            return ix;
        }
        ix += 1;
        ix;
    }
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0 && fmt_ok(n) == 0
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"eval.c\0" as *const u8 as *const libc::c_char,
            917 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"bad `%sFMT' specification `%s'\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            if n == (*CONVFMT_node).sub.nodep.l.lptr {
                b"CONV\0" as *const u8 as *const libc::c_char
            } else if n == (*OFMT_node).sub.nodep.l.lptr {
                b"O\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            (*n).sub.val.sp,
        );
    }
    *((*n).sub.val.sp).offset((*n).sub.val.slen as isize) = save;
    if fmt_hiwater >= fmt_num {
        fmt_num *= 2 as libc::c_int;
        fmt_list = erealloc_real(
            fmt_list as *mut libc::c_void,
            (fmt_num as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut NODE>() as libc::c_ulong),
            b"fmt_index\0" as *const u8 as *const libc::c_char,
            b"fmt_list\0" as *const u8 as *const libc::c_char,
            b"eval.c\0" as *const u8 as *const libc::c_char,
            926 as libc::c_int,
        ) as *mut *mut NODE;
    }
    let ref mut fresh9 = *fmt_list.offset(fmt_hiwater as isize);
    *fresh9 = dupnode(n);
    let fresh10 = fmt_hiwater;
    fmt_hiwater = fmt_hiwater + 1;
    return fresh10;
}
pub unsafe extern "C" fn set_OFMT() {
    OFMTidx = fmt_index((*OFMT_node).sub.nodep.l.lptr);
    OFMT = (**fmt_list.offset(OFMTidx as isize)).sub.val.sp;
}
pub unsafe extern "C" fn set_CONVFMT() {
    CONVFMTidx = fmt_index((*CONVFMT_node).sub.nodep.l.lptr);
    CONVFMT = (**fmt_list.offset(CONVFMTidx as isize)).sub.val.sp;
}
pub unsafe extern "C" fn set_LINT() {
    let mut old_lint: libc::c_int = (do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint)
        as libc::c_int;
    let mut n: *mut NODE = fixtype((*LINT_node).sub.nodep.l.lptr);
    lintfunc = Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ());
    do_flags = ::std::mem::transmute::<
        libc::c_uint,
        do_flag_values,
    >(
        do_flags as libc::c_uint
            & !(DO_LINT_ALL as libc::c_int | DO_LINT_INVALID as libc::c_int)
                as libc::c_uint,
    );
    if (*n).flags as libc::c_uint & STRING as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        let mut lintval: *const libc::c_char = 0 as *const libc::c_char;
        let mut lintlen: size_t = 0;
        lintval = (*n).sub.val.sp;
        lintlen = (*n).sub.val.slen;
        if lintlen > 0 as libc::c_int as libc::c_ulong {
            if lintlen == 7 as libc::c_int as libc::c_ulong
                && strncmp(
                    lintval,
                    b"invalid\0" as *const u8 as *const libc::c_char,
                    7 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                do_flags = ::std::mem::transmute::<
                    libc::c_uint,
                    do_flag_values,
                >(
                    do_flags as libc::c_uint
                        | DO_LINT_INVALID as libc::c_int as libc::c_uint,
                );
            } else if lintlen == 6 as libc::c_int as libc::c_ulong
                && strncmp(
                    lintval,
                    b"no-ext\0" as *const u8 as *const libc::c_char,
                    6 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                do_flags = ::std::mem::transmute::<
                    libc::c_uint,
                    do_flag_values,
                >(
                    do_flags as libc::c_uint
                        & !(DO_LINT_EXTENSIONS as libc::c_int) as libc::c_uint,
                );
            } else {
                do_flags = ::std::mem::transmute::<
                    libc::c_uint,
                    do_flag_values,
                >(do_flags as libc::c_uint | DO_LINT_ALL as libc::c_int as libc::c_uint);
                if lintlen == 5 as libc::c_int as libc::c_ulong
                    && strncmp(
                        lintval,
                        b"fatal\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                {
                    lintfunc = Some(
                        r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                    );
                }
            }
        }
    } else if !((*n).sub.val.fltnum == 0.0f64) {
        do_flags = ::std::mem::transmute::<
            libc::c_uint,
            do_flag_values,
        >(do_flags as libc::c_uint | DO_LINT_ALL as libc::c_int as libc::c_uint);
    }
    if old_lint as libc::c_uint
        != do_flags as libc::c_uint
            & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                as libc::c_uint && old_lint != 0
        && do_flags as libc::c_uint
            & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                as libc::c_uint == 0
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"eval.c\0" as *const u8 as *const libc::c_char,
            987 as libc::c_int,
        );
        (Some(
            (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"turning off `--lint' due to assignment to `LINT'\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    update_ext_api();
}
pub unsafe extern "C" fn set_TEXTDOMAIN() {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    (*TEXTDOMAIN_node)
        .sub
        .nodep
        .l
        .lptr = force_string_fmt(
        (*TEXTDOMAIN_node).sub.nodep.l.lptr,
        CONVFMT,
        CONVFMTidx,
    );
    tmp = (*TEXTDOMAIN_node).sub.nodep.l.lptr;
    TEXTDOMAIN = (*tmp).sub.val.sp;
}
pub unsafe extern "C" fn update_ERRNO_int(mut errcode: libc::c_int) {
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    update_PROCINFO_num(
        b"errno\0" as *const u8 as *const libc::c_char,
        errcode as libc::c_double,
    );
    if errcode != 0 {
        cp = strerror(errcode);
        cp = dcgettext(0 as *const libc::c_char, cp, 5 as libc::c_int);
    } else {
        cp = b"\0" as *const u8 as *const libc::c_char;
    }
    unref((*ERRNO_node).sub.nodep.l.lptr);
    (*ERRNO_node).sub.nodep.l.lptr = make_str_node(cp, strlen(cp), 0 as libc::c_int);
}
pub unsafe extern "C" fn update_ERRNO_string(mut string: *const libc::c_char) {
    update_PROCINFO_num(
        b"errno\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int as libc::c_double,
    );
    unref((*ERRNO_node).sub.nodep.l.lptr);
    let mut len: size_t = strlen(string);
    (*ERRNO_node).sub.nodep.l.lptr = make_str_node(string, len, 0 as libc::c_int);
}
pub unsafe extern "C" fn unset_ERRNO() {
    update_PROCINFO_num(
        b"errno\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int as libc::c_double,
    );
    unref((*ERRNO_node).sub.nodep.l.lptr);
    (*ERRNO_node).sub.nodep.l.lptr = dupnode(Nnull_string);
}
pub unsafe extern "C" fn update_NR() {
    if (*(*NR_node).sub.nodep.l.lptr).sub.val.fltnum != NR as libc::c_double {
        unref((*NR_node).sub.nodep.l.lptr);
        (*NR_node).sub.nodep.l.lptr = make_number.unwrap()(NR as libc::c_double);
    }
}
pub unsafe extern "C" fn update_NF() {
    let mut l: libc::c_long = 0;
    l = (*(*NF_node).sub.nodep.l.lptr).sub.val.fltnum as libc::c_long;
    if NF == -(1 as libc::c_int) as libc::c_long || l != NF {
        if NF == -(1 as libc::c_int) as libc::c_long {
            get_field(
                9223372036854775807 as libc::c_long - 1 as libc::c_int as libc::c_long,
                0 as *mut Func_ptr,
            );
        }
        unref((*NF_node).sub.nodep.l.lptr);
        (*NF_node).sub.nodep.l.lptr = make_number.unwrap()(NF as libc::c_double);
    }
}
pub unsafe extern "C" fn update_FNR() {
    if (*(*FNR_node).sub.nodep.l.lptr).sub.val.fltnum != FNR as libc::c_double {
        unref((*FNR_node).sub.nodep.l.lptr);
        (*FNR_node).sub.nodep.l.lptr = make_number.unwrap()(FNR as libc::c_double);
    }
}
pub static mut frame_ptr: *mut NODE = 0 as *const NODE as *mut NODE;
pub static mut stack_ptr: *mut STACK_ITEM = 0 as *const STACK_ITEM as *mut STACK_ITEM;
pub static mut stack_bottom: *mut STACK_ITEM = 0 as *const STACK_ITEM as *mut STACK_ITEM;
pub static mut stack_top: *mut STACK_ITEM = 0 as *const STACK_ITEM as *mut STACK_ITEM;
static mut STACK_SIZE: libc::c_ulong = 256 as libc::c_int as libc::c_ulong;
pub static mut max_args: libc::c_int = 0 as libc::c_int;
pub static mut args_array: *mut *mut NODE = 0 as *const *mut NODE as *mut *mut NODE;
pub unsafe extern "C" fn grow_stack() -> *mut STACK_ITEM {
    STACK_SIZE = STACK_SIZE.wrapping_mul(2 as libc::c_int as libc::c_ulong);
    stack_bottom = erealloc_real(
        stack_bottom as *mut libc::c_void,
        STACK_SIZE.wrapping_mul(::std::mem::size_of::<STACK_ITEM>() as libc::c_ulong),
        b"grow_stack\0" as *const u8 as *const libc::c_char,
        b"stack_bottom\0" as *const u8 as *const libc::c_char,
        b"eval.c\0" as *const u8 as *const libc::c_char,
        1132 as libc::c_int,
    ) as *mut STACK_ITEM;
    stack_top = stack_bottom
        .offset(STACK_SIZE as isize)
        .offset(-(1 as libc::c_int as isize));
    stack_ptr = stack_bottom
        .offset(STACK_SIZE.wrapping_div(2 as libc::c_int as libc::c_ulong) as isize);
    return stack_ptr;
}
pub unsafe extern "C" fn r_get_lhs(
    mut n: *mut NODE,
    mut reference: bool,
) -> *mut *mut NODE {
    let mut isparam: bool = 0 as libc::c_int != 0;
    if (*n).type_0 as libc::c_uint == Node_param_list as libc::c_int as libc::c_uint {
        isparam = 1 as libc::c_int != 0;
        n = *((*frame_ptr).sub.nodep.r.av).offset((*n).sub.nodep.l.ll as isize);
    }
    let mut current_block_19: u64;
    match (*n).type_0 as libc::c_uint {
        5 => {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"eval.c\0" as *const u8 as *const libc::c_char,
                1156 as libc::c_int,
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
                array_vname(n),
            );
            return 0 as *mut *mut NODE;
        }
        12 => {
            if (*(*n).sub.nodep.l.lptr).type_0 as libc::c_uint
                == Node_var_array as libc::c_int as libc::c_uint
            {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"eval.c\0" as *const u8 as *const libc::c_char,
                    1161 as libc::c_int,
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
                        b"attempt to use array `%s' in a scalar context\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    array_vname(n),
                );
            }
            if (*(*n).sub.nodep.l.lptr).type_0 as libc::c_uint
                != Node_var as libc::c_int as libc::c_uint
            {
                (*(*n).sub.nodep.l.lptr).type_0 = Node_var;
                (*(*n).sub.nodep.l.lptr).sub.nodep.l.lptr = dupnode(Nnull_string);
            }
            current_block_19 = 13100447413716447472;
        }
        6 => {
            current_block_19 = 13100447413716447472;
        }
        7 => {
            free((*n).sub.val.sp as *mut libc::c_void);
            (*n).sub.val.sp = 0 as *mut libc::c_char;
            (*n).sub.val.slen = 0 as libc::c_int as size_t;
            (*n).type_0 = Node_var;
            (*n).sub.nodep.l.lptr = dupnode(Nnull_string);
            current_block_19 = 5948590327928692120;
        }
        4 => {
            current_block_19 = 5948590327928692120;
        }
        _ => {
            r_fatal(
                b"internal error: file %s, line %d: unexpected variable type %s\0"
                    as *const u8 as *const libc::c_char,
                b"eval.c\0" as *const u8 as *const libc::c_char,
                1185 as libc::c_int,
                nodetype2str((*n).type_0),
            );
            current_block_19 = 5948590327928692120;
        }
    }
    match current_block_19 {
        13100447413716447472 => {
            (*n).type_0 = Node_var;
            (*n).sub.nodep.l.lptr = dupnode(Nnull_string);
        }
        _ => {}
    }
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0 && reference as libc::c_int != 0 && (*n).sub.nodep.l.lptr == Nnull_string
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"eval.c\0" as *const u8 as *const libc::c_char,
            1189 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            if isparam as libc::c_int != 0 {
                dcgettext(
                    0 as *const libc::c_char,
                    b"reference to uninitialized argument `%s'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                )
            } else {
                dcgettext(
                    0 as *const libc::c_char,
                    b"reference to uninitialized variable `%s'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                )
            },
            (*n).sub.nodep.name,
        );
    }
    return &mut (*n).sub.nodep.l.lptr;
}
pub unsafe extern "C" fn r_get_field(
    mut n: *mut NODE,
    mut assign: *mut Func_ptr,
    mut reference: bool,
) -> *mut *mut NODE {
    let mut field_num: libc::c_long = 0;
    let mut lhs: *mut *mut NODE = 0 as *mut *mut NODE;
    if !assign.is_null() {
        *assign = None;
    }
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
    {
        if (*fixtype(n)).flags as libc::c_uint & NUMBER as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"eval.c\0" as *const u8 as *const libc::c_char,
                1209 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"attempt to field reference from non-numeric value\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if (*n).sub.val.slen == 0 as libc::c_int as libc::c_ulong {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"eval.c\0" as *const u8 as *const libc::c_char,
                    1211 as libc::c_int,
                );
                (Some(lintfunc.unwrap()))
                    .unwrap()(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"attempt to field reference from null string\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
        }
    }
    force_number(n);
    field_num = (*n).sub.val.fltnum as libc::c_long;
    if field_num < 0 as libc::c_int as libc::c_long {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"eval.c\0" as *const u8 as *const libc::c_char,
            1219 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"attempt to access field %ld\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            field_num,
        );
    }
    if field_num == 0 as libc::c_int as libc::c_long && field0_valid as libc::c_int != 0
    {
        lhs = &mut *fields_arr.offset(0 as libc::c_int as isize) as *mut *mut NODE;
        if !assign.is_null() {
            *assign = Some(reset_record as unsafe extern "C" fn() -> ());
        }
    } else {
        lhs = get_field(field_num, assign);
    }
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0 && reference as libc::c_int != 0
        && (**lhs).flags as libc::c_uint & NULL_FIELD as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"eval.c\0" as *const u8 as *const libc::c_char,
            1228 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"reference to uninitialized field `$%ld'\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            field_num,
        );
    }
    return lhs;
}
unsafe extern "C" fn calc_exp_posint(
    mut x: libc::c_double,
    mut n: libc::c_long,
) -> libc::c_double {
    let mut mult: libc::c_double = 1 as libc::c_int as libc::c_double;
    while n > 1 as libc::c_int as libc::c_long {
        if n % 2 as libc::c_int as libc::c_long == 1 as libc::c_int as libc::c_long {
            mult *= x;
        }
        x *= x;
        n /= 2 as libc::c_int as libc::c_long;
    }
    return mult * x;
}
pub unsafe extern "C" fn calc_exp(
    mut x1: libc::c_double,
    mut x2: libc::c_double,
) -> libc::c_double {
    let mut lx: libc::c_long = 0;
    lx = x2 as libc::c_long;
    if lx as libc::c_double == x2 {
        if lx == 0 as libc::c_int as libc::c_long {
            return 1 as libc::c_int as libc::c_double;
        }
        return if lx > 0 as libc::c_int as libc::c_long {
            calc_exp_posint(x1, lx)
        } else {
            1.0f64 / calc_exp_posint(x1, -lx)
        };
    }
    return pow(x1, x2);
}
unsafe extern "C" fn setup_frame(mut pc: *mut INSTRUCTION) -> *mut INSTRUCTION {
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut m: *mut NODE = 0 as *mut NODE;
    let mut f: *mut NODE = 0 as *mut NODE;
    let mut fp: *mut NODE = 0 as *mut NODE;
    let mut sp: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut pcount: libc::c_int = 0;
    let mut arg_count: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    f = (*pc).x.xn;
    pcount = (*f).sub.nodep.l.ll as libc::c_int;
    fp = (*f).sub.nodep.rn;
    arg_count = (*pc.offset(1 as libc::c_int as isize)).x.xl as libc::c_int;
    if pcount > 0 as libc::c_int {
        sp = ezalloc_real(
            (pcount as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut NODE>() as libc::c_ulong),
            b"setup_frame\0" as *const u8 as *const libc::c_char,
            b"sp\0" as *const u8 as *const libc::c_char,
            b"eval.c\0" as *const u8 as *const libc::c_char,
            1286 as libc::c_int,
        ) as *mut *mut NODE;
    }
    if arg_count > pcount {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"eval.c\0" as *const u8 as *const libc::c_char,
            1291 as libc::c_int,
        );
        (Some(
            (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"function `%s' called with more arguments than declared\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*f).sub.nodep.name,
        );
        loop {
            let fresh11 = stack_ptr;
            stack_ptr = stack_ptr.offset(-1);
            r = (*fresh11).rptr;
            if (*r).type_0 as libc::c_uint == Node_val as libc::c_int as libc::c_uint {
                DEREF(r);
            }
            arg_count -= 1;
            if !(arg_count > pcount) {
                break;
            }
        }
    }
    i = 0 as libc::c_int;
    j = arg_count - 1 as libc::c_int;
    while i < pcount {
        r = nextfree[BLOCK_NODE as libc::c_int as usize].freep as *mut NODE;
        if !r.is_null() {
            nextfree[BLOCK_NODE as libc::c_int as usize]
                .freep = (*(r as *mut block_item)).freep;
        } else {
            r = more_blocks(BLOCK_NODE as libc::c_int) as *mut NODE;
        };
        memset(
            r as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<NODE>() as libc::c_ulong,
        );
        let ref mut fresh12 = *sp.offset(i as isize);
        *fresh12 = r;
        if i >= arg_count {
            (*r).type_0 = Node_var_new;
            (*r).sub.nodep.name = (*fp.offset(i as isize)).sub.nodep.name;
        } else {
            m = (*stack_ptr.offset(-(j as isize))).rptr;
            if (*m).type_0 as libc::c_uint
                == Node_param_list as libc::c_int as libc::c_uint
            {
                m = *((*frame_ptr).sub.nodep.r.av).offset((*m).sub.nodep.l.ll as isize);
            }
            if m == *fields_arr.offset(0 as libc::c_int as isize) {
                DEREF(m);
                m = dupnode(m);
            }
            match (*m).type_0 as libc::c_uint {
                6 | 5 | 7 => {
                    (*r).type_0 = Node_array_ref;
                    (*r).sub.nodep.r.rptr = m;
                    (*r).sub.nodep.l.lptr = (*r).sub.nodep.r.rptr;
                }
                12 => {
                    (*r).type_0 = Node_array_ref;
                    (*r).sub.nodep.l.lptr = (*m).sub.nodep.l.lptr;
                    (*r).sub.nodep.r.rptr = m;
                }
                4 => {
                    (*r).type_0 = Node_var;
                    (*r).sub.nodep.l.lptr = dupnode(Nnull_string);
                }
                1 => {
                    (*r).type_0 = Node_var;
                    (*r).sub.nodep.l.lptr = m;
                }
                9 | 11 | 10 => {
                    (*r).type_0 = Node_var;
                    (*r)
                        .sub
                        .nodep
                        .l
                        .lptr = make_str_node(
                        (*m).sub.nodep.name,
                        strlen((*m).sub.nodep.name),
                        0 as libc::c_int,
                    );
                }
                _ => {
                    r_fatal(
                        b"internal error: file %s, line %d: unexpected parameter type %s\0"
                            as *const u8 as *const libc::c_char,
                        b"eval.c\0" as *const u8 as *const libc::c_char,
                        1360 as libc::c_int,
                        nodetype2str((*m).type_0),
                    );
                }
            }
            (*r).sub.nodep.name = (*fp.offset(i as isize)).sub.nodep.name;
        }
        i += 1;
        i;
        j -= 1;
        j;
    }
    stack_ptr = stack_ptr.offset(-arg_count as isize);
    if (*pc).opcode as libc::c_uint
        == Op_indirect_func_call as libc::c_int as libc::c_uint
    {
        let fresh13 = stack_ptr;
        stack_ptr = stack_ptr.offset(-1);
        r = (*fresh13).rptr;
        DEREF(r);
    }
    (*frame_ptr).sub.nodep.name = source;
    if do_flags as libc::c_uint & DO_PROFILE as libc::c_int as libc::c_uint != 0
        || do_flags as libc::c_uint & DO_DEBUG as libc::c_int as libc::c_uint != 0
    {
        push_frame(frame_ptr);
    }
    let ref mut fresh14 = (*if stack_ptr < stack_top {
        stack_ptr = stack_ptr.offset(1);
        stack_ptr
    } else {
        grow_stack()
    })
        .rptr;
    *fresh14 = frame_ptr;
    frame_ptr = nextfree[BLOCK_NODE as libc::c_int as usize].freep as *mut NODE;
    if !frame_ptr.is_null() {
        nextfree[BLOCK_NODE as libc::c_int as usize]
            .freep = (*(frame_ptr as *mut block_item)).freep;
    } else {
        frame_ptr = more_blocks(BLOCK_NODE as libc::c_int) as *mut NODE;
    };
    (*frame_ptr).type_0 = Node_frame;
    (*frame_ptr).sub.nodep.r.av = sp;
    (*frame_ptr)
        .sub
        .nodep
        .reflags = stack_ptr.offset_from(stack_bottom) as libc::c_long as reflagvals;
    (*frame_ptr).sub.nodep.x.extra = f;
    (*frame_ptr).sub.nodep.name = 0 as *mut libc::c_char;
    (*frame_ptr).sub.nodep.l.li = pc;
    return (*f).sub.nodep.r.iptr;
}
unsafe extern "C" fn restore_frame(mut fp: *mut NODE) -> *mut INSTRUCTION {
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut sp: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut n: libc::c_int = 0;
    let mut func: *mut NODE = 0 as *mut NODE;
    let mut ri: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    func = (*frame_ptr).sub.nodep.x.extra;
    n = (*func).sub.nodep.l.ll as libc::c_int;
    sp = (*frame_ptr).sub.nodep.r.av;
    while n > 0 as libc::c_int {
        let fresh15 = sp;
        sp = sp.offset(1);
        r = *fresh15;
        if (*r).type_0 as libc::c_uint == Node_var as libc::c_int as libc::c_uint {
            DEREF((*r).sub.nodep.l.lptr);
        } else if (*r).type_0 as libc::c_uint
            == Node_var_array as libc::c_int as libc::c_uint
        {
            ((*(*r).sub.nodep.l.lp).clear).unwrap()(r, 0 as *mut exp_node);
        }
        let ref mut fresh16 = (*(r as *mut block_item)).freep;
        *fresh16 = nextfree[BLOCK_NODE as libc::c_int as usize].freep;
        nextfree[BLOCK_NODE as libc::c_int as usize].freep = r as *mut block_item;
        n -= 1;
        n;
    }
    if !((*frame_ptr).sub.nodep.r.av).is_null() {
        free((*frame_ptr).sub.nodep.r.av as *mut libc::c_void);
    }
    ri = (*frame_ptr).sub.nodep.l.li;
    let ref mut fresh17 = (*(frame_ptr as *mut block_item)).freep;
    *fresh17 = nextfree[BLOCK_NODE as libc::c_int as usize].freep;
    nextfree[BLOCK_NODE as libc::c_int as usize].freep = frame_ptr as *mut block_item;
    if do_flags as libc::c_uint & DO_PROFILE as libc::c_int as libc::c_uint != 0
        || do_flags as libc::c_uint & DO_DEBUG as libc::c_int as libc::c_uint != 0
    {
        pop_frame();
    }
    frame_ptr = fp;
    source = (*fp).sub.nodep.name;
    (*fp).sub.nodep.name = 0 as *mut libc::c_char;
    return (*ri).nexti;
}
#[inline]
unsafe extern "C" fn free_arrayfor(mut r: *mut NODE) {
    if !((*r).sub.nodep.r.av).is_null() {
        let mut n: *mut NODE = 0 as *mut NODE;
        let mut num_elems: size_t = (*r).sub.nodep.reflags as size_t;
        let mut list: *mut *mut NODE = (*r).sub.nodep.r.av;
        while num_elems > 0 as libc::c_int as libc::c_ulong {
            num_elems = num_elems.wrapping_sub(1);
            n = *list.offset(num_elems as isize);
            unref(n);
        }
        free(list as *mut libc::c_void);
    }
    let ref mut fresh18 = (*(r as *mut block_item)).freep;
    *fresh18 = nextfree[BLOCK_NODE as libc::c_int as usize].freep;
    nextfree[BLOCK_NODE as libc::c_int as usize].freep = r as *mut block_item;
}
pub unsafe extern "C" fn unwind_stack(mut n: libc::c_long) -> *mut INSTRUCTION {
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut cp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut sp: *mut STACK_ITEM = 0 as *mut STACK_ITEM;
    if stack_ptr < stack_bottom {
        return 0 as *mut INSTRUCTION;
    }
    sp = stack_bottom.offset(n as isize);
    if stack_ptr < sp {
        return 0 as *mut INSTRUCTION;
    }
    loop {
        let fresh19 = stack_ptr;
        stack_ptr = stack_ptr.offset(-1);
        r = (*fresh19).rptr;
        if r.is_null() {
            break;
        }
        match (*r).type_0 as libc::c_uint {
            17 => {
                cp = restore_frame(r);
            }
            16 => {
                free_arrayfor(r);
            }
            1 => {
                DEREF(r);
            }
            18 => {
                let ref mut fresh20 = (*(r as *mut block_item)).freep;
                *fresh20 = nextfree[BLOCK_NODE as libc::c_int as usize].freep;
                nextfree[BLOCK_NODE as libc::c_int as usize]
                    .freep = r as *mut block_item;
            }
            _ => {
                if in_main_context() != 0 && !exiting {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"eval.c\0" as *const u8 as *const libc::c_char,
                        1497 as libc::c_int,
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
                            b"unwind_stack: unexpected type `%s'\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        nodetype2str((*r).type_0),
                    );
                }
            }
        }
        if stack_ptr < sp {
            break;
        }
    }
    return cp;
}
#[inline]
unsafe extern "C" fn eval_condition(mut t: *mut NODE) -> bool {
    if t == node_Boolean[0 as libc::c_int as usize] {
        return 0 as libc::c_int != 0;
    }
    if t == node_Boolean[1 as libc::c_int as usize] {
        return 1 as libc::c_int != 0;
    }
    return boolval(t);
}
unsafe extern "C" fn cmp_scalars(mut comparison_type: scalar_cmp_t) -> bool {
    let mut t1: *mut NODE = 0 as *mut NODE;
    let mut t2: *mut NODE = 0 as *mut NODE;
    let mut di: libc::c_int = 0;
    let mut ret: bool = false;
    t2 = POP_SCALAR();
    t1 = (*stack_ptr).rptr;
    t1 = elem_new_to_scalar(t1);
    t2 = elem_new_to_scalar(t2);
    if (*t1).type_0 as libc::c_uint == Node_var_array as libc::c_int as libc::c_uint {
        DEREF(t2);
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"eval.c\0" as *const u8 as *const libc::c_char,
            1555 as libc::c_int,
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
    if (*t1).flags as libc::c_uint & STRING as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
        || (*t2).flags as libc::c_uint & STRING as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
    {
        let mut use_strcmp: bool = comparison_type as libc::c_uint
            == SCALAR_EQ as libc::c_int as libc::c_uint
            || comparison_type as libc::c_uint
                == SCALAR_NEQ as libc::c_int as libc::c_uint;
        di = cmp_nodes(t1, t2, use_strcmp);
        match comparison_type as libc::c_uint {
            0 => {
                ret = di == 0 as libc::c_int;
            }
            1 => {
                ret = di != 0 as libc::c_int;
            }
            2 => {
                ret = di < 0 as libc::c_int;
            }
            3 => {
                ret = di <= 0 as libc::c_int;
            }
            4 => {
                ret = di > 0 as libc::c_int;
            }
            5 => {
                ret = di >= 0 as libc::c_int;
            }
            _ => {}
        }
    } else {
        fixtype(t1);
        fixtype(t2);
        ret = cmp_doubles(t1, t2, comparison_type);
    }
    DEREF(t1);
    DEREF(t2);
    return ret;
}
unsafe extern "C" fn cmp_doubles(
    mut t1: *const NODE,
    mut t2: *const NODE,
    mut comparison_type: scalar_cmp_t,
) -> bool {
    let mut t1_nan: bool = ((*t1).sub.val.fltnum).is_nan() as i32 != 0;
    let mut t2_nan: bool = ((*t2).sub.val.fltnum).is_nan() as i32 != 0;
    let mut ret: libc::c_int = 0;
    if (t1_nan as libc::c_int != 0 || t2_nan as libc::c_int != 0)
        && comparison_type as libc::c_uint != SCALAR_NEQ as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int != 0;
    }
    match comparison_type as libc::c_uint {
        0 => {
            ret = ((*t1).sub.val.fltnum == (*t2).sub.val.fltnum) as libc::c_int;
        }
        1 => {
            ret = ((*t1).sub.val.fltnum != (*t2).sub.val.fltnum) as libc::c_int;
        }
        2 => {
            ret = ((*t1).sub.val.fltnum < (*t2).sub.val.fltnum) as libc::c_int;
        }
        3 => {
            ret = ((*t1).sub.val.fltnum <= (*t2).sub.val.fltnum) as libc::c_int;
        }
        4 => {
            ret = ((*t1).sub.val.fltnum > (*t2).sub.val.fltnum) as libc::c_int;
        }
        5 => {
            ret = ((*t1).sub.val.fltnum >= (*t2).sub.val.fltnum) as libc::c_int;
        }
        _ => {}
    }
    return ret != 0;
}
unsafe extern "C" fn op_assign(mut op: OPCODE) {
    let mut lhs: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut t1: *mut NODE = 0 as *mut NODE;
    let mut t2: *mut NODE = 0 as *mut NODE;
    let mut x: libc::c_double = 0.0f64;
    let mut x1: libc::c_double = 0.;
    let mut x2: libc::c_double = 0.;
    let fresh21 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    lhs = (*fresh21).lptr;
    t1 = *lhs;
    x1 = (*force_number(t1)).sub.val.fltnum;
    t2 = TOP_SCALAR();
    x2 = (*force_number(t2)).sub.val.fltnum;
    DEREF(t2);
    match op as libc::c_uint {
        34 => {
            x = x1 + x2;
        }
        35 => {
            x = x1 - x2;
        }
        31 => {
            x = x1 * x2;
        }
        32 => {
            if x2 == 0 as libc::c_int as libc::c_double {
                stack_ptr = stack_ptr.offset(-1);
                stack_ptr;
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"eval.c\0" as *const u8 as *const libc::c_char,
                    1672 as libc::c_int,
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
                        b"division by zero attempted in `/='\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            x = x1 / x2;
        }
        33 => {
            if x2 == 0 as libc::c_int as libc::c_double {
                stack_ptr = stack_ptr.offset(-1);
                stack_ptr;
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"eval.c\0" as *const u8 as *const libc::c_char,
                    1679 as libc::c_int,
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
                        b"division by zero attempted in `%%='\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            x = fmod(x1, x2);
        }
        36 => {
            x = calc_exp(x1, x2);
        }
        _ => {}
    }
    if (*t1).valref == 1 as libc::c_int as libc::c_long
        && (*t1).flags as libc::c_uint
            == (MALLOC as libc::c_int | NUMCUR as libc::c_int | NUMBER as libc::c_int)
                as libc::c_uint
    {
        (*t1).sub.val.fltnum = x;
    } else {
        unref(t1);
        *lhs = make_number.unwrap()(x);
        t1 = *lhs;
    }
    (*t1).valref += 1;
    (*t1).valref;
    (*stack_ptr).rptr = t1;
}
pub unsafe extern "C" fn PUSH_CODE(mut cp: *mut INSTRUCTION) {
    let mut r: *mut NODE = 0 as *mut NODE;
    r = nextfree[BLOCK_NODE as libc::c_int as usize].freep as *mut NODE;
    if !r.is_null() {
        nextfree[BLOCK_NODE as libc::c_int as usize]
            .freep = (*(r as *mut block_item)).freep;
    } else {
        r = more_blocks(BLOCK_NODE as libc::c_int) as *mut NODE;
    };
    (*r).type_0 = Node_instruction;
    (*r).sub.nodep.r.iptr = cp;
    let ref mut fresh22 = (*if stack_ptr < stack_top {
        stack_ptr = stack_ptr.offset(1);
        stack_ptr
    } else {
        grow_stack()
    })
        .rptr;
    *fresh22 = r;
}
pub unsafe extern "C" fn POP_CODE() -> *mut INSTRUCTION {
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut cp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let fresh23 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    r = (*fresh23).rptr;
    cp = (*r).sub.nodep.r.iptr;
    let ref mut fresh24 = (*(r as *mut block_item)).freep;
    *fresh24 = nextfree[BLOCK_NODE as libc::c_int as usize].freep;
    nextfree[BLOCK_NODE as libc::c_int as usize].freep = r as *mut block_item;
    return cp;
}
static mut exec_state_stack: EXEC_STATE = EXEC_STATE {
    next: 0 as *const exec_state as *mut exec_state,
    cptr: 0 as *const INSTRUCTION as *mut INSTRUCTION,
    rule: 0,
    stack_size: 0,
    source: 0 as *const libc::c_char,
};
unsafe extern "C" fn push_exec_state(
    mut cp: *mut INSTRUCTION,
    mut rule: libc::c_int,
    mut src: *mut libc::c_char,
    mut sp: *mut STACK_ITEM,
) {
    let mut es: *mut EXEC_STATE = 0 as *mut EXEC_STATE;
    es = emalloc_real(
        ::std::mem::size_of::<EXEC_STATE>() as libc::c_ulong,
        b"push_exec_state\0" as *const u8 as *const libc::c_char,
        b"es\0" as *const u8 as *const libc::c_char,
        b"eval.c\0" as *const u8 as *const libc::c_char,
        1769 as libc::c_int,
    ) as *mut EXEC_STATE;
    (*es).rule = rule;
    (*es).cptr = cp;
    (*es)
        .stack_size = sp.offset_from(stack_bottom) as libc::c_long
        + 1 as libc::c_int as libc::c_long;
    (*es).source = src;
    (*es).next = exec_state_stack.next;
    exec_state_stack.next = es;
}
unsafe extern "C" fn pop_exec_state(
    mut rule: *mut libc::c_int,
    mut src: *mut *mut libc::c_char,
    mut sz: *mut libc::c_long,
) -> *mut INSTRUCTION {
    let mut cp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut es: *mut EXEC_STATE = 0 as *mut EXEC_STATE;
    es = exec_state_stack.next;
    if es.is_null() {
        return 0 as *mut INSTRUCTION;
    }
    cp = (*es).cptr;
    if !rule.is_null() {
        *rule = (*es).rule;
    }
    if !src.is_null() {
        *src = (*es).source as *mut libc::c_char;
    }
    if !sz.is_null() {
        *sz = (*es).stack_size;
    }
    exec_state_stack.next = (*es).next;
    free(es as *mut libc::c_void);
    return cp;
}
pub unsafe extern "C" fn register_exec_hook(
    mut preh: Func_pre_exec,
    mut posth: Func_post_exec,
) -> libc::c_int {
    let mut pos: libc::c_int = 0 as libc::c_int;
    if preh.is_none() || post_execute.is_some() && posth.is_some() {
        return 0 as libc::c_int;
    }
    if num_exec_hook == 10 as libc::c_int {
        return 0 as libc::c_int;
    }
    if num_exec_hook > 0 as libc::c_int {
        pos = (do_flags as libc::c_uint & DO_DEBUG as libc::c_int as libc::c_uint != 0)
            as libc::c_int;
        if num_exec_hook > pos {
            memmove(
                pre_execute
                    .as_mut_ptr()
                    .offset(pos as isize)
                    .offset(1 as libc::c_int as isize) as *mut libc::c_void,
                pre_execute.as_mut_ptr().offset(pos as isize) as *const libc::c_void,
                ((num_exec_hook - pos) as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<Func_pre_exec>() as libc::c_ulong,
                    ),
            );
        }
    }
    pre_execute[pos as usize] = preh;
    num_exec_hook += 1;
    num_exec_hook;
    if posth.is_some() {
        post_execute = posth;
    }
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn unfield(mut l: *mut *mut NODE, mut r: *mut *mut NODE) {
    if (**r).flags as libc::c_uint & MALLOC as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
        || (**r).valref == 1 as libc::c_int as libc::c_long
    {
        *l = *r;
    } else {
        *l = dupnode(*r);
        DEREF(*r);
    };
}
pub unsafe extern "C" fn r_interpret(mut code: *mut INSTRUCTION) -> libc::c_int {
    let mut current_block: u64;
    let mut pc: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut op: OPCODE = Op_illegal;
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut m: *mut NODE = 0 as *mut NODE;
    let mut ni: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut t1: *mut NODE = 0 as *mut NODE;
    let mut t2: *mut NODE = 0 as *mut NODE;
    let mut lhs: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut x: libc::c_double = 0.;
    let mut x2: libc::c_double = 0.;
    let mut di: libc::c_int = 0;
    let mut rp: *mut Regexp = 0 as *mut Regexp;
    let mut set_array: *mut NODE = 0 as *mut NODE;
    let mut set_idx: *mut NODE = 0 as *mut NODE;
    let mut in_indirect_call: bool = 0 as libc::c_int != 0;
    pc = code;
    '_top: loop {
        if (*pc).source_line as libc::c_int > 0 as libc::c_int {
            sourceline = (*pc).source_line as libc::c_int;
        }
        op = (*pc).opcode;
        if do_itrace {
            fprintf(
                stderr,
                b"+ %s\n\0" as *const u8 as *const libc::c_char,
                opcode2str(op),
            );
            fflush(stderr);
        }
        match op as libc::c_uint {
            51 => {
                currule = (*pc).x.xl as libc::c_int;
                if currule == BEGINFILE as libc::c_int {
                    set_record(
                        b"\0" as *const u8 as *const libc::c_char,
                        0 as libc::c_int as size_t,
                        0 as *const awk_fieldwidth_info_t,
                    );
                }
                current_block = 15848523117724405161;
            }
            101 => {
                current_block = 15848523117724405161;
            }
            107 => {
                let mut stdio_problem: bool = 0 as libc::c_int != 0;
                let mut got_EPIPE: bool = 0 as libc::c_int != 0;
                source = 0 as *mut libc::c_char;
                sourceline = 0 as libc::c_int;
                nextfile(&mut curfile, 1 as libc::c_int != 0);
                close_io(&mut stdio_problem, &mut got_EPIPE);
                if stdio_problem as libc::c_int != 0 && !exiting
                    && exit_val == 0 as libc::c_int
                {
                    exit_val = 1 as libc::c_int;
                }
                close_extensions();
                if got_EPIPE {
                    signal(13 as libc::c_int, None);
                    kill(getpid(), 13 as libc::c_int);
                }
                current_block = 2242099707034464334;
            }
            108 => {
                break;
            }
            78 => {
                m = (*pc).d.dn;
                if do_flags as libc::c_uint
                    & DO_TRADITIONAL as libc::c_int as libc::c_uint == 0
                    && (*m).flags as libc::c_uint
                        & INTLSTR as libc::c_int as libc::c_uint
                        != 0 as libc::c_int as libc::c_uint
                {
                    let mut orig: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut trans: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut save: libc::c_char = 0;
                    save = *((*m).sub.val.sp).offset((*m).sub.val.slen as isize);
                    *((*m).sub.val.sp)
                        .offset(
                            (*m).sub.val.slen as isize,
                        ) = '\0' as i32 as libc::c_char;
                    orig = (*m).sub.val.sp;
                    trans = dcgettext(TEXTDOMAIN, orig, 5 as libc::c_int);
                    *((*m).sub.val.sp).offset((*m).sub.val.slen as isize) = save;
                    if trans != orig {
                        m = make_str_node(trans, strlen(trans), 0 as libc::c_int);
                    } else {
                        (*m).valref += 1;
                        (*m).valref;
                    }
                } else {
                    (*m).valref += 1;
                    (*m).valref;
                }
                let ref mut fresh25 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh25 = m;
                current_block = 2242099707034464334;
            }
            75 | 76 | 77 => {
                let mut current_block_66: u64;
                let mut save_symbol: *mut NODE = 0 as *mut NODE;
                let mut isparam: bool = 0 as libc::c_int != 0;
                m = (*pc).d.dn;
                save_symbol = m;
                if (*m).type_0 as libc::c_uint
                    == Node_param_list as libc::c_int as libc::c_uint
                {
                    isparam = 1 as libc::c_int != 0;
                    m = *((*frame_ptr).sub.nodep.r.av)
                        .offset((*m).sub.nodep.l.ll as isize);
                    save_symbol = m;
                    if (*m).type_0 as libc::c_uint
                        == Node_array_ref as libc::c_int as libc::c_uint
                    {
                        if (*(*m).sub.nodep.l.lptr).type_0 as libc::c_uint
                            == Node_var as libc::c_int as libc::c_uint
                        {
                            current_block_66 = 2519668205773179443;
                        } else {
                            m = (*m).sub.nodep.l.lptr;
                            current_block_66 = 2543120759711851213;
                        }
                    } else {
                        current_block_66 = 2543120759711851213;
                    }
                } else {
                    current_block_66 = 2543120759711851213;
                }
                match current_block_66 {
                    2543120759711851213 => {
                        match (*m).type_0 as libc::c_uint {
                            4 => {
                                current_block_66 = 14551478011259662129;
                                match current_block_66 {
                                    6103714533358002605 => {
                                        r_fatal(
                                            b"internal error: file %s, line %d: unexpected parameter type %s\0"
                                                as *const u8 as *const libc::c_char,
                                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                            263 as libc::c_int,
                                            nodetype2str((*m).type_0),
                                        );
                                    }
                                    14551478011259662129 => {
                                        if do_flags as libc::c_uint
                                            & (DO_LINT_INVALID as libc::c_int
                                                | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                                            && (*m).sub.nodep.l.lptr == Nnull_string
                                        {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const libc::c_char,
                                                    libc::c_int,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                                204 as libc::c_int,
                                            );
                                            (Some(lintfunc.unwrap()))
                                                .unwrap()(
                                                if isparam as libc::c_int != 0 {
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"reference to uninitialized argument `%s'\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    )
                                                } else {
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"reference to uninitialized variable `%s'\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    )
                                                },
                                                (*save_symbol).sub.nodep.name,
                                            );
                                        }
                                        m = (*m).sub.nodep.l.lptr;
                                        (*m).valref += 1;
                                        (*m).valref;
                                        let ref mut fresh26 = (*if stack_ptr < stack_top {
                                            stack_ptr = stack_ptr.offset(1);
                                            stack_ptr
                                        } else {
                                            grow_stack()
                                        })
                                            .rptr;
                                        *fresh26 = m;
                                    }
                                    4929645635333851833 => {
                                        if op as libc::c_uint
                                            == Op_push_arg as libc::c_int as libc::c_uint
                                            || op as libc::c_uint
                                                == Op_push_arg_untyped as libc::c_int as libc::c_uint
                                        {
                                            let ref mut fresh29 = (*if stack_ptr < stack_top {
                                                stack_ptr = stack_ptr.offset(1);
                                                stack_ptr
                                            } else {
                                                grow_stack()
                                            })
                                                .rptr;
                                            *fresh29 = m;
                                        } else {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const libc::c_char,
                                                    libc::c_int,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                                258 as libc::c_int,
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
                                                array_vname(save_symbol),
                                            );
                                        }
                                    }
                                    _ => {
                                        if do_flags as libc::c_uint
                                            & (DO_LINT_INVALID as libc::c_int
                                                | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                                        {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const libc::c_char,
                                                    libc::c_int,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                                236 as libc::c_int,
                                            );
                                            (Some(lintfunc.unwrap()))
                                                .unwrap()(
                                                if isparam as libc::c_int != 0 {
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"reference to uninitialized argument `%s'\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    )
                                                } else {
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"reference to uninitialized variable `%s'\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    )
                                                },
                                                (*save_symbol).sub.nodep.name,
                                            );
                                        }
                                        if op as libc::c_uint
                                            != Op_push_arg_untyped as libc::c_int as libc::c_uint
                                        {
                                            (*m).type_0 = Node_var;
                                            (*m).sub.nodep.l.lptr = dupnode(Nnull_string);
                                            DEREF(m);
                                            m = dupnode(Nnull_string);
                                        }
                                        let ref mut fresh28 = (*if stack_ptr < stack_top {
                                            stack_ptr = stack_ptr.offset(1);
                                            stack_ptr
                                        } else {
                                            grow_stack()
                                        })
                                            .rptr;
                                        *fresh28 = m;
                                    }
                                }
                                current_block_66 = 16779030619667747692;
                            }
                            6 => {
                                current_block_66 = 2519668205773179443;
                            }
                            7 => {
                                current_block_66 = 13727797407265195107;
                                match current_block_66 {
                                    6103714533358002605 => {
                                        r_fatal(
                                            b"internal error: file %s, line %d: unexpected parameter type %s\0"
                                                as *const u8 as *const libc::c_char,
                                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                            263 as libc::c_int,
                                            nodetype2str((*m).type_0),
                                        );
                                    }
                                    14551478011259662129 => {
                                        if do_flags as libc::c_uint
                                            & (DO_LINT_INVALID as libc::c_int
                                                | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                                            && (*m).sub.nodep.l.lptr == Nnull_string
                                        {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const libc::c_char,
                                                    libc::c_int,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                                204 as libc::c_int,
                                            );
                                            (Some(lintfunc.unwrap()))
                                                .unwrap()(
                                                if isparam as libc::c_int != 0 {
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"reference to uninitialized argument `%s'\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    )
                                                } else {
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"reference to uninitialized variable `%s'\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    )
                                                },
                                                (*save_symbol).sub.nodep.name,
                                            );
                                        }
                                        m = (*m).sub.nodep.l.lptr;
                                        (*m).valref += 1;
                                        (*m).valref;
                                        let ref mut fresh26 = (*if stack_ptr < stack_top {
                                            stack_ptr = stack_ptr.offset(1);
                                            stack_ptr
                                        } else {
                                            grow_stack()
                                        })
                                            .rptr;
                                        *fresh26 = m;
                                    }
                                    4929645635333851833 => {
                                        if op as libc::c_uint
                                            == Op_push_arg as libc::c_int as libc::c_uint
                                            || op as libc::c_uint
                                                == Op_push_arg_untyped as libc::c_int as libc::c_uint
                                        {
                                            let ref mut fresh29 = (*if stack_ptr < stack_top {
                                                stack_ptr = stack_ptr.offset(1);
                                                stack_ptr
                                            } else {
                                                grow_stack()
                                            })
                                                .rptr;
                                            *fresh29 = m;
                                        } else {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const libc::c_char,
                                                    libc::c_int,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                                258 as libc::c_int,
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
                                                array_vname(save_symbol),
                                            );
                                        }
                                    }
                                    _ => {
                                        if do_flags as libc::c_uint
                                            & (DO_LINT_INVALID as libc::c_int
                                                | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                                        {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const libc::c_char,
                                                    libc::c_int,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                                236 as libc::c_int,
                                            );
                                            (Some(lintfunc.unwrap()))
                                                .unwrap()(
                                                if isparam as libc::c_int != 0 {
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"reference to uninitialized argument `%s'\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    )
                                                } else {
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"reference to uninitialized variable `%s'\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    )
                                                },
                                                (*save_symbol).sub.nodep.name,
                                            );
                                        }
                                        if op as libc::c_uint
                                            != Op_push_arg_untyped as libc::c_int as libc::c_uint
                                        {
                                            (*m).type_0 = Node_var;
                                            (*m).sub.nodep.l.lptr = dupnode(Nnull_string);
                                            DEREF(m);
                                            m = dupnode(Nnull_string);
                                        }
                                        let ref mut fresh28 = (*if stack_ptr < stack_top {
                                            stack_ptr = stack_ptr.offset(1);
                                            stack_ptr
                                        } else {
                                            grow_stack()
                                        })
                                            .rptr;
                                        *fresh28 = m;
                                    }
                                }
                                current_block_66 = 16779030619667747692;
                            }
                            5 => {
                                current_block_66 = 4929645635333851833;
                                match current_block_66 {
                                    6103714533358002605 => {
                                        r_fatal(
                                            b"internal error: file %s, line %d: unexpected parameter type %s\0"
                                                as *const u8 as *const libc::c_char,
                                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                            263 as libc::c_int,
                                            nodetype2str((*m).type_0),
                                        );
                                    }
                                    14551478011259662129 => {
                                        if do_flags as libc::c_uint
                                            & (DO_LINT_INVALID as libc::c_int
                                                | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                                            && (*m).sub.nodep.l.lptr == Nnull_string
                                        {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const libc::c_char,
                                                    libc::c_int,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                                204 as libc::c_int,
                                            );
                                            (Some(lintfunc.unwrap()))
                                                .unwrap()(
                                                if isparam as libc::c_int != 0 {
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"reference to uninitialized argument `%s'\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    )
                                                } else {
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"reference to uninitialized variable `%s'\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    )
                                                },
                                                (*save_symbol).sub.nodep.name,
                                            );
                                        }
                                        m = (*m).sub.nodep.l.lptr;
                                        (*m).valref += 1;
                                        (*m).valref;
                                        let ref mut fresh26 = (*if stack_ptr < stack_top {
                                            stack_ptr = stack_ptr.offset(1);
                                            stack_ptr
                                        } else {
                                            grow_stack()
                                        })
                                            .rptr;
                                        *fresh26 = m;
                                    }
                                    4929645635333851833 => {
                                        if op as libc::c_uint
                                            == Op_push_arg as libc::c_int as libc::c_uint
                                            || op as libc::c_uint
                                                == Op_push_arg_untyped as libc::c_int as libc::c_uint
                                        {
                                            let ref mut fresh29 = (*if stack_ptr < stack_top {
                                                stack_ptr = stack_ptr.offset(1);
                                                stack_ptr
                                            } else {
                                                grow_stack()
                                            })
                                                .rptr;
                                            *fresh29 = m;
                                        } else {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const libc::c_char,
                                                    libc::c_int,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                                258 as libc::c_int,
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
                                                array_vname(save_symbol),
                                            );
                                        }
                                    }
                                    _ => {
                                        if do_flags as libc::c_uint
                                            & (DO_LINT_INVALID as libc::c_int
                                                | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                                        {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const libc::c_char,
                                                    libc::c_int,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                                236 as libc::c_int,
                                            );
                                            (Some(lintfunc.unwrap()))
                                                .unwrap()(
                                                if isparam as libc::c_int != 0 {
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"reference to uninitialized argument `%s'\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    )
                                                } else {
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"reference to uninitialized variable `%s'\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    )
                                                },
                                                (*save_symbol).sub.nodep.name,
                                            );
                                        }
                                        if op as libc::c_uint
                                            != Op_push_arg_untyped as libc::c_int as libc::c_uint
                                        {
                                            (*m).type_0 = Node_var;
                                            (*m).sub.nodep.l.lptr = dupnode(Nnull_string);
                                            DEREF(m);
                                            m = dupnode(Nnull_string);
                                        }
                                        let ref mut fresh28 = (*if stack_ptr < stack_top {
                                            stack_ptr = stack_ptr.offset(1);
                                            stack_ptr
                                        } else {
                                            grow_stack()
                                        })
                                            .rptr;
                                        *fresh28 = m;
                                    }
                                }
                                current_block_66 = 16779030619667747692;
                            }
                            _ => {
                                current_block_66 = 6103714533358002605;
                                match current_block_66 {
                                    6103714533358002605 => {
                                        r_fatal(
                                            b"internal error: file %s, line %d: unexpected parameter type %s\0"
                                                as *const u8 as *const libc::c_char,
                                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                            263 as libc::c_int,
                                            nodetype2str((*m).type_0),
                                        );
                                    }
                                    14551478011259662129 => {
                                        if do_flags as libc::c_uint
                                            & (DO_LINT_INVALID as libc::c_int
                                                | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                                            && (*m).sub.nodep.l.lptr == Nnull_string
                                        {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const libc::c_char,
                                                    libc::c_int,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                                204 as libc::c_int,
                                            );
                                            (Some(lintfunc.unwrap()))
                                                .unwrap()(
                                                if isparam as libc::c_int != 0 {
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"reference to uninitialized argument `%s'\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    )
                                                } else {
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"reference to uninitialized variable `%s'\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    )
                                                },
                                                (*save_symbol).sub.nodep.name,
                                            );
                                        }
                                        m = (*m).sub.nodep.l.lptr;
                                        (*m).valref += 1;
                                        (*m).valref;
                                        let ref mut fresh26 = (*if stack_ptr < stack_top {
                                            stack_ptr = stack_ptr.offset(1);
                                            stack_ptr
                                        } else {
                                            grow_stack()
                                        })
                                            .rptr;
                                        *fresh26 = m;
                                    }
                                    4929645635333851833 => {
                                        if op as libc::c_uint
                                            == Op_push_arg as libc::c_int as libc::c_uint
                                            || op as libc::c_uint
                                                == Op_push_arg_untyped as libc::c_int as libc::c_uint
                                        {
                                            let ref mut fresh29 = (*if stack_ptr < stack_top {
                                                stack_ptr = stack_ptr.offset(1);
                                                stack_ptr
                                            } else {
                                                grow_stack()
                                            })
                                                .rptr;
                                            *fresh29 = m;
                                        } else {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const libc::c_char,
                                                    libc::c_int,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                                258 as libc::c_int,
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
                                                array_vname(save_symbol),
                                            );
                                        }
                                    }
                                    _ => {
                                        if do_flags as libc::c_uint
                                            & (DO_LINT_INVALID as libc::c_int
                                                | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                                        {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const libc::c_char,
                                                    libc::c_int,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                                236 as libc::c_int,
                                            );
                                            (Some(lintfunc.unwrap()))
                                                .unwrap()(
                                                if isparam as libc::c_int != 0 {
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"reference to uninitialized argument `%s'\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    )
                                                } else {
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"reference to uninitialized variable `%s'\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    )
                                                },
                                                (*save_symbol).sub.nodep.name,
                                            );
                                        }
                                        if op as libc::c_uint
                                            != Op_push_arg_untyped as libc::c_int as libc::c_uint
                                        {
                                            (*m).type_0 = Node_var;
                                            (*m).sub.nodep.l.lptr = dupnode(Nnull_string);
                                            DEREF(m);
                                            m = dupnode(Nnull_string);
                                        }
                                        let ref mut fresh28 = (*if stack_ptr < stack_top {
                                            stack_ptr = stack_ptr.offset(1);
                                            stack_ptr
                                        } else {
                                            grow_stack()
                                        })
                                            .rptr;
                                        *fresh28 = m;
                                    }
                                }
                                current_block_66 = 16779030619667747692;
                            }
                        }
                    }
                    _ => {}
                }
                match current_block_66 {
                    2519668205773179443 => {
                        if do_flags as libc::c_uint
                            & (DO_LINT_INVALID as libc::c_int
                                | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                        {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    libc::c_int,
                                ) -> ())(
                                b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                216 as libc::c_int,
                            );
                            (Some(lintfunc.unwrap()))
                                .unwrap()(
                                if isparam as libc::c_int != 0 {
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"reference to uninitialized argument `%s'\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    )
                                } else {
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"reference to uninitialized variable `%s'\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    )
                                },
                                (*save_symbol).sub.nodep.name,
                            );
                        }
                        if op as libc::c_uint
                            != Op_push_arg_untyped as libc::c_int as libc::c_uint
                        {
                            (*m).type_0 = Node_var;
                            (*m).sub.nodep.l.lptr = dupnode(Nnull_string);
                            m = dupnode(Nnull_string);
                        }
                        (*m).valref += 1;
                        (*m).valref;
                        let ref mut fresh27 = (*if stack_ptr < stack_top {
                            stack_ptr = stack_ptr.offset(1);
                            stack_ptr
                        } else {
                            grow_stack()
                        })
                            .rptr;
                        *fresh27 = m;
                    }
                    _ => {}
                }
                current_block = 2242099707034464334;
            }
            81 => {
                m = (*pc).d.dn;
                if (*m).type_0 as libc::c_uint
                    == Node_param_list as libc::c_int as libc::c_uint
                {
                    m = *((*frame_ptr).sub.nodep.r.av)
                        .offset((*m).sub.nodep.l.ll as isize);
                }
                if (*m).type_0 as libc::c_uint == Node_var as libc::c_int as libc::c_uint
                {
                    m = (*m).sub.nodep.l.lptr;
                    (*m).valref += 1;
                    (*m).valref;
                    let ref mut fresh30 = (*if stack_ptr < stack_top {
                        stack_ptr = stack_ptr.offset(1);
                        stack_ptr
                    } else {
                        grow_stack()
                    })
                        .rptr;
                    *fresh30 = m;
                    current_block = 2242099707034464334;
                } else {
                    current_block = 11276394722809792258;
                }
            }
            80 => {
                current_block = 11276394722809792258;
            }
            82 => {
                lhs = if (*(*pc).d.dn).type_0 as libc::c_uint
                    == Node_var as libc::c_int as libc::c_uint
                    && !((*(*pc).d.dn).sub.nodep.l.lptr == Nnull_string)
                {
                    &mut (*(*pc).d.dn).sub.nodep.l.lptr
                } else {
                    r_get_lhs((*pc).d.dn, (*pc).x.xl != 0)
                };
                let ref mut fresh32 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .lptr;
                *fresh32 = lhs;
                current_block = 2242099707034464334;
            }
            16 => {
                t2 = if (*pc).d.dl == 1 as libc::c_int as libc::c_long {
                    POP_SCALAR()
                } else {
                    concat_exp((*pc).d.dl as libc::c_int, 1 as libc::c_int != 0)
                };
                t1 = POP_ARRAY(0 as libc::c_int != 0);
                if (in_array(t1, t2)).is_null() {
                    t2 = force_string_fmt(t2, CONVFMT, CONVFMTidx);
                    if t1 == func_table {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                            296 as libc::c_int,
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
                                b"reference to uninitialized element `%s[\"%.*s\"] is not allowed'\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            b"FUNCTAB\0" as *const u8 as *const libc::c_char,
                            (*t2).sub.val.slen as libc::c_int,
                            (*t2).sub.val.sp,
                        );
                    } else if t1 == symbol_table {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                            299 as libc::c_int,
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
                                b"reference to uninitialized element `%s[\"%.*s\"] is not allowed'\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            b"SYMTAB\0" as *const u8 as *const libc::c_char,
                            (*t2).sub.val.slen as libc::c_int,
                            (*t2).sub.val.sp,
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
                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                            302 as libc::c_int,
                        );
                        (Some(lintfunc.unwrap()))
                            .unwrap()(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"reference to uninitialized element `%s[\"%.*s\"]'\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            array_vname(t1),
                            (*t2).sub.val.slen as libc::c_int,
                            (*t2).sub.val.sp,
                        );
                        if (*t2).sub.val.slen == 0 as libc::c_int as libc::c_ulong {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    libc::c_int,
                                ) -> ())(
                                b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                305 as libc::c_int,
                            );
                            (Some(lintfunc.unwrap()))
                                .unwrap()(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"subscript of array `%s' is null string\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                array_vname(t1),
                            );
                        }
                    }
                }
                if t1 == func_table {
                    static mut warned: bool = 0 as libc::c_int != 0;
                    if do_flags as libc::c_uint
                        & DO_LINT_EXTENSIONS as libc::c_int as libc::c_uint != 0
                        && !warned
                    {
                        warned = 1 as libc::c_int != 0;
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                            317 as libc::c_int,
                        );
                        (Some(lintfunc.unwrap()))
                            .unwrap()(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"FUNCTAB is a gawk extension\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                    r = t2;
                } else {
                    if t1 == symbol_table {
                        update_global_values();
                    }
                    r = *((*(*t1).sub.nodep.l.lp).lookup).unwrap()(t1, t2);
                }
                DEREF(t2);
                if t1 == symbol_table {
                    static mut warned_0: bool = 0 as libc::c_int != 0;
                    if do_flags as libc::c_uint
                        & DO_LINT_EXTENSIONS as libc::c_int as libc::c_uint != 0
                        && !warned_0
                    {
                        warned_0 = 1 as libc::c_int != 0;
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                            335 as libc::c_int,
                        );
                        (Some(lintfunc.unwrap()))
                            .unwrap()(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"SYMTAB is a gawk extension\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                    if (*r).type_0 as libc::c_uint
                        == Node_var as libc::c_int as libc::c_uint
                    {
                        r = (*r).sub.nodep.l.lptr;
                    } else if (*r).type_0 as libc::c_uint
                        == Node_var_new as libc::c_int as libc::c_uint
                    {
                        (*r).sub.nodep.l.lptr = dupnode(Nnull_string);
                        r = (*r).sub.nodep.l.lptr;
                    }
                }
                if (*r).type_0 as libc::c_uint == Node_val as libc::c_int as libc::c_uint
                    || (*r).type_0 as libc::c_uint
                        == Node_var as libc::c_int as libc::c_uint
                    || (*r).type_0 as libc::c_uint
                        == Node_elem_new as libc::c_int as libc::c_uint
                {
                    (*r).valref += 1;
                    (*r).valref;
                }
                let ref mut fresh33 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh33 = r;
                current_block = 2242099707034464334;
            }
            17 => {
                t2 = if (*pc).d.dl == 1 as libc::c_int as libc::c_long {
                    POP_SCALAR()
                } else {
                    concat_exp((*pc).d.dl as libc::c_int, 1 as libc::c_int != 0)
                };
                t1 = POP_ARRAY(0 as libc::c_int != 0);
                r = in_array(t1, t2);
                if r.is_null() {
                    t2 = force_string_fmt(t2, CONVFMT, CONVFMTidx);
                    if t1 == func_table {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                            362 as libc::c_int,
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
                                b"reference to uninitialized element `%s[\"%.*s\"] is not allowed'\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            b"FUNCTAB\0" as *const u8 as *const libc::c_char,
                            (*t2).sub.val.slen as libc::c_int,
                            (*t2).sub.val.sp,
                        );
                    } else if t1 == symbol_table {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                            365 as libc::c_int,
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
                                b"reference to uninitialized element `%s[\"%.*s\"] is not allowed'\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            b"SYMTAB\0" as *const u8 as *const libc::c_char,
                            (*t2).sub.val.slen as libc::c_int,
                            (*t2).sub.val.sp,
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
                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                            368 as libc::c_int,
                        );
                        (Some(lintfunc.unwrap()))
                            .unwrap()(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"reference to uninitialized element `%s[\"%.*s\"]'\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            array_vname(t1),
                            (*t2).sub.val.slen as libc::c_int,
                            (*t2).sub.val.sp,
                        );
                        if (*t2).sub.val.slen == 0 as libc::c_int as libc::c_ulong {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    libc::c_int,
                                ) -> ())(
                                b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                371 as libc::c_int,
                            );
                            (Some(lintfunc.unwrap()))
                                .unwrap()(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"subscript of array `%s' is null string\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                array_vname(t1),
                            );
                        }
                    }
                }
                if r.is_null() {
                    r = make_array();
                    (*r).sub.nodep.x.extra = t1;
                    t2 = force_string_fmt(t2, CONVFMT, CONVFMTidx);
                    (*r).sub.nodep.name = estrdup((*t2).sub.val.sp, (*t2).sub.val.slen);
                    assoc_set(t1, t2, r);
                } else if (*r).type_0 as libc::c_uint
                    == Node_elem_new as libc::c_int as libc::c_uint
                {
                    r = force_array(r, 0 as libc::c_int != 0);
                    (*r).sub.nodep.x.extra = t1;
                    t2 = force_string_fmt(t2, CONVFMT, CONVFMTidx);
                    (*r).sub.nodep.name = estrdup((*t2).sub.val.sp, (*t2).sub.val.slen);
                } else if (*r).type_0 as libc::c_uint
                    != Node_var_array as libc::c_int as libc::c_uint
                {
                    t2 = force_string_fmt(t2, CONVFMT, CONVFMTidx);
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const libc::c_char,
                        388 as libc::c_int,
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
                            b"attempt to use scalar `%s[\"%.*s\"]' as an array\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        array_vname(t1),
                        (*t2).sub.val.slen as libc::c_int,
                        (*t2).sub.val.sp,
                    );
                } else {
                    DEREF(t2);
                }
                let ref mut fresh34 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh34 = r;
                current_block = 2242099707034464334;
            }
            83 => {
                t2 = if (*pc).d.dl == 1 as libc::c_int as libc::c_long {
                    POP_SCALAR()
                } else {
                    concat_exp((*pc).d.dl as libc::c_int, 1 as libc::c_int != 0)
                };
                t1 = POP_ARRAY(0 as libc::c_int != 0);
                if do_flags as libc::c_uint
                    & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                        as libc::c_uint != 0 && (in_array(t1, t2)).is_null()
                {
                    t2 = force_string_fmt(t2, CONVFMT, CONVFMTidx);
                    if (*pc).x.xl != 0 {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                            402 as libc::c_int,
                        );
                        (Some(lintfunc.unwrap()))
                            .unwrap()(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"reference to uninitialized element `%s[\"%.*s\"]'\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            array_vname(t1),
                            (*t2).sub.val.slen as libc::c_int,
                            (*t2).sub.val.sp,
                        );
                    }
                    if (*t2).sub.val.slen == 0 as libc::c_int as libc::c_ulong {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                            405 as libc::c_int,
                        );
                        (Some(lintfunc.unwrap()))
                            .unwrap()(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"subscript of array `%s' is null string\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            array_vname(t1),
                        );
                    }
                }
                lhs = ((*(*t1).sub.nodep.l.lp).lookup).unwrap()(t1, t2);
                if (**lhs).type_0 as libc::c_uint
                    == Node_var_array as libc::c_int as libc::c_uint
                {
                    t2 = force_string_fmt(t2, CONVFMT, CONVFMTidx);
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const libc::c_char,
                        411 as libc::c_int,
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
                            b"attempt to use array `%s[\"%.*s\"]' in a scalar context\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        array_vname(t1),
                        (*t2).sub.val.slen as libc::c_int,
                        (*t2).sub.val.sp,
                    );
                }
                if t1 == func_table {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const libc::c_char,
                        429 as libc::c_int,
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
                            b"cannot assign to elements of FUNCTAB\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                } else if t1 == symbol_table {
                    if (**lhs).type_0 as libc::c_uint
                        == Node_var as libc::c_int as libc::c_uint
                        || (**lhs).type_0 as libc::c_uint
                            == Node_var_new as libc::c_int as libc::c_uint
                    {
                        update_global_values();
                        (**lhs).type_0 = Node_var;
                        lhs = &mut (**lhs).sub.nodep.l.lptr;
                    } else {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                            437 as libc::c_int,
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
                                b"cannot assign to arbitrary elements of SYMTAB\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                }
                if ((*(*t1).sub.nodep.l.lp).store).is_some() {
                    set_array = t1;
                    set_idx = t2;
                } else {
                    DEREF(t2);
                }
                let ref mut fresh35 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .lptr;
                *fresh35 = lhs;
                current_block = 2242099707034464334;
            }
            24 => {
                t1 = TOP_SCALAR();
                lhs = r_get_field(t1, 0 as *mut Func_ptr, 1 as libc::c_int != 0);
                stack_ptr = stack_ptr.offset(-1);
                stack_ptr;
                DEREF(t1);
                r = *lhs;
                (*r).valref += 1;
                (*r).valref;
                let ref mut fresh36 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh36 = r;
                current_block = 2242099707034464334;
            }
            84 => {
                t1 = TOP_SCALAR();
                lhs = r_get_field(t1, &mut (*(*pc).d.di).x.aptr, (*pc).x.xl != 0);
                stack_ptr = stack_ptr.offset(-1);
                stack_ptr;
                DEREF(t1);
                let ref mut fresh37 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .lptr;
                *fresh37 = lhs;
                current_block = 2242099707034464334;
            }
            105 => {
                if do_flags as libc::c_uint
                    & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                        as libc::c_uint != 0
                {
                    match (*pc).d.dl {
                        1 => {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    libc::c_int,
                                ) -> ())(
                                b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                474 as libc::c_int,
                            );
                            (Some(lintfunc.unwrap()))
                                .unwrap()(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"assignment used in conditional context\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                        }
                        _ => {
                            r_fatal(
                                b"internal error: file %s, line %d: unexpected lint type value %d\0"
                                    as *const u8 as *const libc::c_char,
                                b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                478 as libc::c_int,
                                (*pc).d.dl as libc::c_int,
                            );
                        }
                    }
                }
                current_block = 2242099707034464334;
            }
            106 => {
                t1 = (*stack_ptr).rptr;
                t2 = (*stack_ptr.offset(-(1 as libc::c_int as isize))).rptr;
                if (*t1).flags as libc::c_uint & STRING as libc::c_int as libc::c_uint
                    != 0 as libc::c_int as libc::c_uint
                    && (*t2).flags as libc::c_uint
                        & STRING as libc::c_int as libc::c_uint
                        != 0 as libc::c_int as libc::c_uint
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const libc::c_char,
                        489 as libc::c_int,
                    );
                    (Some(lintfunc.unwrap()))
                        .unwrap()(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"operator `+' used on two string values\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                current_block = 2242099707034464334;
            }
            54 | 55 | 87 => {
                pc = (*pc).d.di;
                continue;
            }
            89 => {
                r = POP_SCALAR();
                di = eval_condition(r) as libc::c_int;
                DEREF(r);
                if di == 0 {
                    pc = (*pc).d.di;
                    continue;
                } else {
                    current_block = 2242099707034464334;
                }
            }
            88 => {
                r = POP_SCALAR();
                di = eval_condition(r) as libc::c_int;
                DEREF(r);
                if di != 0 {
                    pc = (*pc).d.di;
                    continue;
                } else {
                    current_block = 2242099707034464334;
                }
            }
            38 | 40 => {
                t1 = POP_SCALAR();
                di = eval_condition(t1) as libc::c_int;
                DEREF(t1);
                if op as libc::c_uint == Op_and as libc::c_int as libc::c_uint && di != 0
                    || op as libc::c_uint == Op_or as libc::c_int as libc::c_uint
                        && di == 0
                {
                    current_block = 2242099707034464334;
                } else {
                    r = node_Boolean[di as usize];
                    (*r).valref += 1;
                    (*r).valref;
                    let ref mut fresh38 = (*if stack_ptr < stack_top {
                        stack_ptr = stack_ptr.offset(1);
                        stack_ptr
                    } else {
                        grow_stack()
                    })
                        .rptr;
                    *fresh38 = r;
                    ni = (*pc).d.di;
                    pc = (*ni).nexti;
                    continue;
                }
            }
            39 | 41 => {
                t1 = TOP_SCALAR();
                r = node_Boolean[eval_condition(t1) as usize];
                DEREF(t1);
                (*r).valref += 1;
                (*r).valref;
                (*stack_ptr).rptr = r;
                current_block = 2242099707034464334;
            }
            25 => {
                t1 = TOP_SCALAR();
                r = node_Boolean[!eval_condition(t1) as libc::c_int as usize];
                DEREF(t1);
                (*r).valref += 1;
                (*r).valref;
                (*stack_ptr).rptr = r;
                current_block = 2242099707034464334;
            }
            42 => {
                r = node_Boolean[cmp_scalars(SCALAR_EQ) as usize];
                (*r).valref += 1;
                (*r).valref;
                (*stack_ptr).rptr = r;
                current_block = 2242099707034464334;
            }
            43 => {
                r = node_Boolean[cmp_scalars(SCALAR_NEQ) as usize];
                (*r).valref += 1;
                (*r).valref;
                (*stack_ptr).rptr = r;
                current_block = 2242099707034464334;
            }
            44 => {
                r = node_Boolean[cmp_scalars(SCALAR_LT) as usize];
                (*r).valref += 1;
                (*r).valref;
                (*stack_ptr).rptr = r;
                current_block = 2242099707034464334;
            }
            45 => {
                r = node_Boolean[cmp_scalars(SCALAR_GT) as usize];
                (*r).valref += 1;
                (*r).valref;
                (*stack_ptr).rptr = r;
                current_block = 2242099707034464334;
            }
            46 => {
                r = node_Boolean[cmp_scalars(SCALAR_LE) as usize];
                (*r).valref += 1;
                (*r).valref;
                (*stack_ptr).rptr = r;
                current_block = 2242099707034464334;
            }
            47 => {
                r = node_Boolean[cmp_scalars(SCALAR_GE) as usize];
                (*r).valref += 1;
                (*r).valref;
                (*stack_ptr).rptr = r;
                current_block = 2242099707034464334;
            }
            8 => {
                x2 = (*force_number((*pc).d.dn)).sub.val.fltnum;
                current_block = 8953299237562637943;
            }
            7 => {
                t2 = force_number(POP_SCALAR());
                x2 = (*t2).sub.val.fltnum;
                DEREF(t2);
                current_block = 8953299237562637943;
            }
            10 => {
                x2 = (*force_number((*pc).d.dn)).sub.val.fltnum;
                current_block = 2118905164450504584;
            }
            9 => {
                t2 = force_number(POP_SCALAR());
                x2 = (*t2).sub.val.fltnum;
                DEREF(t2);
                current_block = 2118905164450504584;
            }
            2 => {
                x2 = (*force_number((*pc).d.dn)).sub.val.fltnum;
                current_block = 12385243566997464291;
            }
            1 => {
                t2 = force_number(POP_SCALAR());
                x2 = (*t2).sub.val.fltnum;
                DEREF(t2);
                current_block = 12385243566997464291;
            }
            12 => {
                x2 = (*force_number((*pc).d.dn)).sub.val.fltnum;
                current_block = 459172418090906441;
            }
            11 => {
                t2 = force_number(POP_SCALAR());
                x2 = (*t2).sub.val.fltnum;
                DEREF(t2);
                current_block = 459172418090906441;
            }
            4 => {
                x2 = (*force_number((*pc).d.dn)).sub.val.fltnum;
                current_block = 2910841344195686645;
            }
            3 => {
                t2 = force_number(POP_SCALAR());
                x2 = (*t2).sub.val.fltnum;
                DEREF(t2);
                current_block = 2910841344195686645;
            }
            6 => {
                x2 = (*force_number((*pc).d.dn)).sub.val.fltnum;
                current_block = 13208848489772510154;
            }
            5 => {
                t2 = force_number(POP_SCALAR());
                x2 = (*t2).sub.val.fltnum;
                DEREF(t2);
                current_block = 13208848489772510154;
            }
            18 | 19 => {
                x = if op as libc::c_uint
                    == Op_preincrement as libc::c_int as libc::c_uint
                {
                    1.0f64
                } else {
                    -1.0f64
                };
                lhs = (*stack_ptr).lptr;
                t1 = *lhs;
                force_number(t1);
                if (*t1).valref == 1 as libc::c_int as libc::c_long
                    && (*t1).flags as libc::c_uint
                        == (MALLOC as libc::c_int | NUMCUR as libc::c_int
                            | NUMBER as libc::c_int) as libc::c_uint
                {
                    (*t1).sub.val.fltnum += x;
                    r = t1;
                } else {
                    *lhs = make_number.unwrap()((*t1).sub.val.fltnum + x);
                    r = *lhs;
                    unref(t1);
                }
                (*r).valref += 1;
                (*r).valref;
                (*stack_ptr).rptr = r;
                current_block = 2242099707034464334;
            }
            20 | 21 => {
                x = if op as libc::c_uint
                    == Op_postincrement as libc::c_int as libc::c_uint
                {
                    1.0f64
                } else {
                    -1.0f64
                };
                lhs = (*stack_ptr).lptr;
                t1 = *lhs;
                force_number(t1);
                r = make_number.unwrap()((*t1).sub.val.fltnum);
                if (*t1).valref == 1 as libc::c_int as libc::c_long
                    && (*t1).flags as libc::c_uint
                        == (MALLOC as libc::c_int | NUMCUR as libc::c_int
                            | NUMBER as libc::c_int) as libc::c_uint
                {
                    (*t1).sub.val.fltnum += x;
                } else {
                    *lhs = make_number.unwrap()((*t1).sub.val.fltnum + x);
                    unref(t1);
                }
                (*stack_ptr).rptr = r;
                current_block = 2242099707034464334;
            }
            22 => {
                t1 = force_number(TOP_SCALAR());
                r = make_number.unwrap()(-(*t1).sub.val.fltnum);
                DEREF(t1);
                (*stack_ptr).rptr = r;
                current_block = 2242099707034464334;
            }
            23 => {
                t1 = force_number(TOP_SCALAR());
                r = make_number.unwrap()((*t1).sub.val.fltnum);
                DEREF(t1);
                (*stack_ptr).rptr = r;
                current_block = 2242099707034464334;
            }
            28 => {
                t1 = force_array((*pc).d.dn, 1 as libc::c_int != 0);
                t2 = if (*pc).x.xl == 1 as libc::c_int as libc::c_long {
                    POP_SCALAR()
                } else {
                    concat_exp((*pc).x.xl as libc::c_int, 1 as libc::c_int != 0)
                };
                lhs = ((*(*t1).sub.nodep.l.lp).lookup).unwrap()(t1, t2);
                if (**lhs).type_0 as libc::c_uint
                    == Node_var_array as libc::c_int as libc::c_uint
                {
                    t2 = force_string_fmt(t2, CONVFMT, CONVFMTidx);
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const libc::c_char,
                        737 as libc::c_int,
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
                            b"attempt to use array `%s[\"%.*s\"]' in a scalar context\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        array_vname(t1),
                        (*t2).sub.val.slen as libc::c_int,
                        (*t2).sub.val.sp,
                    );
                }
                DEREF(t2);
                if t1 == func_table {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const libc::c_char,
                        755 as libc::c_int,
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
                            b"cannot assign to elements of FUNCTAB\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                } else if t1 == symbol_table {
                    if (**lhs).type_0 as libc::c_uint
                        == Node_var as libc::c_int as libc::c_uint
                        || (**lhs).type_0 as libc::c_uint
                            == Node_var_new as libc::c_int as libc::c_uint
                    {
                        update_global_values();
                        (**lhs).type_0 = Node_var;
                        lhs = &mut (**lhs).sub.nodep.l.lptr;
                    } else {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                            763 as libc::c_int,
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
                                b"cannot assign to arbitrary elements of SYMTAB\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                }
                unref(*lhs);
                r = POP_SCALAR();
                unfield(lhs, &mut r);
                if ((*(*t1).sub.nodep.l.lp).store).is_some() {
                    (Some(((*(*t1).sub.nodep.l.lp).store).unwrap())).unwrap()(t1, t2);
                }
                DEREF(t2);
                current_block = 2242099707034464334;
            }
            27 => {
                lhs = if (*(*pc).d.dn).type_0 as libc::c_uint
                    == Node_var as libc::c_int as libc::c_uint
                    && !((*(*pc).d.dn).sub.nodep.l.lptr == Nnull_string)
                {
                    &mut (*(*pc).d.dn).sub.nodep.l.lptr
                } else {
                    r_get_lhs((*pc).d.dn, 0 as libc::c_int != 0)
                };
                unref(*lhs);
                r = (*pc).x.xn;
                if !r.is_null() {
                    (*r).valref += 1;
                    (*r).valref;
                    *lhs = r;
                } else {
                    r = POP_SCALAR();
                    unfield(lhs, &mut r);
                }
                current_block = 2242099707034464334;
            }
            29 | 30 => {
                let mut assign: Func_ptr = None;
                t1 = TOP_SCALAR();
                lhs = r_get_field(t1, &mut assign, 0 as libc::c_int != 0);
                stack_ptr = stack_ptr.offset(-1);
                stack_ptr;
                DEREF(t1);
                assign.unwrap()();
                unref(*lhs);
                r = POP_SCALAR();
                unfield(lhs, &mut r);
                force_string_fmt(*lhs, CONVFMT, CONVFMTidx);
                if op as libc::c_uint
                    == Op_store_field_exp as libc::c_int as libc::c_uint
                {
                    (**lhs).valref += 1;
                    (**lhs).valref;
                    let ref mut fresh39 = (*if stack_ptr < stack_top {
                        stack_ptr = stack_ptr.offset(1);
                        stack_ptr
                    } else {
                        grow_stack()
                    })
                        .rptr;
                    *fresh39 = *lhs;
                }
                current_block = 2242099707034464334;
            }
            37 => {
                lhs = if (*(*pc).d.dn).type_0 as libc::c_uint
                    == Node_var as libc::c_int as libc::c_uint
                    && !((*(*pc).d.dn).sub.nodep.l.lptr == Nnull_string)
                {
                    &mut (*(*pc).d.dn).sub.nodep.l.lptr
                } else {
                    r_get_lhs((*pc).d.dn, 0 as libc::c_int != 0)
                };
                t1 = force_string_fmt(*lhs, CONVFMT, CONVFMTidx);
                t2 = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
                if t1 != *lhs {
                    unref(*lhs);
                    if (*t1).valref == 1 as libc::c_int as libc::c_long {
                        *lhs = t1;
                    } else {
                        *lhs = dupnode(t1);
                    }
                }
                if t1 != t2 && (*t1).valref == 1 as libc::c_int as libc::c_long
                    && (*t1).flags as libc::c_uint
                        & (MALLOC as libc::c_int | MPFN as libc::c_int
                            | MPZN as libc::c_int) as libc::c_uint
                        == MALLOC as libc::c_int as libc::c_uint
                {
                    let mut nlen: size_t = ((*t1).sub.val.slen)
                        .wrapping_add((*t2).sub.val.slen);
                    (*t1)
                        .sub
                        .val
                        .sp = erealloc_real(
                        (*t1).sub.val.sp as *mut libc::c_void,
                        nlen.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        b"r_interpret\0" as *const u8 as *const libc::c_char,
                        b"t1->stptr\0" as *const u8 as *const libc::c_char,
                        b"./interpret.h\0" as *const u8 as *const libc::c_char,
                        843 as libc::c_int,
                    ) as *mut libc::c_char;
                    memcpy(
                        ((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize)
                            as *mut libc::c_void,
                        (*t2).sub.val.sp as *const libc::c_void,
                        (*t2).sub.val.slen,
                    );
                    (*t1).sub.val.slen = nlen;
                    *((*t1).sub.val.sp)
                        .offset(nlen as isize) = '\0' as i32 as libc::c_char;
                    (*t1)
                        .flags = ::std::mem::transmute::<
                        libc::c_uint,
                        flagvals,
                    >(
                        (*t1).flags as libc::c_uint
                            & WSTRCUR as libc::c_int as libc::c_uint,
                    );
                    (*t1)
                        .flags = ::std::mem::transmute::<
                        libc::c_uint,
                        flagvals,
                    >(
                        (*t1).flags as libc::c_uint
                            | (MALLOC as libc::c_int | STRING as libc::c_int
                                | STRCUR as libc::c_int) as libc::c_uint,
                    );
                    (*t1).sub.val.idx = -(1 as libc::c_int);
                    if (*t1).flags as libc::c_uint
                        & WSTRCUR as libc::c_int as libc::c_uint
                        != 0 as libc::c_int as libc::c_uint
                        && (*t2).flags as libc::c_uint
                            & WSTRCUR as libc::c_int as libc::c_uint
                            != 0 as libc::c_int as libc::c_uint
                    {
                        let mut wlen: size_t = ((*t1).sub.val.wslen)
                            .wrapping_add((*t2).sub.val.wslen);
                        (*t1)
                            .sub
                            .val
                            .wsp = erealloc_real(
                            (*t1).sub.val.wsp as *mut libc::c_void,
                            (::std::mem::size_of::<wchar_t>() as libc::c_ulong)
                                .wrapping_mul(
                                    wlen.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                ),
                            b"r_interpret\0" as *const u8 as *const libc::c_char,
                            b"t1->wstptr\0" as *const u8 as *const libc::c_char,
                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                            860 as libc::c_int,
                        ) as *mut wchar_t;
                        memcpy(
                            ((*t1).sub.val.wsp).offset((*t1).sub.val.wslen as isize)
                                as *mut libc::c_void,
                            (*t2).sub.val.wsp as *const libc::c_void,
                            ((*t2).sub.val.wslen)
                                .wrapping_mul(
                                    ::std::mem::size_of::<wchar_t>() as libc::c_ulong,
                                ),
                        );
                        (*t1).sub.val.wslen = wlen;
                        *((*t1).sub.val.wsp).offset(wlen as isize) = '\0' as i32;
                    } else if (**lhs).flags as libc::c_uint
                        & WSTRCUR as libc::c_int as libc::c_uint != 0
                    {
                        r_free_wstr(*lhs);
                    }
                } else {
                    let mut nlen_0: size_t = ((*t1).sub.val.slen)
                        .wrapping_add((*t2).sub.val.slen);
                    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
                    p = emalloc_real(
                        nlen_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        b"r_interpret\0" as *const u8 as *const libc::c_char,
                        b"p\0" as *const u8 as *const libc::c_char,
                        b"./interpret.h\0" as *const u8 as *const libc::c_char,
                        870 as libc::c_int,
                    ) as *mut libc::c_char;
                    memcpy(
                        p as *mut libc::c_void,
                        (*t1).sub.val.sp as *const libc::c_void,
                        (*t1).sub.val.slen,
                    );
                    memcpy(
                        p.offset((*t1).sub.val.slen as isize) as *mut libc::c_void,
                        (*t2).sub.val.sp as *const libc::c_void,
                        (*t2).sub.val.slen,
                    );
                    unref(*lhs);
                    *lhs = make_str_node(p, nlen_0, 2 as libc::c_int);
                    t1 = *lhs;
                }
                DEREF(t2);
                current_block = 2242099707034464334;
            }
            26 => {
                let fresh40 = stack_ptr;
                stack_ptr = stack_ptr.offset(-1);
                lhs = (*fresh40).lptr;
                r = TOP_SCALAR();
                unref(*lhs);
                if (*r).type_0 as libc::c_uint
                    == Node_elem_new as libc::c_int as libc::c_uint
                {
                    DEREF(r);
                    r = dupnode(Nnull_string);
                }
                (*r).valref += 1;
                (*r).valref;
                unfield(lhs, &mut r);
                (*stack_ptr).rptr = r;
                current_block = 2242099707034464334;
            }
            98 => {
                if !set_idx.is_null() {
                    di = 1 as libc::c_int;
                    if (*pc).d.dl == Op_sub_builtin as libc::c_int as libc::c_long
                        && {
                            r = (*stack_ptr).rptr;
                            !r.is_null()
                        }
                        && (*r).sub.val.fltnum as libc::c_long
                            == 0 as libc::c_int as libc::c_long
                    {
                        di = 0 as libc::c_int;
                    } else if ((*pc).d.dl == Op_K_getline as libc::c_int as libc::c_long
                        || (*pc).d.dl
                            == Op_K_getline_redir as libc::c_int as libc::c_long)
                        && {
                            r = (*stack_ptr).rptr;
                            !r.is_null()
                        }
                        && (*r).sub.val.fltnum as libc::c_long
                            <= 0 as libc::c_int as libc::c_long
                    {
                        di = 0 as libc::c_int;
                    }
                    if di != 0 {
                        (Some(((*(*set_array).sub.nodep.l.lp).store).unwrap()))
                            .unwrap()(set_array, set_idx);
                    }
                    unref(set_idx);
                    set_idx = 0 as *mut NODE;
                }
                current_block = 2242099707034464334;
            }
            34 | 35 | 31 | 32 | 33 | 36 => {
                op_assign(op);
                current_block = 2242099707034464334;
            }
            95 => {
                ((*pc).x.aptr).unwrap()();
                current_block = 2242099707034464334;
            }
            96 | 97 => {
                r = (*stack_ptr).rptr;
                if (*pc).d.dl == Op_sub_builtin as libc::c_int as libc::c_long
                    && (*r).sub.val.fltnum as libc::c_long
                        == 0 as libc::c_int as libc::c_long
                {
                    current_block = 2242099707034464334;
                } else if ((*pc).d.dl == Op_K_getline as libc::c_int as libc::c_long
                    || (*pc).d.dl == Op_K_getline_redir as libc::c_int as libc::c_long)
                    && (*r).sub.val.fltnum as libc::c_long
                        <= 0 as libc::c_int as libc::c_long
                {
                    current_block = 2242099707034464334;
                } else {
                    if op as libc::c_uint == Op_var_assign as libc::c_int as libc::c_uint
                    {
                        ((*pc).x.aptr).unwrap()();
                    } else {
                        ((*pc).x.aptr).unwrap()();
                    }
                    current_block = 2242099707034464334;
                }
            }
            13 => {
                r = concat_exp(
                    (*pc).x.xl as libc::c_int,
                    (*pc).d.dl & 1 as libc::c_int as libc::c_long != 0,
                );
                let ref mut fresh41 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh41 = r;
                current_block = 2242099707034464334;
            }
            52 => {
                if (*pc.offset(1 as libc::c_int as isize)).x.xl != 0 {
                    let fresh42 = stack_ptr;
                    stack_ptr = stack_ptr.offset(-1);
                    m = (*fresh42).rptr;
                    t2 = TOP_SCALAR();
                    t2 = force_string_fmt(t2, CONVFMT, CONVFMTidx);
                    rp = re_update(m);
                    di = (research(
                        rp,
                        (*t2).sub.val.sp,
                        0 as libc::c_int,
                        (*t2).sub.val.slen,
                        0 as libc::c_int,
                    ) >= 0 as libc::c_int) as libc::c_int;
                } else {
                    t1 = POP_SCALAR();
                    t2 = TOP_SCALAR();
                    di = (cmp_nodes(t2, t1, 1 as libc::c_int != 0) == 0 as libc::c_int)
                        as libc::c_int;
                    DEREF(t1);
                }
                if di != 0 {
                    t2 = POP_SCALAR();
                    DEREF(t2);
                    pc = (*pc).d.di;
                    continue;
                } else {
                    current_block = 2242099707034464334;
                }
            }
            63 => {
                t1 = POP_ARRAY(0 as libc::c_int != 0);
                do_delete(t1, (*pc).x.xl as libc::c_int);
                stack_ptr = stack_ptr.offset(-(*pc).x.xl as isize);
                current_block = 2242099707034464334;
            }
            64 => {
                t1 = POP_ARRAY(0 as libc::c_int != 0);
                let fresh43 = stack_ptr;
                stack_ptr = stack_ptr.offset(-1);
                lhs = (*fresh43).lptr;
                do_delete_loop(t1, lhs);
                current_block = 2242099707034464334;
            }
            72 => {
                t1 = POP_ARRAY(0 as libc::c_int != 0);
                t2 = if (*pc).x.xl == 1 as libc::c_int as libc::c_long {
                    POP_SCALAR()
                } else {
                    concat_exp((*pc).x.xl as libc::c_int, 1 as libc::c_int != 0)
                };
                r = node_Boolean[(in_array(t1, t2)
                    != 0 as *mut libc::c_void as *mut NODE) as libc::c_int as usize];
                DEREF(t2);
                (*r).valref += 1;
                (*r).valref;
                let ref mut fresh44 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh44 = r;
                current_block = 2242099707034464334;
            }
            92 => {
                let mut list: *mut *mut NODE = 0 as *mut *mut NODE;
                let mut array: *mut NODE = 0 as *mut NODE;
                let mut sort_str: *mut NODE = 0 as *mut NODE;
                let mut num_elems: size_t = 0 as libc::c_int as size_t;
                static mut sorted_in: *mut NODE = 0 as *const NODE as *mut NODE;
                let mut how_to_sort: *const libc::c_char = b"@unsorted\0" as *const u8
                    as *const libc::c_char;
                let mut save_0: libc::c_char = 0;
                let mut saved_end: bool = 0 as libc::c_int != 0;
                array = POP_ARRAY(1 as libc::c_int != 0);
                num_elems = (*array).sub.nodep.reflags as size_t;
                if !(num_elems == 0 as libc::c_int as libc::c_ulong) {
                    if sorted_in.is_null() {
                        sorted_in = make_str_node(
                            b"sorted_in\0" as *const u8 as *const libc::c_char,
                            9 as libc::c_int as size_t,
                            0 as libc::c_int,
                        );
                    }
                    sort_str = 0 as *mut NODE;
                    if do_flags as libc::c_uint & DO_POSIX as libc::c_int as libc::c_uint
                        == 0 && !PROCINFO_node.is_null()
                    {
                        sort_str = in_array(PROCINFO_node, sorted_in);
                    }
                    if !sort_str.is_null() {
                        sort_str = force_string_fmt(sort_str, CONVFMT, CONVFMTidx);
                        if (*sort_str).sub.val.slen > 0 as libc::c_int as libc::c_ulong {
                            how_to_sort = (*sort_str).sub.val.sp;
                            str_terminate_f(sort_str, &mut save_0);
                            saved_end = 1 as libc::c_int != 0;
                        }
                    }
                    list = assoc_list(array, how_to_sort, SORTED_IN);
                    if saved_end {
                        *((*sort_str).sub.val.sp)
                            .offset((*sort_str).sub.val.slen as isize) = save_0;
                    }
                }
                r = nextfree[BLOCK_NODE as libc::c_int as usize].freep as *mut NODE;
                if !r.is_null() {
                    nextfree[BLOCK_NODE as libc::c_int as usize]
                        .freep = (*(r as *mut block_item)).freep;
                } else {
                    r = more_blocks(BLOCK_NODE as libc::c_int) as *mut NODE;
                };
                (*r).type_0 = Node_arrayfor;
                (*r).sub.nodep.r.av = list;
                (*r).sub.nodep.reflags = num_elems as reflagvals;
                (*r).sub.nodep.l.ll = -(1 as libc::c_int) as libc::c_long;
                (*r).sub.nodep.rn = array;
                let ref mut fresh45 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh45 = r;
                if num_elems == 0 as libc::c_int as libc::c_ulong {
                    pc = (*pc).d.di;
                    continue;
                } else {
                    current_block = 2242099707034464334;
                }
            }
            93 => {
                r = (*stack_ptr).rptr;
                (*r).sub.nodep.l.ll += 1;
                if (*r).sub.nodep.l.ll == (*r).sub.nodep.reflags as libc::c_long {
                    let mut array_0: *mut NODE = 0 as *mut NODE;
                    array_0 = (*r).sub.nodep.rn;
                    if do_flags as libc::c_uint
                        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                            as libc::c_uint != 0
                        && (*array_0).sub.nodep.reflags as libc::c_uint
                            != (*r).sub.nodep.reflags as libc::c_uint
                    {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                            1070 as libc::c_int,
                        );
                        (Some(lintfunc.unwrap()))
                            .unwrap()(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"for loop: array `%s' changed size from %ld to %ld during loop execution\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            array_vname(array_0),
                            (*r).sub.nodep.reflags as libc::c_long,
                            (*array_0).sub.nodep.reflags as libc::c_long,
                        );
                    }
                    pc = (*pc).d.di;
                    continue;
                } else {
                    t1 = *((*r).sub.nodep.r.av).offset((*r).sub.nodep.l.ll as isize);
                    lhs = if (*(*pc).x.xn).type_0 as libc::c_uint
                        == Node_var as libc::c_int as libc::c_uint
                        && !((*(*pc).x.xn).sub.nodep.l.lptr == Nnull_string)
                    {
                        &mut (*(*pc).x.xn).sub.nodep.l.lptr
                    } else {
                        r_get_lhs((*pc).x.xn, 0 as libc::c_int != 0)
                    };
                    unref(*lhs);
                    *lhs = dupnode(t1);
                }
                current_block = 2242099707034464334;
            }
            94 => {
                let fresh46 = stack_ptr;
                stack_ptr = stack_ptr.offset(-1);
                r = (*fresh46).rptr;
                free_arrayfor(r);
                current_block = 2242099707034464334;
            }
            69 => {
                r = ((*pc).d.fptr).unwrap()((*pc).x.xl as libc::c_int);
                let ref mut fresh47 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh47 = r;
                current_block = 2242099707034464334;
            }
            71 => {
                let mut arg_count: size_t = (*pc).x.xl as size_t;
                let mut f: *mut awk_ext_func_t = (*pc.offset(1 as libc::c_int as isize))
                    .x
                    .exf;
                let mut min_req: size_t = (*f).min_required_args;
                let mut max_expect: size_t = (*f).max_expected_args;
                let mut result: awk_value_t = awk_value_t {
                    val_type: AWK_UNDEFINED,
                    u: C2RustUnnamed_0 {
                        s: awk_string_t {
                            str_0: 0 as *mut libc::c_char,
                            len: 0,
                        },
                    },
                };
                if arg_count < min_req {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const libc::c_char,
                        1101 as libc::c_int,
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
                            b"%s: called with %lu arguments, expecting at least %lu\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*pc.offset(1 as libc::c_int as isize)).d.name,
                        arg_count,
                        min_req,
                    );
                }
                if do_flags as libc::c_uint
                    & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                        as libc::c_uint != 0 && (*f).suppress_lint as u64 == 0
                    && arg_count > max_expect
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const libc::c_char,
                        1107 as libc::c_int,
                    );
                    (Some(lintfunc.unwrap()))
                        .unwrap()(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: called with %lu arguments, expecting no more than %lu\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*pc.offset(1 as libc::c_int as isize)).d.name,
                        arg_count,
                        max_expect,
                    );
                }
                PUSH_CODE(pc);
                let mut ef_ret: *mut awk_value_t = ((*pc).d.efptr)
                    .unwrap()(arg_count as libc::c_int, &mut result, f);
                r = awk_value_to_node(ef_ret);
                POP_CODE();
                loop {
                    let fresh48 = arg_count;
                    arg_count = arg_count.wrapping_sub(1);
                    if !(fresh48 > 0 as libc::c_int as libc::c_ulong) {
                        break;
                    }
                    let fresh49 = stack_ptr;
                    stack_ptr = stack_ptr.offset(-1);
                    t1 = (*fresh49).rptr;
                    if (*t1).type_0 as libc::c_uint
                        == Node_val as libc::c_int as libc::c_uint
                        || (*t1).type_0 as libc::c_uint
                            == Node_elem_new as libc::c_int as libc::c_uint
                    {
                        DEREF(t1);
                    }
                }
                free_api_string_copies();
                if in_indirect_call {
                    let fresh50 = stack_ptr;
                    stack_ptr = stack_ptr.offset(-1);
                    let mut fname: *mut NODE = (*fresh50).rptr;
                    DEREF(fname);
                    in_indirect_call = 0 as libc::c_int != 0;
                }
                let ref mut fresh51 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh51 = r;
                current_block = 2242099707034464334;
            }
            70 => {
                r = do_sub((*pc).x.xl as libc::c_int, (*pc).d.dl as libc::c_uint);
                let ref mut fresh52 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh52 = r;
                current_block = 2242099707034464334;
            }
            56 => {
                do_print((*pc).x.xl as libc::c_int, (*pc).d.dl as libc::c_int);
                current_block = 2242099707034464334;
            }
            58 => {
                do_printf((*pc).x.xl as libc::c_int, (*pc).d.dl as libc::c_int);
                current_block = 2242099707034464334;
            }
            57 => {
                do_print_rec((*pc).x.xl as libc::c_int, (*pc).d.dl as libc::c_int);
                current_block = 2242099707034464334;
            }
            79 => {
                m = (*pc).d.dn;
                if (*m).type_0 as libc::c_uint
                    == Node_dynregex as libc::c_int as libc::c_uint
                {
                    r = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
                    unref((*m).sub.nodep.x.extra);
                    (*m).sub.nodep.x.extra = r;
                } else if (*m).type_0 as libc::c_uint
                    == Node_val as libc::c_int as libc::c_uint
                {
                    (*m).valref += 1;
                    (*m).valref;
                }
                let ref mut fresh53 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh53 = m;
                current_block = 2242099707034464334;
            }
            49 => {
                m = (*pc).d.dn;
                t1 = *get_field(0 as libc::c_int as libc::c_long, 0 as *mut Func_ptr);
                current_block = 3452182716914932555;
            }
            50 | 48 => {
                m = (*pc).d.dn;
                t1 = force_string_fmt(TOP_SCALAR(), CONVFMT, CONVFMTidx);
                if (*m).type_0 as libc::c_uint
                    == Node_dynregex as libc::c_int as libc::c_uint
                {
                    unref((*m).sub.nodep.x.extra);
                    (*m).sub.nodep.x.extra = t1;
                    stack_ptr = stack_ptr.offset(-1);
                    stack_ptr;
                    t1 = force_string_fmt(TOP_SCALAR(), CONVFMT, CONVFMTidx);
                }
                current_block = 3452182716914932555;
            }
            74 => {
                let mut f_0: *mut NODE = 0 as *mut NODE;
                let mut arg_count_0: libc::c_int = 0;
                let mut save_1: libc::c_char = 0;
                let mut function_name: *mut NODE = 0 as *mut NODE;
                arg_count_0 = (*pc.offset(1 as libc::c_int as isize)).x.xl
                    as libc::c_int;
                t1 = (*stack_ptr.offset(-(arg_count_0 as isize))).rptr;
                if (*t1).type_0 as libc::c_uint
                    != Node_val as libc::c_int as libc::c_uint
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const libc::c_char,
                        1209 as libc::c_int,
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
                            b"indirect function call requires a simple scalar value\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                t1 = force_string_fmt(t1, CONVFMT, CONVFMTidx);
                str_terminate_f(t1, &mut save_1);
                if (*t1).sub.val.slen > 0 as libc::c_int as libc::c_ulong {
                    f_0 = (*pc).x.xn;
                    if !f_0.is_null()
                        && strcmp((*f_0).sub.nodep.name, (*t1).sub.val.sp)
                            == 0 as libc::c_int
                    {
                        *((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize) = save_1;
                        ni = setup_frame(pc);
                        pc = ni;
                        continue;
                    } else {
                        f_0 = lookup((*t1).sub.val.sp);
                    }
                }
                if f_0.is_null() {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const libc::c_char,
                        1227 as libc::c_int,
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
                            b"`%s' is not a function, so it cannot be called indirectly\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*t1).sub.val.sp,
                    );
                    current_block = 4159430576127784976;
                } else if (*f_0).type_0 as libc::c_uint
                    == Node_builtin_func as libc::c_int as libc::c_uint
                {
                    let mut arg_count_1: libc::c_int = (*pc
                        .offset(1 as libc::c_int as isize))
                        .x
                        .xl as libc::c_int;
                    let mut the_func: builtin_func_t = lookup_builtin((*t1).sub.val.sp);
                    if the_func
                        == ::std::mem::transmute::<
                            Option::<
                                unsafe extern "C" fn(libc::c_int, libc::c_uint) -> *mut NODE,
                            >,
                            builtin_func_t,
                        >(
                            Some(
                                do_sub
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        libc::c_uint,
                                    ) -> *mut NODE,
                            ),
                        )
                    {
                        r = call_sub((*t1).sub.val.sp, arg_count_1);
                    } else if the_func
                        == Some(
                            do_match as unsafe extern "C" fn(libc::c_int) -> *mut NODE,
                        )
                    {
                        r = call_match(arg_count_1);
                    } else if the_func
                        == Some(
                            do_split as unsafe extern "C" fn(libc::c_int) -> *mut NODE,
                        )
                        || the_func
                            == Some(
                                do_patsplit
                                    as unsafe extern "C" fn(libc::c_int) -> *mut NODE,
                            )
                    {
                        r = call_split_func((*t1).sub.val.sp, arg_count_1);
                    } else {
                        r = the_func.unwrap()(arg_count_1);
                    }
                    *((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize) = save_1;
                    let fresh55 = stack_ptr;
                    stack_ptr = stack_ptr.offset(-1);
                    function_name = (*fresh55).rptr;
                    DEREF(function_name);
                    let ref mut fresh56 = (*if stack_ptr < stack_top {
                        stack_ptr = stack_ptr.offset(1);
                        stack_ptr
                    } else {
                        grow_stack()
                    })
                        .rptr;
                    *fresh56 = r;
                    current_block = 2242099707034464334;
                } else if (*f_0).type_0 as libc::c_uint
                    != Node_func as libc::c_int as libc::c_uint
                {
                    *((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize) = save_1;
                    if (*f_0).type_0 as libc::c_uint
                        == Node_ext_func as libc::c_int as libc::c_uint
                    {
                        let mut bc: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                        let mut fname_0: *mut libc::c_char = (*pc).d.name;
                        let mut arg_count_2: libc::c_int = (*pc
                            .offset(1 as libc::c_int as isize))
                            .x
                            .xl as libc::c_int;
                        static mut npc: [INSTRUCTION; 2] = [INSTRUCTION {
                            nexti: 0 as *const exp_instruction as *mut exp_instruction,
                            d: C2RustUnnamed_7 {
                                dn: 0 as *const NODE as *mut NODE,
                            },
                            x: C2RustUnnamed_6 { xl: 0 },
                            comment: 0 as *const exp_instruction as *mut exp_instruction,
                            source_line: 0,
                            pool_size: 0,
                            opcode: Op_illegal,
                        }; 2];
                        npc[0 as libc::c_int as usize] = *pc;
                        bc = (*f_0).sub.nodep.r.iptr;
                        npc[0 as libc::c_int as usize].opcode = Op_ext_builtin;
                        npc[0 as libc::c_int as usize].d.efptr = (*bc).d.efptr;
                        npc[0 as libc::c_int as usize]
                            .x
                            .xl = arg_count_2 as libc::c_long;
                        npc[1 as libc::c_int
                            as usize] = *pc.offset(1 as libc::c_int as isize);
                        npc[1 as libc::c_int as usize].d.name = fname_0;
                        npc[1 as libc::c_int as usize].x.exf = (*bc).x.exf;
                        in_indirect_call = 1 as libc::c_int != 0;
                        ni = npc.as_mut_ptr();
                        pc = ni;
                        continue;
                    } else {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                            1277 as libc::c_int,
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
                                b"function called indirectly through `%s' does not exist\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            (*pc).d.name,
                        );
                    }
                    current_block = 4159430576127784976;
                } else {
                    current_block = 4159430576127784976;
                }
                match current_block {
                    2242099707034464334 => {}
                    _ => {
                        (*pc).x.xn = f_0;
                        *((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize) = save_1;
                        ni = setup_frame(pc);
                        pc = ni;
                        continue;
                    }
                }
            }
            73 => {
                let mut f_1: *mut NODE = 0 as *mut NODE;
                f_1 = (*pc).x.xn;
                if f_1.is_null() {
                    f_1 = lookup((*pc).d.name);
                    if f_1.is_null()
                        || (*f_1).type_0 as libc::c_uint
                            != Node_func as libc::c_int as libc::c_uint
                            && (*f_1).type_0 as libc::c_uint
                                != Node_ext_func as libc::c_int as libc::c_uint
                    {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                            1296 as libc::c_int,
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
                                b"function `%s' not defined\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            (*pc).d.name,
                        );
                    }
                    (*pc).x.xn = f_1;
                }
                if (*f_1).type_0 as libc::c_uint
                    == Node_ext_func as libc::c_int as libc::c_uint
                {
                    let mut bc_0: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                    let mut fname_1: *mut libc::c_char = (*pc).d.name;
                    let mut arg_count_3: libc::c_int = (*pc
                        .offset(1 as libc::c_int as isize))
                        .x
                        .xl as libc::c_int;
                    bc_0 = (*f_1).sub.nodep.r.iptr;
                    (*pc).opcode = Op_ext_builtin;
                    (*pc).d.efptr = (*bc_0).d.efptr;
                    (*pc).x.xl = arg_count_3 as libc::c_long;
                    let ref mut fresh57 = (*pc.offset(1 as libc::c_int as isize)).d.name;
                    *fresh57 = fname_1;
                    let ref mut fresh58 = (*pc.offset(1 as libc::c_int as isize)).x.exf;
                    *fresh58 = (*bc_0).x.exf;
                    ni = pc;
                    pc = ni;
                    continue;
                } else {
                    ni = setup_frame(pc);
                    pc = ni;
                    continue;
                }
            }
            62 => {
                r_fatal(
                    b"internal error: file %s, line %d: unexpected opcode %s\0"
                        as *const u8 as *const libc::c_char,
                    b"./interpret.h\0" as *const u8 as *const libc::c_char,
                    1322 as libc::c_int,
                    opcode2str(op),
                );
                current_block = 2242099707034464334;
            }
            61 => {
                m = POP_SCALAR();
                ni = unwind_stack((*frame_ptr).sub.nodep.reflags as libc::c_long);
                let ref mut fresh59 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh59 = m;
                pc = ni;
                continue;
            }
            65 => {
                r = do_getline_redir((*pc).x.xl as libc::c_int, (*pc).d.dl as redirval);
                let ref mut fresh60 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh60 = r;
                current_block = 2242099707034464334;
            }
            66 => {
                if currule == 0 || currule == BEGINFILE as libc::c_int
                    || currule == ENDFILE as libc::c_int
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const libc::c_char,
                        1342 as libc::c_int,
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
                            b"non-redirected `getline' invalid inside `%s' rule\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        *ruletab.as_ptr().offset(currule as isize),
                    );
                }
                loop {
                    let mut ret: libc::c_int = 0;
                    ret = nextfile(&mut curfile, 0 as libc::c_int != 0);
                    if ret <= 0 as libc::c_int {
                        r = do_getline((*pc).x.xl as libc::c_int, curfile);
                        if !r.is_null() {
                            break;
                        }
                    } else {
                        push_exec_state(pc, currule, source, stack_ptr);
                        if curfile.is_null() {
                            pc = (*pc.offset(1 as libc::c_int as isize)).x.xi;
                            continue '_top;
                        } else {
                            pc = (*pc.offset(1 as libc::c_int as isize)).d.di;
                            continue '_top;
                        }
                    }
                }
                let ref mut fresh61 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh61 = r;
                current_block = 2242099707034464334;
            }
            100 => {
                ni = pop_exec_state(&mut currule, &mut source, 0 as *mut libc::c_long);
                pc = ni;
                continue;
            }
            99 => {
                after_beginfile(&mut curfile);
                ni = pop_exec_state(&mut currule, &mut source, 0 as *mut libc::c_long);
                if (*ni).opcode as libc::c_uint
                    == Op_K_getline as libc::c_int as libc::c_uint || curfile.is_null()
                {
                    pc = ni;
                    continue;
                } else {
                    current_block = 2242099707034464334;
                }
            }
            91 => {
                let mut ret_0: libc::c_int = 0;
                ret_0 = nextfile(&mut curfile, 0 as libc::c_int != 0);
                if ret_0 < 0 as libc::c_int {
                    pc = (*pc).d.di;
                    continue;
                } else if ret_0 == 0 as libc::c_int {
                    pc = (*pc.offset(1 as libc::c_int as isize)).x.xi;
                    continue;
                } else {
                    push_exec_state(pc, currule, source, stack_ptr);
                    if curfile.is_null() {
                        pc = (*pc).x.xi;
                        continue;
                    }
                }
                current_block = 2242099707034464334;
            }
            90 => {
                let mut errcode: libc::c_int = 0 as libc::c_int;
                ni = (*pc).d.di;
                if curfile.is_null() {
                    ni = (*ni).d.di;
                    pc = ni;
                    continue;
                } else if !inrec(curfile, &mut errcode) {
                    if errcode > 0 as libc::c_int {
                        update_ERRNO_int(errcode);
                        if do_flags as libc::c_uint
                            & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0
                            || (*pc).x.xl == 0
                        {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    libc::c_int,
                                ) -> ())(
                                b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                1433 as libc::c_int,
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
                                    b"error reading input file `%s': %s\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                (*curfile).public.name,
                                strerror(errcode),
                            );
                        }
                    }
                    pc = ni;
                    continue;
                }
                current_block = 2242099707034464334;
            }
            67 => {
                let mut ret_1: libc::c_int = 0;
                if currule != Rule as libc::c_int && currule != BEGINFILE as libc::c_int
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const libc::c_char,
                        1448 as libc::c_int,
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
                            b"`nextfile' cannot be called from a `%s' rule\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        *ruletab.as_ptr().offset(currule as isize),
                    );
                }
                ret_1 = nextfile(&mut curfile, 1 as libc::c_int != 0);
                if currule == BEGINFILE as libc::c_int {
                    let mut stack_size: libc::c_long = 0 as libc::c_int as libc::c_long;
                    ni = pop_exec_state(&mut currule, &mut source, &mut stack_size);
                    unwind_stack(stack_size);
                    if ret_1 == 0 as libc::c_int {
                        pc = ni;
                        continue;
                    } else {
                        push_exec_state(ni, currule, source, stack_ptr);
                        pc = (*pc).x.xi;
                        continue;
                    }
                } else {
                    unwind_stack(0 as libc::c_int as libc::c_long);
                    push_exec_state((*pc).d.di, currule, source, stack_ptr);
                    pc = (*pc).x.xi;
                    continue;
                }
            }
            60 => {
                if currule == 0 {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const libc::c_char,
                        1495 as libc::c_int,
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
                            b"`exit' cannot be called in the current context\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                exiting = 1 as libc::c_int != 0;
                t1 = force_number(POP_SCALAR());
                if t1 != Nnull_string {
                    exit_val = (*t1).sub.val.fltnum as libc::c_long as libc::c_int;
                }
                DEREF(t1);
                if currule == BEGINFILE as libc::c_int
                    || currule == ENDFILE as libc::c_int
                {
                    pop_exec_state(&mut currule, &mut source, 0 as *mut libc::c_long);
                }
                unwind_stack(0 as libc::c_int as libc::c_long);
                if currule == END as libc::c_int {
                    ni = (*pc).x.xi;
                } else {
                    ni = (*pc).d.di;
                }
                pc = ni;
                continue;
            }
            59 => {
                if currule != Rule as libc::c_int {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const libc::c_char,
                        1536 as libc::c_int,
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
                            b"`next' cannot be called from a `%s' rule\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        *ruletab.as_ptr().offset(currule as isize),
                    );
                }
                unwind_stack(0 as libc::c_int as libc::c_long);
                pc = (*pc).d.di;
                continue;
            }
            86 => {
                r = POP_SCALAR();
                DEREF(r);
                current_block = 2242099707034464334;
            }
            14 => {
                if (*pc).x.xl != 0 {
                    pc = (*pc).d.di;
                    continue;
                } else {
                    current_block = 2242099707034464334;
                }
            }
            15 => {
                let mut result_0: libc::c_int = 0;
                let mut ip: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                t1 = TOP_SCALAR();
                di = (eval_condition(t1) as libc::c_int != 0 as libc::c_int)
                    as libc::c_int;
                DEREF(t1);
                ip = (*pc).x.xi;
                if (*ip).x.xl == 0 && di != 0 {
                    stack_ptr = stack_ptr.offset(-1);
                    stack_ptr;
                    (*ip).x.xl = 1 as libc::c_int as libc::c_long;
                    pc = (*ip).d.di;
                    continue;
                } else {
                    result_0 = ((*ip).x.xl != 0 || di != 0) as libc::c_int;
                    (*ip).x.xl ^= di as libc::c_long;
                    r = node_Boolean[result_0 as usize];
                    (*r).valref += 1;
                    (*r).valref;
                    (*stack_ptr).rptr = r;
                    pc = (*pc).d.di;
                    continue;
                }
            }
            103 => {
                if do_flags as libc::c_uint & DO_PROFILE as libc::c_int as libc::c_uint
                    != 0
                {
                    (*pc).d.ldl = ((*pc).d.ldl).wrapping_add(1);
                    (*pc).d.ldl;
                }
                current_block = 2242099707034464334;
            }
            85 | 112 | 115 | 113 | 114 | 116 | 53 | 117 | 118 | 120 | 102 | 121 => {
                current_block = 2242099707034464334;
            }
            _ => {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"./interpret.h\0" as *const u8 as *const libc::c_char,
                    1599 as libc::c_int,
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
                        b"Sorry, don't know how to interpret `%s'\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    opcode2str(op),
                );
                current_block = 2242099707034464334;
            }
        }
        match current_block {
            3452182716914932555 => {
                rp = re_update(m);
                di = research(
                    rp,
                    (*t1).sub.val.sp,
                    0 as libc::c_int,
                    (*t1).sub.val.slen,
                    0 as libc::c_int,
                );
                di = (di == -(1 as libc::c_int)) as libc::c_int
                    ^ (op as libc::c_uint != Op_nomatch as libc::c_int as libc::c_uint)
                        as libc::c_int;
                if op as libc::c_uint != Op_match_rec as libc::c_int as libc::c_uint {
                    stack_ptr = stack_ptr.offset(-1);
                    stack_ptr;
                    DEREF(t1);
                    if (*m).type_0 as libc::c_uint
                        == Node_dynregex as libc::c_int as libc::c_uint
                    {
                        DEREF((*m).sub.nodep.x.extra);
                        (*m).sub.nodep.x.extra = 0 as *mut exp_node;
                    }
                }
                r = node_Boolean[di as usize];
                (*r).valref += 1;
                (*r).valref;
                let ref mut fresh54 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh54 = r;
            }
            13208848489772510154 => {
                t1 = force_number(TOP_SCALAR());
                if x2 == 0 as libc::c_int as libc::c_double {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const libc::c_char,
                        664 as libc::c_int,
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
                            b"division by zero attempted in `%%'\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                x = fmod((*t1).sub.val.fltnum, x2);
                r = make_number.unwrap()(x);
                DEREF(t1);
                (*stack_ptr).rptr = r;
            }
            2910841344195686645 => {
                t1 = force_number(TOP_SCALAR());
                if x2 == 0 as libc::c_int as libc::c_double {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const libc::c_char,
                        648 as libc::c_int,
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
                            b"division by zero attempted\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                r = make_number.unwrap()((*t1).sub.val.fltnum / x2);
                DEREF(t1);
                (*stack_ptr).rptr = r;
            }
            459172418090906441 => {
                t1 = force_number(TOP_SCALAR());
                r = make_number.unwrap()(calc_exp((*t1).sub.val.fltnum, x2));
                DEREF(t1);
                (*stack_ptr).rptr = r;
            }
            12385243566997464291 => {
                t1 = force_number(TOP_SCALAR());
                r = make_number.unwrap()((*t1).sub.val.fltnum * x2);
                DEREF(t1);
                (*stack_ptr).rptr = r;
            }
            2118905164450504584 => {
                t1 = force_number(TOP_SCALAR());
                r = make_number.unwrap()((*t1).sub.val.fltnum - x2);
                (*r)
                    .sub
                    .val
                    .fltnum = fix_nan_sign(
                    (*t1).sub.val.fltnum,
                    x2,
                    (*r).sub.val.fltnum,
                );
                DEREF(t1);
                (*stack_ptr).rptr = r;
            }
            8953299237562637943 => {
                t1 = force_number(TOP_SCALAR());
                r = make_number.unwrap()((*t1).sub.val.fltnum + x2);
                (*r)
                    .sub
                    .val
                    .fltnum = fix_nan_sign(
                    (*t1).sub.val.fltnum,
                    x2,
                    (*r).sub.val.fltnum,
                );
                DEREF(t1);
                (*stack_ptr).rptr = r;
            }
            15848523117724405161 => {
                source = (*pc).d.name;
            }
            11276394722809792258 => {
                let ref mut fresh31 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh31 = (*pc).d.dn;
            }
            _ => {}
        }
        pc = (*pc).nexti;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn h_interpret(mut code: *mut INSTRUCTION) -> libc::c_int {
    let mut current_block: u64;
    let mut pc: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut op: OPCODE = Op_illegal;
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut m: *mut NODE = 0 as *mut NODE;
    let mut ni: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut t1: *mut NODE = 0 as *mut NODE;
    let mut t2: *mut NODE = 0 as *mut NODE;
    let mut lhs: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut x: libc::c_double = 0.;
    let mut x2: libc::c_double = 0.;
    let mut di: libc::c_int = 0;
    let mut rp: *mut Regexp = 0 as *mut Regexp;
    let mut set_array: *mut NODE = 0 as *mut NODE;
    let mut set_idx: *mut NODE = 0 as *mut NODE;
    let mut in_indirect_call: bool = 0 as libc::c_int != 0;
    pc = code;
    '_top: loop {
        if (*pc).source_line as libc::c_int > 0 as libc::c_int {
            sourceline = (*pc).source_line as libc::c_int;
        }
        di = 0 as libc::c_int;
        while di < num_exec_hook {
            if (pre_execute[di as usize]).unwrap()(&mut pc) == 0 {
                continue '_top;
            }
            di += 1;
            di;
        }
        op = (*pc).opcode;
        if do_itrace {
            fprintf(
                stderr,
                b"+ %s\n\0" as *const u8 as *const libc::c_char,
                opcode2str(op),
            );
            fflush(stderr);
        }
        match op as libc::c_uint {
            51 => {
                currule = (*pc).x.xl as libc::c_int;
                if currule == BEGINFILE as libc::c_int {
                    set_record(
                        b"\0" as *const u8 as *const libc::c_char,
                        0 as libc::c_int as size_t,
                        0 as *const awk_fieldwidth_info_t,
                    );
                }
                current_block = 7599496272070177918;
            }
            101 => {
                current_block = 7599496272070177918;
            }
            107 => {
                let mut stdio_problem: bool = 0 as libc::c_int != 0;
                let mut got_EPIPE: bool = 0 as libc::c_int != 0;
                source = 0 as *mut libc::c_char;
                sourceline = 0 as libc::c_int;
                nextfile(&mut curfile, 1 as libc::c_int != 0);
                close_io(&mut stdio_problem, &mut got_EPIPE);
                if stdio_problem as libc::c_int != 0 && !exiting
                    && exit_val == 0 as libc::c_int
                {
                    exit_val = 1 as libc::c_int;
                }
                close_extensions();
                if got_EPIPE {
                    signal(13 as libc::c_int, None);
                    kill(getpid(), 13 as libc::c_int);
                }
                current_block = 3518619798157913413;
            }
            108 => {
                break;
            }
            78 => {
                m = (*pc).d.dn;
                if do_flags as libc::c_uint
                    & DO_TRADITIONAL as libc::c_int as libc::c_uint == 0
                    && (*m).flags as libc::c_uint
                        & INTLSTR as libc::c_int as libc::c_uint
                        != 0 as libc::c_int as libc::c_uint
                {
                    let mut orig: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut trans: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut save: libc::c_char = 0;
                    save = *((*m).sub.val.sp).offset((*m).sub.val.slen as isize);
                    *((*m).sub.val.sp)
                        .offset(
                            (*m).sub.val.slen as isize,
                        ) = '\0' as i32 as libc::c_char;
                    orig = (*m).sub.val.sp;
                    trans = dcgettext(TEXTDOMAIN, orig, 5 as libc::c_int);
                    *((*m).sub.val.sp).offset((*m).sub.val.slen as isize) = save;
                    if trans != orig {
                        m = make_str_node(trans, strlen(trans), 0 as libc::c_int);
                    } else {
                        (*m).valref += 1;
                        (*m).valref;
                    }
                } else {
                    (*m).valref += 1;
                    (*m).valref;
                }
                let ref mut fresh62 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh62 = m;
                current_block = 3518619798157913413;
            }
            75 | 76 | 77 => {
                let mut current_block_67: u64;
                let mut save_symbol: *mut NODE = 0 as *mut NODE;
                let mut isparam: bool = 0 as libc::c_int != 0;
                m = (*pc).d.dn;
                save_symbol = m;
                if (*m).type_0 as libc::c_uint
                    == Node_param_list as libc::c_int as libc::c_uint
                {
                    isparam = 1 as libc::c_int != 0;
                    m = *((*frame_ptr).sub.nodep.r.av)
                        .offset((*m).sub.nodep.l.ll as isize);
                    save_symbol = m;
                    if (*m).type_0 as libc::c_uint
                        == Node_array_ref as libc::c_int as libc::c_uint
                    {
                        if (*(*m).sub.nodep.l.lptr).type_0 as libc::c_uint
                            == Node_var as libc::c_int as libc::c_uint
                        {
                            current_block_67 = 6706988388290608591;
                        } else {
                            m = (*m).sub.nodep.l.lptr;
                            current_block_67 = 3160140712158701372;
                        }
                    } else {
                        current_block_67 = 3160140712158701372;
                    }
                } else {
                    current_block_67 = 3160140712158701372;
                }
                match current_block_67 {
                    3160140712158701372 => {
                        match (*m).type_0 as libc::c_uint {
                            4 => {
                                current_block_67 = 253307868955441534;
                                match current_block_67 {
                                    7962750053813663207 => {
                                        r_fatal(
                                            b"internal error: file %s, line %d: unexpected parameter type %s\0"
                                                as *const u8 as *const libc::c_char,
                                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                            263 as libc::c_int,
                                            nodetype2str((*m).type_0),
                                        );
                                    }
                                    253307868955441534 => {
                                        if do_flags as libc::c_uint
                                            & (DO_LINT_INVALID as libc::c_int
                                                | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                                            && (*m).sub.nodep.l.lptr == Nnull_string
                                        {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const libc::c_char,
                                                    libc::c_int,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                                204 as libc::c_int,
                                            );
                                            (Some(lintfunc.unwrap()))
                                                .unwrap()(
                                                if isparam as libc::c_int != 0 {
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"reference to uninitialized argument `%s'\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    )
                                                } else {
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"reference to uninitialized variable `%s'\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    )
                                                },
                                                (*save_symbol).sub.nodep.name,
                                            );
                                        }
                                        m = (*m).sub.nodep.l.lptr;
                                        (*m).valref += 1;
                                        (*m).valref;
                                        let ref mut fresh63 = (*if stack_ptr < stack_top {
                                            stack_ptr = stack_ptr.offset(1);
                                            stack_ptr
                                        } else {
                                            grow_stack()
                                        })
                                            .rptr;
                                        *fresh63 = m;
                                    }
                                    14655262897040144319 => {
                                        if op as libc::c_uint
                                            == Op_push_arg as libc::c_int as libc::c_uint
                                            || op as libc::c_uint
                                                == Op_push_arg_untyped as libc::c_int as libc::c_uint
                                        {
                                            let ref mut fresh66 = (*if stack_ptr < stack_top {
                                                stack_ptr = stack_ptr.offset(1);
                                                stack_ptr
                                            } else {
                                                grow_stack()
                                            })
                                                .rptr;
                                            *fresh66 = m;
                                        } else {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const libc::c_char,
                                                    libc::c_int,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                                258 as libc::c_int,
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
                                                array_vname(save_symbol),
                                            );
                                        }
                                    }
                                    _ => {
                                        if do_flags as libc::c_uint
                                            & (DO_LINT_INVALID as libc::c_int
                                                | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                                        {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const libc::c_char,
                                                    libc::c_int,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                                236 as libc::c_int,
                                            );
                                            (Some(lintfunc.unwrap()))
                                                .unwrap()(
                                                if isparam as libc::c_int != 0 {
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"reference to uninitialized argument `%s'\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    )
                                                } else {
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"reference to uninitialized variable `%s'\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    )
                                                },
                                                (*save_symbol).sub.nodep.name,
                                            );
                                        }
                                        if op as libc::c_uint
                                            != Op_push_arg_untyped as libc::c_int as libc::c_uint
                                        {
                                            (*m).type_0 = Node_var;
                                            (*m).sub.nodep.l.lptr = dupnode(Nnull_string);
                                            DEREF(m);
                                            m = dupnode(Nnull_string);
                                        }
                                        let ref mut fresh65 = (*if stack_ptr < stack_top {
                                            stack_ptr = stack_ptr.offset(1);
                                            stack_ptr
                                        } else {
                                            grow_stack()
                                        })
                                            .rptr;
                                        *fresh65 = m;
                                    }
                                }
                                current_block_67 = 7178192492338286402;
                            }
                            6 => {
                                current_block_67 = 6706988388290608591;
                            }
                            7 => {
                                current_block_67 = 17607631736602194043;
                                match current_block_67 {
                                    7962750053813663207 => {
                                        r_fatal(
                                            b"internal error: file %s, line %d: unexpected parameter type %s\0"
                                                as *const u8 as *const libc::c_char,
                                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                            263 as libc::c_int,
                                            nodetype2str((*m).type_0),
                                        );
                                    }
                                    253307868955441534 => {
                                        if do_flags as libc::c_uint
                                            & (DO_LINT_INVALID as libc::c_int
                                                | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                                            && (*m).sub.nodep.l.lptr == Nnull_string
                                        {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const libc::c_char,
                                                    libc::c_int,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                                204 as libc::c_int,
                                            );
                                            (Some(lintfunc.unwrap()))
                                                .unwrap()(
                                                if isparam as libc::c_int != 0 {
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"reference to uninitialized argument `%s'\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    )
                                                } else {
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"reference to uninitialized variable `%s'\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    )
                                                },
                                                (*save_symbol).sub.nodep.name,
                                            );
                                        }
                                        m = (*m).sub.nodep.l.lptr;
                                        (*m).valref += 1;
                                        (*m).valref;
                                        let ref mut fresh63 = (*if stack_ptr < stack_top {
                                            stack_ptr = stack_ptr.offset(1);
                                            stack_ptr
                                        } else {
                                            grow_stack()
                                        })
                                            .rptr;
                                        *fresh63 = m;
                                    }
                                    14655262897040144319 => {
                                        if op as libc::c_uint
                                            == Op_push_arg as libc::c_int as libc::c_uint
                                            || op as libc::c_uint
                                                == Op_push_arg_untyped as libc::c_int as libc::c_uint
                                        {
                                            let ref mut fresh66 = (*if stack_ptr < stack_top {
                                                stack_ptr = stack_ptr.offset(1);
                                                stack_ptr
                                            } else {
                                                grow_stack()
                                            })
                                                .rptr;
                                            *fresh66 = m;
                                        } else {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const libc::c_char,
                                                    libc::c_int,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                                258 as libc::c_int,
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
                                                array_vname(save_symbol),
                                            );
                                        }
                                    }
                                    _ => {
                                        if do_flags as libc::c_uint
                                            & (DO_LINT_INVALID as libc::c_int
                                                | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                                        {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const libc::c_char,
                                                    libc::c_int,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                                236 as libc::c_int,
                                            );
                                            (Some(lintfunc.unwrap()))
                                                .unwrap()(
                                                if isparam as libc::c_int != 0 {
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"reference to uninitialized argument `%s'\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    )
                                                } else {
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"reference to uninitialized variable `%s'\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    )
                                                },
                                                (*save_symbol).sub.nodep.name,
                                            );
                                        }
                                        if op as libc::c_uint
                                            != Op_push_arg_untyped as libc::c_int as libc::c_uint
                                        {
                                            (*m).type_0 = Node_var;
                                            (*m).sub.nodep.l.lptr = dupnode(Nnull_string);
                                            DEREF(m);
                                            m = dupnode(Nnull_string);
                                        }
                                        let ref mut fresh65 = (*if stack_ptr < stack_top {
                                            stack_ptr = stack_ptr.offset(1);
                                            stack_ptr
                                        } else {
                                            grow_stack()
                                        })
                                            .rptr;
                                        *fresh65 = m;
                                    }
                                }
                                current_block_67 = 7178192492338286402;
                            }
                            5 => {
                                current_block_67 = 14655262897040144319;
                                match current_block_67 {
                                    7962750053813663207 => {
                                        r_fatal(
                                            b"internal error: file %s, line %d: unexpected parameter type %s\0"
                                                as *const u8 as *const libc::c_char,
                                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                            263 as libc::c_int,
                                            nodetype2str((*m).type_0),
                                        );
                                    }
                                    253307868955441534 => {
                                        if do_flags as libc::c_uint
                                            & (DO_LINT_INVALID as libc::c_int
                                                | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                                            && (*m).sub.nodep.l.lptr == Nnull_string
                                        {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const libc::c_char,
                                                    libc::c_int,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                                204 as libc::c_int,
                                            );
                                            (Some(lintfunc.unwrap()))
                                                .unwrap()(
                                                if isparam as libc::c_int != 0 {
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"reference to uninitialized argument `%s'\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    )
                                                } else {
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"reference to uninitialized variable `%s'\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    )
                                                },
                                                (*save_symbol).sub.nodep.name,
                                            );
                                        }
                                        m = (*m).sub.nodep.l.lptr;
                                        (*m).valref += 1;
                                        (*m).valref;
                                        let ref mut fresh63 = (*if stack_ptr < stack_top {
                                            stack_ptr = stack_ptr.offset(1);
                                            stack_ptr
                                        } else {
                                            grow_stack()
                                        })
                                            .rptr;
                                        *fresh63 = m;
                                    }
                                    14655262897040144319 => {
                                        if op as libc::c_uint
                                            == Op_push_arg as libc::c_int as libc::c_uint
                                            || op as libc::c_uint
                                                == Op_push_arg_untyped as libc::c_int as libc::c_uint
                                        {
                                            let ref mut fresh66 = (*if stack_ptr < stack_top {
                                                stack_ptr = stack_ptr.offset(1);
                                                stack_ptr
                                            } else {
                                                grow_stack()
                                            })
                                                .rptr;
                                            *fresh66 = m;
                                        } else {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const libc::c_char,
                                                    libc::c_int,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                                258 as libc::c_int,
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
                                                array_vname(save_symbol),
                                            );
                                        }
                                    }
                                    _ => {
                                        if do_flags as libc::c_uint
                                            & (DO_LINT_INVALID as libc::c_int
                                                | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                                        {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const libc::c_char,
                                                    libc::c_int,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                                236 as libc::c_int,
                                            );
                                            (Some(lintfunc.unwrap()))
                                                .unwrap()(
                                                if isparam as libc::c_int != 0 {
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"reference to uninitialized argument `%s'\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    )
                                                } else {
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"reference to uninitialized variable `%s'\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    )
                                                },
                                                (*save_symbol).sub.nodep.name,
                                            );
                                        }
                                        if op as libc::c_uint
                                            != Op_push_arg_untyped as libc::c_int as libc::c_uint
                                        {
                                            (*m).type_0 = Node_var;
                                            (*m).sub.nodep.l.lptr = dupnode(Nnull_string);
                                            DEREF(m);
                                            m = dupnode(Nnull_string);
                                        }
                                        let ref mut fresh65 = (*if stack_ptr < stack_top {
                                            stack_ptr = stack_ptr.offset(1);
                                            stack_ptr
                                        } else {
                                            grow_stack()
                                        })
                                            .rptr;
                                        *fresh65 = m;
                                    }
                                }
                                current_block_67 = 7178192492338286402;
                            }
                            _ => {
                                current_block_67 = 7962750053813663207;
                                match current_block_67 {
                                    7962750053813663207 => {
                                        r_fatal(
                                            b"internal error: file %s, line %d: unexpected parameter type %s\0"
                                                as *const u8 as *const libc::c_char,
                                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                            263 as libc::c_int,
                                            nodetype2str((*m).type_0),
                                        );
                                    }
                                    253307868955441534 => {
                                        if do_flags as libc::c_uint
                                            & (DO_LINT_INVALID as libc::c_int
                                                | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                                            && (*m).sub.nodep.l.lptr == Nnull_string
                                        {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const libc::c_char,
                                                    libc::c_int,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                                204 as libc::c_int,
                                            );
                                            (Some(lintfunc.unwrap()))
                                                .unwrap()(
                                                if isparam as libc::c_int != 0 {
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"reference to uninitialized argument `%s'\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    )
                                                } else {
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"reference to uninitialized variable `%s'\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    )
                                                },
                                                (*save_symbol).sub.nodep.name,
                                            );
                                        }
                                        m = (*m).sub.nodep.l.lptr;
                                        (*m).valref += 1;
                                        (*m).valref;
                                        let ref mut fresh63 = (*if stack_ptr < stack_top {
                                            stack_ptr = stack_ptr.offset(1);
                                            stack_ptr
                                        } else {
                                            grow_stack()
                                        })
                                            .rptr;
                                        *fresh63 = m;
                                    }
                                    14655262897040144319 => {
                                        if op as libc::c_uint
                                            == Op_push_arg as libc::c_int as libc::c_uint
                                            || op as libc::c_uint
                                                == Op_push_arg_untyped as libc::c_int as libc::c_uint
                                        {
                                            let ref mut fresh66 = (*if stack_ptr < stack_top {
                                                stack_ptr = stack_ptr.offset(1);
                                                stack_ptr
                                            } else {
                                                grow_stack()
                                            })
                                                .rptr;
                                            *fresh66 = m;
                                        } else {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const libc::c_char,
                                                    libc::c_int,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                                258 as libc::c_int,
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
                                                array_vname(save_symbol),
                                            );
                                        }
                                    }
                                    _ => {
                                        if do_flags as libc::c_uint
                                            & (DO_LINT_INVALID as libc::c_int
                                                | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                                        {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const libc::c_char,
                                                    libc::c_int,
                                                ) -> ())(
                                                b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                                236 as libc::c_int,
                                            );
                                            (Some(lintfunc.unwrap()))
                                                .unwrap()(
                                                if isparam as libc::c_int != 0 {
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"reference to uninitialized argument `%s'\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    )
                                                } else {
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"reference to uninitialized variable `%s'\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    )
                                                },
                                                (*save_symbol).sub.nodep.name,
                                            );
                                        }
                                        if op as libc::c_uint
                                            != Op_push_arg_untyped as libc::c_int as libc::c_uint
                                        {
                                            (*m).type_0 = Node_var;
                                            (*m).sub.nodep.l.lptr = dupnode(Nnull_string);
                                            DEREF(m);
                                            m = dupnode(Nnull_string);
                                        }
                                        let ref mut fresh65 = (*if stack_ptr < stack_top {
                                            stack_ptr = stack_ptr.offset(1);
                                            stack_ptr
                                        } else {
                                            grow_stack()
                                        })
                                            .rptr;
                                        *fresh65 = m;
                                    }
                                }
                                current_block_67 = 7178192492338286402;
                            }
                        }
                    }
                    _ => {}
                }
                match current_block_67 {
                    6706988388290608591 => {
                        if do_flags as libc::c_uint
                            & (DO_LINT_INVALID as libc::c_int
                                | DO_LINT_ALL as libc::c_int) as libc::c_uint != 0
                        {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    libc::c_int,
                                ) -> ())(
                                b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                216 as libc::c_int,
                            );
                            (Some(lintfunc.unwrap()))
                                .unwrap()(
                                if isparam as libc::c_int != 0 {
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"reference to uninitialized argument `%s'\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    )
                                } else {
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"reference to uninitialized variable `%s'\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    )
                                },
                                (*save_symbol).sub.nodep.name,
                            );
                        }
                        if op as libc::c_uint
                            != Op_push_arg_untyped as libc::c_int as libc::c_uint
                        {
                            (*m).type_0 = Node_var;
                            (*m).sub.nodep.l.lptr = dupnode(Nnull_string);
                            m = dupnode(Nnull_string);
                        }
                        (*m).valref += 1;
                        (*m).valref;
                        let ref mut fresh64 = (*if stack_ptr < stack_top {
                            stack_ptr = stack_ptr.offset(1);
                            stack_ptr
                        } else {
                            grow_stack()
                        })
                            .rptr;
                        *fresh64 = m;
                    }
                    _ => {}
                }
                current_block = 3518619798157913413;
            }
            81 => {
                m = (*pc).d.dn;
                if (*m).type_0 as libc::c_uint
                    == Node_param_list as libc::c_int as libc::c_uint
                {
                    m = *((*frame_ptr).sub.nodep.r.av)
                        .offset((*m).sub.nodep.l.ll as isize);
                }
                if (*m).type_0 as libc::c_uint == Node_var as libc::c_int as libc::c_uint
                {
                    m = (*m).sub.nodep.l.lptr;
                    (*m).valref += 1;
                    (*m).valref;
                    let ref mut fresh67 = (*if stack_ptr < stack_top {
                        stack_ptr = stack_ptr.offset(1);
                        stack_ptr
                    } else {
                        grow_stack()
                    })
                        .rptr;
                    *fresh67 = m;
                    current_block = 3518619798157913413;
                } else {
                    current_block = 11765585416638830173;
                }
            }
            80 => {
                current_block = 11765585416638830173;
            }
            82 => {
                lhs = if (*(*pc).d.dn).type_0 as libc::c_uint
                    == Node_var as libc::c_int as libc::c_uint
                    && !((*(*pc).d.dn).sub.nodep.l.lptr == Nnull_string)
                {
                    &mut (*(*pc).d.dn).sub.nodep.l.lptr
                } else {
                    r_get_lhs((*pc).d.dn, (*pc).x.xl != 0)
                };
                let ref mut fresh69 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .lptr;
                *fresh69 = lhs;
                current_block = 3518619798157913413;
            }
            16 => {
                t2 = if (*pc).d.dl == 1 as libc::c_int as libc::c_long {
                    POP_SCALAR()
                } else {
                    concat_exp((*pc).d.dl as libc::c_int, 1 as libc::c_int != 0)
                };
                t1 = POP_ARRAY(0 as libc::c_int != 0);
                if (in_array(t1, t2)).is_null() {
                    t2 = force_string_fmt(t2, CONVFMT, CONVFMTidx);
                    if t1 == func_table {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                            296 as libc::c_int,
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
                                b"reference to uninitialized element `%s[\"%.*s\"] is not allowed'\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            b"FUNCTAB\0" as *const u8 as *const libc::c_char,
                            (*t2).sub.val.slen as libc::c_int,
                            (*t2).sub.val.sp,
                        );
                    } else if t1 == symbol_table {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                            299 as libc::c_int,
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
                                b"reference to uninitialized element `%s[\"%.*s\"] is not allowed'\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            b"SYMTAB\0" as *const u8 as *const libc::c_char,
                            (*t2).sub.val.slen as libc::c_int,
                            (*t2).sub.val.sp,
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
                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                            302 as libc::c_int,
                        );
                        (Some(lintfunc.unwrap()))
                            .unwrap()(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"reference to uninitialized element `%s[\"%.*s\"]'\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            array_vname(t1),
                            (*t2).sub.val.slen as libc::c_int,
                            (*t2).sub.val.sp,
                        );
                        if (*t2).sub.val.slen == 0 as libc::c_int as libc::c_ulong {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    libc::c_int,
                                ) -> ())(
                                b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                305 as libc::c_int,
                            );
                            (Some(lintfunc.unwrap()))
                                .unwrap()(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"subscript of array `%s' is null string\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                array_vname(t1),
                            );
                        }
                    }
                }
                if t1 == func_table {
                    static mut warned: bool = 0 as libc::c_int != 0;
                    if do_flags as libc::c_uint
                        & DO_LINT_EXTENSIONS as libc::c_int as libc::c_uint != 0
                        && !warned
                    {
                        warned = 1 as libc::c_int != 0;
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                            317 as libc::c_int,
                        );
                        (Some(lintfunc.unwrap()))
                            .unwrap()(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"FUNCTAB is a gawk extension\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                    r = t2;
                } else {
                    if t1 == symbol_table {
                        update_global_values();
                    }
                    r = *((*(*t1).sub.nodep.l.lp).lookup).unwrap()(t1, t2);
                }
                DEREF(t2);
                if t1 == symbol_table {
                    static mut warned_0: bool = 0 as libc::c_int != 0;
                    if do_flags as libc::c_uint
                        & DO_LINT_EXTENSIONS as libc::c_int as libc::c_uint != 0
                        && !warned_0
                    {
                        warned_0 = 1 as libc::c_int != 0;
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                            335 as libc::c_int,
                        );
                        (Some(lintfunc.unwrap()))
                            .unwrap()(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"SYMTAB is a gawk extension\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                    if (*r).type_0 as libc::c_uint
                        == Node_var as libc::c_int as libc::c_uint
                    {
                        r = (*r).sub.nodep.l.lptr;
                    } else if (*r).type_0 as libc::c_uint
                        == Node_var_new as libc::c_int as libc::c_uint
                    {
                        (*r).sub.nodep.l.lptr = dupnode(Nnull_string);
                        r = (*r).sub.nodep.l.lptr;
                    }
                }
                if (*r).type_0 as libc::c_uint == Node_val as libc::c_int as libc::c_uint
                    || (*r).type_0 as libc::c_uint
                        == Node_var as libc::c_int as libc::c_uint
                    || (*r).type_0 as libc::c_uint
                        == Node_elem_new as libc::c_int as libc::c_uint
                {
                    (*r).valref += 1;
                    (*r).valref;
                }
                let ref mut fresh70 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh70 = r;
                current_block = 3518619798157913413;
            }
            17 => {
                t2 = if (*pc).d.dl == 1 as libc::c_int as libc::c_long {
                    POP_SCALAR()
                } else {
                    concat_exp((*pc).d.dl as libc::c_int, 1 as libc::c_int != 0)
                };
                t1 = POP_ARRAY(0 as libc::c_int != 0);
                r = in_array(t1, t2);
                if r.is_null() {
                    t2 = force_string_fmt(t2, CONVFMT, CONVFMTidx);
                    if t1 == func_table {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                            362 as libc::c_int,
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
                                b"reference to uninitialized element `%s[\"%.*s\"] is not allowed'\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            b"FUNCTAB\0" as *const u8 as *const libc::c_char,
                            (*t2).sub.val.slen as libc::c_int,
                            (*t2).sub.val.sp,
                        );
                    } else if t1 == symbol_table {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                            365 as libc::c_int,
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
                                b"reference to uninitialized element `%s[\"%.*s\"] is not allowed'\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            b"SYMTAB\0" as *const u8 as *const libc::c_char,
                            (*t2).sub.val.slen as libc::c_int,
                            (*t2).sub.val.sp,
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
                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                            368 as libc::c_int,
                        );
                        (Some(lintfunc.unwrap()))
                            .unwrap()(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"reference to uninitialized element `%s[\"%.*s\"]'\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            array_vname(t1),
                            (*t2).sub.val.slen as libc::c_int,
                            (*t2).sub.val.sp,
                        );
                        if (*t2).sub.val.slen == 0 as libc::c_int as libc::c_ulong {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    libc::c_int,
                                ) -> ())(
                                b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                371 as libc::c_int,
                            );
                            (Some(lintfunc.unwrap()))
                                .unwrap()(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"subscript of array `%s' is null string\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                array_vname(t1),
                            );
                        }
                    }
                }
                if r.is_null() {
                    r = make_array();
                    (*r).sub.nodep.x.extra = t1;
                    t2 = force_string_fmt(t2, CONVFMT, CONVFMTidx);
                    (*r).sub.nodep.name = estrdup((*t2).sub.val.sp, (*t2).sub.val.slen);
                    assoc_set(t1, t2, r);
                } else if (*r).type_0 as libc::c_uint
                    == Node_elem_new as libc::c_int as libc::c_uint
                {
                    r = force_array(r, 0 as libc::c_int != 0);
                    (*r).sub.nodep.x.extra = t1;
                    t2 = force_string_fmt(t2, CONVFMT, CONVFMTidx);
                    (*r).sub.nodep.name = estrdup((*t2).sub.val.sp, (*t2).sub.val.slen);
                } else if (*r).type_0 as libc::c_uint
                    != Node_var_array as libc::c_int as libc::c_uint
                {
                    t2 = force_string_fmt(t2, CONVFMT, CONVFMTidx);
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const libc::c_char,
                        388 as libc::c_int,
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
                            b"attempt to use scalar `%s[\"%.*s\"]' as an array\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        array_vname(t1),
                        (*t2).sub.val.slen as libc::c_int,
                        (*t2).sub.val.sp,
                    );
                } else {
                    DEREF(t2);
                }
                let ref mut fresh71 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh71 = r;
                current_block = 3518619798157913413;
            }
            83 => {
                t2 = if (*pc).d.dl == 1 as libc::c_int as libc::c_long {
                    POP_SCALAR()
                } else {
                    concat_exp((*pc).d.dl as libc::c_int, 1 as libc::c_int != 0)
                };
                t1 = POP_ARRAY(0 as libc::c_int != 0);
                if do_flags as libc::c_uint
                    & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                        as libc::c_uint != 0 && (in_array(t1, t2)).is_null()
                {
                    t2 = force_string_fmt(t2, CONVFMT, CONVFMTidx);
                    if (*pc).x.xl != 0 {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                            402 as libc::c_int,
                        );
                        (Some(lintfunc.unwrap()))
                            .unwrap()(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"reference to uninitialized element `%s[\"%.*s\"]'\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            array_vname(t1),
                            (*t2).sub.val.slen as libc::c_int,
                            (*t2).sub.val.sp,
                        );
                    }
                    if (*t2).sub.val.slen == 0 as libc::c_int as libc::c_ulong {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                            405 as libc::c_int,
                        );
                        (Some(lintfunc.unwrap()))
                            .unwrap()(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"subscript of array `%s' is null string\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            array_vname(t1),
                        );
                    }
                }
                lhs = ((*(*t1).sub.nodep.l.lp).lookup).unwrap()(t1, t2);
                if (**lhs).type_0 as libc::c_uint
                    == Node_var_array as libc::c_int as libc::c_uint
                {
                    t2 = force_string_fmt(t2, CONVFMT, CONVFMTidx);
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const libc::c_char,
                        411 as libc::c_int,
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
                            b"attempt to use array `%s[\"%.*s\"]' in a scalar context\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        array_vname(t1),
                        (*t2).sub.val.slen as libc::c_int,
                        (*t2).sub.val.sp,
                    );
                }
                if t1 == func_table {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const libc::c_char,
                        429 as libc::c_int,
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
                            b"cannot assign to elements of FUNCTAB\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                } else if t1 == symbol_table {
                    if (**lhs).type_0 as libc::c_uint
                        == Node_var as libc::c_int as libc::c_uint
                        || (**lhs).type_0 as libc::c_uint
                            == Node_var_new as libc::c_int as libc::c_uint
                    {
                        update_global_values();
                        (**lhs).type_0 = Node_var;
                        lhs = &mut (**lhs).sub.nodep.l.lptr;
                    } else {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                            437 as libc::c_int,
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
                                b"cannot assign to arbitrary elements of SYMTAB\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                }
                if ((*(*t1).sub.nodep.l.lp).store).is_some() {
                    set_array = t1;
                    set_idx = t2;
                } else {
                    DEREF(t2);
                }
                let ref mut fresh72 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .lptr;
                *fresh72 = lhs;
                current_block = 3518619798157913413;
            }
            24 => {
                t1 = TOP_SCALAR();
                lhs = r_get_field(t1, 0 as *mut Func_ptr, 1 as libc::c_int != 0);
                stack_ptr = stack_ptr.offset(-1);
                stack_ptr;
                DEREF(t1);
                r = *lhs;
                (*r).valref += 1;
                (*r).valref;
                let ref mut fresh73 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh73 = r;
                current_block = 3518619798157913413;
            }
            84 => {
                t1 = TOP_SCALAR();
                lhs = r_get_field(t1, &mut (*(*pc).d.di).x.aptr, (*pc).x.xl != 0);
                stack_ptr = stack_ptr.offset(-1);
                stack_ptr;
                DEREF(t1);
                let ref mut fresh74 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .lptr;
                *fresh74 = lhs;
                current_block = 3518619798157913413;
            }
            105 => {
                if do_flags as libc::c_uint
                    & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                        as libc::c_uint != 0
                {
                    match (*pc).d.dl {
                        1 => {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    libc::c_int,
                                ) -> ())(
                                b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                474 as libc::c_int,
                            );
                            (Some(lintfunc.unwrap()))
                                .unwrap()(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"assignment used in conditional context\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                        }
                        _ => {
                            r_fatal(
                                b"internal error: file %s, line %d: unexpected lint type value %d\0"
                                    as *const u8 as *const libc::c_char,
                                b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                478 as libc::c_int,
                                (*pc).d.dl as libc::c_int,
                            );
                        }
                    }
                }
                current_block = 3518619798157913413;
            }
            106 => {
                t1 = (*stack_ptr).rptr;
                t2 = (*stack_ptr.offset(-(1 as libc::c_int as isize))).rptr;
                if (*t1).flags as libc::c_uint & STRING as libc::c_int as libc::c_uint
                    != 0 as libc::c_int as libc::c_uint
                    && (*t2).flags as libc::c_uint
                        & STRING as libc::c_int as libc::c_uint
                        != 0 as libc::c_int as libc::c_uint
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const libc::c_char,
                        489 as libc::c_int,
                    );
                    (Some(lintfunc.unwrap()))
                        .unwrap()(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"operator `+' used on two string values\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                current_block = 3518619798157913413;
            }
            54 | 55 | 87 => {
                if post_execute.is_some() {
                    post_execute.unwrap()(pc);
                }
                pc = (*pc).d.di;
                continue;
            }
            89 => {
                r = POP_SCALAR();
                di = eval_condition(r) as libc::c_int;
                DEREF(r);
                if di == 0 {
                    if post_execute.is_some() {
                        post_execute.unwrap()(pc);
                    }
                    pc = (*pc).d.di;
                    continue;
                } else {
                    current_block = 3518619798157913413;
                }
            }
            88 => {
                r = POP_SCALAR();
                di = eval_condition(r) as libc::c_int;
                DEREF(r);
                if di != 0 {
                    if post_execute.is_some() {
                        post_execute.unwrap()(pc);
                    }
                    pc = (*pc).d.di;
                    continue;
                } else {
                    current_block = 3518619798157913413;
                }
            }
            38 | 40 => {
                t1 = POP_SCALAR();
                di = eval_condition(t1) as libc::c_int;
                DEREF(t1);
                if op as libc::c_uint == Op_and as libc::c_int as libc::c_uint && di != 0
                    || op as libc::c_uint == Op_or as libc::c_int as libc::c_uint
                        && di == 0
                {
                    current_block = 3518619798157913413;
                } else {
                    r = node_Boolean[di as usize];
                    (*r).valref += 1;
                    (*r).valref;
                    let ref mut fresh75 = (*if stack_ptr < stack_top {
                        stack_ptr = stack_ptr.offset(1);
                        stack_ptr
                    } else {
                        grow_stack()
                    })
                        .rptr;
                    *fresh75 = r;
                    ni = (*pc).d.di;
                    if post_execute.is_some() {
                        post_execute.unwrap()(pc);
                    }
                    pc = (*ni).nexti;
                    continue;
                }
            }
            39 | 41 => {
                t1 = TOP_SCALAR();
                r = node_Boolean[eval_condition(t1) as usize];
                DEREF(t1);
                (*r).valref += 1;
                (*r).valref;
                (*stack_ptr).rptr = r;
                current_block = 3518619798157913413;
            }
            25 => {
                t1 = TOP_SCALAR();
                r = node_Boolean[!eval_condition(t1) as libc::c_int as usize];
                DEREF(t1);
                (*r).valref += 1;
                (*r).valref;
                (*stack_ptr).rptr = r;
                current_block = 3518619798157913413;
            }
            42 => {
                r = node_Boolean[cmp_scalars(SCALAR_EQ) as usize];
                (*r).valref += 1;
                (*r).valref;
                (*stack_ptr).rptr = r;
                current_block = 3518619798157913413;
            }
            43 => {
                r = node_Boolean[cmp_scalars(SCALAR_NEQ) as usize];
                (*r).valref += 1;
                (*r).valref;
                (*stack_ptr).rptr = r;
                current_block = 3518619798157913413;
            }
            44 => {
                r = node_Boolean[cmp_scalars(SCALAR_LT) as usize];
                (*r).valref += 1;
                (*r).valref;
                (*stack_ptr).rptr = r;
                current_block = 3518619798157913413;
            }
            45 => {
                r = node_Boolean[cmp_scalars(SCALAR_GT) as usize];
                (*r).valref += 1;
                (*r).valref;
                (*stack_ptr).rptr = r;
                current_block = 3518619798157913413;
            }
            46 => {
                r = node_Boolean[cmp_scalars(SCALAR_LE) as usize];
                (*r).valref += 1;
                (*r).valref;
                (*stack_ptr).rptr = r;
                current_block = 3518619798157913413;
            }
            47 => {
                r = node_Boolean[cmp_scalars(SCALAR_GE) as usize];
                (*r).valref += 1;
                (*r).valref;
                (*stack_ptr).rptr = r;
                current_block = 3518619798157913413;
            }
            8 => {
                x2 = (*force_number((*pc).d.dn)).sub.val.fltnum;
                current_block = 13780331501197148875;
            }
            7 => {
                t2 = force_number(POP_SCALAR());
                x2 = (*t2).sub.val.fltnum;
                DEREF(t2);
                current_block = 13780331501197148875;
            }
            10 => {
                x2 = (*force_number((*pc).d.dn)).sub.val.fltnum;
                current_block = 4151678221539060801;
            }
            9 => {
                t2 = force_number(POP_SCALAR());
                x2 = (*t2).sub.val.fltnum;
                DEREF(t2);
                current_block = 4151678221539060801;
            }
            2 => {
                x2 = (*force_number((*pc).d.dn)).sub.val.fltnum;
                current_block = 1804798499499804236;
            }
            1 => {
                t2 = force_number(POP_SCALAR());
                x2 = (*t2).sub.val.fltnum;
                DEREF(t2);
                current_block = 1804798499499804236;
            }
            12 => {
                x2 = (*force_number((*pc).d.dn)).sub.val.fltnum;
                current_block = 17364872992865346463;
            }
            11 => {
                t2 = force_number(POP_SCALAR());
                x2 = (*t2).sub.val.fltnum;
                DEREF(t2);
                current_block = 17364872992865346463;
            }
            4 => {
                x2 = (*force_number((*pc).d.dn)).sub.val.fltnum;
                current_block = 12664007985747274825;
            }
            3 => {
                t2 = force_number(POP_SCALAR());
                x2 = (*t2).sub.val.fltnum;
                DEREF(t2);
                current_block = 12664007985747274825;
            }
            6 => {
                x2 = (*force_number((*pc).d.dn)).sub.val.fltnum;
                current_block = 15749617915083522005;
            }
            5 => {
                t2 = force_number(POP_SCALAR());
                x2 = (*t2).sub.val.fltnum;
                DEREF(t2);
                current_block = 15749617915083522005;
            }
            18 | 19 => {
                x = if op as libc::c_uint
                    == Op_preincrement as libc::c_int as libc::c_uint
                {
                    1.0f64
                } else {
                    -1.0f64
                };
                lhs = (*stack_ptr).lptr;
                t1 = *lhs;
                force_number(t1);
                if (*t1).valref == 1 as libc::c_int as libc::c_long
                    && (*t1).flags as libc::c_uint
                        == (MALLOC as libc::c_int | NUMCUR as libc::c_int
                            | NUMBER as libc::c_int) as libc::c_uint
                {
                    (*t1).sub.val.fltnum += x;
                    r = t1;
                } else {
                    *lhs = make_number.unwrap()((*t1).sub.val.fltnum + x);
                    r = *lhs;
                    unref(t1);
                }
                (*r).valref += 1;
                (*r).valref;
                (*stack_ptr).rptr = r;
                current_block = 3518619798157913413;
            }
            20 | 21 => {
                x = if op as libc::c_uint
                    == Op_postincrement as libc::c_int as libc::c_uint
                {
                    1.0f64
                } else {
                    -1.0f64
                };
                lhs = (*stack_ptr).lptr;
                t1 = *lhs;
                force_number(t1);
                r = make_number.unwrap()((*t1).sub.val.fltnum);
                if (*t1).valref == 1 as libc::c_int as libc::c_long
                    && (*t1).flags as libc::c_uint
                        == (MALLOC as libc::c_int | NUMCUR as libc::c_int
                            | NUMBER as libc::c_int) as libc::c_uint
                {
                    (*t1).sub.val.fltnum += x;
                } else {
                    *lhs = make_number.unwrap()((*t1).sub.val.fltnum + x);
                    unref(t1);
                }
                (*stack_ptr).rptr = r;
                current_block = 3518619798157913413;
            }
            22 => {
                t1 = force_number(TOP_SCALAR());
                r = make_number.unwrap()(-(*t1).sub.val.fltnum);
                DEREF(t1);
                (*stack_ptr).rptr = r;
                current_block = 3518619798157913413;
            }
            23 => {
                t1 = force_number(TOP_SCALAR());
                r = make_number.unwrap()((*t1).sub.val.fltnum);
                DEREF(t1);
                (*stack_ptr).rptr = r;
                current_block = 3518619798157913413;
            }
            28 => {
                t1 = force_array((*pc).d.dn, 1 as libc::c_int != 0);
                t2 = if (*pc).x.xl == 1 as libc::c_int as libc::c_long {
                    POP_SCALAR()
                } else {
                    concat_exp((*pc).x.xl as libc::c_int, 1 as libc::c_int != 0)
                };
                lhs = ((*(*t1).sub.nodep.l.lp).lookup).unwrap()(t1, t2);
                if (**lhs).type_0 as libc::c_uint
                    == Node_var_array as libc::c_int as libc::c_uint
                {
                    t2 = force_string_fmt(t2, CONVFMT, CONVFMTidx);
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const libc::c_char,
                        737 as libc::c_int,
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
                            b"attempt to use array `%s[\"%.*s\"]' in a scalar context\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        array_vname(t1),
                        (*t2).sub.val.slen as libc::c_int,
                        (*t2).sub.val.sp,
                    );
                }
                DEREF(t2);
                if t1 == func_table {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const libc::c_char,
                        755 as libc::c_int,
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
                            b"cannot assign to elements of FUNCTAB\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                } else if t1 == symbol_table {
                    if (**lhs).type_0 as libc::c_uint
                        == Node_var as libc::c_int as libc::c_uint
                        || (**lhs).type_0 as libc::c_uint
                            == Node_var_new as libc::c_int as libc::c_uint
                    {
                        update_global_values();
                        (**lhs).type_0 = Node_var;
                        lhs = &mut (**lhs).sub.nodep.l.lptr;
                    } else {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                            763 as libc::c_int,
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
                                b"cannot assign to arbitrary elements of SYMTAB\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                }
                unref(*lhs);
                r = POP_SCALAR();
                unfield(lhs, &mut r);
                if ((*(*t1).sub.nodep.l.lp).store).is_some() {
                    (Some(((*(*t1).sub.nodep.l.lp).store).unwrap())).unwrap()(t1, t2);
                }
                DEREF(t2);
                current_block = 3518619798157913413;
            }
            27 => {
                lhs = if (*(*pc).d.dn).type_0 as libc::c_uint
                    == Node_var as libc::c_int as libc::c_uint
                    && !((*(*pc).d.dn).sub.nodep.l.lptr == Nnull_string)
                {
                    &mut (*(*pc).d.dn).sub.nodep.l.lptr
                } else {
                    r_get_lhs((*pc).d.dn, 0 as libc::c_int != 0)
                };
                unref(*lhs);
                r = (*pc).x.xn;
                if !r.is_null() {
                    (*r).valref += 1;
                    (*r).valref;
                    *lhs = r;
                } else {
                    r = POP_SCALAR();
                    unfield(lhs, &mut r);
                }
                current_block = 3518619798157913413;
            }
            29 | 30 => {
                let mut assign: Func_ptr = None;
                t1 = TOP_SCALAR();
                lhs = r_get_field(t1, &mut assign, 0 as libc::c_int != 0);
                stack_ptr = stack_ptr.offset(-1);
                stack_ptr;
                DEREF(t1);
                assign.unwrap()();
                unref(*lhs);
                r = POP_SCALAR();
                unfield(lhs, &mut r);
                force_string_fmt(*lhs, CONVFMT, CONVFMTidx);
                if op as libc::c_uint
                    == Op_store_field_exp as libc::c_int as libc::c_uint
                {
                    (**lhs).valref += 1;
                    (**lhs).valref;
                    let ref mut fresh76 = (*if stack_ptr < stack_top {
                        stack_ptr = stack_ptr.offset(1);
                        stack_ptr
                    } else {
                        grow_stack()
                    })
                        .rptr;
                    *fresh76 = *lhs;
                }
                current_block = 3518619798157913413;
            }
            37 => {
                lhs = if (*(*pc).d.dn).type_0 as libc::c_uint
                    == Node_var as libc::c_int as libc::c_uint
                    && !((*(*pc).d.dn).sub.nodep.l.lptr == Nnull_string)
                {
                    &mut (*(*pc).d.dn).sub.nodep.l.lptr
                } else {
                    r_get_lhs((*pc).d.dn, 0 as libc::c_int != 0)
                };
                t1 = force_string_fmt(*lhs, CONVFMT, CONVFMTidx);
                t2 = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
                if t1 != *lhs {
                    unref(*lhs);
                    if (*t1).valref == 1 as libc::c_int as libc::c_long {
                        *lhs = t1;
                    } else {
                        *lhs = dupnode(t1);
                    }
                }
                if t1 != t2 && (*t1).valref == 1 as libc::c_int as libc::c_long
                    && (*t1).flags as libc::c_uint
                        & (MALLOC as libc::c_int | MPFN as libc::c_int
                            | MPZN as libc::c_int) as libc::c_uint
                        == MALLOC as libc::c_int as libc::c_uint
                {
                    let mut nlen: size_t = ((*t1).sub.val.slen)
                        .wrapping_add((*t2).sub.val.slen);
                    (*t1)
                        .sub
                        .val
                        .sp = erealloc_real(
                        (*t1).sub.val.sp as *mut libc::c_void,
                        nlen.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        b"r_interpret\0" as *const u8 as *const libc::c_char,
                        b"t1->stptr\0" as *const u8 as *const libc::c_char,
                        b"./interpret.h\0" as *const u8 as *const libc::c_char,
                        843 as libc::c_int,
                    ) as *mut libc::c_char;
                    memcpy(
                        ((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize)
                            as *mut libc::c_void,
                        (*t2).sub.val.sp as *const libc::c_void,
                        (*t2).sub.val.slen,
                    );
                    (*t1).sub.val.slen = nlen;
                    *((*t1).sub.val.sp)
                        .offset(nlen as isize) = '\0' as i32 as libc::c_char;
                    (*t1)
                        .flags = ::std::mem::transmute::<
                        libc::c_uint,
                        flagvals,
                    >(
                        (*t1).flags as libc::c_uint
                            & WSTRCUR as libc::c_int as libc::c_uint,
                    );
                    (*t1)
                        .flags = ::std::mem::transmute::<
                        libc::c_uint,
                        flagvals,
                    >(
                        (*t1).flags as libc::c_uint
                            | (MALLOC as libc::c_int | STRING as libc::c_int
                                | STRCUR as libc::c_int) as libc::c_uint,
                    );
                    (*t1).sub.val.idx = -(1 as libc::c_int);
                    if (*t1).flags as libc::c_uint
                        & WSTRCUR as libc::c_int as libc::c_uint
                        != 0 as libc::c_int as libc::c_uint
                        && (*t2).flags as libc::c_uint
                            & WSTRCUR as libc::c_int as libc::c_uint
                            != 0 as libc::c_int as libc::c_uint
                    {
                        let mut wlen: size_t = ((*t1).sub.val.wslen)
                            .wrapping_add((*t2).sub.val.wslen);
                        (*t1)
                            .sub
                            .val
                            .wsp = erealloc_real(
                            (*t1).sub.val.wsp as *mut libc::c_void,
                            (::std::mem::size_of::<wchar_t>() as libc::c_ulong)
                                .wrapping_mul(
                                    wlen.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                ),
                            b"r_interpret\0" as *const u8 as *const libc::c_char,
                            b"t1->wstptr\0" as *const u8 as *const libc::c_char,
                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                            860 as libc::c_int,
                        ) as *mut wchar_t;
                        memcpy(
                            ((*t1).sub.val.wsp).offset((*t1).sub.val.wslen as isize)
                                as *mut libc::c_void,
                            (*t2).sub.val.wsp as *const libc::c_void,
                            ((*t2).sub.val.wslen)
                                .wrapping_mul(
                                    ::std::mem::size_of::<wchar_t>() as libc::c_ulong,
                                ),
                        );
                        (*t1).sub.val.wslen = wlen;
                        *((*t1).sub.val.wsp).offset(wlen as isize) = '\0' as i32;
                    } else if (**lhs).flags as libc::c_uint
                        & WSTRCUR as libc::c_int as libc::c_uint != 0
                    {
                        r_free_wstr(*lhs);
                    }
                } else {
                    let mut nlen_0: size_t = ((*t1).sub.val.slen)
                        .wrapping_add((*t2).sub.val.slen);
                    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
                    p = emalloc_real(
                        nlen_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        b"r_interpret\0" as *const u8 as *const libc::c_char,
                        b"p\0" as *const u8 as *const libc::c_char,
                        b"./interpret.h\0" as *const u8 as *const libc::c_char,
                        870 as libc::c_int,
                    ) as *mut libc::c_char;
                    memcpy(
                        p as *mut libc::c_void,
                        (*t1).sub.val.sp as *const libc::c_void,
                        (*t1).sub.val.slen,
                    );
                    memcpy(
                        p.offset((*t1).sub.val.slen as isize) as *mut libc::c_void,
                        (*t2).sub.val.sp as *const libc::c_void,
                        (*t2).sub.val.slen,
                    );
                    unref(*lhs);
                    *lhs = make_str_node(p, nlen_0, 2 as libc::c_int);
                    t1 = *lhs;
                }
                DEREF(t2);
                current_block = 3518619798157913413;
            }
            26 => {
                let fresh77 = stack_ptr;
                stack_ptr = stack_ptr.offset(-1);
                lhs = (*fresh77).lptr;
                r = TOP_SCALAR();
                unref(*lhs);
                if (*r).type_0 as libc::c_uint
                    == Node_elem_new as libc::c_int as libc::c_uint
                {
                    DEREF(r);
                    r = dupnode(Nnull_string);
                }
                (*r).valref += 1;
                (*r).valref;
                unfield(lhs, &mut r);
                (*stack_ptr).rptr = r;
                current_block = 3518619798157913413;
            }
            98 => {
                if !set_idx.is_null() {
                    di = 1 as libc::c_int;
                    if (*pc).d.dl == Op_sub_builtin as libc::c_int as libc::c_long
                        && {
                            r = (*stack_ptr).rptr;
                            !r.is_null()
                        }
                        && (*r).sub.val.fltnum as libc::c_long
                            == 0 as libc::c_int as libc::c_long
                    {
                        di = 0 as libc::c_int;
                    } else if ((*pc).d.dl == Op_K_getline as libc::c_int as libc::c_long
                        || (*pc).d.dl
                            == Op_K_getline_redir as libc::c_int as libc::c_long)
                        && {
                            r = (*stack_ptr).rptr;
                            !r.is_null()
                        }
                        && (*r).sub.val.fltnum as libc::c_long
                            <= 0 as libc::c_int as libc::c_long
                    {
                        di = 0 as libc::c_int;
                    }
                    if di != 0 {
                        (Some(((*(*set_array).sub.nodep.l.lp).store).unwrap()))
                            .unwrap()(set_array, set_idx);
                    }
                    unref(set_idx);
                    set_idx = 0 as *mut NODE;
                }
                current_block = 3518619798157913413;
            }
            34 | 35 | 31 | 32 | 33 | 36 => {
                op_assign(op);
                current_block = 3518619798157913413;
            }
            95 => {
                ((*pc).x.aptr).unwrap()();
                current_block = 3518619798157913413;
            }
            96 | 97 => {
                r = (*stack_ptr).rptr;
                if (*pc).d.dl == Op_sub_builtin as libc::c_int as libc::c_long
                    && (*r).sub.val.fltnum as libc::c_long
                        == 0 as libc::c_int as libc::c_long
                {
                    current_block = 3518619798157913413;
                } else if ((*pc).d.dl == Op_K_getline as libc::c_int as libc::c_long
                    || (*pc).d.dl == Op_K_getline_redir as libc::c_int as libc::c_long)
                    && (*r).sub.val.fltnum as libc::c_long
                        <= 0 as libc::c_int as libc::c_long
                {
                    current_block = 3518619798157913413;
                } else {
                    if op as libc::c_uint == Op_var_assign as libc::c_int as libc::c_uint
                    {
                        ((*pc).x.aptr).unwrap()();
                    } else {
                        ((*pc).x.aptr).unwrap()();
                    }
                    current_block = 3518619798157913413;
                }
            }
            13 => {
                r = concat_exp(
                    (*pc).x.xl as libc::c_int,
                    (*pc).d.dl & 1 as libc::c_int as libc::c_long != 0,
                );
                let ref mut fresh78 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh78 = r;
                current_block = 3518619798157913413;
            }
            52 => {
                if (*pc.offset(1 as libc::c_int as isize)).x.xl != 0 {
                    let fresh79 = stack_ptr;
                    stack_ptr = stack_ptr.offset(-1);
                    m = (*fresh79).rptr;
                    t2 = TOP_SCALAR();
                    t2 = force_string_fmt(t2, CONVFMT, CONVFMTidx);
                    rp = re_update(m);
                    di = (research(
                        rp,
                        (*t2).sub.val.sp,
                        0 as libc::c_int,
                        (*t2).sub.val.slen,
                        0 as libc::c_int,
                    ) >= 0 as libc::c_int) as libc::c_int;
                } else {
                    t1 = POP_SCALAR();
                    t2 = TOP_SCALAR();
                    di = (cmp_nodes(t2, t1, 1 as libc::c_int != 0) == 0 as libc::c_int)
                        as libc::c_int;
                    DEREF(t1);
                }
                if di != 0 {
                    t2 = POP_SCALAR();
                    DEREF(t2);
                    if post_execute.is_some() {
                        post_execute.unwrap()(pc);
                    }
                    pc = (*pc).d.di;
                    continue;
                } else {
                    current_block = 3518619798157913413;
                }
            }
            63 => {
                t1 = POP_ARRAY(0 as libc::c_int != 0);
                do_delete(t1, (*pc).x.xl as libc::c_int);
                stack_ptr = stack_ptr.offset(-(*pc).x.xl as isize);
                current_block = 3518619798157913413;
            }
            64 => {
                t1 = POP_ARRAY(0 as libc::c_int != 0);
                let fresh80 = stack_ptr;
                stack_ptr = stack_ptr.offset(-1);
                lhs = (*fresh80).lptr;
                do_delete_loop(t1, lhs);
                current_block = 3518619798157913413;
            }
            72 => {
                t1 = POP_ARRAY(0 as libc::c_int != 0);
                t2 = if (*pc).x.xl == 1 as libc::c_int as libc::c_long {
                    POP_SCALAR()
                } else {
                    concat_exp((*pc).x.xl as libc::c_int, 1 as libc::c_int != 0)
                };
                r = node_Boolean[(in_array(t1, t2)
                    != 0 as *mut libc::c_void as *mut NODE) as libc::c_int as usize];
                DEREF(t2);
                (*r).valref += 1;
                (*r).valref;
                let ref mut fresh81 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh81 = r;
                current_block = 3518619798157913413;
            }
            92 => {
                let mut list: *mut *mut NODE = 0 as *mut *mut NODE;
                let mut array: *mut NODE = 0 as *mut NODE;
                let mut sort_str: *mut NODE = 0 as *mut NODE;
                let mut num_elems: size_t = 0 as libc::c_int as size_t;
                static mut sorted_in: *mut NODE = 0 as *const NODE as *mut NODE;
                let mut how_to_sort: *const libc::c_char = b"@unsorted\0" as *const u8
                    as *const libc::c_char;
                let mut save_0: libc::c_char = 0;
                let mut saved_end: bool = 0 as libc::c_int != 0;
                array = POP_ARRAY(1 as libc::c_int != 0);
                num_elems = (*array).sub.nodep.reflags as size_t;
                if !(num_elems == 0 as libc::c_int as libc::c_ulong) {
                    if sorted_in.is_null() {
                        sorted_in = make_str_node(
                            b"sorted_in\0" as *const u8 as *const libc::c_char,
                            9 as libc::c_int as size_t,
                            0 as libc::c_int,
                        );
                    }
                    sort_str = 0 as *mut NODE;
                    if do_flags as libc::c_uint & DO_POSIX as libc::c_int as libc::c_uint
                        == 0 && !PROCINFO_node.is_null()
                    {
                        sort_str = in_array(PROCINFO_node, sorted_in);
                    }
                    if !sort_str.is_null() {
                        sort_str = force_string_fmt(sort_str, CONVFMT, CONVFMTidx);
                        if (*sort_str).sub.val.slen > 0 as libc::c_int as libc::c_ulong {
                            how_to_sort = (*sort_str).sub.val.sp;
                            str_terminate_f(sort_str, &mut save_0);
                            saved_end = 1 as libc::c_int != 0;
                        }
                    }
                    list = assoc_list(array, how_to_sort, SORTED_IN);
                    if saved_end {
                        *((*sort_str).sub.val.sp)
                            .offset((*sort_str).sub.val.slen as isize) = save_0;
                    }
                }
                r = nextfree[BLOCK_NODE as libc::c_int as usize].freep as *mut NODE;
                if !r.is_null() {
                    nextfree[BLOCK_NODE as libc::c_int as usize]
                        .freep = (*(r as *mut block_item)).freep;
                } else {
                    r = more_blocks(BLOCK_NODE as libc::c_int) as *mut NODE;
                };
                (*r).type_0 = Node_arrayfor;
                (*r).sub.nodep.r.av = list;
                (*r).sub.nodep.reflags = num_elems as reflagvals;
                (*r).sub.nodep.l.ll = -(1 as libc::c_int) as libc::c_long;
                (*r).sub.nodep.rn = array;
                let ref mut fresh82 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh82 = r;
                if num_elems == 0 as libc::c_int as libc::c_ulong {
                    if post_execute.is_some() {
                        post_execute.unwrap()(pc);
                    }
                    pc = (*pc).d.di;
                    continue;
                } else {
                    current_block = 3518619798157913413;
                }
            }
            93 => {
                r = (*stack_ptr).rptr;
                (*r).sub.nodep.l.ll += 1;
                if (*r).sub.nodep.l.ll == (*r).sub.nodep.reflags as libc::c_long {
                    let mut array_0: *mut NODE = 0 as *mut NODE;
                    array_0 = (*r).sub.nodep.rn;
                    if do_flags as libc::c_uint
                        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                            as libc::c_uint != 0
                        && (*array_0).sub.nodep.reflags as libc::c_uint
                            != (*r).sub.nodep.reflags as libc::c_uint
                    {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                            1070 as libc::c_int,
                        );
                        (Some(lintfunc.unwrap()))
                            .unwrap()(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"for loop: array `%s' changed size from %ld to %ld during loop execution\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            array_vname(array_0),
                            (*r).sub.nodep.reflags as libc::c_long,
                            (*array_0).sub.nodep.reflags as libc::c_long,
                        );
                    }
                    if post_execute.is_some() {
                        post_execute.unwrap()(pc);
                    }
                    pc = (*pc).d.di;
                    continue;
                } else {
                    t1 = *((*r).sub.nodep.r.av).offset((*r).sub.nodep.l.ll as isize);
                    lhs = if (*(*pc).x.xn).type_0 as libc::c_uint
                        == Node_var as libc::c_int as libc::c_uint
                        && !((*(*pc).x.xn).sub.nodep.l.lptr == Nnull_string)
                    {
                        &mut (*(*pc).x.xn).sub.nodep.l.lptr
                    } else {
                        r_get_lhs((*pc).x.xn, 0 as libc::c_int != 0)
                    };
                    unref(*lhs);
                    *lhs = dupnode(t1);
                }
                current_block = 3518619798157913413;
            }
            94 => {
                let fresh83 = stack_ptr;
                stack_ptr = stack_ptr.offset(-1);
                r = (*fresh83).rptr;
                free_arrayfor(r);
                current_block = 3518619798157913413;
            }
            69 => {
                r = ((*pc).d.fptr).unwrap()((*pc).x.xl as libc::c_int);
                let ref mut fresh84 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh84 = r;
                current_block = 3518619798157913413;
            }
            71 => {
                let mut arg_count: size_t = (*pc).x.xl as size_t;
                let mut f: *mut awk_ext_func_t = (*pc.offset(1 as libc::c_int as isize))
                    .x
                    .exf;
                let mut min_req: size_t = (*f).min_required_args;
                let mut max_expect: size_t = (*f).max_expected_args;
                let mut result: awk_value_t = awk_value_t {
                    val_type: AWK_UNDEFINED,
                    u: C2RustUnnamed_0 {
                        s: awk_string_t {
                            str_0: 0 as *mut libc::c_char,
                            len: 0,
                        },
                    },
                };
                if arg_count < min_req {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const libc::c_char,
                        1101 as libc::c_int,
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
                            b"%s: called with %lu arguments, expecting at least %lu\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*pc.offset(1 as libc::c_int as isize)).d.name,
                        arg_count,
                        min_req,
                    );
                }
                if do_flags as libc::c_uint
                    & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                        as libc::c_uint != 0 && (*f).suppress_lint as u64 == 0
                    && arg_count > max_expect
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const libc::c_char,
                        1107 as libc::c_int,
                    );
                    (Some(lintfunc.unwrap()))
                        .unwrap()(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: called with %lu arguments, expecting no more than %lu\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*pc.offset(1 as libc::c_int as isize)).d.name,
                        arg_count,
                        max_expect,
                    );
                }
                PUSH_CODE(pc);
                let mut ef_ret: *mut awk_value_t = ((*pc).d.efptr)
                    .unwrap()(arg_count as libc::c_int, &mut result, f);
                r = awk_value_to_node(ef_ret);
                POP_CODE();
                loop {
                    let fresh85 = arg_count;
                    arg_count = arg_count.wrapping_sub(1);
                    if !(fresh85 > 0 as libc::c_int as libc::c_ulong) {
                        break;
                    }
                    let fresh86 = stack_ptr;
                    stack_ptr = stack_ptr.offset(-1);
                    t1 = (*fresh86).rptr;
                    if (*t1).type_0 as libc::c_uint
                        == Node_val as libc::c_int as libc::c_uint
                        || (*t1).type_0 as libc::c_uint
                            == Node_elem_new as libc::c_int as libc::c_uint
                    {
                        DEREF(t1);
                    }
                }
                free_api_string_copies();
                if in_indirect_call {
                    let fresh87 = stack_ptr;
                    stack_ptr = stack_ptr.offset(-1);
                    let mut fname: *mut NODE = (*fresh87).rptr;
                    DEREF(fname);
                    in_indirect_call = 0 as libc::c_int != 0;
                }
                let ref mut fresh88 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh88 = r;
                current_block = 3518619798157913413;
            }
            70 => {
                r = do_sub((*pc).x.xl as libc::c_int, (*pc).d.dl as libc::c_uint);
                let ref mut fresh89 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh89 = r;
                current_block = 3518619798157913413;
            }
            56 => {
                do_print((*pc).x.xl as libc::c_int, (*pc).d.dl as libc::c_int);
                current_block = 3518619798157913413;
            }
            58 => {
                do_printf((*pc).x.xl as libc::c_int, (*pc).d.dl as libc::c_int);
                current_block = 3518619798157913413;
            }
            57 => {
                do_print_rec((*pc).x.xl as libc::c_int, (*pc).d.dl as libc::c_int);
                current_block = 3518619798157913413;
            }
            79 => {
                m = (*pc).d.dn;
                if (*m).type_0 as libc::c_uint
                    == Node_dynregex as libc::c_int as libc::c_uint
                {
                    r = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
                    unref((*m).sub.nodep.x.extra);
                    (*m).sub.nodep.x.extra = r;
                } else if (*m).type_0 as libc::c_uint
                    == Node_val as libc::c_int as libc::c_uint
                {
                    (*m).valref += 1;
                    (*m).valref;
                }
                let ref mut fresh90 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh90 = m;
                current_block = 3518619798157913413;
            }
            49 => {
                m = (*pc).d.dn;
                t1 = *get_field(0 as libc::c_int as libc::c_long, 0 as *mut Func_ptr);
                current_block = 3432515699156962603;
            }
            50 | 48 => {
                m = (*pc).d.dn;
                t1 = force_string_fmt(TOP_SCALAR(), CONVFMT, CONVFMTidx);
                if (*m).type_0 as libc::c_uint
                    == Node_dynregex as libc::c_int as libc::c_uint
                {
                    unref((*m).sub.nodep.x.extra);
                    (*m).sub.nodep.x.extra = t1;
                    stack_ptr = stack_ptr.offset(-1);
                    stack_ptr;
                    t1 = force_string_fmt(TOP_SCALAR(), CONVFMT, CONVFMTidx);
                }
                current_block = 3432515699156962603;
            }
            74 => {
                let mut f_0: *mut NODE = 0 as *mut NODE;
                let mut arg_count_0: libc::c_int = 0;
                let mut save_1: libc::c_char = 0;
                let mut function_name: *mut NODE = 0 as *mut NODE;
                arg_count_0 = (*pc.offset(1 as libc::c_int as isize)).x.xl
                    as libc::c_int;
                t1 = (*stack_ptr.offset(-(arg_count_0 as isize))).rptr;
                if (*t1).type_0 as libc::c_uint
                    != Node_val as libc::c_int as libc::c_uint
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const libc::c_char,
                        1209 as libc::c_int,
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
                            b"indirect function call requires a simple scalar value\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                t1 = force_string_fmt(t1, CONVFMT, CONVFMTidx);
                str_terminate_f(t1, &mut save_1);
                if (*t1).sub.val.slen > 0 as libc::c_int as libc::c_ulong {
                    f_0 = (*pc).x.xn;
                    if !f_0.is_null()
                        && strcmp((*f_0).sub.nodep.name, (*t1).sub.val.sp)
                            == 0 as libc::c_int
                    {
                        *((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize) = save_1;
                        ni = setup_frame(pc);
                        if post_execute.is_some() {
                            post_execute.unwrap()(pc);
                        }
                        pc = ni;
                        continue;
                    } else {
                        f_0 = lookup((*t1).sub.val.sp);
                    }
                }
                if f_0.is_null() {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const libc::c_char,
                        1227 as libc::c_int,
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
                            b"`%s' is not a function, so it cannot be called indirectly\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*t1).sub.val.sp,
                    );
                    current_block = 8195495264928745804;
                } else if (*f_0).type_0 as libc::c_uint
                    == Node_builtin_func as libc::c_int as libc::c_uint
                {
                    let mut arg_count_1: libc::c_int = (*pc
                        .offset(1 as libc::c_int as isize))
                        .x
                        .xl as libc::c_int;
                    let mut the_func: builtin_func_t = lookup_builtin((*t1).sub.val.sp);
                    if the_func
                        == ::std::mem::transmute::<
                            Option::<
                                unsafe extern "C" fn(libc::c_int, libc::c_uint) -> *mut NODE,
                            >,
                            builtin_func_t,
                        >(
                            Some(
                                do_sub
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        libc::c_uint,
                                    ) -> *mut NODE,
                            ),
                        )
                    {
                        r = call_sub((*t1).sub.val.sp, arg_count_1);
                    } else if the_func
                        == Some(
                            do_match as unsafe extern "C" fn(libc::c_int) -> *mut NODE,
                        )
                    {
                        r = call_match(arg_count_1);
                    } else if the_func
                        == Some(
                            do_split as unsafe extern "C" fn(libc::c_int) -> *mut NODE,
                        )
                        || the_func
                            == Some(
                                do_patsplit
                                    as unsafe extern "C" fn(libc::c_int) -> *mut NODE,
                            )
                    {
                        r = call_split_func((*t1).sub.val.sp, arg_count_1);
                    } else {
                        r = the_func.unwrap()(arg_count_1);
                    }
                    *((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize) = save_1;
                    let fresh92 = stack_ptr;
                    stack_ptr = stack_ptr.offset(-1);
                    function_name = (*fresh92).rptr;
                    DEREF(function_name);
                    let ref mut fresh93 = (*if stack_ptr < stack_top {
                        stack_ptr = stack_ptr.offset(1);
                        stack_ptr
                    } else {
                        grow_stack()
                    })
                        .rptr;
                    *fresh93 = r;
                    current_block = 3518619798157913413;
                } else if (*f_0).type_0 as libc::c_uint
                    != Node_func as libc::c_int as libc::c_uint
                {
                    *((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize) = save_1;
                    if (*f_0).type_0 as libc::c_uint
                        == Node_ext_func as libc::c_int as libc::c_uint
                    {
                        let mut bc: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                        let mut fname_0: *mut libc::c_char = (*pc).d.name;
                        let mut arg_count_2: libc::c_int = (*pc
                            .offset(1 as libc::c_int as isize))
                            .x
                            .xl as libc::c_int;
                        static mut npc: [INSTRUCTION; 2] = [INSTRUCTION {
                            nexti: 0 as *const exp_instruction as *mut exp_instruction,
                            d: C2RustUnnamed_7 {
                                dn: 0 as *const NODE as *mut NODE,
                            },
                            x: C2RustUnnamed_6 { xl: 0 },
                            comment: 0 as *const exp_instruction as *mut exp_instruction,
                            source_line: 0,
                            pool_size: 0,
                            opcode: Op_illegal,
                        }; 2];
                        npc[0 as libc::c_int as usize] = *pc;
                        bc = (*f_0).sub.nodep.r.iptr;
                        npc[0 as libc::c_int as usize].opcode = Op_ext_builtin;
                        npc[0 as libc::c_int as usize].d.efptr = (*bc).d.efptr;
                        npc[0 as libc::c_int as usize]
                            .x
                            .xl = arg_count_2 as libc::c_long;
                        npc[1 as libc::c_int
                            as usize] = *pc.offset(1 as libc::c_int as isize);
                        npc[1 as libc::c_int as usize].d.name = fname_0;
                        npc[1 as libc::c_int as usize].x.exf = (*bc).x.exf;
                        in_indirect_call = 1 as libc::c_int != 0;
                        ni = npc.as_mut_ptr();
                        if post_execute.is_some() {
                            post_execute.unwrap()(pc);
                        }
                        pc = ni;
                        continue;
                    } else {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                            1277 as libc::c_int,
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
                                b"function called indirectly through `%s' does not exist\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            (*pc).d.name,
                        );
                    }
                    current_block = 8195495264928745804;
                } else {
                    current_block = 8195495264928745804;
                }
                match current_block {
                    3518619798157913413 => {}
                    _ => {
                        (*pc).x.xn = f_0;
                        *((*t1).sub.val.sp).offset((*t1).sub.val.slen as isize) = save_1;
                        ni = setup_frame(pc);
                        if post_execute.is_some() {
                            post_execute.unwrap()(pc);
                        }
                        pc = ni;
                        continue;
                    }
                }
            }
            73 => {
                let mut f_1: *mut NODE = 0 as *mut NODE;
                f_1 = (*pc).x.xn;
                if f_1.is_null() {
                    f_1 = lookup((*pc).d.name);
                    if f_1.is_null()
                        || (*f_1).type_0 as libc::c_uint
                            != Node_func as libc::c_int as libc::c_uint
                            && (*f_1).type_0 as libc::c_uint
                                != Node_ext_func as libc::c_int as libc::c_uint
                    {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"./interpret.h\0" as *const u8 as *const libc::c_char,
                            1296 as libc::c_int,
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
                                b"function `%s' not defined\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            (*pc).d.name,
                        );
                    }
                    (*pc).x.xn = f_1;
                }
                if (*f_1).type_0 as libc::c_uint
                    == Node_ext_func as libc::c_int as libc::c_uint
                {
                    let mut bc_0: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                    let mut fname_1: *mut libc::c_char = (*pc).d.name;
                    let mut arg_count_3: libc::c_int = (*pc
                        .offset(1 as libc::c_int as isize))
                        .x
                        .xl as libc::c_int;
                    bc_0 = (*f_1).sub.nodep.r.iptr;
                    (*pc).opcode = Op_ext_builtin;
                    (*pc).d.efptr = (*bc_0).d.efptr;
                    (*pc).x.xl = arg_count_3 as libc::c_long;
                    let ref mut fresh94 = (*pc.offset(1 as libc::c_int as isize)).d.name;
                    *fresh94 = fname_1;
                    let ref mut fresh95 = (*pc.offset(1 as libc::c_int as isize)).x.exf;
                    *fresh95 = (*bc_0).x.exf;
                    ni = pc;
                    if post_execute.is_some() {
                        post_execute.unwrap()(pc);
                    }
                    pc = ni;
                    continue;
                } else {
                    ni = setup_frame(pc);
                    if post_execute.is_some() {
                        post_execute.unwrap()(pc);
                    }
                    pc = ni;
                    continue;
                }
            }
            62 => {
                r_fatal(
                    b"internal error: file %s, line %d: unexpected opcode %s\0"
                        as *const u8 as *const libc::c_char,
                    b"./interpret.h\0" as *const u8 as *const libc::c_char,
                    1322 as libc::c_int,
                    opcode2str(op),
                );
                current_block = 3518619798157913413;
            }
            61 => {
                m = POP_SCALAR();
                ni = unwind_stack((*frame_ptr).sub.nodep.reflags as libc::c_long);
                let ref mut fresh96 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh96 = m;
                if post_execute.is_some() {
                    post_execute.unwrap()(pc);
                }
                pc = ni;
                continue;
            }
            65 => {
                r = do_getline_redir((*pc).x.xl as libc::c_int, (*pc).d.dl as redirval);
                let ref mut fresh97 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh97 = r;
                current_block = 3518619798157913413;
            }
            66 => {
                if currule == 0 || currule == BEGINFILE as libc::c_int
                    || currule == ENDFILE as libc::c_int
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const libc::c_char,
                        1342 as libc::c_int,
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
                            b"non-redirected `getline' invalid inside `%s' rule\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        *ruletab.as_ptr().offset(currule as isize),
                    );
                }
                loop {
                    let mut ret: libc::c_int = 0;
                    ret = nextfile(&mut curfile, 0 as libc::c_int != 0);
                    if ret <= 0 as libc::c_int {
                        r = do_getline((*pc).x.xl as libc::c_int, curfile);
                        if !r.is_null() {
                            break;
                        }
                    } else {
                        push_exec_state(pc, currule, source, stack_ptr);
                        if curfile.is_null() {
                            if post_execute.is_some() {
                                post_execute.unwrap()(pc);
                            }
                            pc = (*pc.offset(1 as libc::c_int as isize)).x.xi;
                            continue '_top;
                        } else {
                            if post_execute.is_some() {
                                post_execute.unwrap()(pc);
                            }
                            pc = (*pc.offset(1 as libc::c_int as isize)).d.di;
                            continue '_top;
                        }
                    }
                }
                let ref mut fresh98 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh98 = r;
                current_block = 3518619798157913413;
            }
            100 => {
                ni = pop_exec_state(&mut currule, &mut source, 0 as *mut libc::c_long);
                if post_execute.is_some() {
                    post_execute.unwrap()(pc);
                }
                pc = ni;
                continue;
            }
            99 => {
                after_beginfile(&mut curfile);
                ni = pop_exec_state(&mut currule, &mut source, 0 as *mut libc::c_long);
                if (*ni).opcode as libc::c_uint
                    == Op_K_getline as libc::c_int as libc::c_uint || curfile.is_null()
                {
                    if post_execute.is_some() {
                        post_execute.unwrap()(pc);
                    }
                    pc = ni;
                    continue;
                } else {
                    current_block = 3518619798157913413;
                }
            }
            91 => {
                let mut ret_0: libc::c_int = 0;
                ret_0 = nextfile(&mut curfile, 0 as libc::c_int != 0);
                if ret_0 < 0 as libc::c_int {
                    if post_execute.is_some() {
                        post_execute.unwrap()(pc);
                    }
                    pc = (*pc).d.di;
                    continue;
                } else if ret_0 == 0 as libc::c_int {
                    if post_execute.is_some() {
                        post_execute.unwrap()(pc);
                    }
                    pc = (*pc.offset(1 as libc::c_int as isize)).x.xi;
                    continue;
                } else {
                    push_exec_state(pc, currule, source, stack_ptr);
                    if curfile.is_null() {
                        if post_execute.is_some() {
                            post_execute.unwrap()(pc);
                        }
                        pc = (*pc).x.xi;
                        continue;
                    }
                }
                current_block = 3518619798157913413;
            }
            90 => {
                let mut errcode: libc::c_int = 0 as libc::c_int;
                ni = (*pc).d.di;
                if curfile.is_null() {
                    ni = (*ni).d.di;
                    if post_execute.is_some() {
                        post_execute.unwrap()(pc);
                    }
                    pc = ni;
                    continue;
                } else if !inrec(curfile, &mut errcode) {
                    if errcode > 0 as libc::c_int {
                        update_ERRNO_int(errcode);
                        if do_flags as libc::c_uint
                            & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0
                            || (*pc).x.xl == 0
                        {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    libc::c_int,
                                ) -> ())(
                                b"./interpret.h\0" as *const u8 as *const libc::c_char,
                                1433 as libc::c_int,
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
                                    b"error reading input file `%s': %s\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                (*curfile).public.name,
                                strerror(errcode),
                            );
                        }
                    }
                    if post_execute.is_some() {
                        post_execute.unwrap()(pc);
                    }
                    pc = ni;
                    continue;
                }
                current_block = 3518619798157913413;
            }
            67 => {
                let mut ret_1: libc::c_int = 0;
                if currule != Rule as libc::c_int && currule != BEGINFILE as libc::c_int
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const libc::c_char,
                        1448 as libc::c_int,
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
                            b"`nextfile' cannot be called from a `%s' rule\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        *ruletab.as_ptr().offset(currule as isize),
                    );
                }
                ret_1 = nextfile(&mut curfile, 1 as libc::c_int != 0);
                if currule == BEGINFILE as libc::c_int {
                    let mut stack_size: libc::c_long = 0 as libc::c_int as libc::c_long;
                    ni = pop_exec_state(&mut currule, &mut source, &mut stack_size);
                    unwind_stack(stack_size);
                    if ret_1 == 0 as libc::c_int {
                        if post_execute.is_some() {
                            post_execute.unwrap()(pc);
                        }
                        pc = ni;
                        continue;
                    } else {
                        push_exec_state(ni, currule, source, stack_ptr);
                        if post_execute.is_some() {
                            post_execute.unwrap()(pc);
                        }
                        pc = (*pc).x.xi;
                        continue;
                    }
                } else {
                    unwind_stack(0 as libc::c_int as libc::c_long);
                    push_exec_state((*pc).d.di, currule, source, stack_ptr);
                    if post_execute.is_some() {
                        post_execute.unwrap()(pc);
                    }
                    pc = (*pc).x.xi;
                    continue;
                }
            }
            60 => {
                if currule == 0 {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const libc::c_char,
                        1495 as libc::c_int,
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
                            b"`exit' cannot be called in the current context\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                exiting = 1 as libc::c_int != 0;
                t1 = force_number(POP_SCALAR());
                if t1 != Nnull_string {
                    exit_val = (*t1).sub.val.fltnum as libc::c_long as libc::c_int;
                }
                DEREF(t1);
                if currule == BEGINFILE as libc::c_int
                    || currule == ENDFILE as libc::c_int
                {
                    pop_exec_state(&mut currule, &mut source, 0 as *mut libc::c_long);
                }
                unwind_stack(0 as libc::c_int as libc::c_long);
                if currule == END as libc::c_int {
                    ni = (*pc).x.xi;
                } else {
                    ni = (*pc).d.di;
                }
                if post_execute.is_some() {
                    post_execute.unwrap()(pc);
                }
                pc = ni;
                continue;
            }
            59 => {
                if currule != Rule as libc::c_int {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const libc::c_char,
                        1536 as libc::c_int,
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
                            b"`next' cannot be called from a `%s' rule\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        *ruletab.as_ptr().offset(currule as isize),
                    );
                }
                unwind_stack(0 as libc::c_int as libc::c_long);
                if post_execute.is_some() {
                    post_execute.unwrap()(pc);
                }
                pc = (*pc).d.di;
                continue;
            }
            86 => {
                r = POP_SCALAR();
                DEREF(r);
                current_block = 3518619798157913413;
            }
            14 => {
                if (*pc).x.xl != 0 {
                    if post_execute.is_some() {
                        post_execute.unwrap()(pc);
                    }
                    pc = (*pc).d.di;
                    continue;
                } else {
                    current_block = 3518619798157913413;
                }
            }
            15 => {
                let mut result_0: libc::c_int = 0;
                let mut ip: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                t1 = TOP_SCALAR();
                di = (eval_condition(t1) as libc::c_int != 0 as libc::c_int)
                    as libc::c_int;
                DEREF(t1);
                ip = (*pc).x.xi;
                if (*ip).x.xl == 0 && di != 0 {
                    stack_ptr = stack_ptr.offset(-1);
                    stack_ptr;
                    (*ip).x.xl = 1 as libc::c_int as libc::c_long;
                    if post_execute.is_some() {
                        post_execute.unwrap()(pc);
                    }
                    pc = (*ip).d.di;
                    continue;
                } else {
                    result_0 = ((*ip).x.xl != 0 || di != 0) as libc::c_int;
                    (*ip).x.xl ^= di as libc::c_long;
                    r = node_Boolean[result_0 as usize];
                    (*r).valref += 1;
                    (*r).valref;
                    (*stack_ptr).rptr = r;
                    if post_execute.is_some() {
                        post_execute.unwrap()(pc);
                    }
                    pc = (*pc).d.di;
                    continue;
                }
            }
            103 => {
                if do_flags as libc::c_uint & DO_PROFILE as libc::c_int as libc::c_uint
                    != 0
                {
                    (*pc).d.ldl = ((*pc).d.ldl).wrapping_add(1);
                    (*pc).d.ldl;
                }
                current_block = 3518619798157913413;
            }
            85 | 112 | 115 | 113 | 114 | 116 | 53 | 117 | 118 | 120 | 102 | 121 => {
                current_block = 3518619798157913413;
            }
            _ => {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"./interpret.h\0" as *const u8 as *const libc::c_char,
                    1599 as libc::c_int,
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
                        b"Sorry, don't know how to interpret `%s'\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    opcode2str(op),
                );
                current_block = 3518619798157913413;
            }
        }
        match current_block {
            3432515699156962603 => {
                rp = re_update(m);
                di = research(
                    rp,
                    (*t1).sub.val.sp,
                    0 as libc::c_int,
                    (*t1).sub.val.slen,
                    0 as libc::c_int,
                );
                di = (di == -(1 as libc::c_int)) as libc::c_int
                    ^ (op as libc::c_uint != Op_nomatch as libc::c_int as libc::c_uint)
                        as libc::c_int;
                if op as libc::c_uint != Op_match_rec as libc::c_int as libc::c_uint {
                    stack_ptr = stack_ptr.offset(-1);
                    stack_ptr;
                    DEREF(t1);
                    if (*m).type_0 as libc::c_uint
                        == Node_dynregex as libc::c_int as libc::c_uint
                    {
                        DEREF((*m).sub.nodep.x.extra);
                        (*m).sub.nodep.x.extra = 0 as *mut exp_node;
                    }
                }
                r = node_Boolean[di as usize];
                (*r).valref += 1;
                (*r).valref;
                let ref mut fresh91 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh91 = r;
            }
            15749617915083522005 => {
                t1 = force_number(TOP_SCALAR());
                if x2 == 0 as libc::c_int as libc::c_double {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const libc::c_char,
                        664 as libc::c_int,
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
                            b"division by zero attempted in `%%'\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                x = fmod((*t1).sub.val.fltnum, x2);
                r = make_number.unwrap()(x);
                DEREF(t1);
                (*stack_ptr).rptr = r;
            }
            12664007985747274825 => {
                t1 = force_number(TOP_SCALAR());
                if x2 == 0 as libc::c_int as libc::c_double {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"./interpret.h\0" as *const u8 as *const libc::c_char,
                        648 as libc::c_int,
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
                            b"division by zero attempted\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                r = make_number.unwrap()((*t1).sub.val.fltnum / x2);
                DEREF(t1);
                (*stack_ptr).rptr = r;
            }
            17364872992865346463 => {
                t1 = force_number(TOP_SCALAR());
                r = make_number.unwrap()(calc_exp((*t1).sub.val.fltnum, x2));
                DEREF(t1);
                (*stack_ptr).rptr = r;
            }
            1804798499499804236 => {
                t1 = force_number(TOP_SCALAR());
                r = make_number.unwrap()((*t1).sub.val.fltnum * x2);
                DEREF(t1);
                (*stack_ptr).rptr = r;
            }
            4151678221539060801 => {
                t1 = force_number(TOP_SCALAR());
                r = make_number.unwrap()((*t1).sub.val.fltnum - x2);
                (*r)
                    .sub
                    .val
                    .fltnum = fix_nan_sign(
                    (*t1).sub.val.fltnum,
                    x2,
                    (*r).sub.val.fltnum,
                );
                DEREF(t1);
                (*stack_ptr).rptr = r;
            }
            13780331501197148875 => {
                t1 = force_number(TOP_SCALAR());
                r = make_number.unwrap()((*t1).sub.val.fltnum + x2);
                (*r)
                    .sub
                    .val
                    .fltnum = fix_nan_sign(
                    (*t1).sub.val.fltnum,
                    x2,
                    (*r).sub.val.fltnum,
                );
                DEREF(t1);
                (*stack_ptr).rptr = r;
            }
            7599496272070177918 => {
                source = (*pc).d.name;
            }
            11765585416638830173 => {
                let ref mut fresh68 = (*if stack_ptr < stack_top {
                    stack_ptr = stack_ptr.offset(1);
                    stack_ptr
                } else {
                    grow_stack()
                })
                    .rptr;
                *fresh68 = (*pc).d.dn;
            }
            _ => {}
        }
        if post_execute.is_some() {
            post_execute.unwrap()(pc);
        }
        pc = (*pc).nexti;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn init_interpret() {
    let mut newval: libc::c_long = 0;
    newval = getenv_long(b"GAWK_STACKSIZE\0" as *const u8 as *const libc::c_char);
    if newval > 0 as libc::c_int as libc::c_long {
        STACK_SIZE = newval as libc::c_ulong;
    }
    stack_bottom = emalloc_real(
        STACK_SIZE.wrapping_mul(::std::mem::size_of::<STACK_ITEM>() as libc::c_ulong),
        b"grow_stack\0" as *const u8 as *const libc::c_char,
        b"stack_bottom\0" as *const u8 as *const libc::c_char,
        b"eval.c\0" as *const u8 as *const libc::c_char,
        1860 as libc::c_int,
    ) as *mut STACK_ITEM;
    stack_ptr = stack_bottom.offset(-(1 as libc::c_int as isize));
    stack_top = stack_bottom
        .offset(STACK_SIZE as isize)
        .offset(-(1 as libc::c_int as isize));
    frame_ptr = nextfree[BLOCK_NODE as libc::c_int as usize].freep as *mut NODE;
    if !frame_ptr.is_null() {
        nextfree[BLOCK_NODE as libc::c_int as usize]
            .freep = (*(frame_ptr as *mut block_item)).freep;
    } else {
        frame_ptr = more_blocks(BLOCK_NODE as libc::c_int) as *mut NODE;
    };
    (*frame_ptr).type_0 = Node_frame;
    (*frame_ptr).sub.nodep.r.av = 0 as *mut *mut exp_node;
    (*frame_ptr).sub.nodep.x.extra = 0 as *mut exp_node;
    (*frame_ptr).sub.nodep.name = 0 as *mut libc::c_char;
    node_Boolean[0 as libc::c_int as usize] = make_number.unwrap()(0.0f64);
    node_Boolean[1 as libc::c_int as usize] = make_number.unwrap()(1.0f64);
    if 0 as libc::c_int == 0 {
        (*node_Boolean[0 as libc::c_int as usize])
            .flags = ::std::mem::transmute::<
            libc::c_uint,
            flagvals,
        >(
            (*node_Boolean[0 as libc::c_int as usize]).flags as libc::c_uint
                | NUMINT as libc::c_int as libc::c_uint,
        );
        (*node_Boolean[1 as libc::c_int as usize])
            .flags = ::std::mem::transmute::<
            libc::c_uint,
            flagvals,
        >(
            (*node_Boolean[1 as libc::c_int as usize]).flags as libc::c_uint
                | NUMINT as libc::c_int as libc::c_uint,
        );
    }
    if num_exec_hook > 0 as libc::c_int {
        interpret = Some(
            h_interpret as unsafe extern "C" fn(*mut INSTRUCTION) -> libc::c_int,
        );
    } else {
        interpret = Some(
            r_interpret as unsafe extern "C" fn(*mut INSTRUCTION) -> libc::c_int,
        );
    };
}
pub unsafe extern "C" fn elem_new_to_scalar(mut n: *mut NODE) -> *mut NODE {
    if (*n).type_0 as libc::c_uint != Node_elem_new as libc::c_int as libc::c_uint {
        return n;
    }
    if (*n).valref > 1 as libc::c_int as libc::c_long {
        unref(n);
        return dupnode(Nnull_string);
    }
    (*n).type_0 = Node_val;
    return n;
}
unsafe extern "C" fn fix_nan_sign(
    mut left: libc::c_double,
    mut right: libc::c_double,
    mut result: libc::c_double,
) -> libc::c_double {
    if left.is_nan() as i32 != 0 && left.is_sign_negative() as libc::c_int != 0 {
        return copysign(result, -1.0f64)
    } else if right.is_nan() as i32 != 0 && right.is_sign_negative() as libc::c_int != 0
    {
        return copysign(result, -1.0f64)
    } else {
        return result
    };
}
