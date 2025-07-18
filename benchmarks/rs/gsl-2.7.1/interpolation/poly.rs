use ::libc;
extern "C" {
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn gsl_poly_dd_taylor(
        c: *mut libc::c_double,
        xp: libc::c_double,
        dd: *const libc::c_double,
        x: *const libc::c_double,
        size: size_t,
        w: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_poly_dd_init(
        dd: *mut libc::c_double,
        x: *const libc::c_double,
        y: *const libc::c_double,
        size: size_t,
    ) -> libc::c_int;
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
pub struct polynomial_state_t {
    pub d: *mut libc::c_double,
    pub coeff: *mut libc::c_double,
    pub work: *mut libc::c_double,
}
#[inline]
unsafe extern "C" fn gsl_poly_dd_eval(
    mut dd: *const libc::c_double,
    mut xa: *const libc::c_double,
    size: size_t,
    x: libc::c_double,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut y: libc::c_double = *dd
        .offset(size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
    i = size.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    loop {
        let fresh0 = i;
        i = i.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        y = *dd.offset(i as isize) + (x - *xa.offset(i as isize)) * y;
    }
    return y;
}
unsafe extern "C" fn polynomial_alloc(mut size: size_t) -> *mut libc::c_void {
    let mut state: *mut polynomial_state_t = malloc(
        ::std::mem::size_of::<polynomial_state_t>() as libc::c_ulong,
    ) as *mut polynomial_state_t;
    if state.is_null() {
        gsl_error(
            b"failed to allocate space for polynomial state\0" as *const u8
                as *const libc::c_char,
            b"poly.c\0" as *const u8 as *const libc::c_char,
            43 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .d = malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong).wrapping_mul(size),
    ) as *mut libc::c_double;
    if ((*state).d).is_null() {
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for d\0" as *const u8 as *const libc::c_char,
            b"poly.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .coeff = malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong).wrapping_mul(size),
    ) as *mut libc::c_double;
    if ((*state).coeff).is_null() {
        free((*state).d as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for d\0" as *const u8 as *const libc::c_char,
            b"poly.c\0" as *const u8 as *const libc::c_char,
            60 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .work = malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong).wrapping_mul(size),
    ) as *mut libc::c_double;
    if ((*state).work).is_null() {
        free((*state).coeff as *mut libc::c_void);
        free((*state).d as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for d\0" as *const u8 as *const libc::c_char,
            b"poly.c\0" as *const u8 as *const libc::c_char,
            70 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    return state as *mut libc::c_void;
}
unsafe extern "C" fn polynomial_init(
    mut vstate: *mut libc::c_void,
    mut xa: *const libc::c_double,
    mut ya: *const libc::c_double,
    mut size: size_t,
) -> libc::c_int {
    let mut state: *mut polynomial_state_t = vstate as *mut polynomial_state_t;
    let mut status: libc::c_int = gsl_poly_dd_init((*state).d, xa, ya, size);
    return status;
}
unsafe extern "C" fn polynomial_eval(
    mut vstate: *const libc::c_void,
    mut xa: *const libc::c_double,
    mut ya: *const libc::c_double,
    mut size: size_t,
    mut x: libc::c_double,
    mut acc: *mut gsl_interp_accel,
    mut y: *mut libc::c_double,
) -> libc::c_int {
    let mut state: *const polynomial_state_t = vstate as *const polynomial_state_t;
    *y = gsl_poly_dd_eval((*state).d as *const libc::c_double, xa, size, x);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn polynomial_deriv(
    mut vstate: *const libc::c_void,
    mut xa: *const libc::c_double,
    mut ya: *const libc::c_double,
    mut size: size_t,
    mut x: libc::c_double,
    mut acc: *mut gsl_interp_accel,
    mut y: *mut libc::c_double,
) -> libc::c_int {
    let mut state: *const polynomial_state_t = vstate as *const polynomial_state_t;
    gsl_poly_dd_taylor(
        (*state).coeff,
        x,
        (*state).d as *const libc::c_double,
        xa,
        size,
        (*state).work,
    );
    *y = *((*state).coeff).offset(1 as libc::c_int as isize);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn polynomial_deriv2(
    mut vstate: *const libc::c_void,
    mut xa: *const libc::c_double,
    mut ya: *const libc::c_double,
    mut size: size_t,
    mut x: libc::c_double,
    mut acc: *mut gsl_interp_accel,
    mut y: *mut libc::c_double,
) -> libc::c_int {
    let mut state: *const polynomial_state_t = vstate as *const polynomial_state_t;
    gsl_poly_dd_taylor(
        (*state).coeff,
        x,
        (*state).d as *const libc::c_double,
        xa,
        size,
        (*state).work,
    );
    *y = 2.0f64 * *((*state).coeff).offset(2 as libc::c_int as isize);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn polynomial_integ(
    mut vstate: *const libc::c_void,
    mut xa: *const libc::c_double,
    mut ya: *const libc::c_double,
    mut size: size_t,
    mut acc: *mut gsl_interp_accel,
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut result: *mut libc::c_double,
) -> libc::c_int {
    let mut state: *const polynomial_state_t = vstate as *const polynomial_state_t;
    let mut i: size_t = 0;
    let mut sum: libc::c_double = 0.;
    gsl_poly_dd_taylor(
        (*state).coeff,
        0.0f64,
        (*state).d as *const libc::c_double,
        xa,
        size,
        (*state).work,
    );
    sum = *((*state).coeff).offset(0 as libc::c_int as isize) * (b - a);
    i = 1 as libc::c_int as size_t;
    while i < size {
        sum
            += *((*state).coeff).offset(i as isize)
                * (pow(
                    b,
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_double,
                )
                    - pow(
                        a,
                        i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                            as libc::c_double,
                    )) / (i as libc::c_double + 1.0f64);
        i = i.wrapping_add(1);
        i;
    }
    *result = sum;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn polynomial_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut polynomial_state_t = vstate as *mut polynomial_state_t;
    free((*state).d as *mut libc::c_void);
    free((*state).coeff as *mut libc::c_void);
    free((*state).work as *mut libc::c_void);
    free(state as *mut libc::c_void);
}
static mut polynomial_type: gsl_interp_type = {
    let mut init = gsl_interp_type {
        name: b"polynomial\0" as *const u8 as *const libc::c_char,
        min_size: 3 as libc::c_int as libc::c_uint,
        alloc: Some(
            polynomial_alloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void,
        ),
        init: Some(
            polynomial_init
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_double,
                    *const libc::c_double,
                    size_t,
                ) -> libc::c_int,
        ),
        eval: Some(
            polynomial_eval
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_double,
                    *const libc::c_double,
                    size_t,
                    libc::c_double,
                    *mut gsl_interp_accel,
                    *mut libc::c_double,
                ) -> libc::c_int,
        ),
        eval_deriv: Some(
            polynomial_deriv
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_double,
                    *const libc::c_double,
                    size_t,
                    libc::c_double,
                    *mut gsl_interp_accel,
                    *mut libc::c_double,
                ) -> libc::c_int,
        ),
        eval_deriv2: Some(
            polynomial_deriv2
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_double,
                    *const libc::c_double,
                    size_t,
                    libc::c_double,
                    *mut gsl_interp_accel,
                    *mut libc::c_double,
                ) -> libc::c_int,
        ),
        eval_integ: Some(
            polynomial_integ
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_double,
                    *const libc::c_double,
                    size_t,
                    *mut gsl_interp_accel,
                    libc::c_double,
                    libc::c_double,
                    *mut libc::c_double,
                ) -> libc::c_int,
        ),
        free: Some(polynomial_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    };
    init
};
pub static mut gsl_interp_polynomial: *const gsl_interp_type = unsafe {
    &polynomial_type as *const gsl_interp_type
};
