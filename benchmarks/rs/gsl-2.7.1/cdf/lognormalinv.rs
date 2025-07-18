use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn gsl_cdf_ugaussian_Pinv(P: libc::c_double) -> libc::c_double;
    fn gsl_cdf_ugaussian_Qinv(Q: libc::c_double) -> libc::c_double;
}
pub unsafe extern "C" fn gsl_cdf_lognormal_Pinv(
    P: libc::c_double,
    zeta: libc::c_double,
    sigma: libc::c_double,
) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    let mut u: libc::c_double = 0.;
    if P == 1.0f64 {
        return ::std::f32::INFINITY as libc::c_double
    } else if P == 0.0f64 {
        return 0.0f64
    }
    u = gsl_cdf_ugaussian_Pinv(P);
    x = exp(zeta + sigma * u);
    return x;
}
pub unsafe extern "C" fn gsl_cdf_lognormal_Qinv(
    Q: libc::c_double,
    zeta: libc::c_double,
    sigma: libc::c_double,
) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    let mut u: libc::c_double = 0.;
    if Q == 0.0f64 {
        return ::std::f32::INFINITY as libc::c_double
    } else if Q == 1.0f64 {
        return 0.0f64
    }
    u = gsl_cdf_ugaussian_Qinv(Q);
    x = exp(zeta + sigma * u);
    return x;
}
