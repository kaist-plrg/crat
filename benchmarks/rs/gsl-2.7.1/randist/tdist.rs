use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn gsl_sf_lngamma(x: libc::c_double) -> libc::c_double;
    fn gsl_ran_exponential(r: *const gsl_rng, mu: libc::c_double) -> libc::c_double;
    fn gsl_ran_chisq(r: *const gsl_rng, nu: libc::c_double) -> libc::c_double;
    fn gsl_ran_ugaussian(r: *const gsl_rng) -> libc::c_double;
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
pub unsafe extern "C" fn gsl_ran_tdist(
    mut r: *const gsl_rng,
    nu: libc::c_double,
) -> libc::c_double {
    if nu <= 2 as libc::c_int as libc::c_double {
        let mut Y1: libc::c_double = gsl_ran_ugaussian(r);
        let mut Y2: libc::c_double = gsl_ran_chisq(r, nu);
        let mut t: libc::c_double = Y1 / sqrt(Y2 / nu);
        return t;
    } else {
        let mut Y1_0: libc::c_double = 0.;
        let mut Y2_0: libc::c_double = 0.;
        let mut Z: libc::c_double = 0.;
        let mut t_0: libc::c_double = 0.;
        loop {
            Y1_0 = gsl_ran_ugaussian(r);
            Y2_0 = gsl_ran_exponential(
                r,
                1 as libc::c_int as libc::c_double
                    / (nu / 2 as libc::c_int as libc::c_double
                        - 1 as libc::c_int as libc::c_double),
            );
            Z = Y1_0 * Y1_0 / (nu - 2 as libc::c_int as libc::c_double);
            if !(1 as libc::c_int as libc::c_double - Z
                < 0 as libc::c_int as libc::c_double
                || exp(-Y2_0 - Z) > 1 as libc::c_int as libc::c_double - Z)
            {
                break;
            }
        }
        t_0 = Y1_0
            / sqrt(
                (1 as libc::c_int as libc::c_double
                    - 2 as libc::c_int as libc::c_double / nu)
                    * (1 as libc::c_int as libc::c_double - Z),
            );
        return t_0;
    };
}
pub unsafe extern "C" fn gsl_ran_tdist_pdf(
    x: libc::c_double,
    nu: libc::c_double,
) -> libc::c_double {
    let mut p: libc::c_double = 0.;
    let mut lg1: libc::c_double = gsl_sf_lngamma(
        nu / 2 as libc::c_int as libc::c_double,
    );
    let mut lg2: libc::c_double = gsl_sf_lngamma(
        (nu + 1 as libc::c_int as libc::c_double) / 2 as libc::c_int as libc::c_double,
    );
    p = exp(lg2 - lg1) / sqrt(3.14159265358979323846f64 * nu)
        * pow(
            1 as libc::c_int as libc::c_double + x * x / nu,
            -(nu + 1 as libc::c_int as libc::c_double)
                / 2 as libc::c_int as libc::c_double,
        );
    return p;
}
