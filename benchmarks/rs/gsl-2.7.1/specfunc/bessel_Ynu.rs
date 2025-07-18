use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sf_sin_pi_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_cos_pi_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_bessel_Jnupos_e(
        nu: libc::c_double,
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_bessel_JY_mu_restricted(
        mu: libc::c_double,
        x: libc::c_double,
        Jmu: *mut gsl_sf_result,
        Jmup1: *mut gsl_sf_result,
        Ymu: *mut gsl_sf_result,
        Ymup1: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_bessel_Ynu_asymp_Olver_e(
        nu: libc::c_double,
        x: libc::c_double,
        result: *mut gsl_sf_result,
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
pub unsafe extern "C" fn gsl_sf_bessel_Ynupos_e(
    mut nu: libc::c_double,
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if nu > 50.0f64 {
        return gsl_sf_bessel_Ynu_asymp_Olver_e(nu, x, result)
    } else {
        let mut N: libc::c_int = (nu + 0.5f64) as libc::c_int;
        let mut mu: libc::c_double = nu - N as libc::c_double;
        let mut Y_mu: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut Y_mup1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_mu: libc::c_int = 0;
        let mut Ynm1: libc::c_double = 0.;
        let mut Yn: libc::c_double = 0.;
        let mut Ynp1: libc::c_double = 0.;
        let mut n: libc::c_int = 0;
        if x < 2.0f64 {
            stat_mu = gsl_sf_bessel_Y_temme(mu, x, &mut Y_mu, &mut Y_mup1);
        } else {
            let mut J_mu: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut J_mup1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            stat_mu = gsl_sf_bessel_JY_mu_restricted(
                mu,
                x,
                &mut J_mu,
                &mut J_mup1,
                &mut Y_mu,
                &mut Y_mup1,
            );
        }
        Ynm1 = Y_mu.val;
        Yn = Y_mup1.val;
        n = 1 as libc::c_int;
        while n <= N {
            Ynp1 = 2.0f64 * (mu + n as libc::c_double) / x * Yn - Ynm1;
            Ynm1 = Yn;
            Yn = Ynp1;
            n += 1;
            n;
        }
        (*result).val = Ynm1;
        (*result)
            .err = (N as libc::c_double + 1.0f64) * fabs(Ynm1)
            * (fabs(Y_mu.err / Y_mu.val) + fabs(Y_mup1.err / Y_mup1.val));
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs(Ynm1);
        return stat_mu;
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_Ynu_e(
    mut nu: libc::c_double,
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x <= 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"bessel_Ynu.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if nu < 0.0f64 {
        let mut Jstatus: libc::c_int = gsl_sf_bessel_Jnupos_e(-nu, x, result);
        let mut Jval: libc::c_double = (*result).val;
        let mut Jerr: libc::c_double = (*result).err;
        let mut Ystatus: libc::c_int = gsl_sf_bessel_Ynupos_e(-nu, x, result);
        let mut Yval: libc::c_double = (*result).val;
        let mut Yerr: libc::c_double = (*result).err;
        let mut sinstatus: libc::c_int = gsl_sf_sin_pi_e(nu, result);
        let mut s: libc::c_double = (*result).val;
        let mut serr: libc::c_double = (*result).err;
        let mut cosstatus: libc::c_int = gsl_sf_cos_pi_e(nu, result);
        let mut c: libc::c_double = (*result).val;
        let mut cerr: libc::c_double = (*result).err;
        (*result).val = c * Yval - s * Jval;
        (*result)
            .err = fabs(c * Yerr) + fabs(s * Jerr) + fabs(cerr * Yval)
            + fabs(serr * Jval);
        return if Jstatus != GSL_SUCCESS as libc::c_int {
            Jstatus
        } else if Ystatus != GSL_SUCCESS as libc::c_int {
            Ystatus
        } else if sinstatus != GSL_SUCCESS as libc::c_int {
            sinstatus
        } else if cosstatus != GSL_SUCCESS as libc::c_int {
            cosstatus
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else {
        return gsl_sf_bessel_Ynupos_e(nu, x, result)
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_Ynu(
    nu: libc::c_double,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_Ynu_e(nu, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_Ynu_e(nu, x, &result)\0" as *const u8 as *const libc::c_char,
            b"bessel_Ynu.c\0" as *const u8 as *const libc::c_char,
            155 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
