use ::libc;
extern "C" {
    fn log(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
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
pub unsafe extern "C" fn gsl_ran_geometric(
    mut r: *const gsl_rng,
    p: libc::c_double,
) -> libc::c_uint {
    let mut u: libc::c_double = gsl_rng_uniform_pos(r);
    let mut k: libc::c_uint = 0;
    if p == 1 as libc::c_int as libc::c_double {
        k = 1 as libc::c_int as libc::c_uint;
    } else {
        k = (log(u) / log(1 as libc::c_int as libc::c_double - p)
            + 1 as libc::c_int as libc::c_double) as libc::c_uint;
    }
    return k;
}
pub unsafe extern "C" fn gsl_ran_geometric_pdf(
    k: libc::c_uint,
    p: libc::c_double,
) -> libc::c_double {
    if k == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as libc::c_double
    } else if k == 1 as libc::c_int as libc::c_uint {
        return p
    } else {
        let mut P: libc::c_double = p
            * pow(1 as libc::c_int as libc::c_double - p, k as libc::c_double - 1.0f64);
        return P;
    };
}
