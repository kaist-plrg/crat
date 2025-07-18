use ::libc;
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
pub unsafe extern "C" fn gsl_ran_bernoulli(
    mut r: *const gsl_rng,
    mut p: libc::c_double,
) -> libc::c_uint {
    let mut u: libc::c_double = gsl_rng_uniform(r);
    if u < p {
        return 1 as libc::c_int as libc::c_uint
    } else {
        return 0 as libc::c_int as libc::c_uint
    };
}
pub unsafe extern "C" fn gsl_ran_bernoulli_pdf(
    k: libc::c_uint,
    mut p: libc::c_double,
) -> libc::c_double {
    if k == 0 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int as libc::c_double - p
    } else if k == 1 as libc::c_int as libc::c_uint {
        return p
    } else {
        return 0 as libc::c_int as libc::c_double
    };
}
