use ::libc;
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn gsl_spmatrix_cumsum(n: size_t, mut c: *mut libc::c_int) {
    let mut sum: libc::c_int = 0 as libc::c_int;
    let mut k: size_t = 0;
    k = 0 as libc::c_int as size_t;
    while k < n {
        let mut ck: libc::c_int = *c.offset(k as isize);
        *c.offset(k as isize) = sum;
        sum += ck;
        k = k.wrapping_add(1);
        k;
    }
    *c.offset(n as isize) = sum;
}
