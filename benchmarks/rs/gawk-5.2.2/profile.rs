use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type re_dfa_t;
    pub type dfa;
    pub type break_point;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
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
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn time(__timer: *mut time_t) -> time_t;
    fn ctime(__timer: *const time_t) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn close(__fd: libc::c_int) -> libc::c_int;
    static ruletab: [*const libc::c_char; 0];
    static mut Nnull_string: *mut NODE;
    static mut sourceline: libc::c_int;
    static mut source: *mut libc::c_char;
    static mut nextfree: [block_header; 2];
    static mut srcfiles: *mut SRCFILE;
    fn dump_funcs();
    fn getfname(
        _: Option::<unsafe extern "C" fn(libc::c_int) -> *mut NODE>,
        prepend_awk: bool,
    ) -> *const libc::c_char;
    static mut do_flags: do_flag_values;
    static awk_namespace: [libc::c_char; 0];
    static mut current_namespace: *const libc::c_char;
    static mut namespace_changed: bool;
    fn opcode2str(type_0: OPCODE) -> *const libc::c_char;
    fn devopen_simple(
        name: *const libc::c_char,
        mode: *const libc::c_char,
        try_real_open: bool,
    ) -> libc::c_int;
    fn nodetype2str(type_0: NODETYPE) -> *const libc::c_char;
    fn op2str(type_0: OPCODE) -> *const libc::c_char;
    fn dump_fcall_stack(fp: *mut FILE);
    fn more_blocks(id: libc::c_int) -> *mut libc::c_void;
    fn final_exit(status: libc::c_int) -> !;
    fn estrdup(str: *const libc::c_char, len: size_t) -> *mut libc::c_char;
    fn r_fatal(mesg: *const libc::c_char, _: ...);
    fn set_loc(file: *const libc::c_char, line: libc::c_int);
    fn r_warning(mesg: *const libc::c_char, _: ...);
    fn is_all_upper(name: *const libc::c_char) -> bool;
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
pub type time_t = __time_t;
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
pub type Func_print = Option::<
    unsafe extern "C" fn(*mut FILE, *const libc::c_char, ...) -> libc::c_int,
