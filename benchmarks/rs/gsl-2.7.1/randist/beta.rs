use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn log1p(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
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
pub unsafe extern "C" fn gsl_ran_beta(
    mut r: *const gsl_rng,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    if a <= 1.0f64 && b <= 1.0f64 {
        let mut U: libc::c_double = 0.;
        let mut V: libc::c_double = 0.;
        let mut X: libc::c_double = 0.;
        let mut Y: libc::c_double = 0.;
        loop {
            U = gsl_rng_uniform_pos(r);
            V = gsl_rng_uniform_pos(r);
            X = pow(U, 1.0f64 / a);
            Y = pow(V, 1.0f64 / b);
            if X + Y <= 1.0f64 {
                if X + Y > 0 as libc::c_int as libc::c_double {
                    return X / (X + Y)
                } else {
                    let mut logX: libc::c_double = log(U) / a;
                    let mut logY: libc::c_double = log(V) / b;
                    let mut logM: libc::c_double = if logX > logY { logX } else { logY };
                    logX -= logM;
                    logY -= logM;
                    return exp(logX - log(exp(logX) + exp(logY)));
                }
            }
        }
    } else {
        let mut x1: libc::c_double = gsl_ran_gamma(r, a, 1.0f64);
        let mut x2: libc::c_double = gsl_ran_gamma(r, b, 1.0f64);
        return x1 / (x1 + x2);
    };
}
pub unsafe extern "C" fn gsl_ran_beta_pdf(
    x: libc::c_double,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    if x < 0 as libc::c_int as libc::c_double || x > 1 as libc::c_int as libc::c_double {
        return 0 as libc::c_int as libc::c_double
    } else {
        let mut p: libc::c_double = 0.;
        let mut gab: libc::c_double = gsl_sf_lngamma(a + b);
        let mut ga: libc::c_double = gsl_sf_lngamma(a);
        let mut gb: libc::c_double = gsl_sf_lngamma(b);
        if x == 0.0f64 || x == 1.0f64 {
            if a > 1.0f64 && b > 1.0f64 {
                p = 0.0f64;
            } else {
                p = exp(gab - ga - gb) * pow(x, a - 1 as libc::c_int as libc::c_double)
                    * pow(
                        1 as libc::c_int as libc::c_double - x,
                        b - 1 as libc::c_int as libc::c_double,
                    );
            }
        } else {
            p = exp(
                gab - ga - gb + log(x) * (a - 1 as libc::c_int as libc::c_double)
                    + log1p(-x) * (b - 1 as libc::c_int as libc::c_double),
            );
        }
        return p;
    };
}
