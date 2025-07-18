use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
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
    fn gsl_sf_lngamma(x: libc::c_double) -> libc::c_double;
    fn gsl_cdf_beta_P(
        x: libc::c_double,
        a: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_double;
    fn gsl_ran_beta_pdf(
        x: libc::c_double,
        a: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_double;
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
unsafe extern "C" fn bisect(
    mut x: libc::c_double,
    mut P: libc::c_double,
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut xtol: libc::c_double,
    mut Ptol: libc::c_double,
) -> libc::c_double {
    let mut x0: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut x1: libc::c_double = 1 as libc::c_int as libc::c_double;
    let mut Px: libc::c_double = 0.;
    while fabs(x1 - x0) > xtol {
        Px = gsl_cdf_beta_P(x, a, b);
        if fabs(Px - P) < Ptol {
            return x
        } else if Px < P {
            x0 = x;
        } else if Px > P {
            x1 = x;
        }
        x = 0.5f64 * (x0 + x1);
    }
    return x;
}
pub unsafe extern "C" fn gsl_cdf_beta_Pinv(
    P: libc::c_double,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    let mut mean: libc::c_double = 0.;
    if P < 0.0f64 || P > 1.0f64 {
        gsl_error(
            b"P must be in range 0 < P < 1\0" as *const u8 as *const libc::c_char,
            b"betainv.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return ::std::f32::NAN as libc::c_double;
    }
    if a < 0.0f64 {
        gsl_error(
            b"a < 0\0" as *const u8 as *const libc::c_char,
            b"betainv.c\0" as *const u8 as *const libc::c_char,
            82 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return ::std::f32::NAN as libc::c_double;
    }
    if b < 0.0f64 {
        gsl_error(
            b"b < 0\0" as *const u8 as *const libc::c_char,
            b"betainv.c\0" as *const u8 as *const libc::c_char,
            87 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return ::std::f32::NAN as libc::c_double;
    }
    if P == 0.0f64 {
        return 0.0f64;
    }
    if P == 1.0f64 {
        return 1.0f64;
    }
    if P > 0.5f64 {
        return gsl_cdf_beta_Qinv(1 as libc::c_int as libc::c_double - P, a, b);
    }
    mean = a / (a + b);
    if P < 0.1f64 {
        let mut lg_ab: libc::c_double = gsl_sf_lngamma(a + b);
        let mut lg_a: libc::c_double = gsl_sf_lngamma(a);
        let mut lg_b: libc::c_double = gsl_sf_lngamma(b);
        let mut lx: libc::c_double = (log(a) + lg_a + lg_b - lg_ab + log(P)) / a;
        if lx <= 0 as libc::c_int as libc::c_double {
            x = exp(lx);
            x
                *= pow(
                    1 as libc::c_int as libc::c_double - x,
                    -(b - 1 as libc::c_int as libc::c_double) / a,
                );
        } else {
            x = mean;
        }
        if x > mean {
            x = mean;
        }
    } else {
        x = mean;
    }
    x = bisect(x, P, a, b, 0.01f64, 0.01f64);
    let mut lambda: libc::c_double = 0.;
    let mut dP: libc::c_double = 0.;
    let mut phi: libc::c_double = 0.;
    let mut n: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    loop {
        dP = P - gsl_cdf_beta_P(x, a, b);
        phi = gsl_ran_beta_pdf(x, a, b);
        if dP == 0.0f64
            || {
                let fresh0 = n;
                n = n.wrapping_add(1);
                fresh0 > 64 as libc::c_int as libc::c_uint
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
            - (b - 1 as libc::c_int as libc::c_double)
                / (1 as libc::c_int as libc::c_double - x)) * lambda * lambda
            / 2 as libc::c_int as libc::c_double;
        let mut step: libc::c_double = step0;
        if fabs(step1) < fabs(step0) {
            step += step1;
        } else {
            step *= 2 as libc::c_int as libc::c_double * fabs(step0 / step1);
        }
        if x + step > 0 as libc::c_int as libc::c_double
            && x + step < 1 as libc::c_int as libc::c_double
        {
            x += step;
        } else {
            x = sqrt(x) * sqrt(mean);
        }
        if !(fabs(step0) > 1e-10f64 * x) {
            break;
        }
    }
    if fabs(dP) > 1.4901161193847656e-08f64 * P {
        gsl_error(
            b"inverse failed to converge\0" as *const u8 as *const libc::c_char,
            b"betainv.c\0" as *const u8 as *const libc::c_char,
            181 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return ::std::f32::NAN as libc::c_double;
    }
    return x;
}
pub unsafe extern "C" fn gsl_cdf_beta_Qinv(
    Q: libc::c_double,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    if Q < 0.0f64 || Q > 1.0f64 {
        gsl_error(
            b"Q must be inside range 0 < Q < 1\0" as *const u8 as *const libc::c_char,
            b"betainv.c\0" as *const u8 as *const libc::c_char,
            194 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return ::std::f32::NAN as libc::c_double;
    }
    if a < 0.0f64 {
        gsl_error(
            b"a < 0\0" as *const u8 as *const libc::c_char,
            b"betainv.c\0" as *const u8 as *const libc::c_char,
            199 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return ::std::f32::NAN as libc::c_double;
    }
    if b < 0.0f64 {
        gsl_error(
            b"b < 0\0" as *const u8 as *const libc::c_char,
            b"betainv.c\0" as *const u8 as *const libc::c_char,
            204 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return ::std::f32::NAN as libc::c_double;
    }
    if Q == 0.0f64 {
        return 1.0f64;
    }
    if Q == 1.0f64 {
        return 0.0f64;
    }
    if Q > 0.5f64 {
        return gsl_cdf_beta_Pinv(1 as libc::c_int as libc::c_double - Q, a, b)
    } else {
        return 1 as libc::c_int as libc::c_double - gsl_cdf_beta_Pinv(Q, b, a)
    };
}
