use ::libc;
extern "C" {
    fn tan(_: libc::c_double) -> libc::c_double;
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
pub unsafe extern "C" fn gsl_ran_cauchy(
    mut r: *const gsl_rng,
    a: libc::c_double,
) -> libc::c_double {
    let mut u: libc::c_double = 0.;
    loop {
        u = gsl_rng_uniform(r);
        if !(u == 0.5f64) {
            break;
        }
    }
    return a * tan(3.14159265358979323846f64 * u);
}
pub unsafe extern "C" fn gsl_ran_cauchy_pdf(
    x: libc::c_double,
    a: libc::c_double,
) -> libc::c_double {
    let mut u: libc::c_double = x / a;
    let mut p: libc::c_double = 1 as libc::c_int as libc::c_double
        / (3.14159265358979323846f64 * a) / (1 as libc::c_int as libc::c_double + u * u);
    return p;
}
