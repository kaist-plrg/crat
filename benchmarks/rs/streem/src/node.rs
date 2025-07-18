use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn perror(__s: *const libc::c_char);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn strm_time_parse_time(
        s: *const libc::c_char,
        len: strm_int,
        sec: *mut libc::c_long,
        usec: *mut libc::c_long,
        offset: *mut libc::c_int,
    ) -> libc::c_int;
    fn yyparse(p: *mut parser_state) -> libc::c_int;
    fn yyrestart(input_file: *mut FILE);
    fn yy_scan_string(yy_str: *const libc::c_char) -> YY_BUFFER_STATE;
}
pub type size_t = libc::c_ulong;
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
pub type int32_t = __int32_t;
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
pub struct node_array {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub len: libc::c_int,
    pub max: libc::c_int,
    pub data: *mut *mut node,
    pub headers: *mut node_string_0,
    pub ns: node_string_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_ident {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub name: node_string_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_call {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub ident: node_string_0,
    pub args: *mut node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_args {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub len: libc::c_int,
    pub max: libc::c_int,
    pub data: *mut node_string_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_op {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub op: node_string_0,
    pub lhs: *mut node,
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
pub struct node_if {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub cond: *mut node,
    pub then: *mut node,
    pub opt_else: *mut node,
}
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
pub struct node_float {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub value: libc::c_double,
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
pub struct node_bool {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub value: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_pair {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub key: node_string_0,
    pub value: *mut node,
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
    pub lhs: node_string_0,
    pub rhs: *mut node,
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
pub struct node_fcall {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub func: *mut node,
    pub args: *mut node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_genfunc {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub id: node_string_0,
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
pub struct node_ns {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub name: node_string_0,
    pub body: *mut node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_import {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub name: node_string_0,
}
pub unsafe extern "C" fn node_array_new() -> *mut node {
    let mut v: *mut node_array = malloc(
        ::std::mem::size_of::<node_array>() as libc::c_ulong,
    ) as *mut node_array;
    (*v).type_0 = NODE_ARRAY;
    (*v).len = 0 as libc::c_int;
    (*v).max = 0 as libc::c_int;
    (*v).data = 0 as *mut *mut node;
    (*v).headers = 0 as *mut node_string_0;
    (*v).ns = 0 as node_string_0;
    return v as *mut node;
}
pub unsafe extern "C" fn node_pair_new(
    mut key: node_string_0,
    mut value: *mut node,
) -> *mut node {
    let mut npair: *mut node_pair = malloc(
        ::std::mem::size_of::<node_pair>() as libc::c_ulong,
    ) as *mut node_pair;
    (*npair).type_0 = NODE_PAIR;
    (*npair).key = key;
    (*npair).value = value;
    return npair as *mut node;
}
pub unsafe extern "C" fn node_array_headers(mut np: *mut node) -> *mut node {
    let mut i: libc::c_int = 0;
    let mut v: *mut node_array = 0 as *mut node_array;
    let mut headers: *mut node_string_0 = 0 as *mut node_string_0;
    if np.is_null() {
        np = node_array_new();
    }
    v = np as *mut node_array;
    i = 0 as libc::c_int;
    while i < (*v).len {
        let mut npair: *mut node_pair = *((*v).data).offset(i as isize)
            as *mut node_pair;
        if !npair.is_null()
            && (*npair).type_0 as libc::c_uint
                == NODE_PAIR as libc::c_int as libc::c_uint
        {
            if headers.is_null() {
                headers = malloc(
                    (::std::mem::size_of::<node_string_0>() as libc::c_ulong)
                        .wrapping_mul((*v).len as libc::c_ulong),
                ) as *mut node_string_0;
            }
            let ref mut fresh0 = *headers.offset(i as isize);
            *fresh0 = (*npair).key;
            let ref mut fresh1 = *((*v).data).offset(i as isize);
            *fresh1 = (*npair).value;
            free(npair as *mut libc::c_void);
        }
        i += 1;
        i;
    }
    (*v).headers = headers;
    return np;
}
pub unsafe extern "C" fn node_array_add(mut a: *mut node, mut data: *mut node) {
    let mut _v: *mut node_array = a as *mut node_array;
    if (*_v).len == (*_v).max {
        (*_v).max = (*_v).len + 10 as libc::c_int;
        (*_v)
            .data = realloc(
            (*_v).data as *mut libc::c_void,
            (::std::mem::size_of::<*mut node>() as libc::c_ulong)
                .wrapping_mul((*_v).max as libc::c_ulong),
        ) as *mut *mut node;
    }
    let ref mut fresh2 = *((*_v).data).offset((*_v).len as isize);
    *fresh2 = data;
    (*_v).len += 1;
    (*_v).len;
}
pub unsafe extern "C" fn node_array_free(mut v: *mut node_array) {
    let mut i: strm_int = 0;
    i = 0 as libc::c_int;
    while i < (*v).len {
        node_free(*((*v).data).offset(i as isize));
        i += 1;
        i;
    }
    free((*v).data as *mut libc::c_void);
    if !((*v).headers).is_null() {
        i = 0 as libc::c_int;
        while i < (*v).len {
            free(*((*v).headers).offset(i as isize) as *mut libc::c_void);
            i += 1;
            i;
        }
        free((*v).headers as *mut libc::c_void);
    }
    if !((*v).ns).is_null() {
        free((*v).ns as *mut libc::c_void);
    }
    free(v as *mut libc::c_void);
}
pub unsafe extern "C" fn node_nodes_new() -> *mut node {
    let mut v: *mut node_nodes = malloc(
        ::std::mem::size_of::<node_nodes>() as libc::c_ulong,
    ) as *mut node_nodes;
    (*v).type_0 = NODE_NODES;
    (*v).len = 0 as libc::c_int;
    (*v).max = 0 as libc::c_int;
    (*v).data = 0 as *mut *mut node;
    return v as *mut node;
}
pub unsafe extern "C" fn node_nodes_add(mut a: *mut node, mut data: *mut node) {
    let mut _v: *mut node_nodes = a as *mut node_nodes;
    if (*_v).len == (*_v).max {
        (*_v).max = (*_v).len + 10 as libc::c_int;
        (*_v)
            .data = realloc(
            (*_v).data as *mut libc::c_void,
            (::std::mem::size_of::<*mut node>() as libc::c_ulong)
                .wrapping_mul((*_v).max as libc::c_ulong),
        ) as *mut *mut node;
    }
    let ref mut fresh3 = *((*_v).data).offset((*_v).len as isize);
    *fresh3 = data;
    (*_v).len += 1;
    (*_v).len;
}
pub unsafe extern "C" fn node_nodes_prepend(mut a: *mut node, mut data: *mut node) {
    let mut _v: *mut node_nodes = a as *mut node_nodes;
    if (*_v).len == (*_v).max {
        (*_v).max = (*_v).len + 10 as libc::c_int;
        (*_v)
            .data = realloc(
            (*_v).data as *mut libc::c_void,
            (::std::mem::size_of::<*mut node>() as libc::c_ulong)
                .wrapping_mul((*_v).max as libc::c_ulong),
        ) as *mut *mut node;
    }
    memmove(
        ((*_v).data).offset(1 as libc::c_int as isize) as *mut libc::c_void,
        (*_v).data as *const libc::c_void,
        ((*_v).len as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut node>() as libc::c_ulong),
    );
    let ref mut fresh4 = *((*_v).data).offset(0 as libc::c_int as isize);
    *fresh4 = data;
    (*_v).len += 1;
    (*_v).len;
}
pub unsafe extern "C" fn node_nodes_concat(
    mut s: *mut node,
    mut s2: *mut node,
) -> *mut node {
    if s.is_null() {
        return s2;
    }
    if !s2.is_null() {
        let mut v: *mut node_nodes = s as *mut node_nodes;
        let mut v2: *mut node_nodes = s2 as *mut node_nodes;
        if (*v).len + (*v2).len > (*v).max {
            (*v).max = (*v).len + (*v2).len + 10 as libc::c_int;
            (*v)
                .data = realloc(
                (*v).data as *mut libc::c_void,
                (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                    .wrapping_mul((*v).max as libc::c_ulong),
            ) as *mut *mut node;
        }
        memcpy(
            ((*v).data).offset((*v).len as isize) as *mut libc::c_void,
            (*v2).data as *const libc::c_void,
            ((*v2).len as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut node>() as libc::c_ulong),
        );
        (*v).len += (*v2).len;
    }
    return s;
}
pub unsafe extern "C" fn node_nodes_free(mut v: *mut node_nodes) {
    let mut i: strm_int = 0;
    i = 0 as libc::c_int;
    while i < (*v).len {
        node_free(*((*v).data).offset(i as isize));
        i += 1;
        i;
    }
    free((*v).data as *mut libc::c_void);
    free(v as *mut libc::c_void);
}
pub unsafe extern "C" fn node_obj_new(
    mut np: *mut node,
    mut ns: node_string_0,
) -> *mut node {
    let mut v: *mut node_array = 0 as *mut node_array;
    if np.is_null() {
        v = node_array_new() as *mut node_array;
    } else {
        v = np as *mut node_array;
    }
    (*v).ns = ns;
    return v as *mut node;
}
pub unsafe extern "C" fn node_args_new() -> *mut node {
    let mut v: *mut node_args = malloc(
        ::std::mem::size_of::<node_args>() as libc::c_ulong,
    ) as *mut node_args;
    (*v).type_0 = NODE_ARGS;
    (*v).len = 0 as libc::c_int;
    (*v).max = 0 as libc::c_int;
    (*v).data = 0 as *mut node_string_0;
    return v as *mut node;
}
pub unsafe extern "C" fn node_args_add(mut v: *mut node, mut data: node_string_0) {
    let mut _v: *mut node_args = v as *mut node_args;
    if (*_v).len == (*_v).max {
        (*_v).max = (*_v).len + 10 as libc::c_int;
        (*_v)
            .data = realloc(
            (*_v).data as *mut libc::c_void,
            (::std::mem::size_of::<node_string_0>() as libc::c_ulong)
                .wrapping_mul((*_v).max as libc::c_ulong),
        ) as *mut node_string_0;
    }
    let ref mut fresh5 = *((*_v).data).offset((*_v).len as isize);
    *fresh5 = data;
    (*_v).len += 1;
    (*_v).len;
}
pub unsafe extern "C" fn node_args_prepend(mut a: *mut node, mut data: node_string_0) {
    let mut _v: *mut node_args = a as *mut node_args;
    if (*_v).len == (*_v).max {
        (*_v).max = (*_v).len + 10 as libc::c_int;
        (*_v)
            .data = realloc(
            (*_v).data as *mut libc::c_void,
            (::std::mem::size_of::<node_string_0>() as libc::c_ulong)
                .wrapping_mul((*_v).max as libc::c_ulong),
        ) as *mut node_string_0;
    }
    memmove(
        ((*_v).data).offset(1 as libc::c_int as isize) as *mut libc::c_void,
        (*_v).data as *const libc::c_void,
        ((*_v).len as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<node_string_0>() as libc::c_ulong),
    );
    let ref mut fresh6 = *((*_v).data).offset(0 as libc::c_int as isize);
    *fresh6 = data;
    (*_v).len += 1;
    (*_v).len;
}
pub unsafe extern "C" fn node_args_free(mut a: *mut node) {
    let mut i: strm_int = 0;
    let mut v: *mut node_args = a as *mut node_args;
    if (*a).type_0 as libc::c_uint == NODE_ARGS as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"a->type == NODE_ARGS\0" as *const u8 as *const libc::c_char,
            b"node.c\0" as *const u8 as *const libc::c_char,
            191 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void node_args_free(node *)\0"))
                .as_ptr(),
        );
    }
    'c_3319: {
        if (*a).type_0 as libc::c_uint == NODE_ARGS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"a->type == NODE_ARGS\0" as *const u8 as *const libc::c_char,
                b"node.c\0" as *const u8 as *const libc::c_char,
                191 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"void node_args_free(node *)\0"))
                    .as_ptr(),
            );
        }
    };
    i = 0 as libc::c_int;
    while i < (*v).len {
        free(*((*v).data).offset(i as isize) as *mut libc::c_void);
        i += 1;
        i;
    }
    free((*v).data as *mut libc::c_void);
    free(v as *mut libc::c_void);
}
pub unsafe extern "C" fn node_pattern_new(mut type_0: node_type) -> *mut node {
    let mut v: *mut node_nodes = malloc(
        ::std::mem::size_of::<node_nodes>() as libc::c_ulong,
    ) as *mut node_nodes;
    (*v).type_0 = type_0;
    (*v).len = 0 as libc::c_int;
    (*v).max = 0 as libc::c_int;
    (*v).data = 0 as *mut *mut node;
    return v as *mut node;
}
pub unsafe extern "C" fn node_pattern_add(mut v: *mut node, mut data: *mut node) {
    let mut _v: *mut node_nodes = v as *mut node_nodes;
    if (*_v).len == (*_v).max {
        (*_v).max = (*_v).len + 10 as libc::c_int;
        (*_v)
            .data = realloc(
            (*_v).data as *mut libc::c_void,
            (::std::mem::size_of::<*mut node>() as libc::c_ulong)
                .wrapping_mul((*_v).max as libc::c_ulong),
        ) as *mut *mut node;
    }
    let ref mut fresh7 = *((*_v).data).offset((*_v).len as isize);
    *fresh7 = data;
    (*_v).len += 1;
    (*_v).len;
}
pub unsafe extern "C" fn node_psplat_new(
    mut head: *mut node,
    mut mid: *mut node,
    mut tail: *mut node,
) -> *mut node {
    let mut cons: *mut node_psplat = malloc(
        ::std::mem::size_of::<node_psplat>() as libc::c_ulong,
    ) as *mut node_psplat;
    (*cons).type_0 = NODE_PSPLAT;
    (*cons).head = head;
    (*cons).mid = mid;
    (*cons).tail = tail;
    return cons as *mut node;
}
pub unsafe extern "C" fn node_plambda_new(
    mut pat: *mut node,
    mut cond: *mut node,
) -> *mut node {
    let mut lambda: *mut node_plambda = malloc(
        ::std::mem::size_of::<node_plambda>() as libc::c_ulong,
    ) as *mut node_plambda;
    (*lambda).type_0 = NODE_PLAMBDA;
    (*lambda).pat = pat;
    (*lambda).cond = cond;
    (*lambda).body = 0 as *mut node;
    return lambda as *mut node;
}
pub unsafe extern "C" fn node_plambda_body(
    mut n: *mut node,
    mut body: *mut node,
) -> *mut node {
    let mut lambda: *mut node_plambda = n as *mut node_plambda;
    (*lambda).body = body;
    return lambda as *mut node;
}
pub unsafe extern "C" fn node_plambda_add(
    mut n: *mut node,
    mut lambda: *mut node,
) -> *mut node {
    let mut l: *mut node_plambda = n as *mut node_plambda;
    while !((*l).next).is_null() {
        l = (*l).next as *mut node_plambda;
    }
    (*l).next = lambda;
    return n;
}
pub unsafe extern "C" fn node_splat_new(mut n: *mut node) -> *mut node {
    let mut splat: *mut node_splat = malloc(
        ::std::mem::size_of::<node_splat>() as libc::c_ulong,
    ) as *mut node_splat;
    (*splat).type_0 = NODE_SPLAT;
    (*splat).node = n;
    return splat as *mut node;
}
pub unsafe extern "C" fn node_ns_new(
    mut name: node_string_0,
    mut body: *mut node,
) -> *mut node {
    let mut newns: *mut node_ns = malloc(
        ::std::mem::size_of::<node_ns>() as libc::c_ulong,
    ) as *mut node_ns;
    (*newns).type_0 = NODE_NS;
    (*newns).name = name;
    (*newns).body = body;
    return newns as *mut node;
}
pub unsafe extern "C" fn node_import_new(mut name: node_string_0) -> *mut node {
    let mut nimp: *mut node_import = malloc(
        ::std::mem::size_of::<node_import>() as libc::c_ulong,
    ) as *mut node_import;
    (*nimp).type_0 = NODE_IMPORT;
    (*nimp).name = name;
    return nimp as *mut node;
}
pub unsafe extern "C" fn node_let_new(
    mut lhs: node_string_0,
    mut rhs: *mut node,
) -> *mut node {
    let mut nlet: *mut node_let = malloc(
        ::std::mem::size_of::<node_let>() as libc::c_ulong,
    ) as *mut node_let;
    (*nlet).type_0 = NODE_LET;
    (*nlet).lhs = lhs;
    (*nlet).rhs = rhs;
    return nlet as *mut node;
}
pub unsafe extern "C" fn node_op_new(
    mut op: *const libc::c_char,
    mut lhs: *mut node,
    mut rhs: *mut node,
) -> *mut node {
    let mut nop: *mut node_op = malloc(::std::mem::size_of::<node_op>() as libc::c_ulong)
        as *mut node_op;
    (*nop).type_0 = NODE_OP;
    (*nop).lhs = lhs;
    (*nop).op = node_str_new(op, strlen(op) as strm_int);
    (*nop).rhs = rhs;
    return nop as *mut node;
}
pub unsafe extern "C" fn node_lambda_alloc(
    mut args: *mut node,
    mut compstmt: *mut node,
    mut block: libc::c_int,
) -> *mut node {
    let mut lambda: *mut node_lambda = malloc(
        ::std::mem::size_of::<node_lambda>() as libc::c_ulong,
    ) as *mut node_lambda;
    (*lambda).type_0 = NODE_LAMBDA;
    (*lambda).args = args;
    (*lambda).body = compstmt;
    (*lambda).block = block;
    (*lambda)
        .fname = if !compstmt.is_null() {
        (*compstmt).fname
    } else {
        0 as *const libc::c_char
    };
    (*lambda)
        .lineno = if !compstmt.is_null() {
        (*compstmt).lineno
    } else {
        0 as libc::c_int
    };
    return lambda as *mut node;
}
pub unsafe extern "C" fn node_lambda_new(
    mut args: *mut node,
    mut compstmt: *mut node,
) -> *mut node {
    return node_lambda_alloc(args, compstmt, 0 as libc::c_int);
}
pub unsafe extern "C" fn node_block_new(mut compstmt: *mut node) -> *mut node {
    return node_lambda_alloc(0 as *mut node, compstmt, 1 as libc::c_int);
}
pub unsafe extern "C" fn node_method_new(
    mut args: *mut node,
    mut compstmt: *mut node,
) -> *mut node {
    let mut lambda: *mut node_lambda = malloc(
        ::std::mem::size_of::<node_lambda>() as libc::c_ulong,
    ) as *mut node_lambda;
    (*lambda).type_0 = NODE_LAMBDA;
    if !args.is_null() {
        node_args_prepend(
            args,
            node_str_new(b"self\0" as *const u8 as *const libc::c_char, 4 as libc::c_int),
        );
    } else {
        args = node_args_new();
        node_args_add(
            args,
            node_str_new(b"self\0" as *const u8 as *const libc::c_char, 4 as libc::c_int),
        );
    }
    (*lambda).args = args;
    (*lambda).body = compstmt;
    return lambda as *mut node;
}
pub unsafe extern "C" fn node_call_new(
    mut ident: node_string_0,
    mut recv: *mut node,
    mut args: *mut node,
    mut blk: *mut node,
) -> *mut node {
    let mut ncall: *mut node_call = malloc(
        ::std::mem::size_of::<node_call>() as libc::c_ulong,
    ) as *mut node_call;
    (*ncall).type_0 = NODE_CALL;
    (*ncall).ident = ident;
    if args.is_null() {
        args = node_array_new();
    }
    if !recv.is_null() {
        node_nodes_prepend(args, recv);
    }
    if !blk.is_null() {
        node_nodes_add(args, blk);
    }
    (*ncall).args = args;
    return ncall as *mut node;
}
pub unsafe extern "C" fn node_fcall_new(
    mut func: *mut node,
    mut args: *mut node,
    mut blk: *mut node,
) -> *mut node {
    let mut ncall: *mut node_fcall = malloc(
        ::std::mem::size_of::<node_fcall>() as libc::c_ulong,
    ) as *mut node_fcall;
    (*ncall).type_0 = NODE_FCALL;
    (*ncall).func = func;
    if args.is_null() {
        args = node_array_new();
    }
    if !blk.is_null() {
        node_nodes_add(args, blk);
    }
    (*ncall).args = args;
    return ncall as *mut node;
}
pub unsafe extern "C" fn node_genfunc_new(mut id: node_string_0) -> *mut node {
    let mut ngf: *mut node_genfunc = malloc(
        ::std::mem::size_of::<node_genfunc>() as libc::c_ulong,
    ) as *mut node_genfunc;
    (*ngf).type_0 = NODE_GENFUNC;
    (*ngf).id = id;
    return ngf as *mut node;
}
pub unsafe extern "C" fn node_int_new(mut i: libc::c_long) -> *mut node {
    let mut ni: *mut node_int = malloc(
        ::std::mem::size_of::<node_int>() as libc::c_ulong,
    ) as *mut node_int;
    (*ni).type_0 = NODE_INT;
    (*ni).value = i as int32_t;
    return ni as *mut node;
}
pub unsafe extern "C" fn node_float_new(mut d: libc::c_double) -> *mut node {
    let mut nf: *mut node_float = malloc(
        ::std::mem::size_of::<node_float>() as libc::c_ulong,
    ) as *mut node_float;
    (*nf).type_0 = NODE_FLOAT;
    (*nf).value = d;
    return nf as *mut node;
}
pub unsafe extern "C" fn node_time_new(
    mut s: *const libc::c_char,
    mut len: strm_int,
) -> *mut node {
    let mut sec: libc::c_long = 0;
    let mut usec: libc::c_long = 0;
    let mut utc_offset: libc::c_int = 0;
    let mut ns: *mut node_time = 0 as *mut node_time;
    if strm_time_parse_time(s, len, &mut sec, &mut usec, &mut utc_offset)
        < 0 as libc::c_int
    {
        return 0 as *mut node;
    }
    ns = malloc(::std::mem::size_of::<node_time>() as libc::c_ulong) as *mut node_time;
    (*ns).type_0 = NODE_TIME;
    (*ns).sec = sec;
    (*ns).usec = usec;
    (*ns).utc_offset = utc_offset;
    return ns as *mut node;
}
unsafe extern "C" fn string_escape(
    mut s: *mut libc::c_char,
    mut len: strm_int,
) -> strm_int {
    let mut t: *mut libc::c_char = s;
    let mut tend: *mut libc::c_char = t.offset(len as isize);
    let mut p: *mut libc::c_char = s;
    while t < tend {
        match *t as libc::c_int {
            92 => {
                t = t.offset(1);
                t;
                if !(t == tend) {
                    match *t as libc::c_int {
                        110 => {
                            let fresh8 = p;
                            p = p.offset(1);
                            *fresh8 = '\n' as i32 as libc::c_char;
                        }
                        114 => {
                            let fresh9 = p;
                            p = p.offset(1);
                            *fresh9 = '\r' as i32 as libc::c_char;
                        }
                        116 => {
                            let fresh10 = p;
                            p = p.offset(1);
                            *fresh10 = '\t' as i32 as libc::c_char;
                        }
                        101 => {
                            let fresh11 = p;
                            p = p.offset(1);
                            *fresh11 = 0o33 as libc::c_int as libc::c_char;
                        }
                        48 => {
                            let fresh12 = p;
                            p = p.offset(1);
                            *fresh12 = '\0' as i32 as libc::c_char;
                        }
                        120 => {
                            let mut c: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
                            let mut xend: *mut libc::c_char = t
                                .offset(3 as libc::c_int as isize);
                            t = t.offset(1);
                            t;
                            while t < tend && t < xend {
                                match *t as libc::c_int {
                                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                                        c = (c as libc::c_int * 16 as libc::c_int) as libc::c_uchar;
                                        c = (c as libc::c_int + (*t as libc::c_int - '0' as i32))
                                            as libc::c_uchar;
                                    }
                                    97 | 98 | 99 | 100 | 101 | 102 => {
                                        c = (c as libc::c_int * 16 as libc::c_int) as libc::c_uchar;
                                        c = (c as libc::c_int
                                            + (*t as libc::c_int - 'a' as i32 + 10 as libc::c_int))
                                            as libc::c_uchar;
                                    }
                                    _ => {
                                        xend = t;
                                    }
                                }
                                t = t.offset(1);
                                t;
                            }
                            let fresh13 = p;
                            p = p.offset(1);
                            *fresh13 = c as libc::c_char;
                            t = t.offset(-1);
                            t;
                        }
                        _ => {
                            let fresh14 = p;
                            p = p.offset(1);
                            *fresh14 = *t;
                        }
                    }
                    t = t.offset(1);
                    t;
                }
            }
            _ => {
                let fresh15 = t;
                t = t.offset(1);
                let fresh16 = p;
                p = p.offset(1);
                *fresh16 = *fresh15;
            }
        }
    }
    return p.offset_from(s) as libc::c_long as strm_int;
}
pub unsafe extern "C" fn node_string_new(
    mut s: *const libc::c_char,
    mut len: strm_int,
) -> *mut node {
    let mut ns: *mut node_str = malloc(
        ::std::mem::size_of::<node_str>() as libc::c_ulong,
    ) as *mut node_str;
    (*ns).type_0 = NODE_STR;
    len = string_escape(s as *mut libc::c_char, len);
    (*ns).value = node_str_new(s, len);
    return ns as *mut node;
}
pub unsafe extern "C" fn node_ident_new(mut name: node_string_0) -> *mut node {
    let mut ni: *mut node_ident = malloc(
        ::std::mem::size_of::<node_ident>() as libc::c_ulong,
    ) as *mut node_ident;
    (*ni).type_0 = NODE_IDENT;
    (*ni).name = name;
    return ni as *mut node;
}
pub unsafe extern "C" fn node_str_new(
    mut s: *const libc::c_char,
    mut len: strm_int,
) -> node_string_0 {
    let mut str: node_string_0 = 0 as *mut node_string;
    str = malloc(
        (::std::mem::size_of::<node_string>() as libc::c_ulong)
            .wrapping_add(len as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as node_string_0;
    (*str).len = len;
    memcpy(
        ((*str).buf).as_mut_ptr() as *mut libc::c_void,
        s as *const libc::c_void,
        len as libc::c_ulong,
    );
    *((*str).buf).as_mut_ptr().offset(len as isize) = '\0' as i32 as libc::c_char;
    return str;
}
pub unsafe extern "C" fn node_str_escaped(
    mut s: *const libc::c_char,
    mut len: strm_int,
) -> node_string_0 {
    len = string_escape(s as *mut libc::c_char, len);
    return node_str_new(s, len);
}
pub unsafe extern "C" fn node_nil() -> *mut node {
    static mut nd: node = {
        let mut init = node {
            type_0: NODE_NIL,
            fname: 0 as *const libc::c_char,
            lineno: 0,
        };
        init
    };
    return &mut nd;
}
pub unsafe extern "C" fn node_true() -> *mut node {
    static mut nd: node_bool = {
        let mut init = node_bool {
            type_0: NODE_BOOL,
            fname: 0 as *const libc::c_char,
            lineno: 0 as libc::c_int,
            value: 1 as libc::c_int,
        };
        init
    };
    return &mut nd as *mut node_bool as *mut node;
}
pub unsafe extern "C" fn node_false() -> *mut node {
    static mut nd: node_bool = {
        let mut init = node_bool {
            type_0: NODE_BOOL,
            fname: 0 as *const libc::c_char,
            lineno: 0 as libc::c_int,
            value: 0 as libc::c_int,
        };
        init
    };
    return &mut nd as *mut node_bool as *mut node;
}
unsafe extern "C" fn cond_body(mut body: *mut node) -> *mut node {
    if body.is_null() {
        return 0 as *mut node;
    }
    if (*body).type_0 as libc::c_uint == NODE_LAMBDA as libc::c_int as libc::c_uint {
        let mut lambda: *mut node_lambda = body as *mut node_lambda;
        if (*lambda).block != 0 {
            return (*lambda).body;
        }
    }
    return body;
}
pub unsafe extern "C" fn node_if_new(
    mut cond: *mut node,
    mut then: *mut node,
    mut opt_else: *mut node,
) -> *mut node {
    let mut nif: *mut node_if = malloc(::std::mem::size_of::<node_if>() as libc::c_ulong)
        as *mut node_if;
    (*nif).type_0 = NODE_IF;
    (*nif).cond = cond;
    (*nif).then = cond_body(then);
    (*nif).opt_else = cond_body(opt_else);
    return nif as *mut node;
}
pub unsafe extern "C" fn node_emit_new(mut value: *mut node) -> *mut node {
    let mut ne: *mut node_emit = malloc(
        ::std::mem::size_of::<node_emit>() as libc::c_ulong,
    ) as *mut node_emit;
    (*ne).type_0 = NODE_EMIT;
    (*ne).emit = value;
    return ne as *mut node;
}
pub unsafe extern "C" fn node_skip_new() -> *mut node {
    static mut nd: node = {
        let mut init = node {
            type_0: NODE_SKIP,
            fname: 0 as *const libc::c_char,
            lineno: 0,
        };
        init
    };
    return &mut nd;
}
pub unsafe extern "C" fn node_return_new(mut value: *mut node) -> *mut node {
    let mut nreturn: *mut node_return = malloc(
        ::std::mem::size_of::<node_return>() as libc::c_ulong,
    ) as *mut node_return;
    (*nreturn).type_0 = NODE_RETURN;
    (*nreturn).rv = value;
    return nreturn as *mut node;
}
pub unsafe extern "C" fn node_parse_init(mut p: *mut parser_state) {
    (*p).nerr = 0 as libc::c_int;
    (*p).lval = 0 as *mut libc::c_void;
    (*p).fname = 0 as *const libc::c_char;
    (*p).lineno = 1 as libc::c_int;
    (*p).tline = 1 as libc::c_int;
}
pub unsafe extern "C" fn node_parse_file(
    mut p: *mut parser_state,
    mut fname: *const libc::c_char,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut fp: *mut FILE = fopen(fname, b"rb\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        perror(b"fopen\0" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int;
    }
    (*p).fname = fname;
    r = node_parse_input(p, fp, fname);
    fclose(fp);
    return r;
}
pub unsafe extern "C" fn node_parse_input(
    mut p: *mut parser_state,
    mut f: *mut FILE,
    mut fname: *const libc::c_char,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    yyrestart(f);
    n = yyparse(p);
    if n == 0 as libc::c_int && (*p).nerr == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn node_parse_string(
    mut p: *mut parser_state,
    mut prog: *const libc::c_char,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    (*p).fname = b"-e\0" as *const u8 as *const libc::c_char;
    yy_scan_string(prog);
    n = yyparse(p);
    if n == 0 as libc::c_int && (*p).nerr == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn node_free(mut np: *mut node) {
    if np.is_null() {
        return;
    }
    match (*np).type_0 as libc::c_uint {
        6 => {
            node_args_free(np);
        }
        17 => {
            node_free((*(np as *mut node_if)).cond);
            node_free((*(np as *mut node_if)).then);
            node_free((*(np as *mut node_if)).opt_else);
            free(np as *mut libc::c_void);
        }
        18 => {
            node_free((*(np as *mut node_emit)).emit);
            free(np as *mut libc::c_void);
        }
        23 => {
            node_free((*(np as *mut node_op)).lhs);
            node_free((*(np as *mut node_op)).rhs);
            free(np as *mut libc::c_void);
        }
        9 => {
            node_args_free((*(np as *mut node_lambda)).args);
            node_free((*(np as *mut node_lambda)).body);
            free(np as *mut libc::c_void);
        }
        24 => {
            node_free((*(np as *mut node_call)).args);
            free(np as *mut libc::c_void);
        }
        20 => {
            node_free(np);
            free(np as *mut libc::c_void);
        }
        15 => {
            free((*(np as *mut node_ident)).name as *mut libc::c_void);
            free(np as *mut libc::c_void);
        }
        27 => {
            node_array_free(np as *mut node_array);
        }
        0 | 1 => {
            free(np as *mut libc::c_void);
        }
        5 => return,
        3 => {
            free((*(np as *mut node_str)).value as *mut libc::c_void);
            free(np as *mut libc::c_void);
        }
        _ => {}
    };
}
pub unsafe extern "C" fn node_parse_free(mut p: *mut parser_state) {
    node_free((*p).lval as *mut node);
}
