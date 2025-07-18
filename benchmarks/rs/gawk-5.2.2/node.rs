use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type re_dfa_t;
    pub type dfa;
    pub type break_point;
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn using_utf8() -> bool;
    fn towlower(__wc: wint_t) -> wint_t;
    static mut Nnull_string: *mut NODE;
    static mut Null_field: *mut NODE;
    fn r_fatal(mesg: *const libc::c_char, _: ...);
    fn set_loc(file: *const libc::c_char, line: libc::c_int);
    fn nondec2awknum(
        str: *mut libc::c_char,
        len: size_t,
        endptr: *mut *mut libc::c_char,
    ) -> libc::c_double;
    static mut loc: lconv;
    static mut do_flags: do_flag_values;
    fn is_alpha(c: libc::c_int) -> bool;
    fn estrdup(str: *const libc::c_char, len: size_t) -> *mut libc::c_char;
    fn format_tree(
        _: *const libc::c_char,
        _: size_t,
        _: *mut *mut NODE,
        _: libc::c_long,
    ) -> *mut NODE;
    fn double_to_int(d: libc::c_double) -> libc::c_double;
    fn format_nan_inf(n: *mut NODE, format: libc::c_char) -> *mut libc::c_char;
    static mut lintfunc: Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>;
    fn r_warning(mesg: *const libc::c_char, _: ...);
    static mut gawk_mb_cur_max: libc::c_int;
    fn out_of_range(n: *mut NODE) -> bool;
    fn mpfr_unset(n: *mut NODE);
    fn make_regnode(type_0: NODETYPE, exp: *mut NODE) -> *mut NODE;
    fn btowc(__c: libc::c_int) -> wint_t;
    fn mbrtowc(
        __pwc: *mut wchar_t,
        __s: *const libc::c_char,
        __n: size_t,
        __p: *mut mbstate_t,
    ) -> size_t;
    fn wcrtomb(__s: *mut libc::c_char, __wc: wchar_t, __ps: *mut mbstate_t) -> size_t;
    fn __mbrlen(__s: *const libc::c_char, __n: size_t, __ps: *mut mbstate_t) -> size_t;
    static mut fmt_list: *mut *mut NODE;
}
pub type size_t = libc::c_ulong;
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
pub type wchar_t = libc::c_int;
pub type wint_t = libc::c_uint;
pub type mbstate_t = __mbstate_t;
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
pub const DO_TRADITIONAL: do_flag_values = 16;
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
pub const DO_LINT_OLD: do_flag_values = 8;
pub const DO_LINT_ALL: do_flag_values = 4;
pub const DO_LINT_EXTENSIONS: do_flag_values = 2;
pub const DO_LINT_INVALID: do_flag_values = 1;
pub const DO_FLAG_NONE: do_flag_values = 0;
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
unsafe extern "C" fn make_number_node(mut flags: libc::c_uint) -> *mut NODE {
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
        0 as libc::c_int,
        ::std::mem::size_of::<NODE>() as libc::c_ulong,
    );
    (*r).type_0 = Node_val;
    (*r).valref = 1 as libc::c_int as libc::c_long;
    (*r)
        .flags = (flags | MALLOC as libc::c_int as libc::c_uint
        | NUMBER as libc::c_int as libc::c_uint | NUMCUR as libc::c_int as libc::c_uint)
        as flagvals;
    return r;
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
unsafe extern "C" fn btowc_unused(mut __c: libc::c_int) -> wint_t {
    return if 0 != 0 && __c >= '\0' as i32 && __c <= '\u{7f}' as i32 {
        __c as wint_t
    } else {
panic!()
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
pub static mut make_number: Option::<
    unsafe extern "C" fn(libc::c_double) -> *mut NODE,
> = unsafe { Some(r_make_number as unsafe extern "C" fn(libc::c_double) -> *mut NODE) };
pub static mut str2number: Option::<unsafe extern "C" fn(*mut NODE) -> *mut NODE> = unsafe {
    Some(r_force_number as unsafe extern "C" fn(*mut NODE) -> *mut NODE)
};
pub static mut format_val: Option::<
    unsafe extern "C" fn(*const libc::c_char, libc::c_int, *mut NODE) -> *mut NODE,
> = unsafe {
    Some(
        r_format_val
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
                *mut NODE,
            ) -> *mut NODE,
    )
};
pub static mut cmp_numbers: Option::<
    unsafe extern "C" fn(*const NODE, *const NODE) -> libc::c_int,
> = unsafe {
    Some(cmp_awknums as unsafe extern "C" fn(*const NODE, *const NODE) -> libc::c_int)
};
unsafe extern "C" fn is_hex(
    mut str: *const libc::c_char,
    mut cpend: *const libc::c_char,
) -> bool {
    if *str as libc::c_int == '-' as i32 || *str as libc::c_int == '+' as i32 {
        str = str.offset(1);
        str;
    }
    if str.offset(1 as libc::c_int as isize) < cpend
        && *str.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32
        && (*str.offset(1 as libc::c_int as isize) as libc::c_int == 'x' as i32
            || *str.offset(1 as libc::c_int as isize) as libc::c_int == 'X' as i32)
    {
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn r_force_number(mut n: *mut NODE) -> *mut NODE {
    let mut current_block: u64;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cpend: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut save: libc::c_char = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*n).type_0 as libc::c_uint == Node_elem_new as libc::c_int as libc::c_uint {
        (*n).type_0 = Node_val;
        (*n)
            .flags = ::std::mem::transmute::<
            libc::c_uint,
            flagvals,
        >((*n).flags as libc::c_uint & !(STRING as libc::c_int) as libc::c_uint);
        *((*n).sub.val.sp)
            .offset(0 as libc::c_int as isize) = '0' as i32 as libc::c_char;
        (*n).sub.val.slen = 1 as libc::c_int as size_t;
        return n;
    }
    if (*n).flags as libc::c_uint & NUMCUR as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        return n;
    }
    (*n)
        .flags = ::std::mem::transmute::<
        libc::c_uint,
        flagvals,
    >((*n).flags as libc::c_uint | NUMCUR as libc::c_int as libc::c_uint);
    (*n).sub.val.fltnum = 0.0f64;
    cp = (*n).sub.val.sp;
    cpend = cp.offset((*n).sub.val.slen as isize);
    while cp < cpend
        && *(*__ctype_b_loc()).offset(*cp as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        cp = cp.offset(1);
        cp;
    }
    if !(cp == cpend) {
        while *(*__ctype_b_loc())
            .offset(
                *cpend.offset(-(1 as libc::c_int) as isize) as libc::c_uchar
                    as libc::c_int as isize,
            ) as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            cpend = cpend.offset(-1);
            cpend;
        }
        if do_flags as libc::c_uint & DO_POSIX as libc::c_int as libc::c_uint == 0 {
            if is_alpha(*cp as libc::c_uchar as libc::c_int) {
                current_block = 6354158152682790757;
            } else if is_ieee_magic_val(cp) {
                if cpend == cp.offset(4 as libc::c_int as isize) {
                    (*n).sub.val.fltnum = get_ieee_magic_val(cp);
                    current_block = 11387045506869250490;
                } else {
                    current_block = 6354158152682790757;
                }
            } else {
                current_block = 13797916685926291137;
            }
        } else {
            current_block = 13797916685926291137;
        }
        match current_block {
            6354158152682790757 => {}
            _ => {
                match current_block {
                    13797916685926291137 => {
                        if do_flags as libc::c_uint
                            & DO_POSIX as libc::c_int as libc::c_uint == 0
                            && (is_alpha(*cp as libc::c_uchar as libc::c_int)
                                as libc::c_int != 0
                                || do_flags as libc::c_uint
                                    & DO_NON_DEC_DATA as libc::c_int as libc::c_uint == 0
                                    && is_hex(cp, cpend) as libc::c_int != 0)
                        {
                            current_block = 6354158152682790757;
                        } else if cpend.offset_from(cp) as libc::c_long
                            == 1 as libc::c_int as libc::c_long
                        {
                            if *(*__ctype_b_loc())
                                .offset(*cp as libc::c_uchar as libc::c_int as isize)
                                as libc::c_int
                                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                != 0
                            {
                                (*n)
                                    .sub
                                    .val
                                    .fltnum = (*cp as libc::c_int - '0' as i32)
                                    as libc::c_double;
                                if (*n).sub.val.slen == 1 as libc::c_int as libc::c_ulong {
                                    (*n)
                                        .flags = ::std::mem::transmute::<
                                        libc::c_uint,
                                        flagvals,
                                    >(
                                        (*n).flags as libc::c_uint
                                            | NUMINT as libc::c_int as libc::c_uint,
                                    );
                                }
                                current_block = 11387045506869250490;
                            } else {
                                current_block = 6354158152682790757;
                            }
                        } else {
                            *__errno_location() = 0 as libc::c_int;
                            if do_flags as libc::c_uint
                                & DO_NON_DEC_DATA as libc::c_int as libc::c_uint != 0
                                && do_flags as libc::c_uint
                                    & DO_TRADITIONAL as libc::c_int as libc::c_uint == 0
                                && get_numbase(
                                    cp,
                                    cpend.offset_from(cp) as libc::c_long as size_t,
                                    1 as libc::c_int != 0,
                                ) != 10 as libc::c_int
                            {
                                (*n)
                                    .sub
                                    .val
                                    .fltnum = nondec2awknum(
                                    cp,
                                    cpend.offset_from(cp) as libc::c_long as size_t,
                                    &mut ptr,
                                );
                            } else {
                                save = *cpend;
                                *cpend = '\0' as i32 as libc::c_char;
                                (*n)
                                    .sub
                                    .val
                                    .fltnum = strtod(cp as *const libc::c_char, &mut ptr);
                                *cpend = save;
                            }
                            if *__errno_location() == 0 as libc::c_int
                                || *__errno_location() == 34 as libc::c_int
                            {
                                *__errno_location() = 0 as libc::c_int;
                                if ptr == cpend {
                                    current_block = 11387045506869250490;
                                } else {
                                    current_block = 6354158152682790757;
                                }
                            } else {
                                *__errno_location() = 0 as libc::c_int;
                                (*n).sub.val.fltnum = 0 as libc::c_int as libc::c_double;
                                current_block = 6354158152682790757;
                            }
                        }
                    }
                    _ => {}
                }
                match current_block {
                    6354158152682790757 => {}
                    _ => {
                        if ((*n).sub.val.fltnum).is_nan() as i32 != 0
                            && *cp as libc::c_int == '-' as i32
                            && ((*n).sub.val.fltnum).is_sign_negative() as libc::c_int
                                == 0 as libc::c_int
                        {
                            (*n).sub.val.fltnum = -(*n).sub.val.fltnum;
                        }
                        if (*n).flags as libc::c_uint
                            & USER_INPUT as libc::c_int as libc::c_uint
                            != 0 as libc::c_int as libc::c_uint
                        {
                            (*n)
                                .flags = ::std::mem::transmute::<
                                libc::c_uint,
                                flagvals,
                            >(
                                (*n).flags as libc::c_uint
                                    & !(STRING as libc::c_int) as libc::c_uint,
                            );
                            (*n)
                                .flags = ::std::mem::transmute::<
                                libc::c_uint,
                                flagvals,
                            >(
                                (*n).flags as libc::c_uint
                                    | NUMBER as libc::c_int as libc::c_uint,
                            );
                        }
                        return n;
                    }
                }
            }
        }
    }
    (*n)
        .flags = ::std::mem::transmute::<
        libc::c_uint,
        flagvals,
    >((*n).flags as libc::c_uint & !(USER_INPUT as libc::c_int) as libc::c_uint);
    return n;
}
static mut values: [*const libc::c_char; 10] = [
    b"0\0" as *const u8 as *const libc::c_char,
    b"1\0" as *const u8 as *const libc::c_char,
    b"2\0" as *const u8 as *const libc::c_char,
    b"3\0" as *const u8 as *const libc::c_char,
    b"4\0" as *const u8 as *const libc::c_char,
    b"5\0" as *const u8 as *const libc::c_char,
    b"6\0" as *const u8 as *const libc::c_char,
    b"7\0" as *const u8 as *const libc::c_char,
    b"8\0" as *const u8 as *const libc::c_char,
    b"9\0" as *const u8 as *const libc::c_char,
];
pub unsafe extern "C" fn r_format_val(
    mut format: *const libc::c_char,
    mut index: libc::c_int,
    mut s: *mut NODE,
) -> *mut NODE {
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut sp: *mut libc::c_char = buf.as_mut_ptr();
    let mut val: libc::c_double = 0.;
    if out_of_range(s) {
        let mut result: *const libc::c_char = format_nan_inf(
            s,
            'g' as i32 as libc::c_char,
        );
        return make_str_node(result, strlen(result), 0 as libc::c_int);
    } else {
        val = double_to_int((*s).sub.val.fltnum);
        if val != (*s).sub.val.fltnum
            || val
                <= (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                    as libc::c_double
            || val >= 9223372036854775807 as libc::c_long as libc::c_double
        {
            let mut dummy: [*mut NODE; 2] = [0 as *mut NODE; 2];
            let mut r: *mut NODE = 0 as *mut NODE;
            let mut oflags: libc::c_uint = 0;
            dummy[1 as libc::c_int as usize] = s;
            oflags = (*s).flags as libc::c_uint;
            if val == (*s).sub.val.fltnum {
                r = format_tree(
                    b"%.0f\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as size_t,
                    dummy.as_mut_ptr(),
                    2 as libc::c_int as libc::c_long,
                );
                (*s).sub.val.idx = -(1 as libc::c_int);
            } else {
                r = format_tree(
                    format,
                    (**fmt_list.offset(index as isize)).sub.val.slen,
                    dummy.as_mut_ptr(),
                    2 as libc::c_int as libc::c_long,
                );
                (*s).sub.val.idx = index;
            }
            (*s).flags = oflags as flagvals;
            (*s).sub.val.slen = (*r).sub.val.slen;
            if (*s).flags as libc::c_uint
                & (MALLOC as libc::c_int | STRCUR as libc::c_int) as libc::c_uint
                == (MALLOC as libc::c_int | STRCUR as libc::c_int) as libc::c_uint
            {
                free((*s).sub.val.sp as *mut libc::c_void);
            }
            (*s).sub.val.sp = (*r).sub.val.sp;
            let ref mut fresh0 = (*(r as *mut block_item)).freep;
            *fresh0 = nextfree[BLOCK_NODE as libc::c_int as usize].freep;
            nextfree[BLOCK_NODE as libc::c_int as usize].freep = r as *mut block_item;
        } else {
            let mut num: libc::c_long = val as libc::c_long;
            if (num as libc::c_ulong)
                < (::std::mem::size_of::<[*const libc::c_char; 10]>() as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                    ) && num >= 0 as libc::c_int as libc::c_long
            {
                sp = values[num as usize] as *mut libc::c_char;
                (*s).sub.val.slen = 1 as libc::c_int as size_t;
            } else {
                sprintf(sp, b"%ld\0" as *const u8 as *const libc::c_char, num);
                (*s).sub.val.slen = strlen(sp);
            }
            (*s).sub.val.idx = -(1 as libc::c_int);
            if (*s).flags as libc::c_uint & INTIND as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
            {
                (*s)
                    .flags = ::std::mem::transmute::<
                    libc::c_uint,
                    flagvals,
                >(
                    (*s).flags as libc::c_uint
                        & !(INTIND as libc::c_int | NUMBER as libc::c_int)
                            as libc::c_uint,
                );
                (*s)
                    .flags = ::std::mem::transmute::<
                    libc::c_uint,
                    flagvals,
                >((*s).flags as libc::c_uint | STRING as libc::c_int as libc::c_uint);
            }
            if (*s).flags as libc::c_uint
                & (MALLOC as libc::c_int | STRCUR as libc::c_int) as libc::c_uint
                == (MALLOC as libc::c_int | STRCUR as libc::c_int) as libc::c_uint
            {
                free((*s).sub.val.sp as *mut libc::c_void);
            }
            (*s)
                .sub
                .val
                .sp = emalloc_real(
                ((*s).sub.val.slen).wrapping_add(1 as libc::c_int as libc::c_ulong),
                b"format_val\0" as *const u8 as *const libc::c_char,
                b"s->stptr\0" as *const u8 as *const libc::c_char,
                b"node.c\0" as *const u8 as *const libc::c_char,
                300 as libc::c_int,
            ) as *mut libc::c_char;
            memcpy(
                (*s).sub.val.sp as *mut libc::c_void,
                sp as *const libc::c_void,
                ((*s).sub.val.slen).wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
        }
        (*s)
            .flags = ::std::mem::transmute::<
            libc::c_uint,
            flagvals,
        >((*s).flags as libc::c_uint | STRCUR as libc::c_int as libc::c_uint);
        if (*s).flags as libc::c_uint & WSTRCUR as libc::c_int as libc::c_uint != 0 {
            r_free_wstr(s);
        }
        return s;
    };
}
pub unsafe extern "C" fn r_dupnode(mut n: *mut NODE) -> *mut NODE {
    let mut r: *mut NODE = 0 as *mut NODE;
    r = nextfree[BLOCK_NODE as libc::c_int as usize].freep as *mut NODE;
    if !r.is_null() {
        nextfree[BLOCK_NODE as libc::c_int as usize]
            .freep = (*(r as *mut block_item)).freep;
    } else {
        r = more_blocks(BLOCK_NODE as libc::c_int) as *mut NODE;
    };
    *r = *n;
    (*r)
        .flags = ::std::mem::transmute::<
        libc::c_uint,
        flagvals,
    >((*r).flags as libc::c_uint | MALLOC as libc::c_int as libc::c_uint);
    (*r).valref = 1 as libc::c_int as libc::c_long;
    (*r).sub.val.wsp = 0 as *mut wchar_t;
    (*r).sub.val.wslen = 0 as libc::c_int as size_t;
    if (*n).flags as libc::c_uint & STRCUR as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        (*r)
            .sub
            .val
            .sp = emalloc_real(
            ((*n).sub.val.slen).wrapping_add(1 as libc::c_int as libc::c_ulong),
            b"r_dupnode\0" as *const u8 as *const libc::c_char,
            b"r->stptr\0" as *const u8 as *const libc::c_char,
            b"node.c\0" as *const u8 as *const libc::c_char,
            349 as libc::c_int,
        ) as *mut libc::c_char;
        memcpy(
            (*r).sub.val.sp as *mut libc::c_void,
            (*n).sub.val.sp as *const libc::c_void,
            (*n).sub.val.slen,
        );
        *((*r).sub.val.sp)
            .offset((*n).sub.val.slen as isize) = '\0' as i32 as libc::c_char;
        (*r).sub.val.slen = (*n).sub.val.slen;
        if (*n).flags as libc::c_uint & WSTRCUR as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            (*r).sub.val.wslen = (*n).sub.val.wslen;
            (*r)
                .sub
                .val
                .wsp = emalloc_real(
                (::std::mem::size_of::<wchar_t>() as libc::c_ulong)
                    .wrapping_mul(
                        ((*n).sub.val.wslen)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ),
                b"r_dupnode\0" as *const u8 as *const libc::c_char,
                b"r->wstptr\0" as *const u8 as *const libc::c_char,
                b"node.c\0" as *const u8 as *const libc::c_char,
                355 as libc::c_int,
            ) as *mut wchar_t;
            memcpy(
                (*r).sub.val.wsp as *mut libc::c_void,
                (*n).sub.val.wsp as *const libc::c_void,
                ((*n).sub.val.wslen)
                    .wrapping_mul(::std::mem::size_of::<wchar_t>() as libc::c_ulong),
            );
            *((*r).sub.val.wsp).offset((*n).sub.val.wslen as isize) = '\0' as i32;
            (*r)
                .flags = ::std::mem::transmute::<
                libc::c_uint,
                flagvals,
            >((*r).flags as libc::c_uint | WSTRCUR as libc::c_int as libc::c_uint);
        }
    }
    return r;
}
unsafe extern "C" fn r_make_number(mut x: libc::c_double) -> *mut NODE {
    let mut r: *mut NODE = make_number_node(0 as libc::c_int as libc::c_uint);
    (*r).sub.val.fltnum = x;
    return r;
}
pub unsafe extern "C" fn cmp_awknums(
    mut t1: *const NODE,
    mut t2: *const NODE,
) -> libc::c_int {
    if ((*t1).sub.val.fltnum).is_nan() as i32 != 0 {
        return (((*t2).sub.val.fltnum).is_nan() as i32 == 0) as libc::c_int;
    }
    if ((*t2).sub.val.fltnum).is_nan() as i32 != 0 {
        return -(1 as libc::c_int);
    }
    if (*t1).sub.val.fltnum == (*t2).sub.val.fltnum {
        return 0 as libc::c_int;
    }
    if (*t1).sub.val.fltnum < (*t2).sub.val.fltnum {
        return -(1 as libc::c_int);
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn make_str_node(
    mut s: *const libc::c_char,
    mut len: size_t,
    mut flags: libc::c_int,
) -> *mut NODE {
    let mut r: *mut NODE = 0 as *mut NODE;
    r = nextfree[BLOCK_NODE as libc::c_int as usize].freep as *mut NODE;
    if !r.is_null() {
        nextfree[BLOCK_NODE as libc::c_int as usize]
            .freep = (*(r as *mut block_item)).freep;
    } else {
        r = more_blocks(BLOCK_NODE as libc::c_int) as *mut NODE;
    };
    (*r).type_0 = Node_val;
    (*r).sub.val.fltnum = 0 as libc::c_int as libc::c_double;
    (*r)
        .flags = (MALLOC as libc::c_int | STRING as libc::c_int | STRCUR as libc::c_int)
        as flagvals;
    (*r).valref = 1 as libc::c_int as libc::c_long;
    (*r).sub.val.idx = -(1 as libc::c_int);
    (*r).sub.val.wsp = 0 as *mut wchar_t;
    (*r).sub.val.wslen = 0 as libc::c_int as size_t;
    if flags & 2 as libc::c_int != 0 as libc::c_int {
        (*r).sub.val.sp = s as *mut libc::c_char;
    } else {
        (*r)
            .sub
            .val
            .sp = emalloc_real(
            len.wrapping_add(1 as libc::c_int as libc::c_ulong),
            b"make_str_node\0" as *const u8 as *const libc::c_char,
            b"r->stptr\0" as *const u8 as *const libc::c_char,
            b"node.c\0" as *const u8 as *const libc::c_char,
            422 as libc::c_int,
        ) as *mut libc::c_char;
        memcpy((*r).sub.val.sp as *mut libc::c_void, s as *const libc::c_void, len);
    }
    *((*r).sub.val.sp).offset(len as isize) = '\0' as i32 as libc::c_char;
    if flags & 1 as libc::c_int != 0 as libc::c_int {
        let mut pf: *const libc::c_char = 0 as *const libc::c_char;
        let mut ptm: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut c: libc::c_int = 0;
        let mut end: *const libc::c_char = 0 as *const libc::c_char;
        let mut cur_state: mbstate_t = mbstate_t {
            __count: 0,
            __value: C2RustUnnamed { __wch: 0 },
        };
        memset(
            &mut cur_state as *mut mbstate_t as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
        );
        end = &mut *((*r).sub.val.sp).offset(len as isize) as *mut libc::c_char;
        ptm = (*r).sub.val.sp;
        pf = ptm;
        while pf < end {
            if gawk_mb_cur_max > 1 as libc::c_int {
                let mut mblen: libc::c_int = mbrlen(
                    pf,
                    end.offset_from(pf) as libc::c_long as size_t,
                    &mut cur_state,
                ) as libc::c_int;
                if mblen > 1 as libc::c_int {
                    let mut i: libc::c_int = 0;
                    i = 0 as libc::c_int;
                    while i < mblen {
                        let fresh1 = pf;
                        pf = pf.offset(1);
                        let fresh2 = ptm;
                        ptm = ptm.offset(1);
                        *fresh2 = *fresh1;
                        i += 1;
                        i;
                    }
                    continue;
                }
            }
            let fresh3 = pf;
            pf = pf.offset(1);
            c = *fresh3 as libc::c_int;
            if c == '\\' as i32 {
                c = parse_escape(&mut pf);
                if c < 0 as libc::c_int {
                    if do_flags as libc::c_uint
                        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                            as libc::c_uint != 0
                    {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"node.c\0" as *const u8 as *const libc::c_char,
                            460 as libc::c_int,
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
                    if flags & 4 as libc::c_int != 0 as libc::c_int {
                        continue;
                    }
                    c = '\\' as i32;
                }
                let fresh4 = ptm;
                ptm = ptm.offset(1);
                *fresh4 = c as libc::c_char;
            } else {
                let fresh5 = ptm;
                ptm = ptm.offset(1);
                *fresh5 = c as libc::c_char;
            }
        }
        len = ptm.offset_from((*r).sub.val.sp) as libc::c_long as size_t;
        (*r)
            .sub
            .val
            .sp = erealloc_real(
            (*r).sub.val.sp as *mut libc::c_void,
            len.wrapping_add(1 as libc::c_int as libc::c_ulong),
            b"make_str_node\0" as *const u8 as *const libc::c_char,
            b"r->stptr\0" as *const u8 as *const libc::c_char,
            b"node.c\0" as *const u8 as *const libc::c_char,
            470 as libc::c_int,
        ) as *mut libc::c_char;
        *((*r).sub.val.sp).offset(len as isize) = '\0' as i32 as libc::c_char;
    }
    (*r).sub.val.slen = len;
    return r;
}
pub unsafe extern "C" fn make_typed_regex(
    mut re: *const libc::c_char,
    mut len: size_t,
) -> *mut NODE {
    let mut n: *mut NODE = 0 as *mut NODE;
    let mut exp: *mut NODE = 0 as *mut NODE;
    let mut n2: *mut NODE = 0 as *mut NODE;
    exp = make_str_node(re, len, 2 as libc::c_int);
    n = make_regnode(Node_regex, exp);
    if n.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"node.c\0" as *const u8 as *const libc::c_char,
            488 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"could not make typed regex\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    n2 = make_str_node(re, len, 0 as libc::c_int);
    (*n2).sub.val.typre = n;
    (*n2).sub.val.fltnum = 0 as libc::c_int as libc::c_double;
    (*n2)
        .flags = ::std::mem::transmute::<
        libc::c_uint,
        flagvals,
    >(
        (*n2).flags as libc::c_uint
            | (NUMCUR as libc::c_int | STRCUR as libc::c_int | REGEX as libc::c_int)
                as libc::c_uint,
    );
    (*n2)
        .flags = ::std::mem::transmute::<
        libc::c_uint,
        flagvals,
    >(
        (*n2).flags as libc::c_uint
            & !(STRING as libc::c_int | NUMBER as libc::c_int) as libc::c_uint,
    );
    return n2;
}
pub unsafe extern "C" fn r_unref(mut tmp: *mut NODE) {
    if (*tmp).flags as libc::c_uint
        & (MALLOC as libc::c_int | STRCUR as libc::c_int) as libc::c_uint
        == (MALLOC as libc::c_int | STRCUR as libc::c_int) as libc::c_uint
    {
        free((*tmp).sub.val.sp as *mut libc::c_void);
    }
    mpfr_unset(tmp);
    if (*tmp).flags as libc::c_uint & WSTRCUR as libc::c_int as libc::c_uint != 0 {
        r_free_wstr(tmp);
    }
    let ref mut fresh6 = (*(tmp as *mut block_item)).freep;
    *fresh6 = nextfree[BLOCK_NODE as libc::c_int as usize].freep;
    nextfree[BLOCK_NODE as libc::c_int as usize].freep = tmp as *mut block_item;
}
pub unsafe extern "C" fn parse_escape(
    mut string_ptr: *mut *const libc::c_char,
) -> libc::c_int {
    let fresh7 = *string_ptr;
    *string_ptr = (*string_ptr).offset(1);
    let mut c: libc::c_int = *fresh7 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    if do_flags as libc::c_uint & DO_LINT_OLD as libc::c_int as libc::c_uint != 0 {
        match c {
            97 | 98 | 102 | 114 => {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"node.c\0" as *const u8 as *const libc::c_char,
                    561 as libc::c_int,
                );
                (Some(lintfunc.unwrap()))
                    .unwrap()(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"old awk does not support the `\\%c' escape sequence\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    c,
                );
            }
            _ => {}
        }
    }
    match c {
        97 => return '\u{7}' as i32,
        98 => return '\u{8}' as i32,
        102 => return '\u{c}' as i32,
        110 => return '\n' as i32,
        114 => return '\r' as i32,
        116 => return '\t' as i32,
        118 => return '\u{b}' as i32,
        10 => return -(2 as libc::c_int),
        0 => {
            *string_ptr = (*string_ptr).offset(-1);
            *string_ptr;
            return -(1 as libc::c_int);
        }
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 => {
            i = c - '0' as i32;
            count = 0 as libc::c_int;
            loop {
                count += 1;
                if !(count < 3 as libc::c_int) {
                    break;
                }
                let fresh8 = *string_ptr;
                *string_ptr = (*string_ptr).offset(1);
                c = *fresh8 as libc::c_int;
                if c >= '0' as i32 && c <= '7' as i32 {
                    i *= 8 as libc::c_int;
                    i += c - '0' as i32;
                } else {
                    *string_ptr = (*string_ptr).offset(-1);
                    *string_ptr;
                    break;
                }
            }
            return i;
        }
        120 => {
            if do_flags as libc::c_uint
                & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                    as libc::c_uint != 0
            {
                static mut warned: bool = 0 as libc::c_int != 0;
                if !warned {
                    warned = 1 as libc::c_int != 0;
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"node.c\0" as *const u8 as *const libc::c_char,
                        612 as libc::c_int,
                    );
                    (Some(lintfunc.unwrap()))
                        .unwrap()(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"POSIX does not allow `\\x' escapes\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
            }
            if do_flags as libc::c_uint & DO_POSIX as libc::c_int as libc::c_uint != 0 {
                return 'x' as i32;
            }
            if *(*__ctype_b_loc())
                .offset(
                    *(*string_ptr).offset(0 as libc::c_int as isize) as libc::c_uchar
                        as libc::c_int as isize,
                ) as libc::c_int
                & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
            {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"node.c\0" as *const u8 as *const libc::c_char,
                    618 as libc::c_int,
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
                        b"no hex digits in `\\x' escape sequence\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                return 'x' as i32;
            }
            start = *string_ptr;
            j = 0 as libc::c_int;
            i = j;
            while j < 2 as libc::c_int {
                let fresh9 = *string_ptr;
                *string_ptr = (*string_ptr).offset(1);
                c = *fresh9 as libc::c_uchar as libc::c_int;
                if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                    & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    i *= 16 as libc::c_int;
                    if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                    {
                        i += c - '0' as i32;
                    } else if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                        & _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0
                    {
                        i += c - 'A' as i32 + 10 as libc::c_int;
                    } else {
                        i += c - 'a' as i32 + 10 as libc::c_int;
                    }
                    j += 1;
                    j;
                } else {
                    *string_ptr = (*string_ptr).offset(-1);
                    *string_ptr;
                    break;
                }
            }
            if do_flags as libc::c_uint
                & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                    as libc::c_uint != 0 && j == 2 as libc::c_int
                && *(*__ctype_b_loc())
                    .offset(**string_ptr as libc::c_uchar as libc::c_int as isize)
                    as libc::c_int
                    & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"node.c\0" as *const u8 as *const libc::c_char,
                    639 as libc::c_int,
                );
                (Some(lintfunc.unwrap()))
                    .unwrap()(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"hex escape \\x%.*s of %d characters probably not interpreted the way you expect\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    3 as libc::c_int,
                    start,
                    3 as libc::c_int,
                );
            }
            return i;
        }
        92 | 34 => return c,
        _ => {
            static mut warned_0: [bool; 256] = [false; 256];
            let mut uc: libc::c_uchar = c as libc::c_uchar;
            if !warned_0[uc as usize] {
                warned_0[uc as usize] = 1 as libc::c_int != 0;
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"node.c\0" as *const u8 as *const libc::c_char,
                    654 as libc::c_int,
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
                        b"escape sequence `\\%c' treated as plain `%c'\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    uc as libc::c_int,
                    uc as libc::c_int,
                );
            }
            return c;
        }
    };
}
pub unsafe extern "C" fn get_numbase(
    mut s: *const libc::c_char,
    mut len: size_t,
    mut use_locale: bool,
) -> libc::c_int {
    let mut dec_point: libc::c_int = '.' as i32;
    let mut str: *const libc::c_char = s;
    if use_locale as libc::c_int != 0 && !(loc.decimal_point).is_null()
        && *(loc.decimal_point).offset(0 as libc::c_int as isize) as libc::c_int
            != '\0' as i32
    {
        dec_point = *(loc.decimal_point).offset(0 as libc::c_int as isize)
            as libc::c_int;
    }
    if len < 2 as libc::c_int as libc::c_ulong
        || *str.offset(0 as libc::c_int as isize) as libc::c_int != '0' as i32
    {
        return 10 as libc::c_int;
    }
    if *str.offset(1 as libc::c_int as isize) as libc::c_int == 'x' as i32
        || *str.offset(1 as libc::c_int as isize) as libc::c_int == 'X' as i32
    {
        return 16 as libc::c_int;
    }
    while len > 0 as libc::c_int as libc::c_ulong {
        if *str as libc::c_int == 'e' as i32 || *str as libc::c_int == 'E' as i32
            || *str as libc::c_int == dec_point
        {
            return 10 as libc::c_int
        } else {
            if *(*__ctype_b_loc()).offset(*str as libc::c_uchar as libc::c_int as isize)
                as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                == 0
            {
                break;
            }
            len = len.wrapping_sub(1);
            len;
            str = str.offset(1);
            str;
        }
    }
    if *(*__ctype_b_loc())
        .offset(
            *s.offset(1 as libc::c_int as isize) as libc::c_uchar as libc::c_int as isize,
        ) as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
        || *s.offset(1 as libc::c_int as isize) as libc::c_int == '8' as i32
        || *s.offset(1 as libc::c_int as isize) as libc::c_int == '9' as i32
    {
        return 10 as libc::c_int;
    }
    return 8 as libc::c_int;
}
pub unsafe extern "C" fn str2wstr(
    mut n: *mut NODE,
    mut ptr: *mut *mut size_t,
) -> *mut NODE {
    let mut i: size_t = 0;
    let mut count: size_t = 0;
    let mut src_count: size_t = 0;
    let mut sp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mbs: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    let mut wc: wchar_t = 0;
    let mut wsp: *mut wchar_t = 0 as *mut wchar_t;
    static mut warned: bool = 0 as libc::c_int != 0;
    if n == Nnull_string || n == Null_field {
        return n;
    }
    if (*n).flags as libc::c_uint & WSTRCUR as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        if ptr.is_null() {
            return n;
        }
        if (*n).flags as libc::c_uint & WSTRCUR as libc::c_int as libc::c_uint != 0 {
            r_free_wstr(n);
        }
    }
    (*n)
        .sub
        .val
        .wsp = emalloc_real(
        (::std::mem::size_of::<wchar_t>() as libc::c_ulong)
            .wrapping_mul(
                ((*n).sub.val.slen).wrapping_add(1 as libc::c_int as libc::c_ulong),
            ),
        b"str2wstr\0" as *const u8 as *const libc::c_char,
        b"n->wstptr\0" as *const u8 as *const libc::c_char,
        b"node.c\0" as *const u8 as *const libc::c_char,
        748 as libc::c_int,
    ) as *mut wchar_t;
    wsp = (*n).sub.val.wsp;
    if !ptr.is_null() {
        *ptr = ezalloc_real(
            (::std::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_mul((*n).sub.val.slen),
            b"str2wstr\0" as *const u8 as *const libc::c_char,
            b"*ptr\0" as *const u8 as *const libc::c_char,
            b"node.c\0" as *const u8 as *const libc::c_char,
            760 as libc::c_int,
        ) as *mut size_t;
    }
    sp = (*n).sub.val.sp;
    src_count = (*n).sub.val.slen;
    memset(
        &mut mbs as *mut mbstate_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
    );
    i = 0 as libc::c_int as size_t;
    while src_count > 0 as libc::c_int as libc::c_ulong {
        if *btowc_cache
            .as_mut_ptr()
            .offset((*sp as libc::c_int & 0xff as libc::c_int) as isize)
            != 0xffffffff as libc::c_uint
        {
            count = 1 as libc::c_int as size_t;
            wc = *btowc_cache
                .as_mut_ptr()
                .offset((*sp as libc::c_int & 0xff as libc::c_int) as isize) as wchar_t;
        } else {
            count = mbrtowc(&mut wc, sp, src_count, &mut mbs);
        }
        let mut current_block_42: u64;
        match count {
            18446744073709551614 | 18446744073709551615 => {
                memset(
                    &mut mbs as *mut mbstate_t as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
                );
                if !warned {
                    warned = 1 as libc::c_int != 0;
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"node.c\0" as *const u8 as *const libc::c_char,
                        790 as libc::c_int,
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
                            b"Invalid multibyte data detected. There may be a mismatch between your data and your locale\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                if using_utf8() {
                    count = 1 as libc::c_int as size_t;
                    wc = 0xfffd as libc::c_int;
                    current_block_42 = 3483034381522484843;
                } else {
                    sp = sp.offset(1);
                    sp;
                    src_count = src_count.wrapping_sub(1);
                    src_count;
                    current_block_42 = 9007357115414505193;
                }
            }
            0 => {
                count = 1 as libc::c_int as size_t;
                current_block_42 = 3483034381522484843;
            }
            _ => {
                current_block_42 = 3483034381522484843;
            }
        }
        match current_block_42 {
            3483034381522484843 => {
                let fresh10 = wsp;
                wsp = wsp.offset(1);
                *fresh10 = wc;
                src_count = (src_count as libc::c_ulong).wrapping_sub(count) as size_t
                    as size_t;
                loop {
                    let fresh11 = count;
                    count = count.wrapping_sub(1);
                    if !(fresh11 != 0) {
                        break;
                    }
                    if !ptr.is_null() {
                        *(*ptr)
                            .offset(
                                sp.offset_from((*n).sub.val.sp) as libc::c_long as isize,
                            ) = i;
                    }
                    sp = sp.offset(1);
                    sp;
                }
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    *wsp = '\0' as i32;
    (*n).sub.val.wslen = wsp.offset_from((*n).sub.val.wsp) as libc::c_long as size_t;
    (*n)
        .flags = ::std::mem::transmute::<
        libc::c_uint,
        flagvals,
    >((*n).flags as libc::c_uint | WSTRCUR as libc::c_int as libc::c_uint);
    if ((*n).sub.val.slen).wrapping_sub((*n).sub.val.wslen)
        > 100 as libc::c_int as libc::c_ulong
    {
        (*n)
            .sub
            .val
            .wsp = erealloc_real(
            (*n).sub.val.wsp as *mut libc::c_void,
            (::std::mem::size_of::<wchar_t>() as libc::c_ulong)
                .wrapping_mul(
                    ((*n).sub.val.wslen).wrapping_add(1 as libc::c_int as libc::c_ulong),
                ),
            b"str2wstr\0" as *const u8 as *const libc::c_char,
            b"n->wstptr\0" as *const u8 as *const libc::c_char,
            b"node.c\0" as *const u8 as *const libc::c_char,
            837 as libc::c_int,
        ) as *mut wchar_t;
    }
    return n;
}
pub unsafe extern "C" fn wstr2str(mut n: *mut NODE) -> *mut NODE {
    let mut result: size_t = 0;
    let mut length: size_t = 0;
    let mut wp: *mut wchar_t = 0 as *mut wchar_t;
    let mut mbs: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    let mut newval: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    memset(
        &mut mbs as *mut mbstate_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
    );
    length = (*n).sub.val.wslen;
    newval = emalloc_real(
        length
            .wrapping_mul(gawk_mb_cur_max as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
        b"wstr2str\0" as *const u8 as *const libc::c_char,
        b"newval\0" as *const u8 as *const libc::c_char,
        b"node.c\0" as *const u8 as *const libc::c_char,
        864 as libc::c_int,
    ) as *mut libc::c_char;
    wp = (*n).sub.val.wsp;
    cp = newval;
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
    free((*n).sub.val.sp as *mut libc::c_void);
    (*n).sub.val.sp = newval;
    (*n).sub.val.slen = cp.offset_from(newval) as libc::c_long as size_t;
    return n;
}
pub unsafe extern "C" fn r_free_wstr(mut n: *mut NODE) {
    if (*n).flags as libc::c_uint & WSTRCUR as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        free((*n).sub.val.wsp as *mut libc::c_void);
    }
    (*n).sub.val.wsp = 0 as *mut wchar_t;
    (*n).sub.val.wslen = 0 as libc::c_int as size_t;
    (*n)
        .flags = ::std::mem::transmute::<
        libc::c_uint,
        flagvals,
    >((*n).flags as libc::c_uint & !(WSTRCUR as libc::c_int) as libc::c_uint);
}
pub unsafe extern "C" fn wstrstr(
    mut haystack: *const wchar_t,
    mut hs_len: size_t,
    mut needle: *const wchar_t,
    mut needle_len: size_t,
) -> *const wchar_t {
    let mut i: size_t = 0;
    if haystack.is_null() || needle.is_null() || needle_len > hs_len {
        return 0 as *const wchar_t;
    }
    i = 0 as libc::c_int as size_t;
    while i < hs_len {
        if *haystack.offset(i as isize) == *needle.offset(0 as libc::c_int as isize)
            && i.wrapping_add(needle_len).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                < hs_len
            && *haystack
                .offset(
                    i
                        .wrapping_add(needle_len)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                )
                == *needle
                    .offset(
                        needle_len.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    )
        {
            if memcmp(
                haystack.offset(i as isize) as *const libc::c_void,
                needle as *const libc::c_void,
                (::std::mem::size_of::<wchar_t>() as libc::c_ulong)
                    .wrapping_mul(needle_len),
            ) == 0 as libc::c_int
            {
                return haystack.offset(i as isize);
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as *const wchar_t;
}
pub unsafe extern "C" fn wcasestrstr(
    mut haystack: *const wchar_t,
    mut hs_len: size_t,
    mut needle: *const wchar_t,
    mut needle_len: size_t,
) -> *const wchar_t {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    if haystack.is_null() || needle.is_null() || needle_len > hs_len {
        return 0 as *const wchar_t;
    }
    i = 0 as libc::c_int as size_t;
    while i < hs_len {
        let mut current_block_8: u64;
        if towlower(*haystack.offset(i as isize) as wint_t)
            == towlower(*needle.offset(0 as libc::c_int as isize) as wint_t)
            && i.wrapping_add(needle_len).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                < hs_len
            && towlower(
                *haystack
                    .offset(
                        i
                            .wrapping_add(needle_len)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ) as wint_t,
            )
                == towlower(
                    *needle
                        .offset(
                            needle_len.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        ) as wint_t,
                )
        {
            let mut start: *const wchar_t = 0 as *const wchar_t;
            start = haystack.offset(i as isize);
            j = 0 as libc::c_int as size_t;
            loop {
                if !(j < needle_len) {
                    current_block_8 = 1917311967535052937;
                    break;
                }
                let mut h: wchar_t = 0;
                let mut n: wchar_t = 0;
                h = towlower(*start as wint_t) as wchar_t;
                n = towlower(*needle.offset(j as isize) as wint_t) as wchar_t;
                if h != n {
                    current_block_8 = 1841672684692190573;
                    break;
                }
                j = j.wrapping_add(1);
                j;
                start = start.offset(1);
                start;
            }
            match current_block_8 {
                1841672684692190573 => {}
                _ => return haystack.offset(i as isize),
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as *const wchar_t;
}
pub unsafe extern "C" fn is_ieee_magic_val(mut val: *const libc::c_char) -> bool {
    return (*val.offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32
        || *val.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32)
        && ((*val.offset(1 as libc::c_int as isize) as libc::c_int == 'i' as i32
            || *val.offset(1 as libc::c_int as isize) as libc::c_int == 'I' as i32)
            && (*val.offset(2 as libc::c_int as isize) as libc::c_int == 'n' as i32
                || *val.offset(2 as libc::c_int as isize) as libc::c_int == 'N' as i32)
            && (*val.offset(3 as libc::c_int as isize) as libc::c_int == 'f' as i32
                || *val.offset(3 as libc::c_int as isize) as libc::c_int == 'F' as i32)
            || (*val.offset(1 as libc::c_int as isize) as libc::c_int == 'n' as i32
                || *val.offset(1 as libc::c_int as isize) as libc::c_int == 'N' as i32)
                && (*val.offset(2 as libc::c_int as isize) as libc::c_int == 'a' as i32
                    || *val.offset(2 as libc::c_int as isize) as libc::c_int
                        == 'A' as i32)
                && (*val.offset(3 as libc::c_int as isize) as libc::c_int == 'n' as i32
                    || *val.offset(3 as libc::c_int as isize) as libc::c_int
                        == 'N' as i32));
}
unsafe extern "C" fn get_ieee_magic_val(mut val: *mut libc::c_char) -> libc::c_double {
    static mut first: bool = 1 as libc::c_int != 0;
    static mut inf: libc::c_double = 0.;
    static mut nan: libc::c_double = 0.;
    let mut save: libc::c_char = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    save = *val.offset(4 as libc::c_int as isize);
    *val.offset(4 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    let mut v: libc::c_double = strtod(val, &mut ptr);
    *val.offset(4 as libc::c_int as isize) = save;
    if val == ptr {
        if first {
            first = 0 as libc::c_int != 0;
            nan = sqrt(-1.0f64);
            inf = -log(0.0f64);
        }
        v = if *val.offset(1 as libc::c_int as isize) as libc::c_int == 'i' as i32
            || *val.offset(1 as libc::c_int as isize) as libc::c_int == 'I' as i32
        {
            inf
        } else {
            nan
        };
        if *val.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32 {
            v = -v;
        }
    }
    return v;
}
pub static mut btowc_cache: [wint_t; 256] = [0; 256];
pub unsafe extern "C" fn init_btowc_cache() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i <= 255 as libc::c_int {
        btowc_cache[i as usize] = btowc(i);
        i += 1;
        i;
    }
}
pub static mut nextfree: [block_header; 2] = [
    {
        let mut init = block_header {
            freep: 0 as *const block_item as *mut block_item,
            size: ::std::mem::size_of::<NODE>() as libc::c_ulong,
            name: b"node\0" as *const u8 as *const libc::c_char,
            highwater: 0,
        };
        init
    },
    {
        let mut init = block_header {
            freep: 0 as *const block_item as *mut block_item,
            size: ::std::mem::size_of::<BUCKET>() as libc::c_ulong,
            name: b"bucket\0" as *const u8 as *const libc::c_char,
            highwater: 0,
        };
        init
    },
];
pub unsafe extern "C" fn more_blocks(mut id: libc::c_int) -> *mut libc::c_void {
    let mut freep: *mut block_item = 0 as *mut block_item;
    let mut np: *mut block_item = 0 as *mut block_item;
    let mut next: *mut block_item = 0 as *mut block_item;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut endp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: size_t = 0;
    size = nextfree[id as usize].size;
    freep = emalloc_real(
        (100 as libc::c_int as libc::c_ulong).wrapping_mul(size),
        b"more_blocks\0" as *const u8 as *const libc::c_char,
        b"freep\0" as *const u8 as *const libc::c_char,
        b"node.c\0" as *const u8 as *const libc::c_char,
        1075 as libc::c_int,
    ) as *mut block_item;
    p = freep as *mut libc::c_char;
    endp = p.offset((100 as libc::c_int as libc::c_ulong).wrapping_mul(size) as isize);
    np = freep;
    loop {
        p = p.offset(size as isize);
        next = p as *mut block_item;
        if p >= endp {
            (*np).freep = 0 as *mut block_item;
            break;
        } else {
            (*np).freep = next;
            np = next;
        }
    }
    nextfree[id as usize].freep = (*freep).freep;
    nextfree[id as usize].highwater += 100 as libc::c_int as libc::c_long;
    return freep as *mut libc::c_void;
}
pub unsafe extern "C" fn make_bool_node(mut value: bool) -> *mut NODE {
    let mut val: *mut NODE = 0 as *mut NODE;
    let mut sval: *const libc::c_char = 0 as *const libc::c_char;
    let mut nval: libc::c_double = 0.;
    sval = if value as libc::c_int != 0 {
        b"1\0" as *const u8 as *const libc::c_char
    } else {
        b"0\0" as *const u8 as *const libc::c_char
    };
    nval = if value as libc::c_int != 0 { 1.0f64 } else { 0.0f64 };
    val = make_number.unwrap()(nval);
    (*val).sub.val.sp = estrdup(sval, strlen(sval));
    (*val).sub.val.slen = strlen(sval);
    (*val)
        .flags = ::std::mem::transmute::<
        libc::c_uint,
        flagvals,
    >(
        (*val).flags as libc::c_uint
            | (NUMCUR as libc::c_int | STRCUR as libc::c_int | BOOLVAL as libc::c_int)
                as libc::c_uint,
    );
    (*val).sub.val.idx = -(1 as libc::c_int);
    return val;
}
