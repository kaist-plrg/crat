use ::libc;
extern "C" {
    fn log(_: libc::c_double) -> libc::c_double;
}
pub unsafe extern "C" fn gsl_log1p(x: libc::c_double) -> libc::c_double {
    let mut y: libc::c_double = 0.;
    let mut z: libc::c_double = 0.;
    ::std::ptr::write_volatile(
        &mut y as *mut libc::c_double,
        1 as libc::c_int as libc::c_double + x,
    );
    ::std::ptr::write_volatile(
        &mut z as *mut libc::c_double,
        y - 1 as libc::c_int as libc::c_double,
    );
    return log(y) - (z - x) / y;
}
