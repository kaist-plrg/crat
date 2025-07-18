use ::libc;
extern "C" {
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn cosh(_: libc::c_double) -> libc::c_double;
    fn tanh(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_hypot(x: libc::c_double, y: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
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
pub unsafe extern "C" fn gsl_sf_elljac_e(
    mut u: libc::c_double,
    mut m: libc::c_double,
    mut sn: *mut libc::c_double,
    mut cn: *mut libc::c_double,
    mut dn: *mut libc::c_double,
) -> libc::c_int {
    if fabs(m) > 1.0f64 {
        *sn = 0.0f64;
        *cn = 0.0f64;
        *dn = 0.0f64;
        gsl_error(
            b"|m| > 1.0\0" as *const u8 as *const libc::c_char,
            b"elljac.c\0" as *const u8 as *const libc::c_char,
            46 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if fabs(m) < 2.0f64 * 2.2204460492503131e-16f64 {
        *sn = sin(u);
        *cn = cos(u);
        *dn = 1.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if fabs(m - 1.0f64) < 2.0f64 * 2.2204460492503131e-16f64 {
        *sn = tanh(u);
        *cn = 1.0f64 / cosh(u);
        *dn = *cn;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let N: libc::c_int = 16 as libc::c_int;
        let mut mu: [libc::c_double; 16] = [0.; 16];
        let mut nu: [libc::c_double; 16] = [0.; 16];
        let mut c: [libc::c_double; 16] = [0.; 16];
        let mut d: [libc::c_double; 16] = [0.; 16];
        let mut sin_umu: libc::c_double = 0.;
        let mut cos_umu: libc::c_double = 0.;
        let mut t: libc::c_double = 0.;
        let mut r: libc::c_double = 0.;
        let mut n: libc::c_int = 0 as libc::c_int;
        mu[0 as libc::c_int as usize] = 1.0f64;
        nu[0 as libc::c_int as usize] = sqrt(1.0f64 - m);
        while fabs(mu[n as usize] - nu[n as usize])
            > 4.0f64 * 2.2204460492503131e-16f64 * fabs(mu[n as usize] + nu[n as usize])
        {
            mu[(n + 1 as libc::c_int)
                as usize] = 0.5f64 * (mu[n as usize] + nu[n as usize]);
            nu[(n + 1 as libc::c_int) as usize] = sqrt(mu[n as usize] * nu[n as usize]);
            n += 1;
            n;
            if !(n >= N - 1 as libc::c_int) {
                continue;
            }
            status = GSL_EMAXITER as libc::c_int;
            break;
        }
        sin_umu = sin(u * mu[n as usize]);
        cos_umu = cos(u * mu[n as usize]);
        if fabs(sin_umu) < fabs(cos_umu) {
            t = sin_umu / cos_umu;
            c[n as usize] = mu[n as usize] * t;
            d[n as usize] = 1.0f64;
            while n > 0 as libc::c_int {
                n -= 1;
                n;
                c[n
                    as usize] = d[(n + 1 as libc::c_int) as usize]
                    * c[(n + 1 as libc::c_int) as usize];
                r = c[(n + 1 as libc::c_int) as usize]
                    * c[(n + 1 as libc::c_int) as usize]
                    / mu[(n + 1 as libc::c_int) as usize];
                d[n as usize] = (r + nu[n as usize]) / (r + mu[n as usize]);
            }
            *dn = sqrt(1.0f64 - m) / d[n as usize];
            *cn = *dn
                * (if cos_umu >= 0.0f64 {
                    1 as libc::c_int
                } else {
                    -(1 as libc::c_int)
                }) as libc::c_double / gsl_hypot(1.0f64, c[n as usize]);
            *sn = *cn * c[n as usize] / sqrt(1.0f64 - m);
        } else {
            t = cos_umu / sin_umu;
            c[n as usize] = mu[n as usize] * t;
            d[n as usize] = 1.0f64;
            while n > 0 as libc::c_int {
                n -= 1;
                n;
                c[n
                    as usize] = d[(n + 1 as libc::c_int) as usize]
                    * c[(n + 1 as libc::c_int) as usize];
                r = c[(n + 1 as libc::c_int) as usize]
                    * c[(n + 1 as libc::c_int) as usize]
                    / mu[(n + 1 as libc::c_int) as usize];
                d[n as usize] = (r + nu[n as usize]) / (r + mu[n as usize]);
            }
            *dn = d[n as usize];
            *sn = (if sin_umu >= 0.0f64 {
                1 as libc::c_int
            } else {
                -(1 as libc::c_int)
            }) as libc::c_double / gsl_hypot(1.0f64, c[n as usize]);
            *cn = c[n as usize] * *sn;
        }
        return status;
    };
}
