use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_subvector(
        v: *mut gsl_vector,
        i: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_div(a: *mut gsl_vector, b: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_scale(a: *mut gsl_vector, x: libc::c_double) -> libc::c_int;
    fn gsl_matrix_submatrix(
        m: *mut gsl_matrix,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_view;
    fn gsl_matrix_subrow(
        m: *mut gsl_matrix,
        i: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
    fn gsl_matrix_subcolumn(
        m: *mut gsl_matrix,
        j: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
    fn gsl_matrix_const_diagonal(m: *const gsl_matrix) -> _gsl_vector_const_view;
    fn gsl_matrix_const_subcolumn(
        m: *const gsl_matrix,
        j: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_const_view;
    fn gsl_blas_ddot(
        X: *const gsl_vector,
        Y: *const gsl_vector,
        result: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_blas_dasum(X: *const gsl_vector) -> libc::c_double;
    fn gsl_blas_dgemv(
        TransA: CBLAS_TRANSPOSE_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        X: *const gsl_vector,
        beta: libc::c_double,
        Y: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_blas_dtrsv(
        Uplo: CBLAS_UPLO_t,
        TransA: CBLAS_TRANSPOSE_t,
        Diag: CBLAS_DIAG_t,
        A: *const gsl_matrix,
        X: *mut gsl_vector,
    ) -> libc::c_int;
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
pub unsafe extern "C" fn gsl_linalg_ldlt_decomp(mut A: *mut gsl_matrix) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if M != N {
        gsl_error(
            b"LDLT decomposition requires square matrix\0" as *const u8
                as *const libc::c_char,
            b"ldlt.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else {
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut a00: libc::c_double = 0.;
        let mut anorm: libc::c_double = 0.;
        let mut work: gsl_vector_view = gsl_vector_view {
            vector: gsl_vector {
                size: 0,
                stride: 0,
                data: 0 as *mut libc::c_double,
                block: 0 as *mut gsl_block,
                owner: 0,
            },
        };
        let mut v: gsl_vector_view = gsl_vector_view {
            vector: gsl_vector {
                size: 0,
                stride: 0,
                data: 0 as *mut libc::c_double,
                block: 0 as *mut gsl_block,
                owner: 0,
            },
        };
        if N == 1 as libc::c_int as libc::c_ulong {
            return GSL_SUCCESS as libc::c_int;
        }
        anorm = ldlt_norm1(A);
        a00 = gsl_matrix_get(A, 0 as libc::c_int as size_t, 0 as libc::c_int as size_t);
        if a00 == 0.0f64 {
            gsl_error(
                b"matrix is singular\0" as *const u8 as *const libc::c_char,
                b"ldlt.c\0" as *const u8 as *const libc::c_char,
                78 as libc::c_int,
                GSL_EDOM as libc::c_int,
            );
            return GSL_EDOM as libc::c_int;
        }
        v = gsl_matrix_subcolumn(
            A,
            0 as libc::c_int as size_t,
            1 as libc::c_int as size_t,
            N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        gsl_vector_scale(&mut v.vector, 1.0f64 / a00);
        work = gsl_matrix_subrow(
            A,
            0 as libc::c_int as size_t,
            1 as libc::c_int as size_t,
            N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        j = 1 as libc::c_int as size_t;
        while j < N {
            let mut w: gsl_vector_view = gsl_vector_subvector(
                &mut work.vector,
                0 as libc::c_int as size_t,
                j,
            );
            let mut ajj: libc::c_double = gsl_matrix_get(A, j, j);
            let mut dval: libc::c_double = 0.;
            i = 0 as libc::c_int as size_t;
            while i < j {
                let mut aii: libc::c_double = gsl_matrix_get(A, i, i);
                let mut aji: libc::c_double = gsl_matrix_get(A, j, i);
                gsl_vector_set(&mut w.vector, i, aji * aii);
                i = i.wrapping_add(1);
                i;
            }
            v = gsl_matrix_subrow(A, j, 0 as libc::c_int as size_t, j);
            gsl_blas_ddot(&mut v.vector, &mut w.vector, &mut dval);
            ajj -= dval;
            if ajj == 0.0f64 {
                gsl_error(
                    b"matrix is singular\0" as *const u8 as *const libc::c_char,
                    b"ldlt.c\0" as *const u8 as *const libc::c_char,
                    106 as libc::c_int,
                    GSL_EDOM as libc::c_int,
                );
                return GSL_EDOM as libc::c_int;
            }
            gsl_matrix_set(A, j, j, ajj);
            if j < N.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                let mut ajjinv: libc::c_double = 1.0f64 / ajj;
                let mut m: gsl_matrix_view = gsl_matrix_submatrix(
                    A,
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    0 as libc::c_int as size_t,
                    N.wrapping_sub(j).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    j,
                );
                v = gsl_matrix_subcolumn(
                    A,
                    j,
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    N.wrapping_sub(j).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                gsl_blas_dgemv(
                    CblasNoTrans,
                    -ajjinv,
                    &mut m.matrix,
                    &mut w.vector,
                    ajjinv,
                    &mut v.vector,
                );
            }
            j = j.wrapping_add(1);
            j;
        }
        gsl_matrix_set(
            A,
            0 as libc::c_int as size_t,
            N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            anorm,
        );
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_ldlt_solve(
    mut LDLT: *const gsl_matrix,
    mut b: *const gsl_vector,
    mut x: *mut gsl_vector,
) -> libc::c_int {
    if (*LDLT).size1 != (*LDLT).size2 {
        gsl_error(
            b"LDLT matrix must be square\0" as *const u8 as *const libc::c_char,
            b"ldlt.c\0" as *const u8 as *const libc::c_char,
            134 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*LDLT).size1 != (*b).size {
        gsl_error(
            b"matrix size must match b size\0" as *const u8 as *const libc::c_char,
            b"ldlt.c\0" as *const u8 as *const libc::c_char,
            138 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*LDLT).size2 != (*x).size {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8
                as *const libc::c_char,
            b"ldlt.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        gsl_vector_memcpy(x, b);
        status = gsl_linalg_ldlt_svx(LDLT, x);
        return status;
    };
}
pub unsafe extern "C" fn gsl_linalg_ldlt_svx(
    mut LDLT: *const gsl_matrix,
    mut x: *mut gsl_vector,
) -> libc::c_int {
    if (*LDLT).size1 != (*LDLT).size2 {
        gsl_error(
            b"LDLT matrix must be square\0" as *const u8 as *const libc::c_char,
            b"ldlt.c\0" as *const u8 as *const libc::c_char,
            163 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*LDLT).size2 != (*x).size {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8
                as *const libc::c_char,
            b"ldlt.c\0" as *const u8 as *const libc::c_char,
            167 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let diag: gsl_vector_const_view = gsl_matrix_const_diagonal(LDLT);
        gsl_blas_dtrsv(CblasLower, CblasNoTrans, CblasUnit, LDLT, x);
        gsl_vector_div(x, &diag.vector);
        gsl_blas_dtrsv(CblasLower, CblasTrans, CblasUnit, LDLT, x);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_ldlt_rcond(
    mut LDLT: *const gsl_matrix,
    mut rcond: *mut libc::c_double,
    mut work: *mut gsl_vector,
) -> libc::c_int {
    let M: size_t = (*LDLT).size1;
    let N: size_t = (*LDLT).size2;
    if M != N {
        gsl_error(
            b"LDLT matrix must be square\0" as *const u8 as *const libc::c_char,
            b"ldlt.c\0" as *const u8 as *const libc::c_char,
            195 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*work).size != (3 as libc::c_int as libc::c_ulong).wrapping_mul(N) {
        gsl_error(
            b"work vector must have length 3*N\0" as *const u8 as *const libc::c_char,
            b"ldlt.c\0" as *const u8 as *const libc::c_char,
            199 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut Anorm: libc::c_double = 0.;
        let mut Ainvnorm: libc::c_double = 0.;
        if N == 1 as libc::c_int as libc::c_ulong {
            Anorm = fabs(
                gsl_matrix_get(
                    LDLT,
                    0 as libc::c_int as size_t,
                    0 as libc::c_int as size_t,
                ),
            );
        } else {
            Anorm = gsl_matrix_get(
                LDLT,
                0 as libc::c_int as size_t,
                N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
        }
        *rcond = 0.0f64;
        if Anorm == 0.0f64 {
            return GSL_SUCCESS as libc::c_int;
        }
        status = gsl_linalg_invnorm1(
            N,
            Some(
                ldlt_Ainv
                    as unsafe extern "C" fn(
                        CBLAS_TRANSPOSE_t,
                        *mut gsl_vector,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            LDLT as *mut libc::c_void,
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
unsafe extern "C" fn ldlt_norm1(mut A: *const gsl_matrix) -> libc::c_double {
    let N: size_t = (*A).size1;
    let mut max: libc::c_double = 0.0f64;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < N {
        let v: gsl_vector_const_view = gsl_matrix_const_subcolumn(
            A,
            j,
            j,
            N.wrapping_sub(j),
        );
        let mut sum: libc::c_double = gsl_blas_dasum(&v.vector);
        i = 0 as libc::c_int as size_t;
        while i < j {
            let mut Aij: libc::c_double = gsl_matrix_get(A, i, j);
            sum += fabs(Aij);
            i = i.wrapping_add(1);
            i;
        }
        if sum > max {
            max = sum;
        }
        j = j.wrapping_add(1);
        j;
    }
    return max;
}
unsafe extern "C" fn ldlt_Ainv(
    mut TransA: CBLAS_TRANSPOSE_t,
    mut x: *mut gsl_vector,
    mut params: *mut libc::c_void,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut A: *mut gsl_matrix = params as *mut gsl_matrix;
    let diag: gsl_vector_const_view = gsl_matrix_const_diagonal(A);
    status = gsl_blas_dtrsv(CblasLower, CblasNoTrans, CblasUnit, A, x);
    if status != 0 {
        return status;
    }
    gsl_vector_div(x, &diag.vector);
    status = gsl_blas_dtrsv(CblasLower, CblasTrans, CblasUnit, A, x);
    if status != 0 {
        return status;
    }
    return GSL_SUCCESS as libc::c_int;
}
