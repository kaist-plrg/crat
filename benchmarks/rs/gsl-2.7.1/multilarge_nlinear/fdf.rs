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
    fn gsl_matrix_alloc(n1: size_t, n2: size_t) -> *mut gsl_matrix;
    fn gsl_matrix_free(m: *mut gsl_matrix);
    fn gsl_multilarge_nlinear_test(
        xtol: libc::c_double,
        gtol: libc::c_double,
        ftol: libc::c_double,
        info: *mut libc::c_int,
        w: *const gsl_multilarge_nlinear_workspace,
    ) -> libc::c_int;
    static mut gsl_multilarge_nlinear_trs_lm: *const gsl_multilarge_nlinear_trs;
    static mut gsl_multilarge_nlinear_trs_cgst: *const gsl_multilarge_nlinear_trs;
    static mut gsl_multilarge_nlinear_scale_more: *const gsl_multilarge_nlinear_scale;
    static mut gsl_multilarge_nlinear_solver_cholesky: *const gsl_multilarge_nlinear_solver;
    static mut gsl_multilarge_nlinear_solver_mcholesky: *const gsl_multilarge_nlinear_solver;
    static mut gsl_multilarge_nlinear_solver_none: *const gsl_multilarge_nlinear_solver;
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
pub type CBLAS_TRANSPOSE = libc::c_uint;
pub const CblasConjTrans: CBLAS_TRANSPOSE = 113;
pub const CblasTrans: CBLAS_TRANSPOSE = 112;
pub const CblasNoTrans: CBLAS_TRANSPOSE = 111;
pub type CBLAS_TRANSPOSE_t = CBLAS_TRANSPOSE;
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
pub type gsl_multilarge_nlinear_fdtype = libc::c_uint;
pub const GSL_MULTILARGE_NLINEAR_CTRDIFF: gsl_multilarge_nlinear_fdtype = 1;
pub const GSL_MULTILARGE_NLINEAR_FWDIFF: gsl_multilarge_nlinear_fdtype = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multilarge_nlinear_fdf {
    pub f: Option::<
        unsafe extern "C" fn(
            *const gsl_vector,
            *mut libc::c_void,
            *mut gsl_vector,
        ) -> libc::c_int,
    >,
    pub df: Option::<
        unsafe extern "C" fn(
            CBLAS_TRANSPOSE_t,
            *const gsl_vector,
            *const gsl_vector,
            *mut libc::c_void,
            *mut gsl_vector,
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
    pub nevaldfu: size_t,
    pub nevaldf2: size_t,
    pub nevalfvv: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multilarge_nlinear_trs {
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
pub struct gsl_multilarge_nlinear_scale {
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
pub struct gsl_multilarge_nlinear_solver {
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
        unsafe extern "C" fn(
            *mut libc::c_double,
            *const gsl_matrix,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub covar: Option::<
        unsafe extern "C" fn(
            *const gsl_matrix,
            *mut gsl_matrix,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multilarge_nlinear_parameters {
    pub trs: *const gsl_multilarge_nlinear_trs,
    pub scale: *const gsl_multilarge_nlinear_scale,
    pub solver: *const gsl_multilarge_nlinear_solver,
    pub fdtype: gsl_multilarge_nlinear_fdtype,
    pub factor_up: libc::c_double,
    pub factor_down: libc::c_double,
    pub avmax: libc::c_double,
    pub h_df: libc::c_double,
    pub h_fvv: libc::c_double,
    pub max_iter: size_t,
    pub tol: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multilarge_nlinear_type {
    pub name: *const libc::c_char,
    pub alloc: Option::<
        unsafe extern "C" fn(
            *const gsl_multilarge_nlinear_parameters,
            size_t,
            size_t,
        ) -> *mut libc::c_void,
    >,
    pub init: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const gsl_vector,
            *mut gsl_multilarge_nlinear_fdf,
            *const gsl_vector,
            *mut gsl_vector,
            *mut gsl_vector,
            *mut gsl_matrix,
        ) -> libc::c_int,
    >,
    pub iterate: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const gsl_vector,
            *mut gsl_multilarge_nlinear_fdf,
            *mut gsl_vector,
            *mut gsl_vector,
            *mut gsl_vector,
            *mut gsl_matrix,
            *mut gsl_vector,
        ) -> libc::c_int,
    >,
    pub rcond: Option::<
        unsafe extern "C" fn(
            *mut libc::c_double,
            *const gsl_matrix,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub covar: Option::<
        unsafe extern "C" fn(
            *const gsl_matrix,
            *mut gsl_matrix,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub avratio: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double>,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multilarge_nlinear_workspace {
    pub type_0: *const gsl_multilarge_nlinear_type,
    pub fdf: *mut gsl_multilarge_nlinear_fdf,
    pub x: *mut gsl_vector,
    pub f: *mut gsl_vector,
    pub dx: *mut gsl_vector,
    pub g: *mut gsl_vector,
    pub JTJ: *mut gsl_matrix,
    pub sqrt_wts_work: *mut gsl_vector,
    pub sqrt_wts: *mut gsl_vector,
    pub n: size_t,
    pub p: size_t,
    pub niter: size_t,
    pub params: gsl_multilarge_nlinear_parameters,
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
pub unsafe extern "C" fn gsl_multilarge_nlinear_alloc(
    mut T: *const gsl_multilarge_nlinear_type,
    mut params: *const gsl_multilarge_nlinear_parameters,
    n: size_t,
    p: size_t,
) -> *mut gsl_multilarge_nlinear_workspace {
    let mut w: *mut gsl_multilarge_nlinear_workspace = 0
        as *mut gsl_multilarge_nlinear_workspace;
    if n < p {
        gsl_error(
            b"insufficient data points, n < p\0" as *const u8 as *const libc::c_char,
            b"fdf.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_multilarge_nlinear_workspace;
    }
    w = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gsl_multilarge_nlinear_workspace>() as libc::c_ulong,
    ) as *mut gsl_multilarge_nlinear_workspace;
    if w.is_null() {
        gsl_error(
            b"failed to allocate space for workspace\0" as *const u8
                as *const libc::c_char,
            b"fdf.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multilarge_nlinear_workspace;
    }
    (*w).n = n;
    (*w).p = p;
    (*w).type_0 = T;
    (*w).fdf = 0 as *mut gsl_multilarge_nlinear_fdf;
    (*w).niter = 0 as libc::c_int as size_t;
    (*w).params = *params;
    if (*w).params.trs == gsl_multilarge_nlinear_trs_cgst {
        (*w).params.solver = gsl_multilarge_nlinear_solver_none;
    }
    (*w).x = gsl_vector_calloc(p);
    if ((*w).x).is_null() {
        gsl_multilarge_nlinear_free(w);
        gsl_error(
            b"failed to allocate space for x\0" as *const u8 as *const libc::c_char,
            b"fdf.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multilarge_nlinear_workspace;
    }
    (*w).f = gsl_vector_calloc(n);
    if ((*w).f).is_null() {
        gsl_multilarge_nlinear_free(w);
        gsl_error(
            b"failed to allocate space for f\0" as *const u8 as *const libc::c_char,
            b"fdf.c\0" as *const u8 as *const libc::c_char,
            71 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multilarge_nlinear_workspace;
    }
    (*w).dx = gsl_vector_calloc(p);
    if ((*w).dx).is_null() {
        gsl_multilarge_nlinear_free(w);
        gsl_error(
            b"failed to allocate space for dx\0" as *const u8 as *const libc::c_char,
            b"fdf.c\0" as *const u8 as *const libc::c_char,
            78 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multilarge_nlinear_workspace;
    }
    (*w).g = gsl_vector_alloc(p);
    if ((*w).g).is_null() {
        gsl_multilarge_nlinear_free(w);
        gsl_error(
            b"failed to allocate space for g\0" as *const u8 as *const libc::c_char,
            b"fdf.c\0" as *const u8 as *const libc::c_char,
            85 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multilarge_nlinear_workspace;
    }
    if (*w).params.solver == gsl_multilarge_nlinear_solver_cholesky
        || (*w).params.solver == gsl_multilarge_nlinear_solver_mcholesky
    {
        (*w).JTJ = gsl_matrix_alloc(p, p);
        if ((*w).JTJ).is_null() {
            gsl_multilarge_nlinear_free(w);
            gsl_error(
                b"failed to allocate space for JTJ\0" as *const u8
                    as *const libc::c_char,
                b"fdf.c\0" as *const u8 as *const libc::c_char,
                95 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_multilarge_nlinear_workspace;
        }
    }
    (*w).sqrt_wts_work = gsl_vector_calloc(n);
    if ((*w).sqrt_wts_work).is_null() {
        gsl_multilarge_nlinear_free(w);
        gsl_error(
            b"failed to allocate space for weights\0" as *const u8
                as *const libc::c_char,
            b"fdf.c\0" as *const u8 as *const libc::c_char,
            103 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multilarge_nlinear_workspace;
    }
    (*w).state = ((*T).alloc).unwrap()(&mut (*w).params, n, p);
    if ((*w).state).is_null() {
        gsl_multilarge_nlinear_free(w);
        gsl_error(
            b"failed to allocate space for state\0" as *const u8 as *const libc::c_char,
            b"fdf.c\0" as *const u8 as *const libc::c_char,
            110 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multilarge_nlinear_workspace;
    }
    return w;
}
pub unsafe extern "C" fn gsl_multilarge_nlinear_free(
    mut w: *mut gsl_multilarge_nlinear_workspace,
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
    if !((*w).JTJ).is_null() {
        gsl_matrix_free((*w).JTJ);
    }
    free(w as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_multilarge_nlinear_default_parameters() -> gsl_multilarge_nlinear_parameters {
    let mut params: gsl_multilarge_nlinear_parameters = gsl_multilarge_nlinear_parameters {
        trs: 0 as *const gsl_multilarge_nlinear_trs,
        scale: 0 as *const gsl_multilarge_nlinear_scale,
        solver: 0 as *const gsl_multilarge_nlinear_solver,
        fdtype: GSL_MULTILARGE_NLINEAR_FWDIFF,
        factor_up: 0.,
        factor_down: 0.,
        avmax: 0.,
        h_df: 0.,
        h_fvv: 0.,
        max_iter: 0,
        tol: 0.,
    };
    params.trs = gsl_multilarge_nlinear_trs_lm;
    params.scale = gsl_multilarge_nlinear_scale_more;
    params.solver = gsl_multilarge_nlinear_solver_cholesky;
    params.fdtype = GSL_MULTILARGE_NLINEAR_FWDIFF;
    params.factor_up = 3.0f64;
    params.factor_down = 2.0f64;
    params.avmax = 0.75f64;
    params.h_df = 1.4901161193847656e-08f64;
    params.h_fvv = 0.01f64;
    params.max_iter = 0 as libc::c_int as size_t;
    params.tol = 1.0e-6f64;
    return params;
}
pub unsafe extern "C" fn gsl_multilarge_nlinear_init(
    mut x: *const gsl_vector,
    mut fdf: *mut gsl_multilarge_nlinear_fdf,
    mut w: *mut gsl_multilarge_nlinear_workspace,
) -> libc::c_int {
    return gsl_multilarge_nlinear_winit(x, 0 as *const gsl_vector, fdf, w);
}
pub unsafe extern "C" fn gsl_multilarge_nlinear_winit(
    mut x: *const gsl_vector,
    mut wts: *const gsl_vector,
    mut fdf: *mut gsl_multilarge_nlinear_fdf,
    mut w: *mut gsl_multilarge_nlinear_workspace,
) -> libc::c_int {
    let n: size_t = (*(*w).f).size;
    if n != (*fdf).n {
        gsl_error(
            b"function size does not match workspace\0" as *const u8
                as *const libc::c_char,
            b"fdf.c\0" as *const u8 as *const libc::c_char,
            183 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*(*w).x).size != (*x).size {
        gsl_error(
            b"vector length does not match workspace\0" as *const u8
                as *const libc::c_char,
            b"fdf.c\0" as *const u8 as *const libc::c_char,
            187 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if !wts.is_null() && n != (*wts).size {
        gsl_error(
            b"weight vector length does not match workspace\0" as *const u8
                as *const libc::c_char,
            b"fdf.c\0" as *const u8 as *const libc::c_char,
            191 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        (*fdf).nevalf = 0 as libc::c_int as size_t;
        (*fdf).nevaldfu = 0 as libc::c_int as size_t;
        (*fdf).nevaldf2 = 0 as libc::c_int as size_t;
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
            (*w).g,
            (*w).JTJ,
        );
    };
}
pub unsafe extern "C" fn gsl_multilarge_nlinear_iterate(
    mut w: *mut gsl_multilarge_nlinear_workspace,
) -> libc::c_int {
    let mut status: libc::c_int = ((*(*w).type_0).iterate)
        .unwrap()(
        (*w).state,
        (*w).sqrt_wts,
        (*w).fdf,
        (*w).x,
        (*w).f,
        (*w).g,
        (*w).JTJ,
        (*w).dx,
    );
    (*w).niter = ((*w).niter).wrapping_add(1);
    (*w).niter;
    return status;
}
pub unsafe extern "C" fn gsl_multilarge_nlinear_avratio(
    mut w: *const gsl_multilarge_nlinear_workspace,
) -> libc::c_double {
    return ((*(*w).type_0).avratio).unwrap()((*w).state);
}
pub unsafe extern "C" fn gsl_multilarge_nlinear_rcond(
    mut rcond: *mut libc::c_double,
    mut w: *const gsl_multilarge_nlinear_workspace,
) -> libc::c_int {
    let mut status: libc::c_int = ((*(*w).type_0).rcond)
        .unwrap()(rcond, (*w).JTJ, (*w).state);
    return status;
}
pub unsafe extern "C" fn gsl_multilarge_nlinear_covar(
    mut covar: *mut gsl_matrix,
    mut w: *mut gsl_multilarge_nlinear_workspace,
) -> libc::c_int {
    if (*covar).size1 != (*covar).size2 {
        gsl_error(
            b"covariance matrix must be square\0" as *const u8 as *const libc::c_char,
            b"fdf.c\0" as *const u8 as *const libc::c_char,
            257 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*covar).size1 != (*w).p {
        gsl_error(
            b"covariance matrix does not match workspace\0" as *const u8
                as *const libc::c_char,
            b"fdf.c\0" as *const u8 as *const libc::c_char,
            261 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = ((*(*w).type_0).covar)
            .unwrap()((*w).JTJ, covar, (*w).state);
        return status;
    };
}
pub unsafe extern "C" fn gsl_multilarge_nlinear_driver(
    maxiter: size_t,
    xtol: libc::c_double,
    gtol: libc::c_double,
    ftol: libc::c_double,
    mut callback: Option::<
        unsafe extern "C" fn(
            size_t,
            *mut libc::c_void,
            *const gsl_multilarge_nlinear_workspace,
        ) -> (),
    >,
    mut callback_params: *mut libc::c_void,
    mut info: *mut libc::c_int,
    mut w: *mut gsl_multilarge_nlinear_workspace,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut iter: size_t = 0 as libc::c_int as size_t;
    if callback.is_some() {
        callback.unwrap()(iter, callback_params, w);
    }
    loop {
        status = gsl_multilarge_nlinear_iterate(w);
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
        status = gsl_multilarge_nlinear_test(xtol, gtol, ftol, info, w);
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
pub unsafe extern "C" fn gsl_multilarge_nlinear_name(
    mut w: *const gsl_multilarge_nlinear_workspace,
) -> *const libc::c_char {
    return (*(*w).type_0).name;
}
pub unsafe extern "C" fn gsl_multilarge_nlinear_position(
    mut w: *const gsl_multilarge_nlinear_workspace,
) -> *mut gsl_vector {
    return (*w).x;
}
pub unsafe extern "C" fn gsl_multilarge_nlinear_residual(
    mut w: *const gsl_multilarge_nlinear_workspace,
) -> *mut gsl_vector {
    return (*w).f;
}
pub unsafe extern "C" fn gsl_multilarge_nlinear_step(
    mut w: *const gsl_multilarge_nlinear_workspace,
) -> *mut gsl_vector {
    return (*w).dx;
}
pub unsafe extern "C" fn gsl_multilarge_nlinear_niter(
    mut w: *const gsl_multilarge_nlinear_workspace,
) -> size_t {
    return (*w).niter;
}
pub unsafe extern "C" fn gsl_multilarge_nlinear_trs_name(
    mut w: *const gsl_multilarge_nlinear_workspace,
) -> *const libc::c_char {
    return (*(*w).params.trs).name;
}
pub unsafe extern "C" fn gsl_multilarge_nlinear_eval_f(
    mut fdf: *mut gsl_multilarge_nlinear_fdf,
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
pub unsafe extern "C" fn gsl_multilarge_nlinear_eval_df(
    TransJ: CBLAS_TRANSPOSE_t,
    mut x: *const gsl_vector,
    mut f: *const gsl_vector,
    mut u: *const gsl_vector,
    mut swts: *const gsl_vector,
    h: libc::c_double,
    fdtype: gsl_multilarge_nlinear_fdtype,
    mut fdf: *mut gsl_multilarge_nlinear_fdf,
    mut v: *mut gsl_vector,
    mut JTJ: *mut gsl_matrix,
    mut work: *mut gsl_vector,
) -> libc::c_int {
    let n: size_t = (*fdf).n;
    let p: size_t = (*fdf).p;
    if !u.is_null()
        && (TransJ as libc::c_uint == CblasNoTrans as libc::c_int as libc::c_uint
            && (*u).size != p
            || TransJ as libc::c_uint == CblasTrans as libc::c_int as libc::c_uint
                && (*u).size != n)
    {
        gsl_error(
            b"u vector has wrong size\0" as *const u8 as *const libc::c_char,
            b"fdf.c\0" as *const u8 as *const libc::c_char,
            476 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if !v.is_null()
        && (TransJ as libc::c_uint == CblasNoTrans as libc::c_int as libc::c_uint
            && (*v).size != n
            || TransJ as libc::c_uint == CblasTrans as libc::c_int as libc::c_uint
                && (*v).size != p)
    {
        gsl_error(
            b"v vector has wrong size\0" as *const u8 as *const libc::c_char,
            b"fdf.c\0" as *const u8 as *const libc::c_char,
            481 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if !JTJ.is_null() && ((*JTJ).size1 != p || (*JTJ).size2 != p) {
        gsl_error(
            b"JTJ matrix has wrong size\0" as *const u8 as *const libc::c_char,
            b"fdf.c\0" as *const u8 as *const libc::c_char,
            485 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        if ((*fdf).df).is_some() {
            status = (Some(((*fdf).df).unwrap()))
                .unwrap()(TransJ, x, u, (*fdf).params, v, JTJ);
            if !v.is_null() {
                (*fdf).nevaldfu = ((*fdf).nevaldfu).wrapping_add(1);
                (*fdf).nevaldfu;
            }
            if !JTJ.is_null() {
                (*fdf).nevaldf2 = ((*fdf).nevaldf2).wrapping_add(1);
                (*fdf).nevaldf2;
            }
        }
        return status;
    };
}
pub unsafe extern "C" fn gsl_multilarge_nlinear_eval_fvv(
    h: libc::c_double,
    mut x: *const gsl_vector,
    mut v: *const gsl_vector,
    mut f: *const gsl_vector,
    mut swts: *const gsl_vector,
    mut fdf: *mut gsl_multilarge_nlinear_fdf,
    mut yvv: *mut gsl_vector,
    mut work: *mut gsl_vector,
) -> libc::c_int {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    if ((*fdf).fvv).is_some() {
        status = (Some(((*fdf).fvv).unwrap())).unwrap()(x, v, (*fdf).params, yvv);
        (*fdf).nevalfvv = ((*fdf).nevalfvv).wrapping_add(1);
        (*fdf).nevalfvv;
    }
    if !swts.is_null() {
        gsl_vector_mul(yvv, swts);
    }
    return status;
}
