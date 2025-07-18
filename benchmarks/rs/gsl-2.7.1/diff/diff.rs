use ::libc;
extern "C" {
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_function_struct {
    pub function: Option::<
        unsafe extern "C" fn(libc::c_double, *mut libc::c_void) -> libc::c_double,
    >,
    pub params: *mut libc::c_void,
}
pub type gsl_function = gsl_function_struct;
pub type C2RustUnnamed = libc::c_int;
pub const GSL_EOF: C2RustUnnamed = 32;
pub const GSL_ETOLG: C2RustUnnamed = 31;
pub const GSL_ETOLX: C2RustUnnamed = 30;
pub const GSL_ETOLF: C2RustUnnamed = 29;
pub const GSL_ENOPROGJ: C2RustUnnamed = 28;
pub const GSL_ENOPROG: C2RustUnnamed = 27;
pub const GSL_ETABLE: C2RustUnnamed = 26;
pub const GSL_ECACHE: C2RustUnnamed = 25;
pub const GSL_EUNIMPL: C2RustUnnamed = 24;
pub const GSL_EUNSUP: C2RustUnnamed = 23;
pub const GSL_EDIVERGE: C2RustUnnamed = 22;
pub const GSL_ESING: C2RustUnnamed = 21;
pub const GSL_ENOTSQR: C2RustUnnamed = 20;
pub const GSL_EBADLEN: C2RustUnnamed = 19;
pub const GSL_EROUND: C2RustUnnamed = 18;
pub const GSL_ELOSS: C2RustUnnamed = 17;
pub const GSL_EOVRFLW: C2RustUnnamed = 16;
pub const GSL_EUNDRFLW: C2RustUnnamed = 15;
pub const GSL_ETOL: C2RustUnnamed = 14;
pub const GSL_EBADTOL: C2RustUnnamed = 13;
pub const GSL_EZERODIV: C2RustUnnamed = 12;
pub const GSL_EMAXITER: C2RustUnnamed = 11;
pub const GSL_ERUNAWAY: C2RustUnnamed = 10;
pub const GSL_EBADFUNC: C2RustUnnamed = 9;
pub const GSL_ENOMEM: C2RustUnnamed = 8;
pub const GSL_ESANITY: C2RustUnnamed = 7;
pub const GSL_EFACTOR: C2RustUnnamed = 6;
pub const GSL_EFAILED: C2RustUnnamed = 5;
pub const GSL_EINVAL: C2RustUnnamed = 4;
pub const GSL_EFAULT: C2RustUnnamed = 3;
pub const GSL_ERANGE: C2RustUnnamed = 2;
pub const GSL_EDOM: C2RustUnnamed = 1;
pub const GSL_CONTINUE: C2RustUnnamed = -2;
pub const GSL_FAILURE: C2RustUnnamed = -1;
pub const GSL_SUCCESS: C2RustUnnamed = 0;
pub unsafe extern "C" fn gsl_diff_backward(
    mut f: *const gsl_function,
    mut x: libc::c_double,
    mut result: *mut libc::c_double,
    mut abserr: *mut libc::c_double,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut h: libc::c_double = 1.4901161193847656e-08f64;
    let mut a: [libc::c_double; 3] = [0.; 3];
    let mut d: [libc::c_double; 3] = [0.; 3];
    let mut a2: libc::c_double = 0.;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        a[i as usize] = x + (i as libc::c_double - 2.0f64) * h;
        d[i
            as usize] = (Some(((*f).function).unwrap()))
            .unwrap()(a[i as usize], (*f).params);
        i += 1;
        i;
    }
    k = 1 as libc::c_int;
    while k < 4 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int - k {
            d[i
                as usize] = (d[(i + 1 as libc::c_int) as usize] - d[i as usize])
                / (a[(i + k) as usize] - a[i as usize]);
            i += 1;
            i;
        }
        k += 1;
        k;
    }
    a2 = fabs(
        d[0 as libc::c_int as usize] + d[1 as libc::c_int as usize]
            + d[2 as libc::c_int as usize],
    );
    if a2 < 100.0f64 * 1.4901161193847656e-08f64 {
        a2 = 100.0f64 * 1.4901161193847656e-08f64;
    }
    h = sqrt(1.4901161193847656e-08f64 / (2.0f64 * a2));
    if h > 100.0f64 * 1.4901161193847656e-08f64 {
        h = 100.0f64 * 1.4901161193847656e-08f64;
    }
    *result = ((Some(((*f).function).unwrap())).unwrap()(x, (*f).params)
        - (Some(((*f).function).unwrap())).unwrap()(x - h, (*f).params)) / h;
    *abserr = fabs(10.0f64 * a2 * h);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_diff_forward(
    mut f: *const gsl_function,
    mut x: libc::c_double,
    mut result: *mut libc::c_double,
    mut abserr: *mut libc::c_double,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut h: libc::c_double = 1.4901161193847656e-08f64;
    let mut a: [libc::c_double; 3] = [0.; 3];
    let mut d: [libc::c_double; 3] = [0.; 3];
    let mut a2: libc::c_double = 0.;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        a[i as usize] = x + i as libc::c_double * h;
        d[i
            as usize] = (Some(((*f).function).unwrap()))
            .unwrap()(a[i as usize], (*f).params);
        i += 1;
        i;
    }
    k = 1 as libc::c_int;
    while k < 4 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int - k {
            d[i
                as usize] = (d[(i + 1 as libc::c_int) as usize] - d[i as usize])
                / (a[(i + k) as usize] - a[i as usize]);
            i += 1;
            i;
        }
        k += 1;
        k;
    }
    a2 = fabs(
        d[0 as libc::c_int as usize] + d[1 as libc::c_int as usize]
            + d[2 as libc::c_int as usize],
    );
    if a2 < 100.0f64 * 1.4901161193847656e-08f64 {
        a2 = 100.0f64 * 1.4901161193847656e-08f64;
    }
    h = sqrt(1.4901161193847656e-08f64 / (2.0f64 * a2));
    if h > 100.0f64 * 1.4901161193847656e-08f64 {
        h = 100.0f64 * 1.4901161193847656e-08f64;
    }
    *result = ((Some(((*f).function).unwrap())).unwrap()(x + h, (*f).params)
        - (Some(((*f).function).unwrap())).unwrap()(x, (*f).params)) / h;
    *abserr = fabs(10.0f64 * a2 * h);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_diff_central(
    mut f: *const gsl_function,
    mut x: libc::c_double,
    mut result: *mut libc::c_double,
    mut abserr: *mut libc::c_double,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut h: libc::c_double = 1.4901161193847656e-08f64;
    let mut a: [libc::c_double; 4] = [0.; 4];
    let mut d: [libc::c_double; 4] = [0.; 4];
    let mut a3: libc::c_double = 0.;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        a[i as usize] = x + (i as libc::c_double - 2.0f64) * h;
        d[i
            as usize] = (Some(((*f).function).unwrap()))
            .unwrap()(a[i as usize], (*f).params);
        i += 1;
        i;
    }
    k = 1 as libc::c_int;
    while k < 5 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int - k {
            d[i
                as usize] = (d[(i + 1 as libc::c_int) as usize] - d[i as usize])
                / (a[(i + k) as usize] - a[i as usize]);
            i += 1;
            i;
        }
        k += 1;
        k;
    }
    a3 = fabs(
        d[0 as libc::c_int as usize] + d[1 as libc::c_int as usize]
            + d[2 as libc::c_int as usize] + d[3 as libc::c_int as usize],
    );
    if a3 < 100.0f64 * 1.4901161193847656e-08f64 {
        a3 = 100.0f64 * 1.4901161193847656e-08f64;
    }
    h = pow(1.4901161193847656e-08f64 / (2.0f64 * a3), 1.0f64 / 3.0f64);
    if h > 100.0f64 * 1.4901161193847656e-08f64 {
        h = 100.0f64 * 1.4901161193847656e-08f64;
    }
    *result = ((Some(((*f).function).unwrap())).unwrap()(x + h, (*f).params)
        - (Some(((*f).function).unwrap())).unwrap()(x - h, (*f).params)) / (2.0f64 * h);
    *abserr = fabs(100.0f64 * a3 * h * h);
    return GSL_SUCCESS as libc::c_int;
}
