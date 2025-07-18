use ::libc;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_histogram2d {
    pub nx: size_t,
    pub ny: size_t,
    pub xrange: *mut libc::c_double,
    pub yrange: *mut libc::c_double,
    pub bin: *mut libc::c_double,
}
pub unsafe extern "C" fn gsl_histogram2d_xmax(
    mut h: *const gsl_histogram2d,
) -> libc::c_double {
    let nx: libc::c_int = (*h).nx as libc::c_int;
    return *((*h).xrange).offset(nx as isize);
}
pub unsafe extern "C" fn gsl_histogram2d_xmin(
    mut h: *const gsl_histogram2d,
) -> libc::c_double {
    return *((*h).xrange).offset(0 as libc::c_int as isize);
}
pub unsafe extern "C" fn gsl_histogram2d_ymax(
    mut h: *const gsl_histogram2d,
) -> libc::c_double {
    let ny: libc::c_int = (*h).ny as libc::c_int;
    return *((*h).yrange).offset(ny as isize);
}
pub unsafe extern "C" fn gsl_histogram2d_ymin(
    mut h: *const gsl_histogram2d,
) -> libc::c_double {
    return *((*h).yrange).offset(0 as libc::c_int as isize);
}
pub unsafe extern "C" fn gsl_histogram2d_nx(mut h: *const gsl_histogram2d) -> size_t {
    return (*h).nx;
}
pub unsafe extern "C" fn gsl_histogram2d_ny(mut h: *const gsl_histogram2d) -> size_t {
    return (*h).ny;
}
