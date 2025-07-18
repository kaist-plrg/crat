use ::libc;
extern "C" {
    fn cosh(_: libc::c_double) -> libc::c_double;
    fn sinh(_: libc::c_double) -> libc::c_double;
    fn tanh(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn hypot(_: libc::c_double, _: libc::c_double) -> libc::c_double;
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
    fn gsl_sf_lngamma_complex_e(
        zr: libc::c_double,
        zi: libc::c_double,
        lnr: *mut gsl_sf_result,
        arg: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_sin_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_cos_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_lnsinh_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_sin_err_e(
        x: libc::c_double,
        dx: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_conicalP_xgt1_neg_mu_largetau_e(
        mu: libc::c_double,
        tau: libc::c_double,
        x: libc::c_double,
        acosh_x: libc::c_double,
        result: *mut gsl_sf_result,
        ln_multiplier: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_sf_conicalP_large_x_e(
        mu: libc::c_double,
        tau: libc::c_double,
        x: libc::c_double,
        result: *mut gsl_sf_result,
        ln_multiplier: *mut libc::c_double,
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
unsafe extern "C" fn legendre_H3d_lnnorm(
    ell: libc::c_int,
    lambda: libc::c_double,
    mut result: *mut libc::c_double,
) -> libc::c_int {
    let mut abs_lam: libc::c_double = fabs(lambda);
    if abs_lam == 0.0f64 {
        *result = 0.0f64;
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"legendre_H3d.c\0" as *const u8 as *const libc::c_char,
            52 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if lambda > (ell as libc::c_double + 1.0f64) / 6.0554544523933429e-06f64 {
        let mut rat: libc::c_double = (ell as libc::c_double + 1.0f64) / lambda;
        let mut ln_lam2ell2: libc::c_double = 2.0f64 * log(lambda)
            + log(1.0f64 + rat * rat);
        let mut lg_corrected: libc::c_double = -2.0f64 * (ell as libc::c_double + 1.0f64)
            + 1.14472988584940017414342735135f64
            + (ell as libc::c_double + 0.5f64) * ln_lam2ell2
            + 1.0f64 / (288.0f64 * lambda * lambda);
        let mut angle_terms: libc::c_double = lambda * 2.0f64 * rat
            * (1.0f64 - rat * rat / 3.0f64);
        *result = log(abs_lam) + lg_corrected + angle_terms
            - 1.14472988584940017414342735135f64;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut lg_r: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut lg_theta: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut ln_sinh: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        gsl_sf_lngamma_complex_e(
            ell as libc::c_double + 1.0f64,
            lambda,
            &mut lg_r,
            &mut lg_theta,
        );
        gsl_sf_lnsinh_e(3.14159265358979323846f64 * abs_lam, &mut ln_sinh);
        *result = log(abs_lam) + ln_sinh.val + 2.0f64 * lg_r.val
            - 1.14472988584940017414342735135f64;
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn legendre_H3d_series(
    ell: libc::c_int,
    lambda: libc::c_double,
    eta: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let nmax: libc::c_int = 5000 as libc::c_int;
    let shheta: libc::c_double = sinh(0.5f64 * eta);
    let ln_zp1: libc::c_double = 0.69314718055994530942f64
        + log(1.0f64 + shheta * shheta);
    let ln_zm1: libc::c_double = 0.69314718055994530942f64 + 2.0f64 * log(shheta);
    let zeta: libc::c_double = -shheta * shheta;
    let mut lg_lp32: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut term: libc::c_double = 1.0f64;
    let mut sum: libc::c_double = 1.0f64;
    let mut sum_err: libc::c_double = 0.0f64;
    let mut lnsheta: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut lnN: libc::c_double = 0.;
    let mut lnpre_val: libc::c_double = 0.;
    let mut lnpre_err: libc::c_double = 0.;
    let mut lnprepow: libc::c_double = 0.;
    let mut stat_e: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    gsl_sf_lngamma_e(ell as libc::c_double + 3.0f64 / 2.0f64, &mut lg_lp32);
    gsl_sf_lnsinh_e(eta, &mut lnsheta);
    legendre_H3d_lnnorm(ell, lambda, &mut lnN);
    lnprepow = 0.5f64 * (ell as libc::c_double + 0.5f64) * (ln_zm1 - ln_zp1);
    lnpre_val = lnprepow
        + 0.5f64
            * (lnN + 1.14472988584940017414342735135f64 - 0.69314718055994530942f64
                - lnsheta.val) - lg_lp32.val - log(fabs(lambda));
    lnpre_err = lnsheta.err + lg_lp32.err + 2.2204460492503131e-16f64 * fabs(lnpre_val);
    lnpre_err
        += 2.0f64 * 2.2204460492503131e-16f64
            * (fabs(lnN) + 1.14472988584940017414342735135f64
                + 0.69314718055994530942f64);
    lnpre_err
        += 2.0f64 * 2.2204460492503131e-16f64
            * (0.5f64 * (ell as libc::c_double + 0.5f64)
                * (fabs(ln_zm1) + fabs(ln_zp1)));
    n = 1 as libc::c_int;
    while n < nmax {
        let mut aR: libc::c_double = n as libc::c_double - 0.5f64;
        term
            *= (aR * aR + lambda * lambda) * zeta
                / ((ell + n) as libc::c_double + 0.5f64) / n as libc::c_double;
        sum += term;
        sum_err += 2.0f64 * 2.2204460492503131e-16f64 * fabs(term);
        if fabs(term / sum) < 2.0f64 * 2.2204460492503131e-16f64 {
            break;
        }
        n += 1;
        n;
    }
    stat_e = gsl_sf_exp_mult_err_e(
        lnpre_val,
        lnpre_err,
        sum,
        fabs(term) + sum_err,
        result,
    );
    return if stat_e != GSL_SUCCESS as libc::c_int {
        stat_e
    } else if (if n == nmax {
        GSL_EMAXITER as libc::c_int
    } else {
        GSL_SUCCESS as libc::c_int
    }) != GSL_SUCCESS as libc::c_int
    {
        if n == nmax { GSL_EMAXITER as libc::c_int } else { GSL_SUCCESS as libc::c_int }
    } else {
        GSL_SUCCESS as libc::c_int
    };
}
unsafe extern "C" fn legendre_H3d_CF1_ser(
    ell: libc::c_int,
    lambda: libc::c_double,
    coth_eta: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let pre: libc::c_double = hypot(lambda, ell as libc::c_double + 1.0f64)
        / ((2.0f64 * ell as libc::c_double + 3 as libc::c_int as libc::c_double)
            * coth_eta);
    let maxk: libc::c_int = 20000 as libc::c_int;
    let mut tk: libc::c_double = 1.0f64;
    let mut sum: libc::c_double = 1.0f64;
    let mut rhok: libc::c_double = 0.0f64;
    let mut sum_err: libc::c_double = 0.0f64;
    let mut k: libc::c_int = 0;
    k = 1 as libc::c_int;
    while k < maxk {
        let mut tlk: libc::c_double = 2.0f64 * ell as libc::c_double + 1.0f64
            + 2.0f64 * k as libc::c_double;
        let mut l1k: libc::c_double = ell as libc::c_double + 1.0f64
            + k as libc::c_double;
        let mut ak: libc::c_double = -(lambda * lambda + l1k * l1k)
            / (tlk * (tlk + 2.0f64) * coth_eta * coth_eta);
        rhok = -ak * (1.0f64 + rhok) / (1.0f64 + ak * (1.0f64 + rhok));
        tk *= rhok;
        sum += tk;
        sum_err += 2.0f64 * 2.2204460492503131e-16f64 * k as libc::c_double * fabs(tk);
        if fabs(tk / sum) < 2.2204460492503131e-16f64 {
            break;
        }
        k += 1;
        k;
    }
    (*result).val = pre * sum;
    (*result).err = fabs(pre * tk);
    (*result).err += fabs(pre * sum_err);
    (*result).err += 4.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
    if k >= maxk {
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"legendre_H3d.c\0" as *const u8 as *const libc::c_char,
            238 as libc::c_int,
            GSL_EMAXITER as libc::c_int,
        );
        return GSL_EMAXITER as libc::c_int;
    } else {
        return GSL_SUCCESS as libc::c_int
    };
}
pub unsafe extern "C" fn gsl_sf_legendre_H3d_0_e(
    lambda: libc::c_double,
    eta: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if eta < 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"legendre_H3d.c\0" as *const u8 as *const libc::c_char,
            253 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if eta == 0.0f64 || lambda == 0.0f64 {
        (*result).val = 1.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let lam_eta: libc::c_double = lambda * eta;
        let mut s: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        gsl_sf_sin_err_e(
            lam_eta,
            2.0f64 * 2.2204460492503131e-16f64 * fabs(lam_eta),
            &mut s,
        );
        if eta > -0.5f64 * -3.6043653389117154e+01f64 {
            let mut f: libc::c_double = 2.0f64 / lambda * exp(-eta);
            (*result).val = f * s.val;
            (*result)
                .err = fabs(f * s.val) * (fabs(eta) + 1.0f64)
                * 2.2204460492503131e-16f64;
            (*result).err += fabs(f) * s.err;
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        } else {
            let mut f_0: libc::c_double = 1.0f64 / (lambda * sinh(eta));
            (*result).val = f_0 * s.val;
            (*result)
                .err = fabs(f_0 * s.val) * (fabs(eta) + 1.0f64)
                * 2.2204460492503131e-16f64;
            (*result).err += fabs(f_0) * s.err;
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_legendre_H3d_1_e(
    lambda: libc::c_double,
    eta: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let xi: libc::c_double = fabs(eta * lambda);
    let lsq: libc::c_double = lambda * lambda;
    let lsqp1: libc::c_double = lsq + 1.0f64;
    if eta < 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"legendre_H3d.c\0" as *const u8 as *const libc::c_char,
            293 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if eta == 0.0f64 || lambda == 0.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if xi < 7.4009597974140505e-04f64 && eta < 7.4009597974140505e-04f64 {
        let mut etasq: libc::c_double = eta * eta;
        let mut xisq: libc::c_double = xi * xi;
        let mut term1: libc::c_double = (etasq + xisq) / 3.0f64;
        let mut term2: libc::c_double = -(2.0f64 * etasq * etasq + 5.0f64 * etasq * xisq
            + 3.0f64 * xisq * xisq) / 90.0f64;
        let mut sinh_term: libc::c_double = 1.0f64
            - eta * eta / 6.0f64 * (1.0f64 - 7.0f64 / 60.0f64 * eta * eta);
        let mut pre: libc::c_double = sinh_term / sqrt(lsqp1) / eta;
        (*result).val = pre * (term1 + term2);
        (*result).err = pre * 2.2204460492503131e-16f64 * (fabs(term1) + fabs(term2));
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut sin_term: libc::c_double = 0.;
        let mut cos_term: libc::c_double = 0.;
        let mut coth_term: libc::c_double = 0.;
        let mut sinh_term_0: libc::c_double = 0.;
        let mut sin_term_err: libc::c_double = 0.;
        let mut cos_term_err: libc::c_double = 0.;
        let mut t1: libc::c_double = 0.;
        let mut pre_val: libc::c_double = 0.;
        let mut pre_err: libc::c_double = 0.;
        let mut term1_0: libc::c_double = 0.;
        let mut term2_0: libc::c_double = 0.;
        if xi < 7.4009597974140505e-04f64 {
            sin_term = 1.0f64 - xi * xi / 6.0f64 * (1.0f64 - xi * xi / 20.0f64);
            cos_term = 1.0f64 - 0.5f64 * xi * xi * (1.0f64 - xi * xi / 12.0f64);
            sin_term_err = 2.2204460492503131e-16f64;
            cos_term_err = 2.2204460492503131e-16f64;
        } else {
            let mut sin_xi_result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut cos_xi_result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            gsl_sf_sin_e(xi, &mut sin_xi_result);
            gsl_sf_cos_e(xi, &mut cos_xi_result);
            sin_term = sin_xi_result.val / xi;
            cos_term = cos_xi_result.val;
            sin_term_err = sin_xi_result.err / fabs(xi);
            cos_term_err = cos_xi_result.err;
        }
        if eta < 7.4009597974140505e-04f64 {
            coth_term = 1.0f64 + eta * eta / 3.0f64 * (1.0f64 - eta * eta / 15.0f64);
            sinh_term_0 = 1.0f64
                - eta * eta / 6.0f64 * (1.0f64 - 7.0f64 / 60.0f64 * eta * eta);
        } else {
            coth_term = eta / tanh(eta);
            sinh_term_0 = eta / sinh(eta);
        }
        t1 = sqrt(lsqp1) * eta;
        pre_val = sinh_term_0 / t1;
        pre_err = 2.0f64 * 2.2204460492503131e-16f64 * fabs(pre_val);
        term1_0 = sin_term * coth_term;
        term2_0 = cos_term;
        (*result).val = pre_val * (term1_0 - term2_0);
        (*result).err = pre_err * fabs(term1_0 - term2_0);
        (*result).err += pre_val * (sin_term_err * coth_term + cos_term_err);
        (*result).err
            += pre_val * fabs(term1_0 - term2_0) * (fabs(eta) + 1.0f64)
                * 2.2204460492503131e-16f64;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_legendre_H3d_e(
    ell: libc::c_int,
    lambda: libc::c_double,
    eta: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let abs_lam: libc::c_double = fabs(lambda);
    let lsq: libc::c_double = abs_lam * abs_lam;
    let xi: libc::c_double = abs_lam * eta;
    let cosh_eta: libc::c_double = cosh(eta);
    if eta < 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"legendre_H3d.c\0" as *const u8 as *const libc::c_char,
            375 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if eta > 7.0978271289338397e+02f64 {
        (*result).val = ::std::f32::INFINITY as libc::c_double;
        (*result).err = ::std::f32::INFINITY as libc::c_double;
        gsl_error(
            b"overflow\0" as *const u8 as *const libc::c_char,
            b"legendre_H3d.c\0" as *const u8 as *const libc::c_char,
            379 as libc::c_int,
            GSL_EOVRFLW as libc::c_int,
        );
        return GSL_EOVRFLW as libc::c_int;
    } else if ell == 0 as libc::c_int {
        return gsl_sf_legendre_H3d_0_e(lambda, eta, result)
    } else if ell == 1 as libc::c_int {
        return gsl_sf_legendre_H3d_1_e(lambda, eta, result)
    } else if eta == 0.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if xi < 1.0f64 {
        return legendre_H3d_series(ell, lambda, eta, result)
    } else if ((ell * ell) as libc::c_double + lsq) / sqrt(1.0f64 + lsq)
        / (cosh_eta * cosh_eta) < 5.0f64 * 6.0554544523933429e-06f64
    {
        let mut P: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut lm: libc::c_double = 0.;
        let mut stat_P: libc::c_int = gsl_sf_conicalP_large_x_e(
            -ell as libc::c_double - 0.5f64,
            lambda,
            cosh_eta,
            &mut P,
            &mut lm,
        );
        if P.val == 0.0f64 {
            (*result).val = 0.0f64;
            (*result).err = 0.0f64;
            return stat_P;
        } else {
            let mut lnN: libc::c_double = 0.;
            let mut lnsh: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut ln_abslam: libc::c_double = 0.;
            let mut lnpre_val: libc::c_double = 0.;
            let mut lnpre_err: libc::c_double = 0.;
            let mut stat_e: libc::c_int = 0;
            gsl_sf_lnsinh_e(eta, &mut lnsh);
            legendre_H3d_lnnorm(ell, lambda, &mut lnN);
            ln_abslam = log(abs_lam);
            lnpre_val = 0.5f64
                * (1.14472988584940017414342735135f64 + lnN - 0.69314718055994530942f64
                    - lnsh.val) - ln_abslam;
            lnpre_err = lnsh.err;
            lnpre_err
                += 2.0f64 * 2.2204460492503131e-16f64
                    * (0.5f64
                        * (1.14472988584940017414342735135f64 + 0.69314718055994530942f64
                            + fabs(lnN)) + fabs(ln_abslam));
            lnpre_err += 2.0f64 * 2.2204460492503131e-16f64 * fabs(lnpre_val);
            stat_e = gsl_sf_exp_mult_err_e(
                lnpre_val + lm,
                lnpre_err,
                P.val,
                P.err,
                result,
            );
            return if stat_e != GSL_SUCCESS as libc::c_int {
                stat_e
            } else if stat_P != GSL_SUCCESS as libc::c_int {
                stat_P
            } else {
                GSL_SUCCESS as libc::c_int
            };
        }
    } else if abs_lam > 1000.0f64 * ell as libc::c_double * ell as libc::c_double {
        let mut P_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut lm_0: libc::c_double = 0.;
        let mut stat_P_0: libc::c_int = gsl_sf_conicalP_xgt1_neg_mu_largetau_e(
            ell as libc::c_double + 0.5f64,
            lambda,
            cosh_eta,
            eta,
            &mut P_0,
            &mut lm_0,
        );
        if P_0.val == 0.0f64 {
            (*result).val = 0.0f64;
            (*result).err = 0.0f64;
            return stat_P_0;
        } else {
            let mut lnN_0: libc::c_double = 0.;
            let mut lnsh_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut ln_abslam_0: libc::c_double = 0.;
            let mut lnpre_val_0: libc::c_double = 0.;
            let mut lnpre_err_0: libc::c_double = 0.;
            let mut stat_e_0: libc::c_int = 0;
            gsl_sf_lnsinh_e(eta, &mut lnsh_0);
            legendre_H3d_lnnorm(ell, lambda, &mut lnN_0);
            ln_abslam_0 = log(abs_lam);
            lnpre_val_0 = 0.5f64
                * (1.14472988584940017414342735135f64 + lnN_0 - 0.69314718055994530942f64
                    - lnsh_0.val) - ln_abslam_0;
            lnpre_err_0 = lnsh_0.err;
            lnpre_err_0
                += 2.2204460492503131e-16f64
                    * (0.5f64
                        * (1.14472988584940017414342735135f64 + 0.69314718055994530942f64
                            + fabs(lnN_0)) + fabs(ln_abslam_0));
            lnpre_err_0 += 2.0f64 * 2.2204460492503131e-16f64 * fabs(lnpre_val_0);
            stat_e_0 = gsl_sf_exp_mult_err_e(
                lnpre_val_0 + lm_0,
                lnpre_err_0,
                P_0.val,
                P_0.err,
                result,
            );
            return if stat_e_0 != GSL_SUCCESS as libc::c_int {
                stat_e_0
            } else if stat_P_0 != GSL_SUCCESS as libc::c_int {
                stat_P_0
            } else {
                GSL_SUCCESS as libc::c_int
            };
        }
    } else {
        let coth_eta: libc::c_double = 1.0f64 / tanh(eta);
        let coth_err_mult: libc::c_double = fabs(eta) + 1.0f64;
        let mut rH: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_CF1: libc::c_int = legendre_H3d_CF1_ser(
            ell,
            lambda,
            coth_eta,
            &mut rH,
        );
        let mut Hlm1: libc::c_double = 0.;
        let mut Hl: libc::c_double = 1.4916681462400413e-154f64;
        let mut Hlp1: libc::c_double = rH.val * Hl;
        let mut lp: libc::c_int = 0;
        lp = ell;
        while lp > 0 as libc::c_int {
            let mut root_term_0: libc::c_double = hypot(lambda, lp as libc::c_double);
            let mut root_term_1: libc::c_double = hypot(
                lambda,
                lp as libc::c_double + 1.0f64,
            );
            Hlm1 = ((2.0f64 * lp as libc::c_double + 1.0f64) * coth_eta * Hl
                - root_term_1 * Hlp1) / root_term_0;
            Hlp1 = Hl;
            Hl = Hlm1;
            lp -= 1;
            lp;
        }
        if fabs(Hl) > fabs(Hlp1) {
            let mut H0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut stat_H0: libc::c_int = gsl_sf_legendre_H3d_0_e(lambda, eta, &mut H0);
            (*result).val = 1.4916681462400413e-154f64 / Hl * H0.val;
            (*result).err = 1.4916681462400413e-154f64 / fabs(Hl) * H0.err;
            (*result).err
                += fabs(rH.err / rH.val) * (ell as libc::c_double + 1.0f64)
                    * coth_err_mult * fabs((*result).val);
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
            return if stat_H0 != GSL_SUCCESS as libc::c_int {
                stat_H0
            } else if stat_CF1 != GSL_SUCCESS as libc::c_int {
                stat_CF1
            } else {
                GSL_SUCCESS as libc::c_int
            };
        } else {
            let mut H1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut stat_H1: libc::c_int = gsl_sf_legendre_H3d_1_e(lambda, eta, &mut H1);
            (*result).val = 1.4916681462400413e-154f64 / Hlp1 * H1.val;
            (*result).err = 1.4916681462400413e-154f64 / fabs(Hlp1) * H1.err;
            (*result).err
                += fabs(rH.err / rH.val) * (ell as libc::c_double + 1.0f64)
                    * coth_err_mult * fabs((*result).val);
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
            return if stat_H1 != GSL_SUCCESS as libc::c_int {
                stat_H1
            } else if stat_CF1 != GSL_SUCCESS as libc::c_int {
                stat_CF1
            } else {
                GSL_SUCCESS as libc::c_int
            };
        }
    };
}
pub unsafe extern "C" fn gsl_sf_legendre_H3d_array(
    lmax: libc::c_int,
    lambda: libc::c_double,
    eta: libc::c_double,
    mut result_array: *mut libc::c_double,
) -> libc::c_int {
    if eta < 0.0f64 || lmax < 0 as libc::c_int {
        let mut ell: libc::c_int = 0;
        ell = 0 as libc::c_int;
        while ell <= lmax {
            *result_array.offset(ell as isize) = 0.0f64;
            ell += 1;
            ell;
        }
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"legendre_H3d.c\0" as *const u8 as *const libc::c_char,
            503 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if eta > 7.0978271289338397e+02f64 {
        let mut ell_0: libc::c_int = 0;
        ell_0 = 0 as libc::c_int;
        while ell_0 <= lmax {
            *result_array.offset(ell_0 as isize) = 0.0f64;
            ell_0 += 1;
            ell_0;
        }
        gsl_error(
            b"overflow\0" as *const u8 as *const libc::c_char,
            b"legendre_H3d.c\0" as *const u8 as *const libc::c_char,
            509 as libc::c_int,
            GSL_EOVRFLW as libc::c_int,
        );
        return GSL_EOVRFLW as libc::c_int;
    } else if lmax == 0 as libc::c_int {
        let mut H0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat: libc::c_int = gsl_sf_legendre_H3d_e(
            0 as libc::c_int,
            lambda,
            eta,
            &mut H0,
        );
        *result_array.offset(0 as libc::c_int as isize) = H0.val;
        return stat;
    } else {
        let mut r_Hlp1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut r_Hl: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_lmax: libc::c_int = gsl_sf_legendre_H3d_e(
            lmax,
            lambda,
            eta,
            &mut r_Hlp1,
        );
        let mut stat_lmaxm1: libc::c_int = gsl_sf_legendre_H3d_e(
            lmax - 1 as libc::c_int,
            lambda,
            eta,
            &mut r_Hl,
        );
        let mut stat_max: libc::c_int = if stat_lmax != GSL_SUCCESS as libc::c_int {
            stat_lmax
        } else if stat_lmaxm1 != GSL_SUCCESS as libc::c_int {
            stat_lmaxm1
        } else {
            GSL_SUCCESS as libc::c_int
        };
        let coth_eta: libc::c_double = 1.0f64 / tanh(eta);
        let mut stat_recursion: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut Hlp1: libc::c_double = r_Hlp1.val;
        let mut Hl: libc::c_double = r_Hl.val;
        let mut Hlm1: libc::c_double = 0.;
        let mut ell_1: libc::c_int = 0;
        *result_array.offset(lmax as isize) = Hlp1;
        *result_array.offset((lmax - 1 as libc::c_int) as isize) = Hl;
        ell_1 = lmax - 1 as libc::c_int;
        while ell_1 > 0 as libc::c_int {
            let mut root_term_0: libc::c_double = hypot(lambda, ell_1 as libc::c_double);
            let mut root_term_1: libc::c_double = hypot(
                lambda,
                ell_1 as libc::c_double + 1.0f64,
            );
            Hlm1 = ((2.0f64 * ell_1 as libc::c_double + 1.0f64) * coth_eta * Hl
                - root_term_1 * Hlp1) / root_term_0;
            *result_array.offset((ell_1 - 1 as libc::c_int) as isize) = Hlm1;
            if !(Hlm1 < 1.7976931348623157e+308f64) {
                stat_recursion = GSL_EOVRFLW as libc::c_int;
            }
            Hlp1 = Hl;
            Hl = Hlm1;
            ell_1 -= 1;
            ell_1;
        }
        return if stat_recursion != GSL_SUCCESS as libc::c_int {
            stat_recursion
        } else if stat_max != GSL_SUCCESS as libc::c_int {
            stat_max
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
pub unsafe extern "C" fn gsl_sf_legendre_H3d_0(
    lambda: libc::c_double,
    eta: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_legendre_H3d_0_e(lambda, eta, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_legendre_H3d_0_e(lambda, eta, &result)\0" as *const u8
                as *const libc::c_char,
            b"legendre_H3d.c\0" as *const u8 as *const libc::c_char,
            557 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_legendre_H3d_1(
    lambda: libc::c_double,
    eta: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_legendre_H3d_1_e(lambda, eta, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_legendre_H3d_1_e(lambda, eta, &result)\0" as *const u8
                as *const libc::c_char,
            b"legendre_H3d.c\0" as *const u8 as *const libc::c_char,
            562 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_legendre_H3d(
    l: libc::c_int,
    lambda: libc::c_double,
    eta: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_legendre_H3d_e(l, lambda, eta, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_legendre_H3d_e(l, lambda, eta, &result)\0" as *const u8
                as *const libc::c_char,
            b"legendre_H3d.c\0" as *const u8 as *const libc::c_char,
            567 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
