use ::libc;
pub unsafe extern "C" fn cblas_sscal(
    N: libc::c_int,
    alpha: libc::c_float,
    mut X: *mut libc::c_float,
    incX: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut ix: libc::c_int = 0 as libc::c_int;
    if incX <= 0 as libc::c_int {
        return;
    }
    i = 0 as libc::c_int;
    while i < N {
        *X.offset(ix as isize) *= alpha;
        ix += incX;
        i += 1;
        i;
    }
}
