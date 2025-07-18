use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_interp_free(interp: *mut gsl_interp);
    fn gsl_interp_eval_integ(
        obj: *const gsl_interp,
        xa: *const libc::c_double,
        ya: *const libc::c_double,
        a: libc::c_double,
        b: libc::c_double,
        acc: *mut gsl_interp_accel,
    ) -> libc::c_double;
    fn gsl_interp_eval_integ_e(
        obj: *const gsl_interp,
        xa: *const libc::c_double,
        ya: *const libc::c_double,
        a: libc::c_double,
        b: libc::c_double,
        acc: *mut gsl_interp_accel,
        result: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_interp_eval_deriv2(
        obj: *const gsl_interp,
        xa: *const libc::c_double,
        ya: *const libc::c_double,
        x: libc::c_double,
        a: *mut gsl_interp_accel,
    ) -> libc::c_double;
    fn gsl_interp_eval_deriv2_e(
        obj: *const gsl_interp,
        xa: *const libc::c_double,
        ya: *const libc::c_double,
        x: libc::c_double,
        a: *mut gsl_interp_accel,
        d2: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_interp_eval_deriv(
        obj: *const gsl_interp,
        xa: *const libc::c_double,
        ya: *const libc::c_double,
        x: libc::c_double,
        a: *mut gsl_interp_accel,
    ) -> libc::c_double;
    fn gsl_interp_eval_deriv_e(
        obj: *const gsl_interp,
        xa: *const libc::c_double,
        ya: *const libc::c_double,
        x: libc::c_double,
        a: *mut gsl_interp_accel,
        d: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_interp_eval(
        obj: *const gsl_interp,
        xa: *const libc::c_double,
        ya: *const libc::c_double,
        x: libc::c_double,
        a: *mut gsl_interp_accel,
    ) -> libc::c_double;
    fn gsl_interp_eval_e(
        obj: *const gsl_interp,
        xa: *const libc::c_double,
        ya: *const libc::c_double,
        x: libc::c_double,
        a: *mut gsl_interp_accel,
        y: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_interp_min_size(interp: *const gsl_interp) -> libc::c_uint;
    fn gsl_interp_name(interp: *const gsl_interp) -> *const libc::c_char;
    fn gsl_interp_init(
        obj: *mut gsl_interp,
        xa: *const libc::c_double,
        ya: *const libc::c_double,
        size: size_t,
    ) -> libc::c_int;
    fn gsl_interp_alloc(T: *const gsl_interp_type, n: size_t) -> *mut gsl_interp;
    fn free(__ptr: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_spline {
    pub interp: *mut gsl_interp,
    pub x: *mut libc::c_double,
    pub y: *mut libc::c_double,
    pub size: size_t,
}
pub unsafe extern "C" fn gsl_spline_alloc(
    mut T: *const gsl_interp_type,
    mut size: size_t,
) -> *mut gsl_spline {
    let mut spline: *mut gsl_spline = malloc(
        ::std::mem::size_of::<gsl_spline>() as libc::c_ulong,
    ) as *mut gsl_spline;
    if spline.is_null() {
        gsl_error(
            b"failed to allocate space for spline struct\0" as *const u8
                as *const libc::c_char,
            b"spline.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spline;
    }
    (*spline).interp = gsl_interp_alloc(T, size);
    if ((*spline).interp).is_null() {
        free(spline as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for interp\0" as *const u8 as *const libc::c_char,
            b"spline.c\0" as *const u8 as *const libc::c_char,
            42 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spline;
    }
    (*spline)
        .x = malloc(
        size.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*spline).x).is_null() {
        gsl_interp_free((*spline).interp);
        free(spline as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for x\0" as *const u8 as *const libc::c_char,
            b"spline.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spline;
    }
    (*spline)
        .y = malloc(
        size.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*spline).y).is_null() {
        free((*spline).x as *mut libc::c_void);
        gsl_interp_free((*spline).interp);
        free(spline as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for y\0" as *const u8 as *const libc::c_char,
            b"spline.c\0" as *const u8 as *const libc::c_char,
            61 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spline;
    }
    (*spline).size = size;
    return spline;
}
pub unsafe extern "C" fn gsl_spline_init(
    mut spline: *mut gsl_spline,
    mut x_array: *const libc::c_double,
    mut y_array: *const libc::c_double,
    mut size: size_t,
) -> libc::c_int {
    if size != (*spline).size {
        gsl_error(
            b"data must match size of spline object\0" as *const u8
                as *const libc::c_char,
            b"spline.c\0" as *const u8 as *const libc::c_char,
            74 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    memcpy(
        (*spline).x as *mut libc::c_void,
        x_array as *const libc::c_void,
        size.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memcpy(
        (*spline).y as *mut libc::c_void,
        y_array as *const libc::c_void,
        size.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    let mut status: libc::c_int = gsl_interp_init(
        (*spline).interp,
        x_array,
        y_array,
        size,
    );
    return status;
}
pub unsafe extern "C" fn gsl_spline_name(
    mut spline: *const gsl_spline,
) -> *const libc::c_char {
    return gsl_interp_name((*spline).interp);
}
pub unsafe extern "C" fn gsl_spline_min_size(
    mut spline: *const gsl_spline,
) -> libc::c_uint {
    return gsl_interp_min_size((*spline).interp);
}
pub unsafe extern "C" fn gsl_spline_free(mut spline: *mut gsl_spline) {
    if spline.is_null() {
        return;
    }
    gsl_interp_free((*spline).interp);
    free((*spline).x as *mut libc::c_void);
    free((*spline).y as *mut libc::c_void);
    free(spline as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_spline_eval_e(
    mut spline: *const gsl_spline,
    mut x: libc::c_double,
    mut a: *mut gsl_interp_accel,
    mut y: *mut libc::c_double,
) -> libc::c_int {
    return gsl_interp_eval_e(
        (*spline).interp,
        (*spline).x as *const libc::c_double,
        (*spline).y as *const libc::c_double,
        x,
        a,
        y,
    );
}
pub unsafe extern "C" fn gsl_spline_eval(
    mut spline: *const gsl_spline,
    mut x: libc::c_double,
    mut a: *mut gsl_interp_accel,
) -> libc::c_double {
    return gsl_interp_eval(
        (*spline).interp,
        (*spline).x as *const libc::c_double,
        (*spline).y as *const libc::c_double,
        x,
        a,
    );
}
pub unsafe extern "C" fn gsl_spline_eval_deriv_e(
    mut spline: *const gsl_spline,
    mut x: libc::c_double,
    mut a: *mut gsl_interp_accel,
    mut dydx: *mut libc::c_double,
) -> libc::c_int {
    return gsl_interp_eval_deriv_e(
        (*spline).interp,
        (*spline).x as *const libc::c_double,
        (*spline).y as *const libc::c_double,
        x,
        a,
        dydx,
    );
}
pub unsafe extern "C" fn gsl_spline_eval_deriv(
    mut spline: *const gsl_spline,
    mut x: libc::c_double,
    mut a: *mut gsl_interp_accel,
) -> libc::c_double {
    return gsl_interp_eval_deriv(
        (*spline).interp,
        (*spline).x as *const libc::c_double,
        (*spline).y as *const libc::c_double,
        x,
        a,
    );
}
pub unsafe extern "C" fn gsl_spline_eval_deriv2_e(
    mut spline: *const gsl_spline,
    mut x: libc::c_double,
    mut a: *mut gsl_interp_accel,
    mut d2: *mut libc::c_double,
) -> libc::c_int {
    return gsl_interp_eval_deriv2_e(
        (*spline).interp,
        (*spline).x as *const libc::c_double,
        (*spline).y as *const libc::c_double,
        x,
        a,
        d2,
    );
}
pub unsafe extern "C" fn gsl_spline_eval_deriv2(
    mut spline: *const gsl_spline,
    mut x: libc::c_double,
    mut a: *mut gsl_interp_accel,
) -> libc::c_double {
    return gsl_interp_eval_deriv2(
        (*spline).interp,
        (*spline).x as *const libc::c_double,
        (*spline).y as *const libc::c_double,
        x,
        a,
    );
}
pub unsafe extern "C" fn gsl_spline_eval_integ_e(
    mut spline: *const gsl_spline,
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut acc: *mut gsl_interp_accel,
    mut result: *mut libc::c_double,
) -> libc::c_int {
    return gsl_interp_eval_integ_e(
        (*spline).interp,
        (*spline).x as *const libc::c_double,
        (*spline).y as *const libc::c_double,
        a,
        b,
        acc,
        result,
    );
}
pub unsafe extern "C" fn gsl_spline_eval_integ(
    mut spline: *const gsl_spline,
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut acc: *mut gsl_interp_accel,
) -> libc::c_double {
    return gsl_interp_eval_integ(
        (*spline).interp,
        (*spline).x as *const libc::c_double,
        (*spline).y as *const libc::c_double,
        a,
        b,
        acc,
    );
}
