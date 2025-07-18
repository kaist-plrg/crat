use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn log1p(_: libc::c_double) -> libc::c_double;
    fn gsl_ran_beta(
        r: *const gsl_rng,
        a: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_double;
    fn gsl_sf_lnchoose(n: libc::c_uint, m: libc::c_uint) -> libc::c_double;
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
pub unsafe extern "C" fn gsl_ran_binomial_knuth(
    mut r: *const gsl_rng,
    mut p: libc::c_double,
    mut n: libc::c_uint,
) -> libc::c_uint {
    let mut i: libc::c_uint = 0;
    let mut a: libc::c_uint = 0;
    let mut b: libc::c_uint = 0;
    let mut k: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while n > 10 as libc::c_int as libc::c_uint {
        let mut X: libc::c_double = 0.;
        a = (1 as libc::c_int as libc::c_uint)
            .wrapping_add(n.wrapping_div(2 as libc::c_int as libc::c_uint));
        b = (1 as libc::c_int as libc::c_uint).wrapping_add(n).wrapping_sub(a);
        X = gsl_ran_beta(r, a as libc::c_double, b as libc::c_double);
        if X >= p {
            n = a.wrapping_sub(1 as libc::c_int as libc::c_uint);
            p /= X;
        } else {
            k = k.wrapping_add(a);
            n = b.wrapping_sub(1 as libc::c_int as libc::c_uint);
            p = (p - X) / (1 as libc::c_int as libc::c_double - X);
        }
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < n {
        let mut u: libc::c_double = gsl_rng_uniform(r);
        if u < p {
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return k;
}
pub unsafe extern "C" fn gsl_ran_binomial_pdf(
    k: libc::c_uint,
    p: libc::c_double,
    n: libc::c_uint,
) -> libc::c_double {
    if k > n {
        return 0 as libc::c_int as libc::c_double
    } else {
        let mut P: libc::c_double = 0.;
        if p == 0 as libc::c_int as libc::c_double {
            P = (if k == 0 as libc::c_int as libc::c_uint {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            }) as libc::c_double;
        } else if p == 1 as libc::c_int as libc::c_double {
            P = (if k == n { 1 as libc::c_int } else { 0 as libc::c_int })
                as libc::c_double;
        } else {
            let mut ln_Cnk: libc::c_double = gsl_sf_lnchoose(n, k);
            P = ln_Cnk + k as libc::c_double * log(p)
                + n.wrapping_sub(k) as libc::c_double * log1p(-p);
            P = exp(P);
        }
        return P;
    };
}
