use ::libc;
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_matrix_complex_submatrix(
        m: *mut gsl_matrix_complex,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_complex_view;
    fn gsl_matrix_complex_row(
        m: *mut gsl_matrix_complex,
        i: size_t,
    ) -> _gsl_vector_complex_view;
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
    fn gsl_blas_zdotu(
        X: *const gsl_vector_complex,
        Y: *const gsl_vector_complex,
        dotu: *mut gsl_complex,
    ) -> libc::c_int;
    fn gsl_blas_dznrm2(X: *const gsl_vector_complex) -> libc::c_double;
    fn gsl_blas_zscal(alpha: gsl_complex, X: *mut gsl_vector_complex);
    fn gsl_blas_zdscal(alpha: libc::c_double, X: *mut gsl_vector_complex);
    fn gsl_blas_zgemv(
        TransA: CBLAS_TRANSPOSE_t,
        alpha: gsl_complex,
        A: *const gsl_matrix_complex,
        X: *const gsl_vector_complex,
        beta: gsl_complex,
        Y: *mut gsl_vector_complex,
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
    fn gsl_blas_zherk(
        Uplo: CBLAS_UPLO_t,
        Trans: CBLAS_TRANSPOSE_t,
        alpha: libc::c_double,
        A: *const gsl_matrix_complex,
        beta: libc::c_double,
        C: *mut gsl_matrix_complex,
    ) -> libc::c_int;
    fn gsl_complex_abs2(z: gsl_complex) -> libc::c_double;
    fn gsl_complex_add(a: gsl_complex, b: gsl_complex) -> gsl_complex;
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
pub unsafe extern "C" fn gsl_linalg_complex_tri_LHL(
    mut L: *mut gsl_matrix_complex,
) -> libc::c_int {
    return triangular_multherm_L3(CblasLower, L);
}
pub unsafe extern "C" fn gsl_linalg_complex_tri_UL(
    mut LU: *mut gsl_matrix_complex,
) -> libc::c_int {
    return triangular_mult_L3(CblasUpper, LU);
}
unsafe extern "C" fn triangular_multherm_L2(
    mut Uplo: CBLAS_UPLO_t,
    mut T: *mut gsl_matrix_complex,
) -> libc::c_int {
    let N: size_t = (*T).size1;
    if N != (*T).size2 {
        gsl_error(
            b"matrix must be square\0" as *const u8 as *const libc::c_char,
            b"trimult_complex.c\0" as *const u8 as *const libc::c_char,
            75 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else {
        let mut i: size_t = 0;
        if !(Uplo as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint) {
            i = 0 as libc::c_int as size_t;
            while i < N {
                let mut Tii: *mut gsl_complex = gsl_matrix_complex_ptr(T, i, i);
                let mut z0: gsl_complex = *Tii;
                if i < N.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                    let mut v: gsl_vector_complex_view = gsl_matrix_complex_subcolumn(
                        T,
                        i,
                        i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    );
                    let mut norm: libc::c_double = gsl_blas_dznrm2(&mut v.vector);
                    (*Tii)
                        .dat[0 as libc::c_int
                        as usize] = gsl_complex_abs2(*Tii) + norm * norm;
                    if i > 0 as libc::c_int as libc::c_ulong {
                        let mut w: gsl_vector_complex_view = gsl_matrix_complex_subrow(
                            T,
                            i,
                            0 as libc::c_int as size_t,
                            i,
                        );
                        let mut m: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
                            T,
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                            0 as libc::c_int as size_t,
                            N
                                .wrapping_sub(i)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            i,
                        );
                        complex_conj_vector(&mut w.vector);
                        gsl_blas_zgemv(
                            CblasConjTrans,
                            gsl_complex_rect(1.0f64, 0.0f64),
                            &mut m.matrix,
                            &mut v.vector,
                            z0,
                            &mut w.vector,
                        );
                        complex_conj_vector(&mut w.vector);
                    }
                } else {
                    let mut w_0: gsl_vector_complex_view = gsl_matrix_complex_row(T, i);
                    gsl_blas_zdscal(z0.dat[0 as libc::c_int as usize], &mut w_0.vector);
                }
                (*Tii).dat[1 as libc::c_int as usize] = 0.0f64;
                i = i.wrapping_add(1);
                i;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn triangular_multherm_L3(
    mut Uplo: CBLAS_UPLO_t,
    mut T: *mut gsl_matrix_complex,
) -> libc::c_int {
    let N: size_t = (*T).size1;
    if N != (*T).size2 {
        gsl_error(
            b"matrix must be square\0" as *const u8 as *const libc::c_char,
            b"trimult_complex.c\0" as *const u8 as *const libc::c_char,
            143 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N <= 24 as libc::c_int as libc::c_ulong {
        return triangular_multherm_L2(Uplo, T)
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
        let mut T21: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
            T,
            N1,
            0 as libc::c_int as size_t,
            N2,
            N1,
        );
        let mut T22: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
            T,
            N1,
            N1,
            N2,
            N2,
        );
        status = triangular_multherm_L3(Uplo, &mut T11.matrix);
        if status != 0 {
            return status;
        }
        if Uplo as libc::c_uint == CblasLower as libc::c_int as libc::c_uint {
            gsl_blas_zherk(
                Uplo,
                CblasConjTrans,
                1.0f64,
                &mut T21.matrix,
                1.0f64,
                &mut T11.matrix,
            );
            gsl_blas_ztrmm(
                CblasLeft,
                Uplo,
                CblasConjTrans,
                CblasNonUnit,
                gsl_complex_rect(1.0f64, 0.0f64),
                &mut T22.matrix,
                &mut T21.matrix,
            );
        } else {
            gsl_blas_zherk(
                Uplo,
                CblasNoTrans,
                1.0f64,
                &mut T12.matrix,
                1.0f64,
                &mut T11.matrix,
            );
            gsl_blas_ztrmm(
                CblasRight,
                Uplo,
                CblasConjTrans,
                CblasNonUnit,
                gsl_complex_rect(1.0f64, 0.0f64),
                &mut T22.matrix,
                &mut T12.matrix,
            );
        }
        status = triangular_multherm_L3(Uplo, &mut T22.matrix);
        if status != 0 {
            return status;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn triangular_mult_L2(
    mut Uplo: CBLAS_UPLO_t,
    mut LU: *mut gsl_matrix_complex,
) -> libc::c_int {
    let N: size_t = (*LU).size1;
    if N != (*LU).size2 {
        gsl_error(
            b"matrix must be square\0" as *const u8 as *const libc::c_char,
            b"trimult_complex.c\0" as *const u8 as *const libc::c_char,
            221 as libc::c_int,
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
                let mut Aii: *mut gsl_complex = gsl_matrix_complex_ptr(LU, i, i);
                let mut Uii: gsl_complex = *Aii;
                if i < N.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                    let mut lb: gsl_vector_complex_view = gsl_matrix_complex_subcolumn(
                        LU,
                        i,
                        i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    );
                    let mut ur: gsl_vector_complex_view = gsl_matrix_complex_subrow(
                        LU,
                        i,
                        i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    );
                    let mut dot: gsl_complex = gsl_complex { dat: [0.; 2] };
                    gsl_blas_zdotu(&mut lb.vector, &mut ur.vector, &mut dot);
                    *Aii = gsl_complex_add(*Aii, dot);
                    if i > 0 as libc::c_int as libc::c_ulong {
                        let mut U_TR: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
                            LU,
                            0 as libc::c_int as size_t,
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                            i,
                            N
                                .wrapping_sub(i)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        );
                        let mut L_BL: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
                            LU,
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                            0 as libc::c_int as size_t,
                            N
                                .wrapping_sub(i)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            i,
                        );
                        let mut ut: gsl_vector_complex_view = gsl_matrix_complex_subcolumn(
                            LU,
                            i,
                            0 as libc::c_int as size_t,
                            i,
                        );
                        let mut ll: gsl_vector_complex_view = gsl_matrix_complex_subrow(
                            LU,
                            i,
                            0 as libc::c_int as size_t,
                            i,
                        );
                        gsl_blas_zgemv(
                            CblasTrans,
                            gsl_complex_rect(1.0f64, 0.0f64),
                            &mut L_BL.matrix,
                            &mut ur.vector,
                            Uii,
                            &mut ll.vector,
                        );
                        gsl_blas_zgemv(
                            CblasNoTrans,
                            gsl_complex_rect(1.0f64, 0.0f64),
                            &mut U_TR.matrix,
                            &mut lb.vector,
                            gsl_complex_rect(1.0f64, 0.0f64),
                            &mut ut.vector,
                        );
                    }
                } else {
                    let mut v: gsl_vector_complex_view = gsl_matrix_complex_subrow(
                        LU,
                        i,
                        0 as libc::c_int as size_t,
                        i,
                    );
                    gsl_blas_zscal(Uii, &mut v.vector);
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
    mut A: *mut gsl_matrix_complex,
) -> libc::c_int {
    let N: size_t = (*A).size1;
    if N != (*A).size2 {
        gsl_error(
            b"matrix must be square\0" as *const u8 as *const libc::c_char,
            b"trimult_complex.c\0" as *const u8 as *const libc::c_char,
            293 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N <= 24 as libc::c_int as libc::c_ulong {
        return triangular_mult_L2(Uplo, A)
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
        status = triangular_mult_L3(Uplo, &mut A11.matrix);
        if status != 0 {
            return status;
        }
        if !(Uplo as libc::c_uint == CblasLower as libc::c_int as libc::c_uint) {
            gsl_blas_zgemm(
                CblasNoTrans,
                CblasNoTrans,
                gsl_complex_rect(1.0f64, 0.0f64),
                &mut A12.matrix,
                &mut A21.matrix,
                gsl_complex_rect(1.0f64, 0.0f64),
                &mut A11.matrix,
            );
            gsl_blas_ztrmm(
                CblasRight,
                CblasLower,
                CblasNoTrans,
                CblasUnit,
                gsl_complex_rect(1.0f64, 0.0f64),
                &mut A22.matrix,
                &mut A12.matrix,
            );
            gsl_blas_ztrmm(
                CblasLeft,
                CblasUpper,
                CblasNoTrans,
                CblasNonUnit,
                gsl_complex_rect(1.0f64, 0.0f64),
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
unsafe extern "C" fn complex_conj_vector(mut v: *mut gsl_vector_complex) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*v).size {
        let mut vi: *mut gsl_complex = gsl_vector_complex_ptr(v, i);
        (*vi).dat[1 as libc::c_int as usize] = -(*vi).dat[1 as libc::c_int as usize];
        i = i.wrapping_add(1);
        i;
    }
}
