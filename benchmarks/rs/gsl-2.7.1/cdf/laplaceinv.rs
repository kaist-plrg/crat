use ::libc;
extern "C" {
    fn log(_: libc::c_double) -> libc::c_double;
}
pub unsafe extern "C" fn gsl_cdf_laplace_Pinv(
    P: libc::c_double,
    a: libc::c_double,
) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    if P == 1.0f64 {
        return ::std::f32::INFINITY as libc::c_double
    } else if P == 0.0f64 {
        return -::std::f32::INFINITY as libc::c_double
    }
    if P < 0.5f64 {
        x = a * log(2 as libc::c_int as libc::c_double * P);
    } else {
        x = -a
            * log(
                2 as libc::c_int as libc::c_double
                    * (1 as libc::c_int as libc::c_double - P),
            );
    }
    return x;
}
pub unsafe extern "C" fn gsl_cdf_laplace_Qinv(
    Q: libc::c_double,
    a: libc::c_double,
) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    if Q == 0.0f64 {
        return ::std::f32::INFINITY as libc::c_double
    } else if Q == 1.0f64 {
        return -::std::f32::INFINITY as libc::c_double
    }
    if Q < 0.5f64 {
        x = -a * log(2 as libc::c_int as libc::c_double * Q);
    } else {
        x = a
            * log(
                2 as libc::c_int as libc::c_double
                    * (1 as libc::c_int as libc::c_double - Q),
            );
    }
    return x;
}
