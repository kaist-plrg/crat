use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type re_dfa_t;
    pub type dfa;
    pub type instruction_block;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn setbuf(__stream: *mut FILE, __buf: *mut libc::c_char);
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    fn print_vars(table: *mut *mut NODE, print_func: Func_print, fp: *mut FILE);
    fn function_list(sort: bool) -> *mut *mut NODE;
    fn variable_list() -> *mut *mut NODE;
    fn foreach_func(
        table: *mut *mut NODE,
        _: Option::<
            unsafe extern "C" fn(*mut INSTRUCTION, *mut libc::c_void) -> libc::c_int,
        >,
        _: *mut libc::c_void,
    ) -> libc::c_int;
    static mut func_table: *mut NODE;
    fn more_blocks(id: libc::c_int) -> *mut libc::c_void;
    fn grow_stack() -> *mut STACK_ITEM;
    fn lookup(name: *const libc::c_char) -> *mut NODE;
    fn make_str_node(
        s: *const libc::c_char,
        len: size_t,
        flags: libc::c_int,
    ) -> *mut NODE;
    fn install_params(func: *mut NODE);
    fn new_context() -> *mut AWK_CONTEXT;
    fn append_symbol(r: *mut NODE);
    fn remove_params(func: *mut NODE);
    fn bcalloc(op: OPCODE, size: libc::c_int, srcline: libc::c_int) -> *mut INSTRUCTION;
    fn os_isdir(fd: libc::c_int) -> libc::c_int;
    fn register_exec_hook(preh: Func_pre_exec, posth: Func_post_exec) -> libc::c_int;
    fn opcode2str(type_0: OPCODE) -> *const libc::c_char;
    fn reset_record();
    fn genflags2str(flagval: libc::c_int, tab: *const flagtab) -> *const libc::c_char;
    fn flags2str(_: libc::c_int) -> *const libc::c_char;
    fn pp_string_fp(
        print_func: Func_print,
        fp: *mut FILE,
        str: *const libc::c_char,
        namelen: size_t,
        delim: libc::c_int,
        breaklines: bool,
    );
    fn nodetype2str(type_0: NODETYPE) -> *const libc::c_char;
    fn cmp_nodes(t1: *mut NODE, t2: *mut NODE, use_strcmp: bool) -> libc::c_int;
    fn r_dupnode(n: *mut NODE) -> *mut NODE;
    fn push_context(ctxt: *mut AWK_CONTEXT);
    fn elem_new_to_scalar(n: *mut NODE) -> *mut NODE;
    fn nextfile(curfile_0: *mut *mut IOBUF, skipping: bool) -> libc::c_int;
    fn close_io(stdio_problem: *mut bool, got_EPIPE: *mut bool) -> libc::c_int;
    fn close_extensions();
    fn os_devopen(name: *const libc::c_char, flag: libc::c_int) -> libc::c_int;
    fn estrdup(str: *const libc::c_char, len: size_t) -> *mut libc::c_char;
    fn os_isatty(fd: libc::c_int) -> libc::c_int;
    fn pop_context();
    fn srcopen(s: *mut SRCFILE) -> libc::c_int;
    fn os_setbinmode(fd: libc::c_int, mode: libc::c_int) -> libc::c_int;
    fn set_loc(file: *const libc::c_char, line: libc::c_int);
    fn r_fatal(mesg: *const libc::c_char, _: ...);
    fn get_field(num: libc::c_long, assign: *mut Func_ptr) -> *mut *mut NODE;
    fn find_source(
        src: *const libc::c_char,
        stb: *mut stat,
        errcode: *mut libc::c_int,
        is_extlib: libc::c_int,
    ) -> *mut libc::c_char;
    fn free_context(ctxt: *mut AWK_CONTEXT, keep_globals: bool);
    fn in_main_context() -> libc::c_int;
    fn op2str(type_0: OPCODE) -> *const libc::c_char;
    fn format_tree(
        _: *const libc::c_char,
        _: size_t,
        _: *mut *mut NODE,
        _: libc::c_long,
    ) -> *mut NODE;
    fn valinfo(n: *mut NODE, print_func: Func_print, fp: *mut FILE);
    fn files_are_same(path: *mut libc::c_char, src: *mut SRCFILE) -> libc::c_int;
    fn free_srcfile(thisfile: *mut SRCFILE);
    fn add_srcfile(
        stype: srctype,
        src: *mut libc::c_char,
        curr: *mut SRCFILE,
        already_included: *mut bool,
        errcode: *mut libc::c_int,
    ) -> *mut SRCFILE;
    fn getfname(
        _: Option::<unsafe extern "C" fn(libc::c_int) -> *mut NODE>,
        prepend_awk: bool,
    ) -> *const libc::c_char;
    fn parse_program(pcode: *mut *mut INSTRUCTION, from_eval: bool) -> libc::c_int;
    fn assoc_list(
        symbol: *mut NODE,
        sort_str: *const libc::c_char,
        sort_ctxt: sort_context_t,
    ) -> *mut *mut NODE;
    fn array_vname(symbol: *const NODE) -> *const libc::c_char;
    fn make_array() -> *mut NODE;
    static mut fatal_tag_valid: libc::c_int;
    static mut fatal_tag: jmp_buf;
    fn r_unref(tmp: *mut NODE);
    static mut stack_top: *mut STACK_ITEM;
    static mut frame_ptr: *mut NODE;
    static mut stack_ptr: *mut STACK_ITEM;
    static mut exit_val: libc::c_int;
    static mut do_flags: do_flag_values;
    static mut srcfiles: *mut SRCFILE;
    static mut nextfree: [block_header; 2];
    static mut format_val: Option::<
        unsafe extern "C" fn(*const libc::c_char, libc::c_int, *mut NODE) -> *mut NODE,
    >;
    static mut str2number: Option::<unsafe extern "C" fn(*mut NODE) -> *mut NODE>;
    static mut make_number: Option::<unsafe extern "C" fn(libc::c_double) -> *mut NODE>;
    static mut interpret: Option::<
        unsafe extern "C" fn(*mut INSTRUCTION) -> libc::c_int,
    >;
    static mut source: *mut libc::c_char;
    static mut sourceline: libc::c_int;
    static mut Null_field: *mut NODE;
    static mut Nnull_string: *mut NODE;
    static mut CONVFMTidx: libc::c_int;
    static mut CONVFMT: *const libc::c_char;
    static ruletab: [*const libc::c_char; 0];
    fn execvp(
        __file: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn unsetenv(__name: *const libc::c_char) -> libc::c_int;
    fn setenv(
        __name: *const libc::c_char,
        __value: *const libc::c_char,
        __replace: libc::c_int,
    ) -> libc::c_int;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn exit(_: libc::c_int) -> !;
    fn free(__ptr: *mut libc::c_void);
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn add_history(_: *const libc::c_char);
    fn history_list() -> *mut *mut HIST_ENTRY;
    fn read_history(_: *const libc::c_char) -> libc::c_int;
    fn write_history(_: *const libc::c_char) -> libc::c_int;
    fn history_truncate_file(_: *const libc::c_char, _: libc::c_int) -> libc::c_int;
    static mut history_length: libc::c_int;
    static mut output_is_tty: bool;
    fn command_completion(
        text: *const libc::c_char,
        start: libc::c_int,
        end: libc::c_int,
    ) -> *mut *mut libc::c_char;
    fn free_cmdarg(list: *mut CMDARG);
    fn get_command(ctype: libc::c_int) -> Func_cmd;
    fn readline(_: *const libc::c_char) -> *mut libc::c_char;
    static mut rl_readline_name: *const libc::c_char;
    fn rl_reset_terminal(_: *const libc::c_char) -> libc::c_int;
    fn rl_get_screen_size(_: *mut libc::c_int, _: *mut libc::c_int);
    static mut rl_outstream: *mut FILE;
    static mut rl_attempted_completion_function: Option::<rl_completion_func_t>;
    static mut exiting: bool;
    static mut rule_list: *mut INSTRUCTION;
    static mut code_block: *mut INSTRUCTION;
    static mut fcall_list: *mut *mut NODE;
    static mut fcall_count: libc::c_long;
    static mut output_fp: *mut FILE;
    static mut curfile: *mut IOBUF;
    static mut command_file: *const libc::c_char;
    fn get_spec_varname(fptr: Func_ptr) -> *const libc::c_char;
    fn zzparse() -> libc::c_int;
    fn redir2str(redirtype: libc::c_int) -> *const libc::c_char;
    static mut d_argv: *mut *mut libc::c_char;
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
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
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
    pub l: C2RustUnnamed_10,
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
    pub d: C2RustUnnamed_7,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct break_point {
    pub next: *mut break_point,
    pub prev: *mut break_point,
    pub number: libc::c_int,
    pub ignore_count: libc::c_long,
    pub hit_count: libc::c_long,
    pub src: *mut libc::c_char,
    pub bpi: *mut INSTRUCTION,
    pub commands: commands_item,
    pub silent: bool,
    pub cndn: condition,
    pub flags: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct condition {
    pub code: *mut INSTRUCTION,
    pub ctxt: *mut AWK_CONTEXT,
    pub expr: *mut libc::c_char,
}
pub type AWK_CONTEXT = context;
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
pub type NODE = exp_node;
pub type SRCFILE = srcfile;
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
pub type INSTRUCTION = exp_instruction;
pub type srctype = libc::c_uint;
pub const SRC_EXTLIB: srctype = 5;
pub const SRC_INC: srctype = 4;
pub const SRC_FILE: srctype = 3;
pub const SRC_STDIN: srctype = 2;
pub const SRC_CMDLINE: srctype = 1;
pub type INSTRUCTION_POOL = instruction_pool;
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
pub struct commands_item {
    pub next: *mut commands_item,
    pub prev: *mut commands_item,
    pub cmd: libc::c_int,
    pub cmd_string: *mut libc::c_char,
    pub arg: *mut CMDARG,
}
pub type CMDARG = cmd_argument;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd_argument {
    pub next: *mut cmd_argument,
    pub type_0: argtype,
    pub value: C2RustUnnamed_6,
    pub a_count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub lval: libc::c_long,
    pub sval: *mut libc::c_char,
    pub nodeval: *mut NODE,
}
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
pub const D_watch: argtype = 41;
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
pub const D_set: argtype = 30;
pub const D_save: argtype = 29;
pub const D_run: argtype = 28;
pub const D_return: argtype = 27;
pub const D_quit: argtype = 26;
pub const D_printf: argtype = 25;
pub const D_print: argtype = 24;
pub const D_option: argtype = 23;
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
pub const D_display: argtype = 9;
pub const D_disable: argtype = 8;
pub const D_delete: argtype = 7;
pub const D_continue: argtype = 6;
pub const D_condition: argtype = 5;
pub const D_commands: argtype = 4;
pub const D_clear: argtype = 3;
pub const D_break: argtype = 2;
pub const D_backtrace: argtype = 1;
pub const D_illegal: argtype = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub fcall_count: libc::c_long,
    pub sourceline: libc::c_int,
    pub source: *mut libc::c_char,
    pub pc: *mut INSTRUCTION,
    pub repeat_count: libc::c_int,
    pub print_frame: bool,
    pub print_ret: bool,
    pub break_point: libc::c_int,
    pub watch_point: libc::c_int,
    pub check_func: Option::<unsafe extern "C" fn(*mut *mut INSTRUCTION) -> libc::c_int>,
    pub command: argtype,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_item {
    pub next: *mut list_item,
    pub prev: *mut list_item,
    pub number: libc::c_int,
    pub symbol: *mut NODE,
    pub subs: *mut *mut NODE,
    pub num_subs: libc::c_int,
    pub sname: *mut libc::c_char,
    pub fcall_count: libc::c_long,
    pub commands: commands_item,
    pub silent: libc::c_int,
    pub cndn: condition,
    pub value: [C2RustUnnamed_12; 2],
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub n: *mut NODE,
    pub l: libc::c_long,
}
pub type Func_cmd = Option::<
    unsafe extern "C" fn(*mut CMDARG, libc::c_int) -> libc::c_int,
>;
pub type BREAKPOINT = break_point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pf_data {
    pub print_func: Func_print,
    pub defn: bool,
    pub fp: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct command_source {
    pub fd: libc::c_int,
    pub is_tty: libc::c_int,
    pub read_func: Option::<
        unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_char,
    >,
    pub close_func: Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
    pub eof_status: libc::c_int,
    pub cmd: libc::c_int,
    pub str_0: *mut libc::c_char,
    pub next: *mut command_source,
}
pub const OPTION: C2RustUnnamed_13 = 5;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dbg_option {
    pub name: *const libc::c_char,
    pub num_val: *mut libc::c_int,
    pub str_val: *mut *const libc::c_char,
    pub assign: Option::<unsafe extern "C" fn(*const libc::c_char) -> ()>,
    pub help_txt: *const libc::c_char,
}
pub type HIST_ENTRY = _hist_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _hist_entry {
    pub line: *mut libc::c_char,
    pub timestamp: *mut libc::c_char,
    pub data: histdata_t,
}
pub type histdata_t = *mut libc::c_void;
pub const HISTORY: C2RustUnnamed_13 = 4;
pub const DISPLAY: C2RustUnnamed_13 = 3;
pub const BREAK: C2RustUnnamed_13 = 1;
pub const WATCH: C2RustUnnamed_13 = 2;
pub type rl_completion_func_t = unsafe extern "C" fn(
    *const libc::c_char,
    libc::c_int,
    libc::c_int,
) -> *mut *mut libc::c_char;
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
pub type C2RustUnnamed_13 = libc::c_uint;
#[inline]
unsafe extern "C" fn putc_unlocked(
    mut __c: libc::c_int,
    mut __stream: *mut FILE,
) -> libc::c_int {
    return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as libc::c_int
        as libc::c_long != 0
    {
        __overflow(__stream, __c as libc::c_uchar as libc::c_int)
    } else {
        let fresh0 = (*__stream)._IO_write_ptr;
        (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr).offset(1);
        *fresh0 = __c as libc::c_char;
        *fresh0 as libc::c_uchar as libc::c_int
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
#[inline]
unsafe extern "C" fn DEREF(mut r: *mut NODE) {
    (*r).valref -= 1;
    if (*r).valref > 0 as libc::c_int as libc::c_long {
        return;
    }
    r_unref(r);
}
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
static mut linebuf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut linebuf_len: size_t = 0;
pub static mut out_fp: *mut FILE = 0 as *const FILE as *mut FILE;
pub static mut dbg_prompt: *const libc::c_char = 0 as *const libc::c_char;
pub static mut commands_prompt: *const libc::c_char = b"> \0" as *const u8
    as *const libc::c_char;
pub static mut eval_prompt: *const libc::c_char = b"@> \0" as *const u8
    as *const libc::c_char;
pub static mut input_from_tty: bool = 0 as libc::c_int != 0;
pub static mut input_fd: libc::c_int = 0;
static mut cur_srcfile: *mut SRCFILE = 0 as *const SRCFILE as *mut SRCFILE;
static mut cur_frame: libc::c_long = 0 as libc::c_int as libc::c_long;
static mut cur_pc: *mut INSTRUCTION = 0 as *const INSTRUCTION as *mut INSTRUCTION;
static mut cur_rule: libc::c_int = 0 as libc::c_int;
static mut prog_running: bool = 0 as libc::c_int != 0;
static mut breakpoints: BREAKPOINT = unsafe {
    {
        let mut init = break_point {
            next: &breakpoints as *const BREAKPOINT as *mut BREAKPOINT,
            prev: &breakpoints as *const BREAKPOINT as *mut BREAKPOINT,
            number: 0 as libc::c_int,
            ignore_count: 0,
            hit_count: 0,
            src: 0 as *const libc::c_char as *mut libc::c_char,
            bpi: 0 as *const INSTRUCTION as *mut INSTRUCTION,
            commands: commands_item {
                next: 0 as *const commands_item as *mut commands_item,
                prev: 0 as *const commands_item as *mut commands_item,
                cmd: 0,
                cmd_string: 0 as *const libc::c_char as *mut libc::c_char,
                arg: 0 as *const CMDARG as *mut CMDARG,
            },
            silent: false,
            cndn: condition {
                code: 0 as *const INSTRUCTION as *mut INSTRUCTION,
                ctxt: 0 as *const AWK_CONTEXT as *mut AWK_CONTEXT,
                expr: 0 as *const libc::c_char as *mut libc::c_char,
            },
            flags: 0,
        };
        init
    }
};
static mut sess_history_base: libc::c_int = 0 as libc::c_int;
static mut last_printed_line: libc::c_int = 0 as libc::c_int;
static mut last_print_count: libc::c_int = 0;
static mut display_list: list_item = unsafe {
    {
        let mut init = list_item {
            next: &display_list as *const list_item as *mut list_item,
            prev: &display_list as *const list_item as *mut list_item,
            number: 0 as libc::c_int,
            symbol: 0 as *const NODE as *mut NODE,
            subs: 0 as *const *mut NODE as *mut *mut NODE,
            num_subs: 0,
            sname: 0 as *const libc::c_char as *mut libc::c_char,
            fcall_count: 0,
            commands: commands_item {
                next: 0 as *const commands_item as *mut commands_item,
                prev: 0 as *const commands_item as *mut commands_item,
                cmd: 0,
                cmd_string: 0 as *const libc::c_char as *mut libc::c_char,
                arg: 0 as *const CMDARG as *mut CMDARG,
            },
            silent: 0,
            cndn: condition {
                code: 0 as *const INSTRUCTION as *mut INSTRUCTION,
                ctxt: 0 as *const AWK_CONTEXT as *mut AWK_CONTEXT,
                expr: 0 as *const libc::c_char as *mut libc::c_char,
            },
            value: [C2RustUnnamed_12 {
                n: 0 as *const NODE as *mut NODE,
            }; 2],
            flags: 0,
        };
        init
    }
};
static mut watch_list: list_item = unsafe {
    {
        let mut init = list_item {
            next: &watch_list as *const list_item as *mut list_item,
            prev: &watch_list as *const list_item as *mut list_item,
            number: 0 as libc::c_int,
            symbol: 0 as *const NODE as *mut NODE,
            subs: 0 as *const *mut NODE as *mut *mut NODE,
            num_subs: 0,
            sname: 0 as *const libc::c_char as *mut libc::c_char,
            fcall_count: 0,
            commands: commands_item {
                next: 0 as *const commands_item as *mut commands_item,
                prev: 0 as *const commands_item as *mut commands_item,
                cmd: 0,
                cmd_string: 0 as *const libc::c_char as *mut libc::c_char,
                arg: 0 as *const CMDARG as *mut CMDARG,
            },
            silent: 0,
            cndn: condition {
                code: 0 as *const INSTRUCTION as *mut INSTRUCTION,
                ctxt: 0 as *const AWK_CONTEXT as *mut AWK_CONTEXT,
                expr: 0 as *const libc::c_char as *mut libc::c_char,
            },
            value: [C2RustUnnamed_12 {
                n: 0 as *const NODE as *mut NODE,
            }; 2],
            flags: 0,
        };
        init
    }
};
static mut stop: C2RustUnnamed_11 = C2RustUnnamed_11 {
    fcall_count: 0,
    sourceline: 0,
    source: 0 as *const libc::c_char as *mut libc::c_char,
    pc: 0 as *const INSTRUCTION as *mut INSTRUCTION,
    repeat_count: 0,
    print_frame: false,
    print_ret: false,
    break_point: 0,
    watch_point: 0,
    check_func: None,
    command: D_illegal,
};
static mut need_restart: bool = 0 as libc::c_int != 0;
static mut env_variable: [*const libc::c_char; 6] = [
    b"\0" as *const u8 as *const libc::c_char,
    b"DGAWK_BREAK\0" as *const u8 as *const libc::c_char,
    b"DGAWK_WATCH\0" as *const u8 as *const libc::c_char,
    b"DGAWK_DISPLAY\0" as *const u8 as *const libc::c_char,
    b"DGAWK_HISTORY\0" as *const u8 as *const libc::c_char,
    b"DGAWK_OPTION\0" as *const u8 as *const libc::c_char,
];
static mut commands_string: *const libc::c_char = 0 as *const libc::c_char;
static mut commands_string_len: libc::c_int = 0 as libc::c_int;
static mut line_sep: libc::c_char = 0;
static mut options_file: *const libc::c_char = b"./.gawkrc\0" as *const u8
    as *const libc::c_char;
static mut history_file: *const libc::c_char = b"./.gawk_history\0" as *const u8
    as *const libc::c_char;
static mut output_file: *const libc::c_char = b"/dev/stdout\0" as *const u8
    as *const libc::c_char;
pub static mut dgawk_prompt: *const libc::c_char = 0 as *const libc::c_char;
static mut list_size: libc::c_int = 15 as libc::c_int;
static mut do_trace: libc::c_int = 0 as libc::c_int;
static mut do_save_history: libc::c_int = 1 as libc::c_int;
static mut do_save_options: libc::c_int = 1 as libc::c_int;
static mut history_size: libc::c_int = 100 as libc::c_int;
static mut option_list: [dbg_option; 8] = unsafe {
    [
        {
            let mut init = dbg_option {
                name: b"history_size\0" as *const u8 as *const libc::c_char,
                num_val: &history_size as *const libc::c_int as *mut libc::c_int,
                str_val: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                assign: Some(
                    set_history_size as unsafe extern "C" fn(*const libc::c_char) -> (),
                ),
                help_txt: b"set or show the number of lines to keep in history file\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = dbg_option {
                name: b"listsize\0" as *const u8 as *const libc::c_char,
                num_val: &list_size as *const libc::c_int as *mut libc::c_int,
                str_val: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                assign: Some(
                    set_listsize as unsafe extern "C" fn(*const libc::c_char) -> (),
                ),
                help_txt: b"set or show the list command window size\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = dbg_option {
                name: b"outfile\0" as *const u8 as *const libc::c_char,
                num_val: 0 as *const libc::c_int as *mut libc::c_int,
                str_val: &output_file as *const *const libc::c_char
                    as *mut *const libc::c_char,
                assign: Some(
                    set_gawk_output as unsafe extern "C" fn(*const libc::c_char) -> (),
                ),
                help_txt: b"set or show gawk output file\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = dbg_option {
                name: b"prompt\0" as *const u8 as *const libc::c_char,
                num_val: 0 as *const libc::c_int as *mut libc::c_int,
                str_val: &dgawk_prompt as *const *const libc::c_char
                    as *mut *const libc::c_char,
                assign: Some(
                    set_prompt as unsafe extern "C" fn(*const libc::c_char) -> (),
                ),
                help_txt: b"set or show debugger prompt\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = dbg_option {
                name: b"save_history\0" as *const u8 as *const libc::c_char,
                num_val: &do_save_history as *const libc::c_int as *mut libc::c_int,
                str_val: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                assign: Some(
                    set_save_history as unsafe extern "C" fn(*const libc::c_char) -> (),
                ),
                help_txt: b"(un)set or show saving of command history (value=on|off)\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = dbg_option {
                name: b"save_options\0" as *const u8 as *const libc::c_char,
                num_val: &do_save_options as *const libc::c_int as *mut libc::c_int,
                str_val: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                assign: Some(
                    set_save_options as unsafe extern "C" fn(*const libc::c_char) -> (),
                ),
                help_txt: b"(un)set or show saving of options (value=on|off)\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = dbg_option {
                name: b"trace\0" as *const u8 as *const libc::c_char,
                num_val: &do_trace as *const libc::c_int as *mut libc::c_int,
                str_val: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                assign: Some(
                    set_trace as unsafe extern "C" fn(*const libc::c_char) -> (),
                ),
                help_txt: b"(un)set or show instruction tracing (value=on|off)\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = dbg_option {
                name: 0 as *const libc::c_char,
                num_val: 0 as *const libc::c_int as *mut libc::c_int,
                str_val: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                assign: None,
                help_txt: 0 as *const libc::c_char,
            };
            init
        },
    ]
};
pub static mut pager_quit_tag: jmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
pub static mut pager_quit_tag_valid: libc::c_int = 0 as libc::c_int;
static mut screen_width: libc::c_int = 2147483647 as libc::c_int;
static mut screen_height: libc::c_int = 2147483647 as libc::c_int;
static mut pager_lines_printed: libc::c_int = 0 as libc::c_int;
static mut pf_data: pf_data = pf_data {
    print_func: None,
    defn: false,
    fp: 0 as *const FILE as *mut FILE,
};
pub static mut read_a_line: Option::<
    unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_char,
> = None;
static mut cmd_src: *mut command_source = 0 as *const command_source
    as *mut command_source;
unsafe extern "C" fn g_readline(mut prompt: *const libc::c_char) -> *mut libc::c_char {
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line_size: size_t = 100 as libc::c_int as size_t;
    static mut buf: [libc::c_char; 2] = [0; 2];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: libc::c_int = 0;
    if input_from_tty as libc::c_int != 0 && !prompt.is_null()
        && *prompt as libc::c_int != 0
    {
        fprintf(out_fp, b"%s\0" as *const u8 as *const libc::c_char, prompt);
    }
    line = emalloc_real(
        line_size.wrapping_add(1 as libc::c_int as libc::c_ulong),
        b"g_readline\0" as *const u8 as *const libc::c_char,
        b"line\0" as *const u8 as *const libc::c_char,
        b"debug.c\0" as *const u8 as *const libc::c_char,
        390 as libc::c_int,
    ) as *mut libc::c_char;
    p = line;
    end = line.offset(line_size as isize);
    loop {
        n = read(
            input_fd,
            buf.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as size_t,
        ) as libc::c_int;
        if !(n > 0 as libc::c_int) {
            break;
        }
        if buf[0 as libc::c_int as usize] as libc::c_int == '\n' as i32 {
            if p > line
                && *p.offset(-(1 as libc::c_int) as isize) as libc::c_int == '\r' as i32
            {
                p = p.offset(-1);
                p;
            }
            break;
        } else {
            if p == end {
                line = erealloc_real(
                    line as *mut libc::c_void,
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(line_size)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    b"g_readline\0" as *const u8 as *const libc::c_char,
                    b"line\0" as *const u8 as *const libc::c_char,
                    b"debug.c\0" as *const u8 as *const libc::c_char,
                    400 as libc::c_int,
                ) as *mut libc::c_char;
                p = line.offset(line_size as isize);
                line_size = (line_size as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
                end = line.offset(line_size as isize);
            }
            let fresh2 = p;
            p = p.offset(1);
            *fresh2 = buf[0 as libc::c_int as usize];
        }
    }
    if n == -(1 as libc::c_int) || n == 0 as libc::c_int && p == line {
        free(line as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }
    *p = '\0' as i32 as libc::c_char;
    return line;
}
pub unsafe extern "C" fn d_error(mut mesg: *const libc::c_char, mut args: ...) {
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
}
unsafe extern "C" fn find_lines(mut s: *mut SRCFILE) -> libc::c_int {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: libc::c_int = 0;
    let mut ofs: libc::c_int = 0 as libc::c_int;
    let mut pos: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut pos_size: libc::c_int = 0;
    let mut maxlen: libc::c_int = 0 as libc::c_int;
    let mut numlines: libc::c_int = 0 as libc::c_int;
    let mut lastchar: libc::c_char = '\0' as i32 as libc::c_char;
    buf = emalloc_real(
        (*s).bufsize,
        b"find_lines\0" as *const u8 as *const libc::c_char,
        b"buf\0" as *const u8 as *const libc::c_char,
        b"debug.c\0" as *const u8 as *const libc::c_char,
        443 as libc::c_int,
    ) as *mut libc::c_char;
    pos_size = (*s).srclines;
    (*s)
        .line_offset = emalloc_real(
        ((pos_size + 2 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        b"find_lines\0" as *const u8 as *const libc::c_char,
        b"s->line_offset\0" as *const u8 as *const libc::c_char,
        b"debug.c\0" as *const u8 as *const libc::c_char,
        445 as libc::c_int,
    ) as *mut libc::c_int;
    pos = (*s).line_offset;
    *pos.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    loop {
        n = read((*s).fd, buf as *mut libc::c_void, (*s).bufsize) as libc::c_int;
        if !(n > 0 as libc::c_int) {
            break;
        }
        end = buf.offset(n as isize);
        lastchar = *buf.offset((n - 1 as libc::c_int) as isize);
        p = buf;
        while p < end {
            let fresh3 = p;
            p = p.offset(1);
            if *fresh3 as libc::c_int == '\n' as i32 {
                numlines += 1;
                if numlines > pos_size {
                    (*s)
                        .line_offset = erealloc_real(
                        (*s).line_offset as *mut libc::c_void,
                        ((2 as libc::c_int * pos_size + 2 as libc::c_int)
                            as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                        b"find_lines\0" as *const u8 as *const libc::c_char,
                        b"s->line_offset\0" as *const u8 as *const libc::c_char,
                        b"debug.c\0" as *const u8 as *const libc::c_char,
                        456 as libc::c_int,
                    ) as *mut libc::c_int;
                    pos = ((*s).line_offset).offset(pos_size as isize);
                    pos_size *= 2 as libc::c_int;
                }
                pos = pos.offset(1);
                *pos = (ofs as libc::c_long + p.offset_from(buf) as libc::c_long)
                    as libc::c_int;
                if *pos.offset(0 as libc::c_int as isize)
                    - *pos.offset(-(1 as libc::c_int) as isize) > maxlen
                {
                    maxlen = *pos.offset(0 as libc::c_int as isize)
                        - *pos.offset(-(1 as libc::c_int) as isize);
                }
            }
        }
        ofs += n;
    }
    free(buf as *mut libc::c_void);
    if n == -(1 as libc::c_int) {
        d_error(
            dcgettext(
                0 as *const libc::c_char,
                b"cannot read source file `%s': %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*s).src,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    if ofs <= 0 as libc::c_int {
        fprintf(
            out_fp,
            dcgettext(
                0 as *const libc::c_char,
                b"source file `%s' is empty.\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*s).src,
        );
        return -(1 as libc::c_int);
    }
    if lastchar as libc::c_int != '\n' as i32 {
        pos = pos.offset(1);
        *pos = ofs + 1 as libc::c_int;
        numlines += 1;
        numlines;
        if *pos.offset(0 as libc::c_int as isize)
            - *pos.offset(-(1 as libc::c_int) as isize) > maxlen
        {
            maxlen = *pos.offset(0 as libc::c_int as isize)
                - *pos.offset(-(1 as libc::c_int) as isize);
        }
    }
    (*s).maxlen = maxlen;
    (*s).srclines = numlines;
    return 0 as libc::c_int;
}
unsafe extern "C" fn source_find(mut src: *mut libc::c_char) -> *mut SRCFILE {
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
    if src.is_null() || *src as libc::c_int == '\0' as i32 {
        d_error(
            dcgettext(
                0 as *const libc::c_char,
                b"no current source file\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as *mut SRCFILE;
    }
    if (*cur_srcfile).src == src {
        return cur_srcfile;
    }
    s = (*srcfiles).next;
    while s != srcfiles {
        if ((*s).stype as libc::c_uint == SRC_FILE as libc::c_int as libc::c_uint
            || (*s).stype as libc::c_uint == SRC_INC as libc::c_int as libc::c_uint)
            && strcmp((*s).src, src) == 0 as libc::c_int
        {
            return s;
        }
        s = (*s).next;
    }
    path = find_source(src, &mut sbuf, &mut errno_val, 0 as libc::c_int);
    if !path.is_null() {
        s = (*srcfiles).next;
        while s != srcfiles {
            if ((*s).stype as libc::c_uint == SRC_FILE as libc::c_int as libc::c_uint
                || (*s).stype as libc::c_uint == SRC_INC as libc::c_int as libc::c_uint)
                && files_are_same(path, s) != 0
            {
                free(path as *mut libc::c_void);
                return s;
            }
            s = (*s).next;
        }
        free(path as *mut libc::c_void);
    }
    d_error(
        dcgettext(
            0 as *const libc::c_char,
            b"cannot find source file named `%s': %s\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        src,
        strerror(errno_val),
    );
    return 0 as *mut SRCFILE;
}
unsafe extern "C" fn print_lines(
    mut src: *mut libc::c_char,
    mut start_line: libc::c_int,
    mut nlines: libc::c_int,
) -> libc::c_int {
    let mut s: *mut SRCFILE = 0 as *mut SRCFILE;
    let mut pos: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
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
    s = source_find(src);
    if s.is_null() {
        return -(1 as libc::c_int);
    }
    if (*s).fd <= -(1 as libc::c_int)
        && {
            (*s).fd = srcopen(s);
            (*s).fd <= -(1 as libc::c_int)
        }
    {
        d_error(
            dcgettext(
                0 as *const libc::c_char,
                b"cannot open source file `%s' for reading: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            src,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    if fstat((*s).fd, &mut sbuf) == 0 as libc::c_int && (*s).mtime < sbuf.st_mtim.tv_sec
    {
        fprintf(
            out_fp,
            dcgettext(
                0 as *const libc::c_char,
                b"warning: source file `%s' modified since program compilation.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            src,
        );
        free((*s).line_offset as *mut libc::c_void);
        (*s).line_offset = 0 as *mut libc::c_int;
        (*s).mtime = sbuf.st_mtim.tv_sec;
        close((*s).fd);
        (*s).fd = -(1 as libc::c_int);
        (*s).fd = srcopen(s);
        if (*s).fd <= -(1 as libc::c_int) {
            d_error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot open source file `%s' for reading: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                src,
                strerror(*__errno_location()),
            );
            return -(1 as libc::c_int);
        }
    }
    os_setbinmode((*s).fd, 0 as libc::c_int);
    if ((*s).line_offset).is_null() && find_lines(s) != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if start_line < 1 as libc::c_int || start_line > (*s).srclines {
        d_error(
            dcgettext(
                0 as *const libc::c_char,
                b"line number %d out of range; `%s' has %d lines\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            start_line,
            src,
            (*s).srclines,
        );
        return -(1 as libc::c_int);
    }
    if start_line + nlines - 1 as libc::c_int > (*s).srclines {
        nlines = (*s).srclines - start_line + 1 as libc::c_int;
    }
    pos = (*s).line_offset;
    if lseek(
        (*s).fd,
        *pos.offset((start_line - 1 as libc::c_int) as isize) as off_t,
        0 as libc::c_int,
    ) < 0 as libc::c_int as libc::c_long
    {
        d_error(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            src,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    if linebuf.is_null() {
        linebuf = emalloc_real(
            ((*s).maxlen + 20 as libc::c_int) as size_t,
            b"print_lines\0" as *const u8 as *const libc::c_char,
            b"linebuf\0" as *const u8 as *const libc::c_char,
            b"debug.c\0" as *const u8 as *const libc::c_char,
            589 as libc::c_int,
        ) as *mut libc::c_char;
        linebuf_len = (*s).maxlen as size_t;
    } else if linebuf_len < (*s).maxlen as libc::c_ulong {
        linebuf = erealloc_real(
            linebuf as *mut libc::c_void,
            ((*s).maxlen + 20 as libc::c_int) as size_t,
            b"print_lines\0" as *const u8 as *const libc::c_char,
            b"linebuf\0" as *const u8 as *const libc::c_char,
            b"debug.c\0" as *const u8 as *const libc::c_char,
            592 as libc::c_int,
        ) as *mut libc::c_char;
        linebuf_len = (*s).maxlen as size_t;
    }
    i = start_line;
    while i < start_line + nlines {
        let mut supposed_len: libc::c_int = 0;
        let mut len: libc::c_int = 0;
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        sprintf(linebuf, b"%-8d\0" as *const u8 as *const libc::c_char, i);
        if nlines > 1 as libc::c_int {
            let mut b: *mut BREAKPOINT = 0 as *mut BREAKPOINT;
            let mut has_bpt: bool = 0 as libc::c_int != 0;
            b = breakpoints.prev;
            while b != &mut breakpoints as *mut BREAKPOINT {
                if src == (*b).src && i == (*(*b).bpi).source_line as libc::c_int {
                    has_bpt = 1 as libc::c_int != 0;
                    break;
                } else {
                    b = (*b).prev;
                }
            }
            if prog_running as libc::c_int != 0 && src == source && i == sourceline {
                if has_bpt {
                    sprintf(
                        linebuf,
                        b"%-4d:b=>\0" as *const u8 as *const libc::c_char,
                        i,
                    );
                } else {
                    sprintf(
                        linebuf,
                        b"%-4d  =>\0" as *const u8 as *const libc::c_char,
                        i,
                    );
                }
            } else if has_bpt {
                sprintf(linebuf, b"%-4d:b  \0" as *const u8 as *const libc::c_char, i);
            }
        }
        p = linebuf.offset(strlen(linebuf) as isize);
        supposed_len = *pos.offset(i as isize)
            - *pos.offset((i - 1 as libc::c_int) as isize);
        len = read((*s).fd, p as *mut libc::c_void, supposed_len as size_t)
            as libc::c_int;
        match len {
            -1 => {
                d_error(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"cannot read source file `%s': %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    src,
                    strerror(*__errno_location()),
                );
                return -(1 as libc::c_int);
            }
            0 => {
                d_error(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"unexpected eof while reading file `%s', line %d\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    src,
                    i,
                );
                return -(1 as libc::c_int);
            }
            _ => {
                if i == (*s).srclines
                    && *p.offset((len - 1 as libc::c_int) as isize) as libc::c_int
                        != '\n' as i32
                {
                    let fresh4 = len;
                    len = len + 1;
                    *p.offset(fresh4 as isize) = '\n' as i32 as libc::c_char;
                }
                len = (len as libc::c_long + p.offset_from(linebuf) as libc::c_long)
                    as libc::c_int;
                if (if 0 != 0 && 0 != 0
                    && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                        .wrapping_mul(len as size_t) <= 8 as libc::c_int as libc::c_ulong
                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                        != 0 as libc::c_int as libc::c_ulong
                {
                    {
                        let mut __ptr: *const libc::c_char = linebuf
                            as *const libc::c_char;
                        let mut __stream: *mut FILE = out_fp;
                        let mut __cnt: size_t = 0;
                        __cnt = (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                            .wrapping_mul(len as size_t);
                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                            let fresh5 = __ptr;
                            __ptr = __ptr.offset(1);
                            if putc_unlocked(*fresh5 as libc::c_int, __stream)
                                == -(1 as libc::c_int)
                            {
                                break;
                            }
                            __cnt = __cnt.wrapping_sub(1);
                            __cnt;
                        }
                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                            .wrapping_mul(len as size_t)
                            .wrapping_sub(__cnt)
                            .wrapping_div(
                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            )
                    }
                } else {
                    if 0 != 0
                        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                            == 0 as libc::c_int as libc::c_ulong
                        || 0 != 0 && len as size_t == 0 as libc::c_int as libc::c_ulong
                    {
                        0 as libc::c_int as size_t
                    } else {
                        fwrite_unlocked(
                            linebuf as *const libc::c_void,
                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            len as size_t,
                            out_fp,
                        )
                    }
                }) != len as libc::c_ulong
                {
                    return -(1 as libc::c_int);
                }
            }
        }
        i += 1;
        i;
    }
    if cur_srcfile != s {
        if (*cur_srcfile).fd != -(1 as libc::c_int) {
            close((*cur_srcfile).fd);
            (*cur_srcfile).fd = -(1 as libc::c_int);
        }
        cur_srcfile = s;
    }
    return i - 1 as libc::c_int;
}
pub unsafe extern "C" fn do_list(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    let mut line_first: libc::c_long = 0;
    let mut line_last: libc::c_long = 0;
    let mut count: libc::c_long = list_size as libc::c_long;
    let mut rp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut src: *mut libc::c_char = (*cur_srcfile).src;
    line_first = (last_printed_line + 1 as libc::c_int) as libc::c_long;
    if !arg.is_null() {
        let mut current_block_24: u64;
        match (*arg).type_0 as libc::c_uint {
            43 => {
                if (*arg).value.lval < 0 as libc::c_int as libc::c_long {
                    line_first = (last_printed_line - last_print_count - list_size
                        + 1 as libc::c_int) as libc::c_long;
                    if line_first < 1 as libc::c_int as libc::c_long {
                        if last_printed_line != last_print_count {
                            line_first = 1 as libc::c_int as libc::c_long;
                        } else {
                            return 0 as libc::c_int
                        }
                    }
                    current_block_24 = 2891135413264362348;
                } else {
                    current_block_24 = 11748545085139701960;
                }
            }
            51 => {
                current_block_24 = 14675778349424430903;
            }
            44 => {
                src = (*arg).value.sval;
                if !((*arg).next).is_null() {
                    arg = (*arg).next;
                    if (*arg).type_0 as libc::c_uint
                        == D_int as libc::c_int as libc::c_uint
                    {
                        current_block_24 = 11748545085139701960;
                    } else if (*arg).type_0 as libc::c_uint
                        == D_range as libc::c_int as libc::c_uint
                    {
                        current_block_24 = 14675778349424430903;
                    } else if (*arg).type_0 as libc::c_uint
                        == D_func as libc::c_int as libc::c_uint
                    {
                        current_block_24 = 802100070843518202;
                    } else {
                        line_first = 1 as libc::c_int as libc::c_long;
                        current_block_24 = 2891135413264362348;
                    }
                } else {
                    line_first = 1 as libc::c_int as libc::c_long;
                    current_block_24 = 2891135413264362348;
                }
            }
            50 => {
                current_block_24 = 802100070843518202;
            }
            _ => {
                current_block_24 = 2891135413264362348;
            }
        }
        match current_block_24 {
            802100070843518202 => {
                rp = (*(*arg).value.nodeval).sub.nodep.r.iptr;
                src = (*rp).d.name;
                line_first = ((*rp).source_line as libc::c_int
                    - list_size / 2 as libc::c_int) as libc::c_long;
                if line_first < 1 as libc::c_int as libc::c_long {
                    line_first = 1 as libc::c_int as libc::c_long;
                }
            }
            14675778349424430903 => {
                line_first = (*arg).value.lval;
                arg = (*arg).next;
                count = (*arg).value.lval - line_first
                    + 1 as libc::c_int as libc::c_long;
            }
            11748545085139701960 => {
                line_first = (*arg).value.lval
                    - (list_size / 2 as libc::c_int) as libc::c_long;
                if line_first < 1 as libc::c_int as libc::c_long {
                    line_first = 1 as libc::c_int as libc::c_long;
                }
            }
            _ => {}
        }
    }
    line_last = print_lines(src, line_first as libc::c_int, count as libc::c_int)
        as libc::c_long;
    if line_last != -(1 as libc::c_int) as libc::c_long {
        last_printed_line = line_last as libc::c_int;
        last_print_count = (line_last - line_first + 1 as libc::c_int as libc::c_long)
            as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn do_info(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    let mut table: *mut *mut NODE = 0 as *mut *mut NODE;
    if arg.is_null()
        || (*arg).type_0 as libc::c_uint != D_argument as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    match (*arg).value.lval {
        9 => {
            fprintf(
                out_fp,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Current source file: %s\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*cur_srcfile).src,
            );
            fprintf(
                out_fp,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Number of lines: %d\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*cur_srcfile).srclines,
            );
        }
        10 => {
            let mut s: *mut SRCFILE = 0 as *mut SRCFILE;
            s = (*srcfiles).next;
            while s != srcfiles {
                fprintf(
                    out_fp,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Source file (lines): %s (%d)\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    if (*s).stype as libc::c_uint
                        == SRC_FILE as libc::c_int as libc::c_uint
                        || (*s).stype as libc::c_uint
                            == SRC_INC as libc::c_int as libc::c_uint
                    {
                        (*s).src as *const libc::c_char
                    } else {
                        b"cmd. line\0" as *const u8 as *const libc::c_char
                    },
                    (*s).srclines,
                );
                s = (*s).next;
            }
        }
        2 => {
            initialize_pager(out_fp);
            if _setjmp(pager_quit_tag.as_mut_ptr()) == 0 as libc::c_int {
                let mut b: *mut BREAKPOINT = 0 as *mut BREAKPOINT;
                let mut c: *mut commands_item = 0 as *mut commands_item;
                gprintf(
                    out_fp,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Number  Disp  Enabled  Location\n\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                b = breakpoints.prev;
                while b != &mut breakpoints as *mut BREAKPOINT {
                    let mut disp: *const libc::c_char = b"keep\0" as *const u8
                        as *const libc::c_char;
                    if (*b).flags as libc::c_int & 2 as libc::c_int != 0 as libc::c_int {
                        disp = b"dis\0" as *const u8 as *const libc::c_char;
                    } else if (*b).flags as libc::c_int & 4 as libc::c_int
                        != 0 as libc::c_int
                    {
                        disp = b"del\0" as *const u8 as *const libc::c_char;
                    }
                    gprintf(
                        out_fp,
                        b"%-6d  %-4.4s  %-7.7s  file %s, line #%d\n\0" as *const u8
                            as *const libc::c_char,
                        (*b).number,
                        disp,
                        if (*b).flags as libc::c_int & 1 as libc::c_int
                            != 0 as libc::c_int
                        {
                            b"yes\0" as *const u8 as *const libc::c_char
                        } else {
                            b"no\0" as *const u8 as *const libc::c_char
                        },
                        (*b).src,
                        (*(*b).bpi).source_line as libc::c_int,
                    );
                    if (*b).hit_count > 0 as libc::c_int as libc::c_long {
                        gprintf(
                            out_fp,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"\tnumber of hits = %ld\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            (*b).hit_count,
                        );
                    }
                    if (*b).flags as libc::c_int & 8 as libc::c_int != 0 as libc::c_int {
                        gprintf(
                            out_fp,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"\tignore next %ld hit(s)\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            (*b).ignore_count,
                        );
                    }
                    if !((*b).cndn.code).is_null() {
                        gprintf(
                            out_fp,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"\tstop condition: %s\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            (*b).cndn.expr,
                        );
                    }
                    if (*b).commands.next != &mut (*b).commands as *mut commands_item {
                        gprintf(
                            out_fp,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"\tcommands:\n\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                    c = (*b).commands.next;
                    while c != &mut (*b).commands as *mut commands_item {
                        gprintf(
                            out_fp,
                            b"\t%s\n\0" as *const u8 as *const libc::c_char,
                            (*c).cmd_string,
                        );
                        if (*c).cmd == D_eval as libc::c_int {
                            let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
                            let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
                            let mut a: *mut CMDARG = (*c).arg;
                            start = strchr((*a).value.sval, '{' as i32);
                            end = strrchr((*a).value.sval, '}' as i32);
                            if !(start.is_null() || end.is_null()) {
                                start = start.offset(1);
                                start;
                                *end = '\0' as i32 as libc::c_char;
                                gprintf(
                                    out_fp,
                                    b"%s\0" as *const u8 as *const libc::c_char,
                                    start,
                                );
                                *end = '}' as i32 as libc::c_char;
                            }
                        }
                        c = (*c).next;
                    }
                    b = (*b).prev;
                }
            }
        }
        5 => {
            if !prog_running {
                d_error(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"program not running\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                return 0 as libc::c_int;
            }
            fprintf(
                out_fp,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Current frame: \0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            print_numbered_frame(cur_frame);
            if cur_frame < fcall_count {
                fprintf(
                    out_fp,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Called by frame: \0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                print_numbered_frame(cur_frame + 1 as libc::c_int as libc::c_long);
            }
            if cur_frame > 0 as libc::c_int as libc::c_long {
                fprintf(
                    out_fp,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Caller of frame: \0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                print_numbered_frame(cur_frame - 1 as libc::c_int as libc::c_long);
            }
        }
        1 | 7 => {
            let mut f: *mut NODE = 0 as *mut NODE;
            let mut func: *mut NODE = 0 as *mut NODE;
            let mut pc: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
            let mut arg_count: libc::c_int = 0;
            let mut pcount: libc::c_int = 0;
            let mut i: libc::c_int = 0;
            let mut from: libc::c_int = 0;
            let mut to: libc::c_int = 0;
            if !prog_running {
                d_error(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"program not running\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                return 0 as libc::c_int;
            }
            f = find_frame(cur_frame);
            func = (*f).sub.nodep.x.extra;
            if func.is_null() {
                fprintf(
                    out_fp,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"None in main().\n\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                return 0 as libc::c_int;
            }
            pcount = (*func).sub.nodep.l.ll as libc::c_int;
            pc = (*f).sub.nodep.l.li as *mut INSTRUCTION;
            arg_count = (*pc.offset(1 as libc::c_int as isize)).x.xl as libc::c_int;
            if arg_count > pcount {
                arg_count = pcount;
            }
            if (*arg).value.lval == A_ARGS as libc::c_int as libc::c_long {
                from = 0 as libc::c_int;
                to = arg_count - 1 as libc::c_int;
            } else {
                from = arg_count;
                to = pcount - 1 as libc::c_int;
            }
            i = from;
            while i <= to {
                let mut r: *mut NODE = 0 as *mut NODE;
                r = *((*f).sub.nodep.r.av).offset(i as isize);
                if (*r).type_0 as libc::c_uint
                    == Node_array_ref as libc::c_int as libc::c_uint
                {
                    r = (*r).sub.nodep.l.lptr;
                }
                fprintf(
                    out_fp,
                    b"%s = \0" as *const u8 as *const libc::c_char,
                    (*((*func).sub.nodep.rn).offset(i as isize)).sub.nodep.name,
                );
                print_symbol(r, 1 as libc::c_int != 0);
                i += 1;
                i;
            }
            if to < from {
                fprintf(
                    out_fp,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    if (*arg).value.lval == A_ARGS as libc::c_int as libc::c_long {
                        dcgettext(
                            0 as *const libc::c_char,
                            b"No arguments.\n\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        )
                    } else {
                        dcgettext(
                            0 as *const libc::c_char,
                            b"No locals.\n\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        )
                    },
                );
            }
        }
        13 => {
            table = variable_list();
            initialize_pager(out_fp);
            if _setjmp(pager_quit_tag.as_mut_ptr()) == 0 as libc::c_int {
                gprintf(
                    out_fp,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"All defined variables:\n\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                print_vars(
                    table,
                    Some(
                        gprintf
                            as unsafe extern "C" fn(
                                *mut FILE,
                                *const libc::c_char,
                                ...
                            ) -> libc::c_int,
                    ),
                    out_fp,
                );
            }
            free(table as *mut libc::c_void);
        }
        6 => {
            table = function_list(1 as libc::c_int != 0);
            initialize_pager(out_fp);
            if _setjmp(pager_quit_tag.as_mut_ptr()) == 0 as libc::c_int {
                gprintf(
                    out_fp,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"All defined functions:\n\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                pf_data
                    .print_func = Some(
                    gprintf
                        as unsafe extern "C" fn(
                            *mut FILE,
                            *const libc::c_char,
                            ...
                        ) -> libc::c_int,
                );
                pf_data.fp = out_fp;
                pf_data.defn = 1 as libc::c_int != 0;
                foreach_func(
                    table,
                    ::std::mem::transmute::<
                        Option::<
                            unsafe extern "C" fn(
                                *mut INSTRUCTION,
                                *mut libc::c_void,
                            ) -> libc::c_int,
                        >,
                        Option::<
                            unsafe extern "C" fn(
                                *mut INSTRUCTION,
                                *mut libc::c_void,
                            ) -> libc::c_int,
                        >,
                    >(
                        Some(
                            print_function
                                as unsafe extern "C" fn(
                                    *mut INSTRUCTION,
                                    *mut libc::c_void,
                                ) -> libc::c_int,
                        ),
                    ),
                    &mut pf_data as *mut pf_data as *mut libc::c_void,
                );
            }
            free(table as *mut libc::c_void);
        }
        4 | 14 => {
            initialize_pager(out_fp);
            if _setjmp(pager_quit_tag.as_mut_ptr()) == 0 as libc::c_int {
                let mut d: *mut list_item = 0 as *mut list_item;
                let mut list: *mut list_item = 0 as *mut list_item;
                if (*arg).value.lval == A_DISPLAY as libc::c_int as libc::c_long {
                    list = &mut display_list;
                    gprintf(
                        out_fp,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Auto-display variables:\n\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                } else {
                    list = &mut watch_list;
                    gprintf(
                        out_fp,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Watch variables:\n\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                d = (*list).prev;
                while d != list {
                    let mut i_0: libc::c_int = 0;
                    let mut c_0: *mut commands_item = 0 as *mut commands_item;
                    let mut symbol: *mut NODE = (*d).symbol;
                    if (*d).flags & 2 as libc::c_int != 0 as libc::c_int {
                        gprintf(
                            out_fp,
                            b"%d:\t%s\0" as *const u8 as *const libc::c_char,
                            (*d).number,
                            (*d).sname,
                        );
                        i_0 = 0 as libc::c_int;
                        while i_0 < (*d).num_subs {
                            let mut sub: *mut NODE = 0 as *mut NODE;
                            sub = *((*d).subs).offset(i_0 as isize);
                            gprintf(
                                out_fp,
                                b"[\"%.*s\"]\0" as *const u8 as *const libc::c_char,
                                (*sub).sub.val.slen as libc::c_int,
                                (*sub).sub.val.sp,
                            );
                            i_0 += 1;
                            i_0;
                        }
                        gprintf(out_fp, b"\n\0" as *const u8 as *const libc::c_char);
                    } else if (*d).flags & 4 as libc::c_int != 0 as libc::c_int {
                        gprintf(
                            out_fp,
                            b"%d:\t$%ld\n\0" as *const u8 as *const libc::c_char,
                            (*d).number,
                            (*symbol).sub.val.fltnum as libc::c_long,
                        );
                    } else {
                        gprintf(
                            out_fp,
                            b"%d:\t%s\n\0" as *const u8 as *const libc::c_char,
                            (*d).number,
                            (*d).sname,
                        );
                    }
                    if !((*d).cndn.code).is_null() {
                        gprintf(
                            out_fp,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"\tstop condition: %s\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            (*d).cndn.expr,
                        );
                    }
                    if (*d).commands.next != &mut (*d).commands as *mut commands_item {
                        gprintf(
                            out_fp,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"\tcommands:\n\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                    c_0 = (*d).commands.next;
                    while c_0 != &mut (*d).commands as *mut commands_item {
                        gprintf(
                            out_fp,
                            b"\t%s\n\0" as *const u8 as *const libc::c_char,
                            (*c_0).cmd_string,
                        );
                        if (*c_0).cmd == D_eval as libc::c_int {
                            let mut start_0: *mut libc::c_char = 0 as *mut libc::c_char;
                            let mut end_0: *mut libc::c_char = 0 as *mut libc::c_char;
                            let mut a_0: *mut CMDARG = (*c_0).arg;
                            start_0 = strchr((*a_0).value.sval, '{' as i32);
                            end_0 = strrchr((*a_0).value.sval, '}' as i32);
                            if !(start_0.is_null() || end_0.is_null()) {
                                start_0 = start_0.offset(1);
                                start_0;
                                *end_0 = '\0' as i32 as libc::c_char;
                                gprintf(
                                    out_fp,
                                    b"%s\0" as *const u8 as *const libc::c_char,
                                    start_0,
                                );
                                *end_0 = '}' as i32 as libc::c_char;
                            }
                        }
                        c_0 = (*c_0).next;
                    }
                    d = (*d).prev;
                }
            }
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn print_symbol(mut r: *mut NODE, mut isparam: bool) {
    match (*r).type_0 as libc::c_uint {
        6 => {
            fprintf(out_fp, b"untyped variable\n\0" as *const u8 as *const libc::c_char);
        }
        7 => {
            fprintf(out_fp, b"untyped element\n\0" as *const u8 as *const libc::c_char);
        }
        4 => {
            if !isparam && ((*r).sub.nodep.r.uptr).is_some() {
                ((*r).sub.nodep.r.uptr).unwrap()();
            }
            valinfo(
                (*r).sub.nodep.l.lptr,
                Some(
                    fprintf
                        as unsafe extern "C" fn(
                            *mut FILE,
                            *const libc::c_char,
                            ...
                        ) -> libc::c_int,
                ),
                out_fp,
            );
        }
        5 => {
            fprintf(
                out_fp,
                b"array, %ld elements\n\0" as *const u8 as *const libc::c_char,
                (*r).sub.nodep.reflags as libc::c_long,
            );
        }
        9 => {
            fprintf(out_fp, b"`function'\n\0" as *const u8 as *const libc::c_char);
        }
        _ => {}
    };
}
unsafe extern "C" fn find_frame(mut num: libc::c_long) -> *mut NODE {
    if num == 0 as libc::c_int as libc::c_long {
        return frame_ptr;
    }
    return *fcall_list.offset(num as isize);
}
unsafe extern "C" fn find_param(
    mut name: *const libc::c_char,
    mut num: libc::c_long,
    mut pname: *mut *mut libc::c_char,
) -> *mut NODE {
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut f: *mut NODE = 0 as *mut NODE;
    let mut fparam: *mut libc::c_char = 0 as *mut libc::c_char;
    if !pname.is_null() {
        *pname = 0 as *mut libc::c_char;
    }
    if num < 0 as libc::c_int as libc::c_long || num > fcall_count || name.is_null() {
        return 0 as *mut NODE;
    }
    f = find_frame(num);
    if !((*f).sub.nodep.x.extra).is_null() {
        let mut func: *mut NODE = 0 as *mut NODE;
        let mut i: libc::c_int = 0;
        let mut pcount: libc::c_int = 0;
        func = (*f).sub.nodep.x.extra;
        pcount = (*func).sub.nodep.l.ll as libc::c_int;
        i = 0 as libc::c_int;
        while i < pcount {
            fparam = (*((*func).sub.nodep.rn).offset(i as isize)).sub.nodep.name;
            if strcmp(name, fparam) == 0 as libc::c_int {
                r = *((*f).sub.nodep.r.av).offset(i as isize);
                if (*r).type_0 as libc::c_uint
                    == Node_array_ref as libc::c_int as libc::c_uint
                {
                    r = (*r).sub.nodep.l.lptr;
                }
                if !pname.is_null() {
                    *pname = fparam;
                }
                break;
            } else {
                i += 1;
                i;
            }
        }
    }
    return r;
}
unsafe extern "C" fn find_symbol(
    mut name: *const libc::c_char,
    mut pname: *mut *mut libc::c_char,
) -> *mut NODE {
    let mut r: *mut NODE = 0 as *mut NODE;
    if !pname.is_null() {
        *pname = 0 as *mut libc::c_char;
    }
    if prog_running {
        r = find_param(name, cur_frame, pname);
    }
    if r.is_null() {
        r = lookup(name);
    }
    if r.is_null() {
        fprintf(
            out_fp,
            dcgettext(
                0 as *const libc::c_char,
                b"no symbol `%s' in current context\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            name,
        );
    }
    return r;
}
unsafe extern "C" fn find_array(mut name: *const libc::c_char) -> *mut NODE {
    let mut r: *mut NODE = 0 as *mut NODE;
    r = find_symbol(name, 0 as *mut *mut libc::c_char);
    if !r.is_null()
        && (*r).type_0 as libc::c_uint != Node_var_array as libc::c_int as libc::c_uint
    {
        fprintf(
            out_fp,
            dcgettext(
                0 as *const libc::c_char,
                b"`%s' is not an array\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            name,
        );
        return 0 as *mut NODE;
    }
    return r;
}
unsafe extern "C" fn print_field(mut field_num: libc::c_long) {
    let mut lhs: *mut *mut NODE = 0 as *mut *mut NODE;
    lhs = get_field(field_num, 0 as *mut Func_ptr);
    if *lhs == Null_field || *lhs == Nnull_string {
        fprintf(
            out_fp,
            dcgettext(
                0 as *const libc::c_char,
                b"$%ld = uninitialized field\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            field_num,
        );
    } else {
        fprintf(out_fp, b"$%ld = \0" as *const u8 as *const libc::c_char, field_num);
        valinfo(
            *lhs,
            Some(
                fprintf
                    as unsafe extern "C" fn(
                        *mut FILE,
                        *const libc::c_char,
                        ...
                    ) -> libc::c_int,
            ),
            out_fp,
        );
    };
}
unsafe extern "C" fn print_array(
    mut arr: *mut NODE,
    mut arr_name: *mut libc::c_char,
) -> libc::c_int {
    let mut subs: *mut NODE = 0 as *mut NODE;
    let mut list: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut i: libc::c_int = 0;
    let mut num_elems: size_t = 0 as libc::c_int as size_t;
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut pager_quit_tag_stack: jmp_buf = [__jmp_buf_tag {
        __jmpbuf: [0; 8],
        __mask_was_saved: 0,
        __saved_mask: __sigset_t { __val: [0; 16] },
    }; 1];
    if (*(arr as *mut NODE)).sub.nodep.reflags as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        gprintf(
            out_fp,
            dcgettext(
                0 as *const libc::c_char,
                b"array `%s' is empty\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            arr_name,
        );
        return 0 as libc::c_int;
    }
    num_elems = (*(arr as *mut NODE)).sub.nodep.reflags as size_t;
    list = assoc_list(
        arr as *mut NODE,
        b"@ind_str_asc\0" as *const u8 as *const libc::c_char,
        SORTED_IN,
    );
    let fresh6 = pager_quit_tag_valid;
    pager_quit_tag_valid = pager_quit_tag_valid + 1;
    if fresh6 != 0 {
        memcpy(
            pager_quit_tag_stack.as_mut_ptr() as *mut libc::c_char as *mut libc::c_void,
            pager_quit_tag.as_mut_ptr() as *const libc::c_char as *const libc::c_void,
            ::std::mem::size_of::<jmp_buf>() as libc::c_ulong,
        );
    }
    if _setjmp(pager_quit_tag.as_mut_ptr()) == 0 as libc::c_int {
        i = 0 as libc::c_int;
        while ret == 0 as libc::c_int && (i as libc::c_ulong) < num_elems {
            subs = *list.offset(i as isize);
            r = *((*(*(arr as *mut NODE)).sub.nodep.l.lp).lookup)
                .unwrap()(arr as *mut NODE, subs) as *mut NODE;
            if (*r).type_0 as libc::c_uint
                == Node_var_array as libc::c_int as libc::c_uint
            {
                ::std::ptr::write_volatile(
                    &mut ret as *mut libc::c_int,
                    print_array(r, (*r).sub.nodep.name),
                );
            } else {
                gprintf(
                    out_fp,
                    b"%s[\"%.*s\"] = \0" as *const u8 as *const libc::c_char,
                    arr_name,
                    (*subs).sub.val.slen as libc::c_int,
                    (*subs).sub.val.sp,
                );
                valinfo(
                    r as *mut NODE,
                    Some(
                        gprintf
                            as unsafe extern "C" fn(
                                *mut FILE,
                                *const libc::c_char,
                                ...
                            ) -> libc::c_int,
                    ),
                    out_fp,
                );
            }
            i += 1;
            i;
        }
    } else {
        ::std::ptr::write_volatile(&mut ret as *mut libc::c_int, 1 as libc::c_int);
    }
    pager_quit_tag_valid -= 1;
    if pager_quit_tag_valid != 0 {
        memcpy(
            pager_quit_tag.as_mut_ptr() as *mut libc::c_char as *mut libc::c_void,
            pager_quit_tag_stack.as_mut_ptr() as *const libc::c_char
                as *const libc::c_void,
            ::std::mem::size_of::<jmp_buf>() as libc::c_ulong,
        );
    }
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) < num_elems {
        unref(*list.offset(i as isize));
        i += 1;
        i;
    }
    free(list as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn print_subscript(
    mut arr: *mut NODE,
    mut arr_name: *mut libc::c_char,
    mut a: *mut CMDARG,
    mut count: libc::c_int,
) {
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut subs: *mut NODE = 0 as *mut NODE;
    subs = (*a).value.nodeval;
    r = in_array(arr, subs);
    if r.is_null() {
        fprintf(
            out_fp,
            dcgettext(
                0 as *const libc::c_char,
                b"subscript \"%.*s\" is not in array `%s'\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*subs).sub.val.slen as libc::c_int,
            (*subs).sub.val.sp,
            arr_name,
        );
    } else if (*r).type_0 as libc::c_uint
        == Node_var_array as libc::c_int as libc::c_uint
    {
        if count > 1 as libc::c_int {
            print_subscript(r, (*r).sub.nodep.name, (*a).next, count - 1 as libc::c_int);
        } else {
            fprintf(
                out_fp,
                b"%s = \0" as *const u8 as *const libc::c_char,
                (*r).sub.nodep.name,
            );
            print_symbol(r, 0 as libc::c_int != 0);
        }
    } else {
        fprintf(
            out_fp,
            b"%s[\"%.*s\"] = \0" as *const u8 as *const libc::c_char,
            arr_name,
            (*subs).sub.val.slen as libc::c_int,
            (*subs).sub.val.sp,
        );
        valinfo(
            r,
            Some(
                fprintf
                    as unsafe extern "C" fn(
                        *mut FILE,
                        *const libc::c_char,
                        ...
                    ) -> libc::c_int,
            ),
            out_fp,
        );
    };
}
pub unsafe extern "C" fn do_print_var(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut a: *mut CMDARG = 0 as *mut CMDARG;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pname: *mut libc::c_char = 0 as *mut libc::c_char;
    a = arg;
    while !a.is_null() {
        match (*a).type_0 as libc::c_uint {
            45 => {
                name = (*a).value.sval;
                r = find_symbol(name, &mut pname);
                if !r.is_null() {
                    fprintf(
                        out_fp,
                        b"%s = \0" as *const u8 as *const libc::c_char,
                        name,
                    );
                    print_symbol(r, !pname.is_null());
                }
            }
            49 => {
                name = (*a).value.sval;
                r = find_array(name);
                if !r.is_null() {
                    print_subscript(r, name, (*a).next, (*a).a_count);
                }
            }
            48 => {
                name = (*a).value.sval;
                r = find_array(name);
                if !r.is_null() {
                    let mut count: libc::c_int = (*a).a_count;
                    while count > 0 as libc::c_int {
                        let mut value: *mut NODE = 0 as *mut NODE;
                        let mut subs: *mut NODE = 0 as *mut NODE;
                        a = (*a).next;
                        subs = (*a).value.nodeval;
                        value = in_array(r, subs);
                        if value.is_null() {
                            fprintf(
                                out_fp,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"subscript \"%.*s\" is not in array `%s'\n\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                (*subs).sub.val.slen as libc::c_int,
                                (*subs).sub.val.sp,
                                name,
                            );
                            break;
                        } else if (*value).type_0 as libc::c_uint
                            != Node_var_array as libc::c_int as libc::c_uint
                        {
                            fprintf(
                                out_fp,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"`%s[\"%.*s\"]' is not an array\n\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                name,
                                (*subs).sub.val.slen as libc::c_int,
                                (*subs).sub.val.sp,
                            );
                            break;
                        } else {
                            r = value;
                            name = (*r).sub.nodep.name;
                            count -= 1;
                            count;
                        }
                    }
                    if count == 0 as libc::c_int {
                        initialize_pager(out_fp);
                        print_array(r as *mut NODE, name);
                    }
                }
            }
            47 => {
                print_field((*(*a).value.nodeval).sub.val.fltnum as libc::c_long);
            }
            _ => {}
        }
        a = (*a).next;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn do_set_var(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut val: *mut NODE = 0 as *mut NODE;
    let mut lhs: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pname: *mut libc::c_char = 0 as *mut libc::c_char;
    match (*arg).type_0 as libc::c_uint {
        45 => {
            name = (*arg).value.sval;
            arg = (*arg).next;
            val = (*arg).value.nodeval;
            r = find_symbol(name, &mut pname);
            if !r.is_null() {
                let mut current_block_12: u64;
                match (*r).type_0 as libc::c_uint {
                    6 | 7 => {
                        (*r).type_0 = Node_var;
                        (*r).sub.nodep.l.lptr = dupnode(Nnull_string);
                        current_block_12 = 3818596144596519336;
                    }
                    4 => {
                        current_block_12 = 3818596144596519336;
                    }
                    _ => {
                        d_error(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"`%s' is not a scalar variable\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            name,
                        );
                        current_block_12 = 12349973810996921269;
                    }
                }
                match current_block_12 {
                    3818596144596519336 => {
                        lhs = &mut (*r).sub.nodep.l.lptr;
                        unref(*lhs);
                        *lhs = dupnode(val);
                        if pname.is_null() && ((*r).sub.nodep.x.aptr).is_some() {
                            ((*r).sub.nodep.x.aptr).unwrap()();
                        }
                        fprintf(
                            out_fp,
                            b"%s = \0" as *const u8 as *const libc::c_char,
                            name,
                        );
                        print_symbol(r, !pname.is_null());
                    }
                    _ => {}
                }
            }
        }
        49 => {
            let mut subs: *mut NODE = 0 as *mut NODE;
            let mut value: *mut NODE = 0 as *mut NODE;
            let mut count: libc::c_int = (*arg).a_count;
            let mut newval: *mut NODE = 0 as *mut NODE;
            name = (*arg).value.sval;
            r = find_array(name);
            if !r.is_null() {
                while count > 0 as libc::c_int {
                    arg = (*arg).next;
                    subs = (*arg).value.nodeval;
                    value = in_array(r, subs);
                    if count == 1 as libc::c_int {
                        if !value.is_null()
                            && (*value).type_0 as libc::c_uint
                                == Node_var_array as libc::c_int as libc::c_uint
                        {
                            d_error(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"attempt to use array `%s[\"%.*s\"]' in a scalar context\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                name,
                                (*subs).sub.val.slen as libc::c_int,
                                (*subs).sub.val.sp,
                            );
                        } else {
                            arg = (*arg).next;
                            val = (*arg).value.nodeval;
                            newval = dupnode(val);
                            assoc_set(r, dupnode(subs), newval);
                            fprintf(
                                out_fp,
                                b"%s[\"%.*s\"] = \0" as *const u8 as *const libc::c_char,
                                name,
                                (*subs).sub.val.slen as libc::c_int,
                                (*subs).sub.val.sp,
                            );
                            valinfo(
                                newval,
                                Some(
                                    fprintf
                                        as unsafe extern "C" fn(
                                            *mut FILE,
                                            *const libc::c_char,
                                            ...
                                        ) -> libc::c_int,
                                ),
                                out_fp,
                            );
                        }
                    } else if value.is_null() {
                        let mut array: *mut NODE = 0 as *mut NODE;
                        array = make_array();
                        (*array)
                            .sub
                            .nodep
                            .name = estrdup((*subs).sub.val.sp, (*subs).sub.val.slen);
                        (*array).sub.nodep.x.extra = r;
                        assoc_set(r, dupnode(subs), array);
                        r = array;
                    } else if (*value).type_0 as libc::c_uint
                        != Node_var_array as libc::c_int as libc::c_uint
                    {
                        d_error(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"attempt to use scalar `%s[\"%.*s\"]' as array\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            name,
                            (*subs).sub.val.slen as libc::c_int,
                            (*subs).sub.val.sp,
                        );
                        break;
                    } else {
                        r = value;
                        name = (*r).sub.nodep.name;
                    }
                    count -= 1;
                    count;
                }
            }
        }
        47 => {
            let mut field_num: libc::c_long = 0;
            let mut assign: Func_ptr = None;
            field_num = (*(*arg).value.nodeval).sub.val.fltnum as libc::c_long;
            arg = (*arg).next;
            val = (*arg).value.nodeval;
            lhs = get_field(field_num, &mut assign);
            if assign.is_some() {
                assign.unwrap()();
            }
            unref(*lhs);
            *lhs = dupnode(val);
            print_field(field_num);
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn find_item(
    mut list: *mut list_item,
    mut num: libc::c_long,
) -> *mut list_item {
    let mut d: *mut list_item = 0 as *mut list_item;
    if num <= 0 as libc::c_int as libc::c_long {
        return 0 as *mut list_item;
    }
    d = (*list).next;
    while d != list {
        if (*d).number as libc::c_long == num {
            return d;
        }
        d = (*d).next;
    }
    return 0 as *mut list_item;
}
unsafe extern "C" fn delete_item(mut d: *mut list_item) {
    let mut c: *mut commands_item = 0 as *mut commands_item;
    let mut i: libc::c_int = 0;
    if (*d).flags & 2 as libc::c_int != 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < (*d).num_subs {
            unref(*((*d).subs).offset(i as isize));
            i += 1;
            i;
        }
        free((*d).subs as *mut libc::c_void);
    } else if (*d).flags & 4 as libc::c_int != 0 as libc::c_int {
        unref((*d).symbol);
    }
    if (*d).flags & 16 as libc::c_int == 0 as libc::c_int {
        unref((*d).value[0 as libc::c_int as usize].n);
    }
    if (*d).flags & 8 as libc::c_int == 0 as libc::c_int {
        unref((*d).value[1 as libc::c_int as usize].n);
    }
    c = (*d).commands.next;
    while c != &mut (*d).commands as *mut commands_item {
        c = (*c).prev;
        delete_commands_item((*c).next);
        c = (*c).next;
    }
    free_context((*d).cndn.ctxt, 0 as libc::c_int != 0);
    if !((*d).cndn.expr).is_null() {
        free((*d).cndn.expr as *mut libc::c_void);
    }
    (*(*d).next).prev = (*d).prev;
    (*(*d).prev).next = (*d).next;
    free(d as *mut libc::c_void);
}
unsafe extern "C" fn add_item(
    mut list: *mut list_item,
    mut type_0: libc::c_int,
    mut symbol: *mut NODE,
    mut pname: *mut libc::c_char,
) -> *mut list_item {
    let mut d: *mut list_item = 0 as *mut list_item;
    d = ezalloc_real(
        ::std::mem::size_of::<list_item>() as libc::c_ulong,
        b"add_item\0" as *const u8 as *const libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char,
        b"debug.c\0" as *const u8 as *const libc::c_char,
        1403 as libc::c_int,
    ) as *mut list_item;
    (*d).commands.prev = &mut (*d).commands;
    (*d).commands.next = (*d).commands.prev;
    (*list).number += 1;
    (*d).number = (*list).number;
    (*d).sname = (*symbol).sub.nodep.name;
    if !pname.is_null() {
        (*d).sname = pname;
        (*d).flags |= 1 as libc::c_int;
        (*d).fcall_count = fcall_count - cur_frame;
    }
    if type_0 == D_field as libc::c_int {
        (*d).symbol = symbol;
        (*d).flags |= 4 as libc::c_int;
    } else if type_0 == D_subscript as libc::c_int {
        (*d).symbol = symbol;
        (*d).flags |= 2 as libc::c_int;
    } else {
        (*d).symbol = symbol;
    }
    (*d).next = (*list).next;
    (*d).prev = list;
    (*list).next = d;
    (*(*d).next).prev = d;
    return d;
}
unsafe extern "C" fn do_add_item(
    mut list: *mut list_item,
    mut arg: *mut CMDARG,
) -> *mut list_item {
    let mut symbol: *mut NODE = 0 as *mut NODE;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut item: *mut list_item = 0 as *mut list_item;
    match (*arg).type_0 as libc::c_uint {
        49 | 45 => {
            name = (*arg).value.sval;
            symbol = find_symbol(name, &mut pname);
            if symbol.is_null() {
                return 0 as *mut list_item;
            }
            if (*symbol).type_0 as libc::c_uint
                == Node_func as libc::c_int as libc::c_uint
            {
                d_error(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"`%s' is a function\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    name,
                );
                return 0 as *mut list_item;
            }
            if (*arg).type_0 as libc::c_uint
                == D_subscript as libc::c_int as libc::c_uint
                && (*symbol).type_0 as libc::c_uint
                    != Node_var_array as libc::c_int as libc::c_uint
            {
                d_error(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"`%s' is not an array\n\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    name,
                );
                return 0 as *mut list_item;
            }
            item = add_item(list, (*arg).type_0 as libc::c_int, symbol, pname);
            if !item.is_null()
                && (*arg).type_0 as libc::c_uint
                    == D_subscript as libc::c_int as libc::c_uint
            {
                let mut subs: *mut *mut NODE = 0 as *mut *mut NODE;
                let mut count: libc::c_int = (*arg).a_count;
                let mut i: libc::c_int = 0;
                subs = emalloc_real(
                    (count as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut NODE>() as libc::c_ulong,
                        ),
                    b"do_add_item\0" as *const u8 as *const libc::c_char,
                    b"subs\0" as *const u8 as *const libc::c_char,
                    b"debug.c\0" as *const u8 as *const libc::c_char,
                    1466 as libc::c_int,
                ) as *mut *mut NODE;
                i = 0 as libc::c_int;
                while i < count {
                    arg = (*arg).next;
                    let ref mut fresh7 = *subs.offset(i as isize);
                    *fresh7 = dupnode((*arg).value.nodeval);
                    let ref mut fresh8 = *subs.offset(i as isize);
                    *fresh8 = force_string_fmt(
                        *subs.offset(i as isize),
                        CONVFMT,
                        CONVFMTidx,
                    );
                    i += 1;
                    i;
                }
                (*item).subs = subs;
                (*item).num_subs = count;
            }
        }
        47 => {
            symbol = dupnode((*arg).value.nodeval);
            item = add_item(
                list,
                D_field as libc::c_int,
                symbol,
                0 as *mut libc::c_char,
            );
        }
        _ => {}
    }
    if list == &mut watch_list as *mut list_item {
        arg = (*arg).next;
        if !item.is_null() && !arg.is_null() {
            if parse_condition(D_watch as libc::c_int, (*item).number, (*arg).value.sval)
                == 0 as libc::c_int
            {
                (*arg).value.sval = 0 as *mut libc::c_char;
            } else {
                fprintf(
                    out_fp,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"watchpoint %d is unconditional\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*item).number,
                );
            }
        }
    }
    return item;
}
unsafe extern "C" fn do_delete_item(mut list: *mut list_item, mut arg: *mut CMDARG) {
    if arg.is_null() {
        while (*list).next != list {
            delete_item((*list).next);
        }
    }
    while !arg.is_null() {
        let mut d: *mut list_item = 0 as *mut list_item;
        if (*arg).type_0 as libc::c_uint == D_range as libc::c_int as libc::c_uint {
            let mut i: libc::c_long = 0;
            let mut j: libc::c_long = 0;
            i = (*arg).value.lval;
            arg = (*arg).next;
            j = (*arg).value.lval;
            if j > (*list).number as libc::c_long {
                j = (*list).number as libc::c_long;
            }
            while i <= j {
                d = find_item(list, i);
                if !d.is_null() {
                    delete_item(d);
                }
                i += 1;
                i;
            }
        } else {
            d = find_item(list, (*arg).value.lval);
            if d.is_null() {
                if list == &mut display_list as *mut list_item {
                    d_error(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"no display item numbered %ld\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*arg).value.lval,
                    );
                } else {
                    d_error(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"no watch item numbered %ld\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*arg).value.lval,
                    );
                }
            } else {
                delete_item(d);
            }
        }
        arg = (*arg).next;
    }
}
unsafe extern "C" fn display(mut d: *mut list_item) {
    let mut symbol: *mut NODE = 0 as *mut NODE;
    symbol = (*d).symbol;
    if (*d).flags & 1 as libc::c_int != 0 as libc::c_int
        && (*d).fcall_count != fcall_count - cur_frame
    {
        return;
    }
    let mut current_block_17: u64;
    if (*d).flags & 2 as libc::c_int != 0 as libc::c_int {
        let mut sub: *mut NODE = 0 as *mut NODE;
        let mut r: *mut NODE = 0 as *mut NODE;
        let mut i: libc::c_int = 0 as libc::c_int;
        let mut count: libc::c_int = (*d).num_subs;
        i = 0 as libc::c_int;
        loop {
            if !(i < count) {
                current_block_17 = 18317007320854588510;
                break;
            }
            sub = *((*d).subs).offset(i as isize);
            r = in_array(symbol, sub);
            if r.is_null() {
                fprintf(
                    out_fp,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%d: subscript \"%.*s\" is not in array `%s'\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*d).number,
                    (*sub).sub.val.slen as libc::c_int,
                    (*sub).sub.val.sp,
                    (*d).sname,
                );
                current_block_17 = 18317007320854588510;
                break;
            } else {
                if (*r).type_0 as libc::c_uint
                    == Node_var_array as libc::c_int as libc::c_uint
                {
                    symbol = r;
                    if i == count - 1 as libc::c_int {
                        current_block_17 = 10188494153548305905;
                        break;
                    }
                } else {
                    if i != count - 1 as libc::c_int {
                        return;
                    }
                    fprintf(
                        out_fp,
                        b"%d: %s[\"%.*s\"] = \0" as *const u8 as *const libc::c_char,
                        (*d).number,
                        (*d).sname,
                        (*sub).sub.val.slen as libc::c_int,
                        (*sub).sub.val.sp,
                    );
                    valinfo(
                        r,
                        Some(
                            fprintf
                                as unsafe extern "C" fn(
                                    *mut FILE,
                                    *const libc::c_char,
                                    ...
                                ) -> libc::c_int,
                        ),
                        out_fp,
                    );
                }
                i += 1;
                i;
            }
        }
    } else if (*d).flags & 4 as libc::c_int != 0 as libc::c_int {
        let mut r_0: *mut NODE = (*d).symbol;
        fprintf(out_fp, b"%d: \0" as *const u8 as *const libc::c_char, (*d).number);
        print_field((*r_0).sub.val.fltnum as libc::c_long);
        current_block_17 = 18317007320854588510;
    } else {
        current_block_17 = 10188494153548305905;
    }
    match current_block_17 {
        10188494153548305905 => {
            fprintf(
                out_fp,
                b"%d: %s = \0" as *const u8 as *const libc::c_char,
                (*d).number,
                (*d).sname,
            );
            print_symbol(symbol, (*d).flags & 1 as libc::c_int != 0 as libc::c_int);
        }
        _ => {}
    };
}
pub unsafe extern "C" fn do_display(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    let mut d: *mut list_item = 0 as *mut list_item;
    if arg.is_null() {
        d = display_list.prev;
        while d != &mut display_list as *mut list_item {
            display(d);
            d = (*d).prev;
        }
        return 0 as libc::c_int;
    }
    d = do_add_item(&mut display_list, arg);
    if !d.is_null() {
        display(d);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn do_undisplay(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    do_delete_item(&mut display_list, arg);
    return 0 as libc::c_int;
}
unsafe extern "C" fn condition_triggered(mut cndn: *mut condition) -> libc::c_int {
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut di: libc::c_int = 0;
    if ((*cndn).code).is_null() {
        return 1 as libc::c_int;
    }
    push_context((*cndn).ctxt);
    r = execute_code((*cndn).code as *mut INSTRUCTION);
    pop_context();
    if r.is_null() {
        return 0 as libc::c_int;
    }
    force_number(r);
    di = !((*r).sub.val.fltnum == 0.0f64) as libc::c_int;
    DEREF(r);
    return di;
}
unsafe extern "C" fn find_subscript(
    mut item: *mut list_item,
    mut ptr: *mut *mut NODE,
) -> libc::c_int {
    let mut symbol: *mut NODE = (*item).symbol;
    let mut sub: *mut NODE = 0 as *mut NODE;
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut count: libc::c_int = (*item).num_subs;
    *ptr = 0 as *mut NODE;
    r = *ptr;
    i = 0 as libc::c_int;
    while i < count {
        sub = *((*item).subs).offset(i as isize);
        r = in_array(symbol, sub);
        if r.is_null() {
            return 0 as libc::c_int;
        }
        if (*r).type_0 as libc::c_uint == Node_var_array as libc::c_int as libc::c_uint {
            symbol = r;
        } else if i < count - 1 as libc::c_int {
            return -(1 as libc::c_int)
        }
        i += 1;
        i;
    }
    if !r.is_null() {
        *ptr = r;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn cmp_val(
    mut w: *mut list_item,
    mut old: *mut NODE,
    mut new: *mut NODE,
) -> libc::c_int {
    if (*w).flags & 16 as libc::c_int != 0 as libc::c_int {
        let mut size: libc::c_long = 0 as libc::c_int as libc::c_long;
        if new.is_null() {
            return 1 as libc::c_int;
        }
        if (*new).type_0 as libc::c_uint == Node_val as libc::c_int as libc::c_uint {
            return 1 as libc::c_int;
        }
        size = (*new).sub.nodep.reflags as libc::c_long;
        if (*w).value[0 as libc::c_int as usize].l == size {
            return 0 as libc::c_int;
        }
        return 1 as libc::c_int;
    }
    if old.is_null() && new.is_null() {
        return 0 as libc::c_int;
    }
    if old.is_null() && !new.is_null() || !old.is_null() && new.is_null() {
        return 1 as libc::c_int;
    }
    if (*new).type_0 as libc::c_uint == Node_var_array as libc::c_int as libc::c_uint {
        return 1 as libc::c_int;
    }
    return cmp_nodes(old, new, 1 as libc::c_int != 0);
}
unsafe extern "C" fn watchpoint_triggered(mut w: *mut list_item) -> libc::c_int {
    let mut symbol: *mut NODE = 0 as *mut NODE;
    let mut t1: *mut NODE = 0 as *mut NODE;
    let mut t2: *mut NODE = 0 as *mut NODE;
    symbol = (*w).symbol;
    if (*w).flags & 1 as libc::c_int != 0 as libc::c_int
        && (*w).fcall_count != fcall_count - cur_frame
    {
        return 0 as libc::c_int;
    }
    if condition_triggered(&mut (*w).cndn) == 0 {
        return 0 as libc::c_int;
    }
    t1 = (*w).value[0 as libc::c_int as usize].n;
    t2 = 0 as *mut NODE;
    if (*w).flags & 2 as libc::c_int != 0 as libc::c_int {
        find_subscript(w, &mut t2);
    } else if (*w).flags & 4 as libc::c_int != 0 as libc::c_int {
        let mut field_num: libc::c_long = 0;
        field_num = (*(*w).symbol).sub.val.fltnum as libc::c_long;
        t2 = *get_field(field_num, 0 as *mut Func_ptr);
    } else {
        match (*symbol).type_0 as libc::c_uint {
            4 => {
                t2 = (*symbol).sub.nodep.l.lptr;
            }
            5 => {
                t2 = symbol;
            }
            6 | 7 => {}
            _ => {
                r_fatal(
                    b"internal error: file %s, line %d: unexpected symbol type %s\0"
                        as *const u8 as *const libc::c_char,
                    b"debug.c\0" as *const u8 as *const libc::c_char,
                    1740 as libc::c_int,
                    nodetype2str((*symbol).type_0),
                );
            }
        }
    }
    if cmp_val(w, t1, t2) == 0 {
        return 0 as libc::c_int;
    }
    if (*w).flags & 8 as libc::c_int == 0 as libc::c_int {
        unref((*w).value[1 as libc::c_int as usize].n);
    }
    (*w).flags &= !(8 as libc::c_int);
    if (*w).flags & 16 as libc::c_int != 0 as libc::c_int {
        (*w)
            .value[1 as libc::c_int as usize]
            .l = (*w).value[0 as libc::c_int as usize].l;
        (*w).flags |= 8 as libc::c_int;
        if t2.is_null() {
            (*w).flags &= !(16 as libc::c_int);
            (*w).value[0 as libc::c_int as usize].n = 0 as *mut NODE;
        } else if (*t2).type_0 as libc::c_uint == Node_val as libc::c_int as libc::c_uint
        {
            (*w).flags &= !(16 as libc::c_int);
            (*w).value[0 as libc::c_int as usize].n = dupnode(t2);
        } else {
            (*w)
                .value[0 as libc::c_int as usize]
                .l = (if (*t2).type_0 as libc::c_uint
                == Node_var_array as libc::c_int as libc::c_uint
            {
                (*t2).sub.nodep.reflags as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }) as libc::c_long;
        }
    } else if t1.is_null() {
        (*w).value[1 as libc::c_int as usize].n = 0 as *mut NODE;
        if (*t2).type_0 as libc::c_uint == Node_val as libc::c_int as libc::c_uint {
            (*w).value[0 as libc::c_int as usize].n = dupnode(t2);
        } else {
            (*w).flags |= 16 as libc::c_int;
            (*w)
                .value[0 as libc::c_int as usize]
                .l = (if (*t2).type_0 as libc::c_uint
                == Node_var_array as libc::c_int as libc::c_uint
            {
                (*t2).sub.nodep.reflags as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }) as libc::c_long;
        }
    } else {
        (*w)
            .value[1 as libc::c_int as usize]
            .n = (*w).value[0 as libc::c_int as usize].n;
        if t2.is_null() {
            (*w).value[0 as libc::c_int as usize].n = 0 as *mut NODE;
        } else if (*t2).type_0 as libc::c_uint
            == Node_var_array as libc::c_int as libc::c_uint
        {
            (*w).flags |= 16 as libc::c_int;
            (*w)
                .value[0 as libc::c_int as usize]
                .l = (*t2).sub.nodep.reflags as libc::c_long;
        } else {
            (*w).value[0 as libc::c_int as usize].n = dupnode(t2);
        }
    }
    return (*w).number;
}
unsafe extern "C" fn initialize_watch_item(mut w: *mut list_item) -> libc::c_int {
    let mut t: *mut NODE = 0 as *mut NODE;
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut symbol: *mut NODE = (*w).symbol;
    if (*w).flags & 2 as libc::c_int != 0 as libc::c_int {
        if find_subscript(w, &mut r) == -(1 as libc::c_int) {
            d_error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"attempt to use scalar value as array\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return -(1 as libc::c_int);
        }
        if r.is_null() {
            (*w).value[0 as libc::c_int as usize].n = 0 as *mut NODE;
        } else if (*r).type_0 as libc::c_uint
            == Node_var_array as libc::c_int as libc::c_uint
        {
            (*w).flags |= 16 as libc::c_int;
            (*w)
                .value[0 as libc::c_int as usize]
                .l = (*r).sub.nodep.reflags as libc::c_long;
        } else {
            (*w).value[0 as libc::c_int as usize].n = dupnode(r);
        }
    } else if (*w).flags & 4 as libc::c_int != 0 as libc::c_int {
        let mut field_num: libc::c_long = 0;
        t = (*w).symbol;
        field_num = (*t).sub.val.fltnum as libc::c_long;
        r = *get_field(field_num, 0 as *mut Func_ptr);
        (*w).value[0 as libc::c_int as usize].n = dupnode(r);
    } else if (*symbol).type_0 as libc::c_uint
        == Node_var_new as libc::c_int as libc::c_uint
        || (*symbol).type_0 as libc::c_uint
            == Node_elem_new as libc::c_int as libc::c_uint
    {
        (*w).value[0 as libc::c_int as usize].n = 0 as *mut NODE;
    } else if (*symbol).type_0 as libc::c_uint == Node_var as libc::c_int as libc::c_uint
    {
        r = (*symbol).sub.nodep.l.lptr;
        (*w).value[0 as libc::c_int as usize].n = dupnode(r);
    } else if (*symbol).type_0 as libc::c_uint
        == Node_var_array as libc::c_int as libc::c_uint
    {
        (*w).flags |= 16 as libc::c_int;
        (*w)
            .value[0 as libc::c_int as usize]
            .l = (*symbol).sub.nodep.reflags as libc::c_long;
    } else if (*symbol).type_0 as libc::c_uint == Node_val as libc::c_int as libc::c_uint
        && (*symbol).flags as libc::c_uint & REGEX as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
    {
        (*w).value[0 as libc::c_int as usize].n = dupnode(symbol);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn do_watch(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    let mut w: *mut list_item = 0 as *mut list_item;
    let mut symbol: *mut NODE = 0 as *mut NODE;
    let mut sub: *mut NODE = 0 as *mut NODE;
    let mut i: libc::c_int = 0;
    w = do_add_item(&mut watch_list, arg);
    if w.is_null() {
        return 0 as libc::c_int;
    }
    if initialize_watch_item(w) == -(1 as libc::c_int) {
        delete_item(w);
        return 0 as libc::c_int;
    }
    fprintf(
        out_fp,
        b"Watchpoint %d: \0" as *const u8 as *const libc::c_char,
        (*w).number,
    );
    symbol = (*w).symbol;
    if (*w).flags & 2 as libc::c_int != 0 as libc::c_int {
        fprintf(out_fp, b"%s\0" as *const u8 as *const libc::c_char, (*w).sname);
        i = 0 as libc::c_int;
        while i < (*w).num_subs {
            sub = *((*w).subs).offset(i as isize);
            fprintf(
                out_fp,
                b"[\"%.*s\"]\0" as *const u8 as *const libc::c_char,
                (*sub).sub.val.slen as libc::c_int,
                (*sub).sub.val.sp,
            );
            i += 1;
            i;
        }
        fprintf(out_fp, b"\n\0" as *const u8 as *const libc::c_char);
    } else if (*w).flags & 4 as libc::c_int != 0 as libc::c_int {
        fprintf(
            out_fp,
            b"$%ld\n\0" as *const u8 as *const libc::c_char,
            (*symbol).sub.val.fltnum as libc::c_long,
        );
    } else {
        fprintf(out_fp, b"%s\n\0" as *const u8 as *const libc::c_char, (*w).sname);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn do_unwatch(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    do_delete_item(&mut watch_list, arg);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn frame_popped() {
    let mut item: *mut list_item = 0 as *mut list_item;
    item = watch_list.next;
    while item != &mut watch_list as *mut list_item {
        if (*item).flags & 1 as libc::c_int != 0 as libc::c_int
            && (*item).fcall_count > fcall_count
        {
            fprintf(
                out_fp,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Watchpoint %d deleted because parameter is out of scope.\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*item).number,
            );
            item = (*item).prev;
            delete_item((*item).next);
        }
        item = (*item).next;
    }
    item = display_list.next;
    while item != &mut display_list as *mut list_item {
        if (*item).flags & 1 as libc::c_int != 0 as libc::c_int
            && (*item).fcall_count > fcall_count
        {
            fprintf(
                out_fp,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Display %d deleted because parameter is out of scope.\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*item).number,
            );
            item = (*item).prev;
            delete_item((*item).next);
        }
        item = (*item).next;
    }
}
unsafe extern "C" fn print_function(
    mut pc: *mut INSTRUCTION,
    mut x: *mut libc::c_void,
) -> libc::c_int {
    let mut func: *mut NODE = 0 as *mut NODE;
    let mut i: libc::c_int = 0;
    let mut pcount: libc::c_int = 0;
    let mut data: *mut pf_data = x as *mut pf_data;
    let mut defn: libc::c_int = (*data).defn as libc::c_int;
    let mut print_func: Func_print = (*data).print_func;
    let mut fp: *mut FILE = (*data).fp;
    func = (*pc).x.xn;
    pcount = (*func).sub.nodep.l.ll as libc::c_int;
    print_func
        .unwrap()(
        fp,
        b"%s(\0" as *const u8 as *const libc::c_char,
        (*func).sub.nodep.name,
    );
    i = 0 as libc::c_int;
    while i < pcount {
        print_func
            .unwrap()(
            fp,
            b"%s\0" as *const u8 as *const libc::c_char,
            (*((*func).sub.nodep.rn).offset(i as isize)).sub.nodep.name,
        );
        if i < pcount - 1 as libc::c_int {
            print_func.unwrap()(fp, b", \0" as *const u8 as *const libc::c_char);
        }
        i += 1;
        i;
    }
    print_func.unwrap()(fp, b")\0" as *const u8 as *const libc::c_char);
    if defn != 0 {
        print_func
            .unwrap()(
            fp,
            dcgettext(
                0 as *const libc::c_char,
                b" in file `%s', line %d\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*pc).d.name,
            (*pc).source_line as libc::c_int,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn print_frame(
    mut func: *mut NODE,
    mut src: *mut libc::c_char,
    mut srcline: libc::c_int,
) {
    if func.is_null() {
        fprintf(out_fp, b"main()\0" as *const u8 as *const libc::c_char);
    } else {
        pf_data
            .print_func = Some(
            fprintf
                as unsafe extern "C" fn(
                    *mut FILE,
                    *const libc::c_char,
                    ...
                ) -> libc::c_int,
        );
        pf_data.fp = out_fp;
        pf_data.defn = 0 as libc::c_int != 0;
        print_function(
            (*func).sub.nodep.r.iptr,
            &mut pf_data as *mut pf_data as *mut libc::c_void,
        );
    }
    fprintf(
        out_fp,
        dcgettext(
            0 as *const libc::c_char,
            b" at `%s':%d\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        src,
        srcline,
    );
}
unsafe extern "C" fn print_numbered_frame(mut num: libc::c_long) {
    let mut f: *mut NODE = 0 as *mut NODE;
    f = find_frame(num);
    if num == 0 as libc::c_int as libc::c_long {
        fprintf(out_fp, b"#%ld\t \0" as *const u8 as *const libc::c_char, num);
        print_frame((*f).sub.nodep.x.extra, source, sourceline);
    } else {
        fprintf(
            out_fp,
            dcgettext(
                0 as *const libc::c_char,
                b"#%ld\tin \0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            num,
        );
        print_frame(
            (*f).sub.nodep.x.extra,
            (*f).sub.nodep.name,
            (*((*find_frame(num - 1 as libc::c_int as libc::c_long)).sub.nodep.l.li
                as *mut INSTRUCTION))
                .source_line as libc::c_int,
        );
    }
    fprintf(out_fp, b"\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn do_backtrace(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    let mut cur: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut last: libc::c_long = fcall_count;
    if !prog_running {
        d_error(
            dcgettext(
                0 as *const libc::c_char,
                b"program not running\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as libc::c_int;
    }
    if !arg.is_null()
        && (*arg).type_0 as libc::c_uint == D_int as libc::c_int as libc::c_uint
    {
        let mut count: libc::c_long = (*arg).value.lval;
        if count >= 0 as libc::c_int as libc::c_long {
            last = count - 1 as libc::c_int as libc::c_long;
            if last > fcall_count {
                last = fcall_count;
            }
        } else {
            cur = 1 as libc::c_int as libc::c_long + fcall_count + count;
            if cur < 0 as libc::c_int as libc::c_long {
                cur = 0 as libc::c_int as libc::c_long;
            }
        }
    }
    while cur <= last {
        print_numbered_frame(cur);
        cur += 1;
        cur;
    }
    if cur <= fcall_count {
        fprintf(
            out_fp,
            dcgettext(
                0 as *const libc::c_char,
                b"More stack frames follow ...\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn print_cur_frame_and_sourceline() {
    let mut f: *mut NODE = 0 as *mut NODE;
    let mut srcline: libc::c_int = 0;
    let mut src: *mut libc::c_char = 0 as *mut libc::c_char;
    f = find_frame(cur_frame);
    if cur_frame == 0 as libc::c_int as libc::c_long {
        src = source;
        srcline = sourceline;
    } else {
        f = find_frame(cur_frame);
        src = (*f).sub.nodep.name;
        srcline = (*((*find_frame(cur_frame - 1 as libc::c_int as libc::c_long))
            .sub
            .nodep
            .l
            .li as *mut INSTRUCTION))
            .source_line as libc::c_int;
    }
    fprintf(
        out_fp,
        if cur_frame > 0 as libc::c_int as libc::c_long {
            dcgettext(
                0 as *const libc::c_char,
                b"#%ld\tin \0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ) as *const libc::c_char
        } else {
            b"#%ld\t \0" as *const u8 as *const libc::c_char
        },
        cur_frame,
    );
    print_frame((*f).sub.nodep.x.extra, src, srcline);
    fprintf(out_fp, b"\n\0" as *const u8 as *const libc::c_char);
    print_lines(src, srcline, 1 as libc::c_int);
    last_printed_line = srcline - list_size / 2 as libc::c_int;
    if last_printed_line < 0 as libc::c_int {
        last_printed_line = 0 as libc::c_int;
    }
}
pub unsafe extern "C" fn do_frame(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    if !prog_running {
        d_error(
            dcgettext(
                0 as *const libc::c_char,
                b"program not running\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as libc::c_int;
    }
    if !arg.is_null()
        && (*arg).type_0 as libc::c_uint == D_int as libc::c_int as libc::c_uint
    {
        if (*arg).value.lval < 0 as libc::c_int as libc::c_long
            || (*arg).value.lval > fcall_count
        {
            d_error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"invalid frame number\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return 0 as libc::c_int;
        }
        cur_frame = (*arg).value.lval;
    }
    print_cur_frame_and_sourceline();
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn do_up(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    if !prog_running {
        d_error(
            dcgettext(
                0 as *const libc::c_char,
                b"program not running\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as libc::c_int;
    }
    if !arg.is_null()
        && (*arg).type_0 as libc::c_uint == D_int as libc::c_int as libc::c_uint
    {
        cur_frame += (*arg).value.lval;
    } else {
        cur_frame += 1;
        cur_frame;
    }
    if cur_frame < 0 as libc::c_int as libc::c_long {
        cur_frame = 0 as libc::c_int as libc::c_long;
    } else if cur_frame > fcall_count {
        cur_frame = fcall_count;
    }
    print_cur_frame_and_sourceline();
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn do_down(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    if !prog_running {
        d_error(
            dcgettext(
                0 as *const libc::c_char,
                b"program not running\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as libc::c_int;
    }
    if !arg.is_null()
        && (*arg).type_0 as libc::c_uint == D_int as libc::c_int as libc::c_uint
    {
        cur_frame -= (*arg).value.lval;
    } else {
        cur_frame -= 1;
        cur_frame;
    }
    if cur_frame < 0 as libc::c_int as libc::c_long {
        cur_frame = 0 as libc::c_int as libc::c_long;
    } else if cur_frame > fcall_count {
        cur_frame = fcall_count;
    }
    print_cur_frame_and_sourceline();
    return 0 as libc::c_int;
}
unsafe extern "C" fn find_rule(
    mut src: *mut libc::c_char,
    mut lineno: libc::c_long,
) -> *mut INSTRUCTION {
    let mut rp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    if lineno == 0 as libc::c_int as libc::c_long {
        rp = (*rule_list).nexti;
        while !rp.is_null() {
            if (*rp.offset(-(1 as libc::c_int as isize))).d.name == src
                && (*rp.offset(-(1 as libc::c_int as isize))).source_line as libc::c_int
                    > 0 as libc::c_int
            {
                return rp.offset(-(1 as libc::c_int as isize));
            }
            rp = (*rp).nexti;
        }
    } else {
        rp = (*rule_list).nexti;
        while !rp.is_null() {
            if (*rp.offset(-(1 as libc::c_int as isize))).d.name == src
                && lineno
                    >= (*rp.offset(1 as libc::c_int as isize)).source_line
                        as libc::c_long
                && lineno <= (*rp.offset(1 as libc::c_int as isize)).x.xl
            {
                return rp.offset(-(1 as libc::c_int as isize));
            }
            rp = (*rp).nexti;
        }
    }
    return 0 as *mut INSTRUCTION;
}
unsafe extern "C" fn mk_breakpoint(
    mut src: *mut libc::c_char,
    mut srcline: libc::c_int,
) -> *mut INSTRUCTION {
    let mut bp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut b: *mut BREAKPOINT = 0 as *mut BREAKPOINT;
    bp = bcalloc(Op_breakpoint, 1 as libc::c_int, srcline);
    b = emalloc_real(
        ::std::mem::size_of::<BREAKPOINT>() as libc::c_ulong,
        b"mk_breakpoint\0" as *const u8 as *const libc::c_char,
        b"b\0" as *const u8 as *const libc::c_char,
        b"debug.c\0" as *const u8 as *const libc::c_char,
        2134 as libc::c_int,
    ) as *mut BREAKPOINT;
    memset(
        &mut (*b).cndn as *mut condition as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<condition>() as libc::c_ulong,
    );
    (*b).commands.prev = &mut (*b).commands;
    (*b).commands.next = (*b).commands.prev;
    (*b).silent = 0 as libc::c_int != 0;
    watch_list.number += 1;
    (*b).number = watch_list.number;
    (*b).ignore_count = 0 as libc::c_int as libc::c_long;
    (*b).hit_count = 0 as libc::c_int as libc::c_long;
    (*b).flags = 1 as libc::c_int as libc::c_short;
    (*b).src = src;
    (*bp).x.bpt = b;
    (*b).bpi = bp;
    (*b).next = breakpoints.next;
    (*b).prev = &mut breakpoints;
    breakpoints.next = b;
    (*(*b).next).prev = b;
    return bp;
}
unsafe extern "C" fn delete_breakpoint(mut b: *mut BREAKPOINT) {
    let mut pc: *mut INSTRUCTION = (*b).bpi;
    let mut c: *mut commands_item = 0 as *mut commands_item;
    (*pc).opcode = Op_no_op;
    (*pc).source_line = 0 as libc::c_int as libc::c_short;
    (*pc).x.bpt = 0 as *mut break_point;
    c = (*b).commands.next;
    while c != &mut (*b).commands as *mut commands_item {
        c = (*c).prev;
        delete_commands_item((*c).next);
        c = (*c).next;
    }
    free_context((*b).cndn.ctxt, 0 as libc::c_int != 0);
    if !((*b).cndn.expr).is_null() {
        free((*b).cndn.expr as *mut libc::c_void);
    }
    (*(*b).next).prev = (*b).prev;
    (*(*b).prev).next = (*b).next;
    free(b as *mut libc::c_void);
}
unsafe extern "C" fn find_breakpoint(mut num: libc::c_long) -> *mut BREAKPOINT {
    let mut b: *mut BREAKPOINT = 0 as *mut BREAKPOINT;
    if num <= 0 as libc::c_int as libc::c_long {
        return 0 as *mut BREAKPOINT;
    }
    b = breakpoints.next;
    while b != &mut breakpoints as *mut BREAKPOINT {
        if (*b).number as libc::c_long == num {
            return b;
        }
        b = (*b).next;
    }
    return 0 as *mut BREAKPOINT;
}
unsafe extern "C" fn add_breakpoint(
    mut prevp: *mut INSTRUCTION,
    mut ip: *mut INSTRUCTION,
    mut src: *mut libc::c_char,
    mut silent: bool,
) -> *mut BREAKPOINT {
    let mut b: *mut BREAKPOINT = 0 as *mut BREAKPOINT;
    let mut bp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut lineno: libc::c_int = (*ip).source_line as libc::c_int;
    while (*ip).opcode as libc::c_uint == Op_breakpoint as libc::c_int as libc::c_uint
        && (*ip).source_line as libc::c_int == lineno
    {
        if !silent {
            b = (*ip).x.bpt;
            if (*b).flags as libc::c_int & 1 as libc::c_int != 0 as libc::c_int {
                if (*b).flags as libc::c_int & 8 as libc::c_int != 0 as libc::c_int {
                    fprintf(
                        out_fp,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Note: breakpoint %d (enabled, ignore next %ld hits), also set at %s:%d\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*b).number,
                        (*b).ignore_count,
                        (*b).src,
                        lineno,
                    );
                } else {
                    fprintf(
                        out_fp,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Note: breakpoint %d (enabled), also set at %s:%d\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*b).number,
                        (*b).src,
                        lineno,
                    );
                }
            } else if (*b).flags as libc::c_int & 8 as libc::c_int != 0 as libc::c_int {
                fprintf(
                    out_fp,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Note: breakpoint %d (disabled, ignore next %ld hits), also set at %s:%d\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*b).number,
                    (*b).ignore_count,
                    (*b).src,
                    lineno,
                );
            } else {
                fprintf(
                    out_fp,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Note: breakpoint %d (disabled), also set at %s:%d\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*b).number,
                    (*b).src,
                    lineno,
                );
            }
        }
        prevp = ip;
        ip = (*ip).nexti;
    }
    bp = mk_breakpoint(src, lineno);
    (*prevp).nexti = bp;
    (*bp).nexti = ip;
    b = (*bp).x.bpt;
    if !silent {
        fprintf(
            out_fp,
            dcgettext(
                0 as *const libc::c_char,
                b"Breakpoint %d set at file `%s', line %d\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*b).number,
            src,
            lineno,
        );
    }
    return b;
}
unsafe extern "C" fn set_breakpoint_at(
    mut rp: *mut INSTRUCTION,
    mut lineno: libc::c_int,
    mut silent: bool,
) -> *mut BREAKPOINT {
    let mut ip: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut prevp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    prevp = rp;
    ip = (*rp).nexti;
    while !ip.is_null() {
        if (*ip).opcode as libc::c_uint == Op_K_case as libc::c_int as libc::c_uint {
            let mut i1: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
            let mut i2: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
            i2 = (*ip).d.di;
            i1 = (*i2).nexti;
            while i2 != (*ip).x.xi {
                if (*i1).source_line as libc::c_int >= lineno {
                    return add_breakpoint(i2, i1, (*rp).d.name, silent);
                }
                if i1 == (*ip).x.xi {
                    break;
                }
                i2 = i1;
                i1 = (*i1).nexti;
            }
        }
        if (*ip).source_line as libc::c_int >= lineno {
            return add_breakpoint(prevp, ip, (*rp).d.name, silent);
        }
        if ip == (*rp.offset(1 as libc::c_int as isize)).d.di {
            break;
        }
        prevp = ip;
        ip = (*ip).nexti;
    }
    return 0 as *mut BREAKPOINT;
}
unsafe extern "C" fn set_breakpoint_next(
    mut rp: *mut INSTRUCTION,
    mut ip: *mut INSTRUCTION,
) -> *mut BREAKPOINT {
    let mut prevp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    if ip == (*rp.offset(1 as libc::c_int as isize)).d.di {
        return 0 as *mut BREAKPOINT;
    }
    prevp = ip;
    if (*ip).opcode as libc::c_uint != Op_breakpoint as libc::c_int as libc::c_uint {
        ip = (*ip).nexti;
    }
    while !ip.is_null() {
        if (*ip).source_line as libc::c_int > 0 as libc::c_int {
            return add_breakpoint(prevp, ip, (*rp).d.name, 0 as libc::c_int != 0);
        }
        if ip == (*rp.offset(1 as libc::c_int as isize)).d.di {
            break;
        }
        prevp = ip;
        ip = (*ip).nexti;
    }
    return 0 as *mut BREAKPOINT;
}
unsafe extern "C" fn set_breakpoint(
    mut arg: *mut CMDARG,
    mut temporary: bool,
) -> libc::c_int {
    let mut lineno: libc::c_int = 0;
    let mut b: *mut BREAKPOINT = 0 as *mut BREAKPOINT;
    let mut rp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut ip: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut func: *mut NODE = 0 as *mut NODE;
    let mut s: *mut SRCFILE = cur_srcfile;
    let mut src: *mut libc::c_char = (*cur_srcfile).src;
    if arg.is_null() {
        if !prog_running {
            d_error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"program not running\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return 0 as libc::c_int;
        }
        if cur_frame == 0 as libc::c_int as libc::c_long {
            src = source;
            ip = cur_pc;
        } else {
            let mut f: *mut NODE = 0 as *mut NODE;
            f = find_frame(cur_frame);
            src = (*f).sub.nodep.name;
            ip = (*find_frame(cur_frame - 1 as libc::c_int as libc::c_long))
                .sub
                .nodep
                .l
                .li as *mut INSTRUCTION;
        }
        rp = find_rule(src, (*ip).source_line as libc::c_long);
        b = set_breakpoint_next(rp, ip);
        if b.is_null() {
            fprintf(
                out_fp,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot set breakpoint in file `%s'\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                src,
            );
        } else {
            if cur_frame == 0 as libc::c_int as libc::c_long {
                (*b)
                    .flags = ((*b).flags as libc::c_int | 8 as libc::c_int)
                    as libc::c_short;
                (*b).ignore_count = 1 as libc::c_int as libc::c_long;
            }
            if temporary {
                (*b)
                    .flags = ((*b).flags as libc::c_int | 4 as libc::c_int)
                    as libc::c_short;
            }
        }
        return 0 as libc::c_int;
    }
    let mut current_block_52: u64;
    match (*arg).type_0 as libc::c_uint {
        44 => {
            s = source_find((*arg).value.sval);
            arg = (*arg).next;
            if s.is_null() || arg.is_null()
                || (*arg).type_0 as libc::c_uint != D_int as libc::c_int as libc::c_uint
                    && (*arg).type_0 as libc::c_uint
                        != D_func as libc::c_int as libc::c_uint
            {
                return 0 as libc::c_int;
            }
            src = (*s).src;
            if (*arg).type_0 as libc::c_uint == D_func as libc::c_int as libc::c_uint {
                current_block_52 = 9940166174812100807;
            } else {
                current_block_52 = 6619403505443691090;
            }
        }
        43 => {
            current_block_52 = 6619403505443691090;
        }
        50 => {
            current_block_52 = 9940166174812100807;
        }
        _ => return 0 as libc::c_int,
    }
    match current_block_52 {
        6619403505443691090 => {
            lineno = (*arg).value.lval as libc::c_int;
            if lineno <= 0 as libc::c_int || lineno > (*s).srclines {
                d_error(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"line number %d in file `%s' is out of range\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    lineno,
                    src,
                );
            } else {
                rp = find_rule(src, lineno as libc::c_long);
                if rp.is_null() {
                    fprintf(
                        out_fp,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"internal error: cannot find rule\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                if rp.is_null()
                    || {
                        b = set_breakpoint_at(rp, lineno, 0 as libc::c_int != 0);
                        b.is_null()
                    }
                {
                    fprintf(
                        out_fp,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"cannot set breakpoint at `%s':%d\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        src,
                        lineno,
                    );
                }
                if !b.is_null() && temporary as libc::c_int != 0 {
                    (*b)
                        .flags = ((*b).flags as libc::c_int | 4 as libc::c_int)
                        as libc::c_short;
                }
            }
        }
        _ => {
            func = (*arg).value.nodeval;
            rp = (*func).sub.nodep.r.iptr;
            b = set_breakpoint_at(
                rp,
                (*rp).source_line as libc::c_int,
                0 as libc::c_int != 0,
            );
            if b.is_null() {
                fprintf(
                    out_fp,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"cannot set breakpoint in function `%s'\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*func).sub.nodep.name,
                );
            } else {
                if temporary {
                    (*b)
                        .flags = ((*b).flags as libc::c_int | 4 as libc::c_int)
                        as libc::c_short;
                }
                lineno = (*(*b).bpi).source_line as libc::c_int;
            }
        }
    }
    arg = (*arg).next;
    if !b.is_null() && !arg.is_null() {
        if parse_condition(D_break as libc::c_int, (*b).number, (*arg).value.sval)
            == 0 as libc::c_int
        {
            (*arg).value.sval = 0 as *mut libc::c_char;
        } else {
            fprintf(
                out_fp,
                dcgettext(
                    0 as *const libc::c_char,
                    b"breakpoint %d set at file `%s', line %d is unconditional\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*b).number,
                src,
                lineno,
            );
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn breakpoint_triggered(mut b: *mut BREAKPOINT) -> libc::c_int {
    if (*b).flags as libc::c_int & 1 as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*b).flags as libc::c_int & 8 as libc::c_int != 0 as libc::c_int {
        (*b).ignore_count -= 1;
        if (*b).ignore_count <= 0 as libc::c_int as libc::c_long {
            (*b)
                .flags = ((*b).flags as libc::c_int & !(8 as libc::c_int))
                as libc::c_short;
        }
        return 0 as libc::c_int;
    }
    if condition_triggered(&mut (*b).cndn) == 0 {
        return 0 as libc::c_int;
    }
    (*b).hit_count += 1;
    (*b).hit_count;
    if (*b).flags as libc::c_int & 2 as libc::c_int != 0 as libc::c_int {
        (*b).flags = ((*b).flags as libc::c_int & !(2 as libc::c_int)) as libc::c_short;
        (*b).flags = ((*b).flags as libc::c_int & !(1 as libc::c_int)) as libc::c_short;
    }
    return (*b).number;
}
pub unsafe extern "C" fn do_breakpoint(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    return set_breakpoint(arg, 0 as libc::c_int != 0);
}
pub unsafe extern "C" fn do_tmp_breakpoint(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    return set_breakpoint(arg, 1 as libc::c_int != 0);
}
pub unsafe extern "C" fn do_clear(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    let mut lineno: libc::c_int = 0;
    let mut b: *mut BREAKPOINT = 0 as *mut BREAKPOINT;
    let mut rp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut ip: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut func: *mut NODE = 0 as *mut NODE;
    let mut s: *mut SRCFILE = cur_srcfile;
    let mut src: *mut libc::c_char = (*cur_srcfile).src;
    let mut bp_found: libc::c_int = 0 as libc::c_int;
    if arg.is_null() {
        if !prog_running {
            d_error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"program not running\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return 0 as libc::c_int;
        }
        if cur_frame == 0 as libc::c_int as libc::c_long {
            lineno = sourceline;
            src = source;
        } else {
            let mut f: *mut NODE = 0 as *mut NODE;
            f = find_frame(cur_frame);
            src = (*f).sub.nodep.name;
            lineno = (*((*find_frame(cur_frame - 1 as libc::c_int as libc::c_long))
                .sub
                .nodep
                .l
                .li as *mut INSTRUCTION))
                .source_line as libc::c_int;
        }
    } else {
        's_191: {
            let mut current_block_36: u64;
            match (*arg).type_0 as libc::c_uint {
                44 => {
                    s = source_find((*arg).value.sval);
                    arg = (*arg).next;
                    if s.is_null() || arg.is_null()
                        || (*arg).type_0 as libc::c_uint
                            != D_int as libc::c_int as libc::c_uint
                            && (*arg).type_0 as libc::c_uint
                                != D_func as libc::c_int as libc::c_uint
                    {
                        return 0 as libc::c_int;
                    }
                    src = (*s).src;
                    if (*arg).type_0 as libc::c_uint
                        == D_func as libc::c_int as libc::c_uint
                    {
                        current_block_36 = 16564614947133486930;
                    } else {
                        current_block_36 = 13843204317578006203;
                    }
                }
                43 => {
                    current_block_36 = 13843204317578006203;
                }
                50 => {
                    current_block_36 = 16564614947133486930;
                }
                _ => {
                    current_block_36 = 10198629542455579378;
                }
            }
            match current_block_36 {
                16564614947133486930 => {
                    func = (*arg).value.nodeval;
                    rp = (*func).sub.nodep.r.iptr;
                    ip = (*rp).nexti;
                    while !ip.is_null() {
                        if !((*ip).source_line as libc::c_int <= 0 as libc::c_int) {
                            if (*ip).opcode as libc::c_uint
                                != Op_breakpoint as libc::c_int as libc::c_uint
                            {
                                break;
                            }
                            b = (*ip).x.bpt;
                            bp_found += 1;
                            if bp_found == 1 as libc::c_int {
                                fprintf(
                                    out_fp,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"Deleted breakpoint %d\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    (*b).number,
                                );
                            } else {
                                fprintf(
                                    out_fp,
                                    b", %d\0" as *const u8 as *const libc::c_char,
                                    (*b).number,
                                );
                            }
                            delete_breakpoint(b);
                        }
                        ip = (*ip).nexti;
                    }
                    if bp_found == 0 as libc::c_int {
                        fprintf(
                            out_fp,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"No breakpoint(s) at entry to function `%s'\n\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            (*func).sub.nodep.name,
                        );
                    } else {
                        fprintf(out_fp, b"\n\0" as *const u8 as *const libc::c_char);
                    }
                }
                13843204317578006203 => {
                    lineno = (*arg).value.lval as libc::c_int;
                    if lineno <= 0 as libc::c_int || lineno > (*s).srclines {
                        d_error(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"line number %d in file `%s' out of range\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            lineno,
                            src,
                        );
                        return 0 as libc::c_int;
                    }
                    break 's_191;
                }
                _ => {}
            }
            return 0 as libc::c_int;
        }
    }
    rp = find_rule(src, lineno as libc::c_long);
    if !rp.is_null() {
        ip = (*rp).nexti;
        while !ip.is_null() {
            if (*ip).opcode as libc::c_uint
                == Op_breakpoint as libc::c_int as libc::c_uint
                && (*ip).source_line as libc::c_int == lineno
            {
                b = (*ip).x.bpt;
                bp_found += 1;
                if bp_found == 1 as libc::c_int {
                    fprintf(
                        out_fp,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Deleted breakpoint %d\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*b).number,
                    );
                } else {
                    fprintf(
                        out_fp,
                        b", %d\0" as *const u8 as *const libc::c_char,
                        (*b).number,
                    );
                }
                delete_breakpoint(b);
            }
            if ip == (*rp.offset(1 as libc::c_int as isize)).d.di {
                break;
            }
            ip = (*ip).nexti;
        }
    }
    if bp_found == 0 as libc::c_int {
        fprintf(
            out_fp,
            dcgettext(
                0 as *const libc::c_char,
                b"No breakpoint at file `%s', line #%d\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            src,
            lineno,
        );
    } else {
        fprintf(out_fp, b"\n\0" as *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn enable_breakpoint(mut b: *mut BREAKPOINT, mut disp: libc::c_short) {
    (*b)
        .flags = ((*b).flags as libc::c_int & !(2 as libc::c_int | 4 as libc::c_int))
        as libc::c_short;
    (*b).flags = ((*b).flags as libc::c_int | 1 as libc::c_int) as libc::c_short;
    if disp != 0 {
        (*b).flags = ((*b).flags as libc::c_int | disp as libc::c_int) as libc::c_short;
    }
}
pub unsafe extern "C" fn do_enable_breakpoint(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    let mut b: *mut BREAKPOINT = 0 as *mut BREAKPOINT;
    let mut disp: libc::c_short = 0 as libc::c_int as libc::c_short;
    if !arg.is_null()
        && (*arg).type_0 as libc::c_uint == D_argument as libc::c_int as libc::c_uint
    {
        if (*arg).value.lval == A_DEL as libc::c_int as libc::c_long {
            disp = 4 as libc::c_int as libc::c_short;
        } else {
            disp = 2 as libc::c_int as libc::c_short;
        }
        arg = (*arg).next;
    }
    if arg.is_null() {
        b = breakpoints.next;
        while b != &mut breakpoints as *mut BREAKPOINT {
            enable_breakpoint(b, disp);
            b = (*b).next;
        }
    }
    while !arg.is_null() {
        if (*arg).type_0 as libc::c_uint == D_range as libc::c_int as libc::c_uint {
            let mut i: libc::c_long = 0;
            let mut j: libc::c_long = 0;
            i = (*arg).value.lval;
            arg = (*arg).next;
            j = (*arg).value.lval;
            if j > breakpoints.number as libc::c_long {
                j = breakpoints.number as libc::c_long;
            }
            while i <= j {
                b = find_breakpoint(i);
                if !b.is_null() {
                    enable_breakpoint(b, disp);
                }
                i += 1;
                i;
            }
        } else {
            b = find_breakpoint((*arg).value.lval);
            if b.is_null() {
                d_error(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"invalid breakpoint number\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            } else {
                enable_breakpoint(b, disp);
            }
        }
        arg = (*arg).next;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn do_delete_breakpoint(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    if arg.is_null() {
        let mut delete_all: bool = 1 as libc::c_int != 0;
        delete_all = prompt_yes_no(
            dcgettext(
                0 as *const libc::c_char,
                b"Delete all breakpoints? (y or n) \0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            *(dcgettext(
                0 as *const libc::c_char,
                b"y\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ))
                .offset(0 as libc::c_int as isize),
            1 as libc::c_int,
            out_fp,
        ) != 0;
        if delete_all {
            while breakpoints.next != &mut breakpoints as *mut BREAKPOINT {
                delete_breakpoint(breakpoints.next);
            }
        }
    }
    while !arg.is_null() {
        let mut b: *mut BREAKPOINT = 0 as *mut BREAKPOINT;
        if (*arg).type_0 as libc::c_uint == D_range as libc::c_int as libc::c_uint {
            let mut i: libc::c_long = 0;
            let mut j: libc::c_long = 0;
            i = (*arg).value.lval;
            arg = (*arg).next;
            j = (*arg).value.lval;
            if j > breakpoints.number as libc::c_long {
                j = breakpoints.number as libc::c_long;
            }
            while i <= j {
                b = find_breakpoint(i);
                if !b.is_null() {
                    delete_breakpoint(b);
                }
                i += 1;
                i;
            }
        } else {
            b = find_breakpoint((*arg).value.lval);
            if b.is_null() {
                d_error(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"invalid breakpoint number\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            } else {
                delete_breakpoint(b);
            }
        }
        arg = (*arg).next;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn do_ignore_breakpoint(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    let mut b: *mut BREAKPOINT = 0 as *mut BREAKPOINT;
    if arg.is_null()
        || (*arg).type_0 as libc::c_uint != D_int as libc::c_int as libc::c_uint
        || ((*arg).next).is_null()
        || (*(*arg).next).type_0 as libc::c_uint != D_int as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    b = find_breakpoint((*arg).value.lval);
    if b.is_null() {
        d_error(
            dcgettext(
                0 as *const libc::c_char,
                b"invalid breakpoint number\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else {
        (*b).ignore_count = (*(*arg).next).value.lval;
        if (*b).ignore_count > 0 as libc::c_int as libc::c_long {
            (*b).flags = ((*b).flags as libc::c_int | 8 as libc::c_int) as libc::c_short;
            fprintf(
                out_fp,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Will ignore next %ld crossing(s) of breakpoint %d.\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*b).ignore_count,
                (*b).number,
            );
        } else {
            (*b)
                .flags = ((*b).flags as libc::c_int & !(8 as libc::c_int))
                as libc::c_short;
            fprintf(
                out_fp,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Will stop next time breakpoint %d is reached.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*b).number,
            );
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn do_disable_breakpoint(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    let mut b: *mut BREAKPOINT = 0 as *mut BREAKPOINT;
    if arg.is_null() {
        b = breakpoints.next;
        while b != &mut breakpoints as *mut BREAKPOINT {
            (*b)
                .flags = ((*b).flags as libc::c_int & !(1 as libc::c_int))
                as libc::c_short;
            b = (*b).next;
        }
    }
    while !arg.is_null() {
        if (*arg).type_0 as libc::c_uint == D_range as libc::c_int as libc::c_uint {
            let mut i: libc::c_long = 0;
            let mut j: libc::c_long = 0;
            i = (*arg).value.lval;
            arg = (*arg).next;
            j = (*arg).value.lval;
            if j > breakpoints.number as libc::c_long {
                j = breakpoints.number as libc::c_long;
            }
            while i <= j {
                b = find_breakpoint(i);
                if !b.is_null() {
                    (*b)
                        .flags = ((*b).flags as libc::c_int & !(1 as libc::c_int))
                        as libc::c_short;
                }
                i += 1;
                i;
            }
        } else {
            b = find_breakpoint((*arg).value.lval);
            if b.is_null() {
                d_error(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"invalid breakpoint number\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            } else {
                (*b)
                    .flags = ((*b).flags as libc::c_int & !(1 as libc::c_int))
                    as libc::c_short;
            }
        }
        arg = (*arg).next;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn get_function() -> *mut NODE {
    let mut func: *mut NODE = 0 as *mut NODE;
    if !prog_running {
        return 0 as *mut NODE;
    }
    func = (*find_frame(cur_frame)).sub.nodep.x.extra;
    return func;
}
unsafe extern "C" fn initialize_readline() {
    rl_outstream = out_fp;
    rl_readline_name = b"gawk\0" as *const u8 as *const libc::c_char;
    rl_attempted_completion_function = Some(
        command_completion
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
                libc::c_int,
            ) -> *mut *mut libc::c_char,
    );
    read_a_line = Some(
        readline as unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_char,
    );
}
pub unsafe extern "C" fn init_debug() {
    register_exec_hook(
        Some(
            debug_pre_execute
                as unsafe extern "C" fn(*mut *mut INSTRUCTION) -> libc::c_int,
        ),
        Some(debug_post_execute as unsafe extern "C" fn(*mut INSTRUCTION) -> ()),
    );
}
pub unsafe extern "C" fn debug_prog(mut pc: *mut INSTRUCTION) -> libc::c_int {
    let mut run: *mut libc::c_char = 0 as *mut libc::c_char;
    input_fd = fileno(stdin);
    out_fp = stdout;
    if os_isatty(input_fd) != 0 {
        input_from_tty = 1 as libc::c_int != 0;
    }
    if input_fd == 0 as libc::c_int && input_from_tty as libc::c_int != 0 {
        initialize_readline();
    }
    if read_a_line.is_none() {
        read_a_line = Some(
            g_readline as unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_char,
        );
    }
    push_cmd_src(
        input_fd,
        input_from_tty,
        read_a_line,
        None,
        0 as libc::c_int,
        2 as libc::c_int,
    );
    setbuf(out_fp, 0 as *mut libc::c_void as *mut libc::c_char);
    cur_srcfile = (*srcfiles).prev;
    while cur_srcfile != srcfiles {
        if (*cur_srcfile).stype as libc::c_uint
            == SRC_FILE as libc::c_int as libc::c_uint
            || (*cur_srcfile).stype as libc::c_uint
                == SRC_INC as libc::c_int as libc::c_uint
        {
            break;
        }
        cur_srcfile = (*cur_srcfile).prev;
    }
    if cur_srcfile == srcfiles {
        fprintf(
            out_fp,
            dcgettext(
                0 as *const libc::c_char,
                b"Can only debug programs provided with the `-f' option.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        exit(1 as libc::c_int);
    }
    dgawk_prompt = estrdup(
        b"gawk> \0" as *const u8 as *const libc::c_char,
        strlen(b"gawk> \0" as *const u8 as *const libc::c_char),
    );
    dbg_prompt = dgawk_prompt;
    memset(
        &mut stop as *mut C2RustUnnamed_11 as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<C2RustUnnamed_11>() as libc::c_ulong,
    );
    stop.command = D_illegal;
    run = getenv(b"DGAWK_RESTART\0" as *const u8 as *const libc::c_char);
    if !run.is_null() {
        unserialize_list(BREAK as libc::c_int);
        unserialize_list(WATCH as libc::c_int);
        unserialize_list(DISPLAY as libc::c_int);
        unserialize_list(HISTORY as libc::c_int);
        unserialize_list(OPTION as libc::c_int);
        unsetenv(b"DGAWK_RESTART\0" as *const u8 as *const libc::c_char);
        fprintf(
            out_fp,
            dcgettext(
                0 as *const libc::c_char,
                b"Restarting ...\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        if strcasecmp(run, b"true\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            do_run(0 as *mut CMDARG, 0 as libc::c_int);
        }
    } else if !command_file.is_null() {
        let mut fd: libc::c_int = 0;
        fd = open_readfd(command_file);
        if fd == -(1 as libc::c_int) {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot open source file `%s' for reading: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                command_file,
                strerror(*__errno_location()),
            );
            exit(1 as libc::c_int);
        }
        push_cmd_src(
            fd,
            0 as libc::c_int != 0,
            Some(
                g_readline
                    as unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_char,
            ),
            Some(close as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
            0 as libc::c_int,
            1 as libc::c_int,
        );
        (*cmd_src).str_0 = estrdup(command_file, strlen(command_file));
    } else {
        let mut fd_0: libc::c_int = 0;
        read_history(history_file);
        sess_history_base = history_length;
        fd_0 = open_readfd(options_file);
        if fd_0 > -(1 as libc::c_int) {
            push_cmd_src(
                fd_0,
                0 as libc::c_int != 0,
                Some(
                    g_readline
                        as unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_char,
                ),
                Some(close as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
                0 as libc::c_int,
                0 as libc::c_int,
            );
        }
    }
    zzparse();
    return 0 as libc::c_int;
}
unsafe extern "C" fn check_watchpoint() -> libc::c_int {
    let mut w: *mut list_item = 0 as *mut list_item;
    if stop.command as libc::c_uint == D_return as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    w = watch_list.prev;
    while w != &mut watch_list as *mut list_item {
        let mut wnum: libc::c_int = watchpoint_triggered(w);
        if wnum > 0 as libc::c_int {
            stop.watch_point = wnum;
            stop.print_frame = 1 as libc::c_int != 0;
            return 1 as libc::c_int;
        }
        w = (*w).prev;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn check_breakpoint(mut pi: *mut *mut INSTRUCTION) -> libc::c_int {
    let mut pc: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    pc = *pi;
    if stop.command as libc::c_uint == D_return as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    if (*pc).opcode as libc::c_uint == Op_breakpoint as libc::c_int as libc::c_uint {
        let mut bnum: libc::c_int = 0;
        *pi = (*pc).nexti;
        bnum = breakpoint_triggered((*pc).x.bpt);
        if bnum > 0 as libc::c_int {
            stop.break_point = bnum;
            stop.print_frame = 1 as libc::c_int != 0;
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn restart(mut run: bool) -> ! {
    serialize_list(BREAK as libc::c_int);
    serialize_list(WATCH as libc::c_int);
    serialize_list(DISPLAY as libc::c_int);
    serialize_list(HISTORY as libc::c_int);
    serialize_list(OPTION as libc::c_int);
    setenv(
        b"DGAWK_RESTART\0" as *const u8 as *const libc::c_char,
        if run as libc::c_int != 0 {
            b"true\0" as *const u8 as *const libc::c_char
        } else {
            b"false\0" as *const u8 as *const libc::c_char
        },
        1 as libc::c_int,
    );
    close_all();
    execvp(
        *d_argv.offset(0 as libc::c_int as isize),
        d_argv as *const *mut libc::c_char,
    );
    fprintf(
        out_fp,
        dcgettext(
            0 as *const libc::c_char,
            b"Failed to restart debugger\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    exit(1 as libc::c_int);
}
pub unsafe extern "C" fn do_run(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    if prog_running {
        if !input_from_tty {
            need_restart = 1 as libc::c_int != 0;
        } else {
            need_restart = prompt_yes_no(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Program already running. Restart from beginning (y/n)? \0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                *(dcgettext(
                    0 as *const libc::c_char,
                    b"y\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ))
                    .offset(0 as libc::c_int as isize),
                0 as libc::c_int,
                out_fp,
            ) != 0;
            if !need_restart {
                fprintf(
                    out_fp,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Program not restarted\n\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                return 0 as libc::c_int;
            }
        }
    }
    if need_restart {
        if !command_file.is_null() {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"error: cannot restart, operation not allowed\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            exit(1 as libc::c_int);
        }
        if (*cmd_src).cmd == D_source as libc::c_int {
            fprintf(
                out_fp,
                dcgettext(
                    0 as *const libc::c_char,
                    b"error (%s): cannot restart, ignoring rest of the commands\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*cmd_src).str_0,
            );
            pop_cmd_src();
            return 0 as libc::c_int;
        }
        restart(1 as libc::c_int != 0);
    }
    fprintf(
        out_fp,
        dcgettext(
            0 as *const libc::c_char,
            b"Starting program:\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    prog_running = 1 as libc::c_int != 0;
    fatal_tag_valid = 1 as libc::c_int;
    if _setjmp(fatal_tag.as_mut_ptr()) == 0 as libc::c_int {
        interpret.unwrap()(code_block);
    }
    fatal_tag_valid = 0 as libc::c_int;
    prog_running = 0 as libc::c_int != 0;
    fprintf(
        out_fp,
        if !exiting && exit_val != 0 as libc::c_int {
            dcgettext(
                0 as *const libc::c_char,
                b"Program exited abnormally with exit value: %d\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            )
        } else {
            dcgettext(
                0 as *const libc::c_char,
                b"Program exited normally with exit value: %d\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            )
        },
        exit_val,
    );
    need_restart = 1 as libc::c_int != 0;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn do_quit(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    let mut terminate: bool = 1 as libc::c_int != 0;
    if prog_running {
        terminate = prompt_yes_no(
            dcgettext(
                0 as *const libc::c_char,
                b"The program is running. Exit anyway (y/n)? \0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            *(dcgettext(
                0 as *const libc::c_char,
                b"y\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ))
                .offset(0 as libc::c_int as isize),
            1 as libc::c_int,
            out_fp,
        ) != 0;
    }
    if terminate {
        close_all();
        do_trace = 0 as libc::c_int;
        if do_save_history != 0 && input_from_tty as libc::c_int != 0 {
            let mut ret: libc::c_int = 0;
            ret = write_history(history_file);
            if ret == 0 as libc::c_int && history_length > history_size {
                history_truncate_file(history_file, history_size);
            }
        }
        if do_save_options != 0 && input_from_tty as libc::c_int != 0 {
            save_options(options_file);
        }
        exit(exit_val);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn do_continue(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    let mut b: *mut BREAKPOINT = 0 as *mut BREAKPOINT;
    if !prog_running {
        d_error(
            dcgettext(
                0 as *const libc::c_char,
                b"program not running\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as libc::c_int;
    }
    if arg.is_null()
        || (*arg).type_0 as libc::c_uint != D_int as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    if stop.break_point == 0 {
        fprintf(
            out_fp,
            dcgettext(
                0 as *const libc::c_char,
                b"Not stopped at any breakpoint; argument ignored.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 1 as libc::c_int;
    }
    b = find_breakpoint(stop.break_point as libc::c_long);
    if b.is_null() {
        d_error(
            dcgettext(
                0 as *const libc::c_char,
                b"invalid breakpoint number %d\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stop.break_point,
        );
        return 0 as libc::c_int;
    }
    (*b).flags = ((*b).flags as libc::c_int | 8 as libc::c_int) as libc::c_short;
    (*b).ignore_count = (*arg).value.lval;
    fprintf(
        out_fp,
        dcgettext(
            0 as *const libc::c_char,
            b"Will ignore next %ld crossings of breakpoint %d.\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        (*b).ignore_count,
        stop.break_point,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn next_step(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    if !prog_running {
        d_error(
            dcgettext(
                0 as *const libc::c_char,
                b"program not running\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as libc::c_int;
    }
    if !arg.is_null()
        && (*arg).type_0 as libc::c_uint == D_int as libc::c_int as libc::c_uint
    {
        stop.repeat_count = (*arg).value.lval as libc::c_int;
    } else {
        stop.repeat_count = 1 as libc::c_int;
    }
    stop.command = cmd as argtype;
    return 1 as libc::c_int;
}
unsafe extern "C" fn check_step(mut pi: *mut *mut INSTRUCTION) -> libc::c_int {
    if fcall_count != stop.fcall_count {
        stop.fcall_count = fcall_count;
        stop.sourceline = sourceline;
        stop.source = source;
        stop.print_frame = 1 as libc::c_int != 0;
        stop.repeat_count -= 1;
        return (stop.repeat_count == 0 as libc::c_int) as libc::c_int;
    }
    if source != stop.source {
        stop.source = source;
        stop.sourceline = sourceline;
        stop.repeat_count -= 1;
        return (stop.repeat_count == 0 as libc::c_int) as libc::c_int;
    }
    if sourceline != stop.sourceline {
        stop.sourceline = sourceline;
        stop.repeat_count -= 1;
        return (stop.repeat_count == 0 as libc::c_int) as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn do_step(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = next_step(arg, cmd);
    if ret != 0 {
        stop.fcall_count = fcall_count;
        stop.source = source;
        stop.sourceline = sourceline;
        stop
            .check_func = Some(
            check_step as unsafe extern "C" fn(*mut *mut INSTRUCTION) -> libc::c_int,
        );
    }
    return ret;
}
unsafe extern "C" fn check_stepi(mut pi: *mut *mut INSTRUCTION) -> libc::c_int {
    stop.repeat_count -= 1;
    return (stop.repeat_count == 0 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn do_stepi(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = next_step(arg, cmd);
    if ret != 0 {
        stop
            .check_func = Some(
            check_stepi as unsafe extern "C" fn(*mut *mut INSTRUCTION) -> libc::c_int,
        );
    }
    return ret;
}
unsafe extern "C" fn check_next(mut pi: *mut *mut INSTRUCTION) -> libc::c_int {
    if fcall_count < stop.fcall_count {
        stop.fcall_count = fcall_count;
        stop.sourceline = sourceline;
        stop.source = source;
        stop.print_frame = 1 as libc::c_int != 0;
        stop.repeat_count -= 1;
        return (stop.repeat_count == 0 as libc::c_int) as libc::c_int;
    }
    if fcall_count == stop.fcall_count {
        if source != stop.source {
            stop.source = source;
            stop.sourceline = sourceline;
            stop.repeat_count -= 1;
            return (stop.repeat_count == 0 as libc::c_int) as libc::c_int;
        }
        if sourceline != stop.sourceline {
            stop.sourceline = sourceline;
            stop.repeat_count -= 1;
            return (stop.repeat_count == 0 as libc::c_int) as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn do_next(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = next_step(arg, cmd);
    if ret != 0 {
        stop.source = source;
        stop.sourceline = sourceline;
        stop.fcall_count = fcall_count;
        stop
            .check_func = Some(
            check_next as unsafe extern "C" fn(*mut *mut INSTRUCTION) -> libc::c_int,
        );
    }
    return ret;
}
unsafe extern "C" fn check_nexti(mut pi: *mut *mut INSTRUCTION) -> libc::c_int {
    if fcall_count < stop.fcall_count {
        stop.print_frame = 1 as libc::c_int != 0;
        stop.fcall_count = fcall_count;
    }
    return (fcall_count == stop.fcall_count
        && {
            stop.repeat_count -= 1;
            stop.repeat_count == 0 as libc::c_int
        }) as libc::c_int;
}
pub unsafe extern "C" fn do_nexti(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = next_step(arg, cmd);
    if ret != 0 {
        stop.fcall_count = fcall_count;
        stop
            .check_func = Some(
            check_nexti as unsafe extern "C" fn(*mut *mut INSTRUCTION) -> libc::c_int,
        );
    }
    return ret;
}
unsafe extern "C" fn check_finish(mut pi: *mut *mut INSTRUCTION) -> libc::c_int {
    if fcall_count == stop.fcall_count {
        stop.print_frame = 1 as libc::c_int != 0;
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn do_finish(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    if !prog_running {
        d_error(
            dcgettext(
                0 as *const libc::c_char,
                b"program not running\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as libc::c_int;
    }
    if cur_frame == fcall_count {
        fprintf(
            out_fp,
            dcgettext(
                0 as *const libc::c_char,
                b"'finish' not meaningful in the outermost frame main()\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as libc::c_int;
    }
    stop.fcall_count = fcall_count - cur_frame - 1 as libc::c_int as libc::c_long;
    fprintf(
        out_fp,
        dcgettext(
            0 as *const libc::c_char,
            b"Run until return from \0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    print_numbered_frame(cur_frame);
    stop
        .check_func = Some(
        check_finish as unsafe extern "C" fn(*mut *mut INSTRUCTION) -> libc::c_int,
    );
    stop.command = cmd as argtype;
    stop.print_ret = 1 as libc::c_int != 0;
    return 1 as libc::c_int;
}
unsafe extern "C" fn check_return(mut pi: *mut *mut INSTRUCTION) -> libc::c_int {
    if fcall_count == stop.fcall_count {
        stop.print_frame = 1 as libc::c_int != 0;
        return 1 as libc::c_int;
    }
    if fcall_count > stop.fcall_count {
        let mut func: *mut NODE = 0 as *mut NODE;
        func = (*find_frame(cur_frame)).sub.nodep.x.extra;
        *pi = (*((*func).sub.nodep.r.iptr).offset(1 as libc::c_int as isize)).d.di;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn do_return(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    let mut func: *mut NODE = 0 as *mut NODE;
    let mut n: *mut NODE = 0 as *mut NODE;
    if !prog_running {
        d_error(
            dcgettext(
                0 as *const libc::c_char,
                b"program not running\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as libc::c_int;
    }
    func = (*find_frame(cur_frame)).sub.nodep.x.extra;
    if func.is_null() {
        fprintf(
            out_fp,
            dcgettext(
                0 as *const libc::c_char,
                b"'return' not meaningful in the outermost frame main()\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as libc::c_int;
    }
    stop.fcall_count = fcall_count - cur_frame - 1 as libc::c_int as libc::c_long;
    stop.pc = (*((*func).sub.nodep.r.iptr).offset(1 as libc::c_int as isize)).d.di;
    stop.command = cmd as argtype;
    stop
        .check_func = Some(
        check_return as unsafe extern "C" fn(*mut *mut INSTRUCTION) -> libc::c_int,
    );
    if !arg.is_null()
        && (*arg).type_0 as libc::c_uint == D_node as libc::c_int as libc::c_uint
    {
        n = dupnode((*arg).value.nodeval);
    } else {
        n = dupnode(Nnull_string);
    }
    let ref mut fresh9 = (*if stack_ptr < stack_top {
        stack_ptr = stack_ptr.offset(1);
        stack_ptr
    } else {
        grow_stack()
    })
        .rptr;
    *fresh9 = n;
    return 1 as libc::c_int;
}
unsafe extern "C" fn check_until(mut pi: *mut *mut INSTRUCTION) -> libc::c_int {
    if fcall_count < stop.fcall_count {
        stop.print_frame = 1 as libc::c_int != 0;
        return 1 as libc::c_int;
    } else if fcall_count == stop.fcall_count {
        if !(stop.pc).is_null() && *pi == stop.pc {
            return 1 as libc::c_int;
        }
        if stop.sourceline > 0 as libc::c_int && source == stop.source
            && sourceline > stop.sourceline
        {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn do_until(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    let mut s: *mut SRCFILE = cur_srcfile;
    let mut src: *mut libc::c_char = (*cur_srcfile).src;
    let mut lineno: libc::c_int = 0;
    let mut rp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut ip: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut func: *mut NODE = 0 as *mut NODE;
    if !prog_running {
        d_error(
            dcgettext(
                0 as *const libc::c_char,
                b"program not running\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as libc::c_int;
    }
    stop.pc = 0 as *mut INSTRUCTION;
    stop.sourceline = 0 as libc::c_int;
    if arg.is_null() {
        stop.source = source;
        stop.sourceline = sourceline;
        stop.fcall_count = fcall_count - cur_frame;
        stop
            .check_func = Some(
            check_until as unsafe extern "C" fn(*mut *mut INSTRUCTION) -> libc::c_int,
        );
        stop.command = cmd as argtype;
        return 1 as libc::c_int;
    }
    's_166: {
        let mut current_block_39: u64;
        match (*arg).type_0 as libc::c_uint {
            44 => {
                s = source_find((*arg).value.sval);
                arg = (*arg).next;
                if s.is_null() || arg.is_null()
                    || (*arg).type_0 as libc::c_uint
                        != D_int as libc::c_int as libc::c_uint
                        && (*arg).type_0 as libc::c_uint
                            != D_func as libc::c_int as libc::c_uint
                {
                    return 0 as libc::c_int;
                }
                src = (*s).src;
                if (*arg).type_0 as libc::c_uint == D_func as libc::c_int as libc::c_uint
                {
                    current_block_39 = 16107769607128421004;
                } else {
                    current_block_39 = 7591382846069852772;
                }
            }
            43 => {
                current_block_39 = 7591382846069852772;
            }
            50 => {
                current_block_39 = 16107769607128421004;
            }
            _ => {
                current_block_39 = 6088938345876660418;
            }
        }
        match current_block_39 {
            16107769607128421004 => {
                func = (*arg).value.nodeval;
                rp = (*func).sub.nodep.r.iptr;
                ip = (*rp).nexti;
                while !ip.is_null() {
                    if (*ip).opcode as libc::c_uint
                        != Op_breakpoint as libc::c_int as libc::c_uint
                        && (*ip).source_line as libc::c_int > 0 as libc::c_int
                    {
                        stop.pc = ip;
                        stop.fcall_count = fcall_count - cur_frame;
                        stop
                            .check_func = Some(
                            check_until
                                as unsafe extern "C" fn(
                                    *mut *mut INSTRUCTION,
                                ) -> libc::c_int,
                        );
                        stop.command = cmd as argtype;
                        return 1 as libc::c_int;
                    }
                    ip = (*ip).nexti;
                }
                fprintf(
                    out_fp,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"cannot find specified location in function `%s'\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*func).sub.nodep.name,
                );
            }
            7591382846069852772 => {
                lineno = (*arg).value.lval as libc::c_int;
                if lineno <= 0 as libc::c_int || lineno > (*s).srclines {
                    d_error(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"line number %d in file `%s' out of range\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        lineno,
                        src,
                    );
                    return 0 as libc::c_int;
                }
                break 's_166;
            }
            _ => {}
        }
        return 0 as libc::c_int;
    }
    rp = find_rule(src, lineno as libc::c_long);
    if rp.is_null() {
        d_error(
            dcgettext(
                0 as *const libc::c_char,
                b"invalid source line %d in file `%s'\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            lineno,
            src,
        );
        return 0 as libc::c_int;
    }
    ip = (*rp).nexti;
    while !ip.is_null() {
        if (*ip).opcode as libc::c_uint != Op_breakpoint as libc::c_int as libc::c_uint
            && (*ip).source_line as libc::c_int >= lineno
        {
            stop.pc = ip;
            stop.fcall_count = fcall_count - cur_frame;
            stop
                .check_func = Some(
                check_until as unsafe extern "C" fn(*mut *mut INSTRUCTION) -> libc::c_int,
            );
            stop.command = cmd as argtype;
            return 1 as libc::c_int;
        }
        if ip == (*rp.offset(1 as libc::c_int as isize)).d.di {
            break;
        }
        ip = (*ip).nexti;
    }
    fprintf(
        out_fp,
        dcgettext(
            0 as *const libc::c_char,
            b"cannot find specified location %d in file `%s'\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        lineno,
        src,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn print_watch_item(mut w: *mut list_item) {
    let mut symbol: *mut NODE = 0 as *mut NODE;
    let mut sub: *mut NODE = 0 as *mut NODE;
    let mut i: libc::c_int = 0;
    symbol = (*w).symbol;
    if (*w).flags & 2 as libc::c_int != 0 as libc::c_int {
        fprintf(out_fp, b"%s\0" as *const u8 as *const libc::c_char, (*w).sname);
        i = 0 as libc::c_int;
        while i < (*w).num_subs {
            sub = *((*w).subs).offset(i as isize);
            fprintf(
                out_fp,
                b"[\"%.*s\"]\0" as *const u8 as *const libc::c_char,
                (*sub).sub.val.slen as libc::c_int,
                (*sub).sub.val.sp,
            );
            i += 1;
            i;
        }
        fprintf(out_fp, b"\n\0" as *const u8 as *const libc::c_char);
    } else if (*w).flags & 4 as libc::c_int != 0 as libc::c_int {
        fprintf(
            out_fp,
            b"$%ld\n\0" as *const u8 as *const libc::c_char,
            (*symbol).sub.val.fltnum as libc::c_long,
        );
    } else {
        fprintf(out_fp, b"%s\n\0" as *const u8 as *const libc::c_char, (*w).sname);
    }
    fprintf(out_fp, b"  Old value: \0" as *const u8 as *const libc::c_char);
    if (*w).flags & 8 as libc::c_int != 0 as libc::c_int {
        fprintf(
            out_fp,
            b"array, %ld elements\n\0" as *const u8 as *const libc::c_char,
            (*w).value[1 as libc::c_int as usize].l,
        );
    } else if ((*w).value[1 as libc::c_int as usize].n).is_null() {
        fprintf(
            out_fp,
            if (*w).flags & 2 as libc::c_int != 0 as libc::c_int {
                dcgettext(
                    0 as *const libc::c_char,
                    b"element not in array\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            } else {
                dcgettext(
                    0 as *const libc::c_char,
                    b"untyped variable\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            },
        );
    } else {
        valinfo(
            (*w).value[1 as libc::c_int as usize].n,
            Some(
                fprintf
                    as unsafe extern "C" fn(
                        *mut FILE,
                        *const libc::c_char,
                        ...
                    ) -> libc::c_int,
            ),
            out_fp,
        );
    }
    fprintf(out_fp, b"  New value: \0" as *const u8 as *const libc::c_char);
    if (*w).flags & 16 as libc::c_int != 0 as libc::c_int {
        fprintf(
            out_fp,
            b"array, %ld elements\n\0" as *const u8 as *const libc::c_char,
            (*w).value[0 as libc::c_int as usize].l,
        );
    } else if ((*w).value[0 as libc::c_int as usize].n).is_null() {
        fprintf(
            out_fp,
            if (*w).flags & 2 as libc::c_int != 0 as libc::c_int {
                dcgettext(
                    0 as *const libc::c_char,
                    b"element not in array\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            } else {
                dcgettext(
                    0 as *const libc::c_char,
                    b"untyped variable\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            },
        );
    } else {
        valinfo(
            (*w).value[0 as libc::c_int as usize].n,
            Some(
                fprintf
                    as unsafe extern "C" fn(
                        *mut FILE,
                        *const libc::c_char,
                        ...
                    ) -> libc::c_int,
            ),
            out_fp,
        );
    };
}
unsafe extern "C" fn next_command() {
    let mut current_block: u64;
    static mut last_rule: libc::c_int = 0 as libc::c_int;
    let mut d: *mut list_item = 0 as *mut list_item;
    let mut w: *mut list_item = 0 as *mut list_item;
    let mut b: *mut BREAKPOINT = 0 as *mut BREAKPOINT;
    let mut s: *mut SRCFILE = 0 as *mut SRCFILE;
    if source.is_null() {
        stop.command = D_illegal;
        stop.check_func = None;
        return;
    }
    if stop.break_point != 0 {
        b = find_breakpoint(stop.break_point as libc::c_long);
        if (*b).silent {
            current_block = 13046795191238241430;
        } else {
            current_block = 7746791466490516765;
        }
    } else if stop.watch_point != 0 {
        w = find_item(&mut watch_list, stop.watch_point as libc::c_long);
        if (*w).silent != 0 {
            current_block = 13046795191238241430;
        } else {
            current_block = 7746791466490516765;
        }
    } else {
        current_block = 7746791466490516765;
    }
    match current_block {
        7746791466490516765 => {
            if cur_rule != last_rule {
                fprintf(
                    out_fp,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Stopping in %s ...\n\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    *ruletab.as_ptr().offset(cur_rule as isize),
                );
                last_rule = cur_rule;
            }
            if !b.is_null() {
                fprintf(
                    out_fp,
                    b"Breakpoint %d, \0" as *const u8 as *const libc::c_char,
                    (*b).number,
                );
            } else if !w.is_null() {
                fprintf(
                    out_fp,
                    b"Watchpoint %d: \0" as *const u8 as *const libc::c_char,
                    (*w).number,
                );
                print_watch_item(w);
            }
            if stop.print_frame {
                print_frame((*frame_ptr).sub.nodep.x.extra, source, sourceline);
                fprintf(out_fp, b"\n\0" as *const u8 as *const libc::c_char);
                stop.print_frame = 0 as libc::c_int != 0;
            }
            print_lines(source, sourceline, 1 as libc::c_int);
            d = display_list.prev;
            while d != &mut display_list as *mut list_item {
                display(d);
                d = (*d).prev;
            }
        }
        _ => {}
    }
    last_printed_line = sourceline - list_size / 2 as libc::c_int;
    if last_printed_line < 0 as libc::c_int {
        last_printed_line = 0 as libc::c_int;
    }
    s = source_find(source);
    if cur_srcfile != s {
        if (*cur_srcfile).fd != -(1 as libc::c_int) {
            close((*cur_srcfile).fd);
            (*cur_srcfile).fd = -(1 as libc::c_int);
        }
        cur_srcfile = s;
    }
    stop.command = D_illegal;
    stop.check_func = None;
    if !b.is_null() {
        let mut ret: libc::c_int = 0;
        ret = execute_commands(&mut (*b).commands);
        if (*b).flags as libc::c_int & 4 as libc::c_int != 0 as libc::c_int {
            delete_breakpoint(b);
        }
        if ret != 0 {
            return;
        }
    } else if !w.is_null() && execute_commands(&mut (*w).commands) != 0 {
        return
    }
    zzparse();
}
unsafe extern "C" fn debug_post_execute(mut pc: *mut INSTRUCTION) {
    if in_main_context() == 0 {
        return;
    }
    match (*pc).opcode as libc::c_uint {
        59 | 67 | 60 => {
            if stop.command as libc::c_uint == D_finish as libc::c_int as libc::c_uint {
                stop.print_ret = 0 as libc::c_int != 0;
                stop.print_frame = 0 as libc::c_int != 0;
                stop.command = D_illegal;
                stop.check_func = None;
                fprintf(
                    out_fp,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"'finish' not meaningful with non-local jump '%s'\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    op2str((*pc).opcode),
                );
            } else if stop.command as libc::c_uint
                == D_until as libc::c_int as libc::c_uint
            {
                stop.print_frame = 0 as libc::c_int != 0;
                stop.command = D_illegal;
                stop.check_func = None;
                fprintf(
                    out_fp,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"'until' not meaningful with non-local jump '%s'\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    op2str((*pc).opcode),
                );
            }
        }
        61 => {
            if stop.command as libc::c_uint == D_finish as libc::c_int as libc::c_uint
                && fcall_count == stop.fcall_count && stop.print_ret as libc::c_int != 0
            {
                let mut r: *mut NODE = 0 as *mut NODE;
                r = (*stack_ptr).rptr;
                fprintf(
                    out_fp,
                    b"Returned value = \0" as *const u8 as *const libc::c_char,
                );
                valinfo(
                    r,
                    Some(
                        fprintf
                            as unsafe extern "C" fn(
                                *mut FILE,
                                *const libc::c_char,
                                ...
                            ) -> libc::c_int,
                    ),
                    out_fp,
                );
                stop.print_ret = 0 as libc::c_int != 0;
            }
        }
        91 | 90 => return,
        _ => {}
    };
}
unsafe extern "C" fn debug_pre_execute(mut pi: *mut *mut INSTRUCTION) -> libc::c_int {
    static mut cant_stop: bool = 0 as libc::c_int != 0;
    let mut m: *mut NODE = 0 as *mut NODE;
    if in_main_context() == 0 {
        return pre_execute_code(pi);
    }
    cur_pc = *pi;
    stop.break_point = 0 as libc::c_int;
    stop.watch_point = 0 as libc::c_int;
    cur_frame = 0 as libc::c_int as libc::c_long;
    if do_trace != 0
        && (*cur_pc).opcode as libc::c_uint
            != Op_breakpoint as libc::c_int as libc::c_uint
        && stop.command as libc::c_uint != D_return as libc::c_int as libc::c_uint
    {
        print_instruction(
            cur_pc,
            Some(
                fprintf
                    as unsafe extern "C" fn(
                        *mut FILE,
                        *const libc::c_char,
                        ...
                    ) -> libc::c_int,
            ),
            out_fp,
            0 as libc::c_int,
        );
    }
    match (*cur_pc).opcode as libc::c_uint {
        84 => {
            cant_stop = 1 as libc::c_int != 0;
        }
        97 => {
            cant_stop = 0 as libc::c_int != 0;
            return 1 as libc::c_int;
        }
        82 => {
            m = (*cur_pc).d.dn;
            if (*m).type_0 as libc::c_uint == Node_var as libc::c_int as libc::c_uint
                && ((*m).sub.nodep.x.aptr).is_some()
            {
                cant_stop = 1 as libc::c_int != 0;
            }
        }
        93 => {
            m = (*cur_pc).x.xn;
            if (*m).type_0 as libc::c_uint == Node_var as libc::c_int as libc::c_uint
                && ((*m).sub.nodep.x.aptr).is_some()
            {
                cant_stop = 1 as libc::c_int != 0;
            }
        }
        96 => {
            cant_stop = 0 as libc::c_int != 0;
            return 1 as libc::c_int;
        }
        51 => {
            cur_rule = (*cur_pc).x.xl as libc::c_int;
            return 1 as libc::c_int;
        }
        101 | 95 => return 1 as libc::c_int,
        104 => {}
        _ => {
            if (*cur_pc).source_line as libc::c_int <= 0 as libc::c_int {
                return 1 as libc::c_int;
            }
        }
    }
    if cant_stop {
        return 1 as libc::c_int;
    }
    if check_watchpoint() != 0 {
        next_command();
        if stop.command as libc::c_uint == D_return as libc::c_int as libc::c_uint {
            *pi = stop.pc;
        } else if (*cur_pc).opcode as libc::c_uint
            == Op_breakpoint as libc::c_int as libc::c_uint
        {
            cur_pc = (*cur_pc).nexti;
        }
    } else if check_breakpoint(pi) != 0
        || (stop.check_func).is_some() && (stop.check_func).unwrap()(pi) != 0
    {
        next_command();
        if stop.command as libc::c_uint == D_return as libc::c_int as libc::c_uint {
            *pi = stop.pc;
        }
    }
    return (cur_pc == *pi) as libc::c_int;
}
unsafe extern "C" fn print_memory(
    mut m: *mut NODE,
    mut func: *mut NODE,
    mut print_func: Func_print,
    mut fp: *mut FILE,
) {
    match (*m).type_0 as libc::c_uint {
        1 => {
            if m == Nnull_string {
                print_func
                    .unwrap()(fp, b"Nnull_string\0" as *const u8 as *const libc::c_char);
            } else if (*m).flags as libc::c_uint & NUMBER as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
            {
                print_func
                    .unwrap()(
                    fp,
                    b"%g\0" as *const u8 as *const libc::c_char,
                    (*m).sub.val.fltnum,
                );
            } else if (*m).flags as libc::c_uint & STRING as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
            {
                pp_string_fp(
                    print_func,
                    fp,
                    (*m).sub.val.sp,
                    (*m).sub.val.slen,
                    '"' as i32,
                    0 as libc::c_int != 0,
                );
            } else if (*m).flags as libc::c_uint & REGEX as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
            {
                print_func.unwrap()(fp, b"@\0" as *const u8 as *const libc::c_char);
                pp_string_fp(
                    print_func,
                    fp,
                    (*m).sub.val.sp,
                    (*m).sub.val.slen,
                    '/' as i32,
                    0 as libc::c_int != 0,
                );
            } else {
                print_func.unwrap()(fp, b"-?-\0" as *const u8 as *const libc::c_char);
            }
            print_func
                .unwrap()(
                fp,
                b" [%s]\0" as *const u8 as *const libc::c_char,
                flags2str((*m).flags as libc::c_int),
            );
        }
        2 => {
            pp_string_fp(
                print_func,
                fp,
                (*(*m).sub.nodep.x.extra).sub.val.sp,
                (*(*m).sub.nodep.x.extra).sub.val.slen,
                '/' as i32,
                0 as libc::c_int != 0,
            );
        }
        3 => {}
        8 => {
            print_func
                .unwrap()(
                fp,
                b"%s\0" as *const u8 as *const libc::c_char,
                (*((*func).sub.nodep.rn).offset((*m).sub.nodep.l.ll as isize))
                    .sub
                    .nodep
                    .name,
            );
        }
        4 | 6 | 5 => {
            print_func
                .unwrap()(
                fp,
                b"%s\0" as *const u8 as *const libc::c_char,
                (*m).sub.nodep.name,
            );
        }
        7 => {
            print_func
                .unwrap()(fp, b"element - %p\0" as *const u8 as *const libc::c_char, m);
        }
        _ => {
            print_func.unwrap()(fp, b"?\0" as *const u8 as *const libc::c_char);
        }
    };
}
unsafe extern "C" fn print_instruction(
    mut pc: *mut INSTRUCTION,
    mut print_func: Func_print,
    mut fp: *mut FILE,
    mut in_dump: libc::c_int,
) {
    let mut pcount: libc::c_int = 0 as libc::c_int;
    static mut func: *mut NODE = 0 as *const NODE as *mut NODE;
    static mut noffset: libc::c_int = 0 as libc::c_int;
    if noffset == 0 as libc::c_int {
        static mut buf: [libc::c_char; 50] = [0; 50];
        noffset = sprintf(
            buf.as_mut_ptr(),
            b"[      :%p] %-20.20s: \0" as *const u8 as *const libc::c_char,
            pc as *mut libc::c_void,
            opcode2str((*pc).opcode),
        );
    }
    if (*pc).opcode as libc::c_uint == Op_func as libc::c_int as libc::c_uint {
        func = (*pc).x.xn;
        pcount = (*func).sub.nodep.l.ll as libc::c_int;
        if in_dump != 0 {
            let mut j: libc::c_int = 0;
            print_func
                .unwrap()(
                fp,
                b"\n\t# Function: %s (\0" as *const u8 as *const libc::c_char,
                (*func).sub.nodep.name,
            );
            j = 0 as libc::c_int;
            while j < pcount {
                print_func
                    .unwrap()(
                    fp,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    (*((*func).sub.nodep.rn).offset(j as isize)).sub.nodep.name,
                );
                if j < pcount - 1 as libc::c_int {
                    print_func.unwrap()(fp, b", \0" as *const u8 as *const libc::c_char);
                }
                j += 1;
                j;
            }
            print_func.unwrap()(fp, b")\n\n\0" as *const u8 as *const libc::c_char);
        }
    } else if (*pc).opcode as libc::c_uint == Op_rule as libc::c_int as libc::c_uint {
        if in_dump != 0 {
            print_func
                .unwrap()(
                fp,
                b"\n\t# %s\n\n\0" as *const u8 as *const libc::c_char,
                *ruletab.as_ptr().offset((*pc).x.xl as isize),
            );
        }
    }
    if (*pc).opcode as libc::c_uint == Op_newfile as libc::c_int as libc::c_uint {
        print_func.unwrap()(fp, b"\n\0" as *const u8 as *const libc::c_char);
    }
    if (*pc).source_line as libc::c_int <= 0 as libc::c_int {
        print_func
            .unwrap()(
            fp,
            b"[      :%p] %-20.20s: \0" as *const u8 as *const libc::c_char,
            pc,
            opcode2str((*pc).opcode),
        );
    } else {
        print_func
            .unwrap()(
            fp,
            b"[%6d:%p] %-20.20s: \0" as *const u8 as *const libc::c_char,
            (*pc).source_line as libc::c_int,
            pc,
            opcode2str((*pc).opcode),
        );
    }
    if prog_running as libc::c_int != 0 && in_dump == 0 {
        func = (*find_frame(0 as libc::c_int as libc::c_long)).sub.nodep.x.extra;
    }
    let mut current_block_149: u64;
    match (*pc).opcode as libc::c_uint {
        117 => {
            print_func
                .unwrap()(
                fp,
                b"[branch_if = %p] [branch_else = %p] [branch_else->lasti = %p]\n\0"
                    as *const u8 as *const libc::c_char,
                (*pc).d.di,
                (*pc).x.xi,
                (*(*pc).x.xi).d.di,
            );
            current_block_149 = 12129449210080749085;
        }
        118 => {
            print_func
                .unwrap()(
                fp,
                b"[branch_end = %p]\n\0" as *const u8 as *const libc::c_char,
                (*pc).x.xi,
            );
            current_block_149 = 12129449210080749085;
        }
        115 => {
            print_func
                .unwrap()(
                fp,
                b"[while_body = %p] [target_break = %p]\n\0" as *const u8
                    as *const libc::c_char,
                (*pc.offset(1 as libc::c_int as isize)).d.di,
                (*pc).x.xi,
            );
            current_block_149 = 12129449210080749085;
        }
        112 => {
            print_func
                .unwrap()(
                fp,
                b"[doloop_cond = %p] [target_break = %p]\0" as *const u8
                    as *const libc::c_char,
                (*pc.offset(1 as libc::c_int as isize)).d.di,
                (*pc).x.xi,
            );
            if !((*pc).comment).is_null() {
                print_func
                    .unwrap()(
                    fp,
                    b" [comment = %p]\0" as *const u8 as *const libc::c_char,
                    (*pc).comment,
                );
            }
            print_func.unwrap()(fp, b"\n\0" as *const u8 as *const libc::c_char);
            if !((*pc).comment).is_null() {
                print_instruction((*pc).comment, print_func, fp, in_dump);
            }
            current_block_149 = 12129449210080749085;
        }
        113 => {
            print_func
                .unwrap()(
                fp,
                b"[forloop_cond = %p] \0" as *const u8 as *const libc::c_char,
                (*pc.offset(1 as libc::c_int as isize)).d.di,
            );
            current_block_149 = 6757826873918742154;
        }
        114 => {
            current_block_149 = 6757826873918742154;
        }
        116 => {
            let mut need_newline: bool = 0 as libc::c_int != 0;
            print_func
                .unwrap()(
                fp,
                b"[switch_start = %p] [switch_end = %p]\n\0" as *const u8
                    as *const libc::c_char,
                (*pc.offset(1 as libc::c_int as isize)).d.di,
                (*pc.offset(1 as libc::c_int as isize)).x.xi,
            );
            if !((*pc).comment).is_null()
                || !((*(*pc.offset(1 as libc::c_int as isize)).x.xi).comment).is_null()
            {
                print_func
                    .unwrap()(
                    fp,
                    b"%*s\0" as *const u8 as *const libc::c_char,
                    noffset,
                    b"\0" as *const u8 as *const libc::c_char,
                );
            }
            if !((*pc).comment).is_null() {
                print_func
                    .unwrap()(
                    fp,
                    b"[start_comment = %p]\0" as *const u8 as *const libc::c_char,
                    (*pc).comment,
                );
                need_newline = 1 as libc::c_int != 0;
            }
            if !((*(*pc.offset(1 as libc::c_int as isize)).x.xi).comment).is_null() {
                print_func
                    .unwrap()(
                    fp,
                    b"[end_comment = %p]\0" as *const u8 as *const libc::c_char,
                    (*(*pc.offset(1 as libc::c_int as isize)).x.xi).comment,
                );
                need_newline = 1 as libc::c_int != 0;
            }
            if need_newline {
                print_func.unwrap()(fp, b"\n\0" as *const u8 as *const libc::c_char);
            }
            if !((*pc).comment).is_null() {
                print_instruction((*pc).comment, print_func, fp, in_dump);
            }
            if !((*(*pc.offset(1 as libc::c_int as isize)).x.xi).comment).is_null() {
                print_instruction(
                    (*(*pc.offset(1 as libc::c_int as isize)).x.xi).comment,
                    print_func,
                    fp,
                    in_dump,
                );
            }
            current_block_149 = 12129449210080749085;
        }
        53 => {
            print_func
                .unwrap()(
                fp,
                b"[stmt_start = %p] [stmt_end = %p]\0" as *const u8
                    as *const libc::c_char,
                (*pc).d.di,
                (*pc).x.xi,
            );
            if !((*pc).comment).is_null() {
                print_func
                    .unwrap()(
                    fp,
                    b" [comment = %p]\n\0" as *const u8 as *const libc::c_char,
                    (*pc).comment,
                );
                print_instruction((*pc).comment, print_func, fp, in_dump);
            } else {
                print_func.unwrap()(fp, b"\n\0" as *const u8 as *const libc::c_char);
            }
            current_block_149 = 12129449210080749085;
        }
        95 => {
            print_func
                .unwrap()(
                fp,
                b"[update_%s()]\n\0" as *const u8 as *const libc::c_char,
                get_spec_varname((*pc).x.aptr),
            );
            current_block_149 = 12129449210080749085;
        }
        96 => {
            print_func
                .unwrap()(
                fp,
                b"[set_%s()]\0" as *const u8 as *const libc::c_char,
                get_spec_varname((*pc).x.aptr),
            );
            if (*pc).d.dl != 0 as libc::c_int as libc::c_long {
                print_func
                    .unwrap()(
                    fp,
                    b" [assign_ctxt = %s]\0" as *const u8 as *const libc::c_char,
                    opcode2str((*pc).d.dl as OPCODE),
                );
            }
            print_func.unwrap()(fp, b"\n\0" as *const u8 as *const libc::c_char);
            current_block_149 = 12129449210080749085;
        }
        97 => {
            print_func
                .unwrap()(
                fp,
                b"[%s]\n\0" as *const u8 as *const libc::c_char,
                if (*pc).x.aptr == Some(reset_record as unsafe extern "C" fn() -> ()) {
                    b"reset_record()\0" as *const u8 as *const libc::c_char
                } else {
                    b"invalidate_field0()\0" as *const u8 as *const libc::c_char
                },
            );
            current_block_149 = 12129449210080749085;
        }
        84 => {
            print_func
                .unwrap()(
                fp,
                b"[target_assign = %p] [do_reference = %s]\n\0" as *const u8
                    as *const libc::c_char,
                (*pc).d.di,
                if (*pc).x.xl != 0 {
                    b"true\0" as *const u8 as *const libc::c_char
                } else {
                    b"false\0" as *const u8 as *const libc::c_char
                },
            );
            current_block_149 = 12129449210080749085;
        }
        101 => {
            print_func
                .unwrap()(
                fp,
                b"[param_cnt = %d] [source_file = %s]\0" as *const u8
                    as *const libc::c_char,
                pcount,
                if !((*pc).d.name).is_null() {
                    (*pc).d.name as *const libc::c_char
                } else {
                    b"cmd. line\0" as *const u8 as *const libc::c_char
                },
            );
            if !((*pc.offset(3 as libc::c_int as isize)).nexti).is_null() {
                print_func
                    .unwrap()(
                    fp,
                    b"[ns_list = %p]\n\0" as *const u8 as *const libc::c_char,
                    (*pc.offset(3 as libc::c_int as isize)).nexti,
                );
                print_ns_list(
                    (*pc.offset(3 as libc::c_int as isize)).nexti,
                    print_func,
                    fp,
                    in_dump,
                );
            } else {
                print_func.unwrap()(fp, b"\n\0" as *const u8 as *const libc::c_char);
            }
            current_block_149 = 12129449210080749085;
        }
        65 => {
            print_func
                .unwrap()(
                fp,
                b"[into_var = %s] [redir_type = \"%s\"]\n\0" as *const u8
                    as *const libc::c_char,
                if (*pc).x.xl != 0 {
                    b"true\0" as *const u8 as *const libc::c_char
                } else {
                    b"false\0" as *const u8 as *const libc::c_char
                },
                redir2str((*pc).d.dl as libc::c_int),
            );
            current_block_149 = 12129449210080749085;
        }
        66 => {
            print_func
                .unwrap()(
                fp,
                b"[into_var = %s]\n\0" as *const u8 as *const libc::c_char,
                if (*pc).x.xl != 0 {
                    b"true\0" as *const u8 as *const libc::c_char
                } else {
                    b"false\0" as *const u8 as *const libc::c_char
                },
            );
            print_func
                .unwrap()(
                fp,
                b"%*s[target_beginfile = %p] [target_endfile = %p]\n\0" as *const u8
                    as *const libc::c_char,
                noffset,
                b"\0" as *const u8 as *const libc::c_char,
                (*pc.offset(1 as libc::c_int as isize)).d.di,
                (*pc.offset(1 as libc::c_int as isize)).x.xi,
            );
            current_block_149 = 12129449210080749085;
        }
        57 => {
            print_func
                .unwrap()(
                fp,
                b"[redir_type = \"%s\"]\n\0" as *const u8 as *const libc::c_char,
                redir2str((*pc).d.dl as libc::c_int),
            );
            current_block_149 = 12129449210080749085;
        }
        56 | 58 => {
            print_func
                .unwrap()(
                fp,
                b"[expr_count = %ld] [redir_type = \"%s\"]\n\0" as *const u8
                    as *const libc::c_char,
                (*pc).x.xl,
                redir2str((*pc).d.dl as libc::c_int),
            );
            current_block_149 = 12129449210080749085;
        }
        74 | 73 => {
            print_func
                .unwrap()(
                fp,
                b"[func_name = %s] [arg_count = %ld]\n\0" as *const u8
                    as *const libc::c_char,
                (*pc).d.name,
                (*pc.offset(1 as libc::c_int as isize)).x.xl,
            );
            current_block_149 = 12129449210080749085;
        }
        67 => {
            print_func
                .unwrap()(
                fp,
                b"[target_newfile = %p] [target_endfile = %p]\n\0" as *const u8
                    as *const libc::c_char,
                (*pc).d.di,
                (*pc).x.xi,
            );
            current_block_149 = 12129449210080749085;
        }
        91 => {
            print_func
                .unwrap()(
                fp,
                b"[target_jmp = %p] [target_endfile = %p]\n\0" as *const u8
                    as *const libc::c_char,
                (*pc).d.di,
                (*pc).x.xi,
            );
            print_func
                .unwrap()(
                fp,
                b"%*s[target_get_record = %p]\n\0" as *const u8 as *const libc::c_char,
                noffset,
                b"\0" as *const u8 as *const libc::c_char,
                (*pc.offset(1 as libc::c_int as isize)).x.xi,
            );
            current_block_149 = 12129449210080749085;
        }
        90 => {
            print_func
                .unwrap()(
                fp,
                b"[target_newfile = %p]\n\0" as *const u8 as *const libc::c_char,
                (*pc).d.di,
            );
            current_block_149 = 12129449210080749085;
        }
        87 | 89 | 88 | 38 | 40 | 59 | 92 | 54 | 55 => {
            print_func
                .unwrap()(
                fp,
                b"[target_jmp = %p]\n\0" as *const u8 as *const libc::c_char,
                (*pc).d.di,
            );
            current_block_149 = 12129449210080749085;
        }
        60 => {
            print_func
                .unwrap()(
                fp,
                b"[target_end = %p] [target_atexit = %p]\n\0" as *const u8
                    as *const libc::c_char,
                (*pc).d.di,
                (*pc).x.xi,
            );
            current_block_149 = 12129449210080749085;
        }
        52 => {
            print_func
                .unwrap()(
                fp,
                b"[target_jmp = %p] [match_exp = %s]\0" as *const u8
                    as *const libc::c_char,
                (*pc).d.di,
                if (*pc.offset(1 as libc::c_int as isize)).x.xl != 0 {
                    b"true\0" as *const u8 as *const libc::c_char
                } else {
                    b"false\0" as *const u8 as *const libc::c_char
                },
            );
            if !((*pc).comment).is_null() {
                print_func
                    .unwrap()(
                    fp,
                    b" [comment = %p]\n\0" as *const u8 as *const libc::c_char,
                    (*pc).comment,
                );
                print_instruction((*pc).comment, print_func, fp, in_dump);
            } else {
                print_func.unwrap()(fp, b"\n\0" as *const u8 as *const libc::c_char);
            }
            current_block_149 = 12129449210080749085;
        }
        68 => {
            print_func
                .unwrap()(
                fp,
                b"[namespace = %s]\0" as *const u8 as *const libc::c_char,
                (*pc).d.name,
            );
            if !((*pc).nexti).is_null() {
                print_func
                    .unwrap()(
                    fp,
                    b"[nexti = %p]\0" as *const u8 as *const libc::c_char,
                    (*pc).nexti,
                );
            }
            if !((*pc).comment).is_null() {
                print_func
                    .unwrap()(
                    fp,
                    b"[comment = %p]\0" as *const u8 as *const libc::c_char,
                    (*pc).comment,
                );
            }
            print_func.unwrap()(fp, b"\n\0" as *const u8 as *const libc::c_char);
            current_block_149 = 12129449210080749085;
        }
        93 => {
            print_func
                .unwrap()(
                fp,
                b"[array_var = %s] [target_jmp = %p]\n\0" as *const u8
                    as *const libc::c_char,
                if (*(*pc).x.xn).type_0 as libc::c_uint
                    == Node_param_list as libc::c_int as libc::c_uint
                {
                    (*((*func).sub.nodep.rn)
                        .offset((*(*pc).x.xn).sub.nodep.l.ll as isize))
                        .sub
                        .nodep
                        .name
                } else {
                    (*(*pc).x.xn).sub.nodep.name
                },
                (*pc).d.di,
            );
            current_block_149 = 12129449210080749085;
        }
        14 => {
            print_func
                .unwrap()(
                fp,
                b"[triggered = %ld] [target_jmp = %p]\n\0" as *const u8
                    as *const libc::c_char,
                (*pc).x.xl,
                (*pc).d.di,
            );
            current_block_149 = 12129449210080749085;
        }
        15 => {
            print_func
                .unwrap()(
                fp,
                b"[line_range = %p] [target_jmp = %p]\n\0" as *const u8
                    as *const libc::c_char,
                (*pc).x.xi,
                (*pc).d.di,
            );
            current_block_149 = 12129449210080749085;
        }
        70 => {
            let mut fname: *const libc::c_char = b"sub\0" as *const u8
                as *const libc::c_char;
            static mut values: [flagtab; 4] = [
                {
                    let mut init = flagtab {
                        val: 0x1 as libc::c_int,
                        name: b"GSUB\0" as *const u8 as *const libc::c_char,
                    };
                    init
                },
                {
                    let mut init = flagtab {
                        val: 0x2 as libc::c_int,
                        name: b"GENSUB\0" as *const u8 as *const libc::c_char,
                    };
                    init
                },
                {
                    let mut init = flagtab {
                        val: 0x4 as libc::c_int,
                        name: b"LITERAL\0" as *const u8 as *const libc::c_char,
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
            if (*pc).d.dl & 0x1 as libc::c_int as libc::c_long
                != 0 as libc::c_int as libc::c_long
            {
                fname = b"gsub\0" as *const u8 as *const libc::c_char;
            } else if (*pc).d.dl & 0x2 as libc::c_int as libc::c_long
                != 0 as libc::c_int as libc::c_long
            {
                fname = b"gensub\0" as *const u8 as *const libc::c_char;
            }
            print_func
                .unwrap()(
                fp,
                b"%s [arg_count = %ld] [sub_flags = %s]\n\0" as *const u8
                    as *const libc::c_char,
                fname,
                (*pc).x.xl,
                genflags2str((*pc).d.dl as libc::c_int, values.as_ptr()),
            );
            current_block_149 = 12129449210080749085;
        }
        69 => {
            print_func
                .unwrap()(
                fp,
                b"%s [arg_count = %ld]\n\0" as *const u8 as *const libc::c_char,
                getfname((*pc).d.fptr, 0 as libc::c_int != 0),
                (*pc).x.xl,
            );
            current_block_149 = 12129449210080749085;
        }
        71 => {
            print_func
                .unwrap()(
                fp,
                b"%s [arg_count = %ld]\n\0" as *const u8 as *const libc::c_char,
                (*pc.offset(1 as libc::c_int as isize)).d.name,
                (*pc).x.xl,
            );
            current_block_149 = 12129449210080749085;
        }
        16 | 17 => {
            print_func
                .unwrap()(
                fp,
                b"[sub_count = %ld]\n\0" as *const u8 as *const libc::c_char,
                (*pc).d.dl,
            );
            current_block_149 = 12129449210080749085;
        }
        28 => {
            print_memory((*pc).d.dn, func, print_func, fp);
            print_func
                .unwrap()(
                fp,
                b" [sub_count = %ld]\n\0" as *const u8 as *const libc::c_char,
                (*pc).x.xl,
            );
            current_block_149 = 12129449210080749085;
        }
        83 => {
            print_func
                .unwrap()(
                fp,
                b"[sub_count = %ld] [do_reference = %s]\n\0" as *const u8
                    as *const libc::c_char,
                (*pc).d.dl,
                if (*pc).x.xl != 0 {
                    b"true\0" as *const u8 as *const libc::c_char
                } else {
                    b"false\0" as *const u8 as *const libc::c_char
                },
            );
            current_block_149 = 12129449210080749085;
        }
        63 | 72 => {
            print_func
                .unwrap()(
                fp,
                b"[expr_count = %ld]\n\0" as *const u8 as *const libc::c_char,
                (*pc).x.xl,
            );
            current_block_149 = 12129449210080749085;
        }
        13 => {
            print_func
                .unwrap()(
                fp,
                b"[expr_count = %ld] [concat_flag = %s]\n\0" as *const u8
                    as *const libc::c_char,
                (*pc).x.xl,
                if (*pc).d.dl & 1 as libc::c_int as libc::c_long
                    != 0 as libc::c_int as libc::c_long
                {
                    b"CSUBSEP\0" as *const u8 as *const libc::c_char
                } else {
                    b"0\0" as *const u8 as *const libc::c_char
                },
            );
            current_block_149 = 12129449210080749085;
        }
        51 => {
            print_func
                .unwrap()(
                fp,
                b"[in_rule = %s] [source_file = %s]\0" as *const u8
                    as *const libc::c_char,
                *ruletab.as_ptr().offset((*pc).x.xl as isize),
                if !((*pc).d.name).is_null() {
                    (*pc).d.name as *const libc::c_char
                } else {
                    b"cmd. line\0" as *const u8 as *const libc::c_char
                },
            );
            if !((*pc.offset(3 as libc::c_int as isize)).nexti).is_null() {
                print_func
                    .unwrap()(
                    fp,
                    b"[ns_list = %p]\n\0" as *const u8 as *const libc::c_char,
                    (*pc.offset(3 as libc::c_int as isize)).nexti,
                );
                print_ns_list(
                    (*pc.offset(3 as libc::c_int as isize)).nexti,
                    print_func,
                    fp,
                    in_dump,
                );
            } else {
                print_func.unwrap()(fp, b"\n\0" as *const u8 as *const libc::c_char);
            }
            current_block_149 = 12129449210080749085;
        }
        105 => {
            static mut linttypetab: [*const libc::c_char; 3] = [
                b"LINT_illegal\0" as *const u8 as *const libc::c_char,
                b"LINT_assign_in_cond\0" as *const u8 as *const libc::c_char,
                b"LINT_no_effect\0" as *const u8 as *const libc::c_char,
            ];
            print_func
                .unwrap()(
                fp,
                b"[lint_type = %s]\n\0" as *const u8 as *const libc::c_char,
                linttypetab[(*pc).d.dl as usize],
            );
            current_block_149 = 12129449210080749085;
        }
        103 => {
            print_func
                .unwrap()(
                fp,
                b"[exec_count = %llu]\n\0" as *const u8 as *const libc::c_char,
                (*pc).d.ldl,
            );
            current_block_149 = 12129449210080749085;
        }
        27 => {
            print_memory((*pc).d.dn, func, print_func, fp);
            if !((*pc).x.xn).is_null() {
                print_func.unwrap()(fp, b" = \0" as *const u8 as *const libc::c_char);
                print_memory((*pc).x.xn, func, print_func, fp);
            }
            print_func.unwrap()(fp, b"\n\0" as *const u8 as *const libc::c_char);
            current_block_149 = 12129449210080749085;
        }
        82 => {
            print_memory((*pc).d.dn, func, print_func, fp);
            print_func
                .unwrap()(
                fp,
                b" [do_reference = %s]\n\0" as *const u8 as *const libc::c_char,
                if (*pc).x.xl != 0 {
                    b"true\0" as *const u8 as *const libc::c_char
                } else {
                    b"false\0" as *const u8 as *const libc::c_char
                },
            );
            current_block_149 = 12129449210080749085;
        }
        102 => {
            print_memory((*pc).d.dn, func, print_func, fp);
            print_func
                .unwrap()(
                fp,
                b" [comment_type = %s]\0" as *const u8 as *const libc::c_char,
                if (*(*pc).d.dn).sub.val.comtype as libc::c_uint
                    == EOL_COMMENT as libc::c_int as libc::c_uint
                {
                    b"EOL\0" as *const u8 as *const libc::c_char
                } else {
                    b"BLOCK\0" as *const u8 as *const libc::c_char
                },
            );
            if !((*pc).comment).is_null() {
                print_func
                    .unwrap()(
                    fp,
                    b" [comment = %p]\n\0" as *const u8 as *const libc::c_char,
                    (*pc).comment,
                );
                print_instruction((*pc).comment, print_func, fp, in_dump);
            } else {
                print_func.unwrap()(fp, b"\n\0" as *const u8 as *const libc::c_char);
            }
            current_block_149 = 12129449210080749085;
        }
        78 | 75 | 76 | 77 | 81 | 80 | 79 | 49 | 48 | 50 | 8 | 10 | 2 | 12 | 4 | 6
        | 37 => {
            print_memory((*pc).d.dn, func, print_func, fp);
            current_block_149 = 4354390081814099988;
        }
        _ => {
            current_block_149 = 4354390081814099988;
        }
    }
    match current_block_149 {
        6757826873918742154 => {
            print_func
                .unwrap()(
                fp,
                b"[forloop_body = %p] \0" as *const u8 as *const libc::c_char,
                (*pc.offset(1 as libc::c_int as isize)).x.xi,
            );
            print_func
                .unwrap()(
                fp,
                b"[target_break = %p] [target_continue = %p]\0" as *const u8
                    as *const libc::c_char,
                (*pc).x.xi,
                (*pc).d.di,
            );
            if !((*pc).comment).is_null() {
                print_func
                    .unwrap()(
                    fp,
                    b" [comment = %p]\n\0" as *const u8 as *const libc::c_char,
                    (*pc).comment,
                );
                print_instruction((*pc).comment, print_func, fp, in_dump);
            } else {
                print_func.unwrap()(fp, b"\n\0" as *const u8 as *const libc::c_char);
            }
        }
        4354390081814099988 => {
            print_func.unwrap()(fp, b"\n\0" as *const u8 as *const libc::c_char);
        }
        _ => {}
    };
}
pub unsafe extern "C" fn do_trace_instruction(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    if !arg.is_null()
        && (*arg).type_0 as libc::c_uint == D_argument as libc::c_int as libc::c_uint
        && (*arg).value.lval == A_TRACE_ON as libc::c_int as libc::c_long
    {
        do_trace = 1 as libc::c_int;
    } else {
        do_trace = 0 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn print_code(
    mut pc: *mut INSTRUCTION,
    mut x: *mut libc::c_void,
) -> libc::c_int {
    let mut data: *mut pf_data = x as *mut pf_data;
    while !pc.is_null() {
        print_instruction(
            pc,
            (*data).print_func,
            (*data).fp,
            (*data).defn as libc::c_int,
        );
        pc = (*pc).nexti;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn print_ns_list(
    mut pc: *mut INSTRUCTION,
    mut print_func: Func_print,
    mut fp: *mut FILE,
    mut in_dump: libc::c_int,
) {
    while !pc.is_null() {
        print_instruction(pc, print_func, fp, in_dump);
        if !((*pc).comment).is_null() {
            print_instruction((*pc).comment, print_func, fp, in_dump);
        }
        pc = (*pc).nexti;
    }
}
pub unsafe extern "C" fn do_dump_instructions(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut funcs: *mut *mut NODE = 0 as *mut *mut NODE;
    if !arg.is_null()
        && (*arg).type_0 as libc::c_uint == D_string as libc::c_int as libc::c_uint
    {
        fp = fopen((*arg).value.sval, b"w\0" as *const u8 as *const libc::c_char);
        if fp.is_null() {
            d_error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"could not open `%s' for writing: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*arg).value.sval,
                strerror(*__errno_location()),
            );
            return 0 as libc::c_int;
        }
        pf_data
            .print_func = Some(
            fprintf
                as unsafe extern "C" fn(
                    *mut FILE,
                    *const libc::c_char,
                    ...
                ) -> libc::c_int,
        );
        pf_data.fp = fp;
        pf_data.defn = 1 as libc::c_int != 0;
        print_code(code_block, &mut pf_data as *mut pf_data as *mut libc::c_void);
        funcs = function_list(1 as libc::c_int != 0);
        foreach_func(
            funcs,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut INSTRUCTION,
                        *mut libc::c_void,
                    ) -> libc::c_int,
                >,
                Option::<
                    unsafe extern "C" fn(
                        *mut INSTRUCTION,
                        *mut libc::c_void,
                    ) -> libc::c_int,
                >,
            >(
                Some(
                    print_code
                        as unsafe extern "C" fn(
                            *mut INSTRUCTION,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            ),
            &mut pf_data as *mut pf_data as *mut libc::c_void,
        );
        free(funcs as *mut libc::c_void);
        fclose(fp);
        return 0 as libc::c_int;
    }
    funcs = function_list(1 as libc::c_int != 0);
    initialize_pager(out_fp);
    if _setjmp(pager_quit_tag.as_mut_ptr()) == 0 as libc::c_int {
        pf_data
            .print_func = Some(
            gprintf
                as unsafe extern "C" fn(
                    *mut FILE,
                    *const libc::c_char,
                    ...
                ) -> libc::c_int,
        );
        pf_data.fp = out_fp;
        pf_data.defn = 1 as libc::c_int != 0;
        print_code(code_block, &mut pf_data as *mut pf_data as *mut libc::c_void);
        foreach_func(
            funcs,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut INSTRUCTION,
                        *mut libc::c_void,
                    ) -> libc::c_int,
                >,
                Option::<
                    unsafe extern "C" fn(
                        *mut INSTRUCTION,
                        *mut libc::c_void,
                    ) -> libc::c_int,
                >,
            >(
                Some(
                    print_code
                        as unsafe extern "C" fn(
                            *mut INSTRUCTION,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            ),
            &mut pf_data as *mut pf_data as *mut libc::c_void,
        );
    }
    free(funcs as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn do_save(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut hist_list: *mut *mut HIST_ENTRY = 0 as *mut *mut HIST_ENTRY;
    let mut i: libc::c_int = 0;
    fp = fopen((*arg).value.sval, b"w\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        d_error(
            dcgettext(
                0 as *const libc::c_char,
                b"could not open `%s' for writing: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*arg).value.sval,
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    hist_list = history_list();
    if !hist_list.is_null() && history_length > sess_history_base {
        i = sess_history_base;
        while !(*hist_list.offset(i as isize)).is_null() {
            let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
            line = (**hist_list.offset(i as isize)).line;
            if !(strlen(line) > 1 as libc::c_int as libc::c_ulong
                && strncmp(
                    line,
                    b"sa\0" as *const u8 as *const libc::c_char,
                    2 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int)
            {
                fprintf(fp, b"%s\n\0" as *const u8 as *const libc::c_char, line);
            }
            i += 1;
            i;
        }
    }
    fclose(fp);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn do_option(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    let mut opt: *const dbg_option = 0 as *const dbg_option;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    if arg.is_null() {
        opt = option_list.as_ptr();
        while !((*opt).name).is_null() {
            if !((*opt).str_val).is_null() {
                fprintf(
                    out_fp,
                    b"%s = \"%s\"\n\0" as *const u8 as *const libc::c_char,
                    (*opt).name,
                    *(*opt).str_val,
                );
            } else {
                fprintf(
                    out_fp,
                    b"%s = %d\n\0" as *const u8 as *const libc::c_char,
                    (*opt).name,
                    *(*opt).num_val,
                );
            }
            opt = opt.offset(1);
            opt;
        }
        return 0 as libc::c_int;
    }
    name = (*arg).value.sval;
    arg = (*arg).next;
    value = if !arg.is_null() { (*arg).value.sval } else { 0 as *mut libc::c_char };
    opt = option_list.as_ptr();
    while !((*opt).name).is_null() {
        if strcmp(name, (*opt).name) == 0 as libc::c_int {
            break;
        }
        opt = opt.offset(1);
        opt;
    }
    if ((*opt).name).is_null() {
        return 0 as libc::c_int;
    }
    if value.is_null() {
        if !((*opt).str_val).is_null() {
            fprintf(
                out_fp,
                b"%s = \"%s\"\n\0" as *const u8 as *const libc::c_char,
                (*opt).name,
                *(*opt).str_val,
            );
        } else {
            fprintf(
                out_fp,
                b"%s = %d\n\0" as *const u8 as *const libc::c_char,
                (*opt).name,
                *(*opt).num_val,
            );
        }
    } else {
        (Some(((*opt).assign).unwrap())).unwrap()(value);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn initialize_pager(mut fp: *mut FILE) {
    if os_isatty(fileno(fp)) == 0 || !input_from_tty || input_fd != 0 as libc::c_int {
        screen_width = 2147483647 as libc::c_int;
        screen_height = 2147483647 as libc::c_int;
    } else {
        rl_reset_terminal(0 as *const libc::c_char);
        rl_get_screen_size(&mut screen_height, &mut screen_width);
        if screen_height <= 1 as libc::c_int {
            screen_height = 2147483647 as libc::c_int;
        }
        if screen_width <= 1 as libc::c_int {
            screen_width = 2147483647 as libc::c_int;
        }
    }
    pager_lines_printed = 0 as libc::c_int;
}
unsafe extern "C" fn prompt_continue(mut fp: *mut FILE) {
    let mut quit_pager: bool = 0 as libc::c_int != 0;
    if os_isatty(fileno(fp)) != 0 && input_fd == 0 as libc::c_int {
        quit_pager = prompt_yes_no(
            dcgettext(
                0 as *const libc::c_char,
                b"\t------[Enter] to continue or [q] + [Enter] to quit------\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            'q' as i32 as libc::c_char,
            0 as libc::c_int,
            fp,
        ) != 0;
    }
    if quit_pager {
        longjmp(pager_quit_tag.as_mut_ptr(), 1 as libc::c_int);
    }
    pager_lines_printed = 0 as libc::c_int;
}
pub unsafe extern "C" fn gprintf(
    mut fp: *mut FILE,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut args_0: ::std::ffi::VaListImpl;
    static mut buf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut buflen: size_t = 0 as libc::c_int as size_t;
    static mut bl: libc::c_int = 0 as libc::c_int;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nchar: libc::c_int = 0;
    if buf.is_null() {
        buflen = 512 as libc::c_int as size_t;
        buf = emalloc_real(
            buflen.wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            b"gprintf\0" as *const u8 as *const libc::c_char,
            b"buf\0" as *const u8 as *const libc::c_char,
            b"debug.c\0" as *const u8 as *const libc::c_char,
            4364 as libc::c_int,
        ) as *mut libc::c_char;
    } else if buflen.wrapping_sub(bl as libc::c_ulong)
        < (512 as libc::c_int / 2 as libc::c_int) as libc::c_ulong
    {
        buflen = (buflen as libc::c_ulong)
            .wrapping_add(512 as libc::c_int as libc::c_ulong) as size_t as size_t;
        buf = erealloc_real(
            buf as *mut libc::c_void,
            buflen.wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            b"gprintf\0" as *const u8 as *const libc::c_char,
            b"buf\0" as *const u8 as *const libc::c_char,
            b"debug.c\0" as *const u8 as *const libc::c_char,
            4367 as libc::c_int,
        ) as *mut libc::c_char;
    }
    loop {
        args_0 = args.clone();
        nchar = vsnprintf(
            buf.offset(bl as isize),
            buflen.wrapping_sub(bl as libc::c_ulong),
            format,
            args_0.as_va_list(),
        );
        if nchar == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        if nchar > 0 as libc::c_int
            && (nchar as libc::c_ulong) < buflen.wrapping_sub(bl as libc::c_ulong)
        {
            bl += nchar;
            if *buf.offset((bl - 1 as libc::c_int) as isize) as libc::c_int
                != '\n' as i32
            {
                return nchar;
            }
            break;
        } else {
            buflen = (buflen as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
            buf = erealloc_real(
                buf as *mut libc::c_void,
                buflen
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    ),
                b"gprintf\0" as *const u8 as *const libc::c_char,
                b"buf\0" as *const u8 as *const libc::c_char,
                b"debug.c\0" as *const u8 as *const libc::c_char,
                4386 as libc::c_int,
            ) as *mut libc::c_char;
        }
    }
    bl = 0 as libc::c_int;
    p = buf;
    loop {
        q = strchr(p, '\n' as i32);
        if q.is_null() {
            break;
        }
        let mut sz: libc::c_int = q.offset_from(p) as libc::c_long as libc::c_int;
        while sz > 0 as libc::c_int {
            let mut cnt: libc::c_int = 0;
            cnt = if sz > screen_width { screen_width } else { sz };
            if cnt < sz && pager_lines_printed == screen_height - 2 as libc::c_int {
                prompt_continue(fp);
            }
            if (if 0 != 0 && 0 != 0
                && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_mul(cnt as size_t) <= 8 as libc::c_int as libc::c_ulong
                && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    != 0 as libc::c_int as libc::c_ulong
            {
                {
                    let mut __ptr: *const libc::c_char = p as *const libc::c_char;
                    let mut __stream: *mut FILE = fp;
                    let mut __cnt: size_t = 0;
                    __cnt = (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                        .wrapping_mul(cnt as size_t);
                    while __cnt > 0 as libc::c_int as libc::c_ulong {
                        let fresh10 = __ptr;
                        __ptr = __ptr.offset(1);
                        if putc_unlocked(*fresh10 as libc::c_int, __stream)
                            == -(1 as libc::c_int)
                        {
                            break;
                        }
                        __cnt = __cnt.wrapping_sub(1);
                        __cnt;
                    }
                    (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                        .wrapping_mul(cnt as size_t)
                        .wrapping_sub(__cnt)
                        .wrapping_div(
                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        )
                }
            } else {
                if 0 != 0
                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                        == 0 as libc::c_int as libc::c_ulong
                    || 0 != 0 && cnt as size_t == 0 as libc::c_int as libc::c_ulong
                {
                    0 as libc::c_int as size_t
                } else {
                    fwrite_unlocked(
                        p as *const libc::c_void,
                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        cnt as size_t,
                        fp,
                    )
                }
            }) != cnt as libc::c_ulong
            {
                return -(1 as libc::c_int);
            }
            if cnt == sz {
                break;
            }
            pager_lines_printed += 1;
            if pager_lines_printed == screen_height - 1 as libc::c_int {
                prompt_continue(fp);
            }
            sz -= screen_width;
            p = p.offset(cnt as isize);
        }
        fprintf(fp, b"\n\0" as *const u8 as *const libc::c_char);
        pager_lines_printed += 1;
        if pager_lines_printed == screen_height - 1 as libc::c_int {
            prompt_continue(fp);
        }
        p = p.offset(1);
        p;
        p = q.offset(1 as libc::c_int as isize);
    }
    return nchar;
}
unsafe extern "C" fn serialize_subscript(
    mut buf: *mut libc::c_char,
    mut buflen: libc::c_int,
    mut item: *mut list_item,
) -> libc::c_int {
    let mut bl: libc::c_int = 0;
    let mut nchar: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut sub: *mut NODE = 0 as *mut NODE;
    nchar = snprintf(
        buf,
        buflen as libc::c_ulong,
        b"%d%c%d%c%s%c%d%c\0" as *const u8 as *const libc::c_char,
        (*item).number,
        '\u{1f}' as i32 as libc::c_char as libc::c_int,
        D_subscript as libc::c_int,
        '\u{1f}' as i32 as libc::c_char as libc::c_int,
        (*item).sname,
        '\u{1f}' as i32 as libc::c_char as libc::c_int,
        (*item).num_subs,
        '\u{1f}' as i32 as libc::c_char as libc::c_int,
    );
    if nchar <= 0 as libc::c_int {
        return 0 as libc::c_int
    } else if nchar >= buflen {
        return nchar
    }
    bl = nchar;
    i = 0 as libc::c_int;
    while i < (*item).num_subs {
        sub = *((*item).subs).offset(i as isize);
        nchar = snprintf(
            buf.offset(bl as isize),
            (buflen - bl) as libc::c_ulong,
            b"%lu%c%.*s%c\0" as *const u8 as *const libc::c_char,
            (*sub).sub.val.slen,
            '\u{1f}' as i32 as libc::c_char as libc::c_int,
            (*sub).sub.val.slen as libc::c_int,
            (*sub).sub.val.sp,
            '\u{1f}' as i32 as libc::c_char as libc::c_int,
        );
        if nchar <= 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        bl += nchar;
        if bl >= buflen {
            return bl;
        }
        i += 1;
        i;
    }
    return bl;
}
unsafe extern "C" fn serialize_list(mut type_0: libc::c_int) {
    let mut current_block: u64;
    static mut buf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut buflen: libc::c_int = 0 as libc::c_int;
    let mut bl: libc::c_int = 0;
    let mut b: *mut BREAKPOINT = 0 as *mut BREAKPOINT;
    let mut wd: *mut list_item = 0 as *mut list_item;
    let mut hist_list: *mut *mut HIST_ENTRY = 0 as *mut *mut HIST_ENTRY;
    let mut hist_index: libc::c_int = 0 as libc::c_int;
    let mut opt: *mut dbg_option = 0 as *mut dbg_option;
    let mut commands: *mut commands_item = 0 as *mut commands_item;
    let mut c: *mut commands_item = 0 as *mut commands_item;
    let mut cnum: libc::c_int = 0 as libc::c_int;
    let mut cndn: *mut condition = 0 as *mut condition;
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut end_ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut h: *mut HIST_ENTRY = 0 as *mut HIST_ENTRY;
    match type_0 {
        1 => {
            end_ptr = &mut breakpoints as *mut BREAKPOINT as *mut libc::c_void;
            ptr = breakpoints.prev as *mut libc::c_void;
        }
        2 => {
            end_ptr = &mut watch_list as *mut list_item as *mut libc::c_void;
            ptr = watch_list.prev as *mut libc::c_void;
        }
        3 => {
            end_ptr = &mut display_list as *mut list_item as *mut libc::c_void;
            ptr = display_list.prev as *mut libc::c_void;
        }
        4 => {
            hist_list = history_list();
            if hist_list.is_null() {
                return;
            }
            end_ptr = 0 as *mut libc::c_void;
            ptr = *hist_list.offset(0 as libc::c_int as isize) as *mut libc::c_void;
        }
        5 => {
            let mut n: libc::c_int = 0;
            n = (::std::mem::size_of::<[dbg_option; 8]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<dbg_option>() as libc::c_ulong)
                as libc::c_int;
            end_ptr = &*option_list.as_ptr().offset((n - 1 as libc::c_int) as isize)
                as *const dbg_option as *mut libc::c_void;
            ptr = option_list.as_ptr() as *mut libc::c_void;
        }
        _ => return,
    }
    if type_0 != HISTORY as libc::c_int && ptr == end_ptr {
        return;
    }
    if buf.is_null() {
        buflen = 512 as libc::c_int;
        buf = emalloc_real(
            (buflen + 1 as libc::c_int) as size_t,
            b"serialize\0" as *const u8 as *const libc::c_char,
            b"buf\0" as *const u8 as *const libc::c_char,
            b"debug.c\0" as *const u8 as *const libc::c_char,
            4516 as libc::c_int,
        ) as *mut libc::c_char;
    }
    bl = 0 as libc::c_int;
    while ptr != end_ptr {
        let mut nchar: libc::c_int = 0 as libc::c_int;
        if buflen - bl < 512 as libc::c_int / 2 as libc::c_int {
            current_block = 2402620514876481339;
        } else {
            current_block = 7205609094909031804;
        }
        loop {
            match current_block {
                2402620514876481339 => {
                    buflen *= 2 as libc::c_int;
                    buf = erealloc_real(
                        buf as *mut libc::c_void,
                        (buflen + 1 as libc::c_int) as size_t,
                        b"serialize\0" as *const u8 as *const libc::c_char,
                        b"buf\0" as *const u8 as *const libc::c_char,
                        b"debug.c\0" as *const u8 as *const libc::c_char,
                        4525 as libc::c_int,
                    ) as *mut libc::c_char;
                    current_block = 7205609094909031804;
                }
                _ => {
                    match type_0 {
                        1 => {
                            b = ptr as *mut BREAKPOINT;
                            nchar = snprintf(
                                buf.offset(bl as isize),
                                (buflen - bl) as libc::c_ulong,
                                b"%s%c%d%c%d%c%d%c%d%c%d%c\0" as *const u8
                                    as *const libc::c_char,
                                (*b).src,
                                '\u{1f}' as i32 as libc::c_char as libc::c_int,
                                (*(*b).bpi).source_line as libc::c_int,
                                '\u{1f}' as i32 as libc::c_char as libc::c_int,
                                (*b).flags as libc::c_int,
                                '\u{1f}' as i32 as libc::c_char as libc::c_int,
                                (*b).ignore_count as libc::c_int,
                                '\u{1f}' as i32 as libc::c_char as libc::c_int,
                                (*b).hit_count as libc::c_int,
                                '\u{1f}' as i32 as libc::c_char as libc::c_int,
                                (*b).number,
                                '\u{1f}' as i32 as libc::c_char as libc::c_int,
                            );
                            cnum = (*b).number;
                            commands = &mut (*b).commands;
                            cndn = &mut (*b).cndn;
                        }
                        3 | 2 => {
                            wd = ptr as *mut list_item;
                            if (*wd).flags & 1 as libc::c_int != 0 as libc::c_int {
                                nchar = 0 as libc::c_int;
                            } else if (*wd).flags & 2 as libc::c_int != 0 as libc::c_int
                            {
                                nchar = serialize_subscript(
                                    buf.offset(bl as isize),
                                    buflen - bl,
                                    wd,
                                );
                            } else if (*wd).flags & 4 as libc::c_int != 0 as libc::c_int
                            {
                                nchar = snprintf(
                                    buf.offset(bl as isize),
                                    (buflen - bl) as libc::c_ulong,
                                    b"%d%c%d%c%d%c\0" as *const u8 as *const libc::c_char,
                                    (*wd).number,
                                    '\u{1f}' as i32 as libc::c_char as libc::c_int,
                                    D_field as libc::c_int,
                                    '\u{1f}' as i32 as libc::c_char as libc::c_int,
                                    (*(*wd).symbol).sub.val.fltnum as libc::c_long
                                        as libc::c_int,
                                    '\u{1f}' as i32 as libc::c_char as libc::c_int,
                                );
                            } else {
                                nchar = snprintf(
                                    buf.offset(bl as isize),
                                    (buflen - bl) as libc::c_ulong,
                                    b"%d%c%d%c%s%c\0" as *const u8 as *const libc::c_char,
                                    (*wd).number,
                                    '\u{1f}' as i32 as libc::c_char as libc::c_int,
                                    D_variable as libc::c_int,
                                    '\u{1f}' as i32 as libc::c_char as libc::c_int,
                                    (*wd).sname,
                                    '\u{1f}' as i32 as libc::c_char as libc::c_int,
                                );
                            }
                            cnum = (*wd).number;
                            commands = &mut (*wd).commands;
                            cndn = &mut (*wd).cndn;
                        }
                        4 => {
                            h = ptr as *mut HIST_ENTRY;
                            nchar = strlen((*h).line) as libc::c_int;
                            if nchar >= buflen - bl {
                                current_block = 2402620514876481339;
                                continue;
                            }
                            strcpy(buf.offset(bl as isize), (*h).line);
                        }
                        5 => {
                            opt = ptr as *mut dbg_option;
                            if !((*opt).num_val).is_null() {
                                nchar = snprintf(
                                    buf.offset(bl as isize),
                                    (buflen - bl) as libc::c_ulong,
                                    b"%s%c%d%c\0" as *const u8 as *const libc::c_char,
                                    (*opt).name,
                                    '\u{1f}' as i32 as libc::c_char as libc::c_int,
                                    *(*opt).num_val,
                                    '\u{1f}' as i32 as libc::c_char as libc::c_int,
                                );
                            } else {
                                nchar = snprintf(
                                    buf.offset(bl as isize),
                                    (buflen - bl) as libc::c_ulong,
                                    b"%s%c%s%c\0" as *const u8 as *const libc::c_char,
                                    (*opt).name,
                                    '\u{1f}' as i32 as libc::c_char as libc::c_int,
                                    *(*opt).str_val,
                                    '\u{1f}' as i32 as libc::c_char as libc::c_int,
                                );
                            }
                        }
                        _ => {}
                    }
                    if nchar == 0 as libc::c_int {
                        break;
                    }
                    if !(nchar > 0 as libc::c_int && nchar < buflen - bl) {
                        current_block = 2402620514876481339;
                        continue;
                    }
                    bl += nchar;
                    *buf.offset(bl as isize) = '\u{1e}' as i32 as libc::c_char;
                    bl += 1;
                    *buf.offset(bl as isize) = '\0' as i32 as libc::c_char;
                    break;
                }
            }
        }
        match type_0 {
            1 | 2 => {
                bl -= 1;
                bl;
                nchar = 0 as libc::c_int;
                c = (*commands).next;
                while c != commands {
                    nchar = (nchar as libc::c_ulong)
                        .wrapping_add(
                            (strlen((*c).cmd_string))
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    if (*c).cmd == D_eval as libc::c_int {
                        let mut a: *mut CMDARG = (*c).arg;
                        nchar = (nchar as libc::c_ulong)
                            .wrapping_add(
                                (strlen((*a).value.sval))
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                            ) as libc::c_int as libc::c_int;
                        nchar = (nchar as libc::c_ulong)
                            .wrapping_add(
                                (strlen(b"end\0" as *const u8 as *const libc::c_char))
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                            ) as libc::c_int as libc::c_int;
                    }
                    c = (*c).next;
                }
                if nchar > 0 as libc::c_int {
                    nchar = (nchar as libc::c_ulong)
                        .wrapping_add(
                            (strlen(b"commands \0" as *const u8 as *const libc::c_char))
                                .wrapping_add(20 as libc::c_int as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(
                                    strlen(b"end\0" as *const u8 as *const libc::c_char),
                                )
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    if nchar >= buflen - bl {
                        buflen = bl + nchar + 1 as libc::c_int;
                        buf = erealloc_real(
                            buf as *mut libc::c_void,
                            (buflen + 1 as libc::c_int) as size_t,
                            b"serialize_list\0" as *const u8 as *const libc::c_char,
                            b"buf\0" as *const u8 as *const libc::c_char,
                            b"debug.c\0" as *const u8 as *const libc::c_char,
                            4629 as libc::c_int,
                        ) as *mut libc::c_char;
                    }
                    nchar = sprintf(
                        buf.offset(bl as isize),
                        b"commands %d\0" as *const u8 as *const libc::c_char,
                        cnum,
                    );
                    bl += nchar;
                    let fresh11 = bl;
                    bl = bl + 1;
                    *buf.offset(fresh11 as isize) = '\u{1d}' as i32 as libc::c_char;
                    c = (*commands).next;
                    while c != commands {
                        nchar = strlen((*c).cmd_string) as libc::c_int;
                        memcpy(
                            buf.offset(bl as isize) as *mut libc::c_void,
                            (*c).cmd_string as *const libc::c_void,
                            nchar as libc::c_ulong,
                        );
                        bl += nchar;
                        let fresh12 = bl;
                        bl = bl + 1;
                        *buf.offset(fresh12 as isize) = '\u{1d}' as i32 as libc::c_char;
                        if (*c).cmd == D_eval as libc::c_int {
                            let mut a_0: *mut CMDARG = (*c).arg;
                            nchar = strlen((*a_0).value.sval) as libc::c_int;
                            memcpy(
                                buf.offset(bl as isize) as *mut libc::c_void,
                                (*a_0).value.sval as *const libc::c_void,
                                nchar as libc::c_ulong,
                            );
                            bl += nchar;
                            let fresh13 = bl;
                            bl = bl + 1;
                            *buf
                                .offset(fresh13 as isize) = '\u{1d}' as i32 as libc::c_char;
                            nchar = strlen(b"end\0" as *const u8 as *const libc::c_char)
                                as libc::c_int;
                            memcpy(
                                buf.offset(bl as isize) as *mut libc::c_void,
                                b"end\0" as *const u8 as *const libc::c_char
                                    as *const libc::c_void,
                                nchar as libc::c_ulong,
                            );
                            bl += nchar;
                            let fresh14 = bl;
                            bl = bl + 1;
                            *buf
                                .offset(fresh14 as isize) = '\u{1d}' as i32 as libc::c_char;
                        }
                        c = (*c).next;
                    }
                    nchar = strlen(b"end\0" as *const u8 as *const libc::c_char)
                        as libc::c_int;
                    memcpy(
                        buf.offset(bl as isize) as *mut libc::c_void,
                        b"end\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        nchar as libc::c_ulong,
                    );
                    bl += nchar;
                    let fresh15 = bl;
                    bl = bl + 1;
                    *buf.offset(fresh15 as isize) = '\u{1f}' as i32 as libc::c_char;
                }
                let fresh16 = bl;
                bl = bl + 1;
                *buf.offset(fresh16 as isize) = '\u{1e}' as i32 as libc::c_char;
                *buf.offset(bl as isize) = '\0' as i32 as libc::c_char;
                if !((*cndn).expr).is_null() {
                    bl -= 1;
                    bl;
                    nchar = strlen((*cndn).expr) as libc::c_int;
                    if nchar + 1 as libc::c_int >= buflen - bl {
                        buflen = bl + nchar + 1 as libc::c_int + 1 as libc::c_int;
                        buf = erealloc_real(
                            buf as *mut libc::c_void,
                            (buflen + 1 as libc::c_int) as size_t,
                            b"serialize_list\0" as *const u8 as *const libc::c_char,
                            b"buf\0" as *const u8 as *const libc::c_char,
                            b"debug.c\0" as *const u8 as *const libc::c_char,
                            4666 as libc::c_int,
                        ) as *mut libc::c_char;
                    }
                    memcpy(
                        buf.offset(bl as isize) as *mut libc::c_void,
                        (*cndn).expr as *const libc::c_void,
                        nchar as libc::c_ulong,
                    );
                    bl += nchar;
                    let fresh17 = bl;
                    bl = bl + 1;
                    *buf.offset(fresh17 as isize) = '\u{1f}' as i32 as libc::c_char;
                    let fresh18 = bl;
                    bl = bl + 1;
                    *buf.offset(fresh18 as isize) = '\u{1e}' as i32 as libc::c_char;
                    *buf.offset(bl as isize) = '\0' as i32 as libc::c_char;
                }
                ptr = if type_0 == BREAK as libc::c_int {
                    (*b).prev as *mut libc::c_void
                } else {
                    (*wd).prev as *mut libc::c_void
                };
            }
            3 => {
                ptr = (*wd).prev as *mut libc::c_void;
            }
            4 => {
                hist_index += 1;
                ptr = *hist_list.offset(hist_index as isize) as *mut libc::c_void;
            }
            5 => {
                opt = opt.offset(1);
                ptr = opt as *mut libc::c_void;
            }
            _ => {}
        }
    }
    if bl > 0 as libc::c_int {
        setenv(env_variable[type_0 as usize], buf, 1 as libc::c_int);
    }
}
unsafe extern "C" fn unserialize_commands(
    mut str: *mut libc::c_char,
    mut str_len: libc::c_int,
) {
    if str_len <= 0 as libc::c_int || str.is_null() {
        return;
    }
    commands_string = str;
    commands_string_len = str_len;
    push_cmd_src(
        -(1 as libc::c_int),
        0 as libc::c_int != 0,
        Some(
            read_commands_string
                as unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_char,
        ),
        None,
        0 as libc::c_int,
        2 as libc::c_int,
    );
    line_sep = '\u{1d}' as i32 as libc::c_char;
    zzparse();
    pop_cmd_src();
}
unsafe extern "C" fn unserialize_list_item(
    mut list: *mut list_item,
    mut pstr: *mut *mut libc::c_char,
    mut pstr_len: *mut libc::c_int,
    mut field_cnt: libc::c_int,
) -> *mut list_item {
    let mut num: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut l: *mut list_item = 0 as *mut list_item;
    let mut symbol: *mut NODE = 0 as *mut NODE;
    let mut sub_cnt: libc::c_int = 0 as libc::c_int;
    let mut cnt: libc::c_int = 0;
    let mut subs: *mut *mut NODE = 0 as *mut *mut NODE;
    num = strtol(
        *pstr.offset(0 as libc::c_int as isize),
        0 as *mut *mut libc::c_char,
        0 as libc::c_int,
    ) as libc::c_int;
    type_0 = strtol(
        *pstr.offset(1 as libc::c_int as isize),
        0 as *mut *mut libc::c_char,
        0 as libc::c_int,
    ) as libc::c_int;
    if type_0 == D_field as libc::c_int {
        let mut field_num: libc::c_int = 0;
        field_num = strtol(
            *pstr.offset(2 as libc::c_int as isize),
            0 as *mut *mut libc::c_char,
            0 as libc::c_int,
        ) as libc::c_int;
        symbol = make_number.unwrap()(field_num as libc::c_double);
        cnt = 3 as libc::c_int;
    } else {
        let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
        name = estrdup(
            *pstr.offset(2 as libc::c_int as isize),
            *pstr_len.offset(2 as libc::c_int as isize) as size_t,
        );
        symbol = find_symbol(name, 0 as *mut *mut libc::c_char);
        free(name as *mut libc::c_void);
        if symbol.is_null() {
            return 0 as *mut list_item;
        }
        cnt = 3 as libc::c_int;
        if type_0 == D_subscript as libc::c_int {
            let mut sub_len: libc::c_int = 0;
            sub_cnt = strtol(
                *pstr.offset(3 as libc::c_int as isize),
                0 as *mut *mut libc::c_char,
                0 as libc::c_int,
            ) as libc::c_int;
            subs = emalloc_real(
                (sub_cnt as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<*mut NODE>() as libc::c_ulong),
                b"unserialize_list_item\0" as *const u8 as *const libc::c_char,
                b"subs\0" as *const u8 as *const libc::c_char,
                b"debug.c\0" as *const u8 as *const libc::c_char,
                4745 as libc::c_int,
            ) as *mut *mut NODE;
            cnt += 1;
            cnt;
            i = 0 as libc::c_int;
            while i < sub_cnt {
                sub_len = strtol(
                    *pstr.offset(cnt as isize),
                    0 as *mut *mut libc::c_char,
                    0 as libc::c_int,
                ) as libc::c_int;
                let ref mut fresh19 = *subs.offset(i as isize);
                *fresh19 = make_str_node(
                    *pstr.offset((cnt + 1 as libc::c_int) as isize),
                    sub_len as size_t,
                    0 as libc::c_int,
                );
                cnt += 2 as libc::c_int;
                i += 1;
                i;
            }
        }
    }
    l = add_item(list, type_0, symbol, 0 as *mut libc::c_char);
    if type_0 == D_subscript as libc::c_int {
        (*l).num_subs = sub_cnt;
        (*l).subs = subs;
    }
    (*l).number = num;
    if list == &mut watch_list as *mut list_item {
        initialize_watch_item(l);
        unserialize_commands(*pstr.offset(cnt as isize), *pstr_len.offset(cnt as isize));
        cnt += 1;
        cnt;
        if field_cnt > cnt {
            let mut expr: *mut libc::c_char = 0 as *mut libc::c_char;
            expr = estrdup(
                *pstr.offset(cnt as isize),
                *pstr_len.offset(cnt as isize) as size_t,
            );
            if parse_condition(D_watch as libc::c_int, (*l).number, expr)
                != 0 as libc::c_int
            {
                free(expr as *mut libc::c_void);
            }
        }
        if num > (*list).number {
            (*list).number = num;
        }
    } else {
        (*list).number = num;
    }
    return l;
}
unsafe extern "C" fn unserialize_breakpoint(
    mut pstr: *mut *mut libc::c_char,
    mut pstr_len: *mut libc::c_int,
    mut field_cnt: libc::c_int,
) -> *mut BREAKPOINT {
    let mut src: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lineno: libc::c_int = 0;
    let mut b: *mut BREAKPOINT = 0 as *mut BREAKPOINT;
    let mut rp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut s: *mut SRCFILE = 0 as *mut SRCFILE;
    src = estrdup(
        *pstr.offset(0 as libc::c_int as isize),
        *pstr_len.offset(0 as libc::c_int as isize) as size_t,
    );
    s = source_find(src);
    free(src as *mut libc::c_void);
    if s.is_null() {
        return 0 as *mut BREAKPOINT;
    }
    src = (*s).src;
    lineno = strtol(
        *pstr.offset(1 as libc::c_int as isize),
        0 as *mut *mut libc::c_char,
        0 as libc::c_int,
    ) as libc::c_int;
    if lineno <= 0 as libc::c_int || lineno > (*s).srclines {
        return 0 as *mut BREAKPOINT;
    }
    rp = find_rule(src, lineno as libc::c_long);
    if rp.is_null()
        || {
            b = set_breakpoint_at(rp, lineno, 1 as libc::c_int != 0);
            b.is_null()
        }
    {
        return 0 as *mut BREAKPOINT;
    }
    (*b)
        .flags = strtol(
        *pstr.offset(2 as libc::c_int as isize),
        0 as *mut *mut libc::c_char,
        0 as libc::c_int,
    ) as libc::c_short;
    (*b)
        .ignore_count = strtol(
        *pstr.offset(3 as libc::c_int as isize),
        0 as *mut *mut libc::c_char,
        0 as libc::c_int,
    );
    (*b)
        .hit_count = strtol(
        *pstr.offset(4 as libc::c_int as isize),
        0 as *mut *mut libc::c_char,
        0 as libc::c_int,
    );
    (*b)
        .number = strtol(
        *pstr.offset(5 as libc::c_int as isize),
        0 as *mut *mut libc::c_char,
        0 as libc::c_int,
    ) as libc::c_int;
    if field_cnt > 6 as libc::c_int {
        unserialize_commands(
            *pstr.offset(6 as libc::c_int as isize),
            *pstr_len.offset(6 as libc::c_int as isize),
        );
    }
    if field_cnt > 7 as libc::c_int {
        let mut expr: *mut libc::c_char = 0 as *mut libc::c_char;
        expr = estrdup(
            *pstr.offset(7 as libc::c_int as isize),
            *pstr_len.offset(7 as libc::c_int as isize) as size_t,
        );
        if parse_condition(D_break as libc::c_int, (*b).number, expr) != 0 as libc::c_int
        {
            free(expr as *mut libc::c_void);
        }
    }
    if (*b).number > watch_list.number {
        watch_list.number = (*b).number;
    }
    return b;
}
unsafe extern "C" fn unserialize_option(
    mut pstr: *mut *mut libc::c_char,
    mut pstr_len: *mut libc::c_int,
    mut field_cnt: libc::c_int,
) -> *mut dbg_option {
    let mut opt: *const dbg_option = 0 as *const dbg_option;
    opt = option_list.as_ptr();
    while !((*opt).name).is_null() {
        if strncmp(
            *pstr.offset(0 as libc::c_int as isize),
            (*opt).name,
            *pstr_len.offset(0 as libc::c_int as isize) as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
            value = estrdup(
                *pstr.offset(1 as libc::c_int as isize),
                *pstr_len.offset(1 as libc::c_int as isize) as size_t,
            );
            (Some(((*opt).assign).unwrap())).unwrap()(value);
            free(value as *mut libc::c_void);
            return opt as *mut dbg_option;
        }
        opt = opt.offset(1);
        opt;
    }
    return 0 as *mut dbg_option;
}
unsafe extern "C" fn unserialize_list(mut type_0: libc::c_int) {
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut pstr: [*mut libc::c_char; 30] = [0 as *const libc::c_char
        as *mut libc::c_char; 30];
    static mut pstr_len: [libc::c_int; 30] = [0; 30];
    val = getenv(env_variable[type_0 as usize]);
    if val.is_null() {
        return;
    }
    p = val;
    loop {
        q = strchr(p, '\u{1e}' as i32 as libc::c_char as libc::c_int);
        if q.is_null() {
            break;
        }
        let mut field_cnt: libc::c_int = 0 as libc::c_int;
        if type_0 == HISTORY as libc::c_int {
            *q = '\0' as i32 as libc::c_char;
            add_history(p);
            *q = '\u{1e}' as i32 as libc::c_char;
        } else {
            r = p;
            loop {
                s = strchr(r, '\u{1f}' as i32 as libc::c_char as libc::c_int);
                if !(!s.is_null() && s < q) {
                    break;
                }
                pstr[field_cnt as usize] = r;
                pstr_len[field_cnt
                    as usize] = s.offset_from(r) as libc::c_long as libc::c_int;
                r = s.offset(1 as libc::c_int as isize);
                field_cnt += 1;
                field_cnt;
                if field_cnt == 30 as libc::c_int {
                    return;
                }
            }
            match type_0 {
                1 => {
                    unserialize_breakpoint(
                        pstr.as_mut_ptr(),
                        pstr_len.as_mut_ptr(),
                        field_cnt,
                    );
                }
                3 => {
                    unserialize_list_item(
                        &mut display_list,
                        pstr.as_mut_ptr(),
                        pstr_len.as_mut_ptr(),
                        field_cnt,
                    );
                }
                2 => {
                    unserialize_list_item(
                        &mut watch_list,
                        pstr.as_mut_ptr(),
                        pstr_len.as_mut_ptr(),
                        field_cnt,
                    );
                }
                5 => {
                    unserialize_option(
                        pstr.as_mut_ptr(),
                        pstr_len.as_mut_ptr(),
                        field_cnt,
                    );
                }
                4 | _ => {}
            }
        }
        p = q.offset(1 as libc::c_int as isize);
    }
    if type_0 == HISTORY as libc::c_int {
        sess_history_base = history_length;
    }
    unsetenv(env_variable[type_0 as usize]);
}
unsafe extern "C" fn prompt_yes_no(
    mut mesg: *const libc::c_char,
    mut res_true: libc::c_char,
    mut res_default: libc::c_int,
    mut fp: *mut FILE,
) -> libc::c_int {
    let mut in_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_int = res_default;
    if input_from_tty {
        fprintf(
            fp,
            b"%s\0" as *const u8 as *const libc::c_char,
            dcgettext(0 as *const libc::c_char, mesg, 5 as libc::c_int),
        );
        in_str = read_a_line.unwrap()(0 as *const libc::c_char);
        if in_str.is_null() {
            exit(1 as libc::c_int);
        }
        ret = (*in_str as libc::c_int == res_true as libc::c_int) as libc::c_int;
        free(in_str as *mut libc::c_void);
    }
    return ret;
}
pub unsafe extern "C" fn has_break_or_watch_point(
    mut pnum: *mut libc::c_int,
    mut any: bool,
) -> libc::c_int {
    let mut b: *mut BREAKPOINT = 0 as *mut BREAKPOINT;
    let mut w: *mut list_item = 0 as *mut list_item;
    if any {
        if breakpoints.next != &mut breakpoints as *mut BREAKPOINT {
            b = breakpoints.next;
        }
        if watch_list.next != &mut watch_list as *mut list_item {
            w = watch_list.next;
        }
        if b.is_null() && w.is_null() {
            return 0 as libc::c_int;
        }
        if !b.is_null() && w.is_null() {
            *pnum = (*b).number;
            return D_break as libc::c_int;
        }
        if !w.is_null() && b.is_null() {
            *pnum = (*w).number;
            return D_watch as libc::c_int;
        }
        if (*w).number > (*b).number {
            *pnum = (*w).number;
            return D_watch as libc::c_int;
        }
        *pnum = (*b).number;
        return D_break as libc::c_int;
    }
    b = breakpoints.next;
    while b != &mut breakpoints as *mut BREAKPOINT {
        if (*b).number == *pnum {
            return D_break as libc::c_int;
        }
        b = (*b).next;
    }
    w = watch_list.next;
    while w != &mut watch_list as *mut list_item {
        if (*w).number == *pnum {
            return D_watch as libc::c_int;
        }
        w = (*w).next;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn delete_commands_item(mut c: *mut commands_item) {
    free((*c).cmd_string as *mut libc::c_void);
    free_cmdarg((*c).arg);
    (*(*c).next).prev = (*c).prev;
    (*(*c).prev).next = (*c).next;
    free(c as *mut libc::c_void);
}
pub unsafe extern "C" fn do_commands(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    static mut b: *mut BREAKPOINT = 0 as *const BREAKPOINT as *mut BREAKPOINT;
    static mut w: *mut list_item = 0 as *const list_item as *mut list_item;
    static mut commands: *mut commands_item = 0 as *const commands_item
        as *mut commands_item;
    let mut c: *mut commands_item = 0 as *mut commands_item;
    if cmd == D_commands as libc::c_int {
        let mut num: libc::c_int = -(1 as libc::c_int);
        let mut type_0: libc::c_int = 0;
        if arg.is_null() {
            type_0 = has_break_or_watch_point(&mut num, 1 as libc::c_int != 0);
        } else {
            num = (*arg).value.lval as libc::c_int;
            type_0 = has_break_or_watch_point(&mut num, 0 as libc::c_int != 0);
        }
        b = 0 as *mut BREAKPOINT;
        w = 0 as *mut list_item;
        if type_0 == D_break as libc::c_int {
            b = find_breakpoint(num as libc::c_long);
        } else if type_0 == D_watch as libc::c_int {
            w = find_item(&mut watch_list, num as libc::c_long);
        }
        commands = if !b.is_null() { &mut (*b).commands } else { &mut (*w).commands };
        c = (*commands).next;
        while c != commands {
            c = (*c).prev;
            delete_commands_item((*c).next);
            c = (*c).next;
        }
        return 0 as libc::c_int;
    } else if cmd == D_end as libc::c_int {
        commands = 0 as *mut commands_item;
        if read_a_line
            == Some(
                read_commands_string
                    as unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_char,
            )
        {
            return 1 as libc::c_int;
        }
        return 0 as libc::c_int;
    } else if cmd == D_silent as libc::c_int {
        if !b.is_null() {
            (*b).silent = 1 as libc::c_int != 0;
        } else if !w.is_null() {
            (*w).silent = 1 as libc::c_int;
        }
    }
    c = emalloc_real(
        ::std::mem::size_of::<commands_item>() as libc::c_ulong,
        b"do_commands\0" as *const u8 as *const libc::c_char,
        b"c\0" as *const u8 as *const libc::c_char,
        b"debug.c\0" as *const u8 as *const libc::c_char,
        5054 as libc::c_int,
    ) as *mut commands_item;
    (*c).next = 0 as *mut commands_item;
    (*c).cmd = cmd;
    (*c).cmd_string = (*arg).value.sval;
    (*c).arg = (*arg).next;
    free(arg as *mut libc::c_void);
    (*c).prev = (*commands).prev;
    (*c).next = commands;
    (*commands).prev = c;
    (*(*c).prev).next = c;
    return 0 as libc::c_int;
}
unsafe extern "C" fn execute_commands(mut commands: *mut commands_item) -> libc::c_int {
    let mut c: *mut commands_item = 0 as *mut commands_item;
    let mut cmd_ptr: Func_cmd = None;
    let mut ret: bool = 0 as libc::c_int != 0;
    c = (*commands).next;
    while c != commands {
        if !((*c).cmd == D_silent as libc::c_int) {
            cmd_ptr = get_command((*c).cmd);
            ret = (Some(cmd_ptr.unwrap())).unwrap()((*c).arg, (*c).cmd) != 0;
            if ret {
                break;
            }
        }
        c = (*c).next;
    }
    return ret as libc::c_int;
}
pub unsafe extern "C" fn do_print_f(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut a: *mut CMDARG = 0 as *mut CMDARG;
    let mut tmp: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut fatal_tag_stack: jmp_buf = [__jmp_buf_tag {
        __jmpbuf: [0; 8],
        __mask_was_saved: 0,
        __saved_mask: __sigset_t { __val: [0; 16] },
    }; 1];
    a = arg;
    while !a.is_null() {
        count += 1;
        count;
        a = (*a).next;
    }
    tmp = emalloc_real(
        (count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut NODE>() as libc::c_ulong),
        b"do_print_f\0" as *const u8 as *const libc::c_char,
        b"tmp\0" as *const u8 as *const libc::c_char,
        b"debug.c\0" as *const u8 as *const libc::c_char,
        5110 as libc::c_int,
    ) as *mut *mut NODE;
    i = 0 as libc::c_int;
    a = arg;
    's_33: loop {
        if a.is_null() {
            current_block = 1847472278776910194;
            break;
        }
        match (*a).type_0 as libc::c_uint {
            45 => {
                name = (*a).value.sval;
                r = find_symbol(name, 0 as *mut *mut libc::c_char);
                if r.is_null() {
                    current_block = 17049662695537587088;
                    break;
                }
                if (*r).type_0 as libc::c_uint
                    == Node_var_new as libc::c_int as libc::c_uint
                    || (*r).type_0 as libc::c_uint
                        == Node_elem_new as libc::c_int as libc::c_uint
                {
                    let ref mut fresh20 = *tmp.offset(i as isize);
                    *fresh20 = Nnull_string;
                } else if (*r).type_0 as libc::c_uint
                    != Node_var as libc::c_int as libc::c_uint
                {
                    d_error(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"`%s' is not a scalar variable\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        name,
                    );
                    current_block = 17049662695537587088;
                    break;
                } else {
                    let ref mut fresh21 = *tmp.offset(i as isize);
                    *fresh21 = (*r).sub.nodep.l.lptr;
                }
            }
            47 => {
                let mut field_num: libc::c_long = 0;
                r = (*a).value.nodeval;
                field_num = (*r).sub.val.fltnum as libc::c_long;
                let ref mut fresh22 = *tmp.offset(i as isize);
                *fresh22 = *get_field(field_num, 0 as *mut Func_ptr);
            }
            49 => {
                let mut cnt: libc::c_int = (*a).a_count;
                name = (*a).value.sval;
                r = find_array(name);
                if r.is_null() {
                    current_block = 17049662695537587088;
                    break;
                }
                while cnt > 0 as libc::c_int {
                    let mut value: *mut NODE = 0 as *mut NODE;
                    let mut subs: *mut NODE = 0 as *mut NODE;
                    a = (*a).next;
                    subs = (*a).value.nodeval;
                    value = in_array(r, subs);
                    if cnt == 1 as libc::c_int {
                        if value.is_null() {
                            let ref mut fresh23 = *tmp.offset(i as isize);
                            *fresh23 = Nnull_string;
                        } else if (*value).type_0 as libc::c_uint
                            == Node_var_array as libc::c_int as libc::c_uint
                        {
                            d_error(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"attempt to use array `%s[\"%.*s\"]' in a scalar context\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                name,
                                (*subs).sub.val.slen as libc::c_int,
                                (*subs).sub.val.sp,
                            );
                            current_block = 17049662695537587088;
                            break 's_33;
                        } else {
                            let ref mut fresh24 = *tmp.offset(i as isize);
                            *fresh24 = value;
                        }
                    } else if value.is_null() {
                        d_error(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"[\"%.*s\"] not in array `%s'\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            (*subs).sub.val.slen as libc::c_int,
                            (*subs).sub.val.sp,
                            name,
                        );
                        current_block = 17049662695537587088;
                        break 's_33;
                    } else if (*value).type_0 as libc::c_uint
                        != Node_var_array as libc::c_int as libc::c_uint
                    {
                        d_error(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"attempt to use scalar `%s[\"%.*s\"]' as array\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            name,
                            (*subs).sub.val.slen as libc::c_int,
                            (*subs).sub.val.sp,
                        );
                        current_block = 17049662695537587088;
                        break 's_33;
                    } else {
                        r = value;
                        name = (*r).sub.nodep.name;
                    }
                    cnt -= 1;
                    cnt;
                }
            }
            46 => {
                let ref mut fresh25 = *tmp.offset(i as isize);
                *fresh25 = (*a).value.nodeval;
            }
            _ => {}
        }
        i += 1;
        i;
        a = (*a).next;
    }
    match current_block {
        1847472278776910194 => {
            let ref mut fresh26 = *tmp.offset(0 as libc::c_int as isize);
            *fresh26 = force_string_fmt(
                *tmp.offset(0 as libc::c_int as isize),
                CONVFMT,
                CONVFMTidx,
            );
            let fresh27 = fatal_tag_valid;
            fatal_tag_valid = fatal_tag_valid + 1;
            if fresh27 != 0 {
                memcpy(
                    fatal_tag_stack.as_mut_ptr() as *mut libc::c_char
                        as *mut libc::c_void,
                    fatal_tag.as_mut_ptr() as *const libc::c_char as *const libc::c_void,
                    ::std::mem::size_of::<jmp_buf>() as libc::c_ulong,
                );
            }
            if _setjmp(fatal_tag.as_mut_ptr()) == 0 as libc::c_int {
                r = format_tree(
                    (**tmp.offset(0 as libc::c_int as isize)).sub.val.sp,
                    (**tmp.offset(0 as libc::c_int as isize)).sub.val.slen,
                    tmp,
                    i as libc::c_long,
                );
            } else {
                exit_val = 0 as libc::c_int;
                r = 0 as *mut NODE;
            }
            fatal_tag_valid -= 1;
            if fatal_tag_valid != 0 {
                memcpy(
                    fatal_tag.as_mut_ptr() as *mut libc::c_char as *mut libc::c_void,
                    fatal_tag_stack.as_mut_ptr() as *const libc::c_char
                        as *const libc::c_void,
                    ::std::mem::size_of::<jmp_buf>() as libc::c_ulong,
                );
            }
            if !r.is_null() {
                if 0 != 0 && 0 != 0
                    && (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                        .wrapping_mul((*r).sub.val.slen)
                        <= 8 as libc::c_int as libc::c_ulong
                    && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                        != 0 as libc::c_int as libc::c_ulong
                {
                    ({
                        let mut __ptr: *const libc::c_char = (*r).sub.val.sp
                            as *const libc::c_char;
                        let mut __stream: *mut FILE = out_fp;
                        let mut __cnt: size_t = 0;
                        __cnt = (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                            .wrapping_mul((*r).sub.val.slen);
                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                            let fresh28 = __ptr;
                            __ptr = __ptr.offset(1);
                            if putc_unlocked(*fresh28 as libc::c_int, __stream)
                                == -(1 as libc::c_int)
                            {
                                break;
                            }
                            __cnt = __cnt.wrapping_sub(1);
                            __cnt;
                        }
                        0u8
                    });
                } else {
                    if 0 != 0
                        && ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                            == 0 as libc::c_int as libc::c_ulong
                        || 0 != 0
                            && (*r).sub.val.slen == 0 as libc::c_int as libc::c_ulong
                    {} else {
                        fwrite_unlocked(
                            (*r).sub.val.sp as *const libc::c_void,
                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            (*r).sub.val.slen,
                            out_fp,
                        );
                    };
                };
                0u8;
                unref(r);
            }
        }
        _ => {}
    }
    free(tmp as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn do_source(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut file: *mut libc::c_char = (*arg).value.sval;
    fd = open_readfd(file);
    if fd <= -(1 as libc::c_int) {
        d_error(
            dcgettext(
                0 as *const libc::c_char,
                b"cannot open source file `%s' for reading: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            file,
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    push_cmd_src(
        fd,
        0 as libc::c_int != 0,
        Some(
            g_readline as unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_char,
        ),
        Some(close as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
        D_source as libc::c_int,
        0 as libc::c_int,
    );
    (*cmd_src).str_0 = estrdup(file, strlen(file));
    return 0 as libc::c_int;
}
unsafe extern "C" fn open_readfd(mut file: *const libc::c_char) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    fd = open(file, 0 as libc::c_int);
    if fd <= -(1 as libc::c_int) {
        return -(1 as libc::c_int)
    } else if os_isdir(fd) != 0 {
        close(fd);
        *__errno_location() = 21 as libc::c_int;
        return -(1 as libc::c_int);
    }
    return fd;
}
pub unsafe extern "C" fn find_option(mut name: *mut libc::c_char) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut idx: libc::c_int = 0;
    idx = 0 as libc::c_int;
    loop {
        p = option_list[idx as usize].name;
        if p.is_null() {
            break;
        }
        if strcmp(p, name) == 0 as libc::c_int {
            return idx;
        }
        idx += 1;
        idx;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn option_help() {
    let mut opt: *const dbg_option = 0 as *const dbg_option;
    opt = option_list.as_ptr();
    while !((*opt).name).is_null() {
        fprintf(
            out_fp,
            b"\t%-15.15s - %s\n\0" as *const u8 as *const libc::c_char,
            (*opt).name,
            dcgettext(0 as *const libc::c_char, (*opt).help_txt, 5 as libc::c_int),
        );
        opt = opt.offset(1);
        opt;
    }
}
pub unsafe extern "C" fn option_generator(
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
    loop {
        let fresh29 = idx;
        idx = idx + 1;
        name = option_list[fresh29 as usize].name;
        if name.is_null() {
            break;
        }
        if strncmp(name, text, textlen) == 0 as libc::c_int {
            return estrdup(name, strlen(name));
        }
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn set_gawk_output(mut file: *const libc::c_char) {
    let mut fd: libc::c_int = -(1 as libc::c_int);
    let mut fp: *mut FILE = 0 as *mut FILE;
    if output_fp != stdout {
        if output_fp != stderr {
            fclose(output_fp);
            free(output_file as *mut libc::c_void);
        }
        output_fp = stdout;
        output_is_tty = os_isatty(fileno(stdout)) != 0;
        output_file = b"/dev/stdout\0" as *const u8 as *const libc::c_char;
    }
    if file.is_null()
        || *file.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        return;
    }
    *__errno_location() = 0 as libc::c_int;
    fd = os_devopen(file, 0o1 as libc::c_int);
    if fd != -(1 as libc::c_int) {
        fp = fdopen(fd, b"w\0" as *const u8 as *const libc::c_char);
        if fp.is_null() {
            close(fd);
        }
    } else if strncmp(
        file,
        b"/dev/\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        let mut cp: *mut libc::c_char = (file as *mut libc::c_char)
            .offset(5 as libc::c_int as isize);
        if strcmp(cp, b"stdout\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            return;
        }
        if strcmp(cp, b"stderr\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            output_fp = stderr;
            output_file = b"/dev/stderr\0" as *const u8 as *const libc::c_char;
            output_is_tty = os_isatty(fileno(stderr)) != 0;
            return;
        }
        if strncmp(
            cp,
            b"fd/\0" as *const u8 as *const libc::c_char,
            3 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            cp = cp.offset(3 as libc::c_int as isize);
            fd = strtoul(cp, 0 as *mut *mut libc::c_char, 10 as libc::c_int)
                as libc::c_int;
            if *__errno_location() == 0 as libc::c_int && fd > -(1 as libc::c_int) {
                fp = fdopen(fd, b"w\0" as *const u8 as *const libc::c_char);
                if fp.is_null() {
                    fd = -(1 as libc::c_int);
                }
            } else {
                fd = -(1 as libc::c_int);
            }
        } else {
            fd = open(file, 0o1 as libc::c_int);
        }
        if fd > -(1 as libc::c_int) && fp.is_null() {
            fp = fdopen(fd, b"w\0" as *const u8 as *const libc::c_char);
            if fp.is_null() {
                close(fd);
            }
        }
    } else {
        fp = fopen(file, b"w\0" as *const u8 as *const libc::c_char);
    }
    if !fp.is_null() {
        output_fp = fp;
        output_file = estrdup(file, strlen(file));
        setbuf(fp, 0 as *mut libc::c_void as *mut libc::c_char);
        output_is_tty = os_isatty(fileno(fp)) != 0;
    } else {
        d_error(
            dcgettext(
                0 as *const libc::c_char,
                b"could not open `%s' for writing: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            file,
            if *__errno_location() != 0 as libc::c_int {
                strerror(*__errno_location())
            } else {
                dcgettext(
                    0 as *const libc::c_char,
                    b"reason unknown\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            },
        );
        fprintf(
            out_fp,
            dcgettext(
                0 as *const libc::c_char,
                b"sending output to stdout\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    };
}
unsafe extern "C" fn set_prompt(mut value: *const libc::c_char) {
    free(dgawk_prompt as *mut libc::c_void);
    dgawk_prompt = estrdup(value, strlen(value));
    dbg_prompt = dgawk_prompt;
}
unsafe extern "C" fn set_option_flag(mut value: *const libc::c_char) -> libc::c_int {
    let mut n: libc::c_long = 0;
    if strcmp(value, b"on\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if strcmp(value, b"off\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    *__errno_location() = 0 as libc::c_int;
    n = strtol(value, 0 as *mut *mut libc::c_char, 0 as libc::c_int);
    return (*__errno_location() == 0 as libc::c_int
        && n != 0 as libc::c_int as libc::c_long) as libc::c_int;
}
unsafe extern "C" fn set_option_num(
    mut pnum: *mut libc::c_int,
    mut value: *const libc::c_char,
) {
    let mut n: libc::c_long = 0;
    *__errno_location() = 0 as libc::c_int;
    n = strtol(value, 0 as *mut *mut libc::c_char, 0 as libc::c_int);
    if *__errno_location() == 0 as libc::c_int && n > 0 as libc::c_int as libc::c_long {
        *pnum = n as libc::c_int;
    } else {
        d_error(
            dcgettext(
                0 as *const libc::c_char,
                b"invalid number\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    };
}
unsafe extern "C" fn set_listsize(mut value: *const libc::c_char) {
    set_option_num(&mut list_size, value);
}
unsafe extern "C" fn set_trace(mut value: *const libc::c_char) {
    do_trace = set_option_flag(value);
}
unsafe extern "C" fn set_save_history(mut value: *const libc::c_char) {
    do_save_history = set_option_flag(value);
}
unsafe extern "C" fn set_save_options(mut value: *const libc::c_char) {
    do_save_options = set_option_flag(value);
}
unsafe extern "C" fn set_history_size(mut value: *const libc::c_char) {
    set_option_num(&mut history_size, value);
}
pub unsafe extern "C" fn read_commands_string(
    mut prompt: *const libc::c_char,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    if commands_string.is_null() {
        return 0 as *mut libc::c_char;
    }
    p = commands_string as *mut libc::c_char;
    end = (commands_string as *mut libc::c_char).offset(commands_string_len as isize);
    while p < end {
        if *p as libc::c_int == line_sep as libc::c_int {
            line = estrdup(
                commands_string,
                p.offset_from(commands_string) as libc::c_long as size_t,
            );
            commands_string = p.offset(1 as libc::c_int as isize);
            commands_string_len = end.offset_from(commands_string) as libc::c_long
                as libc::c_int;
            return line;
        }
        p = p.offset(1);
        p;
    }
    line = estrdup(commands_string, commands_string_len as size_t);
    commands_string = 0 as *const libc::c_char;
    commands_string_len = 0 as libc::c_int;
    return line;
}
unsafe extern "C" fn save_options(mut file: *const libc::c_char) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut opt: *const dbg_option = 0 as *const dbg_option;
    fp = fopen(file, b"w\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        return;
    }
    opt = option_list.as_ptr();
    while !((*opt).name).is_null() {
        if !((*opt).str_val).is_null() {
            fprintf(
                fp,
                b"option %s = \"%s\"\n\0" as *const u8 as *const libc::c_char,
                (*opt).name,
                *(*opt).str_val,
            );
        } else {
            fprintf(
                fp,
                b"option %s = %d\n\0" as *const u8 as *const libc::c_char,
                (*opt).name,
                *(*opt).num_val,
            );
        }
        opt = opt.offset(1);
        opt;
    }
    fclose(fp);
    chmod(file, 0o600 as libc::c_int as __mode_t);
}
unsafe extern "C" fn close_all() {
    let mut stdio_problem: bool = false;
    let mut got_EPIPE: bool = false;
    let mut cs: *mut command_source = 0 as *mut command_source;
    nextfile(&mut curfile, 1 as libc::c_int != 0);
    close_io(&mut stdio_problem, &mut got_EPIPE);
    if (*cur_srcfile).fd != -(1 as libc::c_int) {
        close((*cur_srcfile).fd);
        (*cur_srcfile).fd = -(1 as libc::c_int);
    }
    cs = cmd_src;
    while !cs.is_null() {
        if ((*cs).close_func).is_some() && (*cs).fd != -(1 as libc::c_int) {
            ((*cs).close_func).unwrap()((*cs).fd);
            (*cs).fd = -(1 as libc::c_int);
        }
        cs = (*cs).next;
    }
    close_extensions();
    set_gawk_output(0 as *const libc::c_char);
}
unsafe extern "C" fn pre_execute_code(mut pi: *mut *mut INSTRUCTION) -> libc::c_int {
    let mut ei: *mut INSTRUCTION = *pi;
    match (*ei).opcode as libc::c_uint {
        60 | 59 | 67 | 66 => {
            d_error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"`%s' not allowed in current context; statement ignored\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                op2str((*ei).opcode),
            );
            *pi = (*ei).nexti;
        }
        62 => {
            if !((*ei).nexti).is_null() {
                let mut r: *mut NODE = 0 as *mut NODE;
                d_error(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"`return' not allowed in current context; statement ignored\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                r = POP_SCALAR();
                DEREF(r);
                *pi = (*ei).nexti;
            }
        }
        _ => {}
    }
    return (ei == *pi) as libc::c_int;
}
unsafe extern "C" fn execute_code(mut code: *mut INSTRUCTION) -> *mut NODE {
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut fatal_tag_stack: jmp_buf = [__jmp_buf_tag {
        __jmpbuf: [0; 8],
        __mask_was_saved: 0,
        __saved_mask: __sigset_t { __val: [0; 16] },
    }; 1];
    let mut save_flags: libc::c_int = do_flags as libc::c_int;
    do_flags = DO_FLAG_NONE;
    let fresh30 = fatal_tag_valid;
    fatal_tag_valid = fatal_tag_valid + 1;
    if fresh30 != 0 {
        memcpy(
            fatal_tag_stack.as_mut_ptr() as *mut libc::c_char as *mut libc::c_void,
            fatal_tag.as_mut_ptr() as *const libc::c_char as *const libc::c_void,
            ::std::mem::size_of::<jmp_buf>() as libc::c_ulong,
        );
    }
    if _setjmp(fatal_tag.as_mut_ptr()) == 0 as libc::c_int {
        interpret.unwrap()(code as *mut INSTRUCTION);
        r = POP_SCALAR() as *mut NODE;
    } else {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"fatal error during eval, need to restart.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        restart(0 as libc::c_int != 0);
    }
    fatal_tag_valid -= 1;
    if fatal_tag_valid != 0 {
        memcpy(
            fatal_tag.as_mut_ptr() as *mut libc::c_char as *mut libc::c_void,
            fatal_tag_stack.as_mut_ptr() as *const libc::c_char as *const libc::c_void,
            ::std::mem::size_of::<jmp_buf>() as libc::c_ulong,
        );
    }
    do_flags = save_flags as do_flag_values;
    if exit_val != 0 as libc::c_int {
        exit_val = 0 as libc::c_int;
        return 0 as *mut NODE;
    }
    return r as *mut NODE;
}
pub unsafe extern "C" fn do_eval(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut ret_val: *mut NODE = 0 as *mut NODE;
    let mut f: *mut NODE = 0 as *mut NODE;
    let mut this_frame: *mut NODE = 0 as *mut NODE;
    let mut this_func: *mut NODE = 0 as *mut NODE;
    let mut sp: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut eval: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut code: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut ctxt: *mut AWK_CONTEXT = 0 as *mut AWK_CONTEXT;
    let mut ecount: libc::c_int = 0 as libc::c_int;
    let mut pcount: libc::c_int = 0 as libc::c_int;
    let mut ret: libc::c_int = 0;
    let mut save_flags: libc::c_int = do_flags as libc::c_int;
    let mut the_source: *mut SRCFILE = 0 as *mut SRCFILE;
    if prog_running {
        this_frame = find_frame(0 as libc::c_int as libc::c_long);
        this_func = (*this_frame).sub.nodep.x.extra;
    }
    install_params(this_func);
    ctxt = new_context();
    (*ctxt).install_func = Some(append_symbol as unsafe extern "C" fn(*mut NODE) -> ());
    push_context(ctxt);
    the_source = add_srcfile(
        SRC_CMDLINE,
        (*arg).value.sval,
        srcfiles,
        0 as *mut bool,
        0 as *mut libc::c_int,
    );
    do_flags = ::std::mem::transmute::<
        libc::c_uint,
        do_flag_values,
    >(do_flags as libc::c_uint & DO_MPFR as libc::c_int as libc::c_uint);
    ret = parse_program(&mut code, 1 as libc::c_int != 0);
    do_flags = save_flags as do_flag_values;
    remove_params(this_func);
    if ret != 0 as libc::c_int {
        pop_context();
        free_context(ctxt, 0 as libc::c_int != 0);
        let mut s: *mut NODE = make_str_node(
            b"@eval\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as size_t,
            0 as libc::c_int,
        );
        ((*(*func_table).sub.nodep.l.lp).remove).unwrap()(func_table, s);
        unref(s);
        return 0 as libc::c_int;
    }
    f = lookup(b"@eval\0" as *const u8 as *const libc::c_char);
    if this_func.is_null() {
        eval = bcalloc(Op_func_call, 2 as libc::c_int, 0 as libc::c_int);
        (*eval).d.name = (*cur_srcfile).src;
        (*eval).x.xn = f;
        (*eval).d.name = 0 as *mut libc::c_char;
        (*eval.offset(1 as libc::c_int as isize))
            .x
            .xl = 0 as libc::c_int as libc::c_long;
        (*eval).nexti = bcalloc(Op_stop, 1 as libc::c_int, 0 as libc::c_int);
    } else {
        let mut i: libc::c_int = 0;
        let mut t: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
        eval = (*f).sub.nodep.r.iptr;
        (*eval).d.name = (*cur_srcfile).src;
        t = (*eval.offset(1 as libc::c_int as isize)).d.di;
        (*t).opcode = Op_stop;
        ecount = (*f).sub.nodep.l.ll as libc::c_int;
        pcount = (*this_func).sub.nodep.l.ll as libc::c_int;
        if ecount > 0 as libc::c_int {
            if pcount == 0 as libc::c_int {
                (*this_frame)
                    .sub
                    .nodep
                    .r
                    .av = emalloc_real(
                    (ecount as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut NODE>() as libc::c_ulong,
                        ),
                    b"do_eval\0" as *const u8 as *const libc::c_char,
                    b"this_frame->stack\0" as *const u8 as *const libc::c_char,
                    b"debug.c\0" as *const u8 as *const libc::c_char,
                    5679 as libc::c_int,
                ) as *mut *mut NODE;
            } else {
                (*this_frame)
                    .sub
                    .nodep
                    .r
                    .av = erealloc_real(
                    (*this_frame).sub.nodep.r.av as *mut libc::c_void,
                    ((pcount + ecount) as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut NODE>() as libc::c_ulong,
                        ),
                    b"do_eval\0" as *const u8 as *const libc::c_char,
                    b"this_frame->stack\0" as *const u8 as *const libc::c_char,
                    b"debug.c\0" as *const u8 as *const libc::c_char,
                    5681 as libc::c_int,
                ) as *mut *mut NODE;
            }
            sp = ((*this_frame).sub.nodep.r.av).offset(pcount as isize);
            i = 0 as libc::c_int;
            while i < ecount {
                let mut np: *mut NODE = 0 as *mut NODE;
                np = ((*f).sub.nodep.rn).offset(i as isize);
                (*np).sub.nodep.l.ll += pcount as libc::c_long;
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
                let fresh31 = sp;
                sp = sp.offset(1);
                *fresh31 = r;
                (*r).type_0 = Node_var_new;
                (*r).sub.nodep.name = (*np).sub.nodep.name;
                i += 1;
                i;
            }
            (*this_func).sub.nodep.l.ll += ecount as libc::c_long;
        }
    }
    ret_val = execute_code(eval as *mut INSTRUCTION);
    if !ret_val.is_null() {
        DEREF(ret_val);
    }
    if !this_func.is_null() && ecount > 0 as libc::c_int {
        let mut i_0: libc::c_int = 0;
        sp = ((*this_frame).sub.nodep.r.av).offset(pcount as isize);
        i_0 = ecount;
        while i_0 > 0 as libc::c_int {
            r = *sp;
            if (*r).type_0 as libc::c_uint == Node_var as libc::c_int as libc::c_uint {
                DEREF((*r).sub.nodep.l.lptr);
            } else if (*r).type_0 as libc::c_uint
                == Node_var_array as libc::c_int as libc::c_uint
            {
                ((*(*r).sub.nodep.l.lp).clear).unwrap()(r, 0 as *mut exp_node);
            }
            let ref mut fresh32 = (*(r as *mut block_item)).freep;
            *fresh32 = nextfree[BLOCK_NODE as libc::c_int as usize].freep;
            nextfree[BLOCK_NODE as libc::c_int as usize].freep = r as *mut block_item;
            let fresh33 = sp;
            sp = sp.offset(1);
            *fresh33 = 0 as *mut NODE;
            i_0 -= 1;
            i_0;
        }
        if pcount == 0 as libc::c_int {
            free((*this_frame).sub.nodep.r.av as *mut libc::c_void);
            (*this_frame).sub.nodep.r.av = 0 as *mut *mut exp_node;
        }
        (*this_func).sub.nodep.l.ll -= ecount as libc::c_long;
    }
    pop_context();
    free_context(ctxt, !ret_val.is_null());
    if !ret_val.is_null() {
        let mut s_0: *mut NODE = make_str_node(
            b"@eval\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as size_t,
            0 as libc::c_int,
        );
        ((*(*func_table).sub.nodep.l.lp).remove).unwrap()(func_table, s_0);
        unref(s_0);
    }
    free((*f).sub.nodep.name as *mut libc::c_void);
    let ref mut fresh34 = (*(f as *mut block_item)).freep;
    *fresh34 = nextfree[BLOCK_NODE as libc::c_int as usize].freep;
    nextfree[BLOCK_NODE as libc::c_int as usize].freep = f as *mut block_item;
    free_srcfile(the_source);
    return 0 as libc::c_int;
}
static mut invalid_symbol: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn check_symbol(mut r: *mut NODE) {
    invalid_symbol += 1;
    invalid_symbol;
    d_error(
        dcgettext(
            0 as *const libc::c_char,
            b"no symbol `%s' in current context\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        (*r).sub.nodep.name,
    );
    append_symbol(r);
}
unsafe extern "C" fn parse_condition(
    mut type_0: libc::c_int,
    mut num: libc::c_int,
    mut expr: *mut libc::c_char,
) -> libc::c_int {
    let mut code: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut ctxt: *mut AWK_CONTEXT = 0 as *mut AWK_CONTEXT;
    let mut ret: libc::c_int = 0;
    let mut b: *mut BREAKPOINT = 0 as *mut BREAKPOINT;
    let mut w: *mut list_item = 0 as *mut list_item;
    let mut this_func: *mut NODE = 0 as *mut NODE;
    let mut it: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut stop_0: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut rule: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut cndn: *mut condition = 0 as *mut condition;
    let mut save_flags: libc::c_int = do_flags as libc::c_int;
    if type_0 == D_break as libc::c_int
        && {
            b = find_breakpoint(num as libc::c_long);
            !b.is_null()
        }
    {
        let mut rp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
        cndn = &mut (*b).cndn;
        rp = find_rule((*b).src, (*(*b).bpi).source_line as libc::c_long);
        if !rp.is_null()
            && (*rp).opcode as libc::c_uint == Op_func as libc::c_int as libc::c_uint
        {
            this_func = (*rp).x.xn;
        }
    } else if type_0 == D_watch as libc::c_int
        && {
            w = find_item(&mut watch_list, num as libc::c_long);
            !w.is_null()
        }
    {
        cndn = &mut (*w).cndn;
        this_func = (*find_frame(cur_frame)).sub.nodep.x.extra;
    }
    if cndn.is_null() {
        return -(1 as libc::c_int);
    }
    if !expr.is_null() {
        install_params(this_func);
        ctxt = new_context();
        invalid_symbol = 0 as libc::c_int;
        (*ctxt)
            .install_func = Some(check_symbol as unsafe extern "C" fn(*mut NODE) -> ());
        push_context(ctxt);
        add_srcfile(SRC_CMDLINE, expr, srcfiles, 0 as *mut bool, 0 as *mut libc::c_int);
        do_flags = DO_FLAG_NONE;
        ret = parse_program(&mut code, 1 as libc::c_int != 0);
        do_flags = save_flags as do_flag_values;
        remove_params(this_func);
        pop_context();
        if ret != 0 as libc::c_int || invalid_symbol != 0 {
            free_context(ctxt, 0 as libc::c_int != 0);
            return -(1 as libc::c_int);
        }
        rule = (*ctxt).rule_list.nexti;
        stop_0 = bcalloc(Op_stop, 1 as libc::c_int, 0 as libc::c_int);
        it = (*rule).x.xi;
        (*it).opcode = Op_push_i;
        (*it).d.dn = make_number.unwrap()(1.0f64);
        (*it).nexti = bcalloc(Op_jmp, 1 as libc::c_int, 0 as libc::c_int);
        (*(*it).nexti).d.di = stop_0;
        (*(*it).nexti).nexti = (*rule).d.di;
        it = (*rule).d.di;
        (*it).opcode = Op_push_i;
        (*it).d.dn = make_number.unwrap()(0.0f64);
        (*it).nexti = stop_0;
    }
    if !((*cndn).expr).is_null() {
        free((*cndn).expr as *mut libc::c_void);
    }
    free_context((*cndn).ctxt, 0 as libc::c_int != 0);
    (*cndn).code = code;
    (*cndn).expr = expr;
    (*cndn).ctxt = ctxt;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn do_condition(
    mut arg: *mut CMDARG,
    mut cmd: libc::c_int,
) -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut expr: *mut libc::c_char = 0 as *mut libc::c_char;
    num = (*arg).value.lval as libc::c_int;
    type_0 = has_break_or_watch_point(&mut num, 0 as libc::c_int != 0);
    if type_0 == 0 {
        return 0 as libc::c_int;
    }
    arg = (*arg).next;
    if !arg.is_null() {
        expr = (*arg).value.sval;
    }
    if parse_condition(type_0, num, expr) == 0 as libc::c_int && !arg.is_null() {
        (*arg).value.sval = 0 as *mut libc::c_char;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn in_cmd_src(mut filename: *const libc::c_char) -> libc::c_int {
    let mut cs: *mut command_source = 0 as *mut command_source;
    cs = cmd_src;
    while !cs.is_null() {
        if !((*cs).str_0).is_null() && strcmp((*cs).str_0, filename) == 0 as libc::c_int
        {
            return 1 as libc::c_int;
        }
        cs = (*cs).next;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn get_eof_status() -> libc::c_int {
    if cmd_src.is_null() {
        return 2 as libc::c_int;
    }
    return (*cmd_src).eof_status;
}
pub unsafe extern "C" fn push_cmd_src(
    mut fd: libc::c_int,
    mut istty: bool,
    mut readfunc: Option::<
        unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_char,
    >,
    mut closefunc: Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
    mut ctype: libc::c_int,
    mut eofstatus: libc::c_int,
) {
    let mut cs: *mut command_source = 0 as *mut command_source;
    cs = emalloc_real(
        ::std::mem::size_of::<command_source>() as libc::c_ulong,
        b"push_cmd_src\0" as *const u8 as *const libc::c_char,
        b"cs\0" as *const u8 as *const libc::c_char,
        b"debug.c\0" as *const u8 as *const libc::c_char,
        5923 as libc::c_int,
    ) as *mut command_source;
    (*cs).fd = fd;
    (*cs).is_tty = istty as libc::c_int;
    (*cs).read_func = readfunc;
    (*cs).close_func = closefunc;
    (*cs).cmd = ctype;
    (*cs).eof_status = eofstatus;
    (*cs).str_0 = 0 as *mut libc::c_char;
    (*cs).next = cmd_src;
    cmd_src = cs;
    input_fd = fd;
    input_from_tty = istty;
    read_a_line = readfunc;
}
pub unsafe extern "C" fn pop_cmd_src() -> libc::c_int {
    let mut cs: *mut command_source = 0 as *mut command_source;
    if ((*cmd_src).next).is_null() {
        return -(1 as libc::c_int);
    }
    cs = cmd_src;
    cmd_src = (*cs).next;
    if ((*cs).close_func).is_some() && (*cs).fd != -(1 as libc::c_int) {
        ((*cs).close_func).unwrap()((*cs).fd);
    }
    if !((*cs).str_0).is_null() {
        free((*cs).str_0 as *mut libc::c_void);
    }
    free(cs as *mut libc::c_void);
    input_fd = (*cmd_src).fd;
    input_from_tty = (*cmd_src).is_tty != 0;
    read_a_line = (*cmd_src).read_func;
    return 0 as libc::c_int;
}
