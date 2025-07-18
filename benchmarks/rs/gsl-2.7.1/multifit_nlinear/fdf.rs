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
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_mul(a: *mut gsl_vector, b: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_scale(a: *mut gsl_vector, x: libc::c_double) -> libc::c_int;
    fn gsl_matrix_alloc(n1: size_t, n2: size_t) -> *mut gsl_matrix;
    fn gsl_matrix_free(m: *mut gsl_matrix);
    fn gsl_matrix_row(m: *mut gsl_matrix, i: size_t) -> _gsl_vector_view;
    static mut gsl_multifit_nlinear_solver_qr: *const gsl_multifit_nlinear_solver;
    static mut gsl_multifit_nlinear_scale_more: *const gsl_multifit_nlinear_scale;
    static mut gsl_multifit_nlinear_trs_lm: *const gsl_multifit_nlinear_trs;
    fn gsl_multifit_nlinear_test(
        xtol: libc::c_double,
        gtol: libc::c_double,
        ftol: libc::c_double,
        info: *mut libc::c_int,
        w: *const gsl_multifit_nlinear_workspace,
    ) -> libc::c_int;
    fn gsl_multifit_nlinear_fdfvv(
        h: libc::c_double,
        x: *const gsl_vector,
        v: *const gsl_vector,
        f: *const gsl_vector,
        J: *const gsl_matrix,
        swts: *const gsl_vector,
        fdf: *mut gsl_multifit_nlinear_fdf,
        fvv: *mut gsl_vector,
        work: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_multifit_nlinear_df(
        h: libc::c_double,
        fdtype: gsl_multifit_nlinear_fdtype,
        x: *const gsl_vector,
        wts: *const gsl_vector,
        fdf: *mut gsl_multifit_nlinear_fdf,
        f: *const gsl_vector,
        J: *mut gsl_matrix,
        work: *mut gsl_vector,
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
#[inline]
unsafe extern "C" fn gsl_vector_set(
    mut v: *mut gsl_vector,
    i: size_t,
    mut x: libc::c_double,
) {
    *((*v).data).offset(i.wrapping_mul((*v).stride) as isize) = x;
}
pub unsafe extern "C" fn gsl_multifit_nlinear_alloc(
    mut T: *const gsl_multifit_nlinear_type,
    mut params: *const gsl_multifit_nlinear_parameters,
    n: size_t,
    p: size_t,
) -> *mut gsl_multifit_nlinear_workspace {
    let mut w: *mut gsl_multifit_nlinear_workspace = 0
        as *mut gsl_multifit_nlinear_workspace;
    if n < p {
        gsl_error(
            b"insufficient data points, n < p\0" as *const u8 as *const libc::c_char,
            b"fdf.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_multifit_nlinear_workspace;
    }
    w = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gsl_multifit_nlinear_workspace>() as libc::c_ulong,
    ) as *mut gsl_multifit_nlinear_workspace;
    if w.is_null() {
        gsl_error(
            b"failed to allocate space for multifit workspace\0" as *const u8
                as *const libc::c_char,
            b"fdf.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_nlinear_workspace;
    }
    (*w).x = gsl_vector_calloc(p);
    if ((*w).x).is_null() {
        gsl_multifit_nlinear_free(w);
        gsl_error(
            b"failed to allocate space for x\0" as *const u8 as *const libc::c_char,
            b"fdf.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_nlinear_workspace;
    }
    (*w).f = gsl_vector_calloc(n);
    if ((*w).f).is_null() {
        gsl_multifit_nlinear_free(w);
        gsl_error(
            b"failed to allocate space for f\0" as *const u8 as *const libc::c_char,
            b"fdf.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_nlinear_workspace;
    }
    (*w).dx = gsl_vector_calloc(p);
    if ((*w).dx).is_null() {
        gsl_multifit_nlinear_free(w);
        gsl_error(
            b"failed to allocate space for dx\0" as *const u8 as *const libc::c_char,
            b"fdf.c\0" as *const u8 as *const libc::c_char,
            65 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_nlinear_workspace;
    }
    (*w).g = gsl_vector_alloc(p);
    if ((*w).g).is_null() {
        gsl_multifit_nlinear_free(w);
        gsl_error(
            b"failed to allocate space for g\0" as *const u8 as *const libc::c_char,
            b"fdf.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_nlinear_workspace;
    }
    (*w).J = gsl_matrix_alloc(n, p);
    if ((*w).J).is_null() {
        gsl_multifit_nlinear_free(w);
        gsl_error(
            b"failed to allocate space for Jacobian\0" as *const u8
                as *const libc::c_char,
            b"fdf.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_nlinear_workspace;
    }
    (*w).sqrt_wts_work = gsl_vector_calloc(n);
    if ((*w).sqrt_wts_work).is_null() {
        gsl_multifit_nlinear_free(w);
        gsl_error(
            b"failed to allocate space for weights\0" as *const u8
                as *const libc::c_char,
            b"fdf.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_nlinear_workspace;
    }
    (*w).state = ((*T).alloc).unwrap()(params, n, p);
    if ((*w).state).is_null() {
        gsl_multifit_nlinear_free(w);
        gsl_error(
            b"failed to allocate space for multifit state\0" as *const u8
                as *const libc::c_char,
            b"fdf.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_nlinear_workspace;
    }
    (*w).type_0 = T;
    (*w).fdf = 0 as *mut gsl_multifit_nlinear_fdf;
    (*w).niter = 0 as libc::c_int as size_t;
    (*w).params = *params;
    return w;
}
pub unsafe extern "C" fn gsl_multifit_nlinear_free(
    mut w: *mut gsl_multifit_nlinear_workspace,
) {
    if w.is_null() {
        return;
    }
    if !((*w).state).is_null() {
        ((*(*w).type_0).free).unwrap()((*w).state);
    }
    if !((*w).dx).is_null() {
        gsl_vector_free((*w).dx);
    }
    if !((*w).x).is_null() {
        gsl_vector_free((*w).x);
    }
    if !((*w).f).is_null() {
        gsl_vector_free((*w).f);
    }
    if !((*w).sqrt_wts_work).is_null() {
        gsl_vector_free((*w).sqrt_wts_work);
    }
    if !((*w).g).is_null() {
        gsl_vector_free((*w).g);
    }
    if !((*w).J).is_null() {
        gsl_matrix_free((*w).J);
    }
    free(w as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_multifit_nlinear_default_parameters() -> gsl_multifit_nlinear_parameters {
    let mut params: gsl_multifit_nlinear_parameters = gsl_multifit_nlinear_parameters {
        trs: 0 as *const gsl_multifit_nlinear_trs,
        scale: 0 as *const gsl_multifit_nlinear_scale,
        solver: 0 as *const gsl_multifit_nlinear_solver,
        fdtype: GSL_MULTIFIT_NLINEAR_FWDIFF,
        factor_up: 0.,
        factor_down: 0.,
        avmax: 0.,
        h_df: 0.,
        h_fvv: 0.,
    };
    params.trs = gsl_multifit_nlinear_trs_lm;
    params.scale = gsl_multifit_nlinear_scale_more;
    params.solver = gsl_multifit_nlinear_solver_qr;
    params.fdtype = GSL_MULTIFIT_NLINEAR_FWDIFF;
    params.factor_up = 3.0f64;
    params.factor_down = 2.0f64;
    params.avmax = 0.75f64;
    params.h_df = 1.4901161193847656e-08f64;
    params.h_fvv = 0.02f64;
    return params;
}
pub unsafe extern "C" fn gsl_multifit_nlinear_init(
    mut x: *const gsl_vector,
    mut fdf: *mut gsl_multifit_nlinear_fdf,
    mut w: *mut gsl_multifit_nlinear_workspace,
) -> libc::c_int {
    return gsl_multifit_nlinear_winit(x, 0 as *const gsl_vector, fdf, w);
}
pub unsafe extern "C" fn gsl_multifit_nlinear_winit(
    mut x: *const gsl_vector,
    mut wts: *const gsl_vector,
    mut fdf: *mut gsl_multifit_nlinear_fdf,
    mut w: *mut gsl_multifit_nlinear_workspace,
) -> libc::c_int {
    let n: size_t = (*(*w).f).size;
    if n != (*fdf).n {
        gsl_error(
            b"function size does not match workspace\0" as *const u8
                as *const libc::c_char,
            b"fdf.c\0" as *const u8 as *const libc::c_char,
            169 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*(*w).x).size != (*x).size {
        gsl_error(
            b"vector length does not match workspace\0" as *const u8
                as *const libc::c_char,
            b"fdf.c\0" as *const u8 as *const libc::c_char,
            173 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if !wts.is_null() && n != (*wts).size {
        gsl_error(
            b"weight vector length does not match workspace\0" as *const u8
                as *const libc::c_char,
            b"fdf.c\0" as *const u8 as *const libc::c_char,
            177 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        (*fdf).nevalf = 0 as libc::c_int as size_t;
        (*fdf).nevaldf = 0 as libc::c_int as size_t;
        (*fdf).nevalfvv = 0 as libc::c_int as size_t;
        (*w).fdf = fdf;
        gsl_vector_memcpy((*w).x, x);
        (*w).niter = 0 as libc::c_int as size_t;
        if !wts.is_null() {
            (*w).sqrt_wts = (*w).sqrt_wts_work;
            i = 0 as libc::c_int as size_t;
            while i < n {
                let mut wi: libc::c_double = gsl_vector_get(wts, i);
                gsl_vector_set((*w).sqrt_wts, i, sqrt(wi));
                i = i.wrapping_add(1);
                i;
            }
        } else {
            (*w).sqrt_wts = 0 as *mut gsl_vector;
        }
        return ((*(*w).type_0).init)
            .unwrap()(
            (*w).state,
            (*w).sqrt_wts,
            (*w).fdf,
            (*w).x,
            (*w).f,
            (*w).J,
            (*w).g,
        );
    };
}
pub unsafe extern "C" fn gsl_multifit_nlinear_iterate(
    mut w: *mut gsl_multifit_nlinear_workspace,
) -> libc::c_int {
    let mut status: libc::c_int = ((*(*w).type_0).iterate)
        .unwrap()(
        (*w).state,
        (*w).sqrt_wts,
        (*w).fdf,
        (*w).x,
        (*w).f,
        (*w).J,
        (*w).g,
        (*w).dx,
    );
    (*w).niter = ((*w).niter).wrapping_add(1);
    (*w).niter;
    return status;
}
pub unsafe extern "C" fn gsl_multifit_nlinear_avratio(
    mut w: *const gsl_multifit_nlinear_workspace,
) -> libc::c_double {
    return ((*(*w).type_0).avratio).unwrap()((*w).state);
}
pub unsafe extern "C" fn gsl_multifit_nlinear_driver(
    maxiter: size_t,
    xtol: libc::c_double,
    gtol: libc::c_double,
    ftol: libc::c_double,
    mut callback: Option::<
        unsafe extern "C" fn(
            size_t,
            *mut libc::c_void,
            *const gsl_multifit_nlinear_workspace,
        ) -> (),
    >,
    mut callback_params: *mut libc::c_void,
    mut info: *mut libc::c_int,
    mut w: *mut gsl_multifit_nlinear_workspace,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut iter: size_t = 0 as libc::c_int as size_t;
    if callback.is_some() {
        callback.unwrap()(iter, callback_params, w);
    }
    loop {
        status = gsl_multifit_nlinear_iterate(w);
        if status == GSL_ENOPROG as libc::c_int
            && iter == 0 as libc::c_int as libc::c_ulong
        {
            *info = status;
            return GSL_EMAXITER as libc::c_int;
        }
        iter = iter.wrapping_add(1);
        iter;
        if callback.is_some() {
            callback.unwrap()(iter, callback_params, w);
        }
        status = gsl_multifit_nlinear_test(xtol, gtol, ftol, info, w);
        if !(status == GSL_CONTINUE as libc::c_int && iter < maxiter) {
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
pub unsafe extern "C" fn gsl_multifit_nlinear_jac(
    mut w: *const gsl_multifit_nlinear_workspace,
) -> *mut gsl_matrix {
    return (*w).J;
}
pub unsafe extern "C" fn gsl_multifit_nlinear_name(
    mut w: *const gsl_multifit_nlinear_workspace,
) -> *const libc::c_char {
    return (*(*w).type_0).name;
}
pub unsafe extern "C" fn gsl_multifit_nlinear_position(
    mut w: *const gsl_multifit_nlinear_workspace,
) -> *mut gsl_vector {
    return (*w).x;
}
pub unsafe extern "C" fn gsl_multifit_nlinear_residual(
    mut w: *const gsl_multifit_nlinear_workspace,
) -> *mut gsl_vector {
    return (*w).f;
}
pub unsafe extern "C" fn gsl_multifit_nlinear_niter(
    mut w: *const gsl_multifit_nlinear_workspace,
) -> size_t {
    return (*w).niter;
}
pub unsafe extern "C" fn gsl_multifit_nlinear_rcond(
    mut rcond: *mut libc::c_double,
    mut w: *const gsl_multifit_nlinear_workspace,
) -> libc::c_int {
    let mut status: libc::c_int = ((*(*w).type_0).rcond).unwrap()(rcond, (*w).state);
    return status;
}
pub unsafe extern "C" fn gsl_multifit_nlinear_trs_name(
    mut w: *const gsl_multifit_nlinear_workspace,
) -> *const libc::c_char {
    return (*(*w).params.trs).name;
}
pub unsafe extern "C" fn gsl_multifit_nlinear_eval_f(
    mut fdf: *mut gsl_multifit_nlinear_fdf,
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
pub unsafe extern "C" fn gsl_multifit_nlinear_eval_df(
    mut x: *const gsl_vector,
    mut f: *const gsl_vector,
    mut swts: *const gsl_vector,
    h: libc::c_double,
    fdtype: gsl_multifit_nlinear_fdtype,
    mut fdf: *mut gsl_multifit_nlinear_fdf,
    mut df: *mut gsl_matrix,
    mut work: *mut gsl_vector,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    if ((*fdf).df).is_some() {
        status = (Some(((*fdf).df).unwrap())).unwrap()(x, (*fdf).params, df);
        (*fdf).nevaldf = ((*fdf).nevaldf).wrapping_add(1);
        (*fdf).nevaldf;
        if !swts.is_null() {
            let n: size_t = (*swts).size;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < n {
                let mut swi: libc::c_double = gsl_vector_get(swts, i);
                let mut v: gsl_vector_view = gsl_matrix_row(df, i);
                gsl_vector_scale(&mut v.vector, swi);
                i = i.wrapping_add(1);
                i;
            }
        }
    } else {
        status = gsl_multifit_nlinear_df(h, fdtype, x, swts, fdf, f, df, work);
    }
    return status;
}
pub unsafe extern "C" fn gsl_multifit_nlinear_eval_fvv(
    h: libc::c_double,
    mut x: *const gsl_vector,
    mut v: *const gsl_vector,
    mut f: *const gsl_vector,
    mut J: *const gsl_matrix,
    mut swts: *const gsl_vector,
    mut fdf: *mut gsl_multifit_nlinear_fdf,
    mut yvv: *mut gsl_vector,
    mut work: *mut gsl_vector,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    if ((*fdf).fvv).is_some() {
        status = (Some(((*fdf).fvv).unwrap())).unwrap()(x, v, (*fdf).params, yvv);
        (*fdf).nevalfvv = ((*fdf).nevalfvv).wrapping_add(1);
        (*fdf).nevalfvv;
        if !swts.is_null() {
            gsl_vector_mul(yvv, swts);
        }
    } else {
        status = gsl_multifit_nlinear_fdfvv(h, x, v, f, J, swts, fdf, yvv, work);
    }
    return status;
}
