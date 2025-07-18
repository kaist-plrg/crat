use ::libc;
extern "C" {
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn modf(_: libc::c_double, _: *mut libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_sf_result_struct {
    pub val: libc::c_double,
    pub err: libc::c_double,
}
pub type gsl_sf_result = gsl_sf_result_struct;
pub type C2RustUnnamed = libc::c_int;
pub const GSL_EOF: C2RustUnnamed = 32;
pub const GSL_ETOLG: C2RustUnnamed = 31;
pub const GSL_ETOLX: C2RustUnnamed = 30;
pub const GSL_ETOLF: C2RustUnnamed = 29;
pub const GSL_ENOPROGJ: C2RustUnnamed = 28;
pub const GSL_ENOPROG: C2RustUnnamed = 27;
pub const GSL_ETABLE: C2RustUnnamed = 26;
pub const GSL_ECACHE: C2RustUnnamed = 25;
pub const GSL_EUNIMPL: C2RustUnnamed = 24;
pub const GSL_EUNSUP: C2RustUnnamed = 23;
pub const GSL_EDIVERGE: C2RustUnnamed = 22;
pub const GSL_ESING: C2RustUnnamed = 21;
pub const GSL_ENOTSQR: C2RustUnnamed = 20;
pub const GSL_EBADLEN: C2RustUnnamed = 19;
pub const GSL_EROUND: C2RustUnnamed = 18;
pub const GSL_ELOSS: C2RustUnnamed = 17;
pub const GSL_EOVRFLW: C2RustUnnamed = 16;
pub const GSL_EUNDRFLW: C2RustUnnamed = 15;
pub const GSL_ETOL: C2RustUnnamed = 14;
pub const GSL_EBADTOL: C2RustUnnamed = 13;
pub const GSL_EZERODIV: C2RustUnnamed = 12;
pub const GSL_EMAXITER: C2RustUnnamed = 11;
pub const GSL_ERUNAWAY: C2RustUnnamed = 10;
pub const GSL_EBADFUNC: C2RustUnnamed = 9;
pub const GSL_ENOMEM: C2RustUnnamed = 8;
pub const GSL_ESANITY: C2RustUnnamed = 7;
pub const GSL_EFACTOR: C2RustUnnamed = 6;
pub const GSL_EFAILED: C2RustUnnamed = 5;
pub const GSL_EINVAL: C2RustUnnamed = 4;
pub const GSL_EFAULT: C2RustUnnamed = 3;
pub const GSL_ERANGE: C2RustUnnamed = 2;
pub const GSL_EDOM: C2RustUnnamed = 1;
pub const GSL_CONTINUE: C2RustUnnamed = -2;
pub const GSL_FAILURE: C2RustUnnamed = -1;
pub const GSL_SUCCESS: C2RustUnnamed = 0;
unsafe extern "C" fn sin_pi_taylor(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    (*result).val = 0.0f64;
    (*result).err = 0.0f64;
    if 16.0f64 * fabs(x) < 1.0f64 {
        let y: libc::c_double = 3.14159265358979323846f64 * x;
        let a: libc::c_double = y * y;
        (*result)
            .val = y
            * (1.0f64
                - a
                    * (1.0f64
                        - a
                            * (1.0f64
                                - a * (1.0f64 - a * (1.0f64 - a / 110.0f64) / 72.0f64)
                                    / 42.0f64) / 20.0f64) / 6.0f64);
    } else {
        (*result).val = sin(3.14159265358979323846f64 * x);
    }
    (*result).err = 2.2204460492503131e-16f64 * fabs((*result).val);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn cos_pi_taylor(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    (*result).val = 0.0f64;
    (*result).err = 0.0f64;
    if 20.0f64 * fabs(x) < 1.0f64 {
        let y: libc::c_double = 3.14159265358979323846f64 * x;
        let a: libc::c_double = y * y;
        (*result)
            .val = 1.0f64
            - 0.5f64 * a
                * (1.0f64
                    - a
                        * (1.0f64
                            - a * (1.0f64 - a * (1.0f64 - a / 90.0f64) / 56.0f64)
                                / 30.0f64) / 12.0f64);
    } else {
        (*result).val = cos(3.14159265358979323846f64 * x);
    }
    (*result).err = 2.2204460492503131e-16f64 * fabs((*result).val);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sf_sin_pi_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut intx: libc::c_double = 0.0f64;
    let mut fracx: libc::c_double = 0.0f64;
    let mut q: libc::c_long = 0;
    let mut sign: libc::c_int = 1 as libc::c_int;
    let mut status: libc::c_int = 0;
    (*result).val = 0.0f64;
    (*result).err = 0.0f64;
    fracx = modf(x, &mut intx);
    if fracx == 0.0f64 {
        return GSL_SUCCESS as libc::c_int;
    }
    if fabs(intx) >= 2.0f64 / 2.2204460492503131e-16f64 {
        return GSL_SUCCESS as libc::c_int;
    }
    q = (if intx
        >= (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long) as libc::c_double
        && intx <= 9223372036854775807 as libc::c_long as libc::c_double
    {
        intx
    } else {
        fmod(intx, 2.0f64)
    }) as libc::c_long;
    sign = if q % 2 as libc::c_int as libc::c_long != 0 {
        -(1 as libc::c_int)
    } else {
        1 as libc::c_int
    };
    if fabs(fracx) == 0.5f64 {
        if fracx < 0.0f64 {
            sign = -sign;
        }
        (*result).val = if sign != 1 as libc::c_int { -1.0f64 } else { 1.0f64 };
        return GSL_SUCCESS as libc::c_int;
    }
    if fabs(fracx) > 0.5f64 {
        sign = -sign;
        fracx = if fracx > 0.0f64 { fracx - 1.0f64 } else { fracx + 1.0f64 };
    }
    status = 0 as libc::c_int;
    if fracx > 0.25f64 {
        status = cos_pi_taylor(fracx - 0.5f64, result);
    } else if fracx < -0.25f64 {
        status = cos_pi_taylor(fracx + 0.5f64, result);
        sign = -sign;
    } else {
        status = sin_pi_taylor(fracx, result);
    }
    if sign != 1 as libc::c_int {
        (*result).val = -(*result).val;
    }
    return status;
}
pub unsafe extern "C" fn gsl_sf_cos_pi_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut intx: libc::c_double = 0.0f64;
    let mut fracx: libc::c_double = 0.0f64;
    let mut q: libc::c_long = 0;
    let mut sign: libc::c_int = 1 as libc::c_int;
    let mut status: libc::c_int = 0;
    (*result).val = 0.0f64;
    (*result).err = 0.0f64;
    fracx = modf(x, &mut intx);
    if fabs(fracx) == 0.5f64 {
        return GSL_SUCCESS as libc::c_int;
    }
    if fabs(intx) >= 2.0f64 / 2.2204460492503131e-16f64 {
        (*result).val = 1.0f64;
        return GSL_SUCCESS as libc::c_int;
    }
    q = (if intx
        >= (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long) as libc::c_double
        && intx <= 9223372036854775807 as libc::c_long as libc::c_double
    {
        intx
    } else {
        fmod(intx, 2.0f64)
    }) as libc::c_long;
    sign = if q % 2 as libc::c_int as libc::c_long != 0 {
        -(1 as libc::c_int)
    } else {
        1 as libc::c_int
    };
    if fracx == 0.0f64 {
        (*result).val = if sign != 1 as libc::c_int { -1.0f64 } else { 1.0f64 };
        return GSL_SUCCESS as libc::c_int;
    }
    if fabs(fracx) > 0.5f64 {
        sign = -sign;
        fracx = if fracx > 0.0f64 { fracx - 1.0f64 } else { fracx + 1.0f64 };
    }
    status = 0 as libc::c_int;
    if fracx > 0.25f64 {
        status = sin_pi_taylor(fracx - 0.5f64, result);
        sign = -sign;
    } else if fracx < -0.25f64 {
        status = sin_pi_taylor(fracx + 0.5f64, result);
    } else {
        status = cos_pi_taylor(fracx, result);
    }
    if sign != 1 as libc::c_int {
        (*result).val = -(*result).val;
    }
    return status;
}
pub unsafe extern "C" fn gsl_sf_sin_pi(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_sin_pi_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_sin_pi_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"sincos_pi.c\0" as *const u8 as *const libc::c_char,
            235 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_cos_pi(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_cos_pi_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_cos_pi_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"sincos_pi.c\0" as *const u8 as *const libc::c_char,
            241 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
