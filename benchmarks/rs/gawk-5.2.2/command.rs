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
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    fn variable_list() -> *mut *mut NODE;
    fn lookup(name: *const libc::c_char) -> *mut NODE;
    fn make_str_node(
        s: *const libc::c_char,
        len: size_t,
        flags: libc::c_int,
    ) -> *mut NODE;
    fn r_dupnode(n: *mut NODE) -> *mut NODE;
    fn r_fatal(mesg: *const libc::c_char, _: ...);
    fn set_loc(file: *const libc::c_char, line: libc::c_int);
    fn estrdup(str: *const libc::c_char, len: size_t) -> *mut libc::c_char;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    static mut CONVFMT: *const libc::c_char;
    static mut CONVFMTidx: libc::c_int;
    static mut SUBSEP_node: *mut NODE;
    static mut make_number: Option::<unsafe extern "C" fn(libc::c_double) -> *mut NODE>;
    static mut format_val: Option::<
        unsafe extern "C" fn(*const libc::c_char, libc::c_int, *mut NODE) -> *mut NODE,
    >;
    static mut nextfree: [block_header; 2];
    static mut srcfiles: *mut SRCFILE;
    static mut exit_val: libc::c_int;
    fn r_unref(tmp: *mut NODE);
    fn negate_num(n: *mut NODE);
    fn is_alpha(c: libc::c_int) -> bool;
    fn is_letter(c: libc::c_int) -> bool;
    fn is_identchar(c: libc::c_int) -> bool;
    fn rl_completion_matches(
        _: *const libc::c_char,
        _: Option::<rl_compentry_func_t>,
    ) -> *mut *mut libc::c_char;
    static mut rl_line_buffer: *mut libc::c_char;
    fn has_break_or_watch_point(pnum: *mut libc::c_int, any: bool) -> libc::c_int;
    fn pop_cmd_src() -> libc::c_int;
    fn get_eof_status() -> libc::c_int;
    fn in_cmd_src(_: *const libc::c_char) -> libc::c_int;
    fn read_commands_string(prompt: *const libc::c_char) -> *mut libc::c_char;
    static mut read_a_line: Option::<
        unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_char,
    >;
    fn find_option(name: *mut libc::c_char) -> libc::c_int;
    fn d_error(mesg: *const libc::c_char, _: ...);
    static mut dgawk_prompt: *const libc::c_char;
    static mut eval_prompt: *const libc::c_char;
    static mut commands_prompt: *const libc::c_char;
    static mut dbg_prompt: *const libc::c_char;
    static mut input_from_tty: bool;
    static mut input_fd: libc::c_int;
    fn do_breakpoint(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    fn do_clear(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    fn do_condition(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    fn do_continue(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    fn do_delete_breakpoint(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    fn do_disable_breakpoint(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    fn do_display(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    fn do_down(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    fn do_dump_instructions(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    fn do_enable_breakpoint(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    fn do_eval(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    fn do_finish(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    fn do_frame(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    fn initialize_pager(fp: *mut FILE);
    fn using_history();
    fn add_history(_: *const libc::c_char);
    fn previous_history() -> *mut HIST_ENTRY;
    fn history_expand(_: *mut libc::c_char, _: *mut *mut libc::c_char) -> libc::c_int;
    static mut pager_quit_tag: jmp_buf;
    fn get_function() -> *mut NODE;
    fn gprintf(fp: *mut FILE, format: *const libc::c_char, _: ...) -> libc::c_int;
    fn option_help();
    static mut out_fp: *mut FILE;
    fn do_ignore_breakpoint(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    fn do_info(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    fn do_list(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    fn do_next(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    fn do_backtrace(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    fn do_nexti(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    fn do_watch(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    fn do_option(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    fn do_up(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    fn do_print_var(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    fn do_unwatch(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    fn do_print_f(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    fn do_until(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    fn do_quit(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    fn do_undisplay(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    fn do_return(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    fn do_trace_instruction(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    fn do_run(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    fn do_tmp_breakpoint(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    fn do_save(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    fn do_stepi(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    fn do_set_var(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    fn do_step(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    fn do_commands(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    fn do_source(arg: *mut CMDARG, cmd: libc::c_int) -> libc::c_int;
    static mut rl_attempted_completion_over: libc::c_int;
    static mut rl_inhibit_completion: libc::c_int;
    fn option_generator(
        text: *const libc::c_char,
        state: libc::c_int,
    ) -> *mut libc::c_char;
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
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub type rl_compentry_func_t = unsafe extern "C" fn(
    *const libc::c_char,
    libc::c_int,
) -> *mut libc::c_char;
pub type histdata_t = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _hist_entry {
    pub line: *mut libc::c_char,
    pub timestamp: *mut libc::c_char,
    pub data: histdata_t,
}
pub type HIST_ENTRY = _hist_entry;
pub const D_printf: argtype = 25;
pub const D_illegal: argtype = 0;
pub const D_print: argtype = 24;
pub const D_watch: argtype = 41;
pub const D_display: argtype = 9;
pub const D_set: argtype = 30;
pub const D_option: argtype = 23;
pub type argtype = libc::c_uint;
pub const D_range: argtype = 51;
pub const D_func: argtype = 50;
pub const D_subscript: argtype = 49;
pub const D_array: argtype = 48;
pub const D_field: argtype = 47;
pub const D_node: argtype = 46;
pub const D_variable: argtype = 45;
pub const D_string: argtype = 44;
pub const D_int: argtype = 43;
pub const D_argument: argtype = 42;
pub const D_up: argtype = 40;
pub const D_unwatch: argtype = 39;
pub const D_until: argtype = 38;
pub const D_undisplay: argtype = 37;
pub const D_trace: argtype = 36;
pub const D_tbreak: argtype = 35;
pub const D_stepi: argtype = 34;
pub const D_step: argtype = 33;
pub const D_source: argtype = 32;
pub const D_silent: argtype = 31;
pub const D_save: argtype = 29;
pub const D_run: argtype = 28;
pub const D_return: argtype = 27;
pub const D_quit: argtype = 26;
pub const D_nexti: argtype = 22;
pub const D_next: argtype = 21;
pub const D_list: argtype = 20;
pub const D_info: argtype = 19;
pub const D_ignore: argtype = 18;
pub const D_help: argtype = 17;
pub const D_frame: argtype = 16;
pub const D_finish: argtype = 15;
pub const D_eval: argtype = 14;
pub const D_end: argtype = 13;
pub const D_enable: argtype = 12;
pub const D_dump: argtype = 11;
pub const D_down: argtype = 10;
pub const D_disable: argtype = 8;
pub const D_delete: argtype = 7;
pub const D_continue: argtype = 6;
pub const D_condition: argtype = 5;
pub const D_commands: argtype = 4;
pub const D_clear: argtype = 3;
pub const D_break: argtype = 2;
pub const D_backtrace: argtype = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argtoken {
    pub name: *const libc::c_char,
    pub cmd: argtype,
    pub value: nametypeval,
}
pub type nametypeval = libc::c_uint;
pub const A_WATCH: nametypeval = 14;
pub const A_VARIABLES: nametypeval = 13;
pub const A_TRACE_OFF: nametypeval = 12;
pub const A_TRACE_ON: nametypeval = 11;
pub const A_SOURCES: nametypeval = 10;
pub const A_SOURCE: nametypeval = 9;
pub const A_ONCE: nametypeval = 8;
pub const A_LOCALS: nametypeval = 7;
pub const A_FUNCTIONS: nametypeval = 6;
pub const A_FRAME: nametypeval = 5;
pub const A_DISPLAY: nametypeval = 4;
pub const A_DEL: nametypeval = 3;
pub const A_BREAK: nametypeval = 2;
pub const A_ARGS: nametypeval = 1;
pub const A_NONE: nametypeval = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmdtoken {
    pub name: *const libc::c_char,
    pub abbrvn: *const libc::c_char,
    pub type_0: argtype,
    pub lex_class: libc::c_int,
    pub cf_ptr: Func_cmd,
    pub help_txt: *const libc::c_char,
}
pub type Func_cmd = Option::<
    unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
>;
pub type CMDARG = cmd_argument;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd_argument {
    pub next: *mut cmd_argument,
    pub type_0: argtype,
    pub value: C2RustUnnamed_11,
    pub a_count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub lval: libc::c_long,
    pub sval: *mut libc::c_char,
    pub nodeval: *mut NODE,
}
pub type yy_state_t = yytype_uint8;
pub type yytype_uint8 = libc::c_uchar;
pub type yysymbol_kind_t = libc::c_int;
pub const YYSYMBOL_nls: yysymbol_kind_t = 113;
pub const YYSYMBOL_integer: yysymbol_kind_t = 112;
pub const YYSYMBOL_plus_integer: yysymbol_kind_t = 111;
pub const YYSYMBOL_opt_integer: yysymbol_kind_t = 110;
pub const YYSYMBOL_opt_plus_integer: yysymbol_kind_t = 109;
pub const YYSYMBOL_node: yysymbol_kind_t = 108;
pub const YYSYMBOL_variable: yysymbol_kind_t = 107;
pub const YYSYMBOL_subscript_list: yysymbol_kind_t = 106;
pub const YYSYMBOL_subscript: yysymbol_kind_t = 105;
pub const YYSYMBOL_exp_list: yysymbol_kind_t = 104;
pub const YYSYMBOL_integer_list: yysymbol_kind_t = 103;
pub const YYSYMBOL_opt_integer_list: yysymbol_kind_t = 102;
pub const YYSYMBOL_integer_range: yysymbol_kind_t = 101;
pub const YYSYMBOL_list_args: yysymbol_kind_t = 100;
pub const YYSYMBOL_printf_args: yysymbol_kind_t = 99;
pub const YYSYMBOL_printf_exp: yysymbol_kind_t = 98;
pub const YYSYMBOL_print_args: yysymbol_kind_t = 97;
pub const YYSYMBOL_print_exp: yysymbol_kind_t = 96;
pub const YYSYMBOL_enable_args: yysymbol_kind_t = 95;
pub const YYSYMBOL_help_args: yysymbol_kind_t = 94;
pub const YYSYMBOL_opt_node: yysymbol_kind_t = 93;
pub const YYSYMBOL_opt_string: yysymbol_kind_t = 92;
pub const YYSYMBOL_opt_variable: yysymbol_kind_t = 91;
pub const YYSYMBOL_90_10: yysymbol_kind_t = 90;
pub const YYSYMBOL_89_9: yysymbol_kind_t = 89;
pub const YYSYMBOL_break_args: yysymbol_kind_t = 88;
pub const YYSYMBOL_location: yysymbol_kind_t = 87;
pub const YYSYMBOL_func_name: yysymbol_kind_t = 86;
pub const YYSYMBOL_option_args: yysymbol_kind_t = 85;
pub const YYSYMBOL_string_node: yysymbol_kind_t = 84;
pub const YYSYMBOL_opt_string_node: yysymbol_kind_t = 83;
pub const YYSYMBOL_param_list: yysymbol_kind_t = 82;
pub const YYSYMBOL_opt_param_list: yysymbol_kind_t = 81;
pub const YYSYMBOL_commands_arg: yysymbol_kind_t = 80;
pub const YYSYMBOL_condition_exp: yysymbol_kind_t = 79;
pub const YYSYMBOL_78_8: yysymbol_kind_t = 78;
pub const YYSYMBOL_77_7: yysymbol_kind_t = 77;
pub const YYSYMBOL_76_6: yysymbol_kind_t = 76;
pub const YYSYMBOL_75_5: yysymbol_kind_t = 75;
pub const YYSYMBOL_74_4: yysymbol_kind_t = 74;
pub const YYSYMBOL_73_3: yysymbol_kind_t = 73;
pub const YYSYMBOL_72_2: yysymbol_kind_t = 72;
pub const YYSYMBOL_command: yysymbol_kind_t = 71;
pub const YYSYMBOL_eval_cmd: yysymbol_kind_t = 70;
pub const YYSYMBOL_69_1: yysymbol_kind_t = 69;
pub const YYSYMBOL_statement_list: yysymbol_kind_t = 68;
pub const YYSYMBOL_eval_prologue: yysymbol_kind_t = 67;
pub const YYSYMBOL_set_want_nodeval: yysymbol_kind_t = 66;
pub const YYSYMBOL_break_cmd: yysymbol_kind_t = 65;
pub const YYSYMBOL_frame_cmd: yysymbol_kind_t = 64;
pub const YYSYMBOL_d_cmd: yysymbol_kind_t = 63;
pub const YYSYMBOL_control_cmd: yysymbol_kind_t = 62;
pub const YYSYMBOL_line: yysymbol_kind_t = 61;
pub const YYSYMBOL_input: yysymbol_kind_t = 60;
pub const YYSYMBOL_YYACCEPT: yysymbol_kind_t = 59;
pub const YYSYMBOL_58_n_: yysymbol_kind_t = 58;
pub const YYSYMBOL_57_: yysymbol_kind_t = 57;
pub const YYSYMBOL_56_: yysymbol_kind_t = 56;
pub const YYSYMBOL_55_: yysymbol_kind_t = 55;
pub const YYSYMBOL_54_: yysymbol_kind_t = 54;
pub const YYSYMBOL_53_: yysymbol_kind_t = 53;
pub const YYSYMBOL_52_: yysymbol_kind_t = 52;
pub const YYSYMBOL_51_: yysymbol_kind_t = 51;
pub const YYSYMBOL_50_: yysymbol_kind_t = 50;
pub const YYSYMBOL_49_: yysymbol_kind_t = 49;
pub const YYSYMBOL_D_STATEMENT: yysymbol_kind_t = 48;
pub const YYSYMBOL_D_CONDITION: yysymbol_kind_t = 47;
pub const YYSYMBOL_D_EVAL: yysymbol_kind_t = 46;
pub const YYSYMBOL_D_SAVE: yysymbol_kind_t = 45;
pub const YYSYMBOL_D_SOURCE: yysymbol_kind_t = 44;
pub const YYSYMBOL_D_SILENT: yysymbol_kind_t = 43;
pub const YYSYMBOL_D_END: yysymbol_kind_t = 42;
pub const YYSYMBOL_D_COMMANDS: yysymbol_kind_t = 41;
pub const YYSYMBOL_D_OPTION: yysymbol_kind_t = 40;
pub const YYSYMBOL_D_VARIABLE: yysymbol_kind_t = 39;
pub const YYSYMBOL_D_NODE: yysymbol_kind_t = 38;
pub const YYSYMBOL_D_STRING: yysymbol_kind_t = 37;
pub const YYSYMBOL_D_INT: yysymbol_kind_t = 36;
pub const YYSYMBOL_D_TRACE: yysymbol_kind_t = 35;
pub const YYSYMBOL_D_DUMP: yysymbol_kind_t = 34;
pub const YYSYMBOL_D_UNWATCH: yysymbol_kind_t = 33;
pub const YYSYMBOL_D_WATCH: yysymbol_kind_t = 32;
pub const YYSYMBOL_D_UNDISPLAY: yysymbol_kind_t = 31;
pub const YYSYMBOL_D_DISPLAY: yysymbol_kind_t = 30;
pub const YYSYMBOL_D_UNTIL: yysymbol_kind_t = 29;
pub const YYSYMBOL_D_UP: yysymbol_kind_t = 28;
pub const YYSYMBOL_D_TBREAK: yysymbol_kind_t = 27;
pub const YYSYMBOL_D_STEPI: yysymbol_kind_t = 26;
pub const YYSYMBOL_D_STEP: yysymbol_kind_t = 25;
pub const YYSYMBOL_D_SET: yysymbol_kind_t = 24;
pub const YYSYMBOL_D_RUN: yysymbol_kind_t = 23;
pub const YYSYMBOL_D_RETURN: yysymbol_kind_t = 22;
pub const YYSYMBOL_D_QUIT: yysymbol_kind_t = 21;
pub const YYSYMBOL_D_PRINTF: yysymbol_kind_t = 20;
pub const YYSYMBOL_D_PRINT: yysymbol_kind_t = 19;
pub const YYSYMBOL_D_NEXTI: yysymbol_kind_t = 18;
pub const YYSYMBOL_D_NEXT: yysymbol_kind_t = 17;
pub const YYSYMBOL_D_LIST: yysymbol_kind_t = 16;
pub const YYSYMBOL_D_INFO: yysymbol_kind_t = 15;
pub const YYSYMBOL_D_IGNORE: yysymbol_kind_t = 14;
pub const YYSYMBOL_D_HELP: yysymbol_kind_t = 13;
pub const YYSYMBOL_D_FRAME: yysymbol_kind_t = 12;
pub const YYSYMBOL_D_FINISH: yysymbol_kind_t = 11;
pub const YYSYMBOL_D_ENABLE: yysymbol_kind_t = 10;
pub const YYSYMBOL_D_DOWN: yysymbol_kind_t = 9;
pub const YYSYMBOL_D_DISABLE: yysymbol_kind_t = 8;
pub const YYSYMBOL_D_DELETE: yysymbol_kind_t = 7;
pub const YYSYMBOL_D_CONTINUE: yysymbol_kind_t = 6;
pub const YYSYMBOL_D_CLEAR: yysymbol_kind_t = 5;
pub const YYSYMBOL_D_BREAK: yysymbol_kind_t = 4;
pub const YYSYMBOL_D_BACKTRACE: yysymbol_kind_t = 3;
pub const YYSYMBOL_YYUNDEF: yysymbol_kind_t = 2;
pub const YYSYMBOL_YYerror: yysymbol_kind_t = 1;
pub const YYSYMBOL_YYEOF: yysymbol_kind_t = 0;
pub const YYSYMBOL_YYEMPTY: yysymbol_kind_t = -2;
pub type yytype_int8 = libc::c_schar;
pub type yy_state_fast_t = libc::c_int;
pub type yytype_int16 = libc::c_short;
#[derive(Copy, Clone)]
#[repr(C)]
pub union yyalloc {
    pub yyss_alloc: yy_state_t,
    pub yyvs_alloc: *mut CMDARG,
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
static mut want_nodeval: bool = 0 as libc::c_int != 0;
static mut cmd_idx: libc::c_int = -(1 as libc::c_int);
static mut repeat_idx: libc::c_int = -(1 as libc::c_int);
static mut arg_list: *mut CMDARG = 0 as *const CMDARG as *mut CMDARG;
static mut dbg_errcount: libc::c_long = 0 as libc::c_int as libc::c_long;
static mut lexptr_begin: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut in_commands: bool = 0 as libc::c_int != 0;
static mut num_dim: libc::c_int = 0;
static mut in_eval: bool = 0 as libc::c_int != 0;
static mut start_EVAL: [libc::c_char; 18] = unsafe {
    *::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"function @eval(){\0")
};
static mut end_EVAL: [libc::c_char; 2] = unsafe {
    *::std::mem::transmute::<&[u8; 2], &[libc::c_char; 2]>(b"}\0")
};
static mut this_cmd: libc::c_int = D_illegal as libc::c_int;
unsafe extern "C" fn append_statement(
    mut stmt_list: *mut CMDARG,
    mut stmt: *mut libc::c_char,
) -> *mut CMDARG {
    let mut a: *mut CMDARG = 0 as *mut CMDARG;
    let mut arg: *mut CMDARG = 0 as *mut CMDARG;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    let mut slen: libc::c_int = 0;
    let mut ssize: libc::c_int = 0;
    if stmt == start_EVAL.as_ptr() as *mut libc::c_char {
        len = ::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
            as libc::c_int;
        a = stmt_list;
        while !a.is_null() {
            len = (len as libc::c_ulong)
                .wrapping_add(
                    (strlen((*a).value.sval))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as libc::c_int as libc::c_int;
            a = (*a).next;
        }
        len += 512 as libc::c_int;
        s = emalloc_real(
            ((len + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            b"append_statement\0" as *const u8 as *const libc::c_char,
            b"s\0" as *const u8 as *const libc::c_char,
            b"command.y\0" as *const u8 as *const libc::c_char,
            770 as libc::c_int,
        ) as *mut libc::c_char;
        arg = mk_cmdarg(D_string);
        (*arg).value.sval = s;
        (*arg).a_count = len;
        slen = (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
        memcpy(
            s as *mut libc::c_void,
            start_EVAL.as_ptr() as *const libc::c_void,
            slen as libc::c_ulong,
        );
        a = stmt_list;
        while !a.is_null() {
            len = strlen((*a).value.sval) as libc::c_int;
            memcpy(
                s.offset(slen as isize) as *mut libc::c_void,
                (*a).value.sval as *const libc::c_void,
                len as libc::c_ulong,
            );
            slen += len;
            if !((*a).next).is_null() {
                let fresh0 = slen;
                slen = slen + 1;
                *s.offset(fresh0 as isize) = ',' as i32 as libc::c_char;
            }
            a = (*a).next;
        }
        let fresh1 = slen;
        slen = slen + 1;
        *s.offset(fresh1 as isize) = ')' as i32 as libc::c_char;
        let fresh2 = slen;
        slen = slen + 1;
        *s.offset(fresh2 as isize) = '{' as i32 as libc::c_char;
        *s.offset(slen as isize) = '\0' as i32 as libc::c_char;
        return arg;
    }
    len = (strlen(stmt)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    s = (*stmt_list).value.sval;
    slen = strlen(s) as libc::c_int;
    ssize = (*stmt_list).a_count;
    if len > ssize - slen {
        ssize = slen + len + 512 as libc::c_int;
        s = erealloc_real(
            s as *mut libc::c_void,
            ((ssize + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            b"append_statement\0" as *const u8 as *const libc::c_char,
            b"s\0" as *const u8 as *const libc::c_char,
            b"command.y\0" as *const u8 as *const libc::c_char,
            797 as libc::c_int,
        ) as *mut libc::c_char;
        (*stmt_list).value.sval = s;
        (*stmt_list).a_count = ssize;
    }
    memcpy(
        s.offset(slen as isize) as *mut libc::c_void,
        stmt as *const libc::c_void,
        len as libc::c_ulong,
    );
    slen += len;
    if slen >= 2 as libc::c_int
        && *s.offset((slen - 2 as libc::c_int) as isize) as libc::c_int != '\n' as i32
    {
        *s.offset((slen - 1 as libc::c_int) as isize) = '\n' as i32 as libc::c_char;
        *s.offset(slen as isize) = '\0' as i32 as libc::c_char;
    }
    if stmt == end_EVAL.as_ptr() as *mut libc::c_char {
        (*stmt_list)
            .value
            .sval = erealloc_real(
            (*stmt_list).value.sval as *mut libc::c_void,
            (slen + 1 as libc::c_int) as size_t,
            b"append_statement\0" as *const u8 as *const libc::c_char,
            b"stmt_list->a_string\0" as *const u8 as *const libc::c_char,
            b"command.y\0" as *const u8 as *const libc::c_char,
            809 as libc::c_int,
        ) as *mut libc::c_char;
    }
    return stmt_list;
}
static mut yytranslate: [yytype_int8; 304] = [
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
    58 as libc::c_int as yytype_int8,
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
    57 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
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
    51 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    49 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    52 as libc::c_int as yytype_int8,
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
    55 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    56 as libc::c_int as yytype_int8,
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
];
pub static mut zz_debug_cmdtab: [cmdtoken; 44] = unsafe {
    [
        {
            let mut init = cmdtoken {
                name: b"backtrace\0" as *const u8 as *const libc::c_char,
                abbrvn: b"bt\0" as *const u8 as *const libc::c_char,
                type_0: D_backtrace,
                lex_class: 258 as libc::c_int,
                cf_ptr: Some(
                    do_backtrace
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"backtrace [N] - print trace of all or N innermost (outermost if N < 0) frames\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"break\0" as *const u8 as *const libc::c_char,
                abbrvn: b"b\0" as *const u8 as *const libc::c_char,
                type_0: D_break,
                lex_class: 259 as libc::c_int,
                cf_ptr: Some(
                    do_breakpoint
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"break [[filename:]N|function] - set breakpoint at the specified location\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"clear\0" as *const u8 as *const libc::c_char,
                abbrvn: b"\0" as *const u8 as *const libc::c_char,
                type_0: D_clear,
                lex_class: 260 as libc::c_int,
                cf_ptr: Some(
                    do_clear
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"clear [[filename:]N|function] - delete breakpoints previously set\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"commands\0" as *const u8 as *const libc::c_char,
                abbrvn: b"\0" as *const u8 as *const libc::c_char,
                type_0: D_commands,
                lex_class: 296 as libc::c_int,
                cf_ptr: Some(
                    do_commands
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"commands [num] - starts a list of commands to be executed at a breakpoint(watchpoint) hit\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"condition\0" as *const u8 as *const libc::c_char,
                abbrvn: b"\0" as *const u8 as *const libc::c_char,
                type_0: D_condition,
                lex_class: 302 as libc::c_int,
                cf_ptr: Some(
                    do_condition
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"condition num [expr] - set or clear breakpoint or watchpoint condition\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"continue\0" as *const u8 as *const libc::c_char,
                abbrvn: b"c\0" as *const u8 as *const libc::c_char,
                type_0: D_continue,
                lex_class: 261 as libc::c_int,
                cf_ptr: Some(
                    do_continue
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"continue [COUNT] - continue program being debugged\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"delete\0" as *const u8 as *const libc::c_char,
                abbrvn: b"d\0" as *const u8 as *const libc::c_char,
                type_0: D_delete,
                lex_class: 262 as libc::c_int,
                cf_ptr: Some(
                    do_delete_breakpoint
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"delete [breakpoints] [range] - delete specified breakpoints\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"disable\0" as *const u8 as *const libc::c_char,
                abbrvn: b"\0" as *const u8 as *const libc::c_char,
                type_0: D_disable,
                lex_class: 263 as libc::c_int,
                cf_ptr: Some(
                    do_disable_breakpoint
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"disable [breakpoints] [range] - disable specified breakpoints\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"display\0" as *const u8 as *const libc::c_char,
                abbrvn: b"\0" as *const u8 as *const libc::c_char,
                type_0: D_display,
                lex_class: 285 as libc::c_int,
                cf_ptr: Some(
                    do_display
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"display [var] - print value of variable each time the program stops\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"down\0" as *const u8 as *const libc::c_char,
                abbrvn: b"\0" as *const u8 as *const libc::c_char,
                type_0: D_down,
                lex_class: 264 as libc::c_int,
                cf_ptr: Some(
                    do_down
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"down [N] - move N frames down the stack\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"dump\0" as *const u8 as *const libc::c_char,
                abbrvn: b"\0" as *const u8 as *const libc::c_char,
                type_0: D_dump,
                lex_class: 289 as libc::c_int,
                cf_ptr: Some(
                    do_dump_instructions
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"dump [filename] - dump instructions to file or stdout\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"enable\0" as *const u8 as *const libc::c_char,
                abbrvn: b"e\0" as *const u8 as *const libc::c_char,
                type_0: D_enable,
                lex_class: 265 as libc::c_int,
                cf_ptr: Some(
                    do_enable_breakpoint
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"enable [once|del] [breakpoints] [range] - enable specified breakpoints\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"end\0" as *const u8 as *const libc::c_char,
                abbrvn: b"\0" as *const u8 as *const libc::c_char,
                type_0: D_end,
                lex_class: 297 as libc::c_int,
                cf_ptr: Some(
                    do_commands
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"end - end a list of commands or awk statements\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"eval\0" as *const u8 as *const libc::c_char,
                abbrvn: b"\0" as *const u8 as *const libc::c_char,
                type_0: D_eval,
                lex_class: 301 as libc::c_int,
                cf_ptr: Some(
                    do_eval
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"eval stmt|[p1, p2, ...] - evaluate awk statement(s)\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"exit\0" as *const u8 as *const libc::c_char,
                abbrvn: b"\0" as *const u8 as *const libc::c_char,
                type_0: D_quit,
                lex_class: 276 as libc::c_int,
                cf_ptr: Some(
                    do_quit
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"exit - (same as quit) exit debugger\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"finish\0" as *const u8 as *const libc::c_char,
                abbrvn: b"\0" as *const u8 as *const libc::c_char,
                type_0: D_finish,
                lex_class: 266 as libc::c_int,
                cf_ptr: Some(
                    do_finish
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"finish - execute until selected stack frame returns\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"frame\0" as *const u8 as *const libc::c_char,
                abbrvn: b"f\0" as *const u8 as *const libc::c_char,
                type_0: D_frame,
                lex_class: 267 as libc::c_int,
                cf_ptr: Some(
                    do_frame
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"frame [N] - select and print stack frame number N\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"help\0" as *const u8 as *const libc::c_char,
                abbrvn: b"h\0" as *const u8 as *const libc::c_char,
                type_0: D_help,
                lex_class: 268 as libc::c_int,
                cf_ptr: Some(
                    do_help
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"help [command] - print list of commands or explanation of command\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"ignore\0" as *const u8 as *const libc::c_char,
                abbrvn: b"\0" as *const u8 as *const libc::c_char,
                type_0: D_ignore,
                lex_class: 269 as libc::c_int,
                cf_ptr: Some(
                    do_ignore_breakpoint
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"ignore N COUNT - set ignore-count of breakpoint number N to COUNT\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"info\0" as *const u8 as *const libc::c_char,
                abbrvn: b"i\0" as *const u8 as *const libc::c_char,
                type_0: D_info,
                lex_class: 270 as libc::c_int,
                cf_ptr: Some(
                    do_info
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"info topic - source|sources|variables|functions|break|frame|args|locals|display|watch\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"list\0" as *const u8 as *const libc::c_char,
                abbrvn: b"l\0" as *const u8 as *const libc::c_char,
                type_0: D_list,
                lex_class: 271 as libc::c_int,
                cf_ptr: Some(
                    do_list
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"list [-|+|[filename:]lineno|function|range] - list specified line(s)\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"next\0" as *const u8 as *const libc::c_char,
                abbrvn: b"n\0" as *const u8 as *const libc::c_char,
                type_0: D_next,
                lex_class: 272 as libc::c_int,
                cf_ptr: Some(
                    do_next
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"next [COUNT] - step program, proceeding through subroutine calls\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"nexti\0" as *const u8 as *const libc::c_char,
                abbrvn: b"ni\0" as *const u8 as *const libc::c_char,
                type_0: D_nexti,
                lex_class: 273 as libc::c_int,
                cf_ptr: Some(
                    do_nexti
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"nexti [COUNT] - step one instruction, but proceed through subroutine calls\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"option\0" as *const u8 as *const libc::c_char,
                abbrvn: b"o\0" as *const u8 as *const libc::c_char,
                type_0: D_option,
                lex_class: 295 as libc::c_int,
                cf_ptr: Some(
                    do_option
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"option [name[=value]] - set or display debugger option(s)\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"print\0" as *const u8 as *const libc::c_char,
                abbrvn: b"p\0" as *const u8 as *const libc::c_char,
                type_0: D_print,
                lex_class: 274 as libc::c_int,
                cf_ptr: Some(
                    do_print_var
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"print var [var] - print value of a variable or array\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"printf\0" as *const u8 as *const libc::c_char,
                abbrvn: b"\0" as *const u8 as *const libc::c_char,
                type_0: D_printf,
                lex_class: 275 as libc::c_int,
                cf_ptr: Some(
                    do_print_f
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"printf format, [arg], ... - formatted output\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"quit\0" as *const u8 as *const libc::c_char,
                abbrvn: b"q\0" as *const u8 as *const libc::c_char,
                type_0: D_quit,
                lex_class: 276 as libc::c_int,
                cf_ptr: Some(
                    do_quit
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"quit - exit debugger\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"return\0" as *const u8 as *const libc::c_char,
                abbrvn: b"\0" as *const u8 as *const libc::c_char,
                type_0: D_return,
                lex_class: 277 as libc::c_int,
                cf_ptr: Some(
                    do_return
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"return [value] - make selected stack frame return to its caller\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"run\0" as *const u8 as *const libc::c_char,
                abbrvn: b"r\0" as *const u8 as *const libc::c_char,
                type_0: D_run,
                lex_class: 278 as libc::c_int,
                cf_ptr: Some(
                    do_run
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"run - start or restart executing program\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"save\0" as *const u8 as *const libc::c_char,
                abbrvn: b"\0" as *const u8 as *const libc::c_char,
                type_0: D_save,
                lex_class: 300 as libc::c_int,
                cf_ptr: Some(
                    do_save
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"save filename - save commands from the session to file\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"set\0" as *const u8 as *const libc::c_char,
                abbrvn: b"\0" as *const u8 as *const libc::c_char,
                type_0: D_set,
                lex_class: 279 as libc::c_int,
                cf_ptr: Some(
                    do_set_var
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"set var = value - assign value to a scalar variable\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"silent\0" as *const u8 as *const libc::c_char,
                abbrvn: b"\0" as *const u8 as *const libc::c_char,
                type_0: D_silent,
                lex_class: 298 as libc::c_int,
                cf_ptr: Some(
                    do_commands
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"silent - suspends usual message when stopped at a breakpoint/watchpoint\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"source\0" as *const u8 as *const libc::c_char,
                abbrvn: b"\0" as *const u8 as *const libc::c_char,
                type_0: D_source,
                lex_class: 299 as libc::c_int,
                cf_ptr: Some(
                    do_source
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"source file - execute commands from file\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"step\0" as *const u8 as *const libc::c_char,
                abbrvn: b"s\0" as *const u8 as *const libc::c_char,
                type_0: D_step,
                lex_class: 280 as libc::c_int,
                cf_ptr: Some(
                    do_step
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"step [COUNT] - step program until it reaches a different source line\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"stepi\0" as *const u8 as *const libc::c_char,
                abbrvn: b"si\0" as *const u8 as *const libc::c_char,
                type_0: D_stepi,
                lex_class: 281 as libc::c_int,
                cf_ptr: Some(
                    do_stepi
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"stepi [COUNT] - step one instruction exactly\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"tbreak\0" as *const u8 as *const libc::c_char,
                abbrvn: b"t\0" as *const u8 as *const libc::c_char,
                type_0: D_tbreak,
                lex_class: 282 as libc::c_int,
                cf_ptr: Some(
                    do_tmp_breakpoint
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"tbreak [[filename:]N|function] - set a temporary breakpoint\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"trace\0" as *const u8 as *const libc::c_char,
                abbrvn: b"\0" as *const u8 as *const libc::c_char,
                type_0: D_trace,
                lex_class: 290 as libc::c_int,
                cf_ptr: Some(
                    do_trace_instruction
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"trace on|off - print instruction before executing\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"undisplay\0" as *const u8 as *const libc::c_char,
                abbrvn: b"\0" as *const u8 as *const libc::c_char,
                type_0: D_undisplay,
                lex_class: 286 as libc::c_int,
                cf_ptr: Some(
                    do_undisplay
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"undisplay [N] - remove variable(s) from automatic display list\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"until\0" as *const u8 as *const libc::c_char,
                abbrvn: b"u\0" as *const u8 as *const libc::c_char,
                type_0: D_until,
                lex_class: 284 as libc::c_int,
                cf_ptr: Some(
                    do_until
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"until [[filename:]N|function] - execute until program reaches a different line or line N within current frame\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"unwatch\0" as *const u8 as *const libc::c_char,
                abbrvn: b"\0" as *const u8 as *const libc::c_char,
                type_0: D_unwatch,
                lex_class: 288 as libc::c_int,
                cf_ptr: Some(
                    do_unwatch
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"unwatch [N] - remove variable(s) from watch list\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"up\0" as *const u8 as *const libc::c_char,
                abbrvn: b"\0" as *const u8 as *const libc::c_char,
                type_0: D_up,
                lex_class: 283 as libc::c_int,
                cf_ptr: Some(
                    do_up
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"up [N] - move N frames up the stack\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"watch\0" as *const u8 as *const libc::c_char,
                abbrvn: b"w\0" as *const u8 as *const libc::c_char,
                type_0: D_watch,
                lex_class: 287 as libc::c_int,
                cf_ptr: Some(
                    do_watch
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"watch var - set a watchpoint for a variable\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"where\0" as *const u8 as *const libc::c_char,
                abbrvn: b"\0" as *const u8 as *const libc::c_char,
                type_0: D_backtrace,
                lex_class: 258 as libc::c_int,
                cf_ptr: Some(
                    do_backtrace
                        as unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
                ),
                help_txt: b"where [N] - (same as backtrace) print trace of all or N innermost (outermost if N < 0) frames\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: 0 as *const libc::c_char,
                abbrvn: 0 as *const libc::c_char,
                type_0: D_illegal,
                lex_class: 0 as libc::c_int,
                cf_ptr: None,
                help_txt: 0 as *const libc::c_char,
            };
            init
        },
    ]
};
static mut yypact: [yytype_int16; 203] = [
    -(151 as libc::c_int) as yytype_int16,
    145 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(34 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    50 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    10 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(10 as libc::c_int) as yytype_int16,
    59 as libc::c_int as yytype_int16,
    -(9 as libc::c_int) as yytype_int16,
    43 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    50 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(8 as libc::c_int) as yytype_int16,
    -(6 as libc::c_int) as yytype_int16,
    14 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    59 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    59 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    64 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(34 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    24 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    13 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    59 as libc::c_int as yytype_int16,
    48 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    80 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    67 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    48 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    -(20 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(20 as libc::c_int) as yytype_int16,
    -(20 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    70 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    16 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    84 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    73 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    40 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    74 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    48 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    74 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    71 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    91 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    42 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    81 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    92 as libc::c_int as yytype_int16,
    94 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    86 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    6 as libc::c_int as yytype_int16,
    96 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(34 as libc::c_int) as yytype_int16,
    75 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    6 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    74 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    48 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    71 as libc::c_int as yytype_int16,
    71 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    52 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(17 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    69 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    95 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(34 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    17 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    71 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    6 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    69 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
];
static mut yydefact: [yytype_uint8; 203] = [
    2 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    32 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    101 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    117 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    9 as libc::c_int as yytype_uint8,
    38 as libc::c_int as yytype_uint8,
    40 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    31 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    10 as libc::c_int as yytype_uint8,
    11 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    12 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    97 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    79 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    61 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    22 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    156 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    147 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    149 as libc::c_int as yytype_uint8,
    88 as libc::c_int as yytype_uint8,
    24 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    82 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    85 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    84 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    37 as libc::c_int as yytype_uint8,
    131 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    128 as libc::c_int as yytype_uint8,
    130 as libc::c_int as yytype_uint8,
    102 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    35 as libc::c_int as yytype_uint8,
    82 as libc::c_int as yytype_uint8,
    118 as libc::c_int as yytype_uint8,
    119 as libc::c_int as yytype_uint8,
    121 as libc::c_int as yytype_uint8,
    42 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    120 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    99 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    95 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    98 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    62 as libc::c_int as yytype_uint8,
    80 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    68 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    67 as libc::c_int as yytype_uint8,
    148 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    63 as libc::c_int as yytype_uint8,
    33 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    153 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    34 as libc::c_int as yytype_uint8,
    150 as libc::c_int as yytype_uint8,
    82 as libc::c_int as yytype_uint8,
    91 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    89 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    152 as libc::c_int as yytype_uint8,
    104 as libc::c_int as yytype_uint8,
    133 as libc::c_int as yytype_uint8,
    132 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    36 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    111 as libc::c_int as yytype_uint8,
    141 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    108 as libc::c_int as yytype_uint8,
    39 as libc::c_int as yytype_uint8,
    105 as libc::c_int as yytype_uint8,
    116 as libc::c_int as yytype_uint8,
    112 as libc::c_int as yytype_uint8,
    114 as libc::c_int as yytype_uint8,
    41 as libc::c_int as yytype_uint8,
    113 as libc::c_int as yytype_uint8,
    144 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    100 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    74 as libc::c_int as yytype_uint8,
    78 as libc::c_int as yytype_uint8,
    71 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    70 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    154 as libc::c_int as yytype_uint8,
    155 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    27 as libc::c_int as yytype_uint8,
    25 as libc::c_int as yytype_uint8,
    82 as libc::c_int as yytype_uint8,
    87 as libc::c_int as yytype_uint8,
    86 as libc::c_int as yytype_uint8,
    126 as libc::c_int as yytype_uint8,
    124 as libc::c_int as yytype_uint8,
    125 as libc::c_int as yytype_uint8,
    123 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    139 as libc::c_int as yytype_uint8,
    143 as libc::c_int as yytype_uint8,
    106 as libc::c_int as yytype_uint8,
    142 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    109 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    145 as libc::c_int as yytype_uint8,
    146 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    77 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    66 as libc::c_int as yytype_uint8,
    76 as libc::c_int as yytype_uint8,
    81 as libc::c_int as yytype_uint8,
    23 as libc::c_int as yytype_uint8,
    72 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    94 as libc::c_int as yytype_uint8,
    92 as libc::c_int as yytype_uint8,
    90 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    136 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    134 as libc::c_int as yytype_uint8,
    140 as libc::c_int as yytype_uint8,
    107 as libc::c_int as yytype_uint8,
    110 as libc::c_int as yytype_uint8,
    115 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    73 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    26 as libc::c_int as yytype_uint8,
    138 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    137 as libc::c_int as yytype_uint8,
    93 as libc::c_int as yytype_uint8,
    135 as libc::c_int as yytype_uint8,
];
pub static mut zz_debug_argtab: [argtoken; 15] = [
    {
        let mut init = argtoken {
            name: b"args\0" as *const u8 as *const libc::c_char,
            cmd: D_info,
            value: A_ARGS,
        };
        init
    },
    {
        let mut init = argtoken {
            name: b"break\0" as *const u8 as *const libc::c_char,
            cmd: D_info,
            value: A_BREAK,
        };
        init
    },
    {
        let mut init = argtoken {
            name: b"del\0" as *const u8 as *const libc::c_char,
            cmd: D_enable,
            value: A_DEL,
        };
        init
    },
    {
        let mut init = argtoken {
            name: b"display\0" as *const u8 as *const libc::c_char,
            cmd: D_info,
            value: A_DISPLAY,
        };
        init
    },
    {
        let mut init = argtoken {
            name: b"frame\0" as *const u8 as *const libc::c_char,
            cmd: D_info,
            value: A_FRAME,
        };
        init
    },
    {
        let mut init = argtoken {
            name: b"functions\0" as *const u8 as *const libc::c_char,
            cmd: D_info,
            value: A_FUNCTIONS,
        };
        init
    },
    {
        let mut init = argtoken {
            name: b"locals\0" as *const u8 as *const libc::c_char,
            cmd: D_info,
            value: A_LOCALS,
        };
        init
    },
    {
        let mut init = argtoken {
            name: b"off\0" as *const u8 as *const libc::c_char,
            cmd: D_trace,
            value: A_TRACE_OFF,
        };
        init
    },
    {
        let mut init = argtoken {
            name: b"on\0" as *const u8 as *const libc::c_char,
            cmd: D_trace,
            value: A_TRACE_ON,
        };
        init
    },
    {
        let mut init = argtoken {
            name: b"once\0" as *const u8 as *const libc::c_char,
            cmd: D_enable,
            value: A_ONCE,
        };
        init
    },
    {
        let mut init = argtoken {
            name: b"source\0" as *const u8 as *const libc::c_char,
            cmd: D_info,
            value: A_SOURCE,
        };
        init
    },
    {
        let mut init = argtoken {
            name: b"sources\0" as *const u8 as *const libc::c_char,
            cmd: D_info,
            value: A_SOURCES,
        };
        init
    },
    {
        let mut init = argtoken {
            name: b"variables\0" as *const u8 as *const libc::c_char,
            cmd: D_info,
            value: A_VARIABLES,
        };
        init
    },
    {
        let mut init = argtoken {
            name: b"watch\0" as *const u8 as *const libc::c_char,
            cmd: D_info,
            value: A_WATCH,
        };
        init
    },
    {
        let mut init = argtoken {
            name: 0 as *const libc::c_char,
            cmd: D_illegal,
            value: A_NONE,
        };
        init
    },
];
pub unsafe extern "C" fn get_command(mut ctype: libc::c_int) -> Func_cmd {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !(zz_debug_cmdtab[i as usize].name).is_null() {
        if zz_debug_cmdtab[i as usize].type_0 as libc::c_uint == ctype as libc::c_uint {
            return zz_debug_cmdtab[i as usize].cf_ptr;
        }
        i += 1;
        i;
    }
    return None;
}
static mut yypgoto: [yytype_int16; 55] = [
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(119 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    38 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(15 as libc::c_int) as yytype_int16,
    108 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(90 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(31 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(14 as libc::c_int) as yytype_int16,
    -(25 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(150 as libc::c_int) as yytype_int16,
    -(26 as libc::c_int) as yytype_int16,
    -(77 as libc::c_int) as yytype_int16,
    -(147 as libc::c_int) as yytype_int16,
    97 as libc::c_int as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(5 as libc::c_int) as yytype_int16,
    -(151 as libc::c_int) as yytype_int16,
    -(3 as libc::c_int) as yytype_int16,
];
pub unsafe extern "C" fn get_command_name(
    mut ctype: libc::c_int,
) -> *const libc::c_char {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !(zz_debug_cmdtab[i as usize].name).is_null() {
        if zz_debug_cmdtab[i as usize].type_0 as libc::c_uint == ctype as libc::c_uint {
            return zz_debug_cmdtab[i as usize].name;
        }
        i += 1;
        i;
    }
    return 0 as *const libc::c_char;
}
static mut yydefgoto: [yytype_uint8; 55] = [
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    98 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    111 as libc::c_int as yytype_uint8,
    186 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    80 as libc::c_int as yytype_uint8,
    81 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    82 as libc::c_int as yytype_uint8,
    85 as libc::c_int as yytype_uint8,
    86 as libc::c_int as yytype_uint8,
    149 as libc::c_int as yytype_uint8,
    175 as libc::c_int as yytype_uint8,
    93 as libc::c_int as yytype_uint8,
    146 as libc::c_int as yytype_uint8,
    147 as libc::c_int as yytype_uint8,
    176 as libc::c_int as yytype_uint8,
    177 as libc::c_int as yytype_uint8,
    91 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    109 as libc::c_int as yytype_uint8,
    153 as libc::c_int as yytype_uint8,
    196 as libc::c_int as yytype_uint8,
    139 as libc::c_int as yytype_uint8,
    88 as libc::c_int as yytype_uint8,
    136 as libc::c_int as yytype_uint8,
    70 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    125 as libc::c_int as yytype_uint8,
    126 as libc::c_int as yytype_uint8,
    130 as libc::c_int as yytype_uint8,
    131 as libc::c_int as yytype_uint8,
    77 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    66 as libc::c_int as yytype_uint8,
    67 as libc::c_int as yytype_uint8,
    188 as libc::c_int as yytype_uint8,
    164 as libc::c_int as yytype_uint8,
    165 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    137 as libc::c_int as yytype_uint8,
    94 as libc::c_int as yytype_uint8,
    105 as libc::c_int as yytype_uint8,
    68 as libc::c_int as yytype_uint8,
    106 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
];
unsafe extern "C" fn mk_cmdarg(mut type_0: argtype) -> *mut CMDARG {
    let mut arg: *mut CMDARG = 0 as *mut CMDARG;
    arg = ezalloc_real(
        ::std::mem::size_of::<CMDARG>() as libc::c_ulong,
        b"mk_cmdarg\0" as *const u8 as *const libc::c_char,
        b"arg\0" as *const u8 as *const libc::c_char,
        b"command.y\0" as *const u8 as *const libc::c_char,
        962 as libc::c_int,
    ) as *mut CMDARG;
    (*arg).type_0 = type_0;
    return arg;
}
static mut yytable: [yytype_int16; 204] = [
    55 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    78 as libc::c_int as yytype_int16,
    132 as libc::c_int as yytype_int16,
    121 as libc::c_int as yytype_int16,
    138 as libc::c_int as yytype_int16,
    174 as libc::c_int as yytype_int16,
    140 as libc::c_int as yytype_int16,
    141 as libc::c_int as yytype_int16,
    71 as libc::c_int as yytype_int16,
    62 as libc::c_int as yytype_int16,
    79 as libc::c_int as yytype_int16,
    92 as libc::c_int as yytype_int16,
    62 as libc::c_int as yytype_int16,
    190 as libc::c_int as yytype_int16,
    189 as libc::c_int as yytype_int16,
    143 as libc::c_int as yytype_int16,
    198 as libc::c_int as yytype_int16,
    122 as libc::c_int as yytype_int16,
    128 as libc::c_int as yytype_int16,
    129 as libc::c_int as yytype_int16,
    122 as libc::c_int as yytype_int16,
    101 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    194 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    72 as libc::c_int as yytype_int16,
    87 as libc::c_int as yytype_int16,
    182 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    187 as libc::c_int as yytype_int16,
    95 as libc::c_int as yytype_int16,
    185 as libc::c_int as yytype_int16,
    108 as libc::c_int as yytype_int16,
    169 as libc::c_int as yytype_int16,
    124 as libc::c_int as yytype_int16,
    115 as libc::c_int as yytype_int16,
    99 as libc::c_int as yytype_int16,
    124 as libc::c_int as yytype_int16,
    190 as libc::c_int as yytype_int16,
    95 as libc::c_int as yytype_int16,
    122 as libc::c_int as yytype_int16,
    144 as libc::c_int as yytype_int16,
    110 as libc::c_int as yytype_int16,
    56 as libc::c_int as yytype_int16,
    63 as libc::c_int as yytype_int16,
    56 as libc::c_int as yytype_int16,
    56 as libc::c_int as yytype_int16,
    112 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    202 as libc::c_int as yytype_int16,
    116 as libc::c_int as yytype_int16,
    144 as libc::c_int as yytype_int16,
    145 as libc::c_int as yytype_int16,
    123 as libc::c_int as yytype_int16,
    129 as libc::c_int as yytype_int16,
    122 as libc::c_int as yytype_int16,
    96 as libc::c_int as yytype_int16,
    97 as libc::c_int as yytype_int16,
    124 as libc::c_int as yytype_int16,
    117 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    -(75 as libc::c_int) as yytype_int16,
    58 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    199 as libc::c_int as yytype_int16,
    -(127 as libc::c_int) as yytype_int16,
    133 as libc::c_int as yytype_int16,
    -(147 as libc::c_int) as yytype_int16,
    -(127 as libc::c_int) as yytype_int16,
    102 as libc::c_int as yytype_int16,
    200 as libc::c_int as yytype_int16,
    -(69 as libc::c_int) as yytype_int16,
    113 as libc::c_int as yytype_int16,
    124 as libc::c_int as yytype_int16,
    201 as libc::c_int as yytype_int16,
    192 as libc::c_int as yytype_int16,
    56 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    122 as libc::c_int as yytype_int16,
    154 as libc::c_int as yytype_int16,
    114 as libc::c_int as yytype_int16,
    134 as libc::c_int as yytype_int16,
    135 as libc::c_int as yytype_int16,
    56 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    155 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    104 as libc::c_int as yytype_int16,
    122 as libc::c_int as yytype_int16,
    168 as libc::c_int as yytype_int16,
    132 as libc::c_int as yytype_int16,
    123 as libc::c_int as yytype_int16,
    56 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    157 as libc::c_int as yytype_int16,
    124 as libc::c_int as yytype_int16,
    56 as libc::c_int as yytype_int16,
    107 as libc::c_int as yytype_int16,
    118 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    123 as libc::c_int as yytype_int16,
    160 as libc::c_int as yytype_int16,
    161 as libc::c_int as yytype_int16,
    133 as libc::c_int as yytype_int16,
    158 as libc::c_int as yytype_int16,
    124 as libc::c_int as yytype_int16,
    56 as libc::c_int as yytype_int16,
    156 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    159 as libc::c_int as yytype_int16,
    180 as libc::c_int as yytype_int16,
    162 as libc::c_int as yytype_int16,
    119 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    120 as libc::c_int as yytype_int16,
    142 as libc::c_int as yytype_int16,
    150 as libc::c_int as yytype_int16,
    151 as libc::c_int as yytype_int16,
    134 as libc::c_int as yytype_int16,
    135 as libc::c_int as yytype_int16,
    152 as libc::c_int as yytype_int16,
    181 as libc::c_int as yytype_int16,
    163 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    166 as libc::c_int as yytype_int16,
    167 as libc::c_int as yytype_int16,
    171 as libc::c_int as yytype_int16,
    170 as libc::c_int as yytype_int16,
    172 as libc::c_int as yytype_int16,
    178 as libc::c_int as yytype_int16,
    195 as libc::c_int as yytype_int16,
    173 as libc::c_int as yytype_int16,
    148 as libc::c_int as yytype_int16,
    183 as libc::c_int as yytype_int16,
    84 as libc::c_int as yytype_int16,
    193 as libc::c_int as yytype_int16,
    191 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    179 as libc::c_int as yytype_int16,
    100 as libc::c_int as yytype_int16,
    2 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    184 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
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
    36 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    197 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
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
    45 as libc::c_int as yytype_int16,
];
unsafe extern "C" fn append_cmdarg(mut arg: *mut CMDARG) {
    static mut savetail: *mut CMDARG = 0 as *const CMDARG as *mut CMDARG;
    if arg_list.is_null() {
        arg_list = arg;
    } else {
        (*savetail).next = arg;
    }
    savetail = arg;
}
pub unsafe extern "C" fn free_cmdarg(mut list: *mut CMDARG) {
    let mut arg: *mut CMDARG = 0 as *mut CMDARG;
    let mut nexta: *mut CMDARG = 0 as *mut CMDARG;
    arg = list;
    while !arg.is_null() {
        nexta = (*arg).next;
        match (*arg).type_0 as libc::c_uint {
            45 | 49 | 48 | 44 => {
                if !((*arg).value.sval).is_null() {
                    free((*arg).value.sval as *mut libc::c_void);
                }
            }
            46 | 47 => {
                unref((*arg).value.nodeval);
            }
            _ => {}
        }
        free(arg as *mut libc::c_void);
        arg = nexta;
    }
}
static mut yycheck: [yytype_int16; 204] = [
    3 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    81 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    83 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    165 as libc::c_int as yytype_int16,
    163 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    48 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    173 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    149 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    153 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    126 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    63 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    191 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    199 as libc::c_int as yytype_int16,
    67 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    52 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    67 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    56 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    196 as libc::c_int as yytype_int16,
    168 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    48 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    170 as libc::c_int as yytype_int16,
    52 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    52 as libc::c_int as yytype_int16,
    120 as libc::c_int as yytype_int16,
    120 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    118 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    120 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    49 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    55 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    49 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    152 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    170 as libc::c_int as yytype_int16,
    166 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    146 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    152 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
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
    186 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    40 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
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
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    58 as libc::c_int as yytype_int16,
];
unsafe extern "C" fn zzerror(mut mesg: *const libc::c_char, mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    fprintf(
        out_fp,
        dcgettext(
            0 as *const libc::c_char,
            b"error: \0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    vfprintf(out_fp, mesg, args_0.as_va_list());
    fprintf(out_fp, b"\n\0" as *const u8 as *const libc::c_char);
    dbg_errcount += 1;
    dbg_errcount;
    repeat_idx = -(1 as libc::c_int);
}
static mut yystos: [yytype_int8; 203] = [
    0 as libc::c_int as yytype_int8,
    60 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
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
    40 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    44 as libc::c_int as yytype_int8,
    45 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    58 as libc::c_int as yytype_int8,
    61 as libc::c_int as yytype_int8,
    62 as libc::c_int as yytype_int8,
    63 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    65 as libc::c_int as yytype_int8,
    67 as libc::c_int as yytype_int8,
    70 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    113 as libc::c_int as yytype_int8,
    113 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
    86 as libc::c_int as yytype_int8,
    87 as libc::c_int as yytype_int8,
    111 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    95 as libc::c_int as yytype_int8,
    101 as libc::c_int as yytype_int8,
    102 as libc::c_int as yytype_int8,
    103 as libc::c_int as yytype_int8,
    111 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    94 as libc::c_int as yytype_int8,
    111 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
    86 as libc::c_int as yytype_int8,
    100 as libc::c_int as yytype_int8,
    101 as libc::c_int as yytype_int8,
    111 as libc::c_int as yytype_int8,
    72 as libc::c_int as yytype_int8,
    73 as libc::c_int as yytype_int8,
    75 as libc::c_int as yytype_int8,
    74 as libc::c_int as yytype_int8,
    87 as libc::c_int as yytype_int8,
    76 as libc::c_int as yytype_int8,
    77 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    92 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    85 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    80 as libc::c_int as yytype_int8,
    109 as libc::c_int as yytype_int8,
    111 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    66 as libc::c_int as yytype_int8,
    111 as libc::c_int as yytype_int8,
    109 as libc::c_int as yytype_int8,
    102 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
    110 as libc::c_int as yytype_int8,
    112 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    86 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    111 as libc::c_int as yytype_int8,
    68 as libc::c_int as yytype_int8,
    113 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    102 as libc::c_int as yytype_int8,
    101 as libc::c_int as yytype_int8,
    111 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    52 as libc::c_int as yytype_int8,
    57 as libc::c_int as yytype_int8,
    96 as libc::c_int as yytype_int8,
    97 as libc::c_int as yytype_int8,
    107 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    98 as libc::c_int as yytype_int8,
    99 as libc::c_int as yytype_int8,
    107 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    108 as libc::c_int as yytype_int8,
    107 as libc::c_int as yytype_int8,
    91 as libc::c_int as yytype_int8,
    107 as libc::c_int as yytype_int8,
    107 as libc::c_int as yytype_int8,
    49 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    81 as libc::c_int as yytype_int8,
    82 as libc::c_int as yytype_int8,
    84 as libc::c_int as yytype_int8,
    78 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    89 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    48 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    86 as libc::c_int as yytype_int8,
    111 as libc::c_int as yytype_int8,
    111 as libc::c_int as yytype_int8,
    86 as libc::c_int as yytype_int8,
    101 as libc::c_int as yytype_int8,
    111 as libc::c_int as yytype_int8,
    55 as libc::c_int as yytype_int8,
    105 as libc::c_int as yytype_int8,
    106 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    96 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    49 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    79 as libc::c_int as yytype_int8,
    83 as libc::c_int as yytype_int8,
    84 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    113 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    79 as libc::c_int as yytype_int8,
    86 as libc::c_int as yytype_int8,
    111 as libc::c_int as yytype_int8,
    79 as libc::c_int as yytype_int8,
    69 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    104 as libc::c_int as yytype_int8,
    108 as libc::c_int as yytype_int8,
    105 as libc::c_int as yytype_int8,
    106 as libc::c_int as yytype_int8,
    96 as libc::c_int as yytype_int8,
    98 as libc::c_int as yytype_int8,
    108 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    90 as libc::c_int as yytype_int8,
    113 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    56 as libc::c_int as yytype_int8,
    79 as libc::c_int as yytype_int8,
    108 as libc::c_int as yytype_int8,
];
unsafe extern "C" fn zzlex() -> libc::c_int {
    static mut lexptr: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut lexend: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    let mut c: libc::c_int = 0;
    let mut tokstart: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut toklen: size_t = 0;
    zzlval = 0 as *mut libc::c_void as *mut CMDARG;
    if dbg_errcount > 0 as libc::c_int as libc::c_long && lexptr_begin.is_null() {
        dbg_errcount = 0 as libc::c_int as libc::c_long;
        return '\n' as i32;
    }
    if lexptr_begin.is_null() {
        's_160: {
            loop {
                lexptr_begin = read_a_line.unwrap()(dbg_prompt);
                if lexptr_begin.is_null() {
                    if get_eof_status() == 2 as libc::c_int {
                        exit(2 as libc::c_int);
                    }
                    if get_eof_status() == 1 as libc::c_int {
                        static mut seen_eof: libc::c_int = 0 as libc::c_int;
                        if seen_eof == 0 {
                            if *__errno_location() != 0 as libc::c_int {
                                fprintf(
                                    stderr,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"cannot read command: %s\n\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    strerror(*__errno_location()),
                                );
                                exit_val = 1 as libc::c_int;
                            }
                            seen_eof = 1 as libc::c_int;
                            return '\n' as i32;
                        } else {
                            let fresh3 = seen_eof;
                            seen_eof = seen_eof + 1;
                            if fresh3 == 1 as libc::c_int {
                                cmd_idx = find_command(
                                    b"quit\0" as *const u8 as *const libc::c_char,
                                    4 as libc::c_int as size_t,
                                );
                                return 276 as libc::c_int;
                            } else {
                                return '\n' as i32
                            }
                        }
                    }
                    if *__errno_location() != 0 as libc::c_int {
                        d_error(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"cannot read command: %s\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            strerror(*__errno_location()),
                        );
                    }
                    if pop_cmd_src() == 0 as libc::c_int {
                        continue;
                    }
                    exit(2 as libc::c_int);
                } else {
                    if !in_commands && !in_eval && input_from_tty as libc::c_int != 0 {
                        history_expand_line(&mut lexptr_begin);
                    }
                    lexptr = lexptr_begin;
                    lexend = lexptr.offset(strlen(lexptr) as isize);
                    if *lexptr as libc::c_int == '\0' as i32
                        && repeat_idx >= 0 as libc::c_int
                        && input_from_tty as libc::c_int != 0 && !in_eval
                    {
                        let mut h: *mut HIST_ENTRY = 0 as *mut HIST_ENTRY;
                        h = previous_history();
                        if !h.is_null() {
                            add_history((*h).line);
                        }
                        cmd_idx = repeat_idx;
                        return zz_debug_cmdtab[cmd_idx as usize].lex_class;
                    }
                    repeat_idx = -(1 as libc::c_int);
                    break 's_160;
                }
            }
        }
    }
    c = *lexptr as libc::c_int;
    while c == ' ' as i32 || c == '\t' as i32 {
        lexptr = lexptr.offset(1);
        c = *lexptr as libc::c_int;
    }
    if !input_from_tty && c == '#' as i32 {
        return '\n' as i32;
    }
    tokstart = lexptr;
    if lexptr >= lexend {
        return '\n' as i32;
    }
    if cmd_idx < 0 as libc::c_int {
        if c == '?' as i32
            && *tokstart.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
            && !in_eval
        {
            lexptr = lexptr.offset(1);
            lexptr;
            cmd_idx = find_command(
                b"help\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as size_t,
            );
            return 268 as libc::c_int;
        }
        while c != '\0' as i32 && c != ' ' as i32 && c != '\t' as i32 {
            if !is_alpha(c) && !in_eval {
                zzerror(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"invalid character in command\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                return '\n' as i32;
            }
            lexptr = lexptr.offset(1);
            c = *lexptr as libc::c_int;
        }
        toklen = lexptr.offset_from(tokstart) as libc::c_long as size_t;
        if in_eval {
            if toklen == 3 as libc::c_int as libc::c_ulong
                && *tokstart.offset(3 as libc::c_int as isize) as libc::c_int
                    == '\0' as i32
                && *tokstart.offset(0 as libc::c_int as isize) as libc::c_int
                    == 'e' as i32
                && *tokstart.offset(1 as libc::c_int as isize) as libc::c_int
                    == 'n' as i32
                && *tokstart.offset(2 as libc::c_int as isize) as libc::c_int
                    == 'd' as i32
            {
                cmd_idx = find_command(tokstart, toklen);
                return 297 as libc::c_int;
            }
            lexptr = lexend;
            return 303 as libc::c_int;
        }
        cmd_idx = find_command(tokstart, toklen);
        if cmd_idx >= 0 as libc::c_int {
            if in_commands as libc::c_int != 0
                && zz_debug_cmdtab[cmd_idx as usize].type_0 as libc::c_uint
                    != D_eval as libc::c_int as libc::c_uint
            {
                let mut arg: *mut CMDARG = 0 as *mut CMDARG;
                arg = mk_cmdarg(D_string);
                (*arg)
                    .value
                    .sval = estrdup(
                    lexptr_begin,
                    lexend.offset_from(lexptr_begin) as libc::c_long as size_t,
                );
                append_cmdarg(arg);
            }
            return zz_debug_cmdtab[cmd_idx as usize].lex_class;
        } else {
            zzerror(
                dcgettext(
                    0 as *const libc::c_char,
                    b"unknown command - `%.*s', try help\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                toklen,
                tokstart,
            );
            return '\n' as i32;
        }
    }
    c = *lexptr as libc::c_int;
    if zz_debug_cmdtab[cmd_idx as usize].type_0 as libc::c_uint
        == D_option as libc::c_int as libc::c_uint
    {
        if c == '=' as i32 {
            let fresh4 = lexptr;
            lexptr = lexptr.offset(1);
            return *fresh4 as libc::c_int;
        }
    } else if c == '-' as i32 || c == '+' as i32 || c == ':' as i32 || c == '|' as i32 {
        let fresh5 = lexptr;
        lexptr = lexptr.offset(1);
        return *fresh5 as libc::c_int;
    }
    if c == '"' as i32 {
        let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut flags: libc::c_int = 2 as libc::c_int;
        let mut esc_seen: bool = 0 as libc::c_int != 0;
        toklen = lexend.offset_from(lexptr) as libc::c_long as size_t;
        str = emalloc_real(
            toklen.wrapping_add(1 as libc::c_int as libc::c_ulong),
            b"yylex\0" as *const u8 as *const libc::c_char,
            b"str\0" as *const u8 as *const libc::c_char,
            b"command.y\0" as *const u8 as *const libc::c_char,
            1181 as libc::c_int,
        ) as *mut libc::c_char;
        p = str;
        loop {
            lexptr = lexptr.offset(1);
            c = *lexptr as libc::c_int;
            if !(c != '"' as i32) {
                break;
            }
            's_411: {
                if !(lexptr == lexend) {
                    if c == '\\' as i32 {
                        lexptr = lexptr.offset(1);
                        c = *lexptr as libc::c_int;
                        esc_seen = 1 as libc::c_int != 0;
                        if want_nodeval as libc::c_int != 0 || c != '"' as i32 {
                            let fresh6 = p;
                            p = p.offset(1);
                            *fresh6 = '\\' as i32 as libc::c_char;
                        }
                    }
                    if !(lexptr == lexend) {
                        let fresh7 = p;
                        p = p.offset(1);
                        *fresh7 = c as libc::c_char;
                        break 's_411;
                    }
                }
                free(str as *mut libc::c_void);
                zzerror(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"unterminated string\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                return '\n' as i32;
            }
        }
        lexptr = lexptr.offset(1);
        lexptr;
        *p = '\0' as i32 as libc::c_char;
        if !want_nodeval {
            zzlval = mk_cmdarg(D_string);
            (*zzlval).value.sval = str;
            append_cmdarg(zzlval);
            return 292 as libc::c_int;
        } else {
            if esc_seen {
                flags |= 1 as libc::c_int;
            }
            zzlval = mk_cmdarg(D_node);
            (*zzlval)
                .value
                .nodeval = make_str_node(
                str,
                p.offset_from(str) as libc::c_long as size_t,
                flags,
            );
            append_cmdarg(zzlval);
            return 293 as libc::c_int;
        }
    }
    if !want_nodeval {
        loop {
            lexptr = lexptr.offset(1);
            c = *lexptr as libc::c_int;
            if !(c != '\0' as i32 && c != ':' as i32 && c != '-' as i32
                && c != ' ' as i32 && c != '\t' as i32 && c != '=' as i32)
            {
                break;
            }
        }
        if *(*__ctype_b_loc())
            .offset(
                *tokstart.offset(0 as libc::c_int as isize) as libc::c_uchar
                    as libc::c_int as isize,
            ) as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
            && zz_debug_cmdtab[cmd_idx as usize].type_0 as libc::c_uint
                != D_option as libc::c_int as libc::c_uint
        {
            let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut l: libc::c_long = 0;
            *__errno_location() = 0 as libc::c_int;
            l = strtol(tokstart, &mut end, 0 as libc::c_int);
            if *__errno_location() != 0 as libc::c_int {
                zzerror(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    strerror(*__errno_location()),
                );
                *__errno_location() = 0 as libc::c_int;
                return '\n' as i32;
            }
            if lexptr == end {
                zzlval = mk_cmdarg(D_int);
                (*zzlval).value.lval = l;
                append_cmdarg(zzlval);
                return 291 as libc::c_int;
            }
        }
        zzlval = mk_cmdarg(D_string);
        (*zzlval)
            .value
            .sval = estrdup(
            tokstart,
            lexptr.offset_from(tokstart) as libc::c_long as size_t,
        );
        append_cmdarg(zzlval);
        return 292 as libc::c_int;
    }
    if *(*__ctype_b_loc())
        .offset(
            *tokstart.offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                as isize,
        ) as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        let mut r: *mut NODE = 0 as *mut NODE;
        *__errno_location() = 0 as libc::c_int;
        r = make_number.unwrap()(strtod(tokstart, &mut lexptr));
        if *__errno_location() != 0 as libc::c_int {
            zzerror(strerror(*__errno_location()));
            unref(r);
            *__errno_location() = 0 as libc::c_int;
            return '\n' as i32;
        }
        zzlval = mk_cmdarg(D_node);
        (*zzlval).value.nodeval = r;
        append_cmdarg(zzlval);
        return 293 as libc::c_int;
    }
    c = *lexptr as libc::c_int;
    if c == '$' as i32 || c == '@' as i32 || c == '[' as i32 || c == ']' as i32
        || c == ',' as i32 || c == '=' as i32
    {
        let fresh8 = lexptr;
        lexptr = lexptr.offset(1);
        return *fresh8 as libc::c_int;
    }
    if !is_letter(c) {
        zzerror(
            dcgettext(
                0 as *const libc::c_char,
                b"invalid character\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return '\n' as i32;
    }
    while is_identchar(c) {
        lexptr = lexptr.offset(1);
        c = *lexptr as libc::c_int;
    }
    toklen = lexptr.offset_from(tokstart) as libc::c_long as size_t;
    zzlval = mk_cmdarg(D_variable);
    (*zzlval).value.sval = estrdup(tokstart, toklen);
    append_cmdarg(zzlval);
    return 294 as libc::c_int;
}
static mut yyr1: [yytype_int8; 157] = [
    0 as libc::c_int as yytype_int8,
    59 as libc::c_int as yytype_int8,
    60 as libc::c_int as yytype_int8,
    60 as libc::c_int as yytype_int8,
    61 as libc::c_int as yytype_int8,
    61 as libc::c_int as yytype_int8,
    61 as libc::c_int as yytype_int8,
    62 as libc::c_int as yytype_int8,
    62 as libc::c_int as yytype_int8,
    62 as libc::c_int as yytype_int8,
    62 as libc::c_int as yytype_int8,
    62 as libc::c_int as yytype_int8,
    63 as libc::c_int as yytype_int8,
    63 as libc::c_int as yytype_int8,
    63 as libc::c_int as yytype_int8,
    63 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    65 as libc::c_int as yytype_int8,
    65 as libc::c_int as yytype_int8,
    66 as libc::c_int as yytype_int8,
    67 as libc::c_int as yytype_int8,
    68 as libc::c_int as yytype_int8,
    69 as libc::c_int as yytype_int8,
    68 as libc::c_int as yytype_int8,
    70 as libc::c_int as yytype_int8,
    70 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    72 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    73 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    74 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    75 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    76 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    77 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    78 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    79 as libc::c_int as yytype_int8,
    80 as libc::c_int as yytype_int8,
    80 as libc::c_int as yytype_int8,
    81 as libc::c_int as yytype_int8,
    81 as libc::c_int as yytype_int8,
    82 as libc::c_int as yytype_int8,
    82 as libc::c_int as yytype_int8,
    82 as libc::c_int as yytype_int8,
    82 as libc::c_int as yytype_int8,
    83 as libc::c_int as yytype_int8,
    83 as libc::c_int as yytype_int8,
    83 as libc::c_int as yytype_int8,
    84 as libc::c_int as yytype_int8,
    85 as libc::c_int as yytype_int8,
    85 as libc::c_int as yytype_int8,
    85 as libc::c_int as yytype_int8,
    86 as libc::c_int as yytype_int8,
    87 as libc::c_int as yytype_int8,
    87 as libc::c_int as yytype_int8,
    87 as libc::c_int as yytype_int8,
    87 as libc::c_int as yytype_int8,
    87 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    89 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    90 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    91 as libc::c_int as yytype_int8,
    91 as libc::c_int as yytype_int8,
    92 as libc::c_int as yytype_int8,
    92 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    94 as libc::c_int as yytype_int8,
    94 as libc::c_int as yytype_int8,
    95 as libc::c_int as yytype_int8,
    95 as libc::c_int as yytype_int8,
    96 as libc::c_int as yytype_int8,
    96 as libc::c_int as yytype_int8,
    96 as libc::c_int as yytype_int8,
    97 as libc::c_int as yytype_int8,
    97 as libc::c_int as yytype_int8,
    97 as libc::c_int as yytype_int8,
    97 as libc::c_int as yytype_int8,
    98 as libc::c_int as yytype_int8,
    98 as libc::c_int as yytype_int8,
    99 as libc::c_int as yytype_int8,
    99 as libc::c_int as yytype_int8,
    99 as libc::c_int as yytype_int8,
    100 as libc::c_int as yytype_int8,
    100 as libc::c_int as yytype_int8,
    100 as libc::c_int as yytype_int8,
    100 as libc::c_int as yytype_int8,
    100 as libc::c_int as yytype_int8,
    100 as libc::c_int as yytype_int8,
    100 as libc::c_int as yytype_int8,
    100 as libc::c_int as yytype_int8,
    100 as libc::c_int as yytype_int8,
    101 as libc::c_int as yytype_int8,
    102 as libc::c_int as yytype_int8,
    102 as libc::c_int as yytype_int8,
    102 as libc::c_int as yytype_int8,
    103 as libc::c_int as yytype_int8,
    103 as libc::c_int as yytype_int8,
    103 as libc::c_int as yytype_int8,
    103 as libc::c_int as yytype_int8,
    104 as libc::c_int as yytype_int8,
    104 as libc::c_int as yytype_int8,
    104 as libc::c_int as yytype_int8,
    105 as libc::c_int as yytype_int8,
    105 as libc::c_int as yytype_int8,
    106 as libc::c_int as yytype_int8,
    106 as libc::c_int as yytype_int8,
    107 as libc::c_int as yytype_int8,
    107 as libc::c_int as yytype_int8,
    107 as libc::c_int as yytype_int8,
    108 as libc::c_int as yytype_int8,
    108 as libc::c_int as yytype_int8,
    108 as libc::c_int as yytype_int8,
    109 as libc::c_int as yytype_int8,
    109 as libc::c_int as yytype_int8,
    110 as libc::c_int as yytype_int8,
    110 as libc::c_int as yytype_int8,
    111 as libc::c_int as yytype_int8,
    111 as libc::c_int as yytype_int8,
    112 as libc::c_int as yytype_int8,
    112 as libc::c_int as yytype_int8,
    112 as libc::c_int as yytype_int8,
    113 as libc::c_int as yytype_int8,
];
static mut yyr2: [yytype_int8; 157] = [
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
];
unsafe extern "C" fn yydestruct(
    mut yymsg: *const libc::c_char,
    mut yykind: yysymbol_kind_t,
    mut yyvaluep: *mut *mut CMDARG,
) {
    if yymsg.is_null() {
        yymsg = b"Deleting\0" as *const u8 as *const libc::c_char;
    }
}
pub static mut zzchar: libc::c_int = 0;
pub static mut zzlval: *mut CMDARG = 0 as *const CMDARG as *mut CMDARG;
pub static mut zznerrs: libc::c_int = 0;
pub unsafe extern "C" fn zzparse() -> libc::c_int {
    let mut current_block: u64;
    let mut yystate: yy_state_fast_t = 0 as libc::c_int;
    let mut yyerrstatus: libc::c_int = 0 as libc::c_int;
    let mut yystacksize: libc::c_long = 200 as libc::c_int as libc::c_long;
    let mut yyssa: [yy_state_t; 200] = [0; 200];
    let mut yyss: *mut yy_state_t = yyssa.as_mut_ptr();
    let mut yyssp: *mut yy_state_t = yyss;
    let mut yyvsa: [*mut CMDARG; 200] = [0 as *mut CMDARG; 200];
    let mut yyvs: *mut *mut CMDARG = yyvsa.as_mut_ptr();
    let mut yyvsp: *mut *mut CMDARG = yyvs;
    let mut yyn: libc::c_int = 0;
    let mut yyresult: libc::c_int = 0;
    let mut yytoken: yysymbol_kind_t = YYSYMBOL_YYEMPTY;
    let mut yyval: *mut CMDARG = 0 as *mut CMDARG;
    let mut yylen: libc::c_int = 0 as libc::c_int;
    zzchar = -(2 as libc::c_int);
    's_46: loop {
        (0 as libc::c_int != 0
            && (0 as libc::c_int <= yystate && yystate < 203 as libc::c_int))
            as libc::c_int;
        *yyssp = yystate as yy_state_t;
        if yyss.offset(yystacksize as isize).offset(-(1 as libc::c_int as isize))
            <= yyssp
        {
            let mut yysize: libc::c_long = yyssp.offset_from(yyss) as libc::c_long
                + 1 as libc::c_int as libc::c_long;
            if 10000 as libc::c_int as libc::c_long <= yystacksize {
                current_block = 5184769617358624772;
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
                        + ::std::mem::size_of::<*mut CMDARG>() as libc::c_ulong
                            as libc::c_long)
                    + (::std::mem::size_of::<yyalloc>() as libc::c_ulong as libc::c_long
                        - 1 as libc::c_int as libc::c_long)) as libc::c_ulong,
            ) as *mut yyalloc;
            if yyptr.is_null() {
                current_block = 5184769617358624772;
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
                &mut (*yyptr).yyvs_alloc as *mut *mut CMDARG as *mut libc::c_void,
                yyvs as *const libc::c_void,
                (yysize as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<*mut CMDARG>() as libc::c_ulong)
                    as libc::size_t,
            );
            yyvs = &mut (*yyptr).yyvs_alloc;
            yynewbytes_0 = yystacksize
                * ::std::mem::size_of::<*mut CMDARG>() as libc::c_ulong as libc::c_long
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
                current_block = 17450030176436579763;
                break;
            }
        }
        if yystate == 2 as libc::c_int {
            current_block = 7232461487979279659;
            break;
        }
        yyn = yypact[yystate as usize] as libc::c_int;
        if yyn == -(151 as libc::c_int) {
            current_block = 12926286301195594175;
        } else {
            if zzchar == -(2 as libc::c_int) {
                zzchar = zzlex();
            }
            if zzchar <= 0 as libc::c_int {
                zzchar = 0 as libc::c_int;
                yytoken = YYSYMBOL_YYEOF;
                current_block = 6174974146017752131;
            } else if zzchar == 256 as libc::c_int {
                zzchar = 257 as libc::c_int;
                yytoken = YYSYMBOL_YYerror;
                current_block = 13974732688382125050;
            } else {
                yytoken = (if 0 as libc::c_int <= zzchar && zzchar <= 303 as libc::c_int
                {
                    yytranslate[zzchar as usize] as yysymbol_kind_t as libc::c_int
                } else {
                    YYSYMBOL_YYUNDEF as libc::c_int
                }) as yysymbol_kind_t;
                current_block = 6174974146017752131;
            }
            match current_block {
                13974732688382125050 => {}
                _ => {
                    yyn += yytoken as libc::c_int;
                    if yyn < 0 as libc::c_int || (203 as libc::c_int) < yyn
                        || yycheck[yyn as usize] as libc::c_int != yytoken as libc::c_int
                    {
                        current_block = 12926286301195594175;
                    } else {
                        yyn = yytable[yyn as usize] as libc::c_int;
                        if yyn <= 0 as libc::c_int {
                            yyn = -yyn;
                            current_block = 11820060424501343596;
                        } else {
                            if yyerrstatus != 0 {
                                yyerrstatus -= 1;
                                yyerrstatus;
                            }
                            yystate = yyn;
                            yyvsp = yyvsp.offset(1);
                            *yyvsp = zzlval;
                            zzchar = -(2 as libc::c_int);
                            current_block = 6048750584474863085;
                        }
                    }
                }
            }
        }
        match current_block {
            12926286301195594175 => {
                yyn = yydefact[yystate as usize] as libc::c_int;
                if yyn == 0 as libc::c_int {
                    yytoken = (if zzchar == -(2 as libc::c_int) {
                        YYSYMBOL_YYEMPTY as libc::c_int
                    } else if 0 as libc::c_int <= zzchar && zzchar <= 303 as libc::c_int
                    {
                        yytranslate[zzchar as usize] as yysymbol_kind_t as libc::c_int
                    } else {
                        YYSYMBOL_YYUNDEF as libc::c_int
                    }) as yysymbol_kind_t;
                    if yyerrstatus == 0 {
                        zznerrs += 1;
                        zznerrs;
                        zzerror(b"syntax error\0" as *const u8 as *const libc::c_char);
                    }
                    if yyerrstatus == 3 as libc::c_int {
                        if zzchar <= 0 as libc::c_int {
                            if zzchar == 0 as libc::c_int {
                                current_block = 17450030176436579763;
                                break;
                            }
                        } else {
                            yydestruct(
                                b"Error: discarding\0" as *const u8 as *const libc::c_char,
                                yytoken,
                                &mut zzlval,
                            );
                            zzchar = -(2 as libc::c_int);
                        }
                    }
                    current_block = 13974732688382125050;
                } else {
                    current_block = 11820060424501343596;
                }
            }
            _ => {}
        }
        match current_block {
            11820060424501343596 => {
                yylen = yyr2[yyn as usize] as libc::c_int;
                yyval = *yyvsp.offset((1 as libc::c_int - yylen) as isize);
                match yyn {
                    3 => {
                        cmd_idx = -(1 as libc::c_int);
                        want_nodeval = 0 as libc::c_int != 0;
                        if !lexptr_begin.is_null() {
                            if input_from_tty as libc::c_int != 0
                                && *lexptr_begin.offset(0 as libc::c_int as isize)
                                    as libc::c_int != '\0' as i32
                            {
                                add_history(lexptr_begin);
                            }
                            free(lexptr_begin as *mut libc::c_void);
                            lexptr_begin = 0 as *mut libc::c_char;
                        }
                        if !arg_list.is_null() {
                            free_cmdarg(arg_list);
                            arg_list = 0 as *mut CMDARG;
                        }
                    }
                    5 => {
                        if dbg_errcount == 0 as libc::c_int as libc::c_long
                            && cmd_idx >= 0 as libc::c_int
                        {
                            let mut cmdfunc: Func_cmd = None;
                            let mut terminate: bool = 0 as libc::c_int != 0;
                            let mut args: *mut CMDARG = 0 as *mut CMDARG;
                            let mut ctype: libc::c_int = 0 as libc::c_int;
                            ctype = (*zz_debug_cmdtab
                                .as_mut_ptr()
                                .offset(cmd_idx as isize))
                                .type_0 as libc::c_int;
                            if (ctype == D_list as libc::c_int
                                || ctype == D_next as libc::c_int
                                || ctype == D_step as libc::c_int
                                || ctype == D_nexti as libc::c_int
                                || ctype == D_stepi as libc::c_int
                                || ctype == D_continue as libc::c_int) && arg_list.is_null()
                                && !in_commands && input_from_tty as libc::c_int != 0
                            {
                                repeat_idx = cmd_idx;
                            } else {
                                repeat_idx = -(1 as libc::c_int);
                            }
                            cmdfunc = (*zz_debug_cmdtab
                                .as_mut_ptr()
                                .offset(cmd_idx as isize))
                                .cf_ptr;
                            if in_commands {
                                cmdfunc = Some(
                                    do_commands
                                        as unsafe extern "C" fn(
                                            *mut CMDARG,
                                            libc::c_int,
                                        ) -> libc::c_int,
                                );
                            }
                            cmd_idx = -(1 as libc::c_int);
                            want_nodeval = 0 as libc::c_int != 0;
                            args = arg_list;
                            arg_list = 0 as *mut CMDARG;
                            terminate = (Some(cmdfunc.unwrap())).unwrap()(args, ctype)
                                != 0;
                            if !in_commands || ctype == D_commands as libc::c_int {
                                free_cmdarg(args);
                            }
                            if terminate {
                                current_block = 7232461487979279659;
                                break;
                            }
                        }
                    }
                    6 => {
                        yyerrstatus = 0 as libc::c_int;
                    }
                    22 => {
                        want_nodeval = 1 as libc::c_int != 0;
                    }
                    23 => {
                        if dbg_errcount == 0 as libc::c_int as libc::c_long {
                            if input_from_tty {
                                dbg_prompt = eval_prompt;
                                fprintf(
                                    out_fp,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"Type (g)awk statement(s). End with the command `end'\n\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                rl_inhibit_completion = 1 as libc::c_int;
                            }
                            cmd_idx = -(1 as libc::c_int);
                            in_eval = 1 as libc::c_int != 0;
                        }
                    }
                    24 => {
                        yyval = append_statement(
                            arg_list,
                            start_EVAL.as_ptr() as *mut libc::c_char,
                        );
                        if read_a_line
                            == Some(
                                read_commands_string
                                    as unsafe extern "C" fn(
                                        *const libc::c_char,
                                    ) -> *mut libc::c_char,
                            )
                        {
                            *((*yyval).value.sval)
                                .offset(
                                    0 as libc::c_int as isize,
                                ) = '\0' as i32 as libc::c_char;
                        }
                        free_cmdarg(arg_list);
                        arg_list = 0 as *mut CMDARG;
                    }
                    25 => {
                        yyval = append_statement(
                            *yyvsp.offset(-(1 as libc::c_int) as isize),
                            lexptr_begin,
                        );
                    }
                    26 => {
                        yyval = *yyvsp.offset(-(1 as libc::c_int) as isize);
                    }
                    27 => {
                        arg_list = append_statement(
                            *yyvsp.offset(-(1 as libc::c_int) as isize),
                            end_EVAL.as_ptr() as *mut libc::c_char,
                        );
                        if read_a_line
                            == Some(
                                read_commands_string
                                    as unsafe extern "C" fn(
                                        *const libc::c_char,
                                    ) -> *mut libc::c_char,
                            )
                        {
                            let mut str: *mut libc::c_char = (*arg_list).value.sval;
                            let mut len: size_t = strlen(str);
                            *str
                                .offset(
                                    len.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize,
                                ) = '\0' as i32 as libc::c_char;
                        }
                        if input_from_tty {
                            dbg_prompt = if in_commands as libc::c_int != 0 {
                                commands_prompt
                            } else {
                                dgawk_prompt
                            };
                            rl_inhibit_completion = 0 as libc::c_int;
                        }
                        cmd_idx = find_command(
                            b"eval\0" as *const u8 as *const libc::c_char,
                            4 as libc::c_int as size_t,
                        );
                        in_eval = 0 as libc::c_int != 0;
                    }
                    28 => {
                        let mut n: *mut NODE = 0 as *mut NODE;
                        let mut arg: *mut CMDARG = 0 as *mut CMDARG;
                        n = (**yyvsp.offset(0 as libc::c_int as isize)).value.nodeval;
                        arg = append_statement(
                            0 as *mut CMDARG,
                            start_EVAL.as_ptr() as *mut libc::c_char,
                        );
                        append_statement(arg, (*n).sub.val.sp);
                        append_statement(arg, end_EVAL.as_ptr() as *mut libc::c_char);
                        free_cmdarg(arg_list);
                        arg_list = arg;
                    }
                    34 => {
                        if (*zz_debug_cmdtab.as_mut_ptr().offset(cmd_idx as isize))
                            .lex_class == 267 as libc::c_int
                            && !(*yyvsp.offset(0 as libc::c_int as isize)).is_null()
                            && (**yyvsp.offset(0 as libc::c_int as isize)).value.lval
                                < 0 as libc::c_int as libc::c_long
                        {
                            zzerror(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"invalid frame number: %d\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                (**yyvsp.offset(0 as libc::c_int as isize)).value.lval,
                            );
                        }
                    }
                    35 => {
                        let mut idx: libc::c_int = find_argument(
                            *yyvsp.offset(0 as libc::c_int as isize),
                        );
                        if idx < 0 as libc::c_int {
                            zzerror(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"info: invalid option - `%s'\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                (**yyvsp.offset(0 as libc::c_int as isize)).value.sval,
                            );
                        } else {
                            free(
                                (**yyvsp.offset(0 as libc::c_int as isize)).value.sval
                                    as *mut libc::c_void,
                            );
                            let ref mut fresh9 = (**yyvsp
                                .offset(0 as libc::c_int as isize))
                                .value
                                .sval;
                            *fresh9 = 0 as *mut libc::c_char;
                            (**yyvsp.offset(0 as libc::c_int as isize))
                                .type_0 = D_argument;
                            (**yyvsp.offset(0 as libc::c_int as isize))
                                .value
                                .lval = (*zz_debug_argtab.as_mut_ptr().offset(idx as isize))
                                .value as libc::c_long;
                        }
                    }
                    38 => {
                        want_nodeval = 1 as libc::c_int != 0;
                    }
                    40 => {
                        want_nodeval = 1 as libc::c_int != 0;
                    }
                    46 => {
                        want_nodeval = 1 as libc::c_int != 0;
                    }
                    49 => {
                        want_nodeval = 1 as libc::c_int != 0;
                    }
                    51 => {
                        want_nodeval = 1 as libc::c_int != 0;
                    }
                    53 => {
                        want_nodeval = 1 as libc::c_int != 0;
                    }
                    57 => {
                        if in_cmd_src(
                            (**yyvsp.offset(0 as libc::c_int as isize)).value.sval,
                        ) != 0
                        {
                            zzerror(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"source: `%s': already sourced\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                (**yyvsp.offset(0 as libc::c_int as isize)).value.sval,
                            );
                        }
                    }
                    58 => {
                        if !input_from_tty {
                            zzerror(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"save: `%s': command not permitted\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                (**yyvsp.offset(0 as libc::c_int as isize)).value.sval,
                            );
                        }
                    }
                    59 => {
                        let mut type_0: libc::c_int = 0 as libc::c_int;
                        let mut num: libc::c_int = 0;
                        if !(*yyvsp.offset(0 as libc::c_int as isize)).is_null() {
                            num = (**yyvsp.offset(0 as libc::c_int as isize)).value.lval
                                as libc::c_int;
                        }
                        if !(dbg_errcount != 0 as libc::c_int as libc::c_long) {
                            if in_commands {
                                zzerror(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"cannot use command `commands' for breakpoint/watchpoint commands\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            } else if (*yyvsp.offset(0 as libc::c_int as isize))
                                .is_null()
                                && {
                                    type_0 = has_break_or_watch_point(
                                        &mut num,
                                        1 as libc::c_int != 0,
                                    );
                                    type_0 == 0
                                }
                            {
                                zzerror(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"no breakpoint/watchpoint has been set yet\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            } else if !(*yyvsp.offset(0 as libc::c_int as isize))
                                .is_null()
                                && {
                                    type_0 = has_break_or_watch_point(
                                        &mut num,
                                        0 as libc::c_int != 0,
                                    );
                                    type_0 == 0
                                }
                            {
                                zzerror(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"invalid breakpoint/watchpoint number\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                        }
                        if type_0 != 0 {
                            in_commands = 1 as libc::c_int != 0;
                            if input_from_tty {
                                dbg_prompt = commands_prompt;
                                fprintf(
                                    out_fp,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"Type commands for when %s %d is hit, one per line.\n\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    if type_0 == D_break as libc::c_int {
                                        b"breakpoint\0" as *const u8 as *const libc::c_char
                                    } else {
                                        b"watchpoint\0" as *const u8 as *const libc::c_char
                                    },
                                    num,
                                );
                                fprintf(
                                    out_fp,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"End with the command `end'\n\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                        }
                    }
                    60 => {
                        if !in_commands {
                            zzerror(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"`end' valid only in command `commands' or `eval'\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                        } else {
                            if input_from_tty {
                                dbg_prompt = dgawk_prompt;
                            }
                            in_commands = 0 as libc::c_int != 0;
                        }
                    }
                    61 => {
                        if !in_commands {
                            zzerror(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"`silent' valid only in command `commands'\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                        }
                    }
                    62 => {
                        let mut idx_0: libc::c_int = find_argument(
                            *yyvsp.offset(0 as libc::c_int as isize),
                        );
                        if idx_0 < 0 as libc::c_int {
                            zzerror(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"trace: invalid option - `%s'\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                (**yyvsp.offset(0 as libc::c_int as isize)).value.sval,
                            );
                        } else {
                            free(
                                (**yyvsp.offset(0 as libc::c_int as isize)).value.sval
                                    as *mut libc::c_void,
                            );
                            let ref mut fresh10 = (**yyvsp
                                .offset(0 as libc::c_int as isize))
                                .value
                                .sval;
                            *fresh10 = 0 as *mut libc::c_char;
                            (**yyvsp.offset(0 as libc::c_int as isize))
                                .type_0 = D_argument;
                            (**yyvsp.offset(0 as libc::c_int as isize))
                                .value
                                .lval = (*zz_debug_argtab
                                .as_mut_ptr()
                                .offset(idx_0 as isize))
                                .value as libc::c_long;
                        }
                    }
                    63 => {
                        want_nodeval = 1 as libc::c_int != 0;
                    }
                    64 => {
                        let mut type_1: libc::c_int = 0;
                        let mut num_0: libc::c_int = (**yyvsp
                            .offset(-(2 as libc::c_int) as isize))
                            .value
                            .lval as libc::c_int;
                        type_1 = has_break_or_watch_point(
                            &mut num_0,
                            0 as libc::c_int != 0,
                        );
                        if type_1 == 0 {
                            zzerror(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"condition: invalid breakpoint/watchpoint number\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                        }
                    }
                    65 => {
                        if in_commands {
                            let mut arg_0: *mut CMDARG = 0 as *mut CMDARG;
                            arg_0 = mk_cmdarg(D_string);
                            (*arg_0)
                                .value
                                .sval = estrdup(
                                b"eval\0" as *const u8 as *const libc::c_char,
                                4 as libc::c_int as size_t,
                            );
                            (*arg_0).next = arg_list;
                            arg_list = arg_0;
                        }
                    }
                    66 => {
                        if !(*yyvsp.offset(0 as libc::c_int as isize)).is_null() {
                            let mut n_0: *mut NODE = (**yyvsp
                                .offset(0 as libc::c_int as isize))
                                .value
                                .nodeval;
                            (**yyvsp.offset(0 as libc::c_int as isize))
                                .type_0 = D_string;
                            let ref mut fresh11 = (**yyvsp
                                .offset(0 as libc::c_int as isize))
                                .value
                                .sval;
                            *fresh11 = (*n_0).sub.val.sp;
                            let ref mut fresh12 = (*(n_0 as *mut block_item)).freep;
                            *fresh12 = nextfree[BLOCK_NODE as libc::c_int as usize]
                                .freep;
                            nextfree[BLOCK_NODE as libc::c_int as usize]
                                .freep = n_0 as *mut block_item;
                        }
                        yyval = *yyvsp.offset(0 as libc::c_int as isize);
                    }
                    68 => {
                        yyval = 0 as *mut CMDARG;
                    }
                    69 => {
                        yyval = 0 as *mut CMDARG;
                    }
                    74 => {
                        yyval = 0 as *mut CMDARG;
                    }
                    75 => {
                        yyval = 0 as *mut CMDARG;
                    }
                    77 => {
                        yyval = 0 as *mut CMDARG;
                    }
                    78 => {
                        let mut n_1: *mut NODE = 0 as *mut NODE;
                        n_1 = (**yyvsp.offset(0 as libc::c_int as isize)).value.nodeval;
                        if (*n_1).flags as libc::c_uint
                            & STRING as libc::c_int as libc::c_uint
                            == 0 as libc::c_int as libc::c_uint
                        {
                            zzerror(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"argument not a string\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                        }
                    }
                    79 => {
                        yyval = 0 as *mut CMDARG;
                    }
                    80 => {
                        if find_option(
                            (**yyvsp.offset(0 as libc::c_int as isize)).value.sval,
                        ) < 0 as libc::c_int
                        {
                            zzerror(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"option: invalid parameter - `%s'\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                (**yyvsp.offset(0 as libc::c_int as isize)).value.sval,
                            );
                        }
                    }
                    81 => {
                        if find_option(
                            (**yyvsp.offset(-(2 as libc::c_int) as isize)).value.sval,
                        ) < 0 as libc::c_int
                        {
                            zzerror(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"option: invalid parameter - `%s'\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                (**yyvsp.offset(-(2 as libc::c_int) as isize)).value.sval,
                            );
                        }
                    }
                    82 => {
                        let mut n_2: *mut NODE = 0 as *mut NODE;
                        n_2 = lookup(
                            (**yyvsp.offset(0 as libc::c_int as isize)).value.sval,
                        );
                        if n_2.is_null()
                            || (*n_2).type_0 as libc::c_uint
                                != Node_func as libc::c_int as libc::c_uint
                        {
                            zzerror(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"no such function - `%s'\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                (**yyvsp.offset(0 as libc::c_int as isize)).value.sval,
                            );
                        } else {
                            (**yyvsp.offset(0 as libc::c_int as isize)).type_0 = D_func;
                            free(
                                (**yyvsp.offset(0 as libc::c_int as isize)).value.sval
                                    as *mut libc::c_void,
                            );
                            let ref mut fresh13 = (**yyvsp
                                .offset(0 as libc::c_int as isize))
                                .value
                                .sval;
                            *fresh13 = 0 as *mut libc::c_char;
                            let ref mut fresh14 = (**yyvsp
                                .offset(0 as libc::c_int as isize))
                                .value
                                .nodeval;
                            *fresh14 = n_2;
                        }
                    }
                    83 => {
                        yyval = 0 as *mut CMDARG;
                    }
                    88 => {
                        yyval = 0 as *mut CMDARG;
                    }
                    89 => {
                        want_nodeval = 1 as libc::c_int != 0;
                    }
                    92 => {
                        want_nodeval = 1 as libc::c_int != 0;
                    }
                    95 => {
                        yyval = 0 as *mut CMDARG;
                    }
                    97 => {
                        yyval = 0 as *mut CMDARG;
                    }
                    99 => {
                        yyval = 0 as *mut CMDARG;
                    }
                    104 => {
                        let mut idx_1: libc::c_int = find_argument(
                            *yyvsp.offset(-(1 as libc::c_int) as isize),
                        );
                        if idx_1 < 0 as libc::c_int {
                            zzerror(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"enable: invalid option - `%s'\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                (**yyvsp.offset(-(1 as libc::c_int) as isize)).value.sval,
                            );
                        } else {
                            free(
                                (**yyvsp.offset(-(1 as libc::c_int) as isize)).value.sval
                                    as *mut libc::c_void,
                            );
                            let ref mut fresh15 = (**yyvsp
                                .offset(-(1 as libc::c_int) as isize))
                                .value
                                .sval;
                            *fresh15 = 0 as *mut libc::c_char;
                            (**yyvsp.offset(-(1 as libc::c_int) as isize))
                                .type_0 = D_argument;
                            (**yyvsp.offset(-(1 as libc::c_int) as isize))
                                .value
                                .lval = (*zz_debug_argtab
                                .as_mut_ptr()
                                .offset(idx_1 as isize))
                                .value as libc::c_long;
                        }
                    }
                    106 => {
                        (**yyvsp.offset(0 as libc::c_int as isize)).type_0 = D_array;
                        (**yyvsp.offset(0 as libc::c_int as isize))
                            .a_count = 0 as libc::c_int;
                    }
                    107 => {
                        (**yyvsp.offset(-(1 as libc::c_int) as isize)).type_0 = D_array;
                        (**yyvsp.offset(-(1 as libc::c_int) as isize)).a_count = num_dim;
                    }
                    117 => {
                        yyval = 0 as *mut CMDARG;
                    }
                    118 => {
                        yyval = 0 as *mut CMDARG;
                    }
                    119 => {
                        let mut a: *mut CMDARG = 0 as *mut CMDARG;
                        a = mk_cmdarg(D_int);
                        (*a).value.lval = -(1 as libc::c_int) as libc::c_long;
                        append_cmdarg(a);
                    }
                    126 => {
                        if (**yyvsp.offset(-(2 as libc::c_int) as isize)).value.lval
                            > (**yyvsp.offset(0 as libc::c_int as isize)).value.lval
                        {
                            zzerror(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"invalid range specification: %d - %d\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                (**yyvsp.offset(-(2 as libc::c_int) as isize)).value.lval,
                                (**yyvsp.offset(0 as libc::c_int as isize)).value.lval,
                            );
                        } else {
                            (**yyvsp.offset(-(2 as libc::c_int) as isize))
                                .type_0 = D_range;
                        }
                        yyval = *yyvsp.offset(-(2 as libc::c_int) as isize);
                    }
                    127 => {
                        yyval = 0 as *mut CMDARG;
                    }
                    134 => {
                        yyval = *yyvsp.offset(0 as libc::c_int as isize);
                    }
                    135 => {
                        yyval = *yyvsp.offset(-(2 as libc::c_int) as isize);
                    }
                    137 => {
                        let mut a_0: *mut CMDARG = 0 as *mut CMDARG;
                        let mut subs: *mut NODE = 0 as *mut NODE;
                        let mut count: libc::c_int = 0 as libc::c_int;
                        a_0 = *yyvsp.offset(-(1 as libc::c_int) as isize);
                        while !a_0.is_null() {
                            count += 1;
                            count;
                            a_0 = (*a_0).next;
                        }
                        subs = concat_args(
                            *yyvsp.offset(-(1 as libc::c_int) as isize),
                            count,
                        );
                        free_cmdarg((**yyvsp.offset(-(1 as libc::c_int) as isize)).next);
                        let ref mut fresh16 = (**yyvsp
                            .offset(-(1 as libc::c_int) as isize))
                            .next;
                        *fresh16 = 0 as *mut cmd_argument;
                        (**yyvsp.offset(-(1 as libc::c_int) as isize)).type_0 = D_node;
                        let ref mut fresh17 = (**yyvsp
                            .offset(-(1 as libc::c_int) as isize))
                            .value
                            .nodeval;
                        *fresh17 = subs;
                        yyval = *yyvsp.offset(-(1 as libc::c_int) as isize);
                    }
                    139 => {
                        yyval = *yyvsp.offset(0 as libc::c_int as isize);
                        num_dim = 1 as libc::c_int;
                    }
                    140 => {
                        yyval = *yyvsp.offset(-(1 as libc::c_int) as isize);
                        num_dim += 1;
                        num_dim;
                    }
                    142 => {
                        let mut n_3: *mut NODE = (**yyvsp
                            .offset(0 as libc::c_int as isize))
                            .value
                            .nodeval;
                        if (*n_3).flags as libc::c_uint
                            & NUMBER as libc::c_int as libc::c_uint
                            == 0 as libc::c_int as libc::c_uint
                        {
                            zzerror(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"non-numeric value for field number\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                        } else {
                            (**yyvsp.offset(0 as libc::c_int as isize)).type_0 = D_field;
                        }
                        yyval = *yyvsp.offset(0 as libc::c_int as isize);
                    }
                    143 => {
                        (**yyvsp.offset(-(1 as libc::c_int) as isize))
                            .type_0 = D_subscript;
                        (**yyvsp.offset(-(1 as libc::c_int) as isize)).a_count = num_dim;
                        yyval = *yyvsp.offset(-(1 as libc::c_int) as isize);
                    }
                    144 => {
                        yyval = *yyvsp.offset(0 as libc::c_int as isize);
                    }
                    145 => {
                        let mut n_4: *mut NODE = (**yyvsp
                            .offset(0 as libc::c_int as isize))
                            .value
                            .nodeval;
                        if (*n_4).flags as libc::c_uint
                            & NUMBER as libc::c_int as libc::c_uint
                            == 0 as libc::c_int as libc::c_uint
                        {
                            zzerror(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"non-numeric value found, numeric expected\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                        }
                        yyval = *yyvsp.offset(0 as libc::c_int as isize);
                    }
                    146 => {
                        let mut n_5: *mut NODE = (**yyvsp
                            .offset(0 as libc::c_int as isize))
                            .value
                            .nodeval;
                        if (*n_5).flags as libc::c_uint
                            & NUMBER as libc::c_int as libc::c_uint
                            == 0 as libc::c_int as libc::c_uint
                        {
                            zzerror(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"non-numeric value found, numeric expected\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                        } else {
                            negate_num(n_5);
                        }
                        yyval = *yyvsp.offset(0 as libc::c_int as isize);
                    }
                    147 => {
                        yyval = 0 as *mut CMDARG;
                    }
                    148 => {
                        yyval = *yyvsp.offset(0 as libc::c_int as isize);
                    }
                    149 => {
                        yyval = 0 as *mut CMDARG;
                    }
                    150 => {
                        yyval = *yyvsp.offset(0 as libc::c_int as isize);
                    }
                    151 => {
                        if (**yyvsp.offset(0 as libc::c_int as isize)).value.lval
                            == 0 as libc::c_int as libc::c_long
                        {
                            zzerror(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"non-zero integer value\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                        }
                        yyval = *yyvsp.offset(0 as libc::c_int as isize);
                    }
                    152 => {
                        if (**yyvsp.offset(0 as libc::c_int as isize)).value.lval
                            == 0 as libc::c_int as libc::c_long
                        {
                            zzerror(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"non-zero integer value\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                        }
                        yyval = *yyvsp.offset(0 as libc::c_int as isize);
                    }
                    153 => {
                        yyval = *yyvsp.offset(0 as libc::c_int as isize);
                    }
                    154 => {
                        yyval = *yyvsp.offset(0 as libc::c_int as isize);
                    }
                    155 => {
                        (**yyvsp.offset(0 as libc::c_int as isize))
                            .value
                            .lval = -(**yyvsp.offset(0 as libc::c_int as isize))
                            .value
                            .lval;
                        yyval = *yyvsp.offset(0 as libc::c_int as isize);
                    }
                    156 => {
                        if !lexptr_begin.is_null() {
                            if input_from_tty as libc::c_int != 0
                                && *lexptr_begin.offset(0 as libc::c_int as isize)
                                    as libc::c_int != '\0' as i32
                            {
                                add_history(lexptr_begin);
                            }
                            free(lexptr_begin as *mut libc::c_void);
                            lexptr_begin = 0 as *mut libc::c_char;
                        }
                    }
                    _ => {}
                }
                yyvsp = yyvsp.offset(-(yylen as isize));
                yyssp = yyssp.offset(-(yylen as isize));
                yylen = 0 as libc::c_int;
                yyvsp = yyvsp.offset(1);
                *yyvsp = yyval;
                let yylhs: libc::c_int = yyr1[yyn as usize] as libc::c_int
                    - 59 as libc::c_int;
                let yyi: libc::c_int = yypgoto[yylhs as usize] as libc::c_int
                    + *yyssp as libc::c_int;
                yystate = if 0 as libc::c_int <= yyi && yyi <= 203 as libc::c_int
                    && yycheck[yyi as usize] as libc::c_int == *yyssp as libc::c_int
                {
                    yytable[yyi as usize] as libc::c_int
                } else {
                    yydefgoto[yylhs as usize] as libc::c_int
                };
            }
            13974732688382125050 => {
                yyerrstatus = 3 as libc::c_int;
                loop {
                    yyn = yypact[yystate as usize] as libc::c_int;
                    if !(yyn == -(151 as libc::c_int)) {
                        yyn += YYSYMBOL_YYerror as libc::c_int;
                        if 0 as libc::c_int <= yyn && yyn <= 203 as libc::c_int
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
                        current_block = 17450030176436579763;
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
                *yyvsp = zzlval;
                yystate = yyn;
            }
            _ => {}
        }
        yyssp = yyssp.offset(1);
        yyssp;
    }
    match current_block {
        7232461487979279659 => {
            yyresult = 0 as libc::c_int;
        }
        17450030176436579763 => {
            yyresult = 1 as libc::c_int;
        }
        _ => {
            zzerror(b"memory exhausted\0" as *const u8 as *const libc::c_char);
            yyresult = 2 as libc::c_int;
        }
    }
    if zzchar != -(2 as libc::c_int) {
        yytoken = (if 0 as libc::c_int <= zzchar && zzchar <= 303 as libc::c_int {
            yytranslate[zzchar as usize] as yysymbol_kind_t as libc::c_int
        } else {
            YYSYMBOL_YYUNDEF as libc::c_int
        }) as yysymbol_kind_t;
        yydestruct(
            b"Cleanup: discarding lookahead\0" as *const u8 as *const libc::c_char,
            yytoken,
            &mut zzlval,
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
unsafe extern "C" fn find_argument(mut arg: *mut CMDARG) -> libc::c_int {
    let mut idx: libc::c_int = 0;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    name = (*arg).value.sval;
    len = strlen(name);
    idx = 0 as libc::c_int;
    loop {
        p = zz_debug_argtab[idx as usize].name as *mut libc::c_char;
        if p.is_null() {
            break;
        }
        if zz_debug_cmdtab[cmd_idx as usize].type_0 as libc::c_uint
            == zz_debug_argtab[idx as usize].cmd as libc::c_uint
            && *p as libc::c_int == *name as libc::c_int && strlen(p) == len
            && strncmp(p, name, len) == 0 as libc::c_int
        {
            return idx;
        }
        idx += 1;
        idx;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn concat_args(
    mut arg: *mut CMDARG,
    mut count: libc::c_int,
) -> *mut NODE {
    let mut n: *mut NODE = 0 as *mut NODE;
    let mut tmp: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut subsep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_long = 0;
    let mut subseplen: libc::c_long = 0;
    let mut i: libc::c_int = 0;
    if count == 1 as libc::c_int {
        n = force_string_fmt((*arg).value.nodeval, CONVFMT, CONVFMTidx);
        return dupnode(n);
    }
    tmp = emalloc_real(
        (count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut NODE>() as libc::c_ulong),
        b"concat_args\0" as *const u8 as *const libc::c_char,
        b"tmp\0" as *const u8 as *const libc::c_char,
        b"command.y\0" as *const u8 as *const libc::c_char,
        1381 as libc::c_int,
    ) as *mut *mut NODE;
    subseplen = (*(*SUBSEP_node).sub.nodep.l.lptr).sub.val.slen as libc::c_long;
    subsep = (*(*SUBSEP_node).sub.nodep.l.lptr).sub.val.sp;
    len = -subseplen;
    i = 0 as libc::c_int;
    while i < count {
        n = force_string_fmt((*arg).value.nodeval, CONVFMT, CONVFMTidx);
        len = (len as libc::c_ulong)
            .wrapping_add(((*n).sub.val.slen).wrapping_add(subseplen as libc::c_ulong))
            as libc::c_long as libc::c_long;
        let ref mut fresh18 = *tmp.offset(i as isize);
        *fresh18 = n;
        arg = (*arg).next;
        i += 1;
        i;
    }
    str = emalloc_real(
        (len + 1 as libc::c_int as libc::c_long) as size_t,
        b"concat_args\0" as *const u8 as *const libc::c_char,
        b"str\0" as *const u8 as *const libc::c_char,
        b"command.y\0" as *const u8 as *const libc::c_char,
        1393 as libc::c_int,
    ) as *mut libc::c_char;
    n = *tmp.offset(0 as libc::c_int as isize);
    memcpy(
        str as *mut libc::c_void,
        (*n).sub.val.sp as *const libc::c_void,
        (*n).sub.val.slen,
    );
    p = str.offset((*n).sub.val.slen as isize);
    i = 1 as libc::c_int;
    while i < count {
        if subseplen == 1 as libc::c_int as libc::c_long {
            let fresh19 = p;
            p = p.offset(1);
            *fresh19 = *subsep;
        } else if subseplen > 0 as libc::c_int as libc::c_long {
            memcpy(
                p as *mut libc::c_void,
                subsep as *const libc::c_void,
                subseplen as libc::c_ulong,
            );
            p = p.offset(subseplen as isize);
        }
        n = *tmp.offset(i as isize);
        memcpy(
            p as *mut libc::c_void,
            (*n).sub.val.sp as *const libc::c_void,
            (*n).sub.val.slen,
        );
        p = p.offset((*n).sub.val.slen as isize);
        i += 1;
        i;
    }
    *str.offset(len as isize) = '\0' as i32 as libc::c_char;
    free(tmp as *mut libc::c_void);
    return make_str_node(str, len as size_t, 2 as libc::c_int);
}
unsafe extern "C" fn find_command(
    mut token: *const libc::c_char,
    mut toklen: size_t,
) -> libc::c_int {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut abrv: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut try_exact: bool = 1 as libc::c_int != 0;
    let mut abrv_match: libc::c_int = -(1 as libc::c_int);
    let mut partial_match: libc::c_int = -(1 as libc::c_int);
    k = (::std::mem::size_of::<[cmdtoken; 44]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<cmdtoken>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    i = 0 as libc::c_int;
    while i < k {
        name = zz_debug_cmdtab[i as usize].name as *mut libc::c_char;
        if try_exact as libc::c_int != 0 && *token as libc::c_int == *name as libc::c_int
            && toklen == strlen(name) && strncmp(name, token, toklen) == 0 as libc::c_int
        {
            return i;
        }
        if *name as libc::c_int > *token as libc::c_int || i == k - 1 as libc::c_int {
            try_exact = 0 as libc::c_int != 0;
        }
        if abrv_match < 0 as libc::c_int {
            abrv = zz_debug_cmdtab[i as usize].abbrvn;
            if *abrv.offset(0 as libc::c_int as isize) as libc::c_int
                == *token.offset(0 as libc::c_int as isize) as libc::c_int
            {
                if toklen == 1 as libc::c_int as libc::c_ulong
                    && *abrv.offset(1 as libc::c_int as isize) == 0
                {
                    abrv_match = i;
                } else if toklen == 2 as libc::c_int as libc::c_ulong
                    && *abrv.offset(1 as libc::c_int as isize) as libc::c_int
                        == *token.offset(1 as libc::c_int as isize) as libc::c_int
                {
                    abrv_match = i;
                }
            }
        }
        if !try_exact && abrv_match >= 0 as libc::c_int {
            return abrv_match;
        }
        if partial_match < 0 as libc::c_int {
            if *token as libc::c_int == *name as libc::c_int && toklen < strlen(name)
                && strncmp(name, token, toklen) == 0 as libc::c_int
            {
                if (i == k - 1 as libc::c_int
                    || strncmp(
                        zz_debug_cmdtab[(i + 1 as libc::c_int) as usize].name,
                        token,
                        toklen,
                    ) != 0 as libc::c_int)
                    && (i == 0 as libc::c_int
                        || strncmp(
                            zz_debug_cmdtab[(i - 1 as libc::c_int) as usize].name,
                            token,
                            toklen,
                        ) != 0 as libc::c_int)
                {
                    partial_match = i;
                }
            }
        }
        i += 1;
        i;
    }
    return partial_match;
}
pub unsafe extern "C" fn do_help(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if arg.is_null() {
        initialize_pager(out_fp);
        if _setjmp(pager_quit_tag.as_mut_ptr()) == 0 as libc::c_int {
            i = 0 as libc::c_int;
            while !(zz_debug_cmdtab[i as usize].name).is_null() {
                gprintf(
                    out_fp,
                    b"%s:\n\0" as *const u8 as *const libc::c_char,
                    zz_debug_cmdtab[i as usize].name,
                );
                gprintf(
                    out_fp,
                    b"\t%s\n\0" as *const u8 as *const libc::c_char,
                    dcgettext(
                        0 as *const libc::c_char,
                        zz_debug_cmdtab[i as usize].help_txt,
                        5 as libc::c_int,
                    ),
                );
                i += 1;
                i;
            }
        }
    } else if (*arg).type_0 as libc::c_uint == D_string as libc::c_int as libc::c_uint {
        let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
        name = (*arg).value.sval;
        i = find_command(name, strlen(name));
        if i >= 0 as libc::c_int {
            fprintf(
                out_fp,
                b"%s\n\0" as *const u8 as *const libc::c_char,
                zz_debug_cmdtab[i as usize].help_txt,
            );
            if strcmp(
                zz_debug_cmdtab[i as usize].name,
                b"option\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                option_help();
            }
        } else {
            fprintf(
                out_fp,
                dcgettext(
                    0 as *const libc::c_char,
                    b"undefined command: %s\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                name,
            );
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn next_word(
    mut p: *mut libc::c_char,
    mut len: libc::c_int,
    mut endp: *mut *mut libc::c_char,
) -> *mut libc::c_char {
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    if p.is_null() || len <= 0 as libc::c_int {
        return 0 as *mut libc::c_char;
    }
    i = 0 as libc::c_int;
    while i < len {
        if *p as libc::c_int != ' ' as i32 && *p as libc::c_int != '\t' as i32 {
            break;
        }
        i += 1;
        i;
        p = p.offset(1);
        p;
    }
    if i == len {
        return 0 as *mut libc::c_char;
    }
    if !endp.is_null() {
        i += 1;
        i;
        q = p.offset(1 as libc::c_int as isize);
        while i < len {
            if *q as libc::c_int == ' ' as i32 || *q as libc::c_int == '\t' as i32 {
                break;
            }
            i += 1;
            i;
            q = q.offset(1);
            q;
        }
        *endp = q;
    }
    return p;
}
pub unsafe extern "C" fn command_completion(
    mut text: *const libc::c_char,
    mut start: libc::c_int,
    mut end: libc::c_int,
) -> *mut *mut libc::c_char {
    let mut cmdtok: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut idx: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    rl_attempted_completion_over = 1 as libc::c_int;
    this_cmd = D_illegal as libc::c_int;
    len = start;
    cmdtok = next_word(rl_line_buffer, len, &mut e);
    if cmdtok.is_null() {
        return rl_completion_matches(
            text,
            Some(
                command_generator
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> *mut libc::c_char,
            ),
        );
    }
    len = (len as libc::c_long - e.offset_from(rl_line_buffer) as libc::c_long)
        as libc::c_int;
    idx = find_command(cmdtok, e.offset_from(cmdtok) as libc::c_long as size_t);
    if idx < 0 as libc::c_int {
        return 0 as *mut *mut libc::c_char;
    }
    this_cmd = zz_debug_cmdtab[idx as usize].type_0 as libc::c_int;
    if (next_word(e, len, 0 as *mut *mut libc::c_char)).is_null() {
        match this_cmd {
            2 | 20 | 38 | 35 | 3 => {
                return rl_completion_matches(
                    text,
                    Some(
                        srcfile_generator
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> *mut libc::c_char,
                    ),
                );
            }
            19 | 12 | 36 | 17 => {
                return rl_completion_matches(
                    text,
                    Some(
                        argument_generator
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> *mut libc::c_char,
                    ),
                );
            }
            23 => {
                return rl_completion_matches(
                    text,
                    Some(
                        option_generator
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> *mut libc::c_char,
                    ),
                );
            }
            24 | 25 | 30 | 9 | 41 => {
                return rl_completion_matches(
                    text,
                    Some(
                        variable_generator
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                libc::c_int,
                            ) -> *mut libc::c_char,
                    ),
                );
            }
            _ => return 0 as *mut *mut libc::c_char,
        }
    }
    if this_cmd == D_print as libc::c_int || this_cmd == D_printf as libc::c_int {
        return rl_completion_matches(
            text,
            Some(
                variable_generator
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> *mut libc::c_char,
            ),
        );
    }
    return 0 as *mut *mut libc::c_char;
}
unsafe extern "C" fn command_generator(
    mut text: *const libc::c_char,
    mut state: libc::c_int,
) -> *mut libc::c_char {
    static mut textlen: size_t = 0;
    static mut idx: libc::c_int = 0 as libc::c_int;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    if state == 0 {
        textlen = strlen(text);
        idx = 0 as libc::c_int;
    }
    loop {
        name = zz_debug_cmdtab[idx as usize].name as *mut libc::c_char;
        if name.is_null() {
            break;
        }
        idx += 1;
        idx;
        if strncmp(name, text, textlen) == 0 as libc::c_int {
            return estrdup(name, strlen(name));
        }
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn srcfile_generator(
    mut text: *const libc::c_char,
    mut state: libc::c_int,
) -> *mut libc::c_char {
    static mut textlen: size_t = 0;
    static mut s: *mut SRCFILE = 0 as *const SRCFILE as *mut SRCFILE;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    extern "C" {
        #[link_name = "srcfiles"]
        static mut srcfiles_0: *mut SRCFILE;
    }
    if state == 0 {
        textlen = strlen(text);
        s = (*srcfiles).next;
    }
    while s != srcfiles {
        if (*s).stype as libc::c_uint != SRC_FILE as libc::c_int as libc::c_uint
            && (*s).stype as libc::c_uint != SRC_INC as libc::c_int as libc::c_uint
        {
            s = (*s).next;
        } else {
            name = (*s).src;
            s = (*s).next;
            if strncmp(name, text, textlen) == 0 as libc::c_int {
                return estrdup(name, strlen(name));
            }
        }
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn argument_generator(
    mut text: *const libc::c_char,
    mut state: libc::c_int,
) -> *mut libc::c_char {
    static mut textlen: size_t = 0;
    static mut idx: libc::c_int = 0;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    if state == 0 {
        textlen = strlen(text);
        idx = 0 as libc::c_int;
    }
    if this_cmd == D_help as libc::c_int {
        loop {
            let fresh20 = idx;
            idx = idx + 1;
            name = zz_debug_cmdtab[fresh20 as usize].name;
            if name.is_null() {
                break;
            }
            if strncmp(name, text, textlen) == 0 as libc::c_int {
                return estrdup(name, strlen(name));
            }
        }
    } else {
        loop {
            name = zz_debug_argtab[idx as usize].name;
            if name.is_null() {
                break;
            }
            let fresh21 = idx;
            idx = idx + 1;
            if this_cmd as libc::c_uint
                != zz_debug_argtab[fresh21 as usize].cmd as libc::c_uint
            {
                continue;
            }
            if strncmp(name, text, textlen) == 0 as libc::c_int {
                return estrdup(name, strlen(name));
            }
        }
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn variable_generator(
    mut text: *const libc::c_char,
    mut state: libc::c_int,
) -> *mut libc::c_char {
    static mut textlen: size_t = 0;
    static mut idx: libc::c_int = 0 as libc::c_int;
    static mut func: *mut NODE = 0 as *const NODE as *mut NODE;
    static mut vars: *mut *mut NODE = 0 as *const *mut NODE as *mut *mut NODE;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut r: *mut NODE = 0 as *mut NODE;
    if state == 0 {
        textlen = strlen(text);
        if !vars.is_null() {
            free(vars as *mut libc::c_void);
        }
        vars = variable_list();
        idx = 0 as libc::c_int;
        func = get_function();
    }
    while !func.is_null() {
        if idx as libc::c_long >= (*func).sub.nodep.l.ll {
            func = 0 as *mut NODE;
            idx = 0 as libc::c_int;
            break;
        } else {
            let fresh22 = idx;
            idx = idx + 1;
            name = (*((*func).sub.nodep.rn).offset(fresh22 as isize)).sub.nodep.name;
            if strncmp(name, text, textlen) == 0 as libc::c_int {
                return estrdup(name, strlen(name));
            }
        }
    }
    loop {
        let fresh23 = idx;
        idx = idx + 1;
        r = *vars.offset(fresh23 as isize);
        if r.is_null() {
            break;
        }
        name = (*r).sub.nodep.name;
        if strncmp(name, text, textlen) == 0 as libc::c_int {
            return estrdup(name, strlen(name));
        }
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn history_expand_line(mut line: *mut *mut libc::c_char) {
    let mut ret: libc::c_int = 0;
    let mut expansion: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*line).is_null() || input_fd != 0 as libc::c_int || !input_from_tty {
        return;
    }
    using_history();
    ret = history_expand(*line, &mut expansion);
    if ret < 0 as libc::c_int || ret == 2 as libc::c_int {
        free(expansion as *mut libc::c_void);
    } else {
        free(*line as *mut libc::c_void);
        *line = expansion;
    };
}
