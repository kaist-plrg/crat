use ::libc;
extern "C" {
    fn acos(_: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
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
pub unsafe extern "C" fn gsl_sf_gegenpoly_1_e(
    mut lambda: libc::c_double,
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if lambda == 0.0f64 {
        (*result).val = 2.0f64 * x;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*result).val = 2.0f64 * lambda * x;
        (*result).err = 4.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_gegenpoly_2_e(
    mut lambda: libc::c_double,
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if lambda == 0.0f64 {
        let txx: libc::c_double = 2.0f64 * x * x;
        (*result).val = -1.0f64 + txx;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs(txx);
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*result).val = lambda * (-1.0f64 + 2.0f64 * (1.0f64 + lambda) * x * x);
        (*result)
            .err = 2.2204460492503131e-16f64
            * (2.0f64 * fabs((*result).val) + fabs(lambda));
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_gegenpoly_3_e(
    mut lambda: libc::c_double,
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if lambda == 0.0f64 {
        (*result).val = x * (-2.0f64 + 4.0f64 / 3.0f64 * x * x);
        (*result)
            .err = 2.2204460492503131e-16f64 * (2.0f64 * fabs((*result).val) + fabs(x));
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut c: libc::c_double = 4.0f64 + lambda * (6.0f64 + 2.0f64 * lambda);
        (*result).val = 2.0f64 * lambda * x * (-1.0f64 - lambda + c * x * x / 3.0f64);
        (*result)
            .err = 2.2204460492503131e-16f64
            * (2.0f64 * fabs((*result).val) + fabs(lambda * x));
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_gegenpoly_n_e(
    mut n: libc::c_int,
    mut lambda: libc::c_double,
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if lambda <= -0.5f64 || n < 0 as libc::c_int {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"gegenbauer.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if n == 0 as libc::c_int {
        (*result).val = 1.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if n == 1 as libc::c_int {
        return gsl_sf_gegenpoly_1_e(lambda, x, result)
    } else if n == 2 as libc::c_int {
        return gsl_sf_gegenpoly_2_e(lambda, x, result)
    } else if n == 3 as libc::c_int {
        return gsl_sf_gegenpoly_3_e(lambda, x, result)
    } else if lambda == 0.0f64 && (x >= -1.0f64 && x <= 1.0f64) {
        let z: libc::c_double = n as libc::c_double * acos(x);
        (*result).val = 2.0f64 * cos(z) / n as libc::c_double;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs(z * (*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut k: libc::c_int = 0;
        let mut g2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut g3: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_g2: libc::c_int = gsl_sf_gegenpoly_2_e(lambda, x, &mut g2);
        let mut stat_g3: libc::c_int = gsl_sf_gegenpoly_3_e(lambda, x, &mut g3);
        let mut stat_g: libc::c_int = if stat_g2 != GSL_SUCCESS as libc::c_int {
            stat_g2
        } else if stat_g3 != GSL_SUCCESS as libc::c_int {
            stat_g3
        } else {
            GSL_SUCCESS as libc::c_int
        };
        let mut gkm2: libc::c_double = g2.val;
        let mut gkm1: libc::c_double = g3.val;
        let mut gk: libc::c_double = 0.0f64;
        k = 4 as libc::c_int;
        while k <= n {
            gk = (2.0f64 * (k as libc::c_double + lambda - 1.0f64) * x * gkm1
                - (k as libc::c_double + 2.0f64 * lambda - 2.0f64) * gkm2)
                / k as libc::c_double;
            gkm2 = gkm1;
            gkm1 = gk;
            k += 1;
            k;
        }
        (*result).val = gk;
        (*result)
            .err = 2.0f64 * 2.2204460492503131e-16f64 * 0.5f64 * n as libc::c_double
            * fabs(gk);
        return stat_g;
    };
}
pub unsafe extern "C" fn gsl_sf_gegenpoly_array(
    mut nmax: libc::c_int,
    mut lambda: libc::c_double,
    mut x: libc::c_double,
    mut result_array: *mut libc::c_double,
) -> libc::c_int {
    let mut k: libc::c_int = 0;
    if lambda <= -0.5f64 || nmax < 0 as libc::c_int {
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"gegenbauer.c\0" as *const u8 as *const libc::c_char,
            148 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    }
    *result_array.offset(0 as libc::c_int as isize) = 1.0f64;
    if nmax == 0 as libc::c_int {
        return GSL_SUCCESS as libc::c_int;
    }
    if lambda == 0.0f64 {
        *result_array.offset(1 as libc::c_int as isize) = 2.0f64 * x;
    } else {
        *result_array.offset(1 as libc::c_int as isize) = 2.0f64 * lambda * x;
    }
    k = 2 as libc::c_int;
    while k <= nmax {
        let mut term1: libc::c_double = 2.0f64 * (k as libc::c_double + lambda - 1.0f64)
            * x * *result_array.offset((k - 1 as libc::c_int) as isize);
        let mut term2: libc::c_double = (k as libc::c_double + 2.0f64 * lambda - 2.0f64)
            * *result_array.offset((k - 2 as libc::c_int) as isize);
        *result_array.offset(k as isize) = (term1 - term2) / k as libc::c_double;
        k += 1;
        k;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sf_gegenpoly_1(
    mut lambda: libc::c_double,
    mut x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_gegenpoly_1_e(lambda, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_gegenpoly_1_e(lambda, x, &result)\0" as *const u8
                as *const libc::c_char,
            b"gegenbauer.c\0" as *const u8 as *const libc::c_char,
            178 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_gegenpoly_2(
    mut lambda: libc::c_double,
    mut x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_gegenpoly_2_e(lambda, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_gegenpoly_2_e(lambda, x, &result)\0" as *const u8
                as *const libc::c_char,
            b"gegenbauer.c\0" as *const u8 as *const libc::c_char,
            183 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_gegenpoly_3(
    mut lambda: libc::c_double,
    mut x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_gegenpoly_3_e(lambda, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_gegenpoly_3_e(lambda, x, &result)\0" as *const u8
                as *const libc::c_char,
            b"gegenbauer.c\0" as *const u8 as *const libc::c_char,
            188 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_gegenpoly_n(
    mut n: libc::c_int,
    mut lambda: libc::c_double,
    mut x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_gegenpoly_n_e(n, lambda, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_gegenpoly_n_e(n, lambda, x, &result)\0" as *const u8
                as *const libc::c_char,
            b"gegenbauer.c\0" as *const u8 as *const libc::c_char,
            193 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
