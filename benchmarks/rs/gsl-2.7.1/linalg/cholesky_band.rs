use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_set_zero(v: *mut gsl_vector);
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_max(v: *const gsl_vector) -> libc::c_double;
    fn cblas_dtbsv(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: libc::c_int,
        K: libc::c_int,
        A: *const libc::c_double,
        lda: libc::c_int,
        X: *mut libc::c_double,
        incX: libc::c_int,
    );
    fn gsl_matrix_submatrix(
        m: *mut gsl_matrix,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_view;
    fn gsl_matrix_column(m: *mut gsl_matrix, j: size_t) -> _gsl_vector_view;
    fn gsl_matrix_subdiagonal(m: *mut gsl_matrix, k: size_t) -> _gsl_vector_view;
    fn gsl_matrix_subrow(
        m: *mut gsl_matrix,
        i: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
    fn gsl_matrix_const_column(
        m: *const gsl_matrix,
        j: size_t,
    ) -> _gsl_vector_const_view;
    fn gsl_matrix_const_subrow(
        m: *const gsl_matrix,
        i: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_const_view;
    fn gsl_matrix_const_subcolumn(
        m: *const gsl_matrix,
        j: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_const_view;
    fn gsl_matrix_memcpy(dest: *mut gsl_matrix, src: *const gsl_matrix) -> libc::c_int;
    fn gsl_linalg_cholesky_invert(cholesky: *mut gsl_matrix) -> libc::c_int;
    fn gsl_blas_idamax(X: *const gsl_vector) -> CBLAS_INDEX_t;
    fn gsl_blas_dasum(X: *const gsl_vector) -> libc::c_double;
    fn gsl_blas_dsyr(
        Uplo: CBLAS_UPLO_t,
        alpha: libc::c_double,
        X: *const gsl_vector,
        A: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_blas_dscal(alpha: libc::c_double, X: *mut gsl_vector);
    fn gsl_linalg_invnorm1(
        N: size_t,
        Ainvx: Option::<
            unsafe extern "C" fn(
                CBLAS_TRANSPOSE_t,
                *mut gsl_vector,
                *mut libc::c_void,
            ) -> libc::c_int,
        >,
        params: *mut libc::c_void,
        Ainvnorm: *mut libc::c_double,
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
pub struct _gsl_vector_const_view {
    pub vector: gsl_vector,
}
pub type gsl_vector_const_view = _gsl_vector_const_view;
pub type CBLAS_ORDER = libc::c_uint;
pub const CblasColMajor: CBLAS_ORDER = 102;
pub const CblasRowMajor: CBLAS_ORDER = 101;
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
pub type CBLAS_INDEX_t = size_t;
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
#[inline]
unsafe extern "C" fn gsl_matrix_ptr(
    mut m: *mut gsl_matrix,
    i: size_t,
    j: size_t,
) -> *mut libc::c_double {
    return ((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
pub unsafe extern "C" fn gsl_linalg_cholesky_band_decomp(
    mut A: *mut gsl_matrix,
) -> libc::c_int {
    let N: size_t = (*A).size1;
    let ndiag: size_t = (*A).size2;
    if ndiag > N {
        gsl_error(
            b"invalid matrix dimensions\0" as *const u8 as *const libc::c_char,
            b"cholesky_band.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let p: size_t = ndiag.wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let kld: libc::c_int = (if 1 as libc::c_int as libc::c_ulong > p {
            1 as libc::c_int as libc::c_ulong
        } else {
            p
        }) as libc::c_int;
        let mut j: size_t = 0;
        if ndiag > 1 as libc::c_int as libc::c_ulong {
            let mut Anorm: libc::c_double = cholesky_band_norm1(A);
            gsl_matrix_set(
                A,
                N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                p,
                Anorm,
            );
        }
        j = 0 as libc::c_int as size_t;
        while j < N {
            let mut ajj: libc::c_double = gsl_matrix_get(
                A,
                j,
                0 as libc::c_int as size_t,
            );
            let mut lenv: size_t = 0;
            if ajj <= 0.0f64 {
                gsl_error(
                    b"matrix is not positive definite\0" as *const u8
                        as *const libc::c_char,
                    b"cholesky_band.c\0" as *const u8 as *const libc::c_char,
                    104 as libc::c_int,
                    GSL_EDOM as libc::c_int,
                );
                return GSL_EDOM as libc::c_int;
            }
            ajj = sqrt(ajj);
            gsl_matrix_set(A, j, 0 as libc::c_int as size_t, ajj);
            lenv = if p
                < N.wrapping_sub(j).wrapping_sub(1 as libc::c_int as libc::c_ulong)
            {
                p
            } else {
                N.wrapping_sub(j).wrapping_sub(1 as libc::c_int as libc::c_ulong)
            };
            if lenv > 0 as libc::c_int as libc::c_ulong {
                let mut v: gsl_vector_view = gsl_matrix_subrow(
                    A,
                    j,
                    1 as libc::c_int as size_t,
                    lenv,
                );
                let mut m: gsl_matrix_view = gsl_matrix_submatrix(
                    A,
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    0 as libc::c_int as size_t,
                    lenv,
                    lenv,
                );
                gsl_blas_dscal(1.0f64 / ajj, &mut v.vector);
                m.matrix.tda = kld as size_t;
                gsl_blas_dsyr(CblasUpper, -1.0f64, &mut v.vector, &mut m.matrix);
            }
            j = j.wrapping_add(1);
            j;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_cholesky_band_solve(
    mut LLT: *const gsl_matrix,
    mut b: *const gsl_vector,
    mut x: *mut gsl_vector,
) -> libc::c_int {
    if (*LLT).size1 != (*b).size {
        gsl_error(
            b"matrix size must match b size\0" as *const u8 as *const libc::c_char,
            b"cholesky_band.c\0" as *const u8 as *const libc::c_char,
            137 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*LLT).size1 != (*x).size {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8
                as *const libc::c_char,
            b"cholesky_band.c\0" as *const u8 as *const libc::c_char,
            141 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        gsl_vector_memcpy(x, b);
        status = gsl_linalg_cholesky_band_svx(LLT, x);
        return status;
    };
}
pub unsafe extern "C" fn gsl_linalg_cholesky_band_svx(
    mut LLT: *const gsl_matrix,
    mut x: *mut gsl_vector,
) -> libc::c_int {
    if (*LLT).size1 != (*x).size {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8
                as *const libc::c_char,
            b"cholesky_band.c\0" as *const u8 as *const libc::c_char,
            161 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        cblas_dtbsv(
            CblasColMajor,
            CblasLower,
            CblasNoTrans,
            CblasNonUnit,
            (*LLT).size1 as libc::c_int,
            ((*LLT).size2).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                as libc::c_int,
            (*LLT).data,
            (*LLT).tda as libc::c_int,
            (*x).data,
            (*x).stride as libc::c_int,
        );
        cblas_dtbsv(
            CblasColMajor,
            CblasLower,
            CblasTrans,
            CblasNonUnit,
            (*LLT).size1 as libc::c_int,
            ((*LLT).size2).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                as libc::c_int,
            (*LLT).data,
            (*LLT).tda as libc::c_int,
            (*x).data,
            (*x).stride as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_cholesky_band_solvem(
    mut LLT: *const gsl_matrix,
    mut B: *const gsl_matrix,
    mut X: *mut gsl_matrix,
) -> libc::c_int {
    if (*LLT).size1 != (*B).size1 {
        gsl_error(
            b"LLT size1 must match B size1\0" as *const u8 as *const libc::c_char,
            b"cholesky_band.c\0" as *const u8 as *const libc::c_char,
            186 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*LLT).size1 != (*X).size1 {
        gsl_error(
            b"LLT size1 must match solution size1\0" as *const u8 as *const libc::c_char,
            b"cholesky_band.c\0" as *const u8 as *const libc::c_char,
            190 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*B).size2 != (*X).size2 {
        gsl_error(
            b"B size2 must match X size2\0" as *const u8 as *const libc::c_char,
            b"cholesky_band.c\0" as *const u8 as *const libc::c_char,
            194 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        gsl_matrix_memcpy(X, B);
        status = gsl_linalg_cholesky_band_svxm(LLT, X);
        return status;
    };
}
pub unsafe extern "C" fn gsl_linalg_cholesky_band_svxm(
    mut LLT: *const gsl_matrix,
    mut X: *mut gsl_matrix,
) -> libc::c_int {
    if (*LLT).size1 != (*X).size1 {
        gsl_error(
            b"LLT size1 must match solution size1\0" as *const u8 as *const libc::c_char,
            b"cholesky_band.c\0" as *const u8 as *const libc::c_char,
            214 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let nrhs: size_t = (*X).size2;
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < nrhs {
            let mut xj: gsl_vector_view = gsl_matrix_column(X, j);
            status = gsl_linalg_cholesky_band_svx(LLT, &mut xj.vector);
            if status != 0 {
                return status;
            }
            j = j.wrapping_add(1);
            j;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_cholesky_band_invert(
    mut LLT: *const gsl_matrix,
    mut Ainv: *mut gsl_matrix,
) -> libc::c_int {
    if (*Ainv).size1 != (*Ainv).size2 {
        gsl_error(
            b"Ainv must be square\0" as *const u8 as *const libc::c_char,
            b"cholesky_band.c\0" as *const u8 as *const libc::c_char,
            240 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*LLT).size1 != (*Ainv).size1 {
        gsl_error(
            b"cholesky matrix has different dimensions from Ainv\0" as *const u8
                as *const libc::c_char,
            b"cholesky_band.c\0" as *const u8 as *const libc::c_char,
            244 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        status = gsl_linalg_cholesky_band_unpack(LLT, Ainv);
        if status != 0 {
            return status;
        }
        status = gsl_linalg_cholesky_invert(Ainv);
        if status != 0 {
            return status;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_cholesky_band_unpack(
    mut LLT: *const gsl_matrix,
    mut L: *mut gsl_matrix,
) -> libc::c_int {
    let N: size_t = (*LLT).size1;
    if N != (*L).size1 {
        gsl_error(
            b"L matrix does not match LLT dimensions\0" as *const u8
                as *const libc::c_char,
            b"cholesky_band.c\0" as *const u8 as *const libc::c_char,
            271 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*L).size1 != (*L).size2 {
        gsl_error(
            b"L matrix is not square\0" as *const u8 as *const libc::c_char,
            b"cholesky_band.c\0" as *const u8 as *const libc::c_char,
            275 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else {
        let p: size_t = ((*LLT).size2).wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < p.wrapping_add(1 as libc::c_int as libc::c_ulong) {
            let v: gsl_vector_const_view = gsl_matrix_const_subcolumn(
                LLT,
                i,
                0 as libc::c_int as size_t,
                N.wrapping_sub(i),
            );
            let mut w: gsl_vector_view = gsl_matrix_subdiagonal(L, i);
            gsl_vector_memcpy(&mut w.vector, &v.vector);
            i = i.wrapping_add(1);
            i;
        }
        i = p.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while i < N {
            let mut w_0: gsl_vector_view = gsl_matrix_subdiagonal(L, i);
            gsl_vector_set_zero(&mut w_0.vector);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_cholesky_band_rcond(
    mut LLT: *const gsl_matrix,
    mut rcond: *mut libc::c_double,
    mut work: *mut gsl_vector,
) -> libc::c_int {
    let N: size_t = (*LLT).size1;
    if (*work).size != (3 as libc::c_int as libc::c_ulong).wrapping_mul(N) {
        gsl_error(
            b"work vector must have length 3*N\0" as *const u8 as *const libc::c_char,
            b"cholesky_band.c\0" as *const u8 as *const libc::c_char,
            308 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let ndiag: size_t = (*LLT).size2;
        let mut Anorm: libc::c_double = 0.;
        let mut Ainvnorm: libc::c_double = 0.;
        if ndiag == 1 as libc::c_int as libc::c_ulong {
            let v: gsl_vector_const_view = gsl_matrix_const_column(
                LLT,
                0 as libc::c_int as size_t,
            );
            Anorm = gsl_vector_max(&v.vector);
            Anorm = Anorm * Anorm;
        } else {
            Anorm = gsl_matrix_get(
                LLT,
                N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ndiag.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
        }
        *rcond = 0.0f64;
        if Anorm == 0.0f64 {
            return GSL_SUCCESS as libc::c_int;
        }
        status = gsl_linalg_invnorm1(
            N,
            Some(
                cholesky_band_Ainv
                    as unsafe extern "C" fn(
                        CBLAS_TRANSPOSE_t,
                        *mut gsl_vector,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            LLT as *mut libc::c_void,
            &mut Ainvnorm,
            work,
        );
        if status != 0 {
            return status;
        }
        if Ainvnorm != 0.0f64 {
            *rcond = 1.0f64 / Anorm / Ainvnorm;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_cholesky_band_scale(
    mut A: *const gsl_matrix,
    mut S: *mut gsl_vector,
) -> libc::c_int {
    let N: size_t = (*A).size1;
    let ndiag: size_t = (*A).size2;
    if ndiag > N {
        gsl_error(
            b"invalid matrix dimensions\0" as *const u8 as *const libc::c_char,
            b"cholesky_band.c\0" as *const u8 as *const libc::c_char,
            372 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if N != (*S).size {
        gsl_error(
            b"S must have length N\0" as *const u8 as *const libc::c_char,
            b"cholesky_band.c\0" as *const u8 as *const libc::c_char,
            376 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let mut Aii: libc::c_double = gsl_matrix_get(
                A,
                i,
                0 as libc::c_int as size_t,
            );
            if Aii <= 0.0f64 {
                gsl_vector_set(S, i, 1.0f64);
            } else {
                gsl_vector_set(S, i, 1.0f64 / sqrt(Aii));
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_cholesky_band_scale_apply(
    mut A: *mut gsl_matrix,
    mut S: *const gsl_vector,
) -> libc::c_int {
    let N: size_t = (*A).size1;
    let ndiag: size_t = (*A).size2;
    if ndiag > N {
        gsl_error(
            b"invalid matrix dimensions\0" as *const u8 as *const libc::c_char,
            b"cholesky_band.c\0" as *const u8 as *const libc::c_char,
            417 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if N != (*S).size {
        gsl_error(
            b"S must have length N\0" as *const u8 as *const libc::c_char,
            b"cholesky_band.c\0" as *const u8 as *const libc::c_char,
            421 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < N {
            let mut sj: libc::c_double = gsl_vector_get(S, j);
            i = j;
            while i < (if N < j.wrapping_add(ndiag) { N } else { j.wrapping_add(ndiag) })
            {
                let mut si: libc::c_double = gsl_vector_get(S, i);
                let mut ptr: *mut libc::c_double = gsl_matrix_ptr(
                    A,
                    j,
                    i.wrapping_sub(j),
                );
                *ptr *= sj * si;
                i = i.wrapping_add(1);
                i;
            }
            j = j.wrapping_add(1);
            j;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn cholesky_band_norm1(mut A: *const gsl_matrix) -> libc::c_double {
    let N: size_t = (*A).size1;
    let ndiag: size_t = (*A).size2;
    let mut value: libc::c_double = 0.;
    if ndiag == 1 as libc::c_int as libc::c_ulong {
        let v: gsl_vector_const_view = gsl_matrix_const_column(
            A,
            0 as libc::c_int as size_t,
        );
        let mut idx: CBLAS_INDEX_t = gsl_blas_idamax(&v.vector);
        value = gsl_vector_get(&v.vector, idx);
    } else {
        let mut j: size_t = 0;
        value = 0.0f64;
        j = 0 as libc::c_int as size_t;
        while j < N {
            let mut ncol: size_t = if ndiag < N.wrapping_sub(j) {
                ndiag
            } else {
                N.wrapping_sub(j)
            };
            let v_0: gsl_vector_const_view = gsl_matrix_const_subrow(
                A,
                j,
                0 as libc::c_int as size_t,
                ncol,
            );
            let mut sum: libc::c_double = gsl_blas_dasum(&v_0.vector);
            let mut k: size_t = 0;
            let mut l: size_t = 0;
            k = j;
            l = 1 as libc::c_int as size_t;
            while k > 0 as libc::c_int as libc::c_ulong && l < ndiag {
                k = k.wrapping_sub(1);
                let fresh0 = l;
                l = l.wrapping_add(1);
                let mut Akl: libc::c_double = gsl_matrix_get(A, k, fresh0);
                sum += fabs(Akl);
            }
            value = if value > sum { value } else { sum };
            j = j.wrapping_add(1);
            j;
        }
    }
    return value;
}
unsafe extern "C" fn cholesky_band_Ainv(
    mut TransA: CBLAS_TRANSPOSE_t,
    mut x: *mut gsl_vector,
    mut params: *mut libc::c_void,
) -> libc::c_int {
    let mut LLT: *mut gsl_matrix = params as *mut gsl_matrix;
    cblas_dtbsv(
        CblasColMajor,
        CblasLower,
        CblasNoTrans,
        CblasNonUnit,
        (*LLT).size1 as libc::c_int,
        ((*LLT).size2).wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        (*LLT).data,
        (*LLT).tda as libc::c_int,
        (*x).data,
        (*x).stride as libc::c_int,
    );
    cblas_dtbsv(
        CblasColMajor,
        CblasLower,
        CblasTrans,
        CblasNonUnit,
        (*LLT).size1 as libc::c_int,
        ((*LLT).size2).wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        (*LLT).data,
        (*LLT).tda as libc::c_int,
        (*x).data,
        (*x).stride as libc::c_int,
    );
    return GSL_SUCCESS as libc::c_int;
}
