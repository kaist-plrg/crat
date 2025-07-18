use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
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
pub unsafe extern "C" fn gsl_ran_dirichlet(
    mut r: *const gsl_rng,
    K: size_t,
    mut alpha: *const libc::c_double,
    mut theta: *mut libc::c_double,
) {
    let mut i: size_t = 0;
    let mut norm: libc::c_double = 0.0f64;
    i = 0 as libc::c_int as size_t;
    while i < K {
        *theta.offset(i as isize) = gsl_ran_gamma(r, *alpha.offset(i as isize), 1.0f64);
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < K {
        norm += *theta.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    if norm < 1.4916681462400413e-154f64 {
        ran_dirichlet_small(r, K, alpha, theta);
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < K {
        *theta.offset(i as isize) /= norm;
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn ran_dirichlet_small(
    mut r: *const gsl_rng,
    K: size_t,
    mut alpha: *const libc::c_double,
    mut theta: *mut libc::c_double,
) {
    let mut i: size_t = 0;
    let mut norm: libc::c_double = 0.0f64;
    let mut umax: libc::c_double = 0 as libc::c_int as libc::c_double;
    i = 0 as libc::c_int as size_t;
    while i < K {
        let mut u: libc::c_double = log(gsl_rng_uniform_pos(r))
            / *alpha.offset(i as isize);
        *theta.offset(i as isize) = u;
        if u > umax || i == 0 as libc::c_int as libc::c_ulong {
            umax = u;
        }
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < K {
        *theta.offset(i as isize) = exp(*theta.offset(i as isize) - umax);
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < K {
        *theta
            .offset(
                i as isize,
            ) = *theta.offset(i as isize)
            * gsl_ran_gamma(r, *alpha.offset(i as isize) + 1.0f64, 1.0f64);
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < K {
        norm += *theta.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < K {
        *theta.offset(i as isize) /= norm;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_ran_dirichlet_pdf(
    K: size_t,
    mut alpha: *const libc::c_double,
    mut theta: *const libc::c_double,
) -> libc::c_double {
    return exp(gsl_ran_dirichlet_lnpdf(K, alpha, theta));
}
pub unsafe extern "C" fn gsl_ran_dirichlet_lnpdf(
    K: size_t,
    mut alpha: *const libc::c_double,
    mut theta: *const libc::c_double,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut log_p: libc::c_double = 0.0f64;
    let mut sum_alpha: libc::c_double = 0.0f64;
    i = 0 as libc::c_int as size_t;
    while i < K {
        log_p += (*alpha.offset(i as isize) - 1.0f64) * log(*theta.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < K {
        sum_alpha += *alpha.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    log_p += gsl_sf_lngamma(sum_alpha);
    i = 0 as libc::c_int as size_t;
    while i < K {
        log_p -= gsl_sf_lngamma(*alpha.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    return log_p;
}
