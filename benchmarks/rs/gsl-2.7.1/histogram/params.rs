use ::libc;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_histogram {
    pub n: size_t,
    pub range: *mut libc::c_double,
    pub bin: *mut libc::c_double,
}
pub unsafe extern "C" fn gsl_histogram_max(
    mut h: *const gsl_histogram,
) -> libc::c_double {
    let n: libc::c_int = (*h).n as libc::c_int;
    return *((*h).range).offset(n as isize);
}
pub unsafe extern "C" fn gsl_histogram_min(
    mut h: *const gsl_histogram,
) -> libc::c_double {
    return *((*h).range).offset(0 as libc::c_int as isize);
}
pub unsafe extern "C" fn gsl_histogram_bins(mut h: *const gsl_histogram) -> size_t {
    return (*h).n;
}
