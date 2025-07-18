use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
}
pub unsafe extern "C" fn cblas_drotg(
    mut a: *mut libc::c_double,
    mut b: *mut libc::c_double,
    mut c: *mut libc::c_double,
    mut s: *mut libc::c_double,
) {
    let roe: libc::c_double = if fabs(*a) > fabs(*b) { *a } else { *b };
    let scale: libc::c_double = fabs(*a) + fabs(*b);
    let mut r: libc::c_double = 0.;
    let mut z: libc::c_double = 0.;
    if scale != 0.0f64 {
        let aos: libc::c_double = *a / scale;
        let bos: libc::c_double = *b / scale;
        r = scale * sqrt(aos * aos + bos * bos);
        r = (if roe >= 0.0f64 { 1 as libc::c_int } else { -(1 as libc::c_int) })
            as libc::c_double * r;
        *c = *a / r;
        *s = *b / r;
        z = 1.0f64;
        if fabs(*a) > fabs(*b) {
            z = *s;
        }
        if fabs(*b) >= fabs(*a) && *c != 0.0f64 {
            z = 1.0f64 / *c;
        }
    } else {
        *c = 1.0f64;
        *s = 0.0f64;
        r = 0.0f64;
        z = 0.0f64;
    }
    *a = r;
    *b = z;
}
