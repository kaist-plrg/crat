use ::libc;
extern "C" {
    fn acos(_: libc::c_double) -> libc::c_double;
    fn atan(_: libc::c_double) -> libc::c_double;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn tan(_: libc::c_double) -> libc::c_double;
    fn cosh(_: libc::c_double) -> libc::c_double;
    fn sinh(_: libc::c_double) -> libc::c_double;
    fn tanh(_: libc::c_double) -> libc::c_double;
    fn acosh(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sf_exp_mult_e(
        x: libc::c_double,
        y: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_exp_err_e(
        x: libc::c_double,
        dx: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_exp_mult_err_e(
        x: libc::c_double,
        dx: libc::c_double,
        y: libc::c_double,
        dy: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_cos_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_lnsinh_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_sin_err_e(
        x: libc::c_double,
        dx: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_cos_err_e(
        x: libc::c_double,
        dx: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_lngamma_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_lngamma_complex_e(
        zr: libc::c_double,
        zi: libc::c_double,
        lnr: *mut gsl_sf_result,
        arg: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_ellint_Kcomp_e(
        k: libc::c_double,
        mode: gsl_mode_t,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_ellint_Ecomp_e(
        k: libc::c_double,
        mode: gsl_mode_t,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_bessel_J0_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_bessel_J1_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_bessel_I0_scaled_e(
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_bessel_I1_scaled_e(
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_bessel_Jnu_e(
        nu: libc::c_double,
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_bessel_Inu_e(
        nu: libc::c_double,
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_hyperg_2F1_conj_e(
        aR: libc::c_double,
        aI: libc::c_double,
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
pub type gsl_mode_t = libc::c_uint;
#[inline]
unsafe extern "C" fn GSL_MAX_DBL(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return if a > b { a } else { b };
}
unsafe extern "C" fn conicalP_negmu_xlt1_CF1(
    mu: libc::c_double,
    ell: libc::c_int,
    tau: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let RECUR_BIG: libc::c_double = 1.3407807929942596e+154f64;
    let maxiter: libc::c_int = 5000 as libc::c_int;
    let mut n: libc::c_int = 1 as libc::c_int;
    let mut xi: libc::c_double = x / (sqrt(1.0f64 - x) * sqrt(1.0f64 + x));
    let mut Anm2: libc::c_double = 1.0f64;
    let mut Bnm2: libc::c_double = 0.0f64;
    let mut Anm1: libc::c_double = 0.0f64;
    let mut Bnm1: libc::c_double = 1.0f64;
    let mut a1: libc::c_double = 1.0f64;
    let mut b1: libc::c_double = 2.0f64 * (mu + ell as libc::c_double + 1.0f64) * xi;
    let mut An: libc::c_double = b1 * Anm1 + a1 * Anm2;
    let mut Bn: libc::c_double = b1 * Bnm1 + a1 * Bnm2;
    let mut an: libc::c_double = 0.;
    let mut bn: libc::c_double = 0.;
    let mut fn_0: libc::c_double = An / Bn;
    while n < maxiter {
        let mut old_fn: libc::c_double = 0.;
        let mut del: libc::c_double = 0.;
        n += 1;
        n;
        Anm2 = Anm1;
        Bnm2 = Bnm1;
        Anm1 = An;
        Bnm1 = Bn;
        an = tau * tau
            + (mu - 0.5f64 + ell as libc::c_double + n as libc::c_double)
                * (mu - 0.5f64 + ell as libc::c_double + n as libc::c_double);
        bn = 2.0f64 * (ell as libc::c_double + mu + n as libc::c_double) * xi;
        An = bn * Anm1 + an * Anm2;
        Bn = bn * Bnm1 + an * Bnm2;
        if fabs(An) > RECUR_BIG || fabs(Bn) > RECUR_BIG {
            An /= RECUR_BIG;
            Bn /= RECUR_BIG;
            Anm1 /= RECUR_BIG;
            Bnm1 /= RECUR_BIG;
            Anm2 /= RECUR_BIG;
            Bnm2 /= RECUR_BIG;
        }
        old_fn = fn_0;
        fn_0 = An / Bn;
        del = old_fn / fn_0;
        if fabs(del - 1.0f64) < 2.0f64 * 2.2204460492503131e-16f64 {
            break;
        }
    }
    (*result).val = fn_0;
    (*result)
        .err = 4.0f64 * 2.2204460492503131e-16f64 * (sqrt(n as libc::c_double) + 1.0f64)
        * fabs(fn_0);
    if n >= maxiter {
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"legendre_con.c\0" as *const u8 as *const libc::c_char,
            108 as libc::c_int,
            GSL_EMAXITER as libc::c_int,
        );
        return GSL_EMAXITER as libc::c_int;
    } else {
        return GSL_SUCCESS as libc::c_int
    };
}
unsafe extern "C" fn conicalP_negmu_xgt1_CF1(
    mu: libc::c_double,
    ell: libc::c_int,
    tau: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let maxk: libc::c_int = 20000 as libc::c_int;
    let gamma: libc::c_double = 1.0f64 - 1.0f64 / (x * x);
    let pre: libc::c_double = sqrt(x - 1.0f64) * sqrt(x + 1.0f64)
        / (x * (2.0f64 * (ell as libc::c_double + mu + 1.0f64)));
    let mut tk: libc::c_double = 1.0f64;
    let mut sum: libc::c_double = 1.0f64;
    let mut rhok: libc::c_double = 0.0f64;
    let mut k: libc::c_int = 0;
    k = 1 as libc::c_int;
    while k < maxk {
        let mut tlk: libc::c_double = 2.0f64
            * (ell as libc::c_double + mu + k as libc::c_double);
        let mut l1k: libc::c_double = ell as libc::c_double + mu - 0.5f64 + 1.0f64
            + k as libc::c_double;
        let mut ak: libc::c_double = -(tau * tau + l1k * l1k) / (tlk * (tlk + 2.0f64))
            * gamma;
        rhok = -ak * (1.0f64 + rhok) / (1.0f64 + ak * (1.0f64 + rhok));
        tk *= rhok;
        sum += tk;
        if fabs(tk / sum) < 2.2204460492503131e-16f64 {
            break;
        }
        k += 1;
        k;
    }
    (*result).val = pre * sum;
    (*result).err = fabs(pre * tk);
    (*result).err
        += 2.0f64 * 2.2204460492503131e-16f64 * (sqrt(k as libc::c_double) + 1.0f64)
            * fabs(pre * sum);
    if k >= maxk {
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"legendre_con.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            GSL_EMAXITER as libc::c_int,
        );
        return GSL_EMAXITER as libc::c_int;
    } else {
        return GSL_SUCCESS as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn olver_U1(
    mut beta2: libc::c_double,
    mut p: libc::c_double,
) -> libc::c_double {
    return (p - 1.0f64) / (24.0f64 * (1.0f64 + beta2))
        * (3.0f64 + beta2 * (2.0f64 + 5.0f64 * p * (1.0f64 + p)));
}
#[inline]
unsafe extern "C" fn olver_U2(
    mut beta2: libc::c_double,
    mut p: libc::c_double,
) -> libc::c_double {
    let mut beta4: libc::c_double = beta2 * beta2;
    let mut p2: libc::c_double = p * p;
    let mut poly1: libc::c_double = 4.0f64 * beta4 + 84.0f64 * beta2 - 63.0f64;
    let mut poly2: libc::c_double = 16.0f64 * beta4 + 90.0f64 * beta2 - 81.0f64;
    let mut poly3: libc::c_double = beta2 * p2
        * (97.0f64 * beta2 - 432.0f64 + 77.0f64 * p * (beta2 - 6.0f64)
            - 385.0f64 * beta2 * p2 * (1.0f64 + p));
    return (1.0f64 - p) / (1152.0f64 * (1.0f64 + beta2)) * (poly1 + poly2 + poly3);
}
pub unsafe extern "C" fn gsl_sf_conicalP_xlt1_large_neg_mu_e(
    mut mu: libc::c_double,
    mut tau: libc::c_double,
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
    mut ln_multiplier: *mut libc::c_double,
) -> libc::c_int {
    let mut beta: libc::c_double = tau / mu;
    let mut beta2: libc::c_double = beta * beta;
    let mut S: libc::c_double = beta * acos((1.0f64 - beta2) / (1.0f64 + beta2));
    let mut p: libc::c_double = x / sqrt(beta2 * (1.0f64 - x * x) + 1.0f64);
    let mut lg_mup1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut lg_stat: libc::c_int = gsl_sf_lngamma_e(mu + 1.0f64, &mut lg_mup1);
    let mut ln_pre_1: libc::c_double = 0.5f64 * mu
        * (S - log(1.0f64 + beta2) + log((1.0f64 - p) / (1.0f64 + p))) - lg_mup1.val;
    let mut ln_pre_2: libc::c_double = -0.25f64 * log(1.0f64 + beta2 * (1.0f64 - x));
    let mut ln_pre_3: libc::c_double = -tau * atan(p * beta);
    let mut ln_pre: libc::c_double = ln_pre_1 + ln_pre_2 + ln_pre_3;
    let mut sum: libc::c_double = 1.0f64 - olver_U1(beta2, p) / mu
        + olver_U2(beta2, p) / (mu * mu);
    if sum == 0.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        *ln_multiplier = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut stat_e: libc::c_int = gsl_sf_exp_mult_e(ln_pre, sum, result);
        if stat_e != GSL_SUCCESS as libc::c_int {
            (*result).val = sum;
            (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs(sum);
            *ln_multiplier = ln_pre;
        } else {
            *ln_multiplier = 0.0f64;
        }
        return lg_stat;
    };
}
#[inline]
unsafe extern "C" fn olver_B0_xi(
    mut mu: libc::c_double,
    mut xi: libc::c_double,
) -> libc::c_double {
    return (1.0f64 - 4.0f64 * mu * mu) / (8.0f64 * xi)
        * (1.0f64 / tanh(xi) - 1.0f64 / xi);
}
unsafe extern "C" fn olver_A1_xi(
    mut mu: libc::c_double,
    mut xi: libc::c_double,
    mut x: libc::c_double,
) -> libc::c_double {
    let mut B: libc::c_double = olver_B0_xi(mu, xi);
    let mut psi: libc::c_double = 0.;
    if fabs(x - 1.0f64) < 1.2207031250000000e-04f64 {
        let mut y: libc::c_double = x - 1.0f64;
        let mut s: libc::c_double = -1.0f64 / 3.0f64
            + y
                * (2.0f64 / 15.0f64
                    - y * (61.0f64 / 945.0f64 - 452.0f64 / 14175.0f64 * y));
        psi = (4.0f64 * mu * mu - 1.0f64) / 16.0f64 * s;
    } else {
        psi = (4.0f64 * mu * mu - 1.0f64) / 16.0f64
            * (1.0f64 / (x * x - 1.0f64) - 1.0f64 / (xi * xi));
    }
    return 0.5f64 * xi * xi * B * B + (mu + 0.5f64) * B - psi
        + mu / 6.0f64 * (0.25f64 - mu * mu);
}
#[inline]
unsafe extern "C" fn olver_B0_th(
    mut mu: libc::c_double,
    mut theta: libc::c_double,
) -> libc::c_double {
    return -(1.0f64 - 4.0f64 * mu * mu) / (8.0f64 * theta)
        * (1.0f64 / tan(theta) - 1.0f64 / theta);
}
unsafe extern "C" fn olver_A1_th(
    mut mu: libc::c_double,
    mut theta: libc::c_double,
    mut x: libc::c_double,
) -> libc::c_double {
    let mut B: libc::c_double = olver_B0_th(mu, theta);
    let mut psi: libc::c_double = 0.;
    if fabs(x - 1.0f64) < 1.2207031250000000e-04f64 {
        let mut y: libc::c_double = 1.0f64 - x;
        let mut s: libc::c_double = -1.0f64 / 3.0f64
            + y
                * (2.0f64 / 15.0f64
                    - y * (61.0f64 / 945.0f64 - 452.0f64 / 14175.0f64 * y));
        psi = (4.0f64 * mu * mu - 1.0f64) / 16.0f64 * s;
    } else {
        psi = (4.0f64 * mu * mu - 1.0f64) / 16.0f64
            * (1.0f64 / (x * x - 1.0f64) + 1.0f64 / (theta * theta));
    }
    return -0.5f64 * theta * theta * B * B + (mu + 0.5f64) * B - psi
        + mu / 6.0f64 * (0.25f64 - mu * mu);
}
pub unsafe extern "C" fn gsl_sf_conicalP_xgt1_neg_mu_largetau_e(
    mu: libc::c_double,
    tau: libc::c_double,
    x: libc::c_double,
    mut acosh_x: libc::c_double,
    mut result: *mut gsl_sf_result,
    mut ln_multiplier: *mut libc::c_double,
) -> libc::c_int {
    let mut xi: libc::c_double = acosh_x;
    let mut ln_xi_pre: libc::c_double = 0.;
    let mut ln_pre: libc::c_double = 0.;
    let mut sumA: libc::c_double = 0.;
    let mut sumB: libc::c_double = 0.;
    let mut sum: libc::c_double = 0.;
    let mut arg: libc::c_double = 0.;
    let mut J_mup1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut J_mu: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut J_mum1: libc::c_double = 0.;
    if xi < 1.2207031250000000e-04f64 {
        ln_xi_pre = -xi * xi / 6.0f64;
    } else {
        let mut lnshxi: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        gsl_sf_lnsinh_e(xi, &mut lnshxi);
        ln_xi_pre = log(xi) - lnshxi.val;
    }
    ln_pre = 0.5f64 * ln_xi_pre - mu * log(tau);
    arg = tau * xi;
    gsl_sf_bessel_Jnu_e(mu + 1.0f64, arg, &mut J_mup1);
    gsl_sf_bessel_Jnu_e(mu, arg, &mut J_mu);
    J_mum1 = -J_mup1.val + 2.0f64 * mu / arg * J_mu.val;
    sumA = 1.0f64 - olver_A1_xi(-mu, xi, x) / (tau * tau);
    sumB = olver_B0_xi(-mu, xi);
    sum = J_mu.val * sumA - xi / tau * J_mum1 * sumB;
    if sum == 0.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        *ln_multiplier = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut stat_e: libc::c_int = gsl_sf_exp_mult_e(ln_pre, sum, result);
        if stat_e != GSL_SUCCESS as libc::c_int {
            (*result).val = sum;
            (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs(sum);
            *ln_multiplier = ln_pre;
        } else {
            *ln_multiplier = 0.0f64;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_conicalP_xlt1_neg_mu_largetau_e(
    mu: libc::c_double,
    tau: libc::c_double,
    x: libc::c_double,
    acos_x: libc::c_double,
    mut result: *mut gsl_sf_result,
    mut ln_multiplier: *mut libc::c_double,
) -> libc::c_int {
    let mut theta: libc::c_double = acos_x;
    let mut ln_th_pre: libc::c_double = 0.;
    let mut ln_pre: libc::c_double = 0.;
    let mut sumA: libc::c_double = 0.;
    let mut sumB: libc::c_double = 0.;
    let mut sum: libc::c_double = 0.;
    let mut sumerr: libc::c_double = 0.;
    let mut arg: libc::c_double = 0.;
    let mut I_mup1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut I_mu: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut I_mum1: libc::c_double = 0.;
    if theta < 1.2207031250000000e-04f64 {
        ln_th_pre = theta * theta / 6.0f64;
    } else {
        ln_th_pre = log(theta / sin(theta));
    }
    ln_pre = 0.5f64 * ln_th_pre - mu * log(tau);
    arg = tau * theta;
    gsl_sf_bessel_Inu_e(mu + 1.0f64, arg, &mut I_mup1);
    gsl_sf_bessel_Inu_e(mu, arg, &mut I_mu);
    I_mum1 = I_mup1.val + 2.0f64 * mu / arg * I_mu.val;
    sumA = 1.0f64 - olver_A1_th(-mu, theta, x) / (tau * tau);
    sumB = olver_B0_th(-mu, theta);
    sum = I_mu.val * sumA - theta / tau * I_mum1 * sumB;
    sumerr = fabs(I_mu.err * sumA);
    sumerr += fabs(I_mup1.err * theta / tau * sumB);
    sumerr += fabs(I_mu.err * theta / tau * sumB * 2.0f64 * mu / arg);
    if sum == 0.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        *ln_multiplier = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut stat_e: libc::c_int = gsl_sf_exp_mult_e(ln_pre, sum, result);
        if stat_e != GSL_SUCCESS as libc::c_int {
            (*result).val = sum;
            (*result).err = sumerr;
            (*result).err += 2.2204460492503131e-16f64 * fabs(sum);
            *ln_multiplier = ln_pre;
        } else {
            *ln_multiplier = 0.0f64;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn conicalP_hyperg_large_x(
    mu: libc::c_double,
    tau: libc::c_double,
    y: libc::c_double,
    mut reF: *mut libc::c_double,
    mut imF: *mut libc::c_double,
) -> libc::c_int {
    let kmax: libc::c_int = 1000 as libc::c_int;
    let re_a: libc::c_double = 0.25f64 - 0.5f64 * mu;
    let re_b: libc::c_double = 0.75f64 - 0.5f64 * mu;
    let re_c: libc::c_double = 1.0f64;
    let im_a: libc::c_double = -0.5f64 * tau;
    let im_b: libc::c_double = -0.5f64 * tau;
    let im_c: libc::c_double = -tau;
    let mut re_sum: libc::c_double = 1.0f64;
    let mut im_sum: libc::c_double = 0.0f64;
    let mut re_term: libc::c_double = 1.0f64;
    let mut im_term: libc::c_double = 0.0f64;
    let mut k: libc::c_int = 0;
    k = 1 as libc::c_int;
    while k <= kmax {
        let mut re_ak: libc::c_double = re_a + k as libc::c_double - 1.0f64;
        let mut re_bk: libc::c_double = re_b + k as libc::c_double - 1.0f64;
        let mut re_ck: libc::c_double = re_c + k as libc::c_double - 1.0f64;
        let mut im_ak: libc::c_double = im_a;
        let mut im_bk: libc::c_double = im_b;
        let mut im_ck: libc::c_double = im_c;
        let mut den: libc::c_double = re_ck * re_ck + im_ck * im_ck;
        let mut re_multiplier: libc::c_double = ((re_ak * re_bk - im_ak * im_bk) * re_ck
            + im_ck * (im_ak * re_bk + re_ak * im_bk)) / den;
        let mut im_multiplier: libc::c_double = ((im_ak * re_bk + re_ak * im_bk) * re_ck
            - im_ck * (re_ak * re_bk - im_ak * im_bk)) / den;
        let mut re_tmp: libc::c_double = re_multiplier * re_term
            - im_multiplier * im_term;
        let mut im_tmp: libc::c_double = im_multiplier * re_term
            + re_multiplier * im_term;
        let mut asum: libc::c_double = fabs(re_sum) + fabs(im_sum);
        re_term = y / k as libc::c_double * re_tmp;
        im_term = y / k as libc::c_double * im_tmp;
        if fabs(re_term / asum) < 2.2204460492503131e-16f64
            && fabs(im_term / asum) < 2.2204460492503131e-16f64
        {
            break;
        }
        re_sum += re_term;
        im_sum += im_term;
        k += 1;
        k;
    }
    *reF = re_sum;
    *imF = im_sum;
    if k == kmax {
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"legendre_con.c\0" as *const u8 as *const libc::c_char,
            473 as libc::c_int,
            GSL_EMAXITER as libc::c_int,
        );
        return GSL_EMAXITER as libc::c_int;
    } else {
        return GSL_SUCCESS as libc::c_int
    };
}
pub unsafe extern "C" fn gsl_sf_conicalP_large_x_e(
    mu: libc::c_double,
    tau: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
    mut ln_multiplier: *mut libc::c_double,
) -> libc::c_int {
    let mut y: libc::c_double = if x < 0.5f64 * 1.3407807929942596e+154f64 {
        1.0f64 / (x * x)
    } else {
        0.0f64
    };
    let mut reF: libc::c_double = 0.;
    let mut imF: libc::c_double = 0.;
    let mut stat_F: libc::c_int = conicalP_hyperg_large_x(
        mu,
        tau,
        y,
        &mut reF,
        &mut imF,
    );
    let mut lgr_num: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut lgth_num: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut lgr_den: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut lgth_den: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut stat_gn: libc::c_int = gsl_sf_lngamma_complex_e(
        0.0f64,
        tau,
        &mut lgr_num,
        &mut lgth_num,
    );
    let mut stat_gd: libc::c_int = gsl_sf_lngamma_complex_e(
        0.5f64 - mu,
        tau,
        &mut lgr_den,
        &mut lgth_den,
    );
    let mut angle: libc::c_double = lgth_num.val - lgth_den.val + atan2(imF, reF);
    let mut lnx: libc::c_double = log(x);
    let mut lnxp1: libc::c_double = log(x + 1.0f64);
    let mut lnxm1: libc::c_double = log(x - 1.0f64);
    let mut lnpre_const: libc::c_double = 0.5f64 * 0.69314718055994530942f64
        - 0.5f64 * 1.14472988584940017414342735135f64;
    let mut lnpre_comm: libc::c_double = (mu - 0.5f64) * lnx
        - 0.5f64 * mu * (lnxp1 + lnxm1);
    let mut lnpre_err: libc::c_double = 2.2204460492503131e-16f64
        * (0.5f64 * 0.69314718055994530942f64
            + 0.5f64 * 1.14472988584940017414342735135f64)
        + 2.2204460492503131e-16f64 * fabs((mu - 0.5f64) * lnx)
        + 2.2204460492503131e-16f64 * fabs(0.5f64 * mu) * (fabs(lnxp1) + fabs(lnxm1));
    let mut cos_result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut stat_cos: libc::c_int = gsl_sf_cos_e(
        angle + tau * (log(x) + 0.69314718055994530942f64),
        &mut cos_result,
    );
    let mut status: libc::c_int = if stat_cos != GSL_SUCCESS as libc::c_int {
        stat_cos
    } else if stat_gd != GSL_SUCCESS as libc::c_int {
        stat_gd
    } else if stat_gn != GSL_SUCCESS as libc::c_int {
        stat_gn
    } else if stat_F != GSL_SUCCESS as libc::c_int {
        stat_F
    } else {
        GSL_SUCCESS as libc::c_int
    };
    if cos_result.val == 0.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return status;
    } else {
        let mut lnFf_val: libc::c_double = 0.5f64 * log(reF * reF + imF * imF)
            + lgr_num.val - lgr_den.val;
        let mut lnFf_err: libc::c_double = lgr_num.err + lgr_den.err
            + 2.2204460492503131e-16f64 * fabs(lnFf_val);
        let mut lnnoc_val: libc::c_double = lnpre_const + lnpre_comm + lnFf_val;
        let mut lnnoc_err: libc::c_double = lnpre_err + lnFf_err
            + 2.2204460492503131e-16f64 * fabs(lnnoc_val);
        let mut stat_e: libc::c_int = gsl_sf_exp_mult_err_e(
            lnnoc_val,
            lnnoc_err,
            cos_result.val,
            cos_result.err,
            result,
        );
        if stat_e == GSL_SUCCESS as libc::c_int {
            *ln_multiplier = 0.0f64;
        } else {
            (*result).val = cos_result.val;
            (*result).err = cos_result.err;
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
            *ln_multiplier = lnnoc_val;
        }
        return status;
    };
}
unsafe extern "C" fn conicalP_xlt1_hyperg_A(
    mut mu: libc::c_double,
    mut tau: libc::c_double,
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut x2: libc::c_double = x * x;
    let mut err_amp: libc::c_double = 1.0f64
        + 1.0f64 / (2.2204460492503131e-16f64 + fabs(1.0f64 - fabs(x)));
    let mut pre_val: libc::c_double = 1.77245385090551602729816748334f64
        / pow(0.5f64 * sqrt(1 as libc::c_int as libc::c_double - x2), mu);
    let mut pre_err: libc::c_double = err_amp * 2.2204460492503131e-16f64
        * (fabs(mu) + 1.0f64) * fabs(pre_val);
    let mut ln_g1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut ln_g2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut arg_g1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut arg_g2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut F1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut F2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut pre1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut pre2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut t1_val: libc::c_double = 0.;
    let mut t1_err: libc::c_double = 0.;
    let mut t2_val: libc::c_double = 0.;
    let mut t2_err: libc::c_double = 0.;
    let mut stat_F1: libc::c_int = gsl_sf_hyperg_2F1_conj_e(
        0.25f64 - 0.5f64 * mu,
        0.5f64 * tau,
        0.5f64,
        x2,
        &mut F1,
    );
    let mut stat_F2: libc::c_int = gsl_sf_hyperg_2F1_conj_e(
        0.75f64 - 0.5f64 * mu,
        0.5f64 * tau,
        1.5f64,
        x2,
        &mut F2,
    );
    let mut status: libc::c_int = if stat_F1 != GSL_SUCCESS as libc::c_int {
        stat_F1
    } else if stat_F2 != GSL_SUCCESS as libc::c_int {
        stat_F2
    } else {
        GSL_SUCCESS as libc::c_int
    };
    gsl_sf_lngamma_complex_e(
        0.75f64 - 0.5f64 * mu,
        -0.5f64 * tau,
        &mut ln_g1,
        &mut arg_g1,
    );
    gsl_sf_lngamma_complex_e(
        0.25f64 - 0.5f64 * mu,
        -0.5f64 * tau,
        &mut ln_g2,
        &mut arg_g2,
    );
    gsl_sf_exp_err_e(-2.0f64 * ln_g1.val, 2.0f64 * ln_g1.err, &mut pre1);
    gsl_sf_exp_err_e(-2.0f64 * ln_g2.val, 2.0f64 * ln_g2.err, &mut pre2);
    pre2.val *= -2.0f64 * x;
    pre2.err *= 2.0f64 * fabs(x);
    pre2.err += 2.2204460492503131e-16f64 * fabs(pre2.val);
    t1_val = pre1.val * F1.val;
    t1_err = fabs(pre1.val) * F1.err + pre1.err * fabs(F1.val);
    t2_val = pre2.val * F2.val;
    t2_err = fabs(pre2.val) * F2.err + pre2.err * fabs(F2.val);
    (*result).val = pre_val * (t1_val + t2_val);
    (*result).err = pre_val * (t1_err + t2_err);
    (*result).err += pre_err * fabs(t1_val + t2_val);
    (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
    return status;
}
unsafe extern "C" fn conicalP_0_V(
    t: libc::c_double,
    f: libc::c_double,
    tau: libc::c_double,
    sgn: libc::c_double,
    mut V0: *mut libc::c_double,
    mut V1: *mut libc::c_double,
) -> libc::c_int {
    let mut C: [libc::c_double; 8] = [0.; 8];
    let mut T: [libc::c_double; 8] = [0.; 8];
    let mut H: [libc::c_double; 8] = [0.; 8];
    let mut V: [libc::c_double; 12] = [0.; 12];
    let mut i: libc::c_int = 0;
    T[0 as libc::c_int as usize] = 1.0f64;
    H[0 as libc::c_int as usize] = 1.0f64;
    V[0 as libc::c_int as usize] = 1.0f64;
    i = 1 as libc::c_int;
    while i <= 7 as libc::c_int {
        T[i as usize] = T[(i - 1 as libc::c_int) as usize] * t;
        H[i as usize] = H[(i - 1 as libc::c_int) as usize] * (t * f);
        i += 1;
        i;
    }
    i = 1 as libc::c_int;
    while i <= 11 as libc::c_int {
        V[i as usize] = V[(i - 1 as libc::c_int) as usize] * tau;
        i += 1;
        i;
    }
    C[0 as libc::c_int as usize] = 1.0f64;
    C[1 as libc::c_int
        as usize] = (H[1 as libc::c_int as usize] - 1.0f64)
        / (8.0f64 * T[1 as libc::c_int as usize]);
    C[2 as libc::c_int
        as usize] = (9.0f64 * H[2 as libc::c_int as usize]
        + 6.0f64 * H[1 as libc::c_int as usize] - 15.0f64
        - sgn * 8.0f64 * T[2 as libc::c_int as usize])
        / (128.0f64 * T[2 as libc::c_int as usize]);
    C[3 as libc::c_int
        as usize] = 5.0f64
        * (15.0f64 * H[3 as libc::c_int as usize]
            + 27.0f64 * H[2 as libc::c_int as usize]
            + 21.0f64 * H[1 as libc::c_int as usize] - 63.0f64
            - sgn * T[2 as libc::c_int as usize]
                * (16.0f64 * H[1 as libc::c_int as usize] + 24.0f64))
        / (1024.0f64 * T[3 as libc::c_int as usize]);
    C[4 as libc::c_int
        as usize] = 7.0f64
        * (525.0f64 * H[4 as libc::c_int as usize]
            + 1500.0f64 * H[3 as libc::c_int as usize]
            + 2430.0f64 * H[2 as libc::c_int as usize]
            + 1980.0f64 * H[1 as libc::c_int as usize] - 6435.0f64
            + 192.0f64 * T[4 as libc::c_int as usize]
            - sgn * T[2 as libc::c_int as usize]
                * (720.0f64 * H[2 as libc::c_int as usize]
                    + 1600.0f64 * H[1 as libc::c_int as usize] + 2160.0f64))
        / (32768.0f64 * T[4 as libc::c_int as usize]);
    C[5 as libc::c_int
        as usize] = 21.0f64
        * (2835.0f64 * H[5 as libc::c_int as usize]
            + 11025.0f64 * H[4 as libc::c_int as usize]
            + 24750.0f64 * H[3 as libc::c_int as usize]
            + 38610.0f64 * H[2 as libc::c_int as usize]
            + 32175.0f64 * H[1 as libc::c_int as usize] - 109395.0f64
            + T[4 as libc::c_int as usize]
                * (1984.0f64 * H[1 as libc::c_int as usize] + 4032.0f64)
            - sgn * T[2 as libc::c_int as usize]
                * (4800.0f64 * H[3 as libc::c_int as usize]
                    + 15120.0f64 * H[2 as libc::c_int as usize]
                    + 26400.0f64 * H[1 as libc::c_int as usize] + 34320.0f64))
        / (262144.0f64 * T[5 as libc::c_int as usize]);
    C[6 as libc::c_int
        as usize] = 11.0f64
        * (218295.0f64 * H[6 as libc::c_int as usize]
            + 1071630.0f64 * H[5 as libc::c_int as usize]
            + 3009825.0f64 * H[4 as libc::c_int as usize]
            + 6142500.0f64 * H[3 as libc::c_int as usize]
            + 9398025.0f64 * H[2 as libc::c_int as usize]
            + 7936110.0f64 * H[1 as libc::c_int as usize] - 27776385.0f64
            + T[4 as libc::c_int as usize]
                * (254016.0f64 * H[2 as libc::c_int as usize]
                    + 749952.0f64 * H[1 as libc::c_int as usize] + 1100736.0f64)
            - sgn * T[2 as libc::c_int as usize]
                * (441000.0f64 * H[4 as libc::c_int as usize]
                    + 1814400.0f64 * H[3 as libc::c_int as usize]
                    + 4127760.0f64 * H[2 as libc::c_int as usize]
                    + 6552000.0f64 * H[1 as libc::c_int as usize] + 8353800.0f64
                    + 31232.0f64 * T[4 as libc::c_int as usize]))
        / (4194304.0f64 * T[6 as libc::c_int as usize]);
    *V0 = C[0 as libc::c_int as usize]
        + (-4.0f64 * C[3 as libc::c_int as usize] / T[1 as libc::c_int as usize]
            + C[4 as libc::c_int as usize]) / V[4 as libc::c_int as usize]
        + (-192.0f64 * C[5 as libc::c_int as usize] / T[3 as libc::c_int as usize]
            + 144.0f64 * C[6 as libc::c_int as usize] / T[2 as libc::c_int as usize])
            / V[8 as libc::c_int as usize]
        + sgn
            * (-C[2 as libc::c_int as usize] / V[2 as libc::c_int as usize]
                + (-24.0f64 * C[4 as libc::c_int as usize] / T[2 as libc::c_int as usize]
                    + 12.0f64 * C[5 as libc::c_int as usize]
                        / T[1 as libc::c_int as usize] - C[6 as libc::c_int as usize])
                    / V[6 as libc::c_int as usize]
                + -1920.0f64 * C[6 as libc::c_int as usize]
                    / T[4 as libc::c_int as usize] / V[10 as libc::c_int as usize]);
    *V1 = C[1 as libc::c_int as usize] / V[1 as libc::c_int as usize]
        + (8.0f64
            * (C[3 as libc::c_int as usize] / T[2 as libc::c_int as usize]
                - C[4 as libc::c_int as usize] / T[1 as libc::c_int as usize])
            + C[5 as libc::c_int as usize]) / V[5 as libc::c_int as usize]
        + (384.0f64 * C[5 as libc::c_int as usize] / T[4 as libc::c_int as usize]
            - 768.0f64 * C[6 as libc::c_int as usize] / T[3 as libc::c_int as usize])
            / V[9 as libc::c_int as usize]
        + sgn
            * ((2.0f64 * C[2 as libc::c_int as usize] / T[1 as libc::c_int as usize]
                - C[3 as libc::c_int as usize]) / V[3 as libc::c_int as usize]
                + (48.0f64 * C[4 as libc::c_int as usize] / T[3 as libc::c_int as usize]
                    - 72.0f64 * C[5 as libc::c_int as usize]
                        / T[2 as libc::c_int as usize]
                    + 18.0f64 * C[6 as libc::c_int as usize]
                        / T[1 as libc::c_int as usize]) / V[7 as libc::c_int as usize]
                + 3840.0f64 * C[6 as libc::c_int as usize] / T[5 as libc::c_int as usize]
                    / V[11 as libc::c_int as usize]);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn conicalP_1_V(
    t: libc::c_double,
    f: libc::c_double,
    tau: libc::c_double,
    sgn: libc::c_double,
    mut V0: *mut libc::c_double,
    mut V1: *mut libc::c_double,
) -> libc::c_int {
    let mut Cm1: libc::c_double = 0.;
    let mut C: [libc::c_double; 8] = [0.; 8];
    let mut T: [libc::c_double; 8] = [0.; 8];
    let mut H: [libc::c_double; 8] = [0.; 8];
    let mut V: [libc::c_double; 12] = [0.; 12];
    let mut i: libc::c_int = 0;
    T[0 as libc::c_int as usize] = 1.0f64;
    H[0 as libc::c_int as usize] = 1.0f64;
    V[0 as libc::c_int as usize] = 1.0f64;
    i = 1 as libc::c_int;
    while i <= 7 as libc::c_int {
        T[i as usize] = T[(i - 1 as libc::c_int) as usize] * t;
        H[i as usize] = H[(i - 1 as libc::c_int) as usize] * (t * f);
        i += 1;
        i;
    }
    i = 1 as libc::c_int;
    while i <= 11 as libc::c_int {
        V[i as usize] = V[(i - 1 as libc::c_int) as usize] * tau;
        i += 1;
        i;
    }
    Cm1 = -1.0f64;
    C[0 as libc::c_int
        as usize] = 3.0f64 * (1.0f64 - H[1 as libc::c_int as usize])
        / (8.0f64 * T[1 as libc::c_int as usize]);
    C[1 as libc::c_int
        as usize] = (-15.0f64 * H[2 as libc::c_int as usize]
        + 6.0f64 * H[1 as libc::c_int as usize] + 9.0f64
        + sgn * 8.0f64 * T[2 as libc::c_int as usize])
        / (128.0f64 * T[2 as libc::c_int as usize]);
    C[2 as libc::c_int
        as usize] = 3.0f64
        * (-35.0f64 * H[3 as libc::c_int as usize]
            - 15.0f64 * H[2 as libc::c_int as usize]
            + 15.0f64 * H[1 as libc::c_int as usize] + 35.0f64
            + sgn * T[2 as libc::c_int as usize]
                * (32.0f64 * H[1 as libc::c_int as usize] + 8.0f64))
        / (1024.0f64 * T[3 as libc::c_int as usize]);
    C[3 as libc::c_int
        as usize] = (-4725.0f64 * H[4 as libc::c_int as usize]
        - 6300.0f64 * H[3 as libc::c_int as usize]
        - 3150.0f64 * H[2 as libc::c_int as usize]
        + 3780.0f64 * H[1 as libc::c_int as usize] + 10395.0f64
        - 1216.0f64 * T[4 as libc::c_int as usize]
        + sgn * T[2 as libc::c_int as usize]
            * (6000.0f64 * H[2 as libc::c_int as usize]
                + 5760.0f64 * H[1 as libc::c_int as usize] + 1680.0f64))
        / (32768.0f64 * T[4 as libc::c_int as usize]);
    C[4 as libc::c_int
        as usize] = 7.0f64
        * (-10395.0f64 * H[5 as libc::c_int as usize]
            - 23625.0f64 * H[4 as libc::c_int as usize]
            - 28350.0f64 * H[3 as libc::c_int as usize]
            - 14850.0f64 * H[2 as libc::c_int as usize]
            + 19305.0f64 * H[1 as libc::c_int as usize] + 57915.0f64
            - T[4 as libc::c_int as usize]
                * (6336.0f64 * H[1 as libc::c_int as usize] + 6080.0f64)
            + sgn * T[2 as libc::c_int as usize]
                * (16800.0f64 * H[3 as libc::c_int as usize]
                    + 30000.0f64 * H[2 as libc::c_int as usize]
                    + 25920.0f64 * H[1 as libc::c_int as usize] + 7920.0f64))
        / (262144.0f64 * T[5 as libc::c_int as usize]);
    C[5 as libc::c_int
        as usize] = (-2837835.0f64 * H[6 as libc::c_int as usize]
        - 9168390.0f64 * H[5 as libc::c_int as usize]
        - 16372125.0f64 * H[4 as libc::c_int as usize]
        - 18918900 as libc::c_int as libc::c_double * H[3 as libc::c_int as usize]
        - 10135125.0f64 * H[2 as libc::c_int as usize]
        + 13783770.0f64 * H[1 as libc::c_int as usize] + 43648605.0f64
        - T[4 as libc::c_int as usize]
            * (3044160.0f64 * H[2 as libc::c_int as usize]
                + 5588352.0f64 * H[1 as libc::c_int as usize] + 4213440.0f64)
        + sgn * T[2 as libc::c_int as usize]
            * (5556600.0f64 * H[4 as libc::c_int as usize]
                + 14817600.0f64 * H[3 as libc::c_int as usize]
                + 20790000.0f64 * H[2 as libc::c_int as usize]
                + 17297280.0f64 * H[1 as libc::c_int as usize] + 5405400.0f64
                + 323072.0f64 * T[4 as libc::c_int as usize]))
        / (4194304.0f64 * T[6 as libc::c_int as usize]);
    C[6 as libc::c_int as usize] = 0.0f64;
    *V0 = C[0 as libc::c_int as usize]
        + (-4.0f64 * C[3 as libc::c_int as usize] / T[1 as libc::c_int as usize]
            + C[4 as libc::c_int as usize]) / V[4 as libc::c_int as usize]
        + (-192.0f64 * C[5 as libc::c_int as usize] / T[3 as libc::c_int as usize]
            + 144.0f64 * C[6 as libc::c_int as usize] / T[2 as libc::c_int as usize])
            / V[8 as libc::c_int as usize]
        + sgn
            * (-C[2 as libc::c_int as usize] / V[2 as libc::c_int as usize]
                + (-24.0f64 * C[4 as libc::c_int as usize] / T[2 as libc::c_int as usize]
                    + 12.0f64 * C[5 as libc::c_int as usize]
                        / T[1 as libc::c_int as usize] - C[6 as libc::c_int as usize])
                    / V[6 as libc::c_int as usize]);
    *V1 = C[1 as libc::c_int as usize] / V[1 as libc::c_int as usize]
        + (8.0f64
            * (C[3 as libc::c_int as usize] / T[2 as libc::c_int as usize]
                - C[4 as libc::c_int as usize] / T[1 as libc::c_int as usize])
            + C[5 as libc::c_int as usize]) / V[5 as libc::c_int as usize]
        + (384.0f64 * C[5 as libc::c_int as usize] / T[4 as libc::c_int as usize]
            - 768.0f64 * C[6 as libc::c_int as usize] / T[3 as libc::c_int as usize])
            / V[9 as libc::c_int as usize]
        + sgn
            * (Cm1 * V[1 as libc::c_int as usize]
                + (2.0f64 * C[2 as libc::c_int as usize] / T[1 as libc::c_int as usize]
                    - C[3 as libc::c_int as usize]) / V[3 as libc::c_int as usize]
                + (48.0f64 * C[4 as libc::c_int as usize] / T[3 as libc::c_int as usize]
                    - 72.0f64 * C[5 as libc::c_int as usize]
                        / T[2 as libc::c_int as usize]
                    + 18.0f64 * C[6 as libc::c_int as usize]
                        / T[1 as libc::c_int as usize]) / V[7 as libc::c_int as usize]);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sf_conicalP_0_e(
    lambda: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x <= -1.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"legendre_con.c\0" as *const u8 as *const libc::c_char,
            771 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x == 1.0f64 {
        (*result).val = 1.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if lambda == 0.0f64 {
        let mut K: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_K: libc::c_int = 0;
        if x < 1.0f64 {
            let th: libc::c_double = acos(x);
            let s: libc::c_double = sin(0.5f64 * th);
            stat_K = gsl_sf_ellint_Kcomp_e(s, 0 as libc::c_int as gsl_mode_t, &mut K);
            (*result).val = 2.0f64 / 3.14159265358979323846f64 * K.val;
            (*result).err = 2.0f64 / 3.14159265358979323846f64 * K.err;
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
            return stat_K;
        } else {
            let xi: libc::c_double = acosh(x);
            let c: libc::c_double = cosh(0.5f64 * xi);
            let t: libc::c_double = tanh(0.5f64 * xi);
            stat_K = gsl_sf_ellint_Kcomp_e(t, 0 as libc::c_int as gsl_mode_t, &mut K);
            (*result).val = 2.0f64 / 3.14159265358979323846f64 / c * K.val;
            (*result).err = 2.0f64 / 3.14159265358979323846f64 / c * K.err;
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
            return stat_K;
        }
    } else if x <= 0.0f64 && lambda < 1000.0f64 || x < 0.1f64 && lambda < 17.0f64
        || x < 0.2f64 && lambda < 5.0f64
    {
        return conicalP_xlt1_hyperg_A(0.0f64, lambda, x, result)
    } else if x <= 0.2f64 && lambda < 17.0f64 || x <= 1.5f64 && lambda < 20.0f64 {
        return gsl_sf_hyperg_2F1_conj_e(
            0.5f64,
            lambda,
            1.0f64,
            (1.0f64 - x) / 2 as libc::c_int as libc::c_double,
            result,
        )
    } else if 1.5f64 < x && lambda < (if x > 20.0f64 { x } else { 20.0f64 }) {
        let mut P: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut lm: libc::c_double = 0.;
        let mut stat_P: libc::c_int = gsl_sf_conicalP_large_x_e(
            0.0f64,
            lambda,
            x,
            &mut P,
            &mut lm,
        );
        let mut stat_e: libc::c_int = gsl_sf_exp_mult_err_e(
            lm,
            2.0f64 * 2.2204460492503131e-16f64 * fabs(lm),
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
    } else {
        let mut V0: libc::c_double = 0.;
        let mut V1: libc::c_double = 0.;
        if x < 1.0f64 {
            let mut th_0: libc::c_double = acos(x);
            let mut sth: libc::c_double = sqrt(1.0f64 - x * x);
            let mut I0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut I1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut stat_I0: libc::c_int = gsl_sf_bessel_I0_scaled_e(
                th_0 * lambda,
                &mut I0,
            );
            let mut stat_I1: libc::c_int = gsl_sf_bessel_I1_scaled_e(
                th_0 * lambda,
                &mut I1,
            );
            let mut stat_I: libc::c_int = if stat_I0 != GSL_SUCCESS as libc::c_int {
                stat_I0
            } else if stat_I1 != GSL_SUCCESS as libc::c_int {
                stat_I1
            } else {
                GSL_SUCCESS as libc::c_int
            };
            let mut stat_V: libc::c_int = conicalP_0_V(
                th_0,
                x / sth,
                lambda,
                -1.0f64,
                &mut V0,
                &mut V1,
            );
            let mut bessterm: libc::c_double = V0 * I0.val + V1 * I1.val;
            let mut besserr: libc::c_double = fabs(V0) * I0.err + fabs(V1) * I1.err;
            let mut arg1: libc::c_double = th_0 * lambda;
            let mut sqts: libc::c_double = sqrt(th_0 / sth);
            let mut stat_e_0: libc::c_int = gsl_sf_exp_mult_err_e(
                arg1,
                4.0f64 * 2.2204460492503131e-16f64 * fabs(arg1),
                sqts * bessterm,
                sqts * besserr,
                result,
            );
            return if stat_e_0 != GSL_SUCCESS as libc::c_int {
                stat_e_0
            } else if stat_V != GSL_SUCCESS as libc::c_int {
                stat_V
            } else if stat_I != GSL_SUCCESS as libc::c_int {
                stat_I
            } else {
                GSL_SUCCESS as libc::c_int
            };
        } else {
            let mut sh: libc::c_double = sqrt(x - 1.0f64) * sqrt(x + 1.0f64);
            let mut xi_0: libc::c_double = log(x + sh);
            let mut J0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut J1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut stat_J0: libc::c_int = gsl_sf_bessel_J0_e(xi_0 * lambda, &mut J0);
            let mut stat_J1: libc::c_int = gsl_sf_bessel_J1_e(xi_0 * lambda, &mut J1);
            let mut stat_J: libc::c_int = if stat_J0 != GSL_SUCCESS as libc::c_int {
                stat_J0
            } else if stat_J1 != GSL_SUCCESS as libc::c_int {
                stat_J1
            } else {
                GSL_SUCCESS as libc::c_int
            };
            let mut stat_V_0: libc::c_int = conicalP_0_V(
                xi_0,
                x / sh,
                lambda,
                1.0f64,
                &mut V0,
                &mut V1,
            );
            let mut bessterm_0: libc::c_double = V0 * J0.val + V1 * J1.val;
            let mut besserr_0: libc::c_double = fabs(V0) * J0.err + fabs(V1) * J1.err;
            let mut pre_val: libc::c_double = sqrt(xi_0 / sh);
            let mut pre_err: libc::c_double = 2.0f64 * fabs(pre_val);
            (*result).val = pre_val * bessterm_0;
            (*result).err = pre_val * besserr_0;
            (*result).err += pre_err * fabs(bessterm_0);
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
            return if stat_V_0 != GSL_SUCCESS as libc::c_int {
                stat_V_0
            } else if stat_J != GSL_SUCCESS as libc::c_int {
                stat_J
            } else {
                GSL_SUCCESS as libc::c_int
            };
        }
    };
}
pub unsafe extern "C" fn gsl_sf_conicalP_1_e(
    lambda: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x <= -1.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"legendre_con.c\0" as *const u8 as *const libc::c_char,
            872 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if lambda == 0.0f64 {
        let mut K: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut E: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_K: libc::c_int = 0;
        let mut stat_E: libc::c_int = 0;
        if x == 1.0f64 {
            (*result).val = 0.0f64;
            (*result).err = 0.0f64;
            return GSL_SUCCESS as libc::c_int;
        } else if x < 1.0f64 {
            if 1.0f64 - x < 1.4901161193847656e-08f64 {
                let mut err_amp: libc::c_double = GSL_MAX_DBL(
                    1.0f64,
                    1.0f64 / (2.2204460492503131e-16f64 + fabs(1.0f64 - x)),
                );
                (*result)
                    .val = 0.25f64 / 1.41421356237309504880f64 * sqrt(1.0f64 - x)
                    * (1.0f64 + 5.0f64 / 16.0f64 * (1.0f64 - x));
                (*result)
                    .err = err_amp * 3.0f64 * 2.2204460492503131e-16f64
                    * fabs((*result).val);
                return GSL_SUCCESS as libc::c_int;
            } else {
                let th: libc::c_double = acos(x);
                let s: libc::c_double = sin(0.5f64 * th);
                let c2: libc::c_double = 1.0f64 - s * s;
                let sth: libc::c_double = sin(th);
                let pre: libc::c_double = 2.0f64 / (3.14159265358979323846f64 * sth);
                stat_K = gsl_sf_ellint_Kcomp_e(
                    s,
                    0 as libc::c_int as gsl_mode_t,
                    &mut K,
                );
                stat_E = gsl_sf_ellint_Ecomp_e(
                    s,
                    0 as libc::c_int as gsl_mode_t,
                    &mut E,
                );
                (*result).val = pre * (E.val - c2 * K.val);
                (*result).err = pre * (E.err + fabs(c2) * K.err);
                (*result).err
                    += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
                return if stat_K != GSL_SUCCESS as libc::c_int {
                    stat_K
                } else if stat_E != GSL_SUCCESS as libc::c_int {
                    stat_E
                } else {
                    GSL_SUCCESS as libc::c_int
                };
            }
        } else if x - 1.0f64 < 1.4901161193847656e-08f64 {
            let mut err_amp_0: libc::c_double = GSL_MAX_DBL(
                1.0f64,
                1.0f64 / (2.2204460492503131e-16f64 + fabs(1.0f64 - x)),
            );
            (*result)
                .val = -0.25f64 / 1.41421356237309504880f64 * sqrt(x - 1.0f64)
                * (1.0f64 - 5.0f64 / 16.0f64 * (x - 1.0f64));
            (*result)
                .err = err_amp_0 * 3.0f64 * 2.2204460492503131e-16f64
                * fabs((*result).val);
            return GSL_SUCCESS as libc::c_int;
        } else {
            let xi: libc::c_double = acosh(x);
            let c: libc::c_double = cosh(0.5f64 * xi);
            let t: libc::c_double = tanh(0.5f64 * xi);
            let sxi: libc::c_double = sinh(xi);
            let pre_0: libc::c_double = 2.0f64 / (3.14159265358979323846f64 * sxi) * c;
            stat_K = gsl_sf_ellint_Kcomp_e(t, 0 as libc::c_int as gsl_mode_t, &mut K);
            stat_E = gsl_sf_ellint_Ecomp_e(t, 0 as libc::c_int as gsl_mode_t, &mut E);
            (*result).val = pre_0 * (E.val - K.val);
            (*result).err = pre_0 * (E.err + K.err);
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
            return if stat_K != GSL_SUCCESS as libc::c_int {
                stat_K
            } else if stat_E != GSL_SUCCESS as libc::c_int {
                stat_E
            } else {
                GSL_SUCCESS as libc::c_int
            };
        }
    } else if x <= 0.0f64 && lambda < 1000.0f64 || x < 0.1f64 && lambda < 17.0f64
        || x < 0.2f64 && lambda < 5.0f64
    {
        return conicalP_xlt1_hyperg_A(1.0f64, lambda, x, result)
    } else if x <= 0.2f64 && lambda < 17.0f64 || x < 1.5f64 && lambda < 20.0f64 {
        let arg: libc::c_double = fabs(x * x - 1.0f64);
        let sgn: libc::c_double = (if 1.0f64 - x >= 0.0f64 {
            1 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_double;
        let pre_1: libc::c_double = 0.5f64 * (lambda * lambda + 0.25f64) * sgn
            * sqrt(arg);
        let mut F: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_F: libc::c_int = gsl_sf_hyperg_2F1_conj_e(
            1.5f64,
            lambda,
            2.0f64,
            (1.0f64 - x) / 2 as libc::c_int as libc::c_double,
            &mut F,
        );
        (*result).val = pre_1 * F.val;
        (*result).err = fabs(pre_1) * F.err;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return stat_F;
    } else if 1.5f64 <= x && lambda < (if x > 20.0f64 { x } else { 20.0f64 }) {
        let mut P: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut lm: libc::c_double = 0.;
        let mut stat_P: libc::c_int = gsl_sf_conicalP_large_x_e(
            1.0f64,
            lambda,
            x,
            &mut P,
            &mut lm,
        );
        let mut stat_e: libc::c_int = gsl_sf_exp_mult_err_e(
            lm,
            2.0f64 * 2.2204460492503131e-16f64 * fabs(lm),
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
    } else {
        let mut V0: libc::c_double = 0.;
        let mut V1: libc::c_double = 0.;
        if x < 1.0f64 {
            let sqrt_1mx: libc::c_double = sqrt(1.0f64 - x);
            let sqrt_1px: libc::c_double = sqrt(1.0f64 + x);
            let th_0: libc::c_double = acos(x);
            let sth_0: libc::c_double = sqrt_1mx * sqrt_1px;
            let mut I0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut I1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut stat_I0: libc::c_int = gsl_sf_bessel_I0_scaled_e(
                th_0 * lambda,
                &mut I0,
            );
            let mut stat_I1: libc::c_int = gsl_sf_bessel_I1_scaled_e(
                th_0 * lambda,
                &mut I1,
            );
            let mut stat_I: libc::c_int = if stat_I0 != GSL_SUCCESS as libc::c_int {
                stat_I0
            } else if stat_I1 != GSL_SUCCESS as libc::c_int {
                stat_I1
            } else {
                GSL_SUCCESS as libc::c_int
            };
            let mut stat_V: libc::c_int = conicalP_1_V(
                th_0,
                x / sth_0,
                lambda,
                -1.0f64,
                &mut V0,
                &mut V1,
            );
            let mut bessterm: libc::c_double = V0 * I0.val + V1 * I1.val;
            let mut besserr: libc::c_double = fabs(V0) * I0.err + fabs(V1) * I1.err
                + 2.0f64 * 2.2204460492503131e-16f64 * fabs(V0 * I0.val)
                + 2.0f64 * 2.2204460492503131e-16f64 * fabs(V1 * I1.val);
            let mut arg1: libc::c_double = th_0 * lambda;
            let mut sqts: libc::c_double = sqrt(th_0 / sth_0);
            let mut stat_e_0: libc::c_int = gsl_sf_exp_mult_err_e(
                arg1,
                2.0f64 * 2.2204460492503131e-16f64 * fabs(arg1),
                sqts * bessterm,
                sqts * besserr,
                result,
            );
            (*result).err *= 1.0f64 / sqrt_1mx;
            return if stat_e_0 != GSL_SUCCESS as libc::c_int {
                stat_e_0
            } else if stat_V != GSL_SUCCESS as libc::c_int {
                stat_V
            } else if stat_I != GSL_SUCCESS as libc::c_int {
                stat_I
            } else {
                GSL_SUCCESS as libc::c_int
            };
        } else {
            let sqrt_xm1: libc::c_double = sqrt(x - 1.0f64);
            let sqrt_xp1: libc::c_double = sqrt(x + 1.0f64);
            let sh: libc::c_double = sqrt_xm1 * sqrt_xp1;
            let xi_0: libc::c_double = log(x + sh);
            let xi_lam: libc::c_double = xi_0 * lambda;
            let mut J0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut J1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let stat_J0: libc::c_int = gsl_sf_bessel_J0_e(xi_lam, &mut J0);
            let stat_J1: libc::c_int = gsl_sf_bessel_J1_e(xi_lam, &mut J1);
            let stat_J: libc::c_int = if stat_J0 != GSL_SUCCESS as libc::c_int {
                stat_J0
            } else if stat_J1 != GSL_SUCCESS as libc::c_int {
                stat_J1
            } else {
                GSL_SUCCESS as libc::c_int
            };
            let stat_V_0: libc::c_int = conicalP_1_V(
                xi_0,
                x / sh,
                lambda,
                1.0f64,
                &mut V0,
                &mut V1,
            );
            let bessterm_0: libc::c_double = V0 * J0.val + V1 * J1.val;
            let besserr_0: libc::c_double = fabs(V0) * J0.err + fabs(V1) * J1.err
                + 512.0f64 * 2.0f64 * 2.2204460492503131e-16f64 * fabs(V0 * J0.val)
                + 512.0f64 * 2.0f64 * 2.2204460492503131e-16f64 * fabs(V1 * J1.val)
                + 2.2204460492503131e-16f64 * fabs(xi_lam * V0 * J1.val)
                + 2.2204460492503131e-16f64 * fabs(xi_lam * V1 * J0.val);
            let pre_2: libc::c_double = sqrt(xi_0 / sh);
            (*result).val = pre_2 * bessterm_0;
            (*result).err = pre_2 * besserr_0 * sqrt_xp1 / sqrt_xm1;
            (*result).err += 4.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
            return if stat_V_0 != GSL_SUCCESS as libc::c_int {
                stat_V_0
            } else if stat_J != GSL_SUCCESS as libc::c_int {
                stat_J
            } else {
                GSL_SUCCESS as libc::c_int
            };
        }
    };
}
pub unsafe extern "C" fn gsl_sf_conicalP_half_e(
    lambda: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x <= -1.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"legendre_con.c\0" as *const u8 as *const libc::c_char,
            1017 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x < 1.0f64 {
        let mut err_amp: libc::c_double = 1.0f64
            + 1.0f64 / (2.2204460492503131e-16f64 + fabs(1.0f64 - fabs(x)));
        let mut ac: libc::c_double = acos(x);
        let mut den: libc::c_double = sqrt(sqrt(1.0f64 - x) * sqrt(1.0f64 + x));
        (*result).val = 0.797884560802865355879892f64 / den * cosh(ac * lambda);
        (*result)
            .err = err_amp * 3.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        (*result).err *= fabs(ac * lambda) + 1.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if x == 1.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut err_amp_0: libc::c_double = 1.0f64
            + 1.0f64 / (2.2204460492503131e-16f64 + fabs(1.0f64 - fabs(x)));
        let mut sq_term: libc::c_double = sqrt(x - 1.0f64) * sqrt(x + 1.0f64);
        let mut ln_term: libc::c_double = log(x + sq_term);
        let mut den_0: libc::c_double = sqrt(sq_term);
        let mut carg_val: libc::c_double = lambda * ln_term;
        let mut carg_err: libc::c_double = 2.0f64 * 2.2204460492503131e-16f64
            * fabs(carg_val);
        let mut cos_result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_cos: libc::c_int = gsl_sf_cos_err_e(
            carg_val,
            carg_err,
            &mut cos_result,
        );
        (*result).val = 0.797884560802865355879892f64 / den_0 * cos_result.val;
        (*result)
            .err = err_amp_0 * 0.797884560802865355879892f64 / den_0 * cos_result.err;
        (*result).err += 4.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return stat_cos;
    };
}
pub unsafe extern "C" fn gsl_sf_conicalP_mhalf_e(
    lambda: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x <= -1.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"legendre_con.c\0" as *const u8 as *const libc::c_char,
            1060 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x < 1.0f64 {
        let mut ac: libc::c_double = acos(x);
        let mut den: libc::c_double = sqrt(sqrt(1.0f64 - x) * sqrt(1.0f64 + x));
        let mut arg: libc::c_double = ac * lambda;
        let mut err_amp: libc::c_double = 1.0f64
            + 1.0f64 / (2.2204460492503131e-16f64 + fabs(1.0f64 - fabs(x)));
        if fabs(arg) < 1.4901161193847656e-08f64 {
            (*result).val = 0.797884560802865355879892f64 / den * ac;
            (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
            (*result).err *= err_amp;
        } else {
            (*result).val = 0.797884560802865355879892f64 / (den * lambda) * sinh(arg);
            (*result)
                .err = 2.2204460492503131e-16f64 * (fabs(arg) + 1.0f64)
                * fabs((*result).val);
            (*result).err *= err_amp;
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        }
        return GSL_SUCCESS as libc::c_int;
    } else if x == 1.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut sq_term: libc::c_double = sqrt(x - 1.0f64) * sqrt(x + 1.0f64);
        let mut ln_term: libc::c_double = log(x + sq_term);
        let mut den_0: libc::c_double = sqrt(sq_term);
        let mut arg_val: libc::c_double = lambda * ln_term;
        let mut arg_err: libc::c_double = 2.0f64 * 2.2204460492503131e-16f64
            * fabs(arg_val);
        if arg_val < 1.4901161193847656e-08f64 {
            (*result).val = 0.797884560802865355879892f64 / den_0 * ln_term;
            (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
            return GSL_SUCCESS as libc::c_int;
        } else {
            let mut sin_result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut stat_sin: libc::c_int = gsl_sf_sin_err_e(
                arg_val,
                arg_err,
                &mut sin_result,
            );
            (*result)
                .val = 0.797884560802865355879892f64 / (den_0 * lambda) * sin_result.val;
            (*result)
                .err = 0.797884560802865355879892f64 / fabs(den_0 * lambda)
                * sin_result.err;
            (*result).err += 3.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
            return stat_sin;
        }
    };
}
pub unsafe extern "C" fn gsl_sf_conicalP_sph_reg_e(
    l: libc::c_int,
    lambda: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x <= -1.0f64 || l < -(1 as libc::c_int) {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"legendre_con.c\0" as *const u8 as *const libc::c_char,
            1117 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if l == -(1 as libc::c_int) {
        return gsl_sf_conicalP_half_e(lambda, x, result)
    } else if l == 0 as libc::c_int {
        return gsl_sf_conicalP_mhalf_e(lambda, x, result)
    } else if x == 1.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if x < 0.0f64 {
        let mut c: libc::c_double = 1.0f64 / sqrt(1.0f64 - x * x);
        let mut r_Pellm1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut r_Pell: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_0: libc::c_int = gsl_sf_conicalP_half_e(lambda, x, &mut r_Pellm1);
        let mut stat_1: libc::c_int = gsl_sf_conicalP_mhalf_e(lambda, x, &mut r_Pell);
        let mut stat_P: libc::c_int = if stat_0 != GSL_SUCCESS as libc::c_int {
            stat_0
        } else if stat_1 != GSL_SUCCESS as libc::c_int {
            stat_1
        } else {
            GSL_SUCCESS as libc::c_int
        };
        let mut Pellm1: libc::c_double = r_Pellm1.val;
        let mut Pell: libc::c_double = r_Pell.val;
        let mut Pellp1: libc::c_double = 0.;
        let mut ell: libc::c_int = 0;
        ell = 0 as libc::c_int;
        while ell < l {
            let mut d: libc::c_double = (ell as libc::c_double + 1.0f64)
                * (ell as libc::c_double + 1.0f64) + lambda * lambda;
            Pellp1 = (Pellm1 - (2.0f64 * ell as libc::c_double + 1.0f64) * c * x * Pell)
                / d;
            Pellm1 = Pell;
            Pell = Pellp1;
            ell += 1;
            ell;
        }
        (*result).val = Pell;
        (*result)
            .err = (0.5f64 * l as libc::c_double + 1.0f64) * 2.2204460492503131e-16f64
            * fabs(Pell);
        (*result).err
            += 2.2204460492503131e-16f64 * l as libc::c_double * fabs((*result).val);
        return stat_P;
    } else if x < 1.0f64 {
        let xi: libc::c_double = x / (sqrt(1.0f64 - x) * sqrt(1.0f64 + x));
        let mut rat: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut Phf: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_CF1: libc::c_int = conicalP_negmu_xlt1_CF1(
            0.5f64,
            l,
            lambda,
            x,
            &mut rat,
        );
        let mut stat_Phf: libc::c_int = gsl_sf_conicalP_half_e(lambda, x, &mut Phf);
        let mut Pellp1_0: libc::c_double = rat.val * 1.4916681462400413e-154f64;
        let mut Pell_0: libc::c_double = 1.4916681462400413e-154f64;
        let mut Pellm1_0: libc::c_double = 0.;
        let mut ell_0: libc::c_int = 0;
        ell_0 = l;
        while ell_0 >= 0 as libc::c_int {
            let mut d_0: libc::c_double = (ell_0 as libc::c_double + 1.0f64)
                * (ell_0 as libc::c_double + 1.0f64) + lambda * lambda;
            Pellm1_0 = (2.0f64 * ell_0 as libc::c_double + 1.0f64) * xi * Pell_0
                + d_0 * Pellp1_0;
            Pellp1_0 = Pell_0;
            Pell_0 = Pellm1_0;
            ell_0 -= 1;
            ell_0;
        }
        (*result).val = 1.4916681462400413e-154f64 * Phf.val / Pell_0;
        (*result).err = 1.4916681462400413e-154f64 * Phf.err / fabs(Pell_0);
        (*result).err
            += fabs(rat.err / rat.val) * (l as libc::c_double + 1.0f64)
                * fabs((*result).val);
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return if stat_Phf != GSL_SUCCESS as libc::c_int {
            stat_Phf
        } else if stat_CF1 != GSL_SUCCESS as libc::c_int {
            stat_CF1
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else if x == 1.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let xi_0: libc::c_double = x / sqrt((x - 1.0f64) * (x + 1.0f64));
        let mut rat_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_CF1_0: libc::c_int = conicalP_negmu_xgt1_CF1(
            0.5f64,
            l,
            lambda,
            x,
            &mut rat_0,
        );
        let mut stat_P_0: libc::c_int = 0;
        let mut Pellp1_1: libc::c_double = rat_0.val * 1.4916681462400413e-154f64;
        let mut Pell_1: libc::c_double = 1.4916681462400413e-154f64;
        let mut Pellm1_1: libc::c_double = 0.;
        let mut ell_1: libc::c_int = 0;
        ell_1 = l;
        while ell_1 >= 0 as libc::c_int {
            let mut d_1: libc::c_double = (ell_1 as libc::c_double + 1.0f64)
                * (ell_1 as libc::c_double + 1.0f64) + lambda * lambda;
            Pellm1_1 = (2.0f64 * ell_1 as libc::c_double + 1.0f64) * xi_0 * Pell_1
                - d_1 * Pellp1_1;
            Pellp1_1 = Pell_1;
            Pell_1 = Pellm1_1;
            ell_1 -= 1;
            ell_1;
        }
        if fabs(Pell_1) > fabs(Pellp1_1) {
            let mut Phf_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            stat_P_0 = gsl_sf_conicalP_half_e(lambda, x, &mut Phf_0);
            (*result).val = 1.4916681462400413e-154f64 * Phf_0.val / Pell_1;
            (*result)
                .err = 2.0f64 * 1.4916681462400413e-154f64 * Phf_0.err / fabs(Pell_1);
            (*result).err
                += 2.0f64 * fabs(rat_0.err / rat_0.val) * (l as libc::c_double + 1.0f64)
                    * fabs((*result).val);
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        } else {
            let mut Pmhf: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            stat_P_0 = gsl_sf_conicalP_mhalf_e(lambda, x, &mut Pmhf);
            (*result).val = 1.4916681462400413e-154f64 * Pmhf.val / Pellp1_1;
            (*result)
                .err = 2.0f64 * 1.4916681462400413e-154f64 * Pmhf.err / fabs(Pellp1_1);
            (*result).err
                += 2.0f64 * fabs(rat_0.err / rat_0.val) * (l as libc::c_double + 1.0f64)
                    * fabs((*result).val);
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        }
        return if stat_P_0 != GSL_SUCCESS as libc::c_int {
            stat_P_0
        } else if stat_CF1_0 != GSL_SUCCESS as libc::c_int {
            stat_CF1_0
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
pub unsafe extern "C" fn gsl_sf_conicalP_cyl_reg_e(
    m: libc::c_int,
    lambda: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x <= -1.0f64 || m < -(1 as libc::c_int) {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"legendre_con.c\0" as *const u8 as *const libc::c_char,
            1233 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if m == -(1 as libc::c_int) {
        return gsl_sf_conicalP_1_e(lambda, x, result)
    } else if m == 0 as libc::c_int {
        return gsl_sf_conicalP_0_e(lambda, x, result)
    } else if x == 1.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if x < 0.0f64 {
        let mut c: libc::c_double = 1.0f64 / sqrt(1.0f64 - x * x);
        let mut r_Pkm1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut r_Pk: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_0: libc::c_int = gsl_sf_conicalP_1_e(lambda, x, &mut r_Pkm1);
        let mut stat_1: libc::c_int = gsl_sf_conicalP_0_e(lambda, x, &mut r_Pk);
        let mut stat_P: libc::c_int = if stat_0 != GSL_SUCCESS as libc::c_int {
            stat_0
        } else if stat_1 != GSL_SUCCESS as libc::c_int {
            stat_1
        } else {
            GSL_SUCCESS as libc::c_int
        };
        let mut Pkm1: libc::c_double = r_Pkm1.val;
        let mut Pk: libc::c_double = r_Pk.val;
        let mut Pkp1: libc::c_double = 0.;
        let mut k: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < m {
            let mut d: libc::c_double = (k as libc::c_double + 0.5f64)
                * (k as libc::c_double + 0.5f64) + lambda * lambda;
            Pkp1 = (Pkm1 - 2.0f64 * k as libc::c_double * c * x * Pk) / d;
            Pkm1 = Pk;
            Pk = Pkp1;
            k += 1;
            k;
        }
        (*result).val = Pk;
        (*result)
            .err = (m as libc::c_double + 2.0f64) * 2.2204460492503131e-16f64 * fabs(Pk);
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return stat_P;
    } else if x < 1.0f64 {
        let xi: libc::c_double = x / (sqrt(1.0f64 - x) * sqrt(1.0f64 + x));
        let mut rat: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut P0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_CF1: libc::c_int = conicalP_negmu_xlt1_CF1(
            0.0f64,
            m,
            lambda,
            x,
            &mut rat,
        );
        let mut stat_P0: libc::c_int = gsl_sf_conicalP_0_e(lambda, x, &mut P0);
        let mut Pkp1_0: libc::c_double = rat.val * 1.4916681462400413e-154f64;
        let mut Pk_0: libc::c_double = 1.4916681462400413e-154f64;
        let mut Pkm1_0: libc::c_double = 0.;
        let mut k_0: libc::c_int = 0;
        k_0 = m;
        while k_0 > 0 as libc::c_int {
            let mut d_0: libc::c_double = (k_0 as libc::c_double + 0.5f64)
                * (k_0 as libc::c_double + 0.5f64) + lambda * lambda;
            Pkm1_0 = 2.0f64 * k_0 as libc::c_double * xi * Pk_0 + d_0 * Pkp1_0;
            Pkp1_0 = Pk_0;
            Pk_0 = Pkm1_0;
            k_0 -= 1;
            k_0;
        }
        (*result).val = 1.4916681462400413e-154f64 * P0.val / Pk_0;
        (*result).err = 2.0f64 * 1.4916681462400413e-154f64 * P0.err / fabs(Pk_0);
        (*result).err
            += 2.0f64 * fabs(rat.err / rat.val) * (m as libc::c_double + 1.0f64)
                * fabs((*result).val);
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return if stat_P0 != GSL_SUCCESS as libc::c_int {
            stat_P0
        } else if stat_CF1 != GSL_SUCCESS as libc::c_int {
            stat_CF1
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else if x == 1.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let xi_0: libc::c_double = x / sqrt((x - 1.0f64) * (x + 1.0f64));
        let mut rat_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_CF1_0: libc::c_int = conicalP_negmu_xgt1_CF1(
            0.0f64,
            m,
            lambda,
            x,
            &mut rat_0,
        );
        let mut stat_P_0: libc::c_int = 0;
        let mut Pkp1_1: libc::c_double = rat_0.val * 1.4916681462400413e-154f64;
        let mut Pk_1: libc::c_double = 1.4916681462400413e-154f64;
        let mut Pkm1_1: libc::c_double = 0.;
        let mut k_1: libc::c_int = 0;
        k_1 = m;
        while k_1 > -(1 as libc::c_int) {
            let mut d_1: libc::c_double = (k_1 as libc::c_double + 0.5f64)
                * (k_1 as libc::c_double + 0.5f64) + lambda * lambda;
            Pkm1_1 = 2.0f64 * k_1 as libc::c_double * xi_0 * Pk_1 - d_1 * Pkp1_1;
            Pkp1_1 = Pk_1;
            Pk_1 = Pkm1_1;
            k_1 -= 1;
            k_1;
        }
        if fabs(Pk_1) > fabs(Pkp1_1) {
            let mut P1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            stat_P_0 = gsl_sf_conicalP_1_e(lambda, x, &mut P1);
            (*result).val = 1.4916681462400413e-154f64 * P1.val / Pk_1;
            (*result).err = 2.0f64 * 1.4916681462400413e-154f64 * P1.err / fabs(Pk_1);
            (*result).err
                += 2.0f64 * fabs(rat_0.err / rat_0.val) * (m as libc::c_double + 2.0f64)
                    * fabs((*result).val);
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        } else {
            let mut P0_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            stat_P_0 = gsl_sf_conicalP_0_e(lambda, x, &mut P0_0);
            (*result).val = 1.4916681462400413e-154f64 * P0_0.val / Pkp1_1;
            (*result)
                .err = 2.0f64 * 1.4916681462400413e-154f64 * P0_0.err / fabs(Pkp1_1);
            (*result).err
                += 2.0f64 * fabs(rat_0.err / rat_0.val) * (m as libc::c_double + 2.0f64)
                    * fabs((*result).val);
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        }
        return if stat_P_0 != GSL_SUCCESS as libc::c_int {
            stat_P_0
        } else if stat_CF1_0 != GSL_SUCCESS as libc::c_int {
            stat_CF1_0
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
pub unsafe extern "C" fn gsl_sf_conicalP_0(
    lambda: libc::c_double,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_conicalP_0_e(lambda, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_conicalP_0_e(lambda, x, &result)\0" as *const u8
                as *const libc::c_char,
            b"legendre_con.c\0" as *const u8 as *const libc::c_char,
            1348 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_conicalP_1(
    lambda: libc::c_double,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_conicalP_1_e(lambda, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_conicalP_1_e(lambda, x, &result)\0" as *const u8
                as *const libc::c_char,
            b"legendre_con.c\0" as *const u8 as *const libc::c_char,
            1353 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_conicalP_half(
    lambda: libc::c_double,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_conicalP_half_e(lambda, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_conicalP_half_e(lambda, x, &result)\0" as *const u8
                as *const libc::c_char,
            b"legendre_con.c\0" as *const u8 as *const libc::c_char,
            1358 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_conicalP_mhalf(
    lambda: libc::c_double,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_conicalP_mhalf_e(lambda, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_conicalP_mhalf_e(lambda, x, &result)\0" as *const u8
                as *const libc::c_char,
            b"legendre_con.c\0" as *const u8 as *const libc::c_char,
            1363 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_conicalP_sph_reg(
    l: libc::c_int,
    lambda: libc::c_double,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_conicalP_sph_reg_e(l, lambda, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_conicalP_sph_reg_e(l, lambda, x, &result)\0" as *const u8
                as *const libc::c_char,
            b"legendre_con.c\0" as *const u8 as *const libc::c_char,
            1368 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_conicalP_cyl_reg(
    m: libc::c_int,
    lambda: libc::c_double,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_conicalP_cyl_reg_e(m, lambda, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_conicalP_cyl_reg_e(m, lambda, x, &result)\0" as *const u8
                as *const libc::c_char,
            b"legendre_con.c\0" as *const u8 as *const libc::c_char,
            1373 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
