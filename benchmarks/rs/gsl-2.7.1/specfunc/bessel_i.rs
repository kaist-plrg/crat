use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sf_bessel_IJ_taylor_e(
        nu: libc::c_double,
        x: libc::c_double,
        sign: libc::c_int,
        kmax: libc::c_int,
        threshold: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_bessel_Inu_scaled_asymp_unif_e(
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
unsafe extern "C" fn bessel_il_CF1(
    l: libc::c_int,
    x: libc::c_double,
    threshold: libc::c_double,
    mut ratio: *mut libc::c_double,
) -> libc::c_int {
    let kmax: libc::c_int = 2000 as libc::c_int;
    let mut tk: libc::c_double = 1.0f64;
    let mut sum: libc::c_double = 1.0f64;
    let mut rhok: libc::c_double = 0.0f64;
    let mut k: libc::c_int = 0;
    k = 1 as libc::c_int;
    while k <= kmax {
        let mut ak: libc::c_double = x
            / (2.0f64 * l as libc::c_double + 1.0f64 + 2.0f64 * k as libc::c_double)
            * (x
                / (2.0f64 * l as libc::c_double + 3.0f64
                    + 2.0f64 * k as libc::c_double));
        rhok = -ak * (1.0f64 + rhok) / (1.0f64 + ak * (1.0f64 + rhok));
        tk *= rhok;
        sum += tk;
        if fabs(tk / sum) < threshold {
            break;
        }
        k += 1;
        k;
    }
    *ratio = x / (2.0f64 * l as libc::c_double + 3.0f64) * sum;
    if k == kmax {
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"bessel_i.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int,
            GSL_EMAXITER as libc::c_int,
        );
        return GSL_EMAXITER as libc::c_int;
    } else {
        return GSL_SUCCESS as libc::c_int
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_i0_scaled_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut ax: libc::c_double = fabs(x);
    if x == 0.0f64 {
        (*result).val = 1.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if ax < 0.2f64 {
        let eax: libc::c_double = exp(-ax);
        let y: libc::c_double = ax * ax;
        let c1: libc::c_double = 1.0f64 / 6.0f64;
        let c2: libc::c_double = 1.0f64 / 120.0f64;
        let c3: libc::c_double = 1.0f64 / 5040.0f64;
        let c4: libc::c_double = 1.0f64 / 362880.0f64;
        let c5: libc::c_double = 1.0f64 / 39916800.0f64;
        let sum: libc::c_double = 1.0f64
            + y * (c1 + y * (c2 + y * (c3 + y * (c4 + y * c5))));
        (*result).val = eax * sum;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * (*result).val;
    } else if ax < -0.5f64 * -3.6043653389117154e+01f64 {
        (*result).val = (1.0f64 - exp(-2.0f64 * ax)) / (2.0f64 * ax);
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * (*result).val;
    } else {
        (*result).val = 1.0f64 / (2.0f64 * ax);
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * (*result).val;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sf_bessel_i1_scaled_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut ax: libc::c_double = fabs(x);
    if x == 0.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if ax < 3.0f64 * 2.2250738585072014e-308f64 {
        (*result).val = 0.0f64;
        (*result).err = 2.2250738585072014e-308f64;
        gsl_error(
            b"underflow\0" as *const u8 as *const libc::c_char,
            b"bessel_i.c\0" as *const u8 as *const libc::c_char,
            111 as libc::c_int,
            GSL_EUNDRFLW as libc::c_int,
        );
        return GSL_EUNDRFLW as libc::c_int;
    } else if ax < 0.25f64 {
        let eax: libc::c_double = exp(-ax);
        let y: libc::c_double = x * x;
        let c1: libc::c_double = 1.0f64 / 10.0f64;
        let c2: libc::c_double = 1.0f64 / 280.0f64;
        let c3: libc::c_double = 1.0f64 / 15120.0f64;
        let c4: libc::c_double = 1.0f64 / 1330560.0f64;
        let c5: libc::c_double = 1.0f64 / 172972800.0f64;
        let sum: libc::c_double = 1.0f64
            + y * (c1 + y * (c2 + y * (c3 + y * (c4 + y * c5))));
        (*result).val = eax * x / 3.0f64 * sum;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut ex: libc::c_double = exp(-2.0f64 * ax);
        (*result).val = 0.5f64 * (ax * (1.0f64 + ex) - (1.0f64 - ex)) / (ax * ax);
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        if x < 0.0f64 {
            (*result).val = -(*result).val;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_i2_scaled_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut ax: libc::c_double = fabs(x);
    if x == 0.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if ax < 4.0f64 * 1.4916681462400413e-154f64 {
        (*result).val = 0.0f64;
        (*result).err = 2.2250738585072014e-308f64;
        gsl_error(
            b"underflow\0" as *const u8 as *const libc::c_char,
            b"bessel_i.c\0" as *const u8 as *const libc::c_char,
            148 as libc::c_int,
            GSL_EUNDRFLW as libc::c_int,
        );
        return GSL_EUNDRFLW as libc::c_int;
    } else if ax < 0.25f64 {
        let y: libc::c_double = x * x;
        let c1: libc::c_double = 1.0f64 / 14.0f64;
        let c2: libc::c_double = 1.0f64 / 504.0f64;
        let c3: libc::c_double = 1.0f64 / 33264.0f64;
        let c4: libc::c_double = 1.0f64 / 3459456.0f64;
        let c5: libc::c_double = 1.0f64 / 518918400.0f64;
        let sum: libc::c_double = 1.0f64
            + y * (c1 + y * (c2 + y * (c3 + y * (c4 + y * c5))));
        let pre: libc::c_double = exp(-ax) * x * x / 15.0f64;
        (*result).val = pre * sum;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut ex: libc::c_double = exp(-2.0f64 * ax);
        let mut x2: libc::c_double = x * x;
        (*result)
            .val = 0.5f64 * ((3.0f64 + x2) * (1.0f64 - ex) - 3.0f64 * ax * (1.0f64 + ex))
            / (ax * ax * ax);
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_il_scaled_e(
    l: libc::c_int,
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut sgn: libc::c_double = 1.0f64;
    let mut ax: libc::c_double = fabs(x);
    if x < 0.0f64 {
        sgn = if l & 1 as libc::c_int != 0 { -1.0f64 } else { 1.0f64 };
        x = -x;
    }
    if l < 0 as libc::c_int {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"bessel_i.c\0" as *const u8 as *const libc::c_char,
            185 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x == 0.0f64 {
        (*result).val = if l == 0 as libc::c_int { 1.0f64 } else { 0.0f64 };
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if l == 0 as libc::c_int {
        let mut il: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_il: libc::c_int = gsl_sf_bessel_i0_scaled_e(x, &mut il);
        (*result).val = sgn * il.val;
        (*result).err = il.err;
        return stat_il;
    } else if l == 1 as libc::c_int {
        let mut il_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_il_0: libc::c_int = gsl_sf_bessel_i1_scaled_e(x, &mut il_0);
        (*result).val = sgn * il_0.val;
        (*result).err = il_0.err;
        return stat_il_0;
    } else if l == 2 as libc::c_int {
        let mut il_1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_il_1: libc::c_int = gsl_sf_bessel_i2_scaled_e(x, &mut il_1);
        (*result).val = sgn * il_1.val;
        (*result).err = il_1.err;
        return stat_il_1;
    } else if x * x < 10.0f64 * (l as libc::c_double + 1.5f64) / 2.7182818284590452354f64
    {
        let mut b: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat: libc::c_int = gsl_sf_bessel_IJ_taylor_e(
            l as libc::c_double + 0.5f64,
            x,
            1 as libc::c_int,
            50 as libc::c_int,
            2.2204460492503131e-16f64,
            &mut b,
        );
        let mut pre: libc::c_double = exp(-ax)
            * sqrt(0.5f64 * 3.14159265358979323846f64 / x);
        (*result).val = sgn * pre * b.val;
        (*result).err = pre * b.err;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return stat;
    } else if l < 150 as libc::c_int {
        let mut i0_scaled: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_i0: libc::c_int = gsl_sf_bessel_i0_scaled_e(ax, &mut i0_scaled);
        let mut rat: libc::c_double = 0.;
        let mut stat_CF1: libc::c_int = bessel_il_CF1(
            l,
            ax,
            2.2204460492503131e-16f64,
            &mut rat,
        );
        let mut iellp1: libc::c_double = rat * 1.4916681462400413e-154f64;
        let mut iell: libc::c_double = 1.4916681462400413e-154f64;
        let mut iellm1: libc::c_double = 0.;
        let mut ell: libc::c_int = 0;
        ell = l;
        while ell >= 1 as libc::c_int {
            iellm1 = iellp1
                + (2 as libc::c_int * ell + 1 as libc::c_int) as libc::c_double / x
                    * iell;
            iellp1 = iell;
            iell = iellm1;
            ell -= 1;
            ell;
        }
        (*result).val = sgn * i0_scaled.val * (1.4916681462400413e-154f64 / iell);
        (*result).err = i0_scaled.err * (1.4916681462400413e-154f64 / iell);
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return if stat_i0 != GSL_SUCCESS as libc::c_int {
            stat_i0
        } else if stat_CF1 != GSL_SUCCESS as libc::c_int {
            stat_CF1
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else if (if 0.29f64 / ((l * l) as libc::c_double + 1.0f64)
        < 0.5f64 / ((l * l) as libc::c_double + 1.0f64 + x * x)
    {
        0.29f64 / ((l * l) as libc::c_double + 1.0f64)
    } else {
        0.5f64 / ((l * l) as libc::c_double + 1.0f64 + x * x)
    }) < 0.5f64 * 6.0554544523933429e-06f64
    {
        let mut status: libc::c_int = gsl_sf_bessel_Inu_scaled_asymp_unif_e(
            l as libc::c_double + 0.5f64,
            x,
            result,
        );
        let mut pre_0: libc::c_double = sqrt(0.5f64 * 3.14159265358979323846f64 / x);
        (*result).val *= sgn * pre_0;
        (*result).err *= pre_0;
        return status;
    } else {
        let mut rt_term: libc::c_double = sqrt(0.5f64 * 3.14159265358979323846f64 / x);
        let LMAX: libc::c_int = 2 as libc::c_int
            + (1.2f64 / 2.4607833005759251e-03f64) as libc::c_int;
        let mut r_iellp1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut r_iell: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_a1: libc::c_int = gsl_sf_bessel_Inu_scaled_asymp_unif_e(
            (LMAX + 1 as libc::c_int) as libc::c_double + 0.5f64,
            x,
            &mut r_iellp1,
        );
        let mut stat_a2: libc::c_int = gsl_sf_bessel_Inu_scaled_asymp_unif_e(
            LMAX as libc::c_double + 0.5f64,
            x,
            &mut r_iell,
        );
        let mut iellp1_0: libc::c_double = r_iellp1.val;
        let mut iell_0: libc::c_double = r_iell.val;
        let mut iellm1_0: libc::c_double = 0.0f64;
        let mut ell_0: libc::c_int = 0;
        iellp1_0 *= rt_term;
        iell_0 *= rt_term;
        ell_0 = LMAX;
        while ell_0 >= l + 1 as libc::c_int {
            iellm1_0 = iellp1_0
                + (2 as libc::c_int * ell_0 + 1 as libc::c_int) as libc::c_double / x
                    * iell_0;
            iellp1_0 = iell_0;
            iell_0 = iellm1_0;
            ell_0 -= 1;
            ell_0;
        }
        (*result).val = sgn * iellm1_0;
        (*result)
            .err = fabs((*result).val)
            * (2.2204460492503131e-16f64 + fabs(r_iellp1.err / r_iellp1.val)
                + fabs(r_iell.err / r_iell.val));
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return if stat_a1 != GSL_SUCCESS as libc::c_int {
            stat_a1
        } else if stat_a2 != GSL_SUCCESS as libc::c_int {
            stat_a2
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_il_scaled_array(
    lmax: libc::c_int,
    x: libc::c_double,
    mut result_array: *mut libc::c_double,
) -> libc::c_int {
    if x == 0.0f64 {
        let mut ell: libc::c_int = 0;
        *result_array.offset(0 as libc::c_int as isize) = 1.0f64;
        ell = lmax;
        while ell >= 1 as libc::c_int {
            *result_array.offset(ell as isize) = 0.0f64;
            ell -= 1;
            ell;
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut ell_0: libc::c_int = 0;
        let mut r_iellp1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut r_iell: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_0: libc::c_int = gsl_sf_bessel_il_scaled_e(
            lmax + 1 as libc::c_int,
            x,
            &mut r_iellp1,
        );
        let mut stat_1: libc::c_int = gsl_sf_bessel_il_scaled_e(lmax, x, &mut r_iell);
        let mut iellp1: libc::c_double = r_iellp1.val;
        let mut iell: libc::c_double = r_iell.val;
        let mut iellm1: libc::c_double = 0.;
        *result_array.offset(lmax as isize) = iell;
        ell_0 = lmax;
        while ell_0 >= 1 as libc::c_int {
            iellm1 = iellp1
                + (2 as libc::c_int * ell_0 + 1 as libc::c_int) as libc::c_double / x
                    * iell;
            iellp1 = iell;
            iell = iellm1;
            *result_array.offset((ell_0 - 1 as libc::c_int) as isize) = iellm1;
            ell_0 -= 1;
            ell_0;
        }
        return if stat_0 != GSL_SUCCESS as libc::c_int {
            stat_0
        } else if stat_1 != GSL_SUCCESS as libc::c_int {
            stat_1
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_i0_scaled(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_i0_scaled_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_i0_scaled_e(x, &result)\0" as *const u8
                as *const libc::c_char,
            b"bessel_i.c\0" as *const u8 as *const libc::c_char,
            312 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_bessel_i1_scaled(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_i1_scaled_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_i1_scaled_e(x, &result)\0" as *const u8
                as *const libc::c_char,
            b"bessel_i.c\0" as *const u8 as *const libc::c_char,
            317 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_bessel_i2_scaled(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_i2_scaled_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_i2_scaled_e(x, &result)\0" as *const u8
                as *const libc::c_char,
            b"bessel_i.c\0" as *const u8 as *const libc::c_char,
            322 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_bessel_il_scaled(
    l: libc::c_int,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_il_scaled_e(l, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_il_scaled_e(l, x, &result)\0" as *const u8
                as *const libc::c_char,
            b"bessel_i.c\0" as *const u8 as *const libc::c_char,
            327 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
