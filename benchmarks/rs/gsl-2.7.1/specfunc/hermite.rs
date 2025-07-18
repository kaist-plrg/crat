use ::libc;
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sf_airy_zero_Ai(s: libc::c_uint) -> libc::c_double;
    fn acos(_: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_fcmp(
        x1: libc::c_double,
        x2: libc::c_double,
        epsilon: libc::c_double,
    ) -> libc::c_int;
    fn gsl_pow_int(x: libc::c_double, n: libc::c_int) -> libc::c_double;
    fn gsl_sf_pow_int(x: libc::c_double, n: libc::c_int) -> libc::c_double;
    fn gsl_sf_fact(n: libc::c_uint) -> libc::c_double;
    fn gsl_sf_doublefact_e(n: libc::c_uint, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_lnfact_e(n: libc::c_uint, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_choose(n: libc::c_uint, m: libc::c_uint) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
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
unsafe extern "C" fn GSL_MAX_INT(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn GSL_MIN_INT(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
pub unsafe extern "C" fn gsl_sf_hermite_prob_e(
    n: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if n < 0 as libc::c_int {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"hermite.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if n == 0 as libc::c_int {
        (*result).val = 1.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if n == 1 as libc::c_int {
        (*result).val = x;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if x == 0.0f64 {
        if n & 1 as libc::c_int != 0 {
            (*result).val = 0.0f64;
            (*result).err = 0.0f64;
            return GSL_SUCCESS as libc::c_int;
        } else {
            let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
            if n - 1 as libc::c_int > 297 as libc::c_int {
                status = GSL_EOVRFLW as libc::c_int;
                (*result)
                    .val = (if n / 2 as libc::c_int & 1 as libc::c_int != 0 {
                    -::std::f32::INFINITY
                } else {
                    ::std::f32::INFINITY
                }) as libc::c_double;
                (*result).err = ::std::f32::INFINITY as libc::c_double;
            } else {
                gsl_sf_doublefact_e((n - 1 as libc::c_int) as libc::c_uint, result);
                if n / 2 as libc::c_int & 1 as libc::c_int != 0 {
                    (*result).val = -(*result).val;
                }
            }
            return status;
        }
    } else {
        let mut status_0: libc::c_int = GSL_SUCCESS as libc::c_int;
        let abs_x: libc::c_double = fabs(x);
        let thresh1: libc::c_double = if abs_x > 1.0f64 {
            0.9f64 * 1.7976931348623157e+308f64 / abs_x
        } else {
            1.7976931348623157e+308f64
        };
        let thresh2: libc::c_double = 0.9f64 * 1.7976931348623157e+308f64;
        let mut p_n0: libc::c_double = 1.0f64;
        let mut p_n1: libc::c_double = x;
        let mut p_n: libc::c_double = p_n1;
        let mut e_n0: libc::c_double = 2.2204460492503131e-16f64;
        let mut e_n1: libc::c_double = fabs(x) * 2.2204460492503131e-16f64;
        let mut e_n: libc::c_double = e_n1;
        let mut j: libc::c_int = 0;
        j = 1 as libc::c_int;
        while j < n {
            if fabs(p_n1) > thresh1 || fabs(p_n0) > thresh2 / j as libc::c_double {
                status_0 = GSL_EOVRFLW as libc::c_int;
                break;
            } else {
                p_n = x * p_n1 - j as libc::c_double * p_n0;
                p_n0 = p_n1;
                p_n1 = p_n;
                e_n = fabs(x) * e_n1 + j as libc::c_double * e_n0;
                e_n0 = e_n1;
                e_n1 = e_n;
                j += 1;
                j;
            }
        }
        (*result).val = p_n;
        (*result).err = e_n + fabs((*result).val) * 2.2204460492503131e-16f64;
        return status_0;
    };
}
pub unsafe extern "C" fn gsl_sf_hermite_prob(
    n: libc::c_int,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_hermite_prob_e(n, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_hermite_prob_e(n, x, &result)\0" as *const u8
                as *const libc::c_char,
            b"hermite.c\0" as *const u8 as *const libc::c_char,
            143 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_hermite_prob_deriv_e(
    m: libc::c_int,
    n: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if n < 0 as libc::c_int || m < 0 as libc::c_int {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"hermite.c\0" as *const u8 as *const libc::c_char,
            153 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if n < m {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut f: libc::c_double = gsl_sf_choose(n as libc::c_uint, m as libc::c_uint)
            * gsl_sf_fact(m as libc::c_uint);
        let mut He: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        status = gsl_sf_hermite_prob_e(n - m, x, &mut He);
        if status == GSL_SUCCESS as libc::c_int {
            (*result).val = He.val * f;
            (*result).err = He.err * f + 2.2204460492503131e-16f64 * fabs((*result).val);
        } else {
            (*result).val = He.val;
            (*result).err = ::std::f32::INFINITY as libc::c_double;
        }
        return status;
    };
}
pub unsafe extern "C" fn gsl_sf_hermite_prob_deriv(
    m: libc::c_int,
    n: libc::c_int,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_hermite_prob_deriv_e(m, n, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_hermite_prob_deriv_e(m, n, x, &result)\0" as *const u8
                as *const libc::c_char,
            b"hermite.c\0" as *const u8 as *const libc::c_char,
            186 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_hermite_e(
    n: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if n < 0 as libc::c_int {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"hermite.c\0" as *const u8 as *const libc::c_char,
            195 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if n == 0 as libc::c_int {
        (*result).val = 1.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if n == 1 as libc::c_int {
        (*result).val = 2.0f64 * x;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if x == 0.0f64 {
        if n & 1 as libc::c_int != 0 {
            (*result).val = 0.0f64;
            (*result).err = 0.0f64;
            return GSL_SUCCESS as libc::c_int;
        } else {
            let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
            let mut m: libc::c_int = n >> 1 as libc::c_int;
            if n - 1 as libc::c_int > 297 as libc::c_int {
                status = GSL_EOVRFLW as libc::c_int;
                (*result)
                    .val = (if m & 1 as libc::c_int != 0 {
                    -::std::f32::INFINITY
                } else {
                    ::std::f32::INFINITY
                }) as libc::c_double;
                (*result).err = ::std::f32::INFINITY as libc::c_double;
            } else {
                let mut f: libc::c_double = gsl_pow_int(2.0f64, m);
                gsl_sf_doublefact_e((n - 1 as libc::c_int) as libc::c_uint, result);
                if (*result).val > 0.9f64 * 1.7976931348623157e+308f64 / f {
                    status = GSL_EOVRFLW as libc::c_int;
                    (*result)
                        .val = (if m & 1 as libc::c_int != 0 {
                        -::std::f32::INFINITY
                    } else {
                        ::std::f32::INFINITY
                    }) as libc::c_double;
                    (*result).err = ::std::f32::INFINITY as libc::c_double;
                } else {
                    (*result).val *= f;
                    (*result).err *= f;
                    if m & 1 as libc::c_int != 0 {
                        (*result).val = -(*result).val;
                    }
                }
            }
            return status;
        }
    } else {
        let mut status_0: libc::c_int = GSL_SUCCESS as libc::c_int;
        let two_x: libc::c_double = 2.0f64 * x;
        let abs_two_x: libc::c_double = fabs(two_x);
        let thresh1: libc::c_double = if abs_two_x > 1.0f64 {
            0.9f64 * 1.7976931348623157e+308f64 / abs_two_x
        } else {
            1.7976931348623157e+308f64
        };
        let thresh2: libc::c_double = 0.9f64 * 1.7976931348623157e+308f64 / 2.0f64;
        let mut p_n0: libc::c_double = 1.0f64;
        let mut p_n1: libc::c_double = two_x;
        let mut p_n: libc::c_double = p_n1;
        let mut e_n0: libc::c_double = 2.2204460492503131e-16f64;
        let mut e_n1: libc::c_double = 2.0f64 * fabs(x) * 2.2204460492503131e-16f64;
        let mut e_n: libc::c_double = e_n1;
        let mut j: libc::c_int = 0;
        j = 1 as libc::c_int;
        while j <= n - 1 as libc::c_int {
            if fabs(p_n1) > thresh1 || fabs(p_n0) > thresh2 / j as libc::c_double {
                status_0 = GSL_EOVRFLW as libc::c_int;
                break;
            } else {
                p_n = two_x * p_n1 - 2.0f64 * j as libc::c_double * p_n0;
                p_n0 = p_n1;
                p_n1 = p_n;
                e_n = 2.0f64 * (fabs(x) * e_n1 + j as libc::c_double * e_n0);
                e_n0 = e_n1;
                e_n1 = e_n;
                j += 1;
                j;
            }
        }
        (*result).val = p_n;
        (*result).err = e_n + fabs((*result).val) * 2.2204460492503131e-16f64;
        return status_0;
    };
}
pub unsafe extern "C" fn gsl_sf_hermite(
    n: libc::c_int,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_hermite_e(n, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_hermite_e(n, x, &result)\0" as *const u8 as *const libc::c_char,
            b"hermite.c\0" as *const u8 as *const libc::c_char,
            302 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_hermite_deriv_e(
    m: libc::c_int,
    n: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if n < 0 as libc::c_int || m < 0 as libc::c_int {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"hermite.c\0" as *const u8 as *const libc::c_char,
            312 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if n < m {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut f: libc::c_double = gsl_sf_choose(n as libc::c_uint, m as libc::c_uint)
            * gsl_sf_fact(m as libc::c_uint)
            * gsl_sf_pow_int(2 as libc::c_int as libc::c_double, m);
        let mut H: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        status = gsl_sf_hermite_e(n - m, x, &mut H);
        if status == GSL_SUCCESS as libc::c_int {
            (*result).val = H.val * f;
            (*result).err = H.err * f + 2.2204460492503131e-16f64 * fabs((*result).val);
        } else {
            (*result).val = H.val;
            (*result).err = ::std::f32::INFINITY as libc::c_double;
        }
        return status;
    };
}
pub unsafe extern "C" fn gsl_sf_hermite_deriv(
    m: libc::c_int,
    n: libc::c_int,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_hermite_deriv_e(m, n, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_hermite_deriv_e(m, n, x, &result)\0" as *const u8
                as *const libc::c_char,
            b"hermite.c\0" as *const u8 as *const libc::c_char,
            345 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_hermite_func_e(
    n: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if n < 0 as libc::c_int {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"hermite.c\0" as *const u8 as *const libc::c_char,
            354 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x == 0.0f64 {
        if n & 1 as libc::c_int != 0 {
            (*result).val = 0.0f64;
            (*result).err = 0.0f64;
            return GSL_SUCCESS as libc::c_int;
        } else {
            let mut f: libc::c_double = if n / 2 as libc::c_int & 1 as libc::c_int != 0 {
                -1.0f64
            } else {
                1.0f64
            };
            let mut j: libc::c_int = 0;
            j = 1 as libc::c_int;
            while j < n {
                f *= sqrt(j as libc::c_double / (j as libc::c_double + 1.0f64));
                j += 2 as libc::c_int;
            }
            (*result).val = f / sqrt(1.77245385090551602729816748334f64);
            (*result).err = 2.2204460492503131e-16f64 * fabs((*result).val);
            return GSL_SUCCESS as libc::c_int;
        }
    } else if n == 0 as libc::c_int {
        (*result).val = exp(-0.5f64 * x * x) / sqrt(1.77245385090551602729816748334f64);
        (*result).err = 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if n == 1 as libc::c_int {
        (*result)
            .val = 1.41421356237309504880f64 * x * exp(-0.5f64 * x * x)
            / sqrt(1.77245385090551602729816748334f64);
        (*result).err = 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut hi2: libc::c_double = 1.0f64 / sqrt(1.77245385090551602729816748334f64);
        let mut hi1: libc::c_double = 1.41421356237309504880f64 * x * hi2;
        let mut hi: libc::c_double = 0.0f64;
        let mut sum_log_scale: libc::c_double = 0.0f64;
        let mut abshi: libc::c_double = 0.;
        let mut i: libc::c_int = 0;
        i = 2 as libc::c_int;
        while i <= n {
            hi = sqrt(2.0f64 / i as libc::c_double) * x * hi1
                - sqrt((i as libc::c_double - 1.0f64) / i as libc::c_double) * hi2;
            hi2 = hi1;
            hi1 = hi;
            abshi = fabs(hi);
            if abshi > 1.0f64 {
                let mut log_scale: libc::c_double = (if log(abshi)
                    >= 0 as libc::c_int as libc::c_double
                {
                    (log(abshi) + 0.5f64) as libc::c_int
                } else {
                    (log(abshi) - 0.5f64) as libc::c_int
                }) as libc::c_double;
                let mut scale: libc::c_double = exp(-log_scale);
                hi *= scale;
                hi1 *= scale;
                hi2 *= scale;
                sum_log_scale += log_scale;
            }
            i += 1;
            i;
        }
        (*result).val = hi * exp(-0.5f64 * x * x + sum_log_scale);
        (*result)
            .err = n as libc::c_double * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_hermite_func(
    n: libc::c_int,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_hermite_func_e(n, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_hermite_func_e(n, x, &result)\0" as *const u8
                as *const libc::c_char,
            b"hermite.c\0" as *const u8 as *const libc::c_char,
            445 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_hermite_func_fast_e(
    n: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if n < 1000 as libc::c_int || x == 0.0f64 {
        return gsl_sf_hermite_func_e(n, x, result)
    } else {
        let mut j: size_t = 0;
        let k: libc::c_double = sqrt(0.5f64 * n as libc::c_double);
        let steps: size_t = ceil(6.211f64 * sqrt(n as libc::c_double)) as size_t;
        let dt: libc::c_double = 3.14159265358979323846f64 / steps as libc::c_double;
        let invn2: libc::c_double = 1.0f64 / (n * n) as libc::c_double;
        let mut ex: libc::c_double = 0.;
        let mut ex_e: libc::c_double = 0.;
        let mut cs: libc::c_double = 0.;
        let mut cs_e: libc::c_double = 0.;
        let mut sn: libc::c_double = 0.;
        let mut sn2: libc::c_double = 0.;
        let mut t: libc::c_double = 0.;
        let mut lngamma: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        if n < 36 as libc::c_int {
            gsl_sf_lnfact_e(n as libc::c_uint, &mut lngamma);
            lngamma.val *= 0.5f64;
            lngamma.err *= 0.5f64;
            t = 0.5f64 * n as libc::c_double * log(n as libc::c_double)
                + 0.25f64 * 1.14472988584940017414342735135f64;
            cs = 0.5f64 * n as libc::c_double;
            lngamma.val += cs - t;
            lngamma.err += (cs + t) * 2.2204460492503131e-16f64;
        } else {
            lngamma.val = 0.25f64 * log((2 as libc::c_int * n) as libc::c_double);
            lngamma
                .err = (lngamma.val
                + (((invn2 / 3360 as libc::c_int as libc::c_double
                    + 1.0f64 / 2520 as libc::c_int as libc::c_double) * invn2
                    + 1.0f64 / 720 as libc::c_int as libc::c_double) * invn2
                    + 1.0f64 / 24 as libc::c_int as libc::c_double)
                    / n as libc::c_double) * 2.2204460492503131e-16f64;
            lngamma.val
                -= (((invn2 / 3360 as libc::c_int as libc::c_double
                    - 1.0f64 / 2520 as libc::c_int as libc::c_double) * invn2
                    + 1.0f64 / 720 as libc::c_int as libc::c_double) * invn2
                    - 1.0f64 / 24 as libc::c_int as libc::c_double)
                    / n as libc::c_double;
        }
        ex = exp(
            lngamma.val - n as libc::c_double - 0.5f64 * x * x
                - 2 as libc::c_int as libc::c_double * x * k,
        );
        cs = (if n & 1 as libc::c_int != 0 {
            -(1 as libc::c_int)
        } else {
            1 as libc::c_int
        }) as libc::c_double;
        (*result).val = 0.5f64 * ex * cs;
        (*result)
            .err = 0.5f64 * ex
            * (lngamma.err
                + (n as libc::c_double + 0.5f64 * x * x
                    + fabs(2 as libc::c_int as libc::c_double * x * k)
                    + 1 as libc::c_int as libc::c_double) * 2.2204460492503131e-16f64);
        ex = exp(
            lngamma.val - n as libc::c_double - 0.5f64 * x * x
                + 2 as libc::c_int as libc::c_double * x * k,
        );
        (*result).val += 0.5f64 * ex;
        (*result).err
            += 0.5f64 * ex
                * (lngamma.err
                    + (n as libc::c_double + 0.5f64 * x * x
                        + fabs(2 as libc::c_int as libc::c_double * x * k)
                        + 1 as libc::c_int as libc::c_double)
                        * 2.2204460492503131e-16f64);
        j = 1 as libc::c_int as size_t;
        while j < steps {
            t = j as libc::c_double * dt;
            cs = cos(t);
            ex = exp(
                lngamma.val - 0.5f64 * x * x
                    + (2 as libc::c_int as libc::c_double * x * k
                        - n as libc::c_double * cs) * cs,
            );
            ex_e = ex
                * (lngamma.err
                    + 2.2204460492503131e-16f64
                        * (1 as libc::c_int as libc::c_double + 0.5f64 * x * x
                            + (fabs(2 as libc::c_int as libc::c_double * x * k)
                                + fabs(n as libc::c_double * cs)) * fabs(cs)));
            sn = sin(t);
            sn2 = sin(2 as libc::c_int as libc::c_double * t);
            cs = cos(
                2 as libc::c_int as libc::c_double * x * k * sn
                    - 0.5f64 * n as libc::c_double * sn2 - n as libc::c_double * t,
            );
            cs_e = if 1.0f64 + fabs(cs)
                < 2.2204460492503131e-16f64
                    * (fabs(cs)
                        + (fabs(2 as libc::c_int as libc::c_double * x * k * sn)
                            + fabs(0.5f64 * n as libc::c_double * sn2)
                            + n as libc::c_double * t)
                            * fabs(
                                sin(
                                    2 as libc::c_int as libc::c_double * x * k * sn
                                        - 0.5f64 * n as libc::c_double * sn2
                                        - n as libc::c_double * t,
                                ),
                            ))
            {
                1.0f64 + fabs(cs)
            } else {
                2.2204460492503131e-16f64
                    * (fabs(cs)
                        + (fabs(2 as libc::c_int as libc::c_double * x * k * sn)
                            + fabs(0.5f64 * n as libc::c_double * sn2)
                            + n as libc::c_double * t)
                            * fabs(
                                sin(
                                    2 as libc::c_int as libc::c_double * x * k * sn
                                        - 0.5f64 * n as libc::c_double * sn2
                                        - n as libc::c_double * t,
                                ),
                            ))
            };
            (*result).val += ex * cs;
            (*result).err
                += ex * cs_e + ex_e * fabs(cs)
                    + 2.2204460492503131e-16f64 * fabs(ex * cs);
            j = j.wrapping_add(1);
            j;
        }
        (*result).val *= 0.31830988618379067154f64 * dt;
        (*result)
            .err = 0.31830988618379067154f64 * dt * (*result).err
            + 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_hermite_func_fast(
    n: libc::c_int,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_hermite_func_fast_e(n, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_hermite_func_fast_e(n, x, &result)\0" as *const u8
                as *const libc::c_char,
            b"hermite.c\0" as *const u8 as *const libc::c_char,
            524 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_hermite_prob_array(
    nmax: libc::c_int,
    x: libc::c_double,
    mut result_array: *mut libc::c_double,
) -> libc::c_int {
    if nmax < 0 as libc::c_int {
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"hermite.c\0" as *const u8 as *const libc::c_char,
            534 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if nmax == 0 as libc::c_int {
        *result_array.offset(0 as libc::c_int as isize) = 1.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if nmax == 1 as libc::c_int {
        *result_array.offset(0 as libc::c_int as isize) = 1.0f64;
        *result_array.offset(1 as libc::c_int as isize) = x;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let abs_x: libc::c_double = fabs(x);
        let thresh1: libc::c_double = if abs_x > 1.0f64 {
            0.9f64 * 1.7976931348623157e+308f64 / abs_x
        } else {
            1.7976931348623157e+308f64
        };
        let thresh2: libc::c_double = 0.9f64 * 1.7976931348623157e+308f64;
        let mut p_n0: libc::c_double = 1.0f64;
        let mut p_n1: libc::c_double = x;
        let mut p_n: libc::c_double = p_n1;
        let mut j: libc::c_int = 0;
        *result_array.offset(0 as libc::c_int as isize) = 1.0f64;
        *result_array.offset(1 as libc::c_int as isize) = x;
        j = 1 as libc::c_int;
        while j < nmax {
            if fabs(p_n1) > thresh1 || fabs(p_n0) > thresh2 / j as libc::c_double {
                status = GSL_EOVRFLW as libc::c_int;
                break;
            } else {
                p_n = x * p_n1 - j as libc::c_double * p_n0;
                p_n0 = p_n1;
                p_n1 = p_n;
                *result_array.offset((j + 1 as libc::c_int) as isize) = p_n;
                j += 1;
                j;
            }
        }
        return status;
    };
}
pub unsafe extern "C" fn gsl_sf_hermite_prob_array_deriv(
    m: libc::c_int,
    nmax: libc::c_int,
    x: libc::c_double,
    mut result_array: *mut libc::c_double,
) -> libc::c_int {
    if nmax < 0 as libc::c_int || m < 0 as libc::c_int {
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"hermite.c\0" as *const u8 as *const libc::c_char,
            594 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if m == 0 as libc::c_int {
        gsl_sf_hermite_prob_array(nmax, x, result_array);
        return GSL_SUCCESS as libc::c_int;
    } else if nmax < m {
        let mut j: libc::c_int = 0;
        j = 0 as libc::c_int;
        while j <= nmax {
            *result_array.offset(j as isize) = 0.0f64;
            j += 1;
            j;
        }
        return GSL_SUCCESS as libc::c_int;
    } else if nmax == m {
        let mut j_0: libc::c_int = 0;
        j_0 = 0 as libc::c_int;
        while j_0 < m {
            *result_array.offset(j_0 as isize) = 0.0f64;
            j_0 += 1;
            j_0;
        }
        *result_array.offset(nmax as isize) = gsl_sf_fact(m as libc::c_uint);
        return GSL_SUCCESS as libc::c_int;
    } else if nmax == m + 1 as libc::c_int {
        let mut j_1: libc::c_int = 0;
        j_1 = 0 as libc::c_int;
        while j_1 < m {
            *result_array.offset(j_1 as isize) = 0.0f64;
            j_1 += 1;
            j_1;
        }
        *result_array
            .offset((nmax - 1 as libc::c_int) as isize) = gsl_sf_fact(m as libc::c_uint);
        *result_array
            .offset(
                nmax as isize,
            ) = *result_array.offset((nmax - 1 as libc::c_int) as isize)
            * (m + 1 as libc::c_int) as libc::c_double * x;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut p_n0: libc::c_double = gsl_sf_fact(m as libc::c_uint);
        let mut p_n1: libc::c_double = p_n0 * (m + 1 as libc::c_int) as libc::c_double
            * x;
        let mut p_n: libc::c_double = p_n1;
        let mut j_2: libc::c_int = 0;
        j_2 = 0 as libc::c_int;
        while j_2 < m {
            *result_array.offset(j_2 as isize) = 0.0f64;
            j_2 += 1;
            j_2;
        }
        *result_array.offset(m as isize) = p_n0;
        *result_array.offset((m + 1 as libc::c_int) as isize) = p_n1;
        j_2 = m + 1 as libc::c_int;
        while j_2 <= nmax - 1 as libc::c_int {
            p_n = (x * p_n1 - j_2 as libc::c_double * p_n0)
                * (j_2 as libc::c_double + 1.0f64)
                / ((j_2 - m) as libc::c_double + 1.0f64);
            p_n0 = p_n1;
            p_n1 = p_n;
            *result_array.offset((j_2 + 1 as libc::c_int) as isize) = p_n;
            j_2 += 1;
            j_2;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_hermite_prob_deriv_array(
    mmax: libc::c_int,
    n: libc::c_int,
    x: libc::c_double,
    mut result_array: *mut libc::c_double,
) -> libc::c_int {
    if n < 0 as libc::c_int || mmax < 0 as libc::c_int {
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"hermite.c\0" as *const u8 as *const libc::c_char,
            666 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if n == 0 as libc::c_int {
        let mut j: libc::c_int = 0;
        *result_array.offset(0 as libc::c_int as isize) = 1.0f64;
        j = 1 as libc::c_int;
        while j <= mmax {
            *result_array.offset(j as isize) = 0.0f64;
            j += 1;
            j;
        }
        return GSL_SUCCESS as libc::c_int;
    } else if n == 1 as libc::c_int && mmax > 0 as libc::c_int {
        let mut j_0: libc::c_int = 0;
        *result_array.offset(0 as libc::c_int as isize) = x;
        *result_array.offset(1 as libc::c_int as isize) = 1.0f64;
        j_0 = 2 as libc::c_int;
        while j_0 <= mmax {
            *result_array.offset(j_0 as isize) = 0.0f64;
            j_0 += 1;
            j_0;
        }
        return GSL_SUCCESS as libc::c_int;
    } else if mmax == 0 as libc::c_int {
        *result_array.offset(0 as libc::c_int as isize) = gsl_sf_hermite_prob(n, x);
        return GSL_SUCCESS as libc::c_int;
    } else if mmax == 1 as libc::c_int {
        *result_array.offset(0 as libc::c_int as isize) = gsl_sf_hermite_prob(n, x);
        *result_array
            .offset(
                1 as libc::c_int as isize,
            ) = n as libc::c_double * gsl_sf_hermite_prob(n - 1 as libc::c_int, x);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut k: libc::c_int = GSL_MAX_INT(0 as libc::c_int, n - mmax);
        let mut p_n0: libc::c_double = gsl_sf_hermite_prob(k, x);
        let mut p_n1: libc::c_double = gsl_sf_hermite_prob(k + 1 as libc::c_int, x);
        let mut p_n: libc::c_double = p_n1;
        let mut j_1: libc::c_int = 0;
        j_1 = n + 1 as libc::c_int;
        while j_1 <= mmax {
            *result_array.offset(j_1 as isize) = 0.0f64;
            j_1 += 1;
            j_1;
        }
        *result_array.offset(GSL_MIN_INT(n, mmax) as isize) = p_n0;
        *result_array.offset((GSL_MIN_INT(n, mmax) - 1 as libc::c_int) as isize) = p_n1;
        j_1 = GSL_MIN_INT(mmax, n) - 1 as libc::c_int;
        while j_1 > 0 as libc::c_int {
            k += 1;
            k;
            p_n = x * p_n1 - k as libc::c_double * p_n0;
            p_n0 = p_n1;
            p_n1 = p_n;
            *result_array.offset((j_1 - 1 as libc::c_int) as isize) = p_n;
            j_1 -= 1;
            j_1;
        }
        p_n = 1.0f64;
        j_1 = 1 as libc::c_int;
        while j_1 <= GSL_MIN_INT(n, mmax) {
            p_n = p_n * (n - j_1 + 1 as libc::c_int) as libc::c_double;
            *result_array
                .offset(j_1 as isize) = p_n * *result_array.offset(j_1 as isize);
            j_1 += 1;
            j_1;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_hermite_prob_series_e(
    n: libc::c_int,
    x: libc::c_double,
    mut a: *const libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if n < 0 as libc::c_int {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"hermite.c\0" as *const u8 as *const libc::c_char,
            744 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if n == 0 as libc::c_int {
        (*result).val = *a.offset(0 as libc::c_int as isize);
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if n == 1 as libc::c_int {
        (*result)
            .val = *a.offset(0 as libc::c_int as isize)
            + *a.offset(1 as libc::c_int as isize) * x;
        (*result)
            .err = 2.0f64 * 2.2204460492503131e-16f64
            * (fabs(*a.offset(0 as libc::c_int as isize))
                + fabs(*a.offset(1 as libc::c_int as isize) * x));
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut b0: libc::c_double = 0.0f64;
        let mut b1: libc::c_double = 0.0f64;
        let mut btmp: libc::c_double = 0.0f64;
        let mut e0: libc::c_double = 0.0f64;
        let mut e1: libc::c_double = 0.0f64;
        let mut etmp: libc::c_double = e1;
        let mut j: libc::c_int = 0;
        j = n;
        while j >= 0 as libc::c_int {
            btmp = b0;
            b0 = *a.offset(j as isize) + x * b0
                - (j + 1 as libc::c_int) as libc::c_double * b1;
            b1 = btmp;
            etmp = e0;
            e0 = 2.2204460492503131e-16f64 * fabs(*a.offset(j as isize)) + fabs(x) * e0
                + (j + 1 as libc::c_int) as libc::c_double * e1;
            e1 = etmp;
            j -= 1;
            j;
        }
        (*result).val = b0;
        (*result).err = e0 + fabs(b0) * 2.2204460492503131e-16f64;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_hermite_prob_series(
    n: libc::c_int,
    x: libc::c_double,
    mut a: *const libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_hermite_prob_series_e(n, x, a, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_hermite_prob_series_e(n, x, a, &result)\0" as *const u8
                as *const libc::c_char,
            b"hermite.c\0" as *const u8 as *const libc::c_char,
            788 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_hermite_array(
    nmax: libc::c_int,
    x: libc::c_double,
    mut result_array: *mut libc::c_double,
) -> libc::c_int {
    if nmax < 0 as libc::c_int {
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"hermite.c\0" as *const u8 as *const libc::c_char,
            798 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if nmax == 0 as libc::c_int {
        *result_array.offset(0 as libc::c_int as isize) = 1.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if nmax == 1 as libc::c_int {
        *result_array.offset(0 as libc::c_int as isize) = 1.0f64;
        *result_array.offset(1 as libc::c_int as isize) = 2.0f64 * x;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let two_x: libc::c_double = 2.0f64 * x;
        let abs_two_x: libc::c_double = fabs(two_x);
        let thresh1: libc::c_double = if abs_two_x > 1.0f64 {
            0.9f64 * 1.7976931348623157e+308f64 / abs_two_x
        } else {
            1.7976931348623157e+308f64
        };
        let thresh2: libc::c_double = 0.9f64 * 1.7976931348623157e+308f64 / 2.0f64;
        let mut p_n0: libc::c_double = 1.0f64;
        let mut p_n1: libc::c_double = two_x;
        let mut p_n: libc::c_double = p_n1;
        let mut j: libc::c_int = 0;
        *result_array.offset(0 as libc::c_int as isize) = 1.0f64;
        *result_array.offset(1 as libc::c_int as isize) = 2.0f64 * x;
        j = 1 as libc::c_int;
        while j < nmax {
            if fabs(p_n1) > thresh1 || fabs(p_n0) > thresh2 / j as libc::c_double {
                status = GSL_EOVRFLW as libc::c_int;
            }
            p_n = two_x * p_n1 - 2.0f64 * j as libc::c_double * p_n0;
            p_n0 = p_n1;
            p_n1 = p_n;
            *result_array.offset((j + 1 as libc::c_int) as isize) = p_n;
            j += 1;
            j;
        }
        return status;
    };
}
pub unsafe extern "C" fn gsl_sf_hermite_array_deriv(
    m: libc::c_int,
    nmax: libc::c_int,
    x: libc::c_double,
    mut result_array: *mut libc::c_double,
) -> libc::c_int {
    if nmax < 0 as libc::c_int || m < 0 as libc::c_int {
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"hermite.c\0" as *const u8 as *const libc::c_char,
            856 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if m == 0 as libc::c_int {
        gsl_sf_hermite_array(nmax, x, result_array);
        return GSL_SUCCESS as libc::c_int;
    } else if nmax < m {
        let mut j: libc::c_int = 0;
        j = 0 as libc::c_int;
        while j <= nmax {
            *result_array.offset(j as isize) = 0.0f64;
            j += 1;
            j;
        }
        return GSL_SUCCESS as libc::c_int;
    } else if nmax == m {
        let mut j_0: libc::c_int = 0;
        j_0 = 0 as libc::c_int;
        while j_0 < m {
            *result_array.offset(j_0 as isize) = 0.0f64;
            j_0 += 1;
            j_0;
        }
        *result_array
            .offset(
                nmax as isize,
            ) = gsl_sf_pow_int(2 as libc::c_int as libc::c_double, m)
            * gsl_sf_fact(m as libc::c_uint);
        return GSL_SUCCESS as libc::c_int;
    } else if nmax == m + 1 as libc::c_int {
        let mut j_1: libc::c_int = 0;
        j_1 = 0 as libc::c_int;
        while j_1 < m {
            *result_array.offset(j_1 as isize) = 0.0f64;
            j_1 += 1;
            j_1;
        }
        *result_array
            .offset(
                (nmax - 1 as libc::c_int) as isize,
            ) = gsl_sf_pow_int(2 as libc::c_int as libc::c_double, m)
            * gsl_sf_fact(m as libc::c_uint);
        *result_array
            .offset(
                nmax as isize,
            ) = *result_array.offset((nmax - 1 as libc::c_int) as isize)
            * 2 as libc::c_int as libc::c_double
            * (m + 1 as libc::c_int) as libc::c_double * x;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut p_n0: libc::c_double = gsl_sf_pow_int(
            2 as libc::c_int as libc::c_double,
            m,
        ) * gsl_sf_fact(m as libc::c_uint);
        let mut p_n1: libc::c_double = p_n0 * 2 as libc::c_int as libc::c_double
            * (m + 1 as libc::c_int) as libc::c_double * x;
        let mut p_n: libc::c_double = 0.;
        let mut j_2: libc::c_int = 0;
        j_2 = 0 as libc::c_int;
        while j_2 < m {
            *result_array.offset(j_2 as isize) = 0.0f64;
            j_2 += 1;
            j_2;
        }
        *result_array.offset(m as isize) = p_n0;
        *result_array.offset((m + 1 as libc::c_int) as isize) = p_n1;
        j_2 = m + 1 as libc::c_int;
        while j_2 < nmax {
            p_n = (x * p_n1 - j_2 as libc::c_double * p_n0)
                * 2 as libc::c_int as libc::c_double * (j_2 as libc::c_double + 1.0f64)
                / ((j_2 - m) as libc::c_double + 1.0f64);
            p_n0 = p_n1;
            p_n1 = p_n;
            *result_array.offset((j_2 + 1 as libc::c_int) as isize) = p_n;
            j_2 += 1;
            j_2;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_hermite_deriv_array(
    mmax: libc::c_int,
    n: libc::c_int,
    x: libc::c_double,
    mut result_array: *mut libc::c_double,
) -> libc::c_int {
    if n < 0 as libc::c_int || mmax < 0 as libc::c_int {
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"hermite.c\0" as *const u8 as *const libc::c_char,
            929 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if n == 0 as libc::c_int {
        let mut j: libc::c_int = 0;
        *result_array.offset(0 as libc::c_int as isize) = 1.0f64;
        j = 1 as libc::c_int;
        while j <= mmax {
            *result_array.offset(j as isize) = 0.0f64;
            j += 1;
            j;
        }
        return GSL_SUCCESS as libc::c_int;
    } else if n == 1 as libc::c_int && mmax > 0 as libc::c_int {
        let mut j_0: libc::c_int = 0;
        *result_array
            .offset(0 as libc::c_int as isize) = 2 as libc::c_int as libc::c_double * x;
        *result_array.offset(1 as libc::c_int as isize) = 1.0f64;
        j_0 = 2 as libc::c_int;
        while j_0 <= mmax {
            *result_array.offset(j_0 as isize) = 0.0f64;
            j_0 += 1;
            j_0;
        }
        return GSL_SUCCESS as libc::c_int;
    } else if mmax == 0 as libc::c_int {
        *result_array.offset(0 as libc::c_int as isize) = gsl_sf_hermite(n, x);
        return GSL_SUCCESS as libc::c_int;
    } else if mmax == 1 as libc::c_int {
        *result_array.offset(0 as libc::c_int as isize) = gsl_sf_hermite(n, x);
        *result_array
            .offset(
                1 as libc::c_int as isize,
            ) = (2 as libc::c_int * n) as libc::c_double
            * gsl_sf_hermite(n - 1 as libc::c_int, x);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut k: libc::c_int = GSL_MAX_INT(0 as libc::c_int, n - mmax);
        let mut p_n0: libc::c_double = gsl_sf_hermite(k, x);
        let mut p_n1: libc::c_double = gsl_sf_hermite(k + 1 as libc::c_int, x);
        let mut p_n: libc::c_double = p_n1;
        let mut j_1: libc::c_int = 0;
        j_1 = n + 1 as libc::c_int;
        while j_1 <= mmax {
            *result_array.offset(j_1 as isize) = 0.0f64;
            j_1 += 1;
            j_1;
        }
        *result_array.offset(GSL_MIN_INT(n, mmax) as isize) = p_n0;
        *result_array.offset((GSL_MIN_INT(n, mmax) - 1 as libc::c_int) as isize) = p_n1;
        j_1 = GSL_MIN_INT(mmax, n) - 1 as libc::c_int;
        while j_1 > 0 as libc::c_int {
            k += 1;
            k;
            p_n = 2 as libc::c_int as libc::c_double * x * p_n1
                - (2 as libc::c_int * k) as libc::c_double * p_n0;
            p_n0 = p_n1;
            p_n1 = p_n;
            *result_array.offset((j_1 - 1 as libc::c_int) as isize) = p_n;
            j_1 -= 1;
            j_1;
        }
        p_n = 1.0f64;
        j_1 = 1 as libc::c_int;
        while j_1 <= GSL_MIN_INT(n, mmax) {
            p_n *= 2.0f64 * ((n - j_1) as libc::c_double + 1.0f64);
            *result_array.offset(j_1 as isize) *= p_n;
            j_1 += 1;
            j_1;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_hermite_series_e(
    n: libc::c_int,
    x: libc::c_double,
    mut a: *const libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if n < 0 as libc::c_int {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"hermite.c\0" as *const u8 as *const libc::c_char,
            1006 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if n == 0 as libc::c_int {
        (*result).val = *a.offset(0 as libc::c_int as isize);
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if n == 1 as libc::c_int {
        (*result)
            .val = *a.offset(0 as libc::c_int as isize)
            + *a.offset(1 as libc::c_int as isize) * 2.0f64 * x;
        (*result)
            .err = 2.0f64 * 2.2204460492503131e-16f64
            * (fabs(*a.offset(0 as libc::c_int as isize))
                + fabs(*a.offset(1 as libc::c_int as isize) * 2.0f64 * x));
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut b0: libc::c_double = 0.0f64;
        let mut b1: libc::c_double = 0.0f64;
        let mut btmp: libc::c_double = 0.0f64;
        let mut e0: libc::c_double = 0.0f64;
        let mut e1: libc::c_double = 0.0f64;
        let mut etmp: libc::c_double = e1;
        let mut j: libc::c_int = 0;
        j = n;
        while j >= 0 as libc::c_int {
            btmp = b0;
            b0 = *a.offset(j as isize) + 2.0f64 * x * b0
                - 2.0f64 * (j + 1 as libc::c_int) as libc::c_double * b1;
            b1 = btmp;
            etmp = e0;
            e0 = 2.2204460492503131e-16f64 * fabs(*a.offset(j as isize))
                + fabs(2.0f64 * x) * e0
                + 2.0f64 * (j + 1 as libc::c_int) as libc::c_double * e1;
            e1 = etmp;
            j -= 1;
            j;
        }
        (*result).val = b0;
        (*result).err = e0 + fabs(b0) * 2.2204460492503131e-16f64;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_hermite_series(
    n: libc::c_int,
    x: libc::c_double,
    mut a: *const libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_hermite_series_e(n, x, a, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_hermite_series_e(n, x, a, &result)\0" as *const u8
                as *const libc::c_char,
            b"hermite.c\0" as *const u8 as *const libc::c_char,
            1050 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_hermite_func_array(
    nmax: libc::c_int,
    x: libc::c_double,
    mut result_array: *mut libc::c_double,
) -> libc::c_int {
    if nmax < 0 as libc::c_int {
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"hermite.c\0" as *const u8 as *const libc::c_char,
            1061 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if nmax == 0 as libc::c_int {
        *result_array
            .offset(
                0 as libc::c_int as isize,
            ) = exp(-0.5f64 * x * x) / sqrt(1.77245385090551602729816748334f64);
        return GSL_SUCCESS as libc::c_int;
    } else if nmax == 1 as libc::c_int {
        *result_array
            .offset(
                0 as libc::c_int as isize,
            ) = exp(-0.5f64 * x * x) / sqrt(1.77245385090551602729816748334f64);
        *result_array
            .offset(
                1 as libc::c_int as isize,
            ) = *result_array.offset(0 as libc::c_int as isize)
            * 1.41421356237309504880f64 * x;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let arg: libc::c_double = -0.5f64 * x * x;
        let mut hi2: libc::c_double = 1.0f64 / sqrt(1.77245385090551602729816748334f64);
        let mut hi1: libc::c_double = 1.41421356237309504880f64 * x * hi2;
        let mut hi: libc::c_double = 0.0f64;
        let mut sum_log_scale: libc::c_double = 0.0f64;
        let mut abshi: libc::c_double = 0.;
        let mut i: libc::c_int = 0;
        *result_array.offset(0 as libc::c_int as isize) = exp(arg) * hi2;
        *result_array
            .offset(
                1 as libc::c_int as isize,
            ) = *result_array.offset(0 as libc::c_int as isize)
            * 1.41421356237309504880f64 * x;
        i = 2 as libc::c_int;
        while i <= nmax {
            hi = sqrt(2.0f64 / i as libc::c_double) * x * hi1
                - sqrt((i as libc::c_double - 1.0f64) / i as libc::c_double) * hi2;
            hi2 = hi1;
            hi1 = hi;
            abshi = fabs(hi);
            if abshi > 1.0f64 {
                let mut log_scale: libc::c_double = (if log(abshi)
                    >= 0 as libc::c_int as libc::c_double
                {
                    (log(abshi) + 0.5f64) as libc::c_int
                } else {
                    (log(abshi) - 0.5f64) as libc::c_int
                }) as libc::c_double;
                let mut scale: libc::c_double = exp(-log_scale);
                hi *= scale;
                hi1 *= scale;
                hi2 *= scale;
                sum_log_scale += log_scale;
            }
            *result_array.offset(i as isize) = hi * exp(arg + sum_log_scale);
            i += 1;
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_hermite_func_series_e(
    n: libc::c_int,
    x: libc::c_double,
    mut a: *const libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if n < 0 as libc::c_int {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"hermite.c\0" as *const u8 as *const libc::c_char,
            1122 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if n == 0 as libc::c_int {
        (*result)
            .val = *a.offset(0 as libc::c_int as isize) * exp(-0.5f64 * x * x)
            / sqrt(1.77245385090551602729816748334f64);
        (*result).err = 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if n == 1 as libc::c_int {
        (*result)
            .val = (*a.offset(0 as libc::c_int as isize)
            + *a.offset(1 as libc::c_int as isize) * 1.41421356237309504880f64 * x)
            * exp(-0.5f64 * x * x) / sqrt(1.77245385090551602729816748334f64);
        (*result)
            .err = 2.0f64 * 2.2204460492503131e-16f64
            * (fabs(*a.offset(0 as libc::c_int as isize))
                + fabs(
                    *a.offset(1 as libc::c_int as isize) * 1.41421356237309504880f64 * x,
                )) * exp(-0.5f64 * x * x) / sqrt(1.77245385090551602729816748334f64);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut b0: libc::c_double = 0.0f64;
        let mut b1: libc::c_double = 0.0f64;
        let mut btmp: libc::c_double = 0.0f64;
        let mut e0: libc::c_double = 0.0f64;
        let mut e1: libc::c_double = 0.0f64;
        let mut etmp: libc::c_double = e1;
        let mut j: libc::c_int = 0;
        j = n;
        while j >= 0 as libc::c_int {
            btmp = b0;
            b0 = *a.offset(j as isize)
                + sqrt(2.0f64 / (j + 1 as libc::c_int) as libc::c_double) * x * b0
                - sqrt((j as libc::c_double + 1.0f64) / (j as libc::c_double + 2.0f64))
                    * b1;
            b1 = btmp;
            etmp = e0;
            e0 = 2.2204460492503131e-16f64 * fabs(*a.offset(j as isize))
                + sqrt(2.0f64 / (j + 1 as libc::c_int) as libc::c_double) * fabs(x) * e0
                + sqrt((j as libc::c_double + 1.0f64) / (j as libc::c_double + 2.0f64))
                    * e1;
            e1 = etmp;
            j -= 1;
            j;
        }
        (*result)
            .val = b0 * exp(-0.5f64 * x * x) / sqrt(1.77245385090551602729816748334f64);
        (*result).err = e0 + fabs((*result).val) * 2.2204460492503131e-16f64;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_hermite_func_series(
    n: libc::c_int,
    x: libc::c_double,
    mut a: *const libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_hermite_func_series_e(n, x, a, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_hermite_func_series_e(n, x, a, &result)\0" as *const u8
                as *const libc::c_char,
            b"hermite.c\0" as *const u8 as *const libc::c_char,
            1171 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_hermite_func_der_e(
    m: libc::c_int,
    n: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if m < 0 as libc::c_int || n < 0 as libc::c_int {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"hermite.c\0" as *const u8 as *const libc::c_char,
            1183 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if m == 0 as libc::c_int {
        return gsl_sf_hermite_func_e(n, x, result)
    } else if m == 1 as libc::c_int {
        let mut hi2: libc::c_double = 1.0f64 / sqrt(1.77245385090551602729816748334f64);
        let mut hi1: libc::c_double = 1.41421356237309504880f64 * x * hi2;
        let mut hi: libc::c_double = 0.0f64;
        let mut sum_log_scale: libc::c_double = 0.0f64;
        let mut abshi: libc::c_double = 0.;
        let mut i: libc::c_int = 0;
        i = 2 as libc::c_int;
        while i <= n {
            hi = sqrt(2.0f64 / i as libc::c_double) * x * hi1
                - sqrt((i as libc::c_double - 1.0f64) / i as libc::c_double) * hi2;
            hi2 = hi1;
            hi1 = hi;
            abshi = fabs(hi);
            if abshi > 1.0f64 {
                let mut log_scale: libc::c_double = (if log(abshi)
                    >= 0 as libc::c_int as libc::c_double
                {
                    (log(abshi) + 0.5f64) as libc::c_int
                } else {
                    (log(abshi) - 0.5f64) as libc::c_int
                }) as libc::c_double;
                let mut scale: libc::c_double = exp(-log_scale);
                hi *= scale;
                hi1 *= scale;
                hi2 *= scale;
                sum_log_scale += log_scale;
            }
            i += 1;
            i;
        }
        (*result)
            .val = (sqrt(2.0f64 * n as libc::c_double) * hi2 - x * hi)
            * exp(-0.5f64 * x * x + sum_log_scale);
        (*result)
            .err = n as libc::c_double * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut j: libc::c_int = 0;
        let mut r: libc::c_double = 0.;
        let mut er: libc::c_double = 0.;
        let mut b: libc::c_double = 0.;
        let mut h0: libc::c_double = 1.0f64;
        let mut h1: libc::c_double = x;
        let mut eh0: libc::c_double = 2.2204460492503131e-16f64;
        let mut eh1: libc::c_double = 2.2204460492503131e-16f64;
        let mut p0: libc::c_double = 1.0f64;
        let mut p1: libc::c_double = 1.41421356237309504880f64 * x;
        let mut ep0: libc::c_double = 2.2204460492503131e-16f64;
        let mut ep1: libc::c_double = 1.41421356237309504880f64
            * 2.2204460492503131e-16f64;
        let mut f: libc::c_double = 1.0f64;
        j = GSL_MAX_INT(1 as libc::c_int, n - m + 1 as libc::c_int);
        while j <= n {
            f *= sqrt(2.0f64 * j as libc::c_double);
            j += 1;
            j;
        }
        if m > n {
            f = if m - n & 1 as libc::c_int != 0 { -f } else { f };
            j = 0 as libc::c_int;
            while j < GSL_MIN_INT(n, m - n) {
                f *= (m - j) as libc::c_double / (j as libc::c_double + 1.0f64);
                j += 1;
                j;
            }
        }
        j = 1 as libc::c_int;
        while j <= m - n {
            b = x * h1 - j as libc::c_double * h0;
            h0 = h1;
            h1 = b;
            b = fabs(x) * eh1 + j as libc::c_double * eh0;
            eh0 = eh1;
            eh1 = b;
            j += 1;
            j;
        }
        b = 0.0f64;
        j = 1 as libc::c_int;
        while j <= n - m {
            b = (1.41421356237309504880f64 * x * p1 - sqrt(j as libc::c_double) * p0)
                / sqrt(j as libc::c_double + 1.0f64);
            p0 = p1;
            p1 = b;
            b = (1.41421356237309504880f64 * fabs(x) * ep1
                + sqrt(j as libc::c_double) * ep0) / sqrt(j as libc::c_double + 1.0f64);
            ep0 = ep1;
            ep1 = b;
            j += 1;
            j;
        }
        b = 0.0f64;
        r = 0.0f64;
        er = 0.0f64;
        j = GSL_MAX_INT(0 as libc::c_int, m - n);
        while j <= m {
            r += f * h0 * p0;
            er
                += eh0 * fabs(f * p0) + ep0 * fabs(f * h0)
                    + 2.2204460492503131e-16f64 * fabs(f * h0 * p0);
            b = x * h1 - (j as libc::c_double + 1.0f64) * h0;
            h0 = h1;
            h1 = b;
            b = 0.5f64 * (fabs(x) * eh1 + (j as libc::c_double + 1.0f64) * eh0);
            eh0 = eh1;
            eh1 = b;
            b = (1.41421356237309504880f64 * x * p1
                - sqrt((n - m + j) as libc::c_double + 1.0f64) * p0)
                / sqrt((n - m + j) as libc::c_double + 2.0f64);
            p0 = p1;
            p1 = b;
            b = 0.5f64
                * (1.41421356237309504880f64 * fabs(x) * ep1
                    + sqrt((n - m + j) as libc::c_double + 1.0f64) * ep0)
                / sqrt((n - m + j) as libc::c_double + 2.0f64);
            ep0 = ep1;
            ep1 = b;
            f
                *= -(m - j) as libc::c_double / (j as libc::c_double + 1.0f64)
                    / sqrt((n - m + j) as libc::c_double + 1.0f64)
                    * 0.70710678118654752440f64;
            j += 1;
            j;
        }
        (*result)
            .val = r * exp(-0.5f64 * x * x) / sqrt(1.77245385090551602729816748334f64);
        (*result)
            .err = er
            * fabs(exp(-0.5f64 * x * x) / sqrt(1.77245385090551602729816748334f64))
            + 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_hermite_func_der(
    m: libc::c_int,
    n: libc::c_int,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_hermite_func_der_e(m, n, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_hermite_func_der_e(m, n, x, &result)\0" as *const u8
                as *const libc::c_char,
            b"hermite.c\0" as *const u8 as *const libc::c_char,
            1308 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
unsafe extern "C" fn H_zero_init(n: libc::c_int, k: libc::c_int) -> libc::c_double {
    let mut p: libc::c_double = 1.0f64;
    let mut x: libc::c_double = 1.0f64;
    let mut y: libc::c_double = 1.0f64;
    if k == 1 as libc::c_int && n > 50 as libc::c_int {
        x = if n & 1 as libc::c_int != 0 {
            1.0f64 / sqrt((n - 1 as libc::c_int) as libc::c_double / 6.0f64)
        } else {
            1.0f64 / sqrt(0.5f64 * n as libc::c_double)
        };
    } else {
        p = -0.7937005259840997373758528196f64
            * gsl_sf_airy_zero_Ai(
                (n / 2 as libc::c_int - k + 1 as libc::c_int) as libc::c_uint,
            );
        x = sqrt((2 as libc::c_int * n) as libc::c_double + 1.0f64);
        y = pow(
            (2 as libc::c_int * n) as libc::c_double + 1.0f64,
            1 as libc::c_int as libc::c_double / 6.0f64,
        );
        x = x - p / y - 0.1f64 * p * p / (x * y * y)
            + (9 as libc::c_int as libc::c_double / 280.0f64
                - p * p * p * 11 as libc::c_int as libc::c_double / 350.0f64)
                / (x * x * x)
            + (p * 277 as libc::c_int as libc::c_double / 12600.0f64
                - gsl_sf_pow_int(p, 4 as libc::c_int)
                    * 823 as libc::c_int as libc::c_double / 63000.0f64)
                / gsl_sf_pow_int(x, 4 as libc::c_int) / y;
    }
    p = acos(x / sqrt((2 as libc::c_int * n) as libc::c_double + 1.0f64));
    y = 3.14159265358979323846f64
        * ((-(2 as libc::c_int) * (n / 2 as libc::c_int - k)) as libc::c_double - 1.5f64)
        / (n as libc::c_double + 0.5f64);
    if gsl_fcmp(
        y,
        sin(2.0f64 * p) - 2 as libc::c_int as libc::c_double * p,
        1.4901161193847656e-08f64,
    ) == 0 as libc::c_int
    {
        return x;
    }
    if y > -2.2204460492503131e-16f64 {
        return sqrt((2 as libc::c_int * n) as libc::c_double + 1.0f64);
    }
    if p < 2.2204460492503131e-16f64 {
        p = 2.2204460492503131e-16f64;
    }
    if p > 1.57079632679489661923f64 {
        p = 1.57079632679489661923f64;
    }
    if sin(2.0f64 * p) - 2 as libc::c_int as libc::c_double * p > y {
        x = if (sin(2.0f64 * p) - 2 as libc::c_int as libc::c_double * p - y) / 4.0f64
            > 1.4901161193847656e-08f64
        {
            (sin(2.0f64 * p) - 2 as libc::c_int as libc::c_double * p - y) / 4.0f64
        } else {
            1.4901161193847656e-08f64
        };
        loop {
            x *= 2.0f64;
            p += x;
            if !(sin(2.0f64 * p) - 2 as libc::c_int as libc::c_double * p > y) {
                break;
            }
        }
    }
    loop {
        x = p;
        p -= (sin(2.0f64 * p) - 2.0f64 * p - y) / (2.0f64 * cos(2.0f64 * p) - 2.0f64);
        if p < 0.0f64 || p > 1.57079632679489661923f64 {
            p = 1.57079632679489661923f64;
        }
        if !(gsl_fcmp(
            x,
            p,
            100 as libc::c_int as libc::c_double * 2.2204460492503131e-16f64,
        ) != 0 as libc::c_int)
        {
            break;
        }
    }
    return sqrt((2 as libc::c_int * n) as libc::c_double + 1.0f64) * cos(p);
}
static mut He_zero_tab: [libc::c_double; 99] = [
    1.73205080756887729352744634151f64,
    0.741963784302725857648513596726f64,
    2.33441421833897723931751226721f64,
    1.35562617997426586583052129087f64,
    2.85697001387280565416230426401f64,
    0.616706590192594152193686099399f64,
    1.88917587775371067550566789858f64,
    3.32425743355211895236183546247f64,
    1.154405394739968127239597758838f64,
    2.36675941073454128861885646856f64,
    3.75043971772574225630392202571f64,
    0.539079811351375108072461918694f64,
    1.63651904243510799922544657297f64,
    2.80248586128754169911301080618f64,
    4.14454718612589433206019783917f64,
    1.023255663789132524828148225810f64,
    2.07684797867783010652215614374f64,
    3.20542900285646994336567590292f64,
    4.51274586339978266756667884317f64,
    0.484935707515497653046233483105f64,
    1.46598909439115818325066466416f64,
    2.48432584163895458087625118368f64,
    3.58182348355192692277623675546f64,
    4.85946282833231215015516494660f64,
    0.928868997381063940144111999584f64,
    1.87603502015484584534137013967f64,
    2.86512316064364499771968407254f64,
    3.93616660712997692868589612142f64,
    5.18800122437487094818666404539f64,
    0.444403001944138945299732445510f64,
    1.34037519715161672153112945211f64,
    2.25946445100079912386492979448f64,
    3.22370982877009747166319001956f64,
    4.27182584793228172295999293076f64,
    5.50090170446774760081221630899f64,
    0.856679493519450033897376121795f64,
    1.72541837958823916151095838741f64,
    2.62068997343221478063807762201f64,
    3.56344438028163409162493844661f64,
    4.59139844893652062705231872720f64,
    5.80016725238650030586450565322f64,
    0.412590457954601838167454145167f64,
    1.24268895548546417895063983219f64,
    2.08834474570194417097139675101f64,
    2.96303657983866750254927123447f64,
    3.88692457505976938384755016476f64,
    4.89693639734556468372449782879f64,
    6.08740954690129132226890147034f64,
    0.799129068324547999424888414207f64,
    1.60671006902872973652322479373f64,
    2.43243682700975804116311571682f64,
    3.28908242439876638890856229770f64,
    4.19620771126901565957404160583f64,
    5.19009359130478119946445431715f64,
    6.36394788882983831771116094427f64,
    0.386760604500557347721047189801f64,
    1.16382910055496477419336819907f64,
    1.95198034571633346449212362880f64,
    2.76024504763070161684598142269f64,
    3.60087362417154828824902745506f64,
    4.49295530252001124266582263095f64,
    5.47222570594934308841242925805f64,
    6.63087819839312848022981922233f64,
    0.751842600703896170737870774614f64,
    1.50988330779674075905491513417f64,
    2.28101944025298889535537879396f64,
    3.07379717532819355851658337833f64,
    3.90006571719800990903311840097f64,
    4.77853158962998382710540812497f64,
    5.74446007865940618125547815768f64,
    6.88912243989533223256205432938f64,
    0.365245755507697595916901619097f64,
    1.09839551809150122773848360538f64,
    1.83977992150864548966395498992f64,
    2.59583368891124032910545091458f64,
    3.37473653577809099529779309480f64,
    4.18802023162940370448450911428f64,
    5.05407268544273984538327527397f64,
    6.00774591135959752029303858752f64,
    7.13946484914647887560975631213f64,
    0.712085044042379940413609979021f64,
    1.42887667607837287134157901452f64,
    2.15550276131693514033871248449f64,
    2.89805127651575312007902775275f64,
    3.66441654745063847665304033851f64,
    4.46587262683103133615452574019f64,
    5.32053637733603803162823765939f64,
    6.26289115651325170419416064557f64,
    7.38257902403043186766326977122f64,
    0.346964157081355927973322447164f64,
    1.04294534880275103146136681143f64,
    1.74524732081412671493067861704f64,
    2.45866361117236775131735057433f64,
    3.18901481655338941485371744116f64,
    3.94396735065731626033176813604f64,
    4.73458133404605534390170946748f64,
    5.57873880589320115268040332802f64,
    6.51059015701365448636289263918f64,
    7.61904854167975829138128156060f64,
];
pub unsafe extern "C" fn gsl_sf_hermite_prob_zero_e(
    n: libc::c_int,
    s: libc::c_int,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if n <= 0 as libc::c_int || s < 0 as libc::c_int || s > n / 2 as libc::c_int {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"hermite.c\0" as *const u8 as *const libc::c_char,
            1471 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if s == 0 as libc::c_int {
        if n & 1 as libc::c_int == 1 as libc::c_int {
            (*result).val = 0.0f64;
            (*result).err = 0.0f64;
            return GSL_SUCCESS as libc::c_int;
        } else {
            (*result).val = ::std::f32::NAN as libc::c_double;
            (*result).err = ::std::f32::NAN as libc::c_double;
            gsl_error(
                b"domain error\0" as *const u8 as *const libc::c_char,
                b"hermite.c\0" as *const u8 as *const libc::c_char,
                1483 as libc::c_int,
                GSL_EDOM as libc::c_int,
            );
            return GSL_EDOM as libc::c_int;
        }
    } else if n == 2 as libc::c_int {
        (*result).val = 1.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if n < 21 as libc::c_int {
        (*result)
            .val = He_zero_tab[((if n & 1 as libc::c_int != 0 {
            n / 2 as libc::c_int
        } else {
            0 as libc::c_int
        }) + n / 2 as libc::c_int * (n / 2 as libc::c_int - 1 as libc::c_int) + s
            - 2 as libc::c_int) as usize];
        (*result).err = 2.2204460492503131e-16f64 * (*result).val;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut d: libc::c_double = 1.0f64;
        let mut x: libc::c_double = 1.0f64;
        let mut x0: libc::c_double = 1.0f64;
        let mut j: libc::c_int = 0;
        x = H_zero_init(n, s) * 1.41421356237309504880f64;
        loop {
            x0 = x;
            d = 0.0f64;
            j = 1 as libc::c_int;
            while j < n {
                d = j as libc::c_double / (x - d);
                j += 1;
                j;
            }
            x -= (x - d) / n as libc::c_double;
            if !(gsl_fcmp(
                x,
                x0,
                10 as libc::c_int as libc::c_double * 2.2204460492503131e-16f64,
            ) != 0 as libc::c_int)
            {
                break;
            }
        }
        (*result).val = x;
        (*result)
            .err = 2 as libc::c_int as libc::c_double * 2.2204460492503131e-16f64 * x
            + fabs(x - x0);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_hermite_prob_zero(
    n: libc::c_int,
    s: libc::c_int,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_hermite_prob_zero_e(n, s, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_hermite_prob_zero_e(n, s, &result)\0" as *const u8
                as *const libc::c_char,
            b"hermite.c\0" as *const u8 as *const libc::c_char,
            1527 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
static mut H_zero_tab: [libc::c_double; 99] = [
    1.22474487139158904909864203735f64,
    0.524647623275290317884060253835f64,
    1.65068012388578455588334111112f64,
    0.958572464613818507112770593893f64,
    2.02018287045608563292872408814f64,
    0.436077411927616508679215948251f64,
    1.335849074013696949714895282970f64,
    2.35060497367449222283392198706f64,
    0.816287882858964663038710959027f64,
    1.67355162876747144503180139830f64,
    2.65196135683523349244708200652f64,
    0.381186990207322116854718885584f64,
    1.157193712446780194720765779063f64,
    1.98165675669584292585463063977f64,
    2.93063742025724401922350270524f64,
    0.723551018752837573322639864579f64,
    1.46855328921666793166701573925f64,
    2.26658058453184311180209693284f64,
    3.19099320178152760723004779538f64,
    0.342901327223704608789165025557f64,
    1.03661082978951365417749191676f64,
    1.75668364929988177345140122011f64,
    2.53273167423278979640896079775f64,
    3.43615911883773760332672549432f64,
    0.656809566882099765024611575383f64,
    1.32655708449493285594973473558f64,
    2.02594801582575533516591283121f64,
    2.78329009978165177083671870152f64,
    3.66847084655958251845837146485f64,
    0.314240376254359111276611634095f64,
    0.947788391240163743704578131060f64,
    1.59768263515260479670966277090f64,
    2.27950708050105990018772856942f64,
    3.02063702512088977171067937518f64,
    3.88972489786978191927164274724f64,
    0.605763879171060113080537108602f64,
    1.22005503659074842622205526637f64,
    1.85310765160151214200350644316f64,
    2.51973568567823788343040913628f64,
    3.24660897837240998812205115236f64,
    4.10133759617863964117891508007f64,
    0.291745510672562078446113075799f64,
    0.878713787329399416114679311861f64,
    1.47668273114114087058350654421f64,
    2.09518325850771681573497272630f64,
    2.74847072498540256862499852415f64,
    3.46265693360227055020891736115f64,
    4.30444857047363181262129810037f64,
    0.565069583255575748526020337198f64,
    1.13611558521092066631913490556f64,
    1.71999257518648893241583152515f64,
    2.32573248617385774545404479449f64,
    2.96716692790560324848896036355f64,
    3.66995037340445253472922383312f64,
    4.49999070730939155366438053053f64,
    0.273481046138152452158280401965f64,
    0.822951449144655892582454496734f64,
    1.38025853919888079637208966969f64,
    1.95178799091625397743465541496f64,
    2.54620215784748136215932870545f64,
    3.17699916197995602681399455926f64,
    3.86944790486012269871942409801f64,
    4.68873893930581836468849864875f64,
    0.531633001342654731349086553718f64,
    1.06764872574345055363045773799f64,
    1.61292431422123133311288254454f64,
    2.17350282666662081927537907149f64,
    2.75776291570388873092640349574f64,
    3.37893209114149408338327069289f64,
    4.06194667587547430689245559698f64,
    4.87134519367440308834927655662f64,
    0.258267750519096759258116098711f64,
    0.776682919267411661316659462284f64,
    1.30092085838961736566626555439f64,
    1.83553160426162889225383944409f64,
    2.38629908916668600026459301424f64,
    2.96137750553160684477863254906f64,
    3.57376906848626607950067599377f64,
    4.24811787356812646302342016090f64,
    5.04836400887446676837203757885f64,
    0.503520163423888209373811765050f64,
    1.01036838713431135136859873726f64,
    1.52417061939353303183354859367f64,
    2.04923170985061937575050838669f64,
    2.59113378979454256492128084112f64,
    3.15784881834760228184318034120f64,
    3.76218735196402009751489394104f64,
    4.42853280660377943723498532226f64,
    5.22027169053748216460967142500f64,
    0.245340708300901249903836530634f64,
    0.737473728545394358705605144252f64,
    1.23407621539532300788581834696f64,
    1.73853771211658620678086566214f64,
    2.25497400208927552308233334473f64,
    2.78880605842813048052503375640f64,
    3.34785456738321632691492452300f64,
    3.94476404011562521037562880052f64,
    4.60368244955074427307767524898f64,
    5.38748089001123286201690041068f64,
];
pub unsafe extern "C" fn gsl_sf_hermite_zero_e(
    n: libc::c_int,
    s: libc::c_int,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if n <= 0 as libc::c_int || s < 0 as libc::c_int || s > n / 2 as libc::c_int {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"hermite.c\0" as *const u8 as *const libc::c_char,
            1656 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if s == 0 as libc::c_int {
        if n & 1 as libc::c_int == 1 as libc::c_int {
            (*result).val = 0.0f64;
            (*result).err = 0.0f64;
            return GSL_SUCCESS as libc::c_int;
        } else {
            (*result).val = ::std::f32::NAN as libc::c_double;
            (*result).err = ::std::f32::NAN as libc::c_double;
            gsl_error(
                b"domain error\0" as *const u8 as *const libc::c_char,
                b"hermite.c\0" as *const u8 as *const libc::c_char,
                1668 as libc::c_int,
                GSL_EDOM as libc::c_int,
            );
            return GSL_EDOM as libc::c_int;
        }
    } else if n == 2 as libc::c_int {
        (*result).val = 0.70710678118654752440f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if n < 21 as libc::c_int {
        (*result)
            .val = H_zero_tab[((if n & 1 as libc::c_int != 0 {
            n / 2 as libc::c_int
        } else {
            0 as libc::c_int
        }) + n / 2 as libc::c_int * (n / 2 as libc::c_int - 1 as libc::c_int) + s
            - 2 as libc::c_int) as usize];
        (*result).err = 2.2204460492503131e-16f64 * (*result).val;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut d: libc::c_double = 1.0f64;
        let mut x: libc::c_double = 1.0f64;
        let mut x0: libc::c_double = 1.0f64;
        let mut j: libc::c_int = 0;
        x = H_zero_init(n, s);
        loop {
            x0 = x;
            d = 0.0f64;
            j = 1 as libc::c_int;
            while j < n {
                d = (2 as libc::c_int * j) as libc::c_double / (2.0f64 * x - d);
                j += 1;
                j;
            }
            x
                -= (2 as libc::c_int as libc::c_double * x - d) * 0.5f64
                    / n as libc::c_double;
            if !(gsl_fcmp(
                x,
                x0,
                10 as libc::c_int as libc::c_double * 2.2204460492503131e-16f64,
            ) != 0 as libc::c_int)
            {
                break;
            }
        }
        (*result).val = x;
        (*result)
            .err = 2 as libc::c_int as libc::c_double * 2.2204460492503131e-16f64 * x
            + fabs(x - x0);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_hermite_zero(
    n: libc::c_int,
    s: libc::c_int,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_hermite_zero_e(n, s, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_hermite_zero_e(n, s, &result)\0" as *const u8
                as *const libc::c_char,
            b"hermite.c\0" as *const u8 as *const libc::c_char,
            1714 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_hermite_func_zero_e(
    n: libc::c_int,
    s: libc::c_int,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    return gsl_sf_hermite_zero_e(n, s, result);
}
pub unsafe extern "C" fn gsl_sf_hermite_func_zero(
    n: libc::c_int,
    s: libc::c_int,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_hermite_func_zero_e(n, s, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_hermite_func_zero_e(n, s, &result)\0" as *const u8
                as *const libc::c_char,
            b"hermite.c\0" as *const u8 as *const libc::c_char,
            1726 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_hermite_phys_e(
    n: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    return gsl_sf_hermite_e(n, x, result);
}
pub unsafe extern "C" fn gsl_sf_hermite_phys(
    n: libc::c_int,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_hermite_phys_e(n, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_hermite_phys_e(n, x, &result)\0" as *const u8
                as *const libc::c_char,
            b"hermite.c\0" as *const u8 as *const libc::c_char,
            1740 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_hermite_phys_der_e(
    m: libc::c_int,
    n: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    return gsl_sf_hermite_deriv_e(m, n, x, result);
}
pub unsafe extern "C" fn gsl_sf_hermite_phys_der(
    m: libc::c_int,
    n: libc::c_int,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_hermite_phys_der_e(m, n, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_hermite_phys_der_e(m, n, x, &result)\0" as *const u8
                as *const libc::c_char,
            b"hermite.c\0" as *const u8 as *const libc::c_char,
            1752 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_hermite_phys_array(
    nmax: libc::c_int,
    x: libc::c_double,
    mut result_array: *mut libc::c_double,
) -> libc::c_int {
    return gsl_sf_hermite_array(nmax, x, result_array);
}
pub unsafe extern "C" fn gsl_sf_hermite_phys_series_e(
    n: libc::c_int,
    x: libc::c_double,
    mut a: *const libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    return gsl_sf_hermite_series_e(n, x, a, result);
}
pub unsafe extern "C" fn gsl_sf_hermite_phys_series(
    n: libc::c_int,
    x: libc::c_double,
    mut a: *const libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_hermite_phys_series_e(n, x, a, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_hermite_phys_series_e(n, x, a, &result)\0" as *const u8
                as *const libc::c_char,
            b"hermite.c\0" as *const u8 as *const libc::c_char,
            1770 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_hermite_phys_array_der(
    m: libc::c_int,
    nmax: libc::c_int,
    x: libc::c_double,
    mut result_array: *mut libc::c_double,
) -> libc::c_int {
    return gsl_sf_hermite_array_deriv(m, nmax, x, result_array);
}
pub unsafe extern "C" fn gsl_sf_hermite_phys_der_array(
    mmax: libc::c_int,
    n: libc::c_int,
    x: libc::c_double,
    mut result_array: *mut libc::c_double,
) -> libc::c_int {
    return gsl_sf_hermite_deriv_array(mmax, n, x, result_array);
}
pub unsafe extern "C" fn gsl_sf_hermite_phys_zero_e(
    n: libc::c_int,
    s: libc::c_int,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    return gsl_sf_hermite_zero_e(n, s, result);
}
pub unsafe extern "C" fn gsl_sf_hermite_phys_zero(
    n: libc::c_int,
    s: libc::c_int,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_hermite_zero_e(n, s, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_hermite_zero_e(n, s, &result)\0" as *const u8
                as *const libc::c_char,
            b"hermite.c\0" as *const u8 as *const libc::c_char,
            1794 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_hermite_prob_array_der(
    m: libc::c_int,
    nmax: libc::c_int,
    x: libc::c_double,
    mut result_array: *mut libc::c_double,
) -> libc::c_int {
    return gsl_sf_hermite_prob_array_deriv(m, nmax, x, result_array);
}
pub unsafe extern "C" fn gsl_sf_hermite_prob_der_array(
    mmax: libc::c_int,
    n: libc::c_int,
    x: libc::c_double,
    mut result_array: *mut libc::c_double,
) -> libc::c_int {
    return gsl_sf_hermite_prob_deriv_array(mmax, n, x, result_array);
}
pub unsafe extern "C" fn gsl_sf_hermite_prob_der_e(
    m: libc::c_int,
    n: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    return gsl_sf_hermite_prob_deriv_e(m, n, x, result);
}
pub unsafe extern "C" fn gsl_sf_hermite_prob_der(
    m: libc::c_int,
    n: libc::c_int,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_hermite_prob_deriv_e(m, n, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_hermite_prob_deriv_e(m, n, x, &result)\0" as *const u8
                as *const libc::c_char,
            b"hermite.c\0" as *const u8 as *const libc::c_char,
            1818 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
