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
unsafe extern "C" fn gsl_rng_uniform(mut r: *const gsl_rng) -> libc::c_double {
    return ((*(*r).type_0).get_double).unwrap()((*r).state);
}
pub unsafe extern "C" fn gsl_ran_lognormal(
    mut r: *const gsl_rng,
    zeta: libc::c_double,
    sigma: libc::c_double,
) -> libc::c_double {
    let mut u: libc::c_double = 0.;
    let mut v: libc::c_double = 0.;
    let mut r2: libc::c_double = 0.;
    let mut normal: libc::c_double = 0.;
    let mut z: libc::c_double = 0.;
    loop {
        u = -(1 as libc::c_int) as libc::c_double
            + 2 as libc::c_int as libc::c_double * gsl_rng_uniform(r);
        v = -(1 as libc::c_int) as libc::c_double
            + 2 as libc::c_int as libc::c_double * gsl_rng_uniform(r);
        r2 = u * u + v * v;
        if !(r2 > 1.0f64 || r2 == 0 as libc::c_int as libc::c_double) {
            break;
        }
    }
    normal = u * sqrt(-2.0f64 * log(r2) / r2);
    z = exp(sigma * normal + zeta);
    return z;
}
pub unsafe extern "C" fn gsl_ran_lognormal_pdf(
    x: libc::c_double,
    zeta: libc::c_double,
    sigma: libc::c_double,
) -> libc::c_double {
    if x <= 0 as libc::c_int as libc::c_double {
        return 0 as libc::c_int as libc::c_double
    } else {
        let mut u: libc::c_double = (log(x) - zeta) / sigma;
        let mut p: libc::c_double = 1 as libc::c_int as libc::c_double
            / (x * fabs(sigma)
                * sqrt(2 as libc::c_int as libc::c_double * 3.14159265358979323846f64))
            * exp(-(u * u) / 2 as libc::c_int as libc::c_double);
        return p;
    };
}
