use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn cblas_isamax(
    N: libc::c_int,
    mut X: *const libc::c_float,
    incX: libc::c_int,
) -> size_t {
    let mut max: libc::c_float = 0.0f64 as libc::c_float;
    let mut ix: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut result: size_t = 0 as libc::c_int as size_t;
    if incX <= 0 as libc::c_int {
        return 0 as libc::c_int as size_t;
    }
    i = 0 as libc::c_int;
    while i < N {
        if fabs(*X.offset(ix as isize) as libc::c_double) > max as libc::c_double {
            max = fabs(*X.offset(ix as isize) as libc::c_double) as libc::c_float;
            result = i as size_t;
        }
        ix += incX;
        i += 1;
        i;
    }
    return result;
}
