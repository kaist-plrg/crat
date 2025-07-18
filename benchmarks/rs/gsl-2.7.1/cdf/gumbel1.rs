use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn expm1(_: libc::c_double) -> libc::c_double;
}
pub unsafe extern "C" fn gsl_cdf_gumbel1_P(
    x: libc::c_double,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    let mut u: libc::c_double = a * x - log(b);
    let mut P: libc::c_double = exp(-exp(-u));
    return P;
}
pub unsafe extern "C" fn gsl_cdf_gumbel1_Q(
    x: libc::c_double,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    let mut u: libc::c_double = a * x - log(b);
    let mut Q: libc::c_double = 0.;
    let mut P: libc::c_double = exp(-exp(-u));
    if P < 0.5f64 {
        Q = 1 as libc::c_int as libc::c_double - P;
    } else {
        Q = -expm1(-exp(-u));
    }
    return Q;
}
