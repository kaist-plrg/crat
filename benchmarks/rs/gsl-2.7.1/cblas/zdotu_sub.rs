use ::libc;
pub unsafe extern "C" fn cblas_zdotu_sub(
    N: libc::c_int,
    mut X: *const libc::c_void,
    incX: libc::c_int,
    mut Y: *const libc::c_void,
    incY: libc::c_int,
    mut result: *mut libc::c_void,
) {
    let mut r_real: libc::c_double = 0.0f64;
    let mut r_imag: libc::c_double = 0.0f64;
    let mut i: libc::c_int = 0;
    let mut ix: libc::c_int = if incX > 0 as libc::c_int {
        0 as libc::c_int
    } else {
        (N - 1 as libc::c_int) * -incX
    };
    let mut iy: libc::c_int = if incY > 0 as libc::c_int {
        0 as libc::c_int
    } else {
        (N - 1 as libc::c_int) * -incY
    };
    i = 0 as libc::c_int;
    while i < N {
        let x_real: libc::c_double = *(X as *const libc::c_double)
            .offset((2 as libc::c_int * ix) as isize);
        let x_imag: libc::c_double = *(X as *const libc::c_double)
            .offset((2 as libc::c_int * ix + 1 as libc::c_int) as isize);
        let y_real: libc::c_double = *(Y as *const libc::c_double)
            .offset((2 as libc::c_int * iy) as isize);
        let y_imag: libc::c_double = *(Y as *const libc::c_double)
            .offset((2 as libc::c_int * iy + 1 as libc::c_int) as isize);
        r_real += x_real * y_real - 1.0f64 * x_imag * y_imag;
        r_imag += x_real * y_imag + 1.0f64 * x_imag * y_real;
        ix += incX;
        iy += incY;
        i += 1;
        i;
    }
    *(result as *mut libc::c_double).offset(0 as libc::c_int as isize) = r_real;
    *(result as *mut libc::c_double).offset(1 as libc::c_int as isize) = r_imag;
}
