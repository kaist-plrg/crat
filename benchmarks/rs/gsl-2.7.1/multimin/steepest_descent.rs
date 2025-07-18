use ::libc;
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_alloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_vector_set_zero(v: *mut gsl_vector);
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_equal(u: *const gsl_vector, v: *const gsl_vector) -> libc::c_int;
    fn gsl_blas_dnrm2(X: *const gsl_vector) -> libc::c_double;
    fn gsl_blas_daxpy(
        alpha: libc::c_double,
        X: *const gsl_vector,
        Y: *mut gsl_vector,
    ) -> libc::c_int;
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
pub struct gsl_block_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
}
pub type gsl_block = gsl_block_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multimin_function_fdf_struct {
    pub f: Option::<
        unsafe extern "C" fn(*const gsl_vector, *mut libc::c_void) -> libc::c_double,
    >,
    pub df: Option::<
        unsafe extern "C" fn(*const gsl_vector, *mut libc::c_void, *mut gsl_vector) -> (),
    >,
    pub fdf: Option::<
        unsafe extern "C" fn(
            *const gsl_vector,
            *mut libc::c_void,
            *mut libc::c_double,
            *mut gsl_vector,
        ) -> (),
    >,
    pub n: size_t,
    pub params: *mut libc::c_void,
}
pub type gsl_multimin_function_fdf = gsl_multimin_function_fdf_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multimin_fdfminimizer_type {
    pub name: *const libc::c_char,
    pub size: size_t,
    pub alloc: Option::<unsafe extern "C" fn(*mut libc::c_void, size_t) -> libc::c_int>,
    pub set: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut gsl_multimin_function_fdf,
            *const gsl_vector,
            *mut libc::c_double,
            *mut gsl_vector,
            libc::c_double,
            libc::c_double,
        ) -> libc::c_int,
    >,
    pub iterate: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut gsl_multimin_function_fdf,
            *mut gsl_vector,
            *mut libc::c_double,
            *mut gsl_vector,
            *mut gsl_vector,
        ) -> libc::c_int,
    >,
    pub restart: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct steepest_descent_state_t {
    pub step: libc::c_double,
    pub max_step: libc::c_double,
    pub tol: libc::c_double,
    pub x1: *mut gsl_vector,
    pub g1: *mut gsl_vector,
}
unsafe extern "C" fn steepest_descent_alloc(
    mut vstate: *mut libc::c_void,
    mut n: size_t,
) -> libc::c_int {
    let mut state: *mut steepest_descent_state_t = vstate
        as *mut steepest_descent_state_t;
    (*state).x1 = gsl_vector_alloc(n);
    if ((*state).x1).is_null() {
        gsl_error(
            b"failed to allocate space for x1\0" as *const u8 as *const libc::c_char,
            b"steepest_descent.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).g1 = gsl_vector_alloc(n);
    if ((*state).g1).is_null() {
        gsl_vector_free((*state).x1);
        gsl_error(
            b"failed to allocate space for g1\0" as *const u8 as *const libc::c_char,
            b"steepest_descent.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn steepest_descent_set(
    mut vstate: *mut libc::c_void,
    mut fdf: *mut gsl_multimin_function_fdf,
    mut x: *const gsl_vector,
    mut f: *mut libc::c_double,
    mut gradient: *mut gsl_vector,
    mut step_size: libc::c_double,
    mut tol: libc::c_double,
) -> libc::c_int {
    let mut state: *mut steepest_descent_state_t = vstate
        as *mut steepest_descent_state_t;
    (Some(((*fdf).fdf).unwrap())).unwrap()(x, (*fdf).params, f, gradient);
    (*state).step = step_size;
    (*state).max_step = step_size;
    (*state).tol = tol;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn steepest_descent_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut steepest_descent_state_t = vstate
        as *mut steepest_descent_state_t;
    gsl_vector_free((*state).x1);
    gsl_vector_free((*state).g1);
}
unsafe extern "C" fn steepest_descent_restart(
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut state: *mut steepest_descent_state_t = vstate
        as *mut steepest_descent_state_t;
    (*state).step = (*state).max_step;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn steepest_descent_iterate(
    mut vstate: *mut libc::c_void,
    mut fdf: *mut gsl_multimin_function_fdf,
    mut x: *mut gsl_vector,
    mut f: *mut libc::c_double,
    mut gradient: *mut gsl_vector,
    mut dx: *mut gsl_vector,
) -> libc::c_int {
    let mut state: *mut steepest_descent_state_t = vstate
        as *mut steepest_descent_state_t;
    let mut x1: *mut gsl_vector = (*state).x1;
    let mut g1: *mut gsl_vector = (*state).g1;
    let mut f0: libc::c_double = *f;
    let mut f1: libc::c_double = 0.;
    let mut step: libc::c_double = (*state).step;
    let mut tol: libc::c_double = (*state).tol;
    let mut failed: libc::c_int = 0 as libc::c_int;
    let mut gnorm: libc::c_double = gsl_blas_dnrm2(gradient);
    if gnorm == 0.0f64 {
        gsl_vector_set_zero(dx);
        return GSL_ENOPROG as libc::c_int;
    }
    loop {
        gsl_vector_set_zero(dx);
        gsl_blas_daxpy(-step / gnorm, gradient, dx);
        gsl_vector_memcpy(x1, x);
        gsl_blas_daxpy(1.0f64, dx, x1);
        if gsl_vector_equal(x, x1) != 0 {
            return GSL_ENOPROG as libc::c_int;
        }
        (Some(((*fdf).fdf).unwrap())).unwrap()(x1, (*fdf).params, &mut f1, g1);
        if !(f1 > f0) {
            break;
        }
        failed = 1 as libc::c_int;
        step *= tol;
    }
    if failed != 0 {
        step *= tol;
    } else {
        step *= 2.0f64;
    }
    (*state).step = step;
    gsl_vector_memcpy(x, x1);
    gsl_vector_memcpy(gradient, g1);
    *f = f1;
    return GSL_SUCCESS as libc::c_int;
}
static mut steepest_descent_type: gsl_multimin_fdfminimizer_type = {
    let mut init = gsl_multimin_fdfminimizer_type {
        name: b"steepest_descent\0" as *const u8 as *const libc::c_char,
        size: ::std::mem::size_of::<steepest_descent_state_t>() as libc::c_ulong,
        alloc: Some(
            steepest_descent_alloc
                as unsafe extern "C" fn(*mut libc::c_void, size_t) -> libc::c_int,
        ),
        set: Some(
            steepest_descent_set
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut gsl_multimin_function_fdf,
                    *const gsl_vector,
                    *mut libc::c_double,
                    *mut gsl_vector,
                    libc::c_double,
                    libc::c_double,
                ) -> libc::c_int,
        ),
        iterate: Some(
            steepest_descent_iterate
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut gsl_multimin_function_fdf,
                    *mut gsl_vector,
                    *mut libc::c_double,
                    *mut gsl_vector,
                    *mut gsl_vector,
                ) -> libc::c_int,
        ),
        restart: Some(
            steepest_descent_restart
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
        ),
        free: Some(
            steepest_descent_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
        ),
    };
    init
};
pub static mut gsl_multimin_fdfminimizer_steepest_descent: *const gsl_multimin_fdfminimizer_type = unsafe {
    &steepest_descent_type as *const gsl_multimin_fdfminimizer_type
};
