use ::libc;
extern "C" {
    fn gsl_sf_gamma_inc_Q(a: libc::c_double, x: libc::c_double) -> libc::c_double;
    fn gsl_sf_gamma_inc_P(a: libc::c_double, x: libc::c_double) -> libc::c_double;
}
pub unsafe extern "C" fn gsl_cdf_gamma_P(
    x: libc::c_double,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    let mut P: libc::c_double = 0.;
    let mut y: libc::c_double = x / b;
    if x <= 0.0f64 {
        return 0.0f64;
    }
    if y > a {
        P = 1 as libc::c_int as libc::c_double - gsl_sf_gamma_inc_Q(a, y);
    } else {
        P = gsl_sf_gamma_inc_P(a, y);
    }
    return P;
}
pub unsafe extern "C" fn gsl_cdf_gamma_Q(
    x: libc::c_double,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    let mut Q: libc::c_double = 0.;
    let mut y: libc::c_double = x / b;
    if x <= 0.0f64 {
        return 1.0f64;
    }
    if y < a {
        Q = 1 as libc::c_int as libc::c_double - gsl_sf_gamma_inc_P(a, y);
    } else {
        Q = gsl_sf_gamma_inc_Q(a, y);
    }
    return Q;
}
