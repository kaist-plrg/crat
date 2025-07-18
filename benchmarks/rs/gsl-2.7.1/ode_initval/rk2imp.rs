use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
pub struct gsl_odeiv_system {
    pub function: Option::<
        unsafe extern "C" fn(
            libc::c_double,
            *const libc::c_double,
            *mut libc::c_double,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub jacobian: Option::<
        unsafe extern "C" fn(
            libc::c_double,
            *const libc::c_double,
            *mut libc::c_double,
            *mut libc::c_double,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub dimension: size_t,
    pub params: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_odeiv_step_type {
    pub name: *const libc::c_char,
    pub can_use_dydt_in: libc::c_int,
    pub gives_exact_dydt_out: libc::c_int,
    pub alloc: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub apply: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            size_t,
            libc::c_double,
            libc::c_double,
            *mut libc::c_double,
            *mut libc::c_double,
            *const libc::c_double,
            *mut libc::c_double,
            *const gsl_odeiv_system,
        ) -> libc::c_int,
    >,
    pub reset: Option::<unsafe extern "C" fn(*mut libc::c_void, size_t) -> libc::c_int>,
    pub order: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint>,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rk2imp_state_t {
    pub Y1: *mut libc::c_double,
    pub y0: *mut libc::c_double,
    pub ytmp: *mut libc::c_double,
    pub y_onestep: *mut libc::c_double,
    pub y0_orig: *mut libc::c_double,
}
unsafe extern "C" fn rk2imp_alloc(mut dim: size_t) -> *mut libc::c_void {
    let mut state: *mut rk2imp_state_t = malloc(
        ::std::mem::size_of::<rk2imp_state_t>() as libc::c_ulong,
    ) as *mut rk2imp_state_t;
    if state.is_null() {
        gsl_error(
            b"failed to allocate space for rk2imp_state\0" as *const u8
                as *const libc::c_char,
            b"rk2imp.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .Y1 = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).Y1).is_null() {
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for Y1\0" as *const u8 as *const libc::c_char,
            b"rk2imp.c\0" as *const u8 as *const libc::c_char,
            66 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .ytmp = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).ytmp).is_null() {
        free((*state).Y1 as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for ytmp\0" as *const u8 as *const libc::c_char,
            b"rk2imp.c\0" as *const u8 as *const libc::c_char,
            75 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .y0 = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).y0).is_null() {
        free((*state).Y1 as *mut libc::c_void);
        free((*state).ytmp as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for y0\0" as *const u8 as *const libc::c_char,
            b"rk2imp.c\0" as *const u8 as *const libc::c_char,
            85 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .y_onestep = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).y_onestep).is_null() {
        free((*state).Y1 as *mut libc::c_void);
        free((*state).ytmp as *mut libc::c_void);
        free((*state).y0 as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for y_onestep\0" as *const u8
                as *const libc::c_char,
            b"rk2imp.c\0" as *const u8 as *const libc::c_char,
            96 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .y0_orig = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).y0_orig).is_null() {
        free((*state).y_onestep as *mut libc::c_void);
        free((*state).Y1 as *mut libc::c_void);
        free((*state).ytmp as *mut libc::c_void);
        free((*state).y0 as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for y0_orig\0" as *const u8
                as *const libc::c_char,
            b"rk2imp.c\0" as *const u8 as *const libc::c_char,
            108 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    return state as *mut libc::c_void;
}
unsafe extern "C" fn rk2imp_step(
    mut y: *mut libc::c_double,
    mut state: *mut rk2imp_state_t,
    h: libc::c_double,
    t: libc::c_double,
    dim: size_t,
    mut sys: *const gsl_odeiv_system,
) -> libc::c_int {
    let mut y0: *const libc::c_double = (*state).y0;
    let mut Y1: *mut libc::c_double = (*state).Y1;
    let mut ytmp: *mut libc::c_double = (*state).ytmp;
    let mut max_iter: libc::c_int = 3 as libc::c_int;
    let mut nu: libc::c_int = 0;
    let mut i: size_t = 0;
    nu = 0 as libc::c_int;
    while nu < max_iter {
        i = 0 as libc::c_int as size_t;
        while i < dim {
            *ytmp
                .offset(
                    i as isize,
                ) = *y0.offset(i as isize) + 0.5f64 * h * *Y1.offset(i as isize);
            i = i.wrapping_add(1);
            i;
        }
        let mut s: libc::c_int = (Some(((*sys).function).unwrap()))
            .unwrap()(t + 0.5f64 * h, ytmp as *const libc::c_double, Y1, (*sys).params);
        if s != GSL_SUCCESS as libc::c_int {
            return s;
        }
        nu += 1;
        nu;
    }
    i = 0 as libc::c_int as size_t;
    while i < dim {
        *y.offset(i as isize) = *y0.offset(i as isize) + h * *Y1.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn rk2imp_apply(
    mut vstate: *mut libc::c_void,
    mut dim: size_t,
    mut t: libc::c_double,
    mut h: libc::c_double,
    mut y: *mut libc::c_double,
    mut yerr: *mut libc::c_double,
    mut dydt_in: *const libc::c_double,
    mut dydt_out: *mut libc::c_double,
    mut sys: *const gsl_odeiv_system,
) -> libc::c_int {
    let mut state: *mut rk2imp_state_t = vstate as *mut rk2imp_state_t;
    let mut i: size_t = 0;
    let mut Y1: *mut libc::c_double = (*state).Y1;
    let mut y0: *mut libc::c_double = (*state).y0;
    let mut y_onestep: *mut libc::c_double = (*state).y_onestep;
    let mut y0_orig: *mut libc::c_double = (*state).y0_orig;
    memcpy(
        y0 as *mut libc::c_void,
        y as *const libc::c_void,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memcpy(
        y0_orig as *mut libc::c_void,
        y as *const libc::c_void,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    if !dydt_in.is_null() {
        memcpy(
            Y1 as *mut libc::c_void,
            dydt_in as *const libc::c_void,
            dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
    } else {
        let mut s: libc::c_int = (Some(((*sys).function).unwrap()))
            .unwrap()(t, y as *const libc::c_double, Y1, (*sys).params);
        if s != GSL_SUCCESS as libc::c_int {
            return s;
        }
    }
    memcpy(
        y_onestep as *mut libc::c_void,
        y as *const libc::c_void,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    let mut s_0: libc::c_int = rk2imp_step(y_onestep, state, h, t, dim, sys);
    if s_0 != GSL_SUCCESS as libc::c_int {
        return s_0;
    }
    let mut s_1: libc::c_int = rk2imp_step(y, state, h / 2.0f64, t, dim, sys);
    if s_1 != GSL_SUCCESS as libc::c_int {
        memcpy(
            y as *mut libc::c_void,
            y0_orig as *const libc::c_void,
            dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
        return s_1;
    }
    memcpy(
        y0 as *mut libc::c_void,
        y as *const libc::c_void,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    let mut s_2: libc::c_int = (Some(((*sys).function).unwrap()))
        .unwrap()(t + h / 2.0f64, y as *const libc::c_double, Y1, (*sys).params);
    if s_2 != GSL_SUCCESS as libc::c_int {
        memcpy(
            y as *mut libc::c_void,
            y0_orig as *const libc::c_void,
            dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
        return s_2;
    }
    let mut s_3: libc::c_int = rk2imp_step(
        y,
        state,
        h / 2.0f64,
        t + h / 2.0f64,
        dim,
        sys,
    );
    if s_3 != GSL_SUCCESS as libc::c_int {
        memcpy(
            y as *mut libc::c_void,
            y0_orig as *const libc::c_void,
            dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
        return s_3;
    }
    if !dydt_out.is_null() {
        let mut s_4: libc::c_int = (Some(((*sys).function).unwrap()))
            .unwrap()(t + h, y as *const libc::c_double, dydt_out, (*sys).params);
        if s_4 != GSL_SUCCESS as libc::c_int {
            memcpy(
                y as *mut libc::c_void,
                y0_orig as *const libc::c_void,
                dim
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    ),
            );
            return s_4;
        }
    }
    i = 0 as libc::c_int as size_t;
    while i < dim {
        *yerr
            .offset(
                i as isize,
            ) = 4.0f64 * (*y.offset(i as isize) - *y_onestep.offset(i as isize))
            / 3.0f64;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn rk2imp_reset(
    mut vstate: *mut libc::c_void,
    mut dim: size_t,
) -> libc::c_int {
    let mut state: *mut rk2imp_state_t = vstate as *mut rk2imp_state_t;
    memset(
        (*state).Y1 as *mut libc::c_void,
        0 as libc::c_int,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memset(
        (*state).ytmp as *mut libc::c_void,
        0 as libc::c_int,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memset(
        (*state).y0 as *mut libc::c_void,
        0 as libc::c_int,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memset(
        (*state).y_onestep as *mut libc::c_void,
        0 as libc::c_int,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memset(
        (*state).y0_orig as *mut libc::c_void,
        0 as libc::c_int,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn rk2imp_order(mut vstate: *mut libc::c_void) -> libc::c_uint {
    let mut state: *mut rk2imp_state_t = vstate as *mut rk2imp_state_t;
    state = 0 as *mut rk2imp_state_t;
    return 2 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn rk2imp_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut rk2imp_state_t = vstate as *mut rk2imp_state_t;
    free((*state).Y1 as *mut libc::c_void);
    free((*state).ytmp as *mut libc::c_void);
    free((*state).y0 as *mut libc::c_void);
    free((*state).y_onestep as *mut libc::c_void);
    free((*state).y0_orig as *mut libc::c_void);
    free(state as *mut libc::c_void);
}
static mut rk2imp_type: gsl_odeiv_step_type = {
    let mut init = gsl_odeiv_step_type {
        name: b"rk2imp\0" as *const u8 as *const libc::c_char,
        can_use_dydt_in: 1 as libc::c_int,
        gives_exact_dydt_out: 1 as libc::c_int,
        alloc: Some(rk2imp_alloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
        apply: Some(
            rk2imp_apply
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    size_t,
                    libc::c_double,
                    libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *const libc::c_double,
                    *mut libc::c_double,
                    *const gsl_odeiv_system,
                ) -> libc::c_int,
        ),
        reset: Some(
            rk2imp_reset
                as unsafe extern "C" fn(*mut libc::c_void, size_t) -> libc::c_int,
        ),
        order: Some(
            rk2imp_order as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint,
        ),
        free: Some(rk2imp_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    };
    init
};
pub static mut gsl_odeiv_step_rk2imp: *const gsl_odeiv_step_type = unsafe {
    &rk2imp_type as *const gsl_odeiv_step_type
};
