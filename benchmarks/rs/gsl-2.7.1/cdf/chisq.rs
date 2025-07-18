use ::libc;
extern "C" {
    fn gsl_cdf_gamma_P(
        x: libc::c_double,
        a: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_double;
    fn gsl_cdf_gamma_Q(
        x: libc::c_double,
        a: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_double;
}
pub unsafe extern "C" fn gsl_cdf_chisq_P(
    x: libc::c_double,
    nu: libc::c_double,
) -> libc::c_double {
    return gsl_cdf_gamma_P(x, nu / 2 as libc::c_int as libc::c_double, 2.0f64);
}
pub unsafe extern "C" fn gsl_cdf_chisq_Q(
    x: libc::c_double,
    nu: libc::c_double,
) -> libc::c_double {
    return gsl_cdf_gamma_Q(x, nu / 2 as libc::c_int as libc::c_double, 2.0f64);
}
