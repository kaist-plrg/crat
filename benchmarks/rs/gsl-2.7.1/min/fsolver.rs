use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_finite(x: libc::c_double) -> libc::c_int;
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
pub struct gsl_function_struct {
    pub function: Option::<
        unsafe extern "C" fn(libc::c_double, *mut libc::c_void) -> libc::c_double,
    >,
    pub params: *mut libc::c_void,
}
pub type gsl_function = gsl_function_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_min_fminimizer_type {
    pub name: *const libc::c_char,
    pub size: size_t,
    pub set: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut gsl_function,
            libc::c_double,
            libc::c_double,
            libc::c_double,
            libc::c_double,
            libc::c_double,
            libc::c_double,
        ) -> libc::c_int,
    >,
    pub iterate: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut gsl_function,
            *mut libc::c_double,
            *mut libc::c_double,
            *mut libc::c_double,
            *mut libc::c_double,
            *mut libc::c_double,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_min_fminimizer {
    pub type_0: *const gsl_min_fminimizer_type,
    pub function: *mut gsl_function,
    pub x_minimum: libc::c_double,
    pub x_lower: libc::c_double,
    pub x_upper: libc::c_double,
    pub f_minimum: libc::c_double,
    pub f_lower: libc::c_double,
    pub f_upper: libc::c_double,
    pub state: *mut libc::c_void,
}
unsafe extern "C" fn compute_f_values(
    mut f: *mut gsl_function,
    mut x_minimum: libc::c_double,
    mut f_minimum: *mut libc::c_double,
    mut x_lower: libc::c_double,
    mut f_lower: *mut libc::c_double,
    mut x_upper: libc::c_double,
    mut f_upper: *mut libc::c_double,
) -> libc::c_int {
    *f_lower = (Some(((*f).function).unwrap())).unwrap()(x_lower, (*f).params);
    if gsl_finite(*f_lower) == 0 {
        gsl_error(
            b"computed function value is infinite or NaN\0" as *const u8
                as *const libc::c_char,
            b"fsolver.c\0" as *const u8 as *const libc::c_char,
            39 as libc::c_int,
            GSL_EBADFUNC as libc::c_int,
        );
        return GSL_EBADFUNC as libc::c_int;
    }
    *f_upper = (Some(((*f).function).unwrap())).unwrap()(x_upper, (*f).params);
    if gsl_finite(*f_upper) == 0 {
        gsl_error(
            b"computed function value is infinite or NaN\0" as *const u8
                as *const libc::c_char,
            b"fsolver.c\0" as *const u8 as *const libc::c_char,
            40 as libc::c_int,
            GSL_EBADFUNC as libc::c_int,
        );
        return GSL_EBADFUNC as libc::c_int;
    }
    *f_minimum = (Some(((*f).function).unwrap())).unwrap()(x_minimum, (*f).params);
    if gsl_finite(*f_minimum) == 0 {
        gsl_error(
            b"computed function value is infinite or NaN\0" as *const u8
                as *const libc::c_char,
            b"fsolver.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EBADFUNC as libc::c_int,
        );
        return GSL_EBADFUNC as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_min_fminimizer_set(
    mut s: *mut gsl_min_fminimizer,
    mut f: *mut gsl_function,
    mut x_minimum: libc::c_double,
    mut x_lower: libc::c_double,
    mut x_upper: libc::c_double,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut f_minimum: libc::c_double = 0.;
    let mut f_lower: libc::c_double = 0.;
    let mut f_upper: libc::c_double = 0.;
    status = compute_f_values(
        f,
        x_minimum,
        &mut f_minimum,
        x_lower,
        &mut f_lower,
        x_upper,
        &mut f_upper,
    );
    if status != GSL_SUCCESS as libc::c_int {
        return status;
    }
    status = gsl_min_fminimizer_set_with_values(
        s,
        f,
        x_minimum,
        f_minimum,
        x_lower,
        f_lower,
        x_upper,
        f_upper,
    );
    return status;
}
pub unsafe extern "C" fn gsl_min_fminimizer_alloc(
    mut T: *const gsl_min_fminimizer_type,
) -> *mut gsl_min_fminimizer {
    let mut s: *mut gsl_min_fminimizer = malloc(
        ::std::mem::size_of::<gsl_min_fminimizer>() as libc::c_ulong,
    ) as *mut gsl_min_fminimizer;
    if s.is_null() {
        gsl_error(
            b"failed to allocate space for minimizer struct\0" as *const u8
                as *const libc::c_char,
            b"fsolver.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_min_fminimizer;
    }
    (*s).state = malloc((*T).size);
    if ((*s).state).is_null() {
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for minimizer state\0" as *const u8
                as *const libc::c_char,
            b"fsolver.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_min_fminimizer;
    }
    (*s).type_0 = T;
    (*s).function = 0 as *mut gsl_function;
    return s;
}
pub unsafe extern "C" fn gsl_min_fminimizer_set_with_values(
    mut s: *mut gsl_min_fminimizer,
    mut f: *mut gsl_function,
    mut x_minimum: libc::c_double,
    mut f_minimum: libc::c_double,
    mut x_lower: libc::c_double,
    mut f_lower: libc::c_double,
    mut x_upper: libc::c_double,
    mut f_upper: libc::c_double,
) -> libc::c_int {
    (*s).function = f;
    (*s).x_minimum = x_minimum;
    (*s).x_lower = x_lower;
    (*s).x_upper = x_upper;
    if x_lower > x_upper {
        gsl_error(
            b"invalid interval (lower > upper)\0" as *const u8 as *const libc::c_char,
            b"fsolver.c\0" as *const u8 as *const libc::c_char,
            111 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if x_minimum >= x_upper || x_minimum <= x_lower {
        gsl_error(
            b"x_minimum must lie inside interval (lower < x < upper)\0" as *const u8
                as *const libc::c_char,
            b"fsolver.c\0" as *const u8 as *const libc::c_char,
            117 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    (*s).f_lower = f_lower;
    (*s).f_upper = f_upper;
    (*s).f_minimum = f_minimum;
    if f_minimum >= f_lower || f_minimum >= f_upper {
        gsl_error(
            b"endpoints do not enclose a minimum\0" as *const u8 as *const libc::c_char,
            b"fsolver.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    return ((*(*s).type_0).set)
        .unwrap()(
        (*s).state,
        (*s).function,
        x_minimum,
        f_minimum,
        x_lower,
        f_lower,
        x_upper,
        f_upper,
    );
}
pub unsafe extern "C" fn gsl_min_fminimizer_iterate(
    mut s: *mut gsl_min_fminimizer,
) -> libc::c_int {
    return ((*(*s).type_0).iterate)
        .unwrap()(
        (*s).state,
        (*s).function,
        &mut (*s).x_minimum,
        &mut (*s).f_minimum,
        &mut (*s).x_lower,
        &mut (*s).f_lower,
        &mut (*s).x_upper,
        &mut (*s).f_upper,
    );
}
pub unsafe extern "C" fn gsl_min_fminimizer_free(mut s: *mut gsl_min_fminimizer) {
    if s.is_null() {
        return;
    }
    free((*s).state);
    free(s as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_min_fminimizer_name(
    mut s: *const gsl_min_fminimizer,
) -> *const libc::c_char {
    return (*(*s).type_0).name;
}
pub unsafe extern "C" fn gsl_min_fminimizer_minimum(
    mut s: *const gsl_min_fminimizer,
) -> libc::c_double {
    return (*s).x_minimum;
}
pub unsafe extern "C" fn gsl_min_fminimizer_x_minimum(
    mut s: *const gsl_min_fminimizer,
) -> libc::c_double {
    return (*s).x_minimum;
}
pub unsafe extern "C" fn gsl_min_fminimizer_x_lower(
    mut s: *const gsl_min_fminimizer,
) -> libc::c_double {
    return (*s).x_lower;
}
pub unsafe extern "C" fn gsl_min_fminimizer_x_upper(
    mut s: *const gsl_min_fminimizer,
) -> libc::c_double {
    return (*s).x_upper;
}
pub unsafe extern "C" fn gsl_min_fminimizer_f_minimum(
    mut s: *const gsl_min_fminimizer,
) -> libc::c_double {
    return (*s).f_minimum;
}
pub unsafe extern "C" fn gsl_min_fminimizer_f_lower(
    mut s: *const gsl_min_fminimizer,
) -> libc::c_double {
    return (*s).f_lower;
}
pub unsafe extern "C" fn gsl_min_fminimizer_f_upper(
    mut s: *const gsl_min_fminimizer,
) -> libc::c_double {
    return (*s).f_upper;
}
