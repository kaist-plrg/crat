use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sf_fact_e(n: libc::c_uint, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_lnfact_e(n: libc::c_uint, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_psi_int_e(n: libc::c_int, result: *mut gsl_sf_result) -> libc::c_int;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn gsl_sf_bessel_K1_scaled_e(
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_bessel_K0_scaled_e(
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
unsafe extern "C" fn bessel_Kn_scaled_small_x(
    n: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut k: libc::c_int = 0;
    let mut y: libc::c_double = 0.25f64 * x * x;
    let mut ln_x_2: libc::c_double = log(0.5f64 * x);
    let mut ex: libc::c_double = exp(x);
    let mut ln_nm1_fact: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut k_term: libc::c_double = 0.;
    let mut term1: libc::c_double = 0.;
    let mut sum1: libc::c_double = 0.;
    let mut ln_pre1: libc::c_double = 0.;
    let mut term2: libc::c_double = 0.;
    let mut sum2: libc::c_double = 0.;
    let mut pre2: libc::c_double = 0.;
    gsl_sf_lnfact_e((n - 1 as libc::c_int) as libc::c_uint, &mut ln_nm1_fact);
    ln_pre1 = -n as libc::c_double * ln_x_2 + ln_nm1_fact.val;
    if ln_pre1 > 7.0978271289338397e+02f64 - 3.0f64 {
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"bessel_Kn.c\0" as *const u8 as *const libc::c_char,
            54 as libc::c_int,
            GSL_EOVRFLW as libc::c_int,
        );
        return GSL_EOVRFLW as libc::c_int;
    }
    sum1 = 1.0f64;
    k_term = 1.0f64;
    k = 1 as libc::c_int;
    while k <= n - 1 as libc::c_int {
        k_term *= -y / (k * (n - k)) as libc::c_double;
        sum1 += k_term;
        k += 1;
        k;
    }
    term1 = 0.5f64 * exp(ln_pre1) * sum1;
    pre2 = 0.5f64 * exp(n as libc::c_double * ln_x_2);
    if pre2 > 0.0f64 {
        let KMAX: libc::c_int = 20 as libc::c_int;
        let mut psi_n: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut npk_fact: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut yk: libc::c_double = 1.0f64;
        let mut k_fact: libc::c_double = 1.0f64;
        let mut psi_kp1: libc::c_double = -0.57721566490153286060651209008f64;
        let mut psi_npkp1: libc::c_double = 0.;
        gsl_sf_psi_int_e(n, &mut psi_n);
        gsl_sf_fact_e(n as libc::c_uint, &mut npk_fact);
        psi_npkp1 = psi_n.val + 1.0f64 / n as libc::c_double;
        sum2 = (psi_kp1 + psi_npkp1 - 2.0f64 * ln_x_2) / npk_fact.val;
        k = 1 as libc::c_int;
        while k < KMAX {
            psi_kp1 += 1.0f64 / k as libc::c_double;
            psi_npkp1 += 1.0f64 / (n + k) as libc::c_double;
            k_fact *= k as libc::c_double;
            npk_fact.val *= (n + k) as libc::c_double;
            yk *= y;
            k_term = yk * (psi_kp1 + psi_npkp1 - 2.0f64 * ln_x_2)
                / (k_fact * npk_fact.val);
            sum2 += k_term;
            k += 1;
            k;
        }
        term2 = (if n & 1 as libc::c_int != 0 { -1.0f64 } else { 1.0f64 }) * pre2 * sum2;
    } else {
        term2 = 0.0f64;
    }
    (*result).val = ex * (term1 + term2);
    (*result)
        .err = ex * 2.2204460492503131e-16f64
        * (fabs(ln_pre1) * fabs(term1) + fabs(term2));
    (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sf_bessel_Kn_scaled_e(
    mut n: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    n = abs(n);
    if x <= 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"bessel_Kn.c\0" as *const u8 as *const libc::c_char,
            109 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if n == 0 as libc::c_int {
        return gsl_sf_bessel_K0_scaled_e(x, result)
    } else if n == 1 as libc::c_int {
        return gsl_sf_bessel_K1_scaled_e(x, result)
    } else if x <= 5.0f64 {
        return bessel_Kn_scaled_small_x(n, x, result)
    } else if 6.0554544523933429e-06f64 * x
        > 0.25f64 * (n * n + 1 as libc::c_int) as libc::c_double
    {
        return gsl_sf_bessel_Knu_scaled_asympx_e(n as libc::c_double, x, result)
    } else if (if (0.29f64 / (n * n) as libc::c_double)
        < 0.5f64 / ((n * n) as libc::c_double + x * x)
    {
        0.29f64 / (n * n) as libc::c_double
    } else {
        0.5f64 / ((n * n) as libc::c_double + x * x)
    }) < 6.0554544523933429e-06f64
    {
        return gsl_sf_bessel_Knu_scaled_asymp_unif_e(n as libc::c_double, x, result)
    } else {
        let mut two_over_x: libc::c_double = 2.0f64 / x;
        let mut r_b_jm1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut r_b_j: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_0: libc::c_int = gsl_sf_bessel_K0_scaled_e(x, &mut r_b_jm1);
        let mut stat_1: libc::c_int = gsl_sf_bessel_K1_scaled_e(x, &mut r_b_j);
        let mut b_jm1: libc::c_double = r_b_jm1.val;
        let mut b_j: libc::c_double = r_b_j.val;
        let mut b_jp1: libc::c_double = 0.;
        let mut j: libc::c_int = 0;
        j = 1 as libc::c_int;
        while j < n {
            b_jp1 = b_jm1 + j as libc::c_double * two_over_x * b_j;
            b_jm1 = b_j;
            b_j = b_jp1;
            j += 1;
            j;
        }
        (*result).val = b_j;
        (*result)
            .err = n as libc::c_double
            * (fabs(b_j)
                * (fabs(r_b_jm1.err / r_b_jm1.val) + fabs(r_b_j.err / r_b_j.val)));
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return if stat_0 != GSL_SUCCESS as libc::c_int {
            stat_0
        } else if stat_1 != GSL_SUCCESS as libc::c_int {
            stat_1
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_Kn_e(
    n: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let status: libc::c_int = gsl_sf_bessel_Kn_scaled_e(n, x, result);
    let ex: libc::c_double = exp(-x);
    (*result).val *= ex;
    (*result).err *= ex;
    (*result).err += x * 2.2204460492503131e-16f64 * fabs((*result).val);
    return status;
}
pub unsafe extern "C" fn gsl_sf_bessel_Kn_scaled_array(
    nmin: libc::c_int,
    nmax: libc::c_int,
    x: libc::c_double,
    mut result_array: *mut libc::c_double,
) -> libc::c_int {
    if nmin < 0 as libc::c_int || nmax < nmin || x <= 0.0f64 {
        let mut j: libc::c_int = 0;
        j = 0 as libc::c_int;
        while j <= nmax - nmin {
            *result_array.offset(j as isize) = 0.0f64;
            j += 1;
            j;
        }
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"bessel_Kn.c\0" as *const u8 as *const libc::c_char,
            171 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if nmax == 0 as libc::c_int {
        let mut b: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat: libc::c_int = gsl_sf_bessel_K0_scaled_e(x, &mut b);
        *result_array.offset(0 as libc::c_int as isize) = b.val;
        return stat;
    } else {
        let mut two_over_x: libc::c_double = 2.0f64 / x;
        let mut r_Knm1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut r_Kn: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_0: libc::c_int = gsl_sf_bessel_Kn_scaled_e(nmin, x, &mut r_Knm1);
        let mut stat_1: libc::c_int = gsl_sf_bessel_Kn_scaled_e(
            nmin + 1 as libc::c_int,
            x,
            &mut r_Kn,
        );
        let mut stat_2: libc::c_int = if stat_0 != GSL_SUCCESS as libc::c_int {
            stat_0
        } else if stat_1 != GSL_SUCCESS as libc::c_int {
            stat_1
        } else {
            GSL_SUCCESS as libc::c_int
        };
        let mut Knp1: libc::c_double = 0.;
        let mut Kn: libc::c_double = r_Kn.val;
        let mut Knm1: libc::c_double = r_Knm1.val;
        let mut n: libc::c_int = 0;
        n = nmin + 1 as libc::c_int;
        while n <= nmax + 1 as libc::c_int {
            if Knm1 < 1.7976931348623157e+308f64 {
                *result_array.offset((n - 1 as libc::c_int - nmin) as isize) = Knm1;
                Knp1 = Knm1 + n as libc::c_double * two_over_x * Kn;
                Knm1 = Kn;
                Kn = Knp1;
            } else {
                let mut j_0: libc::c_int = 0;
                j_0 = n;
                while j_0 <= nmax + 1 as libc::c_int {
                    *result_array
                        .offset((j_0 - 1 as libc::c_int - nmin) as isize) = 0.0f64;
                    j_0 += 1;
                    j_0;
                }
                gsl_error(
                    b"overflow\0" as *const u8 as *const libc::c_char,
                    b"bessel_Kn.c\0" as *const u8 as *const libc::c_char,
                    208 as libc::c_int,
                    GSL_EOVRFLW as libc::c_int,
                );
                return GSL_EOVRFLW as libc::c_int;
            }
            n += 1;
            n;
        }
        return stat_2;
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_Kn_array(
    nmin: libc::c_int,
    nmax: libc::c_int,
    x: libc::c_double,
    mut result_array: *mut libc::c_double,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_sf_bessel_Kn_scaled_array(
        nmin,
        nmax,
        x,
        result_array,
    );
    let mut ex: libc::c_double = exp(-x);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i <= nmax - nmin {
        *result_array.offset(i as isize) *= ex;
        i += 1;
        i;
    }
    return status;
}
pub unsafe extern "C" fn gsl_sf_bessel_Kn_scaled(
    n: libc::c_int,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_Kn_scaled_e(n, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_Kn_scaled_e(n, x, &result)\0" as *const u8
                as *const libc::c_char,
            b"bessel_Kn.c\0" as *const u8 as *const libc::c_char,
            234 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_bessel_Kn(
    n: libc::c_int,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_Kn_e(n, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_Kn_e(n, x, &result)\0" as *const u8 as *const libc::c_char,
            b"bessel_Kn.c\0" as *const u8 as *const libc::c_char,
            239 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
