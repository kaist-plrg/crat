use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
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
pub struct gsl_sum_levin_utrunc_workspace {
    pub size: size_t,
    pub i: size_t,
    pub terms_used: size_t,
    pub sum_plain: libc::c_double,
    pub q_num: *mut libc::c_double,
    pub q_den: *mut libc::c_double,
    pub dsum: *mut libc::c_double,
}
pub unsafe extern "C" fn gsl_sum_levin_utrunc_accel(
    mut array: *const libc::c_double,
    array_size: size_t,
    mut w: *mut gsl_sum_levin_utrunc_workspace,
    mut sum_accel: *mut libc::c_double,
    mut abserr_trunc: *mut libc::c_double,
) -> libc::c_int {
    return gsl_sum_levin_utrunc_minmax(
        array,
        array_size,
        0 as libc::c_int as size_t,
        array_size.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        w,
        sum_accel,
        abserr_trunc,
    );
}
pub unsafe extern "C" fn gsl_sum_levin_utrunc_minmax(
    mut array: *const libc::c_double,
    array_size: size_t,
    min_terms: size_t,
    max_terms: size_t,
    mut w: *mut gsl_sum_levin_utrunc_workspace,
    mut sum_accel: *mut libc::c_double,
    mut abserr_trunc: *mut libc::c_double,
) -> libc::c_int {
    if array_size == 0 as libc::c_int as libc::c_ulong {
        *sum_accel = 0.0f64;
        *abserr_trunc = 0.0f64;
        (*w).sum_plain = 0.0f64;
        (*w).terms_used = 0 as libc::c_int as size_t;
        return GSL_SUCCESS as libc::c_int;
    } else if array_size == 1 as libc::c_int as libc::c_ulong {
        *sum_accel = *array.offset(0 as libc::c_int as isize);
        *abserr_trunc = ::std::f32::INFINITY as libc::c_double;
        (*w).sum_plain = *array.offset(0 as libc::c_int as isize);
        (*w).terms_used = 1 as libc::c_int as size_t;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let SMALL: libc::c_double = 0.01f64;
        let nmax: size_t = (if max_terms > array_size { max_terms } else { array_size })
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut trunc_n: libc::c_double = 0.0f64;
        let mut trunc_nm1: libc::c_double = 0.0f64;
        let mut actual_trunc_n: libc::c_double = 0.0f64;
        let mut actual_trunc_nm1: libc::c_double = 0.0f64;
        let mut result_n: libc::c_double = 0.0f64;
        let mut result_nm1: libc::c_double = 0.0f64;
        let mut n: size_t = 0;
        let mut better: libc::c_int = 0 as libc::c_int;
        let mut before: libc::c_int = 0 as libc::c_int;
        let mut converging: libc::c_int = 0 as libc::c_int;
        let mut least_trunc: libc::c_double = 1.7976931348623157e+308f64;
        let mut result_least_trunc: libc::c_double = 0.;
        n = 0 as libc::c_int as size_t;
        while n < min_terms {
            let t: libc::c_double = *array.offset(n as isize);
            result_nm1 = result_n;
            gsl_sum_levin_utrunc_step(t, n, w, &mut result_n);
            n = n.wrapping_add(1);
            n;
        }
        result_least_trunc = result_n;
        while n <= nmax {
            let t_0: libc::c_double = *array.offset(n as isize);
            result_nm1 = result_n;
            gsl_sum_levin_utrunc_step(t_0, n, w, &mut result_n);
            actual_trunc_nm1 = actual_trunc_n;
            actual_trunc_n = fabs(result_n - result_nm1);
            trunc_nm1 = trunc_n;
            trunc_n = 0.5f64 * (actual_trunc_n + actual_trunc_nm1);
            better = (trunc_n < trunc_nm1 || trunc_n < SMALL * fabs(result_n))
                as libc::c_int;
            converging = (converging != 0 || better != 0 && before != 0) as libc::c_int;
            before = better;
            if converging != 0 {
                if trunc_n < least_trunc {
                    least_trunc = trunc_n;
                    result_least_trunc = result_n;
                }
                if fabs(trunc_n / result_n) < 10.0f64 * 2.2204460492503131e-16f64 {
                    break;
                }
            }
            n = n.wrapping_add(1);
            n;
        }
        if converging != 0 {
            *sum_accel = result_least_trunc;
            *abserr_trunc = least_trunc;
            (*w).terms_used = n;
            return GSL_SUCCESS as libc::c_int;
        } else {
            *sum_accel = result_n;
            *abserr_trunc = trunc_n;
            (*w).terms_used = n;
            return GSL_SUCCESS as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn gsl_sum_levin_utrunc_step(
    term: libc::c_double,
    n: size_t,
    mut w: *mut gsl_sum_levin_utrunc_workspace,
    mut sum_accel: *mut libc::c_double,
) -> libc::c_int {
    if term == 0.0f64 {
        return GSL_EZERODIV as libc::c_int
    } else if n == 0 as libc::c_int as libc::c_ulong {
        *sum_accel = term;
        (*w).sum_plain = term;
        *((*w).q_den).offset(0 as libc::c_int as isize) = 1.0f64 / term;
        *((*w).q_num).offset(0 as libc::c_int as isize) = 1.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut factor: libc::c_double = 1.0f64;
        let mut ratio: libc::c_double = n as libc::c_double
            / (n as libc::c_double + 1.0f64);
        let mut j: libc::c_int = 0;
        (*w).sum_plain += term;
        *((*w).q_den)
            .offset(
                n as isize,
            ) = 1.0f64
            / (term * (n as libc::c_double + 1.0f64) * (n as libc::c_double + 1.0f64));
        *((*w).q_num)
            .offset(n as isize) = (*w).sum_plain * *((*w).q_den).offset(n as isize);
        j = n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
        while j >= 0 as libc::c_int {
            let mut c: libc::c_double = factor * (j + 1 as libc::c_int) as libc::c_double
                / n.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_double;
            factor *= ratio;
            *((*w).q_den)
                .offset(
                    j as isize,
                ) = *((*w).q_den).offset((j + 1 as libc::c_int) as isize)
                - c * *((*w).q_den).offset(j as isize);
            *((*w).q_num)
                .offset(
                    j as isize,
                ) = *((*w).q_num).offset((j + 1 as libc::c_int) as isize)
                - c * *((*w).q_num).offset(j as isize);
            j -= 1;
            j;
        }
        *sum_accel = *((*w).q_num).offset(0 as libc::c_int as isize)
            / *((*w).q_den).offset(0 as libc::c_int as isize);
        return GSL_SUCCESS as libc::c_int;
    };
}
