use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn log1p(_: libc::c_double) -> libc::c_double;
}
pub unsafe extern "C" fn gsl_cdf_pareto_Pinv(
    P: libc::c_double,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    if P == 1.0f64 {
        return ::std::f32::INFINITY as libc::c_double
    } else if P == 0.0f64 {
        return b
    }
    x = b * exp(-log1p(-P) / a);
    return x;
}
pub unsafe extern "C" fn gsl_cdf_pareto_Qinv(
    Q: libc::c_double,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    if Q == 0.0f64 {
        return ::std::f32::INFINITY as libc::c_double
    } else if Q == 1.0f64 {
        return b
    }
    x = b * exp(-log(Q) / a);
    return x;
}
