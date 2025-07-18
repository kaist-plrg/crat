use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
}
pub type size_t = libc::c_ulong;
pub type C2RustUnnamed = libc::c_int;
pub const GSL_EOF: C2RustUnnamed = 32;
pub const GSL_ETOLG: C2RustUnnamed = 31;
pub const GSL_ETOLX: C2RustUnnamed = 30;
pub const GSL_ETOLF: C2RustUnnamed = 29;
pub const GSL_ENOPROGJ: C2RustUnnamed = 28;
pub const GSL_ENOPROG: C2RustUnnamed = 27;
pub const GSL_ETABLE: C2RustUnnamed = 26;
pub const GSL_ECACHE: C2RustUnnamed = 25;
pub const GSL_EUNIMPL: C2RustUnnamed = 24;
pub const GSL_EUNSUP: C2RustUnnamed = 23;
pub const GSL_EDIVERGE: C2RustUnnamed = 22;
pub const GSL_ESING: C2RustUnnamed = 21;
pub const GSL_ENOTSQR: C2RustUnnamed = 20;
pub const GSL_EBADLEN: C2RustUnnamed = 19;
pub const GSL_EROUND: C2RustUnnamed = 18;
pub const GSL_ELOSS: C2RustUnnamed = 17;
pub const GSL_EOVRFLW: C2RustUnnamed = 16;
pub const GSL_EUNDRFLW: C2RustUnnamed = 15;
pub const GSL_ETOL: C2RustUnnamed = 14;
pub const GSL_EBADTOL: C2RustUnnamed = 13;
pub const GSL_EZERODIV: C2RustUnnamed = 12;
pub const GSL_EMAXITER: C2RustUnnamed = 11;
pub const GSL_ERUNAWAY: C2RustUnnamed = 10;
pub const GSL_EBADFUNC: C2RustUnnamed = 9;
pub const GSL_ENOMEM: C2RustUnnamed = 8;
pub const GSL_ESANITY: C2RustUnnamed = 7;
pub const GSL_EFACTOR: C2RustUnnamed = 6;
pub const GSL_EFAILED: C2RustUnnamed = 5;
pub const GSL_EINVAL: C2RustUnnamed = 4;
pub const GSL_EFAULT: C2RustUnnamed = 3;
pub const GSL_ERANGE: C2RustUnnamed = 2;
pub const GSL_EDOM: C2RustUnnamed = 1;
pub const GSL_CONTINUE: C2RustUnnamed = -2;
pub const GSL_FAILURE: C2RustUnnamed = -1;
pub const GSL_SUCCESS: C2RustUnnamed = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_monte_function_struct {
    pub f: Option::<
        unsafe extern "C" fn(
            *mut libc::c_double,
            size_t,
            *mut libc::c_void,
        ) -> libc::c_double,
    >,
    pub dim: size_t,
    pub params: *mut libc::c_void,
}
pub type gsl_monte_function = gsl_monte_function_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_monte_plain_state {
    pub dim: size_t,
    pub x: *mut libc::c_double,
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
pub unsafe extern "C" fn gsl_monte_plain_integrate(
    mut f: *const gsl_monte_function,
    mut xl: *const libc::c_double,
    mut xu: *const libc::c_double,
    dim: size_t,
    calls: size_t,
    mut r: *mut gsl_rng,
    mut state: *mut gsl_monte_plain_state,
    mut result: *mut libc::c_double,
    mut abserr: *mut libc::c_double,
) -> libc::c_int {
    let mut vol: libc::c_double = 0.;
    let mut m: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut q: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut x: *mut libc::c_double = (*state).x;
    let mut n: size_t = 0;
    let mut i: size_t = 0;
    if dim != (*state).dim {
        gsl_error(
            b"number of dimensions must match allocated size\0" as *const u8
                as *const libc::c_char,
            b"plain.c\0" as *const u8 as *const libc::c_char,
            45 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < dim {
        if *xu.offset(i as isize) <= *xl.offset(i as isize) {
            gsl_error(
                b"xu must be greater than xl\0" as *const u8 as *const libc::c_char,
                b"plain.c\0" as *const u8 as *const libc::c_char,
                52 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        if *xu.offset(i as isize) - *xl.offset(i as isize) > 1.7976931348623157e+308f64 {
            gsl_error(
                b"Range of integration is too large, please rescale\0" as *const u8
                    as *const libc::c_char,
                b"plain.c\0" as *const u8 as *const libc::c_char,
                58 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    vol = 1 as libc::c_int as libc::c_double;
    i = 0 as libc::c_int as size_t;
    while i < dim {
        vol *= *xu.offset(i as isize) - *xl.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    n = 0 as libc::c_int as size_t;
    while n < calls {
        i = 0 as libc::c_int as size_t;
        while i < dim {
            *x
                .offset(
                    i as isize,
                ) = *xl.offset(i as isize)
                + gsl_rng_uniform_pos(r)
                    * (*xu.offset(i as isize) - *xl.offset(i as isize));
            i = i.wrapping_add(1);
            i;
        }
        let mut fval: libc::c_double = (Some(((*f).f).unwrap()))
            .unwrap()(x, (*f).dim, (*f).params);
        let mut d: libc::c_double = fval - m;
        m += d / (n as libc::c_double + 1.0f64);
        q += d * d * (n as libc::c_double / (n as libc::c_double + 1.0f64));
        n = n.wrapping_add(1);
        n;
    }
    *result = vol * m;
    if calls < 2 as libc::c_int as libc::c_ulong {
        *abserr = ::std::f32::INFINITY as libc::c_double;
    } else {
        *abserr = vol
            * sqrt(q / (calls as libc::c_double * (calls as libc::c_double - 1.0f64)));
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_monte_plain_alloc(
    mut dim: size_t,
) -> *mut gsl_monte_plain_state {
    let mut s: *mut gsl_monte_plain_state = malloc(
        ::std::mem::size_of::<gsl_monte_plain_state>() as libc::c_ulong,
    ) as *mut gsl_monte_plain_state;
    if s.is_null() {
        gsl_error(
            b"failed to allocate space for state struct\0" as *const u8
                as *const libc::c_char,
            b"plain.c\0" as *const u8 as *const libc::c_char,
            114 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_monte_plain_state;
    }
    (*s)
        .x = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*s).x).is_null() {
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for working vector\0" as *const u8
                as *const libc::c_char,
            b"plain.c\0" as *const u8 as *const libc::c_char,
            123 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_monte_plain_state;
    }
    (*s).dim = dim;
    return s;
}
pub unsafe extern "C" fn gsl_monte_plain_init(
    mut s: *mut gsl_monte_plain_state,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*s).dim {
        *((*s).x).offset(i as isize) = 0.0f64;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_monte_plain_free(mut s: *mut gsl_monte_plain_state) {
    if s.is_null() {
        return;
    }
    free((*s).x as *mut libc::c_void);
    free(s as *mut libc::c_void);
}
