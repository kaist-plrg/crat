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
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
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
    fn gsl_matrix_const_diagonal(m: *const gsl_matrix) -> _gsl_vector_const_view;
    fn gsl_matrix_const_subdiagonal(
        m: *const gsl_matrix,
        k: size_t,
    ) -> _gsl_vector_const_view;
    fn gsl_matrix_const_subcolumn(
        m: *const gsl_matrix,
        j: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_const_view;
    fn gsl_matrix_set_identity(m: *mut gsl_matrix);
    fn gsl_blas_ddot(
        X: *const gsl_vector,
        Y: *const gsl_vector,
        result: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_blas_daxpy(
        alpha: libc::c_double,
        X: *const gsl_vector,
        Y: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_blas_dsymv(
        Uplo: CBLAS_UPLO_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        X: *const gsl_vector,
        beta: libc::c_double,
        Y: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_blas_dsyr2(
        Uplo: CBLAS_UPLO_t,
        alpha: libc::c_double,
        X: *const gsl_vector,
        Y: *const gsl_vector,
        A: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_linalg_householder_transform(v: *mut gsl_vector) -> libc::c_double;
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
pub type CBLAS_UPLO_t = CBLAS_UPLO;
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
pub unsafe extern "C" fn gsl_linalg_symmtd_decomp(
    mut A: *mut gsl_matrix,
    mut tau: *mut gsl_vector,
) -> libc::c_int {
    if (*A).size1 != (*A).size2 {
        gsl_error(
            b"symmetric tridiagonal decomposition requires square matrix\0" as *const u8
                as *const libc::c_char,
            b"symmtd.c\0" as *const u8 as *const libc::c_char,
            70 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if ((*tau).size).wrapping_add(1 as libc::c_int as libc::c_ulong) != (*A).size1
    {
        gsl_error(
            b"size of tau must be N-1\0" as *const u8 as *const libc::c_char,
            b"symmtd.c\0" as *const u8 as *const libc::c_char,
            74 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let N: size_t = (*A).size1;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N.wrapping_sub(2 as libc::c_int as libc::c_ulong) {
            let mut v: gsl_vector_view = gsl_matrix_subcolumn(
                A,
                i,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            let mut tau_i: libc::c_double = gsl_linalg_householder_transform(
                &mut v.vector,
            );
            if tau_i != 0.0f64 {
                let mut m: gsl_matrix_view = gsl_matrix_submatrix(
                    A,
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                let mut ei: libc::c_double = gsl_vector_get(
                    &mut v.vector,
                    0 as libc::c_int as size_t,
                );
                let mut x: gsl_vector_view = gsl_vector_subvector(
                    tau,
                    i,
                    N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                gsl_vector_set(&mut v.vector, 0 as libc::c_int as size_t, 1.0f64);
                gsl_blas_dsymv(
                    CblasLower,
                    tau_i,
                    &mut m.matrix,
                    &mut v.vector,
                    0.0f64,
                    &mut x.vector,
                );
                let mut xv: libc::c_double = 0.;
                let mut alpha: libc::c_double = 0.;
                gsl_blas_ddot(&mut x.vector, &mut v.vector, &mut xv);
                alpha = -0.5f64 * tau_i * xv;
                gsl_blas_daxpy(alpha, &mut v.vector, &mut x.vector);
                gsl_blas_dsyr2(
                    CblasLower,
                    -1.0f64,
                    &mut v.vector,
                    &mut x.vector,
                    &mut m.matrix,
                );
                gsl_vector_set(&mut v.vector, 0 as libc::c_int as size_t, ei);
            }
            gsl_vector_set(tau, i, tau_i);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_symmtd_unpack(
    mut A: *const gsl_matrix,
    mut tau: *const gsl_vector,
    mut Q: *mut gsl_matrix,
    mut diag: *mut gsl_vector,
    mut sdiag: *mut gsl_vector,
) -> libc::c_int {
    if (*A).size1 != (*A).size2 {
        gsl_error(
            b"matrix A must be square\0" as *const u8 as *const libc::c_char,
            b"symmtd.c\0" as *const u8 as *const libc::c_char,
            132 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if ((*tau).size).wrapping_add(1 as libc::c_int as libc::c_ulong) != (*A).size1
    {
        gsl_error(
            b"size of tau must be (matrix size - 1)\0" as *const u8
                as *const libc::c_char,
            b"symmtd.c\0" as *const u8 as *const libc::c_char,
            136 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*Q).size1 != (*A).size1 || (*Q).size2 != (*A).size1 {
        gsl_error(
            b"size of Q must match size of A\0" as *const u8 as *const libc::c_char,
            b"symmtd.c\0" as *const u8 as *const libc::c_char,
            140 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*diag).size != (*A).size1 {
        gsl_error(
            b"size of diagonal must match size of A\0" as *const u8
                as *const libc::c_char,
            b"symmtd.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if ((*sdiag).size).wrapping_add(1 as libc::c_int as libc::c_ulong)
        != (*A).size1
    {
        gsl_error(
            b"size of subdiagonal must be (matrix size - 1)\0" as *const u8
                as *const libc::c_char,
            b"symmtd.c\0" as *const u8 as *const libc::c_char,
            148 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let N: size_t = (*A).size1;
        let d: gsl_vector_const_view = gsl_matrix_const_diagonal(A);
        let sd: gsl_vector_const_view = gsl_matrix_const_subdiagonal(
            A,
            1 as libc::c_int as size_t,
        );
        let mut i: size_t = 0;
        gsl_matrix_set_identity(Q);
        i = N.wrapping_sub(2 as libc::c_int as libc::c_ulong);
        loop {
            let fresh0 = i;
            i = i.wrapping_sub(1);
            if !(fresh0 > 0 as libc::c_int as libc::c_ulong) {
                break;
            }
            let h: gsl_vector_const_view = gsl_matrix_const_subcolumn(
                A,
                i,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            let mut ti: libc::c_double = gsl_vector_get(tau, i);
            let mut m: gsl_matrix_view = gsl_matrix_submatrix(
                Q,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            let mut work: gsl_vector_view = gsl_vector_subvector(
                diag,
                0 as libc::c_int as size_t,
                N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            let mut ptr: *mut libc::c_double = gsl_vector_ptr(
                &h.vector as *const gsl_vector as *mut gsl_vector,
                0 as libc::c_int as size_t,
            );
            let mut tmp: libc::c_double = *ptr;
            *ptr = 1.0f64;
            gsl_linalg_householder_left(ti, &h.vector, &mut m.matrix, &mut work.vector);
            *ptr = tmp;
        }
        gsl_vector_memcpy(diag, &d.vector);
        gsl_vector_memcpy(sdiag, &sd.vector);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_symmtd_unpack_T(
    mut A: *const gsl_matrix,
    mut diag: *mut gsl_vector,
    mut sdiag: *mut gsl_vector,
) -> libc::c_int {
    if (*A).size1 != (*A).size2 {
        gsl_error(
            b"matrix A must be square\0" as *const u8 as *const libc::c_char,
            b"symmtd.c\0" as *const u8 as *const libc::c_char,
            192 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*diag).size != (*A).size1 {
        gsl_error(
            b"size of diagonal must match size of A\0" as *const u8
                as *const libc::c_char,
            b"symmtd.c\0" as *const u8 as *const libc::c_char,
            196 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if ((*sdiag).size).wrapping_add(1 as libc::c_int as libc::c_ulong)
        != (*A).size1
    {
        gsl_error(
            b"size of subdiagonal must be (matrix size - 1)\0" as *const u8
                as *const libc::c_char,
            b"symmtd.c\0" as *const u8 as *const libc::c_char,
            200 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let d: gsl_vector_const_view = gsl_matrix_const_diagonal(A);
        let sd: gsl_vector_const_view = gsl_matrix_const_subdiagonal(
            A,
            1 as libc::c_int as size_t,
        );
        gsl_vector_memcpy(diag, &d.vector);
        gsl_vector_memcpy(sdiag, &sd.vector);
        return GSL_SUCCESS as libc::c_int;
    };
}
