use ::libc;
extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_alloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_vector_div(a: *mut gsl_vector, b: *const gsl_vector) -> libc::c_int;
    fn gsl_matrix_alloc(n1: size_t, n2: size_t) -> *mut gsl_matrix;
    fn gsl_matrix_free(m: *mut gsl_matrix);
    fn gsl_matrix_column(m: *mut gsl_matrix, j: size_t) -> _gsl_vector_view;
    fn gsl_matrix_const_column(
        m: *const gsl_matrix,
        j: size_t,
    ) -> _gsl_vector_const_view;
    fn gsl_matrix_set_zero(m: *mut gsl_matrix);
    fn gsl_linalg_SV_decomp(
        A: *mut gsl_matrix,
        V: *mut gsl_matrix,
        S: *mut gsl_vector,
        work: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_blas_daxpy(
        alpha: libc::c_double,
        X: *const gsl_vector,
        Y: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_blas_dgemv(
        TransA: CBLAS_TRANSPOSE_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        X: *const gsl_vector,
        beta: libc::c_double,
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
pub struct _gsl_vector_view {
    pub vector: gsl_vector,
}
pub type gsl_vector_view = _gsl_vector_view;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_vector_const_view {
    pub vector: gsl_vector,
}
pub type gsl_vector_const_view = _gsl_vector_const_view;
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
pub struct gsl_multifit_nlinear_trust_state {
    pub x: *const gsl_vector,
    pub f: *const gsl_vector,
    pub g: *const gsl_vector,
    pub J: *const gsl_matrix,
    pub diag: *const gsl_vector,
    pub sqrt_wts: *const gsl_vector,
    pub mu: *const libc::c_double,
    pub params: *const gsl_multifit_nlinear_parameters,
    pub solver_state: *mut libc::c_void,
    pub fdf: *mut gsl_multifit_nlinear_fdf,
    pub avratio: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct svd_state_t {
    pub n: size_t,
    pub p: size_t,
    pub U: *mut gsl_matrix,
    pub V: *mut gsl_matrix,
    pub S: *mut gsl_vector,
    pub workp: *mut gsl_vector,
    pub mu: libc::c_double,
}
#[inline]
unsafe extern "C" fn gsl_vector_get(
    mut v: *const gsl_vector,
    i: size_t,
) -> libc::c_double {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_vector_ptr(
    mut v: *mut gsl_vector,
    i: size_t,
) -> *mut libc::c_double {
    return ((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
unsafe extern "C" fn svd_alloc(n: size_t, p: size_t) -> *mut libc::c_void {
    let mut state: *mut svd_state_t = 0 as *mut svd_state_t;
    state = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<svd_state_t>() as libc::c_ulong,
    ) as *mut svd_state_t;
    if state.is_null() {
        gsl_error(
            b"failed to allocate svd state\0" as *const u8 as *const libc::c_char,
            b"svd.c\0" as *const u8 as *const libc::c_char,
            78 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).U = gsl_matrix_alloc(n, p);
    if ((*state).U).is_null() {
        gsl_error(
            b"failed to allocate space for U\0" as *const u8 as *const libc::c_char,
            b"svd.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).V = gsl_matrix_alloc(p, p);
    if ((*state).V).is_null() {
        gsl_error(
            b"failed to allocate space for V\0" as *const u8 as *const libc::c_char,
            b"svd.c\0" as *const u8 as *const libc::c_char,
            90 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).S = gsl_vector_alloc(p);
    if ((*state).S).is_null() {
        gsl_error(
            b"failed to allocate space for S\0" as *const u8 as *const libc::c_char,
            b"svd.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).workp = gsl_vector_alloc(p);
    if ((*state).workp).is_null() {
        gsl_error(
            b"failed to allocate space for workp\0" as *const u8 as *const libc::c_char,
            b"svd.c\0" as *const u8 as *const libc::c_char,
            104 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).mu = 0.0f64;
    (*state).n = n;
    (*state).p = p;
    return state as *mut libc::c_void;
}
unsafe extern "C" fn svd_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut svd_state_t = vstate as *mut svd_state_t;
    if !((*state).U).is_null() {
        gsl_matrix_free((*state).U);
    }
    if !((*state).V).is_null() {
        gsl_matrix_free((*state).V);
    }
    if !((*state).S).is_null() {
        gsl_vector_free((*state).S);
    }
    if !((*state).workp).is_null() {
        gsl_vector_free((*state).workp);
    }
    free(state as *mut libc::c_void);
}
unsafe extern "C" fn svd_init(
    mut vtrust_state: *const libc::c_void,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut trust_state: *const gsl_multifit_nlinear_trust_state = vtrust_state
        as *const gsl_multifit_nlinear_trust_state;
    let mut state: *mut svd_state_t = vstate as *mut svd_state_t;
    let mut i: size_t = 0;
    gsl_matrix_set_zero((*state).U);
    i = 0 as libc::c_int as size_t;
    while i < (*state).p {
        let Ji: gsl_vector_const_view = gsl_matrix_const_column((*trust_state).J, i);
        let mut ui: gsl_vector_view = gsl_matrix_column((*state).U, i);
        let mut di: libc::c_double = gsl_vector_get((*trust_state).diag, i);
        gsl_blas_daxpy(1.0f64 / di, &Ji.vector, &mut ui.vector);
        i = i.wrapping_add(1);
        i;
    }
    status = gsl_linalg_SV_decomp((*state).U, (*state).V, (*state).S, (*state).workp);
    return status;
}
unsafe extern "C" fn svd_presolve(
    mu: libc::c_double,
    mut vtrust_state: *const libc::c_void,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut state: *mut svd_state_t = vstate as *mut svd_state_t;
    (*state).mu = mu;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn svd_solve(
    mut f: *const gsl_vector,
    mut x: *mut gsl_vector,
    mut vtrust_state: *const libc::c_void,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut trust_state: *const gsl_multifit_nlinear_trust_state = vtrust_state
        as *const gsl_multifit_nlinear_trust_state;
    let mut state: *mut svd_state_t = vstate as *mut svd_state_t;
    let p: size_t = (*state).p;
    let tol: libc::c_double = 2.2204460492503131e-16f64;
    let s0: libc::c_double = gsl_vector_get((*state).S, 0 as libc::c_int as size_t);
    let mut j: size_t = 0;
    gsl_blas_dgemv(CblasTrans, -1.0f64, (*state).U, f, 0.0f64, (*state).workp);
    if (*state).mu == 0.0f64 {
        j = 0 as libc::c_int as size_t;
        while j < p {
            let mut sj: libc::c_double = gsl_vector_get((*state).S, j);
            let mut ptr: *mut libc::c_double = gsl_vector_ptr((*state).workp, j);
            let mut alpha: libc::c_double = 0.;
            if sj <= tol * s0 {
                alpha = 0.0f64;
            } else {
                alpha = 1.0f64 / sj;
            }
            *ptr *= alpha;
            j = j.wrapping_add(1);
            j;
        }
    } else {
        j = 0 as libc::c_int as size_t;
        while j < p {
            let mut sj_0: libc::c_double = gsl_vector_get((*state).S, j);
            let mut ptr_0: *mut libc::c_double = gsl_vector_ptr((*state).workp, j);
            *ptr_0 *= sj_0 / (sj_0 * sj_0 + (*state).mu);
            j = j.wrapping_add(1);
            j;
        }
    }
    gsl_blas_dgemv(CblasNoTrans, 1.0f64, (*state).V, (*state).workp, 0.0f64, x);
    gsl_vector_div(x, (*trust_state).diag);
    return status;
}
unsafe extern "C" fn svd_rcond(
    mut rcond: *mut libc::c_double,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut state: *mut svd_state_t = vstate as *mut svd_state_t;
    let mut smax: libc::c_double = gsl_vector_get(
        (*state).S,
        0 as libc::c_int as size_t,
    );
    let mut smin: libc::c_double = gsl_vector_get(
        (*state).S,
        ((*state).p).wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    *rcond = smin / smax;
    return status;
}
static mut svd_type: gsl_multifit_nlinear_solver = unsafe {
    {
        let mut init = gsl_multifit_nlinear_solver {
            name: b"svd\0" as *const u8 as *const libc::c_char,
            alloc: Some(
                svd_alloc as unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void,
            ),
            init: Some(
                svd_init
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            presolve: Some(
                svd_presolve
                    as unsafe extern "C" fn(
                        libc::c_double,
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            solve: Some(
                svd_solve
                    as unsafe extern "C" fn(
                        *const gsl_vector,
                        *mut gsl_vector,
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            rcond: Some(
                svd_rcond
                    as unsafe extern "C" fn(
                        *mut libc::c_double,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            free: Some(svd_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        };
        init
    }
};
pub static mut gsl_multifit_nlinear_solver_svd: *const gsl_multifit_nlinear_solver = unsafe {
    &svd_type as *const gsl_multifit_nlinear_solver
};
