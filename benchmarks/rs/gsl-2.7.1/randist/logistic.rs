use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
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
pub unsafe extern "C" fn gsl_ran_logistic(
    mut r: *const gsl_rng,
    a: libc::c_double,
) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    let mut z: libc::c_double = 0.;
    loop {
        x = gsl_rng_uniform_pos(r);
        if !(x == 1 as libc::c_int as libc::c_double) {
            break;
        }
    }
    z = log(x / (1 as libc::c_int as libc::c_double - x));
    return a * z;
}
pub unsafe extern "C" fn gsl_ran_logistic_pdf(
    x: libc::c_double,
    a: libc::c_double,
) -> libc::c_double {
    let mut u: libc::c_double = exp(-fabs(x) / a);
    let mut p: libc::c_double = u
        / (fabs(a) * (1 as libc::c_int as libc::c_double + u)
            * (1 as libc::c_int as libc::c_double + u));
    return p;
}
