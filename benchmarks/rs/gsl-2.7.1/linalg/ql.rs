use ::libc;
extern "C" {
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
    fn gsl_matrix_submatrix(
        m: *mut gsl_matrix,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_view;
    fn gsl_matrix_subcolumn(
        m: *mut gsl_matrix,
        j: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
    fn gsl_matrix_const_submatrix(
        m: *const gsl_matrix,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_const_view;
    fn gsl_matrix_const_subcolumn(
        m: *const gsl_matrix,
        j: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_const_view;
    fn gsl_matrix_set_zero(m: *mut gsl_matrix);
    fn gsl_matrix_set_identity(m: *mut gsl_matrix);
    fn gsl_matrix_memcpy(dest: *mut gsl_matrix, src: *const gsl_matrix) -> libc::c_int;
    fn gsl_matrix_tricpy(
        Uplo: CBLAS_UPLO_t,
        Diag: CBLAS_DIAG_t,
        dest: *mut gsl_matrix,
        src: *const gsl_matrix,
    ) -> libc::c_int;
    fn gsl_linalg_householder_transform2(
        alpha: *mut libc::c_double,
        v: *mut gsl_vector,
    ) -> libc::c_double;
    fn gsl_linalg_householder_left(
        tau: libc::c_double,
        v: *const gsl_vector,
        A: *mut gsl_matrix,
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
pub type CBLAS_UPLO = libc::c_uint;
pub const CblasLower: CBLAS_UPLO = 122;
pub const CblasUpper: CBLAS_UPLO = 121;
pub type CBLAS_DIAG = libc::c_uint;
pub const CblasUnit: CBLAS_DIAG = 132;
pub const CblasNonUnit: CBLAS_DIAG = 131;
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
pub struct _gsl_matrix_const_view {
    pub matrix: gsl_matrix,
}
pub type gsl_matrix_const_view = _gsl_matrix_const_view;
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
unsafe extern "C" fn gsl_matrix_ptr(
    mut m: *mut gsl_matrix,
    i: size_t,
    j: size_t,
) -> *mut libc::c_double {
    return ((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
pub unsafe extern "C" fn gsl_linalg_QL_decomp(
    mut A: *mut gsl_matrix,
    mut tau: *mut gsl_vector,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if (*tau).size != N {
        gsl_error(
            b"size of tau must be N\0" as *const u8 as *const libc::c_char,
            b"ql.c\0" as *const u8 as *const libc::c_char,
            74 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let K: size_t = if M < N { M } else { N };
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < K {
            let mut c: gsl_vector_view = gsl_matrix_subcolumn(
                A,
                N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                0 as libc::c_int as size_t,
                M.wrapping_sub(i),
            );
            let mut alpha: *mut libc::c_double = gsl_matrix_ptr(
                A,
                M.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            let mut tau_j: libc::c_double = gsl_linalg_householder_transform2(
                alpha,
                &mut c.vector,
            );
            if i.wrapping_add(1 as libc::c_int as libc::c_ulong) < N {
                let mut work: gsl_vector_view = gsl_vector_subvector(
                    tau,
                    0 as libc::c_int as size_t,
                    N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                let mut m: gsl_matrix_view = gsl_matrix_submatrix(
                    A,
                    0 as libc::c_int as size_t,
                    0 as libc::c_int as size_t,
                    M.wrapping_sub(i),
                    N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                let mut tmp: libc::c_double = *alpha;
                *alpha = 1.0f64;
                gsl_linalg_householder_left(
                    tau_j,
                    &mut c.vector,
                    &mut m.matrix,
                    &mut work.vector,
                );
                *alpha = tmp;
            }
            gsl_vector_set(
                tau,
                N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                tau_j,
            );
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_QL_unpack(
    mut QL: *const gsl_matrix,
    mut tau: *const gsl_vector,
    mut Q: *mut gsl_matrix,
    mut L: *mut gsl_matrix,
) -> libc::c_int {
    let M: size_t = (*QL).size1;
    let N: size_t = (*QL).size2;
    if (*Q).size1 != M || (*Q).size2 != M {
        gsl_error(
            b"Q matrix must be M x M\0" as *const u8 as *const libc::c_char,
            b"ql.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*L).size1 != M || (*L).size2 != N {
        gsl_error(
            b"L matrix must be M x N\0" as *const u8 as *const libc::c_char,
            b"ql.c\0" as *const u8 as *const libc::c_char,
            122 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*tau).size != N {
        gsl_error(
            b"size of tau must be N\0" as *const u8 as *const libc::c_char,
            b"ql.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let K: size_t = if M < N { M } else { N };
        let mut i: size_t = 0;
        gsl_matrix_set_identity(Q);
        i = 0 as libc::c_int as size_t;
        while i < K {
            let h: gsl_vector_const_view = gsl_matrix_const_subcolumn(
                QL,
                N.wrapping_sub(K).wrapping_add(i),
                0 as libc::c_int as size_t,
                M
                    .wrapping_sub(K)
                    .wrapping_add(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            let mut m: gsl_matrix_view = gsl_matrix_submatrix(
                Q,
                0 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                M
                    .wrapping_sub(K)
                    .wrapping_add(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                M
                    .wrapping_sub(K)
                    .wrapping_add(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            let mut work: gsl_vector_view = gsl_matrix_subcolumn(
                L,
                0 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                M
                    .wrapping_sub(K)
                    .wrapping_add(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            let mut ti: libc::c_double = gsl_vector_get(
                tau,
                N.wrapping_sub(K).wrapping_add(i),
            );
            let mut ptr: *mut libc::c_double = gsl_matrix_ptr(
                QL as *mut gsl_matrix,
                M.wrapping_sub(K).wrapping_add(i),
                N.wrapping_sub(K).wrapping_add(i),
            );
            let mut tmp: libc::c_double = *ptr;
            *ptr = 1.0f64;
            gsl_linalg_householder_left(ti, &h.vector, &mut m.matrix, &mut work.vector);
            *ptr = tmp;
            i = i.wrapping_add(1);
            i;
        }
        gsl_matrix_set_zero(L);
        if M >= N {
            let src: gsl_matrix_const_view = gsl_matrix_const_submatrix(
                QL,
                M.wrapping_sub(N),
                0 as libc::c_int as size_t,
                N,
                N,
            );
            let mut dest: gsl_matrix_view = gsl_matrix_submatrix(
                L,
                M.wrapping_sub(N),
                0 as libc::c_int as size_t,
                N,
                N,
            );
            gsl_matrix_tricpy(CblasLower, CblasNonUnit, &mut dest.matrix, &src.matrix);
        } else {
            let src1: gsl_matrix_const_view = gsl_matrix_const_submatrix(
                QL,
                0 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                M,
                N.wrapping_sub(M),
            );
            let mut dest1: gsl_matrix_view = gsl_matrix_submatrix(
                L,
                0 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                M,
                N.wrapping_sub(M),
            );
            let src2: gsl_matrix_const_view = gsl_matrix_const_submatrix(
                QL,
                0 as libc::c_int as size_t,
                N.wrapping_sub(M),
                M,
                M,
            );
            let mut dest2: gsl_matrix_view = gsl_matrix_submatrix(
                L,
                0 as libc::c_int as size_t,
                N.wrapping_sub(M),
                M,
                M,
            );
            gsl_matrix_memcpy(&mut dest1.matrix, &src1.matrix);
            gsl_matrix_tricpy(CblasLower, CblasNonUnit, &mut dest2.matrix, &src2.matrix);
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
