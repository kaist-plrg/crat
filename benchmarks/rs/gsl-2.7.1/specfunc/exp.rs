use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sf_lnfact_e(n: libc::c_uint, result: *mut gsl_sf_result) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_sf_result_e10_struct {
    pub val: libc::c_double,
    pub err: libc::c_double,
    pub e10: libc::c_int,
}
pub type gsl_sf_result_e10 = gsl_sf_result_e10_struct;
#[inline]
unsafe extern "C" fn GSL_MAX_DBL(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return if a > b { a } else { b };
}
unsafe extern "C" fn exprel_n_CF(
    N: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let RECUR_BIG: libc::c_double = 1.3407807929942596e+154f64;
    let maxiter: libc::c_int = 5000 as libc::c_int;
    let mut n: libc::c_int = 1 as libc::c_int;
    let mut Anm2: libc::c_double = 1.0f64;
    let mut Bnm2: libc::c_double = 0.0f64;
    let mut Anm1: libc::c_double = 0.0f64;
    let mut Bnm1: libc::c_double = 1.0f64;
    let mut a1: libc::c_double = 1.0f64;
    let mut b1: libc::c_double = 1.0f64;
    let mut a2: libc::c_double = -x;
    let mut b2: libc::c_double = N + 1 as libc::c_int as libc::c_double;
    let mut an: libc::c_double = 0.;
    let mut bn: libc::c_double = 0.;
    let mut fn_0: libc::c_double = 0.;
    let mut An: libc::c_double = b1 * Anm1 + a1 * Anm2;
    let mut Bn: libc::c_double = b1 * Bnm1 + a1 * Bnm2;
    n += 1;
    n;
    Anm2 = Anm1;
    Bnm2 = Bnm1;
    Anm1 = An;
    Bnm1 = Bn;
    An = b2 * Anm1 + a2 * Anm2;
    Bn = b2 * Bnm1 + a2 * Bnm2;
    fn_0 = An / Bn;
    while n < maxiter {
        let mut old_fn: libc::c_double = 0.;
        let mut del: libc::c_double = 0.;
        n += 1;
        n;
        Anm2 = Anm1;
        Bnm2 = Bnm1;
        Anm1 = An;
        Bnm1 = Bn;
        an = if n & 1 as libc::c_int != 0 {
            ((n - 1 as libc::c_int) / 2 as libc::c_int) as libc::c_double * x
        } else {
            -(N + (n / 2 as libc::c_int) as libc::c_double
                - 1 as libc::c_int as libc::c_double) * x
        };
        bn = N + n as libc::c_double - 1 as libc::c_int as libc::c_double;
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
        if fabs(del - 1.0f64) < 2.0f64 * 2.2204460492503131e-16f64 {
            break;
        }
    }
    (*result).val = fn_0;
    (*result)
        .err = 4.0f64 * (n as libc::c_double + 1.0f64) * 2.2204460492503131e-16f64
        * fabs(fn_0);
    if n == maxiter {
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"exp.c\0" as *const u8 as *const libc::c_char,
            99 as libc::c_int,
            GSL_EMAXITER as libc::c_int,
        );
        return GSL_EMAXITER as libc::c_int;
    } else {
        return GSL_SUCCESS as libc::c_int
    };
}
pub unsafe extern "C" fn gsl_sf_exp_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x > 7.0978271289338397e+02f64 {
        (*result).val = ::std::f32::INFINITY as libc::c_double;
        (*result).err = ::std::f32::INFINITY as libc::c_double;
        gsl_error(
            b"overflow\0" as *const u8 as *const libc::c_char,
            b"exp.c\0" as *const u8 as *const libc::c_char,
            110 as libc::c_int,
            GSL_EOVRFLW as libc::c_int,
        );
        return GSL_EOVRFLW as libc::c_int;
    } else if x < -7.0839641853226408e+02f64 {
        (*result).val = 0.0f64;
        (*result).err = 2.2250738585072014e-308f64;
        gsl_error(
            b"underflow\0" as *const u8 as *const libc::c_char,
            b"exp.c\0" as *const u8 as *const libc::c_char,
            113 as libc::c_int,
            GSL_EUNDRFLW as libc::c_int,
        );
        return GSL_EUNDRFLW as libc::c_int;
    } else {
        (*result).val = exp(x);
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_exp_e10_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result_e10,
) -> libc::c_int {
    if x > (2147483647 as libc::c_int - 1 as libc::c_int) as libc::c_double {
        (*result).val = ::std::f32::INFINITY as libc::c_double;
        (*result).err = ::std::f32::INFINITY as libc::c_double;
        (*result).e10 = 0 as libc::c_int;
        gsl_error(
            b"overflow\0" as *const u8 as *const libc::c_char,
            b"exp.c\0" as *const u8 as *const libc::c_char,
            125 as libc::c_int,
            GSL_EOVRFLW as libc::c_int,
        );
        return GSL_EOVRFLW as libc::c_int;
    } else if x
        < (-(2147483647 as libc::c_int) - 1 as libc::c_int + 1 as libc::c_int)
            as libc::c_double
    {
        (*result).val = 0.0f64;
        (*result).err = 2.2250738585072014e-308f64;
        (*result).e10 = 0 as libc::c_int;
        gsl_error(
            b"underflow\0" as *const u8 as *const libc::c_char,
            b"exp.c\0" as *const u8 as *const libc::c_char,
            128 as libc::c_int,
            GSL_EUNDRFLW as libc::c_int,
        );
        return GSL_EUNDRFLW as libc::c_int;
    } else {
        let N: libc::c_int = if x > 7.0978271289338397e+02f64
            || x < -7.0839641853226408e+02f64
        {
            floor(x / 2.30258509299404568402f64) as libc::c_int
        } else {
            0 as libc::c_int
        };
        (*result).val = exp(x - N as libc::c_double * 2.30258509299404568402f64);
        (*result)
            .err = 2.0f64 * (fabs(x) + 1.0f64) * 2.2204460492503131e-16f64
            * fabs((*result).val);
        (*result).e10 = N;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_exp_mult_e(
    x: libc::c_double,
    y: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let ay: libc::c_double = fabs(y);
    if y == 0.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if x < 0.5f64 * 7.0978271289338397e+02f64
        && x > 0.5f64 * -7.0839641853226408e+02f64
        && (ay < 0.8f64 * 1.3407807929942596e+154f64
            && ay > 1.2f64 * 1.4916681462400413e-154f64)
    {
        let ex: libc::c_double = exp(x);
        (*result).val = y * ex;
        (*result)
            .err = (2.0f64 + fabs(x)) * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let ly: libc::c_double = log(ay);
        let lnr: libc::c_double = x + ly;
        if lnr > 7.0978271289338397e+02f64 - 0.01f64 {
            (*result).val = ::std::f32::INFINITY as libc::c_double;
            (*result).err = ::std::f32::INFINITY as libc::c_double;
            gsl_error(
                b"overflow\0" as *const u8 as *const libc::c_char,
                b"exp.c\0" as *const u8 as *const libc::c_char,
                162 as libc::c_int,
                GSL_EOVRFLW as libc::c_int,
            );
            return GSL_EOVRFLW as libc::c_int;
        } else if lnr < -7.0839641853226408e+02f64 + 0.01f64 {
            (*result).val = 0.0f64;
            (*result).err = 2.2250738585072014e-308f64;
            gsl_error(
                b"underflow\0" as *const u8 as *const libc::c_char,
                b"exp.c\0" as *const u8 as *const libc::c_char,
                165 as libc::c_int,
                GSL_EUNDRFLW as libc::c_int,
            );
            return GSL_EUNDRFLW as libc::c_int;
        } else {
            let sy: libc::c_double = (if y >= 0.0f64 {
                1 as libc::c_int
            } else {
                -(1 as libc::c_int)
            }) as libc::c_double;
            let M: libc::c_double = floor(x);
            let N: libc::c_double = floor(ly);
            let a: libc::c_double = x - M;
            let b: libc::c_double = ly - N;
            let berr: libc::c_double = 2.0f64 * 2.2204460492503131e-16f64
                * (fabs(ly) + fabs(N));
            (*result).val = sy * exp(M + N) * exp(a + b);
            (*result).err = berr * fabs((*result).val);
            (*result).err
                += 2.0f64 * 2.2204460492503131e-16f64 * (M + N + 1.0f64)
                    * fabs((*result).val);
            return GSL_SUCCESS as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn gsl_sf_exp_mult_e10_e(
    x: libc::c_double,
    y: libc::c_double,
    mut result: *mut gsl_sf_result_e10,
) -> libc::c_int {
    let ay: libc::c_double = fabs(y);
    if y == 0.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        (*result).e10 = 0 as libc::c_int;
        return GSL_SUCCESS as libc::c_int;
    } else if x < 0.5f64 * 7.0978271289338397e+02f64
        && x > 0.5f64 * -7.0839641853226408e+02f64
        && (ay < 0.8f64 * 1.3407807929942596e+154f64
            && ay > 1.2f64 * 1.4916681462400413e-154f64)
    {
        let ex: libc::c_double = exp(x);
        (*result).val = y * ex;
        (*result)
            .err = (2.0f64 + fabs(x)) * 2.2204460492503131e-16f64 * fabs((*result).val);
        (*result).e10 = 0 as libc::c_int;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let ly: libc::c_double = log(ay);
        let l10_val: libc::c_double = (x + ly) / 2.30258509299404568402f64;
        if l10_val > (2147483647 as libc::c_int - 1 as libc::c_int) as libc::c_double {
            (*result).val = ::std::f32::INFINITY as libc::c_double;
            (*result).err = ::std::f32::INFINITY as libc::c_double;
            (*result).e10 = 0 as libc::c_int;
            gsl_error(
                b"overflow\0" as *const u8 as *const libc::c_char,
                b"exp.c\0" as *const u8 as *const libc::c_char,
                207 as libc::c_int,
                GSL_EOVRFLW as libc::c_int,
            );
            return GSL_EOVRFLW as libc::c_int;
        } else if l10_val
            < (-(2147483647 as libc::c_int) - 1 as libc::c_int + 1 as libc::c_int)
                as libc::c_double
        {
            (*result).val = 0.0f64;
            (*result).err = 2.2250738585072014e-308f64;
            (*result).e10 = 0 as libc::c_int;
            gsl_error(
                b"underflow\0" as *const u8 as *const libc::c_char,
                b"exp.c\0" as *const u8 as *const libc::c_char,
                210 as libc::c_int,
                GSL_EUNDRFLW as libc::c_int,
            );
            return GSL_EUNDRFLW as libc::c_int;
        } else {
            let sy: libc::c_double = (if y >= 0.0f64 {
                1 as libc::c_int
            } else {
                -(1 as libc::c_int)
            }) as libc::c_double;
            let N: libc::c_int = floor(l10_val) as libc::c_int;
            let arg_val: libc::c_double = (l10_val - N as libc::c_double)
                * 2.30258509299404568402f64;
            let arg_err: libc::c_double = 2.0f64 * 2.2204460492503131e-16f64
                * (fabs(x) + fabs(ly)
                    + 2.30258509299404568402f64 * fabs(N as libc::c_double));
            (*result).val = sy * exp(arg_val);
            (*result).err = arg_err * fabs((*result).val);
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
            (*result).e10 = N;
            return GSL_SUCCESS as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn gsl_sf_exp_mult_err_e(
    x: libc::c_double,
    dx: libc::c_double,
    y: libc::c_double,
    dy: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let ay: libc::c_double = fabs(y);
    if y == 0.0f64 {
        (*result).val = 0.0f64;
        (*result).err = fabs(dy * exp(x));
        return GSL_SUCCESS as libc::c_int;
    } else if x < 0.5f64 * 7.0978271289338397e+02f64
        && x > 0.5f64 * -7.0839641853226408e+02f64
        && (ay < 0.8f64 * 1.3407807929942596e+154f64
            && ay > 1.2f64 * 1.4916681462400413e-154f64)
    {
        let mut ex: libc::c_double = exp(x);
        (*result).val = y * ex;
        (*result).err = ex * (fabs(dy) + fabs(y * dx));
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let ly: libc::c_double = log(ay);
        let lnr: libc::c_double = x + ly;
        if lnr > 7.0978271289338397e+02f64 - 0.01f64 {
            (*result).val = ::std::f32::INFINITY as libc::c_double;
            (*result).err = ::std::f32::INFINITY as libc::c_double;
            gsl_error(
                b"overflow\0" as *const u8 as *const libc::c_char,
                b"exp.c\0" as *const u8 as *const libc::c_char,
                254 as libc::c_int,
                GSL_EOVRFLW as libc::c_int,
            );
            return GSL_EOVRFLW as libc::c_int;
        } else if lnr < -7.0839641853226408e+02f64 + 0.01f64 {
            (*result).val = 0.0f64;
            (*result).err = 2.2250738585072014e-308f64;
            gsl_error(
                b"underflow\0" as *const u8 as *const libc::c_char,
                b"exp.c\0" as *const u8 as *const libc::c_char,
                257 as libc::c_int,
                GSL_EUNDRFLW as libc::c_int,
            );
            return GSL_EUNDRFLW as libc::c_int;
        } else {
            let sy: libc::c_double = (if y >= 0.0f64 {
                1 as libc::c_int
            } else {
                -(1 as libc::c_int)
            }) as libc::c_double;
            let M: libc::c_double = floor(x);
            let N: libc::c_double = floor(ly);
            let a: libc::c_double = x - M;
            let b: libc::c_double = ly - N;
            let eMN: libc::c_double = exp(M + N);
            let eab: libc::c_double = exp(a + b);
            (*result).val = sy * eMN * eab;
            (*result).err = eMN * eab * 2.0f64 * 2.2204460492503131e-16f64;
            (*result).err += eMN * eab * fabs(dy / y);
            (*result).err += eMN * eab * fabs(dx);
            return GSL_SUCCESS as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn gsl_sf_exp_mult_err_e10_e(
    x: libc::c_double,
    dx: libc::c_double,
    y: libc::c_double,
    dy: libc::c_double,
    mut result: *mut gsl_sf_result_e10,
) -> libc::c_int {
    let ay: libc::c_double = fabs(y);
    if y == 0.0f64 {
        (*result).val = 0.0f64;
        (*result).err = fabs(dy * exp(x));
        (*result).e10 = 0 as libc::c_int;
        return GSL_SUCCESS as libc::c_int;
    } else if x < 0.5f64 * 7.0978271289338397e+02f64
        && x > 0.5f64 * -7.0839641853226408e+02f64
        && (ay < 0.8f64 * 1.3407807929942596e+154f64
            && ay > 1.2f64 * 1.4916681462400413e-154f64)
    {
        let ex: libc::c_double = exp(x);
        (*result).val = y * ex;
        (*result).err = ex * (fabs(dy) + fabs(y * dx));
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        (*result).e10 = 0 as libc::c_int;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let ly: libc::c_double = log(ay);
        let l10_val: libc::c_double = (x + ly) / 2.30258509299404568402f64;
        if l10_val > (2147483647 as libc::c_int - 1 as libc::c_int) as libc::c_double {
            (*result).val = ::std::f32::INFINITY as libc::c_double;
            (*result).err = ::std::f32::INFINITY as libc::c_double;
            (*result).e10 = 0 as libc::c_int;
            gsl_error(
                b"overflow\0" as *const u8 as *const libc::c_char,
                b"exp.c\0" as *const u8 as *const libc::c_char,
                304 as libc::c_int,
                GSL_EOVRFLW as libc::c_int,
            );
            return GSL_EOVRFLW as libc::c_int;
        } else if l10_val
            < (-(2147483647 as libc::c_int) - 1 as libc::c_int + 1 as libc::c_int)
                as libc::c_double
        {
            (*result).val = 0.0f64;
            (*result).err = 2.2250738585072014e-308f64;
            (*result).e10 = 0 as libc::c_int;
            gsl_error(
                b"underflow\0" as *const u8 as *const libc::c_char,
                b"exp.c\0" as *const u8 as *const libc::c_char,
                307 as libc::c_int,
                GSL_EUNDRFLW as libc::c_int,
            );
            return GSL_EUNDRFLW as libc::c_int;
        } else {
            let sy: libc::c_double = (if y >= 0.0f64 {
                1 as libc::c_int
            } else {
                -(1 as libc::c_int)
            }) as libc::c_double;
            let N: libc::c_int = floor(l10_val) as libc::c_int;
            let arg_val: libc::c_double = (l10_val - N as libc::c_double)
                * 2.30258509299404568402f64;
            let arg_err: libc::c_double = dy / fabs(y) + dx
                + 2.0f64 * 2.2204460492503131e-16f64 * fabs(arg_val);
            (*result).val = sy * exp(arg_val);
            (*result).err = arg_err * fabs((*result).val);
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
            (*result).e10 = N;
            return GSL_SUCCESS as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn gsl_sf_expm1_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let cut: libc::c_double = 0.002f64;
    if x < -7.0839641853226408e+02f64 {
        (*result).val = -1.0f64;
        (*result).err = 2.2204460492503131e-16f64;
        return GSL_SUCCESS as libc::c_int;
    } else if x < -cut {
        (*result).val = exp(x) - 1.0f64;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x < cut {
        (*result)
            .val = x
            * (1.0f64
                + 0.5f64 * x
                    * (1.0f64
                        + x / 3.0f64 * (1.0f64 + 0.25f64 * x * (1.0f64 + 0.2f64 * x))));
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x < 7.0978271289338397e+02f64 {
        (*result).val = exp(x) - 1.0f64;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*result).val = ::std::f32::INFINITY as libc::c_double;
        (*result).err = ::std::f32::INFINITY as libc::c_double;
        gsl_error(
            b"overflow\0" as *const u8 as *const libc::c_char,
            b"exp.c\0" as *const u8 as *const libc::c_char,
            351 as libc::c_int,
            GSL_EOVRFLW as libc::c_int,
        );
        return GSL_EOVRFLW as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_exprel_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let cut: libc::c_double = 0.002f64;
    if x < -7.0839641853226408e+02f64 {
        (*result).val = -1.0f64 / x;
        (*result).err = 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x < -cut {
        (*result).val = (exp(x) - 1.0f64) / x;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x < cut {
        (*result)
            .val = 1.0f64
            + 0.5f64 * x
                * (1.0f64 + x / 3.0f64 * (1.0f64 + 0.25f64 * x * (1.0f64 + 0.2f64 * x)));
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x < 7.0978271289338397e+02f64 {
        (*result).val = (exp(x) - 1.0f64) / x;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*result).val = ::std::f32::INFINITY as libc::c_double;
        (*result).err = ::std::f32::INFINITY as libc::c_double;
        gsl_error(
            b"overflow\0" as *const u8 as *const libc::c_char,
            b"exp.c\0" as *const u8 as *const libc::c_char,
            381 as libc::c_int,
            GSL_EOVRFLW as libc::c_int,
        );
        return GSL_EOVRFLW as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_exprel_2_e(
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let cut: libc::c_double = 0.002f64;
    if x < -7.0839641853226408e+02f64 {
        (*result).val = -2.0f64 / x * (1.0f64 + 1.0f64 / x);
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x < -cut {
        (*result).val = 2.0f64 * (exp(x) - 1.0f64 - x) / (x * x);
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x < cut {
        (*result)
            .val = 1.0f64
            + 1.0f64 / 3.0f64 * x
                * (1.0f64
                    + 0.25f64 * x
                        * (1.0f64 + 0.2f64 * x * (1.0f64 + 1.0f64 / 6.0f64 * x)));
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x < 7.0978271289338397e+02f64 {
        (*result).val = 2.0f64 * (exp(x) - 1.0f64 - x) / (x * x);
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*result).val = ::std::f32::INFINITY as libc::c_double;
        (*result).err = ::std::f32::INFINITY as libc::c_double;
        gsl_error(
            b"overflow\0" as *const u8 as *const libc::c_char,
            b"exp.c\0" as *const u8 as *const libc::c_char,
            411 as libc::c_int,
            GSL_EOVRFLW as libc::c_int,
        );
        return GSL_EOVRFLW as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_exprel_n_CF_e(
    N: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    return exprel_n_CF(N, x, result);
}
pub unsafe extern "C" fn gsl_sf_exprel_n_e(
    N: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if N < 0 as libc::c_int {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"exp.c\0" as *const u8 as *const libc::c_char,
            426 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x == 0.0f64 {
        (*result).val = 1.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if fabs(x) < 6.0554544523933429e-06f64 * N as libc::c_double {
        (*result)
            .val = 1.0f64
            + x / (N + 1 as libc::c_int) as libc::c_double
                * (1.0f64 + x / (N + 2 as libc::c_int) as libc::c_double);
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64;
        return GSL_SUCCESS as libc::c_int;
    } else if N == 0 as libc::c_int {
        return gsl_sf_exp_e(x, result)
    } else if N == 1 as libc::c_int {
        return gsl_sf_exprel_e(x, result)
    } else if N == 2 as libc::c_int {
        return gsl_sf_exprel_2_e(x, result)
    } else if x > N as libc::c_double
        && -x + N as libc::c_double * (1.0f64 + log(x / N as libc::c_double))
            < -3.6043653389117154e+01f64
    {
        let mut lnf_N: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut lnr_val: libc::c_double = 0.;
        let mut lnr_err: libc::c_double = 0.;
        let mut lnterm: libc::c_double = 0.;
        gsl_sf_lnfact_e(N as libc::c_uint, &mut lnf_N);
        lnterm = N as libc::c_double * log(x);
        lnr_val = x + lnf_N.val - lnterm;
        lnr_err = 2.2204460492503131e-16f64 * (fabs(x) + fabs(lnf_N.val) + fabs(lnterm));
        lnr_err += lnf_N.err;
        return gsl_sf_exp_err_e(lnr_val, lnr_err, result);
    } else if x > N as libc::c_double {
        let mut ln_x: libc::c_double = log(x);
        let mut lnf_N_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut lg_N: libc::c_double = 0.;
        let mut lnpre_val: libc::c_double = 0.;
        let mut lnpre_err: libc::c_double = 0.;
        gsl_sf_lnfact_e(N as libc::c_uint, &mut lnf_N_0);
        lg_N = lnf_N_0.val - log(N as libc::c_double);
        lnpre_val = x + lnf_N_0.val - N as libc::c_double * ln_x;
        lnpre_err = 2.2204460492503131e-16f64
            * (fabs(x) + fabs(lnf_N_0.val) + fabs(N as libc::c_double * ln_x));
        lnpre_err += lnf_N_0.err;
        if lnpre_val < 7.0978271289338397e+02f64 - 5.0f64 {
            let mut stat_eG: libc::c_int = 0;
            let mut bigG_ratio: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut pre: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut stat_ex: libc::c_int = gsl_sf_exp_err_e(
                lnpre_val,
                lnpre_err,
                &mut pre,
            );
            let mut ln_bigG_ratio_pre: libc::c_double = -x
                + (N - 1 as libc::c_int) as libc::c_double * ln_x - lg_N;
            let mut bigGsum: libc::c_double = 1.0f64;
            let mut term: libc::c_double = 1.0f64;
            let mut k: libc::c_int = 0;
            k = 1 as libc::c_int;
            while k < N {
                term *= (N - k) as libc::c_double / x;
                bigGsum += term;
                k += 1;
                k;
            }
            stat_eG = gsl_sf_exp_mult_e(ln_bigG_ratio_pre, bigGsum, &mut bigG_ratio);
            if stat_eG == GSL_SUCCESS as libc::c_int {
                (*result).val = pre.val * (1.0f64 - bigG_ratio.val);
                (*result)
                    .err = pre.val
                    * (2.0f64 * 2.2204460492503131e-16f64 + bigG_ratio.err);
                (*result).err += pre.err * fabs(1.0f64 - bigG_ratio.val);
                (*result).err
                    += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
                return stat_ex;
            } else {
                (*result).val = 0.0f64;
                (*result).err = 0.0f64;
                return stat_eG;
            }
        } else {
            (*result).val = ::std::f32::INFINITY as libc::c_double;
            (*result).err = ::std::f32::INFINITY as libc::c_double;
            gsl_error(
                b"overflow\0" as *const u8 as *const libc::c_char,
                b"exp.c\0" as *const u8 as *const libc::c_char,
                508 as libc::c_int,
                GSL_EOVRFLW as libc::c_int,
            );
            return GSL_EOVRFLW as libc::c_int;
        }
    } else if x > -10.0f64 * N as libc::c_double {
        return exprel_n_CF(N as libc::c_double, x, result)
    } else {
        let mut sum: libc::c_double = 1.0f64;
        let mut term_0: libc::c_double = 1.0f64;
        let mut k_0: libc::c_int = 0;
        k_0 = 1 as libc::c_int;
        while k_0 < N {
            term_0 *= (N - k_0) as libc::c_double / x;
            sum += term_0;
            k_0 += 1;
            k_0;
        }
        (*result).val = -N as libc::c_double / x * sum;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_exp_err_e(
    x: libc::c_double,
    dx: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let adx: libc::c_double = fabs(dx);
    if x + adx > 7.0978271289338397e+02f64 {
        (*result).val = ::std::f32::INFINITY as libc::c_double;
        (*result).err = ::std::f32::INFINITY as libc::c_double;
        gsl_error(
            b"overflow\0" as *const u8 as *const libc::c_char,
            b"exp.c\0" as *const u8 as *const libc::c_char,
            542 as libc::c_int,
            GSL_EOVRFLW as libc::c_int,
        );
        return GSL_EOVRFLW as libc::c_int;
    } else if x - adx < -7.0839641853226408e+02f64 {
        (*result).val = 0.0f64;
        (*result).err = 2.2250738585072014e-308f64;
        gsl_error(
            b"underflow\0" as *const u8 as *const libc::c_char,
            b"exp.c\0" as *const u8 as *const libc::c_char,
            545 as libc::c_int,
            GSL_EUNDRFLW as libc::c_int,
        );
        return GSL_EUNDRFLW as libc::c_int;
    } else {
        let ex: libc::c_double = exp(x);
        let edx: libc::c_double = exp(adx);
        (*result).val = ex;
        (*result).err = ex * GSL_MAX_DBL(2.2204460492503131e-16f64, edx - 1.0f64 / edx);
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_exp_err_e10_e(
    x: libc::c_double,
    dx: libc::c_double,
    mut result: *mut gsl_sf_result_e10,
) -> libc::c_int {
    let adx: libc::c_double = fabs(dx);
    if x + adx > (2147483647 as libc::c_int - 1 as libc::c_int) as libc::c_double {
        (*result).val = ::std::f32::INFINITY as libc::c_double;
        (*result).err = ::std::f32::INFINITY as libc::c_double;
        (*result).e10 = 0 as libc::c_int;
        gsl_error(
            b"overflow\0" as *const u8 as *const libc::c_char,
            b"exp.c\0" as *const u8 as *const libc::c_char,
            566 as libc::c_int,
            GSL_EOVRFLW as libc::c_int,
        );
        return GSL_EOVRFLW as libc::c_int;
    } else if x - adx
        < (-(2147483647 as libc::c_int) - 1 as libc::c_int + 1 as libc::c_int)
            as libc::c_double
    {
        (*result).val = 0.0f64;
        (*result).err = 2.2250738585072014e-308f64;
        (*result).e10 = 0 as libc::c_int;
        gsl_error(
            b"underflow\0" as *const u8 as *const libc::c_char,
            b"exp.c\0" as *const u8 as *const libc::c_char,
            569 as libc::c_int,
            GSL_EUNDRFLW as libc::c_int,
        );
        return GSL_EUNDRFLW as libc::c_int;
    } else {
        let N: libc::c_int = floor(x / 2.30258509299404568402f64) as libc::c_int;
        let ex: libc::c_double = exp(
            x - N as libc::c_double * 2.30258509299404568402f64,
        );
        (*result).val = ex;
        (*result)
            .err = ex * (2.0f64 * 2.2204460492503131e-16f64 * (fabs(x) + 1.0f64) + adx);
        (*result).e10 = N;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_exp(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_exp_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_exp_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"exp.c\0" as *const u8 as *const libc::c_char,
            588 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_exp_mult(
    x: libc::c_double,
    y: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_exp_mult_e(x, y, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_exp_mult_e(x, y, &result)\0" as *const u8 as *const libc::c_char,
            b"exp.c\0" as *const u8 as *const libc::c_char,
            593 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_expm1(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_expm1_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_expm1_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"exp.c\0" as *const u8 as *const libc::c_char,
            598 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_exprel(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_exprel_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_exprel_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"exp.c\0" as *const u8 as *const libc::c_char,
            603 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_exprel_2(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_exprel_2_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_exprel_2_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"exp.c\0" as *const u8 as *const libc::c_char,
            608 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_exprel_n(
    n: libc::c_int,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_exprel_n_e(n, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_exprel_n_e(n, x, &result)\0" as *const u8 as *const libc::c_char,
            b"exp.c\0" as *const u8 as *const libc::c_char,
            613 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
