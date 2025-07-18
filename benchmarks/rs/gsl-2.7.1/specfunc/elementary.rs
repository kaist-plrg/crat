use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_coerce_double(x: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_sf_result_struct {
    pub val: libc::c_double,
    pub err: libc::c_double,
}
pub type gsl_sf_result = gsl_sf_result_struct;
#[inline]
unsafe extern "C" fn GSL_MAX_DBL(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn GSL_MIN_DBL(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return if a < b { a } else { b };
}
pub unsafe extern "C" fn gsl_sf_multiply_e(
    x: libc::c_double,
    y: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let ax: libc::c_double = fabs(x);
    let ay: libc::c_double = fabs(y);
    if x == 0.0f64 || y == 0.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if ax <= 1.0f64 && ay >= 1.0f64 || ay <= 1.0f64 && ax >= 1.0f64 {
        (*result).val = x * y;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let f: libc::c_double = 1.0f64 - 2.0f64 * 2.2204460492503131e-16f64;
        let min: libc::c_double = GSL_MIN_DBL(fabs(x), fabs(y));
        let max: libc::c_double = GSL_MAX_DBL(fabs(x), fabs(y));
        if max < 0.9f64 * 1.3407807929942596e+154f64
            || min < f * 1.7976931348623157e+308f64 / max
        {
            (*result).val = gsl_coerce_double(x * y);
            (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
            if fabs((*result).val) < 2.2250738585072014e-308f64 {
                gsl_error(
                    b"underflow\0" as *const u8 as *const libc::c_char,
                    b"elementary.c\0" as *const u8 as *const libc::c_char,
                    57 as libc::c_int,
                    GSL_EUNDRFLW as libc::c_int,
                );
                return GSL_EUNDRFLW as libc::c_int;
            }
            return GSL_SUCCESS as libc::c_int;
        } else {
            (*result).val = ::std::f32::INFINITY as libc::c_double;
            (*result).err = ::std::f32::INFINITY as libc::c_double;
            gsl_error(
                b"overflow\0" as *const u8 as *const libc::c_char,
                b"elementary.c\0" as *const u8 as *const libc::c_char,
                61 as libc::c_int,
                GSL_EOVRFLW as libc::c_int,
            );
            return GSL_EOVRFLW as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn gsl_sf_multiply_err_e(
    x: libc::c_double,
    dx: libc::c_double,
    y: libc::c_double,
    dy: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_sf_multiply_e(x, y, result);
    (*result).err += fabs(dx * y) + fabs(dy * x);
    return status;
}
pub unsafe extern "C" fn gsl_sf_multiply(
    x: libc::c_double,
    y: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_multiply_e(x, y, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_multiply_e(x, y, &result)\0" as *const u8 as *const libc::c_char,
            b"elementary.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
