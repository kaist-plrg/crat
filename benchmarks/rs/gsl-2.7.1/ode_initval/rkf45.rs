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
pub struct rkf45_state_t {
    pub k1: *mut libc::c_double,
    pub k2: *mut libc::c_double,
    pub k3: *mut libc::c_double,
    pub k4: *mut libc::c_double,
    pub k5: *mut libc::c_double,
    pub k6: *mut libc::c_double,
    pub y0: *mut libc::c_double,
    pub ytmp: *mut libc::c_double,
}
static mut ah: [libc::c_double; 5] = [
    1.0f64 / 4.0f64,
    3.0f64 / 8.0f64,
    12.0f64 / 13.0f64,
    1.0f64,
    1.0f64 / 2.0f64,
];
static mut b3: [libc::c_double; 2] = [3.0f64 / 32.0f64, 9.0f64 / 32.0f64];
static mut b4: [libc::c_double; 3] = [
    1932.0f64 / 2197.0f64,
    -7200.0f64 / 2197.0f64,
    7296.0f64 / 2197.0f64,
];
static mut b5: [libc::c_double; 4] = [
    8341.0f64 / 4104.0f64,
    -32832.0f64 / 4104.0f64,
    29440.0f64 / 4104.0f64,
    -845.0f64 / 4104.0f64,
];
static mut b6: [libc::c_double; 5] = [
    -6080.0f64 / 20520.0f64,
    41040.0f64 / 20520.0f64,
    -28352.0f64 / 20520.0f64,
    9295.0f64 / 20520.0f64,
    -5643.0f64 / 20520.0f64,
];
static mut c1: libc::c_double = 902880.0f64 / 7618050.0f64;
static mut c3: libc::c_double = 3953664.0f64 / 7618050.0f64;
static mut c4: libc::c_double = 3855735.0f64 / 7618050.0f64;
static mut c5: libc::c_double = -1371249.0f64 / 7618050.0f64;
static mut c6: libc::c_double = 277020.0f64 / 7618050.0f64;
static mut ec: [libc::c_double; 7] = [
    0.0f64,
    1.0f64 / 360.0f64,
    0.0f64,
    -128.0f64 / 4275.0f64,
    -2197.0f64 / 75240.0f64,
    1.0f64 / 50.0f64,
    2.0f64 / 55.0f64,
];
unsafe extern "C" fn rkf45_alloc(mut dim: size_t) -> *mut libc::c_void {
    let mut state: *mut rkf45_state_t = malloc(
        ::std::mem::size_of::<rkf45_state_t>() as libc::c_ulong,
    ) as *mut rkf45_state_t;
    if state.is_null() {
        gsl_error(
            b"failed to allocate space for rkf45_state\0" as *const u8
                as *const libc::c_char,
            b"rkf45.c\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .k1 = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).k1).is_null() {
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for k1\0" as *const u8 as *const libc::c_char,
            b"rkf45.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .k2 = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).k2).is_null() {
        free((*state).k1 as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for k2\0" as *const u8 as *const libc::c_char,
            b"rkf45.c\0" as *const u8 as *const libc::c_char,
            98 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .k3 = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).k3).is_null() {
        free((*state).k2 as *mut libc::c_void);
        free((*state).k1 as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for k3\0" as *const u8 as *const libc::c_char,
            b"rkf45.c\0" as *const u8 as *const libc::c_char,
            108 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .k4 = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).k4).is_null() {
        free((*state).k3 as *mut libc::c_void);
        free((*state).k2 as *mut libc::c_void);
        free((*state).k1 as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for k4\0" as *const u8 as *const libc::c_char,
            b"rkf45.c\0" as *const u8 as *const libc::c_char,
            119 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .k5 = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).k5).is_null() {
        free((*state).k4 as *mut libc::c_void);
        free((*state).k3 as *mut libc::c_void);
        free((*state).k2 as *mut libc::c_void);
        free((*state).k1 as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for k5\0" as *const u8 as *const libc::c_char,
            b"rkf45.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .k6 = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).k6).is_null() {
        free((*state).k5 as *mut libc::c_void);
        free((*state).k4 as *mut libc::c_void);
        free((*state).k3 as *mut libc::c_void);
        free((*state).k2 as *mut libc::c_void);
        free((*state).k1 as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for k6\0" as *const u8 as *const libc::c_char,
            b"rkf45.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .y0 = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).y0).is_null() {
        free((*state).k6 as *mut libc::c_void);
        free((*state).k5 as *mut libc::c_void);
        free((*state).k4 as *mut libc::c_void);
        free((*state).k3 as *mut libc::c_void);
        free((*state).k2 as *mut libc::c_void);
        free((*state).k1 as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for y0\0" as *const u8 as *const libc::c_char,
            b"rkf45.c\0" as *const u8 as *const libc::c_char,
            158 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .ytmp = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).ytmp).is_null() {
        free((*state).y0 as *mut libc::c_void);
        free((*state).k6 as *mut libc::c_void);
        free((*state).k5 as *mut libc::c_void);
        free((*state).k4 as *mut libc::c_void);
        free((*state).k3 as *mut libc::c_void);
        free((*state).k2 as *mut libc::c_void);
        free((*state).k1 as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for ytmp\0" as *const u8 as *const libc::c_char,
            b"rkf45.c\0" as *const u8 as *const libc::c_char,
            173 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    return state as *mut libc::c_void;
}
unsafe extern "C" fn rkf45_apply(
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
    let mut state: *mut rkf45_state_t = vstate as *mut rkf45_state_t;
    let mut i: size_t = 0;
    let k1: *mut libc::c_double = (*state).k1;
    let k2: *mut libc::c_double = (*state).k2;
    let k3: *mut libc::c_double = (*state).k3;
    let k4: *mut libc::c_double = (*state).k4;
    let k5: *mut libc::c_double = (*state).k5;
    let k6: *mut libc::c_double = (*state).k6;
    let ytmp: *mut libc::c_double = (*state).ytmp;
    let y0: *mut libc::c_double = (*state).y0;
    memcpy(
        y0 as *mut libc::c_void,
        y as *const libc::c_void,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    if !dydt_in.is_null() {
        memcpy(
            k1 as *mut libc::c_void,
            dydt_in as *const libc::c_void,
            dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
    } else {
        let mut s: libc::c_int = (Some(((*sys).function).unwrap()))
            .unwrap()(t, y as *const libc::c_double, k1, (*sys).params);
        if s != GSL_SUCCESS as libc::c_int {
            return s;
        }
    }
    i = 0 as libc::c_int as size_t;
    while i < dim {
        *ytmp
            .offset(
                i as isize,
            ) = *y.offset(i as isize)
            + ah[0 as libc::c_int as usize] * h * *k1.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    let mut s_0: libc::c_int = (Some(((*sys).function).unwrap()))
        .unwrap()(
        t + ah[0 as libc::c_int as usize] * h,
        ytmp as *const libc::c_double,
        k2,
        (*sys).params,
    );
    if s_0 != GSL_SUCCESS as libc::c_int {
        return s_0;
    }
    i = 0 as libc::c_int as size_t;
    while i < dim {
        *ytmp
            .offset(
                i as isize,
            ) = *y.offset(i as isize)
            + h
                * (b3[0 as libc::c_int as usize] * *k1.offset(i as isize)
                    + b3[1 as libc::c_int as usize] * *k2.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    let mut s_1: libc::c_int = (Some(((*sys).function).unwrap()))
        .unwrap()(
        t + ah[1 as libc::c_int as usize] * h,
        ytmp as *const libc::c_double,
        k3,
        (*sys).params,
    );
    if s_1 != GSL_SUCCESS as libc::c_int {
        return s_1;
    }
    i = 0 as libc::c_int as size_t;
    while i < dim {
        *ytmp
            .offset(
                i as isize,
            ) = *y.offset(i as isize)
            + h
                * (b4[0 as libc::c_int as usize] * *k1.offset(i as isize)
                    + b4[1 as libc::c_int as usize] * *k2.offset(i as isize)
                    + b4[2 as libc::c_int as usize] * *k3.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    let mut s_2: libc::c_int = (Some(((*sys).function).unwrap()))
        .unwrap()(
        t + ah[2 as libc::c_int as usize] * h,
        ytmp as *const libc::c_double,
        k4,
        (*sys).params,
    );
    if s_2 != GSL_SUCCESS as libc::c_int {
        return s_2;
    }
    i = 0 as libc::c_int as size_t;
    while i < dim {
        *ytmp
            .offset(
                i as isize,
            ) = *y.offset(i as isize)
            + h
                * (b5[0 as libc::c_int as usize] * *k1.offset(i as isize)
                    + b5[1 as libc::c_int as usize] * *k2.offset(i as isize)
                    + b5[2 as libc::c_int as usize] * *k3.offset(i as isize)
                    + b5[3 as libc::c_int as usize] * *k4.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    let mut s_3: libc::c_int = (Some(((*sys).function).unwrap()))
        .unwrap()(
        t + ah[3 as libc::c_int as usize] * h,
        ytmp as *const libc::c_double,
        k5,
        (*sys).params,
    );
    if s_3 != GSL_SUCCESS as libc::c_int {
        return s_3;
    }
    i = 0 as libc::c_int as size_t;
    while i < dim {
        *ytmp
            .offset(
                i as isize,
            ) = *y.offset(i as isize)
            + h
                * (b6[0 as libc::c_int as usize] * *k1.offset(i as isize)
                    + b6[1 as libc::c_int as usize] * *k2.offset(i as isize)
                    + b6[2 as libc::c_int as usize] * *k3.offset(i as isize)
                    + b6[3 as libc::c_int as usize] * *k4.offset(i as isize)
                    + b6[4 as libc::c_int as usize] * *k5.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    let mut s_4: libc::c_int = (Some(((*sys).function).unwrap()))
        .unwrap()(
        t + ah[4 as libc::c_int as usize] * h,
        ytmp as *const libc::c_double,
        k6,
        (*sys).params,
    );
    if s_4 != GSL_SUCCESS as libc::c_int {
        return s_4;
    }
    i = 0 as libc::c_int as size_t;
    while i < dim {
        let d_i: libc::c_double = c1 * *k1.offset(i as isize)
            + c3 * *k3.offset(i as isize) + c4 * *k4.offset(i as isize)
            + c5 * *k5.offset(i as isize) + c6 * *k6.offset(i as isize);
        *y.offset(i as isize) += h * d_i;
        i = i.wrapping_add(1);
        i;
    }
    if !dydt_out.is_null() {
        let mut s_5: libc::c_int = (Some(((*sys).function).unwrap()))
            .unwrap()(t + h, y as *const libc::c_double, dydt_out, (*sys).params);
        if s_5 != GSL_SUCCESS as libc::c_int {
            memcpy(
                y as *mut libc::c_void,
                y0 as *const libc::c_void,
                dim
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    ),
            );
            return s_5;
        }
    }
    i = 0 as libc::c_int as size_t;
    while i < dim {
        *yerr
            .offset(
                i as isize,
            ) = h
            * (ec[1 as libc::c_int as usize] * *k1.offset(i as isize)
                + ec[3 as libc::c_int as usize] * *k3.offset(i as isize)
                + ec[4 as libc::c_int as usize] * *k4.offset(i as isize)
                + ec[5 as libc::c_int as usize] * *k5.offset(i as isize)
                + ec[6 as libc::c_int as usize] * *k6.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn rkf45_reset(
    mut vstate: *mut libc::c_void,
    mut dim: size_t,
) -> libc::c_int {
    let mut state: *mut rkf45_state_t = vstate as *mut rkf45_state_t;
    memset(
        (*state).k1 as *mut libc::c_void,
        0 as libc::c_int,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memset(
        (*state).k2 as *mut libc::c_void,
        0 as libc::c_int,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memset(
        (*state).k3 as *mut libc::c_void,
        0 as libc::c_int,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memset(
        (*state).k4 as *mut libc::c_void,
        0 as libc::c_int,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memset(
        (*state).k5 as *mut libc::c_void,
        0 as libc::c_int,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memset(
        (*state).k6 as *mut libc::c_void,
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
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn rkf45_order(mut vstate: *mut libc::c_void) -> libc::c_uint {
    let mut state: *mut rkf45_state_t = vstate as *mut rkf45_state_t;
    state = 0 as *mut rkf45_state_t;
    return 5 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn rkf45_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut rkf45_state_t = vstate as *mut rkf45_state_t;
    free((*state).ytmp as *mut libc::c_void);
    free((*state).y0 as *mut libc::c_void);
    free((*state).k6 as *mut libc::c_void);
    free((*state).k5 as *mut libc::c_void);
    free((*state).k4 as *mut libc::c_void);
    free((*state).k3 as *mut libc::c_void);
    free((*state).k2 as *mut libc::c_void);
    free((*state).k1 as *mut libc::c_void);
    free(state as *mut libc::c_void);
}
static mut rkf45_type: gsl_odeiv_step_type = {
    let mut init = gsl_odeiv_step_type {
        name: b"rkf45\0" as *const u8 as *const libc::c_char,
        can_use_dydt_in: 1 as libc::c_int,
        gives_exact_dydt_out: 0 as libc::c_int,
        alloc: Some(rkf45_alloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
        apply: Some(
            rkf45_apply
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
            rkf45_reset as unsafe extern "C" fn(*mut libc::c_void, size_t) -> libc::c_int,
        ),
        order: Some(
            rkf45_order as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint,
        ),
        free: Some(rkf45_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    };
    init
};
pub static mut gsl_odeiv_step_rkf45: *const gsl_odeiv_step_type = unsafe {
    &rkf45_type as *const gsl_odeiv_step_type
};
