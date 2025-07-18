use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
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
pub unsafe extern "C" fn gsl_ran_gaussian(
    mut r: *const gsl_rng,
    sigma: libc::c_double,
) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut r2: libc::c_double = 0.;
    loop {
        x = -(1 as libc::c_int) as libc::c_double
            + 2 as libc::c_int as libc::c_double * gsl_rng_uniform_pos(r);
        y = -(1 as libc::c_int) as libc::c_double
            + 2 as libc::c_int as libc::c_double * gsl_rng_uniform_pos(r);
        r2 = x * x + y * y;
        if !(r2 > 1.0f64 || r2 == 0 as libc::c_int as libc::c_double) {
            break;
        }
    }
    return sigma * y * sqrt(-2.0f64 * log(r2) / r2);
}
pub unsafe extern "C" fn gsl_ran_gaussian_ratio_method(
    mut r: *const gsl_rng,
    sigma: libc::c_double,
) -> libc::c_double {
    let mut u: libc::c_double = 0.;
    let mut v: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut Q: libc::c_double = 0.;
    let s: libc::c_double = 0.449871f64;
    let t: libc::c_double = -0.386595f64;
    let a: libc::c_double = 0.19600f64;
    let b: libc::c_double = 0.25472f64;
    let r1: libc::c_double = 0.27597f64;
    let r2: libc::c_double = 0.27846f64;
    loop {
        u = 1 as libc::c_int as libc::c_double - gsl_rng_uniform(r);
        v = gsl_rng_uniform(r) - 0.5f64;
        v *= 1.7156f64;
        x = u - s;
        y = fabs(v) - t;
        Q = x * x + y * (a * y - b * x);
        if !(Q >= r1
            && (Q > r2
                || v * v > -(4 as libc::c_int) as libc::c_double * u * u * log(u)))
        {
            break;
        }
    }
    return sigma * (v / u);
}
pub unsafe extern "C" fn gsl_ran_gaussian_pdf(
    x: libc::c_double,
    sigma: libc::c_double,
) -> libc::c_double {
    let mut u: libc::c_double = x / fabs(sigma);
    let mut p: libc::c_double = 1 as libc::c_int as libc::c_double
        / (sqrt(2 as libc::c_int as libc::c_double * 3.14159265358979323846f64)
            * fabs(sigma)) * exp(-u * u / 2 as libc::c_int as libc::c_double);
    return p;
}
pub unsafe extern "C" fn gsl_ran_ugaussian(mut r: *const gsl_rng) -> libc::c_double {
    return gsl_ran_gaussian(r, 1.0f64);
}
pub unsafe extern "C" fn gsl_ran_ugaussian_ratio_method(
    mut r: *const gsl_rng,
) -> libc::c_double {
    return gsl_ran_gaussian_ratio_method(r, 1.0f64);
}
pub unsafe extern "C" fn gsl_ran_ugaussian_pdf(x: libc::c_double) -> libc::c_double {
    return gsl_ran_gaussian_pdf(x, 1.0f64);
}
