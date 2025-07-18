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
#[inline]
unsafe extern "C" fn GSL_MIN_DBL(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return if a < b { a } else { b };
}
pub unsafe extern "C" fn gsl_min_test_interval(
    mut x_lower: libc::c_double,
    mut x_upper: libc::c_double,
    mut epsabs: libc::c_double,
    mut epsrel: libc::c_double,
) -> libc::c_int {
    let lower: libc::c_double = x_lower;
    let upper: libc::c_double = x_upper;
    let abs_lower: libc::c_double = fabs(lower);
    let abs_upper: libc::c_double = fabs(upper);
    let mut min_abs: libc::c_double = 0.;
    let mut tolerance: libc::c_double = 0.;
    if epsrel < 0.0f64 {
        gsl_error(
            b"relative tolerance is negative\0" as *const u8 as *const libc::c_char,
            b"convergence.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            GSL_EBADTOL as libc::c_int,
        );
        return GSL_EBADTOL as libc::c_int;
    }
    if epsabs < 0.0f64 {
        gsl_error(
            b"absolute tolerance is negative\0" as *const u8 as *const libc::c_char,
            b"convergence.c\0" as *const u8 as *const libc::c_char,
            40 as libc::c_int,
            GSL_EBADTOL as libc::c_int,
        );
        return GSL_EBADTOL as libc::c_int;
    }
    if lower > upper {
        gsl_error(
            b"lower bound larger than upper_bound\0" as *const u8 as *const libc::c_char,
            b"convergence.c\0" as *const u8 as *const libc::c_char,
            43 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if lower > 0 as libc::c_int as libc::c_double
        && upper > 0 as libc::c_int as libc::c_double
        || lower < 0 as libc::c_int as libc::c_double
            && upper < 0 as libc::c_int as libc::c_double
    {
        min_abs = GSL_MIN_DBL(abs_lower, abs_upper);
    } else {
        min_abs = 0 as libc::c_int as libc::c_double;
    }
    tolerance = epsabs + epsrel * min_abs;
    if fabs(upper - lower) < tolerance {
        return GSL_SUCCESS as libc::c_int;
    }
    return GSL_CONTINUE as libc::c_int;
}
