use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
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
pub unsafe extern "C" fn gsl_ran_logarithmic(
    mut r: *const gsl_rng,
    p: libc::c_double,
) -> libc::c_uint {
    let mut c: libc::c_double = log(1 as libc::c_int as libc::c_double - p);
    let mut v: libc::c_double = gsl_rng_uniform_pos(r);
    if v >= p {
        return 1 as libc::c_int as libc::c_uint
    } else {
        let mut u: libc::c_double = gsl_rng_uniform_pos(r);
        let mut q: libc::c_double = 1 as libc::c_int as libc::c_double - exp(c * u);
        if v <= q * q {
            let mut x: libc::c_double = 1 as libc::c_int as libc::c_double
                + log(v) / log(q);
            return x as libc::c_uint;
        } else if v <= q {
            return 2 as libc::c_int as libc::c_uint
        } else {
            return 1 as libc::c_int as libc::c_uint
        }
    };
}
pub unsafe extern "C" fn gsl_ran_logarithmic_pdf(
    k: libc::c_uint,
    p: libc::c_double,
) -> libc::c_double {
    if k == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as libc::c_double
    } else {
        let mut P: libc::c_double = pow(p, k as libc::c_double) / k as libc::c_double
            / log(
                1 as libc::c_int as libc::c_double
                    / (1 as libc::c_int as libc::c_double - p),
            );
        return P;
    };
}
