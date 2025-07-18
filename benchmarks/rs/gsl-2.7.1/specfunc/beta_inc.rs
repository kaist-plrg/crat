use ::libc;
extern "C" {
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sf_log_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_log_1plusx_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_exp_err_e(
        x: libc::c_double,
        dx: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_lnbeta_e(
        a: libc::c_double,
        b: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_beta_e(
        a: libc::c_double,
        b: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_hyperg_2F1_e(
        a: libc::c_double,
        b: libc::c_double,
        c: libc::c_double,
        x: libc::c_double,
        result: *mut gsl_sf_result,
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
unsafe extern "C" fn isnegint(x: libc::c_double) -> libc::c_double {
    return (x < 0 as libc::c_int as libc::c_double && x == floor(x)) as libc::c_int
        as libc::c_double;
}
unsafe extern "C" fn beta_cont_frac(
    a: libc::c_double,
    b: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let max_iter: libc::c_uint = 512 as libc::c_int as libc::c_uint;
    let cutoff: libc::c_double = 2.0f64 * 2.2250738585072014e-308f64;
    let mut iter_count: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut cf: libc::c_double = 0.;
    let mut num_term: libc::c_double = 1.0f64;
    let mut den_term: libc::c_double = 1.0f64 - (a + b) * x / (a + 1.0f64);
    if fabs(den_term) < cutoff {
        den_term = cutoff;
    }
    den_term = 1.0f64 / den_term;
    cf = den_term;
    while iter_count < max_iter {
        let k: libc::c_int = iter_count.wrapping_add(1 as libc::c_int as libc::c_uint)
            as libc::c_int;
        let mut coeff: libc::c_double = k as libc::c_double * (b - k as libc::c_double)
            * x
            / ((a - 1.0f64 + (2 as libc::c_int * k) as libc::c_double)
                * (a + (2 as libc::c_int * k) as libc::c_double));
        let mut delta_frac: libc::c_double = 0.;
        den_term = 1.0f64 + coeff * den_term;
        num_term = 1.0f64 + coeff / num_term;
        if fabs(den_term) < cutoff {
            den_term = cutoff;
        }
        if fabs(num_term) < cutoff {
            num_term = cutoff;
        }
        den_term = 1.0f64 / den_term;
        delta_frac = den_term * num_term;
        cf *= delta_frac;
        coeff = -(a + k as libc::c_double) * (a + b + k as libc::c_double) * x
            / ((a + (2 as libc::c_int * k) as libc::c_double)
                * (a + (2 as libc::c_int * k) as libc::c_double + 1.0f64));
        den_term = 1.0f64 + coeff * den_term;
        num_term = 1.0f64 + coeff / num_term;
        if fabs(den_term) < cutoff {
            den_term = cutoff;
        }
        if fabs(num_term) < cutoff {
            num_term = cutoff;
        }
        den_term = 1.0f64 / den_term;
        delta_frac = den_term * num_term;
        cf *= delta_frac;
        if fabs(delta_frac - 1.0f64) < 2.0f64 * 2.2204460492503131e-16f64 {
            break;
        }
        iter_count = iter_count.wrapping_add(1);
        iter_count;
    }
    (*result).val = cf;
    (*result)
        .err = iter_count as libc::c_double * 4.0f64 * 2.2204460492503131e-16f64
        * fabs(cf);
    if iter_count >= max_iter {
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"beta_inc.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int,
            GSL_EMAXITER as libc::c_int,
        );
        return GSL_EMAXITER as libc::c_int;
    } else {
        return GSL_SUCCESS as libc::c_int
    };
}
pub unsafe extern "C" fn gsl_sf_beta_inc_e(
    a: libc::c_double,
    b: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x < 0.0f64 || x > 1.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"beta_inc.c\0" as *const u8 as *const libc::c_char,
            115 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if isnegint(a) != 0. || isnegint(b) != 0. {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"beta_inc.c\0" as *const u8 as *const libc::c_char,
            117 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if isnegint(a + b) != 0. {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"beta_inc.c\0" as *const u8 as *const libc::c_char,
            119 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x == 0.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if x == 1.0f64 {
        (*result).val = 1.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if a <= 0 as libc::c_int as libc::c_double
        || b <= 0 as libc::c_int as libc::c_double
    {
        let mut f: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut beta: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat: libc::c_int = 0;
        let stat_f: libc::c_int = gsl_sf_hyperg_2F1_e(
            a,
            1 as libc::c_int as libc::c_double - b,
            a + 1 as libc::c_int as libc::c_double,
            x,
            &mut f,
        );
        let stat_beta: libc::c_int = gsl_sf_beta_e(a, b, &mut beta);
        let mut prefactor: libc::c_double = pow(x, a) / a;
        (*result).val = prefactor * f.val / beta.val;
        (*result)
            .err = fabs(prefactor) * f.err / fabs(beta.val)
            + fabs((*result).val / beta.val) * beta.err;
        stat = if stat_f != GSL_SUCCESS as libc::c_int {
            stat_f
        } else if stat_beta != GSL_SUCCESS as libc::c_int {
            stat_beta
        } else {
            GSL_SUCCESS as libc::c_int
        };
        if stat == GSL_SUCCESS as libc::c_int {
            if fabs((*result).val) < 2.2250738585072014e-308f64 {
                gsl_error(
                    b"underflow\0" as *const u8 as *const libc::c_char,
                    b"beta_inc.c\0" as *const u8 as *const libc::c_char,
                    140 as libc::c_int,
                    GSL_EUNDRFLW as libc::c_int,
                );
                return GSL_EUNDRFLW as libc::c_int;
            }
        }
        return stat;
    } else {
        let mut ln_beta: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut ln_x: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut ln_1mx: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut prefactor_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let stat_ln_beta: libc::c_int = gsl_sf_lnbeta_e(a, b, &mut ln_beta);
        let stat_ln_1mx: libc::c_int = gsl_sf_log_1plusx_e(-x, &mut ln_1mx);
        let stat_ln_x: libc::c_int = gsl_sf_log_e(x, &mut ln_x);
        let stat_ln: libc::c_int = if stat_ln_beta != GSL_SUCCESS as libc::c_int {
            stat_ln_beta
        } else if stat_ln_1mx != GSL_SUCCESS as libc::c_int {
            stat_ln_1mx
        } else if stat_ln_x != GSL_SUCCESS as libc::c_int {
            stat_ln_x
        } else {
            GSL_SUCCESS as libc::c_int
        };
        let ln_pre_val: libc::c_double = -ln_beta.val + a * ln_x.val + b * ln_1mx.val;
        let ln_pre_err: libc::c_double = ln_beta.err + fabs(a * ln_x.err)
            + fabs(b * ln_1mx.err);
        let stat_exp: libc::c_int = gsl_sf_exp_err_e(
            ln_pre_val,
            ln_pre_err,
            &mut prefactor_0,
        );
        if stat_ln != GSL_SUCCESS as libc::c_int {
            (*result).val = 0.0f64;
            (*result).err = 0.0f64;
            gsl_error(
                b"error\0" as *const u8 as *const libc::c_char,
                b"beta_inc.c\0" as *const u8 as *const libc::c_char,
                160 as libc::c_int,
                GSL_ESANITY as libc::c_int,
            );
            return GSL_ESANITY as libc::c_int;
        }
        if x < (a + 1.0f64) / (a + b + 2.0f64) {
            let mut cf: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let stat_cf: libc::c_int = beta_cont_frac(a, b, x, &mut cf);
            let mut stat_0: libc::c_int = 0;
            (*result).val = prefactor_0.val * cf.val / a;
            (*result)
                .err = (fabs(prefactor_0.err * cf.val) + fabs(prefactor_0.val * cf.err))
                / a;
            stat_0 = if stat_exp != GSL_SUCCESS as libc::c_int {
                stat_exp
            } else if stat_cf != GSL_SUCCESS as libc::c_int {
                stat_cf
            } else {
                GSL_SUCCESS as libc::c_int
            };
            if stat_0 == GSL_SUCCESS as libc::c_int {
                if fabs((*result).val) < 2.2250738585072014e-308f64 {
                    gsl_error(
                        b"underflow\0" as *const u8 as *const libc::c_char,
                        b"beta_inc.c\0" as *const u8 as *const libc::c_char,
                        173 as libc::c_int,
                        GSL_EUNDRFLW as libc::c_int,
                    );
                    return GSL_EUNDRFLW as libc::c_int;
                }
            }
            return stat_0;
        } else {
            let mut cf_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let stat_cf_0: libc::c_int = beta_cont_frac(b, a, 1.0f64 - x, &mut cf_0);
            let mut stat_1: libc::c_int = 0;
            let term: libc::c_double = prefactor_0.val * cf_0.val / b;
            (*result).val = 1.0f64 - term;
            (*result).err = fabs(prefactor_0.err * cf_0.val) / b;
            (*result).err += fabs(prefactor_0.val * cf_0.err) / b;
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * (1.0f64 + fabs(term));
            if stat_exp != GSL_EUNDRFLW as libc::c_int {
                stat_1 = if stat_exp != GSL_SUCCESS as libc::c_int {
                    stat_exp
                } else if stat_cf_0 != GSL_SUCCESS as libc::c_int {
                    stat_cf_0
                } else {
                    GSL_SUCCESS as libc::c_int
                };
            } else {
                stat_1 = stat_cf_0;
            }
            if stat_1 == GSL_SUCCESS as libc::c_int {
                if fabs((*result).val) < 2.2250738585072014e-308f64 {
                    gsl_error(
                        b"underflow\0" as *const u8 as *const libc::c_char,
                        b"beta_inc.c\0" as *const u8 as *const libc::c_char,
                        195 as libc::c_int,
                        GSL_EUNDRFLW as libc::c_int,
                    );
                    return GSL_EUNDRFLW as libc::c_int;
                }
            }
            return stat_1;
        }
    };
}
pub unsafe extern "C" fn gsl_sf_beta_inc(
    a: libc::c_double,
    b: libc::c_double,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_beta_inc_e(a, b, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_beta_inc_e(a, b, x, &result)\0" as *const u8 as *const libc::c_char,
            b"beta_inc.c\0" as *const u8 as *const libc::c_char,
            209 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
