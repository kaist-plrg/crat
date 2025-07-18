use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn expm1(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
pub unsafe extern "C" fn gsl_cdf_weibull_P(
    x: libc::c_double,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    let mut P: libc::c_double = -expm1(-pow(x / a, b));
    return P;
}
pub unsafe extern "C" fn gsl_cdf_weibull_Q(
    x: libc::c_double,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    let mut Q: libc::c_double = exp(-pow(x / a, b));
    return Q;
}
