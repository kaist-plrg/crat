use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type re_dfa_t;
    pub type dfa;
    pub type break_point;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn re_set_syntax(__syntax: reg_syntax_t) -> reg_syntax_t;
    fn re_compile_pattern(
        __pattern: *const libc::c_char,
        __length: size_t,
        __buffer: *mut re_pattern_buffer,
    ) -> *const libc::c_char;
    fn re_search(
        __buffer: *mut re_pattern_buffer,
        __String: *const libc::c_char,
        __length: regoff_t,
        __start: regoff_t,
        __range: regoff_t,
        __regs: *mut re_registers,
    ) -> regoff_t;
    fn regfree(__preg: *mut regex_t);
    fn dfaalloc() -> *mut dfa;
    fn dfasyntax(_: *mut dfa, _: *const localeinfo, _: reg_syntax_t, _: libc::c_int);
    fn dfacopysyntax(_: *mut dfa, _: *const dfa);
    fn dfacomp(_: *const libc::c_char, _: idx_t, _: *mut dfa, _: bool);
    fn dfaexec(
        d: *mut dfa,
        begin: *const libc::c_char,
        end: *mut libc::c_char,
        allow_nl: bool,
        count: *mut idx_t,
        backref: *mut bool,
    ) -> *mut libc::c_char;
    fn dfasuperset(d: *const dfa) -> *mut dfa;
    fn dfaisfast(_: *const dfa) -> bool;
    fn dfafree(_: *mut dfa);
    fn r_fatal(mesg: *const libc::c_char, _: ...);
    fn set_loc(file: *const libc::c_char, line: libc::c_int);
    static mut IGNORECASE: bool;
    static mut do_flags: do_flag_values;
    static mut gawk_mb_cur_max: libc::c_int;
    static mut casetable: [libc::c_char; 0];
    fn r_unref(tmp: *mut NODE);
    fn cmp_nodes(t1: *mut NODE, t2: *mut NODE, use_strcmp: bool) -> libc::c_int;
    fn genflags2str(flagval: libc::c_int, tab: *const flagtab) -> *const libc::c_char;
    fn error(mesg: *const libc::c_char, _: ...);
    fn r_warning(mesg: *const libc::c_char, _: ...);
    static mut lintfunc: Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>;
    fn r_dupnode(n: *mut NODE) -> *mut NODE;
    fn parse_escape(string_ptr: *mut *const libc::c_char) -> libc::c_int;
    fn mbrtowc(
        __pwc: *mut wchar_t,
        __s: *const libc::c_char,
        __n: size_t,
        __p: *mut mbstate_t,
    ) -> size_t;
    fn __mbrlen(__s: *const libc::c_char, __n: size_t, __ps: *mut mbstate_t) -> size_t;
    fn init_localeinfo(_: *mut localeinfo);
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
pub type regex_t = re_pattern_buffer;
pub type regoff_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_registers {
    pub num_regs: __re_size_t,
    pub start: *mut regoff_t,
    pub end: *mut regoff_t,
}
pub type ptrdiff_t = libc::c_long;
pub type idx_t = ptrdiff_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct localeinfo {
    pub multibyte: bool,
    pub simple: bool,
    pub using_utf8: bool,
    pub sbclen: [libc::c_schar; 256],
    pub sbctowc: [wint_t; 256],
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const DFA_PLUS_WARN: C2RustUnnamed_1 = 32;
pub const DFA_STAR_WARN: C2RustUnnamed_1 = 16;
pub const DFA_STRAY_BACKSLASH_WARN: C2RustUnnamed_1 = 8;
pub const DFA_CONFUSING_BRACKETS_ERROR: C2RustUnnamed_1 = 4;
pub const DFA_EOL_NUL: C2RustUnnamed_1 = 2;
pub const DFA_ANCHOR: C2RustUnnamed_1 = 1;
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
    pub u: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
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
    pub sub: C2RustUnnamed_3,
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
pub union C2RustUnnamed_3 {
    pub nodep: C2RustUnnamed_5,
    pub val: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
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
pub struct C2RustUnnamed_5 {
    pub l: C2RustUnnamed_12,
    pub r: C2RustUnnamed_7,
    pub x: C2RustUnnamed_6,
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
pub union C2RustUnnamed_6 {
    pub extra: *mut exp_node,
    pub aptr: Option::<unsafe extern "C" fn() -> ()>,
    pub xl: libc::c_long,
    pub cmnt: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
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
    pub d: C2RustUnnamed_9,
    pub x: C2RustUnnamed_8,
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
pub union C2RustUnnamed_8 {
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
pub union C2RustUnnamed_9 {
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
    pub hs: C2RustUnnamed_11,
    pub hi: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub next: *mut bucket_item,
    pub li: [libc::c_long; 2],
    pub val: [*mut exp_node; 2],
    pub cnt: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub next: *mut bucket_item,
    pub str_0: *mut libc::c_char,
    pub len: size_t,
    pub code: size_t,
    pub name: *mut exp_node,
    pub val: *mut exp_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
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
pub struct flagtab {
    pub val: libc::c_int,
    pub name: *const libc::c_char,
}
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
pub struct reclass {
    pub name: *const libc::c_char,
    pub len: size_t,
    pub warned: bool,
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
static mut syn: reg_syntax_t = 0;
static mut localeinfo: localeinfo = localeinfo {
    multibyte: false,
    simple: false,
    using_utf8: false,
    sbclen: [0; 256],
    sbctowc: [0; 256],
};
pub unsafe extern "C" fn make_regexp(
    mut s: *const libc::c_char,
    mut len: size_t,
    mut ignorecase: bool,
    mut dfa: bool,
    mut canfatal: bool,
) -> *mut Regexp {
    static mut metas: [libc::c_char; 15] = unsafe {
        *::std::mem::transmute::<
            &[u8; 15],
            &mut [libc::c_char; 15],
        >(b".*+(){}[]|?^$\\\0")
    };
    let mut rp: *mut Regexp = 0 as *mut Regexp;
    let mut rerr: *const libc::c_char = 0 as *const libc::c_char;
    let mut src: *const libc::c_char = s;
    static mut buf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut buflen: size_t = 0;
    let mut end: *const libc::c_char = s.offset(len as isize);
    let mut dest: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    static mut first: bool = 1 as libc::c_int != 0;
    static mut no_dfa: bool = 0 as libc::c_int != 0;
    let mut i: libc::c_int = 0;
    static mut dfaregs: [*mut dfa; 2] = [
        0 as *const dfa as *mut dfa,
        0 as *const dfa as *mut dfa,
    ];
    static mut nul_warned: bool = 0 as libc::c_int != 0;
    if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0 && !nul_warned
        && !(memchr(s as *const libc::c_void, '\0' as i32, len)).is_null()
    {
        nul_warned = 1 as libc::c_int != 0;
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(b"re.c\0" as *const u8 as *const libc::c_char, 58 as libc::c_int);
        (Some(lintfunc.unwrap()))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"behavior of matching a regexp containing NUL characters is not defined by POSIX\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    let mut is_multibyte: size_t = 0 as libc::c_int as size_t;
    let mut mbs: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    memset(
        &mut mbs as *mut mbstate_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
    );
    if first {
        no_dfa = !(getenv(b"GAWK_NO_DFA\0" as *const u8 as *const libc::c_char))
            .is_null();
    }
    check_bracket_exp(s as *mut libc::c_char, len);
    if buf.is_null() {
        buf = emalloc_real(
            len.wrapping_add(1 as libc::c_int as libc::c_ulong),
            b"make_regexp\0" as *const u8 as *const libc::c_char,
            b"buf\0" as *const u8 as *const libc::c_char,
            b"re.c\0" as *const u8 as *const libc::c_char,
            87 as libc::c_int,
        ) as *mut libc::c_char;
        buflen = len;
    } else if len > buflen {
        buf = erealloc_real(
            buf as *mut libc::c_void,
            len.wrapping_add(1 as libc::c_int as libc::c_ulong),
            b"make_regexp\0" as *const u8 as *const libc::c_char,
            b"buf\0" as *const u8 as *const libc::c_char,
            b"re.c\0" as *const u8 as *const libc::c_char,
            90 as libc::c_int,
        ) as *mut libc::c_char;
        buflen = len;
    }
    dest = buf;
    while src < end {
        if gawk_mb_cur_max > 1 as libc::c_int && is_multibyte == 0 {
            is_multibyte = mbrlen(
                src,
                end.offset_from(src) as libc::c_long as size_t,
                &mut mbs,
            );
            if is_multibyte == 1 as libc::c_int as libc::c_ulong
                || is_multibyte == -(1 as libc::c_int) as size_t
                || is_multibyte == -(2 as libc::c_int) as size_t
                || is_multibyte == 0 as libc::c_int as libc::c_ulong
            {
                is_multibyte = 0 as libc::c_int as size_t;
            }
        }
        let mut ok_to_escape: *const libc::c_char = 0 as *const libc::c_char;
        if do_flags as libc::c_uint & DO_POSIX as libc::c_int as libc::c_uint != 0 {
            ok_to_escape = b"{}()|*+?.^$\\[]/-\0" as *const u8 as *const libc::c_char;
        } else if do_flags as libc::c_uint
            & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0
        {
            ok_to_escape = b"()|*+?.^$\\[]/-\0" as *const u8 as *const libc::c_char;
        } else {
            ok_to_escape = b"<>`'BywWsS{}()|*+?.^$\\[]/-\0" as *const u8
                as *const libc::c_char;
        }
        if (gawk_mb_cur_max == 1 as libc::c_int || is_multibyte == 0)
            && *src as libc::c_int == '\\' as i32
        {
            src = src.offset(1);
            c = *src as libc::c_int;
            let mut current_block_61: u64;
            match c {
                0 => {
                    if src >= s.offset(len as isize) {
                        let fresh0 = dest;
                        dest = dest.offset(1);
                        *fresh0 = '\\' as i32 as libc::c_char;
                    } else {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"re.c\0" as *const u8 as *const libc::c_char,
                            127 as libc::c_int,
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
                                b"invalid NUL byte in dynamic regexp\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                    current_block_61 = 2631791190359682872;
                }
                97 | 98 | 102 | 110 | 114 | 116 | 118 | 120 | 48 | 49 | 50 | 51 | 52 | 53
                | 54 | 55 => {
                    c2 = parse_escape(&mut src);
                    if c2 < 0 as libc::c_int {
                        r_fatal(
                            b"internal error: file %s, line %d: received bad result %d from parse_escape()\0"
                                as *const u8 as *const libc::c_char,
                            b"re.c\0" as *const u8 as *const libc::c_char,
                            147 as libc::c_int,
                            c2,
                        );
                    }
                    if do_flags as libc::c_uint
                        & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0
                        && do_flags as libc::c_uint
                            & DO_POSIX as libc::c_int as libc::c_uint == 0
                        && (*(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                            != 0 || c == 'x' as i32)
                        && !(strchr(
                            b"()|*+?.^$\\[]\0" as *const u8 as *const libc::c_char,
                            c2,
                        ))
                            .is_null()
                    {
                        let fresh1 = dest;
                        dest = dest.offset(1);
                        *fresh1 = '\\' as i32 as libc::c_char;
                    }
                    let fresh2 = dest;
                    dest = dest.offset(1);
                    *fresh2 = c2 as libc::c_char;
                    if do_flags as libc::c_uint
                        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int)
                            as libc::c_uint != 0 && !nul_warned && c2 == '\0' as i32
                    {
                        nul_warned = 1 as libc::c_int != 0;
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"re.c\0" as *const u8 as *const libc::c_char,
                            163 as libc::c_int,
                        );
                        (Some(lintfunc.unwrap()))
                            .unwrap()(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"behavior of matching a regexp containing NUL characters is not defined by POSIX\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                    current_block_61 = 2631791190359682872;
                }
                56 | 57 => {
                    let fresh3 = dest;
                    dest = dest.offset(1);
                    *fresh3 = c as libc::c_char;
                    src = src.offset(1);
                    src;
                    static mut warned: [bool; 2] = [false; 2];
                    if !warned[(c - '8' as i32) as usize] {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"re.c\0" as *const u8 as *const libc::c_char,
                            174 as libc::c_int,
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
                                b"regexp escape sequence `\\%c' treated as plain `%c'\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            c,
                            c,
                        );
                        warned[(c - '8' as i32) as usize] = 1 as libc::c_int != 0;
                    }
                    current_block_61 = 2631791190359682872;
                }
                121 => {
                    if do_flags as libc::c_uint
                        & DO_TRADITIONAL as libc::c_int as libc::c_uint == 0
                    {
                        let fresh4 = dest;
                        dest = dest.offset(1);
                        *fresh4 = '\\' as i32 as libc::c_char;
                        let fresh5 = dest;
                        dest = dest.offset(1);
                        *fresh5 = 'b' as i32 as libc::c_char;
                        src = src.offset(1);
                        src;
                        current_block_61 = 2631791190359682872;
                    } else {
                        current_block_61 = 14045013352484388205;
                    }
                }
                _ => {
                    current_block_61 = 14045013352484388205;
                }
            }
            match current_block_61 {
                14045013352484388205 => {
                    if (strchr(ok_to_escape, c)).is_null() {
                        static mut warned_0: [bool; 256] = [false; 256];
                        if !warned_0[(c & 0xff as libc::c_int) as usize] {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    libc::c_int,
                                ) -> ())(
                                b"re.c\0" as *const u8 as *const libc::c_char,
                                193 as libc::c_int,
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
                                    b"regexp escape sequence `\\%c' is not a known regexp operator\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                c,
                            );
                            warned_0[(c & 0xff as libc::c_int)
                                as usize] = 1 as libc::c_int != 0;
                        }
                    }
                    let fresh6 = dest;
                    dest = dest.offset(1);
                    *fresh6 = '\\' as i32 as libc::c_char;
                    let fresh7 = dest;
                    dest = dest.offset(1);
                    *fresh7 = c as libc::c_char;
                    src = src.offset(1);
                    src;
                }
                _ => {}
            }
        } else {
            c = *src as libc::c_int;
            let fresh8 = src;
            src = src.offset(1);
            let fresh9 = dest;
            dest = dest.offset(1);
            *fresh9 = *fresh8;
        }
        if gawk_mb_cur_max > 1 as libc::c_int && is_multibyte != 0 {
            is_multibyte = is_multibyte.wrapping_sub(1);
            is_multibyte;
        }
    }
    *dest = '\0' as i32 as libc::c_char;
    len = dest.offset_from(buf) as libc::c_long as size_t;
    rp = ezalloc_real(
        ::std::mem::size_of::<Regexp>() as libc::c_ulong,
        b"make_regexp\0" as *const u8 as *const libc::c_char,
        b"rp\0" as *const u8 as *const libc::c_char,
        b"re.c\0" as *const u8 as *const libc::c_char,
        213 as libc::c_int,
    ) as *mut Regexp;
    (*rp).pat.allocated = 0 as libc::c_int as __re_long_size_t;
    (*rp)
        .pat
        .fastmap = emalloc_real(
        256 as libc::c_int as size_t,
        b"make_regexp\0" as *const u8 as *const libc::c_char,
        b"rp->pat.fastmap\0" as *const u8 as *const libc::c_char,
        b"re.c\0" as *const u8 as *const libc::c_char,
        215 as libc::c_int,
    ) as *mut libc::c_char;
    ignorecase = ignorecase;
    if ignorecase {
        if gawk_mb_cur_max > 1 as libc::c_int {
            syn
                |= ((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int;
            (*rp).pat.translate = 0 as *mut libc::c_uchar;
        } else {
            syn
                &= !(((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int);
            (*rp).pat.translate = casetable.as_mut_ptr() as *mut libc::c_uchar;
        }
    } else {
        (*rp).pat.translate = 0 as *mut libc::c_uchar;
        syn
            &= !(((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int);
    }
    if first {
        first = 0 as libc::c_int != 0;
        dfaregs[0 as libc::c_int as usize] = dfaalloc();
        dfaregs[1 as libc::c_int as usize] = dfaalloc();
        dfasyntax(
            dfaregs[0 as libc::c_int as usize],
            &mut localeinfo,
            syn,
            DFA_ANCHOR as libc::c_int,
        );
        dfasyntax(
            dfaregs[1 as libc::c_int as usize],
            &mut localeinfo,
            syn
                | ((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int,
            DFA_ANCHOR as libc::c_int,
        );
    }
    re_set_syntax(syn);
    rerr = re_compile_pattern(buf, len, &mut (*rp).pat);
    if !rerr.is_null() {
        refree(rp);
        if !canfatal {
            error(
                b"%s: /%.*s/\0" as *const u8 as *const libc::c_char,
                rerr,
                len as libc::c_int,
                s,
            );
            return 0 as *mut Regexp;
        }
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(b"re.c\0" as *const u8 as *const libc::c_char, 264 as libc::c_int);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            b"invalid regexp: %s: /%.*s/\0" as *const u8 as *const libc::c_char,
            rerr,
            len as libc::c_int,
            s,
        );
    }
    ((*rp).pat).set_newline_anchor(0 as libc::c_int as libc::c_uint);
    if dfa as libc::c_int != 0 && !no_dfa {
        (*rp).dfareg = dfaalloc();
        dfacopysyntax((*rp).dfareg, dfaregs[ignorecase as usize]);
        dfacomp(buf, len as idx_t, (*rp).dfareg, 1 as libc::c_int != 0);
    } else {
        (*rp).dfareg = 0 as *mut dfa;
    }
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) < len {
        if !(strchr(metas.as_mut_ptr(), *buf.offset(i as isize) as libc::c_int))
            .is_null()
        {
            (*rp).has_meta = 1 as libc::c_int != 0;
            break;
        } else {
            i += 1;
            i;
        }
    }
    i = len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    while i >= 0 as libc::c_int {
        if !(strchr(
            b"*+|?{}\0" as *const u8 as *const libc::c_char,
            *buf.offset(i as isize) as libc::c_int,
        ))
            .is_null()
        {
            (*rp).maybe_long = 1 as libc::c_int != 0;
            break;
        } else {
            i -= 1;
            i;
        }
    }
    return rp;
}
pub unsafe extern "C" fn research(
    mut rp: *mut Regexp,
    mut str: *mut libc::c_char,
    mut start: libc::c_int,
    mut len: size_t,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut ret: *const libc::c_char = str;
    let mut try_backref: bool = 0 as libc::c_int != 0;
    let mut need_start: libc::c_int = 0;
    let mut no_bol: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    need_start = (flags & 1 as libc::c_int != 0 as libc::c_int) as libc::c_int;
    no_bol = (flags & 2 as libc::c_int != 0 as libc::c_int) as libc::c_int;
    if no_bol != 0 {
        ((*rp).pat).set_not_bol(1 as libc::c_int as libc::c_uint);
    }
    if !((*rp).dfareg).is_null() && no_bol == 0 && need_start == 0 {
        let mut superset: *mut dfa = dfasuperset((*rp).dfareg);
        if !superset.is_null() {
            ret = dfaexec(
                superset,
                str.offset(start as isize),
                str.offset(start as isize).offset(len as isize),
                1 as libc::c_int != 0,
                0 as *mut idx_t,
                0 as *mut bool,
            );
        }
        if !ret.is_null()
            && (need_start == 0
                || superset.is_null() && dfaisfast((*rp).dfareg) as libc::c_int != 0)
        {
            ret = dfaexec(
                (*rp).dfareg,
                str.offset(start as isize),
                str.offset(start as isize).offset(len as isize),
                1 as libc::c_int != 0,
                0 as *mut idx_t,
                &mut try_backref,
            );
        }
    }
    if !ret.is_null() {
        if ((*rp).dfareg).is_null() || start != 0 as libc::c_int || no_bol != 0
            || need_start != 0 || try_backref as libc::c_int != 0
        {
            res = re_search(
                &mut (*rp).pat,
                str,
                (start as libc::c_ulong).wrapping_add(len) as regoff_t,
                start,
                len as regoff_t,
                if need_start != 0 { &mut (*rp).regs } else { 0 as *mut re_registers },
            );
        } else {
            res = 1 as libc::c_int;
        }
    } else {
        res = -(1 as libc::c_int);
    }
    ((*rp).pat).set_not_bol(0 as libc::c_int as libc::c_uint);
    return res;
}
pub unsafe extern "C" fn refree(mut rp: *mut Regexp) {
    if rp.is_null() {
        return;
    }
    (*rp).pat.translate = 0 as *mut libc::c_uchar;
    regfree(&mut (*rp).pat);
    if !((*rp).regs.start).is_null() {
        free((*rp).regs.start as *mut libc::c_void);
    }
    if !((*rp).regs.end).is_null() {
        free((*rp).regs.end as *mut libc::c_void);
    }
    if !((*rp).dfareg).is_null() {
        dfafree((*rp).dfareg);
        free((*rp).dfareg as *mut libc::c_void);
    }
    free(rp as *mut libc::c_void);
}
pub unsafe extern "C" fn dfaerror(mut s: *const libc::c_char) -> ! {
    (set_loc
        as unsafe extern "C" fn(
            *const libc::c_char,
            libc::c_int,
        ) -> ())(b"re.c\0" as *const u8 as *const libc::c_char, 390 as libc::c_int);
    (Some(
        (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ())).unwrap(),
    ))
        .unwrap()(b"%s\0" as *const u8 as *const libc::c_char, s);
    exit(2 as libc::c_int);
}
#[inline]
unsafe extern "C" fn re_cache_get(mut t: *mut NODE) -> *mut Regexp {
    if ((*t).sub.nodep.r.preg[IGNORECASE as usize]).is_null() {
        (*t)
            .sub
            .nodep
            .r
            .preg[IGNORECASE
            as usize] = make_regexp(
            (*(*t).sub.nodep.x.extra).sub.val.sp,
            (*(*t).sub.nodep.x.extra).sub.val.slen,
            IGNORECASE,
            (*t).flags as u64 != 0,
            1 as libc::c_int != 0,
        );
    }
    return (*t).sub.nodep.r.preg[IGNORECASE as usize];
}
pub unsafe extern "C" fn re_update(mut t: *mut NODE) -> *mut Regexp {
    let mut t1: *mut NODE = 0 as *mut NODE;
    if (*t).type_0 as libc::c_uint == Node_val as libc::c_int as libc::c_uint
        && (*t).flags as libc::c_uint & REGEX as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
    {
        return re_cache_get((*t).sub.val.typre);
    }
    if (*t).sub.nodep.reflags as libc::c_uint & CONSTANT as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        return re_cache_get(t);
    }
    t1 = (*t).sub.nodep.x.extra;
    if !((*t).sub.nodep.l.lptr).is_null() {
        if cmp_nodes((*t).sub.nodep.l.lptr, t1, 1 as libc::c_int != 0)
            == 0 as libc::c_int
        {
            return re_cache_get(t);
        }
        unref((*t).sub.nodep.l.lptr);
    }
    (*t).sub.nodep.l.lptr = dupnode(t1);
    if !((*t).sub.nodep.r.preg[0 as libc::c_int as usize]).is_null() {
        refree((*t).sub.nodep.r.preg[0 as libc::c_int as usize]);
        (*t).sub.nodep.r.preg[0 as libc::c_int as usize] = 0 as *mut Regexp;
    }
    if !((*t).sub.nodep.r.preg[1 as libc::c_int as usize]).is_null() {
        refree((*t).sub.nodep.r.preg[1 as libc::c_int as usize]);
        (*t).sub.nodep.r.preg[1 as libc::c_int as usize] = 0 as *mut Regexp;
    }
    if (*t).flags as libc::c_uint > 0 as libc::c_int as libc::c_uint
        && {
            (*t).flags += 1;
            (*t).flags as libc::c_uint > 10 as libc::c_int as libc::c_uint
        }
    {
        (*t).flags = 0 as flagvals;
    }
    if ((*t).sub.nodep.l.lptr).is_null() {
        t1 = (*t).sub.nodep.x.extra;
        unref((*t).sub.nodep.l.lptr);
        (*t).sub.nodep.l.lptr = dupnode(t1);
    }
    return re_cache_get(t);
}
pub unsafe extern "C" fn resetup() {
    init_localeinfo(&mut localeinfo);
    if do_flags as libc::c_uint & DO_POSIX as libc::c_int as libc::c_uint != 0 {
        syn = ((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
            << 1 as libc::c_int
            | ((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int
            | (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
            | (((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int
            | ((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
            | (((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int
            | ((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
            | ((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int
            | (((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
            | (((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int
            | (((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int
            | (((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int | 1 as libc::c_int as libc::c_ulong
            | (((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int
            | (((((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
            | (((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int;
    } else if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint
        != 0
    {
        syn = 1 as libc::c_int as libc::c_ulong
            | (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
            | (((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
            | ((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int
            | (((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int
            | ((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
            | ((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int
            | (((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int
            | ((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int
            | (((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int
            | (((((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int;
    } else {
        syn = (((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
            << 1 as libc::c_int
            | ((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int
            | (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
            | (((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int
            | ((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
            | (((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int
            | ((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
            | ((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int
            | (((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
            | (((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int
            | (((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int
            | (((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int | 1 as libc::c_int as libc::c_ulong
            | (((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            & !((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | ((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | (((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int);
    }
    if do_flags as libc::c_uint & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0 {
        syn
            |= (((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | ((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int;
    }
    re_set_syntax(syn);
}
pub unsafe extern "C" fn using_utf8() -> bool {
    return localeinfo.using_utf8;
}
pub unsafe extern "C" fn reisstring(
    mut text: *const libc::c_char,
    mut len: size_t,
    mut re: *mut Regexp,
    mut buf: *const libc::c_char,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut matched: *const libc::c_char = 0 as *const libc::c_char;
    if (*re).has_meta {
        return 0 as libc::c_int;
    }
    matched = &*buf
        .offset(*((*re).regs.start).offset(0 as libc::c_int as isize) as isize)
        as *const libc::c_char;
    res = (memcmp(text as *const libc::c_void, matched as *const libc::c_void, len)
        == 0 as libc::c_int) as libc::c_int;
    return res;
}
pub unsafe extern "C" fn reflags2str(mut flagval: libc::c_int) -> *const libc::c_char {
    static mut values: [flagtab; 26] = [
        {
            let mut init = flagtab {
                val: 1 as libc::c_int as libc::c_ulong as libc::c_int,
                name: b"RE_BACKSLASH_ESCAPE_IN_LISTS\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: ((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    as libc::c_int,
                name: b"RE_BK_PLUS_QM\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: (((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) as libc::c_int,
                name: b"RE_CHAR_CLASSES\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: ((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) as libc::c_int,
                name: b"RE_CONTEXT_INDEP_ANCHORS\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: (((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    as libc::c_int,
                name: b"RE_CONTEXT_INDEP_OPS\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: ((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) as libc::c_int,
                name: b"RE_CONTEXT_INVALID_OPS\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) as libc::c_int,
                name: b"RE_DOT_NEWLINE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: ((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    as libc::c_int,
                name: b"RE_DOT_NOT_NULL\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: (((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) as libc::c_int,
                name: b"RE_HAT_LISTS_NOT_NEWLINE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: ((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) as libc::c_int,
                name: b"RE_INTERVALS\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: (((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    as libc::c_int,
                name: b"RE_LIMITED_OPS\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: ((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) as libc::c_int,
                name: b"RE_NEWLINE_ALT\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: (((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) as libc::c_int,
                name: b"RE_NO_BK_BRACES\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: ((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) as libc::c_int,
                name: b"RE_NO_BK_PARENS\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: (((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) as libc::c_int,
                name: b"RE_NO_BK_REFS\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: ((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    as libc::c_int,
                name: b"RE_NO_BK_VBAR\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: (((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) as libc::c_int,
                name: b"RE_NO_EMPTY_RANGES\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: ((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) as libc::c_int,
                name: b"RE_UNMATCHED_RIGHT_PAREN_ORD\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: (((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    as libc::c_int,
                name: b"RE_NO_POSIX_BACKTRACKING\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: ((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) as libc::c_int,
                name: b"RE_NO_GNU_OPS\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: ((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    as libc::c_int,
                name: b"RE_INVALID_INTERVAL_ORD\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: (((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) as libc::c_int,
                name: b"RE_ICASE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: ((((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) as libc::c_int,
                name: b"RE_CARET_ANCHORS_HERE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: (((((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    as libc::c_int,
                name: b"RE_CONTEXT_INVALID_DUP\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagtab {
                val: ((((((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) as libc::c_int,
                name: b"RE_NO_SUB\0" as *const u8 as *const libc::c_char,
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
    if flagval == 0 as libc::c_int {
        return b"RE_SYNTAX_EMACS\0" as *const u8 as *const libc::c_char;
    }
    return genflags2str(flagval, values.as_ptr());
}
pub unsafe extern "C" fn dfawarn(mut dfa_warning: *const libc::c_char) {}
unsafe extern "C" fn check_bracket_exp(mut s: *mut libc::c_char, mut length: size_t) {
    static mut classes: [reclass; 13] = [
        {
            let mut init = reclass {
                name: b"[:alpha:]\0" as *const u8 as *const libc::c_char,
                len: 9 as libc::c_int as size_t,
                warned: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = reclass {
                name: b"[:digit:]\0" as *const u8 as *const libc::c_char,
                len: 9 as libc::c_int as size_t,
                warned: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = reclass {
                name: b"[:alnum:]\0" as *const u8 as *const libc::c_char,
                len: 9 as libc::c_int as size_t,
                warned: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = reclass {
                name: b"[:upper:]\0" as *const u8 as *const libc::c_char,
                len: 9 as libc::c_int as size_t,
                warned: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = reclass {
                name: b"[:lower:]\0" as *const u8 as *const libc::c_char,
                len: 9 as libc::c_int as size_t,
                warned: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = reclass {
                name: b"[:space:]\0" as *const u8 as *const libc::c_char,
                len: 9 as libc::c_int as size_t,
                warned: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = reclass {
                name: b"[:xdigit:]\0" as *const u8 as *const libc::c_char,
                len: 10 as libc::c_int as size_t,
                warned: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = reclass {
                name: b"[:punct:]\0" as *const u8 as *const libc::c_char,
                len: 9 as libc::c_int as size_t,
                warned: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = reclass {
                name: b"[:print:]\0" as *const u8 as *const libc::c_char,
                len: 9 as libc::c_int as size_t,
                warned: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = reclass {
                name: b"[:graph:]\0" as *const u8 as *const libc::c_char,
                len: 9 as libc::c_int as size_t,
                warned: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = reclass {
                name: b"[:cntrl:]\0" as *const u8 as *const libc::c_char,
                len: 9 as libc::c_int as size_t,
                warned: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = reclass {
                name: b"[:blank:]\0" as *const u8 as *const libc::c_char,
                len: 9 as libc::c_int as size_t,
                warned: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = reclass {
                name: 0 as *const libc::c_char,
                len: 0 as libc::c_int as size_t,
                warned: false,
            };
            init
        },
    ];
    let mut i: libc::c_int = 0;
    let mut found: bool = 0 as libc::c_int != 0;
    let mut save: libc::c_char = 0;
    let mut sp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sp2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    if length == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    end = s.offset(length as isize);
    save = *s.offset(length as isize);
    *s.offset(length as isize) = '\0' as i32 as libc::c_char;
    sp = s;
    loop {
        sp2 = memchr(
            sp as *const libc::c_void,
            '[' as i32,
            end.offset_from(sp) as libc::c_long as libc::c_ulong,
        ) as *mut libc::c_char;
        sp = sp2;
        if sp.is_null() {
            break;
        }
        count += 1;
        count;
        sp = sp.offset(1);
        sp;
        while *sp as libc::c_int != '\0' as i32 {
            if *sp as libc::c_int == '[' as i32 {
                count += 1;
                count;
            }
            if *sp as libc::c_int == ']' as i32 && sp > sp2 {
                if !(*sp.offset(-(1 as libc::c_int) as isize) as libc::c_int
                    != '[' as i32
                    && *sp.offset(-(1 as libc::c_int) as isize) as libc::c_int
                        != '\\' as i32)
                {
                    if !(sp.offset_from(sp2) as libc::c_long
                        >= 2 as libc::c_int as libc::c_long
                        && *sp.offset(-(1 as libc::c_int) as isize) as libc::c_int
                            == '^' as i32
                        && *sp.offset(-(2 as libc::c_int) as isize) as libc::c_int
                            == '[' as i32)
                    {
                        count -= 1;
                        count;
                    }
                }
            }
            if count == 0 as libc::c_int {
                sp = sp.offset(1);
                sp;
                break;
            } else {
                sp = sp.offset(1);
                sp;
            }
        }
        if count > 0 as libc::c_int {
            break;
        }
        i = 0 as libc::c_int;
        while !(classes[i as usize].name).is_null() {
            if !classes[i as usize].warned {
                len = classes[i as usize].len as libc::c_int;
                if len as libc::c_long == sp.offset_from(sp2) as libc::c_long
                    && memcmp(
                        sp2 as *const libc::c_void,
                        classes[i as usize].name as *const libc::c_void,
                        len as libc::c_ulong,
                    ) == 0 as libc::c_int
                {
                    found = 1 as libc::c_int != 0;
                    break;
                }
            }
            i += 1;
            i;
        }
        if found as libc::c_int != 0 && !classes[i as usize].warned {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"re.c\0" as *const u8 as *const libc::c_char,
                669 as libc::c_int,
            );
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .unwrap(),
            ))
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"regexp component `%.*s' should probably be `[%.*s]'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                len,
                sp2,
                len,
                sp2,
            );
            classes[i as usize].warned = 1 as libc::c_int != 0;
        }
        if !(sp < end) {
            break;
        }
        found = 0 as libc::c_int != 0;
    }
    *s.offset(length as isize) = save;
}
