use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type bc_struct;
    pub type dc_string;
    pub type dc_array;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    static mut progname: *const libc::c_char;
    fn dc_malloc(_: size_t) -> *mut libc::c_void;
    fn dc_array_free(_: *mut dc_array);
    fn dc_free_num(_: *mut dc_num);
    fn dc_free_str(_: *mut dc_str);
    fn dc_garbage(_: *const libc::c_char, _: libc::c_int);
    fn dc_print(_: dc_data, _: libc::c_int, _: dc_newline, _: dc_discard);
    fn dc_show_id(_: *mut FILE, _: libc::c_int, _: *const libc::c_char);
    fn dc_compare(_: dc_num, _: dc_num) -> libc::c_int;
    fn dc_numlen(_: dc_num) -> libc::c_int;
    fn dc_strlen(_: dc_str) -> size_t;
    fn dc_dup(_: dc_data) -> dc_data;
    fn dc_int2data(_: libc::c_int) -> dc_data;
}
pub type size_t = libc::c_ulong;
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
pub type dc_discard = libc::c_uint;
pub const DC_KEEP: dc_discard = 1;
pub const DC_TOSS: dc_discard = 0;
pub type dc_newline = libc::c_uint;
pub const DC_WITHNL: dc_newline = 1;
pub const DC_NONL: dc_newline = 0;
pub type dc_value_type = libc::c_uint;
pub const DC_STRING: dc_value_type = 2;
pub const DC_NUMBER: dc_value_type = 1;
pub const DC_UNINITIALIZED: dc_value_type = 0;
pub type dc_num = *mut bc_struct;
pub type dc_str = *mut dc_string;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dc_data {
    pub dc_type: dc_value_type,
    pub v: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub number: dc_num,
    pub string: dc_str,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dc_list {
    pub value: dc_data,
    pub array: *mut dc_array,
    pub link: *mut dc_list,
}
pub type dc_listp = *mut dc_list;
static mut dc_stack: *mut dc_list = 0 as *const dc_list as *mut dc_list;
static mut dc_register: [dc_listp; 256] = [0 as *const dc_list as *mut dc_list; 256];
unsafe extern "C" fn dc_alloc() -> *mut dc_list {
    let mut result: *mut dc_list = 0 as *mut dc_list;
    result = dc_malloc(::std::mem::size_of::<dc_list>() as libc::c_ulong)
        as *mut dc_list;
    (*result).value.dc_type = DC_UNINITIALIZED;
    (*result).array = 0 as *mut dc_array;
    (*result).link = 0 as *mut dc_list;
    return result;
}
pub unsafe extern "C" fn dc_binop(
    mut op: Option::<
        unsafe extern "C" fn(dc_num, dc_num, libc::c_int, *mut dc_num) -> libc::c_int,
    >,
    mut kscale: libc::c_int,
) {
    let mut a: dc_data = dc_data {
        dc_type: DC_UNINITIALIZED,
        v: C2RustUnnamed {
            number: 0 as *mut bc_struct,
        },
    };
    let mut b: dc_data = dc_data {
        dc_type: DC_UNINITIALIZED,
        v: C2RustUnnamed {
            number: 0 as *mut bc_struct,
        },
    };
    let mut r: dc_data = dc_data {
        dc_type: DC_UNINITIALIZED,
        v: C2RustUnnamed {
            number: 0 as *mut bc_struct,
        },
    };
    if dc_stack.is_null() || ((*dc_stack).link).is_null() {
        fprintf(
            stderr,
            b"%s: stack empty\n\0" as *const u8 as *const libc::c_char,
            progname,
        );
        return;
    }
    if (*dc_stack).value.dc_type as libc::c_uint
        != DC_NUMBER as libc::c_int as libc::c_uint
        || (*(*dc_stack).link).value.dc_type as libc::c_uint
            != DC_NUMBER as libc::c_int as libc::c_uint
    {
        fprintf(
            stderr,
            b"%s: non-numeric value\n\0" as *const u8 as *const libc::c_char,
            progname,
        );
        return;
    }
    dc_pop(&mut b);
    dc_pop(&mut a);
    if (Some(op.unwrap())).unwrap()(a.v.number, b.v.number, kscale, &mut r.v.number)
        == 0 as libc::c_int
    {
        r.dc_type = DC_NUMBER;
        dc_push(r);
        dc_free_num(&mut a.v.number);
        dc_free_num(&mut b.v.number);
    } else {
        dc_push(a);
        dc_push(b);
    };
}
pub unsafe extern "C" fn dc_binop2(
    mut op: Option::<
        unsafe extern "C" fn(
            dc_num,
            dc_num,
            libc::c_int,
            *mut dc_num,
            *mut dc_num,
        ) -> libc::c_int,
    >,
    mut kscale: libc::c_int,
) {
    let mut a: dc_data = dc_data {
        dc_type: DC_UNINITIALIZED,
        v: C2RustUnnamed {
            number: 0 as *mut bc_struct,
        },
    };
    let mut b: dc_data = dc_data {
        dc_type: DC_UNINITIALIZED,
        v: C2RustUnnamed {
            number: 0 as *mut bc_struct,
        },
    };
    let mut r1: dc_data = dc_data {
        dc_type: DC_UNINITIALIZED,
        v: C2RustUnnamed {
            number: 0 as *mut bc_struct,
        },
    };
    let mut r2: dc_data = dc_data {
        dc_type: DC_UNINITIALIZED,
        v: C2RustUnnamed {
            number: 0 as *mut bc_struct,
        },
    };
    if dc_stack.is_null() || ((*dc_stack).link).is_null() {
        fprintf(
            stderr,
            b"%s: stack empty\n\0" as *const u8 as *const libc::c_char,
            progname,
        );
        return;
    }
    if (*dc_stack).value.dc_type as libc::c_uint
        != DC_NUMBER as libc::c_int as libc::c_uint
        || (*(*dc_stack).link).value.dc_type as libc::c_uint
            != DC_NUMBER as libc::c_int as libc::c_uint
    {
        fprintf(
            stderr,
            b"%s: non-numeric value\n\0" as *const u8 as *const libc::c_char,
            progname,
        );
        return;
    }
    dc_pop(&mut b);
    dc_pop(&mut a);
    if (Some(op.unwrap()))
        .unwrap()(a.v.number, b.v.number, kscale, &mut r1.v.number, &mut r2.v.number)
        == 0 as libc::c_int
    {
        r1.dc_type = DC_NUMBER;
        dc_push(r1);
        r2.dc_type = DC_NUMBER;
        dc_push(r2);
        dc_free_num(&mut a.v.number);
        dc_free_num(&mut b.v.number);
    } else {
        dc_push(a);
        dc_push(b);
    };
}
pub unsafe extern "C" fn dc_cmpop() -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut a: dc_data = dc_data {
        dc_type: DC_UNINITIALIZED,
        v: C2RustUnnamed {
            number: 0 as *mut bc_struct,
        },
    };
    let mut b: dc_data = dc_data {
        dc_type: DC_UNINITIALIZED,
        v: C2RustUnnamed {
            number: 0 as *mut bc_struct,
        },
    };
    if dc_stack.is_null() || ((*dc_stack).link).is_null() {
        fprintf(
            stderr,
            b"%s: stack empty\n\0" as *const u8 as *const libc::c_char,
            progname,
        );
        return 0 as libc::c_int;
    }
    if (*dc_stack).value.dc_type as libc::c_uint
        != DC_NUMBER as libc::c_int as libc::c_uint
        || (*(*dc_stack).link).value.dc_type as libc::c_uint
            != DC_NUMBER as libc::c_int as libc::c_uint
    {
        fprintf(
            stderr,
            b"%s: non-numeric value\n\0" as *const u8 as *const libc::c_char,
            progname,
        );
        return 0 as libc::c_int;
    }
    dc_pop(&mut b);
    dc_pop(&mut a);
    result = dc_compare(b.v.number, a.v.number);
    dc_free_num(&mut a.v.number);
    dc_free_num(&mut b.v.number);
    return result;
}
pub unsafe extern "C" fn dc_triop(
    mut op: Option::<
        unsafe extern "C" fn(
            dc_num,
            dc_num,
            dc_num,
            libc::c_int,
            *mut dc_num,
        ) -> libc::c_int,
    >,
    mut kscale: libc::c_int,
) {
    let mut a: dc_data = dc_data {
        dc_type: DC_UNINITIALIZED,
        v: C2RustUnnamed {
            number: 0 as *mut bc_struct,
        },
    };
    let mut b: dc_data = dc_data {
        dc_type: DC_UNINITIALIZED,
        v: C2RustUnnamed {
            number: 0 as *mut bc_struct,
        },
    };
    let mut c: dc_data = dc_data {
        dc_type: DC_UNINITIALIZED,
        v: C2RustUnnamed {
            number: 0 as *mut bc_struct,
        },
    };
    let mut r: dc_data = dc_data {
        dc_type: DC_UNINITIALIZED,
        v: C2RustUnnamed {
            number: 0 as *mut bc_struct,
        },
    };
    if dc_stack.is_null() || ((*dc_stack).link).is_null()
        || ((*(*dc_stack).link).link).is_null()
    {
        fprintf(
            stderr,
            b"%s: stack empty\n\0" as *const u8 as *const libc::c_char,
            progname,
        );
        return;
    }
    if (*dc_stack).value.dc_type as libc::c_uint
        != DC_NUMBER as libc::c_int as libc::c_uint
        || (*(*dc_stack).link).value.dc_type as libc::c_uint
            != DC_NUMBER as libc::c_int as libc::c_uint
        || (*(*(*dc_stack).link).link).value.dc_type as libc::c_uint
            != DC_NUMBER as libc::c_int as libc::c_uint
    {
        fprintf(
            stderr,
            b"%s: non-numeric value\n\0" as *const u8 as *const libc::c_char,
            progname,
        );
        return;
    }
    dc_pop(&mut c);
    dc_pop(&mut b);
    dc_pop(&mut a);
    if (Some(op.unwrap()))
        .unwrap()(a.v.number, b.v.number, c.v.number, kscale, &mut r.v.number)
        == 0 as libc::c_int
    {
        r.dc_type = DC_NUMBER;
        dc_push(r);
        dc_free_num(&mut a.v.number);
        dc_free_num(&mut b.v.number);
        dc_free_num(&mut c.v.number);
    } else {
        dc_push(a);
        dc_push(b);
        dc_push(c);
    };
}
pub unsafe extern "C" fn dc_register_init() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
    {
        dc_register[i as usize] = 0 as dc_listp;
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn dc_clear_stack() {
    let mut n: *mut dc_list = 0 as *mut dc_list;
    let mut t: *mut dc_list = 0 as *mut dc_list;
    n = dc_stack;
    while !n.is_null() {
        t = (*n).link;
        if (*n).value.dc_type as libc::c_uint == DC_NUMBER as libc::c_int as libc::c_uint
        {
            dc_free_num(&mut (*n).value.v.number);
        } else if (*n).value.dc_type as libc::c_uint
            == DC_STRING as libc::c_int as libc::c_uint
        {
            dc_free_str(&mut (*n).value.v.string);
        } else {
            dc_garbage(
                b"in stack\0" as *const u8 as *const libc::c_char,
                -(1 as libc::c_int),
            );
        }
        dc_array_free((*n).array);
        free(n as *mut libc::c_void);
        n = t;
    }
    dc_stack = 0 as *mut dc_list;
}
pub unsafe extern "C" fn dc_push(mut value: dc_data) {
    let mut n: *mut dc_list = dc_alloc();
    if value.dc_type as libc::c_uint != DC_NUMBER as libc::c_int as libc::c_uint
        && value.dc_type as libc::c_uint != DC_STRING as libc::c_int as libc::c_uint
    {
        dc_garbage(
            b"in data being pushed\0" as *const u8 as *const libc::c_char,
            -(1 as libc::c_int),
        );
    }
    (*n).value = value;
    (*n).link = dc_stack;
    dc_stack = n;
}
pub unsafe extern "C" fn dc_register_push(mut stackid: libc::c_int, mut value: dc_data) {
    let mut n: *mut dc_list = dc_alloc();
    stackid = stackid
        & 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
            - 1 as libc::c_int;
    (*n).value = value;
    (*n).link = dc_register[stackid as usize];
    dc_register[stackid as usize] = n;
}
pub unsafe extern "C" fn dc_top_of_stack(mut result: *mut dc_data) -> libc::c_int {
    if dc_stack.is_null() {
        fprintf(
            stderr,
            b"%s: stack empty\n\0" as *const u8 as *const libc::c_char,
            progname,
        );
        return 2 as libc::c_int;
    }
    if (*dc_stack).value.dc_type as libc::c_uint
        != DC_NUMBER as libc::c_int as libc::c_uint
        && (*dc_stack).value.dc_type as libc::c_uint
            != DC_STRING as libc::c_int as libc::c_uint
    {
        dc_garbage(
            b"at top of stack\0" as *const u8 as *const libc::c_char,
            -(1 as libc::c_int),
        );
    }
    *result = (*dc_stack).value;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn dc_register_get(
    mut regid: libc::c_int,
    mut result: *mut dc_data,
) -> libc::c_int {
    let mut r: *mut dc_list = 0 as *mut dc_list;
    regid = regid
        & 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
            - 1 as libc::c_int;
    r = dc_register[regid as usize];
    if r.is_null() {
        *result = dc_int2data(0 as libc::c_int);
    } else if (*r).value.dc_type as libc::c_uint
        == DC_UNINITIALIZED as libc::c_int as libc::c_uint
    {
        fprintf(
            stderr,
            b"%s: BUG: register \0" as *const u8 as *const libc::c_char,
            progname,
        );
        dc_show_id(
            stderr,
            regid,
            b" exists but is uninitialized?\n\0" as *const u8 as *const libc::c_char,
        );
        return 2 as libc::c_int;
    } else {
        *result = dc_dup((*r).value);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn dc_register_set(mut regid: libc::c_int, mut value: dc_data) {
    let mut r: *mut dc_list = 0 as *mut dc_list;
    regid = regid
        & 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
            - 1 as libc::c_int;
    r = dc_register[regid as usize];
    if r.is_null() {
        dc_register[regid as usize] = dc_alloc();
    } else if (*r).value.dc_type as libc::c_uint
        == DC_NUMBER as libc::c_int as libc::c_uint
    {
        dc_free_num(&mut (*r).value.v.number);
    } else if (*r).value.dc_type as libc::c_uint
        == DC_STRING as libc::c_int as libc::c_uint
    {
        dc_free_str(&mut (*r).value.v.string);
    } else if !((*r).value.dc_type as libc::c_uint
        == DC_UNINITIALIZED as libc::c_int as libc::c_uint)
    {
        dc_garbage(b"\0" as *const u8 as *const libc::c_char, regid);
    }
    (*dc_register[regid as usize]).value = value;
}
pub unsafe extern "C" fn dc_pop(mut result: *mut dc_data) -> libc::c_int {
    let mut r: *mut dc_list = 0 as *mut dc_list;
    r = dc_stack;
    if r.is_null()
        || (*r).value.dc_type as libc::c_uint
            == DC_UNINITIALIZED as libc::c_int as libc::c_uint
    {
        fprintf(
            stderr,
            b"%s: stack empty\n\0" as *const u8 as *const libc::c_char,
            progname,
        );
        return 2 as libc::c_int;
    }
    if (*r).value.dc_type as libc::c_uint != DC_NUMBER as libc::c_int as libc::c_uint
        && (*r).value.dc_type as libc::c_uint != DC_STRING as libc::c_int as libc::c_uint
    {
        dc_garbage(
            b"at top of stack\0" as *const u8 as *const libc::c_char,
            -(1 as libc::c_int),
        );
    }
    *result = (*r).value;
    dc_stack = (*r).link;
    dc_array_free((*r).array);
    free(r as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn dc_register_pop(
    mut stackid: libc::c_int,
    mut result: *mut dc_data,
) -> libc::c_int {
    let mut r: *mut dc_list = 0 as *mut dc_list;
    stackid = stackid
        & 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
            - 1 as libc::c_int;
    r = dc_register[stackid as usize];
    if r.is_null()
        || (*r).value.dc_type as libc::c_uint
            == DC_UNINITIALIZED as libc::c_int as libc::c_uint
    {
        fprintf(
            stderr,
            b"%s: stack register \0" as *const u8 as *const libc::c_char,
            progname,
        );
        dc_show_id(
            stderr,
            stackid,
            b" is empty\n\0" as *const u8 as *const libc::c_char,
        );
        return 2 as libc::c_int;
    }
    if (*r).value.dc_type as libc::c_uint != DC_NUMBER as libc::c_int as libc::c_uint
        && (*r).value.dc_type as libc::c_uint != DC_STRING as libc::c_int as libc::c_uint
    {
        dc_garbage(b" stack\0" as *const u8 as *const libc::c_char, stackid);
    }
    *result = (*r).value;
    dc_register[stackid as usize] = (*r).link;
    dc_array_free((*r).array);
    free(r as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn dc_stack_rotate(mut n: libc::c_int) {
    let mut p: *mut dc_list = 0 as *mut dc_list;
    let mut r: *mut dc_list = 0 as *mut dc_list;
    let mut absn: libc::c_int = if n < 0 as libc::c_int { -n } else { n };
    if dc_stack.is_null() || absn < 2 as libc::c_int {
        return;
    }
    r = 0 as *mut dc_list;
    p = dc_stack;
    while !((*p).link).is_null()
        && {
            absn -= 1;
            absn > 0 as libc::c_int
        }
    {
        r = p;
        p = (*p).link;
    }
    if r.is_null() {
        return;
    }
    if n > 0 as libc::c_int {
        (*r).link = (*p).link;
        (*p).link = dc_stack;
        dc_stack = p;
    } else {
        let mut new_tos: *mut dc_list = (*dc_stack).link;
        (*dc_stack).link = (*p).link;
        (*p).link = dc_stack;
        dc_stack = new_tos;
    };
}
pub unsafe extern "C" fn dc_tell_stackdepth() -> libc::c_int {
    let mut n: *mut dc_list = 0 as *mut dc_list;
    let mut depth: libc::c_int = 0 as libc::c_int;
    n = dc_stack;
    while !n.is_null() {
        depth += 1;
        depth;
        n = (*n).link;
    }
    return depth;
}
pub unsafe extern "C" fn dc_tell_length(
    mut value: dc_data,
    mut discard_p: dc_discard,
) -> libc::c_int {
    let mut length: libc::c_int = 0;
    if value.dc_type as libc::c_uint == DC_NUMBER as libc::c_int as libc::c_uint {
        length = dc_numlen(value.v.number);
        if discard_p as libc::c_uint == DC_TOSS as libc::c_int as libc::c_uint {
            dc_free_num(&mut value.v.number);
        }
    } else if value.dc_type as libc::c_uint == DC_STRING as libc::c_int as libc::c_uint {
        length = dc_strlen(value.v.string) as libc::c_int;
        if discard_p as libc::c_uint == DC_TOSS as libc::c_int as libc::c_uint {
            dc_free_str(&mut value.v.string);
        }
    } else {
        dc_garbage(
            b"in tell_length\0" as *const u8 as *const libc::c_char,
            -(1 as libc::c_int),
        );
        length = 0 as libc::c_int;
    }
    return length;
}
pub unsafe extern "C" fn dc_printall(mut obase: libc::c_int) {
    let mut n: *mut dc_list = 0 as *mut dc_list;
    n = dc_stack;
    while !n.is_null() {
        dc_print((*n).value, obase, DC_WITHNL, DC_KEEP);
        n = (*n).link;
    }
}
pub unsafe extern "C" fn dc_get_stacked_array(
    mut array_id: libc::c_int,
) -> *mut dc_array {
    let mut r: *mut dc_list = dc_register[(array_id
        & 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
            - 1 as libc::c_int) as usize];
    return if r.is_null() { 0 as *mut dc_array } else { (*r).array };
}
pub unsafe extern "C" fn dc_set_stacked_array(
    mut array_id: libc::c_int,
    mut new_head: *mut dc_array,
) {
    let mut r: *mut dc_list = 0 as *mut dc_list;
    array_id = array_id
        & 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
            - 1 as libc::c_int;
    r = dc_register[array_id as usize];
    if r.is_null() {
        dc_register[array_id as usize] = dc_alloc();
        r = dc_register[array_id as usize];
    }
    (*r).array = new_head;
}
