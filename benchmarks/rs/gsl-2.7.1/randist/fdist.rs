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
pub unsafe extern "C" fn gsl_ran_fdist(
    mut r: *const gsl_rng,
    nu1: libc::c_double,
    nu2: libc::c_double,
) -> libc::c_double {
    let mut Y1: libc::c_double = gsl_ran_gamma(
        r,
        nu1 / 2 as libc::c_int as libc::c_double,
        2.0f64,
    );
    let mut Y2: libc::c_double = gsl_ran_gamma(
        r,
        nu2 / 2 as libc::c_int as libc::c_double,
        2.0f64,
    );
    let mut f: libc::c_double = Y1 * nu2 / (Y2 * nu1);
    return f;
}
pub unsafe extern "C" fn gsl_ran_fdist_pdf(
    x: libc::c_double,
    nu1: libc::c_double,
    nu2: libc::c_double,
) -> libc::c_double {
    if x < 0 as libc::c_int as libc::c_double {
        return 0 as libc::c_int as libc::c_double
    } else {
        let mut p: libc::c_double = 0.;
        let mut lglg: libc::c_double = nu1 / 2 as libc::c_int as libc::c_double
            * log(nu1) + nu2 / 2 as libc::c_int as libc::c_double * log(nu2);
        let mut lg12: libc::c_double = gsl_sf_lngamma(
            (nu1 + nu2) / 2 as libc::c_int as libc::c_double,
        );
        let mut lg1: libc::c_double = gsl_sf_lngamma(
            nu1 / 2 as libc::c_int as libc::c_double,
        );
        let mut lg2: libc::c_double = gsl_sf_lngamma(
            nu2 / 2 as libc::c_int as libc::c_double,
        );
        p = exp(
            lglg + lg12 - lg1 - lg2
                + (nu1 / 2 as libc::c_int as libc::c_double
                    - 1 as libc::c_int as libc::c_double) * log(x)
                - (nu1 + nu2) / 2 as libc::c_int as libc::c_double * log(nu2 + nu1 * x),
        );
        return p;
    };
}
