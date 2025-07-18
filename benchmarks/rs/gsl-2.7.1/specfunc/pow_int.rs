use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
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
pub unsafe extern "C" fn gsl_sf_pow_int_e(
    mut x: libc::c_double,
    mut n: libc::c_int,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut value: libc::c_double = 1.0f64;
    let mut count: libc::c_int = 0 as libc::c_int;
    if n < 0 as libc::c_int {
        n = -n;
        if x == 0.0f64 {
            let mut u: libc::c_double = 1.0f64 / x;
            (*result).val = if n % 2 as libc::c_int != 0 { u } else { u * u };
            (*result).err = ::std::f32::INFINITY as libc::c_double;
            gsl_error(
                b"overflow\0" as *const u8 as *const libc::c_char,
                b"pow_int.c\0" as *const u8 as *const libc::c_char,
                45 as libc::c_int,
                GSL_EOVRFLW as libc::c_int,
            );
            return GSL_EOVRFLW as libc::c_int;
        }
        x = 1.0f64 / x;
    }
    loop {
        if n & 1 as libc::c_int != 0 {
            value *= x;
        }
        n >>= 1 as libc::c_int;
        x *= x;
        count += 1;
        count;
        if !(n != 0) {
            break;
        }
    }
    (*result).val = value;
    (*result)
        .err = 2.0f64 * 2.2204460492503131e-16f64 * (count as libc::c_double + 1.0f64)
        * fabs(value);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sf_pow_int(
    x: libc::c_double,
    n: libc::c_int,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_pow_int_e(x, n, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_pow_int_e(x, n, &result)\0" as *const u8 as *const libc::c_char,
            b"pow_int.c\0" as *const u8 as *const libc::c_char,
            73 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
