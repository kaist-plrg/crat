use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type re_dfa_t;
    pub type dfa;
    pub type break_point;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn refree(rp: *mut Regexp);
    fn make_regexp(
        s: *const libc::c_char,
        len: size_t,
        ignorecase: bool,
        dfa: bool,
        canfatal: bool,
    ) -> *mut Regexp;
    fn re_update(t: *mut NODE) -> *mut Regexp;
    static mut lintfunc: Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>;
    fn r_free_wstr(n: *mut NODE);
    fn more_blocks(id: libc::c_int) -> *mut libc::c_void;
    fn research(
        rp: *mut Regexp,
        str: *mut libc::c_char,
        start: libc::c_int,
        len: size_t,
        flags: libc::c_int,
    ) -> libc::c_int;
    fn set_loc(file: *const libc::c_char, line: libc::c_int);
    fn r_fatal(mesg: *const libc::c_char, _: ...);
    fn r_dupnode(n: *mut NODE) -> *mut NODE;
    fn make_str_node(
        s: *const libc::c_char,
        len: size_t,
        flags: libc::c_int,
    ) -> *mut NODE;
    fn elem_new_to_scalar(n: *mut NODE) -> *mut NODE;
    fn check_args_min_max(
        nargs: libc::c_int,
        fname: *const libc::c_char,
        min: libc::c_int,
        max: libc::c_int,
    );
    fn check_symtab_functab(
        dest: *mut NODE,
        fname: *const libc::c_char,
        msg: *const libc::c_char,
    );
    fn array_vname(symbol: *const NODE) -> *const libc::c_char;
    fn force_array(symbol: *mut NODE, canfatal: bool) -> *mut NODE;
    fn r_unref(tmp: *mut NODE);
    static mut stack_ptr: *mut STACK_ITEM;
    static mut gawk_mb_cur_max: libc::c_int;
    static mut do_flags: do_flag_values;
    static mut nextfree: [block_header; 2];
    static mut format_val: Option::<
        unsafe extern "C" fn(*const libc::c_char, libc::c_int, *mut NODE) -> *mut NODE,
    >;
    static mut str2number: Option::<unsafe extern "C" fn(*mut NODE) -> *mut NODE>;
    static mut make_number: Option::<unsafe extern "C" fn(libc::c_double) -> *mut NODE>;
    static mut FPAT_node: *mut NODE;
    static mut PROCINFO_node: *mut NODE;
    static mut RS_node: *mut NODE;
    static mut NF_node: *mut NODE;
    static mut FS_node: *mut NODE;
    static mut FIELDWIDTHS_node: *mut NODE;
    static mut CONVFMTidx: libc::c_int;
    static mut CONVFMT: *const libc::c_char;
    static mut OFSlen: libc::c_int;
    static mut OFS: *mut libc::c_char;
    static mut RS_is_null: bool;
    static mut IGNORECASE: bool;
    static mut NF: libc::c_long;
    fn __mbrlen(__s: *const libc::c_char, __n: size_t, __ps: *mut mbstate_t) -> size_t;
    fn mbrtowc(
        __pwc: *mut wchar_t,
        __s: *const libc::c_char,
        __n: size_t,
        __p: *mut mbstate_t,
    ) -> size_t;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
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
pub type wchar_t = libc::c_int;
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
pub type Func_ptr = Option::<unsafe extern "C" fn() -> ()>;
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
pub type Setfunc = Option::<
    unsafe extern "C" fn(libc::c_long, *mut libc::c_char, libc::c_long, *mut NODE) -> (),
>;
pub type parse_field_func_t = Option::<
    unsafe extern "C" fn(
        libc::c_long,
        *mut *mut libc::c_char,
        libc::c_int,
        *mut NODE,
        *mut Regexp,
        Setfunc,
        *mut NODE,
        *mut NODE,
        bool,
    ) -> libc::c_long,
