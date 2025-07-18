use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sf_erfc_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_exprel_n_CF_e(
        n: libc::c_double,
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_exp_err_e(
        x: libc::c_double,
        dx: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_log_1plusx_mx_e(
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_lngamma_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_gamma_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_gammastar_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_expint_E1_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
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
unsafe extern "C" fn gsl_pow_3(x: libc::c_double) -> libc::c_double {
    return x * x * x;
}
unsafe extern "C" fn gamma_inc_D(
    a: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if a < 10.0f64 {
        let mut lnr: libc::c_double = 0.;
        let mut lg: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        gsl_sf_lngamma_e(a + 1.0f64, &mut lg);
        lnr = a * log(x) - x - lg.val;
        (*result).val = exp(lnr);
        (*result)
            .err = 2.0f64 * 2.2204460492503131e-16f64 * (fabs(lnr) + 1.0f64)
            * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut gstar: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut ln_term: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut term1: libc::c_double = 0.;
        if x < 0.5f64 * a {
            let mut u: libc::c_double = x / a;
            let mut ln_u: libc::c_double = log(u);
            ln_term.val = ln_u - u + 1.0f64;
            ln_term.err = (fabs(ln_u) + fabs(u) + 1.0f64) * 2.2204460492503131e-16f64;
        } else {
            let mut mu: libc::c_double = (x - a) / a;
            gsl_sf_log_1plusx_mx_e(mu, &mut ln_term);
            ln_term.err += 2.2204460492503131e-16f64 * fabs(mu);
        }
        gsl_sf_gammastar_e(a, &mut gstar);
        term1 = exp(a * ln_term.val) / sqrt(2.0f64 * 3.14159265358979323846f64 * a);
        (*result).val = term1 / gstar.val;
        (*result)
            .err = 2.0f64 * 2.2204460492503131e-16f64 * (fabs(a * ln_term.val) + 1.0f64)
            * fabs((*result).val);
        (*result).err += fabs(a) * ln_term.err * fabs((*result).val);
        (*result).err += gstar.err / fabs(gstar.val) * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn gamma_inc_P_series(
    a: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let nmax: libc::c_int = 10000 as libc::c_int;
    let mut D: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut stat_D: libc::c_int = gamma_inc_D(a, x, &mut D);
    if x > 0.995f64 * a && a > 1e5f64 {
        let mut cf_res: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut status: libc::c_int = gsl_sf_exprel_n_CF_e(a, x, &mut cf_res);
        (*result).val = D.val * cf_res.val;
        (*result).err = fabs(D.val * cf_res.err) + fabs(D.err * cf_res.val);
        return status;
    }
    if x > a + nmax as libc::c_double {
        gsl_error(
            b"gamma_inc_P_series x>>a exceeds range\0" as *const u8
                as *const libc::c_char,
            b"gamma_inc.c\0" as *const u8 as *const libc::c_char,
            107 as libc::c_int,
            GSL_EMAXITER as libc::c_int,
        );
        return GSL_EMAXITER as libc::c_int;
    }
    let mut sum: libc::c_double = 1.0f64;
    let mut term: libc::c_double = 1.0f64;
    let mut remainder: libc::c_double = 0.;
    let mut n: libc::c_int = 0;
    let mut nlow: libc::c_int = (if x > a {
        x - a
    } else {
        0 as libc::c_int as libc::c_double
    }) as libc::c_int;
    n = 1 as libc::c_int;
    while n < nlow {
        term *= x / (a + n as libc::c_double);
        sum += term;
        n += 1;
        n;
    }
    while n < nmax {
        term *= x / (a + n as libc::c_double);
        sum += term;
        if fabs(term / sum) < 2.2204460492503131e-16f64 {
            break;
        }
        n += 1;
        n;
    }
    let mut tnp1: libc::c_double = x / (a + n as libc::c_double) * term;
    remainder = tnp1 / (1.0f64 - x / (a + n as libc::c_double + 1.0f64));
    (*result).val = D.val * sum;
    (*result).err = D.err * fabs(sum) + fabs(D.val * remainder);
    (*result).err
        += (1.0f64 + n as libc::c_double) * 2.2204460492503131e-16f64
            * fabs((*result).val);
    if n == nmax && fabs(remainder / sum) > 1.4901161193847656e-08f64 {
        gsl_error(
            b"gamma_inc_P_series failed to converge\0" as *const u8
                as *const libc::c_char,
            b"gamma_inc.c\0" as *const u8 as *const libc::c_char,
            146 as libc::c_int,
            GSL_EMAXITER as libc::c_int,
        );
        return GSL_EMAXITER as libc::c_int;
    } else {
        return stat_D
    };
}
unsafe extern "C" fn gamma_inc_Q_large_x(
    a: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let nmax: libc::c_int = 5000 as libc::c_int;
    let mut D: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let stat_D: libc::c_int = gamma_inc_D(a, x, &mut D);
    let mut sum: libc::c_double = 1.0f64;
    let mut term: libc::c_double = 1.0f64;
    let mut last: libc::c_double = 1.0f64;
    let mut n: libc::c_int = 0;
    n = 1 as libc::c_int;
    while n < nmax {
        term *= (a - n as libc::c_double) / x;
        if fabs(term / last) > 1.0f64 {
            break;
        }
        if fabs(term / sum) < 2.2204460492503131e-16f64 {
            break;
        }
        sum += term;
        last = term;
        n += 1;
        n;
    }
    (*result).val = D.val * (a / x) * sum;
    (*result).err = D.err * fabs(a / x * sum);
    (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
    if n == nmax {
        gsl_error(
            b"error in large x asymptotic\0" as *const u8 as *const libc::c_char,
            b"gamma_inc.c\0" as *const u8 as *const libc::c_char,
            181 as libc::c_int,
            GSL_EMAXITER as libc::c_int,
        );
        return GSL_EMAXITER as libc::c_int;
    } else {
        return stat_D
    };
}
unsafe extern "C" fn gamma_inc_Q_asymp_unif(
    a: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let rta: libc::c_double = sqrt(a);
    let eps: libc::c_double = (x - a) / a;
    let mut ln_term: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let stat_ln: libc::c_int = gsl_sf_log_1plusx_mx_e(eps, &mut ln_term);
    let eta: libc::c_double = (if eps >= 0.0f64 {
        1 as libc::c_int
    } else {
        -(1 as libc::c_int)
    }) as libc::c_double * sqrt(-2.0f64 * ln_term.val);
    let mut erfc: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut R: libc::c_double = 0.;
    let mut c0: libc::c_double = 0.;
    let mut c1: libc::c_double = 0.;
    gsl_sf_erfc_e(eta * rta / 1.41421356237309504880f64, &mut erfc);
    if fabs(eps) < 7.4009597974140505e-04f64 {
        c0 = -1.0f64 / 3.0f64
            + eps
                * (1.0f64 / 12.0f64
                    - eps
                        * (23.0f64 / 540.0f64
                            - eps
                                * (353.0f64 / 12960.0f64 - eps * 589.0f64 / 30240.0f64)));
        c1 = -1.0f64 / 540.0f64 - eps / 288.0f64;
    } else {
        let rt_term: libc::c_double = sqrt(-2.0f64 * ln_term.val / (eps * eps));
        let lam: libc::c_double = x / a;
        c0 = (1.0f64 - 1.0f64 / rt_term) / eps;
        c1 = -(eta * eta * eta * (lam * lam + 10.0f64 * lam + 1.0f64)
            - 12.0f64 * eps * eps * eps) / (12.0f64 * eta * eta * eta * eps * eps * eps);
    }
    R = exp(-0.5f64 * a * eta * eta)
        / (1.41421356237309504880f64 * 1.77245385090551602729816748334f64 * rta)
        * (c0 + c1 / a);
    (*result).val = 0.5f64 * erfc.val + R;
    (*result)
        .err = 2.2204460492503131e-16f64 * fabs(R * 0.5f64 * a * eta * eta)
        + 0.5f64 * erfc.err;
    (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
    return stat_ln;
}
unsafe extern "C" fn gamma_inc_F_CF(
    a: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let nmax: libc::c_int = 5000 as libc::c_int;
    let small: libc::c_double = gsl_pow_3(2.2204460492503131e-16f64);
    let mut hn: libc::c_double = 1.0f64;
    let mut Cn: libc::c_double = 1.0f64 / small;
    let mut Dn: libc::c_double = 1.0f64;
    let mut n: libc::c_int = 0;
    n = 2 as libc::c_int;
    while n < nmax {
        let mut an: libc::c_double = 0.;
        let mut delta: libc::c_double = 0.;
        if n & 1 as libc::c_int != 0 {
            an = 0.5f64 * (n - 1 as libc::c_int) as libc::c_double / x;
        } else {
            an = (0.5f64 * n as libc::c_double - a) / x;
        }
        Dn = 1.0f64 + an * Dn;
        if fabs(Dn) < small {
            Dn = small;
        }
        Cn = 1.0f64 + an / Cn;
        if fabs(Cn) < small {
            Cn = small;
        }
        Dn = 1.0f64 / Dn;
        delta = Cn * Dn;
        hn *= delta;
        if fabs(delta - 1.0f64) < 2.2204460492503131e-16f64 {
            break;
        }
        n += 1;
        n;
    }
    (*result).val = hn;
    (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs(hn);
    (*result).err
        += 2.2204460492503131e-16f64 * (2.0f64 + 0.5f64 * n as libc::c_double)
            * fabs((*result).val);
    if n == nmax {
        gsl_error(
            b"error in CF for F(a,x)\0" as *const u8 as *const libc::c_char,
            b"gamma_inc.c\0" as *const u8 as *const libc::c_char,
            286 as libc::c_int,
            GSL_EMAXITER as libc::c_int,
        );
        return GSL_EMAXITER as libc::c_int;
    } else {
        return GSL_SUCCESS as libc::c_int
    };
}
unsafe extern "C" fn gamma_inc_Q_CF(
    a: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut D: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut F: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let stat_D: libc::c_int = gamma_inc_D(a, x, &mut D);
    let stat_F: libc::c_int = gamma_inc_F_CF(a, x, &mut F);
    (*result).val = D.val * (a / x) * F.val;
    (*result).err = D.err * fabs(a / x * F.val) + fabs(D.val * a / x * F.err);
    return if stat_F != GSL_SUCCESS as libc::c_int {
        stat_F
    } else if stat_D != GSL_SUCCESS as libc::c_int {
        stat_D
    } else {
        GSL_SUCCESS as libc::c_int
    };
}
unsafe extern "C" fn gamma_inc_Q_series(
    a: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut term1: libc::c_double = 0.;
    let mut sum: libc::c_double = 0.;
    let mut stat_sum: libc::c_int = 0;
    let mut term2: libc::c_double = 0.;
    let pg21: libc::c_double = -2.404113806319188570799476f64;
    let lnx: libc::c_double = log(x);
    let el: libc::c_double = 0.57721566490153286060651209008f64 + lnx;
    let c1: libc::c_double = -el;
    let c2: libc::c_double = 3.14159265358979323846f64 * 3.14159265358979323846f64
        / 12.0f64 - 0.5f64 * el * el;
    let c3: libc::c_double = el
        * (3.14159265358979323846f64 * 3.14159265358979323846f64 / 12.0f64
            - el * el / 6.0f64) + pg21 / 6.0f64;
    let c4: libc::c_double = -0.04166666666666666667f64
        * (-1.758243446661483480f64 + lnx) * (-0.764428657272716373f64 + lnx)
        * (0.723980571623507657f64 + lnx) * (4.107554191916823640f64 + lnx);
    let c5: libc::c_double = -0.0083333333333333333f64 * (-2.06563396085715900f64 + lnx)
        * (-1.28459889470864700f64 + lnx) * (-0.27583535756454143f64 + lnx)
        * (1.33677371336239618f64 + lnx) * (5.17537282427561550f64 + lnx);
    let c6: libc::c_double = -0.0013888888888888889f64 * (-2.30814336454783200f64 + lnx)
        * (-1.65846557706987300f64 + lnx) * (-0.88768082560020400f64 + lnx)
        * (0.17043847751371778f64 + lnx) * (1.92135970115863890f64 + lnx)
        * (6.22578557795474900f64 + lnx);
    let c7: libc::c_double = -0.00019841269841269841f64 * (-2.5078657901291800f64 + lnx)
        * (-1.9478900888958200f64 + lnx) * (-1.3194837322612730f64 + lnx)
        * (-0.5281322700249279f64 + lnx) * (0.5913834939078759f64 + lnx)
        * (2.4876819633378140f64 + lnx) * (7.2648160783762400f64 + lnx);
    let c8: libc::c_double = -0.00002480158730158730f64 * (-2.677341544966400f64 + lnx)
        * (-2.182810448271700f64 + lnx) * (-1.649350342277400f64 + lnx)
        * (-1.014099048290790f64 + lnx) * (-0.191366955370652f64 + lnx)
        * (0.995403817918724f64 + lnx) * (3.041323283529310f64 + lnx)
        * (8.295966556941250f64 + lnx);
    let c9: libc::c_double = -2.75573192239859e-6f64 * (-2.8243487670469080f64 + lnx)
        * (-2.3798494322701120f64 + lnx) * (-1.9143674728689960f64 + lnx)
        * (-1.3814529102920370f64 + lnx) * (-0.7294312810261694f64 + lnx)
        * (0.1299079285269565f64 + lnx) * (1.3873333251885240f64 + lnx)
        * (3.5857258865210760f64 + lnx) * (9.3214237073814600f64 + lnx);
    let c10: libc::c_double = -2.75573192239859e-7f64 * (-2.9540329644556910f64 + lnx)
        * (-2.5491366926991850f64 + lnx) * (-2.1348279229279880f64 + lnx)
        * (-1.6741881076349450f64 + lnx) * (-1.1325949616098420f64 + lnx)
        * (-0.4590034650618494f64 + lnx) * (0.4399352987435699f64 + lnx)
        * (1.7702236517651670f64 + lnx) * (4.1231539047474080f64 + lnx)
        * (10.342627908148680f64 + lnx);
    term1 = a
        * (c1
            + a
                * (c2
                    + a
                        * (c3
                            + a
                                * (c4
                                    + a
                                        * (c5
                                            + a * (c6 + a * (c7 + a * (c8 + a * (c9 + a * c10)))))))));
    let nmax: libc::c_int = 5000 as libc::c_int;
    let mut t: libc::c_double = 1.0f64;
    let mut n: libc::c_int = 0;
    sum = 1.0f64;
    n = 1 as libc::c_int;
    while n < nmax {
        t *= -x / (n as libc::c_double + 1.0f64);
        sum += (a + 1.0f64) / (a + n as libc::c_double + 1.0f64) * t;
        if fabs(t / sum) < 2.2204460492503131e-16f64 {
            break;
        }
        n += 1;
        n;
    }
    if n == nmax {
        stat_sum = GSL_EMAXITER as libc::c_int;
    } else {
        stat_sum = GSL_SUCCESS as libc::c_int;
    }
    term2 = (1.0f64 - term1) * a / (a + 1.0f64) * x * sum;
    (*result).val = term1 + term2;
    (*result).err = 2.2204460492503131e-16f64 * (fabs(term1) + 2.0f64 * fabs(term2));
    (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
    return stat_sum;
}
unsafe extern "C" fn gamma_inc_series(
    mut a: libc::c_double,
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut Q: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut G: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let stat_Q: libc::c_int = gamma_inc_Q_series(a, x, &mut Q);
    let stat_G: libc::c_int = gsl_sf_gamma_e(a, &mut G);
    (*result).val = Q.val * G.val;
    (*result).err = fabs(Q.val * G.err) + fabs(Q.err * G.val);
    (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
    return if stat_Q != GSL_SUCCESS as libc::c_int {
        stat_Q
    } else if stat_G != GSL_SUCCESS as libc::c_int {
        stat_G
    } else {
        GSL_SUCCESS as libc::c_int
    };
}
unsafe extern "C" fn gamma_inc_a_gt_0(
    mut a: libc::c_double,
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut Q: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut G: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let stat_Q: libc::c_int = gsl_sf_gamma_inc_Q_e(a, x, &mut Q);
    let stat_G: libc::c_int = gsl_sf_gamma_e(a, &mut G);
    (*result).val = G.val * Q.val;
    (*result).err = fabs(G.val * Q.err) + fabs(G.err * Q.val);
    (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
    return if stat_G != GSL_SUCCESS as libc::c_int {
        stat_G
    } else if stat_Q != GSL_SUCCESS as libc::c_int {
        stat_Q
    } else {
        GSL_SUCCESS as libc::c_int
    };
}
unsafe extern "C" fn gamma_inc_CF(
    mut a: libc::c_double,
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut F: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut pre: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let am1lgx: libc::c_double = (a - 1.0f64) * log(x);
    let stat_F: libc::c_int = gamma_inc_F_CF(a, x, &mut F);
    let stat_E: libc::c_int = gsl_sf_exp_err_e(
        am1lgx - x,
        2.2204460492503131e-16f64 * fabs(am1lgx),
        &mut pre,
    );
    (*result).val = F.val * pre.val;
    (*result).err = fabs(F.err * pre.val) + fabs(F.val * pre.err);
    (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
    return if stat_F != GSL_SUCCESS as libc::c_int {
        stat_F
    } else if stat_E != GSL_SUCCESS as libc::c_int {
        stat_E
    } else {
        GSL_SUCCESS as libc::c_int
    };
}
pub unsafe extern "C" fn gsl_sf_gamma_inc_Q_e(
    a: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if a < 0.0f64 || x < 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"gamma_inc.c\0" as *const u8 as *const libc::c_char,
            503 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x == 0.0f64 {
        (*result).val = 1.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if a == 0.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if x <= 0.5f64 * a {
        let mut P: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_P: libc::c_int = gamma_inc_P_series(a, x, &mut P);
        (*result).val = 1.0f64 - P.val;
        (*result).err = P.err;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return stat_P;
    } else if a >= 1.0e+06f64 && (x - a) * (x - a) < a {
        return gamma_inc_Q_asymp_unif(a, x, result)
    } else if a < 0.2f64 && x < 5.0f64 {
        return gamma_inc_Q_series(a, x, result)
    } else if a <= x {
        if x <= 1.0e+06f64 {
            return gamma_inc_Q_CF(a, x, result)
        } else {
            return gamma_inc_Q_large_x(a, x, result)
        }
    } else if x > a - sqrt(a) {
        return gamma_inc_Q_CF(a, x, result)
    } else {
        let mut P_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_P_0: libc::c_int = gamma_inc_P_series(a, x, &mut P_0);
        (*result).val = 1.0f64 - P_0.val;
        (*result).err = P_0.err;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return stat_P_0;
    };
}
pub unsafe extern "C" fn gsl_sf_gamma_inc_P_e(
    a: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if a <= 0.0f64 || x < 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"gamma_inc.c\0" as *const u8 as *const libc::c_char,
            584 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x == 0.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if x < 20.0f64 || x < 0.5f64 * a {
        return gamma_inc_P_series(a, x, result)
    } else if a > 1.0e+06f64 && (x - a) * (x - a) < a {
        let mut Q: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_Q: libc::c_int = gamma_inc_Q_asymp_unif(a, x, &mut Q);
        (*result).val = 1.0f64 - Q.val;
        (*result).err = Q.err;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return stat_Q;
    } else if a <= x {
        let mut Q_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_Q_0: libc::c_int = 0;
        if a > 0.2f64 * x {
            stat_Q_0 = gamma_inc_Q_CF(a, x, &mut Q_0);
        } else {
            stat_Q_0 = gamma_inc_Q_large_x(a, x, &mut Q_0);
        }
        (*result).val = 1.0f64 - Q_0.val;
        (*result).err = Q_0.err;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return stat_Q_0;
    } else if (x - a) * (x - a) < a {
        let mut Q_1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_Q_1: libc::c_int = gamma_inc_Q_CF(a, x, &mut Q_1);
        (*result).val = 1.0f64 - Q_1.val;
        (*result).err = Q_1.err;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return stat_Q_1;
    } else {
        return gamma_inc_P_series(a, x, result)
    };
}
pub unsafe extern "C" fn gsl_sf_gamma_inc_e(
    a: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x < 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"gamma_inc.c\0" as *const u8 as *const libc::c_char,
            649 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x == 0.0f64 {
        return gsl_sf_gamma_e(a, result)
    } else if a == 0.0f64 {
        return gsl_sf_expint_E1_e(x, result)
    } else if a > 0.0f64 {
        return gamma_inc_a_gt_0(a, x, result)
    } else if x > 0.25f64 {
        return gamma_inc_CF(a, x, result)
    } else if fabs(a) < 0.5f64 {
        return gamma_inc_series(a, x, result)
    } else {
        let fa: libc::c_double = floor(a);
        let da: libc::c_double = a - fa;
        let mut g_da: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let stat_g_da: libc::c_int = if da > 0.0f64 {
            gamma_inc_a_gt_0(da, x, &mut g_da)
        } else {
            gsl_sf_expint_E1_e(x, &mut g_da)
        };
        let mut alpha: libc::c_double = da;
        let mut gax: libc::c_double = g_da.val;
        loop {
            let shift: libc::c_double = exp(-x + (alpha - 1.0f64) * log(x));
            gax = (gax - shift) / (alpha - 1.0f64);
            alpha -= 1.0f64;
            if !(alpha > a) {
                break;
            }
        }
        (*result).val = gax;
        (*result)
            .err = 2.0f64 * (1.0f64 + fabs(a)) * 2.2204460492503131e-16f64 * fabs(gax);
        return stat_g_da;
    };
}
pub unsafe extern "C" fn gsl_sf_gamma_inc_P(
    a: libc::c_double,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_gamma_inc_P_e(a, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_gamma_inc_P_e(a, x, &result)\0" as *const u8 as *const libc::c_char,
            b"gamma_inc.c\0" as *const u8 as *const libc::c_char,
            710 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_gamma_inc_Q(
    a: libc::c_double,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_gamma_inc_Q_e(a, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_gamma_inc_Q_e(a, x, &result)\0" as *const u8 as *const libc::c_char,
            b"gamma_inc.c\0" as *const u8 as *const libc::c_char,
            715 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_gamma_inc(
    a: libc::c_double,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_gamma_inc_e(a, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_gamma_inc_e(a, x, &result)\0" as *const u8 as *const libc::c_char,
            b"gamma_inc.c\0" as *const u8 as *const libc::c_char,
            720 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
