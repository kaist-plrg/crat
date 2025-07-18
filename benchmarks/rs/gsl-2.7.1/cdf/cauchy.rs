use ::libc;
extern "C" {
    fn atan(_: libc::c_double) -> libc::c_double;
}
pub unsafe extern "C" fn gsl_cdf_cauchy_P(
    x: libc::c_double,
    a: libc::c_double,
) -> libc::c_double {
    let mut P: libc::c_double = 0.;
    let mut u: libc::c_double = x / a;
    if u > -(1 as libc::c_int) as libc::c_double {
        P = 0.5f64 + atan(u) / 3.14159265358979323846f64;
    } else {
        P = atan(-(1 as libc::c_int) as libc::c_double / u) / 3.14159265358979323846f64;
    }
    return P;
}
pub unsafe extern "C" fn gsl_cdf_cauchy_Q(
    x: libc::c_double,
    a: libc::c_double,
) -> libc::c_double {
    let mut Q: libc::c_double = 0.;
    let mut u: libc::c_double = x / a;
    if u < 1 as libc::c_int as libc::c_double {
        Q = 0.5f64 - atan(u) / 3.14159265358979323846f64;
    } else {
        Q = atan(1 as libc::c_int as libc::c_double / u) / 3.14159265358979323846f64;
    }
    return Q;
}
