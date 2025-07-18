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
pub unsafe extern "C" fn gsl_histogram2d_reset(mut h: *mut gsl_histogram2d) {
    let mut i: size_t = 0;
    let nx: size_t = (*h).nx;
    let ny: size_t = (*h).ny;
    i = 0 as libc::c_int as size_t;
    while i < nx.wrapping_mul(ny) {
        *((*h).bin).offset(i as isize) = 0 as libc::c_int as libc::c_double;
        i = i.wrapping_add(1);
        i;
    }
}
