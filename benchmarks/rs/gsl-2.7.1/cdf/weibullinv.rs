use ::libc;
extern "C" {
    fn log(_: libc::c_double) -> libc::c_double;
    fn log1p(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
pub unsafe extern "C" fn gsl_cdf_weibull_Pinv(
    P: libc::c_double,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    if P == 1.0f64 {
        return ::std::f32::INFINITY as libc::c_double
    } else if P == 0.0f64 {
        return 0.0f64
    }
    x = a * pow(-log1p(-P), 1 as libc::c_int as libc::c_double / b);
    return x;
}
pub unsafe extern "C" fn gsl_cdf_weibull_Qinv(
    Q: libc::c_double,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    if Q == 0.0f64 {
        return ::std::f32::INFINITY as libc::c_double
    } else if Q == 1.0f64 {
        return 0.0f64
    }
    x = a * pow(-log(Q), 1 as libc::c_int as libc::c_double / b);
    return x;
}
