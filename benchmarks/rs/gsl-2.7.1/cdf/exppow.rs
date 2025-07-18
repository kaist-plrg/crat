use ::libc;
extern "C" {
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn gsl_sf_gamma_inc_Q(a: libc::c_double, x: libc::c_double) -> libc::c_double;
    fn gsl_sf_gamma_inc_P(a: libc::c_double, x: libc::c_double) -> libc::c_double;
}
pub unsafe extern "C" fn gsl_cdf_exppow_P(
    x: libc::c_double,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    let u: libc::c_double = x / a;
    if u < 0 as libc::c_int as libc::c_double {
        let mut P: libc::c_double = 0.5f64 * gsl_sf_gamma_inc_Q(1.0f64 / b, pow(-u, b));
        return P;
    } else {
        let mut P_0: libc::c_double = 0.5f64
            * (1.0f64 + gsl_sf_gamma_inc_P(1.0f64 / b, pow(u, b)));
        return P_0;
    };
}
pub unsafe extern "C" fn gsl_cdf_exppow_Q(
    x: libc::c_double,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    let u: libc::c_double = x / a;
    if u < 0 as libc::c_int as libc::c_double {
        let mut Q: libc::c_double = 0.5f64
            * (1.0f64 + gsl_sf_gamma_inc_P(1.0f64 / b, pow(-u, b)));
        return Q;
    } else {
        let mut Q_0: libc::c_double = 0.5f64 * gsl_sf_gamma_inc_Q(1.0f64 / b, pow(u, b));
        return Q_0;
    };
}
