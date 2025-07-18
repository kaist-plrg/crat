use ::libc;
extern "C" {
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
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
    fn gsl_sf_lngamma_sgn_e(
        x: libc::c_double,
        result_lg: *mut gsl_sf_result,
        sgn: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_sf_bessel_Jnu_e(
        nu: libc::c_double,
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_bessel_Ynu_e(
        nu: libc::c_double,
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_bessel_Inu_scaled_e(
        nu: libc::c_double,
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_bessel_Knu_scaled_e(
        nu: libc::c_double,
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
unsafe extern "C" fn hyperg_0F1_bessel_I(
    nu: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x > 7.0978271289338397e+02f64 {
        (*result).val = ::std::f32::INFINITY as libc::c_double;
        (*result).err = ::std::f32::INFINITY as libc::c_double;
        gsl_error(
            b"overflow\0" as *const u8 as *const libc::c_char,
            b"hyperg_0F1.c\0" as *const u8 as *const libc::c_char,
            45 as libc::c_int,
            GSL_EOVRFLW as libc::c_int,
        );
        return GSL_EOVRFLW as libc::c_int;
    }
    if nu < 0.0f64 {
        let anu: libc::c_double = -nu;
        let s: libc::c_double = 2.0f64 / 3.14159265358979323846f64
            * sin(anu * 3.14159265358979323846f64);
        let ex: libc::c_double = exp(x);
        let mut I: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut K: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_I: libc::c_int = gsl_sf_bessel_Inu_scaled_e(anu, x, &mut I);
        let mut stat_K: libc::c_int = gsl_sf_bessel_Knu_scaled_e(anu, x, &mut K);
        (*result).val = ex * I.val + s * (K.val / ex);
        (*result).err = ex * I.err + fabs(s * K.err / ex);
        (*result).err
            += fabs(s * (K.val / ex)) * 2.2204460492503131e-16f64 * anu
                * 3.14159265358979323846f64;
        return if stat_K != GSL_SUCCESS as libc::c_int {
            stat_K
        } else if stat_I != GSL_SUCCESS as libc::c_int {
            stat_I
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else {
        let ex_0: libc::c_double = exp(x);
        let mut I_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_I_0: libc::c_int = gsl_sf_bessel_Inu_scaled_e(nu, x, &mut I_0);
        (*result).val = ex_0 * I_0.val;
        (*result).err = ex_0 * I_0.err + 2.2204460492503131e-16f64 * fabs((*result).val);
        return stat_I_0;
    };
}
unsafe extern "C" fn hyperg_0F1_bessel_J(
    nu: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if nu < 0.0f64 {
        let anu: libc::c_double = -nu;
        let s: libc::c_double = sin(anu * 3.14159265358979323846f64);
        let c: libc::c_double = cos(anu * 3.14159265358979323846f64);
        let mut J: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut Y: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_J: libc::c_int = gsl_sf_bessel_Jnu_e(anu, x, &mut J);
        let mut stat_Y: libc::c_int = gsl_sf_bessel_Ynu_e(anu, x, &mut Y);
        (*result).val = c * J.val - s * Y.val;
        (*result).err = fabs(c * J.err) + fabs(s * Y.err);
        (*result).err
            += fabs(anu * 3.14159265358979323846f64) * 2.2204460492503131e-16f64
                * fabs(J.val + Y.val);
        return if stat_Y != GSL_SUCCESS as libc::c_int {
            stat_Y
        } else if stat_J != GSL_SUCCESS as libc::c_int {
            stat_J
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else {
        return gsl_sf_bessel_Jnu_e(nu, x, result)
    };
}
pub unsafe extern "C" fn gsl_sf_hyperg_0F1_e(
    mut c: libc::c_double,
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let rintc: libc::c_double = floor(c + 0.5f64);
    let c_neg_integer: libc::c_int = (c < 0.0f64
        && fabs(c - rintc) < 1000.0f64 * 2.2204460492503131e-16f64) as libc::c_int;
    if c == 0.0f64 || c_neg_integer != 0 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"hyperg_0F1.c\0" as *const u8 as *const libc::c_char,
            111 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x < 0.0f64 {
        let mut Jcm1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut lg_c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut sgn: libc::c_double = 0.;
        let mut stat_g: libc::c_int = gsl_sf_lngamma_sgn_e(c, &mut lg_c, &mut sgn);
        let mut stat_J: libc::c_int = hyperg_0F1_bessel_J(
            c - 1.0f64,
            2.0f64 * sqrt(-x),
            &mut Jcm1,
        );
        if stat_g != GSL_SUCCESS as libc::c_int {
            (*result).val = 0.0f64;
            (*result).err = 0.0f64;
            return stat_g;
        } else if Jcm1.val == 0.0f64 {
            (*result).val = 0.0f64;
            (*result).err = 0.0f64;
            return stat_J;
        } else {
            let tl: libc::c_double = log(-x) * 0.5f64 * (1.0f64 - c);
            let mut ln_pre_val: libc::c_double = lg_c.val + tl;
            let mut ln_pre_err: libc::c_double = lg_c.err
                + 2.0f64 * 2.2204460492503131e-16f64 * fabs(tl);
            return gsl_sf_exp_mult_err_e(
                ln_pre_val,
                ln_pre_err,
                sgn * Jcm1.val,
                Jcm1.err,
                result,
            );
        }
    } else if x == 0.0f64 {
        (*result).val = 1.0f64;
        (*result).err = 1.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut Icm1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut lg_c_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut sgn_0: libc::c_double = 0.;
        let mut stat_g_0: libc::c_int = gsl_sf_lngamma_sgn_e(c, &mut lg_c_0, &mut sgn_0);
        let mut stat_I: libc::c_int = hyperg_0F1_bessel_I(
            c - 1.0f64,
            2.0f64 * sqrt(x),
            &mut Icm1,
        );
        if stat_g_0 != GSL_SUCCESS as libc::c_int {
            (*result).val = 0.0f64;
            (*result).err = 0.0f64;
            return stat_g_0;
        } else if Icm1.val == 0.0f64 {
            (*result).val = 0.0f64;
            (*result).err = 0.0f64;
            return stat_I;
        } else {
            let tl_0: libc::c_double = log(x) * 0.5f64 * (1.0f64 - c);
            let ln_pre_val_0: libc::c_double = lg_c_0.val + tl_0;
            let ln_pre_err_0: libc::c_double = lg_c_0.err
                + 2.0f64 * 2.2204460492503131e-16f64 * fabs(tl_0);
            return gsl_sf_exp_mult_err_e(
                ln_pre_val_0,
                ln_pre_err_0,
                sgn_0 * Icm1.val,
                Icm1.err,
                result,
            );
        }
    };
}
pub unsafe extern "C" fn gsl_sf_hyperg_0F1(
    c: libc::c_double,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_hyperg_0F1_e(c, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_hyperg_0F1_e(c, x, &result)\0" as *const u8 as *const libc::c_char,
            b"hyperg_0F1.c\0" as *const u8 as *const libc::c_char,
            177 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
