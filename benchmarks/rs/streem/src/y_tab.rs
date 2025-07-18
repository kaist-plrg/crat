use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn clearerr(__stream: *mut FILE);
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn atol(__nptr: *const libc::c_char) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn node_array_new() -> *mut node;
    fn node_array_headers(_: *mut node) -> *mut node;
    fn node_array_add(_: *mut node, _: *mut node);
    fn node_nodes_new() -> *mut node;
    fn node_nodes_add(_: *mut node, _: *mut node);
    fn node_pair_new(_: node_string_0, _: *mut node) -> *mut node;
    fn node_args_new() -> *mut node;
    fn node_args_add(_: *mut node, _: node_string_0);
    fn node_pattern_new(_: node_type) -> *mut node;
    fn node_pattern_add(_: *mut node, _: *mut node);
    fn node_psplat_new(_: *mut node, _: *mut node, _: *mut node) -> *mut node;
    fn node_splat_new(_: *mut node) -> *mut node;
    fn node_plambda_new(_: *mut node, _: *mut node) -> *mut node;
    fn node_plambda_body(_: *mut node, _: *mut node) -> *mut node;
    fn node_plambda_add(_: *mut node, _: *mut node) -> *mut node;
    fn node_ns_new(_: node_string_0, _: *mut node) -> *mut node;
    fn node_import_new(_: node_string_0) -> *mut node;
    fn node_let_new(_: node_string_0, _: *mut node) -> *mut node;
    fn node_op_new(_: *const libc::c_char, _: *mut node, _: *mut node) -> *mut node;
    fn node_obj_new(_: *mut node, _: node_string_0) -> *mut node;
    fn node_lambda_new(_: *mut node, _: *mut node) -> *mut node;
    fn node_block_new(_: *mut node) -> *mut node;
    fn node_method_new(_: *mut node, _: *mut node) -> *mut node;
    fn node_call_new(
        _: node_string_0,
        _: *mut node,
        _: *mut node,
        _: *mut node,
    ) -> *mut node;
    fn node_fcall_new(_: *mut node, _: *mut node, _: *mut node) -> *mut node;
    fn node_genfunc_new(_: node_string_0) -> *mut node;
    fn node_int_new(_: libc::c_long) -> *mut node;
    fn node_float_new(_: libc::c_double) -> *mut node;
    fn node_time_new(_: *const libc::c_char, _: strm_int) -> *mut node;
    fn node_string_new(_: *const libc::c_char, _: strm_int) -> *mut node;
    fn node_if_new(_: *mut node, _: *mut node, _: *mut node) -> *mut node;
    fn node_emit_new(_: *mut node) -> *mut node;
    fn node_skip_new() -> *mut node;
    fn node_return_new(_: *mut node) -> *mut node;
    fn node_ident_new(_: node_string_0) -> *mut node;
    fn node_str_new(_: *const libc::c_char, len: strm_int) -> node_string_0;
    fn node_str_escaped(s: *const libc::c_char, len: strm_int) -> node_string_0;
    fn node_nil() -> *mut node;
    fn node_true() -> *mut node;
    fn node_false() -> *mut node;
    fn __errno_location() -> *mut libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __int32_t = libc::c_int;
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
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type strm_int = int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_lambda {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub args: *mut node,
    pub body: *mut node,
    pub block: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
}
pub type node_type = libc::c_uint;
pub const NODE_IMPORT: node_type = 30;
pub const NODE_NS: node_type = 29;
pub const NODE_NODES: node_type = 28;
pub const NODE_ARRAY: node_type = 27;
pub const NODE_GENFUNC: node_type = 26;
pub const NODE_FCALL: node_type = 25;
pub const NODE_CALL: node_type = 24;
pub const NODE_OP: node_type = 23;
pub const NODE_CONST: node_type = 22;
pub const NODE_VAR: node_type = 21;
pub const NODE_RETURN: node_type = 20;
pub const NODE_SKIP: node_type = 19;
pub const NODE_EMIT: node_type = 18;
pub const NODE_IF: node_type = 17;
pub const NODE_LET: node_type = 16;
pub const NODE_IDENT: node_type = 15;
pub const NODE_SPLAT: node_type = 14;
pub const NODE_PSPLAT: node_type = 13;
pub const NODE_PSTRUCT: node_type = 12;
pub const NODE_PARRAY: node_type = 11;
pub const NODE_PLAMBDA: node_type = 10;
pub const NODE_LAMBDA: node_type = 9;
pub const NODE_CFUNC: node_type = 8;
pub const NODE_PAIR: node_type = 7;
pub const NODE_ARGS: node_type = 6;
pub const NODE_BOOL: node_type = 5;
pub const NODE_NIL: node_type = 4;
pub const NODE_STR: node_type = 3;
pub const NODE_TIME: node_type = 2;
pub const NODE_FLOAT: node_type = 1;
pub const NODE_INT: node_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_string {
    pub len: strm_int,
    pub buf: [libc::c_char; 0],
}
pub type node_string_0 = *mut node_string;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parser_state {
    pub nerr: libc::c_int,
    pub lval: *mut libc::c_void,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub tline: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_str {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub value: node_string_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union YYSTYPE {
    pub nd: *mut node,
    pub id: node_string_0,
}
pub type yy_state_t = yytype_uint8;
pub type yytype_uint8 = libc::c_uchar;
pub type yytype_int8 = libc::c_schar;
pub type yy_state_fast_t = libc::c_int;
pub type yytype_int16 = libc::c_short;
pub const YYERROR_VERBOSE_ARGS_MAXIMUM: C2RustUnnamed = 5;
pub type C2RustUnnamed = libc::c_uint;
pub type flex_int16_t = int16_t;
pub type yy_state_type = libc::c_int;
pub type YY_CHAR = flex_uint8_t;
pub type flex_uint8_t = uint8_t;
pub type YY_BUFFER_STATE = *mut yy_buffer_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yy_buffer_state {
    pub yy_input_file: *mut FILE,
    pub yy_ch_buf: *mut libc::c_char,
    pub yy_buf_pos: *mut libc::c_char,
    pub yy_buf_size: libc::c_int,
    pub yy_n_chars: libc::c_int,
    pub yy_is_our_buffer: libc::c_int,
    pub yy_is_interactive: libc::c_int,
    pub yy_at_bol: libc::c_int,
    pub yy_bs_lineno: libc::c_int,
    pub yy_bs_column: libc::c_int,
    pub yy_fill_buffer: libc::c_int,
    pub yy_buffer_status: libc::c_int,
}
pub type yy_size_t = size_t;
pub const _ISprint: C2RustUnnamed_0 = 16384;
pub type flex_int32_t = int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union yyalloc {
    pub yyss_alloc: yy_state_t,
    pub yyvs_alloc: YYSTYPE,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_0 = 8;
pub const _ISpunct: C2RustUnnamed_0 = 4;
pub const _IScntrl: C2RustUnnamed_0 = 2;
pub const _ISblank: C2RustUnnamed_0 = 1;
pub const _ISgraph: C2RustUnnamed_0 = 32768;
pub const _ISspace: C2RustUnnamed_0 = 8192;
pub const _ISxdigit: C2RustUnnamed_0 = 4096;
pub const _ISdigit: C2RustUnnamed_0 = 2048;
pub const _ISalpha: C2RustUnnamed_0 = 1024;
pub const _ISlower: C2RustUnnamed_0 = 512;
pub const _ISupper: C2RustUnnamed_0 = 256;
unsafe extern "C" fn node_lineinfo(mut p: *mut parser_state, mut node: *mut node) {
    if node.is_null() {
        return;
    }
    (*node).fname = (*p).fname;
    (*node).lineno = (*p).lineno;
}
static mut yytranslate: [yytype_int8; 302] = [
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
    60 as libc::c_int as yytype_int8,
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
    46 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    52 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    57 as libc::c_int as yytype_int8,
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
    59 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
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
    49 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
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
    48 as libc::c_int as yytype_int8,
];
static mut yyrline: [yytype_int16; 129] = [
    0 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    119 as libc::c_int as yytype_int16,
    123 as libc::c_int as yytype_int16,
    127 as libc::c_int as yytype_int16,
    133 as libc::c_int as yytype_int16,
    142 as libc::c_int as yytype_int16,
    163 as libc::c_int as yytype_int16,
    174 as libc::c_int as yytype_int16,
    182 as libc::c_int as yytype_int16,
    197 as libc::c_int as yytype_int16,
    209 as libc::c_int as yytype_int16,
    213 as libc::c_int as yytype_int16,
    216 as libc::c_int as yytype_int16,
    220 as libc::c_int as yytype_int16,
    224 as libc::c_int as yytype_int16,
    230 as libc::c_int as yytype_int16,
    239 as libc::c_int as yytype_int16,
    254 as libc::c_int as yytype_int16,
    264 as libc::c_int as yytype_int16,
    272 as libc::c_int as yytype_int16,
    280 as libc::c_int as yytype_int16,
    284 as libc::c_int as yytype_int16,
    288 as libc::c_int as yytype_int16,
    292 as libc::c_int as yytype_int16,
    296 as libc::c_int as yytype_int16,
    300 as libc::c_int as yytype_int16,
    303 as libc::c_int as yytype_int16,
    306 as libc::c_int as yytype_int16,
    307 as libc::c_int as yytype_int16,
    313 as libc::c_int as yytype_int16,
    317 as libc::c_int as yytype_int16,
    321 as libc::c_int as yytype_int16,
    325 as libc::c_int as yytype_int16,
    329 as libc::c_int as yytype_int16,
    333 as libc::c_int as yytype_int16,
    337 as libc::c_int as yytype_int16,
    341 as libc::c_int as yytype_int16,
    345 as libc::c_int as yytype_int16,
    349 as libc::c_int as yytype_int16,
    353 as libc::c_int as yytype_int16,
    357 as libc::c_int as yytype_int16,
    361 as libc::c_int as yytype_int16,
    365 as libc::c_int as yytype_int16,
    369 as libc::c_int as yytype_int16,
    373 as libc::c_int as yytype_int16,
    377 as libc::c_int as yytype_int16,
    381 as libc::c_int as yytype_int16,
    385 as libc::c_int as yytype_int16,
    389 as libc::c_int as yytype_int16,
    393 as libc::c_int as yytype_int16,
    397 as libc::c_int as yytype_int16,
    401 as libc::c_int as yytype_int16,
    404 as libc::c_int as yytype_int16,
    410 as libc::c_int as yytype_int16,
    414 as libc::c_int as yytype_int16,
    421 as libc::c_int as yytype_int16,
    424 as libc::c_int as yytype_int16,
    431 as libc::c_int as yytype_int16,
    432 as libc::c_int as yytype_int16,
    436 as libc::c_int as yytype_int16,
    442 as libc::c_int as yytype_int16,
    447 as libc::c_int as yytype_int16,
    454 as libc::c_int as yytype_int16,
    455 as libc::c_int as yytype_int16,
    456 as libc::c_int as yytype_int16,
    457 as libc::c_int as yytype_int16,
    458 as libc::c_int as yytype_int16,
    462 as libc::c_int as yytype_int16,
    472 as libc::c_int as yytype_int16,
    476 as libc::c_int as yytype_int16,
    480 as libc::c_int as yytype_int16,
    481 as libc::c_int as yytype_int16,
    485 as libc::c_int as yytype_int16,
    489 as libc::c_int as yytype_int16,
    493 as libc::c_int as yytype_int16,
    497 as libc::c_int as yytype_int16,
    501 as libc::c_int as yytype_int16,
    505 as libc::c_int as yytype_int16,
    509 as libc::c_int as yytype_int16,
    513 as libc::c_int as yytype_int16,
    517 as libc::c_int as yytype_int16,
    524 as libc::c_int as yytype_int16,
    527 as libc::c_int as yytype_int16,
    530 as libc::c_int as yytype_int16,
    534 as libc::c_int as yytype_int16,
    535 as libc::c_int as yytype_int16,
    536 as libc::c_int as yytype_int16,
    540 as libc::c_int as yytype_int16,
    544 as libc::c_int as yytype_int16,
    548 as libc::c_int as yytype_int16,
    552 as libc::c_int as yytype_int16,
    556 as libc::c_int as yytype_int16,
    560 as libc::c_int as yytype_int16,
    564 as libc::c_int as yytype_int16,
    570 as libc::c_int as yytype_int16,
    575 as libc::c_int as yytype_int16,
    582 as libc::c_int as yytype_int16,
    587 as libc::c_int as yytype_int16,
    594 as libc::c_int as yytype_int16,
    595 as libc::c_int as yytype_int16,
    599 as libc::c_int as yytype_int16,
    603 as libc::c_int as yytype_int16,
    607 as libc::c_int as yytype_int16,
    611 as libc::c_int as yytype_int16,
    612 as libc::c_int as yytype_int16,
    618 as libc::c_int as yytype_int16,
    622 as libc::c_int as yytype_int16,
    626 as libc::c_int as yytype_int16,
    630 as libc::c_int as yytype_int16,
    636 as libc::c_int as yytype_int16,
    640 as libc::c_int as yytype_int16,
    646 as libc::c_int as yytype_int16,
    650 as libc::c_int as yytype_int16,
    654 as libc::c_int as yytype_int16,
    658 as libc::c_int as yytype_int16,
    665 as libc::c_int as yytype_int16,
    669 as libc::c_int as yytype_int16,
    676 as libc::c_int as yytype_int16,
    679 as libc::c_int as yytype_int16,
    682 as libc::c_int as yytype_int16,
    687 as libc::c_int as yytype_int16,
    694 as libc::c_int as yytype_int16,
    695 as libc::c_int as yytype_int16,
    698 as libc::c_int as yytype_int16,
    699 as libc::c_int as yytype_int16,
    702 as libc::c_int as yytype_int16,
    703 as libc::c_int as yytype_int16,
];
static mut yytname: [*const libc::c_char; 93] = [
    b"$end\0" as *const u8 as *const libc::c_char,
    b"error\0" as *const u8 as *const libc::c_char,
    b"$undefined\0" as *const u8 as *const libc::c_char,
    b"keyword_if\0" as *const u8 as *const libc::c_char,
    b"keyword_else\0" as *const u8 as *const libc::c_char,
    b"keyword_case\0" as *const u8 as *const libc::c_char,
    b"keyword_emit\0" as *const u8 as *const libc::c_char,
    b"keyword_skip\0" as *const u8 as *const libc::c_char,
    b"keyword_return\0" as *const u8 as *const libc::c_char,
    b"keyword_namespace\0" as *const u8 as *const libc::c_char,
    b"keyword_class\0" as *const u8 as *const libc::c_char,
    b"keyword_import\0" as *const u8 as *const libc::c_char,
    b"keyword_def\0" as *const u8 as *const libc::c_char,
    b"keyword_method\0" as *const u8 as *const libc::c_char,
    b"keyword_new\0" as *const u8 as *const libc::c_char,
    b"keyword_nil\0" as *const u8 as *const libc::c_char,
    b"keyword_true\0" as *const u8 as *const libc::c_char,
    b"keyword_false\0" as *const u8 as *const libc::c_char,
    b"op_lasgn\0" as *const u8 as *const libc::c_char,
    b"op_rasgn\0" as *const u8 as *const libc::c_char,
    b"op_lambda\0" as *const u8 as *const libc::c_char,
    b"op_lambda2\0" as *const u8 as *const libc::c_char,
    b"op_lambda3\0" as *const u8 as *const libc::c_char,
    b"op_plus\0" as *const u8 as *const libc::c_char,
    b"op_minus\0" as *const u8 as *const libc::c_char,
    b"op_mult\0" as *const u8 as *const libc::c_char,
    b"op_div\0" as *const u8 as *const libc::c_char,
    b"op_mod\0" as *const u8 as *const libc::c_char,
    b"op_eq\0" as *const u8 as *const libc::c_char,
    b"op_neq\0" as *const u8 as *const libc::c_char,
    b"op_lt\0" as *const u8 as *const libc::c_char,
    b"op_le\0" as *const u8 as *const libc::c_char,
    b"op_gt\0" as *const u8 as *const libc::c_char,
    b"op_ge\0" as *const u8 as *const libc::c_char,
    b"op_and\0" as *const u8 as *const libc::c_char,
    b"op_or\0" as *const u8 as *const libc::c_char,
    b"op_bar\0" as *const u8 as *const libc::c_char,
    b"op_amper\0" as *const u8 as *const libc::c_char,
    b"op_colon2\0" as *const u8 as *const libc::c_char,
    b"lit_time\0" as *const u8 as *const libc::c_char,
    b"lit_number\0" as *const u8 as *const libc::c_char,
    b"lit_symbol\0" as *const u8 as *const libc::c_char,
    b"lit_string\0" as *const u8 as *const libc::c_char,
    b"identifier\0" as *const u8 as *const libc::c_char,
    b"label\0" as *const u8 as *const libc::c_char,
    b"op_LOWEST\0" as *const u8 as *const libc::c_char,
    b"'!'\0" as *const u8 as *const libc::c_char,
    b"'~'\0" as *const u8 as *const libc::c_char,
    b"op_HIGHEST\0" as *const u8 as *const libc::c_char,
    b"'{'\0" as *const u8 as *const libc::c_char,
    b"'}'\0" as *const u8 as *const libc::c_char,
    b"'('\0" as *const u8 as *const libc::c_char,
    b"')'\0" as *const u8 as *const libc::c_char,
    b"'='\0" as *const u8 as *const libc::c_char,
    b"','\0" as *const u8 as *const libc::c_char,
    b"'['\0" as *const u8 as *const libc::c_char,
    b"']'\0" as *const u8 as *const libc::c_char,
    b"'.'\0" as *const u8 as *const libc::c_char,
    b"'@'\0" as *const u8 as *const libc::c_char,
    b"';'\0" as *const u8 as *const libc::c_char,
    b"'\\n'\0" as *const u8 as *const libc::c_char,
    b"$accept\0" as *const u8 as *const libc::c_char,
    b"program\0" as *const u8 as *const libc::c_char,
    b"topstmts\0" as *const u8 as *const libc::c_char,
    b"topstmt_list\0" as *const u8 as *const libc::c_char,
    b"topstmt\0" as *const u8 as *const libc::c_char,
    b"stmts\0" as *const u8 as *const libc::c_char,
    b"stmt_list\0" as *const u8 as *const libc::c_char,
    b"stmt\0" as *const u8 as *const libc::c_char,
    b"var\0" as *const u8 as *const libc::c_char,
    b"fname\0" as *const u8 as *const libc::c_char,
    b"expr\0" as *const u8 as *const libc::c_char,
    b"condition\0" as *const u8 as *const libc::c_char,
    b"opt_else\0" as *const u8 as *const libc::c_char,
    b"opt_args\0" as *const u8 as *const libc::c_char,
    b"arg\0" as *const u8 as *const libc::c_char,
    b"args\0" as *const u8 as *const libc::c_char,
    b"primary\0" as *const u8 as *const libc::c_char,
    b"opt_block\0" as *const u8 as *const libc::c_char,
    b"pterm\0" as *const u8 as *const libc::c_char,
    b"pary\0" as *const u8 as *const libc::c_char,
    b"pstruct\0" as *const u8 as *const libc::c_char,
    b"pattern\0" as *const u8 as *const libc::c_char,
    b"cparam\0" as *const u8 as *const libc::c_char,
    b"case_body\0" as *const u8 as *const libc::c_char,
    b"block\0" as *const u8 as *const libc::c_char,
    b"bparam\0" as *const u8 as *const libc::c_char,
    b"opt_f_args\0" as *const u8 as *const libc::c_char,
    b"f_args\0" as *const u8 as *const libc::c_char,
    b"opt_terms\0" as *const u8 as *const libc::c_char,
    b"terms\0" as *const u8 as *const libc::c_char,
    b"term\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut yypact: [yytype_int16; 254] = [
    257 as libc::c_int as yytype_int16,
    -(46 as libc::c_int) as yytype_int16,
    457 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    457 as libc::c_int as yytype_int16,
    -(27 as libc::c_int) as yytype_int16,
    -(12 as libc::c_int) as yytype_int16,
    -(9 as libc::c_int) as yytype_int16,
    98 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    -(2 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    499 as libc::c_int as yytype_int16,
    499 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    -(23 as libc::c_int) as yytype_int16,
    499 as libc::c_int as yytype_int16,
    499 as libc::c_int as yytype_int16,
    315 as libc::c_int as yytype_int16,
    499 as libc::c_int as yytype_int16,
    412 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    56 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    126 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(30 as libc::c_int) as yytype_int16,
    82 as libc::c_int as yytype_int16,
    690 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    257 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    499 as libc::c_int as yytype_int16,
    499 as libc::c_int as yytype_int16,
    499 as libc::c_int as yytype_int16,
    499 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    741 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    16 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    33 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    111 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    71 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    540 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    41 as libc::c_int as yytype_int16,
    126 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(18 as libc::c_int) as yytype_int16,
    10 as libc::c_int as yytype_int16,
    365 as libc::c_int as yytype_int16,
    -(8 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    365 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    630 as libc::c_int as yytype_int16,
    136 as libc::c_int as yytype_int16,
    97 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    113 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    257 as libc::c_int as yytype_int16,
    499 as libc::c_int as yytype_int16,
    457 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    118 as libc::c_int as yytype_int16,
    499 as libc::c_int as yytype_int16,
    499 as libc::c_int as yytype_int16,
    499 as libc::c_int as yytype_int16,
    499 as libc::c_int as yytype_int16,
    499 as libc::c_int as yytype_int16,
    499 as libc::c_int as yytype_int16,
    499 as libc::c_int as yytype_int16,
    499 as libc::c_int as yytype_int16,
    499 as libc::c_int as yytype_int16,
    499 as libc::c_int as yytype_int16,
    499 as libc::c_int as yytype_int16,
    499 as libc::c_int as yytype_int16,
    499 as libc::c_int as yytype_int16,
    499 as libc::c_int as yytype_int16,
    499 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    126 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    660 as libc::c_int as yytype_int16,
    615 as libc::c_int as yytype_int16,
    741 as libc::c_int as yytype_int16,
    741 as libc::c_int as yytype_int16,
    457 as libc::c_int as yytype_int16,
    257 as libc::c_int as yytype_int16,
    257 as libc::c_int as yytype_int16,
    118 as libc::c_int as yytype_int16,
    499 as libc::c_int as yytype_int16,
    118 as libc::c_int as yytype_int16,
    457 as libc::c_int as yytype_int16,
    499 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    185 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    185 as libc::c_int as yytype_int16,
    546 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    108 as libc::c_int as yytype_int16,
    123 as libc::c_int as yytype_int16,
    134 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    365 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    365 as libc::c_int as yytype_int16,
    153 as libc::c_int as yytype_int16,
    540 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    140 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    118 as libc::c_int as yytype_int16,
    126 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    499 as libc::c_int as yytype_int16,
    365 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    741 as libc::c_int as yytype_int16,
    142 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    119 as libc::c_int as yytype_int16,
    119 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    317 as libc::c_int as yytype_int16,
    317 as libc::c_int as yytype_int16,
    157 as libc::c_int as yytype_int16,
    157 as libc::c_int as yytype_int16,
    157 as libc::c_int as yytype_int16,
    157 as libc::c_int as yytype_int16,
    367 as libc::c_int as yytype_int16,
    259 as libc::c_int as yytype_int16,
    756 as libc::c_int as yytype_int16,
    645 as libc::c_int as yytype_int16,
    457 as libc::c_int as yytype_int16,
    121 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    499 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    147 as libc::c_int as yytype_int16,
    149 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    151 as libc::c_int as yytype_int16,
    741 as libc::c_int as yytype_int16,
    158 as libc::c_int as yytype_int16,
    156 as libc::c_int as yytype_int16,
    708 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    108 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    166 as libc::c_int as yytype_int16,
    160 as libc::c_int as yytype_int16,
    170 as libc::c_int as yytype_int16,
    2 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    499 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    365 as libc::c_int as yytype_int16,
    365 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    741 as libc::c_int as yytype_int16,
    167 as libc::c_int as yytype_int16,
    143 as libc::c_int as yytype_int16,
    171 as libc::c_int as yytype_int16,
    457 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    741 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    20 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    185 as libc::c_int as yytype_int16,
    581 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    185 as libc::c_int as yytype_int16,
    108 as libc::c_int as yytype_int16,
    185 as libc::c_int as yytype_int16,
    185 as libc::c_int as yytype_int16,
    726 as libc::c_int as yytype_int16,
    169 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    143 as libc::c_int as yytype_int16,
    172 as libc::c_int as yytype_int16,
    365 as libc::c_int as yytype_int16,
    499 as libc::c_int as yytype_int16,
    365 as libc::c_int as yytype_int16,
    499 as libc::c_int as yytype_int16,
    168 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    174 as libc::c_int as yytype_int16,
    78 as libc::c_int as yytype_int16,
    108 as libc::c_int as yytype_int16,
    108 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    143 as libc::c_int as yytype_int16,
    176 as libc::c_int as yytype_int16,
    741 as libc::c_int as yytype_int16,
    182 as libc::c_int as yytype_int16,
    741 as libc::c_int as yytype_int16,
    185 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    185 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    168 as libc::c_int as yytype_int16,
];
unsafe extern "C" fn lex_return(
    mut p: *mut parser_state,
    mut c: libc::c_int,
) -> libc::c_int {
    (*p).lineno = (*p).tline;
    (*p).tline = yylineno;
    return c;
}
static mut yy_buffer_stack_top: size_t = 0 as libc::c_int as size_t;
static mut yy_buffer_stack_max: size_t = 0 as libc::c_int as size_t;
static mut yy_buffer_stack: *mut YY_BUFFER_STATE = 0 as *const YY_BUFFER_STATE
    as *mut YY_BUFFER_STATE;
static mut yy_hold_char: libc::c_char = 0;
static mut yy_n_chars: libc::c_int = 0;
pub static mut yyleng: libc::c_int = 0;
static mut yy_c_buf_p: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub unsafe extern "C" fn yylex(
    mut lval: *mut YYSTYPE,
    mut p: *mut parser_state,
) -> libc::c_int {
    let mut yy_amount_of_matched_text: libc::c_int = 0;
    let mut yy_next_state: yy_state_type = 0;
    let mut current_block: u64;
    let mut yy_current_state: yy_state_type = 0;
    let mut yy_cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut yy_bp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut yy_act: libc::c_int = 0;
    if yy_init == 0 {
        yy_init = 1 as libc::c_int;
        if yy_start == 0 {
            yy_start = 1 as libc::c_int;
        }
        if yyin.is_null() {
            yyin = stdin;
        }
        if yyout.is_null() {
            yyout = stdout;
        }
        if if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else {
            0 as YY_BUFFER_STATE
        }
            .is_null()
        {
            yyensure_buffer_stack();
            let ref mut fresh0 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
            *fresh0 = yy_create_buffer(yyin, 16384 as libc::c_int);
        }
        yy_load_buffer_state();
    }
    loop {
        yy_cp = yy_c_buf_p;
        *yy_cp = yy_hold_char;
        yy_bp = yy_cp;
        yy_current_state = yy_start;
        '_yy_match: loop {
            loop {
                let mut yy_c: YY_CHAR = yy_ec[*yy_cp as YY_CHAR as usize];
                if yy_accept[yy_current_state as usize] != 0 {
                    yy_last_accepting_state = yy_current_state;
                    yy_last_accepting_cpos = yy_cp;
                }
                while yy_chk[(yy_base[yy_current_state as usize] as libc::c_int
                    + yy_c as libc::c_int) as usize] as libc::c_int != yy_current_state
                {
                    yy_current_state = yy_def[yy_current_state as usize] as libc::c_int;
                    if yy_current_state >= 290 as libc::c_int {
                        yy_c = yy_meta[yy_c as usize];
                    }
                }
                yy_current_state = yy_nxt[(yy_base[yy_current_state as usize]
                    as libc::c_int + yy_c as libc::c_int) as usize] as yy_state_type;
                yy_cp = yy_cp.offset(1);
                yy_cp;
                if !(yy_base[yy_current_state as usize] as libc::c_int
                    != 953 as libc::c_int)
                {
                    break;
                }
            }
            '_yy_find_action: loop {
                yy_act = yy_accept[yy_current_state as usize] as libc::c_int;
                if yy_act == 0 as libc::c_int {
                    yy_cp = yy_last_accepting_cpos;
                    yy_current_state = yy_last_accepting_state;
                    yy_act = yy_accept[yy_current_state as usize] as libc::c_int;
                }
                yytext = yy_bp;
                yyleng = yy_cp.offset_from(yy_bp) as libc::c_long as libc::c_int;
                yy_hold_char = *yy_cp;
                *yy_cp = '\0' as i32 as libc::c_char;
                yy_c_buf_p = yy_cp;
                if yy_act != 56 as libc::c_int
                    && yy_rule_can_match_eol[yy_act as usize] != 0
                {
                    let mut yyl: libc::c_int = 0;
                    yyl = 0 as libc::c_int;
                    while yyl < yyleng {
                        if *yytext.offset(yyl as isize) as libc::c_int == '\n' as i32 {
                            yylineno += 1;
                            yylineno;
                        }
                        yyl += 1;
                        yyl;
                    }
                }
                loop {
                    match yy_act {
                        0 => {
                            *yy_cp = yy_hold_char;
                            yy_cp = yy_last_accepting_cpos;
                            yy_current_state = yy_last_accepting_state;
                            continue '_yy_find_action;
                        }
                        1 => return lex_return(p, 278 as libc::c_int),
                        2 => return lex_return(p, 279 as libc::c_int),
                        3 => return lex_return(p, 280 as libc::c_int),
                        4 => return lex_return(p, 281 as libc::c_int),
                        5 => return lex_return(p, 282 as libc::c_int),
                        6 => return lex_return(p, 283 as libc::c_int),
                        7 => return lex_return(p, 284 as libc::c_int),
                        8 => return lex_return(p, 285 as libc::c_int),
                        9 => return lex_return(p, 286 as libc::c_int),
                        10 => return lex_return(p, 287 as libc::c_int),
                        11 => return lex_return(p, 288 as libc::c_int),
                        12 => return lex_return(p, 289 as libc::c_int),
                        13 => return lex_return(p, 290 as libc::c_int),
                        14 => return lex_return(p, 292 as libc::c_int),
                        15 => return lex_return(p, 273 as libc::c_int),
                        16 => return lex_return(p, 274 as libc::c_int),
                        17 => return lex_return(p, 275 as libc::c_int),
                        18 => return lex_return(p, 276 as libc::c_int),
                        19 => return lex_return(p, 277 as libc::c_int),
                        20 => return lex_return(p, '=' as i32),
                        21 => return lex_return(p, 293 as libc::c_int),
                        22 => return lex_return(p, 258 as libc::c_int),
                        23 => return lex_return(p, 259 as libc::c_int),
                        24 => return lex_return(p, 262 as libc::c_int),
                        25 => return lex_return(p, 260 as libc::c_int),
                        26 => return lex_return(p, 261 as libc::c_int),
                        27 => return lex_return(p, 263 as libc::c_int),
                        28 => return lex_return(p, 264 as libc::c_int),
                        29 => return lex_return(p, 265 as libc::c_int),
                        30 => return lex_return(p, 266 as libc::c_int),
                        31 => return lex_return(p, 267 as libc::c_int),
                        32 => return lex_return(p, 268 as libc::c_int),
                        33 => return lex_return(p, 269 as libc::c_int),
                        34 => return lex_return(p, 270 as libc::c_int),
                        35 => return lex_return(p, 271 as libc::c_int),
                        36 => return lex_return(p, 272 as libc::c_int),
                        37 => {
                            (*lval).id = node_str_new(yytext, yyleng);
                            return lex_return(p, 298 as libc::c_int);
                        }
                        38 => {
                            *yytext
                                .offset(
                                    (yyleng - 1 as libc::c_int) as isize,
                                ) = '\0' as i32 as libc::c_char;
                            (*lval).id = node_str_new(yytext, yyleng - 1 as libc::c_int);
                            return lex_return(p, 299 as libc::c_int);
                        }
                        39 => return lex_return(p, 291 as libc::c_int),
                        40 => return lex_return(p, '.' as i32),
                        41 => {
                            return lex_return(
                                p,
                                *yytext.offset(0 as libc::c_int as isize) as libc::c_int,
                            );
                        }
                        42 => {
                            return lex_return(
                                p,
                                *yytext.offset(0 as libc::c_int as isize) as libc::c_int,
                            );
                        }
                        43 => return lex_return(p, '\n' as i32),
                        44 => return lex_return(p, '\n' as i32),
                        45 => {
                            (*lval).nd = node_int_new(atol(yytext));
                            return lex_return(p, 295 as libc::c_int);
                        }
                        46 => {
                            let mut temp: libc::c_double = 0.;
                            sscanf(
                                yytext,
                                b"%lf\0" as *const u8 as *const libc::c_char,
                                &mut temp as *mut libc::c_double,
                            );
                            (*lval).nd = node_float_new(temp);
                            return lex_return(p, 295 as libc::c_int);
                        }
                        47 => {
                            let mut temp_0: libc::c_ulong = 0;
                            sscanf(
                                yytext.offset(2 as libc::c_int as isize),
                                b"%lx\0" as *const u8 as *const libc::c_char,
                                &mut temp_0 as *mut libc::c_ulong,
                            );
                            (*lval).nd = node_int_new(temp_0 as libc::c_long);
                            return lex_return(p, 295 as libc::c_int);
                        }
                        48 => {
                            let mut temp_1: libc::c_ulong = 0;
                            sscanf(
                                yytext.offset(2 as libc::c_int as isize),
                                b"%lo\0" as *const u8 as *const libc::c_char,
                                &mut temp_1 as *mut libc::c_ulong,
                            );
                            (*lval).nd = node_int_new(temp_1 as libc::c_long);
                            return lex_return(p, 295 as libc::c_int);
                        }
                        49 => {
                            (*lval).nd = node_time_new(yytext, yyleng);
                            if ((*lval).nd).is_null() {
                                yyerror(
                                    p,
                                    b"bad time format\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            return lex_return(p, 294 as libc::c_int);
                        }
                        50 => {
                            (*lval)
                                .nd = node_string_new(
                                yytext.offset(1 as libc::c_int as isize),
                                yyleng - 2 as libc::c_int,
                            );
                            return lex_return(p, 297 as libc::c_int);
                        }
                        51 => {
                            (*lval)
                                .nd = node_string_new(
                                yytext.offset(1 as libc::c_int as isize),
                                yyleng - 1 as libc::c_int,
                            );
                            return lex_return(p, 296 as libc::c_int);
                        }
                        52 => {
                            (*lval)
                                .id = node_str_escaped(
                                yytext.offset(1 as libc::c_int as isize),
                                yyleng - 3 as libc::c_int,
                            );
                            return lex_return(p, 299 as libc::c_int);
                        }
                        53 => {
                            break '_yy_match;
                        }
                        54 => {
                            let mut c: libc::c_char = *yytext
                                .offset(0 as libc::c_int as isize);
                            fprintf(
                                stderr,
                                b"%s:%d:lexical error\0" as *const u8
                                    as *const libc::c_char,
                                (*p).fname,
                                yylineno,
                            );
                            if c as libc::c_int & 0x80 as libc::c_int != 0
                                || *(*__ctype_b_loc()).offset(c as libc::c_int as isize)
                                    as libc::c_int
                                    & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
                                    == 0
                            {
                                fprintf(
                                    stderr,
                                    b"('\\%03o').\n\0" as *const u8 as *const libc::c_char,
                                    c as libc::c_int,
                                );
                            } else if c as libc::c_int == '\\' as i32 {
                                fprintf(
                                    stderr,
                                    b"('\\\\').\n\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                fprintf(
                                    stderr,
                                    b"('%c').\n\0" as *const u8 as *const libc::c_char,
                                    c as libc::c_int,
                                );
                            }
                            exit(1 as libc::c_int);
                        }
                        55 => {
                            fwrite(
                                yytext as *const libc::c_void,
                                yyleng as size_t,
                                1 as libc::c_int as libc::c_ulong,
                                yyout,
                            ) != 0;
                            break '_yy_match;
                        }
                        57 => return 0 as libc::c_int,
                        56 => {
                            yy_amount_of_matched_text = yy_cp.offset_from(yytext)
                                as libc::c_long as libc::c_int - 1 as libc::c_int;
                            *yy_cp = yy_hold_char;
                            if (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                .yy_buffer_status == 0 as libc::c_int
                            {
                                yy_n_chars = (**yy_buffer_stack
                                    .offset(yy_buffer_stack_top as isize))
                                    .yy_n_chars;
                                let ref mut fresh1 = (**yy_buffer_stack
                                    .offset(yy_buffer_stack_top as isize))
                                    .yy_input_file;
                                *fresh1 = yyin;
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_buffer_status = 1 as libc::c_int;
                            }
                            if yy_c_buf_p
                                <= &mut *((**yy_buffer_stack
                                    .offset(yy_buffer_stack_top as isize))
                                    .yy_ch_buf)
                                    .offset(yy_n_chars as isize) as *mut libc::c_char
                            {
                                yy_next_state = 0;
                                yy_c_buf_p = yytext
                                    .offset(yy_amount_of_matched_text as isize);
                                yy_current_state = yy_get_previous_state();
                                yy_next_state = yy_try_NUL_trans(yy_current_state);
                                yy_bp = yytext.offset(0 as libc::c_int as isize);
                                if yy_next_state != 0 {
                                    current_block = 18323319510560553714;
                                    break;
                                } else {
                                    current_block = 12129449210080749085;
                                    break;
                                }
                            } else {
                                match yy_get_next_buffer() {
                                    1 => {
                                        yy_did_buffer_switch_on_eof = 0 as libc::c_int;
                                        yy_c_buf_p = yytext.offset(0 as libc::c_int as isize);
                                        yy_act = 56 as libc::c_int
                                            + (yy_start - 1 as libc::c_int) / 2 as libc::c_int
                                            + 1 as libc::c_int;
                                    }
                                    0 => {
                                        yy_c_buf_p = yytext
                                            .offset(yy_amount_of_matched_text as isize);
                                        yy_current_state = yy_get_previous_state();
                                        yy_cp = yy_c_buf_p;
                                        yy_bp = yytext.offset(0 as libc::c_int as isize);
                                        break '_yy_find_action;
                                    }
                                    2 => {
                                        yy_c_buf_p = &mut *((**yy_buffer_stack
                                            .offset(yy_buffer_stack_top as isize))
                                            .yy_ch_buf)
                                            .offset(yy_n_chars as isize) as *mut libc::c_char;
                                        yy_current_state = yy_get_previous_state();
                                        yy_cp = yy_c_buf_p;
                                        yy_bp = yytext.offset(0 as libc::c_int as isize);
                                        continue '_yy_find_action;
                                    }
                                    _ => {
                                        break '_yy_match;
                                    }
                                }
                            }
                        }
                        _ => {
                            yy_fatal_error(
                                b"fatal flex scanner internal error--no action found\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                    }
                }
                match current_block {
                    12129449210080749085 => {
                        yy_cp = yy_c_buf_p;
                    }
                    _ => {
                        yy_c_buf_p = yy_c_buf_p.offset(1);
                        yy_cp = yy_c_buf_p;
                        yy_current_state = yy_next_state;
                        break;
                    }
                }
            }
        }
    };
}
unsafe extern "C" fn yy_fatal_error(mut msg: *const libc::c_char) -> ! {
    fprintf(stderr, b"%s\n\0" as *const u8 as *const libc::c_char, msg);
    exit(2 as libc::c_int);
}
static mut yy_accept: [flex_int16_t; 290] = [
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    56 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    53 as libc::c_int as flex_int16_t,
    43 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    5 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    41 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    40 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    45 as libc::c_int as flex_int16_t,
    45 as libc::c_int as flex_int16_t,
    41 as libc::c_int as flex_int16_t,
    8 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    10 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    39 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    40 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    39 as libc::c_int as flex_int16_t,
    7 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    50 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    44 as libc::c_int as flex_int16_t,
    5 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    12 as libc::c_int as flex_int16_t,
    41 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    17 as libc::c_int as flex_int16_t,
    40 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    45 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    51 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    8 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    15 as libc::c_int as flex_int16_t,
    9 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    6 as libc::c_int as flex_int16_t,
    16 as libc::c_int as flex_int16_t,
    10 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    11 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    38 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    39 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    13 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    7 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    52 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    12 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    18 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    17 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    46 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    48 as libc::c_int as flex_int16_t,
    47 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    51 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    15 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    9 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    6 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    16 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    11 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    31 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    33 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    13 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    18 as libc::c_int as flex_int16_t,
    18 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    19 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    46 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    24 as libc::c_int as flex_int16_t,
    35 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    19 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    49 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    29 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    36 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    24 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    30 as libc::c_int as flex_int16_t,
    32 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    49 as libc::c_int as flex_int16_t,
    28 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    49 as libc::c_int as flex_int16_t,
    49 as libc::c_int as flex_int16_t,
    49 as libc::c_int as flex_int16_t,
    49 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    49 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
];
pub static mut yytext: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
unsafe extern "C" fn yy_get_previous_state() -> yy_state_type {
    let mut yy_current_state: yy_state_type = 0;
    let mut yy_cp: *mut libc::c_char = 0 as *mut libc::c_char;
    yy_current_state = yy_start;
    yy_cp = yytext.offset(0 as libc::c_int as isize);
    while yy_cp < yy_c_buf_p {
        let mut yy_c: YY_CHAR = (if *yy_cp as libc::c_int != 0 {
            yy_ec[*yy_cp as YY_CHAR as usize] as libc::c_int
        } else {
            1 as libc::c_int
        }) as YY_CHAR;
        if yy_accept[yy_current_state as usize] != 0 {
            yy_last_accepting_state = yy_current_state;
            yy_last_accepting_cpos = yy_cp;
        }
        while yy_chk[(yy_base[yy_current_state as usize] as libc::c_int
            + yy_c as libc::c_int) as usize] as libc::c_int != yy_current_state
        {
            yy_current_state = yy_def[yy_current_state as usize] as libc::c_int;
            if yy_current_state >= 290 as libc::c_int {
                yy_c = yy_meta[yy_c as usize];
            }
        }
        yy_current_state = yy_nxt[(yy_base[yy_current_state as usize] as libc::c_int
            + yy_c as libc::c_int) as usize] as yy_state_type;
        yy_cp = yy_cp.offset(1);
        yy_cp;
    }
    return yy_current_state;
}
static mut yy_ec: [YY_CHAR; 256] = [
    0 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    2 as libc::c_int as YY_CHAR,
    3 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    5 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    7 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    8 as libc::c_int as YY_CHAR,
    9 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    10 as libc::c_int as YY_CHAR,
    11 as libc::c_int as YY_CHAR,
    12 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    10 as libc::c_int as YY_CHAR,
    14 as libc::c_int as YY_CHAR,
    15 as libc::c_int as YY_CHAR,
    16 as libc::c_int as YY_CHAR,
    17 as libc::c_int as YY_CHAR,
    18 as libc::c_int as YY_CHAR,
    18 as libc::c_int as YY_CHAR,
    18 as libc::c_int as YY_CHAR,
    18 as libc::c_int as YY_CHAR,
    18 as libc::c_int as YY_CHAR,
    18 as libc::c_int as YY_CHAR,
    18 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    20 as libc::c_int as YY_CHAR,
    10 as libc::c_int as YY_CHAR,
    21 as libc::c_int as YY_CHAR,
    22 as libc::c_int as YY_CHAR,
    23 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    24 as libc::c_int as YY_CHAR,
    25 as libc::c_int as YY_CHAR,
    25 as libc::c_int as YY_CHAR,
    25 as libc::c_int as YY_CHAR,
    25 as libc::c_int as YY_CHAR,
    25 as libc::c_int as YY_CHAR,
    25 as libc::c_int as YY_CHAR,
    26 as libc::c_int as YY_CHAR,
    26 as libc::c_int as YY_CHAR,
    26 as libc::c_int as YY_CHAR,
    26 as libc::c_int as YY_CHAR,
    26 as libc::c_int as YY_CHAR,
    26 as libc::c_int as YY_CHAR,
    26 as libc::c_int as YY_CHAR,
    26 as libc::c_int as YY_CHAR,
    26 as libc::c_int as YY_CHAR,
    26 as libc::c_int as YY_CHAR,
    26 as libc::c_int as YY_CHAR,
    26 as libc::c_int as YY_CHAR,
    26 as libc::c_int as YY_CHAR,
    27 as libc::c_int as YY_CHAR,
    26 as libc::c_int as YY_CHAR,
    26 as libc::c_int as YY_CHAR,
    26 as libc::c_int as YY_CHAR,
    26 as libc::c_int as YY_CHAR,
    26 as libc::c_int as YY_CHAR,
    28 as libc::c_int as YY_CHAR,
    10 as libc::c_int as YY_CHAR,
    29 as libc::c_int as YY_CHAR,
    24 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    26 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    30 as libc::c_int as YY_CHAR,
    25 as libc::c_int as YY_CHAR,
    31 as libc::c_int as YY_CHAR,
    32 as libc::c_int as YY_CHAR,
    33 as libc::c_int as YY_CHAR,
    34 as libc::c_int as YY_CHAR,
    26 as libc::c_int as YY_CHAR,
    35 as libc::c_int as YY_CHAR,
    36 as libc::c_int as YY_CHAR,
    26 as libc::c_int as YY_CHAR,
    37 as libc::c_int as YY_CHAR,
    38 as libc::c_int as YY_CHAR,
    39 as libc::c_int as YY_CHAR,
    40 as libc::c_int as YY_CHAR,
    41 as libc::c_int as YY_CHAR,
    42 as libc::c_int as YY_CHAR,
    26 as libc::c_int as YY_CHAR,
    43 as libc::c_int as YY_CHAR,
    44 as libc::c_int as YY_CHAR,
    45 as libc::c_int as YY_CHAR,
    46 as libc::c_int as YY_CHAR,
    26 as libc::c_int as YY_CHAR,
    47 as libc::c_int as YY_CHAR,
    48 as libc::c_int as YY_CHAR,
    26 as libc::c_int as YY_CHAR,
    26 as libc::c_int as YY_CHAR,
    49 as libc::c_int as YY_CHAR,
    50 as libc::c_int as YY_CHAR,
    24 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    52 as libc::c_int as YY_CHAR,
    52 as libc::c_int as YY_CHAR,
    52 as libc::c_int as YY_CHAR,
    52 as libc::c_int as YY_CHAR,
    52 as libc::c_int as YY_CHAR,
    52 as libc::c_int as YY_CHAR,
    52 as libc::c_int as YY_CHAR,
    52 as libc::c_int as YY_CHAR,
    52 as libc::c_int as YY_CHAR,
    52 as libc::c_int as YY_CHAR,
    52 as libc::c_int as YY_CHAR,
    52 as libc::c_int as YY_CHAR,
    52 as libc::c_int as YY_CHAR,
    52 as libc::c_int as YY_CHAR,
    52 as libc::c_int as YY_CHAR,
    52 as libc::c_int as YY_CHAR,
    52 as libc::c_int as YY_CHAR,
    52 as libc::c_int as YY_CHAR,
    52 as libc::c_int as YY_CHAR,
    52 as libc::c_int as YY_CHAR,
    52 as libc::c_int as YY_CHAR,
    52 as libc::c_int as YY_CHAR,
    52 as libc::c_int as YY_CHAR,
    52 as libc::c_int as YY_CHAR,
    52 as libc::c_int as YY_CHAR,
    52 as libc::c_int as YY_CHAR,
    52 as libc::c_int as YY_CHAR,
    52 as libc::c_int as YY_CHAR,
    52 as libc::c_int as YY_CHAR,
    52 as libc::c_int as YY_CHAR,
    53 as libc::c_int as YY_CHAR,
    53 as libc::c_int as YY_CHAR,
    53 as libc::c_int as YY_CHAR,
    53 as libc::c_int as YY_CHAR,
    53 as libc::c_int as YY_CHAR,
    53 as libc::c_int as YY_CHAR,
    53 as libc::c_int as YY_CHAR,
    53 as libc::c_int as YY_CHAR,
    53 as libc::c_int as YY_CHAR,
    53 as libc::c_int as YY_CHAR,
    53 as libc::c_int as YY_CHAR,
    53 as libc::c_int as YY_CHAR,
    53 as libc::c_int as YY_CHAR,
    53 as libc::c_int as YY_CHAR,
    53 as libc::c_int as YY_CHAR,
    53 as libc::c_int as YY_CHAR,
    54 as libc::c_int as YY_CHAR,
    54 as libc::c_int as YY_CHAR,
    54 as libc::c_int as YY_CHAR,
    54 as libc::c_int as YY_CHAR,
    54 as libc::c_int as YY_CHAR,
    54 as libc::c_int as YY_CHAR,
    54 as libc::c_int as YY_CHAR,
    54 as libc::c_int as YY_CHAR,
    55 as libc::c_int as YY_CHAR,
    55 as libc::c_int as YY_CHAR,
    55 as libc::c_int as YY_CHAR,
    55 as libc::c_int as YY_CHAR,
    56 as libc::c_int as YY_CHAR,
    56 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
];
static mut yy_base: [flex_int16_t; 324] = [
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    952 as libc::c_int as flex_int16_t,
    953 as libc::c_int as flex_int16_t,
    55 as libc::c_int as flex_int16_t,
    61 as libc::c_int as flex_int16_t,
    929 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    947 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    77 as libc::c_int as flex_int16_t,
    88 as libc::c_int as flex_int16_t,
    57 as libc::c_int as flex_int16_t,
    94 as libc::c_int as flex_int16_t,
    100 as libc::c_int as flex_int16_t,
    106 as libc::c_int as flex_int16_t,
    112 as libc::c_int as flex_int16_t,
    118 as libc::c_int as flex_int16_t,
    109 as libc::c_int as flex_int16_t,
    115 as libc::c_int as flex_int16_t,
    156 as libc::c_int as flex_int16_t,
    133 as libc::c_int as flex_int16_t,
    139 as libc::c_int as flex_int16_t,
    149 as libc::c_int as flex_int16_t,
    953 as libc::c_int as flex_int16_t,
    125 as libc::c_int as flex_int16_t,
    134 as libc::c_int as flex_int16_t,
    135 as libc::c_int as flex_int16_t,
    136 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    161 as libc::c_int as flex_int16_t,
    163 as libc::c_int as flex_int16_t,
    183 as libc::c_int as flex_int16_t,
    164 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    177 as libc::c_int as flex_int16_t,
    220 as libc::c_int as flex_int16_t,
    898 as libc::c_int as flex_int16_t,
    897 as libc::c_int as flex_int16_t,
    896 as libc::c_int as flex_int16_t,
    895 as libc::c_int as flex_int16_t,
    894 as libc::c_int as flex_int16_t,
    226 as libc::c_int as flex_int16_t,
    941 as libc::c_int as flex_int16_t,
    232 as libc::c_int as flex_int16_t,
    905 as libc::c_int as flex_int16_t,
    240 as libc::c_int as flex_int16_t,
    246 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    922 as libc::c_int as flex_int16_t,
    938 as libc::c_int as flex_int16_t,
    937 as libc::c_int as flex_int16_t,
    253 as libc::c_int as flex_int16_t,
    259 as libc::c_int as flex_int16_t,
    936 as libc::c_int as flex_int16_t,
    270 as libc::c_int as flex_int16_t,
    935 as libc::c_int as flex_int16_t,
    276 as libc::c_int as flex_int16_t,
    285 as libc::c_int as flex_int16_t,
    818 as libc::c_int as flex_int16_t,
    178 as libc::c_int as flex_int16_t,
    795 as libc::c_int as flex_int16_t,
    291 as libc::c_int as flex_int16_t,
    752 as libc::c_int as flex_int16_t,
    297 as libc::c_int as flex_int16_t,
    730 as libc::c_int as flex_int16_t,
    303 as libc::c_int as flex_int16_t,
    726 as libc::c_int as flex_int16_t,
    309 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    719 as libc::c_int as flex_int16_t,
    321 as libc::c_int as flex_int16_t,
    718 as libc::c_int as flex_int16_t,
    166 as libc::c_int as flex_int16_t,
    312 as libc::c_int as flex_int16_t,
    82 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    335 as libc::c_int as flex_int16_t,
    291 as libc::c_int as flex_int16_t,
    643 as libc::c_int as flex_int16_t,
    640 as libc::c_int as flex_int16_t,
    638 as libc::c_int as flex_int16_t,
    627 as libc::c_int as flex_int16_t,
    625 as libc::c_int as flex_int16_t,
    346 as libc::c_int as flex_int16_t,
    671 as libc::c_int as flex_int16_t,
    352 as libc::c_int as flex_int16_t,
    358 as libc::c_int as flex_int16_t,
    364 as libc::c_int as flex_int16_t,
    669 as libc::c_int as flex_int16_t,
    370 as libc::c_int as flex_int16_t,
    376 as libc::c_int as flex_int16_t,
    382 as libc::c_int as flex_int16_t,
    632 as libc::c_int as flex_int16_t,
    388 as libc::c_int as flex_int16_t,
    55 as libc::c_int as flex_int16_t,
    953 as libc::c_int as flex_int16_t,
    583 as libc::c_int as flex_int16_t,
    577 as libc::c_int as flex_int16_t,
    564 as libc::c_int as flex_int16_t,
    561 as libc::c_int as flex_int16_t,
    560 as libc::c_int as flex_int16_t,
    187 as libc::c_int as flex_int16_t,
    168 as libc::c_int as flex_int16_t,
    237 as libc::c_int as flex_int16_t,
    188 as libc::c_int as flex_int16_t,
    216 as libc::c_int as flex_int16_t,
    207 as libc::c_int as flex_int16_t,
    252 as libc::c_int as flex_int16_t,
    358 as libc::c_int as flex_int16_t,
    363 as libc::c_int as flex_int16_t,
    364 as libc::c_int as flex_int16_t,
    367 as libc::c_int as flex_int16_t,
    303 as libc::c_int as flex_int16_t,
    368 as libc::c_int as flex_int16_t,
    374 as libc::c_int as flex_int16_t,
    369 as libc::c_int as flex_int16_t,
    414 as libc::c_int as flex_int16_t,
    603 as libc::c_int as flex_int16_t,
    420 as libc::c_int as flex_int16_t,
    381 as libc::c_int as flex_int16_t,
    554 as libc::c_int as flex_int16_t,
    549 as libc::c_int as flex_int16_t,
    532 as libc::c_int as flex_int16_t,
    520 as libc::c_int as flex_int16_t,
    562 as libc::c_int as flex_int16_t,
    520 as libc::c_int as flex_int16_t,
    426 as libc::c_int as flex_int16_t,
    556 as libc::c_int as flex_int16_t,
    953 as libc::c_int as flex_int16_t,
    555 as libc::c_int as flex_int16_t,
    550 as libc::c_int as flex_int16_t,
    432 as libc::c_int as flex_int16_t,
    544 as libc::c_int as flex_int16_t,
    538 as libc::c_int as flex_int16_t,
    438 as libc::c_int as flex_int16_t,
    532 as libc::c_int as flex_int16_t,
    527 as libc::c_int as flex_int16_t,
    510 as libc::c_int as flex_int16_t,
    444 as libc::c_int as flex_int16_t,
    507 as libc::c_int as flex_int16_t,
    497 as libc::c_int as flex_int16_t,
    495 as libc::c_int as flex_int16_t,
    435 as libc::c_int as flex_int16_t,
    186 as libc::c_int as flex_int16_t,
    100 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    453 as libc::c_int as flex_int16_t,
    485 as libc::c_int as flex_int16_t,
    409 as libc::c_int as flex_int16_t,
    431 as libc::c_int as flex_int16_t,
    430 as libc::c_int as flex_int16_t,
    425 as libc::c_int as flex_int16_t,
    424 as libc::c_int as flex_int16_t,
    419 as libc::c_int as flex_int16_t,
    418 as libc::c_int as flex_int16_t,
    408 as libc::c_int as flex_int16_t,
    407 as libc::c_int as flex_int16_t,
    398 as libc::c_int as flex_int16_t,
    441 as libc::c_int as flex_int16_t,
    464 as libc::c_int as flex_int16_t,
    440 as libc::c_int as flex_int16_t,
    470 as libc::c_int as flex_int16_t,
    435 as libc::c_int as flex_int16_t,
    434 as libc::c_int as flex_int16_t,
    476 as libc::c_int as flex_int16_t,
    429 as libc::c_int as flex_int16_t,
    482 as libc::c_int as flex_int16_t,
    428 as libc::c_int as flex_int16_t,
    423 as libc::c_int as flex_int16_t,
    488 as libc::c_int as flex_int16_t,
    422 as libc::c_int as flex_int16_t,
    369 as libc::c_int as flex_int16_t,
    358 as libc::c_int as flex_int16_t,
    336 as libc::c_int as flex_int16_t,
    331 as libc::c_int as flex_int16_t,
    463 as libc::c_int as flex_int16_t,
    464 as libc::c_int as flex_int16_t,
    467 as libc::c_int as flex_int16_t,
    473 as libc::c_int as flex_int16_t,
    469 as libc::c_int as flex_int16_t,
    474 as libc::c_int as flex_int16_t,
    479 as libc::c_int as flex_int16_t,
    486 as libc::c_int as flex_int16_t,
    489 as libc::c_int as flex_int16_t,
    485 as libc::c_int as flex_int16_t,
    495 as libc::c_int as flex_int16_t,
    496 as libc::c_int as flex_int16_t,
    498 as libc::c_int as flex_int16_t,
    499 as libc::c_int as flex_int16_t,
    378 as libc::c_int as flex_int16_t,
    541 as libc::c_int as flex_int16_t,
    373 as libc::c_int as flex_int16_t,
    324 as libc::c_int as flex_int16_t,
    319 as libc::c_int as flex_int16_t,
    336 as libc::c_int as flex_int16_t,
    361 as libc::c_int as flex_int16_t,
    360 as libc::c_int as flex_int16_t,
    547 as libc::c_int as flex_int16_t,
    553 as libc::c_int as flex_int16_t,
    355 as libc::c_int as flex_int16_t,
    559 as libc::c_int as flex_int16_t,
    354 as libc::c_int as flex_int16_t,
    550 as libc::c_int as flex_int16_t,
    555 as libc::c_int as flex_int16_t,
    560 as libc::c_int as flex_int16_t,
    349 as libc::c_int as flex_int16_t,
    300 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    275 as libc::c_int as flex_int16_t,
    270 as libc::c_int as flex_int16_t,
    269 as libc::c_int as flex_int16_t,
    264 as libc::c_int as flex_int16_t,
    311 as libc::c_int as flex_int16_t,
    306 as libc::c_int as flex_int16_t,
    305 as libc::c_int as flex_int16_t,
    299 as libc::c_int as flex_int16_t,
    294 as libc::c_int as flex_int16_t,
    245 as libc::c_int as flex_int16_t,
    240 as libc::c_int as flex_int16_t,
    578 as libc::c_int as flex_int16_t,
    508 as libc::c_int as flex_int16_t,
    584 as libc::c_int as flex_int16_t,
    501 as libc::c_int as flex_int16_t,
    559 as libc::c_int as flex_int16_t,
    560 as libc::c_int as flex_int16_t,
    563 as libc::c_int as flex_int16_t,
    569 as libc::c_int as flex_int16_t,
    580 as libc::c_int as flex_int16_t,
    617 as libc::c_int as flex_int16_t,
    546 as libc::c_int as flex_int16_t,
    282 as libc::c_int as flex_int16_t,
    233 as libc::c_int as flex_int16_t,
    230 as libc::c_int as flex_int16_t,
    623 as libc::c_int as flex_int16_t,
    266 as libc::c_int as flex_int16_t,
    629 as libc::c_int as flex_int16_t,
    262 as libc::c_int as flex_int16_t,
    621 as libc::c_int as flex_int16_t,
    213 as libc::c_int as flex_int16_t,
    207 as libc::c_int as flex_int16_t,
    200 as libc::c_int as flex_int16_t,
    189 as libc::c_int as flex_int16_t,
    187 as libc::c_int as flex_int16_t,
    163 as libc::c_int as flex_int16_t,
    639 as libc::c_int as flex_int16_t,
    167 as libc::c_int as flex_int16_t,
    579 as libc::c_int as flex_int16_t,
    647 as libc::c_int as flex_int16_t,
    166 as libc::c_int as flex_int16_t,
    599 as libc::c_int as flex_int16_t,
    614 as libc::c_int as flex_int16_t,
    626 as libc::c_int as flex_int16_t,
    615 as libc::c_int as flex_int16_t,
    630 as libc::c_int as flex_int16_t,
    659 as libc::c_int as flex_int16_t,
    164 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    145 as libc::c_int as flex_int16_t,
    577 as libc::c_int as flex_int16_t,
    93 as libc::c_int as flex_int16_t,
    72 as libc::c_int as flex_int16_t,
    61 as libc::c_int as flex_int16_t,
    55 as libc::c_int as flex_int16_t,
    84 as libc::c_int as flex_int16_t,
    79 as libc::c_int as flex_int16_t,
    635 as libc::c_int as flex_int16_t,
    637 as libc::c_int as flex_int16_t,
    639 as libc::c_int as flex_int16_t,
    641 as libc::c_int as flex_int16_t,
    75 as libc::c_int as flex_int16_t,
    663 as libc::c_int as flex_int16_t,
    18 as libc::c_int as flex_int16_t,
    654 as libc::c_int as flex_int16_t,
    669 as libc::c_int as flex_int16_t,
    660 as libc::c_int as flex_int16_t,
    682 as libc::c_int as flex_int16_t,
    673 as libc::c_int as flex_int16_t,
    687 as libc::c_int as flex_int16_t,
    690 as libc::c_int as flex_int16_t,
    695 as libc::c_int as flex_int16_t,
    953 as libc::c_int as flex_int16_t,
    698 as libc::c_int as flex_int16_t,
    706 as libc::c_int as flex_int16_t,
    713 as libc::c_int as flex_int16_t,
    718 as libc::c_int as flex_int16_t,
    721 as libc::c_int as flex_int16_t,
    953 as libc::c_int as flex_int16_t,
    741 as libc::c_int as flex_int16_t,
    747 as libc::c_int as flex_int16_t,
    752 as libc::c_int as flex_int16_t,
    756 as libc::c_int as flex_int16_t,
    762 as libc::c_int as flex_int16_t,
    768 as libc::c_int as flex_int16_t,
    774 as libc::c_int as flex_int16_t,
    780 as libc::c_int as flex_int16_t,
    786 as libc::c_int as flex_int16_t,
    792 as libc::c_int as flex_int16_t,
    798 as libc::c_int as flex_int16_t,
    804 as libc::c_int as flex_int16_t,
    810 as libc::c_int as flex_int16_t,
    814 as libc::c_int as flex_int16_t,
    817 as libc::c_int as flex_int16_t,
    823 as libc::c_int as flex_int16_t,
    829 as libc::c_int as flex_int16_t,
    835 as libc::c_int as flex_int16_t,
    841 as libc::c_int as flex_int16_t,
    847 as libc::c_int as flex_int16_t,
    853 as libc::c_int as flex_int16_t,
    859 as libc::c_int as flex_int16_t,
    865 as libc::c_int as flex_int16_t,
    871 as libc::c_int as flex_int16_t,
    877 as libc::c_int as flex_int16_t,
    883 as libc::c_int as flex_int16_t,
    889 as libc::c_int as flex_int16_t,
    895 as libc::c_int as flex_int16_t,
    901 as libc::c_int as flex_int16_t,
    907 as libc::c_int as flex_int16_t,
    913 as libc::c_int as flex_int16_t,
    919 as libc::c_int as flex_int16_t,
    925 as libc::c_int as flex_int16_t,
    931 as libc::c_int as flex_int16_t,
];
static mut yy_nxt: [flex_int16_t; 1010] = [
    0 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    5 as libc::c_int as flex_int16_t,
    6 as libc::c_int as flex_int16_t,
    5 as libc::c_int as flex_int16_t,
    7 as libc::c_int as flex_int16_t,
    8 as libc::c_int as flex_int16_t,
    9 as libc::c_int as flex_int16_t,
    10 as libc::c_int as flex_int16_t,
    11 as libc::c_int as flex_int16_t,
    12 as libc::c_int as flex_int16_t,
    13 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    15 as libc::c_int as flex_int16_t,
    16 as libc::c_int as flex_int16_t,
    17 as libc::c_int as flex_int16_t,
    18 as libc::c_int as flex_int16_t,
    19 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    24 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    28 as libc::c_int as flex_int16_t,
    29 as libc::c_int as flex_int16_t,
    30 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    31 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    32 as libc::c_int as flex_int16_t,
    33 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    35 as libc::c_int as flex_int16_t,
    36 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    12 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    38 as libc::c_int as flex_int16_t,
    39 as libc::c_int as flex_int16_t,
    40 as libc::c_int as flex_int16_t,
    41 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    43 as libc::c_int as flex_int16_t,
    43 as libc::c_int as flex_int16_t,
    43 as libc::c_int as flex_int16_t,
    50 as libc::c_int as flex_int16_t,
    61 as libc::c_int as flex_int16_t,
    44 as libc::c_int as flex_int16_t,
    43 as libc::c_int as flex_int16_t,
    43 as libc::c_int as flex_int16_t,
    43 as libc::c_int as flex_int16_t,
    50 as libc::c_int as flex_int16_t,
    109 as libc::c_int as flex_int16_t,
    44 as libc::c_int as flex_int16_t,
    151 as libc::c_int as flex_int16_t,
    45 as libc::c_int as flex_int16_t,
    62 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    45 as libc::c_int as flex_int16_t,
    55 as libc::c_int as flex_int16_t,
    257 as libc::c_int as flex_int16_t,
    56 as libc::c_int as flex_int16_t,
    56 as libc::c_int as flex_int16_t,
    56 as libc::c_int as flex_int16_t,
    250 as libc::c_int as flex_int16_t,
    51 as libc::c_int as flex_int16_t,
    57 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    58 as libc::c_int as flex_int16_t,
    247 as libc::c_int as flex_int16_t,
    46 as libc::c_int as flex_int16_t,
    51 as libc::c_int as flex_int16_t,
    59 as libc::c_int as flex_int16_t,
    59 as libc::c_int as flex_int16_t,
    59 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    46 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    63 as libc::c_int as flex_int16_t,
    63 as libc::c_int as flex_int16_t,
    63 as libc::c_int as flex_int16_t,
    147 as libc::c_int as flex_int16_t,
    147 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    65 as libc::c_int as flex_int16_t,
    65 as libc::c_int as flex_int16_t,
    65 as libc::c_int as flex_int16_t,
    47 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    66 as libc::c_int as flex_int16_t,
    67 as libc::c_int as flex_int16_t,
    67 as libc::c_int as flex_int16_t,
    67 as libc::c_int as flex_int16_t,
    47 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    68 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    147 as libc::c_int as flex_int16_t,
    147 as libc::c_int as flex_int16_t,
    71 as libc::c_int as flex_int16_t,
    72 as libc::c_int as flex_int16_t,
    72 as libc::c_int as flex_int16_t,
    72 as libc::c_int as flex_int16_t,
    274 as libc::c_int as flex_int16_t,
    74 as libc::c_int as flex_int16_t,
    73 as libc::c_int as flex_int16_t,
    75 as libc::c_int as flex_int16_t,
    75 as libc::c_int as flex_int16_t,
    75 as libc::c_int as flex_int16_t,
    69 as libc::c_int as flex_int16_t,
    74 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    78 as libc::c_int as flex_int16_t,
    78 as libc::c_int as flex_int16_t,
    78 as libc::c_int as flex_int16_t,
    86 as libc::c_int as flex_int16_t,
    86 as libc::c_int as flex_int16_t,
    86 as libc::c_int as flex_int16_t,
    117 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    87 as libc::c_int as flex_int16_t,
    90 as libc::c_int as flex_int16_t,
    90 as libc::c_int as flex_int16_t,
    90 as libc::c_int as flex_int16_t,
    151 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    91 as libc::c_int as flex_int16_t,
    88 as libc::c_int as flex_int16_t,
    238 as libc::c_int as flex_int16_t,
    122 as libc::c_int as flex_int16_t,
    76 as libc::c_int as flex_int16_t,
    94 as libc::c_int as flex_int16_t,
    94 as libc::c_int as flex_int16_t,
    94 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    89 as libc::c_int as flex_int16_t,
    95 as libc::c_int as flex_int16_t,
    77 as libc::c_int as flex_int16_t,
    59 as libc::c_int as flex_int16_t,
    59 as libc::c_int as flex_int16_t,
    59 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    93 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    104 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    257 as libc::c_int as flex_int16_t,
    106 as libc::c_int as flex_int16_t,
    250 as libc::c_int as flex_int16_t,
    247 as libc::c_int as flex_int16_t,
    96 as libc::c_int as flex_int16_t,
    105 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    107 as libc::c_int as flex_int16_t,
    108 as libc::c_int as flex_int16_t,
    79 as libc::c_int as flex_int16_t,
    99 as libc::c_int as flex_int16_t,
    100 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    102 as libc::c_int as flex_int16_t,
    103 as libc::c_int as flex_int16_t,
    61 as libc::c_int as flex_int16_t,
    145 as libc::c_int as flex_int16_t,
    145 as libc::c_int as flex_int16_t,
    145 as libc::c_int as flex_int16_t,
    99 as libc::c_int as flex_int16_t,
    100 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    102 as libc::c_int as flex_int16_t,
    103 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    62 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    110 as libc::c_int as flex_int16_t,
    112 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    179 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    111 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    207 as libc::c_int as flex_int16_t,
    207 as libc::c_int as flex_int16_t,
    207 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    81 as libc::c_int as flex_int16_t,
    82 as libc::c_int as flex_int16_t,
    83 as libc::c_int as flex_int16_t,
    84 as libc::c_int as flex_int16_t,
    85 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    265 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    114 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    115 as libc::c_int as flex_int16_t,
    118 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    119 as libc::c_int as flex_int16_t,
    119 as libc::c_int as flex_int16_t,
    119 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    120 as libc::c_int as flex_int16_t,
    43 as libc::c_int as flex_int16_t,
    43 as libc::c_int as flex_int16_t,
    43 as libc::c_int as flex_int16_t,
    178 as libc::c_int as flex_int16_t,
    181 as libc::c_int as flex_int16_t,
    44 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    71 as libc::c_int as flex_int16_t,
    264 as libc::c_int as flex_int16_t,
    45 as libc::c_int as flex_int16_t,
    119 as libc::c_int as flex_int16_t,
    119 as libc::c_int as flex_int16_t,
    119 as libc::c_int as flex_int16_t,
    183 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    120 as libc::c_int as flex_int16_t,
    129 as libc::c_int as flex_int16_t,
    129 as libc::c_int as flex_int16_t,
    129 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    182 as libc::c_int as flex_int16_t,
    130 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    43 as libc::c_int as flex_int16_t,
    43 as libc::c_int as flex_int16_t,
    43 as libc::c_int as flex_int16_t,
    263 as libc::c_int as flex_int16_t,
    46 as libc::c_int as flex_int16_t,
    44 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    262 as libc::c_int as flex_int16_t,
    238 as libc::c_int as flex_int16_t,
    55 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    45 as libc::c_int as flex_int16_t,
    200 as libc::c_int as flex_int16_t,
    121 as libc::c_int as flex_int16_t,
    180 as libc::c_int as flex_int16_t,
    56 as libc::c_int as flex_int16_t,
    56 as libc::c_int as flex_int16_t,
    56 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    47 as libc::c_int as flex_int16_t,
    57 as libc::c_int as flex_int16_t,
    134 as libc::c_int as flex_int16_t,
    134 as libc::c_int as flex_int16_t,
    134 as libc::c_int as flex_int16_t,
    259 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    135 as libc::c_int as flex_int16_t,
    122 as libc::c_int as flex_int16_t,
    193 as libc::c_int as flex_int16_t,
    46 as libc::c_int as flex_int16_t,
    59 as libc::c_int as flex_int16_t,
    59 as libc::c_int as flex_int16_t,
    59 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    246 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    63 as libc::c_int as flex_int16_t,
    63 as libc::c_int as flex_int16_t,
    63 as libc::c_int as flex_int16_t,
    245 as libc::c_int as flex_int16_t,
    172 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    65 as libc::c_int as flex_int16_t,
    65 as libc::c_int as flex_int16_t,
    65 as libc::c_int as flex_int16_t,
    169 as libc::c_int as flex_int16_t,
    47 as libc::c_int as flex_int16_t,
    66 as libc::c_int as flex_int16_t,
    67 as libc::c_int as flex_int16_t,
    67 as libc::c_int as flex_int16_t,
    67 as libc::c_int as flex_int16_t,
    167 as libc::c_int as flex_int16_t,
    164 as libc::c_int as flex_int16_t,
    68 as libc::c_int as flex_int16_t,
    141 as libc::c_int as flex_int16_t,
    141 as libc::c_int as flex_int16_t,
    141 as libc::c_int as flex_int16_t,
    162 as libc::c_int as flex_int16_t,
    244 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    243 as libc::c_int as flex_int16_t,
    242 as libc::c_int as flex_int16_t,
    71 as libc::c_int as flex_int16_t,
    72 as libc::c_int as flex_int16_t,
    72 as libc::c_int as flex_int16_t,
    72 as libc::c_int as flex_int16_t,
    241 as libc::c_int as flex_int16_t,
    146 as libc::c_int as flex_int16_t,
    73 as libc::c_int as flex_int16_t,
    75 as libc::c_int as flex_int16_t,
    75 as libc::c_int as flex_int16_t,
    75 as libc::c_int as flex_int16_t,
    74 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    78 as libc::c_int as flex_int16_t,
    78 as libc::c_int as flex_int16_t,
    78 as libc::c_int as flex_int16_t,
    149 as libc::c_int as flex_int16_t,
    149 as libc::c_int as flex_int16_t,
    149 as libc::c_int as flex_int16_t,
    151 as libc::c_int as flex_int16_t,
    188 as libc::c_int as flex_int16_t,
    150 as libc::c_int as flex_int16_t,
    152 as libc::c_int as flex_int16_t,
    153 as libc::c_int as flex_int16_t,
    154 as libc::c_int as flex_int16_t,
    155 as libc::c_int as flex_int16_t,
    156 as libc::c_int as flex_int16_t,
    86 as libc::c_int as flex_int16_t,
    86 as libc::c_int as flex_int16_t,
    86 as libc::c_int as flex_int16_t,
    151 as libc::c_int as flex_int16_t,
    149 as libc::c_int as flex_int16_t,
    87 as libc::c_int as flex_int16_t,
    162 as libc::c_int as flex_int16_t,
    162 as libc::c_int as flex_int16_t,
    162 as libc::c_int as flex_int16_t,
    141 as libc::c_int as flex_int16_t,
    200 as libc::c_int as flex_int16_t,
    163 as libc::c_int as flex_int16_t,
    164 as libc::c_int as flex_int16_t,
    164 as libc::c_int as flex_int16_t,
    164 as libc::c_int as flex_int16_t,
    134 as libc::c_int as flex_int16_t,
    129 as libc::c_int as flex_int16_t,
    165 as libc::c_int as flex_int16_t,
    90 as libc::c_int as flex_int16_t,
    90 as libc::c_int as flex_int16_t,
    90 as libc::c_int as flex_int16_t,
    236 as libc::c_int as flex_int16_t,
    235 as libc::c_int as flex_int16_t,
    91 as libc::c_int as flex_int16_t,
    167 as libc::c_int as flex_int16_t,
    167 as libc::c_int as flex_int16_t,
    167 as libc::c_int as flex_int16_t,
    234 as libc::c_int as flex_int16_t,
    193 as libc::c_int as flex_int16_t,
    168 as libc::c_int as flex_int16_t,
    169 as libc::c_int as flex_int16_t,
    169 as libc::c_int as flex_int16_t,
    169 as libc::c_int as flex_int16_t,
    119 as libc::c_int as flex_int16_t,
    221 as libc::c_int as flex_int16_t,
    170 as libc::c_int as flex_int16_t,
    94 as libc::c_int as flex_int16_t,
    94 as libc::c_int as flex_int16_t,
    94 as libc::c_int as flex_int16_t,
    220 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    95 as libc::c_int as flex_int16_t,
    172 as libc::c_int as flex_int16_t,
    172 as libc::c_int as flex_int16_t,
    172 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    173 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    184 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    186 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    185 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    189 as libc::c_int as flex_int16_t,
    187 as libc::c_int as flex_int16_t,
    191 as libc::c_int as flex_int16_t,
    119 as libc::c_int as flex_int16_t,
    119 as libc::c_int as flex_int16_t,
    119 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    120 as libc::c_int as flex_int16_t,
    193 as libc::c_int as flex_int16_t,
    193 as libc::c_int as flex_int16_t,
    193 as libc::c_int as flex_int16_t,
    172 as libc::c_int as flex_int16_t,
    94 as libc::c_int as flex_int16_t,
    194 as libc::c_int as flex_int16_t,
    129 as libc::c_int as flex_int16_t,
    129 as libc::c_int as flex_int16_t,
    129 as libc::c_int as flex_int16_t,
    169 as libc::c_int as flex_int16_t,
    167 as libc::c_int as flex_int16_t,
    130 as libc::c_int as flex_int16_t,
    134 as libc::c_int as flex_int16_t,
    134 as libc::c_int as flex_int16_t,
    134 as libc::c_int as flex_int16_t,
    90 as libc::c_int as flex_int16_t,
    164 as libc::c_int as flex_int16_t,
    135 as libc::c_int as flex_int16_t,
    200 as libc::c_int as flex_int16_t,
    200 as libc::c_int as flex_int16_t,
    201 as libc::c_int as flex_int16_t,
    162 as libc::c_int as flex_int16_t,
    86 as libc::c_int as flex_int16_t,
    202 as libc::c_int as flex_int16_t,
    141 as libc::c_int as flex_int16_t,
    141 as libc::c_int as flex_int16_t,
    141 as libc::c_int as flex_int16_t,
    214 as libc::c_int as flex_int16_t,
    205 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    206 as libc::c_int as flex_int16_t,
    206 as libc::c_int as flex_int16_t,
    206 as libc::c_int as flex_int16_t,
    149 as libc::c_int as flex_int16_t,
    149 as libc::c_int as flex_int16_t,
    149 as libc::c_int as flex_int16_t,
    213 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    150 as libc::c_int as flex_int16_t,
    152 as libc::c_int as flex_int16_t,
    153 as libc::c_int as flex_int16_t,
    154 as libc::c_int as flex_int16_t,
    155 as libc::c_int as flex_int16_t,
    156 as libc::c_int as flex_int16_t,
    162 as libc::c_int as flex_int16_t,
    162 as libc::c_int as flex_int16_t,
    162 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    212 as libc::c_int as flex_int16_t,
    163 as libc::c_int as flex_int16_t,
    164 as libc::c_int as flex_int16_t,
    164 as libc::c_int as flex_int16_t,
    164 as libc::c_int as flex_int16_t,
    211 as libc::c_int as flex_int16_t,
    210 as libc::c_int as flex_int16_t,
    165 as libc::c_int as flex_int16_t,
    167 as libc::c_int as flex_int16_t,
    167 as libc::c_int as flex_int16_t,
    167 as libc::c_int as flex_int16_t,
    209 as libc::c_int as flex_int16_t,
    151 as libc::c_int as flex_int16_t,
    168 as libc::c_int as flex_int16_t,
    169 as libc::c_int as flex_int16_t,
    169 as libc::c_int as flex_int16_t,
    169 as libc::c_int as flex_int16_t,
    203 as libc::c_int as flex_int16_t,
    149 as libc::c_int as flex_int16_t,
    170 as libc::c_int as flex_int16_t,
    172 as libc::c_int as flex_int16_t,
    172 as libc::c_int as flex_int16_t,
    172 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    173 as libc::c_int as flex_int16_t,
    222 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    72 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    224 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    223 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    141 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    67 as libc::c_int as flex_int16_t,
    225 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    226 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    227 as libc::c_int as flex_int16_t,
    228 as libc::c_int as flex_int16_t,
    229 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    65 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    232 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    63 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    231 as libc::c_int as flex_int16_t,
    59 as libc::c_int as flex_int16_t,
    230 as libc::c_int as flex_int16_t,
    193 as libc::c_int as flex_int16_t,
    193 as libc::c_int as flex_int16_t,
    193 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    134 as libc::c_int as flex_int16_t,
    194 as libc::c_int as flex_int16_t,
    200 as libc::c_int as flex_int16_t,
    200 as libc::c_int as flex_int16_t,
    200 as libc::c_int as flex_int16_t,
    249 as libc::c_int as flex_int16_t,
    56 as libc::c_int as flex_int16_t,
    202 as libc::c_int as flex_int16_t,
    200 as libc::c_int as flex_int16_t,
    200 as libc::c_int as flex_int16_t,
    201 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    129 as libc::c_int as flex_int16_t,
    202 as libc::c_int as flex_int16_t,
    238 as libc::c_int as flex_int16_t,
    238 as libc::c_int as flex_int16_t,
    238 as libc::c_int as flex_int16_t,
    197 as libc::c_int as flex_int16_t,
    43 as libc::c_int as flex_int16_t,
    239 as libc::c_int as flex_int16_t,
    240 as libc::c_int as flex_int16_t,
    240 as libc::c_int as flex_int16_t,
    240 as libc::c_int as flex_int16_t,
    205 as libc::c_int as flex_int16_t,
    196 as libc::c_int as flex_int16_t,
    206 as libc::c_int as flex_int16_t,
    206 as libc::c_int as flex_int16_t,
    206 as libc::c_int as flex_int16_t,
    205 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    207 as libc::c_int as flex_int16_t,
    207 as libc::c_int as flex_int16_t,
    207 as libc::c_int as flex_int16_t,
    247 as libc::c_int as flex_int16_t,
    247 as libc::c_int as flex_int16_t,
    247 as libc::c_int as flex_int16_t,
    195 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    248 as libc::c_int as flex_int16_t,
    250 as libc::c_int as flex_int16_t,
    250 as libc::c_int as flex_int16_t,
    250 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    251 as libc::c_int as flex_int16_t,
    252 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    273 as libc::c_int as flex_int16_t,
    273 as libc::c_int as flex_int16_t,
    273 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    122 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    203 as libc::c_int as flex_int16_t,
    253 as libc::c_int as flex_int16_t,
    254 as libc::c_int as flex_int16_t,
    122 as libc::c_int as flex_int16_t,
    119 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    177 as libc::c_int as flex_int16_t,
    176 as libc::c_int as flex_int16_t,
    255 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    175 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    257 as libc::c_int as flex_int16_t,
    257 as libc::c_int as flex_int16_t,
    257 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    256 as libc::c_int as flex_int16_t,
    258 as libc::c_int as flex_int16_t,
    250 as libc::c_int as flex_int16_t,
    250 as libc::c_int as flex_int16_t,
    250 as libc::c_int as flex_int16_t,
    174 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    251 as libc::c_int as flex_int16_t,
    238 as libc::c_int as flex_int16_t,
    238 as libc::c_int as flex_int16_t,
    238 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    94 as libc::c_int as flex_int16_t,
    239 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    240 as libc::c_int as flex_int16_t,
    240 as libc::c_int as flex_int16_t,
    240 as libc::c_int as flex_int16_t,
    247 as libc::c_int as flex_int16_t,
    247 as libc::c_int as flex_int16_t,
    247 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    248 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    261 as libc::c_int as flex_int16_t,
    250 as libc::c_int as flex_int16_t,
    250 as libc::c_int as flex_int16_t,
    250 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    251 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    270 as libc::c_int as flex_int16_t,
    269 as libc::c_int as flex_int16_t,
    268 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    257 as libc::c_int as flex_int16_t,
    257 as libc::c_int as flex_int16_t,
    257 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    258 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    275 as libc::c_int as flex_int16_t,
    271 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    90 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    86 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    160 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    159 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    273 as libc::c_int as flex_int16_t,
    273 as libc::c_int as flex_int16_t,
    273 as libc::c_int as flex_int16_t,
    276 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    277 as libc::c_int as flex_int16_t,
    278 as libc::c_int as flex_int16_t,
    278 as libc::c_int as flex_int16_t,
    278 as libc::c_int as flex_int16_t,
    158 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    157 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    279 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    280 as libc::c_int as flex_int16_t,
    280 as libc::c_int as flex_int16_t,
    281 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    278 as libc::c_int as flex_int16_t,
    278 as libc::c_int as flex_int16_t,
    278 as libc::c_int as flex_int16_t,
    282 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    284 as libc::c_int as flex_int16_t,
    284 as libc::c_int as flex_int16_t,
    284 as libc::c_int as flex_int16_t,
    285 as libc::c_int as flex_int16_t,
    285 as libc::c_int as flex_int16_t,
    285 as libc::c_int as flex_int16_t,
    283 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    286 as libc::c_int as flex_int16_t,
    286 as libc::c_int as flex_int16_t,
    286 as libc::c_int as flex_int16_t,
    284 as libc::c_int as flex_int16_t,
    284 as libc::c_int as flex_int16_t,
    284 as libc::c_int as flex_int16_t,
    287 as libc::c_int as flex_int16_t,
    280 as libc::c_int as flex_int16_t,
    280 as libc::c_int as flex_int16_t,
    72 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    285 as libc::c_int as flex_int16_t,
    285 as libc::c_int as flex_int16_t,
    285 as libc::c_int as flex_int16_t,
    280 as libc::c_int as flex_int16_t,
    280 as libc::c_int as flex_int16_t,
    281 as libc::c_int as flex_int16_t,
    67 as libc::c_int as flex_int16_t,
    286 as libc::c_int as flex_int16_t,
    286 as libc::c_int as flex_int16_t,
    286 as libc::c_int as flex_int16_t,
    65 as libc::c_int as flex_int16_t,
    283 as libc::c_int as flex_int16_t,
    288 as libc::c_int as flex_int16_t,
    288 as libc::c_int as flex_int16_t,
    288 as libc::c_int as flex_int16_t,
    288 as libc::c_int as flex_int16_t,
    288 as libc::c_int as flex_int16_t,
    288 as libc::c_int as flex_int16_t,
    283 as libc::c_int as flex_int16_t,
    49 as libc::c_int as flex_int16_t,
    49 as libc::c_int as flex_int16_t,
    49 as libc::c_int as flex_int16_t,
    49 as libc::c_int as flex_int16_t,
    49 as libc::c_int as flex_int16_t,
    49 as libc::c_int as flex_int16_t,
    52 as libc::c_int as flex_int16_t,
    52 as libc::c_int as flex_int16_t,
    52 as libc::c_int as flex_int16_t,
    52 as libc::c_int as flex_int16_t,
    52 as libc::c_int as flex_int16_t,
    52 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    63 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    127 as libc::c_int as flex_int16_t,
    127 as libc::c_int as flex_int16_t,
    127 as libc::c_int as flex_int16_t,
    127 as libc::c_int as flex_int16_t,
    127 as libc::c_int as flex_int16_t,
    127 as libc::c_int as flex_int16_t,
    132 as libc::c_int as flex_int16_t,
    132 as libc::c_int as flex_int16_t,
    132 as libc::c_int as flex_int16_t,
    132 as libc::c_int as flex_int16_t,
    132 as libc::c_int as flex_int16_t,
    132 as libc::c_int as flex_int16_t,
    133 as libc::c_int as flex_int16_t,
    133 as libc::c_int as flex_int16_t,
    133 as libc::c_int as flex_int16_t,
    133 as libc::c_int as flex_int16_t,
    133 as libc::c_int as flex_int16_t,
    133 as libc::c_int as flex_int16_t,
    136 as libc::c_int as flex_int16_t,
    136 as libc::c_int as flex_int16_t,
    136 as libc::c_int as flex_int16_t,
    136 as libc::c_int as flex_int16_t,
    136 as libc::c_int as flex_int16_t,
    136 as libc::c_int as flex_int16_t,
    138 as libc::c_int as flex_int16_t,
    138 as libc::c_int as flex_int16_t,
    138 as libc::c_int as flex_int16_t,
    138 as libc::c_int as flex_int16_t,
    138 as libc::c_int as flex_int16_t,
    138 as libc::c_int as flex_int16_t,
    139 as libc::c_int as flex_int16_t,
    139 as libc::c_int as flex_int16_t,
    139 as libc::c_int as flex_int16_t,
    139 as libc::c_int as flex_int16_t,
    139 as libc::c_int as flex_int16_t,
    139 as libc::c_int as flex_int16_t,
    140 as libc::c_int as flex_int16_t,
    140 as libc::c_int as flex_int16_t,
    140 as libc::c_int as flex_int16_t,
    140 as libc::c_int as flex_int16_t,
    140 as libc::c_int as flex_int16_t,
    140 as libc::c_int as flex_int16_t,
    143 as libc::c_int as flex_int16_t,
    143 as libc::c_int as flex_int16_t,
    143 as libc::c_int as flex_int16_t,
    143 as libc::c_int as flex_int16_t,
    143 as libc::c_int as flex_int16_t,
    143 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    148 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    148 as libc::c_int as flex_int16_t,
    151 as libc::c_int as flex_int16_t,
    59 as libc::c_int as flex_int16_t,
    151 as libc::c_int as flex_int16_t,
    151 as libc::c_int as flex_int16_t,
    161 as libc::c_int as flex_int16_t,
    161 as libc::c_int as flex_int16_t,
    161 as libc::c_int as flex_int16_t,
    161 as libc::c_int as flex_int16_t,
    161 as libc::c_int as flex_int16_t,
    161 as libc::c_int as flex_int16_t,
    166 as libc::c_int as flex_int16_t,
    166 as libc::c_int as flex_int16_t,
    166 as libc::c_int as flex_int16_t,
    166 as libc::c_int as flex_int16_t,
    166 as libc::c_int as flex_int16_t,
    166 as libc::c_int as flex_int16_t,
    171 as libc::c_int as flex_int16_t,
    171 as libc::c_int as flex_int16_t,
    171 as libc::c_int as flex_int16_t,
    171 as libc::c_int as flex_int16_t,
    171 as libc::c_int as flex_int16_t,
    171 as libc::c_int as flex_int16_t,
    192 as libc::c_int as flex_int16_t,
    192 as libc::c_int as flex_int16_t,
    192 as libc::c_int as flex_int16_t,
    192 as libc::c_int as flex_int16_t,
    192 as libc::c_int as flex_int16_t,
    192 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    199 as libc::c_int as flex_int16_t,
    199 as libc::c_int as flex_int16_t,
    199 as libc::c_int as flex_int16_t,
    199 as libc::c_int as flex_int16_t,
    199 as libc::c_int as flex_int16_t,
    199 as libc::c_int as flex_int16_t,
    204 as libc::c_int as flex_int16_t,
    204 as libc::c_int as flex_int16_t,
    204 as libc::c_int as flex_int16_t,
    204 as libc::c_int as flex_int16_t,
    204 as libc::c_int as flex_int16_t,
    204 as libc::c_int as flex_int16_t,
    208 as libc::c_int as flex_int16_t,
    208 as libc::c_int as flex_int16_t,
    208 as libc::c_int as flex_int16_t,
    208 as libc::c_int as flex_int16_t,
    208 as libc::c_int as flex_int16_t,
    208 as libc::c_int as flex_int16_t,
    215 as libc::c_int as flex_int16_t,
    215 as libc::c_int as flex_int16_t,
    215 as libc::c_int as flex_int16_t,
    215 as libc::c_int as flex_int16_t,
    215 as libc::c_int as flex_int16_t,
    215 as libc::c_int as flex_int16_t,
    216 as libc::c_int as flex_int16_t,
    216 as libc::c_int as flex_int16_t,
    216 as libc::c_int as flex_int16_t,
    216 as libc::c_int as flex_int16_t,
    216 as libc::c_int as flex_int16_t,
    216 as libc::c_int as flex_int16_t,
    217 as libc::c_int as flex_int16_t,
    217 as libc::c_int as flex_int16_t,
    217 as libc::c_int as flex_int16_t,
    217 as libc::c_int as flex_int16_t,
    217 as libc::c_int as flex_int16_t,
    217 as libc::c_int as flex_int16_t,
    218 as libc::c_int as flex_int16_t,
    218 as libc::c_int as flex_int16_t,
    218 as libc::c_int as flex_int16_t,
    218 as libc::c_int as flex_int16_t,
    218 as libc::c_int as flex_int16_t,
    218 as libc::c_int as flex_int16_t,
    219 as libc::c_int as flex_int16_t,
    219 as libc::c_int as flex_int16_t,
    219 as libc::c_int as flex_int16_t,
    219 as libc::c_int as flex_int16_t,
    219 as libc::c_int as flex_int16_t,
    219 as libc::c_int as flex_int16_t,
    233 as libc::c_int as flex_int16_t,
    233 as libc::c_int as flex_int16_t,
    233 as libc::c_int as flex_int16_t,
    233 as libc::c_int as flex_int16_t,
    233 as libc::c_int as flex_int16_t,
    233 as libc::c_int as flex_int16_t,
    237 as libc::c_int as flex_int16_t,
    237 as libc::c_int as flex_int16_t,
    237 as libc::c_int as flex_int16_t,
    237 as libc::c_int as flex_int16_t,
    237 as libc::c_int as flex_int16_t,
    237 as libc::c_int as flex_int16_t,
    260 as libc::c_int as flex_int16_t,
    260 as libc::c_int as flex_int16_t,
    260 as libc::c_int as flex_int16_t,
    260 as libc::c_int as flex_int16_t,
    260 as libc::c_int as flex_int16_t,
    260 as libc::c_int as flex_int16_t,
    266 as libc::c_int as flex_int16_t,
    266 as libc::c_int as flex_int16_t,
    266 as libc::c_int as flex_int16_t,
    266 as libc::c_int as flex_int16_t,
    266 as libc::c_int as flex_int16_t,
    266 as libc::c_int as flex_int16_t,
    267 as libc::c_int as flex_int16_t,
    267 as libc::c_int as flex_int16_t,
    267 as libc::c_int as flex_int16_t,
    267 as libc::c_int as flex_int16_t,
    267 as libc::c_int as flex_int16_t,
    267 as libc::c_int as flex_int16_t,
    272 as libc::c_int as flex_int16_t,
    272 as libc::c_int as flex_int16_t,
    272 as libc::c_int as flex_int16_t,
    272 as libc::c_int as flex_int16_t,
    272 as libc::c_int as flex_int16_t,
    272 as libc::c_int as flex_int16_t,
    56 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    53 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    131 as libc::c_int as flex_int16_t,
    128 as libc::c_int as flex_int16_t,
    43 as libc::c_int as flex_int16_t,
    126 as libc::c_int as flex_int16_t,
    125 as libc::c_int as flex_int16_t,
    124 as libc::c_int as flex_int16_t,
    123 as libc::c_int as flex_int16_t,
    122 as libc::c_int as flex_int16_t,
    53 as libc::c_int as flex_int16_t,
    48 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
];
static mut yy_meta: [YY_CHAR; 57] = [
    0 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    2 as libc::c_int as YY_CHAR,
    2 as libc::c_int as YY_CHAR,
    2 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    2 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    3 as libc::c_int as YY_CHAR,
    3 as libc::c_int as YY_CHAR,
    3 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    5 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    5 as libc::c_int as YY_CHAR,
    5 as libc::c_int as YY_CHAR,
    5 as libc::c_int as YY_CHAR,
    5 as libc::c_int as YY_CHAR,
    5 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
];
static mut yy_def: [flex_int16_t; 324] = [
    0 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    290 as libc::c_int as flex_int16_t,
    291 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    292 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    293 as libc::c_int as flex_int16_t,
    293 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    294 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    290 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    290 as libc::c_int as flex_int16_t,
    291 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    295 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    296 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    297 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    298 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    299 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    300 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    301 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    302 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    303 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    304 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    305 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    306 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    307 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    308 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    294 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    309 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    295 as libc::c_int as flex_int16_t,
    296 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    310 as libc::c_int as flex_int16_t,
    297 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    298 as libc::c_int as flex_int16_t,
    299 as libc::c_int as flex_int16_t,
    300 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    311 as libc::c_int as flex_int16_t,
    301 as libc::c_int as flex_int16_t,
    302 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    303 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    312 as libc::c_int as flex_int16_t,
    304 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    305 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    313 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    314 as libc::c_int as flex_int16_t,
    306 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    316 as libc::c_int as flex_int16_t,
    307 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    308 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    309 as libc::c_int as flex_int16_t,
    310 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    319 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    311 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    312 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    313 as libc::c_int as flex_int16_t,
    314 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    316 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    319 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    320 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    321 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    322 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    323 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    320 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    321 as libc::c_int as flex_int16_t,
    322 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    323 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
];
static mut yy_chk: [flex_int16_t; 1010] = [
    0 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    5 as libc::c_int as flex_int16_t,
    5 as libc::c_int as flex_int16_t,
    5 as libc::c_int as flex_int16_t,
    8 as libc::c_int as flex_int16_t,
    13 as libc::c_int as flex_int16_t,
    5 as libc::c_int as flex_int16_t,
    6 as libc::c_int as flex_int16_t,
    6 as libc::c_int as flex_int16_t,
    6 as libc::c_int as flex_int16_t,
    49 as libc::c_int as flex_int16_t,
    30 as libc::c_int as flex_int16_t,
    6 as libc::c_int as flex_int16_t,
    274 as libc::c_int as flex_int16_t,
    5 as libc::c_int as flex_int16_t,
    13 as libc::c_int as flex_int16_t,
    10 as libc::c_int as flex_int16_t,
    10 as libc::c_int as flex_int16_t,
    10 as libc::c_int as flex_int16_t,
    30 as libc::c_int as flex_int16_t,
    6 as libc::c_int as flex_int16_t,
    10 as libc::c_int as flex_int16_t,
    272 as libc::c_int as flex_int16_t,
    11 as libc::c_int as flex_int16_t,
    11 as libc::c_int as flex_int16_t,
    11 as libc::c_int as flex_int16_t,
    267 as libc::c_int as flex_int16_t,
    8 as libc::c_int as flex_int16_t,
    11 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    11 as libc::c_int as flex_int16_t,
    266 as libc::c_int as flex_int16_t,
    5 as libc::c_int as flex_int16_t,
    49 as libc::c_int as flex_int16_t,
    12 as libc::c_int as flex_int16_t,
    12 as libc::c_int as flex_int16_t,
    12 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    6 as libc::c_int as flex_int16_t,
    12 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    76 as libc::c_int as flex_int16_t,
    76 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    15 as libc::c_int as flex_int16_t,
    15 as libc::c_int as flex_int16_t,
    15 as libc::c_int as flex_int16_t,
    5 as libc::c_int as flex_int16_t,
    265 as libc::c_int as flex_int16_t,
    15 as libc::c_int as flex_int16_t,
    16 as libc::c_int as flex_int16_t,
    16 as libc::c_int as flex_int16_t,
    16 as libc::c_int as flex_int16_t,
    6 as libc::c_int as flex_int16_t,
    264 as libc::c_int as flex_int16_t,
    16 as libc::c_int as flex_int16_t,
    17 as libc::c_int as flex_int16_t,
    17 as libc::c_int as flex_int16_t,
    17 as libc::c_int as flex_int16_t,
    147 as libc::c_int as flex_int16_t,
    147 as libc::c_int as flex_int16_t,
    17 as libc::c_int as flex_int16_t,
    18 as libc::c_int as flex_int16_t,
    18 as libc::c_int as flex_int16_t,
    18 as libc::c_int as flex_int16_t,
    263 as libc::c_int as flex_int16_t,
    19 as libc::c_int as flex_int16_t,
    18 as libc::c_int as flex_int16_t,
    19 as libc::c_int as flex_int16_t,
    19 as libc::c_int as flex_int16_t,
    19 as libc::c_int as flex_int16_t,
    16 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    35 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    35 as libc::c_int as flex_int16_t,
    35 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    262 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    260 as libc::c_int as flex_int16_t,
    259 as libc::c_int as flex_int16_t,
    19 as libc::c_int as flex_int16_t,
    24 as libc::c_int as flex_int16_t,
    24 as libc::c_int as flex_int16_t,
    24 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    24 as libc::c_int as flex_int16_t,
    19 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    28 as libc::c_int as flex_int16_t,
    29 as libc::c_int as flex_int16_t,
    258 as libc::c_int as flex_int16_t,
    28 as libc::c_int as flex_int16_t,
    251 as libc::c_int as flex_int16_t,
    248 as libc::c_int as flex_int16_t,
    24 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    28 as libc::c_int as flex_int16_t,
    29 as libc::c_int as flex_int16_t,
    29 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    61 as libc::c_int as flex_int16_t,
    74 as libc::c_int as flex_int16_t,
    74 as libc::c_int as flex_int16_t,
    74 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    31 as libc::c_int as flex_int16_t,
    61 as libc::c_int as flex_int16_t,
    32 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    31 as libc::c_int as flex_int16_t,
    32 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    105 as libc::c_int as flex_int16_t,
    31 as libc::c_int as flex_int16_t,
    31 as libc::c_int as flex_int16_t,
    32 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    146 as libc::c_int as flex_int16_t,
    146 as libc::c_int as flex_int16_t,
    146 as libc::c_int as flex_int16_t,
    105 as libc::c_int as flex_int16_t,
    36 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    33 as libc::c_int as flex_int16_t,
    246 as libc::c_int as flex_int16_t,
    36 as libc::c_int as flex_int16_t,
    33 as libc::c_int as flex_int16_t,
    104 as libc::c_int as flex_int16_t,
    107 as libc::c_int as flex_int16_t,
    33 as libc::c_int as flex_int16_t,
    36 as libc::c_int as flex_int16_t,
    33 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    104 as libc::c_int as flex_int16_t,
    107 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    43 as libc::c_int as flex_int16_t,
    43 as libc::c_int as flex_int16_t,
    43 as libc::c_int as flex_int16_t,
    104 as libc::c_int as flex_int16_t,
    107 as libc::c_int as flex_int16_t,
    43 as libc::c_int as flex_int16_t,
    45 as libc::c_int as flex_int16_t,
    45 as libc::c_int as flex_int16_t,
    45 as libc::c_int as flex_int16_t,
    109 as libc::c_int as flex_int16_t,
    245 as libc::c_int as flex_int16_t,
    45 as libc::c_int as flex_int16_t,
    244 as libc::c_int as flex_int16_t,
    43 as libc::c_int as flex_int16_t,
    47 as libc::c_int as flex_int16_t,
    47 as libc::c_int as flex_int16_t,
    47 as libc::c_int as flex_int16_t,
    109 as libc::c_int as flex_int16_t,
    108 as libc::c_int as flex_int16_t,
    47 as libc::c_int as flex_int16_t,
    48 as libc::c_int as flex_int16_t,
    48 as libc::c_int as flex_int16_t,
    48 as libc::c_int as flex_int16_t,
    243 as libc::c_int as flex_int16_t,
    108 as libc::c_int as flex_int16_t,
    48 as libc::c_int as flex_int16_t,
    108 as libc::c_int as flex_int16_t,
    53 as libc::c_int as flex_int16_t,
    53 as libc::c_int as flex_int16_t,
    53 as libc::c_int as flex_int16_t,
    242 as libc::c_int as flex_int16_t,
    43 as libc::c_int as flex_int16_t,
    53 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    241 as libc::c_int as flex_int16_t,
    239 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    106 as libc::c_int as flex_int16_t,
    53 as libc::c_int as flex_int16_t,
    237 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    106 as libc::c_int as flex_int16_t,
    56 as libc::c_int as flex_int16_t,
    56 as libc::c_int as flex_int16_t,
    56 as libc::c_int as flex_int16_t,
    106 as libc::c_int as flex_int16_t,
    43 as libc::c_int as flex_int16_t,
    56 as libc::c_int as flex_int16_t,
    58 as libc::c_int as flex_int16_t,
    58 as libc::c_int as flex_int16_t,
    58 as libc::c_int as flex_int16_t,
    235 as libc::c_int as flex_int16_t,
    110 as libc::c_int as flex_int16_t,
    58 as libc::c_int as flex_int16_t,
    234 as libc::c_int as flex_int16_t,
    233 as libc::c_int as flex_int16_t,
    53 as libc::c_int as flex_int16_t,
    59 as libc::c_int as flex_int16_t,
    59 as libc::c_int as flex_int16_t,
    59 as libc::c_int as flex_int16_t,
    110 as libc::c_int as flex_int16_t,
    221 as libc::c_int as flex_int16_t,
    59 as libc::c_int as flex_int16_t,
    63 as libc::c_int as flex_int16_t,
    63 as libc::c_int as flex_int16_t,
    63 as libc::c_int as flex_int16_t,
    220 as libc::c_int as flex_int16_t,
    219 as libc::c_int as flex_int16_t,
    63 as libc::c_int as flex_int16_t,
    65 as libc::c_int as flex_int16_t,
    65 as libc::c_int as flex_int16_t,
    65 as libc::c_int as flex_int16_t,
    218 as libc::c_int as flex_int16_t,
    53 as libc::c_int as flex_int16_t,
    65 as libc::c_int as flex_int16_t,
    67 as libc::c_int as flex_int16_t,
    67 as libc::c_int as flex_int16_t,
    67 as libc::c_int as flex_int16_t,
    217 as libc::c_int as flex_int16_t,
    216 as libc::c_int as flex_int16_t,
    67 as libc::c_int as flex_int16_t,
    69 as libc::c_int as flex_int16_t,
    69 as libc::c_int as flex_int16_t,
    69 as libc::c_int as flex_int16_t,
    215 as libc::c_int as flex_int16_t,
    214 as libc::c_int as flex_int16_t,
    69 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    213 as libc::c_int as flex_int16_t,
    212 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    72 as libc::c_int as flex_int16_t,
    72 as libc::c_int as flex_int16_t,
    72 as libc::c_int as flex_int16_t,
    211 as libc::c_int as flex_int16_t,
    75 as libc::c_int as flex_int16_t,
    72 as libc::c_int as flex_int16_t,
    75 as libc::c_int as flex_int16_t,
    75 as libc::c_int as flex_int16_t,
    75 as libc::c_int as flex_int16_t,
    78 as libc::c_int as flex_int16_t,
    115 as libc::c_int as flex_int16_t,
    78 as libc::c_int as flex_int16_t,
    78 as libc::c_int as flex_int16_t,
    78 as libc::c_int as flex_int16_t,
    79 as libc::c_int as flex_int16_t,
    79 as libc::c_int as flex_int16_t,
    79 as libc::c_int as flex_int16_t,
    210 as libc::c_int as flex_int16_t,
    115 as libc::c_int as flex_int16_t,
    79 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    86 as libc::c_int as flex_int16_t,
    86 as libc::c_int as flex_int16_t,
    86 as libc::c_int as flex_int16_t,
    209 as libc::c_int as flex_int16_t,
    208 as libc::c_int as flex_int16_t,
    86 as libc::c_int as flex_int16_t,
    88 as libc::c_int as flex_int16_t,
    88 as libc::c_int as flex_int16_t,
    88 as libc::c_int as flex_int16_t,
    204 as libc::c_int as flex_int16_t,
    202 as libc::c_int as flex_int16_t,
    88 as libc::c_int as flex_int16_t,
    89 as libc::c_int as flex_int16_t,
    89 as libc::c_int as flex_int16_t,
    89 as libc::c_int as flex_int16_t,
    199 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    89 as libc::c_int as flex_int16_t,
    90 as libc::c_int as flex_int16_t,
    90 as libc::c_int as flex_int16_t,
    90 as libc::c_int as flex_int16_t,
    197 as libc::c_int as flex_int16_t,
    196 as libc::c_int as flex_int16_t,
    90 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    195 as libc::c_int as flex_int16_t,
    194 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    93 as libc::c_int as flex_int16_t,
    93 as libc::c_int as flex_int16_t,
    93 as libc::c_int as flex_int16_t,
    192 as libc::c_int as flex_int16_t,
    177 as libc::c_int as flex_int16_t,
    93 as libc::c_int as flex_int16_t,
    94 as libc::c_int as flex_int16_t,
    94 as libc::c_int as flex_int16_t,
    94 as libc::c_int as flex_int16_t,
    176 as libc::c_int as flex_int16_t,
    111 as libc::c_int as flex_int16_t,
    94 as libc::c_int as flex_int16_t,
    96 as libc::c_int as flex_int16_t,
    96 as libc::c_int as flex_int16_t,
    96 as libc::c_int as flex_int16_t,
    112 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    96 as libc::c_int as flex_int16_t,
    111 as libc::c_int as flex_int16_t,
    114 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    118 as libc::c_int as flex_int16_t,
    111 as libc::c_int as flex_int16_t,
    112 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    117 as libc::c_int as flex_int16_t,
    114 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    118 as libc::c_int as flex_int16_t,
    112 as libc::c_int as flex_int16_t,
    175 as libc::c_int as flex_int16_t,
    117 as libc::c_int as flex_int16_t,
    122 as libc::c_int as flex_int16_t,
    117 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    114 as libc::c_int as flex_int16_t,
    118 as libc::c_int as flex_int16_t,
    119 as libc::c_int as flex_int16_t,
    119 as libc::c_int as flex_int16_t,
    119 as libc::c_int as flex_int16_t,
    122 as libc::c_int as flex_int16_t,
    174 as libc::c_int as flex_int16_t,
    119 as libc::c_int as flex_int16_t,
    121 as libc::c_int as flex_int16_t,
    121 as libc::c_int as flex_int16_t,
    121 as libc::c_int as flex_int16_t,
    173 as libc::c_int as flex_int16_t,
    171 as libc::c_int as flex_int16_t,
    121 as libc::c_int as flex_int16_t,
    129 as libc::c_int as flex_int16_t,
    129 as libc::c_int as flex_int16_t,
    129 as libc::c_int as flex_int16_t,
    170 as libc::c_int as flex_int16_t,
    168 as libc::c_int as flex_int16_t,
    129 as libc::c_int as flex_int16_t,
    134 as libc::c_int as flex_int16_t,
    134 as libc::c_int as flex_int16_t,
    134 as libc::c_int as flex_int16_t,
    166 as libc::c_int as flex_int16_t,
    165 as libc::c_int as flex_int16_t,
    134 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    163 as libc::c_int as flex_int16_t,
    161 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    141 as libc::c_int as flex_int16_t,
    141 as libc::c_int as flex_int16_t,
    141 as libc::c_int as flex_int16_t,
    160 as libc::c_int as flex_int16_t,
    145 as libc::c_int as flex_int16_t,
    141 as libc::c_int as flex_int16_t,
    145 as libc::c_int as flex_int16_t,
    145 as libc::c_int as flex_int16_t,
    145 as libc::c_int as flex_int16_t,
    149 as libc::c_int as flex_int16_t,
    149 as libc::c_int as flex_int16_t,
    149 as libc::c_int as flex_int16_t,
    159 as libc::c_int as flex_int16_t,
    158 as libc::c_int as flex_int16_t,
    149 as libc::c_int as flex_int16_t,
    151 as libc::c_int as flex_int16_t,
    151 as libc::c_int as flex_int16_t,
    151 as libc::c_int as flex_int16_t,
    151 as libc::c_int as flex_int16_t,
    151 as libc::c_int as flex_int16_t,
    162 as libc::c_int as flex_int16_t,
    162 as libc::c_int as flex_int16_t,
    162 as libc::c_int as flex_int16_t,
    157 as libc::c_int as flex_int16_t,
    156 as libc::c_int as flex_int16_t,
    162 as libc::c_int as flex_int16_t,
    164 as libc::c_int as flex_int16_t,
    164 as libc::c_int as flex_int16_t,
    164 as libc::c_int as flex_int16_t,
    155 as libc::c_int as flex_int16_t,
    154 as libc::c_int as flex_int16_t,
    164 as libc::c_int as flex_int16_t,
    167 as libc::c_int as flex_int16_t,
    167 as libc::c_int as flex_int16_t,
    167 as libc::c_int as flex_int16_t,
    153 as libc::c_int as flex_int16_t,
    152 as libc::c_int as flex_int16_t,
    167 as libc::c_int as flex_int16_t,
    169 as libc::c_int as flex_int16_t,
    169 as libc::c_int as flex_int16_t,
    169 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    150 as libc::c_int as flex_int16_t,
    169 as libc::c_int as flex_int16_t,
    172 as libc::c_int as flex_int16_t,
    172 as libc::c_int as flex_int16_t,
    172 as libc::c_int as flex_int16_t,
    178 as libc::c_int as flex_int16_t,
    179 as libc::c_int as flex_int16_t,
    172 as libc::c_int as flex_int16_t,
    178 as libc::c_int as flex_int16_t,
    180 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    182 as libc::c_int as flex_int16_t,
    143 as libc::c_int as flex_int16_t,
    178 as libc::c_int as flex_int16_t,
    179 as libc::c_int as flex_int16_t,
    181 as libc::c_int as flex_int16_t,
    183 as libc::c_int as flex_int16_t,
    180 as libc::c_int as flex_int16_t,
    181 as libc::c_int as flex_int16_t,
    182 as libc::c_int as flex_int16_t,
    179 as libc::c_int as flex_int16_t,
    184 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    181 as libc::c_int as flex_int16_t,
    183 as libc::c_int as flex_int16_t,
    140 as libc::c_int as flex_int16_t,
    182 as libc::c_int as flex_int16_t,
    187 as libc::c_int as flex_int16_t,
    185 as libc::c_int as flex_int16_t,
    184 as libc::c_int as flex_int16_t,
    183 as libc::c_int as flex_int16_t,
    186 as libc::c_int as flex_int16_t,
    184 as libc::c_int as flex_int16_t,
    185 as libc::c_int as flex_int16_t,
    186 as libc::c_int as flex_int16_t,
    187 as libc::c_int as flex_int16_t,
    185 as libc::c_int as flex_int16_t,
    188 as libc::c_int as flex_int16_t,
    189 as libc::c_int as flex_int16_t,
    186 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    191 as libc::c_int as flex_int16_t,
    139 as libc::c_int as flex_int16_t,
    225 as libc::c_int as flex_int16_t,
    191 as libc::c_int as flex_int16_t,
    188 as libc::c_int as flex_int16_t,
    189 as libc::c_int as flex_int16_t,
    138 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    191 as libc::c_int as flex_int16_t,
    223 as libc::c_int as flex_int16_t,
    225 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    136 as libc::c_int as flex_int16_t,
    189 as libc::c_int as flex_int16_t,
    193 as libc::c_int as flex_int16_t,
    193 as libc::c_int as flex_int16_t,
    193 as libc::c_int as flex_int16_t,
    223 as libc::c_int as flex_int16_t,
    135 as libc::c_int as flex_int16_t,
    193 as libc::c_int as flex_int16_t,
    200 as libc::c_int as flex_int16_t,
    200 as libc::c_int as flex_int16_t,
    200 as libc::c_int as flex_int16_t,
    223 as libc::c_int as flex_int16_t,
    133 as libc::c_int as flex_int16_t,
    200 as libc::c_int as flex_int16_t,
    201 as libc::c_int as flex_int16_t,
    201 as libc::c_int as flex_int16_t,
    201 as libc::c_int as flex_int16_t,
    132 as libc::c_int as flex_int16_t,
    130 as libc::c_int as flex_int16_t,
    201 as libc::c_int as flex_int16_t,
    203 as libc::c_int as flex_int16_t,
    203 as libc::c_int as flex_int16_t,
    203 as libc::c_int as flex_int16_t,
    128 as libc::c_int as flex_int16_t,
    127 as libc::c_int as flex_int16_t,
    203 as libc::c_int as flex_int16_t,
    205 as libc::c_int as flex_int16_t,
    205 as libc::c_int as flex_int16_t,
    205 as libc::c_int as flex_int16_t,
    206 as libc::c_int as flex_int16_t,
    126 as libc::c_int as flex_int16_t,
    206 as libc::c_int as flex_int16_t,
    206 as libc::c_int as flex_int16_t,
    206 as libc::c_int as flex_int16_t,
    207 as libc::c_int as flex_int16_t,
    232 as libc::c_int as flex_int16_t,
    207 as libc::c_int as flex_int16_t,
    207 as libc::c_int as flex_int16_t,
    207 as libc::c_int as flex_int16_t,
    222 as libc::c_int as flex_int16_t,
    222 as libc::c_int as flex_int16_t,
    222 as libc::c_int as flex_int16_t,
    125 as libc::c_int as flex_int16_t,
    232 as libc::c_int as flex_int16_t,
    222 as libc::c_int as flex_int16_t,
    224 as libc::c_int as flex_int16_t,
    224 as libc::c_int as flex_int16_t,
    224 as libc::c_int as flex_int16_t,
    226 as libc::c_int as flex_int16_t,
    227 as libc::c_int as flex_int16_t,
    224 as libc::c_int as flex_int16_t,
    226 as libc::c_int as flex_int16_t,
    228 as libc::c_int as flex_int16_t,
    261 as libc::c_int as flex_int16_t,
    261 as libc::c_int as flex_int16_t,
    261 as libc::c_int as flex_int16_t,
    226 as libc::c_int as flex_int16_t,
    227 as libc::c_int as flex_int16_t,
    229 as libc::c_int as flex_int16_t,
    124 as libc::c_int as flex_int16_t,
    228 as libc::c_int as flex_int16_t,
    201 as libc::c_int as flex_int16_t,
    227 as libc::c_int as flex_int16_t,
    228 as libc::c_int as flex_int16_t,
    123 as libc::c_int as flex_int16_t,
    120 as libc::c_int as flex_int16_t,
    229 as libc::c_int as flex_int16_t,
    222 as libc::c_int as flex_int16_t,
    249 as libc::c_int as flex_int16_t,
    230 as libc::c_int as flex_int16_t,
    103 as libc::c_int as flex_int16_t,
    102 as libc::c_int as flex_int16_t,
    229 as libc::c_int as flex_int16_t,
    224 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    222 as libc::c_int as flex_int16_t,
    249 as libc::c_int as flex_int16_t,
    230 as libc::c_int as flex_int16_t,
    231 as libc::c_int as flex_int16_t,
    231 as libc::c_int as flex_int16_t,
    231 as libc::c_int as flex_int16_t,
    224 as libc::c_int as flex_int16_t,
    230 as libc::c_int as flex_int16_t,
    231 as libc::c_int as flex_int16_t,
    236 as libc::c_int as flex_int16_t,
    236 as libc::c_int as flex_int16_t,
    236 as libc::c_int as flex_int16_t,
    100 as libc::c_int as flex_int16_t,
    252 as libc::c_int as flex_int16_t,
    236 as libc::c_int as flex_int16_t,
    238 as libc::c_int as flex_int16_t,
    238 as libc::c_int as flex_int16_t,
    238 as libc::c_int as flex_int16_t,
    99 as libc::c_int as flex_int16_t,
    95 as libc::c_int as flex_int16_t,
    238 as libc::c_int as flex_int16_t,
    252 as libc::c_int as flex_int16_t,
    240 as libc::c_int as flex_int16_t,
    240 as libc::c_int as flex_int16_t,
    240 as libc::c_int as flex_int16_t,
    247 as libc::c_int as flex_int16_t,
    247 as libc::c_int as flex_int16_t,
    247 as libc::c_int as flex_int16_t,
    253 as libc::c_int as flex_int16_t,
    255 as libc::c_int as flex_int16_t,
    247 as libc::c_int as flex_int16_t,
    231 as libc::c_int as flex_int16_t,
    240 as libc::c_int as flex_int16_t,
    250 as libc::c_int as flex_int16_t,
    250 as libc::c_int as flex_int16_t,
    250 as libc::c_int as flex_int16_t,
    253 as libc::c_int as flex_int16_t,
    255 as libc::c_int as flex_int16_t,
    250 as libc::c_int as flex_int16_t,
    231 as libc::c_int as flex_int16_t,
    254 as libc::c_int as flex_int16_t,
    255 as libc::c_int as flex_int16_t,
    254 as libc::c_int as flex_int16_t,
    253 as libc::c_int as flex_int16_t,
    256 as libc::c_int as flex_int16_t,
    257 as libc::c_int as flex_int16_t,
    257 as libc::c_int as flex_int16_t,
    257 as libc::c_int as flex_int16_t,
    254 as libc::c_int as flex_int16_t,
    268 as libc::c_int as flex_int16_t,
    257 as libc::c_int as flex_int16_t,
    269 as libc::c_int as flex_int16_t,
    256 as libc::c_int as flex_int16_t,
    270 as libc::c_int as flex_int16_t,
    256 as libc::c_int as flex_int16_t,
    271 as libc::c_int as flex_int16_t,
    91 as libc::c_int as flex_int16_t,
    268 as libc::c_int as flex_int16_t,
    87 as libc::c_int as flex_int16_t,
    269 as libc::c_int as flex_int16_t,
    85 as libc::c_int as flex_int16_t,
    270 as libc::c_int as flex_int16_t,
    84 as libc::c_int as flex_int16_t,
    271 as libc::c_int as flex_int16_t,
    273 as libc::c_int as flex_int16_t,
    273 as libc::c_int as flex_int16_t,
    273 as libc::c_int as flex_int16_t,
    273 as libc::c_int as flex_int16_t,
    275 as libc::c_int as flex_int16_t,
    275 as libc::c_int as flex_int16_t,
    276 as libc::c_int as flex_int16_t,
    276 as libc::c_int as flex_int16_t,
    276 as libc::c_int as flex_int16_t,
    83 as libc::c_int as flex_int16_t,
    277 as libc::c_int as flex_int16_t,
    82 as libc::c_int as flex_int16_t,
    275 as libc::c_int as flex_int16_t,
    277 as libc::c_int as flex_int16_t,
    81 as libc::c_int as flex_int16_t,
    278 as libc::c_int as flex_int16_t,
    278 as libc::c_int as flex_int16_t,
    278 as libc::c_int as flex_int16_t,
    277 as libc::c_int as flex_int16_t,
    278 as libc::c_int as flex_int16_t,
    278 as libc::c_int as flex_int16_t,
    278 as libc::c_int as flex_int16_t,
    278 as libc::c_int as flex_int16_t,
    279 as libc::c_int as flex_int16_t,
    280 as libc::c_int as flex_int16_t,
    280 as libc::c_int as flex_int16_t,
    280 as libc::c_int as flex_int16_t,
    281 as libc::c_int as flex_int16_t,
    281 as libc::c_int as flex_int16_t,
    281 as libc::c_int as flex_int16_t,
    278 as libc::c_int as flex_int16_t,
    279 as libc::c_int as flex_int16_t,
    282 as libc::c_int as flex_int16_t,
    282 as libc::c_int as flex_int16_t,
    282 as libc::c_int as flex_int16_t,
    284 as libc::c_int as flex_int16_t,
    284 as libc::c_int as flex_int16_t,
    284 as libc::c_int as flex_int16_t,
    284 as libc::c_int as flex_int16_t,
    285 as libc::c_int as flex_int16_t,
    285 as libc::c_int as flex_int16_t,
    73 as libc::c_int as flex_int16_t,
    71 as libc::c_int as flex_int16_t,
    285 as libc::c_int as flex_int16_t,
    285 as libc::c_int as flex_int16_t,
    285 as libc::c_int as flex_int16_t,
    286 as libc::c_int as flex_int16_t,
    286 as libc::c_int as flex_int16_t,
    286 as libc::c_int as flex_int16_t,
    68 as libc::c_int as flex_int16_t,
    286 as libc::c_int as flex_int16_t,
    286 as libc::c_int as flex_int16_t,
    286 as libc::c_int as flex_int16_t,
    66 as libc::c_int as flex_int16_t,
    285 as libc::c_int as flex_int16_t,
    287 as libc::c_int as flex_int16_t,
    287 as libc::c_int as flex_int16_t,
    287 as libc::c_int as flex_int16_t,
    288 as libc::c_int as flex_int16_t,
    288 as libc::c_int as flex_int16_t,
    288 as libc::c_int as flex_int16_t,
    286 as libc::c_int as flex_int16_t,
    290 as libc::c_int as flex_int16_t,
    290 as libc::c_int as flex_int16_t,
    290 as libc::c_int as flex_int16_t,
    290 as libc::c_int as flex_int16_t,
    290 as libc::c_int as flex_int16_t,
    290 as libc::c_int as flex_int16_t,
    291 as libc::c_int as flex_int16_t,
    291 as libc::c_int as flex_int16_t,
    291 as libc::c_int as flex_int16_t,
    291 as libc::c_int as flex_int16_t,
    291 as libc::c_int as flex_int16_t,
    291 as libc::c_int as flex_int16_t,
    292 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    292 as libc::c_int as flex_int16_t,
    292 as libc::c_int as flex_int16_t,
    292 as libc::c_int as flex_int16_t,
    293 as libc::c_int as flex_int16_t,
    293 as libc::c_int as flex_int16_t,
    293 as libc::c_int as flex_int16_t,
    293 as libc::c_int as flex_int16_t,
    294 as libc::c_int as flex_int16_t,
    294 as libc::c_int as flex_int16_t,
    294 as libc::c_int as flex_int16_t,
    294 as libc::c_int as flex_int16_t,
    294 as libc::c_int as flex_int16_t,
    294 as libc::c_int as flex_int16_t,
    295 as libc::c_int as flex_int16_t,
    295 as libc::c_int as flex_int16_t,
    295 as libc::c_int as flex_int16_t,
    295 as libc::c_int as flex_int16_t,
    295 as libc::c_int as flex_int16_t,
    295 as libc::c_int as flex_int16_t,
    296 as libc::c_int as flex_int16_t,
    296 as libc::c_int as flex_int16_t,
    296 as libc::c_int as flex_int16_t,
    296 as libc::c_int as flex_int16_t,
    296 as libc::c_int as flex_int16_t,
    296 as libc::c_int as flex_int16_t,
    297 as libc::c_int as flex_int16_t,
    297 as libc::c_int as flex_int16_t,
    297 as libc::c_int as flex_int16_t,
    297 as libc::c_int as flex_int16_t,
    297 as libc::c_int as flex_int16_t,
    297 as libc::c_int as flex_int16_t,
    298 as libc::c_int as flex_int16_t,
    298 as libc::c_int as flex_int16_t,
    298 as libc::c_int as flex_int16_t,
    298 as libc::c_int as flex_int16_t,
    298 as libc::c_int as flex_int16_t,
    298 as libc::c_int as flex_int16_t,
    299 as libc::c_int as flex_int16_t,
    299 as libc::c_int as flex_int16_t,
    299 as libc::c_int as flex_int16_t,
    299 as libc::c_int as flex_int16_t,
    299 as libc::c_int as flex_int16_t,
    299 as libc::c_int as flex_int16_t,
    300 as libc::c_int as flex_int16_t,
    300 as libc::c_int as flex_int16_t,
    300 as libc::c_int as flex_int16_t,
    300 as libc::c_int as flex_int16_t,
    300 as libc::c_int as flex_int16_t,
    300 as libc::c_int as flex_int16_t,
    301 as libc::c_int as flex_int16_t,
    301 as libc::c_int as flex_int16_t,
    301 as libc::c_int as flex_int16_t,
    301 as libc::c_int as flex_int16_t,
    301 as libc::c_int as flex_int16_t,
    301 as libc::c_int as flex_int16_t,
    302 as libc::c_int as flex_int16_t,
    302 as libc::c_int as flex_int16_t,
    302 as libc::c_int as flex_int16_t,
    302 as libc::c_int as flex_int16_t,
    302 as libc::c_int as flex_int16_t,
    302 as libc::c_int as flex_int16_t,
    303 as libc::c_int as flex_int16_t,
    62 as libc::c_int as flex_int16_t,
    303 as libc::c_int as flex_int16_t,
    304 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    304 as libc::c_int as flex_int16_t,
    304 as libc::c_int as flex_int16_t,
    305 as libc::c_int as flex_int16_t,
    305 as libc::c_int as flex_int16_t,
    305 as libc::c_int as flex_int16_t,
    305 as libc::c_int as flex_int16_t,
    305 as libc::c_int as flex_int16_t,
    305 as libc::c_int as flex_int16_t,
    306 as libc::c_int as flex_int16_t,
    306 as libc::c_int as flex_int16_t,
    306 as libc::c_int as flex_int16_t,
    306 as libc::c_int as flex_int16_t,
    306 as libc::c_int as flex_int16_t,
    306 as libc::c_int as flex_int16_t,
    307 as libc::c_int as flex_int16_t,
    307 as libc::c_int as flex_int16_t,
    307 as libc::c_int as flex_int16_t,
    307 as libc::c_int as flex_int16_t,
    307 as libc::c_int as flex_int16_t,
    307 as libc::c_int as flex_int16_t,
    308 as libc::c_int as flex_int16_t,
    308 as libc::c_int as flex_int16_t,
    308 as libc::c_int as flex_int16_t,
    308 as libc::c_int as flex_int16_t,
    308 as libc::c_int as flex_int16_t,
    308 as libc::c_int as flex_int16_t,
    309 as libc::c_int as flex_int16_t,
    309 as libc::c_int as flex_int16_t,
    309 as libc::c_int as flex_int16_t,
    309 as libc::c_int as flex_int16_t,
    309 as libc::c_int as flex_int16_t,
    309 as libc::c_int as flex_int16_t,
    310 as libc::c_int as flex_int16_t,
    310 as libc::c_int as flex_int16_t,
    310 as libc::c_int as flex_int16_t,
    310 as libc::c_int as flex_int16_t,
    310 as libc::c_int as flex_int16_t,
    310 as libc::c_int as flex_int16_t,
    311 as libc::c_int as flex_int16_t,
    311 as libc::c_int as flex_int16_t,
    311 as libc::c_int as flex_int16_t,
    311 as libc::c_int as flex_int16_t,
    311 as libc::c_int as flex_int16_t,
    311 as libc::c_int as flex_int16_t,
    312 as libc::c_int as flex_int16_t,
    312 as libc::c_int as flex_int16_t,
    312 as libc::c_int as flex_int16_t,
    312 as libc::c_int as flex_int16_t,
    312 as libc::c_int as flex_int16_t,
    312 as libc::c_int as flex_int16_t,
    313 as libc::c_int as flex_int16_t,
    313 as libc::c_int as flex_int16_t,
    313 as libc::c_int as flex_int16_t,
    313 as libc::c_int as flex_int16_t,
    313 as libc::c_int as flex_int16_t,
    313 as libc::c_int as flex_int16_t,
    314 as libc::c_int as flex_int16_t,
    314 as libc::c_int as flex_int16_t,
    314 as libc::c_int as flex_int16_t,
    314 as libc::c_int as flex_int16_t,
    314 as libc::c_int as flex_int16_t,
    314 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    316 as libc::c_int as flex_int16_t,
    316 as libc::c_int as flex_int16_t,
    316 as libc::c_int as flex_int16_t,
    316 as libc::c_int as flex_int16_t,
    316 as libc::c_int as flex_int16_t,
    316 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    319 as libc::c_int as flex_int16_t,
    319 as libc::c_int as flex_int16_t,
    319 as libc::c_int as flex_int16_t,
    319 as libc::c_int as flex_int16_t,
    319 as libc::c_int as flex_int16_t,
    319 as libc::c_int as flex_int16_t,
    320 as libc::c_int as flex_int16_t,
    320 as libc::c_int as flex_int16_t,
    320 as libc::c_int as flex_int16_t,
    320 as libc::c_int as flex_int16_t,
    320 as libc::c_int as flex_int16_t,
    320 as libc::c_int as flex_int16_t,
    321 as libc::c_int as flex_int16_t,
    321 as libc::c_int as flex_int16_t,
    321 as libc::c_int as flex_int16_t,
    321 as libc::c_int as flex_int16_t,
    321 as libc::c_int as flex_int16_t,
    321 as libc::c_int as flex_int16_t,
    322 as libc::c_int as flex_int16_t,
    322 as libc::c_int as flex_int16_t,
    322 as libc::c_int as flex_int16_t,
    322 as libc::c_int as flex_int16_t,
    322 as libc::c_int as flex_int16_t,
    322 as libc::c_int as flex_int16_t,
    323 as libc::c_int as flex_int16_t,
    323 as libc::c_int as flex_int16_t,
    323 as libc::c_int as flex_int16_t,
    323 as libc::c_int as flex_int16_t,
    323 as libc::c_int as flex_int16_t,
    323 as libc::c_int as flex_int16_t,
    57 as libc::c_int as flex_int16_t,
    55 as libc::c_int as flex_int16_t,
    52 as libc::c_int as flex_int16_t,
    51 as libc::c_int as flex_int16_t,
    50 as libc::c_int as flex_int16_t,
    46 as libc::c_int as flex_int16_t,
    44 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    41 as libc::c_int as flex_int16_t,
    40 as libc::c_int as flex_int16_t,
    39 as libc::c_int as flex_int16_t,
    38 as libc::c_int as flex_int16_t,
    9 as libc::c_int as flex_int16_t,
    7 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
];
static mut yy_last_accepting_cpos: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut yy_last_accepting_state: yy_state_type = 0;
static mut yy_start: libc::c_int = 0 as libc::c_int;
pub static mut yyin: *mut FILE = 0 as *const FILE as *mut FILE;
pub unsafe extern "C" fn yyrestart(mut input_file: *mut FILE) {
    if if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        yyensure_buffer_stack();
        let ref mut fresh2 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
        *fresh2 = yy_create_buffer(yyin, 16384 as libc::c_int);
    }
    yy_init_buffer(
        if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else {
            0 as YY_BUFFER_STATE
        },
        input_file,
    );
    yy_load_buffer_state();
}
unsafe extern "C" fn yy_load_buffer_state() {
    yy_n_chars = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars;
    yy_c_buf_p = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_buf_pos;
    yytext = yy_c_buf_p;
    yyin = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_input_file;
    yy_hold_char = *yy_c_buf_p;
}
unsafe extern "C" fn yy_init_buffer(mut b: YY_BUFFER_STATE, mut file: *mut FILE) {
    let mut oerrno: libc::c_int = *__errno_location();
    yy_flush_buffer(b);
    (*b).yy_input_file = file;
    (*b).yy_fill_buffer = 1 as libc::c_int;
    if b
        != (if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else {
            0 as YY_BUFFER_STATE
        })
    {
        (*b).yy_bs_lineno = 1 as libc::c_int;
        (*b).yy_bs_column = 0 as libc::c_int;
    }
    (*b)
        .yy_is_interactive = if !file.is_null() {
        (isatty(fileno(file)) > 0 as libc::c_int) as libc::c_int
    } else {
        0 as libc::c_int
    };
    *__errno_location() = oerrno;
}
pub unsafe extern "C" fn yy_flush_buffer(mut b: YY_BUFFER_STATE) {
    if b.is_null() {
        return;
    }
    (*b).yy_n_chars = 0 as libc::c_int;
    *((*b).yy_ch_buf)
        .offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    *((*b).yy_ch_buf)
        .offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    (*b)
        .yy_buf_pos = &mut *((*b).yy_ch_buf).offset(0 as libc::c_int as isize)
        as *mut libc::c_char;
    (*b).yy_at_bol = 1 as libc::c_int;
    (*b).yy_buffer_status = 0 as libc::c_int;
    if b
        == (if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else {
            0 as YY_BUFFER_STATE
        })
    {
        yy_load_buffer_state();
    }
}
pub unsafe extern "C" fn yy_create_buffer(
    mut file: *mut FILE,
    mut size: libc::c_int,
) -> YY_BUFFER_STATE {
    let mut b: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    b = yyalloc(::std::mem::size_of::<yy_buffer_state>() as libc::c_ulong)
        as YY_BUFFER_STATE;
    if b.is_null() {
        yy_fatal_error(
            b"out of dynamic memory in yy_create_buffer()\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*b).yy_buf_size = size;
    (*b)
        .yy_ch_buf = yyalloc(((*b).yy_buf_size + 2 as libc::c_int) as yy_size_t)
        as *mut libc::c_char;
    if ((*b).yy_ch_buf).is_null() {
        yy_fatal_error(
            b"out of dynamic memory in yy_create_buffer()\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*b).yy_is_our_buffer = 1 as libc::c_int;
    yy_init_buffer(b, file);
    return b;
}
pub unsafe extern "C" fn yyalloc(mut size: yy_size_t) -> *mut libc::c_void {
    return malloc(size);
}
unsafe extern "C" fn yyensure_buffer_stack() {
    let mut num_to_alloc: yy_size_t = 0;
    if yy_buffer_stack.is_null() {
        num_to_alloc = 1 as libc::c_int as yy_size_t;
        yy_buffer_stack = yyalloc(
            num_to_alloc
                .wrapping_mul(
                    ::std::mem::size_of::<*mut yy_buffer_state>() as libc::c_ulong,
                ),
        ) as *mut *mut yy_buffer_state;
        if yy_buffer_stack.is_null() {
            yy_fatal_error(
                b"out of dynamic memory in yyensure_buffer_stack()\0" as *const u8
                    as *const libc::c_char,
            );
        }
        memset(
            yy_buffer_stack as *mut libc::c_void,
            0 as libc::c_int,
            num_to_alloc
                .wrapping_mul(
                    ::std::mem::size_of::<*mut yy_buffer_state>() as libc::c_ulong,
                ),
        );
        yy_buffer_stack_max = num_to_alloc;
        yy_buffer_stack_top = 0 as libc::c_int as size_t;
        return;
    }
    if yy_buffer_stack_top
        >= yy_buffer_stack_max.wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        let mut grow_size: yy_size_t = 8 as libc::c_int as yy_size_t;
        num_to_alloc = yy_buffer_stack_max.wrapping_add(grow_size);
        yy_buffer_stack = yyrealloc(
            yy_buffer_stack as *mut libc::c_void,
            num_to_alloc
                .wrapping_mul(
                    ::std::mem::size_of::<*mut yy_buffer_state>() as libc::c_ulong,
                ),
        ) as *mut *mut yy_buffer_state;
        if yy_buffer_stack.is_null() {
            yy_fatal_error(
                b"out of dynamic memory in yyensure_buffer_stack()\0" as *const u8
                    as *const libc::c_char,
            );
        }
        memset(
            yy_buffer_stack.offset(yy_buffer_stack_max as isize) as *mut libc::c_void,
            0 as libc::c_int,
            grow_size
                .wrapping_mul(
                    ::std::mem::size_of::<*mut yy_buffer_state>() as libc::c_ulong,
                ),
        );
        yy_buffer_stack_max = num_to_alloc;
    }
}
pub unsafe extern "C" fn yyrealloc(
    mut ptr: *mut libc::c_void,
    mut size: yy_size_t,
) -> *mut libc::c_void {
    return realloc(ptr, size);
}
static mut yy_did_buffer_switch_on_eof: libc::c_int = 0;
unsafe extern "C" fn yy_get_next_buffer() -> libc::c_int {
    let mut dest: *mut libc::c_char = (**yy_buffer_stack
        .offset(yy_buffer_stack_top as isize))
        .yy_ch_buf;
    let mut source: *mut libc::c_char = yytext;
    let mut number_to_move: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret_val: libc::c_int = 0;
    if yy_c_buf_p
        > &mut *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
            .offset((yy_n_chars + 1 as libc::c_int) as isize) as *mut libc::c_char
    {
        yy_fatal_error(
            b"fatal flex scanner internal error--end of buffer missed\0" as *const u8
                as *const libc::c_char,
        );
    }
    if (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_fill_buffer
        == 0 as libc::c_int
    {
        if yy_c_buf_p.offset_from(yytext) as libc::c_long
            - 0 as libc::c_int as libc::c_long == 1 as libc::c_int as libc::c_long
        {
            return 1 as libc::c_int
        } else {
            return 2 as libc::c_int
        }
    }
    number_to_move = (yy_c_buf_p.offset_from(yytext) as libc::c_long
        - 1 as libc::c_int as libc::c_long) as libc::c_int;
    i = 0 as libc::c_int;
    while i < number_to_move {
        let fresh3 = source;
        source = source.offset(1);
        let fresh4 = dest;
        dest = dest.offset(1);
        *fresh4 = *fresh3;
        i += 1;
        i;
    }
    if (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_buffer_status
        == 2 as libc::c_int
    {
        yy_n_chars = 0 as libc::c_int;
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars = yy_n_chars;
    } else {
        let mut num_to_read: libc::c_int = (**yy_buffer_stack
            .offset(yy_buffer_stack_top as isize))
            .yy_buf_size - number_to_move - 1 as libc::c_int;
        while num_to_read <= 0 as libc::c_int {
            let mut b: YY_BUFFER_STATE = *yy_buffer_stack
                .offset(yy_buffer_stack_top as isize);
            let mut yy_c_buf_p_offset: libc::c_int = yy_c_buf_p
                .offset_from((*b).yy_ch_buf) as libc::c_long as libc::c_int;
            if (*b).yy_is_our_buffer != 0 {
                let mut new_size: libc::c_int = (*b).yy_buf_size * 2 as libc::c_int;
                if new_size <= 0 as libc::c_int {
                    (*b).yy_buf_size += (*b).yy_buf_size / 8 as libc::c_int;
                } else {
                    (*b).yy_buf_size *= 2 as libc::c_int;
                }
                (*b)
                    .yy_ch_buf = yyrealloc(
                    (*b).yy_ch_buf as *mut libc::c_void,
                    ((*b).yy_buf_size + 2 as libc::c_int) as yy_size_t,
                ) as *mut libc::c_char;
            } else {
                (*b).yy_ch_buf = 0 as *mut libc::c_char;
            }
            if ((*b).yy_ch_buf).is_null() {
                yy_fatal_error(
                    b"fatal error - scanner input buffer overflow\0" as *const u8
                        as *const libc::c_char,
                );
            }
            yy_c_buf_p = &mut *((*b).yy_ch_buf).offset(yy_c_buf_p_offset as isize)
                as *mut libc::c_char;
            num_to_read = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                .yy_buf_size - number_to_move - 1 as libc::c_int;
        }
        if num_to_read > 8192 as libc::c_int {
            num_to_read = 8192 as libc::c_int;
        }
        if (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_is_interactive
            != 0
        {
            let mut c: libc::c_int = '*' as i32;
            let mut n: libc::c_int = 0;
            n = 0 as libc::c_int;
            while n < num_to_read
                && {
                    c = getc(yyin);
                    c != -(1 as libc::c_int)
                } && c != '\n' as i32
            {
                *(&mut *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                    .yy_ch_buf)
                    .offset(number_to_move as isize) as *mut libc::c_char)
                    .offset(n as isize) = c as libc::c_char;
                n += 1;
                n;
            }
            if c == '\n' as i32 {
                let fresh5 = n;
                n = n + 1;
                *(&mut *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                    .yy_ch_buf)
                    .offset(number_to_move as isize) as *mut libc::c_char)
                    .offset(fresh5 as isize) = c as libc::c_char;
            }
            if c == -(1 as libc::c_int) && ferror(yyin) != 0 {
                yy_fatal_error(
                    b"input in flex scanner failed\0" as *const u8 as *const libc::c_char,
                );
            }
            yy_n_chars = n;
        } else {
            *__errno_location() = 0 as libc::c_int;
            loop {
                yy_n_chars = fread(
                    &mut *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                        .yy_ch_buf)
                        .offset(number_to_move as isize) as *mut libc::c_char
                        as *mut libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                    num_to_read as yy_size_t,
                    yyin,
                ) as libc::c_int;
                if !(yy_n_chars == 0 as libc::c_int && ferror(yyin) != 0) {
                    break;
                }
                if *__errno_location() != 4 as libc::c_int {
                    yy_fatal_error(
                        b"input in flex scanner failed\0" as *const u8
                            as *const libc::c_char,
                    );
                } else {
                    *__errno_location() = 0 as libc::c_int;
                    clearerr(yyin);
                }
            }
        }
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars = yy_n_chars;
    }
    if yy_n_chars == 0 as libc::c_int {
        if number_to_move == 0 as libc::c_int {
            ret_val = 1 as libc::c_int;
            yyrestart(yyin);
        } else {
            ret_val = 2 as libc::c_int;
            (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                .yy_buffer_status = 2 as libc::c_int;
        }
    } else {
        ret_val = 0 as libc::c_int;
    }
    if yy_n_chars + number_to_move
        > (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_buf_size
    {
        let mut new_size_0: libc::c_int = yy_n_chars + number_to_move
            + (yy_n_chars >> 1 as libc::c_int);
        let ref mut fresh6 = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
            .yy_ch_buf;
        *fresh6 = yyrealloc(
            (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf
                as *mut libc::c_void,
            new_size_0 as yy_size_t,
        ) as *mut libc::c_char;
        if ((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf).is_null()
        {
            yy_fatal_error(
                b"out of dynamic memory in yy_get_next_buffer()\0" as *const u8
                    as *const libc::c_char,
            );
        }
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
            .yy_buf_size = new_size_0 - 2 as libc::c_int;
    }
    yy_n_chars += number_to_move;
    *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
        .offset(yy_n_chars as isize) = 0 as libc::c_int as libc::c_char;
    *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
        .offset(
            (yy_n_chars + 1 as libc::c_int) as isize,
        ) = 0 as libc::c_int as libc::c_char;
    yytext = &mut *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
        .offset(0 as libc::c_int as isize) as *mut libc::c_char;
    return ret_val;
}
unsafe extern "C" fn yy_try_NUL_trans(
    mut yy_current_state: yy_state_type,
) -> yy_state_type {
    let mut yy_is_jam: libc::c_int = 0;
    let mut yy_cp: *mut libc::c_char = yy_c_buf_p;
    let mut yy_c: YY_CHAR = 1 as libc::c_int as YY_CHAR;
    if yy_accept[yy_current_state as usize] != 0 {
        yy_last_accepting_state = yy_current_state;
        yy_last_accepting_cpos = yy_cp;
    }
    while yy_chk[(yy_base[yy_current_state as usize] as libc::c_int
        + yy_c as libc::c_int) as usize] as libc::c_int != yy_current_state
    {
        yy_current_state = yy_def[yy_current_state as usize] as libc::c_int;
        if yy_current_state >= 290 as libc::c_int {
            yy_c = yy_meta[yy_c as usize];
        }
    }
    yy_current_state = yy_nxt[(yy_base[yy_current_state as usize] as libc::c_int
        + yy_c as libc::c_int) as usize] as yy_state_type;
    yy_is_jam = (yy_current_state == 289 as libc::c_int) as libc::c_int;
    return if yy_is_jam != 0 { 0 as libc::c_int } else { yy_current_state };
}
pub static mut yyout: *mut FILE = 0 as *const FILE as *mut FILE;
pub static mut yylineno: libc::c_int = 1 as libc::c_int;
static mut yy_rule_can_match_eol: [flex_int32_t; 56] = [
    0 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    0 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    0 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    1 as libc::c_int,
    0 as libc::c_int,
    1 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
];
static mut yy_init: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn yy_switch_to_buffer(mut new_buffer: YY_BUFFER_STATE) {
    yyensure_buffer_stack();
    if (if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }) == new_buffer
    {
        return;
    }
    if !if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        *yy_c_buf_p = yy_hold_char;
        let ref mut fresh7 = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
            .yy_buf_pos;
        *fresh7 = yy_c_buf_p;
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars = yy_n_chars;
    }
    let ref mut fresh8 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    *fresh8 = new_buffer;
    yy_load_buffer_state();
    yy_did_buffer_switch_on_eof = 1 as libc::c_int;
}
pub unsafe extern "C" fn yy_delete_buffer(mut b: YY_BUFFER_STATE) {
    if b.is_null() {
        return;
    }
    if b
        == (if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else {
            0 as YY_BUFFER_STATE
        })
    {
        let ref mut fresh9 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
        *fresh9 = 0 as YY_BUFFER_STATE;
    }
    if (*b).yy_is_our_buffer != 0 {
        yyfree((*b).yy_ch_buf as *mut libc::c_void);
    }
    yyfree(b as *mut libc::c_void);
}
pub unsafe extern "C" fn yyfree(mut ptr: *mut libc::c_void) {
    free(ptr as *mut libc::c_char as *mut libc::c_void);
}
pub unsafe extern "C" fn yypush_buffer_state(mut new_buffer: YY_BUFFER_STATE) {
    if new_buffer.is_null() {
        return;
    }
    yyensure_buffer_stack();
    if !if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        *yy_c_buf_p = yy_hold_char;
        let ref mut fresh10 = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
            .yy_buf_pos;
        *fresh10 = yy_c_buf_p;
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars = yy_n_chars;
    }
    if !if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        yy_buffer_stack_top = yy_buffer_stack_top.wrapping_add(1);
        yy_buffer_stack_top;
    }
    let ref mut fresh11 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    *fresh11 = new_buffer;
    yy_load_buffer_state();
    yy_did_buffer_switch_on_eof = 1 as libc::c_int;
}
pub unsafe extern "C" fn yypop_buffer_state() {
    if if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        return;
    }
    yy_delete_buffer(
        if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else {
            0 as YY_BUFFER_STATE
        },
    );
    let ref mut fresh12 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    *fresh12 = 0 as YY_BUFFER_STATE;
    if yy_buffer_stack_top > 0 as libc::c_int as libc::c_ulong {
        yy_buffer_stack_top = yy_buffer_stack_top.wrapping_sub(1);
        yy_buffer_stack_top;
    }
    if !if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        yy_load_buffer_state();
        yy_did_buffer_switch_on_eof = 1 as libc::c_int;
    }
}
pub unsafe extern "C" fn yy_scan_buffer(
    mut base: *mut libc::c_char,
    mut size: yy_size_t,
) -> YY_BUFFER_STATE {
    let mut b: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    if size < 2 as libc::c_int as libc::c_ulong
        || *base.offset(size.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int != 0 as libc::c_int
        || *base.offset(size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int != 0 as libc::c_int
    {
        return 0 as YY_BUFFER_STATE;
    }
    b = yyalloc(::std::mem::size_of::<yy_buffer_state>() as libc::c_ulong)
        as YY_BUFFER_STATE;
    if b.is_null() {
        yy_fatal_error(
            b"out of dynamic memory in yy_scan_buffer()\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*b)
        .yy_buf_size = size.wrapping_sub(2 as libc::c_int as libc::c_ulong)
        as libc::c_int;
    (*b).yy_ch_buf = base;
    (*b).yy_buf_pos = (*b).yy_ch_buf;
    (*b).yy_is_our_buffer = 0 as libc::c_int;
    (*b).yy_input_file = 0 as *mut FILE;
    (*b).yy_n_chars = (*b).yy_buf_size;
    (*b).yy_is_interactive = 0 as libc::c_int;
    (*b).yy_at_bol = 1 as libc::c_int;
    (*b).yy_fill_buffer = 0 as libc::c_int;
    (*b).yy_buffer_status = 0 as libc::c_int;
    yy_switch_to_buffer(b);
    return b;
}
pub unsafe extern "C" fn yy_scan_string(
    mut yystr: *const libc::c_char,
) -> YY_BUFFER_STATE {
    return yy_scan_bytes(yystr, strlen(yystr) as libc::c_int);
}
pub unsafe extern "C" fn yy_scan_bytes(
    mut yybytes: *const libc::c_char,
    mut _yybytes_len: libc::c_int,
) -> YY_BUFFER_STATE {
    let mut b: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: yy_size_t = 0;
    let mut i: libc::c_int = 0;
    n = (_yybytes_len + 2 as libc::c_int) as yy_size_t;
    buf = yyalloc(n) as *mut libc::c_char;
    if buf.is_null() {
        yy_fatal_error(
            b"out of dynamic memory in yy_scan_bytes()\0" as *const u8
                as *const libc::c_char,
        );
    }
    i = 0 as libc::c_int;
    while i < _yybytes_len {
        *buf.offset(i as isize) = *yybytes.offset(i as isize);
        i += 1;
        i;
    }
    let ref mut fresh13 = *buf.offset((_yybytes_len + 1 as libc::c_int) as isize);
    *fresh13 = 0 as libc::c_int as libc::c_char;
    *buf.offset(_yybytes_len as isize) = *fresh13;
    b = yy_scan_buffer(buf, n);
    if b.is_null() {
        yy_fatal_error(
            b"bad buffer in yy_scan_bytes()\0" as *const u8 as *const libc::c_char,
        );
    }
    (*b).yy_is_our_buffer = 1 as libc::c_int;
    return b;
}
pub static mut yy_flex_debug: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn yyset_lineno(mut _line_number: libc::c_int) {
    yylineno = _line_number;
}
pub unsafe extern "C" fn yyget_lineno() -> libc::c_int {
    return yylineno;
}
pub unsafe extern "C" fn yyget_text() -> *mut libc::c_char {
    return yytext;
}
pub unsafe extern "C" fn yyget_leng() -> libc::c_int {
    return yyleng;
}
pub unsafe extern "C" fn yyset_out(mut _out_str: *mut FILE) {
    yyout = _out_str;
}
pub unsafe extern "C" fn yyget_out() -> *mut FILE {
    return yyout;
}
pub unsafe extern "C" fn yyset_in(mut _in_str: *mut FILE) {
    yyin = _in_str;
}
pub unsafe extern "C" fn yyget_in() -> *mut FILE {
    return yyin;
}
pub unsafe extern "C" fn yyset_debug(mut _bdebug: libc::c_int) {
    yy_flex_debug = _bdebug;
}
pub unsafe extern "C" fn yyget_debug() -> libc::c_int {
    return yy_flex_debug;
}
pub unsafe extern "C" fn yylex_destroy() -> libc::c_int {
    while !if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        yy_delete_buffer(
            if !yy_buffer_stack.is_null() {
                *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
            } else {
                0 as YY_BUFFER_STATE
            },
        );
        let ref mut fresh14 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
        *fresh14 = 0 as YY_BUFFER_STATE;
        yypop_buffer_state();
    }
    yyfree(yy_buffer_stack as *mut libc::c_void);
    yy_buffer_stack = 0 as *mut YY_BUFFER_STATE;
    yy_init_globals();
    return 0 as libc::c_int;
}
unsafe extern "C" fn yy_init_globals() -> libc::c_int {
    yylineno = 1 as libc::c_int;
    yy_buffer_stack = 0 as *mut YY_BUFFER_STATE;
    yy_buffer_stack_top = 0 as libc::c_int as size_t;
    yy_buffer_stack_max = 0 as libc::c_int as size_t;
    yy_c_buf_p = 0 as *mut libc::c_char;
    yy_init = 0 as libc::c_int;
    yy_start = 0 as libc::c_int;
    yyin = 0 as *mut FILE;
    yyout = 0 as *mut FILE;
    return 0 as libc::c_int;
}
unsafe extern "C" fn yyerror(mut p: *mut parser_state, mut s: *const libc::c_char) {
    (*p).nerr += 1;
    (*p).nerr;
    if !((*p).fname).is_null() {
        fprintf(
            stderr,
            b"%s:%d:%s\n\0" as *const u8 as *const libc::c_char,
            (*p).fname,
            (*p).lineno,
            s,
        );
    } else {
        fprintf(
            stderr,
            b"%d:%s\n\0" as *const u8 as *const libc::c_char,
            (*p).lineno,
            s,
        );
    };
}
static mut yydefact: [yytype_uint8; 254] = [
    123 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    24 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    73 as libc::c_int as yytype_uint8,
    74 as libc::c_int as yytype_uint8,
    75 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    67 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    66 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    123 as libc::c_int as yytype_uint8,
    119 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    128 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    123 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    68 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    27 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    72 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    124 as libc::c_int as yytype_uint8,
    125 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    68 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    25 as libc::c_int as yytype_uint8,
    62 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    26 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    10 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    82 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    117 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    123 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    68 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    123 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    124 as libc::c_int as yytype_uint8,
    68 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    120 as libc::c_int as yytype_uint8,
    71 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    124 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    77 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    123 as libc::c_int as yytype_uint8,
    126 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    61 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    123 as libc::c_int as yytype_uint8,
    123 as libc::c_int as yytype_uint8,
    119 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    119 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    88 as libc::c_int as yytype_uint8,
    89 as libc::c_int as yytype_uint8,
    90 as libc::c_int as yytype_uint8,
    107 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    86 as libc::c_int as yytype_uint8,
    87 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    85 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    100 as libc::c_int as yytype_uint8,
    105 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    123 as libc::c_int as yytype_uint8,
    113 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    124 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    115 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    118 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    123 as libc::c_int as yytype_uint8,
    69 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    123 as libc::c_int as yytype_uint8,
    70 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    23 as libc::c_int as yytype_uint8,
    31 as libc::c_int as yytype_uint8,
    32 as libc::c_int as yytype_uint8,
    33 as libc::c_int as yytype_uint8,
    34 as libc::c_int as yytype_uint8,
    35 as libc::c_int as yytype_uint8,
    42 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    40 as libc::c_int as yytype_uint8,
    41 as libc::c_int as yytype_uint8,
    38 as libc::c_int as yytype_uint8,
    39 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    36 as libc::c_int as yytype_uint8,
    37 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    63 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    121 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    22 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    98 as libc::c_int as yytype_uint8,
    91 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    109 as libc::c_int as yytype_uint8,
    111 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    123 as libc::c_int as yytype_uint8,
    123 as libc::c_int as yytype_uint8,
    114 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    80 as libc::c_int as yytype_uint8,
    84 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    9 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    76 as libc::c_int as yytype_uint8,
    108 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    93 as libc::c_int as yytype_uint8,
    95 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    97 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    112 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    78 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    123 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    123 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    104 as libc::c_int as yytype_uint8,
    92 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    101 as libc::c_int as yytype_uint8,
    106 as libc::c_int as yytype_uint8,
    99 as libc::c_int as yytype_uint8,
    110 as libc::c_int as yytype_uint8,
    116 as libc::c_int as yytype_uint8,
    81 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    12 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    94 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    79 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    11 as libc::c_int as yytype_uint8,
    102 as libc::c_int as yytype_uint8,
];
static mut yypgoto: [yytype_int16; 31] = [
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    93 as libc::c_int as yytype_int16,
    194 as libc::c_int as yytype_int16,
    154 as libc::c_int as yytype_int16,
    -(66 as libc::c_int) as yytype_int16,
    173 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    3 as libc::c_int as yytype_int16,
    130 as libc::c_int as yytype_int16,
    216 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(189 as libc::c_int) as yytype_int16,
    15 as libc::c_int as yytype_int16,
    -(210 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(125 as libc::c_int) as yytype_int16,
    104 as libc::c_int as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    -(34 as libc::c_int) as yytype_int16,
    -(211 as libc::c_int) as yytype_int16,
    62 as libc::c_int as yytype_int16,
    220 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    -(20 as libc::c_int) as yytype_int16,
];
static mut yydefgoto: [yytype_int16; 31] = [
    -(1 as libc::c_int) as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    67 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    35 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    173 as libc::c_int as yytype_int16,
    48 as libc::c_int as yytype_int16,
    49 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    205 as libc::c_int as yytype_int16,
    130 as libc::c_int as yytype_int16,
    131 as libc::c_int as yytype_int16,
    132 as libc::c_int as yytype_int16,
    133 as libc::c_int as yytype_int16,
    134 as libc::c_int as yytype_int16,
    71 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    72 as libc::c_int as yytype_int16,
    78 as libc::c_int as yytype_int16,
    79 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
];
static mut yytable: [yytype_int16; 794] = [
    34 as libc::c_int as yytype_int16,
    87 as libc::c_int as yytype_int16,
    -(121 as libc::c_int) as yytype_int16,
    187 as libc::c_int as yytype_int16,
    233 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    141 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    -(30 as libc::c_int) as yytype_int16,
    47 as libc::c_int as yytype_int16,
    -(30 as libc::c_int) as yytype_int16,
    47 as libc::c_int as yytype_int16,
    142 as libc::c_int as yytype_int16,
    226 as libc::c_int as yytype_int16,
    138 as libc::c_int as yytype_int16,
    139 as libc::c_int as yytype_int16,
    52 as libc::c_int as yytype_int16,
    119 as libc::c_int as yytype_int16,
    120 as libc::c_int as yytype_int16,
    121 as libc::c_int as yytype_int16,
    106 as libc::c_int as yytype_int16,
    60 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    70 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    218 as libc::c_int as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    63 as libc::c_int as yytype_int16,
    64 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    77 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    -(121 as libc::c_int) as yytype_int16,
    33 as libc::c_int as yytype_int16,
    241 as libc::c_int as yytype_int16,
    253 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
    124 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    125 as libc::c_int as yytype_int16,
    126 as libc::c_int as yytype_int16,
    143 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    220 as libc::c_int as yytype_int16,
    107 as libc::c_int as yytype_int16,
    108 as libc::c_int as yytype_int16,
    109 as libc::c_int as yytype_int16,
    110 as libc::c_int as yytype_int16,
    250 as libc::c_int as yytype_int16,
    191 as libc::c_int as yytype_int16,
    106 as libc::c_int as yytype_int16,
    82 as libc::c_int as yytype_int16,
    128 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    140 as libc::c_int as yytype_int16,
    -(121 as libc::c_int) as yytype_int16,
    -(121 as libc::c_int) as yytype_int16,
    104 as libc::c_int as yytype_int16,
    106 as libc::c_int as yytype_int16,
    129 as libc::c_int as yytype_int16,
    62 as libc::c_int as yytype_int16,
    221 as libc::c_int as yytype_int16,
    193 as libc::c_int as yytype_int16,
    229 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    192 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    230 as libc::c_int as yytype_int16,
    83 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    214 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    84 as libc::c_int as yytype_int16,
    231 as libc::c_int as yytype_int16,
    188 as libc::c_int as yytype_int16,
    201 as libc::c_int as yytype_int16,
    112 as libc::c_int as yytype_int16,
    232 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    55 as libc::c_int as yytype_int16,
    56 as libc::c_int as yytype_int16,
    152 as libc::c_int as yytype_int16,
    151 as libc::c_int as yytype_int16,
    235 as libc::c_int as yytype_int16,
    135 as libc::c_int as yytype_int16,
    150 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    -(121 as libc::c_int) as yytype_int16,
    168 as libc::c_int as yytype_int16,
    153 as libc::c_int as yytype_int16,
    154 as libc::c_int as yytype_int16,
    155 as libc::c_int as yytype_int16,
    156 as libc::c_int as yytype_int16,
    157 as libc::c_int as yytype_int16,
    158 as libc::c_int as yytype_int16,
    159 as libc::c_int as yytype_int16,
    160 as libc::c_int as yytype_int16,
    161 as libc::c_int as yytype_int16,
    162 as libc::c_int as yytype_int16,
    163 as libc::c_int as yytype_int16,
    164 as libc::c_int as yytype_int16,
    165 as libc::c_int as yytype_int16,
    166 as libc::c_int as yytype_int16,
    167 as libc::c_int as yytype_int16,
    136 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    177 as libc::c_int as yytype_int16,
    137 as libc::c_int as yytype_int16,
    177 as libc::c_int as yytype_int16,
    106 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    116 as libc::c_int as yytype_int16,
    181 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    179 as libc::c_int as yytype_int16,
    129 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    182 as libc::c_int as yytype_int16,
    117 as libc::c_int as yytype_int16,
    129 as libc::c_int as yytype_int16,
    129 as libc::c_int as yytype_int16,
    223 as libc::c_int as yytype_int16,
    224 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    249 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    206 as libc::c_int as yytype_int16,
    188 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    183 as libc::c_int as yytype_int16,
    129 as libc::c_int as yytype_int16,
    55 as libc::c_int as yytype_int16,
    56 as libc::c_int as yytype_int16,
    184 as libc::c_int as yytype_int16,
    198 as libc::c_int as yytype_int16,
    91 as libc::c_int as yytype_int16,
    92 as libc::c_int as yytype_int16,
    93 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    170 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    143 as libc::c_int as yytype_int16,
    84 as libc::c_int as yytype_int16,
    200 as libc::c_int as yytype_int16,
    169 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    146 as libc::c_int as yytype_int16,
    147 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    126 as libc::c_int as yytype_int16,
    114 as libc::c_int as yytype_int16,
    243 as libc::c_int as yytype_int16,
    115 as libc::c_int as yytype_int16,
    245 as libc::c_int as yytype_int16,
    188 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    206 as libc::c_int as yytype_int16,
    148 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    203 as libc::c_int as yytype_int16,
    204 as libc::c_int as yytype_int16,
    195 as libc::c_int as yytype_int16,
    194 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    178 as libc::c_int as yytype_int16,
    189 as libc::c_int as yytype_int16,
    180 as libc::c_int as yytype_int16,
    207 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    91 as libc::c_int as yytype_int16,
    92 as libc::c_int as yytype_int16,
    93 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    199 as libc::c_int as yytype_int16,
    190 as libc::c_int as yytype_int16,
    129 as libc::c_int as yytype_int16,
    197 as libc::c_int as yytype_int16,
    137 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    206 as libc::c_int as yytype_int16,
    202 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    208 as libc::c_int as yytype_int16,
    222 as libc::c_int as yytype_int16,
    209 as libc::c_int as yytype_int16,
    119 as libc::c_int as yytype_int16,
    120 as libc::c_int as yytype_int16,
    121 as libc::c_int as yytype_int16,
    210 as libc::c_int as yytype_int16,
    219 as libc::c_int as yytype_int16,
    175 as libc::c_int as yytype_int16,
    176 as libc::c_int as yytype_int16,
    228 as libc::c_int as yytype_int16,
    206 as libc::c_int as yytype_int16,
    215 as libc::c_int as yytype_int16,
    211 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    212 as libc::c_int as yytype_int16,
    217 as libc::c_int as yytype_int16,
    129 as libc::c_int as yytype_int16,
    129 as libc::c_int as yytype_int16,
    216 as libc::c_int as yytype_int16,
    225 as libc::c_int as yytype_int16,
    129 as libc::c_int as yytype_int16,
    240 as libc::c_int as yytype_int16,
    129 as libc::c_int as yytype_int16,
    129 as libc::c_int as yytype_int16,
    247 as libc::c_int as yytype_int16,
    227 as libc::c_int as yytype_int16,
    242 as libc::c_int as yytype_int16,
    124 as libc::c_int as yytype_int16,
    251 as libc::c_int as yytype_int16,
    125 as libc::c_int as yytype_int16,
    126 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    248 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    252 as libc::c_int as yytype_int16,
    236 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    237 as libc::c_int as yytype_int16,
    238 as libc::c_int as yytype_int16,
    244 as libc::c_int as yytype_int16,
    149 as libc::c_int as yytype_int16,
    246 as libc::c_int as yytype_int16,
    128 as libc::c_int as yytype_int16,
    174 as libc::c_int as yytype_int16,
    81 as libc::c_int as yytype_int16,
    196 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    129 as libc::c_int as yytype_int16,
    144 as libc::c_int as yytype_int16,
    129 as libc::c_int as yytype_int16,
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
    1 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    219 as libc::c_int as yytype_int16,
    2 as libc::c_int as yytype_int16,
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
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    91 as libc::c_int as yytype_int16,
    92 as libc::c_int as yytype_int16,
    93 as libc::c_int as yytype_int16,
    94 as libc::c_int as yytype_int16,
    95 as libc::c_int as yytype_int16,
    96 as libc::c_int as yytype_int16,
    97 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    99 as libc::c_int as yytype_int16,
    100 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    65 as libc::c_int as yytype_int16,
    2 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    66 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    91 as libc::c_int as yytype_int16,
    92 as libc::c_int as yytype_int16,
    93 as libc::c_int as yytype_int16,
    -(122 as libc::c_int) as yytype_int16,
    -(122 as libc::c_int) as yytype_int16,
    96 as libc::c_int as yytype_int16,
    97 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    99 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    2 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    91 as libc::c_int as yytype_int16,
    92 as libc::c_int as yytype_int16,
    93 as libc::c_int as yytype_int16,
    94 as libc::c_int as yytype_int16,
    95 as libc::c_int as yytype_int16,
    96 as libc::c_int as yytype_int16,
    97 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    99 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
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
    0 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    80 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
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
    0 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
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
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    118 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    119 as libc::c_int as yytype_int16,
    120 as libc::c_int as yytype_int16,
    121 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    122 as libc::c_int as yytype_int16,
    119 as libc::c_int as yytype_int16,
    120 as libc::c_int as yytype_int16,
    121 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    123 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    123 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    124 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    125 as libc::c_int as yytype_int16,
    126 as libc::c_int as yytype_int16,
    127 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    124 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    125 as libc::c_int as yytype_int16,
    126 as libc::c_int as yytype_int16,
    127 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    128 as libc::c_int as yytype_int16,
    119 as libc::c_int as yytype_int16,
    120 as libc::c_int as yytype_int16,
    121 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    128 as libc::c_int as yytype_int16,
    185 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    186 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    123 as libc::c_int as yytype_int16,
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
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    172 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    124 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    125 as libc::c_int as yytype_int16,
    126 as libc::c_int as yytype_int16,
    127 as libc::c_int as yytype_int16,
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
    128 as libc::c_int as yytype_int16,
    234 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    91 as libc::c_int as yytype_int16,
    92 as libc::c_int as yytype_int16,
    93 as libc::c_int as yytype_int16,
    94 as libc::c_int as yytype_int16,
    95 as libc::c_int as yytype_int16,
    96 as libc::c_int as yytype_int16,
    97 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    99 as libc::c_int as yytype_int16,
    100 as libc::c_int as yytype_int16,
    101 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    91 as libc::c_int as yytype_int16,
    92 as libc::c_int as yytype_int16,
    93 as libc::c_int as yytype_int16,
    94 as libc::c_int as yytype_int16,
    95 as libc::c_int as yytype_int16,
    96 as libc::c_int as yytype_int16,
    97 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    99 as libc::c_int as yytype_int16,
    100 as libc::c_int as yytype_int16,
    101 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    91 as libc::c_int as yytype_int16,
    92 as libc::c_int as yytype_int16,
    93 as libc::c_int as yytype_int16,
    94 as libc::c_int as yytype_int16,
    95 as libc::c_int as yytype_int16,
    96 as libc::c_int as yytype_int16,
    97 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    99 as libc::c_int as yytype_int16,
    100 as libc::c_int as yytype_int16,
    101 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    145 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    91 as libc::c_int as yytype_int16,
    92 as libc::c_int as yytype_int16,
    93 as libc::c_int as yytype_int16,
    94 as libc::c_int as yytype_int16,
    95 as libc::c_int as yytype_int16,
    96 as libc::c_int as yytype_int16,
    97 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    99 as libc::c_int as yytype_int16,
    100 as libc::c_int as yytype_int16,
    101 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
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
    0 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    171 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    91 as libc::c_int as yytype_int16,
    92 as libc::c_int as yytype_int16,
    93 as libc::c_int as yytype_int16,
    94 as libc::c_int as yytype_int16,
    95 as libc::c_int as yytype_int16,
    96 as libc::c_int as yytype_int16,
    97 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    99 as libc::c_int as yytype_int16,
    100 as libc::c_int as yytype_int16,
    101 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    213 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    91 as libc::c_int as yytype_int16,
    92 as libc::c_int as yytype_int16,
    93 as libc::c_int as yytype_int16,
    94 as libc::c_int as yytype_int16,
    95 as libc::c_int as yytype_int16,
    96 as libc::c_int as yytype_int16,
    97 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    99 as libc::c_int as yytype_int16,
    100 as libc::c_int as yytype_int16,
    101 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    239 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    91 as libc::c_int as yytype_int16,
    92 as libc::c_int as yytype_int16,
    93 as libc::c_int as yytype_int16,
    94 as libc::c_int as yytype_int16,
    95 as libc::c_int as yytype_int16,
    96 as libc::c_int as yytype_int16,
    97 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    99 as libc::c_int as yytype_int16,
    100 as libc::c_int as yytype_int16,
    101 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    91 as libc::c_int as yytype_int16,
    92 as libc::c_int as yytype_int16,
    93 as libc::c_int as yytype_int16,
    94 as libc::c_int as yytype_int16,
    95 as libc::c_int as yytype_int16,
    96 as libc::c_int as yytype_int16,
    97 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    99 as libc::c_int as yytype_int16,
    100 as libc::c_int as yytype_int16,
    101 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    91 as libc::c_int as yytype_int16,
    92 as libc::c_int as yytype_int16,
    93 as libc::c_int as yytype_int16,
    94 as libc::c_int as yytype_int16,
    95 as libc::c_int as yytype_int16,
    96 as libc::c_int as yytype_int16,
    97 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    99 as libc::c_int as yytype_int16,
    100 as libc::c_int as yytype_int16,
    101 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
];
static mut yycheck: [yytype_int16; 794] = [
    0 as libc::c_int as yytype_int16,
    35 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    128 as libc::c_int as yytype_int16,
    214 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    72 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    49 as libc::c_int as yytype_int16,
    2 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    202 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    49 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    227 as libc::c_int as yytype_int16,
    249 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    242 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    55 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    84 as libc::c_int as yytype_int16,
    65 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    134 as libc::c_int as yytype_int16,
    49 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    72 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    49 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    147 as libc::c_int as yytype_int16,
    49 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    84 as libc::c_int as yytype_int16,
    49 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    215 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    91 as libc::c_int as yytype_int16,
    92 as libc::c_int as yytype_int16,
    93 as libc::c_int as yytype_int16,
    94 as libc::c_int as yytype_int16,
    95 as libc::c_int as yytype_int16,
    96 as libc::c_int as yytype_int16,
    97 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    99 as libc::c_int as yytype_int16,
    100 as libc::c_int as yytype_int16,
    101 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    112 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    114 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    116 as libc::c_int as yytype_int16,
    137 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    117 as libc::c_int as yytype_int16,
    84 as libc::c_int as yytype_int16,
    115 as libc::c_int as yytype_int16,
    123 as libc::c_int as yytype_int16,
    117 as libc::c_int as yytype_int16,
    118 as libc::c_int as yytype_int16,
    55 as libc::c_int as yytype_int16,
    127 as libc::c_int as yytype_int16,
    128 as libc::c_int as yytype_int16,
    195 as libc::c_int as yytype_int16,
    196 as libc::c_int as yytype_int16,
    49 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    134 as libc::c_int as yytype_int16,
    169 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    137 as libc::c_int as yytype_int16,
    123 as libc::c_int as yytype_int16,
    139 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    127 as libc::c_int as yytype_int16,
    143 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    147 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    112 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    146 as libc::c_int as yytype_int16,
    104 as libc::c_int as yytype_int16,
    112 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    112 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    229 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    231 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    202 as libc::c_int as yytype_int16,
    56 as libc::c_int as yytype_int16,
    49 as libc::c_int as yytype_int16,
    168 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    137 as libc::c_int as yytype_int16,
    168 as libc::c_int as yytype_int16,
    114 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    116 as libc::c_int as yytype_int16,
    172 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
    60 as libc::c_int as yytype_int16,
    144 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    189 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    144 as libc::c_int as yytype_int16,
    49 as libc::c_int as yytype_int16,
    227 as libc::c_int as yytype_int16,
    52 as libc::c_int as yytype_int16,
    195 as libc::c_int as yytype_int16,
    196 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    191 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    52 as libc::c_int as yytype_int16,
    189 as libc::c_int as yytype_int16,
    112 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    204 as libc::c_int as yytype_int16,
    242 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    52 as libc::c_int as yytype_int16,
    204 as libc::c_int as yytype_int16,
    56 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    214 as libc::c_int as yytype_int16,
    215 as libc::c_int as yytype_int16,
    56 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    218 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    220 as libc::c_int as yytype_int16,
    221 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    52 as libc::c_int as yytype_int16,
    52 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    229 as libc::c_int as yytype_int16,
    56 as libc::c_int as yytype_int16,
    231 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    218 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    220 as libc::c_int as yytype_int16,
    221 as libc::c_int as yytype_int16,
    230 as libc::c_int as yytype_int16,
    84 as libc::c_int as yytype_int16,
    232 as libc::c_int as yytype_int16,
    55 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    139 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    247 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    249 as libc::c_int as yytype_int16,
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
    3 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    247 as libc::c_int as yytype_int16,
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
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
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
    37 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    49 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    51 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    55 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    59 as libc::c_int as yytype_int16,
    60 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    5 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    12 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    20 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
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
    -(1 as libc::c_int) as yytype_int16,
    37 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    49 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    51 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    3 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    55 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
    60 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    12 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
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
    -(1 as libc::c_int) as yytype_int16,
    37 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    49 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    55 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    59 as libc::c_int as yytype_int16,
    60 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
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
    -(1 as libc::c_int) as yytype_int16,
    37 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    49 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    51 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    55 as libc::c_int as yytype_int16,
    56 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
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
    -(1 as libc::c_int) as yytype_int16,
    37 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    49 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    51 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    55 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
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
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    37 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    49 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    51 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    55 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    20 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    25 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    25 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    40 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    40 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    55 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    55 as libc::c_int as yytype_int16,
    56 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    58 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    25 as libc::c_int as yytype_int16,
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
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    4 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    40 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
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
    55 as libc::c_int as yytype_int16,
    56 as libc::c_int as yytype_int16,
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
    37 as libc::c_int as yytype_int16,
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
    37 as libc::c_int as yytype_int16,
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
    52 as libc::c_int as yytype_int16,
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
    37 as libc::c_int as yytype_int16,
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
    -(1 as libc::c_int) as yytype_int16,
    19 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    52 as libc::c_int as yytype_int16,
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
    37 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
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
    37 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
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
    37 as libc::c_int as yytype_int16,
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
    37 as libc::c_int as yytype_int16,
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
    37 as libc::c_int as yytype_int16,
];
static mut yystos: [yytype_int8; 254] = [
    0 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
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
    23 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    49 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    55 as libc::c_int as yytype_int8,
    59 as libc::c_int as yytype_int8,
    60 as libc::c_int as yytype_int8,
    62 as libc::c_int as yytype_int8,
    63 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    65 as libc::c_int as yytype_int8,
    68 as libc::c_int as yytype_int8,
    69 as libc::c_int as yytype_int8,
    70 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    77 as libc::c_int as yytype_int8,
    85 as libc::c_int as yytype_int8,
    89 as libc::c_int as yytype_int8,
    90 as libc::c_int as yytype_int8,
    91 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    72 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    44 as libc::c_int as yytype_int8,
    69 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    74 as libc::c_int as yytype_int8,
    75 as libc::c_int as yytype_int8,
    76 as libc::c_int as yytype_int8,
    74 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    70 as libc::c_int as yytype_int8,
    70 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    70 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    66 as libc::c_int as yytype_int8,
    67 as libc::c_int as yytype_int8,
    68 as libc::c_int as yytype_int8,
    69 as libc::c_int as yytype_int8,
    84 as libc::c_int as yytype_int8,
    86 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    89 as libc::c_int as yytype_int8,
    90 as libc::c_int as yytype_int8,
    69 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    87 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    56 as libc::c_int as yytype_int8,
    76 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    89 as libc::c_int as yytype_int8,
    90 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    85 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
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
    57 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    91 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
    49 as libc::c_int as yytype_int8,
    49 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    55 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    44 as libc::c_int as yytype_int8,
    55 as libc::c_int as yytype_int8,
    69 as libc::c_int as yytype_int8,
    79 as libc::c_int as yytype_int8,
    80 as libc::c_int as yytype_int8,
    81 as libc::c_int as yytype_int8,
    82 as libc::c_int as yytype_int8,
    83 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    89 as libc::c_int as yytype_int8,
    90 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    66 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
    67 as libc::c_int as yytype_int8,
    52 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    56 as libc::c_int as yytype_int8,
    65 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    74 as libc::c_int as yytype_int8,
    69 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    70 as libc::c_int as yytype_int8,
    89 as libc::c_int as yytype_int8,
    52 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    73 as libc::c_int as yytype_int8,
    75 as libc::c_int as yytype_int8,
    63 as libc::c_int as yytype_int8,
    63 as libc::c_int as yytype_int8,
    69 as libc::c_int as yytype_int8,
    87 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    87 as libc::c_int as yytype_int8,
    74 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    79 as libc::c_int as yytype_int8,
    79 as libc::c_int as yytype_int8,
    56 as libc::c_int as yytype_int8,
    58 as libc::c_int as yytype_int8,
    82 as libc::c_int as yytype_int8,
    58 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    66 as libc::c_int as yytype_int8,
    68 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    83 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    69 as libc::c_int as yytype_int8,
    89 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    66 as libc::c_int as yytype_int8,
    52 as libc::c_int as yytype_int8,
    74 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    78 as libc::c_int as yytype_int8,
    85 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    52 as libc::c_int as yytype_int8,
    52 as libc::c_int as yytype_int8,
    56 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    56 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    79 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    44 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    66 as libc::c_int as yytype_int8,
    66 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    78 as libc::c_int as yytype_int8,
    52 as libc::c_int as yytype_int8,
    74 as libc::c_int as yytype_int8,
    49 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
    49 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
    80 as libc::c_int as yytype_int8,
    56 as libc::c_int as yytype_int8,
    82 as libc::c_int as yytype_int8,
    79 as libc::c_int as yytype_int8,
    79 as libc::c_int as yytype_int8,
    79 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    78 as libc::c_int as yytype_int8,
    52 as libc::c_int as yytype_int8,
    66 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    66 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
    56 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
    78 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    80 as libc::c_int as yytype_int8,
];
static mut yyr1: [yytype_int8; 129] = [
    0 as libc::c_int as yytype_int8,
    61 as libc::c_int as yytype_int8,
    62 as libc::c_int as yytype_int8,
    63 as libc::c_int as yytype_int8,
    63 as libc::c_int as yytype_int8,
    63 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    65 as libc::c_int as yytype_int8,
    65 as libc::c_int as yytype_int8,
    65 as libc::c_int as yytype_int8,
    65 as libc::c_int as yytype_int8,
    65 as libc::c_int as yytype_int8,
    65 as libc::c_int as yytype_int8,
    66 as libc::c_int as yytype_int8,
    66 as libc::c_int as yytype_int8,
    66 as libc::c_int as yytype_int8,
    67 as libc::c_int as yytype_int8,
    67 as libc::c_int as yytype_int8,
    68 as libc::c_int as yytype_int8,
    68 as libc::c_int as yytype_int8,
    68 as libc::c_int as yytype_int8,
    68 as libc::c_int as yytype_int8,
    68 as libc::c_int as yytype_int8,
    68 as libc::c_int as yytype_int8,
    68 as libc::c_int as yytype_int8,
    68 as libc::c_int as yytype_int8,
    68 as libc::c_int as yytype_int8,
    69 as libc::c_int as yytype_int8,
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
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
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
    73 as libc::c_int as yytype_int8,
    73 as libc::c_int as yytype_int8,
    74 as libc::c_int as yytype_int8,
    74 as libc::c_int as yytype_int8,
    75 as libc::c_int as yytype_int8,
    75 as libc::c_int as yytype_int8,
    75 as libc::c_int as yytype_int8,
    76 as libc::c_int as yytype_int8,
    76 as libc::c_int as yytype_int8,
    77 as libc::c_int as yytype_int8,
    77 as libc::c_int as yytype_int8,
    77 as libc::c_int as yytype_int8,
    77 as libc::c_int as yytype_int8,
    77 as libc::c_int as yytype_int8,
    77 as libc::c_int as yytype_int8,
    77 as libc::c_int as yytype_int8,
    77 as libc::c_int as yytype_int8,
    77 as libc::c_int as yytype_int8,
    77 as libc::c_int as yytype_int8,
    77 as libc::c_int as yytype_int8,
    77 as libc::c_int as yytype_int8,
    77 as libc::c_int as yytype_int8,
    77 as libc::c_int as yytype_int8,
    77 as libc::c_int as yytype_int8,
    77 as libc::c_int as yytype_int8,
    77 as libc::c_int as yytype_int8,
    77 as libc::c_int as yytype_int8,
    77 as libc::c_int as yytype_int8,
    78 as libc::c_int as yytype_int8,
    78 as libc::c_int as yytype_int8,
    79 as libc::c_int as yytype_int8,
    79 as libc::c_int as yytype_int8,
    79 as libc::c_int as yytype_int8,
    79 as libc::c_int as yytype_int8,
    79 as libc::c_int as yytype_int8,
    79 as libc::c_int as yytype_int8,
    79 as libc::c_int as yytype_int8,
    79 as libc::c_int as yytype_int8,
    79 as libc::c_int as yytype_int8,
    79 as libc::c_int as yytype_int8,
    79 as libc::c_int as yytype_int8,
    80 as libc::c_int as yytype_int8,
    80 as libc::c_int as yytype_int8,
    81 as libc::c_int as yytype_int8,
    81 as libc::c_int as yytype_int8,
    82 as libc::c_int as yytype_int8,
    82 as libc::c_int as yytype_int8,
    82 as libc::c_int as yytype_int8,
    82 as libc::c_int as yytype_int8,
    82 as libc::c_int as yytype_int8,
    82 as libc::c_int as yytype_int8,
    82 as libc::c_int as yytype_int8,
    83 as libc::c_int as yytype_int8,
    83 as libc::c_int as yytype_int8,
    83 as libc::c_int as yytype_int8,
    83 as libc::c_int as yytype_int8,
    84 as libc::c_int as yytype_int8,
    84 as libc::c_int as yytype_int8,
    85 as libc::c_int as yytype_int8,
    85 as libc::c_int as yytype_int8,
    85 as libc::c_int as yytype_int8,
    85 as libc::c_int as yytype_int8,
    86 as libc::c_int as yytype_int8,
    86 as libc::c_int as yytype_int8,
    87 as libc::c_int as yytype_int8,
    87 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    89 as libc::c_int as yytype_int8,
    89 as libc::c_int as yytype_int8,
    90 as libc::c_int as yytype_int8,
    90 as libc::c_int as yytype_int8,
    91 as libc::c_int as yytype_int8,
    91 as libc::c_int as yytype_int8,
];
static mut yyr2: [yytype_int8; 129] = [
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
];
unsafe extern "C" fn yy_symbol_value_print(
    mut yyo: *mut FILE,
    mut yytype: libc::c_int,
    yyvaluep: *const YYSTYPE,
    mut p: *mut parser_state,
) {
    let mut yyoutput: *mut FILE = yyo;
    if yyvaluep.is_null() {
        return;
    }
}
unsafe extern "C" fn yy_symbol_print(
    mut yyo: *mut FILE,
    mut yytype: libc::c_int,
    yyvaluep: *const YYSTYPE,
    mut p: *mut parser_state,
) {
    fprintf(
        yyo,
        b"%s %s (\0" as *const u8 as *const libc::c_char,
        if yytype < 61 as libc::c_int {
            b"token\0" as *const u8 as *const libc::c_char
        } else {
            b"nterm\0" as *const u8 as *const libc::c_char
        },
        yytname[yytype as usize],
    );
    yy_symbol_value_print(yyo, yytype, yyvaluep, p);
    fprintf(yyo, b")\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn yy_stack_print(
    mut yybottom: *mut yy_state_t,
    mut yytop: *mut yy_state_t,
) {
    fprintf(stderr, b"Stack now\0" as *const u8 as *const libc::c_char);
    while yybottom <= yytop {
        let mut yybot: libc::c_int = *yybottom as libc::c_int;
        fprintf(stderr, b" %d\0" as *const u8 as *const libc::c_char, yybot);
        yybottom = yybottom.offset(1);
        yybottom;
    }
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn yy_reduce_print(
    mut yyssp: *mut yy_state_t,
    mut yyvsp: *mut YYSTYPE,
    mut yyrule: libc::c_int,
    mut p: *mut parser_state,
) {
    let mut yylno: libc::c_int = yyrline[yyrule as usize] as libc::c_int;
    let mut yynrhs: libc::c_int = yyr2[yyrule as usize] as libc::c_int;
    let mut yyi: libc::c_int = 0;
    fprintf(
        stderr,
        b"Reducing stack by rule %d (line %d):\n\0" as *const u8 as *const libc::c_char,
        yyrule - 1 as libc::c_int,
        yylno,
    );
    yyi = 0 as libc::c_int;
    while yyi < yynrhs {
        fprintf(
            stderr,
            b"   $%d = \0" as *const u8 as *const libc::c_char,
            yyi + 1 as libc::c_int,
        );
        yy_symbol_print(
            stderr,
            yystos[*yyssp.offset((yyi + 1 as libc::c_int - yynrhs) as isize)
                as libc::c_int as usize] as libc::c_int,
            &mut *yyvsp.offset((yyi + 1 as libc::c_int - yynrhs) as isize),
            p,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        yyi += 1;
        yyi;
    }
}
pub static mut yydebug: libc::c_int = 0;
unsafe extern "C" fn yystpcpy(
    mut yydest: *mut libc::c_char,
    mut yysrc: *const libc::c_char,
) -> *mut libc::c_char {
    let mut yyd: *mut libc::c_char = yydest;
    let mut yys: *const libc::c_char = yysrc;
    loop {
        let fresh15 = yys;
        yys = yys.offset(1);
        let fresh16 = yyd;
        yyd = yyd.offset(1);
        *fresh16 = *fresh15;
        if !(*fresh16 as libc::c_int != '\0' as i32) {
            break;
        }
    }
    return yyd.offset(-(1 as libc::c_int as isize));
}
unsafe extern "C" fn yytnamerr(
    mut yyres: *mut libc::c_char,
    mut yystr: *const libc::c_char,
) -> libc::c_long {
    if *yystr as libc::c_int == '"' as i32 {
        let mut yyn: libc::c_long = 0 as libc::c_int as libc::c_long;
        let mut yyp: *const libc::c_char = yystr;
        loop {
            yyp = yyp.offset(1);
            match *yyp as libc::c_int {
                39 | 44 => {
                    break;
                }
                92 => {
                    yyp = yyp.offset(1);
                    if *yyp as libc::c_int != '\\' as i32 {
                        break;
                    }
                }
                34 => {
                    if !yyres.is_null() {
                        *yyres.offset(yyn as isize) = '\0' as i32 as libc::c_char;
                    }
                    return yyn;
                }
                _ => {}
            }
            if !yyres.is_null() {
                *yyres.offset(yyn as isize) = *yyp;
            }
            yyn += 1;
            yyn;
        }
    }
    if !yyres.is_null() {
        return (yystpcpy(yyres, yystr)).offset_from(yyres) as libc::c_long
    } else {
        return strlen(yystr) as libc::c_long
    };
}
unsafe extern "C" fn yysyntax_error(
    mut yymsg_alloc: *mut libc::c_long,
    mut yymsg: *mut *mut libc::c_char,
    mut yyssp: *mut yy_state_t,
    mut yytoken: libc::c_int,
) -> libc::c_int {
    let mut yyformat: *const libc::c_char = 0 as *const libc::c_char;
    let mut yyarg: [*const libc::c_char; 5] = [0 as *const libc::c_char; 5];
    let mut yycount: libc::c_int = 0 as libc::c_int;
    let mut yysize: libc::c_long = 0 as libc::c_int as libc::c_long;
    if yytoken != -(2 as libc::c_int) {
        let mut yyn: libc::c_int = yypact[*yyssp as libc::c_int as usize] as libc::c_int;
        let mut yysize0: libc::c_long = yytnamerr(
            0 as *mut libc::c_char,
            yytname[yytoken as usize],
        );
        yysize = yysize0;
        let fresh17 = yycount;
        yycount = yycount + 1;
        yyarg[fresh17 as usize] = yytname[yytoken as usize];
        if !(yyn == -(211 as libc::c_int)) {
            let mut yyxbegin: libc::c_int = if yyn < 0 as libc::c_int {
                -yyn
            } else {
                0 as libc::c_int
            };
            let mut yychecklim: libc::c_int = 793 as libc::c_int - yyn
                + 1 as libc::c_int;
            let mut yyxend: libc::c_int = if yychecklim < 61 as libc::c_int {
                yychecklim
            } else {
                61 as libc::c_int
            };
            let mut yyx: libc::c_int = 0;
            yyx = yyxbegin;
            while yyx < yyxend {
                if yycheck[(yyx + yyn) as usize] as libc::c_int == yyx
                    && yyx != 1 as libc::c_int
                    && !(yytable[(yyx + yyn) as usize] as libc::c_int
                        == -(122 as libc::c_int))
                {
                    if yycount == YYERROR_VERBOSE_ARGS_MAXIMUM as libc::c_int {
                        yycount = 1 as libc::c_int;
                        yysize = yysize0;
                        break;
                    } else {
                        let fresh18 = yycount;
                        yycount = yycount + 1;
                        yyarg[fresh18 as usize] = yytname[yyx as usize];
                        let mut yysize1: libc::c_long = yysize
                            + yytnamerr(0 as *mut libc::c_char, yytname[yyx as usize]);
                        if yysize <= yysize1
                            && yysize1
                                <= (if (9223372036854775807 as libc::c_long
                                    as libc::c_ulong) < -(1 as libc::c_int) as libc::c_ulong
                                {
                                    9223372036854775807 as libc::c_long as libc::c_ulong
                                } else {
                                    -(1 as libc::c_int) as libc::c_ulong
                                }) as libc::c_long
                        {
                            yysize = yysize1;
                        } else {
                            return 2 as libc::c_int
                        }
                    }
                }
                yyx += 1;
                yyx;
            }
        }
    }
    match yycount {
        1 => {
            yyformat = b"syntax error, unexpected %s\0" as *const u8
                as *const libc::c_char;
        }
        2 => {
            yyformat = b"syntax error, unexpected %s, expecting %s\0" as *const u8
                as *const libc::c_char;
        }
        3 => {
            yyformat = b"syntax error, unexpected %s, expecting %s or %s\0" as *const u8
                as *const libc::c_char;
        }
        4 => {
            yyformat = b"syntax error, unexpected %s, expecting %s or %s or %s\0"
                as *const u8 as *const libc::c_char;
        }
        5 => {
            yyformat = b"syntax error, unexpected %s, expecting %s or %s or %s or %s\0"
                as *const u8 as *const libc::c_char;
        }
        0 | _ => {
            yyformat = b"syntax error\0" as *const u8 as *const libc::c_char;
        }
    }
    let mut yysize1_0: libc::c_long = yysize
        + (strlen(yyformat) as libc::c_long
            - (2 as libc::c_int * yycount) as libc::c_long)
        + 1 as libc::c_int as libc::c_long;
    if yysize <= yysize1_0
        && yysize1_0
            <= (if (9223372036854775807 as libc::c_long as libc::c_ulong)
                < -(1 as libc::c_int) as libc::c_ulong
            {
                9223372036854775807 as libc::c_long as libc::c_ulong
            } else {
                -(1 as libc::c_int) as libc::c_ulong
            }) as libc::c_long
    {
        yysize = yysize1_0;
    } else {
        return 2 as libc::c_int
    }
    if *yymsg_alloc < yysize {
        *yymsg_alloc = 2 as libc::c_int as libc::c_long * yysize;
        if !(yysize <= *yymsg_alloc
            && *yymsg_alloc
                <= (if (9223372036854775807 as libc::c_long as libc::c_ulong)
                    < -(1 as libc::c_int) as libc::c_ulong
                {
                    9223372036854775807 as libc::c_long as libc::c_ulong
                } else {
                    -(1 as libc::c_int) as libc::c_ulong
                }) as libc::c_long)
        {
            *yymsg_alloc = (if (9223372036854775807 as libc::c_long as libc::c_ulong)
                < -(1 as libc::c_int) as libc::c_ulong
            {
                9223372036854775807 as libc::c_long as libc::c_ulong
            } else {
                -(1 as libc::c_int) as libc::c_ulong
            }) as libc::c_long;
        }
        return 1 as libc::c_int;
    }
    let mut yyp: *mut libc::c_char = *yymsg;
    let mut yyi: libc::c_int = 0 as libc::c_int;
    loop {
        *yyp = *yyformat;
        if !(*yyp as libc::c_int != '\0' as i32) {
            break;
        }
        if *yyp as libc::c_int == '%' as i32
            && *yyformat.offset(1 as libc::c_int as isize) as libc::c_int == 's' as i32
            && yyi < yycount
        {
            let fresh19 = yyi;
            yyi = yyi + 1;
            yyp = yyp.offset(yytnamerr(yyp, yyarg[fresh19 as usize]) as isize);
            yyformat = yyformat.offset(2 as libc::c_int as isize);
        } else {
            yyp = yyp.offset(1);
            yyp;
            yyformat = yyformat.offset(1);
            yyformat;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn yydestruct(
    mut yymsg: *const libc::c_char,
    mut yytype: libc::c_int,
    mut yyvaluep: *mut YYSTYPE,
    mut p: *mut parser_state,
) {
    if yymsg.is_null() {
        yymsg = b"Deleting\0" as *const u8 as *const libc::c_char;
    }
    if yydebug != 0 {
        fprintf(stderr, b"%s \0" as *const u8 as *const libc::c_char, yymsg);
        yy_symbol_print(stderr, yytype, yyvaluep, p);
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    }
}
pub unsafe extern "C" fn yyparse(mut p: *mut parser_state) -> libc::c_int {
    let mut current_block: u64;
    let mut yychar: libc::c_int = 0;
    static mut yyval_default: YYSTYPE = YYSTYPE {
        nd: 0 as *const node as *mut node,
    };
    let mut yylval: YYSTYPE = yyval_default;
    let mut yynerrs: libc::c_int = 0;
    let mut yystate: yy_state_fast_t = 0;
    let mut yyerrstatus: libc::c_int = 0;
    let mut yyssa: [yy_state_t; 200] = [0; 200];
    let mut yyss: *mut yy_state_t = 0 as *mut yy_state_t;
    let mut yyssp: *mut yy_state_t = 0 as *mut yy_state_t;
    let mut yyvsa: [YYSTYPE; 200] = [YYSTYPE {
        nd: 0 as *const node as *mut node,
    }; 200];
    let mut yyvs: *mut YYSTYPE = 0 as *mut YYSTYPE;
    let mut yyvsp: *mut YYSTYPE = 0 as *mut YYSTYPE;
    let mut yystacksize: libc::c_long = 0;
    let mut yyn: libc::c_int = 0;
    let mut yyresult: libc::c_int = 0;
    let mut yytoken: libc::c_int = 0 as libc::c_int;
    let mut yyval: YYSTYPE = YYSTYPE {
        nd: 0 as *const node as *mut node,
    };
    let mut yymsgbuf: [libc::c_char; 128] = [0; 128];
    let mut yymsg: *mut libc::c_char = yymsgbuf.as_mut_ptr();
    let mut yymsg_alloc: libc::c_long = ::std::mem::size_of::<[libc::c_char; 128]>()
        as libc::c_ulong as libc::c_long;
    let mut yylen: libc::c_int = 0 as libc::c_int;
    yyss = yyssa.as_mut_ptr();
    yyssp = yyss;
    yyvs = yyvsa.as_mut_ptr();
    yyvsp = yyvs;
    yystacksize = 200 as libc::c_int as libc::c_long;
    if yydebug != 0 {
        fprintf(stderr, b"Starting parse\n\0" as *const u8 as *const libc::c_char);
    }
    yystate = 0 as libc::c_int;
    yyerrstatus = 0 as libc::c_int;
    yynerrs = 0 as libc::c_int;
    yychar = -(2 as libc::c_int);
    '_yysetstate: loop {
        if yydebug != 0 {
            fprintf(
                stderr,
                b"Entering state %d\n\0" as *const u8 as *const libc::c_char,
                yystate,
            );
        }
        (0 as libc::c_int != 0
            && (0 as libc::c_int <= yystate && yystate < 254 as libc::c_int))
            as libc::c_int;
        *yyssp = yystate as yy_state_t;
        if yyss.offset(yystacksize as isize).offset(-(1 as libc::c_int as isize))
            <= yyssp
        {
            let mut yysize: libc::c_long = yyssp.offset_from(yyss) as libc::c_long
                + 1 as libc::c_int as libc::c_long;
            if 10000 as libc::c_int as libc::c_long <= yystacksize {
                current_block = 1117375844326506140;
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
                        + ::std::mem::size_of::<YYSTYPE>() as libc::c_ulong
                            as libc::c_long)
                    + (::std::mem::size_of::<yyalloc>() as libc::c_ulong as libc::c_long
                        - 1 as libc::c_int as libc::c_long)) as libc::c_ulong,
            ) as *mut yyalloc;
            if yyptr.is_null() {
                current_block = 1117375844326506140;
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
                &mut (*yyptr).yyvs_alloc as *mut YYSTYPE as *mut libc::c_void,
                yyvs as *const libc::c_void,
                (yysize as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<YYSTYPE>() as libc::c_ulong)
                    as libc::size_t,
            );
            yyvs = &mut (*yyptr).yyvs_alloc;
            yynewbytes_0 = yystacksize
                * ::std::mem::size_of::<YYSTYPE>() as libc::c_ulong as libc::c_long
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
            if yydebug != 0 {
                fprintf(
                    stderr,
                    b"Stack size increased to %ld\n\0" as *const u8
                        as *const libc::c_char,
                    yystacksize,
                );
            }
            if yyss.offset(yystacksize as isize).offset(-(1 as libc::c_int as isize))
                <= yyssp
            {
                current_block = 8988172551322753953;
                break;
            }
        }
        if yystate == 82 as libc::c_int {
            yyresult = 0 as libc::c_int;
            current_block = 4231856563731314708;
            break;
        } else {
            yyn = yypact[yystate as usize] as libc::c_int;
            if yyn == -(211 as libc::c_int) {
                current_block = 14117185088927096118;
            } else {
                if yychar == -(2 as libc::c_int) {
                    if yydebug != 0 {
                        fprintf(
                            stderr,
                            b"Reading a token: \0" as *const u8 as *const libc::c_char,
                        );
                    }
                    yychar = yylex(&mut yylval, p);
                }
                if yychar <= 0 as libc::c_int {
                    yytoken = 0 as libc::c_int;
                    yychar = yytoken;
                    if yydebug != 0 {
                        fprintf(
                            stderr,
                            b"Now at end of input.\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                } else {
                    yytoken = if 0 as libc::c_int <= yychar
                        && yychar <= 301 as libc::c_int
                    {
                        yytranslate[yychar as usize] as libc::c_int
                    } else {
                        2 as libc::c_int
                    };
                    if yydebug != 0 {
                        fprintf(
                            stderr,
                            b"%s \0" as *const u8 as *const libc::c_char,
                            b"Next token is\0" as *const u8 as *const libc::c_char,
                        );
                        yy_symbol_print(stderr, yytoken, &mut yylval, p);
                        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                    }
                }
                yyn += yytoken;
                if yyn < 0 as libc::c_int || (793 as libc::c_int) < yyn
                    || yycheck[yyn as usize] as libc::c_int != yytoken
                {
                    current_block = 14117185088927096118;
                } else {
                    yyn = yytable[yyn as usize] as libc::c_int;
                    if yyn <= 0 as libc::c_int {
                        if yyn == -(122 as libc::c_int) {
                            current_block = 8024805296599672412;
                        } else {
                            yyn = -yyn;
                            current_block = 11092376670319816733;
                        }
                    } else {
                        if yyerrstatus != 0 {
                            yyerrstatus -= 1;
                            yyerrstatus;
                        }
                        if yydebug != 0 {
                            fprintf(
                                stderr,
                                b"%s \0" as *const u8 as *const libc::c_char,
                                b"Shifting\0" as *const u8 as *const libc::c_char,
                            );
                            yy_symbol_print(stderr, yytoken, &mut yylval, p);
                            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                        }
                        yystate = yyn;
                        yyvsp = yyvsp.offset(1);
                        *yyvsp = yylval;
                        yychar = -(2 as libc::c_int);
                        current_block = 2894812946742415500;
                    }
                }
            }
            match current_block {
                14117185088927096118 => {
                    yyn = yydefact[yystate as usize] as libc::c_int;
                    if yyn == 0 as libc::c_int {
                        current_block = 8024805296599672412;
                    } else {
                        current_block = 11092376670319816733;
                    }
                }
                _ => {}
            }
            match current_block {
                8024805296599672412 => {
                    yytoken = if yychar == -(2 as libc::c_int) {
                        -(2 as libc::c_int)
                    } else if 0 as libc::c_int <= yychar && yychar <= 301 as libc::c_int
                    {
                        yytranslate[yychar as usize] as libc::c_int
                    } else {
                        2 as libc::c_int
                    };
                    if yyerrstatus == 0 {
                        yynerrs += 1;
                        yynerrs;
                        let mut yymsgp: *const libc::c_char = b"syntax error\0"
                            as *const u8 as *const libc::c_char;
                        let mut yysyntax_error_status: libc::c_int = 0;
                        yysyntax_error_status = yysyntax_error(
                            &mut yymsg_alloc,
                            &mut yymsg,
                            yyssp,
                            yytoken,
                        );
                        if yysyntax_error_status == 0 as libc::c_int {
                            yymsgp = yymsg;
                        } else if yysyntax_error_status == 1 as libc::c_int {
                            if yymsg != yymsgbuf.as_mut_ptr() {
                                free(yymsg as *mut libc::c_void);
                            }
                            yymsg = malloc(yymsg_alloc as libc::c_ulong)
                                as *mut libc::c_char;
                            if yymsg.is_null() {
                                yymsg = yymsgbuf.as_mut_ptr();
                                yymsg_alloc = ::std::mem::size_of::<[libc::c_char; 128]>()
                                    as libc::c_ulong as libc::c_long;
                                yysyntax_error_status = 2 as libc::c_int;
                            } else {
                                yysyntax_error_status = yysyntax_error(
                                    &mut yymsg_alloc,
                                    &mut yymsg,
                                    yyssp,
                                    yytoken,
                                );
                                yymsgp = yymsg;
                            }
                        }
                        yyerror(p, yymsgp);
                        if yysyntax_error_status == 2 as libc::c_int {
                            current_block = 1117375844326506140;
                            break;
                        }
                    }
                    if yyerrstatus == 3 as libc::c_int {
                        if yychar <= 0 as libc::c_int {
                            if yychar == 0 as libc::c_int {
                                current_block = 8988172551322753953;
                                break;
                            }
                        } else {
                            yydestruct(
                                b"Error: discarding\0" as *const u8 as *const libc::c_char,
                                yytoken,
                                &mut yylval,
                                p,
                            );
                            yychar = -(2 as libc::c_int);
                        }
                    }
                    yyerrstatus = 3 as libc::c_int;
                    loop {
                        yyn = yypact[yystate as usize] as libc::c_int;
                        if !(yyn == -(211 as libc::c_int)) {
                            yyn += 1 as libc::c_int;
                            if 0 as libc::c_int <= yyn && yyn <= 793 as libc::c_int
                                && yycheck[yyn as usize] as libc::c_int == 1 as libc::c_int
                            {
                                yyn = yytable[yyn as usize] as libc::c_int;
                                if (0 as libc::c_int) < yyn {
                                    break;
                                }
                            }
                        }
                        if yyssp == yyss {
                            current_block = 8988172551322753953;
                            break '_yysetstate;
                        }
                        yydestruct(
                            b"Error: popping\0" as *const u8 as *const libc::c_char,
                            yystos[yystate as usize] as libc::c_int,
                            yyvsp,
                            p,
                        );
                        yyvsp = yyvsp.offset(-(1 as libc::c_int as isize));
                        yyssp = yyssp.offset(-(1 as libc::c_int as isize));
                        yystate = *yyssp as yy_state_fast_t;
                        if yydebug != 0 {
                            yy_stack_print(yyss, yyssp);
                        }
                    }
                    yyvsp = yyvsp.offset(1);
                    *yyvsp = yylval;
                    if yydebug != 0 {
                        fprintf(
                            stderr,
                            b"%s \0" as *const u8 as *const libc::c_char,
                            b"Shifting\0" as *const u8 as *const libc::c_char,
                        );
                        yy_symbol_print(
                            stderr,
                            yystos[yyn as usize] as libc::c_int,
                            yyvsp,
                            p,
                        );
                        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                    }
                    yystate = yyn;
                }
                11092376670319816733 => {
                    yylen = yyr2[yyn as usize] as libc::c_int;
                    yyval = *yyvsp.offset((1 as libc::c_int - yylen) as isize);
                    if yydebug != 0 {
                        yy_reduce_print(yyssp, yyvsp, yyn, p);
                    }
                    match yyn {
                        2 => {
                            (*p)
                                .lval = (*yyvsp.offset(0 as libc::c_int as isize)).nd
                                as *mut libc::c_void;
                        }
                        3 => {
                            yyval.nd = (*yyvsp.offset(-(1 as libc::c_int) as isize)).nd;
                        }
                        4 => {
                            yyval.nd = (*yyvsp.offset(-(1 as libc::c_int) as isize)).nd;
                        }
                        5 => {
                            yyval.nd = 0 as *mut node;
                        }
                        6 => {
                            yyval.nd = node_nodes_new();
                            node_lineinfo(p, yyval.nd);
                            if !((*yyvsp.offset(0 as libc::c_int as isize)).nd).is_null()
                            {
                                node_nodes_add(
                                    yyval.nd,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                                );
                                node_lineinfo(
                                    p,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                                );
                            }
                        }
                        7 => {
                            yyval.nd = (*yyvsp.offset(-(2 as libc::c_int) as isize)).nd;
                            if !((*yyvsp.offset(0 as libc::c_int as isize)).nd).is_null()
                            {
                                if !((*yyvsp.offset(-(2 as libc::c_int) as isize)).nd)
                                    .is_null()
                                {
                                    node_nodes_add(
                                        yyval.nd,
                                        (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                                    );
                                } else {
                                    let ref mut fresh20 = (*yyvsp
                                        .offset(-(2 as libc::c_int) as isize))
                                        .nd;
                                    *fresh20 = (*yyvsp.offset(0 as libc::c_int as isize)).nd;
                                }
                                node_lineinfo(
                                    p,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                                );
                            }
                        }
                        8 => {
                            yyval
                                .nd = node_ns_new(
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).id,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).nd,
                            );
                        }
                        9 => {
                            yyval
                                .nd = node_ns_new(
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).id,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).nd,
                            );
                        }
                        10 => {
                            yyval
                                .nd = node_import_new(
                                (*yyvsp.offset(0 as libc::c_int as isize)).id,
                            );
                        }
                        11 => {
                            yyval
                                .nd = node_let_new(
                                (*yyvsp.offset(-(6 as libc::c_int) as isize)).id,
                                node_method_new(
                                    (*yyvsp.offset(-(4 as libc::c_int) as isize)).nd,
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).nd,
                                ),
                            );
                        }
                        12 => {
                            yyval
                                .nd = node_let_new(
                                (*yyvsp.offset(-(5 as libc::c_int) as isize)).id,
                                node_method_new(
                                    (*yyvsp.offset(-(3 as libc::c_int) as isize)).nd,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                                ),
                            );
                        }
                        14 => {
                            yyval.nd = (*yyvsp.offset(-(1 as libc::c_int) as isize)).nd;
                        }
                        15 => {
                            yyval.nd = (*yyvsp.offset(-(1 as libc::c_int) as isize)).nd;
                        }
                        16 => {
                            yyval.nd = 0 as *mut node;
                        }
                        17 => {
                            yyval.nd = node_nodes_new();
                            node_lineinfo(p, yyval.nd);
                            if !((*yyvsp.offset(0 as libc::c_int as isize)).nd).is_null()
                            {
                                node_nodes_add(
                                    yyval.nd,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                                );
                                node_lineinfo(
                                    p,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                                );
                            }
                        }
                        18 => {
                            yyval.nd = (*yyvsp.offset(-(2 as libc::c_int) as isize)).nd;
                            if !((*yyvsp.offset(0 as libc::c_int as isize)).nd).is_null()
                            {
                                if !((*yyvsp.offset(-(2 as libc::c_int) as isize)).nd)
                                    .is_null()
                                {
                                    node_nodes_add(
                                        yyval.nd,
                                        (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                                    );
                                } else {
                                    let ref mut fresh21 = (*yyvsp
                                        .offset(-(2 as libc::c_int) as isize))
                                        .nd;
                                    *fresh21 = (*yyvsp.offset(0 as libc::c_int as isize)).nd;
                                }
                                node_lineinfo(
                                    p,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                                );
                            }
                        }
                        19 => {
                            yyval
                                .nd = node_let_new(
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).id,
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        20 => {
                            yyval
                                .nd = node_let_new(
                                (*yyvsp.offset(-(6 as libc::c_int) as isize)).id,
                                node_lambda_new(
                                    (*yyvsp.offset(-(4 as libc::c_int) as isize)).nd,
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).nd,
                                ),
                            );
                        }
                        21 => {
                            yyval
                                .nd = node_let_new(
                                (*yyvsp.offset(-(5 as libc::c_int) as isize)).id,
                                node_lambda_new(
                                    (*yyvsp.offset(-(3 as libc::c_int) as isize)).nd,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                                ),
                            );
                        }
                        22 => {
                            yyval
                                .nd = node_let_new(
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).id,
                                node_lambda_new(
                                    0 as *mut node,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                                ),
                            );
                        }
                        23 => {
                            yyval
                                .nd = node_let_new(
                                (*yyvsp.offset(0 as libc::c_int as isize)).id,
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).nd,
                            );
                        }
                        24 => {
                            yyval.nd = node_skip_new();
                        }
                        25 => {
                            yyval
                                .nd = node_emit_new(
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        26 => {
                            yyval
                                .nd = node_return_new(
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        30 => {
                            yyval
                                .id = (*((*yyvsp.offset(0 as libc::c_int as isize)).nd
                                as *mut node_str))
                                .value;
                        }
                        31 => {
                            yyval
                                .nd = node_op_new(
                                b"+\0" as *const u8 as *const libc::c_char,
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).nd,
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        32 => {
                            yyval
                                .nd = node_op_new(
                                b"-\0" as *const u8 as *const libc::c_char,
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).nd,
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        33 => {
                            yyval
                                .nd = node_op_new(
                                b"*\0" as *const u8 as *const libc::c_char,
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).nd,
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        34 => {
                            yyval
                                .nd = node_op_new(
                                b"/\0" as *const u8 as *const libc::c_char,
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).nd,
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        35 => {
                            yyval
                                .nd = node_op_new(
                                b"%\0" as *const u8 as *const libc::c_char,
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).nd,
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        36 => {
                            yyval
                                .nd = node_op_new(
                                b"|\0" as *const u8 as *const libc::c_char,
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).nd,
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        37 => {
                            yyval
                                .nd = node_op_new(
                                b"&\0" as *const u8 as *const libc::c_char,
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).nd,
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        38 => {
                            yyval
                                .nd = node_op_new(
                                b">\0" as *const u8 as *const libc::c_char,
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).nd,
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        39 => {
                            yyval
                                .nd = node_op_new(
                                b">=\0" as *const u8 as *const libc::c_char,
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).nd,
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        40 => {
                            yyval
                                .nd = node_op_new(
                                b"<\0" as *const u8 as *const libc::c_char,
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).nd,
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        41 => {
                            yyval
                                .nd = node_op_new(
                                b"<=\0" as *const u8 as *const libc::c_char,
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).nd,
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        42 => {
                            yyval
                                .nd = node_op_new(
                                b"==\0" as *const u8 as *const libc::c_char,
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).nd,
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        43 => {
                            yyval
                                .nd = node_op_new(
                                b"!=\0" as *const u8 as *const libc::c_char,
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).nd,
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        44 => {
                            yyval.nd = (*yyvsp.offset(0 as libc::c_int as isize)).nd;
                        }
                        45 => {
                            yyval
                                .nd = node_op_new(
                                b"-\0" as *const u8 as *const libc::c_char,
                                0 as *mut node,
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        46 => {
                            yyval
                                .nd = node_op_new(
                                b"!\0" as *const u8 as *const libc::c_char,
                                0 as *mut node,
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        47 => {
                            yyval
                                .nd = node_op_new(
                                b"~\0" as *const u8 as *const libc::c_char,
                                0 as *mut node,
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        48 => {
                            yyval
                                .nd = node_op_new(
                                b"&&\0" as *const u8 as *const libc::c_char,
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).nd,
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        49 => {
                            yyval
                                .nd = node_op_new(
                                b"||\0" as *const u8 as *const libc::c_char,
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).nd,
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        50 => {
                            yyval
                                .nd = node_lambda_new(
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).nd,
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        51 => {
                            yyval
                                .nd = node_lambda_new(
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).nd,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).nd,
                            );
                        }
                        52 => {
                            yyval
                                .nd = node_if_new(
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).nd,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).nd,
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        54 => {
                            yyval.nd = (*yyvsp.offset(-(1 as libc::c_int) as isize)).nd;
                        }
                        55 => {
                            yyval.nd = 0 as *mut node;
                        }
                        56 => {
                            yyval.nd = (*yyvsp.offset(0 as libc::c_int as isize)).nd;
                        }
                        57 => {
                            yyval.nd = 0 as *mut node;
                        }
                        58 => {
                            yyval
                                .nd = node_array_headers(
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        60 => {
                            yyval
                                .nd = node_pair_new(
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).id,
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        61 => {
                            yyval
                                .nd = node_splat_new(
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        62 => {
                            yyval.nd = node_array_new();
                            node_array_add(
                                yyval.nd,
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        63 => {
                            yyval.nd = (*yyvsp.offset(-(2 as libc::c_int) as isize)).nd;
                            node_array_add(
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).nd,
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        68 => {
                            yyval
                                .nd = node_ident_new(
                                (*yyvsp.offset(0 as libc::c_int as isize)).id,
                            );
                        }
                        69 => {
                            if (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).nd).type_0
                                as libc::c_uint
                                == NODE_LAMBDA as libc::c_int as libc::c_uint
                            {
                                let mut lambda: *mut node_lambda = (*yyvsp
                                    .offset(-(1 as libc::c_int) as isize))
                                    .nd as *mut node_lambda;
                                if (*lambda).block != 0 {
                                    (*lambda).block = 0 as libc::c_int;
                                }
                            }
                            yyval.nd = (*yyvsp.offset(-(1 as libc::c_int) as isize)).nd;
                        }
                        70 => {
                            yyval
                                .nd = node_array_headers(
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).nd,
                            );
                        }
                        71 => {
                            yyval.nd = node_array_new();
                        }
                        73 => {
                            yyval.nd = node_nil();
                        }
                        74 => {
                            yyval.nd = node_true();
                        }
                        75 => {
                            yyval.nd = node_false();
                        }
                        76 => {
                            yyval
                                .nd = node_obj_new(
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).nd,
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).id,
                            );
                        }
                        77 => {
                            yyval
                                .nd = node_call_new(
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).id,
                                0 as *mut node,
                                0 as *mut node,
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        78 => {
                            yyval
                                .nd = node_call_new(
                                (*yyvsp.offset(-(4 as libc::c_int) as isize)).id,
                                0 as *mut node,
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).nd,
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        79 => {
                            yyval
                                .nd = node_call_new(
                                (*yyvsp.offset(-(4 as libc::c_int) as isize)).id,
                                (*yyvsp.offset(-(6 as libc::c_int) as isize)).nd,
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).nd,
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        80 => {
                            yyval
                                .nd = node_call_new(
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).id,
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).nd,
                                0 as *mut node,
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        81 => {
                            yyval
                                .nd = node_fcall_new(
                                (*yyvsp.offset(-(5 as libc::c_int) as isize)).nd,
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).nd,
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        82 => {
                            yyval
                                .nd = node_genfunc_new(
                                (*yyvsp.offset(0 as libc::c_int as isize)).id,
                            );
                        }
                        83 => {
                            yyval.nd = 0 as *mut node;
                        }
                        85 => {
                            yyval
                                .nd = node_ident_new(
                                (*yyvsp.offset(0 as libc::c_int as isize)).id,
                            );
                        }
                        88 => {
                            yyval.nd = node_nil();
                        }
                        89 => {
                            yyval.nd = node_true();
                        }
                        90 => {
                            yyval.nd = node_false();
                        }
                        91 => {
                            yyval.nd = node_pattern_new(NODE_PARRAY);
                        }
                        92 => {
                            yyval
                                .nd = node_ns_new(
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).id,
                                node_pattern_new(NODE_PARRAY),
                            );
                        }
                        93 => {
                            yyval.nd = (*yyvsp.offset(-(1 as libc::c_int) as isize)).nd;
                        }
                        94 => {
                            yyval
                                .nd = node_ns_new(
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).id,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).nd,
                            );
                        }
                        95 => {
                            yyval
                                .nd = node_ns_new(
                                (*yyvsp.offset(0 as libc::c_int as isize)).id,
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).nd,
                            );
                        }
                        96 => {
                            yyval.nd = node_pattern_new(NODE_PARRAY);
                            node_pattern_add(
                                yyval.nd,
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        97 => {
                            yyval.nd = (*yyvsp.offset(-(2 as libc::c_int) as isize)).nd;
                            node_pattern_add(
                                yyval.nd,
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        98 => {
                            yyval.nd = node_pattern_new(NODE_PSTRUCT);
                            node_pattern_add(
                                yyval.nd,
                                node_pair_new(
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).id,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                                ),
                            );
                        }
                        99 => {
                            yyval.nd = (*yyvsp.offset(-(3 as libc::c_int) as isize)).nd;
                            node_pattern_add(
                                yyval.nd,
                                node_pair_new(
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).id,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                                ),
                            );
                        }
                        101 => {
                            yyval
                                .nd = node_psplat_new(
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).nd,
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                                0 as *mut node,
                            );
                        }
                        102 => {
                            yyval
                                .nd = node_psplat_new(
                                (*yyvsp.offset(-(5 as libc::c_int) as isize)).nd,
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).nd,
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        103 => {
                            yyval
                                .nd = node_psplat_new(
                                0 as *mut node,
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                                0 as *mut node,
                            );
                        }
                        104 => {
                            yyval
                                .nd = node_psplat_new(
                                0 as *mut node,
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).nd,
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        106 => {
                            yyval
                                .nd = node_psplat_new(
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).nd,
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                                0 as *mut node,
                            );
                        }
                        107 => {
                            yyval
                                .nd = node_plambda_new(
                                node_pattern_new(NODE_PARRAY),
                                0 as *mut node,
                            );
                        }
                        108 => {
                            yyval
                                .nd = node_plambda_new(
                                node_pattern_new(NODE_PARRAY),
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).nd,
                            );
                        }
                        109 => {
                            yyval
                                .nd = node_plambda_new(
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).nd,
                                0 as *mut node,
                            );
                        }
                        110 => {
                            yyval
                                .nd = node_plambda_new(
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).nd,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).nd,
                            );
                        }
                        111 => {
                            yyval
                                .nd = node_plambda_body(
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).nd,
                                (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                            );
                        }
                        112 => {
                            yyval
                                .nd = node_plambda_add(
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).nd,
                                node_plambda_body(
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).nd,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).nd,
                                ),
                            );
                        }
                        113 => {
                            yyval
                                .nd = node_block_new(
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).nd,
                            );
                        }
                        114 => {
                            yyval
                                .nd = node_lambda_new(
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).nd,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).nd,
                            );
                        }
                        115 => {
                            yyval.nd = (*yyvsp.offset(-(1 as libc::c_int) as isize)).nd;
                        }
                        116 => {
                            let mut e: *mut node = node_plambda_new(
                                0 as *mut node,
                                0 as *mut node,
                            );
                            yyval
                                .nd = node_plambda_add(
                                (*yyvsp.offset(-(4 as libc::c_int) as isize)).nd,
                                node_plambda_body(
                                    e,
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).nd,
                                ),
                            );
                        }
                        117 => {
                            yyval.nd = 0 as *mut node;
                        }
                        118 => {
                            yyval.nd = (*yyvsp.offset(-(1 as libc::c_int) as isize)).nd;
                        }
                        119 => {
                            yyval.nd = 0 as *mut node;
                        }
                        121 => {
                            yyval.nd = node_args_new();
                            node_args_add(
                                yyval.nd,
                                (*yyvsp.offset(0 as libc::c_int as isize)).id,
                            );
                        }
                        122 => {
                            yyval.nd = (*yyvsp.offset(-(2 as libc::c_int) as isize)).nd;
                            node_args_add(
                                yyval.nd,
                                (*yyvsp.offset(0 as libc::c_int as isize)).id,
                            );
                        }
                        126 => {
                            yyerrstatus = 0 as libc::c_int;
                        }
                        127 => {
                            yyerrstatus = 0 as libc::c_int;
                        }
                        _ => {}
                    }
                    if yydebug != 0 {
                        fprintf(
                            stderr,
                            b"%s \0" as *const u8 as *const libc::c_char,
                            b"-> $$ =\0" as *const u8 as *const libc::c_char,
                        );
                        yy_symbol_print(
                            stderr,
                            yyr1[yyn as usize] as libc::c_int,
                            &mut yyval,
                            p,
                        );
                        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                    }
                    yyvsp = yyvsp.offset(-(yylen as isize));
                    yyssp = yyssp.offset(-(yylen as isize));
                    yylen = 0 as libc::c_int;
                    if yydebug != 0 {
                        yy_stack_print(yyss, yyssp);
                    }
                    yyvsp = yyvsp.offset(1);
                    *yyvsp = yyval;
                    let yylhs: libc::c_int = yyr1[yyn as usize] as libc::c_int
                        - 61 as libc::c_int;
                    let yyi: libc::c_int = yypgoto[yylhs as usize] as libc::c_int
                        + *yyssp as libc::c_int;
                    yystate = if 0 as libc::c_int <= yyi && yyi <= 793 as libc::c_int
                        && yycheck[yyi as usize] as libc::c_int == *yyssp as libc::c_int
                    {
                        yytable[yyi as usize] as libc::c_int
                    } else {
                        yydefgoto[yylhs as usize] as libc::c_int
                    };
                }
                _ => {}
            }
            yyssp = yyssp.offset(1);
            yyssp;
        }
    }
    match current_block {
        1117375844326506140 => {
            yyerror(p, b"memory exhausted\0" as *const u8 as *const libc::c_char);
            yyresult = 2 as libc::c_int;
        }
        8988172551322753953 => {
            yyresult = 1 as libc::c_int;
        }
        _ => {}
    }
    if yychar != -(2 as libc::c_int) {
        yytoken = if 0 as libc::c_int <= yychar && yychar <= 301 as libc::c_int {
            yytranslate[yychar as usize] as libc::c_int
        } else {
            2 as libc::c_int
        };
        yydestruct(
            b"Cleanup: discarding lookahead\0" as *const u8 as *const libc::c_char,
            yytoken,
            &mut yylval,
            p,
        );
    }
    yyvsp = yyvsp.offset(-(yylen as isize));
    yyssp = yyssp.offset(-(yylen as isize));
    if yydebug != 0 {
        yy_stack_print(yyss, yyssp);
    }
    while yyssp != yyss {
        yydestruct(
            b"Cleanup: popping\0" as *const u8 as *const libc::c_char,
            yystos[*yyssp as libc::c_int as usize] as libc::c_int,
            yyvsp,
            p,
        );
        yyvsp = yyvsp.offset(-(1 as libc::c_int as isize));
        yyssp = yyssp.offset(-(1 as libc::c_int as isize));
    }
    if yyss != yyssa.as_mut_ptr() {
        free(yyss as *mut libc::c_void);
    }
    if yymsg != yymsgbuf.as_mut_ptr() {
        free(yymsg as *mut libc::c_void);
    }
    return yyresult;
}
