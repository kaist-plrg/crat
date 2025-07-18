use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sf_exp_err_e(
        x: libc::c_double,
        dx: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_lngamma_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_lngamma_sgn_e(
        x: libc::c_double,
        result_lg: *mut gsl_sf_result,
        sgn: *mut libc::c_double,
    ) -> libc::c_int;
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
pub unsafe extern "C" fn gsl_sf_hyperg_1F1_series_e(
    a: libc::c_double,
    b: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut an: libc::c_double = a;
    let mut bn: libc::c_double = b;
    let mut n: libc::c_double = 1.0f64;
    let mut del: libc::c_double = 1.0f64;
    let mut abs_del: libc::c_double = 1.0f64;
    let mut max_abs_del: libc::c_double = 1.0f64;
    let mut sum_val: libc::c_double = 1.0f64;
    let mut sum_err: libc::c_double = 0.0f64;
    while abs_del / fabs(sum_val) > 0.25f64 * 2.2204460492503131e-16f64 {
        let mut u: libc::c_double = 0.;
        let mut abs_u: libc::c_double = 0.;
        if bn == 0.0f64 {
            (*result).val = ::std::f32::NAN as libc::c_double;
            (*result).err = ::std::f32::NAN as libc::c_double;
            gsl_error(
                b"domain error\0" as *const u8 as *const libc::c_char,
                b"hyperg.c\0" as *const u8 as *const libc::c_char,
                55 as libc::c_int,
                GSL_EDOM as libc::c_int,
            );
            return GSL_EDOM as libc::c_int;
        }
        if an == 0.0f64 {
            (*result).val = sum_val;
            (*result).err = sum_err;
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * n * fabs(sum_val);
            return GSL_SUCCESS as libc::c_int;
        }
        if n > 10000.0f64 {
            (*result).val = sum_val;
            (*result).err = sum_err;
            gsl_error(
                b"hypergeometric series failed to converge\0" as *const u8
                    as *const libc::c_char,
                b"hyperg.c\0" as *const u8 as *const libc::c_char,
                68 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        u = x * (an / (bn * n));
        abs_u = fabs(u);
        if abs_u > 1.0f64 && max_abs_del > 1.7976931348623157e+308f64 / abs_u {
            (*result).val = sum_val;
            (*result).err = fabs(sum_val);
            gsl_error(
                b"overflow\0" as *const u8 as *const libc::c_char,
                b"hyperg.c\0" as *const u8 as *const libc::c_char,
                76 as libc::c_int,
                GSL_EOVRFLW as libc::c_int,
            );
            return GSL_EOVRFLW as libc::c_int;
        }
        del *= u;
        sum_val += del;
        if fabs(sum_val) > 1.0e-5f64 * 1.7976931348623157e+308f64 {
            (*result).val = sum_val;
            (*result).err = fabs(sum_val);
            gsl_error(
                b"overflow\0" as *const u8 as *const libc::c_char,
                b"hyperg.c\0" as *const u8 as *const libc::c_char,
                83 as libc::c_int,
                GSL_EOVRFLW as libc::c_int,
            );
            return GSL_EOVRFLW as libc::c_int;
        }
        abs_del = fabs(del);
        max_abs_del = GSL_MAX_DBL(abs_del, max_abs_del);
        sum_err += 2.0f64 * 2.2204460492503131e-16f64 * abs_del;
        an += 1.0f64;
        bn += 1.0f64;
        n += 1.0f64;
    }
    (*result).val = sum_val;
    (*result).err = sum_err;
    (*result).err += abs_del;
    (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * n * fabs(sum_val);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sf_hyperg_1F1_large_b_e(
    a: libc::c_double,
    b: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if fabs(x / b) < 1.0f64 {
        let u: libc::c_double = x / b;
        let v: libc::c_double = 1.0f64 / (1.0f64 - u);
        let pre: libc::c_double = pow(v, a);
        let uv: libc::c_double = u * v;
        let uv2: libc::c_double = uv * uv;
        let t1: libc::c_double = a * (a + 1.0f64) / (2.0f64 * b) * uv2;
        let t2a: libc::c_double = a * (a + 1.0f64) / (24.0f64 * b * b) * uv2;
        let t2b: libc::c_double = 12.0f64 + 16.0f64 * (a + 2.0f64) * uv
            + 3.0f64 * (a + 2.0f64) * (a + 3.0f64) * uv2;
        let t2: libc::c_double = t2a * t2b;
        (*result).val = pre * (1.0f64 - t1 + t2);
        (*result).err = pre * 2.2204460492503131e-16f64 * (1.0f64 + fabs(t1) + fabs(t2));
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"hyperg.c\0" as *const u8 as *const libc::c_char,
            123 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_hyperg_U_large_b_e(
    a: libc::c_double,
    b: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
    mut ln_multiplier: *mut libc::c_double,
) -> libc::c_int {
    let mut N: libc::c_double = floor(b);
    let mut eps: libc::c_double = b - N;
    if fabs(eps) < 1.4901161193847656e-08f64 {
        let mut lnpre_val: libc::c_double = 0.;
        let mut lnpre_err: libc::c_double = 0.;
        let mut M: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        if b > 1.0f64 {
            let mut tmp: libc::c_double = (1.0f64 - b) * log(x);
            let mut lg_bm1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut lg_a: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            gsl_sf_lngamma_e(b - 1.0f64, &mut lg_bm1);
            gsl_sf_lngamma_e(a, &mut lg_a);
            lnpre_val = tmp + x + lg_bm1.val - lg_a.val;
            lnpre_err = lg_bm1.err + lg_a.err
                + 2.2204460492503131e-16f64 * (fabs(x) + fabs(tmp));
            gsl_sf_hyperg_1F1_large_b_e(1.0f64 - a, 2.0f64 - b, -x, &mut M);
        } else {
            let mut lg_1mb: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut lg_1pamb: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            gsl_sf_lngamma_e(1.0f64 - b, &mut lg_1mb);
            gsl_sf_lngamma_e(1.0f64 + a - b, &mut lg_1pamb);
            lnpre_val = lg_1mb.val - lg_1pamb.val;
            lnpre_err = lg_1mb.err + lg_1pamb.err;
            gsl_sf_hyperg_1F1_large_b_e(a, b, x, &mut M);
        }
        if lnpre_val > 7.0978271289338397e+02f64 - 10.0f64 {
            (*result).val = M.val;
            (*result).err = M.err;
            *ln_multiplier = lnpre_val;
            gsl_error(
                b"overflow\0" as *const u8 as *const libc::c_char,
                b"hyperg.c\0" as *const u8 as *const libc::c_char,
                165 as libc::c_int,
                GSL_EOVRFLW as libc::c_int,
            );
            return GSL_EOVRFLW as libc::c_int;
        } else {
            let mut epre: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut stat_e: libc::c_int = gsl_sf_exp_err_e(
                lnpre_val,
                lnpre_err,
                &mut epre,
            );
            (*result).val = epre.val * M.val;
            (*result).err = epre.val * M.err + epre.err * fabs(M.val);
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
            *ln_multiplier = 0.0f64;
            return stat_e;
        }
    } else {
        let mut omb_lnx: libc::c_double = (1.0f64 - b) * log(x);
        let mut lg_1mb_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut sgn_1mb: libc::c_double = 0.;
        let mut lg_1pamb_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut sgn_1pamb: libc::c_double = 0.;
        let mut lg_bm1_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut sgn_bm1: libc::c_double = 0.;
        let mut lg_a_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut sgn_a: libc::c_double = 0.;
        let mut M1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut M2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut lnpre1_val: libc::c_double = 0.;
        let mut lnpre2_val: libc::c_double = 0.;
        let mut lnpre1_err: libc::c_double = 0.;
        let mut lnpre2_err: libc::c_double = 0.;
        let mut sgpre1: libc::c_double = 0.;
        let mut sgpre2: libc::c_double = 0.;
        gsl_sf_hyperg_1F1_large_b_e(a, b, x, &mut M1);
        gsl_sf_hyperg_1F1_large_b_e(1.0f64 - a, 2.0f64 - b, x, &mut M2);
        gsl_sf_lngamma_sgn_e(1.0f64 - b, &mut lg_1mb_0, &mut sgn_1mb);
        gsl_sf_lngamma_sgn_e(1.0f64 + a - b, &mut lg_1pamb_0, &mut sgn_1pamb);
        gsl_sf_lngamma_sgn_e(b - 1.0f64, &mut lg_bm1_0, &mut sgn_bm1);
        gsl_sf_lngamma_sgn_e(a, &mut lg_a_0, &mut sgn_a);
        lnpre1_val = lg_1mb_0.val - lg_1pamb_0.val;
        lnpre1_err = lg_1mb_0.err + lg_1pamb_0.err;
        lnpre2_val = lg_bm1_0.val - lg_a_0.val - omb_lnx - x;
        lnpre2_err = lg_bm1_0.err + lg_a_0.err
            + 2.2204460492503131e-16f64 * (fabs(omb_lnx) + fabs(x));
        sgpre1 = sgn_1mb * sgn_1pamb;
        sgpre2 = sgn_bm1 * sgn_a;
        if lnpre1_val > 7.0978271289338397e+02f64 - 10.0f64
            || lnpre2_val > 7.0978271289338397e+02f64 - 10.0f64
        {
            let mut max_lnpre_val: libc::c_double = if lnpre1_val > lnpre2_val {
                lnpre1_val
            } else {
                lnpre2_val
            };
            let mut max_lnpre_err: libc::c_double = if lnpre1_err > lnpre2_err {
                lnpre1_err
            } else {
                lnpre2_err
            };
            let mut lp1: libc::c_double = lnpre1_val - max_lnpre_val;
            let mut lp2: libc::c_double = lnpre2_val - max_lnpre_val;
            let mut t1: libc::c_double = sgpre1 * exp(lp1);
            let mut t2: libc::c_double = sgpre2 * exp(lp2);
            (*result).val = t1 * M1.val + t2 * M2.val;
            (*result).err = fabs(t1) * M1.err + fabs(t2) * M2.err;
            (*result).err
                += 2.2204460492503131e-16f64 * exp(max_lnpre_err)
                    * (fabs(t1 * M1.val) + fabs(t2 * M2.val));
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
            *ln_multiplier = max_lnpre_val;
            gsl_error(
                b"overflow\0" as *const u8 as *const libc::c_char,
                b"hyperg.c\0" as *const u8 as *const libc::c_char,
                215 as libc::c_int,
                GSL_EOVRFLW as libc::c_int,
            );
            return GSL_EOVRFLW as libc::c_int;
        } else {
            let mut t1_0: libc::c_double = sgpre1 * exp(lnpre1_val);
            let mut t2_0: libc::c_double = sgpre2 * exp(lnpre2_val);
            (*result).val = t1_0 * M1.val + t2_0 * M2.val;
            (*result).err = fabs(t1_0) * M1.err + fabs(t2_0) * M2.err;
            (*result).err
                += 2.2204460492503131e-16f64
                    * (exp(lnpre1_err) * fabs(t1_0 * M1.val)
                        + exp(lnpre2_err) * fabs(t2_0 * M2.val));
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
            *ln_multiplier = 0.0f64;
            return GSL_SUCCESS as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn gsl_sf_hyperg_2F0_series_e(
    a: libc::c_double,
    b: libc::c_double,
    x: libc::c_double,
    mut n_trunc: libc::c_int,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let maxiter: libc::c_int = 2000 as libc::c_int;
    let mut an: libc::c_double = a;
    let mut bn: libc::c_double = b;
    let mut n: libc::c_double = 1.0f64;
    let mut sum: libc::c_double = 1.0f64;
    let mut del: libc::c_double = 1.0f64;
    let mut abs_del: libc::c_double = 1.0f64;
    let mut max_abs_del: libc::c_double = 1.0f64;
    let mut last_abs_del: libc::c_double = 1.0f64;
    while abs_del / fabs(sum) > 2.2204460492503131e-16f64
        && n < maxiter as libc::c_double
    {
        let mut u: libc::c_double = an * (bn / n * x);
        let mut abs_u: libc::c_double = fabs(u);
        if abs_u > 1.0f64 && max_abs_del > 1.7976931348623157e+308f64 / abs_u {
            (*result).val = sum;
            (*result).err = fabs(sum);
            gsl_error(
                b"overflow\0" as *const u8 as *const libc::c_char,
                b"hyperg.c\0" as *const u8 as *const libc::c_char,
                263 as libc::c_int,
                GSL_EOVRFLW as libc::c_int,
            );
            return GSL_EOVRFLW as libc::c_int;
        }
        del *= u;
        sum += del;
        abs_del = fabs(del);
        if abs_del > last_abs_del {
            break;
        }
        last_abs_del = abs_del;
        max_abs_del = if abs_del > max_abs_del { abs_del } else { max_abs_del };
        an += 1.0f64;
        bn += 1.0f64;
        n += 1.0f64;
        if an == 0.0f64 || bn == 0.0f64 {
            break;
        }
        if n_trunc >= 0 as libc::c_int && n >= n_trunc as libc::c_double {
            break;
        }
    }
    (*result).val = sum;
    (*result).err = 2.2204460492503131e-16f64 * n + abs_del;
    if n >= maxiter as libc::c_double {
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"hyperg.c\0" as *const u8 as *const libc::c_char,
            288 as libc::c_int,
            GSL_EMAXITER as libc::c_int,
        );
        return GSL_EMAXITER as libc::c_int;
    } else {
        return GSL_SUCCESS as libc::c_int
    };
}
