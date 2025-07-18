use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn log1p(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_cdf_ugaussian_Pinv(P: libc::c_double) -> libc::c_double;
    fn gsl_cdf_ugaussian_Qinv(Q: libc::c_double) -> libc::c_double;
    fn gsl_cdf_gamma_P(
        x: libc::c_double,
        a: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_double;
    fn gsl_cdf_gamma_Q(
        x: libc::c_double,
        a: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_ran_gamma_pdf(
        x: libc::c_double,
        a: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_double;
    fn gsl_sf_lngamma(x: libc::c_double) -> libc::c_double;
}
pub const GSL_EFAILED: C2RustUnnamed = 5;
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
pub const GSL_EINVAL: C2RustUnnamed = 4;
pub const GSL_EFAULT: C2RustUnnamed = 3;
pub const GSL_ERANGE: C2RustUnnamed = 2;
pub const GSL_EDOM: C2RustUnnamed = 1;
pub const GSL_CONTINUE: C2RustUnnamed = -2;
pub const GSL_FAILURE: C2RustUnnamed = -1;
pub const GSL_SUCCESS: C2RustUnnamed = 0;
pub unsafe extern "C" fn gsl_cdf_gamma_Pinv(
    P: libc::c_double,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    if P == 1.0f64 {
        return ::std::f32::INFINITY as libc::c_double
    } else if P == 0.0f64 {
        return 0.0f64
    }
    if P < 0.05f64 {
        let mut x0: libc::c_double = exp((gsl_sf_lngamma(a) + log(P)) / a);
        x = x0;
    } else if P > 0.95f64 {
        let mut x0_0: libc::c_double = -log1p(-P) + gsl_sf_lngamma(a);
        x = x0_0;
    } else {
        let mut xg: libc::c_double = gsl_cdf_ugaussian_Pinv(P);
        let mut x0_1: libc::c_double = if xg < -0.5f64 * sqrt(a) {
            a
        } else {
            sqrt(a) * xg + a
        };
        x = x0_1;
    }
    let mut lambda: libc::c_double = 0.;
    let mut dP: libc::c_double = 0.;
    let mut phi: libc::c_double = 0.;
    let mut n: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    loop {
        dP = P - gsl_cdf_gamma_P(x, a, 1.0f64);
        phi = gsl_ran_gamma_pdf(x, a, 1.0f64);
        if dP == 0.0f64
            || {
                let fresh0 = n;
                n = n.wrapping_add(1);
                fresh0 > 32 as libc::c_int as libc::c_uint
            }
        {
            break;
        }
        lambda = dP
            / (if 2 as libc::c_int as libc::c_double * fabs(dP / x) > phi {
                2 as libc::c_int as libc::c_double * fabs(dP / x)
            } else {
                phi
            });
        let mut step0: libc::c_double = lambda;
        let mut step1: libc::c_double = -((a - 1 as libc::c_int as libc::c_double) / x
            - 1 as libc::c_int as libc::c_double) * lambda * lambda / 4.0f64;
        let mut step: libc::c_double = step0;
        if fabs(step1) < 0.5f64 * fabs(step0) {
            step += step1;
        }
        if x + step > 0 as libc::c_int as libc::c_double {
            x += step;
        } else {
            x /= 2.0f64;
        }
        if !(fabs(step0) > 1e-10f64 * x || fabs(step0 * phi) > 1e-10f64 * P) {
            break;
        }
    }
    if fabs(dP) > 1.4901161193847656e-08f64 * P {
        gsl_error(
            b"inverse failed to converge\0" as *const u8 as *const libc::c_char,
            b"gammainv.c\0" as *const u8 as *const libc::c_char,
            111 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return ::std::f32::NAN as libc::c_double;
    }
    return b * x;
}
pub unsafe extern "C" fn gsl_cdf_gamma_Qinv(
    Q: libc::c_double,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    if Q == 1.0f64 {
        return 0.0f64
    } else if Q == 0.0f64 {
        return ::std::f32::INFINITY as libc::c_double
    }
    if Q < 0.05f64 {
        let mut x0: libc::c_double = -log(Q) + gsl_sf_lngamma(a);
        x = x0;
    } else if Q > 0.95f64 {
        let mut x0_0: libc::c_double = exp((gsl_sf_lngamma(a) + log1p(-Q)) / a);
        x = x0_0;
    } else {
        let mut xg: libc::c_double = gsl_cdf_ugaussian_Qinv(Q);
        let mut x0_1: libc::c_double = if xg < -0.5f64 * sqrt(a) {
            a
        } else {
            sqrt(a) * xg + a
        };
        x = x0_1;
    }
    let mut lambda: libc::c_double = 0.;
    let mut dQ: libc::c_double = 0.;
    let mut phi: libc::c_double = 0.;
    let mut n: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    loop {
        dQ = Q - gsl_cdf_gamma_Q(x, a, 1.0f64);
        phi = gsl_ran_gamma_pdf(x, a, 1.0f64);
        if dQ == 0.0f64
            || {
                let fresh1 = n;
                n = n.wrapping_add(1);
                fresh1 > 32 as libc::c_int as libc::c_uint
            }
        {
            break;
        }
        lambda = -dQ
            / (if 2 as libc::c_int as libc::c_double * fabs(dQ / x) > phi {
                2 as libc::c_int as libc::c_double * fabs(dQ / x)
            } else {
                phi
            });
        let mut step0: libc::c_double = lambda;
        let mut step1: libc::c_double = -((a - 1 as libc::c_int as libc::c_double) / x
            - 1 as libc::c_int as libc::c_double) * lambda * lambda / 4.0f64;
        let mut step: libc::c_double = step0;
        if fabs(step1) < 0.5f64 * fabs(step0) {
            step += step1;
        }
        if x + step > 0 as libc::c_int as libc::c_double {
            x += step;
        } else {
            x /= 2.0f64;
        }
        if !(fabs(step0) > 1e-10f64 * x) {
            break;
        }
    }
    return b * x;
}
