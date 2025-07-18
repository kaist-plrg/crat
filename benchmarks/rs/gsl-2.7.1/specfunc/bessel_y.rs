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
    fn gsl_sf_pow_int(x: libc::c_double, n: libc::c_int) -> libc::c_double;
    fn gsl_sf_doublefact_e(n: libc::c_uint, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_sin_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_cos_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_bessel_Ynu_asympx_e(
        nu: libc::c_double,
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_bessel_Ynu_asymp_Olver_e(
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
unsafe extern "C" fn bessel_yl_small_x(
    mut l: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut num_fact: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut den: libc::c_double = gsl_sf_pow_int(x, l + 1 as libc::c_int);
    let mut stat_df: libc::c_int = gsl_sf_doublefact_e(
        (2 as libc::c_int * l - 1 as libc::c_int) as libc::c_uint,
        &mut num_fact,
    );
    if stat_df != GSL_SUCCESS as libc::c_int || den == 0.0f64 {
        (*result).val = ::std::f32::INFINITY as libc::c_double;
        (*result).err = ::std::f32::INFINITY as libc::c_double;
        gsl_error(
            b"overflow\0" as *const u8 as *const libc::c_char,
            b"bessel_y.c\0" as *const u8 as *const libc::c_char,
            49 as libc::c_int,
            GSL_EOVRFLW as libc::c_int,
        );
        return GSL_EOVRFLW as libc::c_int;
    } else {
        let lmax: libc::c_int = 200 as libc::c_int;
        let mut t: libc::c_double = -0.5f64 * x * x;
        let mut sum: libc::c_double = 1.0f64;
        let mut t_coeff: libc::c_double = 1.0f64;
        let mut t_power: libc::c_double = 1.0f64;
        let mut delta: libc::c_double = 0.;
        let mut i: libc::c_int = 0;
        i = 1 as libc::c_int;
        while i <= lmax {
            t_coeff
                /= (i * (2 as libc::c_int * (i - l) - 1 as libc::c_int))
                    as libc::c_double;
            t_power *= t;
            delta = t_power * t_coeff;
            sum += delta;
            if fabs(delta / sum) < 0.5f64 * 2.2204460492503131e-16f64 {
                break;
            }
            i += 1;
            i;
        }
        (*result).val = -num_fact.val / den * sum;
        (*result).err = 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_y0_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x <= 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"bessel_y.c\0" as *const u8 as *const libc::c_char,
            82 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if 1.0f64 / 1.7976931348623157e+308f64 > 0.0f64
        && x < 1.0f64 / 1.7976931348623157e+308f64
    {
        (*result).val = ::std::f32::INFINITY as libc::c_double;
        (*result).err = ::std::f32::INFINITY as libc::c_double;
        gsl_error(
            b"overflow\0" as *const u8 as *const libc::c_char,
            b"bessel_y.c\0" as *const u8 as *const libc::c_char,
            85 as libc::c_int,
            GSL_EOVRFLW as libc::c_int,
        );
        return GSL_EOVRFLW as libc::c_int;
    } else {
        let mut cos_result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let stat: libc::c_int = gsl_sf_cos_e(x, &mut cos_result);
        (*result).val = -cos_result.val / x;
        (*result).err = fabs(cos_result.err / x);
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return stat;
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_y1_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x <= 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"bessel_y.c\0" as *const u8 as *const libc::c_char,
            103 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x < 1.0f64 / 1.3407807929942596e+154f64 {
        (*result).val = ::std::f32::INFINITY as libc::c_double;
        (*result).err = ::std::f32::INFINITY as libc::c_double;
        gsl_error(
            b"overflow\0" as *const u8 as *const libc::c_char,
            b"bessel_y.c\0" as *const u8 as *const libc::c_char,
            106 as libc::c_int,
            GSL_EOVRFLW as libc::c_int,
        );
        return GSL_EOVRFLW as libc::c_int;
    } else if x < 0.25f64 {
        let y: libc::c_double = x * x;
        let c1: libc::c_double = 1.0f64 / 2.0f64;
        let c2: libc::c_double = -1.0f64 / 8.0f64;
        let c3: libc::c_double = 1.0f64 / 144.0f64;
        let c4: libc::c_double = -1.0f64 / 5760.0f64;
        let c5: libc::c_double = 1.0f64 / 403200.0f64;
        let c6: libc::c_double = -1.0f64 / 43545600.0f64;
        let sum: libc::c_double = 1.0f64
            + y * (c1 + y * (c2 + y * (c3 + y * (c4 + y * (c5 + y * c6)))));
        (*result).val = -sum / y;
        (*result).err = 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut cos_result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut sin_result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let stat_cos: libc::c_int = gsl_sf_cos_e(x, &mut cos_result);
        let stat_sin: libc::c_int = gsl_sf_sin_e(x, &mut sin_result);
        let cx: libc::c_double = cos_result.val;
        let sx: libc::c_double = sin_result.val;
        (*result).val = -(cx / x + sx) / x;
        (*result).err = (fabs(cos_result.err / x) + sin_result.err) / fabs(x);
        (*result).err += 2.2204460492503131e-16f64 * (fabs(sx / x) + fabs(cx / (x * x)));
        return if stat_cos != GSL_SUCCESS as libc::c_int {
            stat_cos
        } else if stat_sin != GSL_SUCCESS as libc::c_int {
            stat_sin
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_y2_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x <= 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"bessel_y.c\0" as *const u8 as *const libc::c_char,
            141 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x < 1.0f64 / 5.6438030941222897e+102f64 {
        (*result).val = ::std::f32::INFINITY as libc::c_double;
        (*result).err = ::std::f32::INFINITY as libc::c_double;
        gsl_error(
            b"overflow\0" as *const u8 as *const libc::c_char,
            b"bessel_y.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            GSL_EOVRFLW as libc::c_int,
        );
        return GSL_EOVRFLW as libc::c_int;
    } else if x < 0.5f64 {
        let y: libc::c_double = x * x;
        let c1: libc::c_double = 1.0f64 / 6.0f64;
        let c2: libc::c_double = 1.0f64 / 24.0f64;
        let c3: libc::c_double = -1.0f64 / 144.0f64;
        let c4: libc::c_double = 1.0f64 / 3456.0f64;
        let c5: libc::c_double = -1.0f64 / 172800.0f64;
        let c6: libc::c_double = 1.0f64 / 14515200.0f64;
        let c7: libc::c_double = -1.0f64 / 1828915200.0f64;
        let sum: libc::c_double = 1.0f64
            + y * (c1 + y * (c2 + y * (c3 + y * (c4 + y * (c5 + y * (c6 + y * c7))))));
        (*result).val = -3.0f64 / (x * x * x) * sum;
        (*result).err = 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut cos_result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut sin_result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let stat_cos: libc::c_int = gsl_sf_cos_e(x, &mut cos_result);
        let stat_sin: libc::c_int = gsl_sf_sin_e(x, &mut sin_result);
        let sx: libc::c_double = sin_result.val;
        let cx: libc::c_double = cos_result.val;
        let a: libc::c_double = 3.0f64 / (x * x);
        (*result).val = (1.0f64 - a) / x * cx - a * sx;
        (*result)
            .err = cos_result.err * fabs((1.0f64 - a) / x) + sin_result.err * fabs(a);
        (*result).err += 2.2204460492503131e-16f64 * (fabs(cx / x) + fabs(sx / (x * x)));
        return if stat_cos != GSL_SUCCESS as libc::c_int {
            stat_cos
        } else if stat_sin != GSL_SUCCESS as libc::c_int {
            stat_sin
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_yl_e(
    mut l: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if l < 0 as libc::c_int || x <= 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"bessel_y.c\0" as *const u8 as *const libc::c_char,
            181 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if l == 0 as libc::c_int {
        return gsl_sf_bessel_y0_e(x, result)
    } else if l == 1 as libc::c_int {
        return gsl_sf_bessel_y1_e(x, result)
    } else if l == 2 as libc::c_int {
        return gsl_sf_bessel_y2_e(x, result)
    } else if x < 3.0f64 {
        return bessel_yl_small_x(l, x, result)
    } else if 6.0554544523933429e-06f64 * x > (l * l + l) as libc::c_double + 1.0f64 {
        let mut status: libc::c_int = gsl_sf_bessel_Ynu_asympx_e(
            l as libc::c_double + 0.5f64,
            x,
            result,
        );
        let mut pre: libc::c_double = sqrt(0.5f64 * 3.14159265358979323846f64 / x);
        (*result).val *= pre;
        (*result).err *= pre;
        return status;
    } else if l > 40 as libc::c_int {
        let mut status_0: libc::c_int = gsl_sf_bessel_Ynu_asymp_Olver_e(
            l as libc::c_double + 0.5f64,
            x,
            result,
        );
        let mut pre_0: libc::c_double = sqrt(0.5f64 * 3.14159265358979323846f64 / x);
        (*result).val *= pre_0;
        (*result).err *= pre_0;
        return status_0;
    } else {
        let mut r_by: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut r_bym: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_1: libc::c_int = gsl_sf_bessel_y1_e(x, &mut r_by);
        let mut stat_0: libc::c_int = gsl_sf_bessel_y0_e(x, &mut r_bym);
        let mut bym: libc::c_double = r_bym.val;
        let mut by: libc::c_double = r_by.val;
        let mut byp: libc::c_double = 0.;
        let mut j: libc::c_int = 0;
        j = 1 as libc::c_int;
        while j < l {
            byp = (2 as libc::c_int * j + 1 as libc::c_int) as libc::c_double / x * by
                - bym;
            bym = by;
            by = byp;
            j += 1;
            j;
        }
        (*result).val = by;
        (*result)
            .err = fabs((*result).val)
            * (2.2204460492503131e-16f64 + fabs(r_by.err / r_by.val)
                + fabs(r_bym.err / r_bym.val));
        return if stat_1 != GSL_SUCCESS as libc::c_int {
            stat_1
        } else if stat_0 != GSL_SUCCESS as libc::c_int {
            stat_0
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_yl_array(
    lmax: libc::c_int,
    x: libc::c_double,
    mut result_array: *mut libc::c_double,
) -> libc::c_int {
    if lmax < 0 as libc::c_int || x <= 0.0f64 {
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"bessel_y.c\0" as *const u8 as *const libc::c_char,
            237 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if lmax == 0 as libc::c_int {
        let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat: libc::c_int = gsl_sf_bessel_y0_e(x, &mut result);
        *result_array.offset(0 as libc::c_int as isize) = result.val;
        return stat;
    } else {
        let mut r_yell: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut r_yellm1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_1: libc::c_int = gsl_sf_bessel_y1_e(x, &mut r_yell);
        let mut stat_0: libc::c_int = gsl_sf_bessel_y0_e(x, &mut r_yellm1);
        let mut yellp1: libc::c_double = 0.;
        let mut yell: libc::c_double = r_yell.val;
        let mut yellm1: libc::c_double = r_yellm1.val;
        let mut ell: libc::c_int = 0;
        *result_array.offset(0 as libc::c_int as isize) = yellm1;
        *result_array.offset(1 as libc::c_int as isize) = yell;
        ell = 1 as libc::c_int;
        while ell < lmax {
            yellp1 = (2 as libc::c_int * ell + 1 as libc::c_int) as libc::c_double / x
                * yell - yellm1;
            *result_array.offset((ell + 1 as libc::c_int) as isize) = yellp1;
            yellm1 = yell;
            yell = yellp1;
            ell += 1;
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
pub unsafe extern "C" fn gsl_sf_bessel_y0(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_y0_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_y0_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"bessel_y.c\0" as *const u8 as *const libc::c_char,
            274 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_bessel_y1(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_y1_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_y1_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"bessel_y.c\0" as *const u8 as *const libc::c_char,
            279 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_bessel_y2(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_y2_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_y2_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"bessel_y.c\0" as *const u8 as *const libc::c_char,
            284 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_bessel_yl(
    l: libc::c_int,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_yl_e(l, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_yl_e(l, x, &result)\0" as *const u8 as *const libc::c_char,
            b"bessel_y.c\0" as *const u8 as *const libc::c_char,
            289 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
