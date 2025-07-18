use ::libc;
pub unsafe extern "C" fn gsl_coerce_double(x: libc::c_double) -> libc::c_double {
    let mut y: libc::c_double = 0.;
    ::std::ptr::write_volatile(&mut y as *mut libc::c_double, x);
    return y;
}
pub unsafe extern "C" fn gsl_coerce_float(x: libc::c_float) -> libc::c_float {
    let mut y: libc::c_float = 0.;
    ::std::ptr::write_volatile(&mut y as *mut libc::c_float, x);
    return y;
}
pub unsafe extern "C" fn gsl_coerce_long_double(x: f128::f128) -> f128::f128 {
    let mut y: f128::f128 = f128::f128::ZERO;
    ::std::ptr::write_volatile(&mut y as *mut f128::f128, x);
    return y;
}
