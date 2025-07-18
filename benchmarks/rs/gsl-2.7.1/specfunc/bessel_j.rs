use ::libc;
extern "C" {
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn hypot(_: libc::c_double, _: libc::c_double) -> libc::c_double;
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
    fn gsl_sf_bessel_Jnu_asympx_e(
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
    fn gsl_sf_bessel_Jnu_asymp_Olver_e(
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
pub unsafe extern "C" fn gsl_sf_bessel_j0_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut ax: libc::c_double = fabs(x);
    if ax < 0.5f64 {
        let y: libc::c_double = x * x;
        let c1: libc::c_double = -1.0f64 / 6.0f64;
        let c2: libc::c_double = 1.0f64 / 120.0f64;
        let c3: libc::c_double = -1.0f64 / 5040.0f64;
        let c4: libc::c_double = 1.0f64 / 362880.0f64;
        let c5: libc::c_double = -1.0f64 / 39916800.0f64;
        let c6: libc::c_double = 1.0f64 / 6227020800.0f64;
        (*result)
            .val = 1.0f64
            + y * (c1 + y * (c2 + y * (c3 + y * (c4 + y * (c5 + y * c6)))));
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*result).val = sin(x) / x;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_j1_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut ax: libc::c_double = fabs(x);
    if x == 0.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if ax < 3.1f64 * 2.2250738585072014e-308f64 {
        (*result).val = 0.0f64;
        (*result).err = 2.2250738585072014e-308f64;
        gsl_error(
            b"underflow\0" as *const u8 as *const libc::c_char,
            b"bessel_j.c\0" as *const u8 as *const libc::c_char,
            74 as libc::c_int,
            GSL_EUNDRFLW as libc::c_int,
        );
        return GSL_EUNDRFLW as libc::c_int;
    } else if ax < 0.25f64 {
        let y: libc::c_double = x * x;
        let c1: libc::c_double = -1.0f64 / 10.0f64;
        let c2: libc::c_double = 1.0f64 / 280.0f64;
        let c3: libc::c_double = -1.0f64 / 15120.0f64;
        let c4: libc::c_double = 1.0f64 / 1330560.0f64;
        let c5: libc::c_double = -1.0f64 / 172972800.0f64;
        let sum: libc::c_double = 1.0f64
            + y * (c1 + y * (c2 + y * (c3 + y * (c4 + y * c5))));
        (*result).val = x / 3.0f64 * sum;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let cos_x: libc::c_double = cos(x);
        let sin_x: libc::c_double = sin(x);
        (*result).val = (sin_x / x - cos_x) / x;
        (*result)
            .err = 2.0f64 * 2.2204460492503131e-16f64
            * (fabs(sin_x / (x * x)) + fabs(cos_x / x));
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_j2_e(
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
            b"bessel_j.c\0" as *const u8 as *const libc::c_char,
            111 as libc::c_int,
            GSL_EUNDRFLW as libc::c_int,
        );
        return GSL_EUNDRFLW as libc::c_int;
    } else if ax < 1.3f64 {
        let y: libc::c_double = x * x;
        let c1: libc::c_double = -1.0f64 / 14.0f64;
        let c2: libc::c_double = 1.0f64 / 504.0f64;
        let c3: libc::c_double = -1.0f64 / 33264.0f64;
        let c4: libc::c_double = 1.0f64 / 3459456.0f64;
        let c5: libc::c_double = -1.0f64 / 518918400 as libc::c_int as libc::c_double;
        let c6: libc::c_double = 1.0f64 / 105859353600.0f64;
        let c7: libc::c_double = -1.0f64 / 28158588057600.0f64;
        let c8: libc::c_double = 1.0f64 / 9461285587353600.0f64;
        let c9: libc::c_double = -1.0f64 / 3916972233164390400.0f64;
        let sum: libc::c_double = 1.0f64
            + y
                * (c1
                    + y
                        * (c2
                            + y
                                * (c3
                                    + y
                                        * (c4
                                            + y * (c5 + y * (c6 + y * (c7 + y * (c8 + y * c9))))))));
        (*result).val = y / 15.0f64 * sum;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let cos_x: libc::c_double = cos(x);
        let sin_x: libc::c_double = sin(x);
        let f: libc::c_double = 3.0f64 / (x * x) - 1.0f64;
        (*result).val = (f * sin_x - 3.0f64 * cos_x / x) / x;
        (*result)
            .err = 2.0f64 * 2.2204460492503131e-16f64
            * (fabs(f * sin_x / x) + 3.0f64 * fabs(cos_x / (x * x)));
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_jl_e(
    l: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if l < 0 as libc::c_int || x < 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"bessel_j.c\0" as *const u8 as *const libc::c_char,
            162 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x == 0.0f64 {
        (*result).val = if l > 0 as libc::c_int { 0.0f64 } else { 1.0f64 };
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if l == 0 as libc::c_int {
        return gsl_sf_bessel_j0_e(x, result)
    } else if l == 1 as libc::c_int {
        return gsl_sf_bessel_j1_e(x, result)
    } else if l == 2 as libc::c_int {
        return gsl_sf_bessel_j2_e(x, result)
    } else if x * x < 10.0f64 * (l as libc::c_double + 0.5f64) / 2.7182818284590452354f64
    {
        let mut b: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut status: libc::c_int = gsl_sf_bessel_IJ_taylor_e(
            l as libc::c_double + 0.5f64,
            x,
            -(1 as libc::c_int),
            50 as libc::c_int,
            2.2204460492503131e-16f64,
            &mut b,
        );
        let mut pre: libc::c_double = sqrt(0.5f64 * 3.14159265358979323846f64 / x);
        (*result).val = pre * b.val;
        (*result).err = pre * b.err;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return status;
    } else if 1.2207031250000000e-04f64 * x > (l * l + l) as libc::c_double + 1.0f64 {
        let mut b_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut status_0: libc::c_int = gsl_sf_bessel_Jnu_asympx_e(
            l as libc::c_double + 0.5f64,
            x,
            &mut b_0,
        );
        let mut pre_0: libc::c_double = sqrt(0.5f64 * 3.14159265358979323846f64 / x);
        (*result).val = pre_0 * b_0.val;
        (*result)
            .err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val)
            + pre_0 * b_0.err;
        return status_0;
    } else if l as libc::c_double > 1.0f64 / 2.4607833005759251e-03f64 {
        let mut b_1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut status_1: libc::c_int = gsl_sf_bessel_Jnu_asymp_Olver_e(
            l as libc::c_double + 0.5f64,
            x,
            &mut b_1,
        );
        let mut pre_1: libc::c_double = sqrt(0.5f64 * 3.14159265358979323846f64 / x);
        (*result).val = pre_1 * b_1.val;
        (*result)
            .err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val)
            + pre_1 * b_1.err;
        return status_1;
    } else if x > 1000.0f64 && x > (l * l) as libc::c_double {
        let mut b_2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut status_2: libc::c_int = gsl_sf_bessel_Jnu_asympx_e(
            l as libc::c_double + 0.5f64,
            x,
            &mut b_2,
        );
        let mut pre_2: libc::c_double = sqrt(0.5f64 * 3.14159265358979323846f64 / x);
        (*result).val = pre_2 * b_2.val;
        (*result)
            .err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val)
            + pre_2 * b_2.err;
        return status_2;
    } else {
        let mut sgn: libc::c_double = 0.;
        let mut ratio: libc::c_double = 0.;
        let mut stat_CF1: libc::c_int = gsl_sf_bessel_J_CF1(
            l as libc::c_double + 0.5f64,
            x,
            &mut ratio,
            &mut sgn,
        );
        let BESSEL_J_SMALL: libc::c_double = 2.2250738585072014e-308f64
            / 2.2204460492503131e-16f64;
        let mut jellp1: libc::c_double = BESSEL_J_SMALL * ratio;
        let mut jell: libc::c_double = BESSEL_J_SMALL;
        let mut jellm1: libc::c_double = 0.;
        let mut ell: libc::c_int = 0;
        ell = l;
        while ell > 0 as libc::c_int {
            jellm1 = -jellp1
                + (2 as libc::c_int * ell + 1 as libc::c_int) as libc::c_double / x
                    * jell;
            jellp1 = jell;
            jell = jellm1;
            ell -= 1;
            ell;
        }
        if fabs(jell) > fabs(jellp1) {
            let mut j0_result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut stat_j0: libc::c_int = gsl_sf_bessel_j0_e(x, &mut j0_result);
            let mut pre_3: libc::c_double = BESSEL_J_SMALL / jell;
            (*result).val = j0_result.val * pre_3;
            (*result).err = j0_result.err * fabs(pre_3);
            (*result).err
                += 4.0f64 * 2.2204460492503131e-16f64
                    * (0.5f64 * l as libc::c_double + 1.0f64) * fabs((*result).val);
            return if stat_j0 != GSL_SUCCESS as libc::c_int {
                stat_j0
            } else if stat_CF1 != GSL_SUCCESS as libc::c_int {
                stat_CF1
            } else {
                GSL_SUCCESS as libc::c_int
            };
        } else {
            let mut j1_result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut stat_j1: libc::c_int = gsl_sf_bessel_j1_e(x, &mut j1_result);
            let mut pre_4: libc::c_double = BESSEL_J_SMALL / jellp1;
            (*result).val = j1_result.val * pre_4;
            (*result).err = j1_result.err * fabs(pre_4);
            (*result).err
                += 4.0f64 * 2.2204460492503131e-16f64
                    * (0.5f64 * l as libc::c_double + 1.0f64) * fabs((*result).val);
            return if stat_j1 != GSL_SUCCESS as libc::c_int {
                stat_j1
            } else if stat_CF1 != GSL_SUCCESS as libc::c_int {
                stat_CF1
            } else {
                GSL_SUCCESS as libc::c_int
            };
        }
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_jl_array(
    lmax: libc::c_int,
    x: libc::c_double,
    mut result_array: *mut libc::c_double,
) -> libc::c_int {
    if lmax < 0 as libc::c_int || x < 0.0f64 {
        let mut j: libc::c_int = 0;
        j = 0 as libc::c_int;
        while j <= lmax {
            *result_array.offset(j as isize) = 0.0f64;
            j += 1;
            j;
        }
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"bessel_j.c\0" as *const u8 as *const libc::c_char,
            259 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x == 0.0f64 {
        let mut j_0: libc::c_int = 0;
        j_0 = 1 as libc::c_int;
        while j_0 <= lmax {
            *result_array.offset(j_0 as isize) = 0.0f64;
            j_0 += 1;
            j_0;
        }
        *result_array.offset(0 as libc::c_int as isize) = 1.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut r_jellp1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut r_jell: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_0: libc::c_int = gsl_sf_bessel_jl_e(
            lmax + 1 as libc::c_int,
            x,
            &mut r_jellp1,
        );
        let mut stat_1: libc::c_int = gsl_sf_bessel_jl_e(lmax, x, &mut r_jell);
        let mut jellp1: libc::c_double = r_jellp1.val;
        let mut jell: libc::c_double = r_jell.val;
        let mut jellm1: libc::c_double = 0.;
        let mut ell: libc::c_int = 0;
        *result_array.offset(lmax as isize) = jell;
        ell = lmax;
        while ell >= 1 as libc::c_int {
            jellm1 = -jellp1
                + (2 as libc::c_int * ell + 1 as libc::c_int) as libc::c_double / x
                    * jell;
            jellp1 = jell;
            jell = jellm1;
            *result_array.offset((ell - 1 as libc::c_int) as isize) = jellm1;
            ell -= 1;
            ell;
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
pub unsafe extern "C" fn gsl_sf_bessel_jl_steed_array(
    lmax: libc::c_int,
    x: libc::c_double,
    mut jl_x: *mut libc::c_double,
) -> libc::c_int {
    if lmax < 0 as libc::c_int || x < 0.0f64 {
        let mut j: libc::c_int = 0;
        j = 0 as libc::c_int;
        while j <= lmax {
            *jl_x.offset(j as isize) = 0.0f64;
            j += 1;
            j;
        }
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"bessel_j.c\0" as *const u8 as *const libc::c_char,
            297 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x == 0.0f64 {
        let mut j_0: libc::c_int = 0;
        j_0 = 1 as libc::c_int;
        while j_0 <= lmax {
            *jl_x.offset(j_0 as isize) = 0.0f64;
            j_0 += 1;
            j_0;
        }
        *jl_x.offset(0 as libc::c_int as isize) = 1.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if x < 2.0f64 * 1.2207031250000000e-04f64 {
        let mut inv_fact: libc::c_double = 1.0f64;
        let mut x_l: libc::c_double = 1.0f64;
        let mut l: libc::c_int = 0;
        l = 0 as libc::c_int;
        while l <= lmax {
            *jl_x.offset(l as isize) = x_l * inv_fact;
            *jl_x.offset(l as isize)
                *= 1.0f64 - 0.5f64 * x * x / (2.0f64 * l as libc::c_double + 3.0f64);
            inv_fact /= 2.0f64 * l as libc::c_double + 3.0f64;
            x_l *= x;
            l += 1;
            l;
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut x_inv: libc::c_double = 1.0f64 / x;
        let mut W: libc::c_double = 2.0f64 * x_inv;
        let mut F: libc::c_double = 1.0f64;
        let mut FP: libc::c_double = (lmax as libc::c_double + 1.0f64) * x_inv;
        let mut B: libc::c_double = 2.0f64 * FP + x_inv;
        let mut end: libc::c_double = B + 20000.0f64 * W;
        let mut D: libc::c_double = 1.0f64 / B;
        let mut del: libc::c_double = -D;
        FP += del;
        loop {
            B += W;
            D = 1.0f64 / (B - D);
            del *= B * D - 1.0f64;
            FP += del;
            if D < 0.0f64 {
                F = -F;
            }
            if B > end {
                gsl_error(
                    b"error\0" as *const u8 as *const libc::c_char,
                    b"bessel_j.c\0" as *const u8 as *const libc::c_char,
                    339 as libc::c_int,
                    GSL_EMAXITER as libc::c_int,
                );
                return GSL_EMAXITER as libc::c_int;
            }
            if !(fabs(del) >= fabs(FP) * 2.2204460492503131e-16f64) {
                break;
            }
        }
        FP *= F;
        if lmax > 0 as libc::c_int {
            let mut XP2: libc::c_double = FP;
            let mut PL: libc::c_double = lmax as libc::c_double * x_inv;
            let mut L: libc::c_int = lmax;
            let mut LP: libc::c_int = 0;
            *jl_x.offset(lmax as isize) = F;
            LP = 1 as libc::c_int;
            while LP <= lmax {
                *jl_x
                    .offset(
                        (L - 1 as libc::c_int) as isize,
                    ) = PL * *jl_x.offset(L as isize) + XP2;
                FP = PL * *jl_x.offset((L - 1 as libc::c_int) as isize)
                    - *jl_x.offset(L as isize);
                XP2 = FP;
                PL -= x_inv;
                L -= 1;
                L;
                LP += 1;
                LP;
            }
            F = *jl_x.offset(0 as libc::c_int as isize);
        }
        W = x_inv / hypot(FP, F);
        *jl_x.offset(0 as libc::c_int as isize) = W * F;
        if lmax > 0 as libc::c_int {
            let mut L_0: libc::c_int = 0;
            L_0 = 1 as libc::c_int;
            while L_0 <= lmax {
                *jl_x.offset(L_0 as isize) *= W;
                L_0 += 1;
                L_0;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_j0(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_j0_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_j0_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"bessel_j.c\0" as *const u8 as *const libc::c_char,
            384 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_bessel_j1(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_j1_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_j1_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"bessel_j.c\0" as *const u8 as *const libc::c_char,
            389 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_bessel_j2(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_j2_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_j2_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"bessel_j.c\0" as *const u8 as *const libc::c_char,
            394 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_bessel_jl(
    l: libc::c_int,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_jl_e(l, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_jl_e(l, x, &result)\0" as *const u8 as *const libc::c_char,
            b"bessel_j.c\0" as *const u8 as *const libc::c_char,
            399 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
