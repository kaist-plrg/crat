use ::libc;
extern "C" {
    fn tan(_: libc::c_double) -> libc::c_double;
    fn expm1(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_cdf_ugaussian_Pinv(P: libc::c_double) -> libc::c_double;
    fn gsl_cdf_ugaussian_Qinv(Q: libc::c_double) -> libc::c_double;
    fn gsl_cdf_tdist_P(x: libc::c_double, nu: libc::c_double) -> libc::c_double;
    fn gsl_cdf_tdist_Q(x: libc::c_double, nu: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_ran_tdist_pdf(x: libc::c_double, nu: libc::c_double) -> libc::c_double;
    fn gsl_sf_beta(a: libc::c_double, b: libc::c_double) -> libc::c_double;
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
unsafe extern "C" fn inv_cornish_fisher(
    mut z: libc::c_double,
    mut nu: libc::c_double,
) -> libc::c_double {
    let mut a: libc::c_double = 1 as libc::c_int as libc::c_double / (nu - 0.5f64);
    let mut b: libc::c_double = 48.0f64 / (a * a);
    let mut cf1: libc::c_double = z * (3 as libc::c_int as libc::c_double + z * z);
    let mut cf2: libc::c_double = z
        * (945 as libc::c_int as libc::c_double
            + z * z
                * (360 as libc::c_int as libc::c_double
                    + z * z
                        * (63 as libc::c_int as libc::c_double
                            + z * z * 4 as libc::c_int as libc::c_double)));
    let mut y: libc::c_double = z - cf1 / b
        + cf2 / (10 as libc::c_int as libc::c_double * b * b);
    let mut t: libc::c_double = (if z >= 0.0f64 {
        1 as libc::c_int
    } else {
        -(1 as libc::c_int)
    }) as libc::c_double * sqrt(nu * expm1(a * y * y));
    return t;
}
pub unsafe extern "C" fn gsl_cdf_tdist_Pinv(
    P: libc::c_double,
    nu: libc::c_double,
) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    let mut ptail: libc::c_double = 0.;
    if P == 1.0f64 {
        return ::std::f32::INFINITY as libc::c_double
    } else if P == 0.0f64 {
        return -::std::f32::INFINITY as libc::c_double
    }
    if nu == 1.0f64 {
        x = tan(3.14159265358979323846f64 * (P - 0.5f64));
        return x;
    } else if nu == 2.0f64 {
        x = (2 as libc::c_int as libc::c_double * P - 1 as libc::c_int as libc::c_double)
            / sqrt(
                2 as libc::c_int as libc::c_double * P
                    * (1 as libc::c_int as libc::c_double - P),
            );
        return x;
    }
    ptail = if P < 0.5f64 { P } else { 1 as libc::c_int as libc::c_double - P };
    if sqrt(3.14159265358979323846f64 * nu / 2 as libc::c_int as libc::c_double) * ptail
        > pow(0.05f64, nu / 2 as libc::c_int as libc::c_double)
    {
        let mut xg: libc::c_double = gsl_cdf_ugaussian_Pinv(P);
        x = inv_cornish_fisher(xg, nu);
    } else {
        let mut beta: libc::c_double = gsl_sf_beta(
            0.5f64,
            nu / 2 as libc::c_int as libc::c_double,
        );
        if P < 0.5f64 {
            x = -sqrt(nu) * pow(beta * nu * P, -1.0f64 / nu);
        } else {
            x = sqrt(nu)
                * pow(
                    beta * nu * (1 as libc::c_int as libc::c_double - P),
                    -1.0f64 / nu,
                );
        }
        x /= sqrt(1 as libc::c_int as libc::c_double + nu / (x * x));
    }
    let mut dP: libc::c_double = 0.;
    let mut phi: libc::c_double = 0.;
    let mut n: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    loop {
        dP = P - gsl_cdf_tdist_P(x, nu);
        phi = gsl_ran_tdist_pdf(x, nu);
        if dP == 0.0f64
            || {
                let fresh0 = n;
                n = n.wrapping_add(1);
                fresh0 > 32 as libc::c_int as libc::c_uint
            }
        {
            break;
        }
        let mut lambda: libc::c_double = dP / phi;
        let mut step0: libc::c_double = lambda;
        let mut step1: libc::c_double = (nu + 1 as libc::c_int as libc::c_double) * x
            / (x * x + nu) * (lambda * lambda / 4.0f64);
        let mut step: libc::c_double = step0;
        if fabs(step1) < fabs(step0) {
            step += step1;
        }
        if P > 0.5f64 && x + step < 0 as libc::c_int as libc::c_double {
            x /= 2 as libc::c_int as libc::c_double;
        } else if P < 0.5f64 && x + step > 0 as libc::c_int as libc::c_double {
            x /= 2 as libc::c_int as libc::c_double;
        } else {
            x += step;
        }
        if !(fabs(step) > 1e-10f64 * fabs(x)) {
            break;
        }
    }
    if fabs(dP) > 1.4901161193847656e-08f64 * P {
        gsl_error(
            b"inverse failed to converge\0" as *const u8 as *const libc::c_char,
            b"tdistinv.c\0" as *const u8 as *const libc::c_char,
            139 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return ::std::f32::NAN as libc::c_double;
    }
    return x;
}
pub unsafe extern "C" fn gsl_cdf_tdist_Qinv(
    Q: libc::c_double,
    nu: libc::c_double,
) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    let mut qtail: libc::c_double = 0.;
    if Q == 0.0f64 {
        return ::std::f32::INFINITY as libc::c_double
    } else if Q == 1.0f64 {
        return -::std::f32::INFINITY as libc::c_double
    }
    if nu == 1.0f64 {
        x = tan(3.14159265358979323846f64 * (0.5f64 - Q));
        return x;
    } else if nu == 2.0f64 {
        x = (1 as libc::c_int as libc::c_double - 2 as libc::c_int as libc::c_double * Q)
            / sqrt(
                2 as libc::c_int as libc::c_double * Q
                    * (1 as libc::c_int as libc::c_double - Q),
            );
        return x;
    }
    qtail = if Q < 0.5f64 { Q } else { 1 as libc::c_int as libc::c_double - Q };
    if sqrt(3.14159265358979323846f64 * nu / 2 as libc::c_int as libc::c_double) * qtail
        > pow(0.05f64, nu / 2 as libc::c_int as libc::c_double)
    {
        let mut xg: libc::c_double = gsl_cdf_ugaussian_Qinv(Q);
        x = inv_cornish_fisher(xg, nu);
    } else {
        let mut beta: libc::c_double = gsl_sf_beta(
            0.5f64,
            nu / 2 as libc::c_int as libc::c_double,
        );
        if Q < 0.5f64 {
            x = sqrt(nu) * pow(beta * nu * Q, -1.0f64 / nu);
        } else {
            x = -sqrt(nu)
                * pow(
                    beta * nu * (1 as libc::c_int as libc::c_double - Q),
                    -1.0f64 / nu,
                );
        }
        x /= sqrt(1 as libc::c_int as libc::c_double + nu / (x * x));
    }
    let mut dQ: libc::c_double = 0.;
    let mut phi: libc::c_double = 0.;
    let mut n: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    loop {
        dQ = Q - gsl_cdf_tdist_Q(x, nu);
        phi = gsl_ran_tdist_pdf(x, nu);
        if dQ == 0.0f64
            || {
                let fresh1 = n;
                n = n.wrapping_add(1);
                fresh1 > 32 as libc::c_int as libc::c_uint
            }
        {
            break;
        }
        let mut lambda: libc::c_double = -dQ / phi;
        let mut step0: libc::c_double = lambda;
        let mut step1: libc::c_double = (nu + 1 as libc::c_int as libc::c_double) * x
            / (x * x + nu) * (lambda * lambda / 4.0f64);
        let mut step: libc::c_double = step0;
        if fabs(step1) < fabs(step0) {
            step += step1;
        }
        if Q < 0.5f64 && x + step < 0 as libc::c_int as libc::c_double {
            x /= 2 as libc::c_int as libc::c_double;
        } else if Q > 0.5f64 && x + step > 0 as libc::c_int as libc::c_double {
            x /= 2 as libc::c_int as libc::c_double;
        } else {
            x += step;
        }
        if !(fabs(step) > 1e-10f64 * fabs(x)) {
            break;
        }
    }
    return x;
}
