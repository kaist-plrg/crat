use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type re_dfa_t;
    pub type dfa;
    pub type break_point;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    static mut CONVFMT: *const libc::c_char;
    static mut CONVFMTidx: libc::c_int;
    static mut make_number: Option::<unsafe extern "C" fn(libc::c_double) -> *mut NODE>;
    static mut str2number: Option::<unsafe extern "C" fn(*mut NODE) -> *mut NODE>;
    static mut format_val: Option::<
        unsafe extern "C" fn(*const libc::c_char, libc::c_int, *mut NODE) -> *mut NODE,
    >;
    static str_array_func: array_funcs_t;
    fn r_unref(tmp: *mut NODE);
    fn assoc_info(
        subs: *mut NODE,
        val: *mut NODE,
        p: *mut NODE,
        aname: *const libc::c_char,
    );
    fn make_aname(symbol: *const NODE) -> *const libc::c_char;
    static int_array_func: array_funcs_t;
    fn flags2str(_: libc::c_int) -> *const libc::c_char;
    fn array_vname(symbol: *const NODE) -> *const libc::c_char;
    fn make_array() -> *mut NODE;
    fn assoc_copy(symbol: *mut NODE, newsymb: *mut NODE) -> *mut NODE;
    fn estrdup(str: *const libc::c_char, len: size_t) -> *mut libc::c_char;
    fn r_dupnode(n: *mut NODE) -> *mut NODE;
    fn r_fatal(mesg: *const libc::c_char, _: ...);
    fn set_loc(file: *const libc::c_char, line: libc::c_int);
    fn more_blocks(id: libc::c_int) -> *mut libc::c_void;
    static mut nextfree: [block_header; 2];
    fn make_str_node(
        s: *const libc::c_char,
        len: size_t,
        flags: libc::c_int,
    ) -> *mut NODE;
    static mut success_node: *mut NODE;
    fn new_array_element() -> *mut NODE;
    fn null_array(symbol: *mut NODE);
    fn getenv_long(name: *const libc::c_char) -> libc::c_long;
    static mut do_flags: do_flag_values;
    fn is_identchar(c: libc::c_int) -> bool;
    fn is_letter(c: libc::c_int) -> bool;
    static mut output_fp: *mut FILE;
    fn indent(indent_level: libc::c_int);
    fn is_integer(symbol: *mut NODE, subs: *mut NODE) -> *mut *mut NODE;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type wchar_t = libc::c_int;
