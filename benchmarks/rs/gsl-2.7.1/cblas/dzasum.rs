use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
}
pub unsafe extern "C" fn cblas_dzasum(
    N: libc::c_int,
    mut X: *const libc::c_void,
    incX: libc::c_int,
) -> libc::c_double {
    let mut r: libc::c_double = 0.0f64;
    let mut i: libc::c_int = 0;
    let mut ix: libc::c_int = 0 as libc::c_int;
    if incX <= 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_double;
    }
    i = 0 as libc::c_int;
    while i < N {
        r
            += fabs(
                *(X as *const libc::c_double).offset((2 as libc::c_int * ix) as isize),
            )
                + fabs(
                    *(X as *const libc::c_double)
                        .offset((2 as libc::c_int * ix + 1 as libc::c_int) as isize),
                );
        ix += incX;
        i += 1;
        i;
    }
    return r;
}
