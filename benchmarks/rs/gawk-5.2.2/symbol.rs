use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type re_dfa_t;
    pub type dfa;
    pub type break_point;
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
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    static mut PROCINFO_node: *mut NODE;
    static mut Nnull_string: *mut NODE;
    static mut sourceline: libc::c_int;
    static mut source: *mut libc::c_char;
    static mut nextfree: [block_header; 2];
    static mut srcfiles: *mut SRCFILE;
    static mut do_flags: do_flag_values;
    static mut using_persistent_malloc: bool;
    fn r_unref(tmp: *mut NODE);
    fn null_array(symbol: *mut NODE);
    fn assoc_list(
        symbol: *mut NODE,
        sort_str: *const libc::c_char,
        sort_ctxt: sort_context_t,
    ) -> *mut *mut NODE;
    fn valinfo(n: *mut NODE, print_func: Func_print, fp: *mut FILE);
    fn nodetype2str(type_0: NODETYPE) -> *const libc::c_char;
    fn opcode2str(type_0: OPCODE) -> *const libc::c_char;
    fn estrdup(str: *const libc::c_char, len: size_t) -> *mut libc::c_char;
    fn update_global_values();
    fn error(mesg: *const libc::c_char, _: ...);
    fn set_loc(file: *const libc::c_char, line: libc::c_int);
    fn r_fatal(mesg: *const libc::c_char, _: ...);
    fn r_dupnode(n: *mut NODE) -> *mut NODE;
    fn make_str_node(
        s: *const libc::c_char,
        len: size_t,
        flags: libc::c_int,
    ) -> *mut NODE;
    fn more_blocks(id: libc::c_int) -> *mut libc::c_void;
    fn refree(rp: *mut Regexp);
    static mut rule_list: *mut INSTRUCTION;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type time_t = __time_t;
