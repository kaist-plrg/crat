use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn gsl_sf_lngamma(x: libc::c_double) -> libc::c_double;
    fn gsl_ran_gamma(
        r: *const gsl_rng,
        a: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_double;
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
pub unsafe extern "C" fn gsl_ran_chisq(
    mut r: *const gsl_rng,
    nu: libc::c_double,
) -> libc::c_double {
    let mut chisq: libc::c_double = 2 as libc::c_int as libc::c_double
        * gsl_ran_gamma(r, nu / 2 as libc::c_int as libc::c_double, 1.0f64);
    return chisq;
}
pub unsafe extern "C" fn gsl_ran_chisq_pdf(
    x: libc::c_double,
    nu: libc::c_double,
) -> libc::c_double {
    if x < 0 as libc::c_int as libc::c_double {
        return 0 as libc::c_int as libc::c_double
    } else if nu == 2.0f64 {
        return exp(-x / 2.0f64) / 2.0f64
    } else {
        let mut p: libc::c_double = 0.;
        let mut lngamma: libc::c_double = gsl_sf_lngamma(
            nu / 2 as libc::c_int as libc::c_double,
        );
        p = exp(
            (nu / 2 as libc::c_int as libc::c_double
                - 1 as libc::c_int as libc::c_double)
                * log(x / 2 as libc::c_int as libc::c_double)
                - x / 2 as libc::c_int as libc::c_double - lngamma,
        ) / 2 as libc::c_int as libc::c_double;
        return p;
    };
}
