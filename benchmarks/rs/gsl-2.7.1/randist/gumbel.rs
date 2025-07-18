use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
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
pub unsafe extern "C" fn gsl_ran_gumbel1(
    mut r: *const gsl_rng,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    let mut x: libc::c_double = gsl_rng_uniform_pos(r);
    let mut z: libc::c_double = (log(b) - log(-log(x))) / a;
    return z;
}
pub unsafe extern "C" fn gsl_ran_gumbel1_pdf(
    x: libc::c_double,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    let mut p: libc::c_double = a * b * exp(-(b * exp(-a * x) + a * x));
    return p;
}
pub unsafe extern "C" fn gsl_ran_gumbel2(
    mut r: *const gsl_rng,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    let mut x: libc::c_double = gsl_rng_uniform_pos(r);
    let mut z: libc::c_double = pow(-b / log(x), 1 as libc::c_int as libc::c_double / a);
    return z;
}
pub unsafe extern "C" fn gsl_ran_gumbel2_pdf(
    x: libc::c_double,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    if x <= 0 as libc::c_int as libc::c_double {
        return 0 as libc::c_int as libc::c_double
    } else {
        let mut p: libc::c_double = b * a
            * pow(x, -(a + 1 as libc::c_int as libc::c_double)) * exp(-b * pow(x, -a));
        return p;
    };
}
