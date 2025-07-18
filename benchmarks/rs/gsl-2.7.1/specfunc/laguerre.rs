use ::libc;
extern "C" {
    fn asin(_: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sf_exp_mult_err_e(
        x: libc::c_double,
        dx: libc::c_double,
        y: libc::c_double,
        dy: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_lngamma_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_lngamma_sgn_e(
        x: libc::c_double,
        result_lg: *mut gsl_sf_result,
        sgn: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_sf_taylorcoeff_e(
        n: libc::c_int,
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_lnfact_e(n: libc::c_uint, result: *mut gsl_sf_result) -> libc::c_int;
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
unsafe extern "C" fn laguerre_large_n(
    n: libc::c_int,
    alpha: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let a: libc::c_double = -n as libc::c_double;
    let b: libc::c_double = alpha + 1.0f64;
    let eta: libc::c_double = 2.0f64 * b - 4.0f64 * a;
    let cos2th: libc::c_double = x / eta;
    let sin2th: libc::c_double = 1.0f64 - cos2th;
    let eps: libc::c_double = asin(sqrt(cos2th));
    let pre_h: libc::c_double = 0.25f64 * 3.14159265358979323846f64
        * 3.14159265358979323846f64 * eta * eta * cos2th * sin2th;
    let mut lg_b: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut lnfact: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut stat_lg: libc::c_int = gsl_sf_lngamma_e(b + n as libc::c_double, &mut lg_b);
    let mut stat_lf: libc::c_int = gsl_sf_lnfact_e(n as libc::c_uint, &mut lnfact);
    let mut pre_term1: libc::c_double = 0.5f64 * (1.0f64 - b) * log(0.25f64 * x * eta);
    let mut pre_term2: libc::c_double = 0.25f64 * log(pre_h);
    let mut lnpre_val: libc::c_double = lg_b.val - lnfact.val + 0.5f64 * x + pre_term1
        - pre_term2;
    let mut lnpre_err: libc::c_double = lg_b.err + lnfact.err
        + 2.2204460492503131e-16f64 * (fabs(pre_term1) + fabs(pre_term2));
    let mut phi1: libc::c_double = 0.25f64 * eta
        * (2 as libc::c_int as libc::c_double * eps + sin(2.0f64 * eps));
    let mut ser_term1: libc::c_double = -sin(phi1);
    let mut A1: libc::c_double = 1.0f64 / 12.0f64
        * (5.0f64 / (4.0f64 * sin2th) + (3.0f64 * b * b - 6.0f64 * b + 2.0f64) * sin2th
            - 1.0f64);
    let mut ser_term2: libc::c_double = -A1 * cos(phi1)
        / (0.25f64 * eta * sin(2.0f64 * eps));
    let mut ser_val: libc::c_double = ser_term1 + ser_term2;
    let mut ser_err: libc::c_double = ser_term2 * ser_term2
        + 2.2204460492503131e-16f64 * (fabs(ser_term1) + fabs(ser_term2));
    let mut stat_e: libc::c_int = gsl_sf_exp_mult_err_e(
        lnpre_val,
        lnpre_err,
        ser_val,
        ser_err,
        result,
    );
    (*result).err += 2.0f64 * 1.4901161193847656e-08f64 * fabs((*result).val);
    return if stat_e != GSL_SUCCESS as libc::c_int {
        stat_e
    } else if stat_lf != GSL_SUCCESS as libc::c_int {
        stat_lf
    } else if stat_lg != GSL_SUCCESS as libc::c_int {
        stat_lg
    } else {
        GSL_SUCCESS as libc::c_int
    };
}
unsafe extern "C" fn laguerre_n_cp(
    n: libc::c_int,
    a: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut lnfact: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut lg1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut lg2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut s1: libc::c_double = 0.;
    let mut s2: libc::c_double = 0.;
    let mut stat_f: libc::c_int = gsl_sf_lnfact_e(n as libc::c_uint, &mut lnfact);
    let mut stat_g1: libc::c_int = gsl_sf_lngamma_sgn_e(
        a + 1.0f64 + n as libc::c_double,
        &mut lg1,
        &mut s1,
    );
    let mut stat_g2: libc::c_int = gsl_sf_lngamma_sgn_e(a + 1.0f64, &mut lg2, &mut s2);
    let mut poly_1F1_val: libc::c_double = 1.0f64;
    let mut poly_1F1_err: libc::c_double = 0.0f64;
    let mut stat_e: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut lnpre_val: libc::c_double = lg1.val - lg2.val - lnfact.val;
    let mut lnpre_err: libc::c_double = lg1.err + lg2.err + lnfact.err
        + 2.0f64 * 2.2204460492503131e-16f64 * fabs(lnpre_val);
    k = n - 1 as libc::c_int;
    while k >= 0 as libc::c_int {
        let mut t: libc::c_double = (-n + k) as libc::c_double
            / (a + 1.0f64 + k as libc::c_double)
            * (x / (k + 1 as libc::c_int) as libc::c_double);
        let mut r: libc::c_double = t + 1.0f64 / poly_1F1_val;
        if r > 0.9f64 * 1.7976931348623157e+308f64 / poly_1F1_val {
            (*result).val = ::std::f32::INFINITY as libc::c_double;
            (*result).err = ::std::f32::INFINITY as libc::c_double;
            return GSL_EOVRFLW as libc::c_int;
        } else {
            poly_1F1_val = 1.0f64 + t * poly_1F1_val;
            poly_1F1_err += 2.2204460492503131e-16f64 + fabs(t) * poly_1F1_err;
        }
        k -= 1;
        k;
    }
    stat_e = gsl_sf_exp_mult_err_e(
        lnpre_val,
        lnpre_err,
        poly_1F1_val,
        poly_1F1_err,
        result,
    );
    return if stat_e != GSL_SUCCESS as libc::c_int {
        stat_e
    } else if stat_f != GSL_SUCCESS as libc::c_int {
        stat_f
    } else if stat_g1 != GSL_SUCCESS as libc::c_int {
        stat_g1
    } else if stat_g2 != GSL_SUCCESS as libc::c_int {
        stat_g2
    } else {
        GSL_SUCCESS as libc::c_int
    };
}
unsafe extern "C" fn laguerre_n_poly_safe(
    n: libc::c_int,
    a: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let b: libc::c_double = a + 1.0f64;
    let mx: libc::c_double = -x;
    let tc_sgn: libc::c_double = if x < 0.0f64 {
        1.0f64
    } else if n & 1 as libc::c_int != 0 {
        -1.0f64
    } else {
        1.0f64
    };
    let mut tc: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut stat_tc: libc::c_int = gsl_sf_taylorcoeff_e(n, fabs(x), &mut tc);
    if stat_tc == GSL_SUCCESS as libc::c_int {
        let mut term: libc::c_double = tc.val * tc_sgn;
        let mut sum_val: libc::c_double = term;
        let mut sum_err: libc::c_double = tc.err;
        let mut k: libc::c_int = 0;
        k = n - 1 as libc::c_int;
        while k >= 0 as libc::c_int {
            term
                *= (b + k as libc::c_double) / (n - k) as libc::c_double
                    * (k as libc::c_double + 1.0f64) / mx;
            sum_val += term;
            sum_err += 4.0f64 * 2.2204460492503131e-16f64 * fabs(term);
            k -= 1;
            k;
        }
        (*result).val = sum_val;
        (*result)
            .err = sum_err + 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if stat_tc == GSL_EOVRFLW as libc::c_int {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return stat_tc;
    } else {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return stat_tc;
    };
}
pub unsafe extern "C" fn gsl_sf_laguerre_1_e(
    a: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    (*result).val = 1.0f64 + a - x;
    (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * (1.0f64 + fabs(a) + fabs(x));
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sf_laguerre_2_e(
    a: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if a == -2.0f64 {
        (*result).val = 0.5f64 * x * x;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut c0: libc::c_double = 0.5f64 * (2.0f64 + a) * (1.0f64 + a);
        let mut c1: libc::c_double = -(2.0f64 + a);
        let mut c2: libc::c_double = -0.5f64 / (2.0f64 + a);
        (*result).val = c0 + c1 * x * (1.0f64 + c2 * x);
        (*result)
            .err = 2.0f64 * 2.2204460492503131e-16f64
            * (fabs(c0) + 2.0f64 * fabs(c1 * x) * (1.0f64 + 2.0f64 * fabs(c2 * x)));
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_laguerre_3_e(
    a: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if a == -2.0f64 {
        let mut x2_6: libc::c_double = x * x / 6.0f64;
        (*result).val = x2_6 * (3.0f64 - x);
        (*result).err = x2_6 * (3.0f64 + fabs(x)) * 2.0f64 * 2.2204460492503131e-16f64;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if a == -3.0f64 {
        (*result).val = -x * x / 6.0f64;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut c0: libc::c_double = (3.0f64 + a) * (2.0f64 + a) * (1.0f64 + a) / 6.0f64;
        let mut c1: libc::c_double = -c0 * 3.0f64 / (1.0f64 + a);
        let mut c2: libc::c_double = -1.0f64 / (2.0f64 + a);
        let mut c3: libc::c_double = -1.0f64 / (3.0f64 * (3.0f64 + a));
        (*result).val = c0 + c1 * x * (1.0f64 + c2 * x * (1.0f64 + c3 * x));
        (*result).err = 1.0f64 + 2.0f64 * fabs(c3 * x);
        (*result).err = 1.0f64 + 2.0f64 * fabs(c2 * x) * (*result).err;
        (*result)
            .err = 2.0f64 * 2.2204460492503131e-16f64
            * (fabs(c0) + 2.0f64 * fabs(c1 * x) * (*result).err);
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_laguerre_n_e(
    n: libc::c_int,
    a: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if n < 0 as libc::c_int {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"laguerre.c\0" as *const u8 as *const libc::c_char,
            244 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if n == 0 as libc::c_int {
        (*result).val = 1.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if n == 1 as libc::c_int {
        (*result).val = 1.0f64 + a - x;
        (*result)
            .err = 2.0f64 * 2.2204460492503131e-16f64 * (1.0f64 + fabs(a) + fabs(x));
        return GSL_SUCCESS as libc::c_int;
    } else if x == 0.0f64 {
        let mut product: libc::c_double = a + 1.0f64;
        let mut k: libc::c_int = 0;
        k = 2 as libc::c_int;
        while k <= n {
            product *= (a + k as libc::c_double) / k as libc::c_double;
            k += 1;
            k;
        }
        (*result).val = product;
        (*result)
            .err = 2.0f64 * (n as libc::c_double + 1.0f64) * 2.2204460492503131e-16f64
            * fabs(product) + 2.2204460492503131e-16f64;
        return GSL_SUCCESS as libc::c_int;
    } else if x < 0.0f64 && a > -1.0f64 {
        return laguerre_n_cp(n, a, x, result)
    } else if n < 5 as libc::c_int
        || x > 0.0f64 && a < (-n - 1 as libc::c_int) as libc::c_double
    {
        if laguerre_n_cp(n, a, x, result) == GSL_SUCCESS as libc::c_int {
            return GSL_SUCCESS as libc::c_int
        } else {
            return laguerre_n_poly_safe(n, a, x, result)
        }
    } else if n as libc::c_double > 1.0e+07f64 && x > 0.0f64 && a > -1.0f64
        && x < 2.0f64 * (a + 1.0f64) + 4.0f64 * n as libc::c_double
    {
        return laguerre_large_n(n, a, x, result)
    } else if a >= 0.0f64 || x > 0.0f64 && a < (-n - 1 as libc::c_int) as libc::c_double
    {
        let mut lg2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_lg2: libc::c_int = gsl_sf_laguerre_2_e(a, x, &mut lg2);
        let mut Lkm1: libc::c_double = 1.0f64 + a - x;
        let mut Lk: libc::c_double = lg2.val;
        let mut Lkp1: libc::c_double = 0.;
        let mut k_0: libc::c_int = 0;
        k_0 = 2 as libc::c_int;
        while k_0 < n {
            Lkp1 = (-(k_0 as libc::c_double + a) * Lkm1
                + (2.0f64 * k_0 as libc::c_double + a + 1.0f64 - x) * Lk)
                / (k_0 as libc::c_double + 1.0f64);
            Lkm1 = Lk;
            Lk = Lkp1;
            k_0 += 1;
            k_0;
        }
        (*result).val = Lk;
        (*result)
            .err = (fabs(lg2.err / lg2.val) + 2.2204460492503131e-16f64)
            * n as libc::c_double * fabs(Lk);
        return stat_lg2;
    } else {
        return laguerre_n_poly_safe(n, a, x, result)
    };
}
pub unsafe extern "C" fn gsl_sf_laguerre_1(
    mut a: libc::c_double,
    mut x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_laguerre_1_e(a, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_laguerre_1_e(a, x, &result)\0" as *const u8 as *const libc::c_char,
            b"laguerre.c\0" as *const u8 as *const libc::c_char,
            319 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_laguerre_2(
    mut a: libc::c_double,
    mut x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_laguerre_2_e(a, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_laguerre_2_e(a, x, &result)\0" as *const u8 as *const libc::c_char,
            b"laguerre.c\0" as *const u8 as *const libc::c_char,
            324 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_laguerre_3(
    mut a: libc::c_double,
    mut x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_laguerre_3_e(a, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_laguerre_3_e(a, x, &result)\0" as *const u8 as *const libc::c_char,
            b"laguerre.c\0" as *const u8 as *const libc::c_char,
            329 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_laguerre_n(
    mut n: libc::c_int,
    mut a: libc::c_double,
    mut x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_laguerre_n_e(n, a, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_laguerre_n_e(n, a, x, &result)\0" as *const u8
                as *const libc::c_char,
            b"laguerre.c\0" as *const u8 as *const libc::c_char,
            334 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
