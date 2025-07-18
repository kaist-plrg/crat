use ::libc;
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_complex_memcpy(
        dest: *mut gsl_vector_complex,
        src: *const gsl_vector_complex,
    ) -> libc::c_int;
    fn gsl_matrix_complex_submatrix(
        m: *mut gsl_matrix_complex,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_complex_view;
    fn gsl_matrix_complex_subrow(
        m: *mut gsl_matrix_complex,
        i: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_complex_view;
    fn gsl_matrix_complex_subcolumn(
        m: *mut gsl_matrix_complex,
        j: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_complex_view;
    fn gsl_matrix_complex_const_subrow(
        m: *const gsl_matrix_complex,
        i: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_complex_const_view;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn gsl_linalg_complex_tri_invert(
        Uplo: CBLAS_UPLO_t,
        Diag: CBLAS_DIAG_t,
        T: *mut gsl_matrix_complex,
    ) -> libc::c_int;
    fn gsl_linalg_complex_tri_LHL(L: *mut gsl_matrix_complex) -> libc::c_int;
    fn gsl_blas_zdotc(
        X: *const gsl_vector_complex,
        Y: *const gsl_vector_complex,
        dotc: *mut gsl_complex,
    ) -> libc::c_int;
    fn gsl_blas_zdscal(alpha: libc::c_double, X: *mut gsl_vector_complex);
    fn gsl_blas_zgemv(
        TransA: CBLAS_TRANSPOSE_t,
        alpha: gsl_complex,
        A: *const gsl_matrix_complex,
        X: *const gsl_vector_complex,
        beta: gsl_complex,
        Y: *mut gsl_vector_complex,
    ) -> libc::c_int;
    fn gsl_blas_ztrsv(
        Uplo: CBLAS_UPLO_t,
        TransA: CBLAS_TRANSPOSE_t,
        Diag: CBLAS_DIAG_t,
        A: *const gsl_matrix_complex,
        X: *mut gsl_vector_complex,
    ) -> libc::c_int;
    fn gsl_blas_ztrsm(
        Side: CBLAS_SIDE_t,
        Uplo: CBLAS_UPLO_t,
        TransA: CBLAS_TRANSPOSE_t,
        Diag: CBLAS_DIAG_t,
        alpha: gsl_complex,
        A: *const gsl_matrix_complex,
        B: *mut gsl_matrix_complex,
    ) -> libc::c_int;
    fn gsl_blas_zherk(
        Uplo: CBLAS_UPLO_t,
        Trans: CBLAS_TRANSPOSE_t,
        alpha: libc::c_double,
        A: *const gsl_matrix_complex,
        beta: libc::c_double,
        C: *mut gsl_matrix_complex,
    ) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_vector_complex_const_view {
    pub vector: gsl_vector_complex,
}
pub type gsl_vector_complex_const_view = _gsl_vector_complex_const_view;
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
unsafe extern "C" fn gsl_matrix_complex_set(
    mut m: *mut gsl_matrix_complex,
    i: size_t,
    j: size_t,
    x: gsl_complex,
) {
    *(((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        ) as *mut gsl_complex) = x;
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
pub unsafe extern "C" fn gsl_linalg_complex_cholesky_decomp(
    mut A: *mut gsl_matrix_complex,
) -> libc::c_int {
    let N: size_t = (*A).size1;
    if N != (*A).size2 {
        gsl_error(
            b"Cholesky decomposition requires square matrix\0" as *const u8
                as *const libc::c_char,
            b"choleskyc.c\0" as *const u8 as *const libc::c_char,
            62 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else {
        return complex_cholesky_decomp_L3(A)
    };
}
pub unsafe extern "C" fn gsl_linalg_complex_cholesky_solve(
    mut cholesky: *const gsl_matrix_complex,
    mut b: *const gsl_vector_complex,
    mut x: *mut gsl_vector_complex,
) -> libc::c_int {
    if (*cholesky).size1 != (*cholesky).size2 {
        gsl_error(
            b"cholesky matrix must be square\0" as *const u8 as *const libc::c_char,
            b"choleskyc.c\0" as *const u8 as *const libc::c_char,
            82 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*cholesky).size1 != (*b).size {
        gsl_error(
            b"matrix size must match b size\0" as *const u8 as *const libc::c_char,
            b"choleskyc.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*cholesky).size2 != (*x).size {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8
                as *const libc::c_char,
            b"choleskyc.c\0" as *const u8 as *const libc::c_char,
            90 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        gsl_vector_complex_memcpy(x, b);
        return gsl_linalg_complex_cholesky_svx(cholesky, x);
    };
}
pub unsafe extern "C" fn gsl_linalg_complex_cholesky_svx(
    mut cholesky: *const gsl_matrix_complex,
    mut x: *mut gsl_vector_complex,
) -> libc::c_int {
    if (*cholesky).size1 != (*cholesky).size2 {
        gsl_error(
            b"cholesky matrix must be square\0" as *const u8 as *const libc::c_char,
            b"choleskyc.c\0" as *const u8 as *const libc::c_char,
            110 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*cholesky).size2 != (*x).size {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8
                as *const libc::c_char,
            b"choleskyc.c\0" as *const u8 as *const libc::c_char,
            114 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        gsl_blas_ztrsv(CblasLower, CblasNoTrans, CblasNonUnit, cholesky, x);
        gsl_blas_ztrsv(CblasLower, CblasConjTrans, CblasNonUnit, cholesky, x);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_complex_cholesky_invert(
    mut LLT: *mut gsl_matrix_complex,
) -> libc::c_int {
    if (*LLT).size1 != (*LLT).size2 {
        gsl_error(
            b"cholesky matrix must be square\0" as *const u8 as *const libc::c_char,
            b"choleskyc.c\0" as *const u8 as *const libc::c_char,
            146 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut N: size_t = (*LLT).size1;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        status = gsl_linalg_complex_tri_invert(CblasLower, CblasNonUnit, LLT);
        if status != 0 {
            return status;
        }
        status = gsl_linalg_complex_tri_LHL(LLT);
        if status != 0 {
            return status;
        }
        i = 1 as libc::c_int as size_t;
        while i < N {
            j = 0 as libc::c_int as size_t;
            while j < i {
                let mut z: gsl_complex = gsl_matrix_complex_get(LLT, i, j);
                gsl_matrix_complex_set(LLT, j, i, gsl_complex_conjugate(z));
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn cholesky_complex_conj_vector(mut v: *mut gsl_vector_complex) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*v).size {
        let mut vi: *mut gsl_complex = gsl_vector_complex_ptr(v, i);
        (*vi).dat[1 as libc::c_int as usize] = -(*vi).dat[1 as libc::c_int as usize];
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn complex_cholesky_decomp_L2(
    mut A: *mut gsl_matrix_complex,
) -> libc::c_int {
    let N: size_t = (*A).size1;
    if N != (*A).size2 {
        gsl_error(
            b"Cholesky decomposition requires square matrix\0" as *const u8
                as *const libc::c_char,
            b"choleskyc.c\0" as *const u8 as *const libc::c_char,
            216 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else {
        let mut j: size_t = 0;
        let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
        let mut ajj: libc::c_double = 0.;
        j = 0 as libc::c_int as size_t;
        while j < N {
            z = gsl_matrix_complex_get(A, j, j);
            ajj = z.dat[0 as libc::c_int as usize];
            if j > 0 as libc::c_int as libc::c_ulong {
                let aj: gsl_vector_complex_const_view = gsl_matrix_complex_const_subrow(
                    A,
                    j,
                    0 as libc::c_int as size_t,
                    j,
                );
                gsl_blas_zdotc(&aj.vector, &aj.vector, &mut z);
                ajj -= z.dat[0 as libc::c_int as usize];
            }
            if ajj <= 0.0f64 {
                gsl_error(
                    b"matrix is not positive definite\0" as *const u8
                        as *const libc::c_char,
                    b"choleskyc.c\0" as *const u8 as *const libc::c_char,
                    240 as libc::c_int,
                    GSL_EDOM as libc::c_int,
                );
                return GSL_EDOM as libc::c_int;
            }
            ajj = sqrt(ajj);
            z.dat[0 as libc::c_int as usize] = ajj;
            z.dat[1 as libc::c_int as usize] = 0.0f64;
            gsl_matrix_complex_set(A, j, j, z);
            if j < N.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                let mut av: gsl_vector_complex_view = gsl_matrix_complex_subcolumn(
                    A,
                    j,
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    N.wrapping_sub(j).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                if j > 0 as libc::c_int as libc::c_ulong {
                    let mut aj_0: gsl_vector_complex_view = gsl_matrix_complex_subrow(
                        A,
                        j,
                        0 as libc::c_int as size_t,
                        j,
                    );
                    let mut am: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
                        A,
                        j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        0 as libc::c_int as size_t,
                        N
                            .wrapping_sub(j)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        j,
                    );
                    cholesky_complex_conj_vector(&mut aj_0.vector);
                    gsl_blas_zgemv(
                        CblasNoTrans,
                        gsl_complex_rect(-1.0f64, 0.0f64),
                        &mut am.matrix,
                        &mut aj_0.vector,
                        gsl_complex_rect(1.0f64, 0.0f64),
                        &mut av.vector,
                    );
                    cholesky_complex_conj_vector(&mut aj_0.vector);
                }
                gsl_blas_zdscal(1.0f64 / ajj, &mut av.vector);
            }
            j = j.wrapping_add(1);
            j;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn complex_cholesky_decomp_L3(
    mut A: *mut gsl_matrix_complex,
) -> libc::c_int {
    let N: size_t = (*A).size1;
    if N != (*A).size2 {
        gsl_error(
            b"Cholesky decomposition requires square matrix\0" as *const u8
                as *const libc::c_char,
            b"choleskyc.c\0" as *const u8 as *const libc::c_char,
            301 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N <= 24 as libc::c_int as libc::c_ulong {
        return complex_cholesky_decomp_L2(A)
    } else {
        let mut status: libc::c_int = 0;
        let N1: size_t = if N >= 8 as libc::c_int as libc::c_ulong {
            N.wrapping_add(4 as libc::c_int as libc::c_ulong)
                .wrapping_div(8 as libc::c_int as libc::c_ulong)
                .wrapping_mul(4 as libc::c_int as libc::c_ulong)
        } else {
            N.wrapping_div(2 as libc::c_int as libc::c_ulong)
        };
        let N2: size_t = N.wrapping_sub(N1);
        let mut A11: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
            A,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N1,
            N1,
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
        status = complex_cholesky_decomp_L3(&mut A11.matrix);
        if status != 0 {
            return status;
        }
        gsl_blas_ztrsm(
            CblasRight,
            CblasLower,
            CblasConjTrans,
            CblasNonUnit,
            gsl_complex_rect(1.0f64, 0.0f64),
            &mut A11.matrix,
            &mut A21.matrix,
        );
        gsl_blas_zherk(
            CblasLower,
            CblasNoTrans,
            -1.0f64,
            &mut A21.matrix,
            1.0f64,
            &mut A22.matrix,
        );
        status = complex_cholesky_decomp_L3(&mut A22.matrix);
        if status != 0 {
            return status;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
