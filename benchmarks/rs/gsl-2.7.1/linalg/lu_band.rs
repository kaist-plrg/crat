use ::libc;
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_view_array_with_stride(
        base: *mut libc::c_double,
        stride: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
    fn gsl_vector_subvector(
        v: *mut gsl_vector,
        i: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
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
    fn gsl_matrix_row(m: *mut gsl_matrix, i: size_t) -> _gsl_vector_view;
    fn gsl_matrix_superdiagonal(m: *mut gsl_matrix, k: size_t) -> _gsl_vector_view;
    fn gsl_matrix_subrow(
        m: *mut gsl_matrix,
        i: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
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
    fn gsl_matrix_set_zero(m: *mut gsl_matrix);
    fn gsl_matrix_set_identity(m: *mut gsl_matrix);
    fn gsl_blas_idamax(X: *const gsl_vector) -> CBLAS_INDEX_t;
    fn gsl_blas_dswap(X: *mut gsl_vector, Y: *mut gsl_vector) -> libc::c_int;
    fn gsl_blas_daxpy(
        alpha: libc::c_double,
        X: *const gsl_vector,
        Y: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_blas_dscal(alpha: libc::c_double, X: *mut gsl_vector);
    fn gsl_blas_dger(
        alpha: libc::c_double,
        X: *const gsl_vector,
        Y: *const gsl_vector,
        A: *mut gsl_matrix,
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
unsafe extern "C" fn gsl_vector_ptr(
    mut v: *mut gsl_vector,
    i: size_t,
) -> *mut libc::c_double {
    return ((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
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
unsafe extern "C" fn gsl_matrix_get(
    mut m: *const gsl_matrix,
    i: size_t,
    j: size_t,
) -> libc::c_double {
    return *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
#[inline]
unsafe extern "C" fn gsl_matrix_ptr(
    mut m: *mut gsl_matrix,
    i: size_t,
    j: size_t,
) -> *mut libc::c_double {
    return ((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
pub unsafe extern "C" fn gsl_linalg_LU_band_decomp(
    M: size_t,
    lb: size_t,
    ub: size_t,
    mut AB: *mut gsl_matrix,
    mut piv: *mut gsl_vector_uint,
) -> libc::c_int {
    let N: size_t = (*AB).size1;
    let minMN: size_t = if M < N { M } else { N };
    if lb >= M {
        gsl_error(
            b"lower bandwidth must be less than M\0" as *const u8 as *const libc::c_char,
            b"lu_band.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if ub >= N {
        gsl_error(
            b"upper bandwidth must be less than N\0" as *const u8 as *const libc::c_char,
            b"lu_band.c\0" as *const u8 as *const libc::c_char,
            68 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if (*AB).size2
        != (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(lb)
            .wrapping_add(ub)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        gsl_error(
            b"matrix size inconsistent with bandwidths\0" as *const u8
                as *const libc::c_char,
            b"lu_band.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*piv).size != minMN {
        gsl_error(
            b"pivot vector must have length MIN(M,N)\0" as *const u8
                as *const libc::c_char,
            b"lu_band.c\0" as *const u8 as *const libc::c_char,
            76 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        status = LU_band_decomp_L2(M, lb, ub, AB, piv);
        return status;
    };
}
pub unsafe extern "C" fn gsl_linalg_LU_band_solve(
    lb: size_t,
    ub: size_t,
    mut LUB: *const gsl_matrix,
    mut piv: *const gsl_vector_uint,
    mut b: *const gsl_vector,
    mut x: *mut gsl_vector,
) -> libc::c_int {
    let N: size_t = (*LUB).size1;
    if N != (*x).size {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8
                as *const libc::c_char,
            b"lu_band.c\0" as *const u8 as *const libc::c_char,
            96 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if N != (*b).size {
        gsl_error(
            b"matrix size must match rhs size\0" as *const u8 as *const libc::c_char,
            b"lu_band.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if lb >= N {
        gsl_error(
            b"lower bandwidth must be less than N\0" as *const u8 as *const libc::c_char,
            b"lu_band.c\0" as *const u8 as *const libc::c_char,
            104 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if ub >= N {
        gsl_error(
            b"upper bandwidth must be less than N\0" as *const u8 as *const libc::c_char,
            b"lu_band.c\0" as *const u8 as *const libc::c_char,
            108 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if (*LUB).size2
        != (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(lb)
            .wrapping_add(ub)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        gsl_error(
            b"matrix size inconsistent with bandwidths\0" as *const u8
                as *const libc::c_char,
            b"lu_band.c\0" as *const u8 as *const libc::c_char,
            112 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*piv).size != N {
        gsl_error(
            b"pivot vector must have length N\0" as *const u8 as *const libc::c_char,
            b"lu_band.c\0" as *const u8 as *const libc::c_char,
            116 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        gsl_vector_memcpy(x, b);
        status = gsl_linalg_LU_band_svx(lb, ub, LUB, piv, x);
        return status;
    };
}
pub unsafe extern "C" fn gsl_linalg_LU_band_svx(
    lb: size_t,
    ub: size_t,
    mut LUB: *const gsl_matrix,
    mut piv: *const gsl_vector_uint,
    mut x: *mut gsl_vector,
) -> libc::c_int {
    let N: size_t = (*LUB).size1;
    if N != (*x).size {
        gsl_error(
            b"matrix size must match solution/rhs size\0" as *const u8
                as *const libc::c_char,
            b"lu_band.c\0" as *const u8 as *const libc::c_char,
            138 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if lb >= N {
        gsl_error(
            b"lower bandwidth must be less than N\0" as *const u8 as *const libc::c_char,
            b"lu_band.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if ub >= N {
        gsl_error(
            b"upper bandwidth must be less than N\0" as *const u8 as *const libc::c_char,
            b"lu_band.c\0" as *const u8 as *const libc::c_char,
            146 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if (*LUB).size2
        != (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(lb)
            .wrapping_add(ub)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        gsl_error(
            b"matrix size inconsistent with bandwidths\0" as *const u8
                as *const libc::c_char,
            b"lu_band.c\0" as *const u8 as *const libc::c_char,
            150 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*piv).size != N {
        gsl_error(
            b"pivot vector must have length N\0" as *const u8 as *const libc::c_char,
            b"lu_band.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        if lb > 0 as libc::c_int as libc::c_ulong {
            let mut j: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < N.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                let mut pj: size_t = gsl_vector_uint_get(piv, j) as size_t;
                let mut xj: *mut libc::c_double = gsl_vector_ptr(x, j);
                let mut lm: size_t = if lb
                    < N.wrapping_sub(j).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                {
                    lb
                } else {
                    N.wrapping_sub(j).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                };
                let mut xv: gsl_vector_view = gsl_vector_subvector(
                    x,
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    lm,
                );
                let yv: gsl_vector_const_view = gsl_matrix_const_subrow(
                    LUB,
                    j,
                    lb.wrapping_add(ub).wrapping_add(1 as libc::c_int as libc::c_ulong),
                    lm,
                );
                if j != pj {
                    let mut xl: libc::c_double = gsl_vector_get(x, pj);
                    gsl_vector_set(x, pj, *xj);
                    *xj = xl;
                }
                gsl_blas_daxpy(-*xj, &yv.vector, &mut xv.vector);
                j = j.wrapping_add(1);
                j;
            }
        }
        cblas_dtbsv(
            CblasColMajor,
            CblasUpper,
            CblasNoTrans,
            CblasNonUnit,
            N as libc::c_int,
            lb.wrapping_add(ub) as libc::c_int,
            (*LUB).data,
            (*LUB).tda as libc::c_int,
            (*x).data,
            (*x).stride as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_LU_band_unpack(
    M: size_t,
    lb: size_t,
    ub: size_t,
    mut LUB: *const gsl_matrix,
    mut piv: *const gsl_vector_uint,
    mut L: *mut gsl_matrix,
    mut U: *mut gsl_matrix,
) -> libc::c_int {
    let N: size_t = (*LUB).size1;
    let minMN: size_t = if M < N { M } else { N };
    if ub >= N {
        gsl_error(
            b"upper bandwidth must be < N\0" as *const u8 as *const libc::c_char,
            b"lu_band.c\0" as *const u8 as *const libc::c_char,
            199 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if lb >= M {
        gsl_error(
            b"lower bandwidth must be < M\0" as *const u8 as *const libc::c_char,
            b"lu_band.c\0" as *const u8 as *const libc::c_char,
            203 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if (*LUB).size2
        != (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(lb)
            .wrapping_add(ub)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        gsl_error(
            b"matrix size inconsistent with bandwidths\0" as *const u8
                as *const libc::c_char,
            b"lu_band.c\0" as *const u8 as *const libc::c_char,
            207 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*piv).size != minMN {
        gsl_error(
            b"pivot vector must have length MIN(M,N)\0" as *const u8
                as *const libc::c_char,
            b"lu_band.c\0" as *const u8 as *const libc::c_char,
            211 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*L).size1 != M || (*L).size2 != minMN {
        gsl_error(
            b"L matrix has wrong dimensions\0" as *const u8 as *const libc::c_char,
            b"lu_band.c\0" as *const u8 as *const libc::c_char,
            215 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*U).size1 != minMN || (*U).size2 != N {
        gsl_error(
            b"U matrix has wrong dimensions\0" as *const u8 as *const libc::c_char,
            b"lu_band.c\0" as *const u8 as *const libc::c_char,
            219 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let ub_U: size_t = lb.wrapping_add(ub);
        let mut j: size_t = 0;
        gsl_matrix_set_identity(L);
        gsl_matrix_set_zero(U);
        if lb > 0 as libc::c_int as libc::c_ulong {
            let jstart: size_t = if M > N {
                minMN
            } else {
                minMN.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            };
            let mut j_0: size_t = 0;
            j_0 = jstart;
            while j_0 > 0 as libc::c_int as libc::c_ulong
                && {
                    let fresh0 = j_0;
                    j_0 = j_0.wrapping_sub(1);
                    fresh0 != 0
                }
            {
                let mut pj: size_t = gsl_vector_uint_get(piv, j_0) as size_t;
                let mut lm: size_t = if lb
                    < M.wrapping_sub(j_0).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                {
                    lb
                } else {
                    M.wrapping_sub(j_0).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                };
                let xv: gsl_vector_const_view = gsl_matrix_const_subrow(
                    LUB,
                    j_0,
                    lb.wrapping_add(ub).wrapping_add(1 as libc::c_int as libc::c_ulong),
                    lm,
                );
                let yv: gsl_vector_const_view = gsl_matrix_const_subrow(
                    L,
                    j_0,
                    0 as libc::c_int as size_t,
                    minMN,
                );
                let mut m: gsl_matrix_view = gsl_matrix_submatrix(
                    L,
                    j_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    0 as libc::c_int as size_t,
                    lm,
                    minMN,
                );
                gsl_blas_dger(1.0f64, &xv.vector, &yv.vector, &mut m.matrix);
                if j_0 != pj {
                    let mut Lj: gsl_vector_view = gsl_matrix_row(L, j_0);
                    let mut Lpj: gsl_vector_view = gsl_matrix_row(L, pj);
                    gsl_blas_dswap(&mut Lj.vector, &mut Lpj.vector);
                }
            }
        }
        j = 0 as libc::c_int as size_t;
        while j
            <= (if ub_U < N.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                ub_U
            } else {
                N.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            })
        {
            let src: gsl_vector_const_view = gsl_matrix_const_subcolumn(
                LUB,
                ub_U.wrapping_sub(j),
                j,
                if M < N.wrapping_sub(j) { M } else { N.wrapping_sub(j) },
            );
            let mut dest: gsl_vector_view = gsl_matrix_superdiagonal(U, j);
            gsl_vector_memcpy(&mut dest.vector, &src.vector);
            j = j.wrapping_add(1);
            j;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn LU_band_decomp_L2(
    M: size_t,
    lb: size_t,
    ub: size_t,
    mut AB: *mut gsl_matrix,
    mut ipiv: *mut gsl_vector_uint,
) -> libc::c_int {
    let N: size_t = (*AB).size1;
    let minMN: size_t = if M < N { M } else { N };
    if (*ipiv).size != minMN {
        gsl_error(
            b"ipiv length must equal MIN(M,N)\0" as *const u8 as *const libc::c_char,
            b"lu_band.c\0" as *const u8 as *const libc::c_char,
            290 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if lb >= M {
        gsl_error(
            b"lower bandwidth must be less than M\0" as *const u8 as *const libc::c_char,
            b"lu_band.c\0" as *const u8 as *const libc::c_char,
            294 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if ub >= N {
        gsl_error(
            b"upper bandwidth must be less than N\0" as *const u8 as *const libc::c_char,
            b"lu_band.c\0" as *const u8 as *const libc::c_char,
            298 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if (*AB).size2
        != (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(lb)
            .wrapping_add(ub)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        gsl_error(
            b"matrix size inconsistent with bandwidths\0" as *const u8
                as *const libc::c_char,
            b"lu_band.c\0" as *const u8 as *const libc::c_char,
            302 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let ub_U: size_t = lb.wrapping_add(ub);
        let ldab: size_t = (*AB).size2;
        let mut ju: size_t = 0 as libc::c_int as size_t;
        let mut j: size_t = 0;
        if lb > 0 as libc::c_int as libc::c_ulong {
            let mut m: gsl_matrix_view = gsl_matrix_submatrix(
                AB,
                0 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                N,
                lb,
            );
            gsl_matrix_set_zero(&mut m.matrix);
        }
        j = 0 as libc::c_int as size_t;
        while j < minMN {
            let mut lbj: size_t = if lb
                < M.wrapping_sub(j).wrapping_sub(1 as libc::c_int as libc::c_ulong)
            {
                lb
            } else {
                M.wrapping_sub(j).wrapping_sub(1 as libc::c_int as libc::c_ulong)
            };
            let mut x: gsl_vector_view = gsl_matrix_subrow(
                AB,
                j,
                ub_U,
                lbj.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            let mut y: gsl_vector_view = gsl_vector_view {
                vector: gsl_vector {
                    size: 0,
                    stride: 0,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block,
                    owner: 0,
                },
            };
            let mut j_pivot: CBLAS_INDEX_t = gsl_blas_idamax(&mut x.vector);
            let mut ptr: *mut libc::c_double = 0 as *mut libc::c_double;
            gsl_vector_uint_set(ipiv, j, j.wrapping_add(j_pivot) as libc::c_uint);
            ptr = gsl_matrix_ptr(AB, j, ub_U.wrapping_add(j_pivot));
            if *ptr != 0.0f64 {
                ju = if ju
                    > (if j.wrapping_add(ub).wrapping_add(j_pivot)
                        < N.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    {
                        j.wrapping_add(ub).wrapping_add(j_pivot)
                    } else {
                        N.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    })
                {
                    ju
                } else if j.wrapping_add(ub).wrapping_add(j_pivot)
                    < N.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                {
                    j.wrapping_add(ub).wrapping_add(j_pivot)
                } else {
                    N.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                };
            }
            if j_pivot != 0 as libc::c_int as libc::c_ulong {
                let mut ptr2: *mut libc::c_double = 0 as *mut libc::c_double;
                x = gsl_vector_view_array_with_stride(
                    ptr,
                    ldab.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ju.wrapping_sub(j).wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
                ptr2 = gsl_matrix_ptr(AB, j, ub_U);
                y = gsl_vector_view_array_with_stride(
                    ptr2,
                    ldab.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ju.wrapping_sub(j).wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
                gsl_blas_dswap(&mut x.vector, &mut y.vector);
            }
            if lbj > 0 as libc::c_int as libc::c_ulong {
                let mut tmp: libc::c_double = gsl_matrix_get(AB, j, ub_U);
                x = gsl_matrix_subrow(
                    AB,
                    j,
                    ub_U.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    lbj,
                );
                gsl_blas_dscal(1.0f64 / tmp, &mut x.vector);
                if ju > j {
                    let mut m_0: gsl_matrix_view = gsl_matrix_submatrix(
                        AB,
                        j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ub_U,
                        ju.wrapping_sub(j),
                        lbj,
                    );
                    ptr = gsl_matrix_ptr(
                        AB,
                        j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ub_U.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    );
                    y = gsl_vector_view_array_with_stride(
                        ptr,
                        ldab.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        ju.wrapping_sub(j),
                    );
                    m_0
                        .matrix
                        .tda = ldab.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                    gsl_blas_dger(
                        -1.0f64,
                        &mut y.vector,
                        &mut x.vector,
                        &mut m_0.matrix,
                    );
                }
            }
            j = j.wrapping_add(1);
            j;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