>;
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
static mut pp_stack: *mut NODE = 0 as *const NODE as *mut NODE;
static mut func_params: *mut NODE = 0 as *const NODE as *mut NODE;
static mut prof_fp: *mut FILE = 0 as *const FILE as *mut FILE;
static mut indent_level: libc::c_long = 0 as libc::c_int as libc::c_long;
static mut tabs: [libc::c_char; 28] = unsafe {
    *::std::mem::transmute::<
        &[u8; 28],
        &[libc::c_char; 28],
    >(b"\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\0")
};
static mut tabs_len: size_t = 0;
pub unsafe extern "C" fn set_prof_file(mut file: *const libc::c_char) {
    let mut fd: libc::c_int = 0;
    fd = devopen_simple(
        file,
        b"w\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int != 0,
    );
    if fd == -(1 as libc::c_int) {
        prof_fp = 0 as *mut FILE;
    } else if fd == fileno(stdout) {
        prof_fp = stdout;
    } else if fd == fileno(stderr) {
        prof_fp = stderr;
    } else {
        prof_fp = fdopen(fd, b"w\0" as *const u8 as *const libc::c_char);
    }
    if prof_fp.is_null() {
        let mut e: libc::c_int = *__errno_location();
        if fd != -(1 as libc::c_int) && fd != fileno(stdout) && fd != fileno(stderr) {
            close(fd);
        }
        *__errno_location() = e;
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"profile.c\0" as *const u8 as *const libc::c_char,
            110 as libc::c_int,
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
            file,
            strerror(*__errno_location()),
        );
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"profile.c\0" as *const u8 as *const libc::c_char,
            112 as libc::c_int,
        );
        (Some(
            (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"sending profile to standard error\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        prof_fp = stderr;
    }
}
pub unsafe extern "C" fn init_profiling_signals() {
    signal(
        1 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(libc::c_int) -> !>,
            __sighandler_t,
        >(Some(dump_and_exit as unsafe extern "C" fn(libc::c_int) -> !)),
    );
    signal(
        10 as libc::c_int,
        Some(just_dump as unsafe extern "C" fn(libc::c_int) -> ()),
    );
}
unsafe extern "C" fn indent(mut count: exec_count_t) {
    let mut i: libc::c_int = 0;
    if do_flags as libc::c_uint & DO_PROFILE as libc::c_int as libc::c_uint != 0 {
        if count == 0 as libc::c_int as libc::c_ulonglong {
            fprintf(prof_fp, b"\t\0" as *const u8 as *const libc::c_char);
        } else {
            fprintf(prof_fp, b"%6llu  \0" as *const u8 as *const libc::c_char, count);
        }
    }
    i = 0 as libc::c_int;
    while (i as libc::c_long) < indent_level {
        fprintf(prof_fp, b"\t\0" as *const u8 as *const libc::c_char);
        i += 1;
        i;
    }
}
unsafe extern "C" fn indent_in() {
    indent_level += 1;
    indent_level;
}
unsafe extern "C" fn indent_out() {
    indent_level -= 1;
    indent_level;
}
unsafe extern "C" fn pp_push(
    mut type_0: libc::c_int,
    mut s: *mut libc::c_char,
    mut flag: libc::c_int,
    mut comment: *mut INSTRUCTION,
) {
    let mut n: *mut NODE = 0 as *mut NODE;
    n = nextfree[BLOCK_NODE as libc::c_int as usize].freep as *mut NODE;
    if !n.is_null() {
        nextfree[BLOCK_NODE as libc::c_int as usize]
            .freep = (*(n as *mut block_item)).freep;
    } else {
        n = more_blocks(BLOCK_NODE as libc::c_int) as *mut NODE;
    };
    (*n).sub.nodep.name = s;
    (*n).sub.nodep.reserved = strlen(s);
    (*n).flags = flag as flagvals;
    (*n).type_0 = type_0 as NODETYPE;
    (*n).sub.nodep.r.rptr = pp_stack;
    (*n).sub.nodep.x.cmnt = comment as *mut libc::c_void;
    pp_stack = n;
}
unsafe extern "C" fn pp_pop() -> *mut NODE {
    let mut n: *mut NODE = 0 as *mut NODE;
    n = pp_stack;
    pp_stack = (*n).sub.nodep.r.rptr;
    return n;
}
unsafe extern "C" fn pp_free(mut n: *mut NODE) {
    if (*n).flags as libc::c_uint & 2 as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        free((*n).sub.nodep.name as *mut libc::c_void);
    }
    let ref mut fresh0 = (*(n as *mut block_item)).freep;
    *fresh0 = nextfree[BLOCK_NODE as libc::c_int as usize].freep;
    nextfree[BLOCK_NODE as libc::c_int as usize].freep = n as *mut block_item;
}
unsafe extern "C" fn pprint(
    mut startp: *mut INSTRUCTION,
    mut endp: *mut INSTRUCTION,
    mut flags: libc::c_int,
) {
    let mut current_block: u64;
    let mut pc: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut t1: *mut NODE = 0 as *mut NODE;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t2: *mut NODE = 0 as *mut NODE;
    let mut ip1: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut ip2: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut m: *mut NODE = 0 as *mut NODE;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rule: libc::c_int = 0;
    static mut rule_count: [libc::c_int; 6] = [0; 6];
    static mut skip_comment: bool = 0 as libc::c_int != 0;
    pc = startp;
    while pc != endp {
        if (*pc).source_line as libc::c_int > 0 as libc::c_int {
            sourceline = (*pc).source_line as libc::c_int;
        }
        if (*pc).opcode as libc::c_uint == Op_comment as libc::c_int as libc::c_uint
            && (*(*pc).d.dn).sub.val.comtype as libc::c_uint
                == EOL_COMMENT as libc::c_int as libc::c_uint
            && skip_comment as libc::c_int != 0
        {
            skip_comment = 0 as libc::c_int != 0;
        } else {
            skip_comment = 0 as libc::c_int != 0;
            match (*pc).opcode as libc::c_uint {
                51 => {
                    source = (*pc).d.name;
                    rule = (*pc).x.xl as libc::c_int;
                    pp_namespace_list((*pc.offset(3 as libc::c_int as isize)).nexti);
                    if rule != Rule as libc::c_int {
                        if (*pc).nexti != (*pc.offset(1 as libc::c_int as isize)).x.xi
                            && (*(*pc).nexti).opcode as libc::c_uint
                                == Op_comment as libc::c_int as libc::c_uint
                            && (*(*(*pc).nexti).d.dn).sub.val.comtype as libc::c_uint
                                == BLOCK_COMMENT as libc::c_int as libc::c_uint
                        {
                            print_comment(
                                (*pc).nexti,
                                -(1 as libc::c_int) as libc::c_long,
                            );
                        }
                        ip1 = (*pc.offset(1 as libc::c_int as isize)).x.xi;
                        ip2 = (*pc.offset(1 as libc::c_int as isize)).d.di;
                        if do_flags as libc::c_uint
                            & DO_PROFILE as libc::c_int as libc::c_uint != 0
                        {
                            let fresh1 = rule_count[rule as usize];
                            rule_count[rule as usize] = rule_count[rule as usize] + 1;
                            if fresh1 == 0 {
                                fprintf(
                                    prof_fp,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"\t# %s rule(s)\n\n\0" as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    *ruletab.as_ptr().offset(rule as isize),
                                );
                            }
                            indent(0 as libc::c_int as exec_count_t);
                        }
                        fprintf(
                            prof_fp,
                            b"%s {\0" as *const u8 as *const libc::c_char,
                            *ruletab.as_ptr().offset(rule as isize),
                        );
                        end_line(pc);
                        skip_comment = 1 as libc::c_int != 0;
                    } else {
                        if do_flags as libc::c_uint
                            & DO_PROFILE as libc::c_int as libc::c_uint != 0
                            && {
                                let fresh2 = rule_count[rule as usize];
                                rule_count[rule as usize] = rule_count[rule as usize] + 1;
                                fresh2 == 0
                            }
                        {
                            fprintf(
                                prof_fp,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"\t# Rule(s)\n\n\0" as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                        }
                        ip1 = (*pc).nexti;
                        indent((*ip1).d.ldl);
                        if ip1 != (*pc.offset(1 as libc::c_int as isize)).x.xi {
                            pprint(
                                (*ip1).nexti,
                                (*pc.offset(1 as libc::c_int as isize)).x.xi,
                                0 as libc::c_int,
                            );
                            if (*(*(*ip1).nexti).nexti).nexti
                                != (*pc.offset(1 as libc::c_int as isize)).x.xi
                                || (*(*ip1).nexti).opcode as libc::c_uint
                                    != Op_comment as libc::c_int as libc::c_uint
                            {
                                t1 = pp_pop();
                                fprintf(
                                    prof_fp,
                                    b"%s {\0" as *const u8 as *const libc::c_char,
                                    (*t1).sub.nodep.name,
                                );
                                pp_free(t1);
                            } else {
                                fprintf(
                                    prof_fp,
                                    b"{\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            ip1 = (*pc.offset(1 as libc::c_int as isize)).x.xi;
                            ip2 = (*pc.offset(1 as libc::c_int as isize)).d.di;
                            if do_flags as libc::c_uint
                                & DO_PROFILE as libc::c_int as libc::c_uint != 0
                                && (*ip1).d.ldl > 0 as libc::c_int as libc::c_ulonglong
                            {
                                fprintf(
                                    prof_fp,
                                    b" # %llu\0" as *const u8 as *const libc::c_char,
                                    (*ip1).d.ldl,
                                );
                            }
                            end_line(ip1);
                            skip_comment = 1 as libc::c_int != 0;
                        } else {
                            fprintf(
                                prof_fp,
                                b"{\n\0" as *const u8 as *const libc::c_char,
                            );
                            ip1 = (*pc.offset(1 as libc::c_int as isize)).x.xi;
                            ip2 = (*pc.offset(1 as libc::c_int as isize)).d.di;
                        }
                        ip1 = (*ip1).nexti;
                    }
                    indent_in();
                    pprint(ip1, ip2, 0 as libc::c_int);
                    indent_out();
                    if do_flags as libc::c_uint
                        & DO_PROFILE as libc::c_int as libc::c_uint != 0
                    {
                        indent(0 as libc::c_int as exec_count_t);
                    }
                    fprintf(prof_fp, b"}\n\n\0" as *const u8 as *const libc::c_char);
                    pc = (*pc.offset(1 as libc::c_int as isize)).d.di;
                    current_block = 4014953649076014837;
                }
                108 => {
                    memset(
                        rule_count.as_mut_ptr() as *mut libc::c_void,
                        0 as libc::c_int,
                        (MAXRULE as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    );
                    current_block = 4014953649076014837;
                }
                78 => {
                    m = (*pc).d.dn;
                    if m == Nnull_string {
                        pp_push(
                            (*pc).opcode as libc::c_int,
                            (*m).sub.val.sp,
                            1 as libc::c_int,
                            (*pc).comment,
                        );
                    } else if (*m).flags as libc::c_uint
                        & NUMBER as libc::c_int as libc::c_uint
                        != 0 as libc::c_int as libc::c_uint
                    {
                        pp_push(
                            (*pc).opcode as libc::c_int,
                            pp_number(m),
                            2 as libc::c_int,
                            (*pc).comment,
                        );
                    } else {
                        str = pp_string((*m).sub.val.sp, (*m).sub.val.slen, '"' as i32);
                        if (*m).flags as libc::c_uint
                            & INTLSTR as libc::c_int as libc::c_uint
                            != 0 as libc::c_int as libc::c_uint
                        {
                            let mut tmp_0: *mut libc::c_char = str;
                            str = pp_group3(
                                b"_\0" as *const u8 as *const libc::c_char,
                                tmp_0,
                                b"\0" as *const u8 as *const libc::c_char,
                            );
                            free(tmp_0 as *mut libc::c_void);
                        }
                        pp_push(
                            (*pc).opcode as libc::c_int,
                            str,
                            2 as libc::c_int,
                            (*pc).comment,
                        );
                    }
                    current_block = 4014953649076014837;
                }
                27 => {
                    if !((*pc).x.xn).is_null() {
                        pp_push(
                            Op_push_i as libc::c_int,
                            pp_node((*pc).x.xn),
                            2 as libc::c_int,
                            (*pc).comment,
                        );
                    }
                    current_block = 7152033355135659062;
                }
                28 | 37 | 82 | 81 | 80 | 75 | 76 | 77 => {
                    current_block = 7152033355135659062;
                }
                17 | 83 | 16 => {
                    tmp = pp_list(
                        (*pc).d.dl as libc::c_int,
                        op2str((*pc).opcode),
                        b", \0" as *const u8 as *const libc::c_char,
                    );
                    t1 = pp_pop();
                    str = pp_group3(
                        (*t1).sub.nodep.name,
                        tmp,
                        b"\0" as *const u8 as *const libc::c_char,
                    );
                    free(tmp as *mut libc::c_void);
                    pp_free(t1);
                    pp_push(
                        (*pc).opcode as libc::c_int,
                        str,
                        2 as libc::c_int,
                        (*pc).comment,
                    );
                    current_block = 4014953649076014837;
                }
                38 | 40 => {
                    pprint((*pc).nexti, (*pc).d.di, flags);
                    t2 = pp_pop();
                    t1 = pp_pop();
                    parenthesize((*pc).opcode as libc::c_int, t1, t2);
                    if ((*pc).comment).is_null() {
                        str = pp_group3(
                            (*t1).sub.nodep.name,
                            op2str((*pc).opcode),
                            (*t2).sub.nodep.name,
                        );
                    } else {
                        if (indent_level + 1 as libc::c_int as libc::c_long)
                            as libc::c_ulong > tabs_len
                        {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    libc::c_int,
                                ) -> ())(
                                b"profile.c\0" as *const u8 as *const libc::c_char,
                                428 as libc::c_int,
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
                                    b"Program indentation level too deep. Consider refactoring your code\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                        }
                        let mut len: size_t = (strlen((*t1).sub.nodep.name))
                            .wrapping_add(strlen(op2str((*pc).opcode)))
                            .wrapping_add(strlen((*t2).sub.nodep.name))
                            .wrapping_add(indent_level as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_add((*(*(*pc).comment).d.dn).sub.val.slen)
                            .wrapping_add(3 as libc::c_int as libc::c_ulong);
                        str = emalloc_real(
                            len,
                            b"pprint\0" as *const u8 as *const libc::c_char,
                            b"str\0" as *const u8 as *const libc::c_char,
                            b"profile.c\0" as *const u8 as *const libc::c_char,
                            435 as libc::c_int,
                        ) as *mut libc::c_char;
                        sprintf(
                            str,
                            b"%s%s%s%.*s %s\0" as *const u8 as *const libc::c_char,
                            (*t1).sub.nodep.name,
                            op2str((*pc).opcode),
                            (*(*(*pc).comment).d.dn).sub.val.sp,
                            (indent_level + 1 as libc::c_int as libc::c_long)
                                as libc::c_int,
                            tabs.as_ptr(),
                            (*t2).sub.nodep.name,
                        );
                    }
                    pp_free(t1);
                    pp_free(t2);
                    pp_push(
                        (*pc).opcode as libc::c_int,
                        str,
                        2 as libc::c_int,
                        (*pc).comment,
                    );
                    pc = (*pc).d.di;
                    current_block = 4014953649076014837;
                }
                8 | 10 | 2 | 12 | 4 | 6 => {
                    m = (*pc).d.dn;
                    t1 = pp_pop();
                    if prec_level((*pc).opcode as libc::c_int)
                        > prec_level((*t1).type_0 as libc::c_int)
                        && is_binary((*t1).type_0 as libc::c_int) as libc::c_int != 0
                    {
                        pp_parenthesize(t1);
                    }
                    if (*m).flags as libc::c_uint & NUMBER as libc::c_int as libc::c_uint
                        != 0 as libc::c_int as libc::c_uint
                    {
                        tmp = pp_number(m);
                    } else {
                        tmp = pp_string((*m).sub.val.sp, (*m).sub.val.slen, '"' as i32);
                    }
                    str = pp_group3((*t1).sub.nodep.name, op2str((*pc).opcode), tmp);
                    free(tmp as *mut libc::c_void);
                    pp_free(t1);
                    pp_push(
                        (*pc).opcode as libc::c_int,
                        str,
                        2 as libc::c_int,
                        (*pc).comment,
                    );
                    current_block = 4014953649076014837;
                }
                121 => {
                    t1 = pp_pop();
                    str = pp_group3(
                        b"(\0" as *const u8 as *const libc::c_char,
                        (*t1).sub.nodep.name,
                        b")\0" as *const u8 as *const libc::c_char,
                    );
                    pp_free(t1);
                    pp_push(
                        (*pc).opcode as libc::c_int,
                        str,
                        2 as libc::c_int,
                        (*pc).comment,
                    );
                    current_block = 4014953649076014837;
                }
                7 | 9 | 1 | 11 | 3 | 5 | 42 | 43 | 44 | 45 | 46 | 47 => {
                    t2 = pp_pop();
                    t1 = pp_pop();
                    parenthesize((*pc).opcode as libc::c_int, t1, t2);
                    str = pp_group3(
                        (*t1).sub.nodep.name,
                        op2str((*pc).opcode),
                        (*t2).sub.nodep.name,
                    );
                    pp_free(t1);
                    pp_free(t2);
                    pp_push(
                        (*pc).opcode as libc::c_int,
                        str,
                        2 as libc::c_int,
                        (*pc).comment,
                    );
                    current_block = 4014953649076014837;
                }
                18 | 19 | 20 | 21 => {
                    t1 = pp_pop();
                    if (*pc).opcode as libc::c_uint
                        == Op_preincrement as libc::c_int as libc::c_uint
                        || (*pc).opcode as libc::c_uint
                            == Op_predecrement as libc::c_int as libc::c_uint
                    {
                        str = pp_group3(
                            op2str((*pc).opcode),
                            (*t1).sub.nodep.name,
                            b"\0" as *const u8 as *const libc::c_char,
                        );
                    } else {
                        str = pp_group3(
                            (*t1).sub.nodep.name,
                            op2str((*pc).opcode),
                            b"\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    pp_free(t1);
                    pp_push(
                        (*pc).opcode as libc::c_int,
                        str,
                        2 as libc::c_int,
                        (*pc).comment,
                    );
                    current_block = 4014953649076014837;
                }
                24 | 84 | 22 | 23 | 25 => {
                    t1 = pp_pop();
                    if is_binary((*t1).type_0 as libc::c_int) as libc::c_int != 0
                        || (*t1).type_0 as OPCODE as libc::c_uint
                            == (*pc).opcode as libc::c_uint
                            && ((*pc).opcode as libc::c_uint
                                == Op_unary_minus as libc::c_int as libc::c_uint
                                || (*pc).opcode as libc::c_uint
                                    == Op_unary_plus as libc::c_int as libc::c_uint)
                    {
                        pp_parenthesize(t1);
                    }
                    str = pp_group3(
                        op2str((*pc).opcode),
                        (*t1).sub.nodep.name,
                        b"\0" as *const u8 as *const libc::c_char,
                    );
                    pp_free(t1);
                    pp_push(
                        (*pc).opcode as libc::c_int,
                        str,
                        2 as libc::c_int,
                        (*pc).comment,
                    );
                    current_block = 4014953649076014837;
                }
                26 | 34 | 35 | 31 | 32 | 33 | 36 => {
                    t2 = pp_pop();
                    t1 = pp_pop();
                    str = pp_group3(
                        (*t2).sub.nodep.name,
                        op2str((*pc).opcode),
                        (*t1).sub.nodep.name,
                    );
                    pp_free(t2);
                    pp_free(t1);
                    pp_push(
                        (*pc).opcode as libc::c_int,
                        str,
                        2 as libc::c_int,
                        (*pc).comment,
                    );
                    current_block = 4014953649076014837;
                }
                29 | 30 => {
                    let mut assignment: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut final_0: *mut libc::c_char = 0 as *mut libc::c_char;
                    t1 = pp_pop();
                    if is_binary((*t1).type_0 as libc::c_int) {
                        pp_parenthesize(t1);
                    }
                    t2 = pp_pop();
                    assignment = pp_group3(
                        (*t1).sub.nodep.name,
                        op2str((*pc).opcode),
                        (*t2).sub.nodep.name,
                    );
                    final_0 = pp_group3(
                        b"$\0" as *const u8 as *const libc::c_char,
                        assignment,
                        b"\0" as *const u8 as *const libc::c_char,
                    );
                    free(assignment as *mut libc::c_void);
                    pp_free(t2);
                    pp_free(t1);
                    if (*pc).opcode as libc::c_uint
                        == Op_store_field_exp as libc::c_int as libc::c_uint
                    {
                        pp_push(
                            (*pc).opcode as libc::c_int,
                            final_0,
                            2 as libc::c_int,
                            0 as *mut INSTRUCTION,
                        );
                    } else {
                        fprintf(
                            prof_fp,
                            b"%s\0" as *const u8 as *const libc::c_char,
                            final_0,
                        );
                        free(final_0 as *mut libc::c_void);
                        if flags & 1 as libc::c_int == 0 as libc::c_int {
                            pc = end_line(pc);
                        }
                    }
                    current_block = 4014953649076014837;
                }
                13 => {
                    str = pp_concat((*pc).x.xl as libc::c_int);
                    pp_push(
                        Op_concat as libc::c_int,
                        str,
                        2 as libc::c_int,
                        (*pc).comment,
                    );
                    current_block = 4014953649076014837;
                }
                63 => {
                    let mut array: *mut libc::c_char = 0 as *mut libc::c_char;
                    t1 = pp_pop();
                    array = (*t1).sub.nodep.name;
                    if (*pc).x.xl > 0 as libc::c_int as libc::c_long {
                        let mut sub: *mut libc::c_char = 0 as *mut libc::c_char;
                        sub = pp_list(
                            (*pc).x.xl as libc::c_int,
                            0 as *const libc::c_char,
                            if (*pc).x.xl > 1 as libc::c_int as libc::c_long {
                                b"][\0" as *const u8 as *const libc::c_char
                            } else {
                                b", \0" as *const u8 as *const libc::c_char
                            },
                        );
                        fprintf(
                            prof_fp,
                            b"%s %s[%s]\0" as *const u8 as *const libc::c_char,
                            op2str(Op_K_delete),
                            array,
                            sub,
                        );
                        free(sub as *mut libc::c_void);
                    } else {
                        fprintf(
                            prof_fp,
                            b"%s %s\0" as *const u8 as *const libc::c_char,
                            op2str(Op_K_delete),
                            array,
                        );
                    }
                    if flags & 1 as libc::c_int == 0 as libc::c_int {
                        pc = end_line(pc);
                    }
                    pp_free(t1);
                    current_block = 4014953649076014837;
                }
                64 => {
                    r_fatal(
                        b"internal error: file %s, line %d: unexpected opcode %s\0"
                            as *const u8 as *const libc::c_char,
                        b"profile.c\0" as *const u8 as *const libc::c_char,
                        591 as libc::c_int,
                        opcode2str((*pc).opcode),
                    );
                    current_block = 4014953649076014837;
                }
                72 => {
                    let mut array_0: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut sub_0: *mut libc::c_char = 0 as *mut libc::c_char;
                    t1 = pp_pop();
                    array_0 = (*t1).sub.nodep.name;
                    if (*pc).x.xl > 1 as libc::c_int as libc::c_long {
                        sub_0 = pp_list(
                            (*pc).x.xl as libc::c_int,
                            b"()\0" as *const u8 as *const libc::c_char,
                            b", \0" as *const u8 as *const libc::c_char,
                        );
                        str = pp_group3(sub_0, op2str(Op_in_array), array_0);
                        free(sub_0 as *mut libc::c_void);
                    } else {
                        t2 = pp_pop();
                        if prec_level((*t2).type_0 as libc::c_int)
                            < prec_level(Op_in_array as libc::c_int)
                        {
                            pp_parenthesize(t2);
                        }
                        sub_0 = (*t2).sub.nodep.name;
                        str = pp_group3(sub_0, op2str(Op_in_array), array_0);
                        pp_free(t2);
                    }
                    pp_free(t1);
                    pp_push(
                        Op_in_array as libc::c_int,
                        str,
                        2 as libc::c_int,
                        (*pc).comment,
                    );
                    current_block = 4014953649076014837;
                }
                70 => {
                    let mut fname: *const libc::c_char = b"sub\0" as *const u8
                        as *const libc::c_char;
                    if (*pc).d.dl & 0x1 as libc::c_int as libc::c_long
                        != 0 as libc::c_int as libc::c_long
                    {
                        fname = b"gsub\0" as *const u8 as *const libc::c_char;
                    } else if (*pc).d.dl & 0x2 as libc::c_int as libc::c_long
                        != 0 as libc::c_int as libc::c_long
                    {
                        fname = b"gensub\0" as *const u8 as *const libc::c_char;
                    }
                    tmp = pp_list(
                        (*pc).x.xl as libc::c_int,
                        b"()\0" as *const u8 as *const libc::c_char,
                        b", \0" as *const u8 as *const libc::c_char,
                    );
                    str = pp_group3(
                        fname,
                        tmp,
                        b"\0" as *const u8 as *const libc::c_char,
                    );
                    free(tmp as *mut libc::c_void);
                    pp_push(
                        Op_sub_builtin as libc::c_int,
                        str,
                        2 as libc::c_int,
                        (*pc).comment,
                    );
                    current_block = 4014953649076014837;
                }
                69 | 71 => {
                    let mut fname_0: *const libc::c_char = 0 as *const libc::c_char;
                    if (*pc).opcode as libc::c_uint
                        == Op_builtin as libc::c_int as libc::c_uint
                    {
                        let mut prepend_awk: bool = current_namespace
                            != awk_namespace.as_ptr()
                            && strcmp(current_namespace, awk_namespace.as_ptr())
                                != 0 as libc::c_int;
                        fname_0 = getfname((*pc).d.fptr, prepend_awk);
                    } else {
                        fname_0 = (*pc.offset(1 as libc::c_int as isize)).d.name;
                    }
                    if !fname_0.is_null() {
                        if (*pc).x.xl > 0 as libc::c_int as libc::c_long {
                            tmp = pp_list(
                                (*pc).x.xl as libc::c_int,
                                b"()\0" as *const u8 as *const libc::c_char,
                                b", \0" as *const u8 as *const libc::c_char,
                            );
                            str = pp_group3(
                                fname_0,
                                tmp,
                                b"\0" as *const u8 as *const libc::c_char,
                            );
                            free(tmp as *mut libc::c_void);
                        } else {
                            str = pp_group3(
                                fname_0,
                                b"()\0" as *const u8 as *const libc::c_char,
                                b"\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        pp_push(
                            Op_builtin as libc::c_int,
                            str,
                            2 as libc::c_int,
                            (*pc).comment,
                        );
                    } else {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> ())(
                            b"profile.c\0" as *const u8 as *const libc::c_char,
                            670 as libc::c_int,
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
                                b"internal error: builtin with null fname\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                    current_block = 4014953649076014837;
                }
                56 | 58 | 57 => {
                    if (*pc).opcode as libc::c_uint
                        == Op_K_print_rec as libc::c_int as libc::c_uint
                    {
                        tmp = estrdup(
                            b"\0" as *const u8 as *const libc::c_char,
                            0 as libc::c_int as size_t,
                        );
                    } else if (*pc).d.dl != 0 as libc::c_int as libc::c_long {
                        let mut n: *mut NODE = pp_stack;
                        if (*pc).x.xl == 1 as libc::c_int as libc::c_long
                            && *((*n).sub.nodep.name).offset(0 as libc::c_int as isize)
                                as libc::c_int == '(' as i32
                            && *((*n).sub.nodep.name)
                                .offset(
                                    ((*n).sub.nodep.reserved)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                ) as libc::c_int == ')' as i32
                        {
                            n = pp_pop();
                            tmp = estrdup(
                                (*n).sub.nodep.name,
                                strlen((*n).sub.nodep.name),
                            );
                            pp_free(n);
                        } else {
                            tmp = pp_list(
                                (*pc).x.xl as libc::c_int,
                                b"()\0" as *const u8 as *const libc::c_char,
                                b", \0" as *const u8 as *const libc::c_char,
                            );
                        }
                    } else {
                        tmp = pp_list(
                            (*pc).x.xl as libc::c_int,
                            b"  \0" as *const u8 as *const libc::c_char,
                            b", \0" as *const u8 as *const libc::c_char,
                        );
                        *tmp
                            .offset(
                                (strlen(tmp))
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            ) = '\0' as i32 as libc::c_char;
                    }
                    if (*pc).d.dl != 0 as libc::c_int as libc::c_long {
                        t1 = pp_pop();
                        if is_binary((*t1).type_0 as libc::c_int) {
                            pp_parenthesize(t1);
                        }
                        fprintf(
                            prof_fp,
                            b"%s%s%s%s\0" as *const u8 as *const libc::c_char,
                            op2str((*pc).opcode),
                            tmp,
                            redir2str((*pc).d.dl as libc::c_int),
                            (*t1).sub.nodep.name,
                        );
                        pp_free(t1);
                    } else {
                        fprintf(
                            prof_fp,
                            b"%s%s\0" as *const u8 as *const libc::c_char,
                            op2str((*pc).opcode),
                            tmp,
                        );
                    }
                    free(tmp as *mut libc::c_void);
                    if flags & 1 as libc::c_int == 0 as libc::c_int {
                        pc = end_line(pc);
                    }
                    current_block = 4014953649076014837;
                }
                79 => {
                    if (*(*pc).d.dn).type_0 as libc::c_uint
                        != Node_regex as libc::c_int as libc::c_uint
                        && (*(*pc).d.dn).flags as libc::c_uint
                            & REGEX as libc::c_int as libc::c_uint
                            == 0 as libc::c_int as libc::c_uint
                    {
                        current_block = 4014953649076014837;
                    } else {
                        current_block = 16338561549115804736;
                    }
                }
                49 => {
                    current_block = 16338561549115804736;
                }
                50 | 48 => {
                    let mut restr: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut txt: *mut libc::c_char = 0 as *mut libc::c_char;
                    t1 = pp_pop();
                    if is_binary((*t1).type_0 as libc::c_int) {
                        pp_parenthesize(t1);
                    }
                    txt = (*t1).sub.nodep.name;
                    m = (*pc).d.dn;
                    if (*m).type_0 as libc::c_uint
                        == Node_dynregex as libc::c_int as libc::c_uint
                    {
                        restr = txt;
                        t2 = pp_pop();
                        if is_binary((*t2).type_0 as libc::c_int) {
                            pp_parenthesize(t2);
                        }
                        txt = (*t2).sub.nodep.name;
                        str = pp_group3(txt, op2str((*pc).opcode), restr);
                        pp_free(t2);
                    } else if (*m).type_0 as libc::c_uint
                        == Node_val as libc::c_int as libc::c_uint
                        && (*m).flags as libc::c_uint
                            & REGEX as libc::c_int as libc::c_uint
                            != 0 as libc::c_int as libc::c_uint
                    {
                        restr = pp_typed_regex(
                            (*m).sub.val.sp,
                            (*m).sub.val.slen,
                            '/' as i32,
                        );
                        str = pp_group3(txt, op2str((*pc).opcode), restr);
                        free(restr as *mut libc::c_void);
                    } else {
                        let mut re_0: *mut NODE = (*m).sub.nodep.x.extra;
                        restr = pp_string(
                            (*re_0).sub.val.sp,
                            (*re_0).sub.val.slen,
                            '/' as i32,
                        );
                        str = pp_group3(txt, op2str((*pc).opcode), restr);
                        free(restr as *mut libc::c_void);
                    }
                    pp_free(t1);
                    pp_push(
                        (*pc).opcode as libc::c_int,
                        str,
                        2 as libc::c_int,
                        (*pc).comment,
                    );
                    current_block = 4014953649076014837;
                }
                66 | 65 => {
                    if (*pc).x.xl != 0 {
                        t1 = pp_pop();
                        tmp = pp_group3(
                            op2str(Op_K_getline),
                            b" \0" as *const u8 as *const libc::c_char,
                            (*t1).sub.nodep.name,
                        );
                        pp_free(t1);
                    } else {
                        tmp = pp_group3(
                            op2str(Op_K_getline),
                            b"\0" as *const u8 as *const libc::c_char,
                            b"\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    if (*pc).d.dl != 0 as libc::c_int as libc::c_long {
                        let mut before: libc::c_int = ((*pc).d.dl
                            == redirect_pipein as libc::c_int as libc::c_long
                            || (*pc).d.dl
                                == redirect_twoway as libc::c_int as libc::c_long)
                            as libc::c_int;
                        t2 = pp_pop();
                        if is_binary((*t2).type_0 as libc::c_int) {
                            pp_parenthesize(t2);
                        }
                        if before != 0 {
                            str = pp_group3(
                                (*t2).sub.nodep.name,
                                redir2str((*pc).d.dl as libc::c_int),
                                tmp,
                            );
                        } else {
                            str = pp_group3(
                                tmp,
                                redir2str((*pc).d.dl as libc::c_int),
                                (*t2).sub.nodep.name,
                            );
                        }
                        free(tmp as *mut libc::c_void);
                        pp_free(t2);
                    } else {
                        str = tmp;
                    }
                    pp_push(
                        (*pc).opcode as libc::c_int,
                        str,
                        2 as libc::c_int,
                        (*pc).comment,
                    );
                    current_block = 4014953649076014837;
                }
                74 | 73 => {
                    let mut pre: *const libc::c_char = 0 as *const libc::c_char;
                    let mut pcount: libc::c_int = 0;
                    let mut malloced_0: bool = 0 as libc::c_int != 0;
                    let mut fname_1: *mut libc::c_char = adjust_namespace(
                        (*pc).d.name,
                        &mut malloced_0,
                    );
                    if (*pc).opcode as libc::c_uint
                        == Op_indirect_func_call as libc::c_int as libc::c_uint
                    {
                        pre = b"@\0" as *const u8 as *const libc::c_char;
                    } else {
                        pre = b"\0" as *const u8 as *const libc::c_char;
                    }
                    pcount = (*pc.offset(1 as libc::c_int as isize)).x.xl as libc::c_int;
                    if pcount > 0 as libc::c_int {
                        tmp = pp_list(
                            pcount,
                            b"()\0" as *const u8 as *const libc::c_char,
                            b", \0" as *const u8 as *const libc::c_char,
                        );
                        str = pp_group3(pre, fname_1, tmp);
                        free(tmp as *mut libc::c_void);
                    } else {
                        str = pp_group3(
                            pre,
                            fname_1,
                            b"()\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    if (*pc).opcode as libc::c_uint
                        == Op_indirect_func_call as libc::c_int as libc::c_uint
                    {
                        t1 = pp_pop();
                        pp_free(t1);
                    }
                    pp_push(
                        (*pc).opcode as libc::c_int,
                        str,
                        2 as libc::c_int,
                        (*pc).comment,
                    );
                    if malloced_0 {
                        free(fname_1 as *mut libc::c_void);
                    }
                    current_block = 4014953649076014837;
                }
                55 | 54 | 67 | 59 => {
                    fprintf(
                        prof_fp,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        op2str((*pc).opcode),
                    );
                    pc = end_line(pc);
                    current_block = 4014953649076014837;
                }
                61 | 60 => {
                    t1 = pp_pop();
                    if is_binary((*t1).type_0 as libc::c_int) {
                        pp_parenthesize(t1);
                    }
                    if (*pc).source_line as libc::c_int > 0 as libc::c_int {
                        if *((*t1).sub.nodep.name).offset(0 as libc::c_int as isize)
                            as libc::c_int != '\0' as i32
                        {
                            fprintf(
                                prof_fp,
                                b"%s %s\0" as *const u8 as *const libc::c_char,
                                op2str((*pc).opcode),
                                (*t1).sub.nodep.name,
                            );
                        } else {
                            fprintf(
                                prof_fp,
                                b"%s\0" as *const u8 as *const libc::c_char,
                                op2str((*pc).opcode),
                            );
                        }
                        pc = end_line(pc);
                    }
                    pp_free(t1);
                    current_block = 4014953649076014837;
                }
                86 => {
                    t1 = pp_pop();
                    fprintf(
                        prof_fp,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        (*t1).sub.nodep.name,
                    );
                    if flags & 1 as libc::c_int == 0 as libc::c_int {
                        pc = end_line(pc);
                    }
                    pp_free(t1);
                    current_block = 4014953649076014837;
                }
                14 => {
                    ip1 = pc.offset(1 as libc::c_int as isize);
                    pprint((*pc).nexti, (*ip1).d.di, 0 as libc::c_int);
                    pprint((*(*ip1).d.di).nexti, (*ip1).x.xi, 0 as libc::c_int);
                    t2 = pp_pop();
                    t1 = pp_pop();
                    str = pp_group3(
                        (*t1).sub.nodep.name,
                        b", \0" as *const u8 as *const libc::c_char,
                        (*t2).sub.nodep.name,
                    );
                    pp_free(t1);
                    pp_free(t2);
                    pp_push(
                        Op_line_range as libc::c_int,
                        str,
                        2 as libc::c_int,
                        (*pc).comment,
                    );
                    pc = (*ip1).x.xi;
                    current_block = 4014953649076014837;
                }
                115 => {
                    ip1 = pc.offset(1 as libc::c_int as isize);
                    indent((*(*ip1).d.di).d.ldl);
                    fprintf(
                        prof_fp,
                        b"%s (\0" as *const u8 as *const libc::c_char,
                        op2str((*pc).opcode),
                    );
                    pprint((*pc).nexti, (*ip1).d.di, 0 as libc::c_int);
                    t1 = pp_pop();
                    fprintf(
                        prof_fp,
                        b"%s) {\0" as *const u8 as *const libc::c_char,
                        (*t1).sub.nodep.name,
                    );
                    pp_free(t1);
                    (*ip1).d.di = end_line((*ip1).d.di);
                    indent_in();
                    pprint((*(*ip1).d.di).nexti, (*pc).x.xi, 0 as libc::c_int);
                    indent_out();
                    indent(0 as libc::c_int as exec_count_t);
                    fprintf(prof_fp, b"}\0" as *const u8 as *const libc::c_char);
                    pc = end_line((*pc).x.xi);
                    current_block = 4014953649076014837;
                }
                112 => {
                    ip1 = pc.offset(1 as libc::c_int as isize);
                    indent((*(*pc).nexti).d.ldl);
                    fprintf(
                        prof_fp,
                        b"%s {\0" as *const u8 as *const libc::c_char,
                        op2str((*pc).opcode),
                    );
                    end_line((*pc).nexti);
                    skip_comment = 1 as libc::c_int != 0;
                    indent_in();
                    pprint((*(*pc).nexti).nexti, (*ip1).d.di, 0 as libc::c_int);
                    indent_out();
                    pprint((*ip1).d.di, (*pc).x.xi, 0 as libc::c_int);
                    indent(0 as libc::c_int as exec_count_t);
                    t1 = pp_pop();
                    fprintf(
                        prof_fp,
                        b"} %s (%s)\0" as *const u8 as *const libc::c_char,
                        op2str(Op_K_while),
                        (*t1).sub.nodep.name,
                    );
                    if !((*pc).comment).is_null() {
                        fprintf(
                            prof_fp,
                            b"\t%s\0" as *const u8 as *const libc::c_char,
                            (*(*(*pc).comment).d.dn).sub.val.sp,
                        );
                    } else {
                        end_line((*pc).x.xi);
                        skip_comment = 1 as libc::c_int != 0;
                    }
                    pp_free(t1);
                    pc = (*pc).x.xi;
                    current_block = 4014953649076014837;
                }
                113 => {
                    let mut comment1: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                    let mut comment2: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                    if !((*pc).comment).is_null() {
                        comment1 = (*pc).comment;
                        (*pc).comment = 0 as *mut exp_instruction;
                        if !comment1.is_null() && !((*comment1).comment).is_null() {
                            comment2 = (*comment1).comment;
                            (*comment1).comment = 0 as *mut exp_instruction;
                        }
                        if comment2.is_null()
                            && (*(*comment1).d.dn).sub.val.comtype as libc::c_uint
                                == FOR_COMMENT as libc::c_int as libc::c_uint
                        {
                            comment2 = comment1;
                            (*(*comment2).d.dn).sub.val.comtype = EOL_COMMENT;
                            comment1 = 0 as *mut INSTRUCTION;
                        }
                    }
                    ip1 = pc.offset(1 as libc::c_int as isize);
                    indent((*(*ip1).x.xi).d.ldl);
                    fprintf(
                        prof_fp,
                        b"%s (\0" as *const u8 as *const libc::c_char,
                        op2str((*pc).opcode),
                    );
                    if (*(*pc).nexti).opcode as libc::c_uint
                        == Op_no_op as libc::c_int as libc::c_uint
                        && (*ip1).d.di == (*pc).nexti
                        && (*(*pc).d.di).opcode as libc::c_uint
                            == Op_jmp as libc::c_int as libc::c_uint
                        && comment1.is_null() && comment2.is_null()
                    {
                        fprintf(prof_fp, b";;\0" as *const u8 as *const libc::c_char);
                    } else {
                        pprint((*pc).nexti, (*ip1).d.di, 1 as libc::c_int);
                        fprintf(prof_fp, b"; \0" as *const u8 as *const libc::c_char);
                        if !comment1.is_null() {
                            print_comment(comment1, 0 as libc::c_int as libc::c_long);
                            indent((*(*ip1).x.xi).d.ldl);
                            indent(1 as libc::c_int as exec_count_t);
                        }
                        if (*(*ip1).d.di).opcode as libc::c_uint
                            == Op_no_op as libc::c_int as libc::c_uint
                            && (*(*ip1).d.di).nexti == (*ip1).x.xi
                        {
                            fprintf(
                                prof_fp,
                                b"; \0" as *const u8 as *const libc::c_char,
                            );
                        } else {
                            pprint((*ip1).d.di, (*ip1).x.xi, 1 as libc::c_int);
                            t1 = pp_pop();
                            fprintf(
                                prof_fp,
                                b"%s; \0" as *const u8 as *const libc::c_char,
                                (*t1).sub.nodep.name,
                            );
                            pp_free(t1);
                        }
                        if !comment2.is_null() {
                            print_comment(comment2, 0 as libc::c_int as libc::c_long);
                            indent((*(*ip1).x.xi).d.ldl);
                            indent(1 as libc::c_int as exec_count_t);
                        }
                        pprint((*pc).d.di, (*pc).x.xi, 1 as libc::c_int);
                    }
                    fprintf(prof_fp, b") {\0" as *const u8 as *const libc::c_char);
                    end_line((*ip1).x.xi);
                    skip_comment = 1 as libc::c_int != 0;
                    indent_in();
                    pprint((*(*ip1).x.xi).nexti, (*pc).d.di, 0 as libc::c_int);
                    indent_out();
                    indent(0 as libc::c_int as exec_count_t);
                    fprintf(prof_fp, b"}\0" as *const u8 as *const libc::c_char);
                    end_line((*pc).x.xi);
                    skip_comment = 1 as libc::c_int != 0;
                    pc = (*pc).x.xi;
                    current_block = 4014953649076014837;
                }
                114 => {
                    let mut array_1: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut item: *const libc::c_char = 0 as *const libc::c_char;
                    ip1 = pc.offset(1 as libc::c_int as isize);
                    t1 = pp_pop();
                    array_1 = (*t1).sub.nodep.name;
                    m = (*(*ip1).d.di).x.xn;
                    if (*m).type_0 as libc::c_uint
                        == Node_param_list as libc::c_int as libc::c_uint
                    {
                        item = (*func_params.offset((*m).sub.nodep.l.ll as isize))
                            .sub
                            .nodep
                            .name;
                    } else {
                        item = (*m).sub.nodep.name;
                    }
                    indent((*(*ip1).x.xi).d.ldl);
                    fprintf(
                        prof_fp,
                        b"%s (%s%s%s) {\0" as *const u8 as *const libc::c_char,
                        op2str(Op_K_arrayfor),
                        item,
                        op2str(Op_in_array),
                        array_1,
                    );
                    end_line((*ip1).x.xi);
                    skip_comment = 1 as libc::c_int != 0;
                    indent_in();
                    pp_free(t1);
                    pprint((*(*ip1).x.xi).nexti, (*pc).x.xi, 0 as libc::c_int);
                    indent_out();
                    indent(0 as libc::c_int as exec_count_t);
                    fprintf(prof_fp, b"}\0" as *const u8 as *const libc::c_char);
                    end_line((*pc).x.xi);
                    skip_comment = 1 as libc::c_int != 0;
                    pc = (*pc).x.xi;
                    current_block = 4014953649076014837;
                }
                116 => {
                    ip1 = pc.offset(1 as libc::c_int as isize);
                    fprintf(
                        prof_fp,
                        b"%s (\0" as *const u8 as *const libc::c_char,
                        op2str((*pc).opcode),
                    );
                    pprint((*pc).nexti, (*ip1).d.di, 0 as libc::c_int);
                    t1 = pp_pop();
                    fprintf(
                        prof_fp,
                        b"%s) {\0" as *const u8 as *const libc::c_char,
                        (*t1).sub.nodep.name,
                    );
                    if !((*pc).comment).is_null() {
                        print_comment((*pc).comment, 0 as libc::c_int as libc::c_long);
                    } else {
                        fprintf(prof_fp, b"\n\0" as *const u8 as *const libc::c_char);
                    }
                    pp_free(t1);
                    pprint((*ip1).d.di, (*ip1).x.xi, 0 as libc::c_int);
                    indent(0 as libc::c_int as exec_count_t);
                    fprintf(prof_fp, b"}\n\0" as *const u8 as *const libc::c_char);
                    if !((*(*ip1).x.xi).comment).is_null() {
                        print_comment(
                            (*(*ip1).x.xi).comment,
                            0 as libc::c_int as libc::c_long,
                        );
                    }
                    pc = (*pc).x.xi;
                    current_block = 4014953649076014837;
                }
                52 | 53 => {
                    indent((*(*pc).d.di).d.ldl);
                    if (*pc).opcode as libc::c_uint
                        == Op_K_case as libc::c_int as libc::c_uint
                    {
                        t1 = pp_pop();
                        fprintf(
                            prof_fp,
                            b"%s %s:\0" as *const u8 as *const libc::c_char,
                            op2str((*pc).opcode),
                            (*t1).sub.nodep.name,
                        );
                        pp_free(t1);
                    } else {
                        fprintf(
                            prof_fp,
                            b"%s:\0" as *const u8 as *const libc::c_char,
                            op2str((*pc).opcode),
                        );
                    }
                    indent_in();
                    if !((*pc).comment).is_null() {
                        if (*(*(*pc).comment).d.dn).sub.val.comtype as libc::c_uint
                            == EOL_COMMENT as libc::c_int as libc::c_uint
                        {
                            fprintf(
                                prof_fp,
                                b"\t%s\0" as *const u8 as *const libc::c_char,
                                (*(*(*pc).comment).d.dn).sub.val.sp,
                            );
                            if !((*(*pc).comment).comment).is_null() {
                                print_comment((*(*pc).comment).comment, indent_level);
                            }
                        } else {
                            fprintf(
                                prof_fp,
                                b"\n\0" as *const u8 as *const libc::c_char,
                            );
                            print_comment((*pc).comment, indent_level);
                        }
                    } else {
                        fprintf(prof_fp, b"\n\0" as *const u8 as *const libc::c_char);
                    }
                    pprint((*(*pc).d.di).nexti, (*(*pc).x.xi).nexti, 0 as libc::c_int);
                    indent_out();
                    current_block = 4014953649076014837;
                }
                117 => {
                    fprintf(
                        prof_fp,
                        b"%s (\0" as *const u8 as *const libc::c_char,
                        op2str((*pc).opcode),
                    );
                    pprint((*pc).nexti, (*pc).d.di, 0 as libc::c_int);
                    t1 = pp_pop();
                    fprintf(
                        prof_fp,
                        b"%s) {\0" as *const u8 as *const libc::c_char,
                        (*t1).sub.nodep.name,
                    );
                    pp_free(t1);
                    ip1 = (*pc).d.di;
                    if (*ip1).d.ldl > 0 as libc::c_int as libc::c_ulonglong {
                        fprintf(
                            prof_fp,
                            b" # %llu\0" as *const u8 as *const libc::c_char,
                            (*ip1).d.ldl,
                        );
                    }
                    ip1 = end_line(ip1);
                    indent_in();
                    if !((*pc).comment).is_null() {
                        print_comment((*pc).comment, indent_level);
                    }
                    pprint((*ip1).nexti, (*pc).x.xi, 0 as libc::c_int);
                    indent_out();
                    pc = (*pc).x.xi;
                    if (*(*pc).nexti).opcode as libc::c_uint
                        == Op_no_op as libc::c_int as libc::c_uint
                    {
                        indent(0 as libc::c_int as exec_count_t);
                        fprintf(prof_fp, b"}\0" as *const u8 as *const libc::c_char);
                        if (*(*(*pc).nexti).nexti).opcode as libc::c_uint
                            != Op_comment as libc::c_int as libc::c_uint
                            || (*(*(*(*pc).nexti).nexti).d.dn).sub.val.comtype
                                as libc::c_uint
                                == BLOCK_COMMENT as libc::c_int as libc::c_uint
                        {
                            fprintf(
                                prof_fp,
                                b"\n\0" as *const u8 as *const libc::c_char,
                            );
                        }
                    }
                    flags &= !(2 as libc::c_int);
                    current_block = 4014953649076014837;
                }
                118 => {
                    fprintf(
                        prof_fp,
                        b"} %s \0" as *const u8 as *const libc::c_char,
                        op2str((*pc).opcode),
                    );
                    if (*(*(*pc).nexti).nexti).opcode as libc::c_uint
                        == Op_K_if as libc::c_int as libc::c_uint
                        && (*pc).x.xi == (*(*(*(*pc).nexti).nexti).x.xi).d.di
                    {
                        pprint((*pc).nexti, (*pc).x.xi, 2 as libc::c_int);
                    } else {
                        fprintf(prof_fp, b"{\0" as *const u8 as *const libc::c_char);
                        end_line(pc);
                        skip_comment = 1 as libc::c_int != 0;
                        indent_in();
                        if !((*pc).comment).is_null() {
                            print_comment((*pc).comment, indent_level);
                        }
                        pprint((*pc).nexti, (*pc).x.xi, 0 as libc::c_int);
                        indent_out();
                        indent(0 as libc::c_int as exec_count_t);
                        fprintf(prof_fp, b"}\0" as *const u8 as *const libc::c_char);
                        end_line((*pc).x.xi);
                        skip_comment = 1 as libc::c_int != 0;
                    }
                    pc = (*pc).x.xi;
                    current_block = 4014953649076014837;
                }
                120 => {
                    let mut f: *mut NODE = 0 as *mut NODE;
                    let mut t: *mut NODE = 0 as *mut NODE;
                    let mut cond: *mut NODE = 0 as *mut NODE;
                    let mut len_0: size_t = 0;
                    let mut qm_comment: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                    let mut colon_comment: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                    qm_comment = (*pc).comment;
                    pprint((*pc).nexti, (*pc).d.di, 0 as libc::c_int);
                    ip1 = (*pc).d.di;
                    pprint((*ip1).nexti, (*pc).x.xi, 0 as libc::c_int);
                    ip1 = (*(*pc).x.xi).nexti;
                    pc = (*ip1).nexti;
                    colon_comment = (*pc).comment;
                    pprint((*pc).nexti, (*pc).x.xi, 0 as libc::c_int);
                    f = pp_pop();
                    t = pp_pop();
                    cond = pp_pop();
                    len_0 = ((*f).sub.nodep.reserved)
                        .wrapping_add((*t).sub.nodep.reserved)
                        .wrapping_add((*cond).sub.nodep.reserved)
                        .wrapping_add(12 as libc::c_int as libc::c_ulong);
                    if qm_comment.is_null() && colon_comment.is_null() {
                        str = emalloc_real(
                            len_0,
                            b"pprint\0" as *const u8 as *const libc::c_char,
                            b"str\0" as *const u8 as *const libc::c_char,
                            b"profile.c\0" as *const u8 as *const libc::c_char,
                            1155 as libc::c_int,
                        ) as *mut libc::c_char;
                        sprintf(
                            str,
                            b"%s ? %s : %s\0" as *const u8 as *const libc::c_char,
                            (*cond).sub.nodep.name,
                            (*t).sub.nodep.name,
                            (*f).sub.nodep.name,
                        );
                    } else if !qm_comment.is_null() && !colon_comment.is_null() {
                        if (indent_level + 1 as libc::c_int as libc::c_long)
                            as libc::c_ulong > tabs_len
                        {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    libc::c_int,
                                ) -> ())(
                                b"profile.c\0" as *const u8 as *const libc::c_char,
                                1158 as libc::c_int,
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
                                    b"Program indentation level too deep. Consider refactoring your code\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                        }
                        len_0 = (len_0 as libc::c_ulong)
                            .wrapping_add(
                                ((*(*qm_comment).d.dn).sub.val.slen)
                                    .wrapping_add((*(*colon_comment).d.dn).sub.val.slen)
                                    .wrapping_add(
                                        (2 as libc::c_int as libc::c_long
                                            * (indent_level + 1 as libc::c_int as libc::c_long))
                                            as libc::c_ulong,
                                    )
                                    .wrapping_add(3 as libc::c_int as libc::c_ulong)
                                    .wrapping_add((*t).sub.nodep.reserved)
                                    .wrapping_add(6 as libc::c_int as libc::c_ulong),
                            ) as size_t as size_t;
                        str = emalloc_real(
                            len_0,
                            b"pprint\0" as *const u8 as *const libc::c_char,
                            b"str\0" as *const u8 as *const libc::c_char,
                            b"profile.c\0" as *const u8 as *const libc::c_char,
                            1163 as libc::c_int,
                        ) as *mut libc::c_char;
                        sprintf(
                            str,
                            b"%s ? %s%.*s   %s : %s%.*s   %s\0" as *const u8
                                as *const libc::c_char,
                            (*cond).sub.nodep.name,
                            (*(*qm_comment).d.dn).sub.val.sp,
                            (indent_level + 1 as libc::c_int as libc::c_long)
                                as libc::c_int,
                            tabs.as_ptr(),
                            (*t).sub.nodep.name,
                            (*(*colon_comment).d.dn).sub.val.sp,
                            (indent_level + 1 as libc::c_int as libc::c_long)
                                as libc::c_int,
                            tabs.as_ptr(),
                            (*f).sub.nodep.name,
                        );
                    } else if !qm_comment.is_null() {
                        if (indent_level + 1 as libc::c_int as libc::c_long)
                            as libc::c_ulong > tabs_len
                        {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    libc::c_int,
                                ) -> ())(
                                b"profile.c\0" as *const u8 as *const libc::c_char,
                                1178 as libc::c_int,
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
                                    b"Program indentation level too deep. Consider refactoring your code\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                        }
                        len_0 = (len_0 as libc::c_ulong)
                            .wrapping_add(
                                ((*(*qm_comment).d.dn).sub.val.slen)
                                    .wrapping_add(
                                        (1 as libc::c_int as libc::c_long
                                            * (indent_level + 1 as libc::c_int as libc::c_long))
                                            as libc::c_ulong,
                                    )
                                    .wrapping_add(3 as libc::c_int as libc::c_ulong)
                                    .wrapping_add((*t).sub.nodep.reserved)
                                    .wrapping_add(3 as libc::c_int as libc::c_ulong),
                            ) as size_t as size_t;
                        str = emalloc_real(
                            len_0,
                            b"pprint\0" as *const u8 as *const libc::c_char,
                            b"str\0" as *const u8 as *const libc::c_char,
                            b"profile.c\0" as *const u8 as *const libc::c_char,
                            1182 as libc::c_int,
                        ) as *mut libc::c_char;
                        sprintf(
                            str,
                            b"%s ? %s%.*s   %s : %s\0" as *const u8
                                as *const libc::c_char,
                            (*cond).sub.nodep.name,
                            (*(*qm_comment).d.dn).sub.val.sp,
                            (indent_level + 1 as libc::c_int as libc::c_long)
                                as libc::c_int,
                            tabs.as_ptr(),
                            (*t).sub.nodep.name,
                            (*f).sub.nodep.name,
                        );
                    } else {
                        if (indent_level + 1 as libc::c_int as libc::c_long)
                            as libc::c_ulong > tabs_len
                        {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    libc::c_int,
                                ) -> ())(
                                b"profile.c\0" as *const u8 as *const libc::c_char,
                                1194 as libc::c_int,
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
                                    b"Program indentation level too deep. Consider refactoring your code\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                        }
                        len_0 = (len_0 as libc::c_ulong)
                            .wrapping_add(
                                ((*(*colon_comment).d.dn).sub.val.slen)
                                    .wrapping_add(
                                        (1 as libc::c_int as libc::c_long
                                            * (indent_level + 1 as libc::c_int as libc::c_long))
                                            as libc::c_ulong,
                                    )
                                    .wrapping_add(3 as libc::c_int as libc::c_ulong)
                                    .wrapping_add((*t).sub.nodep.reserved)
                                    .wrapping_add(3 as libc::c_int as libc::c_ulong),
                            ) as size_t as size_t;
                        str = emalloc_real(
                            len_0,
                            b"pprint\0" as *const u8 as *const libc::c_char,
                            b"str\0" as *const u8 as *const libc::c_char,
                            b"profile.c\0" as *const u8 as *const libc::c_char,
                            1198 as libc::c_int,
                        ) as *mut libc::c_char;
                        sprintf(
                            str,
                            b"%s ? %s : %s%.*s   %s\0" as *const u8
                                as *const libc::c_char,
                            (*cond).sub.nodep.name,
                            (*t).sub.nodep.name,
                            (*(*colon_comment).d.dn).sub.val.sp,
                            (indent_level + 1 as libc::c_int as libc::c_long)
                                as libc::c_int,
                            tabs.as_ptr(),
                            (*f).sub.nodep.name,
                        );
                    }
                    pp_free(cond);
                    pp_free(t);
                    pp_free(f);
                    pp_push(
                        Op_cond_exp as libc::c_int,
                        str,
                        2 as libc::c_int,
                        (*pc).comment,
                    );
                    pc = (*pc).x.xi;
                    current_block = 4014953649076014837;
                }
                103 => {
                    if flags == 0 as libc::c_int {
                        indent((*pc).d.ldl);
                    }
                    current_block = 4014953649076014837;
                }
                102 => {
                    print_comment(pc, 0 as libc::c_int as libc::c_long);
                    current_block = 4014953649076014837;
                }
                107 | 95 | 96 | 97 | 98 | 92 | 93 | 94 | 91 | 90 | 105 | 87 | 89 | 88
                | 85 | 39 | 41 | 15 | 99 | 100 | 111 => {
                    current_block = 4014953649076014837;
                }
                _ => {
                    r_fatal(
                        b"internal error: file %s, line %d: unexpected opcode %s\0"
                            as *const u8 as *const libc::c_char,
                        b"profile.c\0" as *const u8 as *const libc::c_char,
                        1232 as libc::c_int,
                        opcode2str((*pc).opcode),
                    );
                    current_block = 4014953649076014837;
                }
            }
            match current_block {
                7152033355135659062 => {
                    m = (*pc).d.dn;
                    match (*m).type_0 as libc::c_uint {
                        8 => {
                            pp_push(
                                (*pc).opcode as libc::c_int,
                                (*func_params.offset((*m).sub.nodep.l.ll as isize))
                                    .sub
                                    .nodep
                                    .name,
                                1 as libc::c_int,
                                (*pc).comment,
                            );
                        }
                        4 | 6 | 5 => {
                            if !((*m).sub.nodep.name).is_null() {
                                let mut malloced: bool = 0 as libc::c_int != 0;
                                let mut name: *mut libc::c_char = adjust_namespace(
                                    (*m).sub.nodep.name,
                                    &mut malloced,
                                );
                                pp_push(
                                    (*pc).opcode as libc::c_int,
                                    name,
                                    if malloced as libc::c_int != 0 {
                                        2 as libc::c_int
                                    } else {
                                        1 as libc::c_int
                                    },
                                    (*pc).comment,
                                );
                            } else {
                                (set_loc
                                    as unsafe extern "C" fn(
                                        *const libc::c_char,
                                        libc::c_int,
                                    ) -> ())(
                                    b"profile.c\0" as *const u8 as *const libc::c_char,
                                    366 as libc::c_int,
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
                                        b"internal error: %s with null vname\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    nodetype2str((*m).type_0),
                                );
                            }
                        }
                        _ => {
                            r_fatal(
                                b"internal error: file %s, line %d: got unexpected type %s\0"
                                    as *const u8 as *const libc::c_char,
                                b"profile.c\0" as *const u8 as *const libc::c_char,
                                371 as libc::c_int,
                                nodetype2str((*m).type_0),
                            );
                        }
                    }
                    match (*pc).opcode as libc::c_uint {
                        27 => {
                            current_block = 11250324638245971592;
                            match current_block {
                                11250324638245971592 => {
                                    t2 = pp_pop();
                                    t1 = pp_pop();
                                    fprintf(
                                        prof_fp,
                                        b"%s%s%s\0" as *const u8 as *const libc::c_char,
                                        (*t2).sub.nodep.name,
                                        op2str((*pc).opcode),
                                        (*t1).sub.nodep.name,
                                    );
                                }
                                12282016373282876249 => {
                                    t1 = pp_pop();
                                    tmp = pp_list(
                                        (*pc).x.xl as libc::c_int,
                                        op2str(Op_subscript),
                                        b", \0" as *const u8 as *const libc::c_char,
                                    );
                                    t2 = pp_pop();
                                    fprintf(
                                        prof_fp,
                                        b"%s%s%s%s\0" as *const u8 as *const libc::c_char,
                                        (*t1).sub.nodep.name,
                                        tmp,
                                        op2str((*pc).opcode),
                                        (*t2).sub.nodep.name,
                                    );
                                    free(tmp as *mut libc::c_void);
                                }
                                _ => {
                                    t2 = pp_pop();
                                    t1 = pp_pop();
                                    tmp = pp_group3(
                                        (*t2).sub.nodep.name,
                                        op2str(Op_concat),
                                        (*t1).sub.nodep.name,
                                    );
                                    fprintf(
                                        prof_fp,
                                        b"%s%s%s\0" as *const u8 as *const libc::c_char,
                                        (*t2).sub.nodep.name,
                                        op2str(Op_assign),
                                        tmp,
                                    );
                                    free(tmp as *mut libc::c_void);
                                }
                            }
                            pp_free(t2);
                            pp_free(t1);
                            if flags & 1 as libc::c_int == 0 as libc::c_int {
                                pc = end_line(pc);
                            }
                        }
                        28 => {
                            current_block = 12282016373282876249;
                            match current_block {
                                11250324638245971592 => {
                                    t2 = pp_pop();
                                    t1 = pp_pop();
                                    fprintf(
                                        prof_fp,
                                        b"%s%s%s\0" as *const u8 as *const libc::c_char,
                                        (*t2).sub.nodep.name,
                                        op2str((*pc).opcode),
                                        (*t1).sub.nodep.name,
                                    );
                                }
                                12282016373282876249 => {
                                    t1 = pp_pop();
                                    tmp = pp_list(
                                        (*pc).x.xl as libc::c_int,
                                        op2str(Op_subscript),
                                        b", \0" as *const u8 as *const libc::c_char,
                                    );
                                    t2 = pp_pop();
                                    fprintf(
                                        prof_fp,
                                        b"%s%s%s%s\0" as *const u8 as *const libc::c_char,
                                        (*t1).sub.nodep.name,
                                        tmp,
                                        op2str((*pc).opcode),
                                        (*t2).sub.nodep.name,
                                    );
                                    free(tmp as *mut libc::c_void);
                                }
                                _ => {
                                    t2 = pp_pop();
                                    t1 = pp_pop();
                                    tmp = pp_group3(
                                        (*t2).sub.nodep.name,
                                        op2str(Op_concat),
                                        (*t1).sub.nodep.name,
                                    );
                                    fprintf(
                                        prof_fp,
                                        b"%s%s%s\0" as *const u8 as *const libc::c_char,
                                        (*t2).sub.nodep.name,
                                        op2str(Op_assign),
                                        tmp,
                                    );
                                    free(tmp as *mut libc::c_void);
                                }
                            }
                            pp_free(t2);
                            pp_free(t1);
                            if flags & 1 as libc::c_int == 0 as libc::c_int {
                                pc = end_line(pc);
                            }
                        }
                        37 => {
                            current_block = 12550402795918481164;
                            match current_block {
                                11250324638245971592 => {
                                    t2 = pp_pop();
                                    t1 = pp_pop();
                                    fprintf(
                                        prof_fp,
                                        b"%s%s%s\0" as *const u8 as *const libc::c_char,
                                        (*t2).sub.nodep.name,
                                        op2str((*pc).opcode),
                                        (*t1).sub.nodep.name,
                                    );
                                }
                                12282016373282876249 => {
                                    t1 = pp_pop();
                                    tmp = pp_list(
                                        (*pc).x.xl as libc::c_int,
                                        op2str(Op_subscript),
                                        b", \0" as *const u8 as *const libc::c_char,
                                    );
                                    t2 = pp_pop();
                                    fprintf(
                                        prof_fp,
                                        b"%s%s%s%s\0" as *const u8 as *const libc::c_char,
                                        (*t1).sub.nodep.name,
                                        tmp,
                                        op2str((*pc).opcode),
                                        (*t2).sub.nodep.name,
                                    );
                                    free(tmp as *mut libc::c_void);
                                }
                                _ => {
                                    t2 = pp_pop();
                                    t1 = pp_pop();
                                    tmp = pp_group3(
                                        (*t2).sub.nodep.name,
                                        op2str(Op_concat),
                                        (*t1).sub.nodep.name,
                                    );
                                    fprintf(
                                        prof_fp,
                                        b"%s%s%s\0" as *const u8 as *const libc::c_char,
                                        (*t2).sub.nodep.name,
                                        op2str(Op_assign),
                                        tmp,
                                    );
                                    free(tmp as *mut libc::c_void);
                                }
                            }
                            pp_free(t2);
                            pp_free(t1);
                            if flags & 1 as libc::c_int == 0 as libc::c_int {
                                pc = end_line(pc);
                            }
                        }
                        _ => {}
                    }
                }
                16338561549115804736 => {
                    if (*(*pc).d.dn).type_0 as libc::c_uint
                        == Node_regex as libc::c_int as libc::c_uint
                    {
                        let mut re: *mut NODE = (*(*pc).d.dn).sub.nodep.x.extra;
                        str = pp_string(
                            (*re).sub.val.sp,
                            (*re).sub.val.slen,
                            '/' as i32,
                        );
                    } else {
                        str = pp_typed_regex(
                            (*(*pc).d.dn).sub.val.sp,
                            (*(*pc).d.dn).sub.val.slen,
                            '/' as i32,
                        );
                    }
                    pp_push(
                        (*pc).opcode as libc::c_int,
                        str,
                        2 as libc::c_int,
                        (*pc).comment,
                    );
                }
                _ => {}
            }
            if pc == endp {
                break;
            }
        }
        pc = (*pc).nexti;
    }
}
unsafe extern "C" fn end_line(mut ip: *mut INSTRUCTION) -> *mut INSTRUCTION {
    let mut ret: *mut INSTRUCTION = ip;
    if (*(*ip).nexti).opcode as libc::c_uint == Op_comment as libc::c_int as libc::c_uint
        && (*(*(*ip).nexti).d.dn).sub.val.comtype as libc::c_uint
            == EOL_COMMENT as libc::c_int as libc::c_uint
    {
        fprintf(prof_fp, b"\t\0" as *const u8 as *const libc::c_char);
        print_comment((*ip).nexti, -(1 as libc::c_int) as libc::c_long);
        ret = (*ip).nexti;
    } else {
        fprintf(prof_fp, b"\n\0" as *const u8 as *const libc::c_char);
    }
    return ret;
}
pub unsafe extern "C" fn pp_string_fp(
    mut print_func: Func_print,
    mut fp: *mut FILE,
    mut in_str: *const libc::c_char,
    mut len: size_t,
    mut delim: libc::c_int,
    mut breaklines: bool,
) {
    let mut s: *mut libc::c_char = pp_string(in_str, len, delim);
    let mut count: libc::c_int = 0;
    let mut slen: size_t = 0;
    let mut str: *const libc::c_char = s as *const libc::c_char;
    slen = strlen(str);
    count = 0 as libc::c_int;
    while slen > 0 as libc::c_int as libc::c_ulong {
        print_func
            .unwrap()(
            fp,
            b"%c\0" as *const u8 as *const libc::c_char,
            *str as libc::c_int,
        );
        count += 1;
        if count >= 70 as libc::c_int && breaklines as libc::c_int != 0 {
            print_func
                .unwrap()(
                fp,
                b"%c\n%c\0" as *const u8 as *const libc::c_char,
                delim,
                delim,
            );
            count = 0 as libc::c_int;
        }
        slen = slen.wrapping_sub(1);
        slen;
        str = str.offset(1);
        str;
    }
    free(s as *mut libc::c_void);
}
unsafe extern "C" fn just_dump(mut signum: libc::c_int) {
    extern "C" {
        static mut code_block: *mut INSTRUCTION;
    }
    dump_prog(code_block);
    dump_funcs();
    dump_fcall_stack(prof_fp);
    fflush(prof_fp);
    signal(signum, Some(just_dump as unsafe extern "C" fn(libc::c_int) -> ()));
}
unsafe extern "C" fn dump_and_exit(mut signum: libc::c_int) -> ! {
    just_dump(signum);
    final_exit(1 as libc::c_int);
}
unsafe extern "C" fn print_lib_list(mut prof_fp_0: *mut FILE) {
    let mut s: *mut SRCFILE = 0 as *mut SRCFILE;
    static mut printed_header: bool = 0 as libc::c_int != 0;
    let mut indent_0: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
    let mut found: bool = 0 as libc::c_int != 0;
    if do_flags as libc::c_uint & DO_PROFILE as libc::c_int as libc::c_uint != 0 {
        indent_0 = b"\t\0" as *const u8 as *const libc::c_char;
    }
    s = (*srcfiles).next;
    while s != srcfiles {
        if (*s).stype as libc::c_uint == SRC_EXTLIB as libc::c_int as libc::c_uint {
            if do_flags as libc::c_uint & DO_PROFILE as libc::c_int as libc::c_uint != 0
                && !printed_header
            {
                printed_header = 1 as libc::c_int != 0;
                fprintf(
                    prof_fp_0,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s# Loaded extensions (-l and/or @load)\n\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    indent_0,
                );
            }
            found = 1 as libc::c_int != 0;
            fprintf(
                prof_fp_0,
                b"%s@load \"%s\"\0" as *const u8 as *const libc::c_char,
                indent_0,
                (*s).src,
            );
            if !((*s).comment).is_null() {
                fprintf(prof_fp_0, b"\t\0" as *const u8 as *const libc::c_char);
                print_comment(
                    (*s).comment,
                    indent_level + 1 as libc::c_int as libc::c_long,
                );
            } else {
                fprintf(prof_fp_0, b"\n\0" as *const u8 as *const libc::c_char);
            }
        }
        s = (*s).next;
    }
    if found {
        fprintf(prof_fp_0, b"\n\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn print_include_list(mut prof_fp_0: *mut FILE) {
    let mut s: *mut SRCFILE = 0 as *mut SRCFILE;
    static mut printed_header: bool = 0 as libc::c_int != 0;
    let mut found: bool = 0 as libc::c_int != 0;
    if do_flags as libc::c_uint & DO_PROFILE as libc::c_int as libc::c_uint != 0 {
        return;
    }
    s = (*srcfiles).next;
    while s != srcfiles {
        if (*s).stype as libc::c_uint == SRC_INC as libc::c_int as libc::c_uint {
            if !printed_header {
                printed_header = 1 as libc::c_int != 0;
                fprintf(
                    prof_fp_0,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"\n# Included files (-i and/or @include)\n\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            found = 1 as libc::c_int != 0;
            fprintf(
                prof_fp_0,
                b"# @include \"%s\"\0" as *const u8 as *const libc::c_char,
                (*s).src,
            );
            if !((*s).comment).is_null() {
                fprintf(prof_fp_0, b"\t\0" as *const u8 as *const libc::c_char);
                print_comment(
                    (*s).comment,
                    indent_level + 1 as libc::c_int as libc::c_long,
                );
            } else {
                fprintf(prof_fp_0, b"\n\0" as *const u8 as *const libc::c_char);
            }
        }
        s = (*s).next;
    }
    if found {
        fprintf(prof_fp_0, b"\n\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn print_comment(mut pc: *mut INSTRUCTION, mut in_0: libc::c_long) {
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut count: size_t = 0;
    let mut after_newline: bool = 0 as libc::c_int != 0;
    count = (*(*pc).d.dn).sub.val.slen;
    text = (*(*pc).d.dn).sub.val.sp;
    if in_0 >= 0 as libc::c_int as libc::c_long {
        indent(in_0 as exec_count_t);
    }
    while count > 0 as libc::c_int as libc::c_ulong {
        if after_newline {
            indent(in_0 as exec_count_t);
            after_newline = 0 as libc::c_int != 0;
        }
        putc(*text as libc::c_int, prof_fp);
        after_newline = *text as libc::c_int == '\n' as i32;
        count = count.wrapping_sub(1);
        count;
        text = text.offset(1);
        text;
    }
    if !((*pc).comment).is_null() {
        if (*(*pc).d.dn).sub.val.comtype as libc::c_uint
            == EOL_COMMENT as libc::c_int as libc::c_uint
        {
            in_0 += 1;
            in_0;
        }
        print_comment((*pc).comment, in_0);
    }
}
pub unsafe extern "C" fn dump_prog(mut code: *mut INSTRUCTION) {
    let mut now: time_t = 0;
    time(&mut now);
    if do_flags as libc::c_uint & DO_PROFILE as libc::c_int as libc::c_uint != 0 {
        fprintf(
            prof_fp,
            dcgettext(
                0 as *const libc::c_char,
                b"\t# gawk profile, created %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            ctime(&mut now),
        );
    }
    print_lib_list(prof_fp);
    pprint(code, 0 as *mut INSTRUCTION, 0 as libc::c_int);
    print_include_list(prof_fp);
}
unsafe extern "C" fn prec_level(mut type_0: libc::c_int) -> libc::c_int {
    match type_0 {
        82 | 81 | 80 | 75 | 78 | 79 | 49 | 16 | 83 | 73 | 64 | 69 => {
            return 16 as libc::c_int;
        }
        24 | 84 => return 15 as libc::c_int,
        18 | 19 | 20 | 21 => return 14 as libc::c_int,
        11 | 12 => return 13 as libc::c_int,
        22 | 23 | 25 => return 12 as libc::c_int,
        1 | 2 | 3 | 4 | 5 | 6 => return 11 as libc::c_int,
        7 | 8 | 9 | 10 => return 10 as libc::c_int,
        13 | 37 => return 9 as libc::c_int,
        42 | 43 | 45 | 44 | 46 | 47 => return 8 as libc::c_int,
        48 | 50 => return 7 as libc::c_int,
        66 | 65 => return 6 as libc::c_int,
        72 => return 5 as libc::c_int,
        38 => return 4 as libc::c_int,
        40 => return 3 as libc::c_int,
        120 => return 2 as libc::c_int,
        26 | 31 | 32 | 33 | 34 | 35 | 36 => return 1 as libc::c_int,
        _ => return 0 as libc::c_int,
    };
}
unsafe extern "C" fn is_scalar(mut type_0: libc::c_int) -> bool {
    match type_0 {
        82 | 81 | 80 | 75 | 78 | 79 | 16 | 83 | 73 | 69 | 24 | 84 | 18 | 19 | 20 | 21
        | 22 | 23 | 25 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
unsafe extern "C" fn is_binary(mut type_0: libc::c_int) -> bool {
    match type_0 {
        47 | 46 | 45 | 44 | 43 | 42 | 11 | 1 | 3 | 5 | 7 | 9 | 12 | 2 | 4 | 6 | 8 | 10
        | 13 | 37 | 48 | 50 | 26 | 31 | 32 | 33 | 34 | 35 | 36 | 120 | 38 | 40 | 72 | 65
        | 66 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
unsafe extern "C" fn pp_parenthesize(mut sp: *mut NODE) {
    let mut p: *mut libc::c_char = (*sp).sub.nodep.name;
    let mut len: size_t = (*sp).sub.nodep.reserved;
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == '(' as i32 {
        return;
    }
    p = emalloc_real(
        len.wrapping_add(3 as libc::c_int as libc::c_ulong),
        b"pp_parenthesize\0" as *const u8 as *const libc::c_char,
        b"p\0" as *const u8 as *const libc::c_char,
        b"profile.c\0" as *const u8 as *const libc::c_char,
        1619 as libc::c_int,
    ) as *mut libc::c_char;
    *p = '(' as i32 as libc::c_char;
    memcpy(
        p.offset(1 as libc::c_int as isize) as *mut libc::c_void,
        (*sp).sub.nodep.name as *const libc::c_void,
        len,
    );
    *p
        .offset(
            len.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
        ) = ')' as i32 as libc::c_char;
    *p
        .offset(
            len.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
        ) = '\0' as i32 as libc::c_char;
    if (*sp).flags as libc::c_uint & 2 as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        free((*sp).sub.nodep.name as *mut libc::c_void);
    }
    (*sp).sub.nodep.name = p;
    (*sp)
        .sub
        .nodep
        .reserved = ((*sp).sub.nodep.reserved as libc::c_ulong)
        .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
    (*sp)
        .flags = ::std::mem::transmute::<
        libc::c_uint,
        flagvals,
    >((*sp).flags as libc::c_uint | 2 as libc::c_int as libc::c_uint);
}
unsafe extern "C" fn parenthesize(
    mut type_0: libc::c_int,
    mut left: *mut NODE,
    mut right: *mut NODE,
) {
    let mut rprec: libc::c_int = prec_level((*right).type_0 as libc::c_int);
    let mut lprec: libc::c_int = prec_level((*left).type_0 as libc::c_int);
    let mut prec: libc::c_int = prec_level(type_0);
    if lprec < prec {
        pp_parenthesize(left);
    }
    if rprec < prec {
        pp_parenthesize(right);
    }
}
pub unsafe extern "C" fn pp_string(
    mut in_str: *const libc::c_char,
    mut len: size_t,
    mut delim: libc::c_int,
) -> *mut libc::c_char {
    return pp_string_or_typed_regex(in_str, len, delim, 0 as libc::c_int != 0);
}
unsafe extern "C" fn pp_typed_regex(
    mut in_str: *const libc::c_char,
    mut len: size_t,
    mut delim: libc::c_int,
) -> *mut libc::c_char {
    return pp_string_or_typed_regex(in_str, len, delim, 1 as libc::c_int != 0);
}
unsafe extern "C" fn pp_string_or_typed_regex(
    mut in_str: *const libc::c_char,
    mut len: size_t,
    mut delim: libc::c_int,
    mut typed_regex: bool,
) -> *mut libc::c_char {
    static mut str_escapes: [libc::c_char; 9] = unsafe {
        *::std::mem::transmute::<
            &[u8; 9],
            &mut [libc::c_char; 9],
        >(b"\x07\x08\x0C\n\r\t\x0B\\\0")
    };
    static mut str_printables: [libc::c_char; 9] = unsafe {
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"abfnrtv\\\0")
    };
    static mut re_escapes: [libc::c_char; 8] = unsafe {
        *::std::mem::transmute::<
            &[u8; 8],
            &mut [libc::c_char; 8],
        >(b"\x07\x08\x0C\n\r\t\x0B\0")
    };
    static mut re_printables: [libc::c_char; 8] = unsafe {
        *::std::mem::transmute::<&[u8; 8], &mut [libc::c_char; 8]>(b"abfnrtv\0")
    };
    let mut escapes: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut printables: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut str: *const libc::c_uchar = in_str as *const libc::c_uchar;
    let mut ofre: size_t = 0;
    let mut osiz: size_t = 0;
    let mut obuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut obufout: *mut libc::c_char = 0 as *mut libc::c_char;
    if delim == '/' as i32 {
        escapes = re_escapes.as_mut_ptr();
        printables = re_printables.as_mut_ptr();
    } else {
        escapes = str_escapes.as_mut_ptr();
        printables = str_printables.as_mut_ptr();
    }
    osiz = len
        .wrapping_add(3 as libc::c_int as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_add(
            (typed_regex as libc::c_int == 1 as libc::c_int) as libc::c_int
                as libc::c_ulong,
        );
    obuf = emalloc_real(
        osiz,
        b"pp_string\0" as *const u8 as *const libc::c_char,
        b"obuf\0" as *const u8 as *const libc::c_char,
        b"profile.c\0" as *const u8 as *const libc::c_char,
        1700 as libc::c_int,
    ) as *mut libc::c_char;
    obufout = obuf;
    ofre = osiz.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    if typed_regex {
        let fresh3 = obufout;
        obufout = obufout.offset(1);
        *fresh3 = '@' as i32 as libc::c_char;
    }
    let fresh4 = obufout;
    obufout = obufout.offset(1);
    *fresh4 = delim as libc::c_char;
    while len > 0 as libc::c_int as libc::c_ulong {
        if 2 as libc::c_int as libc::c_ulong > ofre {
            let mut olen: libc::c_long = obufout.offset_from(obuf) as libc::c_long;
            obuf = erealloc_real(
                obuf as *mut libc::c_void,
                osiz.wrapping_mul(2 as libc::c_int as libc::c_ulong),
                b"pp_string\0" as *const u8 as *const libc::c_char,
                b"obuf\0" as *const u8 as *const libc::c_char,
                b"profile.c\0" as *const u8 as *const libc::c_char,
                1709 as libc::c_int,
            ) as *mut libc::c_char;
            obufout = obuf.offset(olen as isize);
            ofre = (ofre as libc::c_ulong).wrapping_add(osiz) as size_t as size_t;
            osiz = (osiz as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        }
        ofre = (ofre as libc::c_ulong).wrapping_sub(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
        if delim != '/' as i32 && *str as libc::c_int == delim {
            let fresh5 = obufout;
            obufout = obufout.offset(1);
            *fresh5 = '\\' as i32 as libc::c_char;
            let fresh6 = obufout;
            obufout = obufout.offset(1);
            *fresh6 = delim as libc::c_char;
        } else if *str as libc::c_int == '\0' as i32 {
            let fresh7 = obufout;
            obufout = obufout.offset(1);
            *fresh7 = '\\' as i32 as libc::c_char;
            let fresh8 = obufout;
            obufout = obufout.offset(1);
            *fresh8 = '0' as i32 as libc::c_char;
            if 2 as libc::c_int as libc::c_ulong > ofre {
                let mut olen_0: libc::c_long = obufout.offset_from(obuf) as libc::c_long;
                obuf = erealloc_real(
                    obuf as *mut libc::c_void,
                    osiz.wrapping_mul(2 as libc::c_int as libc::c_ulong),
                    b"pp_string\0" as *const u8 as *const libc::c_char,
                    b"obuf\0" as *const u8 as *const libc::c_char,
                    b"profile.c\0" as *const u8 as *const libc::c_char,
                    1716 as libc::c_int,
                ) as *mut libc::c_char;
                obufout = obuf.offset(olen_0 as isize);
                ofre = (ofre as libc::c_ulong).wrapping_add(osiz) as size_t as size_t;
                osiz = (osiz as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
            }
            ofre = (ofre as libc::c_ulong)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
            let fresh9 = obufout;
            obufout = obufout.offset(1);
            *fresh9 = '0' as i32 as libc::c_char;
            let fresh10 = obufout;
            obufout = obufout.offset(1);
            *fresh10 = '0' as i32 as libc::c_char;
        } else {
            cp = strchr(escapes, *str as libc::c_int);
            if !cp.is_null() {
                i = cp.offset_from(escapes) as libc::c_long as libc::c_int;
                let fresh11 = obufout;
                obufout = obufout.offset(1);
                *fresh11 = '\\' as i32 as libc::c_char;
                let fresh12 = obufout;
                obufout = obufout.offset(1);
                *fresh12 = *printables.offset(i as isize);
            } else if *str as libc::c_int & !(0x7f as libc::c_int) == 0 as libc::c_int
                && *(*__ctype_b_loc()).offset(*str as libc::c_int as isize)
                    as libc::c_int
                    & _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                let fresh13 = obufout;
                obufout = obufout.offset(1);
                *fresh13 = *str as libc::c_char;
                ofre = (ofre as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t as size_t;
            } else {
                let mut len_0: size_t = 0;
                if 8 as libc::c_int as libc::c_ulong > ofre {
                    let mut olen_1: libc::c_long = obufout.offset_from(obuf)
                        as libc::c_long;
                    obuf = erealloc_real(
                        obuf as *mut libc::c_void,
                        osiz.wrapping_mul(2 as libc::c_int as libc::c_ulong),
                        b"pp_string\0" as *const u8 as *const libc::c_char,
                        b"obuf\0" as *const u8 as *const libc::c_char,
                        b"profile.c\0" as *const u8 as *const libc::c_char,
                        1730 as libc::c_int,
                    ) as *mut libc::c_char;
                    obufout = obuf.offset(olen_1 as isize);
                    ofre = (ofre as libc::c_ulong).wrapping_add(osiz) as size_t
                        as size_t;
                    osiz = (osiz as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                }
                ofre = (ofre as libc::c_ulong)
                    .wrapping_sub(8 as libc::c_int as libc::c_ulong) as size_t as size_t;
                sprintf(
                    obufout,
                    b"\\%03o\0" as *const u8 as *const libc::c_char,
                    *str as libc::c_int & 0xff as libc::c_int,
                );
                len_0 = strlen(obufout);
                ofre = (ofre as libc::c_ulong)
                    .wrapping_add(
                        (10 as libc::c_int as libc::c_ulong).wrapping_sub(len_0),
                    ) as size_t as size_t;
                obufout = obufout.offset(len_0 as isize);
            }
        }
        len = len.wrapping_sub(1);
        len;
        str = str.offset(1);
        str;
    }
    if 2 as libc::c_int as libc::c_ulong > ofre {
        let mut olen_2: libc::c_long = obufout.offset_from(obuf) as libc::c_long;
        obuf = erealloc_real(
            obuf as *mut libc::c_void,
            osiz.wrapping_mul(2 as libc::c_int as libc::c_ulong),
            b"pp_string\0" as *const u8 as *const libc::c_char,
            b"obuf\0" as *const u8 as *const libc::c_char,
            b"profile.c\0" as *const u8 as *const libc::c_char,
            1738 as libc::c_int,
        ) as *mut libc::c_char;
        obufout = obuf.offset(olen_2 as isize);
        ofre = (ofre as libc::c_ulong).wrapping_add(osiz) as size_t as size_t;
        osiz = (osiz as libc::c_ulong).wrapping_mul(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    ofre = (ofre as libc::c_ulong).wrapping_sub(2 as libc::c_int as libc::c_ulong)
        as size_t as size_t;
    let fresh14 = obufout;
    obufout = obufout.offset(1);
    *fresh14 = delim as libc::c_char;
    *obufout = '\0' as i32 as libc::c_char;
    return obuf;
}
pub unsafe extern "C" fn pp_number(mut n: *mut NODE) -> *mut libc::c_char {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    str = emalloc_real(
        ((*n).sub.val.slen).wrapping_add(1 as libc::c_int as libc::c_ulong),
        b"pp_number\0" as *const u8 as *const libc::c_char,
        b"str\0" as *const u8 as *const libc::c_char,
        b"profile.c\0" as *const u8 as *const libc::c_char,
        1753 as libc::c_int,
    ) as *mut libc::c_char;
    strcpy(str, (*n).sub.val.sp);
    return str;
}
pub unsafe extern "C" fn pp_node(mut n: *mut NODE) -> *mut libc::c_char {
    if (*n).flags as libc::c_uint & NUMBER as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        return pp_number(n);
    }
    return pp_string((*n).sub.val.sp, (*n).sub.val.slen, '"' as i32);
}
static mut pp_args: *mut *mut NODE = 0 as *const *mut NODE as *mut *mut NODE;
static mut npp_args: libc::c_int = 0;
unsafe extern "C" fn pp_list(
    mut nargs: libc::c_int,
    mut paren: *const libc::c_char,
    mut delim: *const libc::c_char,
) -> *mut libc::c_char {
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut delimlen: size_t = 0;
    let mut i: libc::c_int = 0;
    let mut comment: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    if pp_args.is_null() {
        npp_args = nargs;
        pp_args = emalloc_real(
            ((nargs + 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut NODE>() as libc::c_ulong),
            b"pp_list\0" as *const u8 as *const libc::c_char,
            b"pp_args\0" as *const u8 as *const libc::c_char,
            b"profile.c\0" as *const u8 as *const libc::c_char,
            1785 as libc::c_int,
        ) as *mut *mut NODE;
    } else if nargs > npp_args {
        npp_args = nargs;
        pp_args = erealloc_real(
            pp_args as *mut libc::c_void,
            ((nargs + 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut NODE>() as libc::c_ulong),
            b"pp_list\0" as *const u8 as *const libc::c_char,
            b"pp_args\0" as *const u8 as *const libc::c_char,
            b"profile.c\0" as *const u8 as *const libc::c_char,
            1788 as libc::c_int,
        ) as *mut *mut NODE;
    }
    delimlen = strlen(delim);
    if nargs == 0 as libc::c_int {
        len = 2 as libc::c_int as size_t;
    } else {
        len = delimlen.wrapping_neg();
        i = 1 as libc::c_int;
        while i <= nargs {
            let ref mut fresh15 = *pp_args.offset(i as isize);
            *fresh15 = pp_pop();
            r = *fresh15;
            len = (len as libc::c_ulong)
                .wrapping_add(((*r).sub.nodep.reserved).wrapping_add(delimlen)) as size_t
                as size_t;
            if !((*r).sub.nodep.x.cmnt).is_null() {
                comment = (*r).sub.nodep.x.cmnt as *mut INSTRUCTION;
                len = (len as libc::c_ulong)
                    .wrapping_add(
                        ((*(*comment).d.dn).sub.val.slen)
                            .wrapping_add(indent_level as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as size_t as size_t;
            }
            i += 1;
            i;
        }
        if !paren.is_null() {
            len = (len as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
        }
    }
    comment = 0 as *mut INSTRUCTION;
    str = emalloc_real(
        len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        b"pp_list\0" as *const u8 as *const libc::c_char,
        b"str\0" as *const u8 as *const libc::c_char,
        b"profile.c\0" as *const u8 as *const libc::c_char,
        1811 as libc::c_int,
    ) as *mut libc::c_char;
    s = str;
    if !paren.is_null() {
        let fresh16 = s;
        s = s.offset(1);
        *fresh16 = *paren.offset(0 as libc::c_int as isize);
    }
    i = nargs;
    while i > 0 as libc::c_int {
        r = *pp_args.offset(i as isize);
        memcpy(
            s as *mut libc::c_void,
            (*r).sub.nodep.name as *const libc::c_void,
            (*r).sub.nodep.reserved,
        );
        s = s.offset((*r).sub.nodep.reserved as isize);
        if i > 1 as libc::c_int && delimlen > 0 as libc::c_int as libc::c_ulong {
            memcpy(s as *mut libc::c_void, delim as *const libc::c_void, delimlen);
            s = s.offset(delimlen as isize);
        }
        if !((*r).sub.nodep.x.cmnt).is_null() {
            if (indent_level + 1 as libc::c_int as libc::c_long) as libc::c_ulong
                > tabs_len
            {
                (set_loc
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> ())(
                    b"profile.c\0" as *const u8 as *const libc::c_char,
                    1830 as libc::c_int,
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
                        b"Program indentation level too deep. Consider refactoring your code\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            comment = (*r).sub.nodep.x.cmnt as *mut INSTRUCTION;
            memcpy(
                s as *mut libc::c_void,
                (*(*comment).d.dn).sub.val.sp as *const libc::c_void,
                (*(*comment).d.dn).sub.val.slen,
            );
            s = s.offset((*(*comment).d.dn).sub.val.slen as isize);
            memcpy(
                s as *mut libc::c_void,
                tabs.as_ptr() as *const libc::c_void,
                (indent_level + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
            );
            s = s.offset((indent_level + 1 as libc::c_int as libc::c_long) as isize);
        }
        pp_free(r);
        i -= 1;
        i;
    }
    if !paren.is_null() {
        let fresh17 = s;
        s = s.offset(1);
        *fresh17 = *paren.offset(1 as libc::c_int as isize);
    }
    *s = '\0' as i32 as libc::c_char;
    return str;
}
unsafe extern "C" fn is_unary_minus(mut str: *const libc::c_char) -> bool {
    return *str.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
        && *str.offset(1 as libc::c_int as isize) as libc::c_int != '-' as i32;
}
unsafe extern "C" fn pp_concat(mut nargs: libc::c_int) -> *mut libc::c_char {
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    static mut delimlen: size_t = 1 as libc::c_int as size_t;
    let mut i: libc::c_int = 0;
    let mut pl_l: libc::c_int = 0;
    let mut pl_r: libc::c_int = 0;
    if pp_args.is_null() {
        npp_args = nargs;
        pp_args = emalloc_real(
            ((nargs + 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut NODE>() as libc::c_ulong),
            b"pp_concat\0" as *const u8 as *const libc::c_char,
            b"pp_args\0" as *const u8 as *const libc::c_char,
            b"profile.c\0" as *const u8 as *const libc::c_char,
            1868 as libc::c_int,
        ) as *mut *mut NODE;
    } else if nargs > npp_args {
        npp_args = nargs;
        pp_args = erealloc_real(
            pp_args as *mut libc::c_void,
            ((nargs + 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut NODE>() as libc::c_ulong),
            b"pp_concat\0" as *const u8 as *const libc::c_char,
            b"pp_args\0" as *const u8 as *const libc::c_char,
            b"profile.c\0" as *const u8 as *const libc::c_char,
            1871 as libc::c_int,
        ) as *mut *mut NODE;
    }
    len = delimlen.wrapping_neg();
    i = nargs;
    while i >= 1 as libc::c_int {
        let ref mut fresh18 = *pp_args.offset(i as isize);
        *fresh18 = pp_pop();
        r = *fresh18;
        len = (len as libc::c_ulong)
            .wrapping_add(
                ((*r).sub.nodep.reserved)
                    .wrapping_add(delimlen)
                    .wrapping_add(2 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
        i -= 1;
        i;
    }
    str = emalloc_real(
        len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        b"pp_concat\0" as *const u8 as *const libc::c_char,
        b"str\0" as *const u8 as *const libc::c_char,
        b"profile.c\0" as *const u8 as *const libc::c_char,
        1885 as libc::c_int,
    ) as *mut libc::c_char;
    s = str;
    i = 1 as libc::c_int;
    while i < nargs {
        r = *pp_args.offset(i as isize);
        if *((*r).sub.nodep.name).offset(0 as libc::c_int as isize) as libc::c_int
            != '(' as i32
        {
            pl_l = prec_level((**pp_args.offset(i as isize)).type_0 as libc::c_int);
            pl_r = prec_level(
                (**pp_args.offset((i + 1 as libc::c_int) as isize)).type_0 as libc::c_int,
            );
            if i >= 2 as libc::c_int
                && is_unary_minus((*r).sub.nodep.name) as libc::c_int != 0
            {
                let fresh19 = s;
                s = s.offset(1);
                *fresh19 = '(' as i32 as libc::c_char;
                memcpy(
                    s as *mut libc::c_void,
                    (*r).sub.nodep.name as *const libc::c_void,
                    (*r).sub.nodep.reserved,
                );
                s = s.offset((*r).sub.nodep.reserved as isize);
                let fresh20 = s;
                s = s.offset(1);
                *fresh20 = ')' as i32 as libc::c_char;
            } else if is_scalar((**pp_args.offset(i as isize)).type_0 as libc::c_int)
                as libc::c_int != 0
                && is_scalar(
                    (**pp_args.offset((i + 1 as libc::c_int) as isize)).type_0
                        as libc::c_int,
                ) as libc::c_int != 0
            {
                memcpy(
                    s as *mut libc::c_void,
                    (*r).sub.nodep.name as *const libc::c_void,
                    (*r).sub.nodep.reserved,
                );
                s = s.offset((*r).sub.nodep.reserved as isize);
            } else if pl_l <= pl_r
                || is_scalar(
                    (**pp_args.offset((i + 1 as libc::c_int) as isize)).type_0
                        as libc::c_int,
                ) as libc::c_int != 0
            {
                let fresh21 = s;
                s = s.offset(1);
                *fresh21 = '(' as i32 as libc::c_char;
                memcpy(
                    s as *mut libc::c_void,
                    (*r).sub.nodep.name as *const libc::c_void,
                    (*r).sub.nodep.reserved,
                );
                s = s.offset((*r).sub.nodep.reserved as isize);
                let fresh22 = s;
                s = s.offset(1);
                *fresh22 = ')' as i32 as libc::c_char;
            } else {
                memcpy(
                    s as *mut libc::c_void,
                    (*r).sub.nodep.name as *const libc::c_void,
                    (*r).sub.nodep.reserved,
                );
                s = s.offset((*r).sub.nodep.reserved as isize);
            }
        } else {
            memcpy(
                s as *mut libc::c_void,
                (*r).sub.nodep.name as *const libc::c_void,
                (*r).sub.nodep.reserved,
            );
            s = s.offset((*r).sub.nodep.reserved as isize);
        }
        if i < nargs {
            let fresh23 = s;
            s = s.offset(1);
            *fresh23 = ' ' as i32 as libc::c_char;
        }
        i += 1;
        i;
    }
    pl_l = prec_level(
        (**pp_args.offset((nargs - 1 as libc::c_int) as isize)).type_0 as libc::c_int,
    );
    pl_r = prec_level((**pp_args.offset(nargs as isize)).type_0 as libc::c_int);
    r = *pp_args.offset(nargs as isize);
    if *((*r).sub.nodep.name).offset(0 as libc::c_int as isize) as libc::c_int
        == '(' as i32
    {
        memcpy(
            s as *mut libc::c_void,
            (*r).sub.nodep.name as *const libc::c_void,
            (*r).sub.nodep.reserved,
        );
        s = s.offset((*r).sub.nodep.reserved as isize);
    } else if is_unary_minus((*r).sub.nodep.name) as libc::c_int != 0
        || pl_l >= pl_r
            && !is_scalar((**pp_args.offset(nargs as isize)).type_0 as libc::c_int)
    {
        let fresh24 = s;
        s = s.offset(1);
        *fresh24 = '(' as i32 as libc::c_char;
        memcpy(
            s as *mut libc::c_void,
            (*r).sub.nodep.name as *const libc::c_void,
            (*r).sub.nodep.reserved,
        );
        s = s.offset((*r).sub.nodep.reserved as isize);
        let fresh25 = s;
        s = s.offset(1);
        *fresh25 = ')' as i32 as libc::c_char;
    } else {
        memcpy(
            s as *mut libc::c_void,
            (*r).sub.nodep.name as *const libc::c_void,
            (*r).sub.nodep.reserved,
        );
        s = s.offset((*r).sub.nodep.reserved as isize);
    }
    i = nargs;
    while i >= 1 as libc::c_int {
        pp_free(*pp_args.offset(i as isize));
        i -= 1;
        i;
    }
    *s = '\0' as i32 as libc::c_char;
    return str;
}
unsafe extern "C" fn pp_group3(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
    mut s3: *const libc::c_char,
) -> *mut libc::c_char {
    let mut len1: size_t = 0;
    let mut len2: size_t = 0;
    let mut len3: size_t = 0;
    let mut l: size_t = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    len1 = strlen(s1);
    len2 = strlen(s2);
    len3 = strlen(s3);
    l = len1
        .wrapping_add(len2)
        .wrapping_add(len3)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    str = emalloc_real(
        l,
        b"pp_group3\0" as *const u8 as *const libc::c_char,
        b"str\0" as *const u8 as *const libc::c_char,
        b"profile.c\0" as *const u8 as *const libc::c_char,
        1959 as libc::c_int,
    ) as *mut libc::c_char;
    s = str;
    if len1 > 0 as libc::c_int as libc::c_ulong {
        memcpy(s as *mut libc::c_void, s1 as *const libc::c_void, len1);
        s = s.offset(len1 as isize);
    }
    if len2 > 0 as libc::c_int as libc::c_ulong {
        memcpy(s as *mut libc::c_void, s2 as *const libc::c_void, len2);
        s = s.offset(len2 as isize);
    }
    if len3 > 0 as libc::c_int as libc::c_ulong {
        memcpy(s as *mut libc::c_void, s3 as *const libc::c_void, len3);
        s = s.offset(len3 as isize);
    }
    *s = '\0' as i32 as libc::c_char;
    return str;
}
pub unsafe extern "C" fn pp_func(
    mut pc: *mut INSTRUCTION,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    static mut first: bool = 1 as libc::c_int != 0;
    let mut func: *mut NODE = 0 as *mut NODE;
    let mut pcount: libc::c_int = 0;
    let mut fp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    if first {
        first = 0 as libc::c_int != 0;
        if do_flags as libc::c_uint & DO_PROFILE as libc::c_int as libc::c_uint != 0 {
            fprintf(
                prof_fp,
                dcgettext(
                    0 as *const libc::c_char,
                    b"\n\t# Functions, listed alphabetically\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
    }
    pp_namespace_list((*pc.offset(3 as libc::c_int as isize)).nexti);
    fp = (*(*pc).nexti).nexti;
    func = (*pc).x.xn;
    fprintf(prof_fp, b"\n\0" as *const u8 as *const libc::c_char);
    if !((*pc).comment).is_null() {
        print_comment((*pc).comment, -(1 as libc::c_int) as libc::c_long);
    }
    indent((*(*pc).nexti).d.ldl);
    let mut malloced: bool = 0 as libc::c_int != 0;
    let mut name: *mut libc::c_char = adjust_namespace(
        (*func).sub.nodep.name,
        &mut malloced,
    );
    fprintf(
        prof_fp,
        b"%s %s(\0" as *const u8 as *const libc::c_char,
        op2str(Op_K_function),
        name,
    );
    if malloced {
        free(name as *mut libc::c_void);
    }
    pcount = (*func).sub.nodep.l.ll as libc::c_int;
    func_params = (*func).sub.nodep.rn;
    j = 0 as libc::c_int;
    while j < pcount {
        fprintf(
            prof_fp,
            b"%s\0" as *const u8 as *const libc::c_char,
            (*func_params.offset(j as isize)).sub.nodep.name,
        );
        if j < pcount - 1 as libc::c_int {
            fprintf(prof_fp, b", \0" as *const u8 as *const libc::c_char);
        }
        j += 1;
        j;
    }
    if (*fp).opcode as libc::c_uint == Op_comment as libc::c_int as libc::c_uint
        && (*(*fp).d.dn).sub.val.comtype as libc::c_uint
            == EOL_COMMENT as libc::c_int as libc::c_uint
    {
        fprintf(prof_fp, b")\0" as *const u8 as *const libc::c_char);
        fp = end_line(fp);
    } else {
        fprintf(prof_fp, b")\n\0" as *const u8 as *const libc::c_char);
    }
    if do_flags as libc::c_uint & DO_PROFILE as libc::c_int as libc::c_uint != 0 {
        indent(0 as libc::c_int as exec_count_t);
    }
    fprintf(prof_fp, b"{\n\0" as *const u8 as *const libc::c_char);
    indent_in();
    pprint(fp, 0 as *mut INSTRUCTION, 0 as libc::c_int);
    indent_out();
    if do_flags as libc::c_uint & DO_PROFILE as libc::c_int as libc::c_uint != 0 {
        indent(0 as libc::c_int as exec_count_t);
    }
    fprintf(prof_fp, b"}\n\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn redir2str(mut redirtype: libc::c_int) -> *const libc::c_char {
    static mut redirtab: [*const libc::c_char; 7] = [
        b"\0" as *const u8 as *const libc::c_char,
        b" > \0" as *const u8 as *const libc::c_char,
        b" >> \0" as *const u8 as *const libc::c_char,
        b" | \0" as *const u8 as *const libc::c_char,
        b" | \0" as *const u8 as *const libc::c_char,
        b" < \0" as *const u8 as *const libc::c_char,
        b" |& \0" as *const u8 as *const libc::c_char,
    ];
    if redirtype < 0 as libc::c_int || redirtype > redirect_twoway as libc::c_int {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"profile.c\0" as *const u8 as *const libc::c_char,
            2052 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .unwrap(),
        ))
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"redir2str: unknown redirection type %d\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            redirtype,
        );
    }
    return redirtab[redirtype as usize];
}
unsafe extern "C" fn pp_namespace(
    mut name: *const libc::c_char,
    mut comment: *mut INSTRUCTION,
) {
    if !namespace_changed {
        return;
    }
    if strcmp(current_namespace, name) == 0 as libc::c_int {
        return;
    }
    current_namespace = name;
    fprintf(prof_fp, b"\n\0" as *const u8 as *const libc::c_char);
    if do_flags as libc::c_uint & DO_PROFILE as libc::c_int as libc::c_uint != 0 {
        indent(0 as libc::c_int as exec_count_t);
    }
    fprintf(prof_fp, b"@namespace \"%s\"\0" as *const u8 as *const libc::c_char, name);
    if !comment.is_null() {
        putc('\t' as i32, prof_fp);
        print_comment(comment, 0 as libc::c_int as libc::c_long);
        putc('\n' as i32, prof_fp);
    } else {
        fprintf(prof_fp, b"\n\n\0" as *const u8 as *const libc::c_char);
    };
}
unsafe extern "C" fn pp_namespace_list(mut list: *mut INSTRUCTION) {
    if list.is_null() {
        return;
    }
    pp_namespace_list((*list).nexti);
    pp_namespace((*list).d.name, (*list).comment);
}
unsafe extern "C" fn adjust_namespace(
    mut name: *mut libc::c_char,
    mut malloced: *mut bool,
) -> *mut libc::c_char {
    *malloced = 0 as libc::c_int != 0;
    if (strchr(name, ':' as i32)).is_null()
        && current_namespace != awk_namespace.as_ptr()
        && strcmp(current_namespace, awk_namespace.as_ptr()) != 0 as libc::c_int
        && !is_all_upper(name)
    {
        let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut len: size_t = (5 as libc::c_int as libc::c_ulong)
            .wrapping_add(strlen(name))
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        buf = emalloc_real(
            len,
            b"adjust_namespace\0" as *const u8 as *const libc::c_char,
            b"buf\0" as *const u8 as *const libc::c_char,
            b"profile.c\0" as *const u8 as *const libc::c_char,
            2116 as libc::c_int,
        ) as *mut libc::c_char;
        sprintf(buf, b"awk::%s\0" as *const u8 as *const libc::c_char, name);
        *malloced = 1 as libc::c_int != 0;
        return buf;
    }
    let mut len_0: size_t = strlen(current_namespace);
    if strncmp(current_namespace, name, len_0) == 0 as libc::c_int
        && *name.offset(len_0 as isize) as libc::c_int == ':' as i32
        && *name.offset(len_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int == ':' as i32
    {
        let mut ret: *mut libc::c_char = name
            .offset(len_0 as isize)
            .offset(2 as libc::c_int as isize);
        return ret;
    }
    return name;
}
unsafe extern "C" fn run_static_initializers() {
    tabs_len = (::std::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
