use ::libc;
extern "C" {
    fn log(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
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
    fn gsl_sf_result_smash_e(
        re: *const gsl_sf_result_e10,
        r: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_lngamma_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_bessel_K0_scaled_e(
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_bessel_K_scaled_steed_temme_CF2(
        nu: libc::c_double,
        x: libc::c_double,
        K_nu: *mut libc::c_double,
        K_nup1: *mut libc::c_double,
        Kp_nu: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_sf_bessel_K_scaled_temme(
        nu: libc::c_double,
        x: libc::c_double,
        K_nu: *mut libc::c_double,
        K_nup1: *mut libc::c_double,
        Kp_nu: *mut libc::c_double,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_sf_result_e10_struct {
    pub val: libc::c_double,
    pub err: libc::c_double,
    pub e10: libc::c_int,
}
pub type gsl_sf_result_e10 = gsl_sf_result_e10_struct;
pub unsafe extern "C" fn gsl_sf_bessel_Knu_scaled_e(
    nu: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x <= 0.0f64 || nu < 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"bessel_Knu.c\0" as *const u8 as *const libc::c_char,
            42 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        let mut result_e10: gsl_sf_result_e10 = gsl_sf_result_e10 {
            val: 0.,
            err: 0.,
            e10: 0,
        };
        let mut status: libc::c_int = gsl_sf_bessel_Knu_scaled_e10_e(
            nu,
            x,
            &mut result_e10,
        );
        let mut status2: libc::c_int = gsl_sf_result_smash_e(&mut result_e10, result);
        return if status != GSL_SUCCESS as libc::c_int {
            status
        } else if status2 != GSL_SUCCESS as libc::c_int {
            status2
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_Knu_scaled_e10_e(
    nu: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result_e10,
) -> libc::c_int {
    if x <= 0.0f64 || nu < 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        (*result).e10 = 0 as libc::c_int;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"bessel_Knu.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        let mut N: libc::c_int = (nu + 0.5f64) as libc::c_int;
        let mut mu: libc::c_double = nu - N as libc::c_double;
        let mut K_mu: libc::c_double = 0.;
        let mut K_mup1: libc::c_double = 0.;
        let mut Kp_mu: libc::c_double = 0.;
        let mut K_nu: libc::c_double = 0.;
        let mut K_nup1: libc::c_double = 0.;
        let mut K_num1: libc::c_double = 0.;
        let mut n: libc::c_int = 0;
        let mut e10: libc::c_int = 0 as libc::c_int;
        if x < 2.0f64 {
            gsl_sf_bessel_K_scaled_temme(mu, x, &mut K_mu, &mut K_mup1, &mut Kp_mu);
        } else {
            gsl_sf_bessel_K_scaled_steed_temme_CF2(
                mu,
                x,
                &mut K_mu,
                &mut K_mup1,
                &mut Kp_mu,
            );
        }
        K_nu = K_mu;
        K_nup1 = K_mup1;
        n = 0 as libc::c_int;
        while n < N {
            K_num1 = K_nu;
            K_nu = K_nup1;
            if fabs(K_nu) > 1.3407807929942596e+154f64 {
                let mut p: libc::c_double = floor(
                    log(fabs(K_nu)) / 2.30258509299404568402f64,
                );
                let mut factor: libc::c_double = pow(10.0f64, p);
                K_num1 /= factor;
                K_nu /= factor;
                e10 = (e10 as libc::c_double + p) as libc::c_int;
            }
            K_nup1 = 2.0f64
                * (mu + n as libc::c_double + 1 as libc::c_int as libc::c_double) / x
                * K_nu + K_num1;
            n += 1;
            n;
        }
        (*result).val = K_nu;
        (*result)
            .err = 2.0f64 * 2.2204460492503131e-16f64 * (N as libc::c_double + 4.0f64)
            * fabs((*result).val);
        (*result).e10 = e10;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_Knu_e(
    nu: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut b: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut stat_K: libc::c_int = gsl_sf_bessel_Knu_scaled_e(nu, x, &mut b);
    let mut stat_e: libc::c_int = gsl_sf_exp_mult_err_e(
        -x,
        0.0f64,
        b.val,
        b.err,
        result,
    );
    return if stat_e != GSL_SUCCESS as libc::c_int {
        stat_e
    } else if stat_K != GSL_SUCCESS as libc::c_int {
        stat_K
    } else {
        GSL_SUCCESS as libc::c_int
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_lnKnu_e(
    nu: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x <= 0.0f64 || nu < 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"bessel_Knu.c\0" as *const u8 as *const libc::c_char,
            116 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if nu == 0.0f64 {
        let mut K_scaled: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        gsl_sf_bessel_K0_scaled_e(x, &mut K_scaled);
        (*result).val = -x + log(fabs(K_scaled.val));
        (*result)
            .err = 2.2204460492503131e-16f64 * fabs(x)
            + fabs(K_scaled.err / K_scaled.val);
        (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x < 2.0f64 && nu > 1.0f64 {
        let mut ln_bound: libc::c_double = 0.;
        let mut lg_nu: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        gsl_sf_lngamma_e(nu, &mut lg_nu);
        ln_bound = -0.69314718055994530942f64 - nu * log(0.5f64 * x) + lg_nu.val;
        if ln_bound > 7.0978271289338397e+02f64 - 20.0f64 {
            let mut xi: libc::c_double = 0.25f64 * x * x;
            let mut sum: libc::c_double = 1.0f64 - xi / (nu - 1.0f64);
            if nu > 2.0f64 {
                sum += xi / (nu - 1.0f64) * (xi / (nu - 2.0f64));
            }
            (*result).val = ln_bound + log(sum);
            (*result).err = lg_nu.err;
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
            return GSL_SUCCESS as libc::c_int;
        }
    }
    let mut K_scaled_0: gsl_sf_result_e10 = gsl_sf_result_e10 {
        val: 0.,
        err: 0.,
        e10: 0,
    };
    let mut status: libc::c_int = gsl_sf_bessel_Knu_scaled_e10_e(nu, x, &mut K_scaled_0);
    (*result)
        .val = -x + log(fabs(K_scaled_0.val))
        + K_scaled_0.e10 as libc::c_double * 2.30258509299404568402f64;
    (*result)
        .err = 2.2204460492503131e-16f64 * fabs(x)
        + fabs(K_scaled_0.err / K_scaled_0.val);
    (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
    return status;
}
pub unsafe extern "C" fn gsl_sf_bessel_Knu_scaled(
    nu: libc::c_double,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_Knu_scaled_e(nu, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_Knu_scaled_e(nu, x, &result)\0" as *const u8
                as *const libc::c_char,
            b"bessel_Knu.c\0" as *const u8 as *const libc::c_char,
            178 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_bessel_Knu(
    nu: libc::c_double,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_Knu_e(nu, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_Knu_e(nu, x, &result)\0" as *const u8 as *const libc::c_char,
            b"bessel_Knu.c\0" as *const u8 as *const libc::c_char,
            183 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_bessel_lnKnu(
    nu: libc::c_double,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_lnKnu_e(nu, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_lnKnu_e(nu, x, &result)\0" as *const u8
                as *const libc::c_char,
            b"bessel_Knu.c\0" as *const u8 as *const libc::c_char,
            188 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
