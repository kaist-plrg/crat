use ::libc;
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_complex_subvector(
        base: *mut gsl_vector_complex,
        i: size_t,
        n: size_t,
    ) -> _gsl_vector_complex_view;
    fn gsl_vector_complex_memcpy(
        dest: *mut gsl_vector_complex,
        src: *const gsl_vector_complex,
    ) -> libc::c_int;
    fn gsl_vector_complex_sub(
        a: *mut gsl_vector_complex,
        b: *const gsl_vector_complex,
    ) -> libc::c_int;
    fn gsl_vector_complex_add_constant(
        a: *mut gsl_vector_complex,
        x: gsl_complex,
    ) -> libc::c_int;
    fn gsl_matrix_complex_submatrix(
        m: *mut gsl_matrix_complex,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_complex_view;
    fn gsl_matrix_complex_column(
        m: *mut gsl_matrix_complex,
        j: size_t,
    ) -> _gsl_vector_complex_view;
    fn gsl_matrix_complex_diagonal(
        m: *mut gsl_matrix_complex,
    ) -> _gsl_vector_complex_view;
    fn gsl_matrix_complex_const_submatrix(
        m: *const gsl_matrix_complex,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_complex_const_view;
    fn gsl_matrix_complex_memcpy(
        dest: *mut gsl_matrix_complex,
        src: *const gsl_matrix_complex,
    ) -> libc::c_int;
    fn gsl_matrix_complex_tricpy(
        Uplo: CBLAS_UPLO_t,
        Diag: CBLAS_DIAG_t,
        dest: *mut gsl_matrix_complex,
        src: *const gsl_matrix_complex,
    ) -> libc::c_int;
    fn gsl_matrix_complex_conjtrans_memcpy(
        dest: *mut gsl_matrix_complex,
        src: *const gsl_matrix_complex,
    ) -> libc::c_int;
    fn gsl_matrix_complex_sub(
        a: *mut gsl_matrix_complex,
        b: *const gsl_matrix_complex,
    ) -> libc::c_int;
    fn gsl_blas_zgemv(
        TransA: CBLAS_TRANSPOSE_t,
        alpha: gsl_complex,
        A: *const gsl_matrix_complex,
        X: *const gsl_vector_complex,
        beta: gsl_complex,
        Y: *mut gsl_vector_complex,
    ) -> libc::c_int;
    fn gsl_blas_ztrmv(
        Uplo: CBLAS_UPLO_t,
        TransA: CBLAS_TRANSPOSE_t,
        Diag: CBLAS_DIAG_t,
        A: *const gsl_matrix_complex,
        X: *mut gsl_vector_complex,
    ) -> libc::c_int;
    fn gsl_blas_ztrsv(
        Uplo: CBLAS_UPLO_t,
        TransA: CBLAS_TRANSPOSE_t,
        Diag: CBLAS_DIAG_t,
        A: *const gsl_matrix_complex,
        X: *mut gsl_vector_complex,
    ) -> libc::c_int;
    fn gsl_blas_zgemm(
        TransA: CBLAS_TRANSPOSE_t,
        TransB: CBLAS_TRANSPOSE_t,
        alpha: gsl_complex,
        A: *const gsl_matrix_complex,
        B: *const gsl_matrix_complex,
        beta: gsl_complex,
        C: *mut gsl_matrix_complex,
    ) -> libc::c_int;
    fn gsl_blas_ztrmm(
        Side: CBLAS_SIDE_t,
        Uplo: CBLAS_UPLO_t,
        TransA: CBLAS_TRANSPOSE_t,
        Diag: CBLAS_DIAG_t,
        alpha: gsl_complex,
        A: *const gsl_matrix_complex,
        B: *mut gsl_matrix_complex,
    ) -> libc::c_int;
    fn gsl_linalg_complex_householder_transform(
        v: *mut gsl_vector_complex,
    ) -> gsl_complex;
    fn gsl_complex_mul(a: gsl_complex, b: gsl_complex) -> gsl_complex;
    fn gsl_complex_conjugate(z: gsl_complex) -> gsl_complex;
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
pub struct _gsl_matrix_complex_view {
    pub matrix: gsl_matrix_complex,
}
pub type gsl_matrix_complex_view = _gsl_matrix_complex_view;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_matrix_complex_const_view {
    pub matrix: gsl_matrix_complex,
}
pub type gsl_matrix_complex_const_view = _gsl_matrix_complex_const_view;
#[inline]
unsafe extern "C" fn gsl_vector_complex_get(
    mut v: *const gsl_vector_complex,
    i: size_t,
) -> gsl_complex {
    return *(&mut *((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*v).stride)
                as isize,
        ) as *mut libc::c_double as *mut gsl_complex);
}
#[inline]
unsafe extern "C" fn gsl_vector_complex_ptr(
    mut v: *mut gsl_vector_complex,
    i: size_t,
) -> *mut gsl_complex {
    return &mut *((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*v).stride)
                as isize,
        ) as *mut libc::c_double as *mut gsl_complex;
}
#[inline]
unsafe extern "C" fn gsl_matrix_complex_get(
    mut m: *const gsl_matrix_complex,
    i: size_t,
    j: size_t,
) -> gsl_complex {
    return *(((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        ) as *mut gsl_complex);
}
#[inline]
unsafe extern "C" fn gsl_matrix_complex_ptr(
    mut m: *mut gsl_matrix_complex,
    i: size_t,
    j: size_t,
) -> *mut gsl_complex {
    return ((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        ) as *mut gsl_complex;
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
pub unsafe extern "C" fn gsl_linalg_complex_QR_decomp_r(
    mut A: *mut gsl_matrix_complex,
    mut T: *mut gsl_matrix_complex,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if M < N {
        gsl_error(
            b"M must be >= N\0" as *const u8 as *const libc::c_char,
            b"rqrc.c\0" as *const u8 as *const libc::c_char,
            73 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*T).size1 != (*T).size2 {
        gsl_error(
            b"T matrix must be square\0" as *const u8 as *const libc::c_char,
            b"rqrc.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*T).size1 != N {
        gsl_error(
            b"T matrix does not match dimensions of A\0" as *const u8
                as *const libc::c_char,
            b"rqrc.c\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if N == 1 as libc::c_int as libc::c_ulong {
        let mut T00: *mut gsl_complex = gsl_matrix_complex_ptr(
            T,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
        );
        let mut v: gsl_vector_complex_view = gsl_matrix_complex_column(
            A,
            0 as libc::c_int as size_t,
        );
        *T00 = gsl_linalg_complex_householder_transform(&mut v.vector);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let N1: size_t = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
        let N2: size_t = N.wrapping_sub(N1);
        let M2: size_t = M.wrapping_sub(N1);
        let mut A11: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
            A,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N1,
            N1,
        );
        let mut A12: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
            A,
            0 as libc::c_int as size_t,
            N1,
            N1,
            N2,
        );
        let mut A21: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
            A,
            N1,
            0 as libc::c_int as size_t,
            M2,
            N1,
        );
        let mut A22: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
            A,
            N1,
            N1,
            M2,
            N2,
        );
        let mut T11: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
            T,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N1,
            N1,
        );
        let mut T12: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
            T,
            0 as libc::c_int as size_t,
            N1,
            N1,
            N2,
        );
        let mut T22: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
            T,
            N1,
            N1,
            N2,
            N2,
        );
        let mut m: gsl_matrix_complex_view = gsl_matrix_complex_view {
            matrix: gsl_matrix_complex {
                size1: 0,
                size2: 0,
                tda: 0,
                data: 0 as *mut libc::c_double,
                block: 0 as *mut gsl_block_complex,
                owner: 0,
            },
        };
        m = gsl_matrix_complex_submatrix(
            A,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            M,
            N1,
        );
        status = gsl_linalg_complex_QR_decomp_r(&mut m.matrix, &mut T11.matrix);
        if status != 0 {
            return status;
        }
        gsl_matrix_complex_memcpy(&mut T12.matrix, &mut A12.matrix);
        gsl_blas_ztrmm(
            CblasLeft,
            CblasLower,
            CblasConjTrans,
            CblasUnit,
            gsl_complex_rect(1.0f64, 0.0f64),
            &mut A11.matrix,
            &mut T12.matrix,
        );
        gsl_blas_zgemm(
            CblasConjTrans,
            CblasNoTrans,
            gsl_complex_rect(1.0f64, 0.0f64),
            &mut A21.matrix,
            &mut A22.matrix,
            gsl_complex_rect(1.0f64, 0.0f64),
            &mut T12.matrix,
        );
        gsl_blas_ztrmm(
            CblasLeft,
            CblasUpper,
            CblasConjTrans,
            CblasNonUnit,
            gsl_complex_rect(1.0f64, 0.0f64),
            &mut T11.matrix,
            &mut T12.matrix,
        );
        gsl_blas_zgemm(
            CblasNoTrans,
            CblasNoTrans,
            gsl_complex_rect(-1.0f64, 0.0f64),
            &mut A21.matrix,
            &mut T12.matrix,
            gsl_complex_rect(1.0f64, 0.0f64),
            &mut A22.matrix,
        );
        gsl_blas_ztrmm(
            CblasLeft,
            CblasLower,
            CblasNoTrans,
            CblasUnit,
            gsl_complex_rect(1.0f64, 0.0f64),
            &mut A11.matrix,
            &mut T12.matrix,
        );
        gsl_matrix_complex_sub(&mut A12.matrix, &mut T12.matrix);
        status = gsl_linalg_complex_QR_decomp_r(&mut A22.matrix, &mut T22.matrix);
        if status != 0 {
            return status;
        }
        m = gsl_matrix_complex_submatrix(
            &mut A21.matrix,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N2,
            N1,
        );
        gsl_matrix_complex_conjtrans_memcpy(&mut T12.matrix, &mut m.matrix);
        m = gsl_matrix_complex_submatrix(A, N1, N1, N2, N2);
        gsl_blas_ztrmm(
            CblasRight,
            CblasLower,
            CblasNoTrans,
            CblasUnit,
            gsl_complex_rect(1.0f64, 0.0f64),
            &mut m.matrix,
            &mut T12.matrix,
        );
        if M > N {
            let mut V31: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
                A,
                N,
                0 as libc::c_int as size_t,
                M.wrapping_sub(N),
                N1,
            );
            let mut V32: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
                A,
                N,
                N1,
                M.wrapping_sub(N),
                N2,
            );
            gsl_blas_zgemm(
                CblasConjTrans,
                CblasNoTrans,
                gsl_complex_rect(1.0f64, 0.0f64),
                &mut V31.matrix,
                &mut V32.matrix,
                gsl_complex_rect(1.0f64, 0.0f64),
                &mut T12.matrix,
            );
        }
        gsl_blas_ztrmm(
            CblasLeft,
            CblasUpper,
            CblasNoTrans,
            CblasNonUnit,
            gsl_complex_rect(-1.0f64, 0.0f64),
            &mut T11.matrix,
            &mut T12.matrix,
        );
        gsl_blas_ztrmm(
            CblasRight,
            CblasUpper,
            CblasNoTrans,
            CblasNonUnit,
            gsl_complex_rect(1.0f64, 0.0f64),
            &mut T22.matrix,
            &mut T12.matrix,
        );
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_complex_QR_solve_r(
    mut QR: *const gsl_matrix_complex,
    mut T: *const gsl_matrix_complex,
    mut b: *const gsl_vector_complex,
    mut x: *mut gsl_vector_complex,
) -> libc::c_int {
    let N: size_t = (*QR).size2;
    if (*QR).size1 != N {
        gsl_error(
            b"QR matrix must be square\0" as *const u8 as *const libc::c_char,
            b"rqrc.c\0" as *const u8 as *const libc::c_char,
            220 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*T).size1 != (*QR).size1 || (*T).size2 != (*QR).size2 {
        gsl_error(
            b"T matrix must be N-by-N\0" as *const u8 as *const libc::c_char,
            b"rqrc.c\0" as *const u8 as *const libc::c_char,
            224 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if N != (*b).size {
        gsl_error(
            b"matrix size must match b size\0" as *const u8 as *const libc::c_char,
            b"rqrc.c\0" as *const u8 as *const libc::c_char,
            228 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if N != (*x).size {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8
                as *const libc::c_char,
            b"rqrc.c\0" as *const u8 as *const libc::c_char,
            232 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        gsl_vector_complex_memcpy(x, b);
        gsl_blas_ztrmv(CblasLower, CblasConjTrans, CblasUnit, QR, x);
        gsl_blas_ztrmv(CblasUpper, CblasConjTrans, CblasNonUnit, T, x);
        gsl_blas_ztrmv(CblasLower, CblasNoTrans, CblasUnit, QR, x);
        i = 0 as libc::c_int as size_t;
        while i < N {
            let mut xi: *mut gsl_complex = gsl_vector_complex_ptr(x, i);
            let mut bi: gsl_complex = gsl_vector_complex_get(b, i);
            (*xi)
                .dat[0 as libc::c_int
                as usize] = bi.dat[0 as libc::c_int as usize]
                - (*xi).dat[0 as libc::c_int as usize];
            (*xi)
                .dat[1 as libc::c_int
                as usize] = bi.dat[1 as libc::c_int as usize]
                - (*xi).dat[1 as libc::c_int as usize];
            i = i.wrapping_add(1);
            i;
        }
        gsl_blas_ztrsv(CblasUpper, CblasNoTrans, CblasNonUnit, QR, x);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_complex_QR_lssolve_r(
    mut QR: *const gsl_matrix_complex,
    mut T: *const gsl_matrix_complex,
    mut b: *const gsl_vector_complex,
    mut x: *mut gsl_vector_complex,
    mut work: *mut gsl_vector_complex,
) -> libc::c_int {
    let M: size_t = (*QR).size1;
    let N: size_t = (*QR).size2;
    if M < N {
        gsl_error(
            b"QR matrix must have M >= N\0" as *const u8 as *const libc::c_char,
            b"rqrc.c\0" as *const u8 as *const libc::c_char,
            290 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*T).size1 != N || (*T).size2 != N {
        gsl_error(
            b"T matrix must be N-by-N\0" as *const u8 as *const libc::c_char,
            b"rqrc.c\0" as *const u8 as *const libc::c_char,
            294 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if M != (*b).size {
        gsl_error(
            b"matrix size must match b size\0" as *const u8 as *const libc::c_char,
            b"rqrc.c\0" as *const u8 as *const libc::c_char,
            298 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if M != (*x).size {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8
                as *const libc::c_char,
            b"rqrc.c\0" as *const u8 as *const libc::c_char,
            302 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if N != (*work).size {
        gsl_error(
            b"matrix size must match work size\0" as *const u8 as *const libc::c_char,
            b"rqrc.c\0" as *const u8 as *const libc::c_char,
            306 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let R: gsl_matrix_complex_const_view = gsl_matrix_complex_const_submatrix(
            QR,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N,
            N,
        );
        let mut x1: gsl_vector_complex_view = gsl_vector_complex_subvector(
            x,
            0 as libc::c_int as size_t,
            N,
        );
        gsl_vector_complex_memcpy(x, b);
        gsl_linalg_complex_QR_QHvec_r(QR, T, x, work);
        gsl_blas_ztrsv(
            CblasUpper,
            CblasNoTrans,
            CblasNonUnit,
            &R.matrix,
            &mut x1.vector,
        );
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_complex_QR_unpack_r(
    mut QR: *const gsl_matrix_complex,
    mut T: *const gsl_matrix_complex,
    mut Q: *mut gsl_matrix_complex,
    mut R: *mut gsl_matrix_complex,
) -> libc::c_int {
    let M: size_t = (*QR).size1;
    let N: size_t = (*QR).size2;
    if M < N {
        gsl_error(
            b"M must be >= N\0" as *const u8 as *const libc::c_char,
            b"rqrc.c\0" as *const u8 as *const libc::c_char,
            348 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*Q).size1 != M || (*Q).size2 != M {
        gsl_error(
            b"Q matrix must be M-by-M\0" as *const u8 as *const libc::c_char,
            b"rqrc.c\0" as *const u8 as *const libc::c_char,
            352 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*R).size1 != N || (*R).size2 != N {
        gsl_error(
            b"R matrix must be N-by-N\0" as *const u8 as *const libc::c_char,
            b"rqrc.c\0" as *const u8 as *const libc::c_char,
            356 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*T).size1 != N || (*T).size2 != N {
        gsl_error(
            b"T matrix must be N-by-N\0" as *const u8 as *const libc::c_char,
            b"rqrc.c\0" as *const u8 as *const libc::c_char,
            360 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let RV: gsl_matrix_complex_const_view = gsl_matrix_complex_const_submatrix(
            QR,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N,
            N,
        );
        let mut Q1: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
            Q,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            M,
            N,
        );
        let mut m: gsl_matrix_complex_view = gsl_matrix_complex_view {
            matrix: gsl_matrix_complex {
                size1: 0,
                size2: 0,
                tda: 0,
                data: 0 as *mut libc::c_double,
                block: 0 as *mut gsl_block_complex,
                owner: 0,
            },
        };
        m = gsl_matrix_complex_submatrix(
            Q,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N,
            N,
        );
        gsl_matrix_complex_tricpy(CblasUpper, CblasNonUnit, &mut m.matrix, T);
        gsl_matrix_complex_tricpy(CblasLower, CblasUnit, &mut m.matrix, &RV.matrix);
        if M > N {
            let tmp: gsl_matrix_complex_const_view = gsl_matrix_complex_const_submatrix(
                QR,
                N,
                0 as libc::c_int as size_t,
                M.wrapping_sub(N),
                N,
            );
            m = gsl_matrix_complex_submatrix(
                Q,
                N,
                0 as libc::c_int as size_t,
                M.wrapping_sub(N),
                N,
            );
            gsl_matrix_complex_memcpy(&mut m.matrix, &tmp.matrix);
        }
        unpack_Q1(&mut Q1.matrix);
        if M > N {
            let mut Q2: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
                Q,
                0 as libc::c_int as size_t,
                N,
                M,
                M.wrapping_sub(N),
            );
            unpack_Q2(QR, T, &mut Q2.matrix);
        }
        gsl_matrix_complex_tricpy(CblasUpper, CblasNonUnit, R, &RV.matrix);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_complex_QR_QHvec_r(
    mut QR: *const gsl_matrix_complex,
    mut T: *const gsl_matrix_complex,
    mut b: *mut gsl_vector_complex,
    mut work: *mut gsl_vector_complex,
) -> libc::c_int {
    let M: size_t = (*QR).size1;
    let N: size_t = (*QR).size2;
    if M < N {
        gsl_error(
            b"M must be >= N\0" as *const u8 as *const libc::c_char,
            b"rqrc.c\0" as *const u8 as *const libc::c_char,
            427 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*T).size1 != N || (*T).size2 != N {
        gsl_error(
            b"T matrix must be N-by-N\0" as *const u8 as *const libc::c_char,
            b"rqrc.c\0" as *const u8 as *const libc::c_char,
            431 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*b).size != M {
        gsl_error(
            b"b vector must have length M\0" as *const u8 as *const libc::c_char,
            b"rqrc.c\0" as *const u8 as *const libc::c_char,
            435 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*work).size != N {
        gsl_error(
            b"workspace must be length N\0" as *const u8 as *const libc::c_char,
            b"rqrc.c\0" as *const u8 as *const libc::c_char,
            439 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let V1: gsl_matrix_complex_const_view = gsl_matrix_complex_const_submatrix(
            QR,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N,
            N,
        );
        let mut b1: gsl_vector_complex_view = gsl_vector_complex_subvector(
            b,
            0 as libc::c_int as size_t,
            N,
        );
        let mut b2: gsl_vector_complex_view = gsl_vector_complex_view {
            vector: gsl_vector_complex {
                size: 0,
                stride: 0,
                data: 0 as *mut libc::c_double,
                block: 0 as *mut gsl_block_complex,
                owner: 0,
            },
        };
        gsl_vector_complex_memcpy(work, &mut b1.vector);
        gsl_blas_ztrmv(CblasLower, CblasConjTrans, CblasUnit, &V1.matrix, work);
        if M > N {
            let V2: gsl_matrix_complex_const_view = gsl_matrix_complex_const_submatrix(
                QR,
                N,
                0 as libc::c_int as size_t,
                M.wrapping_sub(N),
                N,
            );
            b2 = gsl_vector_complex_subvector(b, N, M.wrapping_sub(N));
            gsl_blas_zgemv(
                CblasConjTrans,
                gsl_complex_rect(1.0f64, 0.0f64),
                &V2.matrix,
                &mut b2.vector,
                gsl_complex_rect(1.0f64, 0.0f64),
                work,
            );
        }
        gsl_blas_ztrmv(CblasUpper, CblasConjTrans, CblasNonUnit, T, work);
        if M > N {
            let V2_0: gsl_matrix_complex_const_view = gsl_matrix_complex_const_submatrix(
                QR,
                N,
                0 as libc::c_int as size_t,
                M.wrapping_sub(N),
                N,
            );
            gsl_blas_zgemv(
                CblasNoTrans,
                gsl_complex_rect(-1.0f64, 0.0f64),
                &V2_0.matrix,
                work,
                gsl_complex_rect(1.0f64, 0.0f64),
                &mut b2.vector,
            );
        }
        gsl_blas_ztrmv(CblasLower, CblasNoTrans, CblasUnit, &V1.matrix, work);
        gsl_vector_complex_sub(&mut b1.vector, work);
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn unpack_Q1(mut Q: *mut gsl_matrix_complex) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let M: size_t = (*Q).size1;
    let N: size_t = (*Q).size2;
    let mut Q1: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
        Q,
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
        N,
        N,
    );
    let mut diag: gsl_vector_complex_view = gsl_matrix_complex_diagonal(&mut Q1.matrix);
    status = aux_ULH(&mut Q1.matrix, &mut Q1.matrix);
    if status != 0 {
        return status;
    }
    if M > N {
        let mut V2: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
            Q,
            N,
            0 as libc::c_int as size_t,
            M.wrapping_sub(N),
            N,
        );
        gsl_blas_ztrmm(
            CblasRight,
            CblasUpper,
            CblasNoTrans,
            CblasNonUnit,
            gsl_complex_rect(-1.0f64, 0.0f64),
            &mut Q1.matrix,
            &mut V2.matrix,
        );
    }
    status = aux_mLU(&mut Q1.matrix);
    if status != 0 {
        return status;
    }
    gsl_vector_complex_add_constant(&mut diag.vector, gsl_complex_rect(1.0f64, 0.0f64));
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn unpack_Q2(
    mut QR: *const gsl_matrix_complex,
    mut T: *const gsl_matrix_complex,
    mut Q: *mut gsl_matrix_complex,
) -> libc::c_int {
    let M: size_t = (*QR).size1;
    let N: size_t = (*QR).size2;
    if M <= N {
        gsl_error(
            b"M must be > N\0" as *const u8 as *const libc::c_char,
            b"rqrc.c\0" as *const u8 as *const libc::c_char,
            555 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*T).size1 != N || (*T).size2 != N {
        gsl_error(
            b"T matrix must be N-by-N\0" as *const u8 as *const libc::c_char,
            b"rqrc.c\0" as *const u8 as *const libc::c_char,
            559 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*Q).size1 != M || (*Q).size2 != M.wrapping_sub(N) {
        gsl_error(
            b"Q matrix must be M-by-(M-N)\0" as *const u8 as *const libc::c_char,
            b"rqrc.c\0" as *const u8 as *const libc::c_char,
            563 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let V1: gsl_matrix_complex_const_view = gsl_matrix_complex_const_submatrix(
            QR,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N,
            N,
        );
        let V2: gsl_matrix_complex_const_view = gsl_matrix_complex_const_submatrix(
            QR,
            N,
            0 as libc::c_int as size_t,
            M.wrapping_sub(N),
            N,
        );
        let mut Q1: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
            Q,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N,
            M.wrapping_sub(N),
        );
        let mut Q2: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
            Q,
            N,
            0 as libc::c_int as size_t,
            M.wrapping_sub(N),
            M.wrapping_sub(N),
        );
        let mut diag: gsl_vector_complex_view = gsl_matrix_complex_diagonal(
            &mut Q2.matrix,
        );
        gsl_matrix_complex_conjtrans_memcpy(&mut Q1.matrix, &V2.matrix);
        gsl_blas_ztrmm(
            CblasLeft,
            CblasUpper,
            CblasNoTrans,
            CblasNonUnit,
            gsl_complex_rect(-1.0f64, 0.0f64),
            T,
            &mut Q1.matrix,
        );
        gsl_blas_zgemm(
            CblasNoTrans,
            CblasNoTrans,
            gsl_complex_rect(1.0f64, 0.0f64),
            &V2.matrix,
            &mut Q1.matrix,
            gsl_complex_rect(0.0f64, 0.0f64),
            &mut Q2.matrix,
        );
        gsl_vector_complex_add_constant(
            &mut diag.vector,
            gsl_complex_rect(1.0f64, 0.0f64),
        );
        gsl_blas_ztrmm(
            CblasLeft,
            CblasLower,
            CblasNoTrans,
            CblasUnit,
            gsl_complex_rect(1.0f64, 0.0f64),
            &V1.matrix,
            &mut Q1.matrix,
        );
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn aux_ULH(
    mut L: *const gsl_matrix_complex,
    mut U: *mut gsl_matrix_complex,
) -> libc::c_int {
    let N: size_t = (*L).size1;
    if N != (*L).size2 {
        gsl_error(
            b"L matrix must be square\0" as *const u8 as *const libc::c_char,
            b"rqrc.c\0" as *const u8 as *const libc::c_char,
            600 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*U).size1 != N || (*U).size2 != N {
        gsl_error(
            b"U matrix must be same size as L\0" as *const u8 as *const libc::c_char,
            b"rqrc.c\0" as *const u8 as *const libc::c_char,
            604 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if N == 1 as libc::c_int as libc::c_ulong {
        return GSL_SUCCESS as libc::c_int
    } else {
        let mut status: libc::c_int = 0;
        let N1: size_t = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
        let N2: size_t = N.wrapping_sub(N1);
        let L11: gsl_matrix_complex_const_view = gsl_matrix_complex_const_submatrix(
            L,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N1,
            N1,
        );
        let L21: gsl_matrix_complex_const_view = gsl_matrix_complex_const_submatrix(
            L,
            N1,
            0 as libc::c_int as size_t,
            N2,
            N1,
        );
        let L22: gsl_matrix_complex_const_view = gsl_matrix_complex_const_submatrix(
            L,
            N1,
            N1,
            N2,
            N2,
        );
        let mut U11: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
            U,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N1,
            N1,
        );
        let mut U12: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
            U,
            0 as libc::c_int as size_t,
            N1,
            N1,
            N2,
        );
        let mut U22: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
            U,
            N1,
            N1,
            N2,
            N2,
        );
        gsl_blas_ztrmm(
            CblasRight,
            CblasLower,
            CblasConjTrans,
            CblasUnit,
            gsl_complex_rect(1.0f64, 0.0f64),
            &L22.matrix,
            &mut U12.matrix,
        );
        status = aux_ApUBH(&mut U11.matrix, &L21.matrix, &mut U12.matrix);
        if status != 0 {
            return status;
        }
        status = aux_ULH(&L11.matrix, &mut U11.matrix);
        if status != 0 {
            return status;
        }
        status = aux_ULH(&L22.matrix, &mut U22.matrix);
        if status != 0 {
            return status;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn aux_mLU(mut A: *mut gsl_matrix_complex) -> libc::c_int {
    let N: size_t = (*A).size1;
    if N != (*A).size2 {
        gsl_error(
            b"matrix must be square\0" as *const u8 as *const libc::c_char,
            b"rqrc.c\0" as *const u8 as *const libc::c_char,
            653 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N == 1 as libc::c_int as libc::c_ulong {
        let mut A00: *mut gsl_complex = gsl_matrix_complex_ptr(
            A,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
        );
        (*A00).dat[0 as libc::c_int as usize] = -(*A00).dat[0 as libc::c_int as usize];
        (*A00).dat[1 as libc::c_int as usize] = -(*A00).dat[1 as libc::c_int as usize];
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let N1: size_t = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
        let N2: size_t = N.wrapping_sub(N1);
        let mut A11: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
            A,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N1,
            N1,
        );
        let mut A12: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
            A,
            0 as libc::c_int as size_t,
            N1,
            N1,
            N2,
        );
        let mut A21: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
            A,
            N1,
            0 as libc::c_int as size_t,
            N2,
            N1,
        );
        let mut A22: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
            A,
            N1,
            N1,
            N2,
            N2,
        );
        status = aux_mLU(&mut A22.matrix);
        if status != 0 {
            return status;
        }
        gsl_blas_zgemm(
            CblasNoTrans,
            CblasNoTrans,
            gsl_complex_rect(-1.0f64, 0.0f64),
            &mut A21.matrix,
            &mut A12.matrix,
            gsl_complex_rect(1.0f64, 0.0f64),
            &mut A22.matrix,
        );
        gsl_blas_ztrmm(
            CblasLeft,
            CblasLower,
            CblasNoTrans,
            CblasUnit,
            gsl_complex_rect(-1.0f64, 0.0f64),
            &mut A11.matrix,
            &mut A12.matrix,
        );
        gsl_blas_ztrmm(
            CblasRight,
            CblasUpper,
            CblasNoTrans,
            CblasNonUnit,
            gsl_complex_rect(-1.0f64, 0.0f64),
            &mut A11.matrix,
            &mut A21.matrix,
        );
        status = aux_mLU(&mut A11.matrix);
        if status != 0 {
            return status;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn aux_ApUBH(
    mut U: *const gsl_matrix_complex,
    mut B: *const gsl_matrix_complex,
    mut A: *mut gsl_matrix_complex,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if (*U).size1 != M || (*U).size2 != M {
        gsl_error(
            b"U matrix has wrong dimensions\0" as *const u8 as *const libc::c_char,
            b"rqrc.c\0" as *const u8 as *const libc::c_char,
            705 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*B).size1 != N || (*B).size2 != M {
        gsl_error(
            b"B matrix has wrong dimensions\0" as *const u8 as *const libc::c_char,
            b"rqrc.c\0" as *const u8 as *const libc::c_char,
            709 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if M == 1 as libc::c_int as libc::c_ulong
        && N == 1 as libc::c_int as libc::c_ulong
    {
        let mut aptr: *mut gsl_complex = gsl_matrix_complex_ptr(
            A,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
        );
        let U00: gsl_complex = gsl_matrix_complex_get(
            U,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
        );
        let B00_conj: gsl_complex = gsl_complex_conjugate(
            gsl_matrix_complex_get(
                B,
                0 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
            ),
        );
        let prod: gsl_complex = gsl_complex_mul(U00, B00_conj);
        (*aptr).dat[0 as libc::c_int as usize] += prod.dat[0 as libc::c_int as usize];
        (*aptr).dat[1 as libc::c_int as usize] += prod.dat[1 as libc::c_int as usize];
        return GSL_SUCCESS as libc::c_int;
    } else if M == 1 as libc::c_int as libc::c_ulong {
        let mut U00_0: gsl_complex = gsl_matrix_complex_get(
            U,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
        );
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let mut ai: *mut gsl_complex = gsl_matrix_complex_ptr(
                A,
                0 as libc::c_int as size_t,
                i,
            );
            let mut bi: gsl_complex = gsl_matrix_complex_get(
                B,
                i,
                0 as libc::c_int as size_t,
            );
            let mut prod_0: gsl_complex = gsl_complex_mul(
                U00_0,
                gsl_complex_conjugate(bi),
            );
            (*ai).dat[0 as libc::c_int as usize]
                += prod_0.dat[0 as libc::c_int as usize];
            (*ai).dat[1 as libc::c_int as usize]
                += prod_0.dat[1 as libc::c_int as usize];
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    } else if N == 1 as libc::c_int as libc::c_ulong {
        let mut status: libc::c_int = 0;
        let M1: size_t = M.wrapping_div(2 as libc::c_int as libc::c_ulong);
        let M2: size_t = M.wrapping_sub(M1);
        let mut A11: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
            A,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            M1,
            1 as libc::c_int as size_t,
        );
        let mut A21: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
            A,
            M1,
            0 as libc::c_int as size_t,
            M2,
            1 as libc::c_int as size_t,
        );
        let U11: gsl_matrix_complex_const_view = gsl_matrix_complex_const_submatrix(
            U,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            M1,
            M1,
        );
        let U12: gsl_matrix_complex_const_view = gsl_matrix_complex_const_submatrix(
            U,
            0 as libc::c_int as size_t,
            M1,
            M1,
            M2,
        );
        let U22: gsl_matrix_complex_const_view = gsl_matrix_complex_const_submatrix(
            U,
            M1,
            M1,
            M2,
            M2,
        );
        let B11: gsl_matrix_complex_const_view = gsl_matrix_complex_const_submatrix(
            B,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            1 as libc::c_int as size_t,
            M1,
        );
        let B12: gsl_matrix_complex_const_view = gsl_matrix_complex_const_submatrix(
            B,
            0 as libc::c_int as size_t,
            M1,
            1 as libc::c_int as size_t,
            M2,
        );
        gsl_blas_zgemm(
            CblasNoTrans,
            CblasConjTrans,
            gsl_complex_rect(1.0f64, 0.0f64),
            &U12.matrix,
            &B12.matrix,
            gsl_complex_rect(1.0f64, 0.0f64),
            &mut A11.matrix,
        );
        status = aux_ApUBH(&U11.matrix, &B11.matrix, &mut A11.matrix);
        if status != 0 {
            return status;
        }
        status = aux_ApUBH(&U22.matrix, &B12.matrix, &mut A21.matrix);
        if status != 0 {
            return status;
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut status_0: libc::c_int = 0;
        let M1_0: size_t = M.wrapping_div(2 as libc::c_int as libc::c_ulong);
        let M2_0: size_t = M.wrapping_sub(M1_0);
        let N1: size_t = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
        let N2: size_t = N.wrapping_sub(N1);
        let mut A11_0: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
            A,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            M1_0,
            N1,
        );
        let mut A12: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
            A,
            0 as libc::c_int as size_t,
            N1,
            M1_0,
            N2,
        );
        let mut A21_0: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
            A,
            M1_0,
            0 as libc::c_int as size_t,
            M2_0,
            N1,
        );
        let mut A22: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
            A,
            M1_0,
            N1,
            M2_0,
            N2,
        );
        let U11_0: gsl_matrix_complex_const_view = gsl_matrix_complex_const_submatrix(
            U,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            M1_0,
            M1_0,
        );
        let U12_0: gsl_matrix_complex_const_view = gsl_matrix_complex_const_submatrix(
            U,
            0 as libc::c_int as size_t,
            M1_0,
            M1_0,
            M2_0,
        );
        let U22_0: gsl_matrix_complex_const_view = gsl_matrix_complex_const_submatrix(
            U,
            M1_0,
            M1_0,
            M2_0,
            M2_0,
        );
        let B11_0: gsl_matrix_complex_const_view = gsl_matrix_complex_const_submatrix(
            B,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N1,
            M1_0,
        );
        let B12_0: gsl_matrix_complex_const_view = gsl_matrix_complex_const_submatrix(
            B,
            0 as libc::c_int as size_t,
            M1_0,
            N1,
            M2_0,
        );
        let B21: gsl_matrix_complex_const_view = gsl_matrix_complex_const_submatrix(
            B,
            N1,
            0 as libc::c_int as size_t,
            N2,
            M1_0,
        );
        let B22: gsl_matrix_complex_const_view = gsl_matrix_complex_const_submatrix(
            B,
            N1,
            M1_0,
            N2,
            M2_0,
        );
        status_0 = aux_ApUBH(&U11_0.matrix, &B11_0.matrix, &mut A11_0.matrix);
        if status_0 != 0 {
            return status_0;
        }
        gsl_blas_zgemm(
            CblasNoTrans,
            CblasConjTrans,
            gsl_complex_rect(1.0f64, 0.0f64),
            &U12_0.matrix,
            &B12_0.matrix,
            gsl_complex_rect(1.0f64, 0.0f64),
            &mut A11_0.matrix,
        );
        status_0 = aux_ApUBH(&U11_0.matrix, &B21.matrix, &mut A12.matrix);
        if status_0 != 0 {
            return status_0;
        }
        gsl_blas_zgemm(
            CblasNoTrans,
            CblasConjTrans,
            gsl_complex_rect(1.0f64, 0.0f64),
            &U12_0.matrix,
            &B22.matrix,
            gsl_complex_rect(1.0f64, 0.0f64),
            &mut A12.matrix,
        );
        status_0 = aux_ApUBH(&U22_0.matrix, &B12_0.matrix, &mut A21_0.matrix);
        if status_0 != 0 {
            return status_0;
        }
        status_0 = aux_ApUBH(&U22_0.matrix, &B22.matrix, &mut A22.matrix);
        if status_0 != 0 {
            return status_0;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
