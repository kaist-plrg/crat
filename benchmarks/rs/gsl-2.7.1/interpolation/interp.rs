use ::libc;
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
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
pub struct gsl_interp_accel {
    pub cache: size_t,
    pub miss_count: size_t,
    pub hit_count: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_interp_type {
    pub name: *const libc::c_char,
    pub min_size: libc::c_uint,
    pub alloc: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub init: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_double,
            *const libc::c_double,
            size_t,
        ) -> libc::c_int,
    >,
    pub eval: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            *const libc::c_double,
            *const libc::c_double,
            size_t,
            libc::c_double,
            *mut gsl_interp_accel,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    pub eval_deriv: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            *const libc::c_double,
            *const libc::c_double,
            size_t,
            libc::c_double,
            *mut gsl_interp_accel,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    pub eval_deriv2: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            *const libc::c_double,
            *const libc::c_double,
            size_t,
            libc::c_double,
            *mut gsl_interp_accel,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    pub eval_integ: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            *const libc::c_double,
            *const libc::c_double,
            size_t,
            *mut gsl_interp_accel,
            libc::c_double,
            libc::c_double,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_interp {
    pub type_0: *const gsl_interp_type,
    pub xmin: libc::c_double,
    pub xmax: libc::c_double,
    pub size: size_t,
    pub state: *mut libc::c_void,
}
pub unsafe extern "C" fn gsl_interp_alloc(
    mut T: *const gsl_interp_type,
    mut size: size_t,
) -> *mut gsl_interp {
    let mut interp: *mut gsl_interp = 0 as *mut gsl_interp;
    if size < (*T).min_size as libc::c_ulong {
        gsl_error(
            b"insufficient number of points for interpolation type\0" as *const u8
                as *const libc::c_char,
            b"interp.c\0" as *const u8 as *const libc::c_char,
            38 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_interp;
    }
    interp = malloc(::std::mem::size_of::<gsl_interp>() as libc::c_ulong)
        as *mut gsl_interp;
    if interp.is_null() {
        gsl_error(
            b"failed to allocate space for interp struct\0" as *const u8
                as *const libc::c_char,
            b"interp.c\0" as *const u8 as *const libc::c_char,
            46 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_interp;
    }
    (*interp).type_0 = T;
    (*interp).size = size;
    if ((*(*interp).type_0).alloc).is_none() {
        (*interp).state = 0 as *mut libc::c_void;
        return interp;
    }
    (*interp).state = ((*(*interp).type_0).alloc).unwrap()(size);
    if ((*interp).state).is_null() {
        free(interp as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for interp state\0" as *const u8
                as *const libc::c_char,
            b"interp.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_interp;
    }
    return interp;
}
pub unsafe extern "C" fn gsl_interp_init(
    mut interp: *mut gsl_interp,
    mut x_array: *const libc::c_double,
    mut y_array: *const libc::c_double,
    mut size: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    if size != (*interp).size {
        gsl_error(
            b"data must match size of interpolation object\0" as *const u8
                as *const libc::c_char,
            b"interp.c\0" as *const u8 as *const libc::c_char,
            76 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    i = 1 as libc::c_int as size_t;
    while i < size {
        if !(*x_array.offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            < *x_array.offset(i as isize))
        {
            gsl_error(
                b"x values must be strictly increasing\0" as *const u8
                    as *const libc::c_char,
                b"interp.c\0" as *const u8 as *const libc::c_char,
                83 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    (*interp).xmin = *x_array.offset(0 as libc::c_int as isize);
    (*interp)
        .xmax = *x_array
        .offset(size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
    let mut status: libc::c_int = ((*(*interp).type_0).init)
        .unwrap()((*interp).state, x_array, y_array, size);
    return status;
}
pub unsafe extern "C" fn gsl_interp_name(
    mut interp: *const gsl_interp,
) -> *const libc::c_char {
    return (*(*interp).type_0).name;
}
pub unsafe extern "C" fn gsl_interp_min_size(
    mut interp: *const gsl_interp,
) -> libc::c_uint {
    return (*(*interp).type_0).min_size;
}
pub unsafe extern "C" fn gsl_interp_type_min_size(
    mut T: *const gsl_interp_type,
) -> libc::c_uint {
    return (*T).min_size;
}
pub unsafe extern "C" fn gsl_interp_free(mut interp: *mut gsl_interp) {
    if interp.is_null() {
        return;
    }
    if ((*(*interp).type_0).free).is_some() {
        ((*(*interp).type_0).free).unwrap()((*interp).state);
    }
    free(interp as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_interp_eval_e(
    mut interp: *const gsl_interp,
    mut xa: *const libc::c_double,
    mut ya: *const libc::c_double,
    mut x: libc::c_double,
    mut a: *mut gsl_interp_accel,
    mut y: *mut libc::c_double,
) -> libc::c_int {
    if x < (*interp).xmin || x > (*interp).xmax {
        *y = ::std::f32::NAN as libc::c_double;
        return GSL_EDOM as libc::c_int;
    }
    return ((*(*interp).type_0).eval)
        .unwrap()((*interp).state, xa, ya, (*interp).size, x, a, y);
}
pub unsafe extern "C" fn gsl_interp_eval(
    mut interp: *const gsl_interp,
    mut xa: *const libc::c_double,
    mut ya: *const libc::c_double,
    mut x: libc::c_double,
    mut a: *mut gsl_interp_accel,
) -> libc::c_double {
    let mut y: libc::c_double = 0.;
    let mut status: libc::c_int = 0;
    if x < (*interp).xmin || x > (*interp).xmax {
        gsl_error(
            b"interpolation error\0" as *const u8 as *const libc::c_char,
            b"interp.c\0" as *const u8 as *const libc::c_char,
            150 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return ::std::f32::NAN as libc::c_double;
    }
    status = ((*(*interp).type_0).eval)
        .unwrap()((*interp).state, xa, ya, (*interp).size, x, a, &mut y);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"interpolation error\0" as *const u8 as *const libc::c_char,
            b"interp.c\0" as *const u8 as *const libc::c_char,
            155 as libc::c_int,
            status,
        );
        return ::std::f32::NAN as libc::c_double;
    }
    return y;
}
pub unsafe extern "C" fn gsl_interp_eval_deriv_e(
    mut interp: *const gsl_interp,
    mut xa: *const libc::c_double,
    mut ya: *const libc::c_double,
    mut x: libc::c_double,
    mut a: *mut gsl_interp_accel,
    mut dydx: *mut libc::c_double,
) -> libc::c_int {
    if x < (*interp).xmin || x > (*interp).xmax {
        *dydx = ::std::f32::NAN as libc::c_double;
        return GSL_EDOM as libc::c_int;
    }
    return ((*(*interp).type_0).eval_deriv)
        .unwrap()((*interp).state, xa, ya, (*interp).size, x, a, dydx);
}
pub unsafe extern "C" fn gsl_interp_eval_deriv(
    mut interp: *const gsl_interp,
    mut xa: *const libc::c_double,
    mut ya: *const libc::c_double,
    mut x: libc::c_double,
    mut a: *mut gsl_interp_accel,
) -> libc::c_double {
    let mut dydx: libc::c_double = 0.;
    let mut status: libc::c_int = 0;
    if x < (*interp).xmin || x > (*interp).xmax {
        gsl_error(
            b"interpolation error\0" as *const u8 as *const libc::c_char,
            b"interp.c\0" as *const u8 as *const libc::c_char,
            186 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return ::std::f32::NAN as libc::c_double;
    }
    status = ((*(*interp).type_0).eval_deriv)
        .unwrap()((*interp).state, xa, ya, (*interp).size, x, a, &mut dydx);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"interpolation error\0" as *const u8 as *const libc::c_char,
            b"interp.c\0" as *const u8 as *const libc::c_char,
            191 as libc::c_int,
            status,
        );
        return ::std::f32::NAN as libc::c_double;
    }
    return dydx;
}
pub unsafe extern "C" fn gsl_interp_eval_deriv2_e(
    mut interp: *const gsl_interp,
    mut xa: *const libc::c_double,
    mut ya: *const libc::c_double,
    mut x: libc::c_double,
    mut a: *mut gsl_interp_accel,
    mut d2: *mut libc::c_double,
) -> libc::c_int {
    if x < (*interp).xmin || x > (*interp).xmax {
        *d2 = ::std::f32::NAN as libc::c_double;
        return GSL_EDOM as libc::c_int;
    }
    return ((*(*interp).type_0).eval_deriv2)
        .unwrap()((*interp).state, xa, ya, (*interp).size, x, a, d2);
}
pub unsafe extern "C" fn gsl_interp_eval_deriv2(
    mut interp: *const gsl_interp,
    mut xa: *const libc::c_double,
    mut ya: *const libc::c_double,
    mut x: libc::c_double,
    mut a: *mut gsl_interp_accel,
) -> libc::c_double {
    let mut d2: libc::c_double = 0.;
    let mut status: libc::c_int = 0;
    if x < (*interp).xmin || x > (*interp).xmax {
        gsl_error(
            b"interpolation error\0" as *const u8 as *const libc::c_char,
            b"interp.c\0" as *const u8 as *const libc::c_char,
            222 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return ::std::f32::NAN as libc::c_double;
    }
    status = ((*(*interp).type_0).eval_deriv2)
        .unwrap()((*interp).state, xa, ya, (*interp).size, x, a, &mut d2);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"interpolation error\0" as *const u8 as *const libc::c_char,
            b"interp.c\0" as *const u8 as *const libc::c_char,
            227 as libc::c_int,
            status,
        );
        return ::std::f32::NAN as libc::c_double;
    }
    return d2;
}
pub unsafe extern "C" fn gsl_interp_eval_integ_e(
    mut interp: *const gsl_interp,
    mut xa: *const libc::c_double,
    mut ya: *const libc::c_double,
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut acc: *mut gsl_interp_accel,
    mut result: *mut libc::c_double,
) -> libc::c_int {
    if a > b || a < (*interp).xmin || b > (*interp).xmax {
        *result = ::std::f32::NAN as libc::c_double;
        return GSL_EDOM as libc::c_int;
    } else if a == b {
        *result = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    }
    return ((*(*interp).type_0).eval_integ)
        .unwrap()((*interp).state, xa, ya, (*interp).size, acc, a, b, result);
}
pub unsafe extern "C" fn gsl_interp_eval_integ(
    mut interp: *const gsl_interp,
    mut xa: *const libc::c_double,
    mut ya: *const libc::c_double,
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut acc: *mut gsl_interp_accel,
) -> libc::c_double {
    let mut result: libc::c_double = 0.;
    let mut status: libc::c_int = 0;
    if a > b || a < (*interp).xmin || b > (*interp).xmax {
        gsl_error(
            b"interpolation error\0" as *const u8 as *const libc::c_char,
            b"interp.c\0" as *const u8 as *const libc::c_char,
            266 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return ::std::f32::NAN as libc::c_double;
    } else if a == b {
        return 0.0f64
    }
    status = ((*(*interp).type_0).eval_integ)
        .unwrap()((*interp).state, xa, ya, (*interp).size, acc, a, b, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"interpolation error\0" as *const u8 as *const libc::c_char,
            b"interp.c\0" as *const u8 as *const libc::c_char,
            275 as libc::c_int,
            status,
        );
        return ::std::f32::NAN as libc::c_double;
    }
    return result;
}
