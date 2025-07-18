use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strm_var_def(
        _: *mut strm_state,
        _: *const libc::c_char,
        _: strm_value,
    ) -> libc::c_int;
    fn strm_loop() -> libc::c_int;
    fn strm_ary_struct(_: strm_array) -> *mut strm_array_0;
    fn strm_ary_new(_: *const strm_value, _: strm_int) -> strm_array;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strm_str_new(_: *const libc::c_char, _: strm_int) -> strm_string;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn putchar(__c: libc::c_int) -> libc::c_int;
    fn node_parse_init(_: *mut parser_state);
    fn node_parse_free(_: *mut parser_state);
    fn node_parse_file(_: *mut parser_state, _: *const libc::c_char) -> libc::c_int;
    fn node_parse_input(
        _: *mut parser_state,
        in_0: *mut FILE,
        _: *const libc::c_char,
    ) -> libc::c_int;
    fn node_parse_string(_: *mut parser_state, _: *const libc::c_char) -> libc::c_int;
    fn node_run(_: *mut parser_state) -> libc::c_int;
    fn node_stop();
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
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
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type strm_value = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_state {
    pub env: *mut libc::c_void,
    pub prev: *mut strm_state,
    pub flags: uint32_t,
}
pub type strm_int = int32_t;
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
pub struct node_str {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub value: node_string_0,
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
pub struct node_emit {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub emit: *mut node,
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
pub struct node_ident {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub name: node_string_0,
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
pub struct node_call {
    pub type_0: node_type,
    pub fname: *const libc::c_char,
    pub lineno: libc::c_int,
    pub ident: node_string_0,
    pub args: *mut node,
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
unsafe extern "C" fn fprint_str(mut str: node_string_0, mut f: *mut FILE) {
    fprintf(
        f,
        b"%.*s\0" as *const u8 as *const libc::c_char,
        (*str).len,
        ((*str).buf).as_mut_ptr(),
    );
}
unsafe extern "C" fn print_str(mut name: node_string_0) {
    fprint_str(name, stdout);
    fputs(b"\n\0" as *const u8 as *const libc::c_char, stdout);
}
unsafe extern "C" fn print_id(mut pre: *const libc::c_char, mut name: node_string_0) {
    fputs(pre, stdout);
    print_str(name);
}
unsafe extern "C" fn print_quoted_id(
    mut pre: *const libc::c_char,
    mut name: node_string_0,
) {
    fputs(pre, stdout);
    fputs(b"\"\0" as *const u8 as *const libc::c_char, stdout);
    fprint_str(name, stdout);
    fputs(b"\"\n\0" as *const u8 as *const libc::c_char, stdout);
}
unsafe extern "C" fn dump_node(mut np: *mut node, mut indent: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < indent {
        putchar(' ' as i32);
        i += 1;
        i;
    }
    if np.is_null() {
        printf(b"NIL\n\0" as *const u8 as *const libc::c_char);
        return;
    }
    match (*np).type_0 as libc::c_uint {
        6 => {
            let mut args: *mut node_args = np as *mut node_args;
            printf(b"ARGS(%d):\n\0" as *const u8 as *const libc::c_char, (*args).len);
            i = 0 as libc::c_int;
            while i < (*args).len {
                let mut j: libc::c_int = 0;
                let mut s: node_string_0 = *((*args).data).offset(i as isize);
                j = 0 as libc::c_int;
                while j < indent + 1 as libc::c_int {
                    putchar(' ' as i32);
                    j += 1;
                    j;
                }
                print_str(s);
                i += 1;
                i;
            }
        }
        17 => {
            printf(b"IF:\n\0" as *const u8 as *const libc::c_char);
            dump_node((*(np as *mut node_if)).cond, indent + 1 as libc::c_int);
            i = 0 as libc::c_int;
            while i < indent {
                putchar(' ' as i32);
                i += 1;
                i;
            }
            printf(b"THEN:\n\0" as *const u8 as *const libc::c_char);
            dump_node((*(np as *mut node_if)).then, indent + 1 as libc::c_int);
            let mut opt_else: *mut node = (*(np as *mut node_if)).opt_else;
            if !opt_else.is_null() {
                i = 0 as libc::c_int;
                while i < indent {
                    putchar(' ' as i32);
                    i += 1;
                    i;
                }
                printf(b"ELSE:\n\0" as *const u8 as *const libc::c_char);
                dump_node(opt_else, indent + 1 as libc::c_int);
            }
        }
        18 => {
            printf(b"EMIT:\n\0" as *const u8 as *const libc::c_char);
            dump_node((*(np as *mut node_emit)).emit, indent + 1 as libc::c_int);
        }
        23 => {
            printf(b"OP:\n\0" as *const u8 as *const libc::c_char);
            i = 0 as libc::c_int;
            while i < indent + 1 as libc::c_int {
                putchar(' ' as i32);
                i += 1;
                i;
            }
            print_id(
                b"op: \0" as *const u8 as *const libc::c_char,
                (*(np as *mut node_op)).op,
            );
            dump_node((*(np as *mut node_op)).lhs, indent + 1 as libc::c_int);
            dump_node((*(np as *mut node_op)).rhs, indent + 1 as libc::c_int);
        }
        9 => {
            printf(b"LAMBDA:\n\0" as *const u8 as *const libc::c_char);
            dump_node((*(np as *mut node_lambda)).args, indent + 1 as libc::c_int);
            dump_node((*(np as *mut node_lambda)).body, indent + 1 as libc::c_int);
        }
        10 => {
            printf(b"PLAMBDA:\n\0" as *const u8 as *const libc::c_char);
            dump_node((*(np as *mut node_plambda)).pat, indent + 1 as libc::c_int);
            if !((*(np as *mut node_plambda)).cond).is_null() {
                i = 0 as libc::c_int;
                while i < indent + 2 as libc::c_int {
                    putchar(' ' as i32);
                    i += 1;
                    i;
                }
                printf(b"IF:\n\0" as *const u8 as *const libc::c_char);
                dump_node((*(np as *mut node_plambda)).cond, indent + 2 as libc::c_int);
            }
            dump_node((*(np as *mut node_plambda)).body, indent + 1 as libc::c_int);
            if !((*(np as *mut node_plambda)).next).is_null() {
                dump_node((*(np as *mut node_plambda)).next, indent);
            }
        }
        11 => {
            printf(b"PARRAY:\n\0" as *const u8 as *const libc::c_char);
            let mut ary: *mut node_nodes = np as *mut node_nodes;
            i = 0 as libc::c_int;
            while i < (*ary).len {
                dump_node(*((*ary).data).offset(i as isize), indent + 1 as libc::c_int);
                i += 1;
                i;
            }
        }
        12 => {
            printf(b"PSTRUCT:\n\0" as *const u8 as *const libc::c_char);
            let mut ary_0: *mut node_nodes = np as *mut node_nodes;
            i = 0 as libc::c_int;
            while i < (*ary_0).len {
                dump_node(
                    *((*ary_0).data).offset(i as isize),
                    indent + 1 as libc::c_int,
                );
                i += 1;
                i;
            }
        }
        13 => {
            printf(b"PSPLAT:\n\0" as *const u8 as *const libc::c_char);
            let mut cons: *mut node_psplat = np as *mut node_psplat;
            dump_node((*cons).head, indent + 1 as libc::c_int);
            i = 0 as libc::c_int;
            while i < indent + 1 as libc::c_int {
                putchar(' ' as i32);
                i += 1;
                i;
            }
            printf(b"REST:\n\0" as *const u8 as *const libc::c_char);
            dump_node((*cons).mid, indent + 2 as libc::c_int);
            i = 0 as libc::c_int;
            while i < indent + 1 as libc::c_int {
                putchar(' ' as i32);
                i += 1;
                i;
            }
            printf(b"TAIL:\n\0" as *const u8 as *const libc::c_char);
            dump_node((*cons).tail, indent + 2 as libc::c_int);
        }
        14 => {
            printf(b"SPLAT:\n\0" as *const u8 as *const libc::c_char);
            let mut splat: *mut node_splat = np as *mut node_splat;
            dump_node((*splat).node, indent + 1 as libc::c_int);
        }
        24 => {
            printf(b"CALL:\n\0" as *const u8 as *const libc::c_char);
            i = 0 as libc::c_int;
            while i < indent + 2 as libc::c_int {
                putchar(' ' as i32);
                i += 1;
                i;
            }
            let mut s_0: node_string_0 = (*(np as *mut node_call)).ident;
            print_str(s_0);
            dump_node((*(np as *mut node_call)).args, indent + 2 as libc::c_int);
        }
        25 => {
            printf(b"FCALL:\n\0" as *const u8 as *const libc::c_char);
            i = 0 as libc::c_int;
            while i < indent + 1 as libc::c_int {
                putchar(' ' as i32);
                i += 1;
                i;
            }
            printf(b"FUNC:\n\0" as *const u8 as *const libc::c_char);
            dump_node((*(np as *mut node_fcall)).func, indent + 2 as libc::c_int);
            i = 0 as libc::c_int;
            while i < indent + 1 as libc::c_int {
                putchar(' ' as i32);
                i += 1;
                i;
            }
            printf(b"ARGS:\n\0" as *const u8 as *const libc::c_char);
            dump_node((*(np as *mut node_fcall)).args, indent + 2 as libc::c_int);
        }
        26 => {
            printf(b"GENFUNC: \0" as *const u8 as *const libc::c_char);
            let mut s_1: node_string_0 = (*(np as *mut node_genfunc)).id;
            print_str(s_1);
        }
        20 => {
            printf(b"RETURN:\n\0" as *const u8 as *const libc::c_char);
            dump_node((*(np as *mut node_return)).rv, indent + 1 as libc::c_int);
        }
        16 => {
            print_id(
                b"LET: \0" as *const u8 as *const libc::c_char,
                (*(np as *mut node_let)).lhs,
            );
            dump_node((*(np as *mut node_let)).rhs, indent + 1 as libc::c_int);
        }
        15 => {
            print_id(
                b"IDENT: \0" as *const u8 as *const libc::c_char,
                (*(np as *mut node_ident)).name,
            );
        }
        27 => {
            printf(b"ARRAY:\n\0" as *const u8 as *const libc::c_char);
            let mut j_0: libc::c_int = 0;
            let mut ary_1: *mut node_array = np as *mut node_array;
            if !((*ary_1).headers).is_null() {
                let mut h: *mut node_string_0 = (*ary_1).headers;
                i = 0 as libc::c_int;
                while i < (*ary_1).len {
                    j_0 = 0 as libc::c_int;
                    while j_0 < indent + 1 as libc::c_int {
                        putchar(' ' as i32);
                        j_0 += 1;
                        j_0;
                    }
                    print_quoted_id(
                        b"key: \0" as *const u8 as *const libc::c_char,
                        *h.offset(i as isize),
                    );
                    dump_node(
                        *((*ary_1).data).offset(i as isize),
                        indent + 1 as libc::c_int,
                    );
                    i += 1;
                    i;
                }
            } else {
                i = 0 as libc::c_int;
                while i < (*ary_1).len {
                    dump_node(
                        *((*ary_1).data).offset(i as isize),
                        indent + 1 as libc::c_int,
                    );
                    i += 1;
                    i;
                }
            }
            if !((*ary_1).ns).is_null() {
                let mut ns: node_string_0 = (*ary_1).ns;
                j_0 = 0 as libc::c_int;
                while j_0 < indent + 1 as libc::c_int {
                    putchar(' ' as i32);
                    j_0 += 1;
                    j_0;
                }
                print_quoted_id(b"class: \0" as *const u8 as *const libc::c_char, ns);
            }
        }
        28 => {
            printf(b"NODES:\n\0" as *const u8 as *const libc::c_char);
            let mut ary_2: *mut node_nodes = np as *mut node_nodes;
            i = 0 as libc::c_int;
            while i < (*ary_2).len {
                dump_node(
                    *((*ary_2).data).offset(i as isize),
                    indent + 1 as libc::c_int,
                );
                i += 1;
                i;
            }
        }
        30 => {
            print_id(
                b"IMPORT: \0" as *const u8 as *const libc::c_char,
                (*(np as *mut node_import)).name,
            );
        }
        29 => {
            print_id(
                b"NAMESPACE: \0" as *const u8 as *const libc::c_char,
                (*(np as *mut node_ns)).name,
            );
            dump_node((*(np as *mut node_ns)).body, indent + 1 as libc::c_int);
        }
        0 => {
            printf(
                b"VALUE(NUMBER): %d\n\0" as *const u8 as *const libc::c_char,
                (*(np as *mut node_int)).value,
            );
        }
        1 => {
            printf(
                b"VALUE(NUMBER): %f\n\0" as *const u8 as *const libc::c_char,
                (*(np as *mut node_float)).value,
            );
        }
        2 => {
            let mut nt: *mut node_time = np as *mut node_time;
            printf(
                b"VALUE(TIME): %ld.%ld(%+05d)\n\0" as *const u8 as *const libc::c_char,
                (*nt).sec,
                (*nt).usec,
                (*nt).utc_offset,
            );
        }
        5 => {
            printf(
                b"VALUE(BOOL): %s\n\0" as *const u8 as *const libc::c_char,
                if (*(np as *mut node_bool)).value != 0 {
                    b"true\0" as *const u8 as *const libc::c_char
                } else {
                    b"false\0" as *const u8 as *const libc::c_char
                },
            );
        }
        3 => {
            print_quoted_id(
                b"VALUE(STRING): \0" as *const u8 as *const libc::c_char,
                (*(np as *mut node_str)).value,
            );
        }
        4 => {
            printf(b"VALUE(NIL): nil\n\0" as *const u8 as *const libc::c_char);
        }
        _ => {
            printf(
                b"UNKNOWN(%d)\n\0" as *const u8 as *const libc::c_char,
                (*np).type_0 as libc::c_uint,
            );
        }
    };
}
pub static mut strm_option_verbose: libc::c_int = 0 as libc::c_int;
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *const libc::c_char,
) -> libc::c_int {
    let mut prog: *const libc::c_char = *argv.offset(0 as libc::c_int as isize);
    let mut e_prog: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut verbose: libc::c_int = 0 as libc::c_int;
    let mut check: libc::c_int = 0 as libc::c_int;
    let mut state: parser_state = parser_state {
        nerr: 0,
        lval: 0 as *mut libc::c_void,
        fname: 0 as *const libc::c_char,
        lineno: 0,
        tline: 0,
    };
    while argc > 1 as libc::c_int
        && *(*argv.offset(1 as libc::c_int as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int == '-' as i32
    {
        let mut current_block_13: u64;
        let mut s: *const libc::c_char = (*argv.offset(1 as libc::c_int as isize))
            .offset(1 as libc::c_int as isize);
        while *s != 0 {
            match *s as libc::c_int {
                118 => {
                    verbose = 1 as libc::c_int;
                    current_block_13 = 11093656008690942434;
                }
                119 => {
                    current_block_13 = 11093656008690942434;
                }
                99 => {
                    check = 1 as libc::c_int;
                    current_block_13 = 17407779659766490442;
                }
                101 => {
                    if *s.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
                    {
                        e_prog = *argv.offset(2 as libc::c_int as isize);
                        argc -= 1;
                        argc;
                        argv = argv.offset(1);
                        argv;
                    } else {
                        e_prog = &*s.offset(1 as libc::c_int as isize)
                            as *const libc::c_char;
                    }
                    break;
                }
                _ => {
                    fprintf(
                        stderr,
                        b"%s: unknown option -%c\n\0" as *const u8
                            as *const libc::c_char,
                        prog,
                        *s as libc::c_int,
                    );
                    current_block_13 = 17407779659766490442;
                }
            }
            match current_block_13 {
                11093656008690942434 => {
                    strm_option_verbose = 1 as libc::c_int;
                }
                _ => {}
            }
            s = s.offset(1);
            s;
        }
        argc -= 1;
        argc;
        argv = argv.offset(1);
        argv;
    }
    node_parse_init(&mut state);
    if !e_prog.is_null() {
        n += node_parse_string(&mut state, e_prog);
    } else if argc == 1 as libc::c_int {
        n = node_parse_input(
            &mut state,
            stdin,
            b"stdin\0" as *const u8 as *const libc::c_char,
        );
    } else {
        i = 1 as libc::c_int;
        while i < argc {
            n += node_parse_file(&mut state, *argv.offset(i as isize));
            i += 1;
            i;
        }
    }
    if n == 0 as libc::c_int {
        if verbose != 0 {
            dump_node(state.lval as *mut node, 0 as libc::c_int);
        }
        if check != 0 {
            puts(b"Syntax OK\0" as *const u8 as *const libc::c_char);
        } else {
            let mut av: strm_array = strm_ary_new(0 as *const strm_value, argc);
            let mut buf: *mut strm_value = (*strm_ary_struct(av)).ptr;
            let mut i_0: libc::c_int = 0;
            i_0 = 0 as libc::c_int;
            while i_0 < argc {
                *buf
                    .offset(
                        i_0 as isize,
                    ) = strm_str_new(
                    *argv.offset(i_0 as isize),
                    strlen(*argv.offset(i_0 as isize)) as strm_int,
                );
                i_0 += 1;
                i_0;
            }
            strm_var_def(
                0 as *mut strm_state,
                b"ARGV\0" as *const u8 as *const libc::c_char,
                av,
            );
            node_run(&mut state);
            strm_loop();
            node_stop();
        }
    } else if check != 0 {
        puts(b"Syntax NG\0" as *const u8 as *const libc::c_char);
    }
    node_parse_free(&mut state);
    return if n > 0 as libc::c_int { 1 as libc::c_int } else { 0 as libc::c_int };
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *const libc::c_char,
            ) as i32,
        )
    }
}
