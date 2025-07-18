use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
}
pub unsafe extern "C" fn cblas_srotg(
    mut a: *mut libc::c_float,
    mut b: *mut libc::c_float,
    mut c: *mut libc::c_float,
    mut s: *mut libc::c_float,
) {
    let roe: libc::c_float = if fabs(*a as libc::c_double) > fabs(*b as libc::c_double) {
        *a
    } else {
        *b
    };
    let scale: libc::c_float = (fabs(*a as libc::c_double) + fabs(*b as libc::c_double))
        as libc::c_float;
    let mut r: libc::c_float = 0.;
    let mut z: libc::c_float = 0.;
    if scale as libc::c_double != 0.0f64 {
        let aos: libc::c_float = *a / scale;
        let bos: libc::c_float = *b / scale;
        r = (scale as libc::c_double * sqrt((aos * aos + bos * bos) as libc::c_double))
            as libc::c_float;
        r = (if roe as libc::c_double >= 0.0f64 {
            1 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_float * r;
        *c = *a / r;
        *s = *b / r;
        z = 1.0f64 as libc::c_float;
        if fabs(*a as libc::c_double) > fabs(*b as libc::c_double) {
            z = *s;
        }
        if fabs(*b as libc::c_double) >= fabs(*a as libc::c_double)
            && *c as libc::c_double != 0.0f64
        {
            z = (1.0f64 / *c as libc::c_double) as libc::c_float;
        }
    } else {
        *c = 1.0f64 as libc::c_float;
        *s = 0.0f64 as libc::c_float;
        r = 0.0f64 as libc::c_float;
        z = 0.0f64 as libc::c_float;
    }
    *a = r;
    *b = z;
}
