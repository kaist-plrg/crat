use ::libc;
extern "C" {
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
pub unsafe extern "C" fn gsl_cdf_pareto_P(
    x: libc::c_double,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    let mut P: libc::c_double = 0.;
    if x < b {
        P = 0 as libc::c_int as libc::c_double;
    } else {
        P = 1 as libc::c_int as libc::c_double - pow(b / x, a);
    }
    return P;
}
pub unsafe extern "C" fn gsl_cdf_pareto_Q(
    x: libc::c_double,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    let mut Q: libc::c_double = 0.;
    if x < b {
        Q = 1 as libc::c_int as libc::c_double;
    } else {
        Q = pow(b / x, a);
    }
    return Q;
}
