use ::libc;
extern "C" {
    fn gsl_finite(x: libc::c_double) -> libc::c_int;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_function_struct {
    pub function: Option::<
        unsafe extern "C" fn(libc::c_double, *mut libc::c_void) -> libc::c_double,
    >,
    pub params: *mut libc::c_void,
}
pub type gsl_function = gsl_function_struct;
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
pub unsafe extern "C" fn gsl_min_find_bracket(
    mut f: *mut gsl_function,
    mut x_minimum: *mut libc::c_double,
    mut f_minimum: *mut libc::c_double,
    mut x_lower: *mut libc::c_double,
    mut f_lower: *mut libc::c_double,
    mut x_upper: *mut libc::c_double,
    mut f_upper: *mut libc::c_double,
    mut eval_max: size_t,
) -> libc::c_int {
    let mut f_left: libc::c_double = *f_lower;
    let mut f_right: libc::c_double = *f_upper;
    let mut f_center: libc::c_double = 0.;
    let mut x_left: libc::c_double = *x_lower;
    let mut x_right: libc::c_double = *x_upper;
    let mut x_center: libc::c_double = 0.;
    let golden: libc::c_double = 0.3819660f64;
    let mut nb_eval: size_t = 0 as libc::c_int as size_t;
    if f_right >= f_left {
        x_center = (x_right - x_left) * golden + x_left;
        nb_eval = nb_eval.wrapping_add(1);
        nb_eval;
        ::std::ptr::write_volatile(
            &mut f_center as *mut libc::c_double,
            (Some(((*f).function).unwrap())).unwrap()(x_center, (*f).params),
        );
        if gsl_finite(f_center) == 0 {
            gsl_error(
                b"computed function value is infinite or NaN\0" as *const u8
                    as *const libc::c_char,
                b"bracketing.c\0" as *const u8 as *const libc::c_char,
                58 as libc::c_int,
                GSL_EBADFUNC as libc::c_int,
            );
            return GSL_EBADFUNC as libc::c_int;
        }
    } else {
        x_center = x_right;
        ::std::ptr::write_volatile(&mut f_center as *mut libc::c_double, f_right);
        x_right = (x_center - x_left) / golden + x_left;
        nb_eval = nb_eval.wrapping_add(1);
        nb_eval;
        ::std::ptr::write_volatile(
            &mut f_right as *mut libc::c_double,
            (Some(((*f).function).unwrap())).unwrap()(x_right, (*f).params),
        );
        if gsl_finite(f_right) == 0 {
            gsl_error(
                b"computed function value is infinite or NaN\0" as *const u8
                    as *const libc::c_char,
                b"bracketing.c\0" as *const u8 as *const libc::c_char,
                66 as libc::c_int,
                GSL_EBADFUNC as libc::c_int,
            );
            return GSL_EBADFUNC as libc::c_int;
        }
    }
    loop {
        if f_center < f_left {
            if f_center < f_right {
                *x_lower = x_left;
                *x_upper = x_right;
                *x_minimum = x_center;
                *f_lower = f_left;
                *f_upper = f_right;
                *f_minimum = f_center;
                return GSL_SUCCESS as libc::c_int;
            } else if f_center > f_right {
                x_left = x_center;
                ::std::ptr::write_volatile(&mut f_left as *mut libc::c_double, f_center);
                x_center = x_right;
                ::std::ptr::write_volatile(
                    &mut f_center as *mut libc::c_double,
                    f_right,
                );
                x_right = (x_center - x_left) / golden + x_left;
                nb_eval = nb_eval.wrapping_add(1);
                nb_eval;
                ::std::ptr::write_volatile(
                    &mut f_right as *mut libc::c_double,
                    (Some(((*f).function).unwrap())).unwrap()(x_right, (*f).params),
                );
                if gsl_finite(f_right) == 0 {
                    gsl_error(
                        b"computed function value is infinite or NaN\0" as *const u8
                            as *const libc::c_char,
                        b"bracketing.c\0" as *const u8 as *const libc::c_char,
                        95 as libc::c_int,
                        GSL_EBADFUNC as libc::c_int,
                    );
                    return GSL_EBADFUNC as libc::c_int;
                }
            } else {
                x_right = x_center;
                ::std::ptr::write_volatile(
                    &mut f_right as *mut libc::c_double,
                    f_center,
                );
                x_center = (x_right - x_left) * golden + x_left;
                nb_eval = nb_eval.wrapping_add(1);
                nb_eval;
                ::std::ptr::write_volatile(
                    &mut f_center as *mut libc::c_double,
                    (Some(((*f).function).unwrap())).unwrap()(x_center, (*f).params),
                );
                if gsl_finite(f_center) == 0 {
                    gsl_error(
                        b"computed function value is infinite or NaN\0" as *const u8
                            as *const libc::c_char,
                        b"bracketing.c\0" as *const u8 as *const libc::c_char,
                        103 as libc::c_int,
                        GSL_EBADFUNC as libc::c_int,
                    );
                    return GSL_EBADFUNC as libc::c_int;
                }
            }
        } else {
            x_right = x_center;
            ::std::ptr::write_volatile(&mut f_right as *mut libc::c_double, f_center);
            x_center = (x_right - x_left) * golden + x_left;
            nb_eval = nb_eval.wrapping_add(1);
            nb_eval;
            ::std::ptr::write_volatile(
                &mut f_center as *mut libc::c_double,
                (Some(((*f).function).unwrap())).unwrap()(x_center, (*f).params),
            );
            if gsl_finite(f_center) == 0 {
                gsl_error(
                    b"computed function value is infinite or NaN\0" as *const u8
                        as *const libc::c_char,
                    b"bracketing.c\0" as *const u8 as *const libc::c_char,
                    112 as libc::c_int,
                    GSL_EBADFUNC as libc::c_int,
                );
                return GSL_EBADFUNC as libc::c_int;
            }
        }
        if !(nb_eval < eval_max
            && x_right - x_left
                > 1.4901161193847656e-08f64 * ((x_right + x_left) * 0.5f64)
                    + 1.4901161193847656e-08f64)
        {
            break;
        }
    }
    *x_lower = x_left;
    *x_upper = x_right;
    *x_minimum = x_center;
    *f_lower = f_left;
    *f_upper = f_right;
    *f_minimum = f_center;
    return GSL_FAILURE as libc::c_int;
}
