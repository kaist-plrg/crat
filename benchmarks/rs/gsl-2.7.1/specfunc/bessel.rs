use ::libc;
extern "C" {
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn hypot(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sf_multiply_err_e(
        x: libc::c_double,
        dx: libc::c_double,
        y: libc::c_double,
        dy: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_exp_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_exp_err_e(
        x: libc::c_double,
        dx: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_lngamma_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_taylorcoeff_e(
        n: libc::c_int,
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_poch_e(
        a: libc::c_double,
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_bessel_asymp_Mnu_e(
        nu: libc::c_double,
        x: libc::c_double,
        result: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_sf_bessel_asymp_thetanu_corr_e(
        nu: libc::c_double,
        x: libc::c_double,
        result: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_sf_bessel_Y_temme(
        nu: libc::c_double,
        x: libc::c_double,
        Y_nu: *mut gsl_sf_result,
        Y_nup1: *mut gsl_sf_result,
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
unsafe extern "C" fn debye_u1(mut tpow: *const libc::c_double) -> libc::c_double {
    return (3.0f64 * *tpow.offset(1 as libc::c_int as isize)
        - 5.0f64 * *tpow.offset(3 as libc::c_int as isize)) / 24.0f64;
}
#[inline]
unsafe extern "C" fn debye_u2(mut tpow: *const libc::c_double) -> libc::c_double {
    return (81.0f64 * *tpow.offset(2 as libc::c_int as isize)
        - 462.0f64 * *tpow.offset(4 as libc::c_int as isize)
        + 385.0f64 * *tpow.offset(6 as libc::c_int as isize)) / 1152.0f64;
}
#[inline]
unsafe extern "C" fn debye_u3(mut tpow: *const libc::c_double) -> libc::c_double {
    return (30375.0f64 * *tpow.offset(3 as libc::c_int as isize)
        - 369603.0f64 * *tpow.offset(5 as libc::c_int as isize)
        + 765765.0f64 * *tpow.offset(7 as libc::c_int as isize)
        - 425425.0f64 * *tpow.offset(9 as libc::c_int as isize)) / 414720.0f64;
}
#[inline]
unsafe extern "C" fn debye_u4(mut tpow: *const libc::c_double) -> libc::c_double {
    return (4465125.0f64 * *tpow.offset(4 as libc::c_int as isize)
        - 94121676.0f64 * *tpow.offset(6 as libc::c_int as isize)
        + 349922430.0f64 * *tpow.offset(8 as libc::c_int as isize)
        - 446185740.0f64 * *tpow.offset(10 as libc::c_int as isize)
        + 185910725.0f64 * *tpow.offset(12 as libc::c_int as isize)) / 39813120.0f64;
}
#[inline]
unsafe extern "C" fn debye_u5(mut tpow: *const libc::c_double) -> libc::c_double {
    return (1519035525.0f64 * *tpow.offset(5 as libc::c_int as isize)
        - 49286948607.0f64 * *tpow.offset(7 as libc::c_int as isize)
        + 284499769554.0f64 * *tpow.offset(9 as libc::c_int as isize)
        - 614135872350.0f64 * *tpow.offset(11 as libc::c_int as isize)
        + 566098157625.0f64 * *tpow.offset(13 as libc::c_int as isize)
        - 188699385875.0f64 * *tpow.offset(15 as libc::c_int as isize))
        / 6688604160.0f64;
}
pub unsafe extern "C" fn gsl_sf_bessel_IJ_taylor_e(
    nu: libc::c_double,
    x: libc::c_double,
    sign: libc::c_int,
    kmax: libc::c_int,
    threshold: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if nu < 0.0f64 || x < 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"bessel.c\0" as *const u8 as *const libc::c_char,
            102 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x == 0.0f64 {
        if nu == 0.0f64 {
            (*result).val = 1.0f64;
            (*result).err = 0.0f64;
        } else {
            (*result).val = 0.0f64;
            (*result).err = 0.0f64;
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut prefactor: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut sum: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_pre: libc::c_int = 0;
        let mut stat_sum: libc::c_int = 0;
        let mut stat_mul: libc::c_int = 0;
        if nu == 0.0f64 {
            prefactor.val = 1.0f64;
            prefactor.err = 0.0f64;
            stat_pre = GSL_SUCCESS as libc::c_int;
        } else if nu < (2147483647 as libc::c_int - 1 as libc::c_int) as libc::c_double {
            let N: libc::c_int = floor(nu + 0.5f64) as libc::c_int;
            let f: libc::c_double = nu - N as libc::c_double;
            let mut poch_factor: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut tc_factor: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let stat_poch: libc::c_int = gsl_sf_poch_e(
                N as libc::c_double + 1.0f64,
                f,
                &mut poch_factor,
            );
            let stat_tc: libc::c_int = gsl_sf_taylorcoeff_e(
                N,
                0.5f64 * x,
                &mut tc_factor,
            );
            let p: libc::c_double = pow(0.5f64 * x, f);
            prefactor.val = tc_factor.val * p / poch_factor.val;
            prefactor.err = tc_factor.err * p / poch_factor.val;
            prefactor.err += fabs(prefactor.val) / poch_factor.val * poch_factor.err;
            prefactor.err += 2.0f64 * 2.2204460492503131e-16f64 * fabs(prefactor.val);
            stat_pre = if stat_tc != GSL_SUCCESS as libc::c_int {
                stat_tc
            } else if stat_poch != GSL_SUCCESS as libc::c_int {
                stat_poch
            } else {
                GSL_SUCCESS as libc::c_int
            };
        } else {
            let mut lg: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let stat_lg: libc::c_int = gsl_sf_lngamma_e(nu + 1.0f64, &mut lg);
            let term1: libc::c_double = nu * log(0.5f64 * x);
            let term2: libc::c_double = lg.val;
            let ln_pre: libc::c_double = term1 - term2;
            let ln_pre_err: libc::c_double = 2.2204460492503131e-16f64
                * (fabs(term1) + fabs(term2)) + lg.err;
            let stat_ex: libc::c_int = gsl_sf_exp_err_e(
                ln_pre,
                ln_pre_err,
                &mut prefactor,
            );
            stat_pre = if stat_ex != GSL_SUCCESS as libc::c_int {
                stat_ex
            } else if stat_lg != GSL_SUCCESS as libc::c_int {
                stat_lg
            } else {
                GSL_SUCCESS as libc::c_int
            };
        }
        let y: libc::c_double = sign as libc::c_double * 0.25f64 * x * x;
        let mut sumk: libc::c_double = 1.0f64;
        let mut term: libc::c_double = 1.0f64;
        let mut k: libc::c_int = 0;
        k = 1 as libc::c_int;
        while k <= kmax {
            term *= y / ((nu + k as libc::c_double) * k as libc::c_double);
            sumk += term;
            if fabs(term / sumk) < threshold {
                break;
            }
            k += 1;
            k;
        }
        sum.val = sumk;
        sum.err = threshold * fabs(sumk);
        stat_sum = if k >= kmax {
            GSL_EMAXITER as libc::c_int
        } else {
            GSL_SUCCESS as libc::c_int
        };
        stat_mul = gsl_sf_multiply_err_e(
            prefactor.val,
            prefactor.err,
            sum.val,
            sum.err,
            result,
        );
        return if stat_mul != GSL_SUCCESS as libc::c_int {
            stat_mul
        } else if stat_pre != GSL_SUCCESS as libc::c_int {
            stat_pre
        } else if stat_sum != GSL_SUCCESS as libc::c_int {
            stat_sum
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_Jnu_asympx_e(
    nu: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut mu: libc::c_double = 4.0f64 * nu * nu;
    let mut chi: libc::c_double = x
        - (0.5f64 * nu + 0.25f64) * 3.14159265358979323846f64;
    let mut P: libc::c_double = 0.0f64;
    let mut Q: libc::c_double = 0.0f64;
    let mut k: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut t: libc::c_double = 1 as libc::c_int as libc::c_double;
    let mut convP: libc::c_int = 0;
    let mut convQ: libc::c_int = 0;
    loop {
        t
            *= if k == 0 as libc::c_int as libc::c_double {
                1 as libc::c_int as libc::c_double
            } else {
                -(mu
                    - (2 as libc::c_int as libc::c_double * k
                        - 1 as libc::c_int as libc::c_double)
                        * (2 as libc::c_int as libc::c_double * k
                            - 1 as libc::c_int as libc::c_double))
                    / (k * (8 as libc::c_int as libc::c_double * x))
            };
        convP = (fabs(t) < 2.2204460492503131e-16f64 * fabs(P)) as libc::c_int;
        P += t;
        k += 1.;
        k;
        t
            *= (mu
                - (2 as libc::c_int as libc::c_double * k
                    - 1 as libc::c_int as libc::c_double)
                    * (2 as libc::c_int as libc::c_double * k
                        - 1 as libc::c_int as libc::c_double))
                / (k * (8 as libc::c_int as libc::c_double * x));
        convQ = (fabs(t) < 2.2204460492503131e-16f64 * fabs(Q)) as libc::c_int;
        Q += t;
        if convP != 0 && convQ != 0 && k > nu / 2 as libc::c_int as libc::c_double {
            break;
        }
        k += 1.;
        k;
        if !(k < 1000 as libc::c_int as libc::c_double) {
            break;
        }
    }
    let mut pre: libc::c_double = sqrt(2.0f64 / (3.14159265358979323846f64 * x));
    let mut c: libc::c_double = cos(chi);
    let mut s: libc::c_double = sin(chi);
    (*result).val = pre * (c * P - s * Q);
    (*result)
        .err = pre * 2.2204460492503131e-16f64 * (fabs(c * P) + fabs(s * Q) + fabs(t))
        * (1 as libc::c_int as libc::c_double + fabs(x));
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sf_bessel_Ynu_asympx_e(
    nu: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut ampl: libc::c_double = 0.;
    let mut theta: libc::c_double = 0.;
    let mut alpha: libc::c_double = x;
    let mut beta: libc::c_double = -0.5f64 * nu * 3.14159265358979323846f64;
    let mut stat_a: libc::c_int = gsl_sf_bessel_asymp_Mnu_e(nu, x, &mut ampl);
    let mut stat_t: libc::c_int = gsl_sf_bessel_asymp_thetanu_corr_e(nu, x, &mut theta);
    let mut sin_alpha: libc::c_double = sin(alpha);
    let mut cos_alpha: libc::c_double = cos(alpha);
    let mut sin_chi: libc::c_double = sin(beta + theta);
    let mut cos_chi: libc::c_double = cos(beta + theta);
    let mut sin_term: libc::c_double = sin_alpha * cos_chi + sin_chi * cos_alpha;
    let mut sin_term_mag: libc::c_double = fabs(sin_alpha * cos_chi)
        + fabs(sin_chi * cos_alpha);
    (*result).val = ampl * sin_term;
    (*result).err = fabs(ampl) * 2.2204460492503131e-16f64 * sin_term_mag;
    (*result).err += fabs((*result).val) * 2.0f64 * 2.2204460492503131e-16f64;
    if fabs(alpha) > 1.0f64 / 2.2204460492503131e-16f64 {
        (*result).err *= 0.5f64 * fabs(alpha);
    } else if fabs(alpha) > 1.0f64 / 1.4901161193847656e-08f64 {
        (*result).err *= 256.0f64 * fabs(alpha) * 1.4901161193847656e-08f64;
    }
    return if stat_t != GSL_SUCCESS as libc::c_int {
        stat_t
    } else if stat_a != GSL_SUCCESS as libc::c_int {
        stat_a
    } else {
        GSL_SUCCESS as libc::c_int
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_Inu_scaled_asympx_e(
    nu: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut mu: libc::c_double = 4.0f64 * nu * nu;
    let mut mum1: libc::c_double = mu - 1.0f64;
    let mut mum9: libc::c_double = mu - 9.0f64;
    let mut pre: libc::c_double = 1.0f64 / sqrt(2.0f64 * 3.14159265358979323846f64 * x);
    let mut r: libc::c_double = mu / x;
    (*result)
        .val = pre * (1.0f64 - mum1 / (8.0f64 * x) + mum1 * mum9 / (128.0f64 * x * x));
    (*result)
        .err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val)
        + pre * fabs(0.1f64 * r * r * r);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sf_bessel_Knu_scaled_asympx_e(
    nu: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut mu: libc::c_double = 4.0f64 * nu * nu;
    let mut mum1: libc::c_double = mu - 1.0f64;
    let mut mum9: libc::c_double = mu - 9.0f64;
    let mut pre: libc::c_double = sqrt(3.14159265358979323846f64 / (2.0f64 * x));
    let mut r: libc::c_double = nu / x;
    (*result)
        .val = pre * (1.0f64 + mum1 / (8.0f64 * x) + mum1 * mum9 / (128.0f64 * x * x));
    (*result)
        .err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val)
        + pre * fabs(0.1f64 * r * r * r);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sf_bessel_Inu_scaled_asymp_unif_e(
    nu: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut z: libc::c_double = x / nu;
    let mut root_term: libc::c_double = hypot(1.0f64, z);
    let mut pre: libc::c_double = 1.0f64
        / sqrt(2.0f64 * 3.14159265358979323846f64 * nu * root_term);
    let mut eta: libc::c_double = root_term + log(z / (1.0f64 + root_term));
    let mut ex_arg: libc::c_double = if z < 1.0f64 / 6.0554544523933429e-06f64 {
        nu * (-z + eta)
    } else {
        -0.5f64 * nu / z * (1.0f64 - 1.0f64 / (12.0f64 * z * z))
    };
    let mut ex_result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut stat_ex: libc::c_int = gsl_sf_exp_e(ex_arg, &mut ex_result);
    if stat_ex == GSL_SUCCESS as libc::c_int {
        let mut t: libc::c_double = 1.0f64 / root_term;
        let mut sum: libc::c_double = 0.;
        let mut tpow: [libc::c_double; 16] = [0.; 16];
        tpow[0 as libc::c_int as usize] = 1.0f64;
        i = 1 as libc::c_int;
        while i < 16 as libc::c_int {
            tpow[i as usize] = t * tpow[(i - 1 as libc::c_int) as usize];
            i += 1;
            i;
        }
        sum = 1.0f64 + debye_u1(tpow.as_mut_ptr()) / nu
            + debye_u2(tpow.as_mut_ptr()) / (nu * nu)
            + debye_u3(tpow.as_mut_ptr()) / (nu * nu * nu)
            + debye_u4(tpow.as_mut_ptr()) / (nu * nu * nu * nu)
            + debye_u5(tpow.as_mut_ptr()) / (nu * nu * nu * nu * nu);
        (*result).val = pre * ex_result.val * sum;
        (*result).err = pre * ex_result.val / (nu * nu * nu * nu * nu * nu);
        (*result).err += pre * ex_result.err * fabs(sum);
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return stat_ex;
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_Knu_scaled_asymp_unif_e(
    nu: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut z: libc::c_double = x / nu;
    let mut root_term: libc::c_double = hypot(1.0f64, z);
    let mut pre: libc::c_double = sqrt(
        3.14159265358979323846f64 / (2.0f64 * nu * root_term),
    );
    let mut eta: libc::c_double = root_term + log(z / (1.0f64 + root_term));
    let mut ex_arg: libc::c_double = if z < 1.0f64 / 6.0554544523933429e-06f64 {
        nu * (z - eta)
    } else {
        0.5f64 * nu / z * (1.0f64 + 1.0f64 / (12.0f64 * z * z))
    };
    let mut ex_result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut stat_ex: libc::c_int = gsl_sf_exp_e(ex_arg, &mut ex_result);
    if stat_ex == GSL_SUCCESS as libc::c_int {
        let mut t: libc::c_double = 1.0f64 / root_term;
        let mut sum: libc::c_double = 0.;
        let mut tpow: [libc::c_double; 16] = [0.; 16];
        tpow[0 as libc::c_int as usize] = 1.0f64;
        i = 1 as libc::c_int;
        while i < 16 as libc::c_int {
            tpow[i as usize] = t * tpow[(i - 1 as libc::c_int) as usize];
            i += 1;
            i;
        }
        sum = 1.0f64 - debye_u1(tpow.as_mut_ptr()) / nu
            + debye_u2(tpow.as_mut_ptr()) / (nu * nu)
            - debye_u3(tpow.as_mut_ptr()) / (nu * nu * nu)
            + debye_u4(tpow.as_mut_ptr()) / (nu * nu * nu * nu)
            - debye_u5(tpow.as_mut_ptr()) / (nu * nu * nu * nu * nu);
        (*result).val = pre * ex_result.val * sum;
        (*result).err = pre * ex_result.err * fabs(sum);
        (*result).err += pre * ex_result.val / (nu * nu * nu * nu * nu * nu);
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return stat_ex;
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_JY_mu_restricted(
    mu: libc::c_double,
    x: libc::c_double,
    mut Jmu: *mut gsl_sf_result,
    mut Jmup1: *mut gsl_sf_result,
    mut Ymu: *mut gsl_sf_result,
    mut Ymup1: *mut gsl_sf_result,
) -> libc::c_int {
    if x < 0.0f64 || fabs(mu) > 0.5f64 {
        (*Jmu).val = 0.0f64;
        (*Jmu).err = 0.0f64;
        (*Jmup1).val = 0.0f64;
        (*Jmup1).err = 0.0f64;
        (*Ymu).val = 0.0f64;
        (*Ymu).err = 0.0f64;
        (*Ymup1).val = 0.0f64;
        (*Ymup1).err = 0.0f64;
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"bessel.c\0" as *const u8 as *const libc::c_char,
            447 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x == 0.0f64 {
        if mu == 0.0f64 {
            (*Jmu).val = 1.0f64;
            (*Jmu).err = 0.0f64;
        } else {
            (*Jmu).val = 0.0f64;
            (*Jmu).err = 0.0f64;
        }
        (*Jmup1).val = 0.0f64;
        (*Jmup1).err = 0.0f64;
        (*Ymu).val = 0.0f64;
        (*Ymu).err = 0.0f64;
        (*Ymup1).val = 0.0f64;
        (*Ymup1).err = 0.0f64;
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"bessel.c\0" as *const u8 as *const libc::c_char,
            464 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        let mut stat_Y: libc::c_int = 0;
        let mut stat_J: libc::c_int = 0;
        if x < 2.0f64 {
            let mut Jmup2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut stat_J1: libc::c_int = gsl_sf_bessel_IJ_taylor_e(
                mu + 1.0f64,
                x,
                -(1 as libc::c_int),
                100 as libc::c_int,
                2.2204460492503131e-16f64,
                Jmup1,
            );
            let mut stat_J2: libc::c_int = gsl_sf_bessel_IJ_taylor_e(
                mu + 2.0f64,
                x,
                -(1 as libc::c_int),
                100 as libc::c_int,
                2.2204460492503131e-16f64,
                &mut Jmup2,
            );
            let mut c: libc::c_double = 2.0f64 * (mu + 1.0f64) / x;
            (*Jmu).val = c * (*Jmup1).val - Jmup2.val;
            (*Jmu).err = c * (*Jmup1).err + Jmup2.err;
            (*Jmu).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*Jmu).val);
            stat_J = if stat_J1 != GSL_SUCCESS as libc::c_int {
                stat_J1
            } else if stat_J2 != GSL_SUCCESS as libc::c_int {
                stat_J2
            } else {
                GSL_SUCCESS as libc::c_int
            };
            stat_Y = gsl_sf_bessel_Y_temme(mu, x, Ymu, Ymup1);
            return if stat_J != GSL_SUCCESS as libc::c_int {
                stat_J
            } else if stat_Y != GSL_SUCCESS as libc::c_int {
                stat_Y
            } else {
                GSL_SUCCESS as libc::c_int
            };
        } else if x < 1000.0f64 {
            let mut P: libc::c_double = 0.;
            let mut Q: libc::c_double = 0.;
            let mut J_ratio: libc::c_double = 0.;
            let mut J_sgn: libc::c_double = 0.;
            let stat_CF1: libc::c_int = gsl_sf_bessel_J_CF1(
                mu,
                x,
                &mut J_ratio,
                &mut J_sgn,
            );
            let stat_CF2: libc::c_int = gsl_sf_bessel_JY_steed_CF2(
                mu,
                x,
                &mut P,
                &mut Q,
            );
            let mut Jprime_J_ratio: libc::c_double = mu / x - J_ratio;
            let mut gamma: libc::c_double = (P - Jprime_J_ratio) / Q;
            (*Jmu)
                .val = J_sgn
                * sqrt(
                    2.0f64 / (3.14159265358979323846f64 * x)
                        / (Q + gamma * (P - Jprime_J_ratio)),
                );
            (*Jmu).err = 4.0f64 * 2.2204460492503131e-16f64 * fabs((*Jmu).val);
            (*Jmup1).val = J_ratio * (*Jmu).val;
            (*Jmup1).err = fabs(J_ratio) * (*Jmu).err;
            (*Ymu).val = gamma * (*Jmu).val;
            (*Ymu).err = fabs(gamma) * (*Jmu).err;
            (*Ymup1).val = (*Ymu).val * (mu / x - P - Q / gamma);
            (*Ymup1)
                .err = (*Ymu).err * fabs(mu / x - P - Q / gamma)
                + 4.0f64 * 2.2204460492503131e-16f64 * fabs((*Ymup1).val);
            return if stat_CF1 != GSL_SUCCESS as libc::c_int {
                stat_CF1
            } else if stat_CF2 != GSL_SUCCESS as libc::c_int {
                stat_CF2
            } else {
                GSL_SUCCESS as libc::c_int
            };
        } else {
            let stat_J0: libc::c_int = gsl_sf_bessel_Jnu_asympx_e(mu, x, Jmu);
            let stat_J1_0: libc::c_int = gsl_sf_bessel_Jnu_asympx_e(
                mu + 1.0f64,
                x,
                Jmup1,
            );
            let stat_Y0: libc::c_int = gsl_sf_bessel_Ynu_asympx_e(mu, x, Ymu);
            let stat_Y1: libc::c_int = gsl_sf_bessel_Ynu_asympx_e(mu + 1.0f64, x, Ymup1);
            stat_J = if stat_J0 != GSL_SUCCESS as libc::c_int {
                stat_J0
            } else if stat_J1_0 != GSL_SUCCESS as libc::c_int {
                stat_J1_0
            } else {
                GSL_SUCCESS as libc::c_int
            };
            stat_Y = if stat_Y0 != GSL_SUCCESS as libc::c_int {
                stat_Y0
            } else if stat_Y1 != GSL_SUCCESS as libc::c_int {
                stat_Y1
            } else {
                GSL_SUCCESS as libc::c_int
            };
            return if stat_J != GSL_SUCCESS as libc::c_int {
                stat_J
            } else if stat_Y != GSL_SUCCESS as libc::c_int {
                stat_Y
            } else {
                GSL_SUCCESS as libc::c_int
            };
        }
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_J_CF1(
    nu: libc::c_double,
    x: libc::c_double,
    mut ratio: *mut libc::c_double,
    mut sgn: *mut libc::c_double,
) -> libc::c_int {
    let RECUR_BIG: libc::c_double = 1.3407807929942596e+154f64;
    let RECUR_SMALL: libc::c_double = 1.4916681462400413e-154f64;
    let maxiter: libc::c_int = 10000 as libc::c_int;
    let mut n: libc::c_int = 1 as libc::c_int;
    let mut Anm2: libc::c_double = 1.0f64;
    let mut Bnm2: libc::c_double = 0.0f64;
    let mut Anm1: libc::c_double = 0.0f64;
    let mut Bnm1: libc::c_double = 1.0f64;
    let mut a1: libc::c_double = x / (2.0f64 * (nu + 1.0f64));
    let mut An: libc::c_double = Anm1 + a1 * Anm2;
    let mut Bn: libc::c_double = Bnm1 + a1 * Bnm2;
    let mut an: libc::c_double = 0.;
    let mut fn_0: libc::c_double = An / Bn;
    let mut dn: libc::c_double = a1;
    let mut s: libc::c_double = 1.0f64;
    while n < maxiter {
        let mut old_fn: libc::c_double = 0.;
        let mut del: libc::c_double = 0.;
        n += 1;
        n;
        Anm2 = Anm1;
        Bnm2 = Bnm1;
        Anm1 = An;
        Bnm1 = Bn;
        an = -x * x
            / (4.0f64 * (nu + n as libc::c_double - 1.0f64)
                * (nu + n as libc::c_double));
        An = Anm1 + an * Anm2;
        Bn = Bnm1 + an * Bnm2;
        if fabs(An) > RECUR_BIG || fabs(Bn) > RECUR_BIG {
            An /= RECUR_BIG;
            Bn /= RECUR_BIG;
            Anm1 /= RECUR_BIG;
            Bnm1 /= RECUR_BIG;
            Anm2 /= RECUR_BIG;
        } else if fabs(An) < RECUR_SMALL || fabs(Bn) < RECUR_SMALL {
            An /= RECUR_SMALL;
            Bn /= RECUR_SMALL;
            Anm1 /= RECUR_SMALL;
            Bnm1 /= RECUR_SMALL;
            Anm2 /= RECUR_SMALL;
            Bnm2 /= RECUR_SMALL;
        }
        old_fn = fn_0;
        fn_0 = An / Bn;
        del = old_fn / fn_0;
        dn = 1.0f64 / (2.0f64 * (nu + n as libc::c_double) / x - dn);
        if dn < 0.0f64 {
            s = -s;
        }
        if fabs(del - 1.0f64) < 2.0f64 * 2.2204460492503131e-16f64 {
            break;
        }
    }
    *ratio = fn_0;
    *sgn = s;
    if n >= maxiter {
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"bessel.c\0" as *const u8 as *const libc::c_char,
            584 as libc::c_int,
            GSL_EMAXITER as libc::c_int,
        );
        return GSL_EMAXITER as libc::c_int;
    } else {
        return GSL_SUCCESS as libc::c_int
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_I_CF1_ser(
    nu: libc::c_double,
    x: libc::c_double,
    mut ratio: *mut libc::c_double,
) -> libc::c_int {
    let maxk: libc::c_int = 20000 as libc::c_int;
    let mut tk: libc::c_double = 1.0f64;
    let mut sum: libc::c_double = 1.0f64;
    let mut rhok: libc::c_double = 0.0f64;
    let mut k: libc::c_int = 0;
    k = 1 as libc::c_int;
    while k < maxk {
        let mut ak: libc::c_double = 0.25f64 * (x / (nu + k as libc::c_double)) * x
            / (nu + k as libc::c_double + 1.0f64);
        rhok = -ak * (1.0f64 + rhok) / (1.0f64 + ak * (1.0f64 + rhok));
        tk *= rhok;
        sum += tk;
        if fabs(tk / sum) < 2.2204460492503131e-16f64 {
            break;
        }
        k += 1;
        k;
    }
    *ratio = x / (2.0f64 * (nu + 1.0f64)) * sum;
    if k == maxk {
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"bessel.c\0" as *const u8 as *const libc::c_char,
            656 as libc::c_int,
            GSL_EMAXITER as libc::c_int,
        );
        return GSL_EMAXITER as libc::c_int;
    } else {
        return GSL_SUCCESS as libc::c_int
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_JY_steed_CF2(
    nu: libc::c_double,
    x: libc::c_double,
    mut P: *mut libc::c_double,
    mut Q: *mut libc::c_double,
) -> libc::c_int {
    let max_iter: libc::c_int = 10000 as libc::c_int;
    let SMALL: libc::c_double = 1.0e-100f64;
    let mut i: libc::c_int = 1 as libc::c_int;
    let mut x_inv: libc::c_double = 1.0f64 / x;
    let mut a: libc::c_double = 0.25f64 - nu * nu;
    let mut p: libc::c_double = -0.5f64 * x_inv;
    let mut q: libc::c_double = 1.0f64;
    let mut br: libc::c_double = 2.0f64 * x;
    let mut bi: libc::c_double = 2.0f64;
    let mut fact: libc::c_double = a * x_inv / (p * p + q * q);
    let mut cr: libc::c_double = br + q * fact;
    let mut ci: libc::c_double = bi + p * fact;
    let mut den: libc::c_double = br * br + bi * bi;
    let mut dr: libc::c_double = br / den;
    let mut di: libc::c_double = -bi / den;
    let mut dlr: libc::c_double = cr * dr - ci * di;
    let mut dli: libc::c_double = cr * di + ci * dr;
    let mut temp: libc::c_double = p * dlr - q * dli;
    q = p * dli + q * dlr;
    p = temp;
    i = 2 as libc::c_int;
    while i <= max_iter {
        a += (2 as libc::c_int * (i - 1 as libc::c_int)) as libc::c_double;
        bi += 2.0f64;
        dr = a * dr + br;
        di = a * di + bi;
        if fabs(dr) + fabs(di) < SMALL {
            dr = SMALL;
        }
        fact = a / (cr * cr + ci * ci);
        cr = br + cr * fact;
        ci = bi - ci * fact;
        if fabs(cr) + fabs(ci) < SMALL {
            cr = SMALL;
        }
        den = dr * dr + di * di;
        dr /= den;
        di /= -den;
        dlr = cr * dr - ci * di;
        dli = cr * di + ci * dr;
        temp = p * dlr - q * dli;
        q = p * dli + q * dlr;
        p = temp;
        if fabs(dlr - 1.0f64) + fabs(dli) < 2.2204460492503131e-16f64 {
            break;
        }
        i += 1;
        i;
    }
    *P = p;
    *Q = q;
    if i == max_iter {
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"bessel.c\0" as *const u8 as *const libc::c_char,
            713 as libc::c_int,
            GSL_EMAXITER as libc::c_int,
        );
        return GSL_EMAXITER as libc::c_int;
    } else {
        return GSL_SUCCESS as libc::c_int
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_K_scaled_steed_temme_CF2(
    nu: libc::c_double,
    x: libc::c_double,
    mut K_nu: *mut libc::c_double,
    mut K_nup1: *mut libc::c_double,
    mut Kp_nu: *mut libc::c_double,
) -> libc::c_int {
    let maxiter: libc::c_int = 10000 as libc::c_int;
    let mut i: libc::c_int = 1 as libc::c_int;
    let mut bi: libc::c_double = 2.0f64 * (1.0f64 + x);
    let mut di: libc::c_double = 1.0f64 / bi;
    let mut delhi: libc::c_double = di;
    let mut hi: libc::c_double = di;
    let mut qi: libc::c_double = 0.0f64;
    let mut qip1: libc::c_double = 1.0f64;
    let mut ai: libc::c_double = -(0.25f64 - nu * nu);
    let mut a1: libc::c_double = ai;
    let mut ci: libc::c_double = -ai;
    let mut Qi: libc::c_double = -ai;
    let mut s: libc::c_double = 1.0f64 + Qi * delhi;
    i = 2 as libc::c_int;
    while i <= maxiter {
        let mut dels: libc::c_double = 0.;
        let mut tmp: libc::c_double = 0.;
        ai -= 2.0f64 * (i - 1 as libc::c_int) as libc::c_double;
        ci = -ai * ci / i as libc::c_double;
        tmp = (qi - bi * qip1) / ai;
        qi = qip1;
        qip1 = tmp;
        Qi += ci * qip1;
        bi += 2.0f64;
        di = 1.0f64 / (bi + ai * di);
        delhi = (bi * di - 1.0f64) * delhi;
        hi += delhi;
        dels = Qi * delhi;
        s += dels;
        if fabs(dels / s) < 2.2204460492503131e-16f64 {
            break;
        }
        i += 1;
        i;
    }
    hi *= -a1;
    *K_nu = sqrt(3.14159265358979323846f64 / (2.0f64 * x)) / s;
    *K_nup1 = *K_nu * (nu + x + 0.5f64 - hi) / x;
    *Kp_nu = -*K_nup1 + nu / x * *K_nu;
    if i == maxiter {
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"bessel.c\0" as *const u8 as *const libc::c_char,
            772 as libc::c_int,
            GSL_EMAXITER as libc::c_int,
        );
        return GSL_EMAXITER as libc::c_int;
    } else {
        return GSL_SUCCESS as libc::c_int
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_cos_pi4_e(
    mut y: libc::c_double,
    mut eps: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let sy: libc::c_double = sin(y);
    let cy: libc::c_double = cos(y);
    let s: libc::c_double = sy + cy;
    let d: libc::c_double = sy - cy;
    let abs_sum: libc::c_double = fabs(cy) + fabs(sy);
    let mut seps: libc::c_double = 0.;
    let mut ceps: libc::c_double = 0.;
    if fabs(eps) < 7.4009597974140505e-04f64 {
        let e2: libc::c_double = eps * eps;
        seps = eps * (1.0f64 - e2 / 6.0f64 * (1.0f64 - e2 / 20.0f64));
        ceps = 1.0f64 - e2 / 2.0f64 * (1.0f64 - e2 / 12.0f64);
    } else {
        seps = sin(eps);
        ceps = cos(eps);
    }
    (*result).val = (ceps * s - seps * d) / 1.41421356237309504880f64;
    (*result)
        .err = 2.0f64 * 2.2204460492503131e-16f64 * (fabs(ceps) + fabs(seps)) * abs_sum
        / 1.41421356237309504880f64;
    if y > 1.0f64 / 2.2204460492503131e-16f64 {
        (*result).err *= 0.5f64 * y;
    } else if y > 1.0f64 / 1.4901161193847656e-08f64 {
        (*result).err *= 256.0f64 * y * 1.4901161193847656e-08f64;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sf_bessel_sin_pi4_e(
    mut y: libc::c_double,
    mut eps: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let sy: libc::c_double = sin(y);
    let cy: libc::c_double = cos(y);
    let s: libc::c_double = sy + cy;
    let d: libc::c_double = sy - cy;
    let abs_sum: libc::c_double = fabs(cy) + fabs(sy);
    let mut seps: libc::c_double = 0.;
    let mut ceps: libc::c_double = 0.;
    if fabs(eps) < 7.4009597974140505e-04f64 {
        let e2: libc::c_double = eps * eps;
        seps = eps * (1.0f64 - e2 / 6.0f64 * (1.0f64 - e2 / 20.0f64));
        ceps = 1.0f64 - e2 / 2.0f64 * (1.0f64 - e2 / 12.0f64);
    } else {
        seps = sin(eps);
        ceps = cos(eps);
    }
    (*result).val = (ceps * d + seps * s) / 1.41421356237309504880f64;
    (*result)
        .err = 2.0f64 * 2.2204460492503131e-16f64 * (fabs(ceps) + fabs(seps)) * abs_sum
        / 1.41421356237309504880f64;
    if y > 1.0f64 / 2.2204460492503131e-16f64 {
        (*result).err *= 0.5f64 * y;
    } else if y > 1.0f64 / 1.4901161193847656e-08f64 {
        (*result).err *= 256.0f64 * y * 1.4901161193847656e-08f64;
    }
    return GSL_SUCCESS as libc::c_int;
}
