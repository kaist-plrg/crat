use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn expm1(_: libc::c_double) -> libc::c_double;
}
pub unsafe extern "C" fn gsl_cdf_exponential_P(
    x: libc::c_double,
    mu: libc::c_double,
) -> libc::c_double {
    if x < 0 as libc::c_int as libc::c_double {
        return 0 as libc::c_int as libc::c_double
    } else {
        let mut P: libc::c_double = -expm1(-x / mu);
        return P;
    };
}
pub unsafe extern "C" fn gsl_cdf_exponential_Q(
    x: libc::c_double,
    mu: libc::c_double,
) -> libc::c_double {
    if x < 0 as libc::c_int as libc::c_double {
        return 1 as libc::c_int as libc::c_double
    } else {
        let mut Q: libc::c_double = exp(-x / mu);
        return Q;
    };
}
