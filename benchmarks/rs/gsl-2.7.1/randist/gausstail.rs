use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn gsl_ran_gaussian(r: *const gsl_rng, sigma: libc::c_double) -> libc::c_double;
    fn gsl_sf_erfc(x: libc::c_double) -> libc::c_double;
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
unsafe extern "C" fn gsl_rng_uniform(mut r: *const gsl_rng) -> libc::c_double {
    return ((*(*r).type_0).get_double).unwrap()((*r).state);
}
pub unsafe extern "C" fn gsl_ran_gaussian_tail(
    mut r: *const gsl_rng,
    a: libc::c_double,
    sigma: libc::c_double,
) -> libc::c_double {
    let mut s: libc::c_double = a / sigma;
    if s < 1 as libc::c_int as libc::c_double {
        let mut x: libc::c_double = 0.;
        loop {
            x = gsl_ran_gaussian(r, 1.0f64);
            if !(x < s) {
                break;
            }
        }
        return x * sigma;
    } else {
        let mut u: libc::c_double = 0.;
        let mut v: libc::c_double = 0.;
        let mut x_0: libc::c_double = 0.;
        loop {
            u = gsl_rng_uniform(r);
            loop {
                v = gsl_rng_uniform(r);
                if !(v == 0.0f64) {
                    break;
                }
            }
            x_0 = sqrt(s * s - 2 as libc::c_int as libc::c_double * log(v));
            if !(x_0 * u > s) {
                break;
            }
        }
        return x_0 * sigma;
    };
}
pub unsafe extern "C" fn gsl_ran_gaussian_tail_pdf(
    x: libc::c_double,
    a: libc::c_double,
    sigma: libc::c_double,
) -> libc::c_double {
    if x < a {
        return 0 as libc::c_int as libc::c_double
    } else {
        let mut N: libc::c_double = 0.;
        let mut p: libc::c_double = 0.;
        let mut u: libc::c_double = x / sigma;
        let mut f: libc::c_double = gsl_sf_erfc(a / (sqrt(2.0f64) * sigma));
        N = 0.5f64 * f;
        p = 1 as libc::c_int as libc::c_double
            / (N * sqrt(2 as libc::c_int as libc::c_double * 3.14159265358979323846f64)
                * sigma) * exp(-u * u / 2 as libc::c_int as libc::c_double);
        return p;
    };
}
pub unsafe extern "C" fn gsl_ran_ugaussian_tail(
    mut r: *const gsl_rng,
    a: libc::c_double,
) -> libc::c_double {
    return gsl_ran_gaussian_tail(r, a, 1.0f64);
}
pub unsafe extern "C" fn gsl_ran_ugaussian_tail_pdf(
    x: libc::c_double,
    a: libc::c_double,
) -> libc::c_double {
    return gsl_ran_gaussian_tail_pdf(x, a, 1.0f64);
}
