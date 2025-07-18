use ::libc;
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_matrix_submatrix(
        m: *mut gsl_matrix,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_view;
    fn gsl_matrix_row(m: *mut gsl_matrix, i: size_t) -> _gsl_vector_view;
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
    fn gsl_blas_ddot(
        X: *const gsl_vector,
        Y: *const gsl_vector,
        result: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_blas_dscal(alpha: libc::c_double, X: *mut gsl_vector);
    fn gsl_blas_dgemv(
        TransA: CBLAS_TRANSPOSE_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        X: *const gsl_vector,
        beta: libc::c_double,
        Y: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_blas_dgemm(
        TransA: CBLAS_TRANSPOSE_t,
        TransB: CBLAS_TRANSPOSE_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        B: *const gsl_matrix,
        beta: libc::c_double,
        C: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_blas_dsyrk(
        Uplo: CBLAS_UPLO_t,
        Trans: CBLAS_TRANSPOSE_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        beta: libc::c_double,
        C: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_blas_dtrmm(
        Side: CBLAS_SIDE_t,
        Uplo: CBLAS_UPLO_t,
        TransA: CBLAS_TRANSPOSE_t,
        Diag: CBLAS_DIAG_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        B: *mut gsl_matrix,
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
pub type CBLAS_SIDE = libc::c_uint;
pub const CblasRight: CBLAS_SIDE = 142;
pub const CblasLeft: CBLAS_SIDE = 141;
pub type CBLAS_TRANSPOSE_t = CBLAS_TRANSPOSE;
pub type CBLAS_UPLO_t = CBLAS_UPLO;
pub type CBLAS_DIAG_t = CBLAS_DIAG;
pub type CBLAS_SIDE_t = CBLAS_SIDE;
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
pub unsafe extern "C" fn gsl_linalg_tri_LTL(mut L: *mut gsl_matrix) -> libc::c_int {
    return triangular_multsymm_L3(CblasLower, L);
}
pub unsafe extern "C" fn gsl_linalg_tri_UL(mut LU: *mut gsl_matrix) -> libc::c_int {
    return triangular_mult_L3(CblasUpper, LU);
}
unsafe extern "C" fn triangular_multsymm_L2(
    mut Uplo: CBLAS_UPLO_t,
    mut T: *mut gsl_matrix,
) -> libc::c_int {
    let N: size_t = (*T).size1;
    if N != (*T).size2 {
        gsl_error(
            b"matrix must be square\0" as *const u8 as *const libc::c_char,
            b"trimult.c\0" as *const u8 as *const libc::c_char,
            71 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else {
        let mut v1: gsl_vector_view = gsl_vector_view {
            vector: gsl_vector {
                size: 0,
                stride: 0,
                data: 0 as *mut libc::c_double,
                block: 0 as *mut gsl_block,
                owner: 0,
            },
        };
        let mut v2: gsl_vector_view = gsl_vector_view {
            vector: gsl_vector {
                size: 0,
                stride: 0,
                data: 0 as *mut libc::c_double,
                block: 0 as *mut gsl_block,
                owner: 0,
            },
        };
        let mut i: size_t = 0;
        if !(Uplo as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint) {
            i = 0 as libc::c_int as size_t;
            while i < N {
                let mut Tii: libc::c_double = gsl_matrix_get(T, i, i);
                if i < N.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                    let mut tmp: libc::c_double = 0.;
                    v1 = gsl_matrix_subcolumn(T, i, i, N.wrapping_sub(i));
                    gsl_blas_ddot(&mut v1.vector, &mut v1.vector, &mut tmp);
                    gsl_matrix_set(T, i, i, tmp);
                    if i > 0 as libc::c_int as libc::c_ulong {
                        let mut m: gsl_matrix_view = gsl_matrix_submatrix(
                            T,
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                            0 as libc::c_int as size_t,
                            N
                                .wrapping_sub(i)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            i,
                        );
                        v1 = gsl_matrix_subcolumn(
                            T,
                            i,
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                            N
                                .wrapping_sub(i)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        );
                        v2 = gsl_matrix_subrow(T, i, 0 as libc::c_int as size_t, i);
                        gsl_blas_dgemv(
                            CblasTrans,
                            1.0f64,
                            &mut m.matrix,
                            &mut v1.vector,
                            Tii,
                            &mut v2.vector,
                        );
                    }
                } else {
                    v1 = gsl_matrix_row(
                        T,
                        N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    );
                    gsl_blas_dscal(Tii, &mut v1.vector);
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn triangular_multsymm_L3(
    mut Uplo: CBLAS_UPLO_t,
    mut T: *mut gsl_matrix,
) -> libc::c_int {
    let N: size_t = (*T).size1;
    if N != (*T).size2 {
        gsl_error(
            b"matrix must be square\0" as *const u8 as *const libc::c_char,
            b"trimult.c\0" as *const u8 as *const libc::c_char,
            138 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N <= 24 as libc::c_int as libc::c_ulong {
        return triangular_multsymm_L2(Uplo, T)
    } else {
        let mut status: libc::c_int = 0;
        let N1: size_t = if N >= 16 as libc::c_int as libc::c_ulong {
            N.wrapping_add(8 as libc::c_int as libc::c_ulong)
                .wrapping_div(16 as libc::c_int as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        } else {
            N.wrapping_div(2 as libc::c_int as libc::c_ulong)
        };
        let N2: size_t = N.wrapping_sub(N1);
        let mut T11: gsl_matrix_view = gsl_matrix_submatrix(
            T,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N1,
            N1,
        );
        let mut T12: gsl_matrix_view = gsl_matrix_submatrix(
            T,
            0 as libc::c_int as size_t,
            N1,
            N1,
            N2,
        );
        let mut T21: gsl_matrix_view = gsl_matrix_submatrix(
            T,
            N1,
            0 as libc::c_int as size_t,
            N2,
            N1,
        );
        let mut T22: gsl_matrix_view = gsl_matrix_submatrix(T, N1, N1, N2, N2);
        status = triangular_multsymm_L3(Uplo, &mut T11.matrix);
        if status != 0 {
            return status;
        }
        if Uplo as libc::c_uint == CblasLower as libc::c_int as libc::c_uint {
            gsl_blas_dsyrk(
                Uplo,
                CblasTrans,
                1.0f64,
                &mut T21.matrix,
                1.0f64,
                &mut T11.matrix,
            );
            gsl_blas_dtrmm(
                CblasLeft,
                Uplo,
                CblasTrans,
                CblasNonUnit,
                1.0f64,
                &mut T22.matrix,
                &mut T21.matrix,
            );
        } else {
            gsl_blas_dsyrk(
                Uplo,
                CblasNoTrans,
                1.0f64,
                &mut T12.matrix,
                1.0f64,
                &mut T11.matrix,
            );
            gsl_blas_dtrmm(
                CblasRight,
                Uplo,
                CblasTrans,
                CblasNonUnit,
                1.0f64,
                &mut T22.matrix,
                &mut T12.matrix,
            );
        }
        status = triangular_multsymm_L3(Uplo, &mut T22.matrix);
        if status != 0 {
            return status;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn triangular_mult_L2(
    mut Uplo: CBLAS_UPLO_t,
    mut A: *mut gsl_matrix,
) -> libc::c_int {
    let N: size_t = (*A).size1;
    if N != (*A).size2 {
        gsl_error(
            b"matrix must be square\0" as *const u8 as *const libc::c_char,
            b"trimult.c\0" as *const u8 as *const libc::c_char,
            210 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else {
        let mut i: size_t = 0;
        if N == 1 as libc::c_int as libc::c_ulong {
            return GSL_SUCCESS as libc::c_int;
        }
        if Uplo as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint {
            i = 0 as libc::c_int as size_t;
            while i < N {
                let mut Aii: *mut libc::c_double = gsl_matrix_ptr(A, i, i);
                let mut Uii: libc::c_double = *Aii;
                if i < N.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                    let mut lb: gsl_vector_view = gsl_matrix_subcolumn(
                        A,
                        i,
                        i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    );
                    let mut ur: gsl_vector_view = gsl_matrix_subrow(
                        A,
                        i,
                        i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    );
                    let mut tmp: libc::c_double = 0.;
                    gsl_blas_ddot(&mut lb.vector, &mut ur.vector, &mut tmp);
                    *Aii += tmp;
                    if i > 0 as libc::c_int as libc::c_ulong {
                        let mut U_TR: gsl_matrix_view = gsl_matrix_submatrix(
                            A,
                            0 as libc::c_int as size_t,
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                            i,
                            N
                                .wrapping_sub(i)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        );
                        let mut L_BL: gsl_matrix_view = gsl_matrix_submatrix(
                            A,
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                            0 as libc::c_int as size_t,
                            N
                                .wrapping_sub(i)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            i,
                        );
                        let mut ut: gsl_vector_view = gsl_matrix_subcolumn(
                            A,
                            i,
                            0 as libc::c_int as size_t,
                            i,
                        );
                        let mut ll: gsl_vector_view = gsl_matrix_subrow(
                            A,
                            i,
                            0 as libc::c_int as size_t,
                            i,
                        );
                        gsl_blas_dgemv(
                            CblasTrans,
                            1.0f64,
                            &mut L_BL.matrix,
                            &mut ur.vector,
                            Uii,
                            &mut ll.vector,
                        );
                        gsl_blas_dgemv(
                            CblasNoTrans,
                            1.0f64,
                            &mut U_TR.matrix,
                            &mut lb.vector,
                            1.0f64,
                            &mut ut.vector,
                        );
                    }
                } else {
                    let mut v: gsl_vector_view = gsl_matrix_subrow(
                        A,
                        N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        0 as libc::c_int as size_t,
                        N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    );
                    gsl_blas_dscal(Uii, &mut v.vector);
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn triangular_mult_L3(
    mut Uplo: CBLAS_UPLO_t,
    mut A: *mut gsl_matrix,
) -> libc::c_int {
    let N: size_t = (*A).size1;
    if N != (*A).size2 {
        gsl_error(
            b"matrix must be square\0" as *const u8 as *const libc::c_char,
            b"trimult.c\0" as *const u8 as *const libc::c_char,
            282 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N <= 24 as libc::c_int as libc::c_ulong {
        return triangular_mult_L2(Uplo, A)
    } else {
        let mut status: libc::c_int = 0;
        let N1: size_t = if N >= 16 as libc::c_int as libc::c_ulong {
            N.wrapping_add(8 as libc::c_int as libc::c_ulong)
                .wrapping_div(16 as libc::c_int as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        } else {
            N.wrapping_div(2 as libc::c_int as libc::c_ulong)
        };
        let N2: size_t = N.wrapping_sub(N1);
        let mut A11: gsl_matrix_view = gsl_matrix_submatrix(
            A,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N1,
            N1,
        );
        let mut A12: gsl_matrix_view = gsl_matrix_submatrix(
            A,
            0 as libc::c_int as size_t,
            N1,
            N1,
            N2,
        );
        let mut A21: gsl_matrix_view = gsl_matrix_submatrix(
            A,
            N1,
            0 as libc::c_int as size_t,
            N2,
            N1,
        );
        let mut A22: gsl_matrix_view = gsl_matrix_submatrix(A, N1, N1, N2, N2);
        status = triangular_mult_L3(Uplo, &mut A11.matrix);
        if status != 0 {
            return status;
        }
        if !(Uplo as libc::c_uint == CblasLower as libc::c_int as libc::c_uint) {
            gsl_blas_dgemm(
                CblasNoTrans,
                CblasNoTrans,
                1.0f64,
                &mut A12.matrix,
                &mut A21.matrix,
                1.0f64,
                &mut A11.matrix,
            );
            gsl_blas_dtrmm(
                CblasRight,
                CblasLower,
                CblasNoTrans,
                CblasUnit,
                1.0f64,
                &mut A22.matrix,
                &mut A12.matrix,
            );
            gsl_blas_dtrmm(
                CblasLeft,
                CblasUpper,
                CblasNoTrans,
                CblasNonUnit,
                1.0f64,
                &mut A22.matrix,
                &mut A21.matrix,
            );
        }
        status = triangular_mult_L3(Uplo, &mut A22.matrix);
        if status != 0 {
            return status;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
