use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn cblas_idamax(
    N: libc::c_int,
    mut X: *const libc::c_double,
    incX: libc::c_int,
) -> size_t {
    let mut max: libc::c_double = 0.0f64;
    let mut ix: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut result: size_t = 0 as libc::c_int as size_t;
    if incX <= 0 as libc::c_int {
        return 0 as libc::c_int as size_t;
    }
    i = 0 as libc::c_int;
    while i < N {
        if fabs(*X.offset(ix as isize)) > max {
            max = fabs(*X.offset(ix as isize));
            result = i as size_t;
        }
        ix += incX;
        i += 1;
        i;
    }
    return result;
}
