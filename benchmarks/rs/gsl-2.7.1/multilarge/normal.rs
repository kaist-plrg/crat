use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn gsl_hypot(x: libc::c_double, y: libc::c_double) -> libc::c_double;
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
    fn gsl_vector_set_zero(v: *mut gsl_vector);
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_minmax(
        v: *const gsl_vector,
        min_out: *mut libc::c_double,
        max_out: *mut libc::c_double,
    );
    fn gsl_vector_add_constant(a: *mut gsl_vector, x: libc::c_double) -> libc::c_int;
    fn gsl_matrix_alloc(n1: size_t, n2: size_t) -> *mut gsl_matrix;
    fn gsl_matrix_free(m: *mut gsl_matrix);
    fn gsl_matrix_diagonal(m: *mut gsl_matrix) -> _gsl_vector_view;
    fn gsl_matrix_set_zero(m: *mut gsl_matrix);
    fn gsl_matrix_tricpy(
        Uplo: CBLAS_UPLO_t,
        Diag: CBLAS_DIAG_t,
        dest: *mut gsl_matrix,
        src: *const gsl_matrix,
    ) -> libc::c_int;
    fn gsl_linalg_cholesky_decomp2(
        A: *mut gsl_matrix,
        S: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_linalg_cholesky_solve2(
        LLT: *const gsl_matrix,
        S: *const gsl_vector,
        b: *const gsl_vector,
        x: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_linalg_cholesky_rcond(
        LLT: *const gsl_matrix,
        rcond: *mut libc::c_double,
        work: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_blas_dnrm2(X: *const gsl_vector) -> libc::c_double;
    fn gsl_blas_ddot(
        X: *const gsl_vector,
        Y: *const gsl_vector,
        result: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_blas_dgemv(
        TransA: CBLAS_TRANSPOSE_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        X: *const gsl_vector,
        beta: libc::c_double,
        Y: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_blas_dsyrk(
        Uplo: CBLAS_UPLO_t,
        Trans: CBLAS_TRANSPOSE_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        beta: libc::c_double,
        C: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_blas_dsymv(
        Uplo: CBLAS_UPLO_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        X: *const gsl_vector,
        beta: libc::c_double,
        Y: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_eigen_symm_alloc(n: size_t) -> *mut gsl_eigen_symm_workspace;
    fn gsl_eigen_symm_free(w: *mut gsl_eigen_symm_workspace);
    fn gsl_eigen_symm(
        A: *mut gsl_matrix,
        eval: *mut gsl_vector,
        w: *mut gsl_eigen_symm_workspace,
    ) -> libc::c_int;
    fn gsl_multifit_linear_lreg(
        smin: libc::c_double,
        smax: libc::c_double,
        reg_param: *mut gsl_vector,
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
pub type CBLAS_TRANSPOSE = libc::c_uint;
pub const CblasConjTrans: CBLAS_TRANSPOSE = 113;
pub const CblasTrans: CBLAS_TRANSPOSE = 112;
pub const CblasNoTrans: CBLAS_TRANSPOSE = 111;
pub type CBLAS_UPLO = libc::c_uint;
pub const CblasLower: CBLAS_UPLO = 122;
pub const CblasUpper: CBLAS_UPLO = 121;
pub type CBLAS_DIAG = libc::c_uint;
pub const CblasUnit: CBLAS_DIAG = 132;
pub const CblasNonUnit: CBLAS_DIAG = 131;
pub type CBLAS_TRANSPOSE_t = CBLAS_TRANSPOSE;
pub type CBLAS_UPLO_t = CBLAS_UPLO;
pub type CBLAS_DIAG_t = CBLAS_DIAG;
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
pub struct gsl_eigen_symm_workspace {
    pub size: size_t,
    pub d: *mut libc::c_double,
    pub sd: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multilarge_linear_type {
    pub name: *const libc::c_char,
    pub alloc: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub reset: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub accumulate: Option::<
        unsafe extern "C" fn(
            *mut gsl_matrix,
            *mut gsl_vector,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub solve: Option::<
        unsafe extern "C" fn(
            libc::c_double,
            *mut gsl_vector,
            *mut libc::c_double,
            *mut libc::c_double,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub rcond: Option::<
        unsafe extern "C" fn(*mut libc::c_double, *mut libc::c_void) -> libc::c_int,
    >,
    pub lcurve: Option::<
        unsafe extern "C" fn(
            *mut gsl_vector,
            *mut gsl_vector,
            *mut gsl_vector,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub matrix_ptr: Option::<
        unsafe extern "C" fn(*const libc::c_void) -> *const gsl_matrix,
    >,
    pub rhs_ptr: Option::<
        unsafe extern "C" fn(*const libc::c_void) -> *const gsl_vector,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct normal_state_t {
    pub p: size_t,
    pub ATA: *mut gsl_matrix,
    pub ATb: *mut gsl_vector,
    pub normb: libc::c_double,
    pub work_ATA: *mut gsl_matrix,
    pub workp: *mut gsl_vector,
    pub work3p: *mut gsl_vector,
    pub D: *mut gsl_vector,
    pub c: *mut gsl_vector,
    pub eigen: libc::c_int,
    pub eval_min: libc::c_double,
    pub eval_max: libc::c_double,
    pub eigen_p: *mut gsl_eigen_symm_workspace,
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
unsafe extern "C" fn normal_alloc(p: size_t) -> *mut libc::c_void {
    let mut state: *mut normal_state_t = 0 as *mut normal_state_t;
    if p == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"p must be a positive integer\0" as *const u8 as *const libc::c_char,
            b"normal.c\0" as *const u8 as *const libc::c_char,
            87 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    state = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<normal_state_t>() as libc::c_ulong,
    ) as *mut normal_state_t;
    if state.is_null() {
        gsl_error(
            b"failed to allocate normal state\0" as *const u8 as *const libc::c_char,
            b"normal.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).p = p;
    (*state).ATA = gsl_matrix_alloc(p, p);
    if ((*state).ATA).is_null() {
        normal_free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate ATA matrix\0" as *const u8 as *const libc::c_char,
            b"normal.c\0" as *const u8 as *const libc::c_char,
            102 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).work_ATA = gsl_matrix_alloc(p, p);
    if ((*state).work_ATA).is_null() {
        normal_free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate temporary ATA matrix\0" as *const u8
                as *const libc::c_char,
            b"normal.c\0" as *const u8 as *const libc::c_char,
            109 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).ATb = gsl_vector_alloc(p);
    if ((*state).ATb).is_null() {
        normal_free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate ATb vector\0" as *const u8 as *const libc::c_char,
            b"normal.c\0" as *const u8 as *const libc::c_char,
            116 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).D = gsl_vector_alloc(p);
    if ((*state).D).is_null() {
        normal_free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate D vector\0" as *const u8 as *const libc::c_char,
            b"normal.c\0" as *const u8 as *const libc::c_char,
            123 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).workp = gsl_vector_alloc(p);
    if ((*state).workp).is_null() {
        normal_free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate temporary ATb vector\0" as *const u8
                as *const libc::c_char,
            b"normal.c\0" as *const u8 as *const libc::c_char,
            130 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .work3p = gsl_vector_alloc((3 as libc::c_int as libc::c_ulong).wrapping_mul(p));
    if ((*state).work3p).is_null() {
        normal_free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate work3p\0" as *const u8 as *const libc::c_char,
            b"normal.c\0" as *const u8 as *const libc::c_char,
            137 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).c = gsl_vector_alloc(p);
    if ((*state).c).is_null() {
        normal_free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate c vector\0" as *const u8 as *const libc::c_char,
            b"normal.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).eigen_p = gsl_eigen_symm_alloc(p);
    if ((*state).eigen_p).is_null() {
        normal_free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate eigen workspace\0" as *const u8 as *const libc::c_char,
            b"normal.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    normal_reset(state as *mut libc::c_void);
    return state as *mut libc::c_void;
}
unsafe extern "C" fn normal_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut normal_state_t = vstate as *mut normal_state_t;
    if !((*state).ATA).is_null() {
        gsl_matrix_free((*state).ATA);
    }
    if !((*state).work_ATA).is_null() {
        gsl_matrix_free((*state).work_ATA);
    }
    if !((*state).ATb).is_null() {
        gsl_vector_free((*state).ATb);
    }
    if !((*state).D).is_null() {
        gsl_vector_free((*state).D);
    }
    if !((*state).workp).is_null() {
        gsl_vector_free((*state).workp);
    }
    if !((*state).work3p).is_null() {
        gsl_vector_free((*state).work3p);
    }
    if !((*state).c).is_null() {
        gsl_vector_free((*state).c);
    }
    if !((*state).eigen_p).is_null() {
        gsl_eigen_symm_free((*state).eigen_p);
    }
    free(state as *mut libc::c_void);
}
unsafe extern "C" fn normal_reset(mut vstate: *mut libc::c_void) -> libc::c_int {
    let mut state: *mut normal_state_t = vstate as *mut normal_state_t;
    gsl_matrix_set_zero((*state).ATA);
    gsl_vector_set_zero((*state).ATb);
    (*state).normb = 0.0f64;
    (*state).eigen = 0 as libc::c_int;
    (*state).eval_min = 0.0f64;
    (*state).eval_max = 0.0f64;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn normal_accumulate(
    mut A: *mut gsl_matrix,
    mut b: *mut gsl_vector,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut state: *mut normal_state_t = vstate as *mut normal_state_t;
    let n: size_t = (*A).size1;
    if (*A).size2 != (*state).p {
        gsl_error(
            b"columns of A do not match workspace\0" as *const u8 as *const libc::c_char,
            b"normal.c\0" as *const u8 as *const libc::c_char,
            225 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if n != (*b).size {
        gsl_error(
            b"A and b have different numbers of rows\0" as *const u8
                as *const libc::c_char,
            b"normal.c\0" as *const u8 as *const libc::c_char,
            229 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut s: libc::c_int = 0;
        s = gsl_blas_dsyrk(CblasLower, CblasTrans, 1.0f64, A, 1.0f64, (*state).ATA);
        if s != 0 {
            return s;
        }
        s = gsl_blas_dgemv(CblasTrans, 1.0f64, A, b, 1.0f64, (*state).ATb);
        if s != 0 {
            return s;
        }
        (*state).normb = gsl_hypot((*state).normb, gsl_blas_dnrm2(b));
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn normal_solve(
    lambda: libc::c_double,
    mut x: *mut gsl_vector,
    mut rnorm: *mut libc::c_double,
    mut snorm: *mut libc::c_double,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut state: *mut normal_state_t = vstate as *mut normal_state_t;
    if (*x).size != (*state).p {
        gsl_error(
            b"solution vector does not match workspace\0" as *const u8
                as *const libc::c_char,
            b"normal.c\0" as *const u8 as *const libc::c_char,
            278 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        status = normal_solve_system(lambda, x, state);
        if status != 0 {
            gsl_error(
                b"failed to solve normal equations\0" as *const u8
                    as *const libc::c_char,
                b"normal.c\0" as *const u8 as *const libc::c_char,
                288 as libc::c_int,
                status,
            );
            return status;
        }
        normal_calc_norms(x, rnorm, snorm, state);
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn normal_rcond(
    mut rcond: *mut libc::c_double,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut state: *mut normal_state_t = vstate as *mut normal_state_t;
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut rcond_ATA: libc::c_double = 0.;
    status = gsl_linalg_cholesky_rcond(
        (*state).work_ATA,
        &mut rcond_ATA,
        (*state).work3p,
    );
    if status == GSL_SUCCESS as libc::c_int {
        *rcond = sqrt(rcond_ATA);
    }
    return status;
}
unsafe extern "C" fn normal_lcurve(
    mut reg_param: *mut gsl_vector,
    mut rho: *mut gsl_vector,
    mut eta: *mut gsl_vector,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut state: *mut normal_state_t = vstate as *mut normal_state_t;
    let mut status: libc::c_int = 0;
    let mut smin: libc::c_double = 0.;
    let mut smax: libc::c_double = 0.;
    let mut i: size_t = 0;
    if (*state).eigen == 0 as libc::c_int {
        status = normal_eigen(state);
        if status != 0 {
            return status;
        }
    }
    if (*state).eval_max < 0.0f64 {
        gsl_error(
            b"matrix is not positive definite\0" as *const u8 as *const libc::c_char,
            b"normal.c\0" as *const u8 as *const libc::c_char,
            342 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    }
    smax = sqrt((*state).eval_max);
    if (*state).eval_min > 0.0f64 {
        smin = sqrt((*state).eval_min);
    } else {
        smin = 0.0f64;
    }
    gsl_multifit_linear_lreg(smin, smax, reg_param);
    i = 0 as libc::c_int as size_t;
    while i < (*reg_param).size {
        let mut lambda: libc::c_double = gsl_vector_get(reg_param, i);
        let mut rnorm: libc::c_double = 0.;
        let mut snorm: libc::c_double = 0.;
        status = normal_solve_system(lambda, (*state).c, state);
        if status != 0 {
            return status;
        }
        normal_calc_norms((*state).c, &mut rnorm, &mut snorm, state);
        gsl_vector_set(rho, i, rnorm);
        gsl_vector_set(eta, i, snorm);
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn normal_ATA(mut vstate: *const libc::c_void) -> *const gsl_matrix {
    let mut state: *const normal_state_t = vstate as *const normal_state_t;
    return (*state).ATA;
}
unsafe extern "C" fn normal_ATb(mut vstate: *const libc::c_void) -> *const gsl_vector {
    let mut state: *const normal_state_t = vstate as *const normal_state_t;
    return (*state).ATb;
}
unsafe extern "C" fn normal_solve_system(
    lambda: libc::c_double,
    mut x: *mut gsl_vector,
    mut state: *mut normal_state_t,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let lambda_sq: libc::c_double = lambda * lambda;
    let mut d: gsl_vector_view = gsl_matrix_diagonal((*state).work_ATA);
    gsl_matrix_tricpy(CblasLower, CblasNonUnit, (*state).work_ATA, (*state).ATA);
    gsl_vector_add_constant(&mut d.vector, lambda_sq);
    status = normal_solve_cholesky((*state).work_ATA, (*state).ATb, x, state);
    if status != 0 {
        return status;
    }
    return status;
}
unsafe extern "C" fn normal_solve_cholesky(
    mut ATA: *mut gsl_matrix,
    mut ATb: *const gsl_vector,
    mut x: *mut gsl_vector,
    mut state: *mut normal_state_t,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    status = gsl_linalg_cholesky_decomp2(ATA, (*state).D);
    if status != 0 {
        return status;
    }
    status = gsl_linalg_cholesky_solve2(ATA, (*state).D, ATb, x);
    if status != 0 {
        return status;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn normal_calc_norms(
    mut x: *const gsl_vector,
    mut rnorm: *mut libc::c_double,
    mut snorm: *mut libc::c_double,
    mut state: *mut normal_state_t,
) -> libc::c_int {
    let mut r2: libc::c_double = 0.;
    *snorm = gsl_blas_dnrm2(x);
    gsl_vector_memcpy((*state).workp, (*state).ATb);
    gsl_blas_dsymv(CblasLower, 1.0f64, (*state).ATA, x, -2.0f64, (*state).workp);
    gsl_blas_ddot(x, (*state).workp, &mut r2);
    r2 += (*state).normb * (*state).normb;
    *rnorm = sqrt(r2);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn normal_eigen(mut state: *mut normal_state_t) -> libc::c_int {
    let mut status: libc::c_int = 0;
    gsl_matrix_tricpy(CblasLower, CblasNonUnit, (*state).work_ATA, (*state).ATA);
    status = gsl_eigen_symm((*state).work_ATA, (*state).workp, (*state).eigen_p);
    if status != 0 {
        return status;
    }
    gsl_vector_minmax((*state).workp, &mut (*state).eval_min, &mut (*state).eval_max);
    (*state).eigen = 1 as libc::c_int;
    return GSL_SUCCESS as libc::c_int;
}
static mut normal_type: gsl_multilarge_linear_type = unsafe {
    {
        let mut init = gsl_multilarge_linear_type {
            name: b"normal\0" as *const u8 as *const libc::c_char,
            alloc: Some(
                normal_alloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void,
            ),
            reset: Some(
                normal_reset as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
            ),
            accumulate: Some(
                normal_accumulate
                    as unsafe extern "C" fn(
                        *mut gsl_matrix,
                        *mut gsl_vector,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            solve: Some(
                normal_solve
                    as unsafe extern "C" fn(
                        libc::c_double,
                        *mut gsl_vector,
                        *mut libc::c_double,
                        *mut libc::c_double,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            rcond: Some(
                normal_rcond
                    as unsafe extern "C" fn(
                        *mut libc::c_double,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            lcurve: Some(
                normal_lcurve
                    as unsafe extern "C" fn(
                        *mut gsl_vector,
                        *mut gsl_vector,
                        *mut gsl_vector,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            matrix_ptr: Some(
                normal_ATA
                    as unsafe extern "C" fn(*const libc::c_void) -> *const gsl_matrix,
            ),
            rhs_ptr: Some(
                normal_ATb
                    as unsafe extern "C" fn(*const libc::c_void) -> *const gsl_vector,
            ),
            free: Some(normal_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        };
        init
    }
};
pub static mut gsl_multilarge_linear_normal: *const gsl_multilarge_linear_type = unsafe {
    &normal_type as *const gsl_multilarge_linear_type
};
