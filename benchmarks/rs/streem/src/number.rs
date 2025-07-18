use ::libc;
extern "C" {
    pub type strm_queue;
    pub type node_error;
    fn strm_cfunc_value(_: strm_cfunc) -> strm_value;
    fn strm_bool_value(_: libc::c_int) -> strm_value;
    fn strm_int_value(_: strm_int) -> strm_value;
    fn strm_float_value(_: libc::c_double) -> strm_value;
    fn strm_value_int(_: strm_value) -> strm_int;
    fn strm_value_float(_: strm_value) -> libc::c_double;
    fn strm_number_p(_: strm_value) -> libc::c_int;
    fn strm_int_p(_: strm_value) -> libc::c_int;
    fn strm_float_p(_: strm_value) -> libc::c_int;
    fn strm_parse_args(
        _: *mut strm_stream,
        _: libc::c_int,
        _: *mut strm_value,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strm_var_def(
        _: *mut strm_state,
        _: *const libc::c_char,
        _: strm_value,
    ) -> libc::c_int;
    fn strm_ns_new(_: *mut strm_state, _: *const libc::c_char) -> *mut strm_state;
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
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
unsafe extern "C" fn num_plus(
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
        b"NN\0" as *const u8 as *const libc::c_char,
        &mut x as *mut strm_value,
        &mut y as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if strm_int_p(x) != 0 && strm_int_p(y) != 0 {
        *ret = strm_int_value(strm_value_int(x) + strm_value_int(y));
        return 0 as libc::c_int;
    }
    if strm_number_p(x) != 0 && strm_number_p(y) != 0 {
        *ret = strm_float_value(strm_value_float(x) + strm_value_float(y));
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn num_minus(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    if argc == 1 as libc::c_int {
        if strm_int_p(*args.offset(0 as libc::c_int as isize)) != 0 {
            *ret = strm_int_value(
                -strm_value_int(*args.offset(0 as libc::c_int as isize)),
            );
            return 0 as libc::c_int;
        }
        if strm_float_p(*args.offset(0 as libc::c_int as isize)) != 0 {
            *ret = strm_float_value(
                -strm_value_float(*args.offset(0 as libc::c_int as isize)),
            );
            return 0 as libc::c_int;
        }
    } else {
        let mut x: strm_value = 0;
        let mut y: strm_value = 0;
        if strm_parse_args(
            strm,
            argc,
            args,
            b"NN\0" as *const u8 as *const libc::c_char,
            &mut x as *mut strm_value,
            &mut y as *mut strm_value,
        ) == 1 as libc::c_int
        {
            return 1 as libc::c_int;
        }
        if strm_int_p(x) != 0 && strm_int_p(y) != 0 {
            *ret = strm_int_value(strm_value_int(x) - strm_value_int(y));
            return 0 as libc::c_int;
        }
        if strm_number_p(x) != 0 && strm_number_p(y) != 0 {
            *ret = strm_float_value(strm_value_float(x) - strm_value_float(y));
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn num_mult(
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
        b"NN\0" as *const u8 as *const libc::c_char,
        &mut x as *mut strm_value,
        &mut y as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if strm_int_p(x) != 0 && strm_int_p(y) != 0 {
        *ret = strm_int_value(strm_value_int(x) * strm_value_int(y));
        return 0 as libc::c_int;
    }
    *ret = strm_float_value(strm_value_float(x) * strm_value_float(y));
    return 0 as libc::c_int;
}
unsafe extern "C" fn num_div(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"ff\0" as *const u8 as *const libc::c_char,
        &mut x as *mut libc::c_double,
        &mut y as *mut libc::c_double,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    *ret = strm_float_value(x / y);
    return 0 as libc::c_int;
}
unsafe extern "C" fn num_bar(
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
        b"ii\0" as *const u8 as *const libc::c_char,
        &mut x as *mut strm_value,
        &mut y as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    *ret = strm_int_value(strm_value_int(x) | strm_value_int(y));
    return 0 as libc::c_int;
}
unsafe extern "C" fn num_mod(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut x: strm_value = 0;
    let mut y: strm_int = 0;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"Ni\0" as *const u8 as *const libc::c_char,
        &mut x as *mut strm_value,
        &mut y as *mut strm_int,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if strm_int_p(x) != 0 {
        *ret = strm_int_value(strm_value_int(x) % y);
        return 0 as libc::c_int;
    }
    if strm_float_p(x) != 0 {
        *ret = strm_float_value(fmod(strm_value_float(x), y as libc::c_double));
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn num_gt(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"ff\0" as *const u8 as *const libc::c_char,
        &mut x as *mut libc::c_double,
        &mut y as *mut libc::c_double,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    *ret = strm_bool_value((x > y) as libc::c_int);
    return 0 as libc::c_int;
}
unsafe extern "C" fn num_ge(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"ff\0" as *const u8 as *const libc::c_char,
        &mut x as *mut libc::c_double,
        &mut y as *mut libc::c_double,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    *ret = strm_bool_value((x >= y) as libc::c_int);
    return 0 as libc::c_int;
}
unsafe extern "C" fn num_lt(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"ff\0" as *const u8 as *const libc::c_char,
        &mut x as *mut libc::c_double,
        &mut y as *mut libc::c_double,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    *ret = strm_bool_value((x < y) as libc::c_int);
    return 0 as libc::c_int;
}
unsafe extern "C" fn num_le(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"ff\0" as *const u8 as *const libc::c_char,
        &mut x as *mut libc::c_double,
        &mut y as *mut libc::c_double,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    *ret = strm_bool_value((x <= y) as libc::c_int);
    return 0 as libc::c_int;
}
unsafe extern "C" fn num_number(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    if strm_parse_args(strm, argc, args, b"N\0" as *const u8 as *const libc::c_char, ret)
        == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub static mut strm_ns_number: *mut strm_state = 0 as *const strm_state
    as *mut strm_state;
pub unsafe extern "C" fn strm_number_init(mut state: *mut strm_state) {
    strm_ns_number = strm_ns_new(
        0 as *mut strm_state,
        b"number\0" as *const u8 as *const libc::c_char,
    );
    strm_var_def(
        strm_ns_number,
        b"+\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                num_plus
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
        strm_ns_number,
        b"-\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                num_minus
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
        strm_ns_number,
        b"*\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                num_mult
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
        strm_ns_number,
        b"/\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                num_div
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
        strm_ns_number,
        b"%\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                num_mod
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
        strm_ns_number,
        b"|\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                num_bar
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
        strm_ns_number,
        b"<\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                num_lt
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
        strm_ns_number,
        b"<=\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                num_le
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
        strm_ns_number,
        b">\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                num_gt
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
        strm_ns_number,
        b">=\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                num_ge
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
        b"number\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                num_number
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
