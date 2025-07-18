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
    fn gsl_vector_subvector(
        v: *mut gsl_vector,
        i: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_max_index(v: *const gsl_vector) -> size_t;
    fn gsl_vector_mul(a: *mut gsl_vector, b: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_div(a: *mut gsl_vector, b: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_scale(a: *mut gsl_vector, x: libc::c_double) -> libc::c_int;
    fn gsl_matrix_submatrix(
        m: *mut gsl_matrix,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_view;
    fn gsl_matrix_row(m: *mut gsl_matrix, i: size_t) -> _gsl_vector_view;
    fn gsl_matrix_column(m: *mut gsl_matrix, j: size_t) -> _gsl_vector_view;
    fn gsl_matrix_diagonal(m: *mut gsl_matrix) -> _gsl_vector_view;
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
    fn gsl_matrix_memcpy(dest: *mut gsl_matrix, src: *const gsl_matrix) -> libc::c_int;
    fn gsl_matrix_transpose_tricpy(
        Uplo_src: CBLAS_UPLO_t,
        Diag: CBLAS_DIAG_t,
        dest: *mut gsl_matrix,
        src: *const gsl_matrix,
    ) -> libc::c_int;
    fn gsl_blas_dscal(alpha: libc::c_double, X: *mut gsl_vector);
    fn gsl_blas_dtrsv(
        Uplo: CBLAS_UPLO_t,
        TransA: CBLAS_TRANSPOSE_t,
        Diag: CBLAS_DIAG_t,
        A: *const gsl_matrix,
        X: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_blas_dsyr(
        Uplo: CBLAS_UPLO_t,
        alpha: libc::c_double,
        X: *const gsl_vector,
        A: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_linalg_cholesky_scale(
        A: *const gsl_matrix,
        S: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_linalg_cholesky_scale_apply(
        A: *mut gsl_matrix,
        S: *const gsl_vector,
    ) -> libc::c_int;
    fn gsl_permutation_init(p: *mut gsl_permutation);
    fn gsl_linalg_tri_LTL(L: *mut gsl_matrix) -> libc::c_int;
    fn gsl_linalg_tri_invert(
        Uplo: CBLAS_UPLO_t,
        Diag: CBLAS_DIAG_t,
        T: *mut gsl_matrix,
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
    fn gsl_permutation_swap(
        p: *mut gsl_permutation,
        i: size_t,
        j: size_t,
    ) -> libc::c_int;
    fn gsl_permute_vector_inverse(
        p: *const gsl_permutation,
        v: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_permute_vector(p: *const gsl_permutation, v: *mut gsl_vector) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_permutation_struct {
    pub size: size_t,
    pub data: *mut size_t,
}
pub type gsl_permutation = gsl_permutation_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pcholesky_params {
    pub LDLT: *const gsl_matrix,
    pub perm: *const gsl_permutation,
}
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
unsafe extern "C" fn cholesky_swap_rowcol(
    mut A: *mut gsl_matrix,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    if i != j {
        let N: size_t = (*A).size1;
        let mut Aii: *mut libc::c_double = 0 as *mut libc::c_double;
        let mut Ajj: *mut libc::c_double = 0 as *mut libc::c_double;
        let mut ii: size_t = 0;
        let mut jj: size_t = 0;
        let mut k: size_t = 0;
        if i < j {
            ii = i;
            jj = j;
        } else {
            ii = j;
            jj = i;
        }
        k = 0 as libc::c_int as size_t;
        while k < ii {
            let mut Aik: *mut libc::c_double = gsl_matrix_ptr(A, ii, k);
            let mut Ajk: *mut libc::c_double = gsl_matrix_ptr(A, jj, k);
            let mut tmp: libc::c_double = *Ajk;
            *Ajk = *Aik;
            *Aik = tmp;
            k = k.wrapping_add(1);
            k;
        }
        k = ii.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while k < jj {
            let mut Ajk_0: *mut libc::c_double = gsl_matrix_ptr(A, jj, k);
            let mut Aki: *mut libc::c_double = gsl_matrix_ptr(A, k, ii);
            let mut tmp_0: libc::c_double = *Aki;
            *Aki = *Ajk_0;
            *Ajk_0 = tmp_0;
            k = k.wrapping_add(1);
            k;
        }
        k = jj.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while k < N {
            let mut Aki_0: *mut libc::c_double = gsl_matrix_ptr(A, k, ii);
            let mut Akj: *mut libc::c_double = gsl_matrix_ptr(A, k, jj);
            let mut tmp_1: libc::c_double = *Akj;
            *Akj = *Aki_0;
            *Aki_0 = tmp_1;
            k = k.wrapping_add(1);
            k;
        }
        Aii = gsl_matrix_ptr(A, ii, ii);
        Ajj = gsl_matrix_ptr(A, jj, jj);
        let mut tmp_2: libc::c_double = *Ajj;
        *Ajj = *Aii;
        *Aii = tmp_2;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn pcholesky_decomp(
    copy_uplo: libc::c_int,
    mut A: *mut gsl_matrix,
    mut p: *mut gsl_permutation,
) -> libc::c_int {
    let N: size_t = (*A).size1;
    if N != (*A).size2 {
        gsl_error(
            b"LDLT decomposition requires square matrix\0" as *const u8
                as *const libc::c_char,
            b"pcholesky.c\0" as *const u8 as *const libc::c_char,
            78 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*p).size != N {
        gsl_error(
            b"permutation length must match matrix size\0" as *const u8
                as *const libc::c_char,
            b"pcholesky.c\0" as *const u8 as *const libc::c_char,
            82 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut diag: gsl_vector_view = gsl_matrix_diagonal(A);
        let mut k: size_t = 0;
        if copy_uplo != 0 {
            gsl_matrix_transpose_tricpy(CblasLower, CblasUnit, A, A);
        }
        gsl_permutation_init(p);
        k = 0 as libc::c_int as size_t;
        while k < N {
            let mut w: gsl_vector_view = gsl_vector_view {
                vector: gsl_vector {
                    size: 0,
                    stride: 0,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block,
                    owner: 0,
                },
            };
            let mut j: size_t = 0;
            w = gsl_vector_subvector(&mut diag.vector, k, N.wrapping_sub(k));
            j = (gsl_vector_max_index(&mut w.vector)).wrapping_add(k);
            gsl_permutation_swap(p, k, j);
            cholesky_swap_rowcol(A, k, j);
            if k < N.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                let mut alpha: libc::c_double = gsl_matrix_get(A, k, k);
                let mut alphainv: libc::c_double = 1.0f64 / alpha;
                let mut v: gsl_vector_view = gsl_matrix_subcolumn(
                    A,
                    k,
                    k.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    N.wrapping_sub(k).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                let mut m: gsl_matrix_view = gsl_matrix_submatrix(
                    A,
                    k.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    k.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    N.wrapping_sub(k).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    N.wrapping_sub(k).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                gsl_blas_dsyr(CblasLower, -alphainv, &mut v.vector, &mut m.matrix);
                gsl_vector_scale(&mut v.vector, alphainv);
            }
            k = k.wrapping_add(1);
            k;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_pcholesky_decomp(
    mut A: *mut gsl_matrix,
    mut p: *mut gsl_permutation,
) -> libc::c_int {
    let mut status: libc::c_int = pcholesky_decomp(1 as libc::c_int, A, p);
    return status;
}
pub unsafe extern "C" fn gsl_linalg_pcholesky_solve(
    mut LDLT: *const gsl_matrix,
    mut p: *const gsl_permutation,
    mut b: *const gsl_vector,
    mut x: *mut gsl_vector,
) -> libc::c_int {
    if (*LDLT).size1 != (*LDLT).size2 {
        gsl_error(
            b"LDLT matrix must be square\0" as *const u8 as *const libc::c_char,
            b"pcholesky.c\0" as *const u8 as *const libc::c_char,
            164 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*LDLT).size1 != (*p).size {
        gsl_error(
            b"matrix size must match permutation size\0" as *const u8
                as *const libc::c_char,
            b"pcholesky.c\0" as *const u8 as *const libc::c_char,
            168 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*LDLT).size1 != (*b).size {
        gsl_error(
            b"matrix size must match b size\0" as *const u8 as *const libc::c_char,
            b"pcholesky.c\0" as *const u8 as *const libc::c_char,
            172 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*LDLT).size2 != (*x).size {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8
                as *const libc::c_char,
            b"pcholesky.c\0" as *const u8 as *const libc::c_char,
            176 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        gsl_vector_memcpy(x, b);
        status = gsl_linalg_pcholesky_svx(LDLT, p, x);
        return status;
    };
}
pub unsafe extern "C" fn gsl_linalg_pcholesky_svx(
    mut LDLT: *const gsl_matrix,
    mut p: *const gsl_permutation,
    mut x: *mut gsl_vector,
) -> libc::c_int {
    if (*LDLT).size1 != (*LDLT).size2 {
        gsl_error(
            b"LDLT matrix must be square\0" as *const u8 as *const libc::c_char,
            b"pcholesky.c\0" as *const u8 as *const libc::c_char,
            197 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*LDLT).size1 != (*p).size {
        gsl_error(
            b"matrix size must match permutation size\0" as *const u8
                as *const libc::c_char,
            b"pcholesky.c\0" as *const u8 as *const libc::c_char,
            201 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*LDLT).size2 != (*x).size {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8
                as *const libc::c_char,
            b"pcholesky.c\0" as *const u8 as *const libc::c_char,
            205 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let D: gsl_vector_const_view = gsl_matrix_const_diagonal(LDLT);
        gsl_permute_vector(p, x);
        gsl_blas_dtrsv(CblasLower, CblasNoTrans, CblasUnit, LDLT, x);
        gsl_vector_div(x, &D.vector);
        gsl_blas_dtrsv(CblasLower, CblasTrans, CblasUnit, LDLT, x);
        gsl_permute_vector_inverse(p, x);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_pcholesky_decomp2(
    mut A: *mut gsl_matrix,
    mut p: *mut gsl_permutation,
    mut S: *mut gsl_vector,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if M != N {
        gsl_error(
            b"cholesky decomposition requires square matrix\0" as *const u8
                as *const libc::c_char,
            b"pcholesky.c\0" as *const u8 as *const libc::c_char,
            239 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != (*p).size {
        gsl_error(
            b"matrix size must match permutation size\0" as *const u8
                as *const libc::c_char,
            b"pcholesky.c\0" as *const u8 as *const libc::c_char,
            243 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if N != (*S).size {
        gsl_error(
            b"S must have length N\0" as *const u8 as *const libc::c_char,
            b"pcholesky.c\0" as *const u8 as *const libc::c_char,
            247 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        gsl_matrix_transpose_tricpy(CblasLower, CblasUnit, A, A);
        status = gsl_linalg_cholesky_scale(A, S);
        if status != 0 {
            return status;
        }
        status = gsl_linalg_cholesky_scale_apply(A, S);
        if status != 0 {
            return status;
        }
        status = pcholesky_decomp(0 as libc::c_int, A, p);
        if status != 0 {
            return status;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_pcholesky_solve2(
    mut LDLT: *const gsl_matrix,
    mut p: *const gsl_permutation,
    mut S: *const gsl_vector,
    mut b: *const gsl_vector,
    mut x: *mut gsl_vector,
) -> libc::c_int {
    if (*LDLT).size1 != (*LDLT).size2 {
        gsl_error(
            b"LDLT matrix must be square\0" as *const u8 as *const libc::c_char,
            b"pcholesky.c\0" as *const u8 as *const libc::c_char,
            284 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*LDLT).size1 != (*p).size {
        gsl_error(
            b"matrix size must match permutation size\0" as *const u8
                as *const libc::c_char,
            b"pcholesky.c\0" as *const u8 as *const libc::c_char,
            288 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*LDLT).size1 != (*S).size {
        gsl_error(
            b"matrix size must match S\0" as *const u8 as *const libc::c_char,
            b"pcholesky.c\0" as *const u8 as *const libc::c_char,
            292 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*LDLT).size1 != (*b).size {
        gsl_error(
            b"matrix size must match b size\0" as *const u8 as *const libc::c_char,
            b"pcholesky.c\0" as *const u8 as *const libc::c_char,
            296 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*LDLT).size2 != (*x).size {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8
                as *const libc::c_char,
            b"pcholesky.c\0" as *const u8 as *const libc::c_char,
            300 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        gsl_vector_memcpy(x, b);
        status = gsl_linalg_pcholesky_svx2(LDLT, p, S, x);
        return status;
    };
}
pub unsafe extern "C" fn gsl_linalg_pcholesky_svx2(
    mut LDLT: *const gsl_matrix,
    mut p: *const gsl_permutation,
    mut S: *const gsl_vector,
    mut x: *mut gsl_vector,
) -> libc::c_int {
    if (*LDLT).size1 != (*LDLT).size2 {
        gsl_error(
            b"LDLT matrix must be square\0" as *const u8 as *const libc::c_char,
            b"pcholesky.c\0" as *const u8 as *const libc::c_char,
            322 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*LDLT).size1 != (*p).size {
        gsl_error(
            b"matrix size must match permutation size\0" as *const u8
                as *const libc::c_char,
            b"pcholesky.c\0" as *const u8 as *const libc::c_char,
            326 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*LDLT).size1 != (*S).size {
        gsl_error(
            b"matrix size must match S\0" as *const u8 as *const libc::c_char,
            b"pcholesky.c\0" as *const u8 as *const libc::c_char,
            330 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*LDLT).size2 != (*x).size {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8
                as *const libc::c_char,
            b"pcholesky.c\0" as *const u8 as *const libc::c_char,
            334 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        gsl_vector_mul(x, S);
        status = gsl_linalg_pcholesky_svx(LDLT, p, x);
        if status != 0 {
            return status;
        }
        gsl_vector_mul(x, S);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_pcholesky_invert(
    mut LDLT: *const gsl_matrix,
    mut p: *const gsl_permutation,
    mut Ainv: *mut gsl_matrix,
) -> libc::c_int {
    let M: size_t = (*LDLT).size1;
    let N: size_t = (*LDLT).size2;
    if M != N {
        gsl_error(
            b"LDLT matrix must be square\0" as *const u8 as *const libc::c_char,
            b"pcholesky.c\0" as *const u8 as *const libc::c_char,
            376 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*LDLT).size1 != (*p).size {
        gsl_error(
            b"matrix size must match permutation size\0" as *const u8
                as *const libc::c_char,
            b"pcholesky.c\0" as *const u8 as *const libc::c_char,
            380 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*Ainv).size1 != (*Ainv).size2 {
        gsl_error(
            b"Ainv matrix must be square\0" as *const u8 as *const libc::c_char,
            b"pcholesky.c\0" as *const u8 as *const libc::c_char,
            384 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*Ainv).size1 != M {
        gsl_error(
            b"Ainv matrix has wrong dimensions\0" as *const u8 as *const libc::c_char,
            b"pcholesky.c\0" as *const u8 as *const libc::c_char,
            388 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        gsl_matrix_memcpy(Ainv, LDLT);
        gsl_linalg_tri_invert(CblasLower, CblasUnit, Ainv);
        i = 0 as libc::c_int as size_t;
        while i < N {
            let mut di: libc::c_double = gsl_matrix_get(LDLT, i, i);
            let mut invsqrt_di: libc::c_double = 1.0f64 / sqrt(di);
            if i > 0 as libc::c_int as libc::c_ulong {
                let mut v: gsl_vector_view = gsl_matrix_subrow(
                    Ainv,
                    i,
                    0 as libc::c_int as size_t,
                    i,
                );
                gsl_blas_dscal(invsqrt_di, &mut v.vector);
            }
            gsl_matrix_set(Ainv, i, i, invsqrt_di);
            i = i.wrapping_add(1);
            i;
        }
        gsl_linalg_tri_LTL(Ainv);
        gsl_matrix_transpose_tricpy(CblasLower, CblasUnit, Ainv, Ainv);
        i = 0 as libc::c_int as size_t;
        while i < N {
            let mut v_0: gsl_vector_view = gsl_matrix_row(Ainv, i);
            gsl_permute_vector_inverse(p, &mut v_0.vector);
            i = i.wrapping_add(1);
            i;
        }
        i = 0 as libc::c_int as size_t;
        while i < N {
            let mut v_1: gsl_vector_view = gsl_matrix_column(Ainv, i);
            gsl_permute_vector_inverse(p, &mut v_1.vector);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_pcholesky_rcond(
    mut LDLT: *const gsl_matrix,
    mut p: *const gsl_permutation,
    mut rcond: *mut libc::c_double,
    mut work: *mut gsl_vector,
) -> libc::c_int {
    let M: size_t = (*LDLT).size1;
    let N: size_t = (*LDLT).size2;
    if M != N {
        gsl_error(
            b"cholesky matrix must be square\0" as *const u8 as *const libc::c_char,
            b"pcholesky.c\0" as *const u8 as *const libc::c_char,
            448 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*work).size != (3 as libc::c_int as libc::c_ulong).wrapping_mul(N) {
        gsl_error(
            b"work vector must have length 3*N\0" as *const u8 as *const libc::c_char,
            b"pcholesky.c\0" as *const u8 as *const libc::c_char,
            452 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut Anorm: libc::c_double = cholesky_LDLT_norm1(LDLT, p, work);
        let mut Ainvnorm: libc::c_double = 0.;
        let mut params: pcholesky_params = pcholesky_params {
            LDLT: 0 as *const gsl_matrix,
            perm: 0 as *const gsl_permutation,
        };
        *rcond = 0.0f64;
        if Anorm == 0.0f64 {
            return GSL_SUCCESS as libc::c_int;
        }
        params.LDLT = LDLT;
        params.perm = p;
        status = gsl_linalg_invnorm1(
            N,
            Some(
                cholesky_LDLT_Ainv
                    as unsafe extern "C" fn(
                        CBLAS_TRANSPOSE_t,
                        *mut gsl_vector,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            &mut params as *mut pcholesky_params as *mut libc::c_void,
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
unsafe extern "C" fn cholesky_LDLT_norm1(
    mut LDLT: *const gsl_matrix,
    mut p: *const gsl_permutation,
    mut work: *mut gsl_vector,
) -> libc::c_double {
    let N: size_t = (*LDLT).size1;
    let D: gsl_vector_const_view = gsl_matrix_const_diagonal(LDLT);
    let mut diagA: gsl_vector_view = gsl_vector_subvector(work, N, N);
    let mut max: libc::c_double = 0.0f64;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < N {
        let mut Ajj: libc::c_double = 0.;
        Ajj = gsl_vector_get(&D.vector, j);
        i = 0 as libc::c_int as size_t;
        while i < j {
            let mut Di: libc::c_double = gsl_vector_get(&D.vector, i);
            let mut Lji: libc::c_double = gsl_matrix_get(LDLT, j, i);
            Ajj += Di * Lji * Lji;
            i = i.wrapping_add(1);
            i;
        }
        gsl_vector_set(&mut diagA.vector, j, Ajj);
        j = j.wrapping_add(1);
        j;
    }
    gsl_permute_vector_inverse(p, &mut diagA.vector);
    j = 0 as libc::c_int as size_t;
    while j < N {
        let mut sum: libc::c_double = 0.0f64;
        let mut Ajj_0: libc::c_double = gsl_vector_get(&mut diagA.vector, j);
        i = 0 as libc::c_int as size_t;
        while i < j {
            let mut wi: *mut libc::c_double = gsl_vector_ptr(work, i);
            let mut Aij: libc::c_double = gsl_matrix_get(LDLT, i, j);
            let mut absAij: libc::c_double = fabs(Aij);
            sum += absAij;
            *wi += absAij;
            i = i.wrapping_add(1);
            i;
        }
        gsl_vector_set(work, j, sum + fabs(Ajj_0));
        j = j.wrapping_add(1);
        j;
    }
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut wi_0: libc::c_double = gsl_vector_get(work, i);
        max = if max > wi_0 { max } else { wi_0 };
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
unsafe extern "C" fn cholesky_LDLT_Ainv(
    mut TransA: CBLAS_TRANSPOSE_t,
    mut x: *mut gsl_vector,
    mut params: *mut libc::c_void,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut par: *mut pcholesky_params = params as *mut pcholesky_params;
    status = gsl_linalg_pcholesky_svx((*par).LDLT, (*par).perm, x);
    return status;
}
