use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
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
pub unsafe extern "C" fn gsl_ran_hypergeometric(
    mut r: *const gsl_rng,
    mut n1: libc::c_uint,
    mut n2: libc::c_uint,
    mut t: libc::c_uint,
) -> libc::c_uint {
    let n: libc::c_uint = n1.wrapping_add(n2);
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut a: libc::c_uint = n1;
    let mut b: libc::c_uint = n1.wrapping_add(n2);
    let mut k: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if t > n {
        t = n;
    }
    if t < n.wrapping_div(2 as libc::c_int as libc::c_uint) {
        i = 0 as libc::c_int as libc::c_uint;
        while i < t {
            let mut u: libc::c_double = gsl_rng_uniform(r);
            if b as libc::c_double * u < a as libc::c_double {
                k = k.wrapping_add(1);
                k;
                if k == n1 {
                    return k;
                }
                a = a.wrapping_sub(1);
                a;
            }
            b = b.wrapping_sub(1);
            b;
            i = i.wrapping_add(1);
            i;
        }
        return k;
    } else {
        i = 0 as libc::c_int as libc::c_uint;
        while i < n.wrapping_sub(t) {
            let mut u_0: libc::c_double = gsl_rng_uniform(r);
            if b as libc::c_double * u_0 < a as libc::c_double {
                k = k.wrapping_add(1);
                k;
                if k == n1 {
                    return n1.wrapping_sub(k);
                }
                a = a.wrapping_sub(1);
                a;
            }
            b = b.wrapping_sub(1);
            b;
            i = i.wrapping_add(1);
            i;
        }
        return n1.wrapping_sub(k);
    };
}
pub unsafe extern "C" fn gsl_ran_hypergeometric_pdf(
    k: libc::c_uint,
    n1: libc::c_uint,
    n2: libc::c_uint,
    mut t: libc::c_uint,
) -> libc::c_double {
    if t > n1.wrapping_add(n2) {
        t = n1.wrapping_add(n2);
    }
    if k > n1 || k > t {
        return 0 as libc::c_int as libc::c_double
    } else if t > n2 && k.wrapping_add(n2) < t {
        return 0 as libc::c_int as libc::c_double
    } else {
        let mut p: libc::c_double = 0.;
        let mut c1: libc::c_double = gsl_sf_lnchoose(n1, k);
        let mut c2: libc::c_double = gsl_sf_lnchoose(n2, t.wrapping_sub(k));
        let mut c3: libc::c_double = gsl_sf_lnchoose(n1.wrapping_add(n2), t);
        p = exp(c1 + c2 - c3);
        return p;
    };
}
