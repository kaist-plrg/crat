use ::libc;
extern "C" {
    pub type strm_queue;
    pub type node_error;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn strm_ns_new(_: *mut strm_state, _: *const libc::c_char) -> *mut strm_state;
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
    fn strm_funcall(
        _: *mut strm_stream,
        _: strm_value,
        _: libc::c_int,
        _: *mut strm_value,
        _: *mut strm_value,
    ) -> libc::c_int;
    fn strm_value_eq(_: strm_value, _: strm_value) -> libc::c_int;
    fn strm_value_float(_: strm_value) -> libc::c_double;
    fn strm_nil_value() -> strm_value;
    fn strm_int_value(_: strm_int) -> strm_value;
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
    fn strm_cfunc_value(_: strm_cfunc) -> strm_value;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __intptr_t = libc::c_long;
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
pub type strm_array = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_array_0 {
    pub len: strm_int,
    pub ptr: *mut strm_value,
    pub headers: strm_array,
    pub ns: *mut strm_state,
}
pub unsafe extern "C" fn strm_array_p(mut v: strm_value) -> libc::c_int {
    match v
        & ((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t) << 48 as libc::c_int
    {
        18443366373989023744 | 18443647848965734400 => return 1 as libc::c_int,
        _ => return 0 as libc::c_int,
    };
}
pub unsafe extern "C" fn strm_ary_new(
    mut p: *const strm_value,
    mut len: strm_int,
) -> strm_array {
    let mut ary: *mut strm_array_0 = 0 as *mut strm_array_0;
    let mut buf: *mut strm_value = 0 as *mut strm_value;
    ary = malloc(
        (::std::mem::size_of::<strm_array_0>() as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<strm_value>() as libc::c_ulong)
                    .wrapping_mul(len as libc::c_ulong),
            ),
    ) as *mut strm_array_0;
    buf = &mut *ary.offset(1 as libc::c_int as isize) as *mut strm_array_0
        as *mut strm_value;
    if !p.is_null() {
        memcpy(
            buf as *mut libc::c_void,
            p as *const libc::c_void,
            (::std::mem::size_of::<strm_value>() as libc::c_ulong)
                .wrapping_mul(len as libc::c_ulong),
        );
    } else {
        memset(
            buf as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<strm_value>() as libc::c_ulong)
                .wrapping_mul(len as libc::c_ulong),
        );
    }
    (*ary).ptr = buf;
    (*ary).len = len;
    (*ary).ns = 0 as *mut strm_state;
    (*ary).headers = 0 as libc::c_int as strm_array;
    return STRM_TAG_ARRAY as libc::c_ulong
        | ary as intptr_t as libc::c_ulong
            & !(((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t)
                << 48 as libc::c_int);
}
pub unsafe extern "C" fn strm_ary_eq(
    mut a: strm_array,
    mut b: strm_array,
) -> libc::c_int {
    let mut i: strm_int = 0;
    let mut len: strm_int = 0;
    if a == b {
        return 1 as libc::c_int;
    }
    if (*strm_ary_struct(a)).len != (*strm_ary_struct(b)).len {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    len = (*strm_ary_struct(a)).len;
    while i < len {
        if strm_value_eq(
            *((*strm_ary_struct(a)).ptr).offset(i as isize),
            *((*strm_ary_struct(b)).ptr).offset(i as isize),
        ) == 0
        {
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn strm_ary_struct(mut v: strm_value) -> *mut strm_array_0 {
    return (v
        & !(((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t)
            << 48 as libc::c_int)) as intptr_t as *mut libc::c_void as *mut strm_array_0;
}
pub static mut strm_ns_array: *mut strm_state = 0 as *const strm_state
    as *mut strm_state;
unsafe extern "C" fn ary_length(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut v: *mut strm_value = 0 as *mut strm_value;
    let mut len: strm_int = 0;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"a\0" as *const u8 as *const libc::c_char,
        &mut v as *mut *mut strm_value,
        &mut len as *mut strm_int,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    *ret = strm_int_value(len);
    return 0 as libc::c_int;
}
unsafe extern "C" fn ary_reverse(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut v: *mut strm_value = 0 as *mut strm_value;
    let mut v2: *mut strm_value = 0 as *mut strm_value;
    let mut len: strm_int = 0;
    let mut ary: strm_array = 0;
    let mut i: strm_int = 0;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"a\0" as *const u8 as *const libc::c_char,
        &mut v as *mut *mut strm_value,
        &mut len as *mut strm_int,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    ary = strm_ary_new(0 as *const strm_value, len);
    v2 = (*strm_ary_struct(ary)).ptr;
    i = 0 as libc::c_int;
    while i < len {
        *v2.offset((len - i - 1 as libc::c_int) as isize) = *v.offset(i as isize);
        i += 1;
        i;
    }
    *ret = ary;
    return 0 as libc::c_int;
}
unsafe extern "C" fn ary_minmax(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
    mut min: libc::c_int,
) -> libc::c_int {
    let mut func: strm_value = strm_nil_value();
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut v: *mut strm_value = 0 as *mut strm_value;
    let mut e: strm_value = 0;
    let mut val: strm_value = 0;
    let mut num: libc::c_double = 0.;
    let mut f: libc::c_double = 0.;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"a|v\0" as *const u8 as *const libc::c_char,
        &mut v as *mut *mut strm_value,
        &mut len as *mut libc::c_int,
        &mut func as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if len == 0 as libc::c_int {
        *ret = strm_nil_value();
        return 0 as libc::c_int;
    }
    val = *v.offset(0 as libc::c_int as isize);
    if argc == 2 as libc::c_int {
        if strm_funcall(
            strm,
            func,
            1 as libc::c_int,
            &mut *v.offset(0 as libc::c_int as isize),
            &mut e,
        ) == 1 as libc::c_int
        {
            return 1 as libc::c_int;
        }
    } else {
        e = *v.offset(0 as libc::c_int as isize);
    }
    num = strm_value_float(e);
    i = 1 as libc::c_int;
    while i < len {
        if argc == 2 as libc::c_int {
            if strm_funcall(
                strm,
                func,
                1 as libc::c_int,
                &mut *v.offset(i as isize),
                &mut e,
            ) == 1 as libc::c_int
            {
                return 1 as libc::c_int;
            }
        } else {
            e = *v.offset(0 as libc::c_int as isize);
        }
        f = strm_value_float(e);
        if min != 0 {
            if num > f {
                num = f;
                val = *v.offset(i as isize);
            }
        } else if num < f {
            num = f;
            val = *v.offset(i as isize);
        }
        i += 1;
        i;
    }
    *ret = val;
    return 0 as libc::c_int;
}
unsafe extern "C" fn ary_min(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    return ary_minmax(strm, argc, args, ret, 1 as libc::c_int);
}
unsafe extern "C" fn ary_max(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    return ary_minmax(strm, argc, args, ret, 0 as libc::c_int);
}
pub unsafe extern "C" fn strm_array_init(mut state: *mut strm_state) {
    strm_ns_array = strm_ns_new(
        0 as *mut strm_state,
        b"array\0" as *const u8 as *const libc::c_char,
    );
    strm_var_def(
        strm_ns_array,
        b"length\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                ary_length
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
        strm_ns_array,
        b"reverse\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                ary_reverse
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
        strm_ns_array,
        b"min\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                ary_min
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
        strm_ns_array,
        b"max\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                ary_max
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
