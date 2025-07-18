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
    fn gsl_sf_pow_int(x: libc::c_double, n: libc::c_int) -> libc::c_double;
    fn gsl_sf_doublefact_e(n: libc::c_uint, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_bessel_il_scaled_e(
        l: libc::c_int,
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_bessel_Knu_scaled_asympx_e(
        nu: libc::c_double,
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_bessel_Knu_scaled_asymp_unif_e(
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
unsafe extern "C" fn bessel_kl_scaled_small_x(
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
            b"bessel_k.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_EOVRFLW as libc::c_int,
        );
        return GSL_EOVRFLW as libc::c_int;
    } else {
        let lmax: libc::c_int = 50 as libc::c_int;
        let mut ipos_term: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut ineg_term: libc::c_double = 0.;
        let mut sgn: libc::c_double = if l & 1 as libc::c_int != 0 {
            -1.0f64
        } else {
            1.0f64
        };
        let mut ex: libc::c_double = exp(x);
        let mut t: libc::c_double = 0.5f64 * x * x;
        let mut sum: libc::c_double = 1.0f64;
        let mut t_coeff: libc::c_double = 1.0f64;
        let mut t_power: libc::c_double = 1.0f64;
        let mut delta: libc::c_double = 0.;
        let mut stat_il: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        i = 1 as libc::c_int;
        while i < lmax {
            t_coeff
                /= (i * (2 as libc::c_int * (i - l) - 1 as libc::c_int))
                    as libc::c_double;
            t_power *= t;
            delta = t_power * t_coeff;
            sum += delta;
            if fabs(delta / sum) < 2.2204460492503131e-16f64 {
                break;
            }
            i += 1;
            i;
        }
        stat_il = gsl_sf_bessel_il_scaled_e(l, x, &mut ipos_term);
        ineg_term = sgn * num_fact.val / den * sum;
        (*result)
            .val = -sgn * 0.5f64 * 3.14159265358979323846f64
            * (ex * ipos_term.val - ineg_term);
        (*result).val *= ex;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return stat_il;
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_k0_scaled_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x <= 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"bessel_k.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        (*result).val = 3.14159265358979323846f64 / (2.0f64 * x);
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        if fabs((*result).val) < 2.2250738585072014e-308f64 {
            gsl_error(
                b"underflow\0" as *const u8 as *const libc::c_char,
                b"bessel_k.c\0" as *const u8 as *const libc::c_char,
                94 as libc::c_int,
                GSL_EUNDRFLW as libc::c_int,
            );
            return GSL_EUNDRFLW as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_k1_scaled_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x <= 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"bessel_k.c\0" as *const u8 as *const libc::c_char,
            105 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x
        < (1.77245385090551602729816748334f64 + 1.0f64)
            / (1.41421356237309504880f64 * 1.3407807929942596e+154f64)
    {
        (*result).val = ::std::f32::INFINITY as libc::c_double;
        (*result).err = ::std::f32::INFINITY as libc::c_double;
        gsl_error(
            b"overflow\0" as *const u8 as *const libc::c_char,
            b"bessel_k.c\0" as *const u8 as *const libc::c_char,
            108 as libc::c_int,
            GSL_EOVRFLW as libc::c_int,
        );
        return GSL_EOVRFLW as libc::c_int;
    } else {
        (*result).val = 3.14159265358979323846f64 / (2.0f64 * x) * (1.0f64 + 1.0f64 / x);
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        if fabs((*result).val) < 2.2250738585072014e-308f64 {
            gsl_error(
                b"underflow\0" as *const u8 as *const libc::c_char,
                b"bessel_k.c\0" as *const u8 as *const libc::c_char,
                113 as libc::c_int,
                GSL_EUNDRFLW as libc::c_int,
            );
            return GSL_EUNDRFLW as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_k2_scaled_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x <= 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"bessel_k.c\0" as *const u8 as *const libc::c_char,
            124 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x < 2.0f64 / 5.6438030941222897e+102f64 {
        (*result).val = ::std::f32::INFINITY as libc::c_double;
        (*result).err = ::std::f32::INFINITY as libc::c_double;
        gsl_error(
            b"overflow\0" as *const u8 as *const libc::c_char,
            b"bessel_k.c\0" as *const u8 as *const libc::c_char,
            127 as libc::c_int,
            GSL_EOVRFLW as libc::c_int,
        );
        return GSL_EOVRFLW as libc::c_int;
    } else {
        (*result)
            .val = 3.14159265358979323846f64 / (2.0f64 * x)
            * (1.0f64 + 3.0f64 / x * (1.0f64 + 1.0f64 / x));
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        if fabs((*result).val) < 2.2250738585072014e-308f64 {
            gsl_error(
                b"underflow\0" as *const u8 as *const libc::c_char,
                b"bessel_k.c\0" as *const u8 as *const libc::c_char,
                132 as libc::c_int,
                GSL_EUNDRFLW as libc::c_int,
            );
            return GSL_EUNDRFLW as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_kl_scaled_e(
    mut l: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if l < 0 as libc::c_int || x <= 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"bessel_k.c\0" as *const u8 as *const libc::c_char,
            141 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if l == 0 as libc::c_int {
        return gsl_sf_bessel_k0_scaled_e(x, result)
    } else if l == 1 as libc::c_int {
        return gsl_sf_bessel_k1_scaled_e(x, result)
    } else if l == 2 as libc::c_int {
        return gsl_sf_bessel_k2_scaled_e(x, result)
    } else if x < 3.0f64 {
        return bessel_kl_scaled_small_x(l, x, result)
    } else if 6.0554544523933429e-06f64 * x
        > (l * l + l + 1 as libc::c_int) as libc::c_double
    {
        let mut status: libc::c_int = gsl_sf_bessel_Knu_scaled_asympx_e(
            l as libc::c_double + 0.5f64,
            x,
            result,
        );
        let mut pre: libc::c_double = sqrt(0.5f64 * 3.14159265358979323846f64 / x);
        (*result).val *= pre;
        (*result).err *= pre;
        return status;
    } else if (if 0.29f64 / ((l * l) as libc::c_double + 1.0f64)
        < 0.5f64 / ((l * l) as libc::c_double + 1.0f64 + x * x)
    {
        0.29f64 / ((l * l) as libc::c_double + 1.0f64)
    } else {
        0.5f64 / ((l * l) as libc::c_double + 1.0f64 + x * x)
    }) < 6.0554544523933429e-06f64
    {
        let mut status_0: libc::c_int = gsl_sf_bessel_Knu_scaled_asymp_unif_e(
            l as libc::c_double + 0.5f64,
            x,
            result,
        );
        let mut pre_0: libc::c_double = sqrt(0.5f64 * 3.14159265358979323846f64 / x);
        (*result).val *= pre_0;
        (*result).err *= pre_0;
        return status_0;
    } else {
        let mut r_bk: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut r_bkm: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_1: libc::c_int = gsl_sf_bessel_k1_scaled_e(x, &mut r_bk);
        let mut stat_0: libc::c_int = gsl_sf_bessel_k0_scaled_e(x, &mut r_bkm);
        let mut bkp: libc::c_double = 0.;
        let mut bk: libc::c_double = r_bk.val;
        let mut bkm: libc::c_double = r_bkm.val;
        let mut j: libc::c_int = 0;
        j = 1 as libc::c_int;
        while j < l {
            bkp = (2 as libc::c_int * j + 1 as libc::c_int) as libc::c_double / x * bk
                + bkm;
            bkm = bk;
            bk = bkp;
            j += 1;
            j;
        }
        (*result).val = bk;
        (*result)
            .err = fabs(bk) * (fabs(r_bk.err / r_bk.val) + fabs(r_bkm.err / r_bkm.val));
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return if stat_1 != GSL_SUCCESS as libc::c_int {
            stat_1
        } else if stat_0 != GSL_SUCCESS as libc::c_int {
            stat_0
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_kl_scaled_array(
    lmax: libc::c_int,
    x: libc::c_double,
    mut result_array: *mut libc::c_double,
) -> libc::c_int {
    if lmax < 0 as libc::c_int || x <= 0.0f64 {
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"bessel_k.c\0" as *const u8 as *const libc::c_char,
            196 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if lmax == 0 as libc::c_int {
        let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat: libc::c_int = gsl_sf_bessel_k0_scaled_e(x, &mut result);
        *result_array.offset(0 as libc::c_int as isize) = result.val;
        return stat;
    } else {
        let mut ell: libc::c_int = 0;
        let mut kellp1: libc::c_double = 0.;
        let mut kell: libc::c_double = 0.;
        let mut kellm1: libc::c_double = 0.;
        let mut r_kell: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut r_kellm1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        gsl_sf_bessel_k1_scaled_e(x, &mut r_kell);
        gsl_sf_bessel_k0_scaled_e(x, &mut r_kellm1);
        kell = r_kell.val;
        kellm1 = r_kellm1.val;
        *result_array.offset(0 as libc::c_int as isize) = kellm1;
        *result_array.offset(1 as libc::c_int as isize) = kell;
        ell = 1 as libc::c_int;
        while ell < lmax {
            kellp1 = (2 as libc::c_int * ell + 1 as libc::c_int) as libc::c_double / x
                * kell + kellm1;
            *result_array.offset((ell + 1 as libc::c_int) as isize) = kellp1;
            kellm1 = kell;
            kell = kellp1;
            ell += 1;
            ell;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_k0_scaled(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_k0_scaled_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_k0_scaled_e(x, &result)\0" as *const u8
                as *const libc::c_char,
            b"bessel_k.c\0" as *const u8 as *const libc::c_char,
            230 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_bessel_k1_scaled(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_k1_scaled_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_k1_scaled_e(x, &result)\0" as *const u8
                as *const libc::c_char,
            b"bessel_k.c\0" as *const u8 as *const libc::c_char,
            235 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_bessel_k2_scaled(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_k2_scaled_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_k2_scaled_e(x, &result)\0" as *const u8
                as *const libc::c_char,
            b"bessel_k.c\0" as *const u8 as *const libc::c_char,
            240 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_bessel_kl_scaled(
    l: libc::c_int,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_kl_scaled_e(l, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_kl_scaled_e(l, x, &result)\0" as *const u8
                as *const libc::c_char,
            b"bessel_k.c\0" as *const u8 as *const libc::c_char,
            245 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
