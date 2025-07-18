use ::libc;
extern "C" {
    fn tan(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn gsl_sf_lngamma(x: libc::c_double) -> libc::c_double;
    fn gsl_ran_gaussian_ziggurat(
        r: *const gsl_rng,
        sigma: libc::c_double,
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
#[inline]
unsafe extern "C" fn gsl_rng_uniform(mut r: *const gsl_rng) -> libc::c_double {
    return ((*(*r).type_0).get_double).unwrap()((*r).state);
}
pub unsafe extern "C" fn gsl_ran_gamma_knuth(
    mut r: *const gsl_rng,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    let mut na: libc::c_uint = floor(a) as libc::c_uint;
    if a
        >= (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint) as libc::c_double
    {
        return b * (gamma_large(r, floor(a)) + gamma_frac(r, a - floor(a)))
    } else if a == na as libc::c_double {
        return b * gsl_ran_gamma_int(r, na)
    } else if na == 0 as libc::c_int as libc::c_uint {
        return b * gamma_frac(r, a)
    } else {
        return b * (gsl_ran_gamma_int(r, na) + gamma_frac(r, a - na as libc::c_double))
    };
}
pub unsafe extern "C" fn gsl_ran_gamma_int(
    mut r: *const gsl_rng,
    a: libc::c_uint,
) -> libc::c_double {
    if a < 12 as libc::c_int as libc::c_uint {
        let mut i: libc::c_uint = 0;
        let mut prod: libc::c_double = 1 as libc::c_int as libc::c_double;
        i = 0 as libc::c_int as libc::c_uint;
        while i < a {
            prod *= gsl_rng_uniform_pos(r);
            i = i.wrapping_add(1);
            i;
        }
        return -log(prod);
    } else {
        return gamma_large(r, a as libc::c_double)
    };
}
unsafe extern "C" fn gamma_large(
    mut r: *const gsl_rng,
    a: libc::c_double,
) -> libc::c_double {
    let mut sqa: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut v: libc::c_double = 0.;
    sqa = sqrt(
        2 as libc::c_int as libc::c_double * a - 1 as libc::c_int as libc::c_double,
    );
    loop {
        loop {
            y = tan(3.14159265358979323846f64 * gsl_rng_uniform(r));
            x = sqa * y + a - 1 as libc::c_int as libc::c_double;
            if !(x <= 0 as libc::c_int as libc::c_double) {
                break;
            }
        }
        v = gsl_rng_uniform(r);
        if !(v
            > (1 as libc::c_int as libc::c_double + y * y)
                * exp(
                    (a - 1 as libc::c_int as libc::c_double)
                        * log(x / (a - 1 as libc::c_int as libc::c_double)) - sqa * y,
                ))
        {
            break;
        }
    }
    return x;
}
unsafe extern "C" fn gamma_frac(
    mut r: *const gsl_rng,
    a: libc::c_double,
) -> libc::c_double {
    let mut p: libc::c_double = 0.;
    let mut q: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    let mut u: libc::c_double = 0.;
    let mut v: libc::c_double = 0.;
    if a == 0 as libc::c_int as libc::c_double {
        return 0 as libc::c_int as libc::c_double;
    }
    p = 2.7182818284590452354f64 / (a + 2.7182818284590452354f64);
    loop {
        u = gsl_rng_uniform(r);
        v = gsl_rng_uniform_pos(r);
        if u < p {
            x = exp(1 as libc::c_int as libc::c_double / a * log(v));
            q = exp(-x);
        } else {
            x = 1 as libc::c_int as libc::c_double - log(v);
            q = exp((a - 1 as libc::c_int as libc::c_double) * log(x));
        }
        if !(gsl_rng_uniform(r) >= q) {
            break;
        }
    }
    return x;
}
pub unsafe extern "C" fn gsl_ran_gamma_pdf(
    x: libc::c_double,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    if x < 0 as libc::c_int as libc::c_double {
        return 0 as libc::c_int as libc::c_double
    } else if x == 0 as libc::c_int as libc::c_double {
        if a == 1 as libc::c_int as libc::c_double {
            return 1 as libc::c_int as libc::c_double / b
        } else {
            return 0 as libc::c_int as libc::c_double
        }
    } else if a == 1 as libc::c_int as libc::c_double {
        return exp(-x / b) / b
    } else {
        let mut p: libc::c_double = 0.;
        let mut lngamma: libc::c_double = gsl_sf_lngamma(a);
        p = exp((a - 1 as libc::c_int as libc::c_double) * log(x / b) - x / b - lngamma)
            / b;
        return p;
    };
}
pub unsafe extern "C" fn gsl_ran_gamma_mt(
    mut r: *const gsl_rng,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    return gsl_ran_gamma(r, a, b);
}
pub unsafe extern "C" fn gsl_ran_gamma(
    mut r: *const gsl_rng,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    if a < 1 as libc::c_int as libc::c_double {
        let mut u: libc::c_double = gsl_rng_uniform_pos(r);
        return gsl_ran_gamma(r, 1.0f64 + a, b) * pow(u, 1.0f64 / a);
    }
    let mut x: libc::c_double = 0.;
    let mut v: libc::c_double = 0.;
    let mut u_0: libc::c_double = 0.;
    let mut d: libc::c_double = a - 1.0f64 / 3.0f64;
    let mut c: libc::c_double = 1.0f64 / 3.0f64 / sqrt(d);
    loop {
        loop {
            x = gsl_ran_gaussian_ziggurat(r, 1.0f64);
            v = 1.0f64 + c * x;
            if !(v <= 0 as libc::c_int as libc::c_double) {
                break;
            }
        }
        v = v * v * v;
        u_0 = gsl_rng_uniform_pos(r);
        if u_0 < 1 as libc::c_int as libc::c_double - 0.0331f64 * x * x * x * x {
            break;
        }
        if log(u_0)
            < 0.5f64 * x * x + d * (1 as libc::c_int as libc::c_double - v + log(v))
        {
            break;
        }
    }
    return b * d * v;
}
