use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sf_sin_pi_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_cos_pi_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_bessel_IJ_taylor_e(
        nu: libc::c_double,
        x: libc::c_double,
        sign: libc::c_int,
        kmax: libc::c_int,
        threshold: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_bessel_Jnu_asympx_e(
        nu: libc::c_double,
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_bessel_Ynupos_e(
        nu: libc::c_double,
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_bessel_J_CF1(
        nu: libc::c_double,
        x: libc::c_double,
        ratio: *mut libc::c_double,
        sgn: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_sf_bessel_JY_steed_CF2(
        nu: libc::c_double,
        x: libc::c_double,
        P: *mut libc::c_double,
        Q: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_sf_bessel_Jnu_asymp_Olver_e(
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
pub unsafe extern "C" fn gsl_sf_bessel_Jnupos_e(
    nu: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x == 0.0f64 {
        if nu == 0.0f64 {
            (*result).val = 1.0f64;
            (*result).err = 0.0f64;
        } else {
            (*result).val = 0.0f64;
            (*result).err = 0.0f64;
        }
        return GSL_SUCCESS as libc::c_int;
    } else if x * x < 10.0f64 * (nu + 1.0f64) {
        return gsl_sf_bessel_IJ_taylor_e(
            nu,
            x,
            -(1 as libc::c_int),
            100 as libc::c_int,
            2.2204460492503131e-16f64,
            result,
        )
    } else if nu > 50.0f64 {
        return gsl_sf_bessel_Jnu_asymp_Olver_e(nu, x, result)
    } else if x > 1000.0f64 {
        return gsl_sf_bessel_Jnu_asympx_e(nu, x, result)
    } else {
        let mut N: libc::c_int = (nu + 0.5f64) as libc::c_int;
        let mut mu: libc::c_double = nu - N as libc::c_double;
        let mut Jnup1_Jnu: libc::c_double = 0.;
        let mut sgn_Jnu: libc::c_double = 0.;
        let stat_CF1: libc::c_int = gsl_sf_bessel_J_CF1(
            nu,
            x,
            &mut Jnup1_Jnu,
            &mut sgn_Jnu,
        );
        if x < 2.0f64 {
            let mut Y_mu: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut Y_mup1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let stat_mu: libc::c_int = gsl_sf_bessel_Y_temme(
                mu,
                x,
                &mut Y_mu,
                &mut Y_mup1,
            );
            let mut Ynm1: libc::c_double = Y_mu.val;
            let mut Yn: libc::c_double = Y_mup1.val;
            let mut Ynp1: libc::c_double = 0.0f64;
            let mut n: libc::c_int = 0;
            n = 1 as libc::c_int;
            while n < N {
                Ynp1 = 2.0f64 * (mu + n as libc::c_double) / x * Yn - Ynm1;
                Ynm1 = Yn;
                Yn = Ynp1;
                n += 1;
                n;
            }
            (*result)
                .val = 2.0f64 / (3.14159265358979323846f64 * x)
                / (Jnup1_Jnu * Yn - Ynp1);
            (*result)
                .err = 2.2204460492503131e-16f64 * (N as libc::c_double + 2.0f64)
                * fabs((*result).val);
            return if stat_mu != GSL_SUCCESS as libc::c_int {
                stat_mu
            } else if stat_CF1 != GSL_SUCCESS as libc::c_int {
                stat_CF1
            } else {
                GSL_SUCCESS as libc::c_int
            };
        } else {
            let mut Jmu: libc::c_double = 0.;
            let mut Jmup1_Jmu: libc::c_double = 0.;
            let mut sgn_Jmu: libc::c_double = 0.;
            let mut Jmuprime_Jmu: libc::c_double = 0.;
            let mut P: libc::c_double = 0.;
            let mut Q: libc::c_double = 0.;
            let stat_CF2: libc::c_int = gsl_sf_bessel_JY_steed_CF2(
                mu,
                x,
                &mut P,
                &mut Q,
            );
            let mut gamma: libc::c_double = 0.;
            let mut Jnp1: libc::c_double = sgn_Jnu * 1.4916681462400413e-154f64
                * Jnup1_Jnu;
            let mut Jn: libc::c_double = sgn_Jnu * 1.4916681462400413e-154f64;
            let mut Jnm1: libc::c_double = 0.;
            let mut n_0: libc::c_int = 0;
            n_0 = N;
            while n_0 > 0 as libc::c_int {
                Jnm1 = 2.0f64 * (mu + n_0 as libc::c_double) / x * Jn - Jnp1;
                Jnp1 = Jn;
                Jn = Jnm1;
                n_0 -= 1;
                n_0;
            }
            Jmup1_Jmu = Jnp1 / Jn;
            sgn_Jmu = (if Jn >= 0.0f64 { 1 as libc::c_int } else { -(1 as libc::c_int) })
                as libc::c_double;
            Jmuprime_Jmu = mu / x - Jmup1_Jmu;
            gamma = (P - Jmuprime_Jmu) / Q;
            Jmu = sgn_Jmu
                * sqrt(
                    2.0f64 / (3.14159265358979323846f64 * x)
                        / (Q + gamma * (P - Jmuprime_Jmu)),
                );
            (*result).val = Jmu * (sgn_Jnu * 1.4916681462400413e-154f64) / Jn;
            (*result)
                .err = 2.0f64 * 2.2204460492503131e-16f64
                * (N as libc::c_double + 2.0f64) * fabs((*result).val);
            return if stat_CF2 != GSL_SUCCESS as libc::c_int {
                stat_CF2
            } else if stat_CF1 != GSL_SUCCESS as libc::c_int {
                stat_CF1
            } else {
                GSL_SUCCESS as libc::c_int
            };
        }
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_Jnu_e(
    nu: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x <= 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"bessel_Jnu.c\0" as *const u8 as *const libc::c_char,
            182 as libc::c_int,
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
        (*result).val = s * Yval + c * Jval;
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
        return gsl_sf_bessel_Jnupos_e(nu, x, result)
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_Jnu(
    nu: libc::c_double,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_Jnu_e(nu, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_Jnu_e(nu, x, &result)\0" as *const u8 as *const libc::c_char,
            b"bessel_Jnu.c\0" as *const u8 as *const libc::c_char,
            211 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
