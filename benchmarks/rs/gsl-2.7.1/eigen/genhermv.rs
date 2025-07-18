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
    fn gsl_matrix_complex_column(
        m: *mut gsl_matrix_complex,
        j: size_t,
    ) -> _gsl_vector_complex_view;
    fn gsl_eigen_genherm_standardize(
        A: *mut gsl_matrix_complex,
        B: *const gsl_matrix_complex,
    ) -> libc::c_int;
    fn gsl_eigen_hermv_alloc(n: size_t) -> *mut gsl_eigen_hermv_workspace;
    fn gsl_eigen_hermv_free(w: *mut gsl_eigen_hermv_workspace);
    fn gsl_eigen_hermv(
        A: *mut gsl_matrix_complex,
        eval: *mut gsl_vector,
        evec: *mut gsl_matrix_complex,
        w: *mut gsl_eigen_hermv_workspace,
    ) -> libc::c_int;
    fn gsl_blas_dznrm2(X: *const gsl_vector_complex) -> libc::c_double;
    fn gsl_blas_ztrsm(
        Side: CBLAS_SIDE_t,
        Uplo: CBLAS_UPLO_t,
        TransA: CBLAS_TRANSPOSE_t,
        Diag: CBLAS_DIAG_t,
        alpha: gsl_complex,
        A: *const gsl_matrix_complex,
        B: *mut gsl_matrix_complex,
    ) -> libc::c_int;
    fn gsl_blas_zdscal(alpha: libc::c_double, X: *mut gsl_vector_complex);
    fn gsl_linalg_complex_cholesky_decomp(A: *mut gsl_matrix_complex) -> libc::c_int;
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
pub struct gsl_complex {
    pub dat: [libc::c_double; 2],
}
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
pub struct gsl_block_complex_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
}
pub type gsl_block_complex = gsl_block_complex_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_complex {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block_complex,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_vector_complex_view {
    pub vector: gsl_vector_complex,
}
pub type gsl_vector_complex_view = _gsl_vector_complex_view;
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
pub type CBLAS_SIDE = libc::c_uint;
pub const CblasRight: CBLAS_SIDE = 142;
pub const CblasLeft: CBLAS_SIDE = 141;
pub type CBLAS_TRANSPOSE_t = CBLAS_TRANSPOSE;
pub type CBLAS_UPLO_t = CBLAS_UPLO;
pub type CBLAS_DIAG_t = CBLAS_DIAG;
pub type CBLAS_SIDE_t = CBLAS_SIDE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_complex {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block_complex,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_eigen_hermv_workspace {
    pub size: size_t,
    pub d: *mut libc::c_double,
    pub sd: *mut libc::c_double,
    pub tau: *mut libc::c_double,
    pub gc: *mut libc::c_double,
    pub gs: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_eigen_genhermv_workspace {
    pub size: size_t,
    pub hermv_workspace_p: *mut gsl_eigen_hermv_workspace,
}
#[inline]
unsafe extern "C" fn gsl_complex_rect(
    mut x: libc::c_double,
    mut y: libc::c_double,
) -> gsl_complex {
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    z.dat[0 as libc::c_int as usize] = x;
    z.dat[1 as libc::c_int as usize] = y;
    return z;
}
pub unsafe extern "C" fn gsl_eigen_genhermv_alloc(
    n: size_t,
) -> *mut gsl_eigen_genhermv_workspace {
    let mut w: *mut gsl_eigen_genhermv_workspace = 0
        as *mut gsl_eigen_genhermv_workspace;
    if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix dimension must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"genhermv.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_eigen_genhermv_workspace;
    }
    w = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gsl_eigen_genhermv_workspace>() as libc::c_ulong,
    ) as *mut gsl_eigen_genhermv_workspace;
    if w.is_null() {
        gsl_error(
            b"failed to allocate space for workspace\0" as *const u8
                as *const libc::c_char,
            b"genhermv.c\0" as *const u8 as *const libc::c_char,
            66 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_eigen_genhermv_workspace;
    }
    (*w).size = n;
    (*w).hermv_workspace_p = gsl_eigen_hermv_alloc(n);
    if ((*w).hermv_workspace_p).is_null() {
        gsl_eigen_genhermv_free(w);
        gsl_error(
            b"failed to allocate space for hermv workspace\0" as *const u8
                as *const libc::c_char,
            b"genhermv.c\0" as *const u8 as *const libc::c_char,
            75 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_eigen_genhermv_workspace;
    }
    return w;
}
pub unsafe extern "C" fn gsl_eigen_genhermv_free(
    mut w: *mut gsl_eigen_genhermv_workspace,
) {
    if w.is_null() {
        return;
    }
    if !((*w).hermv_workspace_p).is_null() {
        gsl_eigen_hermv_free((*w).hermv_workspace_p);
    }
    free(w as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_eigen_genhermv(
    mut A: *mut gsl_matrix_complex,
    mut B: *mut gsl_matrix_complex,
    mut eval: *mut gsl_vector,
    mut evec: *mut gsl_matrix_complex,
    mut w: *mut gsl_eigen_genhermv_workspace,
) -> libc::c_int {
    let N: size_t = (*A).size1;
    if N != (*A).size2 {
        gsl_error(
            b"matrix must be square to compute eigenvalues\0" as *const u8
                as *const libc::c_char,
            b"genhermv.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != (*B).size1 || N != (*B).size2 {
        gsl_error(
            b"B matrix dimensions must match A\0" as *const u8 as *const libc::c_char,
            b"genhermv.c\0" as *const u8 as *const libc::c_char,
            130 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*eval).size != N {
        gsl_error(
            b"eigenvalue vector must match matrix size\0" as *const u8
                as *const libc::c_char,
            b"genhermv.c\0" as *const u8 as *const libc::c_char,
            134 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*evec).size1 != (*evec).size2 {
        gsl_error(
            b"eigenvector matrix must be square\0" as *const u8 as *const libc::c_char,
            b"genhermv.c\0" as *const u8 as *const libc::c_char,
            138 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*evec).size1 != N {
        gsl_error(
            b"eigenvector matrix has wrong size\0" as *const u8 as *const libc::c_char,
            b"genhermv.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*w).size != N {
        gsl_error(
            b"matrix size does not match workspace\0" as *const u8
                as *const libc::c_char,
            b"genhermv.c\0" as *const u8 as *const libc::c_char,
            146 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut s: libc::c_int = 0;
        s = gsl_linalg_complex_cholesky_decomp(B);
        if s != GSL_SUCCESS as libc::c_int {
            return s;
        }
        gsl_eigen_genherm_standardize(A, B);
        s = gsl_eigen_hermv(A, eval, evec, (*w).hermv_workspace_p);
        if s != GSL_SUCCESS as libc::c_int {
            return s;
        }
        gsl_blas_ztrsm(
            CblasLeft,
            CblasLower,
            CblasConjTrans,
            CblasNonUnit,
            gsl_complex_rect(1.0f64, 0.0f64),
            B,
            evec,
        );
        genhermv_normalize_eigenvectors(evec);
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn genhermv_normalize_eigenvectors(mut evec: *mut gsl_matrix_complex) {
    let N: size_t = (*evec).size1;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut vi: gsl_vector_complex_view = gsl_matrix_complex_column(evec, i);
        let mut scale: libc::c_double = 1.0f64 / gsl_blas_dznrm2(&mut vi.vector);
        gsl_blas_zdscal(scale, &mut vi.vector);
        i = i.wrapping_add(1);
        i;
    }
}
