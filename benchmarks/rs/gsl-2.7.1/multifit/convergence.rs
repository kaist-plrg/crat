use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_blas_dnrm2(X: *const gsl_vector) -> libc::c_double;
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
pub struct gsl_matrix {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_function_fdf_struct {
    pub f: Option::<
        unsafe extern "C" fn(
            *const gsl_vector,
            *mut libc::c_void,
            *mut gsl_vector,
        ) -> libc::c_int,
    >,
    pub df: Option::<
        unsafe extern "C" fn(
            *const gsl_vector,
            *mut libc::c_void,
            *mut gsl_matrix,
        ) -> libc::c_int,
    >,
    pub fdf: Option::<
        unsafe extern "C" fn(
            *const gsl_vector,
            *mut libc::c_void,
            *mut gsl_vector,
            *mut gsl_matrix,
        ) -> libc::c_int,
    >,
    pub n: size_t,
    pub p: size_t,
    pub params: *mut libc::c_void,
    pub nevalf: size_t,
    pub nevaldf: size_t,
}
pub type gsl_multifit_function_fdf = gsl_multifit_function_fdf_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_fdfsolver_type {
    pub name: *const libc::c_char,
    pub size: size_t,
    pub alloc: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t, size_t) -> libc::c_int,
    >,
    pub set: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const gsl_vector,
            *mut gsl_multifit_function_fdf,
            *mut gsl_vector,
            *mut gsl_vector,
            *mut gsl_vector,
        ) -> libc::c_int,
    >,
    pub iterate: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const gsl_vector,
            *mut gsl_multifit_function_fdf,
            *mut gsl_vector,
            *mut gsl_vector,
            *mut gsl_vector,
        ) -> libc::c_int,
    >,
    pub gradient: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut gsl_vector) -> libc::c_int,
    >,
    pub jac: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut gsl_matrix) -> libc::c_int,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_fdfsolver {
    pub type_0: *const gsl_multifit_fdfsolver_type,
    pub fdf: *mut gsl_multifit_function_fdf,
    pub x: *mut gsl_vector,
    pub f: *mut gsl_vector,
    pub dx: *mut gsl_vector,
    pub g: *mut gsl_vector,
    pub sqrt_wts: *mut gsl_vector,
    pub niter: size_t,
    pub state: *mut libc::c_void,
}
#[inline]
unsafe extern "C" fn gsl_vector_get(
    mut v: *const gsl_vector,
    i: size_t,
) -> libc::c_double {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
pub unsafe extern "C" fn gsl_multifit_fdfsolver_test(
    mut s: *const gsl_multifit_fdfsolver,
    xtol: libc::c_double,
    gtol: libc::c_double,
    ftol: libc::c_double,
    mut info: *mut libc::c_int,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut gnorm: libc::c_double = 0.;
    let mut fnorm: libc::c_double = 0.;
    let mut phi: libc::c_double = 0.;
    *info = 0 as libc::c_int;
    status = gsl_multifit_test_delta((*s).dx, (*s).x, xtol * xtol, xtol);
    if status == GSL_SUCCESS as libc::c_int {
        *info = 1 as libc::c_int;
        return GSL_SUCCESS as libc::c_int;
    }
    ((*(*s).type_0).gradient).unwrap()((*s).state, (*s).g);
    gnorm = scaled_infnorm((*s).x, (*s).g);
    fnorm = gsl_blas_dnrm2((*s).f);
    phi = 0.5f64 * fnorm * fnorm;
    if gnorm <= gtol * (if phi > 1.0f64 { phi } else { 1.0f64 }) {
        *info = 2 as libc::c_int;
        return GSL_SUCCESS as libc::c_int;
    }
    return GSL_CONTINUE as libc::c_int;
}
pub unsafe extern "C" fn gsl_multifit_test_delta(
    mut dx: *const gsl_vector,
    mut x: *const gsl_vector,
    mut epsabs: libc::c_double,
    mut epsrel: libc::c_double,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut ok: libc::c_int = 1 as libc::c_int;
    let n: size_t = (*x).size;
    if epsrel < 0.0f64 {
        gsl_error(
            b"relative tolerance is negative\0" as *const u8 as *const libc::c_char,
            b"convergence.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            GSL_EBADTOL as libc::c_int,
        );
        return GSL_EBADTOL as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_double = gsl_vector_get(x, i);
        let mut dxi: libc::c_double = gsl_vector_get(dx, i);
        let mut tolerance: libc::c_double = epsabs + epsrel * fabs(xi);
        if fabs(dxi) < tolerance {
            ok = 1 as libc::c_int;
            i = i.wrapping_add(1);
            i;
        } else {
            ok = 0 as libc::c_int;
            break;
        }
    }
    if ok != 0 {
        return GSL_SUCCESS as libc::c_int;
    }
    return GSL_CONTINUE as libc::c_int;
}
pub unsafe extern "C" fn gsl_multifit_test_gradient(
    mut g: *const gsl_vector,
    mut epsabs: libc::c_double,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut residual: libc::c_double = 0 as libc::c_int as libc::c_double;
    let n: size_t = (*g).size;
    if epsabs < 0.0f64 {
        gsl_error(
            b"absolute tolerance is negative\0" as *const u8 as *const libc::c_char,
            b"convergence.c\0" as *const u8 as *const libc::c_char,
            137 as libc::c_int,
            GSL_EBADTOL as libc::c_int,
        );
        return GSL_EBADTOL as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut gi: libc::c_double = gsl_vector_get(g, i);
        residual += fabs(gi);
        i = i.wrapping_add(1);
        i;
    }
    if residual < epsabs {
        return GSL_SUCCESS as libc::c_int;
    }
    return GSL_CONTINUE as libc::c_int;
}
unsafe extern "C" fn scaled_infnorm(
    mut x: *const gsl_vector,
    mut g: *const gsl_vector,
) -> libc::c_double {
    let n: size_t = (*x).size;
    let mut i: size_t = 0;
    let mut norm: libc::c_double = 0.0f64;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_double = if gsl_vector_get(x, i) > 1.0f64 {
            gsl_vector_get(x, i)
        } else {
            1.0f64
        };
        let mut gi: libc::c_double = gsl_vector_get(g, i);
        let mut tmp: libc::c_double = fabs(xi * gi);
        if tmp > norm {
            norm = tmp;
        }
        i = i.wrapping_add(1);
        i;
    }
    return norm;
}
