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
    fn gsl_vector_const_view_array(
        v: *const libc::c_double,
        n: size_t,
    ) -> _gsl_vector_const_view;
    fn gsl_vector_view_array(v: *mut libc::c_double, n: size_t) -> _gsl_vector_view;
    fn gsl_vector_add(a: *mut gsl_vector, b: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_scale(a: *mut gsl_vector, x: libc::c_double) -> libc::c_int;
    fn gsl_matrix_view_array(
        base: *mut libc::c_double,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_view;
    fn gsl_linalg_LU_solve(
        LU: *const gsl_matrix,
        p: *const gsl_permutation,
        b: *const gsl_vector,
        x: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_linalg_LU_decomp(
        A: *mut gsl_matrix,
        p: *mut gsl_permutation,
        signum: *mut libc::c_int,
    ) -> libc::c_int;
    fn gsl_matrix_add_diagonal(a: *mut gsl_matrix, x: libc::c_double) -> libc::c_int;
    fn gsl_matrix_scale(a: *mut gsl_matrix, x: libc::c_double) -> libc::c_int;
    fn gsl_permutation_alloc(n: size_t) -> *mut gsl_permutation;
    fn gsl_permutation_free(p: *mut gsl_permutation);
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
pub struct rk2simp_state_t {
    pub Y1: *mut libc::c_double,
    pub y0: *mut libc::c_double,
    pub y0_orig: *mut libc::c_double,
    pub ytmp: *mut libc::c_double,
    pub dfdy: *mut libc::c_double,
    pub dfdt: *mut libc::c_double,
    pub y_onestep: *mut libc::c_double,
    pub p: *mut gsl_permutation,
}
pub type gsl_permutation = gsl_permutation_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_permutation_struct {
    pub size: size_t,
    pub data: *mut size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: libc::c_int,
}
pub type gsl_block = gsl_block_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
}
pub type gsl_vector_const_view = _gsl_vector_const_view;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_vector_const_view {
    pub vector: gsl_vector,
}
pub type gsl_vector_view = _gsl_vector_view;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_vector_view {
    pub vector: gsl_vector,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: libc::c_int,
}
pub type gsl_matrix_view = _gsl_matrix_view;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_matrix_view {
    pub matrix: gsl_matrix,
}
unsafe extern "C" fn rk2simp_alloc(mut dim: size_t) -> *mut libc::c_void {
    let mut state: *mut rk2simp_state_t = malloc(
        ::std::mem::size_of::<rk2simp_state_t>() as libc::c_ulong,
    ) as *mut rk2simp_state_t;
    if state.is_null() {
        gsl_error(
            b"failed to allocate space for rk2simp_state\0" as *const u8
                as *const libc::c_char,
            b"rk2simp.c\0" as *const u8 as *const libc::c_char,
            62 as libc::c_int,
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
            b"rk2simp.c\0" as *const u8 as *const libc::c_char,
            70 as libc::c_int,
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
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for y0\0" as *const u8 as *const libc::c_char,
            b"rk2simp.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .y0_orig = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).y0_orig).is_null() {
        free((*state).Y1 as *mut libc::c_void);
        free((*state).y0 as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for y0_orig\0" as *const u8
                as *const libc::c_char,
            b"rk2simp.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
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
        free((*state).y0 as *mut libc::c_void);
        free((*state).y0_orig as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for ytmp\0" as *const u8 as *const libc::c_char,
            b"rk2simp.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .dfdy = malloc(
        dim
            .wrapping_mul(dim)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).dfdy).is_null() {
        free((*state).Y1 as *mut libc::c_void);
        free((*state).y0 as *mut libc::c_void);
        free((*state).y0_orig as *mut libc::c_void);
        free((*state).ytmp as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for dfdy\0" as *const u8 as *const libc::c_char,
            b"rk2simp.c\0" as *const u8 as *const libc::c_char,
            112 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .dfdt = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).dfdt).is_null() {
        free((*state).Y1 as *mut libc::c_void);
        free((*state).y0 as *mut libc::c_void);
        free((*state).y0_orig as *mut libc::c_void);
        free((*state).ytmp as *mut libc::c_void);
        free((*state).dfdy as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for dfdt\0" as *const u8 as *const libc::c_char,
            b"rk2simp.c\0" as *const u8 as *const libc::c_char,
            125 as libc::c_int,
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
        free((*state).y0 as *mut libc::c_void);
        free((*state).y0_orig as *mut libc::c_void);
        free((*state).ytmp as *mut libc::c_void);
        free((*state).dfdy as *mut libc::c_void);
        free((*state).dfdt as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for y_onestep\0" as *const u8
                as *const libc::c_char,
            b"rk2simp.c\0" as *const u8 as *const libc::c_char,
            139 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).p = gsl_permutation_alloc(dim);
    if ((*state).p).is_null() {
        free((*state).Y1 as *mut libc::c_void);
        free((*state).y0 as *mut libc::c_void);
        free((*state).y0_orig as *mut libc::c_void);
        free((*state).ytmp as *mut libc::c_void);
        free((*state).dfdy as *mut libc::c_void);
        free((*state).dfdt as *mut libc::c_void);
        free((*state).y_onestep as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for p\0" as *const u8 as *const libc::c_char,
            b"rk2simp.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    return state as *mut libc::c_void;
}
unsafe extern "C" fn rk2simp_step(
    mut y: *mut libc::c_double,
    mut state: *mut rk2simp_state_t,
    h: libc::c_double,
    t: libc::c_double,
    dim: size_t,
    mut sys: *const gsl_odeiv_system,
) -> libc::c_int {
    let mut y0: *const libc::c_double = (*state).y0;
    let mut Y1: *mut libc::c_double = (*state).Y1;
    let mut ytmp: *mut libc::c_double = (*state).ytmp;
    let mut i: size_t = 0;
    let mut s: libc::c_int = 0;
    let mut ps: libc::c_int = 0;
    let mut J: gsl_matrix_view = gsl_matrix_view_array((*state).dfdy, dim, dim);
    s = (Some(((*sys).jacobian).unwrap()))
        .unwrap()(t, y0, (*state).dfdy, (*state).dfdt, (*sys).params);
    if s != GSL_SUCCESS as libc::c_int {
        return s;
    }
    gsl_matrix_scale(&mut J.matrix, -h / 2.0f64);
    gsl_matrix_add_diagonal(&mut J.matrix, 1.0f64);
    s += gsl_linalg_LU_decomp(&mut J.matrix, (*state).p, &mut ps);
    if s != GSL_SUCCESS as libc::c_int {
        return GSL_EFAILED as libc::c_int;
    }
    s = (Some(((*sys).function).unwrap()))
        .unwrap()(t + 0.5f64 * h, y0, ytmp, (*sys).params);
    if s != GSL_SUCCESS as libc::c_int {
        return s;
    }
    let y0_view: gsl_vector_const_view = gsl_vector_const_view_array(y0, dim);
    let mut ytmp_view: gsl_vector_view = gsl_vector_view_array(ytmp, dim);
    let mut Y1_view: gsl_vector_view = gsl_vector_view_array(Y1, dim);
    s = gsl_linalg_LU_solve(
        &mut J.matrix,
        (*state).p,
        &mut ytmp_view.vector,
        &mut Y1_view.vector,
    );
    gsl_vector_scale(&mut Y1_view.vector, 0.5f64 * h);
    gsl_vector_add(&mut Y1_view.vector, &y0_view.vector);
    s = (Some(((*sys).function).unwrap()))
        .unwrap()(t + 0.5f64 * h, Y1 as *const libc::c_double, ytmp, (*sys).params);
    if s != GSL_SUCCESS as libc::c_int {
        return s;
    }
    i = 0 as libc::c_int as size_t;
    while i < dim {
        *y.offset(i as isize) = *y0.offset(i as isize) + h * *ytmp.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    return s;
}
unsafe extern "C" fn rk2simp_apply(
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
    let mut state: *mut rk2simp_state_t = vstate as *mut rk2simp_state_t;
    let mut i: size_t = 0;
    let mut y0: *mut libc::c_double = (*state).y0;
    let mut y0_orig: *mut libc::c_double = (*state).y0_orig;
    let mut y_onestep: *mut libc::c_double = (*state).y_onestep;
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
    memcpy(
        y_onestep as *mut libc::c_void,
        y as *const libc::c_void,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    let mut s: libc::c_int = rk2simp_step(y_onestep, state, h, t, dim, sys);
    if s != GSL_SUCCESS as libc::c_int {
        return s;
    }
    let mut s_0: libc::c_int = rk2simp_step(y, state, h / 2.0f64, t, dim, sys);
    if s_0 != GSL_SUCCESS as libc::c_int {
        memcpy(
            y as *mut libc::c_void,
            y0_orig as *const libc::c_void,
            dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
        return s_0;
    }
    memcpy(
        y0 as *mut libc::c_void,
        y as *const libc::c_void,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    let mut s_1: libc::c_int = rk2simp_step(
        y,
        state,
        h / 2.0f64,
        t + h / 2.0f64,
        dim,
        sys,
    );
    if s_1 != GSL_SUCCESS as libc::c_int {
        memcpy(
            y as *mut libc::c_void,
            y0_orig as *const libc::c_void,
            dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
        return s_1;
    }
    if !dydt_out.is_null() {
        let mut s_2: libc::c_int = (Some(((*sys).function).unwrap()))
            .unwrap()(t + h, y as *const libc::c_double, dydt_out, (*sys).params);
        if s_2 != GSL_SUCCESS as libc::c_int {
            memcpy(
                y as *mut libc::c_void,
                y0_orig as *const libc::c_void,
                dim
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    ),
            );
            return s_2;
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
unsafe extern "C" fn rk2simp_reset(
    mut vstate: *mut libc::c_void,
    mut dim: size_t,
) -> libc::c_int {
    let mut state: *mut rk2simp_state_t = vstate as *mut rk2simp_state_t;
    memset(
        (*state).Y1 as *mut libc::c_void,
        0 as libc::c_int,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memset(
        (*state).y0 as *mut libc::c_void,
        0 as libc::c_int,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memset(
        (*state).y0_orig as *mut libc::c_void,
        0 as libc::c_int,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memset(
        (*state).ytmp as *mut libc::c_void,
        0 as libc::c_int,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memset(
        (*state).dfdt as *mut libc::c_void,
        0 as libc::c_int,
        dim
            .wrapping_mul(dim)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memset(
        (*state).dfdt as *mut libc::c_void,
        0 as libc::c_int,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memset(
        (*state).y_onestep as *mut libc::c_void,
        0 as libc::c_int,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn rk2simp_order(mut vstate: *mut libc::c_void) -> libc::c_uint {
    let mut state: *mut rk2simp_state_t = vstate as *mut rk2simp_state_t;
    state = 0 as *mut rk2simp_state_t;
    return 2 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn rk2simp_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut rk2simp_state_t = vstate as *mut rk2simp_state_t;
    free((*state).Y1 as *mut libc::c_void);
    free((*state).y0 as *mut libc::c_void);
    free((*state).y0_orig as *mut libc::c_void);
    free((*state).ytmp as *mut libc::c_void);
    free((*state).dfdy as *mut libc::c_void);
    free((*state).dfdt as *mut libc::c_void);
    free((*state).y_onestep as *mut libc::c_void);
    gsl_permutation_free((*state).p);
    free(state as *mut libc::c_void);
}
static mut rk2simp_type: gsl_odeiv_step_type = {
    let mut init = gsl_odeiv_step_type {
        name: b"rk2simp\0" as *const u8 as *const libc::c_char,
        can_use_dydt_in: 0 as libc::c_int,
        gives_exact_dydt_out: 1 as libc::c_int,
        alloc: Some(rk2simp_alloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
        apply: Some(
            rk2simp_apply
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
            rk2simp_reset
                as unsafe extern "C" fn(*mut libc::c_void, size_t) -> libc::c_int,
        ),
        order: Some(
            rk2simp_order as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint,
        ),
        free: Some(rk2simp_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    };
    init
};
pub static mut gsl_odeiv_step_rk2simp: *const gsl_odeiv_step_type = unsafe {
    &rk2simp_type as *const gsl_odeiv_step_type
};
