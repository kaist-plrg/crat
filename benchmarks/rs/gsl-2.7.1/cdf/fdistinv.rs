use ::libc;
extern "C" {
    fn gsl_cdf_beta_Pinv(
        P: libc::c_double,
        a: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_double;
    fn gsl_cdf_beta_Qinv(
        Q: libc::c_double,
        a: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
}
pub const GSL_EDOM: C2RustUnnamed = 1;
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
pub const GSL_CONTINUE: C2RustUnnamed = -2;
pub const GSL_FAILURE: C2RustUnnamed = -1;
pub const GSL_SUCCESS: C2RustUnnamed = 0;
pub unsafe extern "C" fn gsl_cdf_fdist_Pinv(
    P: libc::c_double,
    nu1: libc::c_double,
    nu2: libc::c_double,
) -> libc::c_double {
    let mut result: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    if P < 0.0f64 {
        gsl_error(
            b"P < 0.0\0" as *const u8 as *const libc::c_char,
            b"fdistinv.c\0" as *const u8 as *const libc::c_char,
            36 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return ::std::f32::NAN as libc::c_double;
    }
    if P > 1.0f64 {
        gsl_error(
            b"P > 1.0\0" as *const u8 as *const libc::c_char,
            b"fdistinv.c\0" as *const u8 as *const libc::c_char,
            40 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return ::std::f32::NAN as libc::c_double;
    }
    if nu1 < 1.0f64 {
        gsl_error(
            b"nu1 < 1\0" as *const u8 as *const libc::c_char,
            b"fdistinv.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return ::std::f32::NAN as libc::c_double;
    }
    if nu2 < 1.0f64 {
        gsl_error(
            b"nu2 < 1\0" as *const u8 as *const libc::c_char,
            b"fdistinv.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return ::std::f32::NAN as libc::c_double;
    }
    if P < 0.5f64 {
        y = gsl_cdf_beta_Pinv(P, nu1 / 2.0f64, nu2 / 2.0f64);
        result = nu2 * y / (nu1 * (1.0f64 - y));
    } else {
        y = gsl_cdf_beta_Qinv(P, nu2 / 2.0f64, nu1 / 2.0f64);
        result = nu2 * (1 as libc::c_int as libc::c_double - y) / (nu1 * y);
    }
    return result;
}
pub unsafe extern "C" fn gsl_cdf_fdist_Qinv(
    Q: libc::c_double,
    nu1: libc::c_double,
    nu2: libc::c_double,
) -> libc::c_double {
    let mut result: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    if Q < 0.0f64 {
        gsl_error(
            b"Q < 0.0\0" as *const u8 as *const libc::c_char,
            b"fdistinv.c\0" as *const u8 as *const libc::c_char,
            75 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return ::std::f32::NAN as libc::c_double;
    }
    if Q > 1.0f64 {
        gsl_error(
            b"Q > 1.0\0" as *const u8 as *const libc::c_char,
            b"fdistinv.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return ::std::f32::NAN as libc::c_double;
    }
    if nu1 < 1.0f64 {
        gsl_error(
            b"nu1 < 1\0" as *const u8 as *const libc::c_char,
            b"fdistinv.c\0" as *const u8 as *const libc::c_char,
            83 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return ::std::f32::NAN as libc::c_double;
    }
    if nu2 < 1.0f64 {
        gsl_error(
            b"nu2 < 1\0" as *const u8 as *const libc::c_char,
            b"fdistinv.c\0" as *const u8 as *const libc::c_char,
            87 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return ::std::f32::NAN as libc::c_double;
    }
    if Q > 0.5f64 {
        y = gsl_cdf_beta_Qinv(Q, nu1 / 2.0f64, nu2 / 2.0f64);
        result = nu2 * y / (nu1 * (1.0f64 - y));
    } else {
        y = gsl_cdf_beta_Pinv(Q, nu2 / 2.0f64, nu1 / 2.0f64);
        result = nu2 * (1 as libc::c_int as libc::c_double - y) / (nu1 * y);
    }
    return result;
}
