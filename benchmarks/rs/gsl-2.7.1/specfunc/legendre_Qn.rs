use ::libc;
extern "C" {
    fn acos(_: libc::c_double) -> libc::c_double;
    fn acosh(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sf_bessel_Y0_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_bessel_Y1_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_bessel_K0_scaled_e(
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_bessel_K1_scaled_e(
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_multiply_e(
        x: libc::c_double,
        y: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_exp_mult_e(
        x: libc::c_double,
        y: libc::c_double,
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
unsafe extern "C" fn legendreQ_CF1_xgt1(
    mut ell: libc::c_int,
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut x: libc::c_double,
    mut result: *mut libc::c_double,
) -> libc::c_int {
    let RECUR_BIG: libc::c_double = 1.3407807929942596e+154f64;
    let maxiter: libc::c_int = 5000 as libc::c_int;
    let mut n: libc::c_int = 1 as libc::c_int;
    let mut Anm2: libc::c_double = 1.0f64;
    let mut Bnm2: libc::c_double = 0.0f64;
    let mut Anm1: libc::c_double = 0.0f64;
    let mut Bnm1: libc::c_double = 1.0f64;
    let mut a1: libc::c_double = ell as libc::c_double + 1.0f64 + a + b;
    let mut b1: libc::c_double = (2.0f64 * (ell as libc::c_double + 1.0f64 + a) + 1.0f64)
        * x;
    let mut An: libc::c_double = b1 * Anm1 + a1 * Anm2;
    let mut Bn: libc::c_double = b1 * Bnm1 + a1 * Bnm2;
    let mut an: libc::c_double = 0.;
    let mut bn: libc::c_double = 0.;
    let mut fn_0: libc::c_double = An / Bn;
    while n < maxiter {
        let mut old_fn: libc::c_double = 0.;
        let mut del: libc::c_double = 0.;
        let mut lna: libc::c_double = 0.;
        n += 1;
        n;
        Anm2 = Anm1;
        Bnm2 = Bnm1;
        Anm1 = An;
        Bnm1 = Bn;
        lna = (ell + n) as libc::c_double + a;
        an = b * b - lna * lna;
        bn = (2.0f64 * lna + 1.0f64) * x;
        An = bn * Anm1 + an * Anm2;
        Bn = bn * Bnm1 + an * Bnm2;
        if fabs(An) > RECUR_BIG || fabs(Bn) > RECUR_BIG {
            An /= RECUR_BIG;
            Bn /= RECUR_BIG;
            Anm1 /= RECUR_BIG;
            Bnm1 /= RECUR_BIG;
            Anm2 /= RECUR_BIG;
            Bnm2 /= RECUR_BIG;
        }
        old_fn = fn_0;
        fn_0 = An / Bn;
        del = old_fn / fn_0;
        if fabs(del - 1.0f64) < 4.0f64 * 2.2204460492503131e-16f64 {
            break;
        }
    }
    *result = fn_0;
    if n == maxiter {
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"legendre_Qn.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
            GSL_EMAXITER as libc::c_int,
        );
        return GSL_EMAXITER as libc::c_int;
    } else {
        return GSL_SUCCESS as libc::c_int
    };
}
unsafe extern "C" fn legendre_Ql_asymp_unif(
    ell: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x < 1.0f64 {
        let mut u: libc::c_double = ell + 0.5f64;
        let mut th: libc::c_double = acos(x);
        let mut Y0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut Y1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_Y0: libc::c_int = 0;
        let mut stat_Y1: libc::c_int = 0;
        let mut stat_m: libc::c_int = 0;
        let mut pre: libc::c_double = 0.;
        let mut B00: libc::c_double = 0.;
        let mut sum: libc::c_double = 0.;
        if th < 1.2207031250000000e-04f64 {
            B00 = (1.0f64 + th * th / 15.0f64) / 24.0f64;
            pre = 1.0f64 + th * th / 12.0f64;
        } else {
            let mut sin_th: libc::c_double = sqrt(1.0f64 - x * x);
            let mut cot_th: libc::c_double = x / sin_th;
            B00 = 1.0f64 / 8.0f64 * (1.0f64 - th * cot_th) / (th * th);
            pre = sqrt(th / sin_th);
        }
        stat_Y0 = gsl_sf_bessel_Y0_e(u * th, &mut Y0);
        stat_Y1 = gsl_sf_bessel_Y1_e(u * th, &mut Y1);
        sum = -0.5f64 * 3.14159265358979323846f64 * (Y0.val + th / u * Y1.val * B00);
        stat_m = gsl_sf_multiply_e(pre, sum, result);
        (*result).err
            += 0.5f64 * 3.14159265358979323846f64 * fabs(pre)
                * (Y0.err + fabs(th / u * B00) * Y1.err);
        (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
        return if stat_m != GSL_SUCCESS as libc::c_int {
            stat_m
        } else if stat_Y0 != GSL_SUCCESS as libc::c_int {
            stat_Y0
        } else if stat_Y1 != GSL_SUCCESS as libc::c_int {
            stat_Y1
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else {
        let mut u_0: libc::c_double = ell + 0.5f64;
        let mut xi: libc::c_double = acosh(x);
        let mut K0_scaled: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut K1_scaled: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_K0: libc::c_int = 0;
        let mut stat_K1: libc::c_int = 0;
        let mut stat_e: libc::c_int = 0;
        let mut pre_0: libc::c_double = 0.;
        let mut B00_0: libc::c_double = 0.;
        let mut sum_0: libc::c_double = 0.;
        if xi < 1.2207031250000000e-04f64 {
            B00_0 = (1.0f64 - xi * xi / 15.0f64) / 24.0f64;
            pre_0 = 1.0f64 - xi * xi / 12.0f64;
        } else {
            let mut sinh_xi: libc::c_double = sqrt(x * x - 1.0f64);
            let mut coth_xi: libc::c_double = x / sinh_xi;
            B00_0 = -1.0f64 / 8.0f64 * (1.0f64 - xi * coth_xi) / (xi * xi);
            pre_0 = sqrt(xi / sinh_xi);
        }
        stat_K0 = gsl_sf_bessel_K0_scaled_e(u_0 * xi, &mut K0_scaled);
        stat_K1 = gsl_sf_bessel_K1_scaled_e(u_0 * xi, &mut K1_scaled);
        sum_0 = K0_scaled.val - xi / u_0 * K1_scaled.val * B00_0;
        stat_e = gsl_sf_exp_mult_e(-u_0 * xi, pre_0 * sum_0, result);
        (*result).err = 2.2204460492503131e-16f64 * fabs((*result).val) * fabs(u_0 * xi);
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return if stat_e != GSL_SUCCESS as libc::c_int {
            stat_e
        } else if stat_K0 != GSL_SUCCESS as libc::c_int {
            stat_K0
        } else if stat_K1 != GSL_SUCCESS as libc::c_int {
            stat_K1
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
pub unsafe extern "C" fn gsl_sf_legendre_Q0_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x <= -1.0f64 || x == 1.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"legendre_Qn.c\0" as *const u8 as *const libc::c_char,
            185 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x * x < 2.4607833005759251e-03f64 {
        let c3: libc::c_double = 1.0f64 / 3.0f64;
        let c5: libc::c_double = 1.0f64 / 5.0f64;
        let c7: libc::c_double = 1.0f64 / 7.0f64;
        let c9: libc::c_double = 1.0f64 / 9.0f64;
        let c11: libc::c_double = 1.0f64 / 11.0f64;
        let y: libc::c_double = x * x;
        let series: libc::c_double = 1.0f64
            + y * (c3 + y * (c5 + y * (c7 + y * (c9 + y * c11))));
        (*result).val = x * series;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs(x);
        return GSL_SUCCESS as libc::c_int;
    } else if x < 1.0f64 {
        (*result).val = 0.5f64 * log((1.0f64 + x) / (1.0f64 - x));
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x < 10.0f64 {
        (*result).val = 0.5f64 * log((x + 1.0f64) / (x - 1.0f64));
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x * 2.2250738585072014e-308f64 < 2.0f64 {
        let y_0: libc::c_double = 1.0f64 / (x * x);
        let c1: libc::c_double = 1.0f64 / 3.0f64;
        let c2: libc::c_double = 1.0f64 / 5.0f64;
        let c3_0: libc::c_double = 1.0f64 / 7.0f64;
        let c4: libc::c_double = 1.0f64 / 9.0f64;
        let c5_0: libc::c_double = 1.0f64 / 11.0f64;
        let c6: libc::c_double = 1.0f64 / 13.0f64;
        let c7_0: libc::c_double = 1.0f64 / 15.0f64;
        (*result)
            .val = 1.0f64 / x
            * (1.0f64
                + y_0
                    * (c1
                        + y_0
                            * (c2
                                + y_0
                                    * (c3_0
                                        + y_0 * (c4 + y_0 * (c5_0 + y_0 * (c6 + y_0 * c7_0)))))));
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*result).val = 0.0f64;
        (*result).err = 2.2250738585072014e-308f64;
        gsl_error(
            b"underflow\0" as *const u8 as *const libc::c_char,
            b"legendre_Qn.c\0" as *const u8 as *const libc::c_char,
            223 as libc::c_int,
            GSL_EUNDRFLW as libc::c_int,
        );
        return GSL_EUNDRFLW as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_legendre_Q1_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x <= -1.0f64 || x == 1.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"legendre_Qn.c\0" as *const u8 as *const libc::c_char,
            234 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x * x < 2.4607833005759251e-03f64 {
        let c3: libc::c_double = 1.0f64 / 3.0f64;
        let c5: libc::c_double = 1.0f64 / 5.0f64;
        let c7: libc::c_double = 1.0f64 / 7.0f64;
        let c9: libc::c_double = 1.0f64 / 9.0f64;
        let c11: libc::c_double = 1.0f64 / 11.0f64;
        let y: libc::c_double = x * x;
        let series: libc::c_double = 1.0f64
            + y * (c3 + y * (c5 + y * (c7 + y * (c9 + y * c11))));
        (*result).val = x * x * series - 1.0f64;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x < 1.0f64 {
        (*result).val = 0.5f64 * x * log((1.0f64 + x) / (1.0f64 - x)) - 1.0f64;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x < 6.0f64 {
        (*result).val = 0.5f64 * x * log((x + 1.0f64) / (x - 1.0f64)) - 1.0f64;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x * 1.4916681462400413e-154f64
        < 0.99f64 / 1.73205080756887729352744634151f64
    {
        let y_0: libc::c_double = 1 as libc::c_int as libc::c_double / (x * x);
        let c1: libc::c_double = 3.0f64 / 5.0f64;
        let c2: libc::c_double = 3.0f64 / 7.0f64;
        let c3_0: libc::c_double = 3.0f64 / 9.0f64;
        let c4: libc::c_double = 3.0f64 / 11.0f64;
        let c5_0: libc::c_double = 3.0f64 / 13.0f64;
        let c6: libc::c_double = 3.0f64 / 15.0f64;
        let c7_0: libc::c_double = 3.0f64 / 17.0f64;
        let c8: libc::c_double = 3.0f64 / 19.0f64;
        let sum: libc::c_double = 1.0f64
            + y_0
                * (c1
                    + y_0
                        * (c2
                            + y_0
                                * (c3_0
                                    + y_0
                                        * (c4
                                            + y_0 * (c5_0 + y_0 * (c6 + y_0 * (c7_0 + y_0 * c8)))))));
        (*result).val = sum / (3.0f64 * x * x);
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*result).val = 0.0f64;
        (*result).err = 2.2250738585072014e-308f64;
        gsl_error(
            b"underflow\0" as *const u8 as *const libc::c_char,
            b"legendre_Qn.c\0" as *const u8 as *const libc::c_char,
            274 as libc::c_int,
            GSL_EUNDRFLW as libc::c_int,
        );
        return GSL_EUNDRFLW as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_legendre_Ql_e(
    l: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x <= -1.0f64 || x == 1.0f64 || l < 0 as libc::c_int {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"legendre_Qn.c\0" as *const u8 as *const libc::c_char,
            285 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if l == 0 as libc::c_int {
        return gsl_sf_legendre_Q0_e(x, result)
    } else if l == 1 as libc::c_int {
        return gsl_sf_legendre_Q1_e(x, result)
    } else if l > 100000 as libc::c_int {
        return legendre_Ql_asymp_unif(l as libc::c_double, x, result)
    } else if x < 1.0f64 {
        let mut Q0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut Q1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_Q0: libc::c_int = gsl_sf_legendre_Q0_e(x, &mut Q0);
        let mut stat_Q1: libc::c_int = gsl_sf_legendre_Q1_e(x, &mut Q1);
        let mut Qellm1: libc::c_double = Q0.val;
        let mut Qell: libc::c_double = Q1.val;
        let mut Qellp1: libc::c_double = 0.;
        let mut ell: libc::c_int = 0;
        ell = 1 as libc::c_int;
        while ell < l {
            Qellp1 = (x * (2.0f64 * ell as libc::c_double + 1.0f64) * Qell
                - ell as libc::c_double * Qellm1) / (ell as libc::c_double + 1.0f64);
            Qellm1 = Qell;
            Qell = Qellp1;
            ell += 1;
            ell;
        }
        (*result).val = Qell;
        (*result)
            .err = 2.2204460492503131e-16f64 * l as libc::c_double * fabs((*result).val);
        return if stat_Q0 != GSL_SUCCESS as libc::c_int {
            stat_Q0
        } else if stat_Q1 != GSL_SUCCESS as libc::c_int {
            stat_Q1
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else {
        let mut rat: libc::c_double = 0.;
        let mut stat_CF1: libc::c_int = legendreQ_CF1_xgt1(
            l,
            0.0f64,
            0.0f64,
            x,
            &mut rat,
        );
        let mut stat_Q: libc::c_int = 0;
        let mut Qellp1_0: libc::c_double = rat * 1.4916681462400413e-154f64;
        let mut Qell_0: libc::c_double = 1.4916681462400413e-154f64;
        let mut Qellm1_0: libc::c_double = 0.;
        let mut ell_0: libc::c_int = 0;
        ell_0 = l;
        while ell_0 > 0 as libc::c_int {
            Qellm1_0 = (x * (2.0f64 * ell_0 as libc::c_double + 1.0f64) * Qell_0
                - (ell_0 as libc::c_double + 1.0f64) * Qellp1_0)
                / ell_0 as libc::c_double;
            Qellp1_0 = Qell_0;
            Qell_0 = Qellm1_0;
            ell_0 -= 1;
            ell_0;
        }
        if fabs(Qell_0) > fabs(Qellp1_0) {
            let mut Q0_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            stat_Q = gsl_sf_legendre_Q0_e(x, &mut Q0_0);
            (*result).val = 1.4916681462400413e-154f64 * Q0_0.val / Qell_0;
            (*result)
                .err = l as libc::c_double * 2.2204460492503131e-16f64
                * fabs((*result).val);
        } else {
            let mut Q1_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            stat_Q = gsl_sf_legendre_Q1_e(x, &mut Q1_0);
            (*result).val = 1.4916681462400413e-154f64 * Q1_0.val / Qellp1_0;
            (*result)
                .err = l as libc::c_double * 2.2204460492503131e-16f64
                * fabs((*result).val);
        }
        return if stat_Q != GSL_SUCCESS as libc::c_int {
            stat_Q
        } else if stat_CF1 != GSL_SUCCESS as libc::c_int {
            stat_CF1
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
pub unsafe extern "C" fn gsl_sf_legendre_Q0(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_legendre_Q0_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_legendre_Q0_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"legendre_Qn.c\0" as *const u8 as *const libc::c_char,
            355 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_legendre_Q1(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_legendre_Q1_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_legendre_Q1_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"legendre_Qn.c\0" as *const u8 as *const libc::c_char,
            360 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_legendre_Ql(
    l: libc::c_int,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_legendre_Ql_e(l, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_legendre_Ql_e(l, x, &result)\0" as *const u8 as *const libc::c_char,
            b"legendre_Qn.c\0" as *const u8 as *const libc::c_char,
            365 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
