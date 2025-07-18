use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sf_bessel_I0_scaled_e(
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_bessel_I1_scaled_e(
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn abs(_: libc::c_int) -> libc::c_int;
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
    fn gsl_sf_bessel_I_CF1_ser(
        nu: libc::c_double,
        x: libc::c_double,
        ratio: *mut libc::c_double,
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
pub unsafe extern "C" fn gsl_sf_bessel_In_scaled_e(
    mut n: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let ax: libc::c_double = fabs(x);
    n = abs(n);
    if n == 0 as libc::c_int {
        return gsl_sf_bessel_I0_scaled_e(x, result)
    } else if n == 1 as libc::c_int {
        return gsl_sf_bessel_I1_scaled_e(x, result)
    } else if x == 0.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if x * x < 10.0f64 * (n as libc::c_double + 1.0f64) / 2.7182818284590452354f64
    {
        let mut t: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut ex: libc::c_double = exp(-ax);
        let mut stat_In: libc::c_int = gsl_sf_bessel_IJ_taylor_e(
            n as libc::c_double,
            ax,
            1 as libc::c_int,
            50 as libc::c_int,
            2.2204460492503131e-16f64,
            &mut t,
        );
        (*result).val = t.val * ex;
        (*result).err = t.err * ex;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        if x < 0.0f64 && n & 1 as libc::c_int != 0 {
            (*result).val = -(*result).val;
        }
        return stat_In;
    } else if n < 150 as libc::c_int && ax < 1e7f64 {
        let mut I0_scaled: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_I0: libc::c_int = gsl_sf_bessel_I0_scaled_e(ax, &mut I0_scaled);
        let mut rat: libc::c_double = 0.;
        let mut stat_CF1: libc::c_int = gsl_sf_bessel_I_CF1_ser(
            n as libc::c_double,
            ax,
            &mut rat,
        );
        let mut Ikp1: libc::c_double = rat * 1.4916681462400413e-154f64;
        let mut Ik: libc::c_double = 1.4916681462400413e-154f64;
        let mut Ikm1: libc::c_double = 0.;
        let mut k: libc::c_int = 0;
        k = n;
        while k >= 1 as libc::c_int {
            Ikm1 = Ikp1 + 2.0f64 * k as libc::c_double / ax * Ik;
            Ikp1 = Ik;
            Ik = Ikm1;
            k -= 1;
            k;
        }
        (*result).val = I0_scaled.val * (1.4916681462400413e-154f64 / Ik);
        (*result).err = I0_scaled.err * (1.4916681462400413e-154f64 / Ik);
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        if x < 0.0f64 && n & 1 as libc::c_int != 0 {
            (*result).val = -(*result).val;
        }
        return if stat_I0 != GSL_SUCCESS as libc::c_int {
            stat_I0
        } else if stat_CF1 != GSL_SUCCESS as libc::c_int {
            stat_CF1
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else if (if (0.29f64 / (n * n) as libc::c_double)
        < 0.5f64 / ((n * n) as libc::c_double + x * x)
    {
        0.29f64 / (n * n) as libc::c_double
    } else {
        0.5f64 / ((n * n) as libc::c_double + x * x)
    }) < 0.5f64 * 6.0554544523933429e-06f64
    {
        let mut stat_as: libc::c_int = gsl_sf_bessel_Inu_scaled_asymp_unif_e(
            n as libc::c_double,
            ax,
            result,
        );
        if x < 0.0f64 && n & 1 as libc::c_int != 0 {
            (*result).val = -(*result).val;
        }
        return stat_as;
    } else {
        let nhi: libc::c_int = 2 as libc::c_int
            + (1.2f64 / 2.4607833005759251e-03f64) as libc::c_int;
        let mut r_Ikp1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut r_Ik: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_a1: libc::c_int = gsl_sf_bessel_Inu_scaled_asymp_unif_e(
            nhi as libc::c_double + 1.0f64,
            ax,
            &mut r_Ikp1,
        );
        let mut stat_a2: libc::c_int = gsl_sf_bessel_Inu_scaled_asymp_unif_e(
            nhi as libc::c_double,
            ax,
            &mut r_Ik,
        );
        let mut Ikp1_0: libc::c_double = r_Ikp1.val;
        let mut Ik_0: libc::c_double = r_Ik.val;
        let mut Ikm1_0: libc::c_double = 0.;
        let mut k_0: libc::c_int = 0;
        k_0 = nhi;
        while k_0 > n {
            Ikm1_0 = Ikp1_0 + 2.0f64 * k_0 as libc::c_double / ax * Ik_0;
            Ikp1_0 = Ik_0;
            Ik_0 = Ikm1_0;
            k_0 -= 1;
            k_0;
        }
        (*result).val = Ik_0;
        (*result).err = Ik_0 * (r_Ikp1.err / r_Ikp1.val + r_Ik.err / r_Ik.val);
        if x < 0.0f64 && n & 1 as libc::c_int != 0 {
            (*result).val = -(*result).val;
        }
        return if stat_a1 != GSL_SUCCESS as libc::c_int {
            stat_a1
        } else if stat_a2 != GSL_SUCCESS as libc::c_int {
            stat_a2
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_In_scaled_array(
    nmin: libc::c_int,
    nmax: libc::c_int,
    x: libc::c_double,
    mut result_array: *mut libc::c_double,
) -> libc::c_int {
    if nmax < nmin || nmin < 0 as libc::c_int {
        let mut j: libc::c_int = 0;
        j = 0 as libc::c_int;
        while j <= nmax - nmin {
            *result_array.offset(j as isize) = 0.0f64;
            j += 1;
            j;
        }
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"bessel_In.c\0" as *const u8 as *const libc::c_char,
            120 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x == 0.0f64 {
        let mut j_0: libc::c_int = 0;
        j_0 = 0 as libc::c_int;
        while j_0 <= nmax - nmin {
            *result_array.offset(j_0 as isize) = 0.0f64;
            j_0 += 1;
            j_0;
        }
        if nmin == 0 as libc::c_int {
            *result_array.offset(0 as libc::c_int as isize) = 1.0f64;
        }
        return GSL_SUCCESS as libc::c_int;
    } else if nmax == 0 as libc::c_int {
        let mut I0_scaled: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat: libc::c_int = gsl_sf_bessel_I0_scaled_e(x, &mut I0_scaled);
        *result_array.offset(0 as libc::c_int as isize) = I0_scaled.val;
        return stat;
    } else {
        let ax: libc::c_double = fabs(x);
        let two_over_x: libc::c_double = 2.0f64 / ax;
        let mut r_Inp1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut r_In: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_0: libc::c_int = gsl_sf_bessel_In_scaled_e(
            nmax + 1 as libc::c_int,
            ax,
            &mut r_Inp1,
        );
        let mut stat_1: libc::c_int = gsl_sf_bessel_In_scaled_e(nmax, ax, &mut r_In);
        let mut Inp1: libc::c_double = r_Inp1.val;
        let mut In: libc::c_double = r_In.val;
        let mut Inm1: libc::c_double = 0.;
        let mut n: libc::c_int = 0;
        n = nmax;
        while n >= nmin {
            *result_array.offset((n - nmin) as isize) = In;
            Inm1 = Inp1 + n as libc::c_double * two_over_x * In;
            Inp1 = In;
            In = Inm1;
            n -= 1;
            n;
        }
        if x < 0.0f64 {
            n = nmin;
            while n <= nmax {
                if n & 1 as libc::c_int != 0 {
                    *result_array
                        .offset(
                            (n - nmin) as isize,
                        ) = -*result_array.offset((n - nmin) as isize);
                }
                n += 1;
                n;
            }
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
pub unsafe extern "C" fn gsl_sf_bessel_In_e(
    n_in: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let ax: libc::c_double = fabs(x);
    let n: libc::c_int = abs(n_in);
    let mut In_scaled: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let stat_In_scaled: libc::c_int = gsl_sf_bessel_In_scaled_e(n, ax, &mut In_scaled);
    if ax > 7.0978271289338397e+02f64 - 1.0f64 {
        (*result).val = ::std::f32::INFINITY as libc::c_double;
        (*result).err = ::std::f32::INFINITY as libc::c_double;
        gsl_error(
            b"overflow\0" as *const u8 as *const libc::c_char,
            b"bessel_In.c\0" as *const u8 as *const libc::c_char,
            179 as libc::c_int,
            GSL_EOVRFLW as libc::c_int,
        );
        return GSL_EOVRFLW as libc::c_int;
    } else {
        let ex: libc::c_double = exp(ax);
        (*result).val = ex * In_scaled.val;
        (*result).err = ex * In_scaled.err;
        (*result).err += ax * 2.2204460492503131e-16f64 * fabs((*result).val);
        if x < 0.0f64 && n & 1 as libc::c_int != 0 {
            (*result).val = -(*result).val;
        }
        return stat_In_scaled;
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_In_array(
    nmin: libc::c_int,
    nmax: libc::c_int,
    x: libc::c_double,
    mut result_array: *mut libc::c_double,
) -> libc::c_int {
    let mut ax: libc::c_double = fabs(x);
    if ax > 7.0978271289338397e+02f64 - 1.0f64 {
        let mut j: libc::c_int = 0;
        j = 0 as libc::c_int;
        while j <= nmax - nmin {
            *result_array.offset(j as isize) = 0.0f64;
            j += 1;
            j;
        }
        gsl_error(
            b"overflow\0" as *const u8 as *const libc::c_char,
            b"bessel_In.c\0" as *const u8 as *const libc::c_char,
            202 as libc::c_int,
            GSL_EOVRFLW as libc::c_int,
        );
        return GSL_EOVRFLW as libc::c_int;
    } else {
        let mut j_0: libc::c_int = 0;
        let mut eax: libc::c_double = exp(ax);
        let mut status: libc::c_int = gsl_sf_bessel_In_scaled_array(
            nmin,
            nmax,
            x,
            result_array,
        );
        j_0 = 0 as libc::c_int;
        while j_0 <= nmax - nmin {
            *result_array.offset(j_0 as isize) *= eax;
            j_0 += 1;
            j_0;
        }
        return status;
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_In_scaled(
    n: libc::c_int,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_In_scaled_e(n, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_In_scaled_e(n, x, &result)\0" as *const u8
                as *const libc::c_char,
            b"bessel_In.c\0" as *const u8 as *const libc::c_char,
            219 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_bessel_In(
    n: libc::c_int,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_In_e(n, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_In_e(n, x, &result)\0" as *const u8 as *const libc::c_char,
            b"bessel_In.c\0" as *const u8 as *const libc::c_char,
            224 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
