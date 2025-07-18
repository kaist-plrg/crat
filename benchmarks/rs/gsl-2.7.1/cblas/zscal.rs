use ::libc;
pub unsafe extern "C" fn cblas_zscal(
    N: libc::c_int,
    mut alpha: *const libc::c_void,
    mut X: *mut libc::c_void,
    incX: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut ix: libc::c_int = 0 as libc::c_int;
    let alpha_real: libc::c_double = *(alpha as *const libc::c_double)
        .offset(0 as libc::c_int as isize);
    let alpha_imag: libc::c_double = *(alpha as *const libc::c_double)
        .offset(1 as libc::c_int as isize);
    if incX <= 0 as libc::c_int {
        return;
    }
    i = 0 as libc::c_int;
    while i < N {
        let x_real: libc::c_double = *(X as *mut libc::c_double)
            .offset((2 as libc::c_int * ix) as isize);
        let x_imag: libc::c_double = *(X as *mut libc::c_double)
            .offset((2 as libc::c_int * ix + 1 as libc::c_int) as isize);
        *(X as *mut libc::c_double)
            .offset(
                (2 as libc::c_int * ix) as isize,
            ) = x_real * alpha_real - x_imag * alpha_imag;
        *(X as *mut libc::c_double)
            .offset(
                (2 as libc::c_int * ix + 1 as libc::c_int) as isize,
            ) = x_real * alpha_imag + x_imag * alpha_real;
        ix += incX;
        i += 1;
        i;
    }
}
