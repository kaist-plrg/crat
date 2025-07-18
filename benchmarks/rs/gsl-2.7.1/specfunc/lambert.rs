use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
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
#[inline]
unsafe extern "C" fn GSL_MAX_DBL(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return if a > b { a } else { b };
}
unsafe extern "C" fn halley_iteration(
    mut x: libc::c_double,
    mut w_initial: libc::c_double,
    mut max_iters: libc::c_uint,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut w: libc::c_double = w_initial;
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < max_iters {
        let mut tol: libc::c_double = 0.;
        let e: libc::c_double = exp(w);
        let p: libc::c_double = w + 1.0f64;
        let mut t: libc::c_double = w * e - x;
        if w > 0 as libc::c_int as libc::c_double {
            t = t / p / e;
        } else {
            t /= e * p - 0.5f64 * (p + 1.0f64) * t / p;
        }
        w -= t;
        tol = 10 as libc::c_int as libc::c_double * 2.2204460492503131e-16f64
            * GSL_MAX_DBL(fabs(w), 1.0f64 / (fabs(p) * e));
        if fabs(t) < tol {
            (*result).val = w;
            (*result).err = 2.0f64 * tol;
            return GSL_SUCCESS as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    (*result).val = w;
    (*result).err = fabs(w);
    return GSL_EMAXITER as libc::c_int;
}
unsafe extern "C" fn series_eval(mut r: libc::c_double) -> libc::c_double {
    static mut c: [libc::c_double; 12] = [
        -1.0f64,
        2.331643981597124203363536062168f64,
        -1.812187885639363490240191647568f64,
        1.936631114492359755363277457668f64,
        -2.353551201881614516821543561516f64,
        3.066858901050631912893148922704f64,
        -4.175335600258177138854984177460f64,
        5.858023729874774148815053846119f64,
        -8.401032217523977370984161688514f64,
        12.250753501314460424f64,
        -18.100697012472442755f64,
        27.029044799010561650f64,
    ];
    let t_8: libc::c_double = c[8 as libc::c_int as usize]
        + r
            * (c[9 as libc::c_int as usize]
                + r
                    * (c[10 as libc::c_int as usize]
                        + r * c[11 as libc::c_int as usize]));
    let t_5: libc::c_double = c[5 as libc::c_int as usize]
        + r
            * (c[6 as libc::c_int as usize]
                + r * (c[7 as libc::c_int as usize] + r * t_8));
    let t_1: libc::c_double = c[1 as libc::c_int as usize]
        + r
            * (c[2 as libc::c_int as usize]
                + r
                    * (c[3 as libc::c_int as usize]
                        + r * (c[4 as libc::c_int as usize] + r * t_5)));
    return c[0 as libc::c_int as usize] + r * t_1;
}
pub unsafe extern "C" fn gsl_sf_lambert_W0_e(
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let one_over_E: libc::c_double = 1.0f64 / 2.7182818284590452354f64;
    let q: libc::c_double = x + one_over_E;
    if x == 0.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if q < 0.0f64 {
        (*result).val = -1.0f64;
        (*result).err = sqrt(-q);
        return GSL_EDOM as libc::c_int;
    } else if q == 0.0f64 {
        (*result).val = -1.0f64;
        (*result).err = 2.2204460492503131e-16f64;
        return GSL_SUCCESS as libc::c_int;
    } else if q < 1.0e-03f64 {
        let r: libc::c_double = sqrt(q);
        (*result).val = series_eval(r);
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        static mut MAX_ITERS: libc::c_uint = 10 as libc::c_int as libc::c_uint;
        let mut w: libc::c_double = 0.;
        if x < 1.0f64 {
            let p: libc::c_double = sqrt(2.0f64 * 2.7182818284590452354f64 * q);
            w = -1.0f64 + p * (1.0f64 + p * (-1.0f64 / 3.0f64 + p * 11.0f64 / 72.0f64));
        } else {
            w = log(x);
            if x > 3.0f64 {
                w -= log(w);
            }
        }
        return halley_iteration(x, w, MAX_ITERS, result);
    };
}
pub unsafe extern "C" fn gsl_sf_lambert_Wm1_e(
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x > 0.0f64 {
        return gsl_sf_lambert_W0_e(x, result)
    } else if x == 0.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else {
        static mut MAX_ITERS: libc::c_uint = 32 as libc::c_int as libc::c_uint;
        let one_over_E: libc::c_double = 1.0f64 / 2.7182818284590452354f64;
        let q: libc::c_double = x + one_over_E;
        let mut w: libc::c_double = 0.;
        if q < 0.0f64 {
            (*result).val = -1.0f64;
            (*result).err = sqrt(-q);
            return GSL_EDOM as libc::c_int;
        }
        if x < -1.0e-6f64 {
            let r: libc::c_double = -sqrt(q);
            w = series_eval(r);
            if q < 3.0e-3f64 {
                (*result).val = w;
                (*result).err = 5.0f64 * 2.2204460492503131e-16f64 * fabs(w);
                return GSL_SUCCESS as libc::c_int;
            }
        } else {
            let L_1: libc::c_double = log(-x);
            let L_2: libc::c_double = log(-L_1);
            w = L_1 - L_2 + L_2 / L_1;
        }
        return halley_iteration(x, w, MAX_ITERS, result);
    };
}
pub unsafe extern "C" fn gsl_sf_lambert_W0(mut x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_lambert_W0_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_lambert_W0_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"lambert.c\0" as *const u8 as *const libc::c_char,
            225 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_lambert_Wm1(mut x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_lambert_Wm1_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_lambert_Wm1_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"lambert.c\0" as *const u8 as *const libc::c_char,
            230 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
