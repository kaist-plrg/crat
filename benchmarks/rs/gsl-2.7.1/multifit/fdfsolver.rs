use ::libc;
extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_alloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_calloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_vector_set_all(v: *mut gsl_vector, x: libc::c_double);
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_mul(a: *mut gsl_vector, b: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_scale(a: *mut gsl_vector, x: libc::c_double) -> libc::c_int;
    fn gsl_matrix_row(m: *mut gsl_matrix, i: size_t) -> _gsl_vector_view;
    fn gsl_multifit_fdfsolver_test(
        s: *const gsl_multifit_fdfsolver,
        xtol: libc::c_double,
        gtol: libc::c_double,
        ftol: libc::c_double,
        info: *mut libc::c_int,
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
pub struct _gsl_vector_view {
    pub vector: gsl_vector,
}
pub type gsl_vector_view = _gsl_vector_view;
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
#[inline]
unsafe extern "C" fn gsl_vector_set(
    mut v: *mut gsl_vector,
    i: size_t,
    mut x: libc::c_double,
) {
    *((*v).data).offset(i.wrapping_mul((*v).stride) as isize) = x;
}
pub unsafe extern "C" fn gsl_multifit_fdfsolver_alloc(
    mut T: *const gsl_multifit_fdfsolver_type,
    mut n: size_t,
    mut p: size_t,
) -> *mut gsl_multifit_fdfsolver {
    let mut status: libc::c_int = 0;
    let mut s: *mut gsl_multifit_fdfsolver = 0 as *mut gsl_multifit_fdfsolver;
    if n < p {
        gsl_error(
            b"insufficient data points, n < p\0" as *const u8 as *const libc::c_char,
            b"fdfsolver.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_multifit_fdfsolver;
    }
    s = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gsl_multifit_fdfsolver>() as libc::c_ulong,
    ) as *mut gsl_multifit_fdfsolver;
    if s.is_null() {
        gsl_error(
            b"failed to allocate space for multifit solver struct\0" as *const u8
                as *const libc::c_char,
            b"fdfsolver.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_fdfsolver;
    }
    (*s).x = gsl_vector_calloc(p);
    if ((*s).x).is_null() {
        gsl_multifit_fdfsolver_free(s);
        gsl_error(
            b"failed to allocate space for x\0" as *const u8 as *const libc::c_char,
            b"fdfsolver.c\0" as *const u8 as *const libc::c_char,
            52 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_fdfsolver;
    }
    (*s).f = gsl_vector_calloc(n);
    if ((*s).f).is_null() {
        gsl_multifit_fdfsolver_free(s);
        gsl_error(
            b"failed to allocate space for f\0" as *const u8 as *const libc::c_char,
            b"fdfsolver.c\0" as *const u8 as *const libc::c_char,
            60 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_fdfsolver;
    }
    (*s).dx = gsl_vector_calloc(p);
    if ((*s).dx).is_null() {
        gsl_multifit_fdfsolver_free(s);
        gsl_error(
            b"failed to allocate space for dx\0" as *const u8 as *const libc::c_char,
            b"fdfsolver.c\0" as *const u8 as *const libc::c_char,
            68 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_fdfsolver;
    }
    (*s).g = gsl_vector_alloc(p);
    if ((*s).g).is_null() {
        gsl_multifit_fdfsolver_free(s);
        gsl_error(
            b"failed to allocate space for g\0" as *const u8 as *const libc::c_char,
            b"fdfsolver.c\0" as *const u8 as *const libc::c_char,
            76 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_fdfsolver;
    }
    (*s).sqrt_wts = gsl_vector_calloc(n);
    if ((*s).sqrt_wts).is_null() {
        gsl_multifit_fdfsolver_free(s);
        gsl_error(
            b"failed to allocate space for sqrt_wts\0" as *const u8
                as *const libc::c_char,
            b"fdfsolver.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_fdfsolver;
    }
    (*s).state = calloc(1 as libc::c_int as libc::c_ulong, (*T).size);
    if ((*s).state).is_null() {
        gsl_multifit_fdfsolver_free(s);
        gsl_error(
            b"failed to allocate space for multifit solver state\0" as *const u8
                as *const libc::c_char,
            b"fdfsolver.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_fdfsolver;
    }
    (*s).type_0 = T;
    status = ((*(*s).type_0).alloc).unwrap()((*s).state, n, p);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_multifit_fdfsolver_free(s);
        gsl_error(
            b"failed to set solver\0" as *const u8 as *const libc::c_char,
            b"fdfsolver.c\0" as *const u8 as *const libc::c_char,
            103 as libc::c_int,
            status,
        );
        return 0 as *mut gsl_multifit_fdfsolver;
    }
    (*s).fdf = 0 as *mut gsl_multifit_function_fdf;
    (*s).niter = 0 as libc::c_int as size_t;
    return s;
}
pub unsafe extern "C" fn gsl_multifit_fdfsolver_set(
    mut s: *mut gsl_multifit_fdfsolver,
    mut f: *mut gsl_multifit_function_fdf,
    mut x: *const gsl_vector,
) -> libc::c_int {
    return gsl_multifit_fdfsolver_wset(s, f, x, 0 as *const gsl_vector);
}
pub unsafe extern "C" fn gsl_multifit_fdfsolver_wset(
    mut s: *mut gsl_multifit_fdfsolver,
    mut f: *mut gsl_multifit_function_fdf,
    mut x: *const gsl_vector,
    mut wts: *const gsl_vector,
) -> libc::c_int {
    let n: size_t = (*(*s).f).size;
    if n != (*f).n {
        gsl_error(
            b"function size does not match solver\0" as *const u8 as *const libc::c_char,
            b"fdfsolver.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*(*s).x).size != (*x).size {
        gsl_error(
            b"vector length does not match solver\0" as *const u8 as *const libc::c_char,
            b"fdfsolver.c\0" as *const u8 as *const libc::c_char,
            135 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if !wts.is_null() && n != (*wts).size {
        gsl_error(
            b"weight vector length does not match solver\0" as *const u8
                as *const libc::c_char,
            b"fdfsolver.c\0" as *const u8 as *const libc::c_char,
            139 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        (*s).fdf = f;
        gsl_vector_memcpy((*s).x, x);
        (*s).niter = 0 as libc::c_int as size_t;
        if !wts.is_null() {
            i = 0 as libc::c_int as size_t;
            while i < n {
                let mut wi: libc::c_double = gsl_vector_get(wts, i);
                gsl_vector_set((*s).sqrt_wts, i, sqrt(wi));
                i = i.wrapping_add(1);
                i;
            }
        } else {
            gsl_vector_set_all((*s).sqrt_wts, 1.0f64);
        }
        return ((*(*s).type_0).set)
            .unwrap()((*s).state, (*s).sqrt_wts, (*s).fdf, (*s).x, (*s).f, (*s).dx);
    };
}
pub unsafe extern "C" fn gsl_multifit_fdfsolver_iterate(
    mut s: *mut gsl_multifit_fdfsolver,
) -> libc::c_int {
    let mut status: libc::c_int = ((*(*s).type_0).iterate)
        .unwrap()((*s).state, (*s).sqrt_wts, (*s).fdf, (*s).x, (*s).f, (*s).dx);
    (*s).niter = ((*s).niter).wrapping_add(1);
    (*s).niter;
    return status;
}
pub unsafe extern "C" fn gsl_multifit_fdfsolver_driver(
    mut s: *mut gsl_multifit_fdfsolver,
    maxiter: size_t,
    xtol: libc::c_double,
    gtol: libc::c_double,
    ftol: libc::c_double,
    mut info: *mut libc::c_int,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut iter: size_t = 0 as libc::c_int as size_t;
    loop {
        status = gsl_multifit_fdfsolver_iterate(s);
        if status != GSL_SUCCESS as libc::c_int && status != GSL_ENOPROG as libc::c_int {
            break;
        }
        status = gsl_multifit_fdfsolver_test(s, xtol, gtol, ftol, info);
        if !(status == GSL_CONTINUE as libc::c_int
            && {
                iter = iter.wrapping_add(1);
                iter < maxiter
            })
        {
            break;
        }
    }
    if status == GSL_ETOLF as libc::c_int || status == GSL_ETOLX as libc::c_int
        || status == GSL_ETOLG as libc::c_int
    {
        *info = status;
        status = GSL_SUCCESS as libc::c_int;
    }
    if iter >= maxiter && status != GSL_SUCCESS as libc::c_int {
        status = GSL_EMAXITER as libc::c_int;
    }
    return status;
}
pub unsafe extern "C" fn gsl_multifit_fdfsolver_jac(
    mut s: *mut gsl_multifit_fdfsolver,
    mut J: *mut gsl_matrix,
) -> libc::c_int {
    let n: size_t = (*(*s).f).size;
    let p: size_t = (*(*s).x).size;
    if n != (*J).size1 || p != (*J).size2 {
        gsl_error(
            b"Jacobian dimensions do not match solver\0" as *const u8
                as *const libc::c_char,
            b"fdfsolver.c\0" as *const u8 as *const libc::c_char,
            251 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        return ((*(*s).type_0).jac).unwrap()((*s).state, J)
    };
}
pub unsafe extern "C" fn gsl_multifit_fdfsolver_free(
    mut s: *mut gsl_multifit_fdfsolver,
) {
    if s.is_null() {
        return;
    }
    if !((*s).state).is_null() {
        ((*(*s).type_0).free).unwrap()((*s).state);
        free((*s).state);
    }
    if !((*s).dx).is_null() {
        gsl_vector_free((*s).dx);
    }
    if !((*s).x).is_null() {
        gsl_vector_free((*s).x);
    }
    if !((*s).f).is_null() {
        gsl_vector_free((*s).f);
    }
    if !((*s).sqrt_wts).is_null() {
        gsl_vector_free((*s).sqrt_wts);
    }
    if !((*s).g).is_null() {
        gsl_vector_free((*s).g);
    }
    free(s as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_multifit_fdfsolver_name(
    mut s: *const gsl_multifit_fdfsolver,
) -> *const libc::c_char {
    return (*(*s).type_0).name;
}
pub unsafe extern "C" fn gsl_multifit_fdfsolver_position(
    mut s: *const gsl_multifit_fdfsolver,
) -> *mut gsl_vector {
    return (*s).x;
}
pub unsafe extern "C" fn gsl_multifit_fdfsolver_residual(
    mut s: *const gsl_multifit_fdfsolver,
) -> *mut gsl_vector {
    return (*s).f;
}
pub unsafe extern "C" fn gsl_multifit_fdfsolver_niter(
    mut s: *const gsl_multifit_fdfsolver,
) -> size_t {
    return (*s).niter;
}
pub unsafe extern "C" fn gsl_multifit_eval_wf(
    mut fdf: *mut gsl_multifit_function_fdf,
    mut x: *const gsl_vector,
    mut swts: *const gsl_vector,
    mut y: *mut gsl_vector,
) -> libc::c_int {
    let mut s: libc::c_int = (Some(((*fdf).f).unwrap())).unwrap()(x, (*fdf).params, y);
    (*fdf).nevalf = ((*fdf).nevalf).wrapping_add(1);
    (*fdf).nevalf;
    if !swts.is_null() {
        gsl_vector_mul(y, swts);
    }
    return s;
}
pub unsafe extern "C" fn gsl_multifit_eval_wdf(
    mut fdf: *mut gsl_multifit_function_fdf,
    mut x: *const gsl_vector,
    mut swts: *const gsl_vector,
    mut dy: *mut gsl_matrix,
) -> libc::c_int {
    let mut status: libc::c_int = (Some(((*fdf).df).unwrap()))
        .unwrap()(x, (*fdf).params, dy);
    (*fdf).nevaldf = ((*fdf).nevaldf).wrapping_add(1);
    (*fdf).nevaldf;
    if !swts.is_null() {
        let n: size_t = (*swts).size;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < n {
            let mut swi: libc::c_double = gsl_vector_get(swts, i);
            let mut v: gsl_vector_view = gsl_matrix_row(dy, i);
            gsl_vector_scale(&mut v.vector, swi);
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