>;
pub const Using_FPAT: field_sep_type = 2;
pub type field_sep_type = libc::c_uint;
pub const Using_API: field_sep_type = 3;
pub const Using_FIELDWIDTHS: field_sep_type = 1;
pub const Using_FS: field_sep_type = 0;
#[inline]
unsafe extern "C" fn POP_PARAM() -> *mut NODE {
    let fresh0 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    let mut t: *mut NODE = (*fresh0).rptr;
    return if (*t).type_0 as libc::c_uint
        == Node_var_array as libc::c_int as libc::c_uint
    {
        t
    } else {
        force_array(t, 0 as libc::c_int != 0)
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
unsafe extern "C" fn POP_SCALAR() -> *mut NODE {
    let fresh1 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    let mut t: *mut NODE = (*fresh1).rptr;
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
unsafe extern "C" fn DEREF(mut r: *mut NODE) {
    (*r).valref -= 1;
    if (*r).valref > 0 as libc::c_int as libc::c_long {
        return;
    }
    r_unref(r);
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
unsafe extern "C" fn is_blank(mut c: libc::c_int) -> libc::c_int {
    return (c == ' ' as i32 || c == '\t' as i32) as libc::c_int;
}
static mut api_parser_override: bool = 0 as libc::c_int != 0;
static mut parse_field: parse_field_func_t = None;
static mut normal_parse_field: parse_field_func_t = None;
static mut api_fw: *const awk_fieldwidth_info_t = 0 as *const awk_fieldwidth_info_t;
static mut parse_extent: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut parse_high_water: libc::c_long = 0 as libc::c_int as libc::c_long;
static mut nf_high_water: libc::c_long = 0 as libc::c_int as libc::c_long;
static mut resave_fs: bool = false;
static mut save_FS: *mut NODE = 0 as *const NODE as *mut NODE;
static mut save_FPAT: *mut NODE = 0 as *const NODE as *mut NODE;
static mut FIELDWIDTHS: *mut awk_fieldwidth_info_t = 0 as *const awk_fieldwidth_info_t
    as *mut awk_fieldwidth_info_t;
pub static mut fields_arr: *mut *mut NODE = 0 as *const *mut NODE as *mut *mut NODE;
pub static mut field0_valid: bool = false;
static mut default_FS: libc::c_int = 0;
static mut FS_re_yes_case: *mut Regexp = 0 as *const Regexp as *mut Regexp;
static mut FS_re_no_case: *mut Regexp = 0 as *const Regexp as *mut Regexp;
static mut FS_regexp: *mut Regexp = 0 as *const Regexp as *mut Regexp;
static mut FPAT_re_yes_case: *mut Regexp = 0 as *const Regexp as *mut Regexp;
static mut FPAT_re_no_case: *mut Regexp = 0 as *const Regexp as *mut Regexp;
static mut FPAT_regexp: *mut Regexp = 0 as *const Regexp as *mut Regexp;
pub static mut Null_field: *mut NODE = 0 as *const NODE as *mut NODE;
pub unsafe extern "C" fn init_fields() {
    fields_arr = emalloc_real(
        ::std::mem::size_of::<*mut NODE>() as libc::c_ulong,
        b"init_fields\0" as *const u8 as *const libc::c_char,
        b"fields_arr\0" as *const u8 as *const libc::c_char,
        b"field.c\0" as *const u8 as *const libc::c_char,
        100 as libc::c_int,
    ) as *mut *mut NODE;
    let ref mut fresh2 = *fields_arr.offset(0 as libc::c_int as isize);
    *fresh2 = make_str_node(
        b"\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int as size_t,
        0 as libc::c_int,
    );
    let ref mut fresh3 = (**fields_arr.offset(0 as libc::c_int as isize)).flags;
    *fresh3 = ::std::mem::transmute::<
        libc::c_uint,
        flagvals,
    >(*fresh3 as libc::c_uint | NULL_FIELD as libc::c_int as libc::c_uint);
    parse_extent = (**fields_arr.offset(0 as libc::c_int as isize)).sub.val.sp;
    save_FS = dupnode((*FS_node).sub.nodep.l.lptr);
    Null_field = make_str_node(
        b"\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int as size_t,
        0 as libc::c_int,
    );
    (*Null_field)
        .flags = (STRCUR as libc::c_int | STRING as libc::c_int
        | NULL_FIELD as libc::c_int) as flagvals;
    field0_valid = 1 as libc::c_int != 0;
}
unsafe extern "C" fn grow_fields_arr(mut num: libc::c_long) {
    let mut t: libc::c_int = 0;
    let mut n: *mut NODE = 0 as *mut NODE;
    fields_arr = erealloc_real(
        fields_arr as *mut libc::c_void,
        ((num + 1 as libc::c_int as libc::c_long) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut NODE>() as libc::c_ulong),
        b"grow_fields_arr\0" as *const u8 as *const libc::c_char,
        b"fields_arr\0" as *const u8 as *const libc::c_char,
        b"field.c\0" as *const u8 as *const libc::c_char,
        122 as libc::c_int,
    ) as *mut *mut NODE;
    t = (nf_high_water + 1 as libc::c_int as libc::c_long) as libc::c_int;
    while t as libc::c_long <= num {
        n = nextfree[BLOCK_NODE as libc::c_int as usize].freep as *mut NODE;
        if !n.is_null() {
            nextfree[BLOCK_NODE as libc::c_int as usize]
                .freep = (*(n as *mut block_item)).freep;
        } else {
            n = more_blocks(BLOCK_NODE as libc::c_int) as *mut NODE;
        };
        *n = *Null_field;
        let ref mut fresh4 = *fields_arr.offset(t as isize);
        *fresh4 = n;
        t += 1;
        t;
    }
    nf_high_water = num;
}
unsafe extern "C" fn set_field(
    mut num: libc::c_long,
    mut str: *mut libc::c_char,
    mut len: libc::c_long,
    mut dummy: *mut NODE,
) {
    let mut n: *mut NODE = 0 as *mut NODE;
    if num > nf_high_water {
        grow_fields_arr(num);
    }
    n = *fields_arr.offset(num as isize);
    (*n).sub.val.sp = str;
    (*n).sub.val.slen = len as size_t;
    (*n)
        .flags = (STRCUR as libc::c_int | STRING as libc::c_int
        | USER_INPUT as libc::c_int) as flagvals;
}
pub unsafe extern "C" fn rebuild_record() {
    let mut tlen: libc::c_ulong = 0;
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut ops: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cops: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_long = 0;
    tlen = 0 as libc::c_int as libc::c_ulong;
    i = NF;
    while i > 0 as libc::c_int as libc::c_long {
        tmp = *fields_arr.offset(i as isize);
        tmp = force_string_fmt(tmp, CONVFMT, CONVFMTidx);
        tlen = tlen.wrapping_add((*tmp).sub.val.slen);
        i -= 1;
        i;
    }
    tlen = tlen
        .wrapping_add(
            ((NF - 1 as libc::c_int as libc::c_long) * OFSlen as libc::c_long)
                as libc::c_ulong,
        );
    if (tlen as libc::c_long) < 0 as libc::c_int as libc::c_long {
        tlen = 0 as libc::c_int as libc::c_ulong;
    }
    ops = emalloc_real(
        tlen.wrapping_add(1 as libc::c_int as libc::c_ulong),
        b"rebuild_record\0" as *const u8 as *const libc::c_char,
        b"ops\0" as *const u8 as *const libc::c_char,
        b"field.c\0" as *const u8 as *const libc::c_char,
        177 as libc::c_int,
    ) as *mut libc::c_char;
    cops = ops;
    *ops.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    i = 1 as libc::c_int as libc::c_long;
    while i <= NF {
        if (**fields_arr.offset(i as isize)).flags as libc::c_uint
            & WSTRCUR as libc::c_int as libc::c_uint != 0
        {
            r_free_wstr(*fields_arr.offset(i as isize));
        }
        tmp = *fields_arr.offset(i as isize);
        if (*tmp).sub.val.slen == 1 as libc::c_int as libc::c_ulong {
            let fresh5 = cops;
            cops = cops.offset(1);
            *fresh5 = *((*tmp).sub.val.sp).offset(0 as libc::c_int as isize);
        } else if (*tmp).sub.val.slen != 0 as libc::c_int as libc::c_ulong {
            memcpy(
                cops as *mut libc::c_void,
                (*tmp).sub.val.sp as *const libc::c_void,
                (*tmp).sub.val.slen,
            );
            cops = cops.offset((*tmp).sub.val.slen as isize);
        }
        if i != NF {
            if OFSlen == 1 as libc::c_int {
                let fresh6 = cops;
                cops = cops.offset(1);
                *fresh6 = *OFS;
            } else if OFSlen != 0 as libc::c_int {
                memcpy(
                    cops as *mut libc::c_void,
                    OFS as *const libc::c_void,
                    OFSlen as libc::c_ulong,
                );
                cops = cops.offset(OFSlen as isize);
            }
        }
        i += 1;
        i;
    }
    tmp = make_str_node(ops, tlen, 2 as libc::c_int);
    cops = ops;
    i = 1 as libc::c_int as libc::c_long;
    while i <= NF {
        let mut r: *mut NODE = *fields_arr.offset(i as isize);
        if (*r).sub.val.slen > 0 as libc::c_int as libc::c_ulong
            && (*r).flags as libc::c_uint & MALLOC as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
        {
            let mut n: *mut NODE = 0 as *mut NODE;
            n = nextfree[BLOCK_NODE as libc::c_int as usize].freep as *mut NODE;
            if !n.is_null() {
                nextfree[BLOCK_NODE as libc::c_int as usize]
                    .freep = (*(n as *mut block_item)).freep;
            } else {
                n = more_blocks(BLOCK_NODE as libc::c_int) as *mut NODE;
            };
            *n = *r;
            if (*r).valref > 1 as libc::c_int as libc::c_long {
                (*r)
                    .sub
                    .val
                    .sp = emalloc_real(
                    ((*r).sub.val.slen).wrapping_add(1 as libc::c_int as libc::c_ulong),
                    b"rebuild_record\0" as *const u8 as *const libc::c_char,
                    b"r->stptr\0" as *const u8 as *const libc::c_char,
                    b"field.c\0" as *const u8 as *const libc::c_char,
                    226 as libc::c_int,
                ) as *mut libc::c_char;
                memcpy(
                    (*r).sub.val.sp as *mut libc::c_void,
                    cops as *const libc::c_void,
                    (*r).sub.val.slen,
                );
                *((*r).sub.val.sp)
                    .offset((*r).sub.val.slen as isize) = '\0' as i32 as libc::c_char;
                (*r)
                    .flags = ::std::mem::transmute::<
                    libc::c_uint,
                    flagvals,
                >((*r).flags as libc::c_uint | MALLOC as libc::c_int as libc::c_uint);
                (*n).valref = 1 as libc::c_int as libc::c_long;
            }
            (*n).sub.val.sp = cops;
            (*n)
                .flags = ::std::mem::transmute::<
                libc::c_uint,
                flagvals,
            >(
                (*n).flags as libc::c_uint
                    & !(MPFN as libc::c_int | MPZN as libc::c_int
                        | NUMCUR as libc::c_int) as libc::c_uint,
            );
            unref(r);
            let ref mut fresh7 = *fields_arr.offset(i as isize);
            *fresh7 = n;
        }
        cops = cops
            .offset(
                ((**fields_arr.offset(i as isize)).sub.val.slen)
                    .wrapping_add(OFSlen as libc::c_ulong) as isize,
            );
        i += 1;
        i;
    }
    unref(*fields_arr.offset(0 as libc::c_int as isize));
    let ref mut fresh8 = *fields_arr.offset(0 as libc::c_int as isize);
    *fresh8 = tmp;
    field0_valid = 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn set_record(
    mut buf: *const libc::c_char,
    mut cnt: size_t,
    mut fw: *const awk_fieldwidth_info_t,
) {
    let mut n: *mut NODE = 0 as *mut NODE;
    static mut databuf: *mut libc::c_char = 0 as *const libc::c_char
        as *mut libc::c_char;
    static mut databuf_size: size_t = 0;
    purge_record();
    if databuf_size == 0 as libc::c_int as libc::c_ulong {
        databuf = ezalloc_real(
            512 as libc::c_int as size_t,
            b"set_record\0" as *const u8 as *const libc::c_char,
            b"databuf\0" as *const u8 as *const libc::c_char,
            b"field.c\0" as *const u8 as *const libc::c_char,
            276 as libc::c_int,
        ) as *mut libc::c_char;
        databuf_size = 512 as libc::c_int as size_t;
    }
    if cnt >= databuf_size {
        loop {
            if databuf_size
                > (!(0 as libc::c_int) as size_t)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong)
            {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"field.c\0" as *const u8 as *const libc::c_char,
                    287 as libc::c_int,
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
                        b"input record too large\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            databuf_size = (databuf_size as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
            if !(cnt >= databuf_size) {
                break;
            }
        }
        databuf = erealloc_real(
            databuf as *mut libc::c_void,
            databuf_size,
            b"set_record\0" as *const u8 as *const libc::c_char,
            b"databuf\0" as *const u8 as *const libc::c_char,
            b"field.c\0" as *const u8 as *const libc::c_char,
            290 as libc::c_int,
        ) as *mut libc::c_char;
        memset(databuf as *mut libc::c_void, '\0' as i32, databuf_size);
    }
    if cnt != 0 as libc::c_int as libc::c_ulong {
        memcpy(databuf as *mut libc::c_void, buf as *const libc::c_void, cnt);
    }
    *databuf.offset(cnt as isize) = '\0' as i32 as libc::c_char;
    unref(*fields_arr.offset(0 as libc::c_int as isize));
    n = nextfree[BLOCK_NODE as libc::c_int as usize].freep as *mut NODE;
    if !n.is_null() {
        nextfree[BLOCK_NODE as libc::c_int as usize]
            .freep = (*(n as *mut block_item)).freep;
    } else {
        n = more_blocks(BLOCK_NODE as libc::c_int) as *mut NODE;
    };
    (*n).sub.val.sp = databuf;
    (*n).sub.val.slen = cnt;
    (*n).valref = 1 as libc::c_int as libc::c_long;
    (*n).type_0 = Node_val;
    (*n).sub.val.idx = -(1 as libc::c_int);
    (*n)
        .flags = (STRING as libc::c_int | STRCUR as libc::c_int
        | USER_INPUT as libc::c_int) as flagvals;
    let ref mut fresh9 = *fields_arr.offset(0 as libc::c_int as isize);
    *fresh9 = n;
    if fw != api_fw {
        api_fw = fw;
        if !api_fw.is_null() {
            if !api_parser_override {
                api_parser_override = 1 as libc::c_int != 0;
                parse_field = Some(
                    fw_parse_field
                        as unsafe extern "C" fn(
                            libc::c_long,
                            *mut *mut libc::c_char,
                            libc::c_int,
                            *mut NODE,
                            *mut Regexp,
                            Setfunc,
                            *mut NODE,
                            *mut NODE,
                            bool,
                        ) -> libc::c_long,
                );
                update_PROCINFO_str(
                    b"FS\0" as *const u8 as *const libc::c_char,
                    b"API\0" as *const u8 as *const libc::c_char,
                );
            }
        } else if api_parser_override {
            api_parser_override = 0 as libc::c_int != 0;
            parse_field = normal_parse_field;
            update_PROCINFO_str(
                b"FS\0" as *const u8 as *const libc::c_char,
                current_field_sep_str(),
            );
        }
    }
}
pub unsafe extern "C" fn reset_record() {
    let ref mut fresh10 = *fields_arr.offset(0 as libc::c_int as isize);
    *fresh10 = force_string_fmt(
        *fields_arr.offset(0 as libc::c_int as isize),
        CONVFMT,
        CONVFMTidx,
    );
    purge_record();
    if api_parser_override {
        api_parser_override = 0 as libc::c_int != 0;
        parse_field = normal_parse_field;
        update_PROCINFO_str(
            b"FS\0" as *const u8 as *const libc::c_char,
            current_field_sep_str(),
        );
    }
}
unsafe extern "C" fn purge_record() {
    let mut i: libc::c_int = 0;
    NF = -(1 as libc::c_int) as libc::c_long;
    i = 1 as libc::c_int;
    while i as libc::c_long <= parse_high_water {
        let mut n: *mut NODE = 0 as *mut NODE;
        let mut r: *mut NODE = *fields_arr.offset(i as isize);
        if (*r).flags as libc::c_uint & MALLOC as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
            && (*r).valref > 1 as libc::c_int as libc::c_long
        {
            let mut save: *const libc::c_char = (*r).sub.val.sp;
            (*r)
                .sub
                .val
                .sp = emalloc_real(
                ((*r).sub.val.slen).wrapping_add(1 as libc::c_int as libc::c_ulong),
                b"purge_record\0" as *const u8 as *const libc::c_char,
                b"r->stptr\0" as *const u8 as *const libc::c_char,
                b"field.c\0" as *const u8 as *const libc::c_char,
                370 as libc::c_int,
            ) as *mut libc::c_char;
            memcpy(
                (*r).sub.val.sp as *mut libc::c_void,
                save as *const libc::c_void,
                (*r).sub.val.slen,
            );
            *((*r).sub.val.sp)
                .offset((*r).sub.val.slen as isize) = '\0' as i32 as libc::c_char;
            (*r)
                .flags = ::std::mem::transmute::<
                libc::c_uint,
                flagvals,
            >((*r).flags as libc::c_uint | MALLOC as libc::c_int as libc::c_uint);
        }
        unref(r);
        n = nextfree[BLOCK_NODE as libc::c_int as usize].freep as *mut NODE;
        if !n.is_null() {
            nextfree[BLOCK_NODE as libc::c_int as usize]
                .freep = (*(n as *mut block_item)).freep;
        } else {
            n = more_blocks(BLOCK_NODE as libc::c_int) as *mut NODE;
        };
        *n = *Null_field;
        let ref mut fresh11 = *fields_arr.offset(i as isize);
        *fresh11 = n;
        i += 1;
        i;
    }
    parse_high_water = 0 as libc::c_int as libc::c_long;
    if resave_fs {
        resave_fs = 0 as libc::c_int != 0;
        unref(save_FS);
        save_FS = dupnode((*FS_node).sub.nodep.l.lptr);
    }
    field0_valid = 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn set_NF() {
    let mut i: libc::c_int = 0;
    let mut nf: libc::c_long = 0;
    let mut n: *mut NODE = 0 as *mut NODE;
    force_number((*NF_node).sub.nodep.l.lptr);
    nf = (*(*NF_node).sub.nodep.l.lptr).sub.val.fltnum as libc::c_long;
    if nf < 0 as libc::c_int as libc::c_long {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"field.c\0" as *const u8 as *const libc::c_char,
            408 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"NF set to negative value\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    static mut warned: bool = 0 as libc::c_int != 0;
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0 && NF > nf && !warned
    {
        warned = 1 as libc::c_int != 0;
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"field.c\0" as *const u8 as *const libc::c_char,
            413 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"decrementing NF is not portable to many awk versions\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    NF = nf;
    if NF > nf_high_water {
        grow_fields_arr(NF);
    }
    if parse_high_water < NF {
        i = (parse_high_water + 1 as libc::c_int as libc::c_long) as libc::c_int;
        while i >= 0 as libc::c_int && i as libc::c_long <= NF {
            unref(*fields_arr.offset(i as isize));
            n = nextfree[BLOCK_NODE as libc::c_int as usize].freep as *mut NODE;
            if !n.is_null() {
                nextfree[BLOCK_NODE as libc::c_int as usize]
                    .freep = (*(n as *mut block_item)).freep;
            } else {
                n = more_blocks(BLOCK_NODE as libc::c_int) as *mut NODE;
            };
            *n = *Null_field;
            let ref mut fresh12 = *fields_arr.offset(i as isize);
            *fresh12 = n;
            i += 1;
            i;
        }
        parse_high_water = NF;
    } else if parse_high_water > 0 as libc::c_int as libc::c_long {
        i = (NF + 1 as libc::c_int as libc::c_long) as libc::c_int;
        while i >= 0 as libc::c_int && i as libc::c_long <= parse_high_water {
            unref(*fields_arr.offset(i as isize));
            n = nextfree[BLOCK_NODE as libc::c_int as usize].freep as *mut NODE;
            if !n.is_null() {
                nextfree[BLOCK_NODE as libc::c_int as usize]
                    .freep = (*(n as *mut block_item)).freep;
            } else {
                n = more_blocks(BLOCK_NODE as libc::c_int) as *mut NODE;
            };
            *n = *Null_field;
            let ref mut fresh13 = *fields_arr.offset(i as isize);
            *fresh13 = n;
            i += 1;
            i;
        }
        parse_high_water = NF;
    }
    field0_valid = 0 as libc::c_int != 0;
}
unsafe extern "C" fn re_parse_field(
    mut up_to: libc::c_long,
    mut buf: *mut *mut libc::c_char,
    mut len: libc::c_int,
    mut fs: *mut NODE,
    mut rp: *mut Regexp,
    mut set: Setfunc,
    mut n: *mut NODE,
    mut sep_arr: *mut NODE,
    mut in_middle: bool,
) -> libc::c_long {
    let mut scan: *mut libc::c_char = *buf;
    let mut nf: libc::c_long = parse_high_water;
    let mut field: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = scan.offset(len as isize);
    let mut regex_flags: libc::c_int = 1 as libc::c_int;
    let mut sep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mbclen: size_t = 0 as libc::c_int as size_t;
    let mut mbs: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    memset(
        &mut mbs as *mut mbstate_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
    );
    if in_middle {
        regex_flags |= 2 as libc::c_int;
    }
    if up_to == 9223372036854775807 as libc::c_long {
        nf = 0 as libc::c_int as libc::c_long;
    }
    if len == 0 as libc::c_int {
        return nf;
    }
    let mut default_field_splitting: bool = RS_is_null as libc::c_int != 0
        && default_FS != 0;
    if default_field_splitting {
        sep = scan;
        while scan < end
            && (*scan as libc::c_int == ' ' as i32 || *scan as libc::c_int == '\t' as i32
                || *scan as libc::c_int == '\n' as i32)
        {
            scan = scan.offset(1);
            scan;
        }
        if !sep_arr.is_null() && sep < scan {
            set_element(nf, sep, scan.offset_from(sep) as libc::c_long, sep_arr);
        }
    }
    if rp.is_null() {
        rp = FS_regexp;
    }
    field = scan;
    while scan < end
        && research(
            rp,
            scan,
            0 as libc::c_int,
            end.offset_from(scan) as libc::c_long as size_t,
            regex_flags,
        ) != -(1 as libc::c_int) && nf < up_to
    {
        regex_flags |= 2 as libc::c_int;
        if *((*rp).regs.end).offset(0 as libc::c_int as isize)
            == *((*rp).regs.start).offset(0 as libc::c_int as isize)
        {
            if gawk_mb_cur_max > 1 as libc::c_int {
                mbclen = mbrlen(
                    scan,
                    end.offset_from(scan) as libc::c_long as size_t,
                    &mut mbs,
                );
                if mbclen == 1 as libc::c_int as libc::c_ulong
                    || mbclen == -(1 as libc::c_int) as size_t
                    || mbclen == -(2 as libc::c_int) as size_t
                    || mbclen == 0 as libc::c_int as libc::c_ulong
                {
                    mbclen = 1 as libc::c_int as size_t;
                }
                scan = scan.offset(mbclen as isize);
            } else {
                scan = scan.offset(1);
                scan;
            }
            if !(scan == end) {
                continue;
            }
            nf += 1;
            (Some(set.unwrap()))
                .unwrap()(nf, field, scan.offset_from(field) as libc::c_long, n);
            up_to = nf;
            break;
        } else {
            nf += 1;
            (Some(set.unwrap()))
                .unwrap()(
                nf,
                field,
                scan
                    .offset(
                        *((*rp).regs.start).offset(0 as libc::c_int as isize) as isize,
                    )
                    .offset_from(field) as libc::c_long,
                n,
            );
            if !sep_arr.is_null() {
                set_element(
                    nf,
                    scan
                        .offset(
                            *((*rp).regs.start).offset(0 as libc::c_int as isize)
                                as isize,
                        ),
                    (*((*rp).regs.end).offset(0 as libc::c_int as isize)
                        - *((*rp).regs.start).offset(0 as libc::c_int as isize))
                        as libc::c_long,
                    sep_arr,
                );
            }
            scan = scan
                .offset(*((*rp).regs.end).offset(0 as libc::c_int as isize) as isize);
            field = scan;
            if scan == end && !default_field_splitting {
                nf += 1;
                (Some(set.unwrap())).unwrap()(nf, field, 0 as libc::c_long, n);
            }
        }
    }
    if nf != up_to && scan < end {
        nf += 1;
        (Some(set.unwrap()))
            .unwrap()(nf, scan, end.offset_from(scan) as libc::c_long, n);
        scan = end;
    }
    *buf = scan;
    return nf;
}
unsafe extern "C" fn def_parse_field(
    mut up_to: libc::c_long,
    mut buf: *mut *mut libc::c_char,
    mut len: libc::c_int,
    mut fs: *mut NODE,
    mut rp: *mut Regexp,
    mut set: Setfunc,
    mut n: *mut NODE,
    mut sep_arr: *mut NODE,
    mut in_middle: bool,
) -> libc::c_long {
    let mut scan: *mut libc::c_char = *buf;
    let mut nf: libc::c_long = parse_high_water;
    let mut field: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = scan.offset(len as isize);
    let mut sav: libc::c_char = 0;
    let mut sep: *mut libc::c_char = 0 as *mut libc::c_char;
    if up_to == 9223372036854775807 as libc::c_long {
        nf = 0 as libc::c_int as libc::c_long;
    }
    if len == 0 as libc::c_int {
        return nf;
    }
    if (*fs).sub.val.slen == 0 as libc::c_int as libc::c_ulong {
        nf += 1;
        (Some(set.unwrap())).unwrap()(nf, *buf, len as libc::c_long, n);
        *buf = (*buf).offset(len as isize);
        return nf;
    }
    sav = *end;
    *end = ' ' as i32 as libc::c_char;
    sep = scan;
    while nf < up_to {
        while scan < end
            && (*scan as libc::c_int == ' ' as i32 || *scan as libc::c_int == '\t' as i32
                || *scan as libc::c_int == '\n' as i32)
        {
            scan = scan.offset(1);
            scan;
        }
        if !sep_arr.is_null() && scan > sep {
            set_element(nf, sep, scan.offset_from(sep) as libc::c_long, sep_arr);
        }
        if scan >= end {
            break;
        }
        field = scan;
        while *scan as libc::c_int != ' ' as i32 && *scan as libc::c_int != '\t' as i32
            && *scan as libc::c_int != '\n' as i32
        {
            scan = scan.offset(1);
            scan;
        }
        nf += 1;
        (Some(set.unwrap()))
            .unwrap()(nf, field, scan.offset_from(field) as libc::c_long, n);
        if scan == end {
            break;
        }
        sep = scan;
        scan = scan.offset(1);
        scan;
    }
    *end = sav;
    *buf = scan;
    return nf;
}
unsafe extern "C" fn null_parse_field(
    mut up_to: libc::c_long,
    mut buf: *mut *mut libc::c_char,
    mut len: libc::c_int,
    mut fs: *mut NODE,
    mut rp: *mut Regexp,
    mut set: Setfunc,
    mut n: *mut NODE,
    mut sep_arr: *mut NODE,
    mut in_middle: bool,
) -> libc::c_long {
    let mut scan: *mut libc::c_char = *buf;
    let mut nf: libc::c_long = parse_high_water;
    let mut end: *mut libc::c_char = scan.offset(len as isize);
    if up_to == 9223372036854775807 as libc::c_long {
        nf = 0 as libc::c_int as libc::c_long;
    }
    if len == 0 as libc::c_int {
        return nf;
    }
    if gawk_mb_cur_max > 1 as libc::c_int {
        let mut mbs: mbstate_t = mbstate_t {
            __count: 0,
            __value: C2RustUnnamed { __wch: 0 },
        };
        memset(
            &mut mbs as *mut mbstate_t as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
        );
        while nf < up_to && scan < end {
            let mut mbclen: size_t = mbrlen(
                scan,
                end.offset_from(scan) as libc::c_long as size_t,
                &mut mbs,
            );
            if mbclen == 1 as libc::c_int as libc::c_ulong
                || mbclen == -(1 as libc::c_int) as size_t
                || mbclen == -(2 as libc::c_int) as size_t
                || mbclen == 0 as libc::c_int as libc::c_ulong
            {
                mbclen = 1 as libc::c_int as size_t;
            }
            if !sep_arr.is_null() && nf > 0 as libc::c_int as libc::c_long {
                set_element(nf, scan, 0 as libc::c_long, sep_arr);
            }
            nf += 1;
            (Some(set.unwrap())).unwrap()(nf, scan, mbclen as libc::c_long, n);
            scan = scan.offset(mbclen as isize);
        }
    } else {
        while nf < up_to && scan < end {
            if !sep_arr.is_null() && nf > 0 as libc::c_int as libc::c_long {
                set_element(nf, scan, 0 as libc::c_long, sep_arr);
            }
            nf += 1;
            (Some(set.unwrap())).unwrap()(nf, scan, 1 as libc::c_long, n);
            scan = scan.offset(1);
            scan;
        }
    }
    *buf = scan;
    return nf;
}
unsafe extern "C" fn sc_parse_field(
    mut up_to: libc::c_long,
    mut buf: *mut *mut libc::c_char,
    mut len: libc::c_int,
    mut fs: *mut NODE,
    mut rp: *mut Regexp,
    mut set: Setfunc,
    mut n: *mut NODE,
    mut sep_arr: *mut NODE,
    mut in_middle: bool,
) -> libc::c_long {
    let mut scan: *mut libc::c_char = *buf;
    let mut fschar: libc::c_char = 0;
    let mut nf: libc::c_long = parse_high_water;
    let mut field: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = scan.offset(len as isize);
    let mut sav: libc::c_char = 0;
    let mut mbclen: size_t = 0 as libc::c_int as size_t;
    let mut mbs: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    memset(
        &mut mbs as *mut mbstate_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
    );
    if up_to == 9223372036854775807 as libc::c_long {
        nf = 0 as libc::c_int as libc::c_long;
    }
    if len == 0 as libc::c_int {
        return nf;
    }
    if RS_is_null as libc::c_int != 0
        && (*fs).sub.val.slen == 0 as libc::c_int as libc::c_ulong
    {
        fschar = '\n' as i32 as libc::c_char;
    } else {
        fschar = *((*fs).sub.val.sp).offset(0 as libc::c_int as isize);
    }
    sav = *end;
    *end = fschar;
    while nf < up_to {
        field = scan;
        if gawk_mb_cur_max > 1 as libc::c_int {
            while *scan as libc::c_int != fschar as libc::c_int {
                mbclen = mbrlen(
                    scan,
                    end.offset_from(scan) as libc::c_long as size_t,
                    &mut mbs,
                );
                if mbclen == 1 as libc::c_int as libc::c_ulong
                    || mbclen == -(1 as libc::c_int) as size_t
                    || mbclen == -(2 as libc::c_int) as size_t
                    || mbclen == 0 as libc::c_int as libc::c_ulong
                {
                    mbclen = 1 as libc::c_int as size_t;
                }
                scan = scan.offset(mbclen as isize);
            }
        } else {
            while *scan as libc::c_int != fschar as libc::c_int {
                scan = scan.offset(1);
                scan;
            }
        }
        nf += 1;
        (Some(set.unwrap()))
            .unwrap()(nf, field, scan.offset_from(field) as libc::c_long, n);
        if scan == end {
            break;
        }
        if !sep_arr.is_null() {
            set_element(nf, scan, 1 as libc::c_long, sep_arr);
        }
        scan = scan.offset(1);
        scan;
        if !(scan == end) {
            continue;
        }
        nf += 1;
        (Some(set.unwrap())).unwrap()(nf, field, 0 as libc::c_long, n);
        break;
    }
    *end = sav;
    *buf = scan;
    return nf;
}
unsafe extern "C" fn calc_mbslen(
    mut scan: *mut libc::c_char,
    mut end: *mut libc::c_char,
    mut len: size_t,
    mut mbs: *mut mbstate_t,
) -> size_t {
    let mut mbclen: size_t = 0;
    let mut mbscan: *mut libc::c_char = scan;
    loop {
        let fresh14 = len;
        len = len.wrapping_sub(1);
        if !(fresh14 > 0 as libc::c_int as libc::c_ulong && mbscan < end) {
            break;
        }
        mbclen = mbrlen(mbscan, end.offset_from(mbscan) as libc::c_long as size_t, mbs);
        if !(mbclen > 0 as libc::c_int as libc::c_ulong
            && mbclen <= end.offset_from(mbscan) as libc::c_long as size_t)
        {
            mbclen = 1 as libc::c_int as size_t;
        }
        mbscan = mbscan.offset(mbclen as isize);
    }
    return mbscan.offset_from(scan) as libc::c_long as size_t;
}
unsafe extern "C" fn fw_parse_field(
    mut up_to: libc::c_long,
    mut buf: *mut *mut libc::c_char,
    mut len: libc::c_int,
    mut fs: *mut NODE,
    mut rp: *mut Regexp,
    mut set: Setfunc,
    mut n: *mut NODE,
    mut dummy: *mut NODE,
    mut in_middle: bool,
) -> libc::c_long {
    let mut scan: *mut libc::c_char = *buf;
    let mut nf: libc::c_long = parse_high_water;
    let mut end: *mut libc::c_char = scan.offset(len as isize);
    let mut fw: *const awk_fieldwidth_info_t = 0 as *const awk_fieldwidth_info_t;
    let mut mbs: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    let mut skiplen: size_t = 0;
    let mut flen: size_t = 0;
    fw = if api_parser_override as libc::c_int != 0 {
        api_fw
    } else {
        FIELDWIDTHS as *const awk_fieldwidth_info_t
    };
    if up_to == 9223372036854775807 as libc::c_long {
        nf = 0 as libc::c_int as libc::c_long;
    }
    if len == 0 as libc::c_int {
        return nf;
    }
    if gawk_mb_cur_max > 1 as libc::c_int && (*fw).use_chars as libc::c_uint != 0 {
        memset(
            &mut mbs as *mut mbstate_t as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
        );
        while nf < up_to && scan < end {
            if nf as libc::c_ulong >= (*fw).nf {
                *buf = end;
                return nf;
            }
            scan = scan
                .offset(
                    calc_mbslen(
                        scan,
                        end,
                        (*((*fw).fields).as_ptr().offset(nf as isize)).skip,
                        &mut mbs,
                    ) as isize,
                );
            flen = calc_mbslen(
                scan,
                end,
                (*((*fw).fields).as_ptr().offset(nf as isize)).len,
                &mut mbs,
            );
            nf += 1;
            (Some(set.unwrap())).unwrap()(nf, scan, flen as libc::c_long, n);
            scan = scan.offset(flen as isize);
        }
    } else {
        while nf < up_to && scan < end {
            if nf as libc::c_ulong >= (*fw).nf {
                *buf = end;
                return nf;
            }
            skiplen = (*((*fw).fields).as_ptr().offset(nf as isize)).skip;
            if skiplen > end.offset_from(scan) as libc::c_long as libc::c_ulong {
                skiplen = end.offset_from(scan) as libc::c_long as size_t;
            }
            scan = scan.offset(skiplen as isize);
            flen = (*((*fw).fields).as_ptr().offset(nf as isize)).len;
            if flen > end.offset_from(scan) as libc::c_long as libc::c_ulong {
                flen = end.offset_from(scan) as libc::c_long as size_t;
            }
            nf += 1;
            (Some(set.unwrap())).unwrap()(nf, scan, flen as libc::c_long, n);
            scan = scan.offset(flen as isize);
        }
    }
    *buf = scan;
    return nf;
}
unsafe extern "C" fn invalidate_field0() {
    field0_valid = 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn get_field(
    mut requested: libc::c_long,
    mut assign: *mut Func_ptr,
) -> *mut *mut NODE {
    let mut in_middle: bool = 0 as libc::c_int != 0;
    static mut warned: bool = 0 as libc::c_int != 0;
    extern "C" {
        static mut currule: libc::c_int;
    }
    let mut saved_fs: *mut NODE = 0 as *mut NODE;
    let mut fs_regexp: *mut Regexp = 0 as *mut Regexp;
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0 && currule == END as libc::c_int && !warned
    {
        warned = 1 as libc::c_int != 0;
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"field.c\0" as *const u8 as *const libc::c_char,
            861 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"accessing fields from an END rule may not be portable\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if requested == 0 as libc::c_int as libc::c_long {
        if !field0_valid {
            if NF == -(1 as libc::c_int) as libc::c_long {
                in_middle = parse_high_water != 0 as libc::c_int as libc::c_long;
                if current_field_sep() as libc::c_uint
                    == Using_FPAT as libc::c_int as libc::c_uint
                {
                    saved_fs = save_FPAT;
                    fs_regexp = FPAT_regexp;
                } else {
                    saved_fs = save_FS;
                    fs_regexp = FS_regexp;
                }
                NF = (Some(parse_field.unwrap()))
                    .unwrap()(
                    9223372036854775807 as libc::c_long
                        - 1 as libc::c_int as libc::c_long,
                    &mut parse_extent,
                    ((**fields_arr.offset(0 as libc::c_int as isize)).sub.val.slen)
                        .wrapping_sub(
                            parse_extent
                                .offset_from(
                                    (**fields_arr.offset(0 as libc::c_int as isize)).sub.val.sp,
                                ) as libc::c_long as libc::c_ulong,
                        ) as libc::c_int,
                    saved_fs,
                    fs_regexp,
                    Some(
                        set_field
                            as unsafe extern "C" fn(
                                libc::c_long,
                                *mut libc::c_char,
                                libc::c_long,
                                *mut NODE,
                            ) -> (),
                    ),
                    0 as *mut libc::c_void as *mut NODE,
                    0 as *mut libc::c_void as *mut NODE,
                    in_middle,
                );
                parse_high_water = NF;
            }
            rebuild_record();
        }
        if !assign.is_null() {
            *assign = Some(reset_record as unsafe extern "C" fn() -> ());
        }
        return &mut *fields_arr.offset(0 as libc::c_int as isize) as *mut *mut NODE;
    }
    if !assign.is_null() {
        *assign = ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Func_ptr,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(invalidate_field0),
            ),
        );
    }
    if requested <= parse_high_water {
        return &mut *fields_arr.offset(requested as isize) as *mut *mut NODE;
    }
    if NF == -(1 as libc::c_int) as libc::c_long {
        if parse_high_water == 0 as libc::c_int as libc::c_long {
            parse_extent = (**fields_arr.offset(0 as libc::c_int as isize)).sub.val.sp;
        } else {
            in_middle = 1 as libc::c_int != 0;
        }
        parse_high_water = (Some(parse_field.unwrap()))
            .unwrap()(
            requested,
            &mut parse_extent,
            ((**fields_arr.offset(0 as libc::c_int as isize)).sub.val.slen)
                .wrapping_sub(
                    parse_extent
                        .offset_from(
                            (**fields_arr.offset(0 as libc::c_int as isize)).sub.val.sp,
                        ) as libc::c_long as libc::c_ulong,
                ) as libc::c_int,
            save_FS,
            0 as *mut Regexp,
            Some(
                set_field
                    as unsafe extern "C" fn(
                        libc::c_long,
                        *mut libc::c_char,
                        libc::c_long,
                        *mut NODE,
                    ) -> (),
            ),
            0 as *mut libc::c_void as *mut NODE,
            0 as *mut libc::c_void as *mut NODE,
            in_middle,
        );
        if parse_extent
            == ((**fields_arr.offset(0 as libc::c_int as isize)).sub.val.sp)
                .offset(
                    (**fields_arr.offset(0 as libc::c_int as isize)).sub.val.slen
                        as isize,
                )
        {
            NF = parse_high_water;
        }
        if requested
            == 9223372036854775807 as libc::c_long - 1 as libc::c_int as libc::c_long
        {
            requested = parse_high_water;
        }
    }
    if parse_high_water < requested {
        if !assign.is_null() {
            if requested > nf_high_water {
                grow_fields_arr(requested);
            }
            NF = requested;
            parse_high_water = requested;
        } else {
            return &mut Null_field
        }
    }
    return &mut *fields_arr.offset(requested as isize) as *mut *mut NODE;
}
unsafe extern "C" fn set_element(
    mut num: libc::c_long,
    mut s: *mut libc::c_char,
    mut len: libc::c_long,
    mut n: *mut NODE,
) {
    let mut it: *mut NODE = 0 as *mut NODE;
    let mut sub: *mut NODE = 0 as *mut NODE;
    it = make_str_node(s, len as size_t, 0 as libc::c_int);
    (*it)
        .flags = ::std::mem::transmute::<
        libc::c_uint,
        flagvals,
    >((*it).flags as libc::c_uint | USER_INPUT as libc::c_int as libc::c_uint);
    sub = make_number.unwrap()(num as libc::c_double);
    assoc_set(n, sub, it);
}
pub unsafe extern "C" fn do_split(mut nargs: libc::c_int) -> *mut NODE {
    let mut src: *mut NODE = 0 as *mut NODE;
    let mut arr: *mut NODE = 0 as *mut NODE;
    let mut sep: *mut NODE = 0 as *mut NODE;
    let mut fs: *mut NODE = 0 as *mut NODE;
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut sep_arr: *mut NODE = 0 as *mut NODE;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut parseit: Option::<
        unsafe extern "C" fn(
            libc::c_long,
            *mut *mut libc::c_char,
            libc::c_int,
            *mut NODE,
            *mut Regexp,
            Setfunc,
            *mut NODE,
            *mut NODE,
            bool,
        ) -> libc::c_long,
    > = None;
    let mut rp: *mut Regexp = 0 as *mut Regexp;
    check_args_min_max(
        nargs,
        b"split\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int,
        4 as libc::c_int,
    );
    if nargs == 4 as libc::c_int {
        static mut warned: bool = 0 as libc::c_int != 0;
        if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0
            || do_flags as libc::c_uint & DO_POSIX as libc::c_int as libc::c_uint != 0
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"field.c\0" as *const u8 as *const libc::c_char,
                988 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"split: fourth argument is a gawk extension\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        sep_arr = POP_PARAM();
        if (*sep_arr).type_0 as libc::c_uint
            != Node_var_array as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"field.c\0" as *const u8 as *const libc::c_char,
                992 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"split: fourth argument is not an array\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        check_symtab_functab(
            sep_arr,
            b"split\0" as *const u8 as *const libc::c_char,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: cannot use %s as fourth argument\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        if (do_flags as libc::c_uint & DO_LINT_EXTENSIONS as libc::c_int as libc::c_uint
            != 0
            || do_flags as libc::c_uint & DO_LINT_OLD as libc::c_int as libc::c_uint
                != 0) && !warned
        {
            warned = 1 as libc::c_int != 0;
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"field.c\0" as *const u8 as *const libc::c_char,
                997 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"split: fourth argument is a gawk extension\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
    }
    let fresh15 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    sep = (*fresh15).rptr;
    arr = POP_PARAM();
    if (*arr).type_0 as libc::c_uint != Node_var_array as libc::c_int as libc::c_uint {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"field.c\0" as *const u8 as *const libc::c_char,
            1004 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"split: second argument is not an array\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    check_symtab_functab(
        arr,
        b"split\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"%s: cannot use %s as second argument\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    if !sep_arr.is_null() {
        if sep_arr == arr {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"field.c\0" as *const u8 as *const libc::c_char,
                1010 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"split: cannot use the same array for second and fourth args\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        tmp = (*sep_arr).sub.nodep.x.extra;
        while !tmp.is_null() {
            if tmp == arr {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"field.c\0" as *const u8 as *const libc::c_char,
                    1015 as libc::c_int,
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
                        b"split: cannot use a subarray of second arg for fourth arg\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            tmp = (*tmp).sub.nodep.x.extra;
        }
        tmp = (*arr).sub.nodep.x.extra;
        while !tmp.is_null() {
            if tmp == sep_arr {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"field.c\0" as *const u8 as *const libc::c_char,
                    1018 as libc::c_int,
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
                        b"split: cannot use a subarray of fourth arg for second arg\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            tmp = (*tmp).sub.nodep.x.extra;
        }
        ((*(*sep_arr).sub.nodep.l.lp).clear).unwrap()(sep_arr, 0 as *mut exp_node);
    }
    ((*(*arr).sub.nodep.l.lp).clear).unwrap()(arr, 0 as *mut exp_node);
    src = force_string_fmt(TOP_SCALAR(), CONVFMT, CONVFMTidx);
    if (*src).sub.val.slen == 0 as libc::c_int as libc::c_ulong {
        tmp = POP_SCALAR();
        DEREF(tmp);
        return make_number.unwrap()(0 as libc::c_int as libc::c_double);
    }
    if (*sep).flags as libc::c_uint & REGEX as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        sep = (*sep).sub.val.typre;
    }
    if (*sep).sub.nodep.reflags as libc::c_uint & FS_DFLT as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
        && current_field_sep() as libc::c_uint == Using_FS as libc::c_int as libc::c_uint
        && !RS_is_null
    {
        parseit = parse_field;
        fs = force_string_fmt((*FS_node).sub.nodep.l.lptr, CONVFMT, CONVFMTidx);
        rp = FS_regexp;
    } else {
        fs = (*sep).sub.nodep.x.extra;
        if (*fs).sub.val.slen == 0 as libc::c_int as libc::c_ulong {
            static mut warned_0: bool = 0 as libc::c_int != 0;
            parseit = Some(
                null_parse_field
                    as unsafe extern "C" fn(
                        libc::c_long,
                        *mut *mut libc::c_char,
                        libc::c_int,
                        *mut NODE,
                        *mut Regexp,
                        Setfunc,
                        *mut NODE,
                        *mut NODE,
                        bool,
                    ) -> libc::c_long,
            );
            if do_flags as libc::c_uint
                & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                    as libc::c_uint != 0 && !warned_0
            {
                warned_0 = 1 as libc::c_int != 0;
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"field.c\0" as *const u8 as *const libc::c_char,
                    1052 as libc::c_int,
                );
                (Some(lintfunc.unwrap()))
                    .unwrap()(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"split: null string for third arg is a non-standard extension\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
        } else if (*fs).sub.val.slen == 1 as libc::c_int as libc::c_ulong
            && (*sep).sub.nodep.reflags as libc::c_uint
                & CONSTANT as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
        {
            if *((*fs).sub.val.sp).offset(0 as libc::c_int as isize) as libc::c_int
                == ' ' as i32
            {
                parseit = Some(
                    def_parse_field
                        as unsafe extern "C" fn(
                            libc::c_long,
                            *mut *mut libc::c_char,
                            libc::c_int,
                            *mut NODE,
                            *mut Regexp,
                            Setfunc,
                            *mut NODE,
                            *mut NODE,
                            bool,
                        ) -> libc::c_long,
                );
            } else {
                parseit = Some(
                    sc_parse_field
                        as unsafe extern "C" fn(
                            libc::c_long,
                            *mut *mut libc::c_char,
                            libc::c_int,
                            *mut NODE,
                            *mut Regexp,
                            Setfunc,
                            *mut NODE,
                            *mut NODE,
                            bool,
                        ) -> libc::c_long,
                );
            }
        } else {
            parseit = Some(
                re_parse_field
                    as unsafe extern "C" fn(
                        libc::c_long,
                        *mut *mut libc::c_char,
                        libc::c_int,
                        *mut NODE,
                        *mut Regexp,
                        Setfunc,
                        *mut NODE,
                        *mut NODE,
                        bool,
                    ) -> libc::c_long,
            );
            rp = re_update(sep);
        }
    }
    s = (*src).sub.val.sp;
    tmp = make_number
        .unwrap()(
        (Some(parseit.unwrap()))
            .unwrap()(
            9223372036854775807 as libc::c_long,
            &mut s,
            (*src).sub.val.slen as libc::c_int,
            fs,
            rp,
            Some(
                set_element
                    as unsafe extern "C" fn(
                        libc::c_long,
                        *mut libc::c_char,
                        libc::c_long,
                        *mut NODE,
                    ) -> (),
            ),
            arr,
            sep_arr,
            0 as libc::c_int != 0,
        ) as libc::c_double,
    );
    src = POP_SCALAR();
    DEREF(src);
    return tmp;
}
pub unsafe extern "C" fn do_patsplit(mut nargs: libc::c_int) -> *mut NODE {
    let mut src: *mut NODE = 0 as *mut NODE;
    let mut arr: *mut NODE = 0 as *mut NODE;
    let mut sep: *mut NODE = 0 as *mut NODE;
    let mut fpat: *mut NODE = 0 as *mut NODE;
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut sep_arr: *mut NODE = 0 as *mut NODE;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rp: *mut Regexp = 0 as *mut Regexp;
    check_args_min_max(
        nargs,
        b"patsplit\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int,
        4 as libc::c_int,
    );
    if nargs == 4 as libc::c_int {
        sep_arr = POP_PARAM();
        if (*sep_arr).type_0 as libc::c_uint
            != Node_var_array as libc::c_int as libc::c_uint
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"field.c\0" as *const u8 as *const libc::c_char,
                1091 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"patsplit: fourth argument is not an array\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        check_symtab_functab(
            sep_arr,
            b"patsplit\0" as *const u8 as *const libc::c_char,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: cannot use %s as fourth argument\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    let fresh16 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    sep = (*fresh16).rptr;
    arr = POP_PARAM();
    if (*arr).type_0 as libc::c_uint != Node_var_array as libc::c_int as libc::c_uint {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"field.c\0" as *const u8 as *const libc::c_char,
            1098 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"patsplit: second argument is not an array\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    check_symtab_functab(
        arr,
        b"patsplit\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"%s: cannot use %s as second argument\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    src = force_string_fmt(TOP_SCALAR(), CONVFMT, CONVFMTidx);
    if (*sep).flags as libc::c_uint & REGEX as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        sep = (*sep).sub.val.typre;
    }
    fpat = (*sep).sub.nodep.x.extra;
    if (*fpat).sub.val.slen == 0 as libc::c_int as libc::c_ulong {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"field.c\0" as *const u8 as *const libc::c_char,
            1109 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"patsplit: third argument must be non-null\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if !sep_arr.is_null() {
        if sep_arr == arr {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"field.c\0" as *const u8 as *const libc::c_char,
                1113 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"patsplit: cannot use the same array for second and fourth args\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        tmp = (*sep_arr).sub.nodep.x.extra;
        while !tmp.is_null() {
            if tmp == arr {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"field.c\0" as *const u8 as *const libc::c_char,
                    1118 as libc::c_int,
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
                        b"patsplit: cannot use a subarray of second arg for fourth arg\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            tmp = (*tmp).sub.nodep.x.extra;
        }
        tmp = (*arr).sub.nodep.x.extra;
        while !tmp.is_null() {
            if tmp == sep_arr {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"field.c\0" as *const u8 as *const libc::c_char,
                    1121 as libc::c_int,
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
                        b"patsplit: cannot use a subarray of fourth arg for second arg\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            tmp = (*tmp).sub.nodep.x.extra;
        }
        ((*(*sep_arr).sub.nodep.l.lp).clear).unwrap()(sep_arr, 0 as *mut exp_node);
    }
    ((*(*arr).sub.nodep.l.lp).clear).unwrap()(arr, 0 as *mut exp_node);
    if (*src).sub.val.slen == 0 as libc::c_int as libc::c_ulong {
        tmp = make_number.unwrap()(0 as libc::c_int as libc::c_double);
    } else {
        rp = re_update(sep);
        s = (*src).sub.val.sp;
        tmp = make_number
            .unwrap()(
            fpat_parse_field(
                9223372036854775807 as libc::c_long,
                &mut s,
                (*src).sub.val.slen as libc::c_int,
                fpat,
                rp,
                Some(
                    set_element
                        as unsafe extern "C" fn(
                            libc::c_long,
                            *mut libc::c_char,
                            libc::c_long,
                            *mut NODE,
                        ) -> (),
                ),
                arr,
                sep_arr,
                0 as libc::c_int != 0,
            ) as libc::c_double,
        );
    }
    src = POP_SCALAR();
    DEREF(src);
    return tmp;
}
unsafe extern "C" fn set_parser(mut func: parse_field_func_t) {
    normal_parse_field = func;
    if !api_parser_override && parse_field != func {
        parse_field = func;
        update_PROCINFO_str(
            b"FS\0" as *const u8 as *const libc::c_char,
            current_field_sep_str(),
        );
    }
}
pub unsafe extern "C" fn set_FIELDWIDTHS() {
    let mut scan: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    static mut fw_alloc: libc::c_int = 4 as libc::c_int;
    static mut warned: bool = 0 as libc::c_int != 0;
    let mut fatal_error: bool = 0 as libc::c_int != 0;
    let mut tmp: *mut NODE = 0 as *mut NODE;
    if do_flags as libc::c_uint & DO_LINT_EXTENSIONS as libc::c_int as libc::c_uint != 0
        && !warned
    {
        warned = 1 as libc::c_int != 0;
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"field.c\0" as *const u8 as *const libc::c_char,
            1171 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"`FIELDWIDTHS' is a gawk extension\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0 {
        return;
    }
    if !fields_arr.is_null() {
        get_field(
            9223372036854775807 as libc::c_long - 1 as libc::c_int as libc::c_long,
            0 as *mut Func_ptr,
        );
    }
    set_parser(
        Some(
            fw_parse_field
                as unsafe extern "C" fn(
                    libc::c_long,
                    *mut *mut libc::c_char,
                    libc::c_int,
                    *mut NODE,
                    *mut Regexp,
                    Setfunc,
                    *mut NODE,
                    *mut NODE,
                    bool,
                ) -> libc::c_long,
        ),
    );
    tmp = force_string_fmt((*FIELDWIDTHS_node).sub.nodep.l.lptr, CONVFMT, CONVFMTidx);
    scan = (*tmp).sub.val.sp;
    if FIELDWIDTHS.is_null() {
        FIELDWIDTHS = emalloc_real(
            (::std::mem::size_of::<awk_fieldwidth_info_t>() as libc::c_ulong)
                .wrapping_add(
                    ((fw_alloc - 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<awk_field_info>() as libc::c_ulong,
                        ),
                ),
            b"set_FIELDWIDTHS\0" as *const u8 as *const libc::c_char,
            b"FIELDWIDTHS\0" as *const u8 as *const libc::c_char,
            b"field.c\0" as *const u8 as *const libc::c_char,
            1188 as libc::c_int,
        ) as *mut awk_fieldwidth_info_t;
        (*FIELDWIDTHS).use_chars = awk_true;
    }
    (*FIELDWIDTHS).nf = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int;
    loop {
        let mut tmp_0: libc::c_ulong = 0;
        if i >= fw_alloc {
            fw_alloc *= 2 as libc::c_int;
            FIELDWIDTHS = erealloc_real(
                FIELDWIDTHS as *mut libc::c_void,
                (::std::mem::size_of::<awk_fieldwidth_info_t>() as libc::c_ulong)
                    .wrapping_add(
                        ((fw_alloc - 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<awk_field_info>() as libc::c_ulong,
                            ),
                    ),
                b"set_FIELDWIDTHS\0" as *const u8 as *const libc::c_char,
                b"FIELDWIDTHS\0" as *const u8 as *const libc::c_char,
                b"field.c\0" as *const u8 as *const libc::c_char,
                1196 as libc::c_int,
            ) as *mut awk_fieldwidth_info_t;
        }
        while is_blank(*scan as libc::c_int) != 0 {
            scan = scan.offset(1);
            scan;
        }
        if *scan as libc::c_int == '-' as i32 {
            fatal_error = 1 as libc::c_int != 0;
            break;
        } else {
            if *scan as libc::c_int == '\0' as i32 {
                break;
            }
            *__errno_location() = 0 as libc::c_int;
            tmp_0 = strtoul(scan, &mut end, 10 as libc::c_int);
            if *__errno_location() == 0 as libc::c_int
                && *end as libc::c_int == ':' as i32
                && ((0 as libc::c_int as libc::c_ulong) < tmp_0
                    && tmp_0
                        <= (2147483647 as libc::c_int as libc::c_uint)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint) as libc::c_ulong)
            {
                (*((*FIELDWIDTHS).fields).as_mut_ptr().offset(i as isize)).skip = tmp_0;
                scan = end.offset(1 as libc::c_int as isize);
                if *scan as libc::c_int == '-' as i32
                    || is_blank(*scan as libc::c_int) != 0
                {
                    fatal_error = 1 as libc::c_int != 0;
                    break;
                } else {
                    tmp_0 = strtoul(scan, &mut end, 10 as libc::c_int);
                }
            } else {
                (*((*FIELDWIDTHS).fields).as_mut_ptr().offset(i as isize))
                    .skip = 0 as libc::c_int as size_t;
            }
            if *__errno_location() != 0 as libc::c_int
                || *end as libc::c_int != '\0' as i32
                    && is_blank(*end as libc::c_int) == 0
                || !((0 as libc::c_int as libc::c_ulong) < tmp_0
                    && tmp_0
                        <= (2147483647 as libc::c_int as libc::c_uint)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint) as libc::c_ulong)
            {
                if *scan as libc::c_int == '*' as i32 {
                    scan = scan.offset(1);
                    scan;
                    while is_blank(*scan as libc::c_int) != 0 {
                        scan = scan.offset(1);
                        scan;
                    }
                    if *scan as libc::c_int != '\0' as i32 {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"field.c\0" as *const u8 as *const libc::c_char,
                            1240 as libc::c_int,
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
                                b"`*' must be the last designator in FIELDWIDTHS\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                    (*((*FIELDWIDTHS).fields).as_mut_ptr().offset(i as isize))
                        .len = (2147483647 as libc::c_int as libc::c_uint)
                        .wrapping_mul(2 as libc::c_uint)
                        .wrapping_add(1 as libc::c_uint) as size_t;
                    (*FIELDWIDTHS).nf = (i + 1 as libc::c_int) as size_t;
                } else {
                    fatal_error = 1 as libc::c_int != 0;
                }
                break;
            } else {
                (*((*FIELDWIDTHS).fields).as_mut_ptr().offset(i as isize)).len = tmp_0;
                (*FIELDWIDTHS).nf = (i + 1 as libc::c_int) as size_t;
                scan = end;
                while is_blank(*scan as libc::c_int) != 0 {
                    scan = scan.offset(1);
                    scan;
                }
                if *scan as libc::c_int == '\0' as i32 {
                    break;
                }
                i += 1;
                i;
            }
        }
    }
    if fatal_error {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"field.c\0" as *const u8 as *const libc::c_char,
            1261 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"invalid FIELDWIDTHS value, for field %d, near `%s'\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            i + 1 as libc::c_int,
            scan,
        );
    }
}
pub unsafe extern "C" fn set_FS() {
    let mut buf: [libc::c_char; 10] = [0; 10];
    let mut fs: *mut NODE = 0 as *mut NODE;
    static mut save_fs: *mut NODE = 0 as *const NODE as *mut NODE;
    static mut save_rs: *mut NODE = 0 as *const NODE as *mut NODE;
    let mut remake_re: bool = 1 as libc::c_int != 0;
    if !fields_arr.is_null() {
        get_field(
            9223372036854775807 as libc::c_long - 1 as libc::c_int as libc::c_long,
            0 as *mut Func_ptr,
        );
    }
    if !save_fs.is_null()
        && (*(*FS_node).sub.nodep.l.lptr).sub.val.slen == (*save_fs).sub.val.slen
        && memcmp(
            (*(*FS_node).sub.nodep.l.lptr).sub.val.sp as *const libc::c_void,
            (*save_fs).sub.val.sp as *const libc::c_void,
            (*save_fs).sub.val.slen,
        ) == 0 as libc::c_int && !save_rs.is_null()
        && (*(*RS_node).sub.nodep.l.lptr).sub.val.slen == (*save_rs).sub.val.slen
        && memcmp(
            (*(*RS_node).sub.nodep.l.lptr).sub.val.sp as *const libc::c_void,
            (*save_rs).sub.val.sp as *const libc::c_void,
            (*save_rs).sub.val.slen,
        ) == 0 as libc::c_int
    {
        if !FS_regexp.is_null() {
            FS_regexp = if IGNORECASE as libc::c_int != 0 {
                FS_re_no_case
            } else {
                FS_re_yes_case
            };
        }
        if current_field_sep() as libc::c_uint == Using_FS as libc::c_int as libc::c_uint
        {
            return
        } else {
            remake_re = 0 as libc::c_int != 0;
        }
    } else {
        unref(save_fs);
        save_fs = dupnode((*FS_node).sub.nodep.l.lptr);
        unref(save_rs);
        save_rs = dupnode((*RS_node).sub.nodep.l.lptr);
        resave_fs = 1 as libc::c_int != 0;
        refree(FS_re_yes_case);
        refree(FS_re_no_case);
        FS_regexp = 0 as *mut Regexp;
        FS_re_no_case = FS_regexp;
        FS_re_yes_case = FS_re_no_case;
    }
    buf[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    default_FS = 0 as libc::c_int;
    fs = force_string_fmt((*FS_node).sub.nodep.l.lptr, CONVFMT, CONVFMTidx);
    if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint == 0
        && (*fs).sub.val.slen == 0 as libc::c_int as libc::c_ulong
    {
        static mut warned: bool = 0 as libc::c_int != 0;
        set_parser(
            Some(
                null_parse_field
                    as unsafe extern "C" fn(
                        libc::c_long,
                        *mut *mut libc::c_char,
                        libc::c_int,
                        *mut NODE,
                        *mut Regexp,
                        Setfunc,
                        *mut NODE,
                        *mut NODE,
                        bool,
                    ) -> libc::c_long,
            ),
        );
        if do_flags as libc::c_uint & DO_LINT_EXTENSIONS as libc::c_int as libc::c_uint
            != 0 && !warned
        {
            warned = 1 as libc::c_int != 0;
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"field.c\0" as *const u8 as *const libc::c_char,
                1334 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"null string for `FS' is a gawk extension\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
    } else if (*fs).sub.val.slen > 1 as libc::c_int as libc::c_ulong
        || (*fs).flags as libc::c_uint & REGEX as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
    {
        if do_flags as libc::c_uint & DO_LINT_OLD as libc::c_int as libc::c_uint != 0 {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"field.c\0" as *const u8 as *const libc::c_char,
                1338 as libc::c_int,
            );
            (Some(lintfunc.unwrap()))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"old awk does not support regexps as value of `FS'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        set_parser(
            Some(
                re_parse_field
                    as unsafe extern "C" fn(
                        libc::c_long,
                        *mut *mut libc::c_char,
                        libc::c_int,
                        *mut NODE,
                        *mut Regexp,
                        Setfunc,
                        *mut NODE,
                        *mut NODE,
                        bool,
                    ) -> libc::c_long,
            ),
        );
    } else if RS_is_null {
        set_parser(
            Some(
                sc_parse_field
                    as unsafe extern "C" fn(
                        libc::c_long,
                        *mut *mut libc::c_char,
                        libc::c_int,
                        *mut NODE,
                        *mut Regexp,
                        Setfunc,
                        *mut NODE,
                        *mut NODE,
                        bool,
                    ) -> libc::c_long,
            ),
        );
        if (*fs).sub.val.slen == 1 as libc::c_int as libc::c_ulong {
            if *((*fs).sub.val.sp).offset(0 as libc::c_int as isize) as libc::c_int
                == ' ' as i32
            {
                default_FS = 1 as libc::c_int;
                strcpy(
                    buf.as_mut_ptr(),
                    b"[ \t\n]+\0" as *const u8 as *const libc::c_char,
                );
            } else if *((*fs).sub.val.sp).offset(0 as libc::c_int as isize)
                as libc::c_int == '\\' as i32
            {
                strcpy(
                    buf.as_mut_ptr(),
                    b"[\\\\\n]\0" as *const u8 as *const libc::c_char,
                );
            } else if *((*fs).sub.val.sp).offset(0 as libc::c_int as isize)
                as libc::c_int == '\0' as i32
            {
                strcpy(
                    buf.as_mut_ptr(),
                    b"[\\000\n]\0" as *const u8 as *const libc::c_char,
                );
            } else if *((*fs).sub.val.sp).offset(0 as libc::c_int as isize)
                as libc::c_int != '\n' as i32
            {
                sprintf(
                    buf.as_mut_ptr(),
                    b"[%c\n]\0" as *const u8 as *const libc::c_char,
                    *((*fs).sub.val.sp).offset(0 as libc::c_int as isize) as libc::c_int,
                );
            }
        }
    } else {
        set_parser(
            Some(
                def_parse_field
                    as unsafe extern "C" fn(
                        libc::c_long,
                        *mut *mut libc::c_char,
                        libc::c_int,
                        *mut NODE,
                        *mut Regexp,
                        Setfunc,
                        *mut NODE,
                        *mut NODE,
                        bool,
                    ) -> libc::c_long,
            ),
        );
        if (*fs).sub.val.slen == 1 as libc::c_int as libc::c_ulong {
            if *((*fs).sub.val.sp).offset(0 as libc::c_int as isize) as libc::c_int
                == ' ' as i32
            {
                default_FS = 1 as libc::c_int;
            } else if *((*fs).sub.val.sp).offset(0 as libc::c_int as isize)
                as libc::c_int == '\\' as i32
            {
                strcpy(
                    buf.as_mut_ptr(),
                    b"[\\\\]\0" as *const u8 as *const libc::c_char,
                );
            } else {
                set_parser(
                    Some(
                        sc_parse_field
                            as unsafe extern "C" fn(
                                libc::c_long,
                                *mut *mut libc::c_char,
                                libc::c_int,
                                *mut NODE,
                                *mut Regexp,
                                Setfunc,
                                *mut NODE,
                                *mut NODE,
                                bool,
                            ) -> libc::c_long,
                    ),
                );
            }
        }
    }
    if remake_re {
        refree(FS_re_yes_case);
        refree(FS_re_no_case);
        FS_regexp = 0 as *mut Regexp;
        FS_re_no_case = FS_regexp;
        FS_re_yes_case = FS_re_no_case;
        if buf[0 as libc::c_int as usize] as libc::c_int != '\0' as i32 {
            FS_re_yes_case = make_regexp(
                buf.as_mut_ptr(),
                strlen(buf.as_mut_ptr()),
                0 as libc::c_int != 0,
                1 as libc::c_int != 0,
                1 as libc::c_int != 0,
            );
            FS_re_no_case = make_regexp(
                buf.as_mut_ptr(),
                strlen(buf.as_mut_ptr()),
                1 as libc::c_int != 0,
                1 as libc::c_int != 0,
                1 as libc::c_int != 0,
            );
            FS_regexp = if IGNORECASE as libc::c_int != 0 {
                FS_re_no_case
            } else {
                FS_re_yes_case
            };
            set_parser(
                Some(
                    re_parse_field
                        as unsafe extern "C" fn(
                            libc::c_long,
                            *mut *mut libc::c_char,
                            libc::c_int,
                            *mut NODE,
                            *mut Regexp,
                            Setfunc,
                            *mut NODE,
                            *mut NODE,
                            bool,
                        ) -> libc::c_long,
                ),
            );
        } else if parse_field
            == Some(
                re_parse_field
                    as unsafe extern "C" fn(
                        libc::c_long,
                        *mut *mut libc::c_char,
                        libc::c_int,
                        *mut NODE,
                        *mut Regexp,
                        Setfunc,
                        *mut NODE,
                        *mut NODE,
                        bool,
                    ) -> libc::c_long,
            )
        {
            FS_re_yes_case = make_regexp(
                (*fs).sub.val.sp,
                (*fs).sub.val.slen,
                0 as libc::c_int != 0,
                1 as libc::c_int != 0,
                1 as libc::c_int != 0,
            );
            FS_re_no_case = make_regexp(
                (*fs).sub.val.sp,
                (*fs).sub.val.slen,
                1 as libc::c_int != 0,
                1 as libc::c_int != 0,
                1 as libc::c_int != 0,
            );
            FS_regexp = if IGNORECASE as libc::c_int != 0 {
                FS_re_no_case
            } else {
                FS_re_yes_case
            };
        } else {
            FS_regexp = 0 as *mut Regexp;
            FS_re_no_case = FS_regexp;
            FS_re_yes_case = FS_re_no_case;
        }
    }
    if (*fs).sub.val.slen == 1 as libc::c_int as libc::c_ulong
        && parse_field
            == Some(
                re_parse_field
                    as unsafe extern "C" fn(
                        libc::c_long,
                        *mut *mut libc::c_char,
                        libc::c_int,
                        *mut NODE,
                        *mut Regexp,
                        Setfunc,
                        *mut NODE,
                        *mut NODE,
                        bool,
                    ) -> libc::c_long,
            )
    {
        FS_regexp = FS_re_yes_case;
    }
}
pub unsafe extern "C" fn current_field_sep() -> field_sep_type {
    if api_parser_override {
        return Using_API
    } else if parse_field
        == Some(
            fw_parse_field
                as unsafe extern "C" fn(
                    libc::c_long,
                    *mut *mut libc::c_char,
                    libc::c_int,
                    *mut NODE,
                    *mut Regexp,
                    Setfunc,
                    *mut NODE,
                    *mut NODE,
                    bool,
                ) -> libc::c_long,
        )
    {
        return Using_FIELDWIDTHS
    } else if parse_field
        == Some(
            fpat_parse_field
                as unsafe extern "C" fn(
                    libc::c_long,
                    *mut *mut libc::c_char,
                    libc::c_int,
                    *mut NODE,
                    *mut Regexp,
                    Setfunc,
                    *mut NODE,
                    *mut NODE,
                    bool,
                ) -> libc::c_long,
        )
    {
        return Using_FPAT
    } else {
        return Using_FS
    };
}
pub unsafe extern "C" fn current_field_sep_str() -> *const libc::c_char {
    if api_parser_override {
        return b"API\0" as *const u8 as *const libc::c_char
    } else if parse_field
        == Some(
            fw_parse_field
                as unsafe extern "C" fn(
                    libc::c_long,
                    *mut *mut libc::c_char,
                    libc::c_int,
                    *mut NODE,
                    *mut Regexp,
                    Setfunc,
                    *mut NODE,
                    *mut NODE,
                    bool,
                ) -> libc::c_long,
        )
    {
        return b"FIELDWIDTHS\0" as *const u8 as *const libc::c_char
    } else if parse_field
        == Some(
            fpat_parse_field
                as unsafe extern "C" fn(
                    libc::c_long,
                    *mut *mut libc::c_char,
                    libc::c_int,
                    *mut NODE,
                    *mut Regexp,
                    Setfunc,
                    *mut NODE,
                    *mut NODE,
                    bool,
                ) -> libc::c_long,
        )
    {
        return b"FPAT\0" as *const u8 as *const libc::c_char
    } else {
        return b"FS\0" as *const u8 as *const libc::c_char
    };
}
pub unsafe extern "C" fn update_PROCINFO_str(
    mut subscript: *const libc::c_char,
    mut str: *const libc::c_char,
) {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    if PROCINFO_node.is_null() {
        return;
    }
    tmp = make_str_node(subscript, strlen(subscript), 0 as libc::c_int);
    assoc_set(PROCINFO_node, tmp, make_str_node(str, strlen(str), 0 as libc::c_int));
}
pub unsafe extern "C" fn update_PROCINFO_num(
    mut subscript: *const libc::c_char,
    mut val: libc::c_double,
) {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    if PROCINFO_node.is_null() {
        return;
    }
    tmp = make_str_node(subscript, strlen(subscript), 0 as libc::c_int);
    assoc_set(PROCINFO_node, tmp, make_number.unwrap()(val));
}
pub unsafe extern "C" fn set_FPAT() {
    static mut warned: bool = 0 as libc::c_int != 0;
    let mut remake_re: bool = 1 as libc::c_int != 0;
    let mut fpat: *mut NODE = 0 as *mut NODE;
    if do_flags as libc::c_uint & DO_LINT_EXTENSIONS as libc::c_int as libc::c_uint != 0
        && !warned
    {
        warned = 1 as libc::c_int != 0;
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"field.c\0" as *const u8 as *const libc::c_char,
            1464 as libc::c_int,
        );
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"`FPAT' is a gawk extension\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0 {
        return;
    }
    if !fields_arr.is_null() {
        get_field(
            9223372036854775807 as libc::c_long - 1 as libc::c_int as libc::c_long,
            0 as *mut Func_ptr,
        );
    }
    if !save_FPAT.is_null()
        && (*(*FPAT_node).sub.nodep.l.lptr).sub.val.slen == (*save_FPAT).sub.val.slen
        && memcmp(
            (*(*FPAT_node).sub.nodep.l.lptr).sub.val.sp as *const libc::c_void,
            (*save_FPAT).sub.val.sp as *const libc::c_void,
            (*save_FPAT).sub.val.slen,
        ) == 0 as libc::c_int
    {
        if !FPAT_regexp.is_null() {
            FPAT_regexp = if IGNORECASE as libc::c_int != 0 {
                FPAT_re_no_case
            } else {
                FPAT_re_yes_case
            };
        }
        if current_field_sep() as libc::c_uint
            == Using_FPAT as libc::c_int as libc::c_uint
        {
            return
        } else {
            remake_re = 0 as libc::c_int != 0;
        }
    } else {
        unref(save_FPAT);
        save_FPAT = dupnode((*FPAT_node).sub.nodep.l.lptr);
        refree(FPAT_re_yes_case);
        refree(FPAT_re_no_case);
        FPAT_regexp = 0 as *mut Regexp;
        FPAT_re_no_case = FPAT_regexp;
        FPAT_re_yes_case = FPAT_re_no_case;
    }
    fpat = force_string_fmt((*FPAT_node).sub.nodep.l.lptr, CONVFMT, CONVFMTidx);
    set_parser(
        Some(
            fpat_parse_field
                as unsafe extern "C" fn(
                    libc::c_long,
                    *mut *mut libc::c_char,
                    libc::c_int,
                    *mut NODE,
                    *mut Regexp,
                    Setfunc,
                    *mut NODE,
                    *mut NODE,
                    bool,
                ) -> libc::c_long,
        ),
    );
    if remake_re {
        refree(FPAT_re_yes_case);
        refree(FPAT_re_no_case);
        FPAT_regexp = 0 as *mut Regexp;
        FPAT_re_no_case = FPAT_regexp;
        FPAT_re_yes_case = FPAT_re_no_case;
        FPAT_re_yes_case = make_regexp(
            (*fpat).sub.val.sp,
            (*fpat).sub.val.slen,
            0 as libc::c_int != 0,
            1 as libc::c_int != 0,
            1 as libc::c_int != 0,
        );
        FPAT_re_no_case = make_regexp(
            (*fpat).sub.val.sp,
            (*fpat).sub.val.slen,
            1 as libc::c_int != 0,
            1 as libc::c_int != 0,
            1 as libc::c_int != 0,
        );
        FPAT_regexp = if IGNORECASE as libc::c_int != 0 {
            FPAT_re_no_case
        } else {
            FPAT_re_yes_case
        };
    }
}
unsafe extern "C" fn incr_scan(
    mut scanp: *mut *mut libc::c_char,
    mut len: size_t,
    mut mbs: *mut mbstate_t,
) {
    let mut mbclen: size_t = 0 as libc::c_int as size_t;
    if gawk_mb_cur_max > 1 as libc::c_int {
        mbclen = mbrlen(*scanp, len, mbs);
        if mbclen == 1 as libc::c_int as libc::c_ulong
            || mbclen == -(1 as libc::c_int) as size_t
            || mbclen == -(2 as libc::c_int) as size_t
            || mbclen == 0 as libc::c_int as libc::c_ulong
        {
            mbclen = 1 as libc::c_int as size_t;
        }
        *scanp = (*scanp).offset(mbclen as isize);
    } else {
        *scanp = (*scanp).offset(1);
        *scanp;
    };
}
unsafe extern "C" fn fpat_parse_field(
    mut up_to: libc::c_long,
    mut buf: *mut *mut libc::c_char,
    mut len: libc::c_int,
    mut fs: *mut NODE,
    mut rp: *mut Regexp,
    mut set: Setfunc,
    mut n: *mut NODE,
    mut sep_arr: *mut NODE,
    mut in_middle: bool,
) -> libc::c_long {
    let mut scan: *mut libc::c_char = *buf;
    let mut nf: libc::c_long = parse_high_water;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = scan.offset(len as isize);
    let mut regex_flags: libc::c_int = 1 as libc::c_int;
    let mut mbs: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    let mut field_start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut field_found: bool = 0 as libc::c_int != 0;
    memset(
        &mut mbs as *mut mbstate_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
    );
    if up_to == 9223372036854775807 as libc::c_long {
        nf = 0 as libc::c_int as libc::c_long;
    }
    if len == 0 as libc::c_int {
        return nf;
    }
    if rp.is_null() {
        rp = FPAT_regexp;
    }
    while scan < end && nf < up_to {
        start = scan;
        field_found = research(
            rp,
            scan,
            0 as libc::c_int,
            end.offset_from(scan) as libc::c_long as size_t,
            regex_flags,
        ) != -(1 as libc::c_int);
        if nf > 0 as libc::c_int as libc::c_long && field_found as libc::c_int != 0
            && *((*rp).regs.end).offset(0 as libc::c_int as isize) == 0 as libc::c_int
        {
            incr_scan(
                &mut scan,
                end.offset_from(scan) as libc::c_long as size_t,
                &mut mbs,
            );
            field_found = research(
                rp,
                scan,
                0 as libc::c_int,
                end.offset_from(scan) as libc::c_long as size_t,
                regex_flags,
            ) != -(1 as libc::c_int);
        }
        if field_found {
            field_start = scan
                .offset(*((*rp).regs.start).offset(0 as libc::c_int as isize) as isize);
            if !sep_arr.is_null() {
                if field_start == start {
                    set_element(nf, start, 0 as libc::c_long, sep_arr);
                } else {
                    set_element(
                        nf,
                        start,
                        field_start.offset_from(start) as libc::c_long,
                        sep_arr,
                    );
                }
            }
            nf += 1;
            (Some(set.unwrap()))
                .unwrap()(
                nf,
                field_start,
                (*((*rp).regs.end).offset(0 as libc::c_int as isize)
                    - *((*rp).regs.start).offset(0 as libc::c_int as isize))
                    as libc::c_long,
                n,
            );
            scan = scan
                .offset(*((*rp).regs.end).offset(0 as libc::c_int as isize) as isize);
        } else {
            if !sep_arr.is_null() {
                set_element(nf, start, end.offset_from(start) as libc::c_long, sep_arr);
            }
            scan = end;
        }
    }
    if !sep_arr.is_null() && scan == end && field_found as libc::c_int != 0 {
        set_element(nf, scan, 0 as libc::c_long, sep_arr);
    }
    *buf = scan;
    return nf;
}
