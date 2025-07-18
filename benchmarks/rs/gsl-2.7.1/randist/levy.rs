use ::libc;
extern "C" {
    fn atan(_: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn tan(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn gsl_ran_exponential(r: *const gsl_rng, mu: libc::c_double) -> libc::c_double;
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
pub unsafe extern "C" fn gsl_ran_levy(
    mut r: *const gsl_rng,
    c: libc::c_double,
    alpha: libc::c_double,
) -> libc::c_double {
    let mut u: libc::c_double = 0.;
    let mut v: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut s: libc::c_double = 0.;
    u = 3.14159265358979323846f64 * (gsl_rng_uniform_pos(r) - 0.5f64);
    if alpha == 1 as libc::c_int as libc::c_double {
        t = tan(u);
        return c * t;
    }
    loop {
        v = gsl_ran_exponential(r, 1.0f64);
        if !(v == 0 as libc::c_int as libc::c_double) {
            break;
        }
    }
    if alpha == 2 as libc::c_int as libc::c_double {
        t = 2 as libc::c_int as libc::c_double * sin(u) * sqrt(v);
        return c * t;
    }
    t = sin(alpha * u) / pow(cos(u), 1 as libc::c_int as libc::c_double / alpha);
    s = pow(
        cos((1 as libc::c_int as libc::c_double - alpha) * u) / v,
        (1 as libc::c_int as libc::c_double - alpha) / alpha,
    );
    return c * t * s;
}
pub unsafe extern "C" fn gsl_ran_levy_skew(
    mut r: *const gsl_rng,
    c: libc::c_double,
    alpha: libc::c_double,
    beta: libc::c_double,
) -> libc::c_double {
    let mut V: libc::c_double = 0.;
    let mut W: libc::c_double = 0.;
    let mut X: libc::c_double = 0.;
    if beta == 0 as libc::c_int as libc::c_double {
        return gsl_ran_levy(r, c, alpha);
    }
    V = 3.14159265358979323846f64 * (gsl_rng_uniform_pos(r) - 0.5f64);
    loop {
        W = gsl_ran_exponential(r, 1.0f64);
        if !(W == 0 as libc::c_int as libc::c_double) {
            break;
        }
    }
    if alpha == 1 as libc::c_int as libc::c_double {
        X = ((1.57079632679489661923f64 + beta * V) * tan(V)
            - beta
                * log(
                    1.57079632679489661923f64 * W * cos(V)
                        / (1.57079632679489661923f64 + beta * V),
                )) / 1.57079632679489661923f64;
        return c * (X + beta * log(c) / 1.57079632679489661923f64);
    } else {
        let mut t: libc::c_double = beta * tan(1.57079632679489661923f64 * alpha);
        let mut B: libc::c_double = atan(t) / alpha;
        let mut S: libc::c_double = pow(
            1 as libc::c_int as libc::c_double + t * t,
            1 as libc::c_int as libc::c_double
                / (2 as libc::c_int as libc::c_double * alpha),
        );
        X = S * sin(alpha * (V + B))
            / pow(cos(V), 1 as libc::c_int as libc::c_double / alpha)
            * pow(
                cos(V - alpha * (V + B)) / W,
                (1 as libc::c_int as libc::c_double - alpha) / alpha,
            );
        return c * X;
    };
}
