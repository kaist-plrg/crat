use ::libc;
pub unsafe extern "C" fn cblas_zdscal(
    N: libc::c_int,
    alpha: libc::c_double,
    mut X: *mut libc::c_void,
    incX: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut ix: libc::c_int = 0 as libc::c_int;
    if incX <= 0 as libc::c_int {
        return;
    }
    i = 0 as libc::c_int;
    while i < N {
        *(X as *mut libc::c_double).offset((2 as libc::c_int * ix) as isize) *= alpha;
        *(X as *mut libc::c_double)
            .offset((2 as libc::c_int * ix + 1 as libc::c_int) as isize) *= alpha;
        ix += incX;
        i += 1;
        i;
    }
}
