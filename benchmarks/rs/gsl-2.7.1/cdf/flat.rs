use ::libc;
pub unsafe extern "C" fn gsl_cdf_flat_P(
    x: libc::c_double,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    let mut P: libc::c_double = 0.;
    if x < a {
        P = 0 as libc::c_int as libc::c_double;
    } else if x > b {
        P = 1 as libc::c_int as libc::c_double;
    } else {
        P = (x - a) / (b - a);
    }
    return P;
}
pub unsafe extern "C" fn gsl_cdf_flat_Q(
    x: libc::c_double,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    let mut Q: libc::c_double = 0.;
    if x < a {
        Q = 1 as libc::c_int as libc::c_double;
    } else if x > b {
        Q = 0 as libc::c_int as libc::c_double;
    } else {
        Q = (b - x) / (b - a);
    }
    return Q;
}