pub type wchar_t = libc::c_int;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
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
    pub sub: C2RustUnnamed_0,
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
pub union C2RustUnnamed_0 {
    pub nodep: C2RustUnnamed_2,
    pub val: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
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
pub struct C2RustUnnamed_2 {
    pub l: C2RustUnnamed_9,
    pub r: C2RustUnnamed_4,
    pub x: C2RustUnnamed_3,
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
pub union C2RustUnnamed_3 {
    pub extra: *mut exp_node,
    pub aptr: Option::<unsafe extern "C" fn() -> ()>,
    pub xl: libc::c_long,
    pub cmnt: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
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
    pub d: C2RustUnnamed_6,
    pub x: C2RustUnnamed_5,
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
pub union C2RustUnnamed_5 {
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
pub union C2RustUnnamed_6 {
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
    pub hs: C2RustUnnamed_8,
    pub hi: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub next: *mut bucket_item,
    pub li: [libc::c_long; 2],
    pub val: [*mut exp_node; 2],
    pub cnt: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub next: *mut bucket_item,
    pub str_0: *mut libc::c_char,
    pub len: size_t,
    pub code: size_t,
    pub name: *mut exp_node,
    pub val: *mut exp_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
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
pub type INSTRUCTION = exp_instruction;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct instruction_block {
    pub next: *mut instruction_block,
    pub i: [INSTRUCTION; 126],
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
pub type sort_context_t = libc::c_uint;
pub const ASORTI: sort_context_t = 3;
pub const ASORT: sort_context_t = 2;
pub const SORTED_IN: sort_context_t = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct root_pointers {
    pub global_table: *mut NODE,
    pub func_table: *mut NODE,
    pub symbol_table: *mut NODE,
    pub nextfree: [block_header; 2],
    pub mpfr: libc::c_int,
    pub first: bool,
}
pub type SYMBOL_TYPE = libc::c_uint;
pub const VARIABLE: SYMBOL_TYPE = 2;
pub const FUNCTION: SYMBOL_TYPE = 1;
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
unsafe extern "C" fn in_array(mut a: *mut NODE, mut s: *mut NODE) -> *mut NODE {
    let mut ret: *mut *mut NODE = 0 as *mut *mut NODE;
    ret = ((*(*a).sub.nodep.l.lp).exists).unwrap()(a, s);
    return if !ret.is_null() { *ret } else { 0 as *mut NODE };
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
static mut symbol_list: *mut NODE = 0 as *const NODE as *mut NODE;
static mut install_func: Option::<unsafe extern "C" fn(*mut NODE) -> ()> = None;
static mut curr_ctxt: *mut AWK_CONTEXT = 0 as *const AWK_CONTEXT as *mut AWK_CONTEXT;
static mut ctxt_level: libc::c_int = 0;
static mut global_table: *mut NODE = 0 as *const NODE as *mut NODE;
static mut param_table: *mut NODE = 0 as *const NODE as *mut NODE;
pub static mut symbol_table: *mut NODE = 0 as *const NODE as *mut NODE;
pub static mut func_table: *mut NODE = 0 as *const NODE as *mut NODE;
static mut installing_specials: bool = 0 as libc::c_int != 0;
pub static mut root_pointers: *mut root_pointers = 0 as *const root_pointers
    as *mut root_pointers;
unsafe extern "C" fn init_the_tables() {
    global_table = nextfree[BLOCK_NODE as libc::c_int as usize].freep as *mut NODE;
    if !global_table.is_null() {
        nextfree[BLOCK_NODE as libc::c_int as usize]
            .freep = (*(global_table as *mut block_item)).freep;
    } else {
        global_table = more_blocks(BLOCK_NODE as libc::c_int) as *mut NODE;
    };
    memset(
        global_table as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<NODE>() as libc::c_ulong,
    );
    null_array(global_table);
    param_table = nextfree[BLOCK_NODE as libc::c_int as usize].freep as *mut NODE;
    if !param_table.is_null() {
        nextfree[BLOCK_NODE as libc::c_int as usize]
            .freep = (*(param_table as *mut block_item)).freep;
    } else {
        param_table = more_blocks(BLOCK_NODE as libc::c_int) as *mut NODE;
    };
    memset(
        param_table as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<NODE>() as libc::c_ulong,
    );
    null_array(param_table);
    installing_specials = 1 as libc::c_int != 0;
    func_table = install_symbol(
        estrdup(
            b"FUNCTAB\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as size_t,
        ),
        Node_var_array,
    );
    symbol_table = install_symbol(
        estrdup(
            b"SYMTAB\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as size_t,
        ),
        Node_var_array,
    );
    installing_specials = 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn init_symbol_table() {
    if !using_persistent_malloc {
        init_the_tables();
        return;
    }
    root_pointers = 0 as *mut libc::c_void as *mut root_pointers;
    if root_pointers.is_null() {
        init_the_tables();
        root_pointers = emalloc_real(
            ::std::mem::size_of::<root_pointers>() as libc::c_ulong,
            b"init_symbol_table\0" as *const u8 as *const libc::c_char,
            b"root_pointers\0" as *const u8 as *const libc::c_char,
            b"symbol.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
        ) as *mut root_pointers;
        memset(
            root_pointers as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<root_pointers>() as libc::c_ulong,
        );
        (*root_pointers).global_table = global_table;
        (*root_pointers).func_table = func_table;
        (*root_pointers).symbol_table = symbol_table;
        (*root_pointers).first = 1 as libc::c_int != 0;
        (*root_pointers).mpfr = 0 as libc::c_int;
    } else {
        global_table = (*root_pointers).global_table;
        func_table = (*root_pointers).func_table;
        symbol_table = (*root_pointers).symbol_table;
        memcpy(
            nextfree.as_mut_ptr() as *mut libc::c_void,
            ((*root_pointers).nextfree).as_mut_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[block_header; 2]>() as libc::c_ulong,
        );
        param_table = nextfree[BLOCK_NODE as libc::c_int as usize].freep as *mut NODE;
        if !param_table.is_null() {
            nextfree[BLOCK_NODE as libc::c_int as usize]
                .freep = (*(param_table as *mut block_item)).freep;
        } else {
            param_table = more_blocks(BLOCK_NODE as libc::c_int) as *mut NODE;
        };
        memset(
            param_table as *mut libc::c_void,
            '\0' as i32,
            ::std::mem::size_of::<NODE>() as libc::c_ulong,
        );
        null_array(param_table);
    };
}
pub unsafe extern "C" fn pma_mpfr_check() {
    if !using_persistent_malloc {
        return;
    }
    if (*root_pointers).first {
        (*root_pointers).first = 0 as libc::c_int != 0;
        (*root_pointers)
            .mpfr = (do_flags as libc::c_uint & DO_MPFR as libc::c_int as libc::c_uint)
            as libc::c_int;
        return;
    }
    if (*root_pointers).mpfr as libc::c_uint
        != do_flags as libc::c_uint & DO_MPFR as libc::c_int as libc::c_uint
    {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"symbol.c\0" as *const u8 as *const libc::c_char,
            137 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"current setting of -M/--bignum does not match saved setting in PMA backing file\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
}
pub unsafe extern "C" fn pma_save_free_lists() {
    if !using_persistent_malloc {
        return;
    }
    memcpy(
        ((*root_pointers).nextfree).as_mut_ptr() as *mut libc::c_void,
        nextfree.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[block_header; 2]>() as libc::c_ulong,
    );
}
pub unsafe extern "C" fn install_symbol(
    mut name: *const libc::c_char,
    mut type_0: NODETYPE,
) -> *mut NODE {
    return install(name, 0 as *mut NODE, type_0);
}
pub unsafe extern "C" fn lookup(mut name: *const libc::c_char) -> *mut NODE {
    let mut n: *mut NODE = 0 as *mut NODE;
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut tables: [*mut NODE; 5] = [0 as *mut NODE; 5];
    let mut i: libc::c_int = 0;
    tables[0 as libc::c_int as usize] = param_table;
    tables[1 as libc::c_int as usize] = global_table;
    tables[2 as libc::c_int as usize] = func_table;
    tables[3 as libc::c_int as usize] = symbol_table;
    tables[4 as libc::c_int as usize] = 0 as *mut NODE;
    tmp = get_name_from_awk_ns(name);
    n = 0 as *mut NODE;
    i = 0 as libc::c_int;
    while !(tables[i as usize]).is_null() {
        if !((*tables[i as usize]).sub.nodep.reflags as libc::c_uint
            == 0 as libc::c_int as libc::c_uint)
        {
            if !((do_flags as libc::c_uint & DO_POSIX as libc::c_int as libc::c_uint != 0
                || do_flags as libc::c_uint
                    & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0)
                && tables[i as usize] == global_table)
            {
                n = in_array(tables[i as usize], tmp);
                if !n.is_null() {
                    break;
                }
            }
        }
        i += 1;
        i;
    }
    unref(tmp);
    if n.is_null()
        || (*n).type_0 as libc::c_uint == Node_val as libc::c_int as libc::c_uint
    {
        return 0 as *mut NODE;
    }
    return n;
}
pub unsafe extern "C" fn make_params(
    mut pnames: *mut *mut libc::c_char,
    mut pcount: libc::c_int,
) -> *mut NODE {
    let mut p: *mut NODE = 0 as *mut NODE;
    let mut parms: *mut NODE = 0 as *mut NODE;
    let mut i: libc::c_int = 0;
    if pcount <= 0 as libc::c_int || pnames.is_null() {
        return 0 as *mut NODE;
    }
    parms = ezalloc_real(
        (pcount as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<NODE>() as libc::c_ulong),
        b"make_params\0" as *const u8 as *const libc::c_char,
        b"parms\0" as *const u8 as *const libc::c_char,
        b"symbol.c\0" as *const u8 as *const libc::c_char,
        218 as libc::c_int,
    ) as *mut NODE;
    i = 0 as libc::c_int;
    p = parms;
    while i < pcount {
        (*p).type_0 = Node_param_list;
        (*p).sub.nodep.name = *pnames.offset(i as isize);
        (*p).sub.nodep.l.ll = i as libc::c_long;
        i += 1;
        i;
        p = p.offset(1);
        p;
    }
    return parms;
}
pub unsafe extern "C" fn install_params(mut func: *mut NODE) {
    let mut i: libc::c_int = 0;
    let mut pcount: libc::c_int = 0;
    let mut parms: *mut NODE = 0 as *mut NODE;
    if func.is_null() {
        return;
    }
    pcount = (*func).sub.nodep.l.ll as libc::c_int;
    if pcount <= 0 as libc::c_int
        || {
            parms = (*func).sub.nodep.rn;
            parms.is_null()
        }
    {
        return;
    }
    i = 0 as libc::c_int;
    while i < pcount {
        install(
            (*parms.offset(i as isize)).sub.nodep.name,
            parms.offset(i as isize),
            Node_param_list,
        );
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn remove_params(mut func: *mut NODE) {
    let mut parms: *mut NODE = 0 as *mut NODE;
    let mut p: *mut NODE = 0 as *mut NODE;
    let mut i: libc::c_int = 0;
    let mut pcount: libc::c_int = 0;
    if func.is_null() {
        return;
    }
    pcount = (*func).sub.nodep.l.ll as libc::c_int;
    if pcount <= 0 as libc::c_int
        || {
            parms = (*func).sub.nodep.rn;
            parms.is_null()
        }
    {
        return;
    }
    i = pcount - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        let mut tmp: *mut NODE = 0 as *mut NODE;
        let mut tmp2: *mut NODE = 0 as *mut NODE;
        p = parms.offset(i as isize);
        tmp = make_str_node(
            (*p).sub.nodep.name,
            strlen((*p).sub.nodep.name),
            0 as libc::c_int,
        );
        tmp2 = in_array(param_table, tmp);
        if !tmp2.is_null() && !((*tmp2).sub.nodep.r.rptr).is_null() {
            (*tmp2).sub.nodep.r.rptr = (*(*tmp2).sub.nodep.r.rptr).sub.nodep.r.rptr;
        } else {
            ((*(*param_table).sub.nodep.l.lp).remove).unwrap()(param_table, tmp);
        }
        unref(tmp);
        i -= 1;
        i;
    }
    ((*(*param_table).sub.nodep.l.lp).clear).unwrap()(param_table, 0 as *mut exp_node);
}
pub unsafe extern "C" fn remove_symbol(mut r: *mut NODE) -> *mut NODE {
    let mut n: *mut NODE = in_array(symbol_table, r);
    if n.is_null() {
        return n;
    }
    n = dupnode(n);
    ((*(*symbol_table).sub.nodep.l.lp).remove).unwrap()(symbol_table, r);
    return n;
}
pub unsafe extern "C" fn destroy_symbol(mut r: *mut NODE) {
    r = remove_symbol(r);
    if r.is_null() {
        return;
    }
    match (*r).type_0 as libc::c_uint {
        9 => {
            if (*r).sub.nodep.l.ll > 0 as libc::c_int as libc::c_long {
                let mut n: *mut NODE = 0 as *mut NODE;
                let mut i: libc::c_int = 0;
                let mut pcount: libc::c_int = (*r).sub.nodep.l.ll as libc::c_int;
                i = 0 as libc::c_int;
                while i < pcount {
                    n = ((*r).sub.nodep.rn).offset(i as isize);
                    free((*n).sub.nodep.name as *mut libc::c_void);
                    i += 1;
                    i;
                }
                free((*r).sub.nodep.rn as *mut libc::c_void);
            }
        }
        10 => {
            bcfree((*r).sub.nodep.r.iptr);
        }
        5 => {
            ((*(*r).sub.nodep.l.lp).clear).unwrap()(r, 0 as *mut exp_node);
        }
        4 => {
            unref((*r).sub.nodep.l.lptr);
        }
        _ => {}
    }
    free((*r).sub.nodep.name as *mut libc::c_void);
    let ref mut fresh0 = (*(r as *mut block_item)).freep;
    *fresh0 = nextfree[BLOCK_NODE as libc::c_int as usize].freep;
    nextfree[BLOCK_NODE as libc::c_int as usize].freep = r as *mut block_item;
}
unsafe extern "C" fn make_symbol(
    mut name: *const libc::c_char,
    mut type_0: NODETYPE,
) -> *mut NODE {
    let mut r: *mut NODE = 0 as *mut NODE;
    r = nextfree[BLOCK_NODE as libc::c_int as usize].freep as *mut NODE;
    if !r.is_null() {
        nextfree[BLOCK_NODE as libc::c_int as usize]
            .freep = (*(r as *mut block_item)).freep;
    } else {
        r = more_blocks(BLOCK_NODE as libc::c_int) as *mut NODE;
    };
    memset(
        r as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<NODE>() as libc::c_ulong,
    );
    if type_0 as libc::c_uint == Node_var_array as libc::c_int as libc::c_uint {
        null_array(r);
    } else if type_0 as libc::c_uint == Node_var as libc::c_int as libc::c_uint {
        (*r).sub.nodep.l.lptr = dupnode(Nnull_string);
    }
    (*r).sub.nodep.name = name as *mut libc::c_char;
    (*r).type_0 = type_0;
    (*r).valref = 1 as libc::c_int as libc::c_long;
    return r;
}
unsafe extern "C" fn install(
    mut name: *const libc::c_char,
    mut parm: *mut NODE,
    mut type_0: NODETYPE,
) -> *mut NODE {
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut table: *mut NODE = 0 as *mut NODE;
    let mut n_name: *mut NODE = 0 as *mut NODE;
    let mut prev: *mut NODE = 0 as *mut NODE;
    n_name = get_name_from_awk_ns(name);
    table = symbol_table;
    if type_0 as libc::c_uint == Node_param_list as libc::c_int as libc::c_uint {
        table = param_table;
    } else if type_0 as libc::c_uint == Node_func as libc::c_int as libc::c_uint
        || type_0 as libc::c_uint == Node_ext_func as libc::c_int as libc::c_uint
        || type_0 as libc::c_uint == Node_builtin_func as libc::c_int as libc::c_uint
    {
        table = func_table;
    } else if installing_specials {
        table = global_table;
    }
    if !parm.is_null() {
        r = parm;
    } else {
        r = make_symbol(name, type_0);
    }
    let mut current_block_19: u64;
    if type_0 as libc::c_uint == Node_param_list as libc::c_int as libc::c_uint {
        prev = in_array(table, n_name);
        if prev.is_null() {
            current_block_19 = 16437787608536535772;
        } else {
            (*r).sub.nodep.r.rptr = (*prev).sub.nodep.r.rptr;
            (*prev).sub.nodep.r.rptr = r;
            unref(n_name);
            current_block_19 = 2838571290723028321;
        }
    } else {
        current_block_19 = 16437787608536535772;
    }
    match current_block_19 {
        16437787608536535772 => {
            assoc_set(table, n_name, r);
        }
        _ => {}
    }
    if install_func.is_some() {
        (Some(install_func.unwrap())).unwrap()(r);
    }
    return r;
}
unsafe extern "C" fn comp_symbol(
    mut v1: *const libc::c_void,
    mut v2: *const libc::c_void,
) -> libc::c_int {
    let mut npp1: *const *const NODE = 0 as *const *const NODE;
    let mut npp2: *const *const NODE = 0 as *const *const NODE;
    let mut n1: *const NODE = 0 as *const NODE;
    let mut n2: *const NODE = 0 as *const NODE;
    npp1 = v1 as *const *const NODE;
    npp2 = v2 as *const *const NODE;
    n1 = *npp1;
    n2 = *npp2;
    let mut n1_is_in_ns: bool = !(strchr((*n1).sub.nodep.name, ':' as i32)).is_null();
    let mut n2_is_in_ns: bool = !(strchr((*n2).sub.nodep.name, ':' as i32)).is_null();
    if n1_is_in_ns as libc::c_int != 0 && n2_is_in_ns as libc::c_int != 0 {
        return strcmp((*n1).sub.nodep.name, (*n2).sub.nodep.name)
    } else if n1_is_in_ns as libc::c_int != 0 && !n2_is_in_ns {
        return 1 as libc::c_int
    } else if !n1_is_in_ns && n2_is_in_ns as libc::c_int != 0 {
        return -(1 as libc::c_int)
    } else {
        return strcmp((*n1).sub.nodep.name, (*n2).sub.nodep.name)
    };
}
unsafe extern "C" fn get_symbols(
    mut what: SYMBOL_TYPE,
    mut sort: bool,
) -> *mut *mut NODE {
    let mut i: libc::c_int = 0;
    let mut table: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut list: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut count: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut max: libc::c_long = 0;
    let mut the_table: *mut NODE = 0 as *mut NODE;
    if what as libc::c_uint == FUNCTION as libc::c_int as libc::c_uint {
        the_table = func_table;
        max = ((*the_table).sub.nodep.reflags as libc::c_uint)
            .wrapping_mul(2 as libc::c_int as libc::c_uint) as libc::c_long;
        list = assoc_list(
            the_table,
            b"@unsorted\0" as *const u8 as *const libc::c_char,
            ASORTI,
        );
        table = emalloc_real(
            (((*the_table).sub.nodep.reflags as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut NODE>() as libc::c_ulong),
            b"get_symbols\0" as *const u8 as *const libc::c_char,
            b"table\0" as *const u8 as *const libc::c_char,
            b"symbol.c\0" as *const u8 as *const libc::c_char,
            483 as libc::c_int,
        ) as *mut *mut NODE;
        count = 0 as libc::c_int as libc::c_long;
        i = count as libc::c_int;
        while (i as libc::c_long) < max {
            r = *list.offset((i + 1 as libc::c_int) as isize);
            if !((*r).type_0 as libc::c_uint
                == Node_ext_func as libc::c_int as libc::c_uint
                || (*r).type_0 as libc::c_uint
                    == Node_builtin_func as libc::c_int as libc::c_uint)
            {
                let fresh1 = count;
                count = count + 1;
                let ref mut fresh2 = *table.offset(fresh1 as isize);
                *fresh2 = r;
            }
            i += 2 as libc::c_int;
        }
    } else {
        update_global_values();
        the_table = symbol_table;
        max = ((*the_table).sub.nodep.reflags as libc::c_uint)
            .wrapping_mul(2 as libc::c_int as libc::c_uint) as libc::c_long;
        list = assoc_list(
            the_table,
            b"@unsorted\0" as *const u8 as *const libc::c_char,
            ASORTI,
        );
        table = emalloc_real(
            (((*the_table).sub.nodep.reflags as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut NODE>() as libc::c_ulong),
            b"get_symbols\0" as *const u8 as *const libc::c_char,
            b"table\0" as *const u8 as *const libc::c_char,
            b"symbol.c\0" as *const u8 as *const libc::c_char,
            500 as libc::c_int,
        ) as *mut *mut NODE;
        count = 0 as libc::c_int as libc::c_long;
        i = count as libc::c_int;
        while (i as libc::c_long) < max {
            r = *list.offset((i + 1 as libc::c_int) as isize);
            if !((*r).type_0 as libc::c_uint == Node_val as libc::c_int as libc::c_uint)
            {
                let fresh3 = count;
                count = count + 1;
                let ref mut fresh4 = *table.offset(fresh3 as isize);
                *fresh4 = r;
            }
            i += 2 as libc::c_int;
        }
        let fresh5 = count;
        count = count + 1;
        let ref mut fresh6 = *table.offset(fresh5 as isize);
        *fresh6 = func_table;
        let fresh7 = count;
        count = count + 1;
        let ref mut fresh8 = *table.offset(fresh7 as isize);
        *fresh8 = symbol_table;
    }
    free(list as *mut libc::c_void);
    if sort as libc::c_int != 0 && count > 1 as libc::c_int as libc::c_long {
        qsort(
            table as *mut libc::c_void,
            count as size_t,
            ::std::mem::size_of::<*mut NODE>() as libc::c_ulong,
            Some(
                comp_symbol
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
    }
    let ref mut fresh9 = *table.offset(count as isize);
    *fresh9 = 0 as *mut NODE;
    return table;
}
pub unsafe extern "C" fn variable_list() -> *mut *mut NODE {
    return get_symbols(VARIABLE, 1 as libc::c_int != 0);
}
pub unsafe extern "C" fn function_list(mut sort: bool) -> *mut *mut NODE {
    return get_symbols(FUNCTION, sort);
}
pub unsafe extern "C" fn print_vars(
    mut table: *mut *mut NODE,
    mut print_func: Option::<
        unsafe extern "C" fn(*mut FILE, *const libc::c_char, ...) -> libc::c_int,
    >,
    mut fp: *mut FILE,
) {
    let mut i: libc::c_int = 0;
    let mut r: *mut NODE = 0 as *mut NODE;
    i = 0 as libc::c_int;
    loop {
        r = *table.offset(i as isize);
        if r.is_null() {
            break;
        }
        if !((*r).type_0 as libc::c_uint == Node_func as libc::c_int as libc::c_uint
            || (*r).type_0 as libc::c_uint
                == Node_ext_func as libc::c_int as libc::c_uint)
        {
            print_func
                .unwrap()(
                fp,
                b"%s: \0" as *const u8 as *const libc::c_char,
                (*r).sub.nodep.name,
            );
            if (*r).type_0 as libc::c_uint
                == Node_var_array as libc::c_int as libc::c_uint
            {
                print_func
                    .unwrap()(
                    fp,
                    b"array, %ld elements\n\0" as *const u8 as *const libc::c_char,
                    (*r).sub.nodep.reflags as libc::c_uint,
                );
            } else if (*r).type_0 as libc::c_uint
                == Node_var_new as libc::c_int as libc::c_uint
            {
                print_func
                    .unwrap()(
                    fp,
                    b"untyped variable\n\0" as *const u8 as *const libc::c_char,
                );
            } else if (*r).type_0 as libc::c_uint
                == Node_var as libc::c_int as libc::c_uint
            {
                valinfo((*r).sub.nodep.l.lptr, print_func, fp);
            } else {
                r_fatal(
                    b"internal error: file %s, line %d: unexpected node type: %s\0"
                        as *const u8 as *const libc::c_char,
                    b"symbol.c\0" as *const u8 as *const libc::c_char,
                    559 as libc::c_int,
                    nodetype2str((*r).type_0),
                );
            }
        }
        i += 1;
        i;
    };
}
pub unsafe extern "C" fn foreach_func(
    mut table: *mut *mut NODE,
    mut pfunc: Option::<
        unsafe extern "C" fn(*mut INSTRUCTION, *mut libc::c_void) -> libc::c_int,
    >,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut ret: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    loop {
        r = *table.offset(i as isize);
        if r.is_null() {
            break;
        }
        ret = pfunc.unwrap()((*r).sub.nodep.r.iptr, data);
        if ret != 0 as libc::c_int {
            break;
        }
        i += 1;
        i;
    }
    return ret;
}
pub unsafe extern "C" fn release_all_vars() {
    ((*(*symbol_table).sub.nodep.l.lp).clear).unwrap()(symbol_table, 0 as *mut exp_node);
    ((*(*func_table).sub.nodep.l.lp).clear).unwrap()(func_table, 0 as *mut exp_node);
    ((*(*global_table).sub.nodep.l.lp).clear).unwrap()(global_table, 0 as *mut exp_node);
}
pub unsafe extern "C" fn append_symbol(mut r: *mut NODE) {
    let mut p: *mut NODE = 0 as *mut NODE;
    p = nextfree[BLOCK_NODE as libc::c_int as usize].freep as *mut NODE;
    if !p.is_null() {
        nextfree[BLOCK_NODE as libc::c_int as usize]
            .freep = (*(p as *mut block_item)).freep;
    } else {
        p = more_blocks(BLOCK_NODE as libc::c_int) as *mut NODE;
    };
    (*p).sub.nodep.l.lptr = r;
    (*p).sub.nodep.r.rptr = (*symbol_list).sub.nodep.r.rptr;
    (*symbol_list).sub.nodep.r.rptr = p;
}
pub unsafe extern "C" fn release_symbols(
    mut symlist: *mut NODE,
    mut keep_globals: libc::c_int,
) {
    let mut p: *mut NODE = 0 as *mut NODE;
    let mut next: *mut NODE = 0 as *mut NODE;
    p = (*symlist).sub.nodep.r.rptr;
    while !p.is_null() {
        if keep_globals == 0 {
            destroy_symbol((*p).sub.nodep.l.lptr);
        }
        next = (*p).sub.nodep.r.rptr;
        let ref mut fresh10 = (*(p as *mut block_item)).freep;
        *fresh10 = nextfree[BLOCK_NODE as libc::c_int as usize].freep;
        nextfree[BLOCK_NODE as libc::c_int as usize].freep = p as *mut block_item;
        p = next;
    }
    (*symlist).sub.nodep.r.rptr = 0 as *mut exp_node;
}
pub unsafe extern "C" fn load_symbols() {
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut sym_array: *mut NODE = 0 as *mut NODE;
    let mut aptr: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut max: libc::c_long = 0;
    let mut user: *mut NODE = 0 as *mut NODE;
    let mut extension: *mut NODE = 0 as *mut NODE;
    let mut untyped: *mut NODE = 0 as *mut NODE;
    let mut scalar: *mut NODE = 0 as *mut NODE;
    let mut array: *mut NODE = 0 as *mut NODE;
    let mut built_in: *mut NODE = 0 as *mut NODE;
    let mut list: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut tables: [*mut NODE; 4] = [0 as *mut NODE; 4];
    if PROCINFO_node.is_null() {
        return;
    }
    tables[0 as libc::c_int as usize] = func_table;
    tables[1 as libc::c_int as usize] = symbol_table;
    tables[2 as libc::c_int as usize] = global_table;
    tables[3 as libc::c_int as usize] = 0 as *mut NODE;
    tmp = make_str_node(
        b"identifiers\0" as *const u8 as *const libc::c_char,
        11 as libc::c_int as size_t,
        0 as libc::c_int,
    );
    aptr = ((*(*PROCINFO_node).sub.nodep.l.lp).lookup).unwrap()(PROCINFO_node, tmp);
    sym_array = nextfree[BLOCK_NODE as libc::c_int as usize].freep as *mut NODE;
    if !sym_array.is_null() {
        nextfree[BLOCK_NODE as libc::c_int as usize]
            .freep = (*(sym_array as *mut block_item)).freep;
    } else {
        sym_array = more_blocks(BLOCK_NODE as libc::c_int) as *mut NODE;
    };
    memset(
        sym_array as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<NODE>() as libc::c_ulong,
    );
    null_array(sym_array);
    unref(tmp);
    unref(*aptr);
    *aptr = sym_array;
    (*sym_array).sub.nodep.x.extra = PROCINFO_node;
    (*sym_array)
        .sub
        .nodep
        .name = estrdup(
        b"identifiers\0" as *const u8 as *const libc::c_char,
        11 as libc::c_int as size_t,
    );
    user = make_str_node(
        b"user\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as size_t,
        0 as libc::c_int,
    );
    extension = make_str_node(
        b"extension\0" as *const u8 as *const libc::c_char,
        9 as libc::c_int as size_t,
        0 as libc::c_int,
    );
    scalar = make_str_node(
        b"scalar\0" as *const u8 as *const libc::c_char,
        6 as libc::c_int as size_t,
        0 as libc::c_int,
    );
    untyped = make_str_node(
        b"untyped\0" as *const u8 as *const libc::c_char,
        7 as libc::c_int as size_t,
        0 as libc::c_int,
    );
    array = make_str_node(
        b"array\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int as size_t,
        0 as libc::c_int,
    );
    built_in = make_str_node(
        b"builtin\0" as *const u8 as *const libc::c_char,
        7 as libc::c_int as size_t,
        0 as libc::c_int,
    );
    i = 0 as libc::c_int as libc::c_long;
    while !(tables[i as usize]).is_null() {
        list = assoc_list(
            tables[i as usize],
            b"@unsorted\0" as *const u8 as *const libc::c_char,
            ASORTI,
        );
        max = ((*tables[i as usize]).sub.nodep.reflags as libc::c_uint)
            .wrapping_mul(2 as libc::c_int as libc::c_uint) as libc::c_long;
        if !(max == 0 as libc::c_int as libc::c_long) {
            j = 0 as libc::c_int as libc::c_long;
            while j < max {
                r = *list.offset((j + 1 as libc::c_int as libc::c_long) as isize);
                if (*r).type_0 as libc::c_uint
                    == Node_ext_func as libc::c_int as libc::c_uint
                    || (*r).type_0 as libc::c_uint
                        == Node_func as libc::c_int as libc::c_uint
                    || (*r).type_0 as libc::c_uint
                        == Node_builtin_func as libc::c_int as libc::c_uint
                    || (*r).type_0 as libc::c_uint
                        == Node_var as libc::c_int as libc::c_uint
                    || (*r).type_0 as libc::c_uint
                        == Node_var_array as libc::c_int as libc::c_uint
                    || (*r).type_0 as libc::c_uint
                        == Node_var_new as libc::c_int as libc::c_uint
                {
                    if strncmp(
                        (*r).sub.nodep.name,
                        b"awk::\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        tmp = make_str_node(
                            ((*r).sub.nodep.name).offset(5 as libc::c_int as isize),
                            (strlen((*r).sub.nodep.name))
                                .wrapping_sub(5 as libc::c_int as libc::c_ulong),
                            0 as libc::c_int,
                        );
                    } else {
                        tmp = make_str_node(
                            (*r).sub.nodep.name,
                            strlen((*r).sub.nodep.name),
                            0 as libc::c_int,
                        );
                    }
                    aptr = ((*(*sym_array).sub.nodep.l.lp).lookup)
                        .unwrap()(sym_array, tmp);
                    unref(tmp);
                    unref(*aptr);
                    match (*r).type_0 as libc::c_uint {
                        10 => {
                            *aptr = dupnode(extension);
                        }
                        9 => {
                            *aptr = dupnode(user);
                        }
                        11 => {
                            *aptr = dupnode(built_in);
                        }
                        4 => {
                            *aptr = dupnode(scalar);
                        }
                        5 => {
                            *aptr = dupnode(array);
                        }
                        6 => {
                            *aptr = dupnode(untyped);
                        }
                        _ => {
                            r_fatal(
                                b"internal error: file %s, line %d: unexpected node type %s\0"
                                    as *const u8 as *const libc::c_char,
                                b"symbol.c\0" as *const u8 as *const libc::c_char,
                                712 as libc::c_int,
                                nodetype2str((*r).type_0),
                            );
                        }
                    }
                }
                j += 2 as libc::c_int as libc::c_long;
            }
            free(list as *mut libc::c_void);
        }
        i += 1;
        i;
    }
    unref(user);
    unref(extension);
    unref(scalar);
    unref(untyped);
    unref(array);
    unref(built_in);
}
pub unsafe extern "C" fn check_param_names() -> bool {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut list: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut f: *mut NODE = 0 as *mut NODE;
    let mut max: libc::c_long = 0;
    let mut result: bool = 1 as libc::c_int != 0;
    let mut n: NODE = NODE {
        sub: C2RustUnnamed_0 {
            nodep: C2RustUnnamed_2 {
                l: C2RustUnnamed_9 {
                    lptr: 0 as *mut exp_node,
                },
                r: C2RustUnnamed_4 {
                    rptr: 0 as *mut exp_node,
                },
                x: C2RustUnnamed_3 {
                    extra: 0 as *mut exp_node,
                },
                name: 0 as *mut libc::c_char,
                reserved: 0,
                rn: 0 as *mut exp_node,
                cnt: 0,
                reflags: 0 as reflagvals,
            },
        },
        type_0: Node_illegal,
        flags: 0 as flagvals,
        valref: 0,
    };
    if (*func_table).sub.nodep.reflags as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        return result;
    }
    max = ((*func_table).sub.nodep.reflags as libc::c_uint)
        .wrapping_mul(2 as libc::c_int as libc::c_uint) as libc::c_long;
    memset(
        &mut n as *mut NODE as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<NODE>() as libc::c_ulong,
    );
    n.type_0 = Node_val;
    n.flags = (STRING as libc::c_int | STRCUR as libc::c_int) as flagvals;
    n.sub.val.idx = -(1 as libc::c_int);
    list = assoc_list(
        func_table,
        b"@unsorted\0" as *const u8 as *const libc::c_char,
        ASORTI,
    );
    i = 0 as libc::c_int;
    while (i as libc::c_long) < max {
        f = *list.offset((i + 1 as libc::c_int) as isize);
        if !((*f).type_0 as libc::c_uint
            == Node_builtin_func as libc::c_int as libc::c_uint
            || (*f).sub.nodep.l.ll == 0 as libc::c_int as libc::c_long)
        {
            j = 0 as libc::c_int;
            while (j as libc::c_long) < (*f).sub.nodep.l.ll {
                n.sub.val.sp = (*((*f).sub.nodep.rn).offset(j as isize)).sub.nodep.name;
                n
                    .sub
                    .val
                    .slen = strlen(
                    (*((*f).sub.nodep.rn).offset(j as isize)).sub.nodep.name,
                );
                if !(in_array(func_table, &mut n)).is_null() {
                    error(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"function `%s': cannot use function `%s' as a parameter name\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (**list.offset(i as isize)).sub.val.sp,
                        (*((*f).sub.nodep.rn).offset(j as isize)).sub.nodep.name,
                    );
                    result = 0 as libc::c_int != 0;
                }
                j += 1;
                j;
            }
        }
        i += 2 as libc::c_int;
    }
    free(list as *mut libc::c_void);
    return result;
}
static mut pools: *mut INSTRUCTION_POOL = 0 as *const INSTRUCTION_POOL
    as *mut INSTRUCTION_POOL;
pub unsafe extern "C" fn bcfree(mut cp: *mut INSTRUCTION) {
    (*cp).opcode = Op_illegal;
    (*cp)
        .nexti = (*pools)
        .pool[((*cp).pool_size as libc::c_int - 1 as libc::c_int) as usize]
        .free_list;
    (*pools)
        .pool[((*cp).pool_size as libc::c_int - 1 as libc::c_int) as usize]
        .free_list = cp;
}
pub unsafe extern "C" fn bcalloc(
    mut op: OPCODE,
    mut size: libc::c_int,
    mut srcline: libc::c_int,
) -> *mut INSTRUCTION {
    let mut cp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut pool: *mut instruction_mem_pool = 0 as *mut instruction_mem_pool;
    pool = &mut *((*pools).pool).as_mut_ptr().offset((size - 1 as libc::c_int) as isize)
        as *mut instruction_mem_pool;
    if !((*pool).free_list).is_null() {
        cp = (*pool).free_list;
        (*pool).free_list = (*cp).nexti;
    } else if !((*pool).free_space).is_null()
        && ((*pool).free_space).offset(size as isize)
            <= &mut *((*(*pool).block_list).i)
                .as_mut_ptr()
                .offset(
                    (2 as libc::c_int * 3 as libc::c_int * 21 as libc::c_int) as isize,
                ) as *mut INSTRUCTION
    {
        cp = (*pool).free_space;
        (*pool).free_space = ((*pool).free_space).offset(size as isize);
    } else {
        let mut block: *mut instruction_block = 0 as *mut instruction_block;
        block = emalloc_real(
            ::std::mem::size_of::<instruction_block>() as libc::c_ulong,
            b"bcalloc\0" as *const u8 as *const libc::c_char,
            b"block\0" as *const u8 as *const libc::c_char,
            b"symbol.c\0" as *const u8 as *const libc::c_char,
            837 as libc::c_int,
        ) as *mut instruction_block;
        (*block).next = (*pool).block_list;
        (*pool).block_list = block;
        cp = &mut *((*block).i).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut INSTRUCTION;
        (*pool)
            .free_space = &mut *((*block).i).as_mut_ptr().offset(size as isize)
            as *mut INSTRUCTION;
    }
    memset(
        cp as *mut libc::c_void,
        0 as libc::c_int,
        (size as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<INSTRUCTION>() as libc::c_ulong),
    );
    (*cp).pool_size = size as libc::c_short;
    (*cp).opcode = op;
    (*cp).source_line = srcline as libc::c_short;
    return cp;
}
pub unsafe extern "C" fn new_context() -> *mut AWK_CONTEXT {
    let mut ctxt: *mut AWK_CONTEXT = 0 as *mut AWK_CONTEXT;
    ctxt = ezalloc_real(
        ::std::mem::size_of::<AWK_CONTEXT>() as libc::c_ulong,
        b"new_context\0" as *const u8 as *const libc::c_char,
        b"ctxt\0" as *const u8 as *const libc::c_char,
        b"symbol.c\0" as *const u8 as *const libc::c_char,
        858 as libc::c_int,
    ) as *mut AWK_CONTEXT;
    (*ctxt).srcfiles.prev = &mut (*ctxt).srcfiles;
    (*ctxt).srcfiles.next = (*ctxt).srcfiles.prev;
    (*ctxt).rule_list.opcode = Op_list;
    (*ctxt).rule_list.d.di = &mut (*ctxt).rule_list;
    return ctxt;
}
unsafe extern "C" fn set_context(mut ctxt: *mut AWK_CONTEXT) {
    pools = &mut (*ctxt).pools;
    symbol_list = &mut (*ctxt).symbols;
    srcfiles = &mut (*ctxt).srcfiles;
    rule_list = &mut (*ctxt).rule_list;
    install_func = (*ctxt).install_func;
    curr_ctxt = ctxt;
}
pub unsafe extern "C" fn push_context(mut ctxt: *mut AWK_CONTEXT) {
    (*ctxt).prev = curr_ctxt;
    if !curr_ctxt.is_null() {
        (*curr_ctxt).sourceline = sourceline;
        (*curr_ctxt).source = source;
    }
    sourceline = 0 as libc::c_int;
    source = 0 as *mut libc::c_char;
    set_context(ctxt);
    ctxt_level += 1;
    ctxt_level;
}
pub unsafe extern "C" fn pop_context() {
    let mut ctxt: *mut AWK_CONTEXT = 0 as *mut AWK_CONTEXT;
    if ((*curr_ctxt).prev).is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"symbol.c\0" as *const u8 as *const libc::c_char,
            910 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"cannot pop main context\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    ctxt = (*curr_ctxt).prev;
    sourceline = (*ctxt).sourceline;
    source = (*ctxt).source;
    set_context(ctxt);
    ctxt_level -= 1;
    ctxt_level;
}
pub unsafe extern "C" fn in_main_context() -> libc::c_int {
    return (ctxt_level == 1 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn free_context(
    mut ctxt: *mut AWK_CONTEXT,
    mut keep_globals: bool,
) {
    let mut s: *mut SRCFILE = 0 as *mut SRCFILE;
    let mut sn: *mut SRCFILE = 0 as *mut SRCFILE;
    if ctxt.is_null() {
        return;
    }
    free_bcpool(&mut (*ctxt).pools);
    release_symbols(&mut (*ctxt).symbols, keep_globals as libc::c_int);
    s = &mut (*ctxt).srcfiles;
    while s != &mut (*ctxt).srcfiles as *mut SRCFILE {
        sn = (*s).next;
        if (*s).stype as libc::c_uint != SRC_CMDLINE as libc::c_int as libc::c_uint
            && (*s).stype as libc::c_uint != SRC_STDIN as libc::c_int as libc::c_uint
        {
            free((*s).fullpath as *mut libc::c_void);
        }
        free((*s).src as *mut libc::c_void);
        free(s as *mut libc::c_void);
        s = sn;
    }
    free(ctxt as *mut libc::c_void);
}
unsafe extern "C" fn free_bc_internal(mut cp: *mut INSTRUCTION) {
    let mut m: *mut NODE = 0 as *mut NODE;
    match (*cp).opcode as libc::c_uint {
        73 => {
            if !((*cp).d.name).is_null() {
                free((*cp).d.name as *mut libc::c_void);
            }
        }
        79 | 49 | 48 | 50 => {
            m = (*cp).d.dn;
            if !((*m).sub.nodep.r.preg[0 as libc::c_int as usize]).is_null() {
                refree((*m).sub.nodep.r.preg[0 as libc::c_int as usize]);
            }
            if !((*m).sub.nodep.r.preg[1 as libc::c_int as usize]).is_null() {
                refree((*m).sub.nodep.r.preg[1 as libc::c_int as usize]);
            }
            if !((*m).sub.nodep.x.extra).is_null() {
                unref((*m).sub.nodep.x.extra);
            }
            if !((*m).sub.nodep.l.lptr).is_null() {
                unref((*m).sub.nodep.l.lptr);
            }
            let ref mut fresh11 = (*(m as *mut block_item)).freep;
            *fresh11 = nextfree[BLOCK_NODE as libc::c_int as usize].freep;
            nextfree[BLOCK_NODE as libc::c_int as usize].freep = m as *mut block_item;
        }
        109 => {
            if !((*cp).d.name).is_null() {
                free((*cp).d.name as *mut libc::c_void);
            }
        }
        78 => {
            m = (*cp).d.dn;
            unref(m);
        }
        27 => {
            m = (*cp).x.xn;
            if !m.is_null() {
                unref(m);
            }
        }
        0 => {
            r_fatal(
                b"internal error: file %s, line %d: unexpected opcode %s\0" as *const u8
                    as *const libc::c_char,
                b"symbol.c\0" as *const u8 as *const libc::c_char,
                1003 as libc::c_int,
                opcode2str((*cp).opcode),
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn free_bc_mempool(
    mut pool: *mut instruction_mem_pool,
    mut size: libc::c_int,
) {
    let mut first: bool = 1 as libc::c_int != 0;
    let mut block: *mut instruction_block = 0 as *mut instruction_block;
    let mut next: *mut instruction_block = 0 as *mut instruction_block;
    block = (*pool).block_list;
    while !block.is_null() {
        let mut cp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
        let mut end: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
        end = if first as libc::c_int != 0 {
            (*pool).free_space
        } else {
            &mut *((*block).i)
                .as_mut_ptr()
                .offset(
                    (2 as libc::c_int * 3 as libc::c_int * 21 as libc::c_int) as isize,
                ) as *mut INSTRUCTION
        };
        cp = &mut *((*block).i).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut INSTRUCTION;
        while cp.offset(size as isize) <= end {
            if (*cp).opcode as libc::c_uint != Op_illegal as libc::c_int as libc::c_uint
            {
                free_bc_internal(cp);
            }
            cp = cp.offset(size as isize);
        }
        next = (*block).next;
        free(block as *mut libc::c_void);
        first = 0 as libc::c_int != 0;
        block = next;
    }
}
unsafe extern "C" fn free_bcpool(mut pl: *mut INSTRUCTION_POOL) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        free_bc_mempool(
            &mut *((*pl).pool).as_mut_ptr().offset(i as isize),
            i + 1 as libc::c_int,
        );
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn is_all_upper(mut name: *const libc::c_char) -> bool {
    while *name as libc::c_int != '\0' as i32 {
        match *name as libc::c_int {
            65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80
            | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 => {}
            _ => return 0 as libc::c_int != 0,
        }
        name = name.offset(1);
        name;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn get_name_from_awk_ns(mut name: *const libc::c_char) -> *mut NODE {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    if strncmp(
        name,
        b"awk::\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        tmp = make_str_node(
            name.offset(5 as libc::c_int as isize),
            (strlen(name)).wrapping_sub(5 as libc::c_int as libc::c_ulong),
            0 as libc::c_int,
        );
    } else {
        tmp = make_str_node(name, strlen(name), 0 as libc::c_int);
    }
    return tmp;
}
