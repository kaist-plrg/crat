use ::libc;
extern "C" {
    fn log(_: libc::c_double) -> libc::c_double;
    fn log1p(_: libc::c_double) -> libc::c_double;
}
pub unsafe extern "C" fn gsl_cdf_exponential_Pinv(
    P: libc::c_double,
    mu: libc::c_double,
) -> libc::c_double {
    let mut x: libc::c_double = -mu * log1p(-P);
    return x;
}
pub unsafe extern "C" fn gsl_cdf_exponential_Qinv(
    Q: libc::c_double,
    mu: libc::c_double,
) -> libc::c_double {
    let mut x: libc::c_double = -mu * log(Q);
    return x;
}
