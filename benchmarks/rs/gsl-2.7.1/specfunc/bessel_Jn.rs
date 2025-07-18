use ::libc;
extern "C" {
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
    fn gsl_sf_bessel_J0_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_bessel_J1_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
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
pub unsafe extern "C" fn gsl_sf_bessel_Jn_e(
    mut n: libc::c_int,
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut sign: libc::c_int = 1 as libc::c_int;
    if n < 0 as libc::c_int {
        n = -n;
        if n & 1 as libc::c_int != 0 {
            sign = -sign;
        }
    }
    if x < 0.0f64 {
        x = -x;
        if n & 1 as libc::c_int != 0 {
            sign = -sign;
        }
    }
    if n == 0 as libc::c_int {
        let mut b0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_J0: libc::c_int = gsl_sf_bessel_J0_e(x, &mut b0);
        (*result).val = sign as libc::c_double * b0.val;
        (*result).err = b0.err;
        return stat_J0;
    } else if n == 1 as libc::c_int {
        let mut b1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_J1: libc::c_int = gsl_sf_bessel_J1_e(x, &mut b1);
        (*result).val = sign as libc::c_double * b1.val;
        (*result).err = b1.err;
        return stat_J1;
    } else if x == 0.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if x * x
        < 10.0f64 * (n as libc::c_double + 1.0f64) * 7.4009597974140505e-04f64
    {
        let mut b: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut status: libc::c_int = gsl_sf_bessel_IJ_taylor_e(
            n as libc::c_double,
            x,
            -(1 as libc::c_int),
            50 as libc::c_int,
            2.2204460492503131e-16f64,
            &mut b,
        );
        (*result).val = sign as libc::c_double * b.val;
        (*result).err = b.err;
        (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
        return status;
    } else if 1.2207031250000000e-04f64 * x > (n * n) as libc::c_double + 1.0f64 {
        let mut status_0: libc::c_int = gsl_sf_bessel_Jnu_asympx_e(
            n as libc::c_double,
            x,
            result,
        );
        (*result).val *= sign as libc::c_double;
        return status_0;
    } else if n > 50 as libc::c_int {
        let mut status_1: libc::c_int = gsl_sf_bessel_Jnu_asymp_Olver_e(
            n as libc::c_double,
            x,
            result,
        );
        (*result).val *= sign as libc::c_double;
        return status_1;
    } else if x > 1000.0f64 {
        let mut status_2: libc::c_int = gsl_sf_bessel_Jnu_asympx_e(
            n as libc::c_double,
            x,
            result,
        );
        (*result).val *= sign as libc::c_double;
        return status_2;
    } else {
        let mut ans: libc::c_double = 0.;
        let mut err: libc::c_double = 0.;
        let mut ratio: libc::c_double = 0.;
        let mut sgn: libc::c_double = 0.;
        let mut stat_b: libc::c_int = 0;
        let mut stat_CF1: libc::c_int = gsl_sf_bessel_J_CF1(
            n as libc::c_double,
            x,
            &mut ratio,
            &mut sgn,
        );
        let mut Jkp1: libc::c_double = 1.4916681462400413e-154f64 * ratio;
        let mut Jk: libc::c_double = 1.4916681462400413e-154f64;
        let mut Jkm1: libc::c_double = 0.;
        let mut k: libc::c_int = 0;
        k = n;
        while k > 0 as libc::c_int {
            Jkm1 = 2.0f64 * k as libc::c_double / x * Jk - Jkp1;
            Jkp1 = Jk;
            Jk = Jkm1;
            k -= 1;
            k;
        }
        if fabs(Jkp1) > fabs(Jk) {
            let mut b1_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            stat_b = gsl_sf_bessel_J1_e(x, &mut b1_0);
            ans = b1_0.val / Jkp1 * 1.4916681462400413e-154f64;
            err = b1_0.err / Jkp1 * 1.4916681462400413e-154f64;
        } else {
            let mut b0_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            stat_b = gsl_sf_bessel_J0_e(x, &mut b0_0);
            ans = b0_0.val / Jk * 1.4916681462400413e-154f64;
            err = b0_0.err / Jk * 1.4916681462400413e-154f64;
        }
        (*result).val = sign as libc::c_double * ans;
        (*result).err = fabs(err);
        return if stat_CF1 != GSL_SUCCESS as libc::c_int {
            stat_CF1
        } else if stat_b != GSL_SUCCESS as libc::c_int {
            stat_b
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_Jn_array(
    mut nmin: libc::c_int,
    mut nmax: libc::c_int,
    mut x: libc::c_double,
    mut result_array: *mut libc::c_double,
) -> libc::c_int {
    if nmin < 0 as libc::c_int || nmax < nmin {
        let mut n: libc::c_int = 0;
        n = nmax;
        while n >= nmin {
            *result_array.offset((n - nmin) as isize) = 0.0f64;
            n -= 1;
            n;
        }
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"bessel_Jn.c\0" as *const u8 as *const libc::c_char,
            152 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x == 0.0f64 {
        let mut n_0: libc::c_int = 0;
        n_0 = nmax;
        while n_0 >= nmin {
            *result_array.offset((n_0 - nmin) as isize) = 0.0f64;
            n_0 -= 1;
            n_0;
        }
        if nmin == 0 as libc::c_int {
            *result_array.offset(0 as libc::c_int as isize) = 1.0f64;
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut r_Jnp1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut r_Jn: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_np1: libc::c_int = gsl_sf_bessel_Jn_e(
            nmax + 1 as libc::c_int,
            x,
            &mut r_Jnp1,
        );
        let mut stat_n: libc::c_int = gsl_sf_bessel_Jn_e(nmax, x, &mut r_Jn);
        let mut stat: libc::c_int = if stat_np1 != GSL_SUCCESS as libc::c_int {
            stat_np1
        } else if stat_n != GSL_SUCCESS as libc::c_int {
            stat_n
        } else {
            GSL_SUCCESS as libc::c_int
        };
        let mut Jnp1: libc::c_double = r_Jnp1.val;
        let mut Jn: libc::c_double = r_Jn.val;
        let mut Jnm1: libc::c_double = 0.;
        let mut n_1: libc::c_int = 0;
        if stat == GSL_SUCCESS as libc::c_int {
            n_1 = nmax;
            while n_1 >= nmin {
                *result_array.offset((n_1 - nmin) as isize) = Jn;
                Jnm1 = -Jnp1 + 2.0f64 * n_1 as libc::c_double / x * Jn;
                Jnp1 = Jn;
                Jn = Jnm1;
                n_1 -= 1;
                n_1;
            }
        } else {
            n_1 = nmax;
            while n_1 >= nmin {
                *result_array.offset((n_1 - nmin) as isize) = 0.0f64;
                n_1 -= 1;
                n_1;
            }
        }
        return stat;
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_Jn(
    n: libc::c_int,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_Jn_e(n, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_Jn_e(n, x, &result)\0" as *const u8 as *const libc::c_char,
            b"bessel_Jn.c\0" as *const u8 as *const libc::c_char,
            198 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
