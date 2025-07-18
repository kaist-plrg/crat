use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn gsl_sf_lnfact(n: libc::c_uint) -> libc::c_double;
    fn gsl_ran_binomial(
        r: *const gsl_rng,
        p: libc::c_double,
        n: libc::c_uint,
    ) -> libc::c_uint;
    fn gsl_ran_gamma_int(r: *const gsl_rng, a: libc::c_uint) -> libc::c_double;
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
pub unsafe extern "C" fn gsl_ran_poisson(
    mut r: *const gsl_rng,
    mut mu: libc::c_double,
) -> libc::c_uint {
    let mut emu: libc::c_double = 0.;
    let mut prod: libc::c_double = 1.0f64;
    let mut k: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while mu > 10 as libc::c_int as libc::c_double {
        let mut m: libc::c_uint = (mu * (7.0f64 / 8.0f64)) as libc::c_uint;
        let mut X: libc::c_double = gsl_ran_gamma_int(r, m);
        if X >= mu {
            return k
                .wrapping_add(
                    gsl_ran_binomial(
                        r,
                        mu / X,
                        m.wrapping_sub(1 as libc::c_int as libc::c_uint),
                    ),
                )
        } else {
            k = k.wrapping_add(m);
            mu -= X;
        }
    }
    emu = exp(-mu);
    loop {
        prod *= gsl_rng_uniform(r);
        k = k.wrapping_add(1);
        k;
        if !(prod > emu) {
            break;
        }
    }
    return k.wrapping_sub(1 as libc::c_int as libc::c_uint);
}
pub unsafe extern "C" fn gsl_ran_poisson_array(
    mut r: *const gsl_rng,
    mut n: size_t,
    mut array: *mut libc::c_uint,
    mut mu: libc::c_double,
) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *array.offset(i as isize) = gsl_ran_poisson(r, mu);
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_ran_poisson_pdf(
    k: libc::c_uint,
    mu: libc::c_double,
) -> libc::c_double {
    let mut p: libc::c_double = 0.;
    let mut lf: libc::c_double = gsl_sf_lnfact(k);
    p = exp(log(mu) * k as libc::c_double - lf - mu);
    return p;
}
