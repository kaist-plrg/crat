use ::libc;
pub unsafe extern "C" fn gsl_utils_placeholder() {
    let mut i: libc::c_int = 0 as libc::c_int;
    i += 1;
    i;
}
