use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_sf_lngamma(x: libc::c_double) -> libc::c_double;
    fn gsl_ran_gaussian(r: *const gsl_rng, sigma: libc::c_double) -> libc::c_double;
    fn gsl_ran_laplace(r: *const gsl_rng, a: libc::c_double) -> libc::c_double;
    fn gsl_ran_gamma(
        r: *const gsl_rng,
        a: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_rng_type {
    pub name: *const libc::c_char,
    pub max: libc::c_ulong,
    pub min: libc::c_ulong,
    pub size: size_t,
    pub set: Option::<unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> ()>,
    pub get: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong>,
    pub get_double: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_rng {
    pub type_0: *const gsl_rng_type,
    pub state: *mut libc::c_void,
}
#[inline]
unsafe extern "C" fn gsl_rng_uniform_pos(mut r: *const gsl_rng) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    loop {
        x = ((*(*r).type_0).get_double).unwrap()((*r).state);
        if !(x == 0 as libc::c_int as libc::c_double) {
            break;
        }
    }
    return x;
}
#[inline]
unsafe extern "C" fn gsl_rng_uniform(mut r: *const gsl_rng) -> libc::c_double {
    return ((*(*r).type_0).get_double).unwrap()((*r).state);
}
pub unsafe extern "C" fn gsl_ran_exppow(
    mut r: *const gsl_rng,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    if b < 1 as libc::c_int as libc::c_double || b > 4 as libc::c_int as libc::c_double {
        let mut u: libc::c_double = gsl_rng_uniform(r);
        let mut v: libc::c_double = gsl_ran_gamma(
            r,
            1 as libc::c_int as libc::c_double / b,
            1.0f64,
        );
        let mut z: libc::c_double = a * pow(v, 1 as libc::c_int as libc::c_double / b);
        if u > 0.5f64 { return z } else { return -z }
    } else if b == 1 as libc::c_int as libc::c_double {
        return gsl_ran_laplace(r, a)
    } else if b < 2 as libc::c_int as libc::c_double {
        let mut x: libc::c_double = 0.;
        let mut h: libc::c_double = 0.;
        let mut u_0: libc::c_double = 0.;
        let mut B: libc::c_double = pow(
            1 as libc::c_int as libc::c_double / b,
            1 as libc::c_int as libc::c_double / b,
        );
        loop {
            x = gsl_ran_laplace(r, B);
            u_0 = gsl_rng_uniform_pos(r);
            h = -pow(fabs(x), b) + fabs(x) / B - 1 as libc::c_int as libc::c_double
                + 1 as libc::c_int as libc::c_double / b;
            if !(log(u_0) > h) {
                break;
            }
        }
        return a * x;
    } else if b == 2 as libc::c_int as libc::c_double {
        return gsl_ran_gaussian(r, a / sqrt(2.0f64))
    } else {
        let mut x_0: libc::c_double = 0.;
        let mut h_0: libc::c_double = 0.;
        let mut u_1: libc::c_double = 0.;
        let mut B_0: libc::c_double = pow(
            1 as libc::c_int as libc::c_double / b,
            1 as libc::c_int as libc::c_double / b,
        );
        loop {
            x_0 = gsl_ran_gaussian(r, B_0);
            u_1 = gsl_rng_uniform_pos(r);
            h_0 = -pow(fabs(x_0), b)
                + x_0 * x_0 / (2 as libc::c_int as libc::c_double * B_0 * B_0)
                + 1 as libc::c_int as libc::c_double / b - 0.5f64;
            if !(log(u_1) > h_0) {
                break;
            }
        }
        return a * x_0;
    };
}
pub unsafe extern "C" fn gsl_ran_exppow_pdf(
    x: libc::c_double,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    let mut p: libc::c_double = 0.;
    let mut lngamma: libc::c_double = gsl_sf_lngamma(
        1 as libc::c_int as libc::c_double + 1 as libc::c_int as libc::c_double / b,
    );
    p = 1 as libc::c_int as libc::c_double / (2 as libc::c_int as libc::c_double * a)
        * exp(-pow(fabs(x / a), b) - lngamma);
    return p;
}
