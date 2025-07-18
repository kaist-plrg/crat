use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type strm_queue;
    pub type node_error;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    static mut stdout: *mut FILE;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn strm_str_eq(a: strm_string, b: strm_string) -> libc::c_int;
    fn strm_ary_eq(a: strm_array, b: strm_array) -> libc::c_int;
    fn strm_array_p(_: strm_value) -> libc::c_int;
    fn strm_string_p(_: strm_value) -> libc::c_int;
    fn strm_str_new(_: *const libc::c_char, _: strm_int) -> strm_string;
    fn strm_str_static(_: *const libc::c_char, _: strm_int) -> strm_string;
    fn strm_strp_ptr(_: *mut strm_string) -> *const libc::c_char;
    fn strm_str_cstr(_: strm_string, buf: *mut libc::c_char) -> *const libc::c_char;
    fn strm_str_len(_: strm_string) -> strm_int;
    fn strm_str_intern_static(p: *const libc::c_char, len: strm_int) -> strm_string;
    fn strm_ary_struct(_: strm_array) -> *mut strm_array_0;
    fn strm_ns_name(_: *mut strm_state) -> strm_string;
    fn strm_funcall(
        _: *mut strm_stream,
        _: strm_value,
        _: libc::c_int,
        _: *mut strm_value,
        _: *mut strm_value,
    ) -> libc::c_int;
    static mut strm_ns_number: *mut strm_state;
    static mut strm_ns_string: *mut strm_state;
    static mut strm_ns_array: *mut strm_state;
    fn strm_var_get(
        _: *mut strm_state,
        _: strm_string,
        _: *mut strm_value,
    ) -> libc::c_int;
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __intptr_t = libc::c_long;
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
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub f: libc::c_double,
    pub i: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub f: libc::c_double,
    pub i: uint64_t,
}
pub type strm_string = uint64_t;
pub type strm_array = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct object {
    pub type_0: strm_ptr_type,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_genfunc {
    pub type_0: strm_ptr_type,
    pub state: *mut strm_state,
    pub id: strm_string,
}
pub type strm_io = *mut strm_io_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_io_0 {
    pub type_0: strm_ptr_type,
    pub fd: libc::c_int,
    pub mode: libc::c_int,
    pub read_stream: *mut strm_stream,
    pub write_stream: *mut strm_stream,
}
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
pub struct strm_misc {
    pub type_0: strm_ptr_type,
    pub ns: *mut strm_state,
}
pub unsafe extern "C" fn strm_ptr_value(mut p: *mut libc::c_void) -> strm_value {
    return STRM_TAG_PTR as libc::c_ulong
        | p as intptr_t as strm_value
            & !(((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t)
                << 48 as libc::c_int);
}
pub unsafe extern "C" fn strm_cfunc_value(mut f: strm_cfunc) -> strm_value {
    return STRM_TAG_CFUNC as libc::c_ulong
        | ::std::mem::transmute::<strm_cfunc, intptr_t>(f) as strm_value
            & !(((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t)
                << 48 as libc::c_int);
}
pub unsafe extern "C" fn strm_bool_value(mut i: libc::c_int) -> strm_value {
    return STRM_TAG_BOOL as libc::c_ulong | (i != 0) as libc::c_int as libc::c_ulong;
}
pub unsafe extern "C" fn strm_int_value(mut i: int32_t) -> strm_value {
    return STRM_TAG_INT as libc::c_ulong
        | i as uint64_t
            & !(((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t)
                << 48 as libc::c_int);
}
pub unsafe extern "C" fn strm_float_value(mut f: libc::c_double) -> strm_value {
    let mut u: C2RustUnnamed_0 = C2RustUnnamed_0 { f: 0. };
    if f.is_nan() as i32 != 0 {
        return STRM_TAG_NAN as libc::c_ulong;
    }
    u.f = f;
    return u.i;
}
pub unsafe extern "C" fn strm_foreign_value(mut p: *mut libc::c_void) -> strm_value {
    return STRM_TAG_FOREIGN as libc::c_ulong
        | p as intptr_t as strm_value
            & !(((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t)
                << 48 as libc::c_int);
}
unsafe extern "C" fn strm_ptr(mut v: strm_value) -> *mut libc::c_void {
    return (v
        & !(((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t)
            << 48 as libc::c_int)) as intptr_t as *mut libc::c_void;
}
unsafe extern "C" fn strm_ptr_type(mut p: *mut libc::c_void) -> strm_ptr_type {
    let mut obj: *mut object = p as *mut object;
    return (*obj).type_0;
}
pub unsafe extern "C" fn strm_value_ptr(
    mut v: strm_value,
    mut e: strm_ptr_type,
) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    if v
        & ((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t) << 48 as libc::c_int
        == STRM_TAG_PTR as libc::c_ulong
    {} else {
        __assert_fail(
            b"strm_value_tag(v) == STRM_TAG_PTR\0" as *const u8 as *const libc::c_char,
            b"value.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"void *strm_value_ptr(strm_value, enum strm_ptr_type)\0"))
                .as_ptr(),
        );
    }
    'c_4917: {
        if v
            & ((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t)
                << 48 as libc::c_int == STRM_TAG_PTR as libc::c_ulong
        {} else {
            __assert_fail(
                b"strm_value_tag(v) == STRM_TAG_PTR\0" as *const u8
                    as *const libc::c_char,
                b"value.c\0" as *const u8 as *const libc::c_char,
                72 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 53],
                    &[libc::c_char; 53],
                >(b"void *strm_value_ptr(strm_value, enum strm_ptr_type)\0"))
                    .as_ptr(),
            );
        }
    };
    p = strm_ptr(v);
    if !p.is_null() && strm_ptr_type(p) as libc::c_uint == e as libc::c_uint {} else {
        __assert_fail(
            b"p && strm_ptr_type(p) == e\0" as *const u8 as *const libc::c_char,
            b"value.c\0" as *const u8 as *const libc::c_char,
            74 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"void *strm_value_ptr(strm_value, enum strm_ptr_type)\0"))
                .as_ptr(),
        );
    }
    'c_4854: {
        if !p.is_null() && strm_ptr_type(p) as libc::c_uint == e as libc::c_uint
        {} else {
            __assert_fail(
                b"p && strm_ptr_type(p) == e\0" as *const u8 as *const libc::c_char,
                b"value.c\0" as *const u8 as *const libc::c_char,
                74 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 53],
                    &[libc::c_char; 53],
                >(b"void *strm_value_ptr(strm_value, enum strm_ptr_type)\0"))
                    .as_ptr(),
            );
        }
    };
    return p;
}
pub unsafe extern "C" fn strm_value_foreign(mut v: strm_value) -> *mut libc::c_void {
    if v
        & ((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t) << 48 as libc::c_int
        == STRM_TAG_FOREIGN as libc::c_ulong
    {} else {
        __assert_fail(
            b"strm_value_tag(v) == STRM_TAG_FOREIGN\0" as *const u8
                as *const libc::c_char,
            b"value.c\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"void *strm_value_foreign(strm_value)\0"))
                .as_ptr(),
        );
    }
    'c_5032: {
        if v
            & ((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t)
                << 48 as libc::c_int == STRM_TAG_FOREIGN as libc::c_ulong
        {} else {
            __assert_fail(
                b"strm_value_tag(v) == STRM_TAG_FOREIGN\0" as *const u8
                    as *const libc::c_char,
                b"value.c\0" as *const u8 as *const libc::c_char,
                81 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 37],
                    &[libc::c_char; 37],
                >(b"void *strm_value_foreign(strm_value)\0"))
                    .as_ptr(),
            );
        }
    };
    return strm_ptr(v);
}
pub unsafe extern "C" fn strm_value_bool(mut v: strm_value) -> libc::c_int {
    let mut i: uint64_t = v
        & !(((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t)
            << 48 as libc::c_int);
    if i == 0 as libc::c_int as libc::c_ulong {
        match v
            & ((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t)
                << 48 as libc::c_int
        {
            18442521949058891776 => {}
            18445899648779419648 => {}
            _ => {
                if v
                    & ((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t)
                        << 48 as libc::c_int == STRM_TAG_BOOL as libc::c_ulong
                {} else {
                    __assert_fail(
                        b"strm_value_tag(v) == STRM_TAG_BOOL\0" as *const u8
                            as *const libc::c_char,
                        b"value.c\0" as *const u8 as *const libc::c_char,
                        96 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 32],
                            &[libc::c_char; 32],
                        >(b"int strm_value_bool(strm_value)\0"))
                            .as_ptr(),
                    );
                }
                'c_4242: {
                    if v
                        & ((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t)
                            << 48 as libc::c_int == STRM_TAG_BOOL as libc::c_ulong
                    {} else {
                        __assert_fail(
                            b"strm_value_tag(v) == STRM_TAG_BOOL\0" as *const u8
                                as *const libc::c_char,
                            b"value.c\0" as *const u8 as *const libc::c_char,
                            96 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 32],
                                &[libc::c_char; 32],
                            >(b"int strm_value_bool(strm_value)\0"))
                                .as_ptr(),
                        );
                    }
                };
            }
        }
        return 0 as libc::c_int;
    } else {
        return 1 as libc::c_int
    };
}
pub unsafe extern "C" fn strm_int_p(mut v: strm_value) -> libc::c_int {
    return (v
        & ((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t) << 48 as libc::c_int
        == STRM_TAG_INT as libc::c_ulong) as libc::c_int;
}
#[inline]
unsafe extern "C" fn strm_to_int(mut v: strm_value) -> int32_t {
    return (v
        & !(((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t)
            << 48 as libc::c_int)) as int32_t;
}
pub unsafe extern "C" fn strm_float_p(mut v: strm_value) -> libc::c_int {
    return (v == STRM_TAG_NAN as libc::c_ulong
        || v & STRM_TAG_NAN as libc::c_ulong != STRM_TAG_NAN as libc::c_ulong)
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn strm_to_float(mut v: strm_value) -> libc::c_double {
    let mut u: C2RustUnnamed_1 = C2RustUnnamed_1 { f: 0. };
    u.i = v;
    return u.f;
}
pub unsafe extern "C" fn strm_value_int(mut v: strm_value) -> strm_int {
    match v
        & ((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t) << 48 as libc::c_int
    {
        18442803424035602432 => return strm_to_int(v),
        _ => {
            if strm_float_p(v) != 0 {
                return strm_to_float(v) as strm_int;
            }
            if v
                & ((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t)
                    << 48 as libc::c_int == STRM_TAG_INT as libc::c_ulong
            {} else {
                __assert_fail(
                    b"strm_value_tag(v) == STRM_TAG_INT\0" as *const u8
                        as *const libc::c_char,
                    b"value.c\0" as *const u8 as *const libc::c_char,
                    146 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 36],
                        &[libc::c_char; 36],
                    >(b"strm_int strm_value_int(strm_value)\0"))
                        .as_ptr(),
                );
            }
            'c_4064: {
                if v
                    & ((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t)
                        << 48 as libc::c_int == STRM_TAG_INT as libc::c_ulong
                {} else {
                    __assert_fail(
                        b"strm_value_tag(v) == STRM_TAG_INT\0" as *const u8
                            as *const libc::c_char,
                        b"value.c\0" as *const u8 as *const libc::c_char,
                        146 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 36],
                            &[libc::c_char; 36],
                        >(b"strm_int strm_value_int(strm_value)\0"))
                            .as_ptr(),
                    );
                }
            };
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn strm_value_float(mut v: strm_value) -> libc::c_double {
    if strm_int_p(v) != 0 {
        return strm_to_int(v) as libc::c_double
    } else if strm_float_p(v) != 0 {
        return strm_to_float(v)
    } else {
        if strm_float_p(v) != 0 {} else {
            __assert_fail(
                b"strm_float_p(v)\0" as *const u8 as *const libc::c_char,
                b"value.c\0" as *const u8 as *const libc::c_char,
                164 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"double strm_value_float(strm_value)\0"))
                    .as_ptr(),
            );
        }
        'c_4356: {
            if strm_float_p(v) != 0 {} else {
                __assert_fail(
                    b"strm_float_p(v)\0" as *const u8 as *const libc::c_char,
                    b"value.c\0" as *const u8 as *const libc::c_char,
                    164 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 36],
                        &[libc::c_char; 36],
                    >(b"double strm_value_float(strm_value)\0"))
                        .as_ptr(),
                );
            }
        };
    }
    return 0.0f64;
}
pub unsafe extern "C" fn strm_value_cfunc(mut v: strm_value) -> strm_cfunc {
    if v
        & ((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t) << 48 as libc::c_int
        == STRM_TAG_CFUNC as libc::c_ulong
    {} else {
        __assert_fail(
            b"strm_value_tag(v) == STRM_TAG_CFUNC\0" as *const u8 as *const libc::c_char,
            b"value.c\0" as *const u8 as *const libc::c_char,
            173 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 40],
                &[libc::c_char; 40],
            >(b"strm_cfunc strm_value_cfunc(strm_value)\0"))
                .as_ptr(),
        );
    }
    'c_3986: {
        if v
            & ((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t)
                << 48 as libc::c_int == STRM_TAG_CFUNC as libc::c_ulong
        {} else {
            __assert_fail(
                b"strm_value_tag(v) == STRM_TAG_CFUNC\0" as *const u8
                    as *const libc::c_char,
                b"value.c\0" as *const u8 as *const libc::c_char,
                173 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_char; 40],
                >(b"strm_cfunc strm_value_cfunc(strm_value)\0"))
                    .as_ptr(),
            );
        }
    };
    return ::std::mem::transmute::<
        libc::intptr_t,
        strm_cfunc,
    >(
        (v
            & !(((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t)
                << 48 as libc::c_int)) as intptr_t as libc::intptr_t,
    );
}
pub unsafe extern "C" fn strm_number_p(mut v: strm_value) -> libc::c_int {
    if strm_int_p(v) != 0 || strm_float_p(v) != 0 {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
pub unsafe extern "C" fn strm_bool_p(mut v: strm_value) -> libc::c_int {
    return if v
        & ((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t) << 48 as libc::c_int
        == STRM_TAG_BOOL as libc::c_ulong
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
pub unsafe extern "C" fn strm_nil_p(mut v: strm_value) -> libc::c_int {
    if v
        & ((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t) << 48 as libc::c_int
        != STRM_TAG_PTR as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    return (v
        & !(((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t)
            << 48 as libc::c_int) == 0 as libc::c_int as libc::c_ulong) as libc::c_int;
}
pub unsafe extern "C" fn strm_cfunc_p(mut v: strm_value) -> libc::c_int {
    return (v
        & ((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t) << 48 as libc::c_int
        == STRM_TAG_CFUNC as libc::c_ulong) as libc::c_int;
}
pub unsafe extern "C" fn strm_ptr_tag_p(
    mut v: strm_value,
    mut e: strm_ptr_type,
) -> libc::c_int {
    if v
        & ((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t) << 48 as libc::c_int
        == STRM_TAG_PTR as libc::c_ulong
    {
        let mut p: *mut libc::c_void = strm_ptr(v);
        return (strm_ptr_type(p) as libc::c_uint == e as libc::c_uint) as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn strm_value_eq(
    mut a: strm_value,
    mut b: strm_value,
) -> libc::c_int {
    let mut current_block: u64;
    if a == b {
        return 1 as libc::c_int;
    }
    if !(a
        & ((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t) << 48 as libc::c_int
        != b
            & ((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t)
                << 48 as libc::c_int)
    {
        match a
            & ((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t)
                << 48 as libc::c_int
        {
            18443366373989023744 | 18443647848965734400 => {
                current_block = 6757825451543678428;
                match current_block {
                    10278175316653942280 => {
                        return ((a
                            & !(((0xfff0 as libc::c_int | 0xf as libc::c_int)
                                as uint64_t) << 48 as libc::c_int)) as intptr_t
                            as *mut libc::c_void
                            == (b
                                & !(((0xfff0 as libc::c_int | 0xf as libc::c_int)
                                    as uint64_t) << 48 as libc::c_int)) as intptr_t
                                as *mut libc::c_void) as libc::c_int;
                    }
                    6079862847828553232 => return strm_str_eq(a, b),
                    11913485640103028541 => {
                        return (::std::mem::transmute::<
                            libc::intptr_t,
                            strm_cfunc,
                        >(
                            (a
                                & !(((0xfff0 as libc::c_int | 0xf as libc::c_int)
                                    as uint64_t) << 48 as libc::c_int)) as intptr_t
                                as libc::intptr_t,
                        )
                            == ::std::mem::transmute::<
                                libc::intptr_t,
                                strm_cfunc,
                            >(
                                (b
                                    & !(((0xfff0 as libc::c_int | 0xf as libc::c_int)
                                        as uint64_t) << 48 as libc::c_int)) as intptr_t
                                    as libc::intptr_t,
                            )) as libc::c_int;
                    }
                    _ => return strm_ary_eq(a, b),
                }
            }
            18444773748872577024 | 18445055223849287680 => {
                current_block = 6079862847828553232;
                match current_block {
                    10278175316653942280 => {
                        return ((a
                            & !(((0xfff0 as libc::c_int | 0xf as libc::c_int)
                                as uint64_t) << 48 as libc::c_int)) as intptr_t
                            as *mut libc::c_void
                            == (b
                                & !(((0xfff0 as libc::c_int | 0xf as libc::c_int)
                                    as uint64_t) << 48 as libc::c_int)) as intptr_t
                                as *mut libc::c_void) as libc::c_int;
                    }
                    6079862847828553232 => return strm_str_eq(a, b),
                    11913485640103028541 => {
                        return (::std::mem::transmute::<
                            libc::intptr_t,
                            strm_cfunc,
                        >(
                            (a
                                & !(((0xfff0 as libc::c_int | 0xf as libc::c_int)
                                    as uint64_t) << 48 as libc::c_int)) as intptr_t
                                as libc::intptr_t,
                        )
                            == ::std::mem::transmute::<
                                libc::intptr_t,
                                strm_cfunc,
                            >(
                                (b
                                    & !(((0xfff0 as libc::c_int | 0xf as libc::c_int)
                                        as uint64_t) << 48 as libc::c_int)) as intptr_t
                                    as libc::intptr_t,
                            )) as libc::c_int;
                    }
                    _ => return strm_ary_eq(a, b),
                }
            }
            18445336698825998336 => {
                current_block = 11913485640103028541;
                match current_block {
                    10278175316653942280 => {
                        return ((a
                            & !(((0xfff0 as libc::c_int | 0xf as libc::c_int)
                                as uint64_t) << 48 as libc::c_int)) as intptr_t
                            as *mut libc::c_void
                            == (b
                                & !(((0xfff0 as libc::c_int | 0xf as libc::c_int)
                                    as uint64_t) << 48 as libc::c_int)) as intptr_t
                                as *mut libc::c_void) as libc::c_int;
                    }
                    6079862847828553232 => return strm_str_eq(a, b),
                    11913485640103028541 => {
                        return (::std::mem::transmute::<
                            libc::intptr_t,
                            strm_cfunc,
                        >(
                            (a
                                & !(((0xfff0 as libc::c_int | 0xf as libc::c_int)
                                    as uint64_t) << 48 as libc::c_int)) as intptr_t
                                as libc::intptr_t,
                        )
                            == ::std::mem::transmute::<
                                libc::intptr_t,
                                strm_cfunc,
                            >(
                                (b
                                    & !(((0xfff0 as libc::c_int | 0xf as libc::c_int)
                                        as uint64_t) << 48 as libc::c_int)) as intptr_t
                                    as libc::intptr_t,
                            )) as libc::c_int;
                    }
                    _ => return strm_ary_eq(a, b),
                }
            }
            18445899648779419648 => {
                current_block = 10278175316653942280;
                match current_block {
                    10278175316653942280 => {
                        return ((a
                            & !(((0xfff0 as libc::c_int | 0xf as libc::c_int)
                                as uint64_t) << 48 as libc::c_int)) as intptr_t
                            as *mut libc::c_void
                            == (b
                                & !(((0xfff0 as libc::c_int | 0xf as libc::c_int)
                                    as uint64_t) << 48 as libc::c_int)) as intptr_t
                                as *mut libc::c_void) as libc::c_int;
                    }
                    6079862847828553232 => return strm_str_eq(a, b),
                    11913485640103028541 => {
                        return (::std::mem::transmute::<
                            libc::intptr_t,
                            strm_cfunc,
                        >(
                            (a
                                & !(((0xfff0 as libc::c_int | 0xf as libc::c_int)
                                    as uint64_t) << 48 as libc::c_int)) as intptr_t
                                as libc::intptr_t,
                        )
                            == ::std::mem::transmute::<
                                libc::intptr_t,
                                strm_cfunc,
                            >(
                                (b
                                    & !(((0xfff0 as libc::c_int | 0xf as libc::c_int)
                                        as uint64_t) << 48 as libc::c_int)) as intptr_t
                                    as libc::intptr_t,
                            )) as libc::c_int;
                    }
                    _ => return strm_ary_eq(a, b),
                }
            }
            _ => {}
        }
    }
    if strm_number_p(a) != 0 && strm_number_p(b) != 0 {
        return (strm_value_float(a) == strm_value_float(b)) as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn str_symbol_p(mut str: strm_string) -> libc::c_int {
    let mut p: *const libc::c_char = strm_strp_ptr(&mut str);
    let mut pend: *const libc::c_char = p.offset(strm_str_len(str) as isize);
    if *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
        & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int == 0
        && *p as libc::c_int != '_' as i32
    {
        return 0 as libc::c_int;
    }
    p = p.offset(1);
    p;
    while p < pend {
        if *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
            & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int == 0
            && *p as libc::c_int != '_' as i32
        {
            return 0 as libc::c_int;
        }
        p = p.offset(1);
        p;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn str_dump_len(mut str: strm_string) -> strm_int {
    let mut len: strm_int = 2 as libc::c_int;
    let mut p: *const libc::c_uchar = strm_strp_ptr(&mut str) as *mut libc::c_uchar;
    let mut pend: *const libc::c_uchar = p.offset(strm_str_len(str) as isize);
    while p < pend {
        match *p as libc::c_int {
            10 | 13 | 9 | 34 => {
                len += 2 as libc::c_int;
            }
            _ => {
                if *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
                    & _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0
                    || *p as libc::c_int & 0xff as libc::c_int > 0x7f as libc::c_int
                {
                    len += 1;
                    len;
                } else {
                    len += 3 as libc::c_int;
                }
            }
        }
        p = p.offset(1);
        p;
    }
    return len;
}
unsafe extern "C" fn str_dump(mut str: strm_string, mut len: strm_int) -> strm_string {
    let mut buf: *mut libc::c_char = malloc(len as libc::c_ulong) as *mut libc::c_char;
    let mut s: *mut libc::c_char = buf;
    let mut p: *mut libc::c_char = strm_strp_ptr(&mut str) as *mut libc::c_char;
    let mut pend: *mut libc::c_char = p.offset(strm_str_len(str) as isize);
    let fresh0 = s;
    s = s.offset(1);
    *fresh0 = '"' as i32 as libc::c_char;
    while p < pend {
        match *p as libc::c_int {
            10 => {
                let fresh1 = s;
                s = s.offset(1);
                *fresh1 = '\\' as i32 as libc::c_char;
                let fresh2 = s;
                s = s.offset(1);
                *fresh2 = 'n' as i32 as libc::c_char;
            }
            13 => {
                let fresh3 = s;
                s = s.offset(1);
                *fresh3 = '\\' as i32 as libc::c_char;
                let fresh4 = s;
                s = s.offset(1);
                *fresh4 = 'r' as i32 as libc::c_char;
            }
            9 => {
                let fresh5 = s;
                s = s.offset(1);
                *fresh5 = '\\' as i32 as libc::c_char;
                let fresh6 = s;
                s = s.offset(1);
                *fresh6 = 't' as i32 as libc::c_char;
            }
            27 => {
                let fresh7 = s;
                s = s.offset(1);
                *fresh7 = '\\' as i32 as libc::c_char;
                let fresh8 = s;
                s = s.offset(1);
                *fresh8 = 'e' as i32 as libc::c_char;
            }
            0 => {
                let fresh9 = s;
                s = s.offset(1);
                *fresh9 = '\\' as i32 as libc::c_char;
                let fresh10 = s;
                s = s.offset(1);
                *fresh10 = '0' as i32 as libc::c_char;
            }
            34 => {
                let fresh11 = s;
                s = s.offset(1);
                *fresh11 = '\\' as i32 as libc::c_char;
                let fresh12 = s;
                s = s.offset(1);
                *fresh12 = '"' as i32 as libc::c_char;
            }
            _ => {
                if *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
                    & _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0
                    || *p as libc::c_int & 0xff as libc::c_int > 0x7f as libc::c_int
                {
                    let fresh13 = s;
                    s = s.offset(1);
                    *fresh13 = (*p as libc::c_int & 0xff as libc::c_int) as libc::c_char;
                } else {
                    sprintf(
                        s,
                        b"\\x%02x\0" as *const u8 as *const libc::c_char,
                        *p as libc::c_int & 0xff as libc::c_int,
                    );
                    s = s.offset(4 as libc::c_int as isize);
                }
            }
        }
        p = p.offset(1);
        p;
    }
    let fresh14 = s;
    s = s.offset(1);
    *fresh14 = '"' as i32 as libc::c_char;
    return strm_str_new(buf, len);
}
pub unsafe extern "C" fn strm_inspect(mut v: strm_value) -> strm_string {
    if strm_string_p(v) != 0 {
        let mut str: strm_string = v;
        return str_dump(str, str_dump_len(str));
    } else if strm_array_p(v) != 0 {
        let mut ns: *mut strm_state = (*strm_ary_struct(v)).ns;
        let mut buf: *mut libc::c_char = malloc(32 as libc::c_int as libc::c_ulong)
            as *mut libc::c_char;
        let mut i: strm_int = 0;
        let mut bi: strm_int = 0 as libc::c_int;
        let mut capa: strm_int = 32 as libc::c_int;
        let mut a: strm_array = v;
        if buf.is_null() {
            return 1 as libc::c_int as strm_string;
        }
        let fresh15 = bi;
        bi = bi + 1;
        *buf.offset(fresh15 as isize) = '[' as i32 as libc::c_char;
        if !ns.is_null() {
            let mut name: strm_string = strm_ns_name(ns);
            let mut nlen: strm_int = strm_str_len(name);
            if name != 0 as libc::c_int as libc::c_ulong {
                let fresh16 = bi;
                bi = bi + 1;
                *buf.offset(fresh16 as isize) = '@' as i32 as libc::c_char;
                if bi + nlen + 2 as libc::c_int > capa {
                    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
                    capa *= 2 as libc::c_int;
                    p = realloc(buf as *mut libc::c_void, capa as libc::c_ulong)
                        as *mut libc::c_char;
                    if p.is_null() {
                        free(buf as *mut libc::c_void);
                        return 1 as libc::c_int as strm_string;
                    }
                    buf = p;
                }
                memcpy(
                    buf.offset(bi as isize) as *mut libc::c_void,
                    strm_strp_ptr(&mut name) as *const libc::c_void,
                    nlen as libc::c_ulong,
                );
                bi += nlen;
                if (*strm_ary_struct(a)).len > 0 as libc::c_int {
                    let fresh17 = bi;
                    bi = bi + 1;
                    *buf.offset(fresh17 as isize) = ' ' as i32 as libc::c_char;
                }
            }
        }
        i = 0 as libc::c_int;
        while i < (*strm_ary_struct(a)).len {
            let mut str_0: strm_string = strm_inspect(
                *((*strm_ary_struct(a)).ptr).offset(i as isize),
            );
            let mut key: strm_string = if (*strm_ary_struct(a)).headers != 0
                && strm_string_p(
                    *((*strm_ary_struct((*strm_ary_struct(a)).headers)).ptr)
                        .offset(i as isize),
                ) != 0
            {
                *((*strm_ary_struct((*strm_ary_struct(a)).headers)).ptr)
                    .offset(i as isize)
            } else {
                0 as libc::c_int as libc::c_ulong
            };
            let mut slen: strm_int = (if key != 0 {
                strm_str_len(key) + 1 as libc::c_int
            } else {
                0 as libc::c_int
            }) + strm_str_len(str_0) + 3 as libc::c_int;
            if bi + slen > capa {
                capa *= 2 as libc::c_int;
                buf = realloc(buf as *mut libc::c_void, capa as libc::c_ulong)
                    as *mut libc::c_char;
            }
            if i > 0 as libc::c_int {
                let fresh18 = bi;
                bi = bi + 1;
                *buf.offset(fresh18 as isize) = ',' as i32 as libc::c_char;
                let fresh19 = bi;
                bi = bi + 1;
                *buf.offset(fresh19 as isize) = ' ' as i32 as libc::c_char;
            }
            if key != 0 {
                if str_symbol_p(key) == 0 {
                    key = str_dump(key, str_dump_len(key));
                }
                memcpy(
                    buf.offset(bi as isize) as *mut libc::c_void,
                    strm_strp_ptr(&mut key) as *const libc::c_void,
                    strm_str_len(key) as libc::c_ulong,
                );
                bi += strm_str_len(key);
                let fresh20 = bi;
                bi = bi + 1;
                *buf.offset(fresh20 as isize) = ':' as i32 as libc::c_char;
            }
            memcpy(
                buf.offset(bi as isize) as *mut libc::c_void,
                strm_strp_ptr(&mut str_0) as *const libc::c_void,
                strm_str_len(str_0) as libc::c_ulong,
            );
            bi += strm_str_len(str_0);
            i += 1;
            i;
        }
        let fresh21 = bi;
        bi = bi + 1;
        *buf.offset(fresh21 as isize) = ']' as i32 as libc::c_char;
        return strm_str_new(buf, bi);
    } else {
        return strm_to_str(v)
    };
}
pub unsafe extern "C" fn strm_to_str(mut v: strm_value) -> strm_string {
    let mut buf: [libc::c_char; 32] = [0; 32];
    let mut n: libc::c_int = 0;
    let mut ns: *mut strm_state = strm_value_ns(v);
    if !ns.is_null() {
        let mut m: strm_value = 0;
        n = strm_var_get(
            ns,
            strm_str_intern_static(
                b"string\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as strm_int,
            ),
            &mut m,
        );
        if n == 0 as libc::c_int {
            n = strm_funcall(0 as *mut strm_stream, m, 1 as libc::c_int, &mut v, &mut m);
            if n == 0 as libc::c_int && strm_string_p(m) != 0 {
                return m;
            }
        }
    }
    match v
        & ((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t) << 48 as libc::c_int
    {
        18442803424035602432 => {
            n = sprintf(
                buf.as_mut_ptr(),
                b"%d\0" as *const u8 as *const libc::c_char,
                strm_to_int(v),
            );
            return strm_str_new(buf.as_mut_ptr(), n);
        }
        18442521949058891776 => {
            n = sprintf(
                buf.as_mut_ptr(),
                if strm_to_int(v) != 0 {
                    b"true\0" as *const u8 as *const libc::c_char
                } else {
                    b"false\0" as *const u8 as *const libc::c_char
                },
            );
            return strm_str_new(buf.as_mut_ptr(), n);
        }
        18445336698825998336 => {
            n = sprintf(
                buf.as_mut_ptr(),
                b"<cfunc:%p>\0" as *const u8 as *const libc::c_char,
                ::std::mem::transmute::<
                    strm_cfunc,
                    *mut libc::c_void,
                >(strm_value_cfunc(v)),
            );
            return strm_str_new(buf.as_mut_ptr(), n);
        }
        18444210798919155712 | 18444492273895866368 | 18444773748872577024
        | 18445055223849287680 => return v,
        18443366373989023744 | 18443647848965734400 => return strm_inspect(v),
        18445899648779419648 => {
            if v
                & !(((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t)
                    << 48 as libc::c_int) == 0 as libc::c_int as libc::c_ulong
            {
                return strm_str_static(
                    b"nil\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as strm_int,
                )
            } else {
                let mut p: *mut libc::c_void = strm_ptr(v);
                match strm_ptr_type(p) as libc::c_uint {
                    0 => {
                        n = sprintf(
                            buf.as_mut_ptr(),
                            b"<stream:%p>\0" as *const u8 as *const libc::c_char,
                            p,
                        );
                    }
                    3 => {
                        let mut io: strm_io = p as strm_io;
                        let mut mode: *mut libc::c_char = 0 as *mut libc::c_char;
                        match (*io).mode & 3 as libc::c_int {
                            1 => {
                                mode = b"r\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char;
                            }
                            2 => {
                                mode = b"w\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char;
                            }
                            3 => {
                                mode = b"rw\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char;
                            }
                            _ => {
                                mode = b"?\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char;
                            }
                        }
                        n = sprintf(
                            buf.as_mut_ptr(),
                            b"<io: fd=%d mode=%s>\0" as *const u8 as *const libc::c_char,
                            (*io).fd,
                            mode,
                        );
                    }
                    1 => {
                        n = sprintf(
                            buf.as_mut_ptr(),
                            b"<lambda:%p>\0" as *const u8 as *const libc::c_char,
                            p,
                        );
                    }
                    2 => {
                        let mut fbuf: [libc::c_char; 7] = [0; 7];
                        let mut gf: *mut strm_genfunc = p as *mut strm_genfunc;
                        n = sprintf(
                            buf.as_mut_ptr(),
                            b"<genfunc:&%s>\0" as *const u8 as *const libc::c_char,
                            strm_str_cstr((*gf).id, fbuf.as_mut_ptr()),
                        );
                    }
                    4 => {
                        n = sprintf(
                            buf.as_mut_ptr(),
                            b"<obj:%p>\0" as *const u8 as *const libc::c_char,
                            p,
                        );
                    }
                    _ => {}
                }
                return strm_str_new(buf.as_mut_ptr(), n);
            }
        }
        _ => {
            if strm_float_p(v) != 0 {
                n = sprintf(
                    buf.as_mut_ptr(),
                    b"%.14g\0" as *const u8 as *const libc::c_char,
                    strm_to_float(v),
                );
                return strm_str_new(buf.as_mut_ptr(), n);
            }
            n = sprintf(
                buf.as_mut_ptr(),
                b"<%p>\0" as *const u8 as *const libc::c_char,
                (v
                    & !(((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t)
                        << 48 as libc::c_int)) as intptr_t as *mut libc::c_void,
            );
            return strm_str_new(buf.as_mut_ptr(), n);
        }
    };
}
pub unsafe extern "C" fn strm_p(mut val: strm_value) -> *const libc::c_char {
    let mut buf: [libc::c_char; 7] = [0; 7];
    let mut str: strm_string = strm_to_str(val);
    let mut p: *const libc::c_char = strm_str_cstr(str, buf.as_mut_ptr());
    fputs(p, stdout);
    fputs(b"\n\0" as *const u8 as *const libc::c_char, stdout);
    return p;
}
pub unsafe extern "C" fn strm_nil_value() -> strm_value {
    return STRM_TAG_PTR as libc::c_ulong | 0 as libc::c_int as libc::c_ulong;
}
pub unsafe extern "C" fn strm_value_ns(mut val: strm_value) -> *mut strm_state {
    if strm_array_p(val) != 0 {
        let mut ns: *mut strm_state = (*strm_ary_struct(val)).ns;
        if !ns.is_null() {
            return ns;
        }
        return strm_ns_array;
    }
    if strm_string_p(val) != 0 {
        return strm_ns_string;
    }
    if strm_number_p(val) != 0 {
        return strm_ns_number;
    }
    if val
        & ((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t) << 48 as libc::c_int
        == STRM_TAG_PTR as libc::c_ulong
    {
        let mut p: *mut strm_misc = strm_ptr(val) as *mut strm_misc;
        if p.is_null() {
            return 0 as *mut strm_state;
        }
        if strm_ptr_type(p as *mut libc::c_void) as libc::c_uint
            == STRM_PTR_AUX as libc::c_int as libc::c_uint
        {
            return (*p).ns;
        }
    }
    return 0 as *mut strm_state;
}
