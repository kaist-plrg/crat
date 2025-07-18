use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
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
    fn gsl_vector_view_array(v: *mut libc::c_double, n: size_t) -> _gsl_vector_view;
    fn gsl_vector_set_zero(v: *mut gsl_vector);
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_mul(a: *mut gsl_vector, b: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_div(a: *mut gsl_vector, b: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_scale(a: *mut gsl_vector, x: libc::c_double) -> libc::c_int;
    fn gsl_matrix_alloc(n1: size_t, n2: size_t) -> *mut gsl_matrix;
    fn gsl_matrix_free(m: *mut gsl_matrix);
    fn gsl_matrix_submatrix(
        m: *mut gsl_matrix,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_view;
    fn gsl_matrix_column(m: *mut gsl_matrix, j: size_t) -> _gsl_vector_view;
    fn gsl_matrix_view_array(
        base: *mut libc::c_double,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_view;
    fn gsl_matrix_memcpy(dest: *mut gsl_matrix, src: *const gsl_matrix) -> libc::c_int;
    fn gsl_linalg_QR_QTvec(
        QR: *const gsl_matrix,
        tau: *const gsl_vector,
        v: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_linalg_QR_Qvec(
        QR: *const gsl_matrix,
        tau: *const gsl_vector,
        v: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_linalg_QR_matQ(
        QR: *const gsl_matrix,
        tau: *const gsl_vector,
        A: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_linalg_QRPT_decomp(
        A: *mut gsl_matrix,
        tau: *mut gsl_vector,
        p: *mut gsl_permutation,
        signum: *mut libc::c_int,
        norm: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_linalg_QRPT_rank(QR: *const gsl_matrix, tol: libc::c_double) -> size_t;
    fn gsl_linalg_mcholesky_decomp(
        A: *mut gsl_matrix,
        p: *mut gsl_permutation,
        E: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_linalg_mcholesky_solve(
        LDLT: *const gsl_matrix,
        p: *const gsl_permutation,
        b: *const gsl_vector,
        x: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_blas_ddot(
        X: *const gsl_vector,
        Y: *const gsl_vector,
        result: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_permutation_free(p: *mut gsl_permutation);
    fn gsl_permutation_alloc(n: size_t) -> *mut gsl_permutation;
    fn gsl_blas_dsymv(
        Uplo: CBLAS_UPLO_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        X: *const gsl_vector,
        beta: libc::c_double,
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
    fn gsl_blas_dnrm2(X: *const gsl_vector) -> libc::c_double;
    fn gsl_blas_dsyrk(
        Uplo: CBLAS_UPLO_t,
        Trans: CBLAS_TRANSPOSE_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        beta: libc::c_double,
        C: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_poly_complex_workspace_alloc(n: size_t) -> *mut gsl_poly_complex_workspace;
    fn gsl_poly_complex_workspace_free(w: *mut gsl_poly_complex_workspace);
    fn gsl_poly_complex_solve(
        a: *const libc::c_double,
        n: size_t,
        w: *mut gsl_poly_complex_workspace,
        z: gsl_complex_packed_ptr,
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
pub type gsl_complex_packed_ptr = *mut libc::c_double;
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
pub type CBLAS_TRANSPOSE_t = CBLAS_TRANSPOSE;
pub type CBLAS_UPLO_t = CBLAS_UPLO;
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
pub struct gsl_permutation_struct {
    pub size: size_t,
    pub data: *mut size_t,
}
pub type gsl_permutation = gsl_permutation_struct;
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
pub struct subspace2D_state_t {
    pub n: size_t,
    pub p: size_t,
    pub dx_gn: *mut gsl_vector,
    pub dx_sd: *mut gsl_vector,
    pub norm_Dgn: libc::c_double,
    pub norm_Dsd: libc::c_double,
    pub workp: *mut gsl_vector,
    pub workn: *mut gsl_vector,
    pub W: *mut gsl_matrix,
    pub JQ: *mut gsl_matrix,
    pub tau: *mut gsl_vector,
    pub subg: *mut gsl_vector,
    pub subB: *mut gsl_matrix,
    pub perm: *mut gsl_permutation,
    pub trB: libc::c_double,
    pub detB: libc::c_double,
    pub normg: libc::c_double,
    pub term0: libc::c_double,
    pub term1: libc::c_double,
    pub rank: size_t,
    pub poly_p: *mut gsl_poly_complex_workspace,
    pub params: gsl_multifit_nlinear_parameters,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_poly_complex_workspace {
    pub nc: size_t,
    pub matrix: *mut libc::c_double,
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
#[inline]
unsafe extern "C" fn gsl_matrix_get(
    mut m: *const gsl_matrix,
    i: size_t,
    j: size_t,
) -> libc::c_double {
    return *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
#[inline]
unsafe extern "C" fn gsl_matrix_set(
    mut m: *mut gsl_matrix,
    i: size_t,
    j: size_t,
    x: libc::c_double,
) {
    *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize) = x;
}
unsafe extern "C" fn scaled_enorm(
    mut d: *const gsl_vector,
    mut f: *const gsl_vector,
) -> libc::c_double {
    let mut e2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: size_t = 0;
    let mut n: size_t = (*f).size;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut fi: libc::c_double = gsl_vector_get(f, i);
        let mut di: libc::c_double = gsl_vector_get(d, i);
        let mut u: libc::c_double = di * fi;
        e2 += u * u;
        i = i.wrapping_add(1);
        i;
    }
    return sqrt(e2);
}
unsafe extern "C" fn quadratic_preduction(
    mut f: *const gsl_vector,
    mut J: *const gsl_matrix,
    mut dx: *const gsl_vector,
    mut work: *mut gsl_vector,
) -> libc::c_double {
    let n: size_t = (*f).size;
    let normf: libc::c_double = gsl_blas_dnrm2(f);
    let mut pred_reduction: libc::c_double = 0.;
    let mut norm_beta: libc::c_double = 0.;
    let mut i: size_t = 0;
    gsl_blas_dgemv(CblasNoTrans, 1.0f64 / normf, J, dx, 0.0f64, work);
    norm_beta = gsl_blas_dnrm2(work);
    pred_reduction = -norm_beta * norm_beta;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut fi: libc::c_double = gsl_vector_get(f, i);
        let mut betai: libc::c_double = gsl_vector_get(work, i);
        pred_reduction -= 2.0f64 * (fi / normf) * betai;
        i = i.wrapping_add(1);
        i;
    }
    return pred_reduction;
}
unsafe extern "C" fn subspace2D_alloc(
    mut params: *const libc::c_void,
    n: size_t,
    p: size_t,
) -> *mut libc::c_void {
    let mut par: *const gsl_multifit_nlinear_parameters = params
        as *const gsl_multifit_nlinear_parameters;
    let mut state: *mut subspace2D_state_t = 0 as *mut subspace2D_state_t;
    state = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<subspace2D_state_t>() as libc::c_ulong,
    ) as *mut subspace2D_state_t;
    if state.is_null() {
        gsl_error(
            b"failed to allocate subspace2D state\0" as *const u8 as *const libc::c_char,
            b"subspace2D.c\0" as *const u8 as *const libc::c_char,
            165 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).dx_gn = gsl_vector_alloc(p);
    if ((*state).dx_gn).is_null() {
        gsl_error(
            b"failed to allocate space for dx_gn\0" as *const u8 as *const libc::c_char,
            b"subspace2D.c\0" as *const u8 as *const libc::c_char,
            171 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).dx_sd = gsl_vector_alloc(p);
    if ((*state).dx_sd).is_null() {
        gsl_error(
            b"failed to allocate space for dx_sd\0" as *const u8 as *const libc::c_char,
            b"subspace2D.c\0" as *const u8 as *const libc::c_char,
            177 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).workp = gsl_vector_alloc(p);
    if ((*state).workp).is_null() {
        gsl_error(
            b"failed to allocate space for workp\0" as *const u8 as *const libc::c_char,
            b"subspace2D.c\0" as *const u8 as *const libc::c_char,
            183 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).workn = gsl_vector_alloc(n);
    if ((*state).workn).is_null() {
        gsl_error(
            b"failed to allocate space for workn\0" as *const u8 as *const libc::c_char,
            b"subspace2D.c\0" as *const u8 as *const libc::c_char,
            189 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).W = gsl_matrix_alloc(p, 2 as libc::c_int as size_t);
    if ((*state).W).is_null() {
        gsl_error(
            b"failed to allocate space for W\0" as *const u8 as *const libc::c_char,
            b"subspace2D.c\0" as *const u8 as *const libc::c_char,
            195 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).JQ = gsl_matrix_alloc(n, p);
    if ((*state).JQ).is_null() {
        gsl_error(
            b"failed to allocate space for JQ\0" as *const u8 as *const libc::c_char,
            b"subspace2D.c\0" as *const u8 as *const libc::c_char,
            201 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).tau = gsl_vector_alloc(2 as libc::c_int as size_t);
    if ((*state).tau).is_null() {
        gsl_error(
            b"failed to allocate space for tau\0" as *const u8 as *const libc::c_char,
            b"subspace2D.c\0" as *const u8 as *const libc::c_char,
            207 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).subg = gsl_vector_alloc(2 as libc::c_int as size_t);
    if ((*state).subg).is_null() {
        gsl_error(
            b"failed to allocate space for subg\0" as *const u8 as *const libc::c_char,
            b"subspace2D.c\0" as *const u8 as *const libc::c_char,
            213 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .subB = gsl_matrix_alloc(2 as libc::c_int as size_t, 2 as libc::c_int as size_t);
    if ((*state).subB).is_null() {
        gsl_error(
            b"failed to allocate space for subB\0" as *const u8 as *const libc::c_char,
            b"subspace2D.c\0" as *const u8 as *const libc::c_char,
            219 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).perm = gsl_permutation_alloc(2 as libc::c_int as size_t);
    if ((*state).perm).is_null() {
        gsl_error(
            b"failed to allocate space for perm\0" as *const u8 as *const libc::c_char,
            b"subspace2D.c\0" as *const u8 as *const libc::c_char,
            225 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).poly_p = gsl_poly_complex_workspace_alloc(5 as libc::c_int as size_t);
    if ((*state).poly_p).is_null() {
        gsl_error(
            b"failed to allocate space for poly workspace\0" as *const u8
                as *const libc::c_char,
            b"subspace2D.c\0" as *const u8 as *const libc::c_char,
            231 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).n = n;
    (*state).p = p;
    (*state).rank = 0 as libc::c_int as size_t;
    (*state).params = *par;
    return state as *mut libc::c_void;
}
unsafe extern "C" fn subspace2D_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut subspace2D_state_t = vstate as *mut subspace2D_state_t;
    if !((*state).dx_gn).is_null() {
        gsl_vector_free((*state).dx_gn);
    }
    if !((*state).dx_sd).is_null() {
        gsl_vector_free((*state).dx_sd);
    }
    if !((*state).workp).is_null() {
        gsl_vector_free((*state).workp);
    }
    if !((*state).workn).is_null() {
        gsl_vector_free((*state).workn);
    }
    if !((*state).W).is_null() {
        gsl_matrix_free((*state).W);
    }
    if !((*state).JQ).is_null() {
        gsl_matrix_free((*state).JQ);
    }
    if !((*state).tau).is_null() {
        gsl_vector_free((*state).tau);
    }
    if !((*state).subg).is_null() {
        gsl_vector_free((*state).subg);
    }
    if !((*state).subB).is_null() {
        gsl_matrix_free((*state).subB);
    }
    if !((*state).perm).is_null() {
        gsl_permutation_free((*state).perm);
    }
    if !((*state).poly_p).is_null() {
        gsl_poly_complex_workspace_free((*state).poly_p);
    }
    free(state as *mut libc::c_void);
}
unsafe extern "C" fn subspace2D_init(
    mut vtrust_state: *const libc::c_void,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn subspace2D_preloop(
    mut vtrust_state: *const libc::c_void,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut trust_state: *const gsl_multifit_nlinear_trust_state = vtrust_state
        as *const gsl_multifit_nlinear_trust_state;
    let mut state: *mut subspace2D_state_t = vstate as *mut subspace2D_state_t;
    let mut v: gsl_vector_view = gsl_vector_view {
        vector: gsl_vector {
            size: 0,
            stride: 0,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0,
        },
    };
    let mut work_data: [libc::c_double; 2] = [0.; 2];
    let mut work: gsl_vector_view = gsl_vector_view_array(
        work_data.as_mut_ptr(),
        2 as libc::c_int as size_t,
    );
    let mut signum: libc::c_int = 0;
    status = subspace2D_calc_gn(trust_state, (*state).dx_gn);
    if status != 0 {
        return status;
    }
    status = subspace2D_calc_sd(trust_state, (*state).dx_sd, state);
    if status != 0 {
        return status;
    }
    (*state).norm_Dgn = scaled_enorm((*trust_state).diag, (*state).dx_gn);
    (*state).norm_Dsd = scaled_enorm((*trust_state).diag, (*state).dx_sd);
    v = gsl_matrix_column((*state).W, 0 as libc::c_int as size_t);
    gsl_vector_memcpy(&mut v.vector, (*state).dx_sd);
    gsl_vector_mul(&mut v.vector, (*trust_state).diag);
    if (*state).norm_Dsd != 0 as libc::c_int as libc::c_double {
        gsl_vector_scale(&mut v.vector, 1.0f64 / (*state).norm_Dsd);
    }
    v = gsl_matrix_column((*state).W, 1 as libc::c_int as size_t);
    gsl_vector_memcpy(&mut v.vector, (*state).dx_gn);
    gsl_vector_mul(&mut v.vector, (*trust_state).diag);
    if (*state).norm_Dgn != 0 as libc::c_int as libc::c_double {
        gsl_vector_scale(&mut v.vector, 1.0f64 / (*state).norm_Dgn);
    }
    gsl_linalg_QRPT_decomp(
        (*state).W,
        (*state).tau,
        (*state).perm,
        &mut signum,
        &mut work.vector,
    );
    (*state).rank = gsl_linalg_QRPT_rank((*state).W, -1.0f64);
    if (*state).rank == 2 as libc::c_int as libc::c_ulong {
        let p: size_t = (*state).p;
        let mut i: size_t = 0;
        let mut JQ: gsl_matrix_view = gsl_matrix_submatrix(
            (*state).JQ,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            (*state).n,
            if (2 as libc::c_int as libc::c_ulong) < p {
                2 as libc::c_int as libc::c_ulong
            } else {
                p
            },
        );
        let mut B00: libc::c_double = 0.;
        let mut B10: libc::c_double = 0.;
        let mut B11: libc::c_double = 0.;
        let mut g0: libc::c_double = 0.;
        let mut g1: libc::c_double = 0.;
        gsl_vector_memcpy((*state).workp, (*trust_state).g);
        gsl_vector_div((*state).workp, (*trust_state).diag);
        gsl_linalg_QR_QTvec((*state).W, (*state).tau, (*state).workp);
        g0 = gsl_vector_get((*state).workp, 0 as libc::c_int as size_t);
        g1 = gsl_vector_get((*state).workp, 1 as libc::c_int as size_t);
        gsl_vector_set((*state).subg, 0 as libc::c_int as size_t, g0);
        gsl_vector_set((*state).subg, 1 as libc::c_int as size_t, g1);
        gsl_matrix_memcpy((*state).JQ, (*trust_state).J);
        i = 0 as libc::c_int as size_t;
        while i < p {
            let mut c: gsl_vector_view = gsl_matrix_column((*state).JQ, i);
            let mut di: libc::c_double = gsl_vector_get((*trust_state).diag, i);
            gsl_vector_scale(&mut c.vector, 1.0f64 / di);
            i = i.wrapping_add(1);
            i;
        }
        gsl_linalg_QR_matQ((*state).W, (*state).tau, (*state).JQ);
        gsl_blas_dsyrk(
            CblasLower,
            CblasTrans,
            1.0f64,
            &mut JQ.matrix,
            0.0f64,
            (*state).subB,
        );
        B00 = gsl_matrix_get(
            (*state).subB,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
        );
        B10 = gsl_matrix_get(
            (*state).subB,
            1 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
        );
        B11 = gsl_matrix_get(
            (*state).subB,
            1 as libc::c_int as size_t,
            1 as libc::c_int as size_t,
        );
        (*state).trB = B00 + B11;
        (*state).detB = B00 * B11 - B10 * B10;
        (*state).normg = gsl_blas_dnrm2((*state).subg);
        (*state)
            .term0 = (B10 * B10 + B11 * B11) * g0 * g0
            - 2 as libc::c_int as libc::c_double * B10 * (B00 + B11) * g0 * g1
            + (B00 * B00 + B10 * B10) * g1 * g1;
        (*state)
            .term1 = B11 * g0 * g0
            + g1 * (B00 * g1 - 2 as libc::c_int as libc::c_double * B10 * g0);
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn subspace2D_step(
    mut vtrust_state: *const libc::c_void,
    delta: libc::c_double,
    mut dx: *mut gsl_vector,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut trust_state: *const gsl_multifit_nlinear_trust_state = vtrust_state
        as *const gsl_multifit_nlinear_trust_state;
    let mut state: *mut subspace2D_state_t = vstate as *mut subspace2D_state_t;
    if (*state).norm_Dgn <= delta {
        gsl_vector_memcpy(dx, (*state).dx_gn);
    } else if (*state).rank < 2 as libc::c_int as libc::c_ulong {
        gsl_vector_memcpy(dx, (*state).dx_sd);
        gsl_vector_scale(dx, delta / (*state).norm_Dsd);
    } else {
        let mut status: libc::c_int = 0;
        let delta_sq: libc::c_double = delta * delta;
        let mut u: libc::c_double = (*state).normg / delta;
        let mut a: [libc::c_double; 5] = [0.; 5];
        let mut z: [libc::c_double; 8] = [0.; 8];
        a[0 as libc::c_int
            as usize] = (*state).detB * (*state).detB - (*state).term0 / delta_sq;
        a[1 as libc::c_int
            as usize] = 2 as libc::c_int as libc::c_double * (*state).detB * (*state).trB
            - 2 as libc::c_int as libc::c_double * (*state).term1 / delta_sq;
        a[2 as libc::c_int
            as usize] = (*state).trB * (*state).trB
            + 2 as libc::c_int as libc::c_double * (*state).detB - u * u;
        a[3 as libc::c_int as usize] = 2 as libc::c_int as libc::c_double * (*state).trB;
        a[4 as libc::c_int as usize] = 1.0f64;
        status = gsl_poly_complex_solve(
            a.as_mut_ptr(),
            5 as libc::c_int as size_t,
            (*state).poly_p,
            z.as_mut_ptr(),
        );
        if status == GSL_SUCCESS as libc::c_int {
            let mut i: size_t = 0;
            let mut min: libc::c_double = 0.0f64;
            let mut mini: libc::c_int = -(1 as libc::c_int);
            let mut x_data: [libc::c_double; 2] = [0.; 2];
            let mut x: gsl_vector_view = gsl_vector_view_array(
                x_data.as_mut_ptr(),
                2 as libc::c_int as size_t,
            );
            i = 0 as libc::c_int as size_t;
            while i < 4 as libc::c_int as libc::c_ulong {
                let mut cost: libc::c_double = 0.;
                let mut normx: libc::c_double = 0.;
                status = subspace2D_solution(
                    z[(2 as libc::c_int as libc::c_ulong).wrapping_mul(i) as usize],
                    &mut x.vector,
                    state,
                );
                if !(status != GSL_SUCCESS as libc::c_int) {
                    normx = gsl_blas_dnrm2(&mut x.vector);
                    if !(normx == 0.0f64) {
                        gsl_vector_scale(&mut x.vector, delta / normx);
                        cost = subspace2D_objective(&mut x.vector, state);
                        if mini < 0 as libc::c_int || cost < min {
                            mini = i as libc::c_int;
                            min = cost;
                        }
                    }
                }
                i = i.wrapping_add(1);
                i;
            }
            if mini < 0 as libc::c_int {
                return GSL_FAILURE as libc::c_int
            } else {
                subspace2D_solution(
                    z[(2 as libc::c_int * mini) as usize],
                    &mut x.vector,
                    state,
                );
                gsl_vector_set_zero(dx);
                gsl_vector_set(
                    dx,
                    0 as libc::c_int as size_t,
                    gsl_vector_get(&mut x.vector, 0 as libc::c_int as size_t),
                );
                gsl_vector_set(
                    dx,
                    1 as libc::c_int as size_t,
                    gsl_vector_get(&mut x.vector, 1 as libc::c_int as size_t),
                );
                gsl_linalg_QR_Qvec((*state).W, (*state).tau, dx);
                gsl_vector_div(dx, (*trust_state).diag);
            }
        } else {
            gsl_error(
                b"gsl_poly_complex_solve failed\0" as *const u8 as *const libc::c_char,
                b"subspace2D.c\0" as *const u8 as *const libc::c_char,
                552 as libc::c_int,
                status,
            );
            return status;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn subspace2D_preduction(
    mut vtrust_state: *const libc::c_void,
    mut dx: *const gsl_vector,
    mut pred: *mut libc::c_double,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut trust_state: *const gsl_multifit_nlinear_trust_state = vtrust_state
        as *const gsl_multifit_nlinear_trust_state;
    let mut state: *mut subspace2D_state_t = vstate as *mut subspace2D_state_t;
    *pred = quadratic_preduction((*trust_state).f, (*trust_state).J, dx, (*state).workn);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn subspace2D_solution(
    lambda: libc::c_double,
    mut x: *mut gsl_vector,
    mut state: *mut subspace2D_state_t,
) -> libc::c_int {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut C_data: [libc::c_double; 4] = [0.; 4];
    let mut C: gsl_matrix_view = gsl_matrix_view_array(
        C_data.as_mut_ptr(),
        2 as libc::c_int as size_t,
        2 as libc::c_int as size_t,
    );
    let mut B00: libc::c_double = gsl_matrix_get(
        (*state).subB,
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
    );
    let mut B10: libc::c_double = gsl_matrix_get(
        (*state).subB,
        1 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
    );
    let mut B11: libc::c_double = gsl_matrix_get(
        (*state).subB,
        1 as libc::c_int as size_t,
        1 as libc::c_int as size_t,
    );
    gsl_matrix_set(
        &mut C.matrix,
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
        B00 + lambda,
    );
    gsl_matrix_set(
        &mut C.matrix,
        1 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
        B10,
    );
    gsl_matrix_set(
        &mut C.matrix,
        0 as libc::c_int as size_t,
        1 as libc::c_int as size_t,
        B10,
    );
    gsl_matrix_set(
        &mut C.matrix,
        1 as libc::c_int as size_t,
        1 as libc::c_int as size_t,
        B11 + lambda,
    );
    gsl_linalg_mcholesky_decomp(&mut C.matrix, (*state).perm, 0 as *mut gsl_vector);
    gsl_linalg_mcholesky_solve(&mut C.matrix, (*state).perm, (*state).subg, x);
    gsl_vector_scale(x, -1.0f64);
    return status;
}
unsafe extern "C" fn subspace2D_objective(
    mut x: *const gsl_vector,
    mut state: *mut subspace2D_state_t,
) -> libc::c_double {
    let mut f: libc::c_double = 0.;
    let mut y_data: [libc::c_double; 2] = [0.; 2];
    let mut y: gsl_vector_view = gsl_vector_view_array(
        y_data.as_mut_ptr(),
        2 as libc::c_int as size_t,
    );
    gsl_vector_memcpy(&mut y.vector, (*state).subg);
    gsl_blas_dsymv(CblasLower, 0.5f64, (*state).subB, x, 1.0f64, &mut y.vector);
    gsl_blas_ddot(x, &mut y.vector, &mut f);
    return f;
}
unsafe extern "C" fn subspace2D_calc_gn(
    mut trust_state: *const gsl_multifit_nlinear_trust_state,
    mut dx: *mut gsl_vector,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut params: *const gsl_multifit_nlinear_parameters = (*trust_state).params;
    status = ((*(*params).solver).init)
        .unwrap()(trust_state as *const libc::c_void, (*trust_state).solver_state);
    if status != 0 {
        return status;
    }
    status = ((*(*params).solver).presolve)
        .unwrap()(
        0.0f64,
        trust_state as *const libc::c_void,
        (*trust_state).solver_state,
    );
    if status != 0 {
        return status;
    }
    status = ((*(*params).solver).solve)
        .unwrap()(
        (*trust_state).f,
        dx,
        trust_state as *const libc::c_void,
        (*trust_state).solver_state,
    );
    if status != 0 {
        return status;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn subspace2D_calc_sd(
    mut trust_state: *const gsl_multifit_nlinear_trust_state,
    mut dx: *mut gsl_vector,
    mut state: *mut subspace2D_state_t,
) -> libc::c_int {
    let mut norm_Dinvg: libc::c_double = 0.;
    let mut norm_JDinv2g: libc::c_double = 0.;
    let mut alpha: libc::c_double = 0.;
    let mut u: libc::c_double = 0.;
    gsl_vector_memcpy((*state).workp, (*trust_state).g);
    gsl_vector_div((*state).workp, (*trust_state).diag);
    norm_Dinvg = gsl_blas_dnrm2((*state).workp);
    gsl_vector_div((*state).workp, (*trust_state).diag);
    gsl_blas_dgemv(
        CblasNoTrans,
        1.0f64,
        (*trust_state).J,
        (*state).workp,
        0.0f64,
        (*state).workn,
    );
    norm_JDinv2g = gsl_blas_dnrm2((*state).workn);
    u = norm_Dinvg / norm_JDinv2g;
    alpha = u * u;
    gsl_vector_memcpy(dx, (*state).workp);
    gsl_vector_scale(dx, -alpha);
    return GSL_SUCCESS as libc::c_int;
}
static mut subspace2D_type: gsl_multifit_nlinear_trs = unsafe {
    {
        let mut init = gsl_multifit_nlinear_trs {
            name: b"2D-subspace\0" as *const u8 as *const libc::c_char,
            alloc: Some(
                subspace2D_alloc
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        size_t,
                        size_t,
                    ) -> *mut libc::c_void,
            ),
            init: Some(
                subspace2D_init
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            preloop: Some(
                subspace2D_preloop
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            step: Some(
                subspace2D_step
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        libc::c_double,
                        *mut gsl_vector,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            preduction: Some(
                subspace2D_preduction
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const gsl_vector,
                        *mut libc::c_double,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            free: Some(subspace2D_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        };
        init
    }
};
pub static mut gsl_multifit_nlinear_trs_subspace2D: *const gsl_multifit_nlinear_trs = unsafe {
    &subspace2D_type as *const gsl_multifit_nlinear_trs
};
