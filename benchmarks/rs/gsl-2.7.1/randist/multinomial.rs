use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn gsl_ran_binomial(
        r: *const gsl_rng,
        p: libc::c_double,
        n: libc::c_uint,
    ) -> libc::c_uint;
    fn gsl_sf_lnfact(n: libc::c_uint) -> libc::c_double;
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
pub unsafe extern "C" fn gsl_ran_multinomial(
    mut r: *const gsl_rng,
    K: size_t,
    N: libc::c_uint,
    mut p: *const libc::c_double,
    mut n: *mut libc::c_uint,
) {
    let mut k: size_t = 0;
    let mut norm: libc::c_double = 0.0f64;
    let mut sum_p: libc::c_double = 0.0f64;
    let mut sum_n: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    k = 0 as libc::c_int as size_t;
    while k < K {
        norm += *p.offset(k as isize);
        k = k.wrapping_add(1);
        k;
    }
    k = 0 as libc::c_int as size_t;
    while k < K {
        if *p.offset(k as isize) > 0.0f64 {
            *n
                .offset(
                    k as isize,
                ) = gsl_ran_binomial(
                r,
                *p.offset(k as isize) / (norm - sum_p),
                N.wrapping_sub(sum_n),
            );
        } else {
            *n.offset(k as isize) = 0 as libc::c_int as libc::c_uint;
        }
        sum_p += *p.offset(k as isize);
        sum_n = sum_n.wrapping_add(*n.offset(k as isize));
        k = k.wrapping_add(1);
        k;
    }
}
pub unsafe extern "C" fn gsl_ran_multinomial_pdf(
    K: size_t,
    mut p: *const libc::c_double,
    mut n: *const libc::c_uint,
) -> libc::c_double {
    return exp(gsl_ran_multinomial_lnpdf(K, p, n));
}
pub unsafe extern "C" fn gsl_ran_multinomial_lnpdf(
    K: size_t,
    mut p: *const libc::c_double,
    mut n: *const libc::c_uint,
) -> libc::c_double {
    let mut k: size_t = 0;
    let mut N: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut log_pdf: libc::c_double = 0.0f64;
    let mut norm: libc::c_double = 0.0f64;
    k = 0 as libc::c_int as size_t;
    while k < K {
        N = N.wrapping_add(*n.offset(k as isize));
        k = k.wrapping_add(1);
        k;
    }
    k = 0 as libc::c_int as size_t;
    while k < K {
        norm += *p.offset(k as isize);
        k = k.wrapping_add(1);
        k;
    }
    log_pdf = gsl_sf_lnfact(N);
    k = 0 as libc::c_int as size_t;
    while k < K {
        if *n.offset(k as isize) > 0 as libc::c_int as libc::c_uint {
            log_pdf
                += log(*p.offset(k as isize) / norm)
                    * *n.offset(k as isize) as libc::c_double
                    - gsl_sf_lnfact(*n.offset(k as isize));
        }
        k = k.wrapping_add(1);
        k;
    }
    return log_pdf;
}
