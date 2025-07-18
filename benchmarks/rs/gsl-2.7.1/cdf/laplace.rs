use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
}
pub unsafe extern "C" fn gsl_cdf_laplace_P(
    x: libc::c_double,
    a: libc::c_double,
) -> libc::c_double {
    let mut P: libc::c_double = 0.;
    let mut u: libc::c_double = x / a;
    if u > 0 as libc::c_int as libc::c_double {
        P = 0.5f64 + 0.5f64 * (1 as libc::c_int as libc::c_double - exp(-u));
    } else {
        P = 0.5f64 * exp(u);
    }
    return P;
}
pub unsafe extern "C" fn gsl_cdf_laplace_Q(
    x: libc::c_double,
    a: libc::c_double,
) -> libc::c_double {
    let mut Q: libc::c_double = 0.;
    let mut u: libc::c_double = x / a;
    if u > 0 as libc::c_int as libc::c_double {
        Q = 0.5f64 * exp(-u);
    } else {
        Q = 1 as libc::c_int as libc::c_double - 0.5f64 * exp(u);
    }
    return Q;
}
