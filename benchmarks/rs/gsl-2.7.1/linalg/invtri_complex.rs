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
    fn gsl_matrix_complex_subcolumn(
        m: *mut gsl_matrix_complex,
        j: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_complex_view;
    fn gsl_blas_zscal(alpha: gsl_complex, X: *mut gsl_vector_complex);
    fn gsl_blas_ztrmv(
        Uplo: CBLAS_UPLO_t,
        TransA: CBLAS_TRANSPOSE_t,
        Diag: CBLAS_DIAG_t,
        A: *const gsl_matrix_complex,
        X: *mut gsl_vector_complex,
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
    fn gsl_blas_ztrsm(
        Side: CBLAS_SIDE_t,
        Uplo: CBLAS_UPLO_t,
        TransA: CBLAS_TRANSPOSE_t,
        Diag: CBLAS_DIAG_t,
        alpha: gsl_complex,
        A: *const gsl_matrix_complex,
        B: *mut gsl_matrix_complex,
    ) -> libc::c_int;
    fn gsl_complex_inverse(a: gsl_complex) -> gsl_complex;
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
pub unsafe extern "C" fn gsl_linalg_complex_tri_invert(
    mut Uplo: CBLAS_UPLO_t,
    mut Diag: CBLAS_DIAG_t,
    mut T: *mut gsl_matrix_complex,
) -> libc::c_int {
    let N: size_t = (*T).size1;
    if N != (*T).size2 {
        gsl_error(
            b"matrix must be square\0" as *const u8 as *const libc::c_char,
            b"invtri_complex.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        status = triangular_singular(T);
        if status != 0 {
            return status;
        }
        return complex_tri_invert_L3(Uplo, Diag, T);
    };
}
unsafe extern "C" fn complex_tri_invert_L2(
    mut Uplo: CBLAS_UPLO_t,
    mut Diag: CBLAS_DIAG_t,
    mut T: *mut gsl_matrix_complex,
) -> libc::c_int {
    let N: size_t = (*T).size1;
    if N != (*T).size2 {
        gsl_error(
            b"matrix must be square\0" as *const u8 as *const libc::c_char,
            b"invtri_complex.c\0" as *const u8 as *const libc::c_char,
            94 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else {
        let mut i: size_t = 0;
        if Uplo as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint {
            i = 0 as libc::c_int as size_t;
            while i < N {
                let mut Tii: *mut gsl_complex = gsl_matrix_complex_ptr(T, i, i);
                let mut aii: gsl_complex = gsl_complex { dat: [0.; 2] };
                if Diag as libc::c_uint == CblasNonUnit as libc::c_int as libc::c_uint {
                    *Tii = gsl_complex_inverse(*Tii);
                    aii
                        .dat[0 as libc::c_int
                        as usize] = -(*Tii).dat[0 as libc::c_int as usize];
                    aii
                        .dat[1 as libc::c_int
                        as usize] = -(*Tii).dat[1 as libc::c_int as usize];
                } else {
                    aii = gsl_complex_rect(-1.0f64, 0.0f64);
                }
                if i > 0 as libc::c_int as libc::c_ulong {
                    let mut m: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
                        T,
                        0 as libc::c_int as size_t,
                        0 as libc::c_int as size_t,
                        i,
                        i,
                    );
                    let mut v: gsl_vector_complex_view = gsl_matrix_complex_subcolumn(
                        T,
                        i,
                        0 as libc::c_int as size_t,
                        i,
                    );
                    gsl_blas_ztrmv(
                        CblasUpper,
                        CblasNoTrans,
                        Diag,
                        &mut m.matrix,
                        &mut v.vector,
                    );
                    gsl_blas_zscal(aii, &mut v.vector);
                }
                i = i.wrapping_add(1);
                i;
            }
        } else {
            i = 0 as libc::c_int as size_t;
            while i < N {
                let mut j: size_t = N
                    .wrapping_sub(i)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                let mut Tjj: *mut gsl_complex = gsl_matrix_complex_ptr(T, j, j);
                let mut ajj: gsl_complex = gsl_complex { dat: [0.; 2] };
                if Diag as libc::c_uint == CblasNonUnit as libc::c_int as libc::c_uint {
                    *Tjj = gsl_complex_inverse(*Tjj);
                    ajj
                        .dat[0 as libc::c_int
                        as usize] = -(*Tjj).dat[0 as libc::c_int as usize];
                    ajj
                        .dat[1 as libc::c_int
                        as usize] = -(*Tjj).dat[1 as libc::c_int as usize];
                } else {
                    ajj = gsl_complex_rect(-1.0f64, 0.0f64);
                }
                if j < N.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                    let mut m_0: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
                        T,
                        j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        N
                            .wrapping_sub(j)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        N.wrapping_sub(j).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    );
                    let mut v_0: gsl_vector_complex_view = gsl_matrix_complex_subcolumn(
                        T,
                        j,
                        j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        N.wrapping_sub(j).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    );
                    gsl_blas_ztrmv(
                        CblasLower,
                        CblasNoTrans,
                        Diag,
                        &mut m_0.matrix,
                        &mut v_0.vector,
                    );
                    gsl_blas_zscal(ajj, &mut v_0.vector);
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn complex_tri_invert_L3(
    mut Uplo: CBLAS_UPLO_t,
    mut Diag: CBLAS_DIAG_t,
    mut T: *mut gsl_matrix_complex,
) -> libc::c_int {
    let N: size_t = (*T).size1;
    if N != (*T).size2 {
        gsl_error(
            b"matrix must be square\0" as *const u8 as *const libc::c_char,
            b"invtri_complex.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N <= 24 as libc::c_int as libc::c_ulong {
        return complex_tri_invert_L2(Uplo, Diag, T)
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
        status = complex_tri_invert_L3(Uplo, Diag, &mut T11.matrix);
        if status != 0 {
            return status;
        }
        if Uplo as libc::c_uint == CblasLower as libc::c_int as libc::c_uint {
            gsl_blas_ztrmm(
                CblasRight,
                Uplo,
                CblasNoTrans,
                Diag,
                gsl_complex_rect(-1.0f64, 0.0f64),
                &mut T11.matrix,
                &mut T21.matrix,
            );
            gsl_blas_ztrsm(
                CblasLeft,
                Uplo,
                CblasNoTrans,
                Diag,
                gsl_complex_rect(1.0f64, 0.0f64),
                &mut T22.matrix,
                &mut T21.matrix,
            );
        } else {
            gsl_blas_ztrmm(
                CblasLeft,
                Uplo,
                CblasNoTrans,
                Diag,
                gsl_complex_rect(-1.0f64, 0.0f64),
                &mut T11.matrix,
                &mut T12.matrix,
            );
            gsl_blas_ztrsm(
                CblasRight,
                Uplo,
                CblasNoTrans,
                Diag,
                gsl_complex_rect(1.0f64, 0.0f64),
                &mut T22.matrix,
                &mut T12.matrix,
            );
        }
        status = complex_tri_invert_L3(Uplo, Diag, &mut T22.matrix);
        if status != 0 {
            return status;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn triangular_singular(
    mut T: *const gsl_matrix_complex,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*T).size1 {
        let mut z: gsl_complex = gsl_matrix_complex_get(T, i, i);
        if z.dat[0 as libc::c_int as usize] == 0.0f64
            && z.dat[1 as libc::c_int as usize] == 0.0f64
        {
            return GSL_ESING as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
