use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn make_int_node(literal: libc::c_int) -> *mut node_t;
    fn make_string_node(literal: *mut libc::c_char) -> *mut node_t;
    fn make_field_node(fieldname: *mut libc::c_char) -> *mut node_t;
    fn make_op_node(op: operation) -> *mut node_t;
    fn yylex() -> libc::c_int;
    static mut zfilter: *mut node_t;
}
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
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
pub type uint64_t = __uint64_t;
pub type operation = libc::c_uint;
pub const GT_EQ: operation = 7;
pub const LT_EQ: operation = 6;
pub const OR: operation = 5;
pub const AND: operation = 4;
pub const NEQ: operation = 3;
pub const EQ: operation = 2;
pub const LT: operation = 1;
pub const GT: operation = 0;
pub type node_type = libc::c_uint;
pub const INT: node_type = 3;
pub const STRING: node_type = 2;
pub const FIELD: node_type = 1;
pub const OP: node_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct field_id {
    pub index: libc::c_int,
    pub fieldname: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union node_value {
    pub field: field_id,
    pub string_literal: *mut libc::c_char,
    pub int_literal: uint64_t,
    pub op: operation,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_st {
    pub left_child: *mut node_st,
    pub right_child: *mut node_st,
    pub type_0: node_type,
    pub value: node_value,
}
pub type node_t = node_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub union YYSTYPE {
    pub int_literal: libc::c_int,
    pub string_literal: *mut libc::c_char,
    pub expr: *mut node_st,
}
pub type YYINT = libc::c_short;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct YYSTACKDATA {
    pub stacksize: libc::c_uint,
    pub s_base: *mut YYINT,
    pub s_mark: *mut YYINT,
    pub s_last: *mut YYINT,
    pub l_base: *mut YYSTYPE,
    pub l_mark: *mut YYSTYPE,
}
pub unsafe extern "C" fn yyerror(mut str: *const libc::c_char) {
    fprintf(stderr, b"Parse error: %s\n\0" as *const u8 as *const libc::c_char, str);
}
pub unsafe extern "C" fn yywrap() -> libc::c_int {
    return 1 as libc::c_int;
}
static mut yylhs: [YYINT; 16] = [
    -(1 as libc::c_int) as YYINT,
    0 as libc::c_int as YYINT,
    4 as libc::c_int as YYINT,
    4 as libc::c_int as YYINT,
    4 as libc::c_int as YYINT,
    4 as libc::c_int as YYINT,
    1 as libc::c_int as YYINT,
    1 as libc::c_int as YYINT,
    2 as libc::c_int as YYINT,
    2 as libc::c_int as YYINT,
    2 as libc::c_int as YYINT,
    2 as libc::c_int as YYINT,
    2 as libc::c_int as YYINT,
    2 as libc::c_int as YYINT,
    3 as libc::c_int as YYINT,
    3 as libc::c_int as YYINT,
];
static mut yylen: [YYINT; 16] = [
    2 as libc::c_int as YYINT,
    1 as libc::c_int as YYINT,
    3 as libc::c_int as YYINT,
    3 as libc::c_int as YYINT,
    3 as libc::c_int as YYINT,
    1 as libc::c_int as YYINT,
    1 as libc::c_int as YYINT,
    1 as libc::c_int as YYINT,
    3 as libc::c_int as YYINT,
    3 as libc::c_int as YYINT,
    3 as libc::c_int as YYINT,
    3 as libc::c_int as YYINT,
    3 as libc::c_int as YYINT,
    3 as libc::c_int as YYINT,
    3 as libc::c_int as YYINT,
    3 as libc::c_int as YYINT,
];
static mut yydefred: [YYINT; 28] = [
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    5 as libc::c_int as YYINT,
    6 as libc::c_int as YYINT,
    7 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    4 as libc::c_int as YYINT,
    11 as libc::c_int as YYINT,
    15 as libc::c_int as YYINT,
    12 as libc::c_int as YYINT,
    9 as libc::c_int as YYINT,
    10 as libc::c_int as YYINT,
    8 as libc::c_int as YYINT,
    14 as libc::c_int as YYINT,
    13 as libc::c_int as YYINT,
    3 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
];
static mut yydgoto: [YYINT; 5] = [
    3 as libc::c_int as YYINT,
    4 as libc::c_int as YYINT,
    5 as libc::c_int as YYINT,
    6 as libc::c_int as YYINT,
    7 as libc::c_int as YYINT,
];
static mut yysindex: [YYINT; 28] = [
    -(40 as libc::c_int) as YYINT,
    -(40 as libc::c_int) as YYINT,
    -(57 as libc::c_int) as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    -(250 as libc::c_int) as YYINT,
    -(39 as libc::c_int) as YYINT,
    -(249 as libc::c_int) as YYINT,
    -(245 as libc::c_int) as YYINT,
    -(244 as libc::c_int) as YYINT,
    -(243 as libc::c_int) as YYINT,
    -(247 as libc::c_int) as YYINT,
    -(242 as libc::c_int) as YYINT,
    -(40 as libc::c_int) as YYINT,
    -(40 as libc::c_int) as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    -(248 as libc::c_int) as YYINT,
];
static mut yyrindex: [YYINT; 28] = [
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    18 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    1 as libc::c_int as YYINT,
];
static mut yygindex: [YYINT; 5] = [
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    5 as libc::c_int as YYINT,
];
static mut yytable: [YYINT; 260] = [
    1 as libc::c_int as YYINT,
    2 as libc::c_int as YYINT,
    17 as libc::c_int as YYINT,
    12 as libc::c_int as YYINT,
    13 as libc::c_int as YYINT,
    11 as libc::c_int as YYINT,
    8 as libc::c_int as YYINT,
    15 as libc::c_int as YYINT,
    16 as libc::c_int as YYINT,
    15 as libc::c_int as YYINT,
    18 as libc::c_int as YYINT,
    19 as libc::c_int as YYINT,
    23 as libc::c_int as YYINT,
    24 as libc::c_int as YYINT,
    20 as libc::c_int as YYINT,
    21 as libc::c_int as YYINT,
    22 as libc::c_int as YYINT,
    25 as libc::c_int as YYINT,
    1 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    26 as libc::c_int as YYINT,
    27 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    2 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    9 as libc::c_int as YYINT,
    10 as libc::c_int as YYINT,
    14 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    15 as libc::c_int as YYINT,
    16 as libc::c_int as YYINT,
    2 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    2 as libc::c_int as YYINT,
];
static mut yycheck: [YYINT; 260] = [
    40 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    41 as libc::c_int as YYINT,
    60 as libc::c_int as YYINT,
    61 as libc::c_int as YYINT,
    62 as libc::c_int as YYINT,
    1 as libc::c_int as YYINT,
    257 as libc::c_int as YYINT,
    258 as libc::c_int as YYINT,
    257 as libc::c_int as YYINT,
    259 as libc::c_int as YYINT,
    260 as libc::c_int as YYINT,
    259 as libc::c_int as YYINT,
    260 as libc::c_int as YYINT,
    259 as libc::c_int as YYINT,
    259 as libc::c_int as YYINT,
    259 as libc::c_int as YYINT,
    259 as libc::c_int as YYINT,
    0 as libc::c_int as YYINT,
    -(1 as libc::c_int) as YYINT,
    15 as libc::c_int as YYINT,
    16 as libc::c_int as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    41 as libc::c_int as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    261 as libc::c_int as YYINT,
    262 as libc::c_int as YYINT,
    263 as libc::c_int as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    257 as libc::c_int as YYINT,
    258 as libc::c_int as YYINT,
    260 as libc::c_int as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    -(1 as libc::c_int) as YYINT,
    258 as libc::c_int as YYINT,
];
pub static mut yydebug: libc::c_int = 0;
pub static mut yynerrs: libc::c_int = 0;
pub static mut yyerrflag: libc::c_int = 0;
pub static mut yychar: libc::c_int = 0;
pub static mut yyval: YYSTYPE = YYSTYPE { int_literal: 0 };
pub static mut yylval: YYSTYPE = YYSTYPE { int_literal: 0 };
static mut yystack: YYSTACKDATA = YYSTACKDATA {
    stacksize: 0,
    s_base: 0 as *const YYINT as *mut YYINT,
    s_mark: 0 as *const YYINT as *mut YYINT,
    s_last: 0 as *const YYINT as *mut YYINT,
    l_base: 0 as *const YYSTYPE as *mut YYSTYPE,
    l_mark: 0 as *const YYSTYPE as *mut YYSTYPE,
};
unsafe extern "C" fn yygrowstack(mut data: *mut YYSTACKDATA) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut newsize: libc::c_uint = 0;
    let mut newss: *mut YYINT = 0 as *mut YYINT;
    let mut newvs: *mut YYSTYPE = 0 as *mut YYSTYPE;
    newsize = (*data).stacksize;
    if newsize == 0 as libc::c_int as libc::c_uint {
        newsize = 200 as libc::c_int as libc::c_uint;
    } else if newsize >= 10000 as libc::c_int as libc::c_uint {
        return -(2 as libc::c_int)
    } else {
        newsize = newsize.wrapping_mul(2 as libc::c_int as libc::c_uint);
        if newsize > 10000 as libc::c_int as libc::c_uint {
            newsize = 10000 as libc::c_int as libc::c_uint;
        }
    }
    i = ((*data).s_mark).offset_from((*data).s_base) as libc::c_long as libc::c_int;
    newss = realloc(
        (*data).s_base as *mut libc::c_void,
        (newsize as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<YYINT>() as libc::c_ulong),
    ) as *mut YYINT;
    if newss.is_null() {
        return -(2 as libc::c_int);
    }
    (*data).s_base = newss;
    (*data).s_mark = newss.offset(i as isize);
    newvs = realloc(
        (*data).l_base as *mut libc::c_void,
        (newsize as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<YYSTYPE>() as libc::c_ulong),
    ) as *mut YYSTYPE;
    if newvs.is_null() {
        return -(2 as libc::c_int);
    }
    (*data).l_base = newvs;
    (*data).l_mark = newvs.offset(i as isize);
    (*data).stacksize = newsize;
    (*data)
        .s_last = ((*data).s_base)
        .offset(newsize as isize)
        .offset(-(1 as libc::c_int as isize));
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn yyparse() -> libc::c_int {
    let mut current_block: u64;
    let mut yym: libc::c_int = 0;
    let mut yyn: libc::c_int = 0;
    let mut yystate: libc::c_int = 0;
    yynerrs = 0 as libc::c_int;
    yyerrflag = 0 as libc::c_int;
    yychar = -(1 as libc::c_int);
    yystate = 0 as libc::c_int;
    if (yystack.s_base).is_null() && yygrowstack(&mut yystack) == -(2 as libc::c_int) {
        current_block = 10553296530965766740;
    } else {
        yystack.s_mark = yystack.s_base;
        yystack.l_mark = yystack.l_base;
        yystate = 0 as libc::c_int;
        *yystack.s_mark = 0 as libc::c_int as YYINT;
        '_yyloop: loop {
            yyn = yydefred[yystate as usize] as libc::c_int;
            if !(yyn != 0 as libc::c_int) {
                if yychar < 0 as libc::c_int {
                    yychar = yylex();
                    if yychar < 0 as libc::c_int {
                        yychar = 0 as libc::c_int;
                    }
                }
                yyn = yysindex[yystate as usize] as libc::c_int;
                if yyn != 0
                    && {
                        yyn += yychar;
                        yyn >= 0 as libc::c_int
                    } && yyn <= 259 as libc::c_int
                    && yycheck[yyn as usize] as libc::c_int == yychar
                {
                    if yystack.s_mark >= yystack.s_last
                        && yygrowstack(&mut yystack) == -(2 as libc::c_int)
                    {
                        current_block = 10553296530965766740;
                        break;
                    }
                    yystate = yytable[yyn as usize] as libc::c_int;
                    yystack.s_mark = (yystack.s_mark).offset(1);
                    *yystack.s_mark = yytable[yyn as usize];
                    yystack.l_mark = (yystack.l_mark).offset(1);
                    *yystack.l_mark = yylval;
                    yychar = -(1 as libc::c_int);
                    if yyerrflag > 0 as libc::c_int {
                        yyerrflag -= 1;
                        yyerrflag;
                    }
                    continue;
                } else {
                    yyn = yyrindex[yystate as usize] as libc::c_int;
                    if yyn != 0
                        && {
                            yyn += yychar;
                            yyn >= 0 as libc::c_int
                        } && yyn <= 259 as libc::c_int
                        && yycheck[yyn as usize] as libc::c_int == yychar
                    {
                        yyn = yytable[yyn as usize] as libc::c_int;
                    } else {
                        if !(yyerrflag != 0) {
                            yyerror(
                                b"syntax error\0" as *const u8 as *const libc::c_char,
                            );
                            yynerrs += 1;
                            yynerrs;
                        }
                        if yyerrflag < 3 as libc::c_int {
                            yyerrflag = 3 as libc::c_int;
                            loop {
                                yyn = yysindex[*yystack.s_mark as usize] as libc::c_int;
                                if yyn != 0
                                    && {
                                        yyn += 256 as libc::c_int;
                                        yyn >= 0 as libc::c_int
                                    } && yyn <= 259 as libc::c_int
                                    && yycheck[yyn as usize] as libc::c_int
                                        == 256 as libc::c_int
                                {
                                    if yystack.s_mark >= yystack.s_last
                                        && yygrowstack(&mut yystack) == -(2 as libc::c_int)
                                    {
                                        current_block = 10553296530965766740;
                                        break '_yyloop;
                                    }
                                    yystate = yytable[yyn as usize] as libc::c_int;
                                    yystack.s_mark = (yystack.s_mark).offset(1);
                                    *yystack.s_mark = yytable[yyn as usize];
                                    yystack.l_mark = (yystack.l_mark).offset(1);
                                    *yystack.l_mark = yylval;
                                    continue '_yyloop;
                                } else {
                                    if yystack.s_mark <= yystack.s_base {
                                        current_block = 2793352396589381719;
                                        break '_yyloop;
                                    }
                                    yystack.s_mark = (yystack.s_mark).offset(-1);
                                    yystack.s_mark;
                                    yystack.l_mark = (yystack.l_mark).offset(-1);
                                    yystack.l_mark;
                                }
                            }
                        } else {
                            if yychar == 0 as libc::c_int {
                                current_block = 2793352396589381719;
                                break;
                            }
                            yychar = -(1 as libc::c_int);
                            continue;
                        }
                    }
                }
            }
            yym = yylen[yyn as usize] as libc::c_int;
            if yym != 0 {
                yyval = *(yystack.l_mark).offset((1 as libc::c_int - yym) as isize);
            } else {
                memset(
                    &mut yyval as *mut YYSTYPE as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<YYSTYPE>() as libc::c_ulong,
                );
            }
            match yyn {
                1 => {
                    zfilter = (*(yystack.l_mark).offset(0 as libc::c_int as isize)).expr;
                }
                2 => {
                    yyval.expr = make_op_node(OR);
                    (*yyval.expr)
                        .left_child = (*(yystack.l_mark)
                        .offset(-(2 as libc::c_int) as isize))
                        .expr;
                    (*yyval.expr)
                        .right_child = (*(yystack.l_mark)
                        .offset(0 as libc::c_int as isize))
                        .expr;
                }
                3 => {
                    yyval.expr = make_op_node(AND);
                    (*yyval.expr)
                        .left_child = (*(yystack.l_mark)
                        .offset(-(2 as libc::c_int) as isize))
                        .expr;
                    (*yyval.expr)
                        .right_child = (*(yystack.l_mark)
                        .offset(0 as libc::c_int as isize))
                        .expr;
                }
                4 => {
                    yyval
                        .expr = (*(yystack.l_mark).offset(-(1 as libc::c_int) as isize))
                        .expr;
                }
                5 => {
                    yyval
                        .expr = (*(yystack.l_mark).offset(0 as libc::c_int as isize))
                        .expr;
                }
                6 => {
                    yyval
                        .expr = (*(yystack.l_mark).offset(0 as libc::c_int as isize))
                        .expr;
                }
                7 => {
                    yyval
                        .expr = (*(yystack.l_mark).offset(0 as libc::c_int as isize))
                        .expr;
                }
                8 => {
                    yyval.expr = make_op_node(EQ);
                    (*yyval.expr)
                        .left_child = make_field_node(
                        (*(yystack.l_mark).offset(-(2 as libc::c_int) as isize))
                            .string_literal,
                    );
                    (*yyval.expr)
                        .right_child = make_int_node(
                        (*(yystack.l_mark).offset(0 as libc::c_int as isize)).int_literal,
                    );
                }
                9 => {
                    yyval.expr = make_op_node(GT);
                    (*yyval.expr)
                        .left_child = make_field_node(
                        (*(yystack.l_mark).offset(-(2 as libc::c_int) as isize))
                            .string_literal,
                    );
                    (*yyval.expr)
                        .right_child = make_int_node(
                        (*(yystack.l_mark).offset(0 as libc::c_int as isize)).int_literal,
                    );
                }
                10 => {
                    yyval.expr = make_op_node(LT);
                    (*yyval.expr)
                        .left_child = make_field_node(
                        (*(yystack.l_mark).offset(-(2 as libc::c_int) as isize))
                            .string_literal,
                    );
                    (*yyval.expr)
                        .right_child = make_int_node(
                        (*(yystack.l_mark).offset(0 as libc::c_int as isize)).int_literal,
                    );
                }
                11 => {
                    yyval.expr = make_op_node(NEQ);
                    (*yyval.expr)
                        .left_child = make_field_node(
                        (*(yystack.l_mark).offset(-(2 as libc::c_int) as isize))
                            .string_literal,
                    );
                    (*yyval.expr)
                        .right_child = make_int_node(
                        (*(yystack.l_mark).offset(0 as libc::c_int as isize)).int_literal,
                    );
                }
                12 => {
                    yyval.expr = make_op_node(GT_EQ);
                    (*yyval.expr)
                        .left_child = make_field_node(
                        (*(yystack.l_mark).offset(-(2 as libc::c_int) as isize))
                            .string_literal,
                    );
                    (*yyval.expr)
                        .right_child = make_int_node(
                        (*(yystack.l_mark).offset(0 as libc::c_int as isize)).int_literal,
                    );
                }
                13 => {
                    yyval.expr = make_op_node(LT_EQ);
                    (*yyval.expr)
                        .left_child = make_field_node(
                        (*(yystack.l_mark).offset(-(2 as libc::c_int) as isize))
                            .string_literal,
                    );
                    (*yyval.expr)
                        .right_child = make_int_node(
                        (*(yystack.l_mark).offset(0 as libc::c_int as isize)).int_literal,
                    );
                }
                14 => {
                    yyval.expr = make_op_node(EQ);
                    (*yyval.expr)
                        .left_child = make_field_node(
                        (*(yystack.l_mark).offset(-(2 as libc::c_int) as isize))
                            .string_literal,
                    );
                    (*yyval.expr)
                        .right_child = make_string_node(
                        (*(yystack.l_mark).offset(0 as libc::c_int as isize))
                            .string_literal,
                    );
                }
                15 => {
                    yyval.expr = make_op_node(NEQ);
                    (*yyval.expr)
                        .left_child = make_field_node(
                        (*(yystack.l_mark).offset(-(2 as libc::c_int) as isize))
                            .string_literal,
                    );
                    (*yyval.expr)
                        .right_child = make_string_node(
                        (*(yystack.l_mark).offset(0 as libc::c_int as isize))
                            .string_literal,
                    );
                }
                _ => {}
            }
            yystack.s_mark = (yystack.s_mark).offset(-(yym as isize));
            yystate = *yystack.s_mark as libc::c_int;
            yystack.l_mark = (yystack.l_mark).offset(-(yym as isize));
            yym = yylhs[yyn as usize] as libc::c_int;
            if yystate == 0 as libc::c_int && yym == 0 as libc::c_int {
                yystate = 3 as libc::c_int;
                yystack.s_mark = (yystack.s_mark).offset(1);
                *yystack.s_mark = 3 as libc::c_int as YYINT;
                yystack.l_mark = (yystack.l_mark).offset(1);
                *yystack.l_mark = yyval;
                if yychar < 0 as libc::c_int {
                    yychar = yylex();
                    if yychar < 0 as libc::c_int {
                        yychar = 0 as libc::c_int;
                    }
                }
                if !(yychar == 0 as libc::c_int) {
                    continue;
                }
                return 0 as libc::c_int;
            } else {
                yyn = yygindex[yym as usize] as libc::c_int;
                if yyn != 0
                    && {
                        yyn += yystate;
                        yyn >= 0 as libc::c_int
                    } && yyn <= 259 as libc::c_int
                    && yycheck[yyn as usize] as libc::c_int == yystate
                {
                    yystate = yytable[yyn as usize] as libc::c_int;
                } else {
                    yystate = yydgoto[yym as usize] as libc::c_int;
                }
                if yystack.s_mark >= yystack.s_last
                    && yygrowstack(&mut yystack) == -(2 as libc::c_int)
                {
                    current_block = 10553296530965766740;
                    break;
                }
                yystack.s_mark = (yystack.s_mark).offset(1);
                *yystack.s_mark = yystate as YYINT;
                yystack.l_mark = (yystack.l_mark).offset(1);
                *yystack.l_mark = yyval;
            }
        }
    }
    match current_block {
        10553296530965766740 => {
            yyerror(b"yacc stack overflow\0" as *const u8 as *const libc::c_char);
        }
        _ => {}
    }
    return 1 as libc::c_int;
}
