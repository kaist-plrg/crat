use ::libc;
extern "C" {
    pub type strm_queue;
    pub type node_error;
    fn acos(_: libc::c_double) -> libc::c_double;
    fn asin(_: libc::c_double) -> libc::c_double;
    fn atan(_: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn tan(_: libc::c_double) -> libc::c_double;
    fn cosh(_: libc::c_double) -> libc::c_double;
    fn sinh(_: libc::c_double) -> libc::c_double;
    fn tanh(_: libc::c_double) -> libc::c_double;
    fn acosh(_: libc::c_double) -> libc::c_double;
    fn asinh(_: libc::c_double) -> libc::c_double;
    fn atanh(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn frexp(_: libc::c_double, _: *mut libc::c_int) -> libc::c_double;
    fn ldexp(_: libc::c_double, _: libc::c_int) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn log10(_: libc::c_double) -> libc::c_double;
    fn log2(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn hypot(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn cbrt(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn erfc(_: libc::c_double) -> libc::c_double;
    fn trunc(_: libc::c_double) -> libc::c_double;
    fn round(_: libc::c_double) -> libc::c_double;
    fn abs(_: libc::c_int) -> libc::c_int;
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
    fn strm_float_value(_: libc::c_double) -> strm_value;
    fn strm_cfunc_value(_: strm_cfunc) -> strm_value;
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
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
unsafe extern "C" fn math_sqrt(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut f: libc::c_double = 0.;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"f\0" as *const u8 as *const libc::c_char,
        &mut f as *mut libc::c_double,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    *ret = strm_float_value(sqrt(f));
    return 0 as libc::c_int;
}
unsafe extern "C" fn math_sin(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut f: libc::c_double = 0.;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"f\0" as *const u8 as *const libc::c_char,
        &mut f as *mut libc::c_double,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    *ret = strm_float_value(sin(f));
    return 0 as libc::c_int;
}
unsafe extern "C" fn math_sinh(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut f: libc::c_double = 0.;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"f\0" as *const u8 as *const libc::c_char,
        &mut f as *mut libc::c_double,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    *ret = strm_float_value(sinh(f));
    return 0 as libc::c_int;
}
unsafe extern "C" fn math_cos(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut f: libc::c_double = 0.;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"f\0" as *const u8 as *const libc::c_char,
        &mut f as *mut libc::c_double,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    *ret = strm_float_value(cos(f));
    return 0 as libc::c_int;
}
unsafe extern "C" fn math_cosh(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut f: libc::c_double = 0.;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"f\0" as *const u8 as *const libc::c_char,
        &mut f as *mut libc::c_double,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    *ret = strm_float_value(cosh(f));
    return 0 as libc::c_int;
}
unsafe extern "C" fn math_tan(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut f: libc::c_double = 0.;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"f\0" as *const u8 as *const libc::c_char,
        &mut f as *mut libc::c_double,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    *ret = strm_float_value(tan(f));
    return 0 as libc::c_int;
}
unsafe extern "C" fn math_tanh(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut f: libc::c_double = 0.;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"f\0" as *const u8 as *const libc::c_char,
        &mut f as *mut libc::c_double,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    *ret = strm_float_value(tanh(f));
    return 0 as libc::c_int;
}
unsafe extern "C" fn math_pow(
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
    *ret = strm_float_value(pow(x, y));
    return 0 as libc::c_int;
}
unsafe extern "C" fn GCD(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if b != 0 { GCD(b, a % b) } else { a };
}
unsafe extern "C" fn math_gcd(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"ii\0" as *const u8 as *const libc::c_char,
        &mut x as *mut libc::c_int,
        &mut y as *mut libc::c_int,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    *ret = strm_float_value(GCD(x, y) as libc::c_double);
    return 0 as libc::c_int;
}
unsafe extern "C" fn math_fabs(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut f: libc::c_double = 0.;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"f\0" as *const u8 as *const libc::c_char,
        &mut f as *mut libc::c_double,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    *ret = strm_float_value(abs(f as libc::c_int) as libc::c_double);
    return 0 as libc::c_int;
}
unsafe extern "C" fn math_acosh(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut f: libc::c_double = 0.;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"f\0" as *const u8 as *const libc::c_char,
        &mut f as *mut libc::c_double,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    *ret = strm_float_value(acosh(f));
    return 0 as libc::c_int;
}
unsafe extern "C" fn math_asinh(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut f: libc::c_double = 0.;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"f\0" as *const u8 as *const libc::c_char,
        &mut f as *mut libc::c_double,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    *ret = strm_float_value(asinh(f));
    return 0 as libc::c_int;
}
unsafe extern "C" fn math_atanh(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut f: libc::c_double = 0.;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"f\0" as *const u8 as *const libc::c_char,
        &mut f as *mut libc::c_double,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    *ret = strm_float_value(atanh(f));
    return 0 as libc::c_int;
}
unsafe extern "C" fn math_acos(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut f: libc::c_double = 0.;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"f\0" as *const u8 as *const libc::c_char,
        &mut f as *mut libc::c_double,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    *ret = strm_float_value(acos(f));
    return 0 as libc::c_int;
}
unsafe extern "C" fn math_asin(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut f: libc::c_double = 0.;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"f\0" as *const u8 as *const libc::c_char,
        &mut f as *mut libc::c_double,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    *ret = strm_float_value(asin(f));
    return 0 as libc::c_int;
}
unsafe extern "C" fn math_atan(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut f: libc::c_double = 0.;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"f\0" as *const u8 as *const libc::c_char,
        &mut f as *mut libc::c_double,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    *ret = strm_float_value(atan(f));
    return 0 as libc::c_int;
}
unsafe extern "C" fn math_log(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut f: libc::c_double = 0.;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"f\0" as *const u8 as *const libc::c_char,
        &mut f as *mut libc::c_double,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    *ret = strm_float_value(log(f));
    return 0 as libc::c_int;
}
unsafe extern "C" fn math_log10(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut f: libc::c_double = 0.;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"f\0" as *const u8 as *const libc::c_char,
        &mut f as *mut libc::c_double,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    *ret = strm_float_value(log10(f));
    return 0 as libc::c_int;
}
unsafe extern "C" fn math_exp(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut f: libc::c_double = 0.;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"f\0" as *const u8 as *const libc::c_char,
        &mut f as *mut libc::c_double,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    *ret = strm_float_value(exp(f));
    return 0 as libc::c_int;
}
unsafe extern "C" fn math_log2(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut f: libc::c_double = 0.;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"f\0" as *const u8 as *const libc::c_char,
        &mut f as *mut libc::c_double,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    *ret = strm_float_value(log2(f));
    return 0 as libc::c_int;
}
unsafe extern "C" fn math_erfc(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut f: libc::c_double = 0.;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"f\0" as *const u8 as *const libc::c_char,
        &mut f as *mut libc::c_double,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    *ret = strm_float_value(erfc(f));
    return 0 as libc::c_int;
}
unsafe extern "C" fn math_cbrt(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut f: libc::c_double = 0.;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"f\0" as *const u8 as *const libc::c_char,
        &mut f as *mut libc::c_double,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    *ret = strm_float_value(cbrt(f));
    return 0 as libc::c_int;
}
unsafe extern "C" fn math_hypot(
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
    *ret = strm_float_value(hypot(x, y));
    return 0 as libc::c_int;
}
unsafe extern "C" fn math_frexp(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_int = 0;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"ff\0" as *const u8 as *const libc::c_char,
        &mut x as *mut libc::c_double,
        &mut y as *mut libc::c_int,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    *ret = strm_float_value(frexp(x, &mut y));
    return 0 as libc::c_int;
}
unsafe extern "C" fn math_ldexp(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_int = 0;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"ff\0" as *const u8 as *const libc::c_char,
        &mut x as *mut libc::c_double,
        &mut y as *mut libc::c_int,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    *ret = strm_float_value(ldexp(x, y));
    return 0 as libc::c_int;
}
unsafe extern "C" fn math_round(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut x: libc::c_double = 0.;
    let mut d: strm_int = 0 as libc::c_int;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"f|i\0" as *const u8 as *const libc::c_char,
        &mut x as *mut libc::c_double,
        &mut d as *mut strm_int,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if argc == 1 as libc::c_int {
        *ret = strm_float_value(round(x));
    } else {
        let mut f: libc::c_double = pow(
            10 as libc::c_int as libc::c_double,
            d as libc::c_double,
        );
        *ret = strm_float_value(round(x * f) / f);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn math_ceil(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut x: libc::c_double = 0.;
    let mut d: strm_int = 0 as libc::c_int;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"f|i\0" as *const u8 as *const libc::c_char,
        &mut x as *mut libc::c_double,
        &mut d as *mut strm_int,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if argc == 1 as libc::c_int {
        *ret = strm_float_value(ceil(x));
    } else {
        let mut f: libc::c_double = pow(
            10 as libc::c_int as libc::c_double,
            d as libc::c_double,
        );
        *ret = strm_float_value(ceil(x * f) / f);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn math_floor(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut x: libc::c_double = 0.;
    let mut d: strm_int = 0 as libc::c_int;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"f|i\0" as *const u8 as *const libc::c_char,
        &mut x as *mut libc::c_double,
        &mut d as *mut strm_int,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if argc == 1 as libc::c_int {
        *ret = strm_float_value(floor(x));
    } else {
        let mut f: libc::c_double = pow(
            10 as libc::c_int as libc::c_double,
            d as libc::c_double,
        );
        *ret = strm_float_value(floor(x * f) / f);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn math_trunc(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut x: libc::c_double = 0.;
    let mut d: strm_int = 0 as libc::c_int;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"f|i\0" as *const u8 as *const libc::c_char,
        &mut x as *mut libc::c_double,
        &mut d as *mut strm_int,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if argc == 1 as libc::c_int {
        *ret = strm_float_value(trunc(x));
    } else {
        let mut f: libc::c_double = pow(
            10 as libc::c_int as libc::c_double,
            d as libc::c_double,
        );
        *ret = strm_float_value(trunc(x * f) / f);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn strm_math_init(mut state: *mut strm_state) {
    strm_var_def(
        state,
        b"PI\0" as *const u8 as *const libc::c_char,
        strm_float_value(3.14159265358979323846f64),
    );
    strm_var_def(
        state,
        b"E\0" as *const u8 as *const libc::c_char,
        strm_float_value(2.7182818284590452354f64),
    );
    strm_var_def(
        state,
        b"sqrt\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                math_sqrt
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
        b"sin\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                math_sin
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
        b"cos\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                math_cos
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
        b"tan\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                math_tan
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
        b"sinh\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                math_sinh
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
        b"cosh\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                math_cosh
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
        b"asin\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                math_asin
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
        b"acos\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                math_acos
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
        b"atan\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                math_atan
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
        b"asinh\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                math_asinh
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
        b"acosh\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                math_acosh
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
        b"atanh\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                math_atanh
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
        b"tanh\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                math_tanh
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
        b"pow\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                math_pow
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
        b"round\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                math_round
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
        b"ceil\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                math_ceil
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
        b"floor\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                math_floor
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
        b"trunc\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                math_trunc
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
        b"int\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                math_trunc
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
        b"fabs\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                math_fabs
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
        b"log\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                math_log
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
        b"log10\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                math_log10
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
        b"log2\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                math_log2
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
        b"exp\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                math_exp
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
        b"erfc\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                math_erfc
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
        b"cbrt\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                math_cbrt
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
        b"hypot\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                math_hypot
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
        b"frexp\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                math_frexp
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
        b"ldexp\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                math_ldexp
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
        b"gcd\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                math_gcd
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
