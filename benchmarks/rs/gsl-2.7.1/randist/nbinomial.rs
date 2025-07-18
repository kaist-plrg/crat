use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn log1p(_: libc::c_double) -> libc::c_double;
    fn gsl_ran_poisson(r: *const gsl_rng, mu: libc::c_double) -> libc::c_uint;
    fn gsl_ran_gamma(
        r: *const gsl_rng,
        a: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_double;
    fn gsl_sf_lngamma(x: libc::c_double) -> libc::c_double;
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
pub unsafe extern "C" fn gsl_ran_negative_binomial(
    mut r: *const gsl_rng,
    mut p: libc::c_double,
    mut n: libc::c_double,
) -> libc::c_uint {
    let mut X: libc::c_double = gsl_ran_gamma(r, n, 1.0f64);
    let mut k: libc::c_uint = gsl_ran_poisson(
        r,
        X * (1 as libc::c_int as libc::c_double - p) / p,
    );
    return k;
}
pub unsafe extern "C" fn gsl_ran_negative_binomial_pdf(
    k: libc::c_uint,
    p: libc::c_double,
    mut n: libc::c_double,
) -> libc::c_double {
    let mut P: libc::c_double = 0.;
    let mut f: libc::c_double = gsl_sf_lngamma(k as libc::c_double + n);
    let mut a: libc::c_double = gsl_sf_lngamma(n);
    let mut b: libc::c_double = gsl_sf_lngamma(k as libc::c_double + 1.0f64);
    P = exp(f - a - b + n * log(p) + k as libc::c_double * log1p(-p));
    return P;
}
