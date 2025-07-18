use ::libc;
pub unsafe extern "C" fn gsl_fdiv(
    x: libc::c_double,
    y: libc::c_double,
) -> libc::c_double {
    return x / y;
}
