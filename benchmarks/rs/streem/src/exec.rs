use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type strm_queue;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn strm_io_new(fd: libc::c_int, mode: libc::c_int) -> strm_value;
    fn strm_var_def(
        _: *mut strm_state,
        _: *const libc::c_char,
        _: strm_value,
    ) -> libc::c_int;
    fn strm_parse_args(
        _: *mut strm_stream,
        _: libc::c_int,
        _: *mut strm_value,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strm_stream_close(strm: *mut strm_stream);
    fn strm_io_stream(io: strm_value, mode: libc::c_int) -> *mut strm_stream;
    fn strm_ns_create(_: *mut strm_state, _: strm_string) -> *mut strm_state;
    fn strm_env_copy(_: *mut strm_state, _: *mut strm_state) -> libc::c_int;
    fn strm_var_get(
        _: *mut strm_state,
        _: strm_string,
        _: *mut strm_value,
    ) -> libc::c_int;
    fn strm_var_set(_: *mut strm_state, _: strm_string, _: strm_value) -> libc::c_int;
    fn strm_var_match(_: *mut strm_state, _: strm_string, _: strm_value) -> libc::c_int;
    fn strm_ns_get(_: strm_string) -> *mut strm_state;
    fn strm_value_ns(_: strm_value) -> *mut strm_state;
    fn strm_time_new(
        sec: libc::c_long,
        usec: libc::c_long,
        offset: libc::c_int,
    ) -> strm_value;
    fn strm_stream_connect(src: *mut strm_stream, dst: *mut strm_stream) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut strm_option_verbose: libc::c_int;
    fn strm_cfunc_value(_: strm_cfunc) -> strm_value;
    fn strm_bool_value(_: libc::c_int) -> strm_value;
    fn strm_int_value(_: strm_int) -> strm_value;
    fn strm_float_value(_: libc::c_double) -> strm_value;
    fn strm_nil_value() -> strm_value;
    fn strm_value_cfunc(_: strm_value) -> strm_cfunc;
    fn strm_value_int(_: strm_value) -> strm_int;
    fn strm_value_bool(_: strm_value) -> libc::c_int;
    fn strm_value_float(_: strm_value) -> libc::c_double;
    fn strm_value_eq(_: strm_value, _: strm_value) -> libc::c_int;
    fn strm_nil_p(_: strm_value) -> libc::c_int;
    fn strm_bool_p(_: strm_value) -> libc::c_int;
    fn strm_number_p(_: strm_value) -> libc::c_int;
    fn strm_cfunc_p(_: strm_value) -> libc::c_int;
    fn strm_array_p(_: strm_value) -> libc::c_int;
    fn strm_string_p(_: strm_value) -> libc::c_int;
    fn strm_int_p(_: strm_value) -> libc::c_int;
    fn strm_float_p(_: strm_value) -> libc::c_int;
    fn strm_ptr_tag_p(_: strm_value, _: strm_ptr_type) -> libc::c_int;
    fn strm_value_ptr(_: strm_value, _: strm_ptr_type) -> *mut libc::c_void;
    fn strm_ptr_value(_: *mut libc::c_void) -> strm_value;
    fn strm_str_new(_: *const libc::c_char, _: strm_int) -> strm_string;
    fn strm_strp_ptr(_: *mut strm_string) -> *const libc::c_char;
    fn strm_str_cstr(_: strm_string, buf: *mut libc::c_char) -> *const libc::c_char;
    fn strm_str_len(_: strm_string) -> strm_int;
    fn strm_str_intern(p: *const libc::c_char, len: strm_int) -> strm_string;
    fn strm_str_eq(a: strm_string, b: strm_string) -> libc::c_int;
    fn strm_to_str(v: strm_value) -> strm_string;
    fn strm_ary_new(_: *const strm_value, _: strm_int) -> strm_array;
    fn strm_ary_struct(_: strm_array) -> *mut strm_array_0;
    fn strm_stream_new(
        mode: strm_stream_mode,
        start: strm_callback,
        close: strm_callback,
        data: *mut libc::c_void,
    ) -> *mut strm_stream;
    fn strm_emit(strm: *mut strm_stream, data: strm_value, cb: strm_callback);
    fn exit(_: libc::c_int) -> !;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn strm_init(_: *mut strm_state);
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __intptr_t = libc::c_long;
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
pub type int32_t = __int32_t;
pub type intptr_t = __intptr_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type strm_value_tag = libc::c_ulong;
pub const STRM_TAG_FOREIGN: strm_value_tag = 18446462598732840960;
pub const STRM_TAG_PTR: strm_value_tag = 18445899648779419648;
pub const STRM_TAG_CFUNC: strm_value_tag = 18445336698825998336;
pub const STRM_TAG_STRING_F: strm_value_tag = 18445055223849287680;
pub const STRM_TAG_STRING_O: strm_value_tag = 18444773748872577024;
pub const STRM_TAG_STRING_6: strm_value_tag = 18444492273895866368;
pub const STRM_TAG_STRING_I: strm_value_tag = 18444210798919155712;
pub const STRM_TAG_STRUCT: strm_value_tag = 18443647848965734400;
pub const STRM_TAG_ARRAY: strm_value_tag = 18443366373989023744;
pub const STRM_TAG_LIST: strm_value_tag = 18443084899012313088;
pub const STRM_TAG_INT: strm_value_tag = 18442803424035602432;
pub const STRM_TAG_BOOL: strm_value_tag = 18442521949058891776;
pub const STRM_TAG_NAN: strm_value_tag = 18442240474082181120;
pub type strm_value = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_state {
    pub env: *mut libc::c_void,
    pub prev: *mut strm_state,
    pub flags: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_stream {
    pub type_0: strm_ptr_type,
    pub flags: libc::c_uint,
    pub mode: strm_stream_mode,
    pub start_func: strm_callback,
    pub close_func: strm_callback,
    pub data: *mut libc::c_void,
    pub dst: *mut strm_stream,
    pub rest: *mut *mut strm_stream,
    pub rsize: size_t,
    pub rcapa: size_t,
    pub exc: *mut node_error,
    pub refcnt: strm_int,
    pub queue: *mut strm_queue,
    pub excl: strm_int,
}
pub type strm_int = int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_error {
    pub type_0: libc::c_int,
    pub arg: strm_value,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
}
pub type strm_callback = Option::<
    unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
>;
pub type strm_stream_mode = libc::c_uint;
pub const strm_killed: strm_stream_mode = 4;
pub const strm_dying: strm_stream_mode = 3;
pub const strm_consumer: strm_stream_mode = 2;
pub const strm_filter: strm_stream_mode = 1;
pub const strm_producer: strm_stream_mode = 0;
pub type strm_ptr_type = libc::c_uint;
pub const STRM_PTR_AUX: strm_ptr_type = 4;
pub const STRM_PTR_IO: strm_ptr_type = 3;
pub const STRM_PTR_GENFUNC: strm_ptr_type = 2;
pub const STRM_PTR_LAMBDA: strm_ptr_type = 1;
pub const STRM_PTR_STREAM: strm_ptr_type = 0;
pub type strm_cfunc = Option::<
    unsafe extern "C" fn(
        *mut strm_stream,
        libc::c_int,
        *mut strm_value,
        *mut strm_value,
    ) -> libc::c_int,
>;
pub type strm_string = uint64_t;
pub type strm_array = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_array_0 {
    pub len: strm_int,
    pub ptr: *mut strm_value,
    pub headers: strm_array,
    pub ns: *mut strm_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_lambda {
    pub type_0: strm_ptr_type,
    pub body: *mut node_lambda,
    pub state: *mut strm_state,
}
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
pub type node_string = *mut node_string_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_string_0 {
    pub len: strm_int,
    pub buf: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_str {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub value: node_string,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_bool {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub value: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_time {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub sec: libc::c_long,
    pub usec: libc::c_long,
    pub utc_offset: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_float {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub value: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_int {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub value: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_nodes {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub len: libc::c_int,
    pub max: libc::c_int,
    pub data: *mut *mut node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_return {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub rv: *mut node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_genfunc {
    pub type_0: strm_ptr_type,
    pub state: *mut strm_state,
    pub id: strm_string,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_genfunc {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub id: node_string,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_plambda {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub pat: *mut node,
    pub cond: *mut node,
    pub body: *mut node,
    pub next: *mut node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_pair {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub key: node_string,
    pub value: *mut node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_psplat {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub head: *mut node,
    pub mid: *mut node,
    pub tail: *mut node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_ns {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub name: node_string,
    pub body: *mut node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_ident {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub name: node_string,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_args {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub len: libc::c_int,
    pub max: libc::c_int,
    pub data: *mut node_string,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_fcall {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub func: *mut node,
    pub args: *mut node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_call {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub ident: node_string,
    pub args: *mut node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_op {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub op: node_string,
    pub lhs: *mut node,
    pub rhs: *mut node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_if {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub cond: *mut node,
    pub then: *mut node,
    pub opt_else: *mut node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_array {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub len: libc::c_int,
    pub max: libc::c_int,
    pub data: *mut *mut node,
    pub headers: *mut node_string,
    pub ns: node_string,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_splat {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub node: *mut node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_let {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub lhs: node_string,
    pub rhs: *mut node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_emit {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub emit: *mut node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_import {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub name: node_string,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct array_data {
    pub n: libc::c_int,
    pub arr: strm_array,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parser_state {
    pub nerr: libc::c_int,
    pub lval: *mut libc::c_void,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub tline: libc::c_int,
}
unsafe extern "C" fn strm_clear_exc(mut strm: *mut strm_stream) {
    if !((*strm).exc).is_null() {
        free((*strm).exc as *mut libc::c_void);
    }
    (*strm).exc = 0 as *mut node_error;
}
unsafe extern "C" fn strm_set_exc(
    mut strm: *mut strm_stream,
    mut type_0: libc::c_int,
    mut arg: strm_value,
) -> *mut node_error {
    let mut exc: *mut node_error = malloc(
        ::std::mem::size_of::<node_error>() as libc::c_ulong,
    ) as *mut node_error;
    if exc.is_null() {
        return 0 as *mut node_error;
    }
    (*exc).type_0 = type_0;
    (*exc).arg = arg;
    (*exc).fname = 0 as *const libc::c_char;
    (*exc).lineno = 0 as libc::c_int;
    strm_clear_exc(strm);
    (*strm).exc = exc;
    return exc;
}
unsafe extern "C" fn node_to_sym(mut s: node_string) -> strm_string {
    return strm_str_intern(((*s).buf).as_mut_ptr(), (*s).len);
}
unsafe extern "C" fn node_to_str(mut s: node_string) -> strm_string {
    return strm_str_new(((*s).buf).as_mut_ptr(), (*s).len);
}
unsafe extern "C" fn exec_eq(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut x: strm_value = 0;
    let mut y: strm_value = 0;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"vv\0" as *const u8 as *const libc::c_char,
        &mut x as *mut strm_value,
        &mut y as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    *ret = strm_bool_value(strm_value_eq(x, y));
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_neq(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut x: strm_value = 0;
    let mut y: strm_value = 0;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"vv\0" as *const u8 as *const libc::c_char,
        &mut x as *mut strm_value,
        &mut y as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    *ret = strm_bool_value((strm_value_eq(x, y) == 0) as libc::c_int);
    return 0 as libc::c_int;
}
unsafe extern "C" fn cfunc_closer(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn strm_connect(
    mut strm: *mut strm_stream,
    mut src: strm_value,
    mut dst: strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    if strm_ptr_tag_p(src, STRM_PTR_IO) != 0 {
        src = strm_ptr_value(strm_io_stream(src, 1 as libc::c_int) as *mut libc::c_void);
    } else if strm_ptr_tag_p(src, STRM_PTR_LAMBDA) != 0 {
        let mut lmbd: *mut strm_lambda = strm_value_ptr(src, STRM_PTR_LAMBDA)
            as *mut strm_lambda;
        src = strm_ptr_value(
            strm_stream_new(
                strm_filter,
                Some(
                    blk_exec
                        as unsafe extern "C" fn(
                            *mut strm_stream,
                            strm_value,
                        ) -> libc::c_int,
                ),
                None,
                lmbd as *mut libc::c_void,
            ) as *mut libc::c_void,
        );
    } else if strm_array_p(src) != 0 {
        let mut arrd: *mut array_data = malloc(
            ::std::mem::size_of::<array_data>() as libc::c_ulong,
        ) as *mut array_data;
        (*arrd).arr = src;
        (*arrd).n = 0 as libc::c_int;
        src = strm_ptr_value(
            strm_stream_new(
                strm_producer,
                Some(
                    arr_exec
                        as unsafe extern "C" fn(
                            *mut strm_stream,
                            strm_value,
                        ) -> libc::c_int,
                ),
                None,
                arrd as *mut libc::c_void,
            ) as *mut libc::c_void,
        );
    }
    if strm_ptr_tag_p(dst, STRM_PTR_IO) != 0 {
        dst = strm_ptr_value(strm_io_stream(dst, 2 as libc::c_int) as *mut libc::c_void);
    } else if strm_ptr_tag_p(dst, STRM_PTR_LAMBDA) != 0 {
        let mut lmbd_0: *mut strm_lambda = strm_value_ptr(dst, STRM_PTR_LAMBDA)
            as *mut strm_lambda;
        dst = strm_ptr_value(
            strm_stream_new(
                strm_filter,
                Some(
                    blk_exec
                        as unsafe extern "C" fn(
                            *mut strm_stream,
                            strm_value,
                        ) -> libc::c_int,
                ),
                None,
                lmbd_0 as *mut libc::c_void,
            ) as *mut libc::c_void,
        );
    } else if strm_cfunc_p(dst) != 0 {
        let mut func: strm_cfunc = strm_value_cfunc(dst);
        dst = strm_ptr_value(
            strm_stream_new(
                strm_filter,
                Some(
                    cfunc_exec
                        as unsafe extern "C" fn(
                            *mut strm_stream,
                            strm_value,
                        ) -> libc::c_int,
                ),
                Some(
                    cfunc_closer
                        as unsafe extern "C" fn(
                            *mut strm_stream,
                            strm_value,
                        ) -> libc::c_int,
                ),
                ::std::mem::transmute::<strm_cfunc, *mut libc::c_void>(func),
            ) as *mut libc::c_void,
        );
    }
    if strm_ptr_tag_p(src, STRM_PTR_STREAM) != 0
        && strm_ptr_tag_p(dst, STRM_PTR_STREAM) != 0
    {
        let mut lstrm: *mut strm_stream = strm_value_ptr(src, STRM_PTR_STREAM)
            as *mut strm_stream;
        let mut rstrm: *mut strm_stream = strm_value_ptr(dst, STRM_PTR_STREAM)
            as *mut strm_stream;
        if lstrm.is_null() || rstrm.is_null()
            || (*lstrm).mode as libc::c_uint
                == strm_consumer as libc::c_int as libc::c_uint
            || (*rstrm).mode as libc::c_uint
                == strm_producer as libc::c_int as libc::c_uint
        {
            strm_raise(strm, b"stream error\0" as *const u8 as *const libc::c_char);
            return 1 as libc::c_int;
        }
        strm_stream_connect(
            strm_value_ptr(src, STRM_PTR_STREAM) as *mut strm_stream,
            strm_value_ptr(dst, STRM_PTR_STREAM) as *mut strm_stream,
        );
        *ret = dst;
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn exec_bar(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut x: strm_value = 0;
    let mut y: strm_value = 0;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"vv\0" as *const u8 as *const libc::c_char,
        &mut x as *mut strm_value,
        &mut y as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    return strm_connect(strm, x, y, ret);
}
unsafe extern "C" fn ary_get(
    mut strm: *mut strm_stream,
    mut ary: strm_value,
    mut argc: libc::c_int,
    mut argv: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut a: *mut strm_array_0 = 0 as *mut strm_array_0;
    let mut idx: strm_value = 0;
    if argc != 1 as libc::c_int {
        strm_raise(
            strm,
            b"wrong number of arguments\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    a = strm_ary_struct(ary);
    idx = *argv.offset(0 as libc::c_int as isize);
    if strm_number_p(idx) != 0 {
        let mut i: strm_int = strm_value_int(idx);
        if i >= (*a).len {
            return 1 as libc::c_int;
        }
        *ret = *((*a).ptr).offset(i as isize);
        return 0 as libc::c_int;
    }
    if strm_string_p(idx) != 0 {
        if (*a).headers != 0 {
            let mut i_0: strm_int = 0;
            let mut len: strm_int = (*a).len;
            i_0 = 0 as libc::c_int;
            while i_0 < len {
                if strm_str_eq(
                    idx,
                    *((*strm_ary_struct((*a).headers)).ptr).offset(i_0 as isize),
                ) != 0
                {
                    *ret = *((*a).ptr).offset(i_0 as isize);
                    return 0 as libc::c_int;
                }
                i_0 += 1;
                i_0;
            }
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn pattern_placeholder_p(mut name: node_string) -> libc::c_int {
    if (*name).len == 1 as libc::c_int
        && *((*name).buf).as_mut_ptr().offset(0 as libc::c_int as isize) as libc::c_int
            == '_' as i32
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn pmatch_struct(
    mut strm: *mut strm_stream,
    mut state: *mut strm_state,
    mut pat: *mut node,
    mut val: strm_value,
    mut tbl: *mut uint64_t,
    mut len: *mut strm_int,
) -> libc::c_int {
    let mut pstr: *mut node_nodes = pat as *mut node_nodes;
    let mut ary: strm_array = val;
    let mut a: *mut strm_array_0 = strm_ary_struct(ary);
    let mut headers: *mut strm_value = 0 as *mut strm_value;
    if (*a).headers == 0 {
        return 1 as libc::c_int;
    }
    if (*pstr).len > (*a).len {
        return 1 as libc::c_int;
    }
    headers = (*strm_ary_struct((*a).headers)).ptr;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*pstr).len {
        let mut npair: *mut node_pair = *((*pstr).data).offset(i as isize)
            as *mut node_pair;
        let mut key: strm_string = 0;
        if (*npair).type_0 as libc::c_uint == NODE_PAIR as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"npair->type == NODE_PAIR\0" as *const u8 as *const libc::c_char,
                b"exec.c\0" as *const u8 as *const libc::c_char,
                203 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 91],
                    &[libc::c_char; 91],
                >(
                    b"int pmatch_struct(strm_stream *, strm_state *, node *, strm_value, uint64_t *, strm_int *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_4728: {
            if (*npair).type_0 as libc::c_uint
                == NODE_PAIR as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"npair->type == NODE_PAIR\0" as *const u8 as *const libc::c_char,
                    b"exec.c\0" as *const u8 as *const libc::c_char,
                    203 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 91],
                        &[libc::c_char; 91],
                    >(
                        b"int pmatch_struct(strm_stream *, strm_state *, node *, strm_value, uint64_t *, strm_int *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        key = node_to_sym((*npair).key);
        let mut j: libc::c_int = 0 as libc::c_int;
        while i < (*a).len {
            if *headers.offset(j as isize) == key {
                if pmatch(strm, state, (*npair).value, *((*a).ptr).offset(j as isize))
                    == 1 as libc::c_int
                {
                    return 1 as libc::c_int;
                }
                if !tbl.is_null() {
                    let mut n: uint64_t = ((1 as libc::c_int) << j % 64 as libc::c_int)
                        as uint64_t;
                    if *tbl.offset((j / 64 as libc::c_int) as isize) & n != 0 {
                        *len -= 1;
                        *len;
                    }
                    let ref mut fresh0 = *tbl.offset((j / 64 as libc::c_int) as isize);
                    *fresh0 |= n;
                }
                break;
            } else {
                j += 1;
                j;
            }
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn pmatch(
    mut strm: *mut strm_stream,
    mut state: *mut strm_state,
    mut pat: *mut node,
    mut val: strm_value,
) -> libc::c_int {
    match (*pat).type_0 as libc::c_uint {
        15 => {
            let mut ni: *mut node_ident = pat as *mut node_ident;
            if pattern_placeholder_p((*ni).name) != 0 {
                return 0 as libc::c_int;
            }
            return strm_var_match(state, node_to_sym((*ni).name), val);
        }
        3 => {
            if strm_string_p(val) != 0 {
                if strm_str_eq(val, node_to_str((*(pat as *mut node_str)).value)) != 0 {
                    return 0 as libc::c_int;
                }
            }
        }
        0 => {
            let mut n: strm_int = (*(pat as *mut node_int)).value;
            if strm_int_p(val) != 0 {
                if n == strm_value_int(val) {
                    return 0 as libc::c_int;
                }
                return 1 as libc::c_int;
            }
            if strm_float_p(val) != 0 {
                if n as libc::c_double == strm_value_float(val) {
                    return 0 as libc::c_int;
                }
                return 1 as libc::c_int;
            }
        }
        4 => {
            if strm_nil_p(val) != 0 {
                return 0 as libc::c_int;
            }
            return 1 as libc::c_int;
        }
        5 => {
            if strm_value_bool(val) == (*(pat as *mut node_bool)).value {
                return 0 as libc::c_int;
            }
            return 1 as libc::c_int;
        }
        1 => {
            if strm_number_p(val) != 0 {
                if (*(pat as *mut node_float)).value == strm_value_float(val) {
                    return 0 as libc::c_int;
                }
                return 1 as libc::c_int;
            }
        }
        29 => {
            let mut ns: *mut node_ns = pat as *mut node_ns;
            let mut s1: *mut strm_state = strm_ns_get(node_to_sym((*ns).name));
            let mut s2: *mut strm_state = strm_value_ns(val);
            if s1 != s2 {
                return 1 as libc::c_int;
            }
            return pmatch(strm, state, (*ns).body, val);
        }
        11 => {
            if strm_array_p(val) != 0 {
                let mut ary: strm_array = val;
                return pattern_match(
                    strm,
                    state,
                    pat,
                    (*strm_ary_struct(ary)).len,
                    (*strm_ary_struct(ary)).ptr,
                );
            }
        }
        13 => {
            if strm_array_p(val) != 0 {
                let mut ary_0: strm_array = val;
                let mut psp: *mut node_psplat = pat as *mut node_psplat;
                if !((*psp).head).is_null()
                    && (*(*psp).head).type_0 as libc::c_uint
                        == NODE_PSTRUCT as libc::c_int as libc::c_uint
                {
                    let mut pstr: *mut node_nodes = (*psp).head as *mut node_nodes;
                    let mut len: strm_int = (*pstr).len;
                    let mut buf: uint64_t = 0 as libc::c_int as uint64_t;
                    let mut tbl: *mut uint64_t = &mut buf;
                    if len > 64 as libc::c_int {
                        tbl = malloc(
                            (len / 64 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
                        ) as *mut uint64_t;
                        memset(
                            tbl as *mut libc::c_void,
                            0 as libc::c_int,
                            (len / 64 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
                        );
                    }
                    if pmatch_struct(strm, state, (*psp).head, val, tbl, &mut len)
                        == 1 as libc::c_int
                    {
                        return 1 as libc::c_int;
                    }
                    if ((*psp).tail).is_null() {} else {
                        __assert_fail(
                            b"psp->tail == NULL\0" as *const u8 as *const libc::c_char,
                            b"exec.c\0" as *const u8 as *const libc::c_char,
                            302 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 60],
                                &[libc::c_char; 60],
                            >(
                                b"int pmatch(strm_stream *, strm_state *, node *, strm_value)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_5059: {
                        if ((*psp).tail).is_null() {} else {
                            __assert_fail(
                                b"psp->tail == NULL\0" as *const u8 as *const libc::c_char,
                                b"exec.c\0" as *const u8 as *const libc::c_char,
                                302 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 60],
                                    &[libc::c_char; 60],
                                >(
                                    b"int pmatch(strm_stream *, strm_state *, node *, strm_value)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    let mut a: *mut strm_array_0 = strm_ary_struct(ary_0);
                    let mut hdr: *mut strm_value = (*strm_ary_struct((*a).headers)).ptr;
                    let mut splat: strm_array = strm_ary_new(
                        0 as *const strm_value,
                        (*a).len - len,
                    );
                    let mut nhdr: strm_array = strm_ary_new(
                        0 as *const strm_value,
                        (*a).len - len,
                    );
                    let mut n_0: libc::c_int = 0 as libc::c_int;
                    let mut i: libc::c_int = 0 as libc::c_int;
                    while i < (*a).len {
                        if !(*tbl.offset((i / 64 as libc::c_int) as isize)
                            & ((1 as libc::c_int) << i % 64 as libc::c_int)
                                as libc::c_ulong != 0)
                        {
                            *((*strm_ary_struct(nhdr)).ptr)
                                .offset(n_0 as isize) = *hdr.offset(i as isize);
                            *((*strm_ary_struct(splat)).ptr)
                                .offset(n_0 as isize) = *((*a).ptr).offset(i as isize);
                            n_0 += 1;
                            n_0;
                        }
                        i += 1;
                        i;
                    }
                    (*strm_ary_struct(splat)).headers = nhdr;
                    return pmatch(strm, state, (*psp).mid, splat);
                }
                return pattern_match(
                    strm,
                    state,
                    pat,
                    (*strm_ary_struct(ary_0)).len,
                    (*strm_ary_struct(ary_0)).ptr,
                );
            }
        }
        12 => {
            if strm_array_p(val) == 0 {
                return 1 as libc::c_int;
            }
            return pmatch_struct(
                strm,
                state,
                pat,
                val,
                0 as *mut uint64_t,
                0 as *mut strm_int,
            );
        }
        _ => {}
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn pattern_match(
    mut strm: *mut strm_stream,
    mut state: *mut strm_state,
    mut npat: *mut node,
    mut argc: libc::c_int,
    mut argv: *mut strm_value,
) -> libc::c_int {
    let mut pat: *mut node_nodes = npat as *mut node_nodes;
    let mut i: libc::c_int = 0;
    if pat.is_null() {
        return 0 as libc::c_int;
    }
    if (*npat).type_0 as libc::c_uint == NODE_PSPLAT as libc::c_int as libc::c_uint {
        let mut psp: *mut node_psplat = pat as *mut node_psplat;
        let mut head: *mut node_nodes = (*psp).head as *mut node_nodes;
        let mut tail: *mut node_nodes = (*psp).tail as *mut node_nodes;
        let mut rest: *mut node = (*psp).mid;
        let mut hlen: strm_int = if !head.is_null() {
            (*head).len
        } else {
            0 as libc::c_int
        };
        if argc < hlen {
            return 1 as libc::c_int;
        }
        if !head.is_null() {
            if pattern_match(strm, state, head as *mut node, hlen, argv)
                == 1 as libc::c_int
            {
                return 1 as libc::c_int;
            }
        }
        if tail.is_null() {
            if pmatch(
                strm,
                state,
                rest,
                strm_ary_new(argv.offset(hlen as isize), argc - hlen),
            ) == 1 as libc::c_int
            {
                return 1 as libc::c_int;
            }
        } else {
            if argc < hlen + (*tail).len {
                return 1 as libc::c_int;
            }
            if pattern_match(
                strm,
                state,
                rest,
                argc - hlen - (*tail).len,
                argv.offset(hlen as isize),
            ) == 1 as libc::c_int
            {
                return 1 as libc::c_int;
            }
            if pattern_match(
                strm,
                state,
                tail as *mut node,
                (*tail).len,
                argv.offset(argc as isize).offset(-((*tail).len as isize)),
            ) == 1 as libc::c_int
            {
                return 1 as libc::c_int;
            }
        }
        return 0 as libc::c_int;
    }
    if (*pat).len != argc {
        return 1 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < (*pat).len {
        if pmatch(
            strm,
            state,
            *((*pat).data).offset(i as isize),
            *argv.offset(i as isize),
        ) == 1 as libc::c_int
        {
            return 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn lambda_call(
    mut strm: *mut strm_stream,
    mut func: strm_value,
    mut argc: libc::c_int,
    mut argv: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut current_block: u64;
    let mut lambda: *mut strm_lambda = strm_value_ptr(func, STRM_PTR_LAMBDA)
        as *mut strm_lambda;
    let mut c: strm_state = {
        let mut init = strm_state {
            env: 0 as *mut libc::c_void,
            prev: 0 as *mut strm_state,
            flags: 0,
        };
        init
    };
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut exc: *mut node_error = 0 as *mut node_error;
    c.prev = (*lambda).state;
    if (*(*lambda).body).type_0 as libc::c_uint
        == NODE_LAMBDA as libc::c_int as libc::c_uint
    {
        let mut nlmbd: *mut node_lambda = (*lambda).body as *mut node_lambda;
        let mut args: *mut node_args = (*nlmbd).args as *mut node_args;
        if args.is_null() {
            if argc > 0 as libc::c_int {
                current_block = 827690812120339081;
            } else {
                current_block = 8515828400728868193;
            }
        } else if (*args).len != argc {
            current_block = 827690812120339081;
        } else {
            current_block = 8515828400728868193;
        }
        match current_block {
            8515828400728868193 => {
                i = 0 as libc::c_int;
                while i < argc {
                    n = strm_var_set(
                        &mut c,
                        node_to_sym(*((*args).data).offset(i as isize)),
                        *argv.offset(i as isize),
                    );
                    if n != 0 {
                        return n;
                    }
                    i += 1;
                    i;
                }
                n = exec_expr(strm, &mut c, (*nlmbd).body, ret);
                current_block = 5494826135382683477;
            }
            _ => {
                strm_raise(
                    strm,
                    b"wrong number of arguments\0" as *const u8 as *const libc::c_char,
                );
                current_block = 16266154392392223561;
            }
        }
    } else if (*(*lambda).body).type_0 as libc::c_uint
        == NODE_PLAMBDA as libc::c_int as libc::c_uint
    {
        let mut plmbd: *mut node_plambda = (*lambda).body as *mut node_plambda;
        let mut nexec: libc::c_int = 0 as libc::c_int;
        while !plmbd.is_null() {
            if pattern_match(strm, &mut c, (*plmbd).pat, argc, argv) == 0 as libc::c_int
            {
                let mut cond: strm_value = 0;
                if !((*plmbd).cond).is_null() {
                    n = exec_expr(strm, &mut c, (*plmbd).cond, &mut cond);
                    if n == 0 as libc::c_int && strm_value_bool(cond) != 0 {
                        nexec += 1;
                        nexec;
                        n = exec_expr(strm, &mut c, (*plmbd).body, ret);
                        break;
                    }
                } else {
                    nexec += 1;
                    nexec;
                    n = exec_expr(strm, &mut c, (*plmbd).body, ret);
                    break;
                }
            }
            c.env = 0 as *mut libc::c_void;
            plmbd = (*plmbd).next as *mut node_plambda;
        }
        if nexec == 0 as libc::c_int {
            strm_raise(strm, b"match failure\0" as *const u8 as *const libc::c_char);
            current_block = 16266154392392223561;
        } else {
            current_block = 5494826135382683477;
        }
    } else {
        return 1 as libc::c_int
    }
    match current_block {
        16266154392392223561 => {
            if !strm.is_null() && !((*strm).exc).is_null() {
                (*(*strm).exc).fname = (*(*lambda).body).fname;
                (*(*strm).exc).lineno = (*(*lambda).body).lineno;
            }
            return 1 as libc::c_int;
        }
        _ => {
            if n == 1 as libc::c_int && !strm.is_null() {
                exc = (*strm).exc;
                if !exc.is_null() && (*exc).type_0 == 1 as libc::c_int {
                    *ret = (*exc).arg;
                    return 0 as libc::c_int;
                }
            }
            return n;
        }
    };
}
unsafe extern "C" fn genfunc_new(
    mut state: *mut strm_state,
    mut id: strm_string,
) -> *mut strm_genfunc {
    let mut gf: *mut strm_genfunc = malloc(
        ::std::mem::size_of::<strm_genfunc>() as libc::c_ulong,
    ) as *mut strm_genfunc;
    if gf.is_null() {
        return 0 as *mut strm_genfunc;
    }
    (*gf).type_0 = STRM_PTR_GENFUNC;
    (*gf).state = state;
    (*gf).id = id;
    return gf;
}
pub unsafe extern "C" fn strm_funcall(
    mut strm: *mut strm_stream,
    mut func: strm_value,
    mut argc: libc::c_int,
    mut argv: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    match func
        & ((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t) << 48 as libc::c_int
    {
        18445336698825998336 => {
            return (strm_value_cfunc(func)).unwrap()(strm, argc, argv, ret);
        }
        18443366373989023744 => return ary_get(strm, func, argc, argv, ret),
        18445899648779419648 => {
            if strm_ptr_tag_p(func, STRM_PTR_GENFUNC) != 0 {
                let mut gf: *mut strm_genfunc = (func
                    & !(((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t)
                        << 48 as libc::c_int)) as intptr_t as *mut libc::c_void
                    as *mut strm_genfunc;
                return exec_call(strm, (*gf).state, (*gf).id, argc, argv, ret);
            } else if strm_ptr_tag_p(func, STRM_PTR_LAMBDA) != 0 {
                return lambda_call(strm, func, argc, argv, ret)
            }
        }
        _ => {}
    }
    strm_raise(strm, b"not a function\0" as *const u8 as *const libc::c_char);
    return 1 as libc::c_int;
}
unsafe extern "C" fn exec_call(
    mut strm: *mut strm_stream,
    mut state: *mut strm_state,
    mut name: strm_string,
    mut argc: libc::c_int,
    mut argv: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut n: libc::c_int = 1 as libc::c_int;
    let mut m: strm_value = 0;
    if argc > 0 as libc::c_int {
        let mut ns: *mut strm_state = strm_value_ns(
            *argv.offset(0 as libc::c_int as isize),
        );
        if !ns.is_null() {
            n = strm_var_get(ns, name, &mut m);
            if n == 1 as libc::c_int {
                if argc > 0 as libc::c_int
                    && strm_array_p(*argv.offset(0 as libc::c_int as isize)) != 0
                {
                    m = name;
                    n = ary_get(
                        strm,
                        *argv.offset(0 as libc::c_int as isize),
                        1 as libc::c_int,
                        &mut m,
                        ret,
                    );
                    if n == 0 as libc::c_int && argc == 1 as libc::c_int {
                        return 0 as libc::c_int;
                    }
                    m = *ret;
                }
            }
        }
    }
    if n == 1 as libc::c_int {
        n = strm_var_get(state, name, &mut m);
    }
    if n == 0 as libc::c_int {
        return strm_funcall(strm, m, argc, argv, ret);
    }
    strm_raise(strm, b"function not found\0" as *const u8 as *const libc::c_char);
    return 1 as libc::c_int;
}
unsafe extern "C" fn ary_headers(
    mut headers: *mut node_string,
    mut len: strm_int,
) -> strm_array {
    let mut ary: strm_array = strm_ary_new(0 as *const strm_value, len);
    let mut p: *mut strm_value = (*strm_ary_struct(ary)).ptr;
    let mut i: strm_int = 0;
    i = 0 as libc::c_int;
    while i < len {
        *p.offset(i as isize) = node_to_sym(*headers.offset(i as isize));
        i += 1;
        i;
    }
    return ary;
}
unsafe extern "C" fn exec_expr(
    mut strm: *mut strm_stream,
    mut state: *mut strm_state,
    mut np: *mut node,
    mut val: *mut strm_value,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    if np.is_null() {
        return 1 as libc::c_int;
    }
    match (*np).type_0 as libc::c_uint {
        29 => {
            let mut ns: *mut node_ns = np as *mut node_ns;
            let mut name: strm_string = node_to_sym((*ns).name);
            let mut s: *mut strm_state = strm_ns_create(state, name);
            if s.is_null() {
                if !(strm_ns_get(name)).is_null() {
                    strm_raise(
                        strm,
                        b"namespace already exists\0" as *const u8 as *const libc::c_char,
                    );
                } else {
                    strm_raise(
                        strm,
                        b"failed to create namespace\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                return 1 as libc::c_int;
            }
            (*s).flags |= 1 as libc::c_int as libc::c_uint;
            if !((*ns).body).is_null() {
                return exec_expr(strm, s, (*ns).body, val);
            }
            return 0 as libc::c_int;
        }
        30 => {
            let mut ns_0: *mut node_import = np as *mut node_import;
            let mut s_0: *mut strm_state = strm_ns_get(node_to_sym((*ns_0).name));
            if s_0.is_null() {
                strm_raise(
                    strm,
                    b"no such namespace\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            n = strm_env_copy(state, s_0);
            if n != 0 {
                strm_raise(
                    strm,
                    b"failed to import\0" as *const u8 as *const libc::c_char,
                );
                return n;
            }
            return 0 as libc::c_int;
        }
        19 => {
            strm_set_exc(strm, 2 as libc::c_int, strm_nil_value());
            return 1 as libc::c_int;
        }
        18 => {
            let mut i: libc::c_int = 0;
            let mut n_0: libc::c_int = 0;
            let mut v0: *mut node_array = 0 as *mut node_array;
            v0 = (*(np as *mut node_emit)).emit as *mut node_array;
            if v0.is_null() {
                strm_emit(strm, strm_nil_value(), None);
            } else {
                i = 0 as libc::c_int;
                while i < (*v0).len {
                    n_0 = exec_expr(strm, state, *((*v0).data).offset(i as isize), val);
                    if n_0 != 0 {
                        return n_0;
                    }
                    strm_emit(strm, *val, None);
                    i += 1;
                    i;
                }
            }
            return 0 as libc::c_int;
        }
        16 => {
            let mut nlet: *mut node_let = np as *mut node_let;
            n = exec_expr(strm, state, (*nlet).rhs, val);
            if n != 0 {
                strm_raise(
                    strm,
                    b"failed to assign\0" as *const u8 as *const libc::c_char,
                );
                return n;
            }
            return strm_var_set(state, node_to_sym((*nlet).lhs), *val);
        }
        27 => {
            let mut v0_0: *mut node_array = np as *mut node_array;
            let mut arr: strm_array = strm_ary_new(0 as *const strm_value, (*v0_0).len);
            let mut ptr: *mut strm_value = (*strm_ary_struct(arr)).ptr;
            let mut splat: libc::c_int = 0 as libc::c_int;
            let mut i_0: libc::c_int = 0 as libc::c_int;
            while i_0 < (*v0_0).len {
                if (**((*v0_0).data).offset(i_0 as isize)).type_0 as libc::c_uint
                    == NODE_SPLAT as libc::c_int as libc::c_uint
                {
                    let mut s_1: *mut node_splat = *((*v0_0).data).offset(i_0 as isize)
                        as *mut node_splat;
                    n = exec_expr(
                        strm,
                        state,
                        (*s_1).node,
                        &mut *ptr.offset(i_0 as isize),
                    );
                    if n != 0 {
                        return n;
                    }
                    if strm_array_p(*ptr.offset(i_0 as isize)) == 0 {
                        strm_raise(
                            strm,
                            b"splat requires array\0" as *const u8 as *const libc::c_char,
                        );
                        return 1 as libc::c_int;
                    }
                    splat = 1 as libc::c_int;
                } else {
                    n = exec_expr(
                        strm,
                        state,
                        *((*v0_0).data).offset(i_0 as isize),
                        &mut *ptr.offset(i_0 as isize),
                    );
                    if n != 0 {
                        return n;
                    }
                }
                i_0 += 1;
                i_0;
            }
            if splat != 0 {
                let mut len: libc::c_int = (*v0_0).len;
                if !((*v0_0).headers).is_null() {
                    strm_raise(
                        strm,
                        b"label(s) and splat(s) in an array\0" as *const u8
                            as *const libc::c_char,
                    );
                    return 1 as libc::c_int;
                }
                let mut i_1: libc::c_int = 0 as libc::c_int;
                while i_1 < (*v0_0).len {
                    if (**((*v0_0).data).offset(i_1 as isize)).type_0 as libc::c_uint
                        == NODE_SPLAT as libc::c_int as libc::c_uint
                    {
                        let mut a: strm_array = *ptr.offset(i_1 as isize);
                        len += (*strm_ary_struct(a)).len - 1 as libc::c_int;
                    }
                    i_1 += 1;
                    i_1;
                }
                if len > (*v0_0).len {
                    let mut nptr: *mut strm_value = 0 as *mut strm_value;
                    arr = strm_ary_new(0 as *const strm_value, len);
                    nptr = (*strm_ary_struct(arr)).ptr;
                    let mut i_2: libc::c_int = 0 as libc::c_int;
                    while i_2 < (*v0_0).len {
                        if (**((*v0_0).data).offset(i_2 as isize)).type_0 as libc::c_uint
                            == NODE_SPLAT as libc::c_int as libc::c_uint
                        {
                            let mut a_0: strm_array = *ptr.offset(i_2 as isize);
                            let mut alen: libc::c_int = (*strm_ary_struct(a_0)).len;
                            let mut aptr: *mut strm_value = (*strm_ary_struct(a_0)).ptr;
                            let mut j: libc::c_int = 0 as libc::c_int;
                            while j < alen {
                                let fresh1 = nptr;
                                nptr = nptr.offset(1);
                                *fresh1 = *aptr.offset(j as isize);
                                j += 1;
                                j;
                            }
                        } else {
                            let fresh2 = nptr;
                            nptr = nptr.offset(1);
                            *fresh2 = *ptr.offset(i_2 as isize);
                        }
                        i_2 += 1;
                        i_2;
                    }
                }
            } else if !((*v0_0).headers).is_null() {
                (*strm_ary_struct(arr))
                    .headers = ary_headers((*v0_0).headers, (*v0_0).len);
            }
            if !((*v0_0).ns).is_null() {
                let mut ns_1: *mut strm_state = strm_ns_get(node_to_sym((*v0_0).ns));
                if (*ns_1).flags & 1 as libc::c_int as libc::c_uint == 0 {
                    strm_raise(
                        strm,
                        b"instantiating primitive class\0" as *const u8
                            as *const libc::c_char,
                    );
                    return 1 as libc::c_int;
                }
                let ref mut fresh3 = (*strm_ary_struct(arr)).ns;
                *fresh3 = ns_1;
            } else {
                let ref mut fresh4 = (*strm_ary_struct(arr)).ns;
                *fresh4 = 0 as *mut strm_state;
            }
            *val = arr;
            return 0 as libc::c_int;
        }
        15 => {
            let mut ni: *mut node_ident = np as *mut node_ident;
            n = strm_var_get(state, node_to_sym((*ni).name), val);
            if n != 0 {
                strm_raise(
                    strm,
                    b"failed to reference variable\0" as *const u8 as *const libc::c_char,
                );
            }
            return n;
        }
        17 => {
            let mut v: strm_value = 0;
            let mut nif: *mut node_if = np as *mut node_if;
            n = exec_expr(strm, state, (*nif).cond, &mut v);
            if n != 0 {
                return n;
            }
            if strm_bool_p(v) != 0 && strm_value_bool(v) != 0 {
                return exec_expr(strm, state, (*nif).then, val)
            } else if !((*nif).opt_else).is_null() {
                return exec_expr(strm, state, (*nif).opt_else, val)
            } else {
                *val = strm_nil_value();
                return 0 as libc::c_int;
            }
        }
        23 => {
            let mut nop: *mut node_op = np as *mut node_op;
            let mut args: [strm_value; 2] = [0; 2];
            let mut i_3: libc::c_int = 0 as libc::c_int;
            if !((*nop).lhs).is_null() {
                let fresh5 = i_3;
                i_3 = i_3 + 1;
                n = exec_expr(
                    strm,
                    state,
                    (*nop).lhs,
                    &mut *args.as_mut_ptr().offset(fresh5 as isize),
                );
                if n != 0 {
                    return n;
                }
            }
            if !((*nop).rhs).is_null() {
                let fresh6 = i_3;
                i_3 = i_3 + 1;
                n = exec_expr(
                    strm,
                    state,
                    (*nop).rhs,
                    &mut *args.as_mut_ptr().offset(fresh6 as isize),
                );
                if n != 0 {
                    return n;
                }
            }
            return exec_call(
                strm,
                state,
                node_to_sym((*nop).op),
                i_3,
                args.as_mut_ptr(),
                val,
            );
        }
        9 | 10 => {
            let mut lambda: *mut strm_lambda = malloc(
                ::std::mem::size_of::<strm_lambda>() as libc::c_ulong,
            ) as *mut strm_lambda;
            if lambda.is_null() {
                return 1 as libc::c_int;
            }
            (*lambda)
                .state = malloc(::std::mem::size_of::<strm_state>() as libc::c_ulong)
                as *mut strm_state;
            if ((*lambda).state).is_null() {
                return 1 as libc::c_int;
            }
            *(*lambda).state = *state;
            (*lambda).type_0 = STRM_PTR_LAMBDA;
            (*lambda).body = np as *mut node_lambda;
            *val = strm_ptr_value(lambda as *mut libc::c_void);
            return 0 as libc::c_int;
        }
        24 => {
            let mut ncall: *mut node_call = np as *mut node_call;
            let mut i_4: libc::c_int = 0;
            let mut v0_1: *mut node_nodes = (*ncall).args as *mut node_nodes;
            let mut args_0: *mut strm_value = 0 as *mut strm_value;
            let mut splat_0: libc::c_int = 0 as libc::c_int;
            i_4 = 0 as libc::c_int;
            while i_4 < (*v0_1).len {
                if (**((*v0_1).data).offset(i_4 as isize)).type_0 as libc::c_uint
                    == NODE_SPLAT as libc::c_int as libc::c_uint
                {
                    splat_0 = 1 as libc::c_int;
                    break;
                } else {
                    i_4 += 1;
                    i_4;
                }
            }
            if splat_0 != 0 {
                let mut aary: strm_value = 0;
                n = exec_expr(strm, state, (*ncall).args, &mut aary);
                args_0 = (*strm_ary_struct(aary)).ptr;
                i_4 = (*strm_ary_struct(aary)).len;
            } else {
                args_0 = malloc(
                    (::std::mem::size_of::<strm_value>() as libc::c_ulong)
                        .wrapping_mul((*v0_1).len as libc::c_ulong),
                ) as *mut strm_value;
                i_4 = 0 as libc::c_int;
                while i_4 < (*v0_1).len {
                    n = exec_expr(
                        strm,
                        state,
                        *((*v0_1).data).offset(i_4 as isize),
                        &mut *args_0.offset(i_4 as isize),
                    );
                    if n == 1 as libc::c_int {
                        free(args_0 as *mut libc::c_void);
                        return n;
                    }
                    i_4 += 1;
                    i_4;
                }
            }
            n = exec_call(strm, state, node_to_sym((*ncall).ident), i_4, args_0, val);
            if splat_0 == 0 {
                free(args_0 as *mut libc::c_void);
            }
            return n;
        }
        25 => {
            let mut ncall_0: *mut node_fcall = np as *mut node_fcall;
            let mut i_5: libc::c_int = 0;
            let mut func: strm_value = 0;
            let mut v0_2: *mut node_nodes = (*ncall_0).args as *mut node_nodes;
            let mut args_1: *mut strm_value = 0 as *mut strm_value;
            let mut splat_1: libc::c_int = 0 as libc::c_int;
            if exec_expr(strm, state, (*ncall_0).func, &mut func) == 1 as libc::c_int {
                return 1 as libc::c_int;
            }
            i_5 = 0 as libc::c_int;
            while i_5 < (*v0_2).len {
                if (**((*v0_2).data).offset(i_5 as isize)).type_0 as libc::c_uint
                    == NODE_SPLAT as libc::c_int as libc::c_uint
                {
                    splat_1 = 1 as libc::c_int;
                    break;
                } else {
                    i_5 += 1;
                    i_5;
                }
            }
            if splat_1 != 0 {
                let mut aary_0: strm_value = 0;
                n = exec_expr(strm, state, (*ncall_0).args, &mut aary_0);
                args_1 = (*strm_ary_struct(aary_0)).ptr;
                i_5 = (*strm_ary_struct(aary_0)).len;
            } else {
                args_1 = malloc(
                    (::std::mem::size_of::<strm_value>() as libc::c_ulong)
                        .wrapping_mul((*v0_2).len as libc::c_ulong),
                ) as *mut strm_value;
                i_5 = 0 as libc::c_int;
                while i_5 < (*v0_2).len {
                    n = exec_expr(
                        strm,
                        state,
                        *((*v0_2).data).offset(i_5 as isize),
                        &mut *args_1.offset(i_5 as isize),
                    );
                    if n == 1 as libc::c_int {
                        free(args_1 as *mut libc::c_void);
                        return n;
                    }
                    i_5 += 1;
                    i_5;
                }
            }
            n = strm_funcall(strm, func, i_5, args_1, val);
            if splat_1 == 0 {
                free(args_1 as *mut libc::c_void);
            }
            return n;
        }
        26 => {
            let mut ngf: *mut node_genfunc = np as *mut node_genfunc;
            let mut gf: *mut strm_genfunc = 0 as *mut strm_genfunc;
            gf = genfunc_new(state, node_to_str((*ngf).id));
            if gf.is_null() {
                return 1 as libc::c_int;
            }
            *val = strm_ptr_value(gf as *mut libc::c_void);
            return 0 as libc::c_int;
        }
        20 => {
            let mut nreturn: *mut node_return = np as *mut node_return;
            let mut args_2: *mut node_nodes = (*nreturn).rv as *mut node_nodes;
            let mut arg: strm_value = 0;
            if args_2.is_null() {
                arg = strm_nil_value();
            } else {
                match (*args_2).len {
                    0 => {
                        arg = strm_nil_value();
                    }
                    1 => {
                        n = exec_expr(
                            strm,
                            state,
                            *((*args_2).data).offset(0 as libc::c_int as isize),
                            &mut arg,
                        );
                        if n != 0 {
                            return n;
                        }
                    }
                    _ => {
                        let mut ary: strm_array = strm_ary_new(
                            0 as *const strm_value,
                            (*args_2).len,
                        );
                        let mut i_6: strm_int = 0;
                        i_6 = 0 as libc::c_int;
                        while i_6 < (*args_2).len {
                            n = exec_expr(
                                strm,
                                state,
                                *((*args_2).data).offset(i_6 as isize),
                                &mut *((*(strm_ary_struct
                                    as unsafe extern "C" fn(
                                        strm_array,
                                    ) -> *mut strm_array_0)(ary))
                                    .ptr)
                                    .offset(i_6 as isize) as *mut strm_value,
                            );
                            if n != 0 {
                                return n;
                            }
                            i_6 += 1;
                            i_6;
                        }
                        arg = ary;
                    }
                }
            }
            strm_set_exc(strm, 1 as libc::c_int, arg);
            return 1 as libc::c_int;
        }
        28 => {
            let mut i_7: libc::c_int = 0;
            let mut v_0: *mut node_nodes = np as *mut node_nodes;
            i_7 = 0 as libc::c_int;
            while i_7 < (*v_0).len {
                n = exec_expr(strm, state, *((*v_0).data).offset(i_7 as isize), val);
                if n != 0 {
                    if !strm.is_null() {
                        let mut exc: *mut node_error = (*strm).exc;
                        if !exc.is_null() {
                            let mut n_1: *mut node = *((*v_0).data).offset(i_7 as isize);
                            (*exc).fname = (*n_1).fname;
                            (*exc).lineno = (*n_1).lineno;
                        }
                    }
                    return n;
                }
                i_7 += 1;
                i_7;
            }
            return 0 as libc::c_int;
        }
        0 => {
            *val = strm_int_value((*(np as *mut node_int)).value);
            return 0 as libc::c_int;
        }
        1 => {
            *val = strm_float_value((*(np as *mut node_float)).value);
            return 0 as libc::c_int;
        }
        2 => {
            let mut nt: *mut node_time = np as *mut node_time;
            *val = strm_time_new((*nt).sec, (*nt).usec, (*nt).utc_offset);
            if strm_nil_p(*val) != 0 {
                return 1 as libc::c_int;
            }
            return 0 as libc::c_int;
        }
        5 => {
            *val = strm_bool_value((*(np as *mut node_bool)).value);
            return 0 as libc::c_int;
        }
        4 => {
            *val = strm_nil_value();
            return 0 as libc::c_int;
        }
        3 => {
            *val = node_to_str((*(np as *mut node_str)).value);
            return 0 as libc::c_int;
        }
        _ => {
            strm_raise(strm, b"unknown node\0" as *const u8 as *const libc::c_char);
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn exec_cputs(
    mut strm: *mut strm_stream,
    mut out: *mut FILE,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < argc {
        let mut s: strm_string = 0;
        if i != 0 as libc::c_int {
            if strm_string_p(*args.offset((i - 1 as libc::c_int) as isize)) == 0 {
                fputs(b" \0" as *const u8 as *const libc::c_char, out);
            }
        }
        s = strm_to_str(*args.offset(i as isize));
        fwrite(
            strm_strp_ptr(&mut s) as *const libc::c_void,
            strm_str_len(s) as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            out,
        );
        i += 1;
        i;
    }
    fputs(b"\n\0" as *const u8 as *const libc::c_char, out);
    *ret = strm_nil_value();
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_puts(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    return exec_cputs(strm, stdout, argc, args, ret);
}
pub unsafe extern "C" fn strm_eprint(mut strm: *mut strm_stream) {
    let mut v: strm_value = 0;
    let mut exc: *mut node_error = (*strm).exc;
    if exc.is_null() {
        return;
    }
    if (*exc).type_0 == 2 as libc::c_int {
        return;
    }
    if !((*exc).fname).is_null() {
        fprintf(
            stderr,
            b"%s:%d:\0" as *const u8 as *const libc::c_char,
            (*exc).fname,
            (*exc).lineno,
        );
    }
    exec_cputs(strm, stderr, 1 as libc::c_int, &mut (*exc).arg, &mut v);
    strm_clear_exc(strm);
}
unsafe extern "C" fn exec_fread(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut path: strm_string = 0;
    let mut buf: [libc::c_char; 7] = [0; 7];
    if strm_parse_args(
        strm,
        argc,
        args,
        b"S\0" as *const u8 as *const libc::c_char,
        &mut path as *mut strm_string,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    fd = open(strm_str_cstr(path, buf.as_mut_ptr()), 0 as libc::c_int);
    if fd < 0 as libc::c_int {
        strm_raise(strm, b"fread() failed\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    *ret = strm_io_new(fd, 1 as libc::c_int);
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_fwrite(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut path: strm_string = 0;
    let mut buf: [libc::c_char; 7] = [0; 7];
    if strm_parse_args(
        strm,
        argc,
        args,
        b"S\0" as *const u8 as *const libc::c_char,
        &mut path as *mut strm_string,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    fd = open(
        strm_str_cstr(path, buf.as_mut_ptr()),
        0o1 as libc::c_int | 0o100 as libc::c_int,
        0o644 as libc::c_int,
    );
    if fd < 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    *ret = strm_io_new(fd, 2 as libc::c_int);
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_exit(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut estatus: strm_int = 0 as libc::c_int;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"|i\0" as *const u8 as *const libc::c_char,
        &mut estatus as *mut strm_int,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    exit(estatus);
}
unsafe extern "C" fn exec_match(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut func: strm_value = 0;
    if argc < 2 as libc::c_int {
        strm_raise(
            strm,
            b"wrong number of arguments\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    func = *args.offset((argc - 1 as libc::c_int) as isize);
    if strm_ptr_tag_p(func, STRM_PTR_LAMBDA) != 0 {
        let mut lambda: *mut strm_lambda = strm_value_ptr(func, STRM_PTR_LAMBDA)
            as *mut strm_lambda;
        if (*(*lambda).body).type_0 as libc::c_uint
            == NODE_LAMBDA as libc::c_int as libc::c_uint
        {
            strm_raise(
                strm,
                b"not a case function\0" as *const u8 as *const libc::c_char,
            );
            return 1 as libc::c_int;
        }
    }
    return strm_funcall(strm, func, argc - 1 as libc::c_int, args, ret);
}
pub unsafe extern "C" fn strm_raise(
    mut strm: *mut strm_stream,
    mut msg: *const libc::c_char,
) {
    if strm.is_null() {
        return;
    }
    strm_set_exc(strm, 0 as libc::c_int, strm_str_new(msg, strlen(msg) as strm_int));
}
unsafe extern "C" fn node_init(mut state: *mut strm_state) {
    strm_init(state);
    strm_var_def(
        state,
        b"stdin\0" as *const u8 as *const libc::c_char,
        strm_io_new(0 as libc::c_int, 1 as libc::c_int),
    );
    strm_var_def(
        state,
        b"stdout\0" as *const u8 as *const libc::c_char,
        strm_io_new(1 as libc::c_int, 2 as libc::c_int),
    );
    strm_var_def(
        state,
        b"stderr\0" as *const u8 as *const libc::c_char,
        strm_io_new(2 as libc::c_int, 2 as libc::c_int),
    );
    strm_var_def(
        state,
        b"puts\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_puts
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_var_def(
        state,
        b"print\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_puts
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_var_def(
        state,
        b"==\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_eq
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_var_def(
        state,
        b"!=\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_neq
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_var_def(
        state,
        b"|\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_bar
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_var_def(
        state,
        b"fread\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_fread
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_var_def(
        state,
        b"fwrite\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_fwrite
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_var_def(
        state,
        b"exit\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_exit
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_var_def(
        state,
        b"match\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_match
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
}
static mut top_state: strm_state = {
    let mut init = strm_state {
        env: 0 as *const libc::c_void as *mut libc::c_void,
        prev: 0 as *const strm_state as *mut strm_state,
        flags: 0,
    };
    init
};
static mut top_strm: strm_stream = {
    let mut init = strm_stream {
        type_0: STRM_PTR_STREAM,
        flags: 0,
        mode: strm_producer,
        start_func: None,
        close_func: None,
        data: 0 as *const libc::c_void as *mut libc::c_void,
        dst: 0 as *const strm_stream as *mut strm_stream,
        rest: 0 as *const *mut strm_stream as *mut *mut strm_stream,
        rsize: 0,
        rcapa: 0,
        exc: 0 as *const node_error as *mut node_error,
        refcnt: 0,
        queue: 0 as *const strm_queue as *mut strm_queue,
        excl: 0,
    };
    init
};
pub unsafe extern "C" fn node_run(mut p: *mut parser_state) -> libc::c_int {
    let mut v: strm_value = 0;
    let mut exc: *mut node_error = 0 as *mut node_error;
    node_init(&mut top_state);
    exec_expr(&mut top_strm, &mut top_state, (*p).lval as *mut node, &mut v);
    exc = top_strm.exc;
    if !exc.is_null() {
        if (*exc).type_0 != 1 as libc::c_int {
            strm_eprint(&mut top_strm);
        }
        strm_clear_exc(&mut top_strm);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn node_stop() {}
unsafe extern "C" fn blk_exec(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut lambda: *mut strm_lambda = (*strm).data as *mut strm_lambda;
    let mut ret: strm_value = strm_nil_value();
    let mut args: *mut node_args = (*(*lambda).body).args as *mut node_args;
    let mut exc: *mut node_error = 0 as *mut node_error;
    let mut n: libc::c_int = 0;
    let mut c: strm_state = {
        let mut init = strm_state {
            env: 0 as *mut libc::c_void,
            prev: 0 as *mut strm_state,
            flags: 0,
        };
        init
    };
    c.prev = (*lambda).state;
    if !args.is_null() {
        if (*args).len == 1 as libc::c_int {} else {
            __assert_fail(
                b"args->len == 1\0" as *const u8 as *const libc::c_char,
                b"exec.c\0" as *const u8 as *const libc::c_char,
                1100 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_char; 40],
                >(b"int blk_exec(strm_stream *, strm_value)\0"))
                    .as_ptr(),
            );
        }
        'c_8018: {
            if (*args).len == 1 as libc::c_int {} else {
                __assert_fail(
                    b"args->len == 1\0" as *const u8 as *const libc::c_char,
                    b"exec.c\0" as *const u8 as *const libc::c_char,
                    1100 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 40],
                        &[libc::c_char; 40],
                    >(b"int blk_exec(strm_stream *, strm_value)\0"))
                        .as_ptr(),
                );
            }
        };
        strm_var_set(
            &mut c,
            node_to_sym(*((*args).data).offset(0 as libc::c_int as isize)),
            data,
        );
    }
    n = exec_expr(strm, &mut c, (*(*lambda).body).body, &mut ret);
    exc = (*strm).exc;
    if !exc.is_null() {
        if (*exc).type_0 == 1 as libc::c_int {
            ret = (*exc).arg;
            strm_clear_exc(strm);
        } else {
            if strm_option_verbose != 0 {
                strm_eprint(strm);
            }
            return 1 as libc::c_int;
        }
    }
    if n != 0 {
        return 1 as libc::c_int;
    }
    strm_emit(strm, ret, None);
    return 0 as libc::c_int;
}
unsafe extern "C" fn arr_exec(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut arrd: *mut array_data = (*strm).data as *mut array_data;
    if (*arrd).n == (*strm_ary_struct((*arrd).arr)).len {
        strm_stream_close(strm);
        return 0 as libc::c_int;
    }
    let fresh7 = (*arrd).n;
    (*arrd).n = (*arrd).n + 1;
    strm_emit(
        strm,
        *((*strm_ary_struct((*arrd).arr)).ptr).offset(fresh7 as isize),
        Some(
            arr_exec as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
        ),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn cfunc_exec(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut ret: strm_value = 0;
    let mut func: strm_cfunc = ::std::mem::transmute::<
        *mut libc::c_void,
        strm_cfunc,
    >((*strm).data);
    if (Some(func.unwrap())).unwrap()(strm, 1 as libc::c_int, &mut data, &mut ret)
        == 0 as libc::c_int
    {
        strm_emit(strm, ret, None);
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