pub type uint32_t = __uint32_t;
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
pub type assoc_kind_t = libc::c_uint;
pub const ADELETE: assoc_kind_t = 256;
pub const ADESC: assoc_kind_t = 128;
pub const AASC: assoc_kind_t = 64;
pub const AVSTR: assoc_kind_t = 32;
pub const AVNUM: assoc_kind_t = 16;
pub const AISTR: assoc_kind_t = 8;
pub const AINUM: assoc_kind_t = 4;
pub const AVALUE: assoc_kind_t = 2;
pub const AINDEX: assoc_kind_t = 1;
pub const ANONE: assoc_kind_t = 0;
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
unsafe extern "C" fn in_array(mut a: *mut NODE, mut s: *mut NODE) -> *mut NODE {
    let mut ret: *mut *mut NODE = 0 as *mut *mut NODE;
    ret = ((*(*a).sub.nodep.l.lp).exists).unwrap()(a, s);
    return if !ret.is_null() { *ret } else { 0 as *mut NODE };
}
static mut NHAT: libc::c_int = 10 as libc::c_int;
static mut THRESHOLD: libc::c_long = 0;
pub static mut cint_array_func: array_funcs_t = unsafe {
    {
        let mut init = array_funcs_t {
            name: b"cint\0" as *const u8 as *const libc::c_char,
            init: Some(
                cint_array_init
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            type_of: Some(
                is_uinteger
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            lookup: Some(
                cint_lookup
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            exists: Some(
                cint_exists
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            clear: Some(
                cint_clear
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            remove: Some(
                cint_remove
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            list: Some(
                cint_list as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            copy: Some(
                cint_copy as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            dump: Some(
                cint_dump as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            store: None,
        };
        init
    }
};
static mut argv_array_func: array_funcs_t = unsafe {
    {
        let mut init = array_funcs_t {
            name: b"argv\0" as *const u8 as *const libc::c_char,
            init: Some(
                cint_array_init
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            type_of: Some(
                is_uinteger
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            lookup: Some(
                cint_lookup
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            exists: Some(
                cint_exists
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            clear: Some(
                cint_clear
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            remove: Some(
                cint_remove
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            list: Some(
                cint_list as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            copy: Some(
                cint_copy as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            dump: Some(
                cint_dump as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            store: Some(
                argv_store
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
        };
        init
    }
};
static mut power_two_table: [libc::c_long; 31] = [
    1 as libc::c_int as libc::c_long,
    2 as libc::c_int as libc::c_long,
    4 as libc::c_int as libc::c_long,
    8 as libc::c_int as libc::c_long,
    16 as libc::c_int as libc::c_long,
    32 as libc::c_int as libc::c_long,
    64 as libc::c_int as libc::c_long,
    128 as libc::c_int as libc::c_long,
    256 as libc::c_int as libc::c_long,
    512 as libc::c_int as libc::c_long,
    1024 as libc::c_int as libc::c_long,
    2048 as libc::c_int as libc::c_long,
    4096 as libc::c_int as libc::c_long,
    8192 as libc::c_int as libc::c_long,
    16384 as libc::c_int as libc::c_long,
    32768 as libc::c_int as libc::c_long,
    65536 as libc::c_int as libc::c_long,
    131072 as libc::c_int as libc::c_long,
    262144 as libc::c_int as libc::c_long,
    524288 as libc::c_int as libc::c_long,
    1048576 as libc::c_int as libc::c_long,
    2097152 as libc::c_int as libc::c_long,
    4194304 as libc::c_int as libc::c_long,
    8388608 as libc::c_int as libc::c_long,
    16777216 as libc::c_int as libc::c_long,
    33554432 as libc::c_int as libc::c_long,
    67108864 as libc::c_int as libc::c_long,
    134217728 as libc::c_int as libc::c_long,
    268435456 as libc::c_int as libc::c_long,
    536870912 as libc::c_int as libc::c_long,
    1073741824 as libc::c_int as libc::c_long,
];
unsafe extern "C" fn cint_array_init(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    if symbol.is_null() {
        let mut newval: libc::c_long = 0;
        let mut nelems: size_t = (::std::mem::size_of::<[libc::c_long; 31]>()
            as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_long>() as libc::c_ulong);
        newval = getenv_long(b"NHAT\0" as *const u8 as *const libc::c_char);
        if newval > 1 as libc::c_int as libc::c_long
            && newval < 32 as libc::c_int as libc::c_long
        {
            NHAT = newval as libc::c_int;
        }
        if NHAT as libc::c_ulong > nelems.wrapping_sub(2 as libc::c_int as libc::c_ulong)
        {
            NHAT = nelems.wrapping_sub(2 as libc::c_int as libc::c_ulong) as libc::c_int;
        }
        THRESHOLD = power_two_table[(NHAT + 1 as libc::c_int) as usize];
    } else {
        null_array(symbol);
    }
    return &mut success_node;
}
unsafe extern "C" fn is_uinteger(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    if !(is_integer(symbol, subs)).is_null()
        && (*subs).sub.val.fltnum >= 0 as libc::c_int as libc::c_double
    {
        return &mut success_node;
    }
    return 0 as *mut *mut NODE;
}
unsafe extern "C" fn cint_lookup(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    let mut lhs: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut k: libc::c_long = 0;
    let mut h1: libc::c_int = -(1 as libc::c_int);
    let mut m: libc::c_int = 0;
    let mut li: libc::c_int = 0;
    let mut tn: *mut NODE = 0 as *mut NODE;
    let mut xn: *mut NODE = 0 as *mut NODE;
    let mut cint_size: libc::c_long = 0;
    let mut capacity: libc::c_long = 0;
    k = -(1 as libc::c_int) as libc::c_long;
    if ((*subs).flags as libc::c_uint & NUMINT as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint || !(is_integer(symbol, subs)).is_null())
        && (*subs).sub.val.fltnum >= 0 as libc::c_int as libc::c_double
    {
        k = (*subs).sub.val.fltnum as libc::c_long;
        h1 = cint_hash(k);
        lhs = cint_find(symbol, k, h1);
        if !lhs.is_null() {
            return lhs;
        }
    }
    xn = (*symbol).sub.nodep.rn;
    if !xn.is_null()
        && {
            lhs = ((*(*xn).sub.nodep.l.lp).exists).unwrap()(xn, subs);
            !lhs.is_null()
        }
    {
        return lhs;
    }
    if !(k < 0 as libc::c_int as libc::c_long) {
        m = h1 - 1 as libc::c_int;
        li = if m > NHAT { m } else { NHAT };
        while li >= NHAT {
            li = (li + 1 as libc::c_int) / 2 as libc::c_int;
        }
        capacity = ((*symbol).sub.nodep.reserved)
            .wrapping_add(power_two_table[li as usize] as libc::c_ulong) as libc::c_long;
        cint_size = (if xn.is_null() {
            (*symbol).sub.nodep.reflags as libc::c_uint
        } else {
            ((*symbol).sub.nodep.reflags as libc::c_uint)
                .wrapping_sub((*xn).sub.nodep.reflags as libc::c_uint)
        }) as libc::c_long;
        if !(capacity - cint_size > THRESHOLD) {
            if ((*symbol).sub.nodep.r.av).is_null() {
                (*symbol).sub.nodep.reserved = 0 as libc::c_int as size_t;
                (*symbol)
                    .sub
                    .nodep
                    .r
                    .av = ezalloc_real(
                    (32 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut NODE>() as libc::c_ulong,
                        ),
                    b"cint_lookup\0" as *const u8 as *const libc::c_char,
                    b"symbol->nodes\0" as *const u8 as *const libc::c_char,
                    b"cint_array.c\0" as *const u8 as *const libc::c_char,
                    249 as libc::c_int,
                ) as *mut *mut NODE;
            }
            (*symbol).sub.nodep.reflags += 1;
            (*symbol).sub.nodep.reflags;
            tn = *((*symbol).sub.nodep.r.av).offset(h1 as isize);
            if tn.is_null() {
                tn = make_node(Node_array_tree);
                let ref mut fresh0 = *((*symbol).sub.nodep.r.av).offset(h1 as isize);
                *fresh0 = tn;
            }
            if m < NHAT {
                return tree_lookup(
                    symbol,
                    tn,
                    k,
                    NHAT,
                    0 as libc::c_int as libc::c_long,
                );
            }
            return tree_lookup(symbol, tn, k, m, power_two_table[m as usize]);
        }
    }
    (*symbol).sub.nodep.reflags += 1;
    (*symbol).sub.nodep.reflags;
    if xn.is_null() {
        (*symbol).sub.nodep.rn = make_array();
        xn = (*symbol).sub.nodep.rn;
        (*xn).sub.nodep.name = (*symbol).sub.nodep.name;
        if !(is_integer(xn, subs)).is_null() {
            (*xn).sub.nodep.l.lp = &int_array_func;
        } else {
            (*xn).sub.nodep.l.lp = &str_array_func;
        }
        (*xn)
            .flags = ::std::mem::transmute::<
            libc::c_uint,
            flagvals,
        >((*xn).flags as libc::c_uint | XARRAY as libc::c_int as libc::c_uint);
    }
    return ((*(*xn).sub.nodep.l.lp).lookup).unwrap()(xn, subs);
}
unsafe extern "C" fn cint_exists(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    let mut xn: *mut NODE = 0 as *mut NODE;
    if ((*subs).flags as libc::c_uint & NUMINT as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint || !(is_integer(symbol, subs)).is_null())
        && (*subs).sub.val.fltnum >= 0 as libc::c_int as libc::c_double
    {
        let mut k: libc::c_long = (*subs).sub.val.fltnum as libc::c_long;
        let mut lhs: *mut *mut NODE = 0 as *mut *mut NODE;
        lhs = cint_find(symbol, k, cint_hash(k));
        if !lhs.is_null() {
            return lhs;
        }
    }
    xn = (*symbol).sub.nodep.rn;
    if xn.is_null() {
        return 0 as *mut *mut NODE;
    }
    return ((*(*xn).sub.nodep.l.lp).exists).unwrap()(xn, subs);
}
unsafe extern "C" fn cint_clear(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    let mut i: size_t = 0;
    let mut tn: *mut NODE = 0 as *mut NODE;
    if !((*symbol).sub.nodep.rn).is_null() {
        let mut xn: *mut NODE = (*symbol).sub.nodep.rn;
        ((*(*xn).sub.nodep.l.lp).clear).unwrap()(xn, 0 as *mut exp_node);
        let ref mut fresh1 = (*(xn as *mut block_item)).freep;
        *fresh1 = nextfree[BLOCK_NODE as libc::c_int as usize].freep;
        nextfree[BLOCK_NODE as libc::c_int as usize].freep = xn as *mut block_item;
        (*symbol).sub.nodep.rn = 0 as *mut exp_node;
    }
    i = NHAT as size_t;
    while i < 32 as libc::c_int as libc::c_ulong {
        tn = *((*symbol).sub.nodep.r.av).offset(i as isize);
        if !tn.is_null() {
            tree_clear(tn);
            let ref mut fresh2 = (*(tn as *mut block_item)).freep;
            *fresh2 = nextfree[BLOCK_NODE as libc::c_int as usize].freep;
            nextfree[BLOCK_NODE as libc::c_int as usize].freep = tn as *mut block_item;
        }
        i = i.wrapping_add(1);
        i;
    }
    free((*symbol).sub.nodep.r.av as *mut libc::c_void);
    ((*(*symbol).sub.nodep.l.lp).init).unwrap()(symbol, 0 as *mut exp_node);
    return 0 as *mut *mut NODE;
}
unsafe extern "C" fn cint_remove(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    let mut k: libc::c_long = 0;
    let mut h1: libc::c_int = 0;
    let mut tn: *mut NODE = 0 as *mut NODE;
    let mut xn: *mut NODE = (*symbol).sub.nodep.rn;
    if (*symbol).sub.nodep.reflags as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        return 0 as *mut *mut NODE;
    }
    if ((*subs).flags as libc::c_uint & NUMINT as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint || !(is_integer(symbol, subs)).is_null())
        && (*subs).sub.val.fltnum >= 0 as libc::c_int as libc::c_double
    {
        k = (*subs).sub.val.fltnum as libc::c_long;
        h1 = cint_hash(k);
        tn = *((*symbol).sub.nodep.r.av).offset(h1 as isize);
        if !(tn.is_null() || tree_remove(symbol, tn, k) == 0) {
            if (*tn).sub.nodep.reflags as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
            {
                let ref mut fresh3 = (*(tn as *mut block_item)).freep;
                *fresh3 = nextfree[BLOCK_NODE as libc::c_int as usize].freep;
                nextfree[BLOCK_NODE as libc::c_int as usize]
                    .freep = tn as *mut block_item;
                let ref mut fresh4 = *((*symbol).sub.nodep.r.av).offset(h1 as isize);
                *fresh4 = 0 as *mut exp_node;
            }
            (*symbol).sub.nodep.reflags -= 1;
            (*symbol).sub.nodep.reflags;
            if xn.is_null()
                && (*symbol).sub.nodep.reflags as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
            {
                free((*symbol).sub.nodep.r.av as *mut libc::c_void);
                ((*(*symbol).sub.nodep.l.lp).init).unwrap()(symbol, 0 as *mut exp_node);
            } else if !xn.is_null()
                && (*symbol).sub.nodep.reflags as libc::c_uint
                    == (*xn).sub.nodep.reflags as libc::c_uint
            {
                (*xn)
                    .flags = ::std::mem::transmute::<
                    libc::c_uint,
                    flagvals,
                >(
                    (*xn).flags as libc::c_uint
                        & !(XARRAY as libc::c_int) as libc::c_uint,
                );
                (*xn).sub.nodep.x.extra = (*symbol).sub.nodep.x.extra;
                free((*symbol).sub.nodep.r.av as *mut libc::c_void);
                *symbol = *xn;
                let ref mut fresh5 = (*(xn as *mut block_item)).freep;
                *fresh5 = nextfree[BLOCK_NODE as libc::c_int as usize].freep;
                nextfree[BLOCK_NODE as libc::c_int as usize]
                    .freep = xn as *mut block_item;
            }
            return &mut success_node;
        }
    }
    xn = (*symbol).sub.nodep.rn;
    if xn.is_null() || (((*(*xn).sub.nodep.l.lp).remove).unwrap()(xn, subs)).is_null() {
        return 0 as *mut *mut NODE;
    }
    if (*xn).sub.nodep.reflags as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        let ref mut fresh6 = (*(xn as *mut block_item)).freep;
        *fresh6 = nextfree[BLOCK_NODE as libc::c_int as usize].freep;
        nextfree[BLOCK_NODE as libc::c_int as usize].freep = xn as *mut block_item;
        (*symbol).sub.nodep.rn = 0 as *mut exp_node;
    }
    (*symbol).sub.nodep.reflags -= 1;
    (*symbol).sub.nodep.reflags;
    return &mut success_node;
}
unsafe extern "C" fn cint_copy(
    mut symbol: *mut NODE,
    mut newsymb: *mut NODE,
) -> *mut *mut NODE {
    let mut old: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut new: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut i: size_t = 0;
    new = ezalloc_real(
        (32 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut NODE>() as libc::c_ulong),
        b"cint_copy\0" as *const u8 as *const libc::c_char,
        b"new\0" as *const u8 as *const libc::c_char,
        b"cint_array.c\0" as *const u8 as *const libc::c_char,
        407 as libc::c_int,
    ) as *mut *mut NODE;
    old = (*symbol).sub.nodep.r.av;
    i = NHAT as size_t;
    while i < 32 as libc::c_int as libc::c_ulong {
        if !(*old.offset(i as isize)).is_null() {
            let ref mut fresh7 = *new.offset(i as isize);
            *fresh7 = make_node(Node_array_tree);
            tree_copy(newsymb, *old.offset(i as isize), *new.offset(i as isize));
        }
        i = i.wrapping_add(1);
        i;
    }
    if !((*symbol).sub.nodep.rn).is_null() {
        let mut xn: *mut NODE = 0 as *mut NODE;
        let mut n: *mut NODE = 0 as *mut NODE;
        xn = (*symbol).sub.nodep.rn;
        n = make_array();
        (*n).sub.nodep.name = (*newsymb).sub.nodep.name;
        ((*(*xn).sub.nodep.l.lp).copy).unwrap()(xn, n);
        (*newsymb).sub.nodep.rn = n;
    } else {
        (*newsymb).sub.nodep.rn = 0 as *mut exp_node;
    }
    (*newsymb).sub.nodep.r.av = new;
    (*newsymb).sub.nodep.reflags = (*symbol).sub.nodep.reflags;
    (*newsymb).sub.nodep.reserved = (*symbol).sub.nodep.reserved;
    (*newsymb).flags = (*symbol).flags;
    return 0 as *mut *mut NODE;
}
unsafe extern "C" fn cint_list(
    mut symbol: *mut NODE,
    mut t: *mut NODE,
) -> *mut *mut NODE {
    let mut list: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut tn: *mut NODE = 0 as *mut NODE;
    let mut xn: *mut NODE = 0 as *mut NODE;
    let mut k: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut num_elems: libc::c_ulong = 0;
    let mut list_size: libc::c_ulong = 0;
    let mut j: size_t = 0;
    let mut ja: size_t = 0;
    let mut jd: size_t = 0;
    let mut elem_size: libc::c_int = 1 as libc::c_int;
    let mut assoc_kind: assoc_kind_t = ANONE;
    num_elems = (*symbol).sub.nodep.reflags as libc::c_ulong;
    if num_elems == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut *mut NODE;
    }
    assoc_kind = (*t).flags as assoc_kind_t;
    if assoc_kind as libc::c_uint
        & (AINDEX as libc::c_int | AVALUE as libc::c_int | ADELETE as libc::c_int)
            as libc::c_uint
        == (AINDEX as libc::c_int | ADELETE as libc::c_int) as libc::c_uint
    {
        num_elems = 1 as libc::c_int as libc::c_ulong;
    }
    if assoc_kind as libc::c_uint
        & (AINDEX as libc::c_int | AVALUE as libc::c_int) as libc::c_uint
        == (AINDEX as libc::c_int | AVALUE as libc::c_int) as libc::c_uint
    {
        elem_size = 2 as libc::c_int;
    }
    list_size = num_elems.wrapping_mul(elem_size as libc::c_ulong);
    if !((*symbol).sub.nodep.rn).is_null() {
        xn = (*symbol).sub.nodep.rn;
        list = ((*(*xn).sub.nodep.l.lp).list).unwrap()(xn, t);
        assoc_kind = ::std::mem::transmute::<
            libc::c_uint,
            assoc_kind_t,
        >(
            assoc_kind as libc::c_uint
                & !(AASC as libc::c_int | ADESC as libc::c_int) as libc::c_uint,
        );
        (*t).flags = assoc_kind as libc::c_uint as flagvals;
        if num_elems == 1 as libc::c_int as libc::c_ulong
            || num_elems == (*xn).sub.nodep.reflags as libc::c_ulong
        {
            return list;
        }
        list = erealloc_real(
            list as *mut libc::c_void,
            list_size.wrapping_mul(::std::mem::size_of::<*mut NODE>() as libc::c_ulong),
            b"cint_list\0" as *const u8 as *const libc::c_char,
            b"list\0" as *const u8 as *const libc::c_char,
            b"cint_array.c\0" as *const u8 as *const libc::c_char,
            467 as libc::c_int,
        ) as *mut *mut NODE;
        k = (elem_size as libc::c_uint)
            .wrapping_mul((*xn).sub.nodep.reflags as libc::c_uint) as libc::c_ulong;
    } else {
        list = emalloc_real(
            list_size.wrapping_mul(::std::mem::size_of::<*mut NODE>() as libc::c_ulong),
            b"cint_list\0" as *const u8 as *const libc::c_char,
            b"list\0" as *const u8 as *const libc::c_char,
            b"cint_array.c\0" as *const u8 as *const libc::c_char,
            470 as libc::c_int,
        ) as *mut *mut NODE;
    }
    if assoc_kind as libc::c_uint & AINUM as libc::c_int as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        assoc_kind = ::std::mem::transmute::<
            libc::c_uint,
            assoc_kind_t,
        >(
            assoc_kind as libc::c_uint
                & !(AASC as libc::c_int | ADESC as libc::c_int) as libc::c_uint,
        );
        (*t).flags = assoc_kind as libc::c_uint as flagvals;
    }
    ja = NHAT as size_t;
    jd = (32 as libc::c_int - 1 as libc::c_int) as size_t;
    while ja < 32 as libc::c_int as libc::c_ulong && jd >= NHAT as libc::c_ulong {
        j = if assoc_kind as libc::c_uint & ADESC as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            let fresh8 = jd;
            jd = jd.wrapping_sub(1);
            fresh8
        } else {
            let fresh9 = ja;
            ja = ja.wrapping_add(1);
            fresh9
        };
        tn = *((*symbol).sub.nodep.r.av).offset(j as isize);
        if tn.is_null() {
            continue;
        }
        k = k
            .wrapping_add(
                tree_list(tn, list.offset(k as isize), assoc_kind) as libc::c_ulong,
            );
        if k >= list_size {
            return list;
        }
    }
    return list;
}
unsafe extern "C" fn cint_dump(
    mut symbol: *mut NODE,
    mut ndump: *mut NODE,
) -> *mut *mut NODE {
    let mut tn: *mut NODE = 0 as *mut NODE;
    let mut xn: *mut NODE = 0 as *mut NODE;
    let mut indent_level: libc::c_int = 0;
    let mut i: size_t = 0;
    let mut cint_size: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut xsize: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut kb: libc::c_double = 0 as libc::c_int as libc::c_double;
    extern "C" {
        #[link_name = "int_kilobytes"]
        fn int_kilobytes(symbol_0: *mut NODE) -> libc::c_double;
    }
    extern "C" {
        #[link_name = "str_kilobytes"]
        fn str_kilobytes(symbol_0: *mut NODE) -> libc::c_double;
    }
    indent_level = (*ndump).sub.nodep.x.xl as libc::c_int;
    if !((*symbol).sub.nodep.rn).is_null() {
        xn = (*symbol).sub.nodep.rn;
        xsize = (*xn).sub.nodep.reflags as libc::c_long;
    }
    cint_size = (*symbol).sub.nodep.reflags as libc::c_long - xsize;
    if (*symbol).flags as libc::c_uint & XARRAY as libc::c_int as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        fprintf(
            output_fp,
            b"%s `%s'\n\0" as *const u8 as *const libc::c_char,
            if ((*symbol).sub.nodep.x.extra).is_null() {
                b"array\0" as *const u8 as *const libc::c_char
            } else {
                b"sub-array\0" as *const u8 as *const libc::c_char
            },
            array_vname(symbol),
        );
    }
    indent_level += 1;
    indent_level;
    indent(indent_level);
    fprintf(
        output_fp,
        b"array_func: cint_array_func\n\0" as *const u8 as *const libc::c_char,
    );
    if (*symbol).flags as libc::c_uint != 0 as libc::c_int as libc::c_uint {
        indent(indent_level);
        fprintf(
            output_fp,
            b"flags: %s\n\0" as *const u8 as *const libc::c_char,
            flags2str((*symbol).flags as libc::c_int),
        );
    }
    indent(indent_level);
    fprintf(output_fp, b"NHAT: %d\n\0" as *const u8 as *const libc::c_char, NHAT);
    indent(indent_level);
    fprintf(
        output_fp,
        b"THRESHOLD: %ld\n\0" as *const u8 as *const libc::c_char,
        THRESHOLD,
    );
    indent(indent_level);
    fprintf(
        output_fp,
        b"table_size: %lu (total), %ld (cint), %ld (int + str)\n\0" as *const u8
            as *const libc::c_char,
        (*symbol).sub.nodep.reflags as libc::c_ulong,
        cint_size,
        xsize,
    );
    indent(indent_level);
    fprintf(
        output_fp,
        b"array_capacity: %lu\n\0" as *const u8 as *const libc::c_char,
        (*symbol).sub.nodep.reserved,
    );
    indent(indent_level);
    fprintf(
        output_fp,
        b"Load Factor: %.2g\n\0" as *const u8 as *const libc::c_char,
        cint_size as libc::c_double / (*symbol).sub.nodep.reserved as libc::c_double,
    );
    i = NHAT as size_t;
    while i < 32 as libc::c_int as libc::c_ulong {
        tn = *((*symbol).sub.nodep.r.av).offset(i as isize);
        if !tn.is_null() {
            kb
                += (::std::mem::size_of::<NODE>() as libc::c_ulong)
                    .wrapping_add(tree_kilobytes(tn)) as libc::c_double / 1024.0f64;
        }
        i = i.wrapping_add(1);
        i;
    }
    kb
        += (32 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut NODE>() as libc::c_ulong)
            as libc::c_double / 1024.0f64;
    kb
        += ((*symbol).sub.nodep.reserved)
            .wrapping_mul(::std::mem::size_of::<*mut NODE>() as libc::c_ulong)
            as libc::c_double / 1024.0f64;
    if !xn.is_null() {
        if (*xn).sub.nodep.l.lp == &int_array_func as *const array_funcs_t {
            kb += int_kilobytes(xn);
        } else {
            kb += str_kilobytes(xn);
        }
    }
    indent(indent_level);
    fprintf(
        output_fp,
        b"memory: %.2g kB (total)\n\0" as *const u8 as *const libc::c_char,
        kb,
    );
    if (*ndump).sub.nodep.l.ll >= 0 as libc::c_int as libc::c_long {
        let mut aname: *const libc::c_char = 0 as *const libc::c_char;
        fprintf(output_fp, b"\n\0" as *const u8 as *const libc::c_char);
        aname = make_aname(symbol);
        i = NHAT as size_t;
        while i < 32 as libc::c_int as libc::c_ulong {
            tn = *((*symbol).sub.nodep.r.av).offset(i as isize);
            if !tn.is_null() {
                tree_info(tn, ndump, aname);
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    if !xn.is_null() {
        fprintf(output_fp, b"\n\0" as *const u8 as *const libc::c_char);
        ((*(*xn).sub.nodep.l.lp).dump).unwrap()(xn, ndump);
    }
    return 0 as *mut *mut NODE;
}
#[inline]
unsafe extern "C" fn cint_hash(mut k: libc::c_long) -> libc::c_int {
    let mut num: uint32_t = 0;
    let mut r: uint32_t = 0;
    let mut shift: uint32_t = 0;
    if k == 0 as libc::c_int as libc::c_long {
        return NHAT;
    }
    num = k as uint32_t;
    r = (((num > 0xffff as libc::c_int as libc::c_uint) as libc::c_int)
        << 4 as libc::c_int) as uint32_t;
    num >>= r;
    shift = (((num > 0xff as libc::c_int as libc::c_uint) as libc::c_int)
        << 3 as libc::c_int) as uint32_t;
    num >>= shift;
    r |= shift;
    shift = (((num > 0xf as libc::c_int as libc::c_uint) as libc::c_int)
        << 2 as libc::c_int) as uint32_t;
    num >>= shift;
    r |= shift;
    shift = (((num > 0x3 as libc::c_int as libc::c_uint) as libc::c_int)
        << 1 as libc::c_int) as uint32_t;
    num >>= shift;
    r |= shift;
    r |= num >> 1 as libc::c_int;
    if r < NHAT as libc::c_uint {
        return NHAT;
    }
    return (1 as libc::c_int as libc::c_uint).wrapping_add(r) as libc::c_int;
}
#[inline]
unsafe extern "C" fn cint_find(
    mut symbol: *mut NODE,
    mut k: libc::c_long,
    mut h1: libc::c_int,
) -> *mut *mut NODE {
    let mut tn: *mut NODE = 0 as *mut NODE;
    if ((*symbol).sub.nodep.r.av).is_null()
        || {
            tn = *((*symbol).sub.nodep.r.av).offset(h1 as isize);
            tn.is_null()
        }
    {
        return 0 as *mut *mut NODE;
    }
    return tree_exists(tn, k);
}
#[inline]
unsafe extern "C" fn make_node(mut type_0: NODETYPE) -> *mut NODE {
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
        '\0' as i32,
        ::std::mem::size_of::<NODE>() as libc::c_ulong,
    );
    (*n).type_0 = type_0;
    return n;
}
unsafe extern "C" fn tree_lookup(
    mut symbol: *mut NODE,
    mut tree: *mut NODE,
    mut k: libc::c_long,
    mut m: libc::c_int,
    mut base: libc::c_long,
) -> *mut *mut NODE {
    let mut lhs: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut tn: *mut NODE = 0 as *mut NODE;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut size: size_t = 0;
    let mut num: libc::c_long = k;
    n = (m + 1 as libc::c_int) / 2 as libc::c_int;
    if (*tree).sub.nodep.reflags as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        let mut actual_size: size_t = 0;
        let mut table: *mut *mut NODE = 0 as *mut *mut NODE;
        actual_size = power_two_table[n as usize] as size_t;
        size = actual_size;
        (*tree).sub.nodep.l.ll = base;
        (*tree).sub.nodep.cnt = size;
        (*tree).sub.nodep.reflags = 0 as reflagvals;
        if n > m / 2 as libc::c_int {
            actual_size = (actual_size as libc::c_ulong)
                .wrapping_div(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
            (*tree)
                .flags = ::std::mem::transmute::<
                libc::c_uint,
                flagvals,
            >((*tree).flags as libc::c_uint | HALFHAT as libc::c_int as libc::c_uint);
        }
        table = ezalloc_real(
            actual_size
                .wrapping_mul(::std::mem::size_of::<*mut NODE>() as libc::c_ulong),
            b"tree_lookup\0" as *const u8 as *const libc::c_char,
            b"table\0" as *const u8 as *const libc::c_char,
            b"cint_array.c\0" as *const u8 as *const libc::c_char,
            773 as libc::c_int,
        ) as *mut *mut NODE;
        (*tree).sub.nodep.r.av = table;
    } else {
        size = (*tree).sub.nodep.cnt;
    }
    num -= (*tree).sub.nodep.l.ll;
    i = (num as libc::c_ulong).wrapping_div(size) as libc::c_int;
    lhs = tree_find(tree, k, i);
    if !lhs.is_null() {
        return lhs;
    }
    (*tree).sub.nodep.reflags += 1;
    (*tree).sub.nodep.reflags;
    base = (base as libc::c_ulong).wrapping_add(size.wrapping_mul(i as libc::c_ulong))
        as libc::c_long as libc::c_long;
    tn = *((*tree).sub.nodep.r.av).offset(i as isize);
    if n > NHAT {
        if tn.is_null() {
            let ref mut fresh10 = *((*tree).sub.nodep.r.av).offset(i as isize);
            *fresh10 = make_node(Node_array_tree);
            tn = *fresh10;
        }
        return tree_lookup(symbol, tn, k, n, base);
    } else {
        if tn.is_null() {
            let ref mut fresh11 = *((*tree).sub.nodep.r.av).offset(i as isize);
            *fresh11 = make_node(Node_array_leaf);
            tn = *fresh11;
        }
        return leaf_lookup(symbol, tn, k, size as libc::c_long, base);
    };
}
unsafe extern "C" fn tree_exists(
    mut tree: *mut NODE,
    mut k: libc::c_long,
) -> *mut *mut NODE {
    let mut i: libc::c_int = 0;
    let mut tn: *mut NODE = 0 as *mut NODE;
    i = ((k - (*tree).sub.nodep.l.ll) as libc::c_ulong)
        .wrapping_div((*tree).sub.nodep.cnt) as libc::c_int;
    tn = *((*tree).sub.nodep.r.av).offset(i as isize);
    if tn.is_null() {
        return 0 as *mut *mut NODE;
    }
    if (*tn).type_0 as libc::c_uint == Node_array_tree as libc::c_int as libc::c_uint {
        return tree_exists(tn, k);
    }
    return leaf_exists(tn, k);
}
unsafe extern "C" fn tree_clear(mut tree: *mut NODE) {
    let mut tn: *mut NODE = 0 as *mut NODE;
    let mut j: size_t = 0;
    let mut hsize: size_t = 0;
    hsize = (*tree).sub.nodep.cnt;
    if (*tree).flags as libc::c_uint & HALFHAT as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        hsize = (hsize as libc::c_ulong).wrapping_div(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    j = 0 as libc::c_int as size_t;
    while j < hsize {
        tn = *((*tree).sub.nodep.r.av).offset(j as isize);
        if !tn.is_null() {
            if (*tn).type_0 as libc::c_uint
                == Node_array_tree as libc::c_int as libc::c_uint
            {
                tree_clear(tn);
            } else {
                leaf_clear(tn);
            }
            let ref mut fresh12 = (*(tn as *mut block_item)).freep;
            *fresh12 = nextfree[BLOCK_NODE as libc::c_int as usize].freep;
            nextfree[BLOCK_NODE as libc::c_int as usize].freep = tn as *mut block_item;
        }
        j = j.wrapping_add(1);
        j;
    }
    free((*tree).sub.nodep.r.av as *mut libc::c_void);
    memset(
        tree as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<NODE>() as libc::c_ulong,
    );
    (*tree).type_0 = Node_array_tree;
}
unsafe extern "C" fn tree_remove(
    mut symbol: *mut NODE,
    mut tree: *mut NODE,
    mut k: libc::c_long,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut tn: *mut NODE = 0 as *mut NODE;
    i = ((k - (*tree).sub.nodep.l.ll) as libc::c_ulong)
        .wrapping_div((*tree).sub.nodep.cnt) as libc::c_int;
    tn = *((*tree).sub.nodep.r.av).offset(i as isize);
    if tn.is_null() {
        return 0 as libc::c_int;
    }
    if (*tn).type_0 as libc::c_uint == Node_array_tree as libc::c_int as libc::c_uint
        && tree_remove(symbol, tn, k) == 0
    {
        return 0 as libc::c_int
    } else if (*tn).type_0 as libc::c_uint
        == Node_array_leaf as libc::c_int as libc::c_uint
        && leaf_remove(symbol, tn, k) == 0
    {
        return 0 as libc::c_int
    }
    if (*tn).sub.nodep.reflags as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        let ref mut fresh13 = (*(tn as *mut block_item)).freep;
        *fresh13 = nextfree[BLOCK_NODE as libc::c_int as usize].freep;
        nextfree[BLOCK_NODE as libc::c_int as usize].freep = tn as *mut block_item;
        let ref mut fresh14 = *((*tree).sub.nodep.r.av).offset(i as isize);
        *fresh14 = 0 as *mut exp_node;
    }
    (*tree).sub.nodep.reflags -= 1;
    if (*tree).sub.nodep.reflags as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        free((*tree).sub.nodep.r.av as *mut libc::c_void);
        memset(
            tree as *mut libc::c_void,
            '\0' as i32,
            ::std::mem::size_of::<NODE>() as libc::c_ulong,
        );
        (*tree).type_0 = Node_array_tree;
    }
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn tree_find(
    mut tree: *mut NODE,
    mut k: libc::c_long,
    mut i: libc::c_int,
) -> *mut *mut NODE {
    let mut tn: *mut NODE = 0 as *mut NODE;
    tn = *((*tree).sub.nodep.r.av).offset(i as isize);
    if !tn.is_null() {
        if (*tn).type_0 as libc::c_uint == Node_array_tree as libc::c_int as libc::c_uint
        {
            return tree_exists(tn, k);
        }
        return leaf_exists(tn, k);
    }
    return 0 as *mut *mut NODE;
}
unsafe extern "C" fn tree_list(
    mut tree: *mut NODE,
    mut list: *mut *mut NODE,
    mut assoc_kind: assoc_kind_t,
) -> libc::c_long {
    let mut tn: *mut NODE = 0 as *mut NODE;
    let mut j: size_t = 0;
    let mut cj: size_t = 0;
    let mut hsize: size_t = 0;
    let mut k: libc::c_long = 0 as libc::c_int as libc::c_long;
    hsize = (*tree).sub.nodep.cnt;
    if (*tree).flags as libc::c_uint & HALFHAT as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        hsize = (hsize as libc::c_ulong).wrapping_div(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    j = 0 as libc::c_int as size_t;
    while j < hsize {
        cj = if assoc_kind as libc::c_uint & ADESC as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            hsize.wrapping_sub(1 as libc::c_int as libc::c_ulong).wrapping_sub(j)
        } else {
            j
        };
        tn = *((*tree).sub.nodep.r.av).offset(cj as isize);
        if !tn.is_null() {
            if (*tn).type_0 as libc::c_uint
                == Node_array_tree as libc::c_int as libc::c_uint
            {
                k += tree_list(tn, list.offset(k as isize), assoc_kind);
            } else {
                k += leaf_list(tn, list.offset(k as isize), assoc_kind);
            }
            if assoc_kind as libc::c_uint & ADELETE as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
                && k >= 1 as libc::c_int as libc::c_long
            {
                return k;
            }
        }
        j = j.wrapping_add(1);
        j;
    }
    return k;
}
unsafe extern "C" fn tree_copy(
    mut newsymb: *mut NODE,
    mut tree: *mut NODE,
    mut newtree: *mut NODE,
) {
    let mut old: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut new: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut j: size_t = 0;
    let mut hsize: size_t = 0;
    hsize = (*tree).sub.nodep.cnt;
    if (*tree).flags as libc::c_uint & HALFHAT as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        hsize = (hsize as libc::c_ulong).wrapping_div(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    new = ezalloc_real(
        hsize.wrapping_mul(::std::mem::size_of::<*mut NODE>() as libc::c_ulong),
        b"tree_copy\0" as *const u8 as *const libc::c_char,
        b"new\0" as *const u8 as *const libc::c_char,
        b"cint_array.c\0" as *const u8 as *const libc::c_char,
        946 as libc::c_int,
    ) as *mut *mut NODE;
    (*newtree).sub.nodep.r.av = new;
    (*newtree).sub.nodep.l.ll = (*tree).sub.nodep.l.ll;
    (*newtree).sub.nodep.cnt = (*tree).sub.nodep.cnt;
    (*newtree).sub.nodep.reflags = (*tree).sub.nodep.reflags;
    (*newtree).flags = (*tree).flags;
    old = (*tree).sub.nodep.r.av;
    j = 0 as libc::c_int as size_t;
    while j < hsize {
        if !(*old.offset(j as isize)).is_null() {
            if (**old.offset(j as isize)).type_0 as libc::c_uint
                == Node_array_tree as libc::c_int as libc::c_uint
            {
                let ref mut fresh15 = *new.offset(j as isize);
                *fresh15 = make_node(Node_array_tree);
                tree_copy(newsymb, *old.offset(j as isize), *new.offset(j as isize));
            } else {
                let ref mut fresh16 = *new.offset(j as isize);
                *fresh16 = make_node(Node_array_leaf);
                leaf_copy(newsymb, *old.offset(j as isize), *new.offset(j as isize));
            }
        }
        j = j.wrapping_add(1);
        j;
    }
}
unsafe extern "C" fn tree_info(
    mut tree: *mut NODE,
    mut ndump: *mut NODE,
    mut aname: *const libc::c_char,
) {
    let mut tn: *mut NODE = 0 as *mut NODE;
    let mut j: size_t = 0;
    let mut hsize: size_t = 0;
    hsize = (*tree).sub.nodep.cnt;
    if (*tree).flags as libc::c_uint & HALFHAT as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        hsize = (hsize as libc::c_ulong).wrapping_div(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    j = 0 as libc::c_int as size_t;
    while j < hsize {
        tn = *((*tree).sub.nodep.r.av).offset(j as isize);
        if !tn.is_null() {
            if (*tn).type_0 as libc::c_uint
                == Node_array_tree as libc::c_int as libc::c_uint
            {
                tree_info(tn, ndump, aname);
            } else {
                leaf_info(tn, ndump, aname);
            }
        }
        j = j.wrapping_add(1);
        j;
    }
}
unsafe extern "C" fn tree_kilobytes(mut tree: *mut NODE) -> size_t {
    let mut tn: *mut NODE = 0 as *mut NODE;
    let mut j: size_t = 0;
    let mut hsize: size_t = 0;
    let mut sz: size_t = 0 as libc::c_int as size_t;
    hsize = (*tree).sub.nodep.cnt;
    if (*tree).flags as libc::c_uint & HALFHAT as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        hsize = (hsize as libc::c_ulong).wrapping_div(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    j = 0 as libc::c_int as size_t;
    while j < hsize {
        tn = *((*tree).sub.nodep.r.av).offset(j as isize);
        if !tn.is_null() {
            sz = (sz as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<NODE>() as libc::c_ulong) as size_t
                as size_t;
            if (*tn).type_0 as libc::c_uint
                == Node_array_tree as libc::c_int as libc::c_uint
            {
                sz = (sz as libc::c_ulong).wrapping_add(tree_kilobytes(tn)) as size_t
                    as size_t;
            }
        }
        j = j.wrapping_add(1);
        j;
    }
    sz = (sz as libc::c_ulong)
        .wrapping_add(
            hsize.wrapping_mul(::std::mem::size_of::<*mut NODE>() as libc::c_ulong),
        ) as size_t as size_t;
    return sz;
}
#[inline]
unsafe extern "C" fn leaf_lookup(
    mut symbol: *mut NODE,
    mut array: *mut NODE,
    mut k: libc::c_long,
    mut size: libc::c_long,
    mut base: libc::c_long,
) -> *mut *mut NODE {
    let mut lhs: *mut *mut NODE = 0 as *mut *mut NODE;
    if ((*array).sub.nodep.r.av).is_null() {
        (*array).sub.nodep.reflags = 0 as reflagvals;
        (*array).sub.nodep.cnt = size as libc::c_ulong;
        (*array).sub.nodep.l.ll = base;
        (*array)
            .sub
            .nodep
            .r
            .av = ezalloc_real(
            (size as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut NODE>() as libc::c_ulong),
            b"leaf_lookup\0" as *const u8 as *const libc::c_char,
            b"array->nodes\0" as *const u8 as *const libc::c_char,
            b"cint_array.c\0" as *const u8 as *const libc::c_char,
            1064 as libc::c_int,
        ) as *mut *mut NODE;
        (*symbol)
            .sub
            .nodep
            .reserved = ((*symbol).sub.nodep.reserved as libc::c_ulong)
            .wrapping_add(size as libc::c_ulong) as size_t as size_t;
    }
    lhs = ((*array).sub.nodep.r.av).offset((k - base) as isize);
    if (*lhs).is_null() {
        (*array).sub.nodep.reflags += 1;
        (*array).sub.nodep.reflags;
        *lhs = new_array_element();
    }
    return lhs;
}
#[inline]
unsafe extern "C" fn leaf_exists(
    mut array: *mut NODE,
    mut k: libc::c_long,
) -> *mut *mut NODE {
    let mut lhs: *mut *mut NODE = 0 as *mut *mut NODE;
    lhs = ((*array).sub.nodep.r.av).offset((k - (*array).sub.nodep.l.ll) as isize);
    return if !(*lhs).is_null() { lhs } else { 0 as *mut *mut NODE };
}
unsafe extern "C" fn leaf_clear(mut array: *mut NODE) {
    let mut i: libc::c_long = 0;
    let mut size: libc::c_long = (*array).sub.nodep.cnt as libc::c_long;
    let mut r: *mut NODE = 0 as *mut NODE;
    i = 0 as libc::c_int as libc::c_long;
    while i < size {
        r = *((*array).sub.nodep.r.av).offset(i as isize);
        if !r.is_null() {
            if (*r).type_0 as libc::c_uint
                == Node_var_array as libc::c_int as libc::c_uint
            {
                ((*(*r).sub.nodep.l.lp).clear).unwrap()(r, 0 as *mut exp_node);
                free((*r).sub.nodep.name as *mut libc::c_void);
                let ref mut fresh17 = (*(r as *mut block_item)).freep;
                *fresh17 = nextfree[BLOCK_NODE as libc::c_int as usize].freep;
                nextfree[BLOCK_NODE as libc::c_int as usize]
                    .freep = r as *mut block_item;
            } else {
                unref(r);
            }
        }
        i += 1;
        i;
    }
    free((*array).sub.nodep.r.av as *mut libc::c_void);
    (*array).sub.nodep.r.av = 0 as *mut *mut exp_node;
    (*array).sub.nodep.reflags = 0 as reflagvals;
    (*array).sub.nodep.cnt = (*array).sub.nodep.reflags as libc::c_ulong;
}
unsafe extern "C" fn leaf_remove(
    mut symbol: *mut NODE,
    mut array: *mut NODE,
    mut k: libc::c_long,
) -> libc::c_int {
    let mut lhs: *mut *mut NODE = 0 as *mut *mut NODE;
    lhs = ((*array).sub.nodep.r.av).offset((k - (*array).sub.nodep.l.ll) as isize);
    if (*lhs).is_null() {
        return 0 as libc::c_int;
    }
    *lhs = 0 as *mut NODE;
    (*array).sub.nodep.reflags -= 1;
    if (*array).sub.nodep.reflags as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        free((*array).sub.nodep.r.av as *mut libc::c_void);
        (*array).sub.nodep.r.av = 0 as *mut *mut exp_node;
        (*symbol)
            .sub
            .nodep
            .reserved = ((*symbol).sub.nodep.reserved as libc::c_ulong)
            .wrapping_sub((*array).sub.nodep.cnt) as size_t as size_t;
        (*array).sub.nodep.cnt = 0 as libc::c_int as libc::c_ulong;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn leaf_copy(
    mut newsymb: *mut NODE,
    mut array: *mut NODE,
    mut newarray: *mut NODE,
) {
    let mut old: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut new: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut size: libc::c_long = 0;
    let mut i: libc::c_long = 0;
    size = (*array).sub.nodep.cnt as libc::c_long;
    new = ezalloc_real(
        (size as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut NODE>() as libc::c_ulong),
        b"leaf_copy\0" as *const u8 as *const libc::c_char,
        b"new\0" as *const u8 as *const libc::c_char,
        b"cint_array.c\0" as *const u8 as *const libc::c_char,
        1143 as libc::c_int,
    ) as *mut *mut NODE;
    (*newarray).sub.nodep.r.av = new;
    (*newarray).sub.nodep.cnt = size as libc::c_ulong;
    (*newarray).sub.nodep.l.ll = (*array).sub.nodep.l.ll;
    (*newarray).flags = (*array).flags;
    (*newarray).sub.nodep.reflags = (*array).sub.nodep.reflags;
    old = (*array).sub.nodep.r.av;
    i = 0 as libc::c_int as libc::c_long;
    while i < size {
        if !(*old.offset(i as isize)).is_null() {
            if (**old.offset(i as isize)).type_0 as libc::c_uint
                == Node_val as libc::c_int as libc::c_uint
            {
                let ref mut fresh18 = *new.offset(i as isize);
                *fresh18 = dupnode(*old.offset(i as isize));
            } else {
                let mut r: *mut NODE = 0 as *mut NODE;
                r = make_array();
                (*r)
                    .sub
                    .nodep
                    .name = estrdup(
                    (**old.offset(i as isize)).sub.nodep.name,
                    strlen((**old.offset(i as isize)).sub.nodep.name),
                );
                (*r).sub.nodep.x.extra = newsymb;
                let ref mut fresh19 = *new.offset(i as isize);
                *fresh19 = assoc_copy(*old.offset(i as isize), r);
            }
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn leaf_list(
    mut array: *mut NODE,
    mut list: *mut *mut NODE,
    mut assoc_kind: assoc_kind_t,
) -> libc::c_long {
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut subs: *mut NODE = 0 as *mut NODE;
    let mut num: libc::c_long = 0;
    let mut i: libc::c_long = 0;
    let mut ci: libc::c_long = 0;
    let mut k: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut size: libc::c_long = (*array).sub.nodep.cnt as libc::c_long;
    static mut buf: [libc::c_char; 100] = [0; 100];
    i = 0 as libc::c_int as libc::c_long;
    while i < size {
        ci = if assoc_kind as libc::c_uint & ADESC as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            size - 1 as libc::c_int as libc::c_long - i
        } else {
            i
        };
        r = *((*array).sub.nodep.r.av).offset(ci as isize);
        if !r.is_null() {
            num = (*array).sub.nodep.l.ll + ci;
            if assoc_kind as libc::c_uint & AISTR as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
            {
                sprintf(
                    buf.as_mut_ptr(),
                    b"%ld\0" as *const u8 as *const libc::c_char,
                    num,
                );
                subs = make_str_node(
                    buf.as_mut_ptr(),
                    strlen(buf.as_mut_ptr()),
                    0 as libc::c_int,
                );
                (*subs).sub.val.fltnum = num as libc::c_double;
                (*subs)
                    .flags = ::std::mem::transmute::<
                    libc::c_uint,
                    flagvals,
                >(
                    (*subs).flags as libc::c_uint
                        | (NUMCUR as libc::c_int | NUMINT as libc::c_int) as libc::c_uint,
                );
            } else {
                subs = make_number.unwrap()(num as libc::c_double);
                (*subs)
                    .flags = ::std::mem::transmute::<
                    libc::c_uint,
                    flagvals,
                >(
                    (*subs).flags as libc::c_uint
                        | (INTIND as libc::c_int | NUMINT as libc::c_int) as libc::c_uint,
                );
            }
            let fresh20 = k;
            k = k + 1;
            let ref mut fresh21 = *list.offset(fresh20 as isize);
            *fresh21 = subs;
            if assoc_kind as libc::c_uint & AVALUE as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
            {
                if (*r).type_0 as libc::c_uint == Node_val as libc::c_int as libc::c_uint
                {
                    if assoc_kind as libc::c_uint & AVNUM as libc::c_int as libc::c_uint
                        != 0 as libc::c_int as libc::c_uint
                    {
                        force_number(r);
                    } else if assoc_kind as libc::c_uint
                        & AVSTR as libc::c_int as libc::c_uint
                        != 0 as libc::c_int as libc::c_uint
                    {
                        r = force_string_fmt(r, CONVFMT, CONVFMTidx);
                    }
                }
                let fresh22 = k;
                k = k + 1;
                let ref mut fresh23 = *list.offset(fresh22 as isize);
                *fresh23 = r;
            }
            if assoc_kind as libc::c_uint & ADELETE as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
                && k >= 1 as libc::c_int as libc::c_long
            {
                return k;
            }
        }
        i += 1;
        i;
    }
    return k;
}
unsafe extern "C" fn leaf_info(
    mut array: *mut NODE,
    mut ndump: *mut NODE,
    mut aname: *const libc::c_char,
) {
    let mut subs: *mut NODE = 0 as *mut NODE;
    let mut val: *mut NODE = 0 as *mut NODE;
    let mut i: size_t = 0;
    let mut size: size_t = 0;
    size = (*array).sub.nodep.cnt;
    subs = make_number.unwrap()(0.0f64);
    (*subs)
        .flags = ::std::mem::transmute::<
        libc::c_uint,
        flagvals,
    >(
        (*subs).flags as libc::c_uint
            | (INTIND as libc::c_int | NUMINT as libc::c_int) as libc::c_uint,
    );
    i = 0 as libc::c_int as size_t;
    while i < size {
        val = *((*array).sub.nodep.r.av).offset(i as isize);
        if !val.is_null() {
            (*subs)
                .sub
                .val
                .fltnum = ((*array).sub.nodep.l.ll as libc::c_ulong).wrapping_add(i)
                as libc::c_double;
            assoc_info(subs, val, ndump, aname);
        }
        i = i.wrapping_add(1);
        i;
    }
    unref(subs);
}
static mut argv_shadow_array: *mut NODE = 0 as *const NODE as *mut NODE;
unsafe extern "C" fn argv_store(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    let mut val: *mut *mut NODE = cint_exists(symbol, subs);
    let mut newval: *mut NODE = *val;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*newval).sub.val.slen == 0 as libc::c_int as libc::c_ulong {
        return val;
    }
    cp = strchr((*newval).sub.val.sp, '=' as i32);
    if cp.is_null() {
        if (in_array(argv_shadow_array, newval)).is_null() {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"cint_array.c\0" as *const u8 as *const libc::c_char,
                1268 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot add a new file (%.*s) to ARGV in sandbox mode\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*newval).sub.val.slen as libc::c_int,
                (*newval).sub.val.sp,
            );
        }
    } else {
        let mut badvar: bool = 0 as libc::c_int != 0;
        let mut arg: *mut libc::c_char = (*newval).sub.val.sp;
        let mut cp2: *mut libc::c_char = 0 as *mut libc::c_char;
        *cp = '\0' as i32 as libc::c_char;
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
        if !badvar {
            let mut cp_0: *mut libc::c_char = strchr(arg, ':' as i32);
            if !cp_0.is_null()
                && (*cp_0.offset(1 as libc::c_int as isize) as libc::c_int != ':' as i32
                    || !(strchr(cp_0.offset(2 as libc::c_int as isize), ':' as i32))
                        .is_null())
            {
                badvar = 1 as libc::c_int != 0;
            }
        }
        *cp = '=' as i32 as libc::c_char;
        if badvar as libc::c_int != 0 && (in_array(argv_shadow_array, newval)).is_null()
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"cint_array.c\0" as *const u8 as *const libc::c_char,
                1296 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot add a new file (%.*s) to ARGV in sandbox mode\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*newval).sub.val.slen as libc::c_int,
                (*newval).sub.val.sp,
            );
        }
    }
    return val;
}
pub unsafe extern "C" fn init_argv_array(
    mut argv_node: *mut NODE,
    mut shadow_node: *mut NODE,
) {
    if do_flags as libc::c_uint & DO_SANDBOX as libc::c_int as libc::c_uint == 0 {
        return;
    }
    (*argv_node).sub.nodep.l.lp = &argv_array_func;
    argv_shadow_array = shadow_node;
}
