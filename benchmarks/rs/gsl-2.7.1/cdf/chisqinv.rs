use ::libc;
extern "C" {
    fn gsl_cdf_gamma_Pinv(
        P: libc::c_double,
        a: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_double;
    fn gsl_cdf_gamma_Qinv(
        Q: libc::c_double,
        a: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_double;
}
pub unsafe extern "C" fn gsl_cdf_chisq_Pinv(
    P: libc::c_double,
    nu: libc::c_double,
) -> libc::c_double {
    return gsl_cdf_gamma_Pinv(P, nu / 2 as libc::c_int as libc::c_double, 2.0f64);
}
pub unsafe extern "C" fn gsl_cdf_chisq_Qinv(
    Q: libc::c_double,
    nu: libc::c_double,
) -> libc::c_double {
    return gsl_cdf_gamma_Qinv(Q, nu / 2 as libc::c_int as libc::c_double, 2.0f64);
}
