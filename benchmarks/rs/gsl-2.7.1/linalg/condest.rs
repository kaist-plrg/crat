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
    fn gsl_vector_set_zero(v: *mut gsl_vector);
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
    fn gsl_blas_dasum(X: *const gsl_vector) -> libc::c_double;
    fn gsl_blas_idamax(X: *const gsl_vector) -> CBLAS_INDEX_t;
    fn gsl_blas_dtrsv(
        Uplo: CBLAS_UPLO_t,
        TransA: CBLAS_TRANSPOSE_t,
        Diag: CBLAS_DIAG_t,
        A: *const gsl_matrix,
        X: *mut gsl_vector,
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
pub type CBLAS_INDEX_t = size_t;
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
pub unsafe extern "C" fn gsl_linalg_tri_rcond(
    mut Uplo: CBLAS_UPLO_t,
    mut A: *const gsl_matrix,
    mut rcond: *mut libc::c_double,
    mut work: *mut gsl_vector,
) -> libc::c_int {
    return condest_tri_rcond(Uplo, A, rcond, work);
}
pub unsafe extern "C" fn gsl_linalg_invnorm1(
    N: size_t,
    mut Ainvx: Option::<
        unsafe extern "C" fn(
            CBLAS_TRANSPOSE_t,
            *mut gsl_vector,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    mut params: *mut libc::c_void,
    mut Ainvnorm: *mut libc::c_double,
    mut work: *mut gsl_vector,
) -> libc::c_int {
    if (*work).size != (3 as libc::c_int as libc::c_ulong).wrapping_mul(N) {
        gsl_error(
            b"work vector must have length 3*N\0" as *const u8 as *const libc::c_char,
            b"condest.c\0" as *const u8 as *const libc::c_char,
            71 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let maxit: size_t = 5 as libc::c_int as size_t;
        let mut x: gsl_vector_view = gsl_vector_subvector(
            work,
            0 as libc::c_int as size_t,
            N,
        );
        let mut v: gsl_vector_view = gsl_vector_subvector(work, N, N);
        let mut xi: gsl_vector_view = gsl_vector_subvector(
            work,
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(N),
            N,
        );
        let mut gamma: libc::c_double = 0.;
        let mut gamma_old: libc::c_double = 0.;
        let mut temp: libc::c_double = 0.;
        let mut i: size_t = 0;
        let mut k: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            gsl_vector_set(&mut x.vector, i, 1.0f64 / N as libc::c_double);
            i = i.wrapping_add(1);
            i;
        }
        gsl_vector_memcpy(&mut v.vector, &mut x.vector);
        (Some(Ainvx.unwrap())).unwrap()(CblasNoTrans, &mut v.vector, params);
        gamma = gsl_blas_dasum(&mut v.vector);
        i = 0 as libc::c_int as size_t;
        while i < N {
            let mut vi: libc::c_double = gsl_vector_get(&mut v.vector, i);
            gsl_vector_set(
                &mut xi.vector,
                i,
                (if vi >= 0.0f64 { 1 as libc::c_int } else { -(1 as libc::c_int) })
                    as libc::c_double,
            );
            i = i.wrapping_add(1);
            i;
        }
        gsl_vector_memcpy(&mut x.vector, &mut xi.vector);
        (Some(Ainvx.unwrap())).unwrap()(CblasTrans, &mut x.vector, params);
        k = 0 as libc::c_int as size_t;
        while k < maxit {
            let mut j: size_t = gsl_blas_idamax(&mut x.vector);
            gsl_vector_set_zero(&mut v.vector);
            gsl_vector_set(&mut v.vector, j, 1.0f64);
            (Some(Ainvx.unwrap())).unwrap()(CblasNoTrans, &mut v.vector, params);
            gamma_old = gamma;
            gamma = gsl_blas_dasum(&mut v.vector);
            if condest_same_sign(&mut v.vector, &mut xi.vector) != 0 || gamma < gamma_old
            {
                break;
            }
            i = 0 as libc::c_int as size_t;
            while i < N {
                let mut vi_0: libc::c_double = gsl_vector_get(&mut v.vector, i);
                gsl_vector_set(
                    &mut xi.vector,
                    i,
                    (if vi_0 >= 0.0f64 { 1 as libc::c_int } else { -(1 as libc::c_int) })
                        as libc::c_double,
                );
                i = i.wrapping_add(1);
                i;
            }
            gsl_vector_memcpy(&mut x.vector, &mut xi.vector);
            (Some(Ainvx.unwrap())).unwrap()(CblasTrans, &mut x.vector, params);
            k = k.wrapping_add(1);
            k;
        }
        temp = 1.0f64;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let mut term: libc::c_double = 1.0f64
                + i as libc::c_double / (N as libc::c_double - 1.0f64);
            gsl_vector_set(&mut x.vector, i, temp * term);
            temp = -temp;
            i = i.wrapping_add(1);
            i;
        }
        (Some(Ainvx.unwrap())).unwrap()(CblasNoTrans, &mut x.vector, params);
        temp = 2.0f64 * gsl_blas_dasum(&mut x.vector) / (3.0f64 * N as libc::c_double);
        if temp > gamma {
            gsl_vector_memcpy(&mut v.vector, &mut x.vector);
            gamma = temp;
        }
        *Ainvnorm = gamma;
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn condest_tri_rcond(
    mut Uplo: CBLAS_UPLO_t,
    mut A: *const gsl_matrix,
    mut rcond: *mut libc::c_double,
    mut work: *mut gsl_vector,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if M != N {
        gsl_error(
            b"matrix must be square\0" as *const u8 as *const libc::c_char,
            b"condest.c\0" as *const u8 as *const libc::c_char,
            163 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*work).size != (3 as libc::c_int as libc::c_ulong).wrapping_mul(N) {
        gsl_error(
            b"work vector must have length 3*N\0" as *const u8 as *const libc::c_char,
            b"condest.c\0" as *const u8 as *const libc::c_char,
            167 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut Anorm: libc::c_double = condest_tri_norm1(Uplo, A);
        let mut Ainvnorm: libc::c_double = 0.;
        *rcond = 0.0f64;
        if Anorm == 0.0f64 {
            return GSL_SUCCESS as libc::c_int;
        }
        if Uplo as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint {
            status = gsl_linalg_invnorm1(
                N,
                Some(
                    condest_invtriu
                        as unsafe extern "C" fn(
                            CBLAS_TRANSPOSE_t,
                            *mut gsl_vector,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
                A as *mut libc::c_void,
                &mut Ainvnorm,
                work,
            );
        } else {
            status = gsl_linalg_invnorm1(
                N,
                Some(
                    condest_invtril
                        as unsafe extern "C" fn(
                            CBLAS_TRANSPOSE_t,
                            *mut gsl_vector,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
                A as *mut libc::c_void,
                &mut Ainvnorm,
                work,
            );
        }
        if status != 0 {
            return status;
        }
        if Ainvnorm != 0.0f64 {
            *rcond = 1.0f64 / Anorm / Ainvnorm;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn condest_tri_norm1(
    mut Uplo: CBLAS_UPLO_t,
    mut A: *const gsl_matrix,
) -> libc::c_double {
    let N: size_t = (*A).size2;
    let mut max: libc::c_double = 0.0f64;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    if Uplo as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint {
        j = 0 as libc::c_int as size_t;
        while j < N {
            let mut sum: libc::c_double = 0.0f64;
            i = 0 as libc::c_int as size_t;
            while i <= j {
                let mut Aij: libc::c_double = gsl_matrix_get(A, i, j);
                sum += fabs(Aij);
                i = i.wrapping_add(1);
                i;
            }
            max = if max > sum { max } else { sum };
            j = j.wrapping_add(1);
            j;
        }
    } else {
        j = 0 as libc::c_int as size_t;
        while j < N {
            let mut sum_0: libc::c_double = 0.0f64;
            i = j;
            while i < N {
                let mut Aij_0: libc::c_double = gsl_matrix_get(A, i, j);
                sum_0 += fabs(Aij_0);
                i = i.wrapping_add(1);
                i;
            }
            max = if max > sum_0 { max } else { sum_0 };
            j = j.wrapping_add(1);
            j;
        }
    }
    return max;
}
unsafe extern "C" fn condest_same_sign(
    mut x: *const gsl_vector,
    mut y: *const gsl_vector,
) -> libc::c_int {
    let n: size_t = (*x).size;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_double = gsl_vector_get(x, i);
        let mut yi: libc::c_double = gsl_vector_get(y, i);
        if (if xi >= 0.0f64 { 1 as libc::c_int } else { -(1 as libc::c_int) })
            != (if yi >= 0.0f64 { 1 as libc::c_int } else { -(1 as libc::c_int) })
        {
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn condest_invtriu(
    mut TransA: CBLAS_TRANSPOSE_t,
    mut x: *mut gsl_vector,
    mut params: *mut libc::c_void,
) -> libc::c_int {
    let mut A: *mut gsl_matrix = params as *mut gsl_matrix;
    return gsl_blas_dtrsv(CblasUpper, TransA, CblasNonUnit, A, x);
}
unsafe extern "C" fn condest_invtril(
    mut TransA: CBLAS_TRANSPOSE_t,
    mut x: *mut gsl_vector,
    mut params: *mut libc::c_void,
) -> libc::c_int {
    let mut A: *mut gsl_matrix = params as *mut gsl_matrix;
    return gsl_blas_dtrsv(CblasLower, TransA, CblasNonUnit, A, x);
}
pub unsafe extern "C" fn gsl_linalg_tri_upper_rcond(
    mut A: *const gsl_matrix,
    mut rcond: *mut libc::c_double,
    mut work: *mut gsl_vector,
) -> libc::c_int {
    let mut status: libc::c_int = condest_tri_rcond(CblasUpper, A, rcond, work);
    return status;
}
pub unsafe extern "C" fn gsl_linalg_tri_lower_rcond(
    mut A: *const gsl_matrix,
    mut rcond: *mut libc::c_double,
    mut work: *mut gsl_vector,
) -> libc::c_int {
    let mut status: libc::c_int = condest_tri_rcond(CblasLower, A, rcond, work);
    return status;
}
