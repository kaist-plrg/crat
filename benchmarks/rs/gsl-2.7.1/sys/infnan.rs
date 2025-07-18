use ::libc;
extern "C" {
    fn gsl_fdiv(x: libc::c_double, y: libc::c_double) -> libc::c_double;
}
pub unsafe extern "C" fn gsl_nan() -> libc::c_double {
    return gsl_fdiv(0.0f64, 0.0f64);
}
pub unsafe extern "C" fn gsl_posinf() -> libc::c_double {
    return gsl_fdiv(1.0f64, 0.0f64);
}
pub unsafe extern "C" fn gsl_neginf() -> libc::c_double {
    return gsl_fdiv(-1.0f64, 0.0f64);
}
pub unsafe extern "C" fn gsl_finite(x: libc::c_double) -> libc::c_int {
    return x.is_finite() as i32;
}
pub unsafe extern "C" fn gsl_isnan(x: libc::c_double) -> libc::c_int {
    return x.is_nan() as i32;
}
pub unsafe extern "C" fn gsl_isinf(x: libc::c_double) -> libc::c_int {
    if if x.is_infinite() { if x.is_sign_positive() { 1 } else { -1 } } else { 0 } != 0 {
        return if x > 0 as libc::c_int as libc::c_double {
            1 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }
    } else {
        return 0 as libc::c_int
    };
}
