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
pub type gsl_multifit_nlinear_fdtype = libc::c_uint;
pub const GSL_MULTIFIT_NLINEAR_CTRDIFF: gsl_multifit_nlinear_fdtype = 1;
pub const GSL_MULTIFIT_NLINEAR_FWDIFF: gsl_multifit_nlinear_fdtype = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_nlinear_fdf {
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
    pub fvv: Option::<
        unsafe extern "C" fn(
            *const gsl_vector,
            *const gsl_vector,
            *mut libc::c_void,
            *mut gsl_vector,
        ) -> libc::c_int,
    >,
    pub n: size_t,
    pub p: size_t,
    pub params: *mut libc::c_void,
    pub nevalf: size_t,
    pub nevaldf: size_t,
    pub nevalfvv: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_nlinear_trs {
    pub name: *const libc::c_char,
    pub alloc: Option::<
        unsafe extern "C" fn(*const libc::c_void, size_t, size_t) -> *mut libc::c_void,
    >,
    pub init: Option::<
        unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> libc::c_int,
    >,
    pub preloop: Option::<
        unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> libc::c_int,
    >,
    pub step: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            libc::c_double,
            *mut gsl_vector,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub preduction: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            *const gsl_vector,
            *mut libc::c_double,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_nlinear_scale {
    pub name: *const libc::c_char,
    pub init: Option::<
        unsafe extern "C" fn(*const gsl_matrix, *mut gsl_vector) -> libc::c_int,
    >,
    pub update: Option::<
        unsafe extern "C" fn(*const gsl_matrix, *mut gsl_vector) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_nlinear_solver {
    pub name: *const libc::c_char,
    pub alloc: Option::<unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void>,
    pub init: Option::<
        unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> libc::c_int,
    >,
    pub presolve: Option::<
        unsafe extern "C" fn(
            libc::c_double,
            *const libc::c_void,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub solve: Option::<
        unsafe extern "C" fn(
            *const gsl_vector,
            *mut gsl_vector,
            *const libc::c_void,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub rcond: Option::<
        unsafe extern "C" fn(*mut libc::c_double, *mut libc::c_void) -> libc::c_int,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_nlinear_parameters {
    pub trs: *const gsl_multifit_nlinear_trs,
    pub scale: *const gsl_multifit_nlinear_scale,
    pub solver: *const gsl_multifit_nlinear_solver,
    pub fdtype: gsl_multifit_nlinear_fdtype,
    pub factor_up: libc::c_double,
    pub factor_down: libc::c_double,
    pub avmax: libc::c_double,
    pub h_df: libc::c_double,
    pub h_fvv: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_nlinear_type {
    pub name: *const libc::c_char,
    pub alloc: Option::<
        unsafe extern "C" fn(
            *const gsl_multifit_nlinear_parameters,
            size_t,
            size_t,
        ) -> *mut libc::c_void,
    >,
    pub init: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const gsl_vector,
            *mut gsl_multifit_nlinear_fdf,
            *const gsl_vector,
            *mut gsl_vector,
            *mut gsl_matrix,
            *mut gsl_vector,
        ) -> libc::c_int,
    >,
    pub iterate: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const gsl_vector,
            *mut gsl_multifit_nlinear_fdf,
            *mut gsl_vector,
            *mut gsl_vector,
            *mut gsl_matrix,
            *mut gsl_vector,
            *mut gsl_vector,
        ) -> libc::c_int,
    >,
    pub rcond: Option::<
        unsafe extern "C" fn(*mut libc::c_double, *mut libc::c_void) -> libc::c_int,
    >,
    pub avratio: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double>,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_nlinear_workspace {
    pub type_0: *const gsl_multifit_nlinear_type,
    pub fdf: *mut gsl_multifit_nlinear_fdf,
    pub x: *mut gsl_vector,
    pub f: *mut gsl_vector,
    pub dx: *mut gsl_vector,
    pub g: *mut gsl_vector,
    pub J: *mut gsl_matrix,
    pub sqrt_wts_work: *mut gsl_vector,
    pub sqrt_wts: *mut gsl_vector,
    pub niter: size_t,
    pub params: gsl_multifit_nlinear_parameters,
    pub state: *mut libc::c_void,
}
#[inline]
unsafe extern "C" fn gsl_vector_get(
    mut v: *const gsl_vector,
    i: size_t,
) -> libc::c_double {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
pub unsafe extern "C" fn gsl_multifit_nlinear_test(
    xtol: libc::c_double,
    gtol: libc::c_double,
    ftol: libc::c_double,
    mut info: *mut libc::c_int,
    mut w: *const gsl_multifit_nlinear_workspace,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut gnorm: libc::c_double = 0.;
    let mut fnorm: libc::c_double = 0.;
    let mut phi: libc::c_double = 0.;
    *info = 0 as libc::c_int;
    status = test_delta((*w).dx, (*w).x, xtol * xtol, xtol);
    if status == GSL_SUCCESS as libc::c_int {
        *info = 1 as libc::c_int;
        return GSL_SUCCESS as libc::c_int;
    }
    gnorm = scaled_infnorm((*w).x, (*w).g);
    fnorm = gsl_blas_dnrm2((*w).f);
    phi = 0.5f64 * fnorm * fnorm;
    if gnorm <= gtol * (if phi > 1.0f64 { phi } else { 1.0f64 }) {
        *info = 2 as libc::c_int;
        return GSL_SUCCESS as libc::c_int;
    }
    return GSL_CONTINUE as libc::c_int;
}
unsafe extern "C" fn test_delta(
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
            102 as libc::c_int,
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
