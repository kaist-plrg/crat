use ::libc;
extern "C" {
    fn log(_: libc::c_double) -> libc::c_double;
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
    fn gsl_vector_uint_alloc(n: size_t) -> *mut gsl_vector_uint;
    fn gsl_vector_uint_free(v: *mut gsl_vector_uint);
    fn gsl_vector_uint_subvector(
        v: *mut gsl_vector_uint,
        i: size_t,
        n: size_t,
    ) -> _gsl_vector_uint_view;
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
    fn gsl_matrix_complex_memcpy(
        dest: *mut gsl_matrix_complex,
        src: *const gsl_matrix_complex,
    ) -> libc::c_int;
    fn gsl_complex_abs(z: gsl_complex) -> libc::c_double;
    fn gsl_complex_mul(a: gsl_complex, b: gsl_complex) -> gsl_complex;
    fn gsl_complex_div_real(a: gsl_complex, x: libc::c_double) -> gsl_complex;
    fn gsl_complex_inverse(a: gsl_complex) -> gsl_complex;
    fn gsl_permutation_init(p: *mut gsl_permutation);
    fn gsl_permute_vector_complex(
        p: *const gsl_permutation,
        v: *mut gsl_vector_complex,
    ) -> libc::c_int;
    fn gsl_permute_vector_complex_inverse(
        p: *const gsl_permutation,
        v: *mut gsl_vector_complex,
    ) -> libc::c_int;
    fn gsl_blas_izamax(X: *const gsl_vector_complex) -> CBLAS_INDEX_t;
    fn gsl_blas_zswap(
        X: *mut gsl_vector_complex,
        Y: *mut gsl_vector_complex,
    ) -> libc::c_int;
    fn gsl_blas_zaxpy(
        alpha: gsl_complex,
        X: *const gsl_vector_complex,
        Y: *mut gsl_vector_complex,
    ) -> libc::c_int;
    fn gsl_blas_zscal(alpha: gsl_complex, X: *mut gsl_vector_complex);
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
    fn gsl_blas_zgeru(
        alpha: gsl_complex,
        X: *const gsl_vector_complex,
        Y: *const gsl_vector_complex,
        A: *mut gsl_matrix_complex,
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
    fn gsl_blas_ztrsm(
        Side: CBLAS_SIDE_t,
        Uplo: CBLAS_UPLO_t,
        TransA: CBLAS_TRANSPOSE_t,
        Diag: CBLAS_DIAG_t,
        alpha: gsl_complex,
        A: *const gsl_matrix_complex,
        B: *mut gsl_matrix_complex,
    ) -> libc::c_int;
    fn gsl_linalg_complex_tri_invert(
        Uplo: CBLAS_UPLO_t,
        Diag: CBLAS_DIAG_t,
        T: *mut gsl_matrix_complex,
    ) -> libc::c_int;
    fn gsl_linalg_complex_tri_UL(LU: *mut gsl_matrix_complex) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_uint_struct {
    pub size: size_t,
    pub data: *mut libc::c_uint,
}
pub type gsl_block_uint = gsl_block_uint_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_uint {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_uint,
    pub block: *mut gsl_block_uint,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_vector_uint_view {
    pub vector: gsl_vector_uint,
}
pub type gsl_vector_uint_view = _gsl_vector_uint_view;
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
pub type CBLAS_INDEX_t = size_t;
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
pub struct gsl_permutation_struct {
    pub size: size_t,
    pub data: *mut size_t,
}
pub type gsl_permutation = gsl_permutation_struct;
#[inline]
unsafe extern "C" fn gsl_vector_uint_get(
    mut v: *const gsl_vector_uint,
    i: size_t,
) -> libc::c_uint {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_vector_uint_set(
    mut v: *mut gsl_vector_uint,
    i: size_t,
    mut x: libc::c_uint,
) {
    *((*v).data).offset(i.wrapping_mul((*v).stride) as isize) = x;
}
#[inline]
unsafe extern "C" fn gsl_vector_uint_ptr(
    mut v: *mut gsl_vector_uint,
    i: size_t,
) -> *mut libc::c_uint {
    return ((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
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
pub unsafe extern "C" fn gsl_linalg_complex_LU_decomp(
    mut A: *mut gsl_matrix_complex,
    mut p: *mut gsl_permutation,
    mut signum: *mut libc::c_int,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    if (*p).size != M {
        gsl_error(
            b"permutation length must match matrix size1\0" as *const u8
                as *const libc::c_char,
            b"luc.c\0" as *const u8 as *const libc::c_char,
            71 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let N: size_t = (*A).size2;
        let minMN: size_t = if M < N { M } else { N };
        let mut ipiv: *mut gsl_vector_uint = gsl_vector_uint_alloc(minMN);
        let mut AL: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
            A,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            M,
            minMN,
        );
        let mut i: size_t = 0;
        status = LU_decomp_L3(&mut AL.matrix, ipiv);
        if M < N {
            let mut AR: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
                A,
                0 as libc::c_int as size_t,
                M,
                M,
                N.wrapping_sub(M),
            );
            apply_pivots(&mut AR.matrix, ipiv);
            gsl_blas_ztrsm(
                CblasLeft,
                CblasLower,
                CblasNoTrans,
                CblasUnit,
                gsl_complex_rect(1.0f64, 0.0f64),
                &mut AL.matrix,
                &mut AR.matrix,
            );
        }
        gsl_permutation_init(p);
        *signum = 1 as libc::c_int;
        i = 0 as libc::c_int as size_t;
        while i < minMN {
            let mut pivi: libc::c_uint = gsl_vector_uint_get(ipiv, i);
            if *((*p).data).offset(pivi as isize) != *((*p).data).offset(i as isize) {
                let mut tmp: size_t = *((*p).data).offset(pivi as isize);
                *((*p).data).offset(pivi as isize) = *((*p).data).offset(i as isize);
                *((*p).data).offset(i as isize) = tmp;
                *signum = -*signum;
            }
            i = i.wrapping_add(1);
            i;
        }
        gsl_vector_uint_free(ipiv);
        return status;
    };
}
unsafe extern "C" fn LU_decomp_L2(
    mut A: *mut gsl_matrix_complex,
    mut ipiv: *mut gsl_vector_uint,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    let minMN: size_t = if M < N { M } else { N };
    if (*ipiv).size != minMN {
        gsl_error(
            b"ipiv length must equal MIN(M,N)\0" as *const u8 as *const libc::c_char,
            b"luc.c\0" as *const u8 as *const libc::c_char,
            140 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < minMN {
            let mut v: gsl_vector_complex_view = gsl_matrix_complex_subcolumn(
                A,
                j,
                j,
                M.wrapping_sub(j),
            );
            let mut j_pivot: size_t = j.wrapping_add(gsl_blas_izamax(&mut v.vector));
            let mut v1: gsl_vector_complex_view = gsl_vector_complex_view {
                vector: gsl_vector_complex {
                    size: 0,
                    stride: 0,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block_complex,
                    owner: 0,
                },
            };
            let mut v2: gsl_vector_complex_view = gsl_vector_complex_view {
                vector: gsl_vector_complex {
                    size: 0,
                    stride: 0,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block_complex,
                    owner: 0,
                },
            };
            gsl_vector_uint_set(ipiv, j, j_pivot as libc::c_uint);
            if j_pivot != j {
                v1 = gsl_matrix_complex_row(A, j);
                v2 = gsl_matrix_complex_row(A, j_pivot);
                gsl_blas_zswap(&mut v1.vector, &mut v2.vector);
            }
            if j < M.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                let mut Ajj: gsl_complex = gsl_matrix_complex_get(A, j, j);
                let mut Ajjinv: gsl_complex = gsl_complex_inverse(Ajj);
                if gsl_complex_abs(Ajj) >= 2.2250738585072014e-308f64 {
                    v1 = gsl_matrix_complex_subcolumn(
                        A,
                        j,
                        j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        M.wrapping_sub(j).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    );
                    gsl_blas_zscal(Ajjinv, &mut v1.vector);
                } else {
                    i = 1 as libc::c_int as size_t;
                    while i < M.wrapping_sub(j) {
                        let mut ptr: *mut gsl_complex = gsl_matrix_complex_ptr(
                            A,
                            j.wrapping_add(i),
                            j,
                        );
                        *ptr = gsl_complex_mul(*ptr, Ajjinv);
                        i = i.wrapping_add(1);
                        i;
                    }
                }
            }
            if j < minMN.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                let mut A22: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
                    A,
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    M.wrapping_sub(j).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    N.wrapping_sub(j).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                v1 = gsl_matrix_complex_subcolumn(
                    A,
                    j,
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    M.wrapping_sub(j).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                v2 = gsl_matrix_complex_subrow(
                    A,
                    j,
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    N.wrapping_sub(j).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                gsl_blas_zgeru(
                    gsl_complex_rect(-1.0f64, 0.0f64),
                    &mut v1.vector,
                    &mut v2.vector,
                    &mut A22.matrix,
                );
            }
            j = j.wrapping_add(1);
            j;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn LU_decomp_L3(
    mut A: *mut gsl_matrix_complex,
    mut ipiv: *mut gsl_vector_uint,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if M < N {
        gsl_error(
            b"matrix must have M >= N\0" as *const u8 as *const libc::c_char,
            b"luc.c\0" as *const u8 as *const libc::c_char,
            216 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*ipiv).size != (if M < N { M } else { N }) {
        gsl_error(
            b"ipiv length must equal MIN(M,N)\0" as *const u8 as *const libc::c_char,
            b"luc.c\0" as *const u8 as *const libc::c_char,
            220 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if N <= 24 as libc::c_int as libc::c_ulong {
        return LU_decomp_L2(A, ipiv)
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
        let mut AL: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
            A,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            M,
            N1,
        );
        let mut AR: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
            A,
            0 as libc::c_int as size_t,
            N1,
            M,
            N2,
        );
        let mut ipiv1: gsl_vector_uint_view = gsl_vector_uint_subvector(
            ipiv,
            0 as libc::c_int as size_t,
            N1,
        );
        let mut ipiv2: gsl_vector_uint_view = gsl_vector_uint_subvector(ipiv, N1, N2);
        let mut i: size_t = 0;
        status = LU_decomp_L3(&mut AL.matrix, &mut ipiv1.vector);
        if status != 0 {
            return status;
        }
        apply_pivots(&mut AR.matrix, &mut ipiv1.vector);
        gsl_blas_ztrsm(
            CblasLeft,
            CblasLower,
            CblasNoTrans,
            CblasUnit,
            gsl_complex_rect(1.0f64, 0.0f64),
            &mut A11.matrix,
            &mut A12.matrix,
        );
        gsl_blas_zgemm(
            CblasNoTrans,
            CblasNoTrans,
            gsl_complex_rect(-1.0f64, 0.0f64),
            &mut A21.matrix,
            &mut A12.matrix,
            gsl_complex_rect(1.0f64, 0.0f64),
            &mut A22.matrix,
        );
        status = LU_decomp_L3(&mut A22.matrix, &mut ipiv2.vector);
        if status != 0 {
            return status;
        }
        apply_pivots(&mut A21.matrix, &mut ipiv2.vector);
        i = 0 as libc::c_int as size_t;
        while i < N2 {
            let mut ptr: *mut libc::c_uint = gsl_vector_uint_ptr(&mut ipiv2.vector, i);
            *ptr = (*ptr as libc::c_ulong).wrapping_add(N1) as libc::c_uint
                as libc::c_uint;
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_complex_LU_solve(
    mut LU: *const gsl_matrix_complex,
    mut p: *const gsl_permutation,
    mut b: *const gsl_vector_complex,
    mut x: *mut gsl_vector_complex,
) -> libc::c_int {
    if (*LU).size1 != (*LU).size2 {
        gsl_error(
            b"LU matrix must be square\0" as *const u8 as *const libc::c_char,
            b"luc.c\0" as *const u8 as *const libc::c_char,
            299 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*LU).size1 != (*p).size {
        gsl_error(
            b"permutation length must match matrix size\0" as *const u8
                as *const libc::c_char,
            b"luc.c\0" as *const u8 as *const libc::c_char,
            303 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*LU).size1 != (*b).size {
        gsl_error(
            b"matrix size must match b size\0" as *const u8 as *const libc::c_char,
            b"luc.c\0" as *const u8 as *const libc::c_char,
            307 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*LU).size2 != (*x).size {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8
                as *const libc::c_char,
            b"luc.c\0" as *const u8 as *const libc::c_char,
            311 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if singular(LU) != 0 {
        gsl_error(
            b"matrix is singular\0" as *const u8 as *const libc::c_char,
            b"luc.c\0" as *const u8 as *const libc::c_char,
            315 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        gsl_vector_complex_memcpy(x, b);
        status = gsl_linalg_complex_LU_svx(LU, p, x);
        return status;
    };
}
pub unsafe extern "C" fn gsl_linalg_complex_LU_svx(
    mut LU: *const gsl_matrix_complex,
    mut p: *const gsl_permutation,
    mut x: *mut gsl_vector_complex,
) -> libc::c_int {
    if (*LU).size1 != (*LU).size2 {
        gsl_error(
            b"LU matrix must be square\0" as *const u8 as *const libc::c_char,
            b"luc.c\0" as *const u8 as *const libc::c_char,
            337 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*LU).size1 != (*p).size {
        gsl_error(
            b"permutation length must match matrix size\0" as *const u8
                as *const libc::c_char,
            b"luc.c\0" as *const u8 as *const libc::c_char,
            341 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*LU).size1 != (*x).size {
        gsl_error(
            b"matrix size must match solution/rhs size\0" as *const u8
                as *const libc::c_char,
            b"luc.c\0" as *const u8 as *const libc::c_char,
            345 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if singular(LU) != 0 {
        gsl_error(
            b"matrix is singular\0" as *const u8 as *const libc::c_char,
            b"luc.c\0" as *const u8 as *const libc::c_char,
            349 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        gsl_permute_vector_complex(p, x);
        gsl_blas_ztrsv(CblasLower, CblasNoTrans, CblasUnit, LU, x);
        gsl_blas_ztrsv(CblasUpper, CblasNoTrans, CblasNonUnit, LU, x);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_complex_LU_refine(
    mut A: *const gsl_matrix_complex,
    mut LU: *const gsl_matrix_complex,
    mut p: *const gsl_permutation,
    mut b: *const gsl_vector_complex,
    mut x: *mut gsl_vector_complex,
    mut work: *mut gsl_vector_complex,
) -> libc::c_int {
    if (*A).size1 != (*A).size2 {
        gsl_error(
            b"matrix a must be square\0" as *const u8 as *const libc::c_char,
            b"luc.c\0" as *const u8 as *const libc::c_char,
            372 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    if (*LU).size1 != (*LU).size2 {
        gsl_error(
            b"LU matrix must be square\0" as *const u8 as *const libc::c_char,
            b"luc.c\0" as *const u8 as *const libc::c_char,
            376 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*A).size1 != (*LU).size2 {
        gsl_error(
            b"LU matrix must be decomposition of a\0" as *const u8
                as *const libc::c_char,
            b"luc.c\0" as *const u8 as *const libc::c_char,
            380 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*LU).size1 != (*p).size {
        gsl_error(
            b"permutation length must match matrix size\0" as *const u8
                as *const libc::c_char,
            b"luc.c\0" as *const u8 as *const libc::c_char,
            384 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*LU).size1 != (*b).size {
        gsl_error(
            b"matrix size must match b size\0" as *const u8 as *const libc::c_char,
            b"luc.c\0" as *const u8 as *const libc::c_char,
            388 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*LU).size1 != (*x).size {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8
                as *const libc::c_char,
            b"luc.c\0" as *const u8 as *const libc::c_char,
            392 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*LU).size1 != (*work).size {
        gsl_error(
            b"matrix size must match workspace size\0" as *const u8
                as *const libc::c_char,
            b"luc.c\0" as *const u8 as *const libc::c_char,
            396 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if singular(LU) != 0 {
        gsl_error(
            b"matrix is singular\0" as *const u8 as *const libc::c_char,
            b"luc.c\0" as *const u8 as *const libc::c_char,
            400 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        gsl_vector_complex_memcpy(work, b);
        let mut one: gsl_complex = gsl_complex_rect(1.0f64, 0.0f64);
        let mut negone: gsl_complex = gsl_complex_rect(-1.0f64, 0.0f64);
        gsl_blas_zgemv(CblasNoTrans, one, A, x, negone, work);
        status = gsl_linalg_complex_LU_svx(LU, p, work);
        let mut negone_0: gsl_complex = gsl_complex_rect(-1.0f64, 0.0f64);
        gsl_blas_zaxpy(negone_0, work, x);
        return status;
    };
}
pub unsafe extern "C" fn gsl_linalg_complex_LU_invert(
    mut LU: *const gsl_matrix_complex,
    mut p: *const gsl_permutation,
    mut inverse: *mut gsl_matrix_complex,
) -> libc::c_int {
    if (*LU).size1 != (*LU).size2 {
        gsl_error(
            b"LU matrix must be square\0" as *const u8 as *const libc::c_char,
            b"luc.c\0" as *const u8 as *const libc::c_char,
            434 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*LU).size1 != (*p).size {
        gsl_error(
            b"permutation length must match matrix size\0" as *const u8
                as *const libc::c_char,
            b"luc.c\0" as *const u8 as *const libc::c_char,
            438 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*inverse).size1 != (*LU).size1 || (*inverse).size2 != (*LU).size2 {
        gsl_error(
            b"inverse matrix must match LU matrix dimensions\0" as *const u8
                as *const libc::c_char,
            b"luc.c\0" as *const u8 as *const libc::c_char,
            442 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        gsl_matrix_complex_memcpy(inverse, LU);
        return gsl_linalg_complex_LU_invx(inverse, p);
    };
}
pub unsafe extern "C" fn gsl_linalg_complex_LU_invx(
    mut LU: *mut gsl_matrix_complex,
    mut p: *const gsl_permutation,
) -> libc::c_int {
    if (*LU).size1 != (*LU).size2 {
        gsl_error(
            b"LU matrix must be square\0" as *const u8 as *const libc::c_char,
            b"luc.c\0" as *const u8 as *const libc::c_char,
            456 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*LU).size1 != (*p).size {
        gsl_error(
            b"permutation length must match matrix size\0" as *const u8
                as *const libc::c_char,
            b"luc.c\0" as *const u8 as *const libc::c_char,
            460 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if singular(LU) != 0 {
        gsl_error(
            b"matrix is singular\0" as *const u8 as *const libc::c_char,
            b"luc.c\0" as *const u8 as *const libc::c_char,
            464 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let N: size_t = (*LU).size1;
        let mut i: size_t = 0;
        status = gsl_linalg_complex_tri_invert(CblasUpper, CblasNonUnit, LU);
        if status != 0 {
            return status;
        }
        status = gsl_linalg_complex_tri_invert(CblasLower, CblasUnit, LU);
        if status != 0 {
            return status;
        }
        status = gsl_linalg_complex_tri_UL(LU);
        if status != 0 {
            return status;
        }
        i = 0 as libc::c_int as size_t;
        while i < N {
            let mut v: gsl_vector_complex_view = gsl_matrix_complex_row(LU, i);
            gsl_permute_vector_complex_inverse(p, &mut v.vector);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_complex_LU_det(
    mut LU: *mut gsl_matrix_complex,
    mut signum: libc::c_int,
) -> gsl_complex {
    let mut i: size_t = 0;
    let mut n: size_t = (*LU).size1;
    let mut det: gsl_complex = gsl_complex_rect(signum as libc::c_double, 0.0f64);
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut zi: gsl_complex = gsl_matrix_complex_get(LU, i, i);
        det = gsl_complex_mul(det, zi);
        i = i.wrapping_add(1);
        i;
    }
    return det;
}
pub unsafe extern "C" fn gsl_linalg_complex_LU_lndet(
    mut LU: *mut gsl_matrix_complex,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut n: size_t = (*LU).size1;
    let mut lndet: libc::c_double = 0.0f64;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut z: gsl_complex = gsl_matrix_complex_get(LU, i, i);
        lndet += log(gsl_complex_abs(z));
        i = i.wrapping_add(1);
        i;
    }
    return lndet;
}
pub unsafe extern "C" fn gsl_linalg_complex_LU_sgndet(
    mut LU: *mut gsl_matrix_complex,
    mut signum: libc::c_int,
) -> gsl_complex {
    let mut i: size_t = 0;
    let mut n: size_t = (*LU).size1;
    let mut phase: gsl_complex = gsl_complex_rect(signum as libc::c_double, 0.0f64);
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut z: gsl_complex = gsl_matrix_complex_get(LU, i, i);
        let mut r: libc::c_double = gsl_complex_abs(z);
        if r == 0 as libc::c_int as libc::c_double {
            phase = gsl_complex_rect(0.0f64, 0.0f64);
            break;
        } else {
            z = gsl_complex_div_real(z, r);
            phase = gsl_complex_mul(phase, z);
            i = i.wrapping_add(1);
            i;
        }
    }
    return phase;
}
unsafe extern "C" fn singular(mut LU: *const gsl_matrix_complex) -> libc::c_int {
    let mut i: size_t = 0;
    let mut n: size_t = (*LU).size1;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut u: gsl_complex = gsl_matrix_complex_get(LU, i, i);
        if u.dat[0 as libc::c_int as usize] == 0 as libc::c_int as libc::c_double
            && u.dat[1 as libc::c_int as usize] == 0 as libc::c_int as libc::c_double
        {
            return 1 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn apply_pivots(
    mut A: *mut gsl_matrix_complex,
    mut ipiv: *const gsl_vector_uint,
) -> libc::c_int {
    if (*A).size1 < (*ipiv).size {
        gsl_error(
            b"matrix does not match pivot vector\0" as *const u8 as *const libc::c_char,
            b"luc.c\0" as *const u8 as *const libc::c_char,
            579 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < (*ipiv).size {
            let mut pi: size_t = gsl_vector_uint_get(ipiv, i) as size_t;
            if i != pi {
                let mut v1: gsl_vector_complex_view = gsl_matrix_complex_row(A, i);
                let mut v2: gsl_vector_complex_view = gsl_matrix_complex_row(A, pi);
                gsl_blas_zswap(&mut v1.vector, &mut v2.vector);
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
