use ::libc;
extern "C" {
    fn log(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sf_exp_err_e(
        x: libc::c_double,
        dx: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_log_1plusx_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_lngamma_sgn_e(
        x: libc::c_double,
        result_lg: *mut gsl_sf_result,
        sgn: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_sf_gamma_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_gammastar_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
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
unsafe extern "C" fn isnegint(x: libc::c_double) -> libc::c_double {
    return (x < 0 as libc::c_int as libc::c_double && x == floor(x)) as libc::c_int
        as libc::c_double;
}
pub unsafe extern "C" fn gsl_sf_lnbeta_e(
    x: libc::c_double,
    y: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut sgn: libc::c_double = 0.;
    let mut status: libc::c_int = gsl_sf_lnbeta_sgn_e(x, y, result, &mut sgn);
    if sgn == -(1 as libc::c_int) as libc::c_double {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"beta.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    }
    return status;
}
pub unsafe extern "C" fn gsl_sf_lnbeta_sgn_e(
    x: libc::c_double,
    y: libc::c_double,
    mut result: *mut gsl_sf_result,
    mut sgn: *mut libc::c_double,
) -> libc::c_int {
    if x == 0.0f64 || y == 0.0f64 {
        *sgn = 0.0f64;
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"beta.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if isnegint(x) != 0. || isnegint(y) != 0. {
        *sgn = 0.0f64;
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"beta.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    }
    if x > 0 as libc::c_int as libc::c_double && y > 0 as libc::c_int as libc::c_double {
        let max: libc::c_double = if x > y { x } else { y };
        let min: libc::c_double = if x < y { x } else { y };
        let rat: libc::c_double = min / max;
        if rat < 0.2f64 {
            let mut lnpre_val: libc::c_double = 0.;
            let mut lnpre_err: libc::c_double = 0.;
            let mut lnpow_val: libc::c_double = 0.;
            let mut lnpow_err: libc::c_double = 0.;
            let mut t1: libc::c_double = 0.;
            let mut t2: libc::c_double = 0.;
            let mut t3: libc::c_double = 0.;
            let mut lnopr: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut gsx: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut gsy: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut gsxy: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            gsl_sf_gammastar_e(x, &mut gsx);
            gsl_sf_gammastar_e(y, &mut gsy);
            gsl_sf_gammastar_e(x + y, &mut gsxy);
            gsl_sf_log_1plusx_e(rat, &mut lnopr);
            lnpre_val = log(
                gsx.val * gsy.val / gsxy.val * 1.41421356237309504880f64
                    * 1.77245385090551602729816748334f64,
            );
            lnpre_err = gsx.err / gsx.val + gsy.err / gsy.val + gsxy.err / gsxy.val;
            t1 = min * log(rat);
            t2 = 0.5f64 * log(min);
            t3 = (x + y - 0.5f64) * lnopr.val;
            lnpow_val = t1 - t2 - t3;
            lnpow_err = 2.2204460492503131e-16f64 * (fabs(t1) + fabs(t2) + fabs(t3));
            lnpow_err += fabs(x + y - 0.5f64) * lnopr.err;
            (*result).val = lnpre_val + lnpow_val;
            (*result).err = lnpre_err + lnpow_err;
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
            *sgn = 1.0f64;
            return GSL_SUCCESS as libc::c_int;
        }
    }
    let mut lgx: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut lgy: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut lgxy: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut sgx: libc::c_double = 0.;
    let mut sgy: libc::c_double = 0.;
    let mut sgxy: libc::c_double = 0.;
    let mut xy: libc::c_double = x + y;
    let mut stat_gx: libc::c_int = gsl_sf_lngamma_sgn_e(x, &mut lgx, &mut sgx);
    let mut stat_gy: libc::c_int = gsl_sf_lngamma_sgn_e(y, &mut lgy, &mut sgy);
    let mut stat_gxy: libc::c_int = gsl_sf_lngamma_sgn_e(xy, &mut lgxy, &mut sgxy);
    *sgn = sgx * sgy * sgxy;
    (*result).val = lgx.val + lgy.val - lgxy.val;
    (*result).err = lgx.err + lgy.err + lgxy.err;
    (*result).err
        += 2.0f64 * 2.2204460492503131e-16f64
            * (fabs(lgx.val) + fabs(lgy.val) + fabs(lgxy.val));
    (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
    return if stat_gx != GSL_SUCCESS as libc::c_int {
        stat_gx
    } else if stat_gy != GSL_SUCCESS as libc::c_int {
        stat_gy
    } else if stat_gxy != GSL_SUCCESS as libc::c_int {
        stat_gxy
    } else {
        GSL_SUCCESS as libc::c_int
    };
}
pub unsafe extern "C" fn gsl_sf_beta_e(
    x: libc::c_double,
    y: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x > 0 as libc::c_int as libc::c_double && y > 0 as libc::c_int as libc::c_double
        && x < 50.0f64 && y < 50.0f64
    {
        let mut gx: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut gy: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut gxy: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        gsl_sf_gamma_e(x, &mut gx);
        gsl_sf_gamma_e(y, &mut gy);
        gsl_sf_gamma_e(x + y, &mut gxy);
        (*result).val = gx.val * gy.val / gxy.val;
        (*result).err = gx.err * fabs(gy.val / gxy.val);
        (*result).err += gy.err * fabs(gx.val / gxy.val);
        (*result).err += fabs(gx.val * gy.val / (gxy.val * gxy.val)) * gxy.err;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if isnegint(x) != 0. || isnegint(y) != 0. {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"beta.c\0" as *const u8 as *const libc::c_char,
            134 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if isnegint(x + y) != 0. {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut lb: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut sgn: libc::c_double = 0.;
        let mut stat_lb: libc::c_int = gsl_sf_lnbeta_sgn_e(x, y, &mut lb, &mut sgn);
        if stat_lb == GSL_SUCCESS as libc::c_int {
            let mut status: libc::c_int = gsl_sf_exp_err_e(lb.val, lb.err, result);
            (*result).val *= sgn;
            return status;
        } else {
            (*result).val = 0.0f64;
            (*result).err = 0.0f64;
            return stat_lb;
        }
    };
}
pub unsafe extern "C" fn gsl_sf_lnbeta(
    x: libc::c_double,
    y: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_lnbeta_e(x, y, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_lnbeta_e(x, y, &result)\0" as *const u8 as *const libc::c_char,
            b"beta.c\0" as *const u8 as *const libc::c_char,
            163 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_beta(
    x: libc::c_double,
    y: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_beta_e(x, y, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_beta_e(x, y, &result)\0" as *const u8 as *const libc::c_char,
            b"beta.c\0" as *const u8 as *const libc::c_char,
            168 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
