use ::libc;
extern "C" {
    fn gsl_ran_negative_binomial_pdf(
        k: libc::c_uint,
        p: libc::c_double,
        n: libc::c_double,
    ) -> libc::c_double;
    fn gsl_ran_negative_binomial(
        r: *const gsl_rng,
        p: libc::c_double,
        n: libc::c_double,
    ) -> libc::c_uint;
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
pub unsafe extern "C" fn gsl_ran_pascal(
    mut r: *const gsl_rng,
    mut p: libc::c_double,
    mut n: libc::c_uint,
) -> libc::c_uint {
    let mut k: libc::c_uint = gsl_ran_negative_binomial(r, p, n as libc::c_double);
    return k;
}
pub unsafe extern "C" fn gsl_ran_pascal_pdf(
    k: libc::c_uint,
    p: libc::c_double,
    mut n: libc::c_uint,
) -> libc::c_double {
    let mut P: libc::c_double = gsl_ran_negative_binomial_pdf(k, p, n as libc::c_double);
    return P;
}
