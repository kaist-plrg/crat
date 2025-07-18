use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    static mut line_no: libc::c_int;
    static mut std_only: libc::c_int;
    static mut yylval: YYSTYPE;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn ct_warn(mesg: *const libc::c_char, _: ...);
    fn yyerror(str: *const libc::c_char, _: ...);
    fn strcopyof(str: *const libc::c_char) -> *mut libc::c_char;
    fn open_new_file() -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __int16_t = libc::c_short;
pub type __int32_t = libc::c_int;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type flex_int16_t = int16_t;
pub type flex_int32_t = int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yy_buffer_state {
    pub yy_input_file: *mut FILE,
    pub yy_ch_buf: *mut libc::c_char,
    pub yy_buf_pos: *mut libc::c_char,
    pub yy_buf_size: yy_size_t,
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
pub type YY_BUFFER_STATE = *mut yy_buffer_state;
pub type YY_CHAR = libc::c_uchar;
pub type yy_state_type = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arg_list {
    pub av_name: libc::c_int,
    pub arg_is_var: libc::c_int,
    pub next: *mut arg_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union YYSTYPE {
    pub s_value: *mut libc::c_char,
    pub c_value: libc::c_char,
    pub i_value: libc::c_int,
    pub a_value: *mut arg_list,
}
static mut yy_buffer_stack_top: size_t = 0 as libc::c_int as size_t;
static mut yy_buffer_stack_max: size_t = 0 as libc::c_int as size_t;
static mut yy_buffer_stack: *mut YY_BUFFER_STATE = 0 as *const YY_BUFFER_STATE
    as *mut YY_BUFFER_STATE;
static mut yy_hold_char: libc::c_char = 0;
static mut yy_n_chars: libc::c_int = 0;
pub static mut yyleng: libc::c_int = 0;
static mut yy_c_buf_p: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut yy_init: libc::c_int = 0 as libc::c_int;
static mut yy_start: libc::c_int = 0 as libc::c_int;
static mut yy_did_buffer_switch_on_eof: libc::c_int = 0;
pub static mut yyin: *mut FILE = 0 as *const FILE as *mut FILE;
pub static mut yyout: *mut FILE = 0 as *const FILE as *mut FILE;
pub static mut yylineno: libc::c_int = 1 as libc::c_int;
pub unsafe extern "C" fn yywrap() -> libc::c_int {
    if open_new_file() == 0 {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
static mut yy_accept: [flex_int16_t; 316] = [
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    47 as libc::c_int as flex_int16_t,
    45 as libc::c_int as flex_int16_t,
    40 as libc::c_int as flex_int16_t,
    38 as libc::c_int as flex_int16_t,
    32 as libc::c_int as flex_int16_t,
    45 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    33 as libc::c_int as flex_int16_t,
    33 as libc::c_int as flex_int16_t,
    29 as libc::c_int as flex_int16_t,
    33 as libc::c_int as flex_int16_t,
    29 as libc::c_int as flex_int16_t,
    29 as libc::c_int as flex_int16_t,
    28 as libc::c_int as flex_int16_t,
    33 as libc::c_int as flex_int16_t,
    44 as libc::c_int as flex_int16_t,
    36 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    36 as libc::c_int as flex_int16_t,
    45 as libc::c_int as flex_int16_t,
    29 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    45 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    40 as libc::c_int as flex_int16_t,
    36 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    43 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    30 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    44 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    41 as libc::c_int as flex_int16_t,
    44 as libc::c_int as flex_int16_t,
    44 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    35 as libc::c_int as flex_int16_t,
    39 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    10 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    31 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    44 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    44 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    9 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    16 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    17 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    6 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    18 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    12 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    5 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    15 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    13 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    11 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    7 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    19 as libc::c_int as flex_int16_t,
    8 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    24 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
];
static mut yy_ec: [flex_int32_t; 256] = [
    0 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
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
    2 as libc::c_int,
    4 as libc::c_int,
    5 as libc::c_int,
    6 as libc::c_int,
    1 as libc::c_int,
    7 as libc::c_int,
    8 as libc::c_int,
    1 as libc::c_int,
    9 as libc::c_int,
    10 as libc::c_int,
    11 as libc::c_int,
    12 as libc::c_int,
    13 as libc::c_int,
    14 as libc::c_int,
    15 as libc::c_int,
    16 as libc::c_int,
    17 as libc::c_int,
    17 as libc::c_int,
    17 as libc::c_int,
    17 as libc::c_int,
    17 as libc::c_int,
    17 as libc::c_int,
    17 as libc::c_int,
    17 as libc::c_int,
    17 as libc::c_int,
    17 as libc::c_int,
    1 as libc::c_int,
    18 as libc::c_int,
    19 as libc::c_int,
    20 as libc::c_int,
    21 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    22 as libc::c_int,
    22 as libc::c_int,
    22 as libc::c_int,
    22 as libc::c_int,
    22 as libc::c_int,
    22 as libc::c_int,
    22 as libc::c_int,
    22 as libc::c_int,
    22 as libc::c_int,
    22 as libc::c_int,
    22 as libc::c_int,
    22 as libc::c_int,
    22 as libc::c_int,
    22 as libc::c_int,
    22 as libc::c_int,
    22 as libc::c_int,
    22 as libc::c_int,
    22 as libc::c_int,
    22 as libc::c_int,
    22 as libc::c_int,
    22 as libc::c_int,
    22 as libc::c_int,
    22 as libc::c_int,
    22 as libc::c_int,
    22 as libc::c_int,
    22 as libc::c_int,
    23 as libc::c_int,
    24 as libc::c_int,
    25 as libc::c_int,
    26 as libc::c_int,
    27 as libc::c_int,
    1 as libc::c_int,
    28 as libc::c_int,
    29 as libc::c_int,
    30 as libc::c_int,
    31 as libc::c_int,
    32 as libc::c_int,
    33 as libc::c_int,
    34 as libc::c_int,
    35 as libc::c_int,
    36 as libc::c_int,
    37 as libc::c_int,
    38 as libc::c_int,
    39 as libc::c_int,
    40 as libc::c_int,
    41 as libc::c_int,
    42 as libc::c_int,
    43 as libc::c_int,
    44 as libc::c_int,
    45 as libc::c_int,
    46 as libc::c_int,
    47 as libc::c_int,
    48 as libc::c_int,
    49 as libc::c_int,
    50 as libc::c_int,
    37 as libc::c_int,
    51 as libc::c_int,
    37 as libc::c_int,
    52 as libc::c_int,
    53 as libc::c_int,
    54 as libc::c_int,
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
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
];
static mut yy_meta: [flex_int32_t; 55] = [
    0 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
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
    3 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
];
static mut yy_base: [flex_int16_t; 320] = [
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    553 as libc::c_int as flex_int16_t,
    554 as libc::c_int as flex_int16_t,
    550 as libc::c_int as flex_int16_t,
    554 as libc::c_int as flex_int16_t,
    531 as libc::c_int as flex_int16_t,
    545 as libc::c_int as flex_int16_t,
    554 as libc::c_int as flex_int16_t,
    529 as libc::c_int as flex_int16_t,
    540 as libc::c_int as flex_int16_t,
    554 as libc::c_int as flex_int16_t,
    527 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    96 as libc::c_int as flex_int16_t,
    96 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    107 as libc::c_int as flex_int16_t,
    526 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    525 as libc::c_int as flex_int16_t,
    541 as libc::c_int as flex_int16_t,
    523 as libc::c_int as flex_int16_t,
    494 as libc::c_int as flex_int16_t,
    496 as libc::c_int as flex_int16_t,
    498 as libc::c_int as flex_int16_t,
    507 as libc::c_int as flex_int16_t,
    499 as libc::c_int as flex_int16_t,
    495 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    83 as libc::c_int as flex_int16_t,
    104 as libc::c_int as flex_int16_t,
    107 as libc::c_int as flex_int16_t,
    507 as libc::c_int as flex_int16_t,
    490 as libc::c_int as flex_int16_t,
    486 as libc::c_int as flex_int16_t,
    106 as libc::c_int as flex_int16_t,
    96 as libc::c_int as flex_int16_t,
    491 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    479 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    529 as libc::c_int as flex_int16_t,
    554 as libc::c_int as flex_int16_t,
    510 as libc::c_int as flex_int16_t,
    141 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    509 as libc::c_int as flex_int16_t,
    520 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    507 as libc::c_int as flex_int16_t,
    133 as libc::c_int as flex_int16_t,
    135 as libc::c_int as flex_int16_t,
    130 as libc::c_int as flex_int16_t,
    139 as libc::c_int as flex_int16_t,
    141 as libc::c_int as flex_int16_t,
    506 as libc::c_int as flex_int16_t,
    150 as libc::c_int as flex_int16_t,
    505 as libc::c_int as flex_int16_t,
    521 as libc::c_int as flex_int16_t,
    503 as libc::c_int as flex_int16_t,
    160 as libc::c_int as flex_int16_t,
    123 as libc::c_int as flex_int16_t,
    125 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    112 as libc::c_int as flex_int16_t,
    130 as libc::c_int as flex_int16_t,
    474 as libc::c_int as flex_int16_t,
    184 as libc::c_int as flex_int16_t,
    185 as libc::c_int as flex_int16_t,
    187 as libc::c_int as flex_int16_t,
    188 as libc::c_int as flex_int16_t,
    134 as libc::c_int as flex_int16_t,
    473 as libc::c_int as flex_int16_t,
    193 as libc::c_int as flex_int16_t,
    183 as libc::c_int as flex_int16_t,
    132 as libc::c_int as flex_int16_t,
    194 as libc::c_int as flex_int16_t,
    467 as libc::c_int as flex_int16_t,
    517 as libc::c_int as flex_int16_t,
    554 as libc::c_int as flex_int16_t,
    513 as libc::c_int as flex_int16_t,
    554 as libc::c_int as flex_int16_t,
    554 as libc::c_int as flex_int16_t,
    554 as libc::c_int as flex_int16_t,
    554 as libc::c_int as flex_int16_t,
    221 as libc::c_int as flex_int16_t,
    514 as libc::c_int as flex_int16_t,
    554 as libc::c_int as flex_int16_t,
    222 as libc::c_int as flex_int16_t,
    232 as libc::c_int as flex_int16_t,
    513 as libc::c_int as flex_int16_t,
    554 as libc::c_int as flex_int16_t,
    554 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    468 as libc::c_int as flex_int16_t,
    482 as libc::c_int as flex_int16_t,
    472 as libc::c_int as flex_int16_t,
    479 as libc::c_int as flex_int16_t,
    465 as libc::c_int as flex_int16_t,
    465 as libc::c_int as flex_int16_t,
    470 as libc::c_int as flex_int16_t,
    462 as libc::c_int as flex_int16_t,
    479 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    460 as libc::c_int as flex_int16_t,
    464 as libc::c_int as flex_int16_t,
    464 as libc::c_int as flex_int16_t,
    475 as libc::c_int as flex_int16_t,
    466 as libc::c_int as flex_int16_t,
    465 as libc::c_int as flex_int16_t,
    459 as libc::c_int as flex_int16_t,
    206 as libc::c_int as flex_int16_t,
    471 as libc::c_int as flex_int16_t,
    453 as libc::c_int as flex_int16_t,
    461 as libc::c_int as flex_int16_t,
    451 as libc::c_int as flex_int16_t,
    459 as libc::c_int as flex_int16_t,
    554 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    492 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    178 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    233 as libc::c_int as flex_int16_t,
    490 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    241 as libc::c_int as flex_int16_t,
    244 as libc::c_int as flex_int16_t,
    489 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    443 as libc::c_int as flex_int16_t,
    67 as libc::c_int as flex_int16_t,
    192 as libc::c_int as flex_int16_t,
    189 as libc::c_int as flex_int16_t,
    219 as libc::c_int as flex_int16_t,
    138 as libc::c_int as flex_int16_t,
    203 as libc::c_int as flex_int16_t,
    221 as libc::c_int as flex_int16_t,
    180 as libc::c_int as flex_int16_t,
    234 as libc::c_int as flex_int16_t,
    442 as libc::c_int as flex_int16_t,
    224 as libc::c_int as flex_int16_t,
    223 as libc::c_int as flex_int16_t,
    233 as libc::c_int as flex_int16_t,
    246 as libc::c_int as flex_int16_t,
    239 as libc::c_int as flex_int16_t,
    240 as libc::c_int as flex_int16_t,
    236 as libc::c_int as flex_int16_t,
    250 as libc::c_int as flex_int16_t,
    251 as libc::c_int as flex_int16_t,
    235 as libc::c_int as flex_int16_t,
    253 as libc::c_int as flex_int16_t,
    245 as libc::c_int as flex_int16_t,
    255 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    278 as libc::c_int as flex_int16_t,
    486 as libc::c_int as flex_int16_t,
    287 as libc::c_int as flex_int16_t,
    288 as libc::c_int as flex_int16_t,
    485 as libc::c_int as flex_int16_t,
    445 as libc::c_int as flex_int16_t,
    458 as libc::c_int as flex_int16_t,
    438 as libc::c_int as flex_int16_t,
    448 as libc::c_int as flex_int16_t,
    451 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    435 as libc::c_int as flex_int16_t,
    434 as libc::c_int as flex_int16_t,
    434 as libc::c_int as flex_int16_t,
    432 as libc::c_int as flex_int16_t,
    444 as libc::c_int as flex_int16_t,
    441 as libc::c_int as flex_int16_t,
    430 as libc::c_int as flex_int16_t,
    434 as libc::c_int as flex_int16_t,
    427 as libc::c_int as flex_int16_t,
    442 as libc::c_int as flex_int16_t,
    441 as libc::c_int as flex_int16_t,
    423 as libc::c_int as flex_int16_t,
    431 as libc::c_int as flex_int16_t,
    422 as libc::c_int as flex_int16_t,
    437 as libc::c_int as flex_int16_t,
    422 as libc::c_int as flex_int16_t,
    427 as libc::c_int as flex_int16_t,
    291 as libc::c_int as flex_int16_t,
    462 as libc::c_int as flex_int16_t,
    297 as libc::c_int as flex_int16_t,
    461 as libc::c_int as flex_int16_t,
    244 as libc::c_int as flex_int16_t,
    268 as libc::c_int as flex_int16_t,
    259 as libc::c_int as flex_int16_t,
    281 as libc::c_int as flex_int16_t,
    286 as libc::c_int as flex_int16_t,
    415 as libc::c_int as flex_int16_t,
    275 as libc::c_int as flex_int16_t,
    277 as libc::c_int as flex_int16_t,
    280 as libc::c_int as flex_int16_t,
    283 as libc::c_int as flex_int16_t,
    293 as libc::c_int as flex_int16_t,
    284 as libc::c_int as flex_int16_t,
    287 as libc::c_int as flex_int16_t,
    295 as libc::c_int as flex_int16_t,
    290 as libc::c_int as flex_int16_t,
    308 as libc::c_int as flex_int16_t,
    309 as libc::c_int as flex_int16_t,
    414 as libc::c_int as flex_int16_t,
    303 as libc::c_int as flex_int16_t,
    297 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    302 as libc::c_int as flex_int16_t,
    310 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    423 as libc::c_int as flex_int16_t,
    424 as libc::c_int as flex_int16_t,
    418 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    416 as libc::c_int as flex_int16_t,
    425 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    409 as libc::c_int as flex_int16_t,
    408 as libc::c_int as flex_int16_t,
    422 as libc::c_int as flex_int16_t,
    406 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    410 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    406 as libc::c_int as flex_int16_t,
    418 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    421 as libc::c_int as flex_int16_t,
    416 as libc::c_int as flex_int16_t,
    399 as libc::c_int as flex_int16_t,
    314 as libc::c_int as flex_int16_t,
    312 as libc::c_int as flex_int16_t,
    313 as libc::c_int as flex_int16_t,
    398 as libc::c_int as flex_int16_t,
    397 as libc::c_int as flex_int16_t,
    311 as libc::c_int as flex_int16_t,
    323 as libc::c_int as flex_int16_t,
    396 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    319 as libc::c_int as flex_int16_t,
    336 as libc::c_int as flex_int16_t,
    322 as libc::c_int as flex_int16_t,
    395 as libc::c_int as flex_int16_t,
    330 as libc::c_int as flex_int16_t,
    394 as libc::c_int as flex_int16_t,
    328 as libc::c_int as flex_int16_t,
    342 as libc::c_int as flex_int16_t,
    393 as libc::c_int as flex_int16_t,
    392 as libc::c_int as flex_int16_t,
    347 as libc::c_int as flex_int16_t,
    345 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    398 as libc::c_int as flex_int16_t,
    406 as libc::c_int as flex_int16_t,
    392 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    401 as libc::c_int as flex_int16_t,
    389 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    394 as libc::c_int as flex_int16_t,
    392 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    391 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    383 as libc::c_int as flex_int16_t,
    338 as libc::c_int as flex_int16_t,
    348 as libc::c_int as flex_int16_t,
    337 as libc::c_int as flex_int16_t,
    382 as libc::c_int as flex_int16_t,
    346 as libc::c_int as flex_int16_t,
    341 as libc::c_int as flex_int16_t,
    381 as libc::c_int as flex_int16_t,
    380 as libc::c_int as flex_int16_t,
    343 as libc::c_int as flex_int16_t,
    351 as libc::c_int as flex_int16_t,
    379 as libc::c_int as flex_int16_t,
    356 as libc::c_int as flex_int16_t,
    378 as libc::c_int as flex_int16_t,
    373 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    364 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    367 as libc::c_int as flex_int16_t,
    365 as libc::c_int as flex_int16_t,
    364 as libc::c_int as flex_int16_t,
    350 as libc::c_int as flex_int16_t,
    363 as libc::c_int as flex_int16_t,
    362 as libc::c_int as flex_int16_t,
    361 as libc::c_int as flex_int16_t,
    359 as libc::c_int as flex_int16_t,
    355 as libc::c_int as flex_int16_t,
    374 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    337 as libc::c_int as flex_int16_t,
    368 as libc::c_int as flex_int16_t,
    237 as libc::c_int as flex_int16_t,
    357 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    168 as libc::c_int as flex_int16_t,
    127 as libc::c_int as flex_int16_t,
    554 as libc::c_int as flex_int16_t,
    416 as libc::c_int as flex_int16_t,
    122 as libc::c_int as flex_int16_t,
    419 as libc::c_int as flex_int16_t,
    422 as libc::c_int as flex_int16_t,
];
static mut yy_def: [flex_int16_t; 320] = [
    0 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    316 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    319 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    316 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    319 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
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
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
];
static mut yy_nxt: [flex_int16_t; 609] = [
    0 as libc::c_int as flex_int16_t,
    6 as libc::c_int as flex_int16_t,
    7 as libc::c_int as flex_int16_t,
    8 as libc::c_int as flex_int16_t,
    9 as libc::c_int as flex_int16_t,
    10 as libc::c_int as flex_int16_t,
    11 as libc::c_int as flex_int16_t,
    12 as libc::c_int as flex_int16_t,
    13 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    15 as libc::c_int as flex_int16_t,
    16 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    17 as libc::c_int as flex_int16_t,
    18 as libc::c_int as flex_int16_t,
    19 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    24 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    6 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    28 as libc::c_int as flex_int16_t,
    29 as libc::c_int as flex_int16_t,
    30 as libc::c_int as flex_int16_t,
    31 as libc::c_int as flex_int16_t,
    32 as libc::c_int as flex_int16_t,
    33 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    32 as libc::c_int as flex_int16_t,
    32 as libc::c_int as flex_int16_t,
    35 as libc::c_int as flex_int16_t,
    32 as libc::c_int as flex_int16_t,
    32 as libc::c_int as flex_int16_t,
    36 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    38 as libc::c_int as flex_int16_t,
    39 as libc::c_int as flex_int16_t,
    40 as libc::c_int as flex_int16_t,
    32 as libc::c_int as flex_int16_t,
    32 as libc::c_int as flex_int16_t,
    41 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    32 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    43 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    44 as libc::c_int as flex_int16_t,
    45 as libc::c_int as flex_int16_t,
    46 as libc::c_int as flex_int16_t,
    47 as libc::c_int as flex_int16_t,
    48 as libc::c_int as flex_int16_t,
    49 as libc::c_int as flex_int16_t,
    50 as libc::c_int as flex_int16_t,
    51 as libc::c_int as flex_int16_t,
    52 as libc::c_int as flex_int16_t,
    52 as libc::c_int as flex_int16_t,
    53 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    52 as libc::c_int as flex_int16_t,
    55 as libc::c_int as flex_int16_t,
    56 as libc::c_int as flex_int16_t,
    57 as libc::c_int as flex_int16_t,
    58 as libc::c_int as flex_int16_t,
    52 as libc::c_int as flex_int16_t,
    59 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    61 as libc::c_int as flex_int16_t,
    58 as libc::c_int as flex_int16_t,
    52 as libc::c_int as flex_int16_t,
    62 as libc::c_int as flex_int16_t,
    52 as libc::c_int as flex_int16_t,
    63 as libc::c_int as flex_int16_t,
    44 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    65 as libc::c_int as flex_int16_t,
    66 as libc::c_int as flex_int16_t,
    67 as libc::c_int as flex_int16_t,
    68 as libc::c_int as flex_int16_t,
    69 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    71 as libc::c_int as flex_int16_t,
    72 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    73 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    74 as libc::c_int as flex_int16_t,
    75 as libc::c_int as flex_int16_t,
    76 as libc::c_int as flex_int16_t,
    77 as libc::c_int as flex_int16_t,
    78 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    79 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    52 as libc::c_int as flex_int16_t,
    81 as libc::c_int as flex_int16_t,
    52 as libc::c_int as flex_int16_t,
    88 as libc::c_int as flex_int16_t,
    88 as libc::c_int as flex_int16_t,
    104 as libc::c_int as flex_int16_t,
    91 as libc::c_int as flex_int16_t,
    89 as libc::c_int as flex_int16_t,
    194 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    86 as libc::c_int as flex_int16_t,
    86 as libc::c_int as flex_int16_t,
    89 as libc::c_int as flex_int16_t,
    105 as libc::c_int as flex_int16_t,
    90 as libc::c_int as flex_int16_t,
    86 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    95 as libc::c_int as flex_int16_t,
    93 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    95 as libc::c_int as flex_int16_t,
    95 as libc::c_int as flex_int16_t,
    93 as libc::c_int as flex_int16_t,
    95 as libc::c_int as flex_int16_t,
    94 as libc::c_int as flex_int16_t,
    95 as libc::c_int as flex_int16_t,
    106 as libc::c_int as flex_int16_t,
    114 as libc::c_int as flex_int16_t,
    108 as libc::c_int as flex_int16_t,
    83 as libc::c_int as flex_int16_t,
    107 as libc::c_int as flex_int16_t,
    115 as libc::c_int as flex_int16_t,
    109 as libc::c_int as flex_int16_t,
    117 as libc::c_int as flex_int16_t,
    119 as libc::c_int as flex_int16_t,
    95 as libc::c_int as flex_int16_t,
    110 as libc::c_int as flex_int16_t,
    84 as libc::c_int as flex_int16_t,
    129 as libc::c_int as flex_int16_t,
    126 as libc::c_int as flex_int16_t,
    130 as libc::c_int as flex_int16_t,
    120 as libc::c_int as flex_int16_t,
    129 as libc::c_int as flex_int16_t,
    132 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    130 as libc::c_int as flex_int16_t,
    127 as libc::c_int as flex_int16_t,
    131 as libc::c_int as flex_int16_t,
    127 as libc::c_int as flex_int16_t,
    133 as libc::c_int as flex_int16_t,
    136 as libc::c_int as flex_int16_t,
    134 as libc::c_int as flex_int16_t,
    127 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    136 as libc::c_int as flex_int16_t,
    136 as libc::c_int as flex_int16_t,
    134 as libc::c_int as flex_int16_t,
    136 as libc::c_int as flex_int16_t,
    135 as libc::c_int as flex_int16_t,
    136 as libc::c_int as flex_int16_t,
    140 as libc::c_int as flex_int16_t,
    139 as libc::c_int as flex_int16_t,
    141 as libc::c_int as flex_int16_t,
    124 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    143 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    158 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    136 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    152 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    84 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    126 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    138 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    156 as libc::c_int as flex_int16_t,
    146 as libc::c_int as flex_int16_t,
    148 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    151 as libc::c_int as flex_int16_t,
    147 as libc::c_int as flex_int16_t,
    149 as libc::c_int as flex_int16_t,
    145 as libc::c_int as flex_int16_t,
    154 as libc::c_int as flex_int16_t,
    159 as libc::c_int as flex_int16_t,
    150 as libc::c_int as flex_int16_t,
    195 as libc::c_int as flex_int16_t,
    155 as libc::c_int as flex_int16_t,
    201 as libc::c_int as flex_int16_t,
    157 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    160 as libc::c_int as flex_int16_t,
    196 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    183 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    162 as libc::c_int as flex_int16_t,
    165 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    162 as libc::c_int as flex_int16_t,
    165 as libc::c_int as flex_int16_t,
    163 as libc::c_int as flex_int16_t,
    166 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    199 as libc::c_int as flex_int16_t,
    93 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    197 as libc::c_int as flex_int16_t,
    184 as libc::c_int as flex_int16_t,
    93 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    94 as libc::c_int as flex_int16_t,
    191 as libc::c_int as flex_int16_t,
    192 as libc::c_int as flex_int16_t,
    133 as libc::c_int as flex_int16_t,
    200 as libc::c_int as flex_int16_t,
    134 as libc::c_int as flex_int16_t,
    202 as libc::c_int as flex_int16_t,
    192 as libc::c_int as flex_int16_t,
    204 as libc::c_int as flex_int16_t,
    193 as libc::c_int as flex_int16_t,
    134 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    135 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    203 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    205 as libc::c_int as flex_int16_t,
    206 as libc::c_int as flex_int16_t,
    207 as libc::c_int as flex_int16_t,
    208 as libc::c_int as flex_int16_t,
    209 as libc::c_int as flex_int16_t,
    210 as libc::c_int as flex_int16_t,
    212 as libc::c_int as flex_int16_t,
    213 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    239 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    214 as libc::c_int as flex_int16_t,
    215 as libc::c_int as flex_int16_t,
    216 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    162 as libc::c_int as flex_int16_t,
    240 as libc::c_int as flex_int16_t,
    211 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    162 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    163 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    89 as libc::c_int as flex_int16_t,
    165 as libc::c_int as flex_int16_t,
    241 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    89 as libc::c_int as flex_int16_t,
    165 as libc::c_int as flex_int16_t,
    90 as libc::c_int as flex_int16_t,
    166 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    192 as libc::c_int as flex_int16_t,
    191 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    242 as libc::c_int as flex_int16_t,
    243 as libc::c_int as flex_int16_t,
    192 as libc::c_int as flex_int16_t,
    249 as libc::c_int as flex_int16_t,
    193 as libc::c_int as flex_int16_t,
    244 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    245 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    246 as libc::c_int as flex_int16_t,
    248 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    247 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    250 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    251 as libc::c_int as flex_int16_t,
    252 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    253 as libc::c_int as flex_int16_t,
    254 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    256 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    257 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    258 as libc::c_int as flex_int16_t,
    259 as libc::c_int as flex_int16_t,
    276 as libc::c_int as flex_int16_t,
    260 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    275 as libc::c_int as flex_int16_t,
    278 as libc::c_int as flex_int16_t,
    277 as libc::c_int as flex_int16_t,
    279 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    280 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    281 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    282 as libc::c_int as flex_int16_t,
    283 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    284 as libc::c_int as flex_int16_t,
    285 as libc::c_int as flex_int16_t,
    286 as libc::c_int as flex_int16_t,
    287 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    288 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    297 as libc::c_int as flex_int16_t,
    298 as libc::c_int as flex_int16_t,
    300 as libc::c_int as flex_int16_t,
    299 as libc::c_int as flex_int16_t,
    302 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    301 as libc::c_int as flex_int16_t,
    312 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    303 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    304 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    313 as libc::c_int as flex_int16_t,
    309 as libc::c_int as flex_int16_t,
    310 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    311 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    314 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    308 as libc::c_int as flex_int16_t,
    307 as libc::c_int as flex_int16_t,
    306 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    84 as libc::c_int as flex_int16_t,
    84 as libc::c_int as flex_int16_t,
    84 as libc::c_int as flex_int16_t,
    122 as libc::c_int as flex_int16_t,
    305 as libc::c_int as flex_int16_t,
    122 as libc::c_int as flex_int16_t,
    125 as libc::c_int as flex_int16_t,
    125 as libc::c_int as flex_int16_t,
    125 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    296 as libc::c_int as flex_int16_t,
    295 as libc::c_int as flex_int16_t,
    294 as libc::c_int as flex_int16_t,
    293 as libc::c_int as flex_int16_t,
    292 as libc::c_int as flex_int16_t,
    291 as libc::c_int as flex_int16_t,
    290 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    274 as libc::c_int as flex_int16_t,
    273 as libc::c_int as flex_int16_t,
    272 as libc::c_int as flex_int16_t,
    271 as libc::c_int as flex_int16_t,
    270 as libc::c_int as flex_int16_t,
    269 as libc::c_int as flex_int16_t,
    268 as libc::c_int as flex_int16_t,
    267 as libc::c_int as flex_int16_t,
    266 as libc::c_int as flex_int16_t,
    265 as libc::c_int as flex_int16_t,
    264 as libc::c_int as flex_int16_t,
    263 as libc::c_int as flex_int16_t,
    262 as libc::c_int as flex_int16_t,
    261 as libc::c_int as flex_int16_t,
    255 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    165 as libc::c_int as flex_int16_t,
    162 as libc::c_int as flex_int16_t,
    238 as libc::c_int as flex_int16_t,
    237 as libc::c_int as flex_int16_t,
    236 as libc::c_int as flex_int16_t,
    235 as libc::c_int as flex_int16_t,
    234 as libc::c_int as flex_int16_t,
    233 as libc::c_int as flex_int16_t,
    232 as libc::c_int as flex_int16_t,
    231 as libc::c_int as flex_int16_t,
    230 as libc::c_int as flex_int16_t,
    229 as libc::c_int as flex_int16_t,
    228 as libc::c_int as flex_int16_t,
    227 as libc::c_int as flex_int16_t,
    226 as libc::c_int as flex_int16_t,
    225 as libc::c_int as flex_int16_t,
    224 as libc::c_int as flex_int16_t,
    223 as libc::c_int as flex_int16_t,
    222 as libc::c_int as flex_int16_t,
    221 as libc::c_int as flex_int16_t,
    220 as libc::c_int as flex_int16_t,
    219 as libc::c_int as flex_int16_t,
    218 as libc::c_int as flex_int16_t,
    217 as libc::c_int as flex_int16_t,
    165 as libc::c_int as flex_int16_t,
    162 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    93 as libc::c_int as flex_int16_t,
    164 as libc::c_int as flex_int16_t,
    123 as libc::c_int as flex_int16_t,
    189 as libc::c_int as flex_int16_t,
    188 as libc::c_int as flex_int16_t,
    187 as libc::c_int as flex_int16_t,
    186 as libc::c_int as flex_int16_t,
    185 as libc::c_int as flex_int16_t,
    182 as libc::c_int as flex_int16_t,
    181 as libc::c_int as flex_int16_t,
    180 as libc::c_int as flex_int16_t,
    179 as libc::c_int as flex_int16_t,
    178 as libc::c_int as flex_int16_t,
    177 as libc::c_int as flex_int16_t,
    176 as libc::c_int as flex_int16_t,
    175 as libc::c_int as flex_int16_t,
    174 as libc::c_int as flex_int16_t,
    173 as libc::c_int as flex_int16_t,
    172 as libc::c_int as flex_int16_t,
    171 as libc::c_int as flex_int16_t,
    170 as libc::c_int as flex_int16_t,
    169 as libc::c_int as flex_int16_t,
    168 as libc::c_int as flex_int16_t,
    167 as libc::c_int as flex_int16_t,
    93 as libc::c_int as flex_int16_t,
    164 as libc::c_int as flex_int16_t,
    85 as libc::c_int as flex_int16_t,
    82 as libc::c_int as flex_int16_t,
    161 as libc::c_int as flex_int16_t,
    153 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    127 as libc::c_int as flex_int16_t,
    96 as libc::c_int as flex_int16_t,
    124 as libc::c_int as flex_int16_t,
    124 as libc::c_int as flex_int16_t,
    127 as libc::c_int as flex_int16_t,
    128 as libc::c_int as flex_int16_t,
    127 as libc::c_int as flex_int16_t,
    124 as libc::c_int as flex_int16_t,
    123 as libc::c_int as flex_int16_t,
    121 as libc::c_int as flex_int16_t,
    118 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    112 as libc::c_int as flex_int16_t,
    111 as libc::c_int as flex_int16_t,
    103 as libc::c_int as flex_int16_t,
    102 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    100 as libc::c_int as flex_int16_t,
    99 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    86 as libc::c_int as flex_int16_t,
    96 as libc::c_int as flex_int16_t,
    83 as libc::c_int as flex_int16_t,
    83 as libc::c_int as flex_int16_t,
    86 as libc::c_int as flex_int16_t,
    87 as libc::c_int as flex_int16_t,
    86 as libc::c_int as flex_int16_t,
    85 as libc::c_int as flex_int16_t,
    83 as libc::c_int as flex_int16_t,
    82 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    5 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
];
static mut yy_chk: [flex_int16_t; 609] = [
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
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    16 as libc::c_int as flex_int16_t,
    17 as libc::c_int as flex_int16_t,
    33 as libc::c_int as flex_int16_t,
    19 as libc::c_int as flex_int16_t,
    18 as libc::c_int as flex_int16_t,
    138 as libc::c_int as flex_int16_t,
    138 as libc::c_int as flex_int16_t,
    17 as libc::c_int as flex_int16_t,
    16 as libc::c_int as flex_int16_t,
    18 as libc::c_int as flex_int16_t,
    33 as libc::c_int as flex_int16_t,
    18 as libc::c_int as flex_int16_t,
    19 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    317 as libc::c_int as flex_int16_t,
    40 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    39 as libc::c_int as flex_int16_t,
    35 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    39 as libc::c_int as flex_int16_t,
    35 as libc::c_int as flex_int16_t,
    40 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    35 as libc::c_int as flex_int16_t,
    48 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    48 as libc::c_int as flex_int16_t,
    56 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    55 as libc::c_int as flex_int16_t,
    57 as libc::c_int as flex_int16_t,
    68 as libc::c_int as flex_int16_t,
    56 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    56 as libc::c_int as flex_int16_t,
    55 as libc::c_int as flex_int16_t,
    58 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    58 as libc::c_int as flex_int16_t,
    57 as libc::c_int as flex_int16_t,
    68 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    58 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    58 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    66 as libc::c_int as flex_int16_t,
    65 as libc::c_int as flex_int16_t,
    67 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    65 as libc::c_int as flex_int16_t,
    69 as libc::c_int as flex_int16_t,
    66 as libc::c_int as flex_int16_t,
    79 as libc::c_int as flex_int16_t,
    314 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    69 as libc::c_int as flex_int16_t,
    75 as libc::c_int as flex_int16_t,
    79 as libc::c_int as flex_int16_t,
    125 as libc::c_int as flex_int16_t,
    75 as libc::c_int as flex_int16_t,
    125 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    67 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    71 as libc::c_int as flex_int16_t,
    78 as libc::c_int as flex_int16_t,
    72 as libc::c_int as flex_int16_t,
    73 as libc::c_int as flex_int16_t,
    313 as libc::c_int as flex_int16_t,
    74 as libc::c_int as flex_int16_t,
    72 as libc::c_int as flex_int16_t,
    73 as libc::c_int as flex_int16_t,
    71 as libc::c_int as flex_int16_t,
    77 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    73 as libc::c_int as flex_int16_t,
    139 as libc::c_int as flex_int16_t,
    77 as libc::c_int as flex_int16_t,
    145 as libc::c_int as flex_int16_t,
    78 as libc::c_int as flex_int16_t,
    145 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    140 as libc::c_int as flex_int16_t,
    78 as libc::c_int as flex_int16_t,
    71 as libc::c_int as flex_int16_t,
    72 as libc::c_int as flex_int16_t,
    115 as libc::c_int as flex_int16_t,
    73 as libc::c_int as flex_int16_t,
    74 as libc::c_int as flex_int16_t,
    140 as libc::c_int as flex_int16_t,
    89 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    139 as libc::c_int as flex_int16_t,
    77 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    89 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    89 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    93 as libc::c_int as flex_int16_t,
    143 as libc::c_int as flex_int16_t,
    93 as libc::c_int as flex_int16_t,
    130 as libc::c_int as flex_int16_t,
    143 as libc::c_int as flex_int16_t,
    141 as libc::c_int as flex_int16_t,
    115 as libc::c_int as flex_int16_t,
    93 as libc::c_int as flex_int16_t,
    130 as libc::c_int as flex_int16_t,
    93 as libc::c_int as flex_int16_t,
    130 as libc::c_int as flex_int16_t,
    133 as libc::c_int as flex_int16_t,
    134 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    134 as libc::c_int as flex_int16_t,
    146 as libc::c_int as flex_int16_t,
    133 as libc::c_int as flex_int16_t,
    149 as libc::c_int as flex_int16_t,
    133 as libc::c_int as flex_int16_t,
    134 as libc::c_int as flex_int16_t,
    141 as libc::c_int as flex_int16_t,
    134 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    148 as libc::c_int as flex_int16_t,
    149 as libc::c_int as flex_int16_t,
    148 as libc::c_int as flex_int16_t,
    150 as libc::c_int as flex_int16_t,
    151 as libc::c_int as flex_int16_t,
    152 as libc::c_int as flex_int16_t,
    153 as libc::c_int as flex_int16_t,
    154 as libc::c_int as flex_int16_t,
    155 as libc::c_int as flex_int16_t,
    156 as libc::c_int as flex_int16_t,
    157 as libc::c_int as flex_int16_t,
    150 as libc::c_int as flex_int16_t,
    146 as libc::c_int as flex_int16_t,
    157 as libc::c_int as flex_int16_t,
    154 as libc::c_int as flex_int16_t,
    309 as libc::c_int as flex_int16_t,
    194 as libc::c_int as flex_int16_t,
    152 as libc::c_int as flex_int16_t,
    153 as libc::c_int as flex_int16_t,
    158 as libc::c_int as flex_int16_t,
    159 as libc::c_int as flex_int16_t,
    160 as libc::c_int as flex_int16_t,
    194 as libc::c_int as flex_int16_t,
    159 as libc::c_int as flex_int16_t,
    151 as libc::c_int as flex_int16_t,
    162 as libc::c_int as flex_int16_t,
    195 as libc::c_int as flex_int16_t,
    155 as libc::c_int as flex_int16_t,
    155 as libc::c_int as flex_int16_t,
    156 as libc::c_int as flex_int16_t,
    162 as libc::c_int as flex_int16_t,
    158 as libc::c_int as flex_int16_t,
    162 as libc::c_int as flex_int16_t,
    160 as libc::c_int as flex_int16_t,
    164 as libc::c_int as flex_int16_t,
    165 as libc::c_int as flex_int16_t,
    196 as libc::c_int as flex_int16_t,
    196 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    164 as libc::c_int as flex_int16_t,
    165 as libc::c_int as flex_int16_t,
    164 as libc::c_int as flex_int16_t,
    165 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    192 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    195 as libc::c_int as flex_int16_t,
    197 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    192 as libc::c_int as flex_int16_t,
    205 as libc::c_int as flex_int16_t,
    192 as libc::c_int as flex_int16_t,
    200 as libc::c_int as flex_int16_t,
    200 as libc::c_int as flex_int16_t,
    201 as libc::c_int as flex_int16_t,
    201 as libc::c_int as flex_int16_t,
    202 as libc::c_int as flex_int16_t,
    204 as libc::c_int as flex_int16_t,
    202 as libc::c_int as flex_int16_t,
    197 as libc::c_int as flex_int16_t,
    203 as libc::c_int as flex_int16_t,
    203 as libc::c_int as flex_int16_t,
    205 as libc::c_int as flex_int16_t,
    206 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    206 as libc::c_int as flex_int16_t,
    207 as libc::c_int as flex_int16_t,
    208 as libc::c_int as flex_int16_t,
    208 as libc::c_int as flex_int16_t,
    209 as libc::c_int as flex_int16_t,
    210 as libc::c_int as flex_int16_t,
    204 as libc::c_int as flex_int16_t,
    212 as libc::c_int as flex_int16_t,
    207 as libc::c_int as flex_int16_t,
    213 as libc::c_int as flex_int16_t,
    213 as libc::c_int as flex_int16_t,
    214 as libc::c_int as flex_int16_t,
    215 as libc::c_int as flex_int16_t,
    241 as libc::c_int as flex_int16_t,
    216 as libc::c_int as flex_int16_t,
    215 as libc::c_int as flex_int16_t,
    212 as libc::c_int as flex_int16_t,
    240 as libc::c_int as flex_int16_t,
    245 as libc::c_int as flex_int16_t,
    242 as libc::c_int as flex_int16_t,
    246 as libc::c_int as flex_int16_t,
    209 as libc::c_int as flex_int16_t,
    210 as libc::c_int as flex_int16_t,
    216 as libc::c_int as flex_int16_t,
    245 as libc::c_int as flex_int16_t,
    241 as libc::c_int as flex_int16_t,
    242 as libc::c_int as flex_int16_t,
    240 as libc::c_int as flex_int16_t,
    214 as libc::c_int as flex_int16_t,
    248 as libc::c_int as flex_int16_t,
    248 as libc::c_int as flex_int16_t,
    249 as libc::c_int as flex_int16_t,
    249 as libc::c_int as flex_int16_t,
    250 as libc::c_int as flex_int16_t,
    251 as libc::c_int as flex_int16_t,
    251 as libc::c_int as flex_int16_t,
    246 as libc::c_int as flex_int16_t,
    253 as libc::c_int as flex_int16_t,
    255 as libc::c_int as flex_int16_t,
    256 as libc::c_int as flex_int16_t,
    259 as libc::c_int as flex_int16_t,
    255 as libc::c_int as flex_int16_t,
    260 as libc::c_int as flex_int16_t,
    253 as libc::c_int as flex_int16_t,
    276 as libc::c_int as flex_int16_t,
    277 as libc::c_int as flex_int16_t,
    280 as libc::c_int as flex_int16_t,
    278 as libc::c_int as flex_int16_t,
    284 as libc::c_int as flex_int16_t,
    250 as libc::c_int as flex_int16_t,
    278 as libc::c_int as flex_int16_t,
    276 as libc::c_int as flex_int16_t,
    281 as libc::c_int as flex_int16_t,
    307 as libc::c_int as flex_int16_t,
    281 as libc::c_int as flex_int16_t,
    256 as libc::c_int as flex_int16_t,
    284 as libc::c_int as flex_int16_t,
    285 as libc::c_int as flex_int16_t,
    260 as libc::c_int as flex_int16_t,
    280 as libc::c_int as flex_int16_t,
    259 as libc::c_int as flex_int16_t,
    277 as libc::c_int as flex_int16_t,
    287 as libc::c_int as flex_int16_t,
    299 as libc::c_int as flex_int16_t,
    285 as libc::c_int as flex_int16_t,
    308 as libc::c_int as flex_int16_t,
    299 as libc::c_int as flex_int16_t,
    304 as libc::c_int as flex_int16_t,
    304 as libc::c_int as flex_int16_t,
    287 as libc::c_int as flex_int16_t,
    310 as libc::c_int as flex_int16_t,
    305 as libc::c_int as flex_int16_t,
    303 as libc::c_int as flex_int16_t,
    310 as libc::c_int as flex_int16_t,
    302 as libc::c_int as flex_int16_t,
    301 as libc::c_int as flex_int16_t,
    300 as libc::c_int as flex_int16_t,
    298 as libc::c_int as flex_int16_t,
    297 as libc::c_int as flex_int16_t,
    296 as libc::c_int as flex_int16_t,
    291 as libc::c_int as flex_int16_t,
    308 as libc::c_int as flex_int16_t,
    316 as libc::c_int as flex_int16_t,
    316 as libc::c_int as flex_int16_t,
    316 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    289 as libc::c_int as flex_int16_t,
    318 as libc::c_int as flex_int16_t,
    319 as libc::c_int as flex_int16_t,
    319 as libc::c_int as flex_int16_t,
    319 as libc::c_int as flex_int16_t,
    288 as libc::c_int as flex_int16_t,
    286 as libc::c_int as flex_int16_t,
    283 as libc::c_int as flex_int16_t,
    282 as libc::c_int as flex_int16_t,
    279 as libc::c_int as flex_int16_t,
    275 as libc::c_int as flex_int16_t,
    273 as libc::c_int as flex_int16_t,
    271 as libc::c_int as flex_int16_t,
    270 as libc::c_int as flex_int16_t,
    267 as libc::c_int as flex_int16_t,
    266 as libc::c_int as flex_int16_t,
    264 as libc::c_int as flex_int16_t,
    263 as libc::c_int as flex_int16_t,
    262 as libc::c_int as flex_int16_t,
    258 as libc::c_int as flex_int16_t,
    257 as libc::c_int as flex_int16_t,
    254 as libc::c_int as flex_int16_t,
    252 as libc::c_int as flex_int16_t,
    247 as libc::c_int as flex_int16_t,
    244 as libc::c_int as flex_int16_t,
    243 as libc::c_int as flex_int16_t,
    239 as libc::c_int as flex_int16_t,
    238 as libc::c_int as flex_int16_t,
    237 as libc::c_int as flex_int16_t,
    234 as libc::c_int as flex_int16_t,
    233 as libc::c_int as flex_int16_t,
    231 as libc::c_int as flex_int16_t,
    229 as libc::c_int as flex_int16_t,
    228 as libc::c_int as flex_int16_t,
    227 as libc::c_int as flex_int16_t,
    226 as libc::c_int as flex_int16_t,
    224 as libc::c_int as flex_int16_t,
    223 as libc::c_int as flex_int16_t,
    220 as libc::c_int as flex_int16_t,
    219 as libc::c_int as flex_int16_t,
    218 as libc::c_int as flex_int16_t,
    211 as libc::c_int as flex_int16_t,
    199 as libc::c_int as flex_int16_t,
    193 as libc::c_int as flex_int16_t,
    191 as libc::c_int as flex_int16_t,
    189 as libc::c_int as flex_int16_t,
    188 as libc::c_int as flex_int16_t,
    187 as libc::c_int as flex_int16_t,
    186 as libc::c_int as flex_int16_t,
    185 as libc::c_int as flex_int16_t,
    184 as libc::c_int as flex_int16_t,
    183 as libc::c_int as flex_int16_t,
    182 as libc::c_int as flex_int16_t,
    181 as libc::c_int as flex_int16_t,
    180 as libc::c_int as flex_int16_t,
    179 as libc::c_int as flex_int16_t,
    178 as libc::c_int as flex_int16_t,
    177 as libc::c_int as flex_int16_t,
    176 as libc::c_int as flex_int16_t,
    175 as libc::c_int as flex_int16_t,
    174 as libc::c_int as flex_int16_t,
    173 as libc::c_int as flex_int16_t,
    171 as libc::c_int as flex_int16_t,
    170 as libc::c_int as flex_int16_t,
    169 as libc::c_int as flex_int16_t,
    168 as libc::c_int as flex_int16_t,
    167 as libc::c_int as flex_int16_t,
    166 as libc::c_int as flex_int16_t,
    163 as libc::c_int as flex_int16_t,
    147 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    135 as libc::c_int as flex_int16_t,
    131 as libc::c_int as flex_int16_t,
    123 as libc::c_int as flex_int16_t,
    120 as libc::c_int as flex_int16_t,
    119 as libc::c_int as flex_int16_t,
    118 as libc::c_int as flex_int16_t,
    117 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    114 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    112 as libc::c_int as flex_int16_t,
    111 as libc::c_int as flex_int16_t,
    110 as libc::c_int as flex_int16_t,
    109 as libc::c_int as flex_int16_t,
    108 as libc::c_int as flex_int16_t,
    106 as libc::c_int as flex_int16_t,
    105 as libc::c_int as flex_int16_t,
    104 as libc::c_int as flex_int16_t,
    103 as libc::c_int as flex_int16_t,
    102 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    100 as libc::c_int as flex_int16_t,
    99 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    94 as libc::c_int as flex_int16_t,
    90 as libc::c_int as flex_int16_t,
    84 as libc::c_int as flex_int16_t,
    82 as libc::c_int as flex_int16_t,
    81 as libc::c_int as flex_int16_t,
    76 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    63 as libc::c_int as flex_int16_t,
    62 as libc::c_int as flex_int16_t,
    61 as libc::c_int as flex_int16_t,
    59 as libc::c_int as flex_int16_t,
    53 as libc::c_int as flex_int16_t,
    51 as libc::c_int as flex_int16_t,
    50 as libc::c_int as flex_int16_t,
    47 as libc::c_int as flex_int16_t,
    45 as libc::c_int as flex_int16_t,
    43 as libc::c_int as flex_int16_t,
    41 as libc::c_int as flex_int16_t,
    38 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    36 as libc::c_int as flex_int16_t,
    31 as libc::c_int as flex_int16_t,
    30 as libc::c_int as flex_int16_t,
    29 as libc::c_int as flex_int16_t,
    28 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    24 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    15 as libc::c_int as flex_int16_t,
    13 as libc::c_int as flex_int16_t,
    12 as libc::c_int as flex_int16_t,
    10 as libc::c_int as flex_int16_t,
    9 as libc::c_int as flex_int16_t,
    7 as libc::c_int as flex_int16_t,
    5 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
    315 as libc::c_int as flex_int16_t,
];
static mut yy_last_accepting_state: yy_state_type = 0;
static mut yy_last_accepting_cpos: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut yy_flex_debug: libc::c_int = 0 as libc::c_int;
pub static mut yytext: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub unsafe extern "C" fn yylex() -> libc::c_int {
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
                let mut yy_c: YY_CHAR = yy_ec[*yy_cp as libc::c_uchar as libc::c_uint
                    as usize] as YY_CHAR;
                if yy_accept[yy_current_state as usize] != 0 {
                    yy_last_accepting_state = yy_current_state;
                    yy_last_accepting_cpos = yy_cp;
                }
                while yy_chk[(yy_base[yy_current_state as usize] as libc::c_int
                    + yy_c as libc::c_int) as usize] as libc::c_int != yy_current_state
                {
                    yy_current_state = yy_def[yy_current_state as usize] as libc::c_int;
                    if yy_current_state >= 316 as libc::c_int {
                        yy_c = yy_meta[yy_c as libc::c_uint as usize] as YY_CHAR;
                    }
                }
                yy_current_state = yy_nxt[(yy_base[yy_current_state as usize]
                    as libc::c_uint)
                    .wrapping_add(yy_c as libc::c_uint) as usize] as yy_state_type;
                yy_cp = yy_cp.offset(1);
                yy_cp;
                if !(yy_base[yy_current_state as usize] as libc::c_int
                    != 554 as libc::c_int)
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
                yyleng = yy_cp.offset_from(yy_bp) as libc::c_long as size_t
                    as libc::c_int;
                yy_hold_char = *yy_cp;
                *yy_cp = '\0' as i32 as libc::c_char;
                yy_c_buf_p = yy_cp;
                loop {
                    match yy_act {
                        0 => {
                            *yy_cp = yy_hold_char;
                            yy_cp = yy_last_accepting_cpos;
                            yy_current_state = yy_last_accepting_state;
                            continue '_yy_find_action;
                        }
                        1 => {
                            if std_only == 0 {
                                yy_start = 1 as libc::c_int
                                    + 2 as libc::c_int * 1 as libc::c_int;
                            } else {
                                yyerror(
                                    b"illegal character: #\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            break '_yy_match;
                        }
                        2 => {
                            yy_start = 1 as libc::c_int
                                + 2 as libc::c_int * 0 as libc::c_int;
                            break '_yy_match;
                        }
                        3 => {
                            line_no += 1;
                            line_no;
                            yy_start = 1 as libc::c_int
                                + 2 as libc::c_int * 0 as libc::c_int;
                            return 258 as libc::c_int;
                        }
                        4 => return 268 as libc::c_int,
                        5 => return 269 as libc::c_int,
                        6 => return 270 as libc::c_int,
                        7 => return 271 as libc::c_int,
                        8 => return 272 as libc::c_int,
                        9 => return 273 as libc::c_int,
                        10 => return 274 as libc::c_int,
                        11 => return 275 as libc::c_int,
                        12 => return 276 as libc::c_int,
                        13 => return 278 as libc::c_int,
                        14 => return 279 as libc::c_int,
                        15 => return 280 as libc::c_int,
                        16 => return 281 as libc::c_int,
                        17 => return 277 as libc::c_int,
                        18 => return 282 as libc::c_int,
                        19 => return 283 as libc::c_int,
                        20 => return 285 as libc::c_int,
                        21 => return 286 as libc::c_int,
                        22 => return 292 as libc::c_int,
                        23 => {
                            yylval.s_value = strcopyof(yytext);
                            return 263 as libc::c_int;
                        }
                        24 => return 284 as libc::c_int,
                        25 => return 287 as libc::c_int,
                        26 => return 288 as libc::c_int,
                        27 => return 289 as libc::c_int,
                        28 => return 286 as libc::c_int,
                        29 => {
                            yylval.c_value = *yytext.offset(0 as libc::c_int as isize);
                            return *yytext.offset(0 as libc::c_int as isize)
                                as libc::c_int;
                        }
                        30 => return 259 as libc::c_int,
                        31 => return 260 as libc::c_int,
                        32 => return 261 as libc::c_int,
                        33 => {
                            yylval.c_value = *yytext.offset(0 as libc::c_int as isize);
                            return *yytext.offset(0 as libc::c_int as isize)
                                as libc::c_int;
                        }
                        34 => {
                            yylval.c_value = *yytext.offset(0 as libc::c_int as isize);
                            return 265 as libc::c_int;
                        }
                        35 => {
                            yylval.c_value = '=' as i32 as libc::c_char;
                            let mut yyless_macro_arg: libc::c_int = 1 as libc::c_int;
                            *yy_cp = yy_hold_char;
                            yy_cp = yy_bp
                                .offset(yyless_macro_arg as isize)
                                .offset(-(0 as libc::c_int as isize));
                            yy_c_buf_p = yy_cp;
                            yytext = yy_bp;
                            yyleng = yy_cp.offset_from(yy_bp) as libc::c_long as size_t
                                as libc::c_int;
                            yy_hold_char = *yy_cp;
                            *yy_cp = '\0' as i32 as libc::c_char;
                            yy_c_buf_p = yy_cp;
                            return 265 as libc::c_int;
                        }
                        36 => {
                            yylval.s_value = strcopyof(yytext);
                            return 266 as libc::c_int;
                        }
                        37 => {
                            yylval.c_value = *yytext.offset(0 as libc::c_int as isize);
                            return 267 as libc::c_int;
                        }
                        38 => {
                            line_no += 1;
                            line_no;
                            return 258 as libc::c_int;
                        }
                        39 => {
                            line_no += 1;
                            line_no;
                            break '_yy_match;
                        }
                        40 => {
                            break '_yy_match;
                        }
                        41 => {
                            let mut c: libc::c_int = 0;
                            loop {
                                loop {
                                    c = input();
                                    if !(c != '*' as i32 && c != -(1 as libc::c_int)) {
                                        break;
                                    }
                                    if c == '\n' as i32 {
                                        line_no += 1;
                                        line_no;
                                    }
                                }
                                if c == '*' as i32 {
                                    loop {
                                        c = input();
                                        if !(c == '*' as i32) {
                                            break;
                                        }
                                    }
                                    if c == '/' as i32 {
                                        break;
                                    }
                                    if c == '\n' as i32 {
                                        line_no += 1;
                                        line_no;
                                    }
                                }
                                if !(c == -(1 as libc::c_int)) {
                                    continue;
                                }
                                fprintf(
                                    stderr,
                                    b"EOF encountered in a comment.\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                break;
                            }
                            break '_yy_match;
                        }
                        42 => {
                            yylval.s_value = strcopyof(yytext);
                            return 263 as libc::c_int;
                        }
                        43 => {
                            let mut look: *const libc::c_char = 0 as *const libc::c_char;
                            let mut count: libc::c_int = 0 as libc::c_int;
                            yylval.s_value = strcopyof(yytext);
                            look = yytext;
                            while *look as libc::c_int != 0 as libc::c_int {
                                if *look as libc::c_int == '\n' as i32 {
                                    line_no += 1;
                                    line_no;
                                }
                                if *look as libc::c_int == '"' as i32 {
                                    count += 1;
                                    count;
                                }
                                look = look.offset(1);
                                look;
                            }
                            if count != 2 as libc::c_int {
                                yyerror(
                                    b"NUL character in string.\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            return 262 as libc::c_int;
                        }
                        44 => {
                            let mut src: *mut libc::c_char = 0 as *mut libc::c_char;
                            let mut dst: *mut libc::c_char = 0 as *mut libc::c_char;
                            let mut len: libc::c_int = 0;
                            len = strlen(yytext) as libc::c_int;
                            if *yytext.offset((len - 1 as libc::c_int) as isize)
                                as libc::c_int == '.' as i32
                            {
                                *yytext
                                    .offset(
                                        (len - 1 as libc::c_int) as isize,
                                    ) = 0 as libc::c_int as libc::c_char;
                            }
                            src = yytext;
                            dst = yytext;
                            while *src as libc::c_int == '0' as i32 {
                                src = src.offset(1);
                                src;
                            }
                            if *src as libc::c_int == 0 as libc::c_int {
                                src = src.offset(-1);
                                src;
                            }
                            while *src as libc::c_int != 0 as libc::c_int {
                                if *src as libc::c_int == '\\' as i32 {
                                    src = src.offset(1);
                                    src;
                                    src = src.offset(1);
                                    src;
                                    line_no += 1;
                                    line_no;
                                }
                                if *src as libc::c_int == ',' as i32 {
                                    src = src.offset(1);
                                    src;
                                    ct_warn(
                                        b"Commas in numbers\0" as *const u8 as *const libc::c_char,
                                    );
                                } else {
                                    let fresh1 = src;
                                    src = src.offset(1);
                                    let fresh2 = dst;
                                    dst = dst.offset(1);
                                    *fresh2 = *fresh1;
                                }
                            }
                            *dst = 0 as libc::c_int as libc::c_char;
                            yylval.s_value = strcopyof(yytext);
                            return 264 as libc::c_int;
                        }
                        45 => {
                            if (*yytext.offset(0 as libc::c_int as isize) as libc::c_int)
                                < ' ' as i32
                            {
                                yyerror(
                                    b"illegal character: ^%c\0" as *const u8
                                        as *const libc::c_char,
                                    *yytext.offset(0 as libc::c_int as isize) as libc::c_int
                                        + '@' as i32,
                                );
                            } else if *yytext.offset(0 as libc::c_int as isize)
                                as libc::c_int > '~' as i32
                            {
                                yyerror(
                                    b"illegal character: \\%03o\0" as *const u8
                                        as *const libc::c_char,
                                    *yytext.offset(0 as libc::c_int as isize) as libc::c_int,
                                );
                            } else {
                                yyerror(
                                    b"illegal character: %s\0" as *const u8
                                        as *const libc::c_char,
                                    yytext,
                                );
                            }
                            break '_yy_match;
                        }
                        46 => {
                            fwrite(
                                yytext as *const libc::c_void,
                                yyleng as libc::c_ulong,
                                1 as libc::c_int as libc::c_ulong,
                                yyout,
                            );
                            break '_yy_match;
                        }
                        48 | 49 => return 0 as libc::c_int,
                        47 => {
                            yy_amount_of_matched_text = yy_cp.offset_from(yytext)
                                as libc::c_long as libc::c_int - 1 as libc::c_int;
                            *yy_cp = yy_hold_char;
                            if (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                .yy_buffer_status == 0 as libc::c_int
                            {
                                yy_n_chars = (**yy_buffer_stack
                                    .offset(yy_buffer_stack_top as isize))
                                    .yy_n_chars;
                                let ref mut fresh3 = (**yy_buffer_stack
                                    .offset(yy_buffer_stack_top as isize))
                                    .yy_input_file;
                                *fresh3 = yyin;
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
                                    current_block = 13718575627189773797;
                                    break;
                                } else {
                                    current_block = 7238532450961708898;
                                    break;
                                }
                            } else {
                                match yy_get_next_buffer() {
                                    1 => {
                                        yy_did_buffer_switch_on_eof = 0 as libc::c_int;
                                        if yywrap() != 0 {
                                            yy_c_buf_p = yytext.offset(0 as libc::c_int as isize);
                                            yy_act = 47 as libc::c_int
                                                + (yy_start - 1 as libc::c_int) / 2 as libc::c_int
                                                + 1 as libc::c_int;
                                        } else {
                                            if yy_did_buffer_switch_on_eof == 0 {
                                                yyrestart(yyin);
                                            }
                                            break '_yy_match;
                                        }
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
                    7238532450961708898 => {
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
    number_to_move = yy_c_buf_p.offset_from(yytext) as libc::c_long as libc::c_int
        - 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < number_to_move {
        let fresh4 = source;
        source = source.offset(1);
        let fresh5 = dest;
        dest = dest.offset(1);
        *fresh5 = *fresh4;
        i += 1;
        i;
    }
    if (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_buffer_status
        == 2 as libc::c_int
    {
        yy_n_chars = 0 as libc::c_int;
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars = yy_n_chars;
    } else {
        let mut num_to_read: libc::c_int = ((**yy_buffer_stack
            .offset(yy_buffer_stack_top as isize))
            .yy_buf_size)
            .wrapping_sub(number_to_move as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
        while num_to_read <= 0 as libc::c_int {
            let mut b: YY_BUFFER_STATE = if !yy_buffer_stack.is_null() {
                *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
            } else {
                0 as YY_BUFFER_STATE
            };
            let mut yy_c_buf_p_offset: libc::c_int = yy_c_buf_p
                .offset_from((*b).yy_ch_buf) as libc::c_long as libc::c_int;
            if (*b).yy_is_our_buffer != 0 {
                let mut new_size: libc::c_int = ((*b).yy_buf_size)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as libc::c_int;
                if new_size <= 0 as libc::c_int {
                    (*b)
                        .yy_buf_size = ((*b).yy_buf_size as libc::c_ulong)
                        .wrapping_add(
                            ((*b).yy_buf_size)
                                .wrapping_div(8 as libc::c_int as libc::c_ulong),
                        ) as yy_size_t as yy_size_t;
                } else {
                    (*b)
                        .yy_buf_size = ((*b).yy_buf_size as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong) as yy_size_t
                        as yy_size_t;
                }
                (*b)
                    .yy_ch_buf = yyrealloc(
                    (*b).yy_ch_buf as *mut libc::c_void,
                    ((*b).yy_buf_size).wrapping_add(2 as libc::c_int as libc::c_ulong),
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
            num_to_read = ((**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                .yy_buf_size)
                .wrapping_sub(number_to_move as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
        }
        if num_to_read > 8192 as libc::c_int {
            num_to_read = 8192 as libc::c_int;
        }
        loop {
            yy_n_chars = read(
                fileno(yyin),
                &mut *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                    .yy_ch_buf)
                    .offset(number_to_move as isize) as *mut libc::c_char
                    as *mut libc::c_void,
                num_to_read as size_t,
            ) as libc::c_int;
            if !(yy_n_chars < 0 as libc::c_int) {
                break;
            }
            if *__errno_location() != 4 as libc::c_int {
                yy_fatal_error(
                    b"read() in flex scanner failed\0" as *const u8
                        as *const libc::c_char,
                );
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
    if (yy_n_chars + number_to_move) as yy_size_t
        > (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_buf_size
    {
        let mut new_size_0: yy_size_t = (yy_n_chars + number_to_move
            + (yy_n_chars >> 1 as libc::c_int)) as yy_size_t;
        let ref mut fresh6 = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
            .yy_ch_buf;
        *fresh6 = yyrealloc(
            (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf
                as *mut libc::c_void,
            new_size_0,
        ) as *mut libc::c_char;
        if ((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf).is_null()
        {
            yy_fatal_error(
                b"out of dynamic memory in yy_get_next_buffer()\0" as *const u8
                    as *const libc::c_char,
            );
        }
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
unsafe extern "C" fn yy_get_previous_state() -> yy_state_type {
    let mut yy_current_state: yy_state_type = 0;
    let mut yy_cp: *mut libc::c_char = 0 as *mut libc::c_char;
    yy_current_state = yy_start;
    yy_cp = yytext.offset(0 as libc::c_int as isize);
    while yy_cp < yy_c_buf_p {
        let mut yy_c: YY_CHAR = (if *yy_cp as libc::c_int != 0 {
            yy_ec[*yy_cp as libc::c_uchar as libc::c_uint as usize]
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
            if yy_current_state >= 316 as libc::c_int {
                yy_c = yy_meta[yy_c as libc::c_uint as usize] as YY_CHAR;
            }
        }
        yy_current_state = yy_nxt[(yy_base[yy_current_state as usize] as libc::c_uint)
            .wrapping_add(yy_c as libc::c_uint) as usize] as yy_state_type;
        yy_cp = yy_cp.offset(1);
        yy_cp;
    }
    return yy_current_state;
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
        if yy_current_state >= 316 as libc::c_int {
            yy_c = yy_meta[yy_c as libc::c_uint as usize] as YY_CHAR;
        }
    }
    yy_current_state = yy_nxt[(yy_base[yy_current_state as usize] as libc::c_uint)
        .wrapping_add(yy_c as libc::c_uint) as usize] as yy_state_type;
    yy_is_jam = (yy_current_state == 315 as libc::c_int) as libc::c_int;
    return if yy_is_jam != 0 { 0 as libc::c_int } else { yy_current_state };
}
unsafe extern "C" fn yyunput(mut c: libc::c_int, mut yy_bp: *mut libc::c_char) {
    let mut yy_cp: *mut libc::c_char = 0 as *mut libc::c_char;
    yy_cp = yy_c_buf_p;
    *yy_cp = yy_hold_char;
    if yy_cp
        < ((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
            .offset(2 as libc::c_int as isize)
    {
        let mut number_to_move: libc::c_int = yy_n_chars + 2 as libc::c_int;
        let mut dest: *mut libc::c_char = &mut *((**yy_buffer_stack
            .offset(yy_buffer_stack_top as isize))
            .yy_ch_buf)
            .offset(
                ((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_buf_size)
                    .wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
            ) as *mut libc::c_char;
        let mut source: *mut libc::c_char = &mut *((**yy_buffer_stack
            .offset(yy_buffer_stack_top as isize))
            .yy_ch_buf)
            .offset(number_to_move as isize) as *mut libc::c_char;
        while source > (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf
        {
            source = source.offset(-1);
            dest = dest.offset(-1);
            *dest = *source;
        }
        yy_cp = yy_cp
            .offset(dest.offset_from(source) as libc::c_long as libc::c_int as isize);
        yy_bp = yy_bp
            .offset(dest.offset_from(source) as libc::c_long as libc::c_int as isize);
        yy_n_chars = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_buf_size
            as libc::c_int;
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars = yy_n_chars;
        if yy_cp
            < ((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
                .offset(2 as libc::c_int as isize)
        {
            yy_fatal_error(
                b"flex scanner push-back overflow\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    yy_cp = yy_cp.offset(-1);
    *yy_cp = c as libc::c_char;
    yytext = yy_bp;
    yy_hold_char = *yy_cp;
    yy_c_buf_p = yy_cp;
}
unsafe extern "C" fn input() -> libc::c_int {
    let mut c: libc::c_int = 0;
    *yy_c_buf_p = yy_hold_char;
    if *yy_c_buf_p as libc::c_int == 0 as libc::c_int {
        if yy_c_buf_p
            < &mut *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
                .offset(yy_n_chars as isize) as *mut libc::c_char
        {
            *yy_c_buf_p = '\0' as i32 as libc::c_char;
        } else {
            let mut offset: libc::c_int = yy_c_buf_p.offset_from(yytext) as libc::c_long
                as libc::c_int;
            yy_c_buf_p = yy_c_buf_p.offset(1);
            yy_c_buf_p;
            let mut current_block_10: u64;
            match yy_get_next_buffer() {
                2 => {
                    yyrestart(yyin);
                    current_block_10 = 3303817703411010724;
                }
                1 => {
                    current_block_10 = 3303817703411010724;
                }
                0 => {
                    yy_c_buf_p = yytext.offset(offset as isize);
                    current_block_10 = 7746791466490516765;
                }
                _ => {
                    current_block_10 = 7746791466490516765;
                }
            }
            match current_block_10 {
                7746791466490516765 => {}
                _ => {
                    if yywrap() != 0 {
                        return -(1 as libc::c_int);
                    }
                    if yy_did_buffer_switch_on_eof == 0 {
                        yyrestart(yyin);
                    }
                    return input();
                }
            }
        }
    }
    c = *(yy_c_buf_p as *mut libc::c_uchar) as libc::c_int;
    *yy_c_buf_p = '\0' as i32 as libc::c_char;
    yy_c_buf_p = yy_c_buf_p.offset(1);
    yy_hold_char = *yy_c_buf_p;
    return c;
}
pub unsafe extern "C" fn yyrestart(mut input_file: *mut FILE) {
    if if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        yyensure_buffer_stack();
        let ref mut fresh7 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
        *fresh7 = yy_create_buffer(yyin, 16384 as libc::c_int);
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
        let ref mut fresh8 = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
            .yy_buf_pos;
        *fresh8 = yy_c_buf_p;
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars = yy_n_chars;
    }
    let ref mut fresh9 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    *fresh9 = new_buffer;
    yy_load_buffer_state();
    yy_did_buffer_switch_on_eof = 1 as libc::c_int;
}
unsafe extern "C" fn yy_load_buffer_state() {
    yy_n_chars = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars;
    yy_c_buf_p = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_buf_pos;
    yytext = yy_c_buf_p;
    yyin = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_input_file;
    yy_hold_char = *yy_c_buf_p;
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
    (*b).yy_buf_size = size as yy_size_t;
    (*b)
        .yy_ch_buf = yyalloc(
        ((*b).yy_buf_size).wrapping_add(2 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
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
        let ref mut fresh10 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
        *fresh10 = 0 as YY_BUFFER_STATE;
    }
    if (*b).yy_is_our_buffer != 0 {
        yyfree((*b).yy_ch_buf as *mut libc::c_void);
    }
    yyfree(b as *mut libc::c_void);
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
        let ref mut fresh11 = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
            .yy_buf_pos;
        *fresh11 = yy_c_buf_p;
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
    let ref mut fresh12 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    *fresh12 = new_buffer;
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
    let ref mut fresh13 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    *fresh13 = 0 as YY_BUFFER_STATE;
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
unsafe extern "C" fn yyensure_buffer_stack() {
    let mut num_to_alloc: libc::c_int = 0;
    if yy_buffer_stack.is_null() {
        num_to_alloc = 1 as libc::c_int;
        yy_buffer_stack = yyalloc(
            (num_to_alloc as libc::c_ulong)
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
            (num_to_alloc as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut yy_buffer_state>() as libc::c_ulong,
                ),
        );
        yy_buffer_stack_max = num_to_alloc as size_t;
        yy_buffer_stack_top = 0 as libc::c_int as size_t;
        return;
    }
    if yy_buffer_stack_top
        >= yy_buffer_stack_max.wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        let mut grow_size: libc::c_int = 8 as libc::c_int;
        num_to_alloc = yy_buffer_stack_max.wrapping_add(grow_size as libc::c_ulong)
            as libc::c_int;
        yy_buffer_stack = yyrealloc(
            yy_buffer_stack as *mut libc::c_void,
            (num_to_alloc as libc::c_ulong)
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
            (grow_size as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut yy_buffer_state>() as libc::c_ulong,
                ),
        );
        yy_buffer_stack_max = num_to_alloc as size_t;
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
    (*b).yy_buf_size = size.wrapping_sub(2 as libc::c_int as libc::c_ulong);
    (*b).yy_ch_buf = base;
    (*b).yy_buf_pos = (*b).yy_ch_buf;
    (*b).yy_is_our_buffer = 0 as libc::c_int;
    (*b).yy_input_file = 0 as *mut FILE;
    (*b).yy_n_chars = (*b).yy_buf_size as libc::c_int;
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
    let ref mut fresh14 = *buf.offset((_yybytes_len + 1 as libc::c_int) as isize);
    *fresh14 = 0 as libc::c_int as libc::c_char;
    *buf.offset(_yybytes_len as isize) = *fresh14;
    b = yy_scan_buffer(buf, n);
    if b.is_null() {
        yy_fatal_error(
            b"bad buffer in yy_scan_bytes()\0" as *const u8 as *const libc::c_char,
        );
    }
    (*b).yy_is_our_buffer = 1 as libc::c_int;
    return b;
}
unsafe extern "C" fn yy_fatal_error(mut msg: *const libc::c_char) -> ! {
    fprintf(stderr, b"%s\n\0" as *const u8 as *const libc::c_char, msg);
    exit(2 as libc::c_int);
}
pub unsafe extern "C" fn yyget_lineno() -> libc::c_int {
    return yylineno;
}
pub unsafe extern "C" fn yyget_in() -> *mut FILE {
    return yyin;
}
pub unsafe extern "C" fn yyget_out() -> *mut FILE {
    return yyout;
}
pub unsafe extern "C" fn yyget_leng() -> libc::c_int {
    return yyleng;
}
pub unsafe extern "C" fn yyget_text() -> *mut libc::c_char {
    return yytext;
}
pub unsafe extern "C" fn yyset_lineno(mut _line_number: libc::c_int) {
    yylineno = _line_number;
}
pub unsafe extern "C" fn yyset_in(mut _in_str: *mut FILE) {
    yyin = _in_str;
}
pub unsafe extern "C" fn yyset_out(mut _out_str: *mut FILE) {
    yyout = _out_str;
}
pub unsafe extern "C" fn yyget_debug() -> libc::c_int {
    return yy_flex_debug;
}
pub unsafe extern "C" fn yyset_debug(mut _bdebug: libc::c_int) {
    yy_flex_debug = _bdebug;
}
unsafe extern "C" fn yy_init_globals() -> libc::c_int {
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
        let ref mut fresh15 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
        *fresh15 = 0 as YY_BUFFER_STATE;
        yypop_buffer_state();
    }
    yyfree(yy_buffer_stack as *mut libc::c_void);
    yy_buffer_stack = 0 as *mut YY_BUFFER_STATE;
    yy_init_globals();
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn yyalloc(mut size: yy_size_t) -> *mut libc::c_void {
    return malloc(size);
}
pub unsafe extern "C" fn yyrealloc(
    mut ptr: *mut libc::c_void,
    mut size: yy_size_t,
) -> *mut libc::c_void {
    return realloc(ptr as *mut libc::c_char as *mut libc::c_void, size);
}
pub unsafe extern "C" fn yyfree(mut ptr: *mut libc::c_void) {
    free(ptr as *mut libc::c_char as *mut libc::c_void);
}
