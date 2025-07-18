use ::libc;
extern "C" {
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
    fn gsl_vector_subvector(
        v: *mut gsl_vector,
        i: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
    fn gsl_vector_set_zero(v: *mut gsl_vector);
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_sub(a: *mut gsl_vector, b: *const gsl_vector) -> libc::c_int;
    fn gsl_matrix_alloc(n1: size_t, n2: size_t) -> *mut gsl_matrix;
    fn gsl_matrix_free(m: *mut gsl_matrix);
    fn gsl_matrix_submatrix(
        m: *mut gsl_matrix,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_view;
    fn gsl_matrix_set_zero(m: *mut gsl_matrix);
    fn gsl_matrix_tricpy(
        Uplo: CBLAS_UPLO_t,
        Diag: CBLAS_DIAG_t,
        dest: *mut gsl_matrix,
        src: *const gsl_matrix,
    ) -> libc::c_int;
    fn gsl_linalg_QR_decomp_r(A: *mut gsl_matrix, T: *mut gsl_matrix) -> libc::c_int;
    fn gsl_linalg_QR_QTvec_r(
        QR: *const gsl_matrix,
        T: *const gsl_matrix,
        b: *mut gsl_vector,
        work: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_linalg_QR_UR_decomp(
        S: *mut gsl_matrix,
        A: *mut gsl_matrix,
        T: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_linalg_tri_rcond(
        Uplo: CBLAS_UPLO_t,
        A: *const gsl_matrix,
        rcond: *mut libc::c_double,
        work: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_blas_dtrsv(
        Uplo: CBLAS_UPLO_t,
        TransA: CBLAS_TRANSPOSE_t,
        Diag: CBLAS_DIAG_t,
        A: *const gsl_matrix,
        X: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_blas_dtrmv(
        Uplo: CBLAS_UPLO_t,
        TransA: CBLAS_TRANSPOSE_t,
        Diag: CBLAS_DIAG_t,
        A: *const gsl_matrix,
        X: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_blas_dgemv(
        TransA: CBLAS_TRANSPOSE_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        X: *const gsl_vector,
        beta: libc::c_double,
        Y: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_blas_dnrm2(X: *const gsl_vector) -> libc::c_double;
    fn gsl_multifit_linear_alloc(
        n: size_t,
        p: size_t,
    ) -> *mut gsl_multifit_linear_workspace;
    fn gsl_multifit_linear_free(w: *mut gsl_multifit_linear_workspace);
    fn gsl_multifit_linear_svd(
        X: *const gsl_matrix,
        work: *mut gsl_multifit_linear_workspace,
    ) -> libc::c_int;
    fn gsl_multifit_linear_solve(
        lambda: libc::c_double,
        X: *const gsl_matrix,
        y: *const gsl_vector,
        c: *mut gsl_vector,
        rnorm: *mut libc::c_double,
        snorm: *mut libc::c_double,
        work: *mut gsl_multifit_linear_workspace,
    ) -> libc::c_int;
    fn gsl_multifit_linear_lcurve(
        y: *const gsl_vector,
        reg_param: *mut gsl_vector,
        rho: *mut gsl_vector,
        eta: *mut gsl_vector,
        work: *mut gsl_multifit_linear_workspace,
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
pub struct _gsl_matrix_view {
    pub matrix: gsl_matrix,
}
pub type gsl_matrix_view = _gsl_matrix_view;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_linear_workspace {
    pub nmax: size_t,
    pub pmax: size_t,
    pub n: size_t,
    pub p: size_t,
    pub A: *mut gsl_matrix,
    pub Q: *mut gsl_matrix,
    pub QSI: *mut gsl_matrix,
    pub S: *mut gsl_vector,
    pub t: *mut gsl_vector,
    pub xt: *mut gsl_vector,
    pub D: *mut gsl_vector,
    pub rcond: libc::c_double,
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
pub struct tsqr_state_t {
    pub p: size_t,
    pub nblocks: libc::c_int,
    pub rnorm: libc::c_double,
    pub svd: libc::c_int,
    pub T: *mut gsl_matrix,
    pub R: *mut gsl_matrix,
    pub QTb: *mut gsl_vector,
    pub work: *mut gsl_vector,
    pub work3: *mut gsl_vector,
    pub multifit_workspace_p: *mut gsl_multifit_linear_workspace,
}
#[inline]
unsafe extern "C" fn gsl_vector_ptr(
    mut v: *mut gsl_vector,
    i: size_t,
) -> *mut libc::c_double {
    return ((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
unsafe extern "C" fn tsqr_alloc(p: size_t) -> *mut libc::c_void {
    let mut state: *mut tsqr_state_t = 0 as *mut tsqr_state_t;
    if p == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"p must be a positive integer\0" as *const u8 as *const libc::c_char,
            b"tsqr.c\0" as *const u8 as *const libc::c_char,
            110 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    state = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<tsqr_state_t>() as libc::c_ulong,
    ) as *mut tsqr_state_t;
    if state.is_null() {
        gsl_error(
            b"failed to allocate tsqr state\0" as *const u8 as *const libc::c_char,
            b"tsqr.c\0" as *const u8 as *const libc::c_char,
            116 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).p = p;
    (*state).nblocks = 0 as libc::c_int;
    (*state).rnorm = 0.0f64;
    (*state).R = gsl_matrix_alloc(p, p);
    if ((*state).R).is_null() {
        tsqr_free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate R matrix\0" as *const u8 as *const libc::c_char,
            b"tsqr.c\0" as *const u8 as *const libc::c_char,
            127 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).QTb = gsl_vector_alloc(p);
    if ((*state).QTb).is_null() {
        tsqr_free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate QTb vector\0" as *const u8 as *const libc::c_char,
            b"tsqr.c\0" as *const u8 as *const libc::c_char,
            134 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).T = gsl_matrix_alloc(p, p);
    if ((*state).T).is_null() {
        tsqr_free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate T matrix\0" as *const u8 as *const libc::c_char,
            b"tsqr.c\0" as *const u8 as *const libc::c_char,
            141 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).work = gsl_vector_alloc(p);
    if ((*state).work).is_null() {
        tsqr_free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate workspace vector\0" as *const u8 as *const libc::c_char,
            b"tsqr.c\0" as *const u8 as *const libc::c_char,
            148 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .work3 = gsl_vector_alloc((3 as libc::c_int as libc::c_ulong).wrapping_mul(p));
    if ((*state).work3).is_null() {
        tsqr_free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate work3 vector\0" as *const u8 as *const libc::c_char,
            b"tsqr.c\0" as *const u8 as *const libc::c_char,
            155 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).multifit_workspace_p = gsl_multifit_linear_alloc(p, p);
    if ((*state).multifit_workspace_p).is_null() {
        tsqr_free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate multifit workspace\0" as *const u8
                as *const libc::c_char,
            b"tsqr.c\0" as *const u8 as *const libc::c_char,
            162 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    return state as *mut libc::c_void;
}
unsafe extern "C" fn tsqr_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut tsqr_state_t = vstate as *mut tsqr_state_t;
    if !((*state).R).is_null() {
        gsl_matrix_free((*state).R);
    }
    if !((*state).QTb).is_null() {
        gsl_vector_free((*state).QTb);
    }
    if !((*state).T).is_null() {
        gsl_matrix_free((*state).T);
    }
    if !((*state).work).is_null() {
        gsl_vector_free((*state).work);
    }
    if !((*state).work3).is_null() {
        gsl_vector_free((*state).work3);
    }
    if !((*state).multifit_workspace_p).is_null() {
        gsl_multifit_linear_free((*state).multifit_workspace_p);
    }
    free(state as *mut libc::c_void);
}
unsafe extern "C" fn tsqr_reset(mut vstate: *mut libc::c_void) -> libc::c_int {
    let mut state: *mut tsqr_state_t = vstate as *mut tsqr_state_t;
    gsl_matrix_set_zero((*state).R);
    gsl_vector_set_zero((*state).QTb);
    (*state).nblocks = 0 as libc::c_int;
    (*state).rnorm = 0.0f64;
    (*state).svd = 0 as libc::c_int;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn tsqr_accumulate(
    mut A: *mut gsl_matrix,
    mut b: *mut gsl_vector,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut state: *mut tsqr_state_t = vstate as *mut tsqr_state_t;
    let n: size_t = (*A).size1;
    let p: size_t = (*A).size2;
    if p != (*state).p {
        gsl_error(
            b"columns of A do not match workspace\0" as *const u8 as *const libc::c_char,
            b"tsqr.c\0" as *const u8 as *const libc::c_char,
            236 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if n != (*b).size {
        gsl_error(
            b"A and b have different numbers of rows\0" as *const u8
                as *const libc::c_char,
            b"tsqr.c\0" as *const u8 as *const libc::c_char,
            240 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*state).nblocks == 0 as libc::c_int && n < p {
        gsl_error(
            b"n must be >= p\0" as *const u8 as *const libc::c_char,
            b"tsqr.c\0" as *const u8 as *const libc::c_char,
            244 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*state).nblocks == 0 as libc::c_int {
        let mut status: libc::c_int = 0;
        let mut R: gsl_matrix_view = gsl_matrix_submatrix(
            A,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            p,
            p,
        );
        let mut QTb: gsl_vector_view = gsl_vector_subvector(
            (*state).QTb,
            0 as libc::c_int as size_t,
            p,
        );
        let mut b1: gsl_vector_view = gsl_vector_subvector(
            b,
            0 as libc::c_int as size_t,
            p,
        );
        status = gsl_linalg_QR_decomp_r(A, (*state).T);
        if status != 0 {
            return status;
        }
        gsl_matrix_tricpy(CblasUpper, CblasNonUnit, (*state).R, &mut R.matrix);
        gsl_linalg_QR_QTvec_r(A, (*state).T, b, (*state).work);
        gsl_vector_memcpy(&mut QTb.vector, &mut b1.vector);
        if n > p {
            let mut b2: gsl_vector_view = gsl_vector_subvector(b, p, n.wrapping_sub(p));
            (*state).rnorm = gsl_blas_dnrm2(&mut b2.vector);
        } else {
            (*state).rnorm = 0.0f64;
        }
        (*state).nblocks = 1 as libc::c_int;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut status_0: libc::c_int = 0;
        status_0 = gsl_linalg_QR_UR_decomp((*state).R, A, (*state).T);
        if status_0 != 0 {
            return status_0;
        }
        gsl_vector_memcpy((*state).work, (*state).QTb);
        gsl_blas_dgemv(CblasTrans, 1.0f64, A, b, 1.0f64, (*state).work);
        gsl_blas_dtrmv(CblasUpper, CblasTrans, CblasNonUnit, (*state).T, (*state).work);
        gsl_vector_sub((*state).QTb, (*state).work);
        gsl_blas_dgemv(CblasNoTrans, -1.0f64, A, (*state).work, 1.0f64, b);
        (*state).rnorm = gsl_hypot((*state).rnorm, gsl_blas_dnrm2(b));
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn tsqr_solve(
    lambda: libc::c_double,
    mut x: *mut gsl_vector,
    mut rnorm: *mut libc::c_double,
    mut snorm: *mut libc::c_double,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut state: *mut tsqr_state_t = vstate as *mut tsqr_state_t;
    if (*x).size != (*state).p {
        gsl_error(
            b"solution vector does not match workspace\0" as *const u8
                as *const libc::c_char,
            b"tsqr.c\0" as *const u8 as *const libc::c_char,
            342 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if lambda < 0.0f64 {
        gsl_error(
            b"regularization parameter should be non-negative\0" as *const u8
                as *const libc::c_char,
            b"tsqr.c\0" as *const u8 as *const libc::c_char,
            346 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        if lambda == 0.0f64 {
            gsl_vector_memcpy(x, (*state).QTb);
            gsl_blas_dtrsv(CblasUpper, CblasNoTrans, CblasNonUnit, (*state).R, x);
            *rnorm = (*state).rnorm;
            *snorm = gsl_blas_dnrm2(x);
        } else {
            let mut status: libc::c_int = 0;
            if (*state).svd == 0 as libc::c_int {
                status = tsqr_svd(state);
                if status != 0 {
                    return status;
                }
            }
            status = gsl_multifit_linear_solve(
                lambda,
                (*state).R,
                (*state).QTb,
                x,
                rnorm,
                snorm,
                (*state).multifit_workspace_p,
            );
            if status != 0 {
                return status;
            }
            *rnorm = gsl_hypot(*rnorm, (*state).rnorm);
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn tsqr_lcurve(
    mut reg_param: *mut gsl_vector,
    mut rho: *mut gsl_vector,
    mut eta: *mut gsl_vector,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut state: *mut tsqr_state_t = vstate as *mut tsqr_state_t;
    let mut status: libc::c_int = 0;
    let mut i: size_t = 0;
    if (*state).svd == 0 as libc::c_int {
        status = tsqr_svd(state);
        if status != 0 {
            return status;
        }
    }
    status = gsl_multifit_linear_lcurve(
        (*state).QTb,
        reg_param,
        rho,
        eta,
        (*state).multifit_workspace_p,
    );
    i = 0 as libc::c_int as size_t;
    while i < (*rho).size {
        let mut rhoi: *mut libc::c_double = gsl_vector_ptr(rho, i);
        *rhoi = gsl_hypot(*rhoi, (*state).rnorm);
        i = i.wrapping_add(1);
        i;
    }
    return status;
}
unsafe extern "C" fn tsqr_R(mut vstate: *const libc::c_void) -> *const gsl_matrix {
    let mut state: *const tsqr_state_t = vstate as *const tsqr_state_t;
    return (*state).R;
}
unsafe extern "C" fn tsqr_QTb(mut vstate: *const libc::c_void) -> *const gsl_vector {
    let mut state: *const tsqr_state_t = vstate as *const tsqr_state_t;
    return (*state).QTb;
}
unsafe extern "C" fn tsqr_rcond(
    mut rcond: *mut libc::c_double,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut state: *mut tsqr_state_t = vstate as *mut tsqr_state_t;
    return gsl_linalg_tri_rcond(CblasUpper, (*state).R, rcond, (*state).work3);
}
unsafe extern "C" fn tsqr_svd(mut state: *mut tsqr_state_t) -> libc::c_int {
    let mut status: libc::c_int = 0;
    status = gsl_multifit_linear_svd((*state).R, (*state).multifit_workspace_p);
    if status != 0 {
        gsl_error(
            b"error computing SVD of R\0" as *const u8 as *const libc::c_char,
            b"tsqr.c\0" as *const u8 as *const libc::c_char,
            464 as libc::c_int,
            status,
        );
        return status;
    }
    (*state).svd = 1 as libc::c_int;
    return GSL_SUCCESS as libc::c_int;
}
static mut tsqr_type: gsl_multilarge_linear_type = unsafe {
    {
        let mut init = gsl_multilarge_linear_type {
            name: b"tsqr\0" as *const u8 as *const libc::c_char,
            alloc: Some(tsqr_alloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
            reset: Some(
                tsqr_reset as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
            ),
            accumulate: Some(
                tsqr_accumulate
                    as unsafe extern "C" fn(
                        *mut gsl_matrix,
                        *mut gsl_vector,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            solve: Some(
                tsqr_solve
                    as unsafe extern "C" fn(
                        libc::c_double,
                        *mut gsl_vector,
                        *mut libc::c_double,
                        *mut libc::c_double,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            rcond: Some(
                tsqr_rcond
                    as unsafe extern "C" fn(
                        *mut libc::c_double,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            lcurve: Some(
                tsqr_lcurve
                    as unsafe extern "C" fn(
                        *mut gsl_vector,
                        *mut gsl_vector,
                        *mut gsl_vector,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            matrix_ptr: Some(
                tsqr_R as unsafe extern "C" fn(*const libc::c_void) -> *const gsl_matrix,
            ),
            rhs_ptr: Some(
                tsqr_QTb
                    as unsafe extern "C" fn(*const libc::c_void) -> *const gsl_vector,
            ),
            free: Some(tsqr_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        };
        init
    }
};
pub static mut gsl_multilarge_linear_tsqr: *const gsl_multilarge_linear_type = unsafe {
    &tsqr_type as *const gsl_multilarge_linear_type
};
