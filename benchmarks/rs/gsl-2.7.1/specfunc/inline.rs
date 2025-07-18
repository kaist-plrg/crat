use ::libc;
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn gsl_sf_legendre_array_index(l: size_t, m: size_t) -> size_t {
    return (l.wrapping_mul(l.wrapping_add(1 as libc::c_int as libc::c_ulong))
        >> 1 as libc::c_int)
        .wrapping_add(m);
}
