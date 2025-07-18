use ::libc;
extern "C" {
    fn gsl_complex_conjugate(z: gsl_complex) -> gsl_complex;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_complex_subvector(
        base: *mut gsl_vector_complex,
        i: size_t,
        n: size_t,
    ) -> _gsl_vector_complex_view;
    fn gsl_vector_complex_const_subvector(
        base: *const gsl_vector_complex,
        i: size_t,
        n: size_t,
    ) -> _gsl_vector_complex_const_view;
    fn gsl_vector_complex_set_zero(v: *mut gsl_vector_complex);
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
    fn gsl_matrix_complex_subcolumn(
        m: *mut gsl_matrix_complex,
        j: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_complex_view;
    fn gsl_matrix_complex_const_submatrix(
        m: *const gsl_matrix_complex,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_complex_const_view;
    fn gsl_matrix_complex_const_column(
        m: *const gsl_matrix_complex,
        j: size_t,
    ) -> _gsl_vector_complex_const_view;
    fn gsl_matrix_complex_const_subcolumn(
        m: *const gsl_matrix_complex,
        j: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_complex_const_view;
    fn gsl_matrix_complex_set_identity(m: *mut gsl_matrix_complex);
    fn gsl_blas_ztrsv(
        Uplo: CBLAS_UPLO_t,
        TransA: CBLAS_TRANSPOSE_t,
        Diag: CBLAS_DIAG_t,
        A: *const gsl_matrix_complex,
        X: *mut gsl_vector_complex,
    ) -> libc::c_int;
    fn gsl_linalg_complex_householder_transform(
        v: *mut gsl_vector_complex,
    ) -> gsl_complex;
    fn gsl_linalg_complex_householder_hv(
        tau: gsl_complex,
        v: *const gsl_vector_complex,
        w: *mut gsl_vector_complex,
    ) -> libc::c_int;
    fn gsl_linalg_complex_householder_left(
        tau: gsl_complex,
        v: *const gsl_vector_complex,
        A: *mut gsl_matrix_complex,
        work: *mut gsl_vector_complex,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_complex {
    pub dat: [libc::c_double; 2],
}
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
pub struct _gsl_matrix_complex_const_view {
    pub matrix: gsl_matrix_complex,
}
pub type gsl_matrix_complex_const_view = _gsl_matrix_complex_const_view;
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
#[inline]
unsafe extern "C" fn gsl_vector_complex_get(
    mut v: *const gsl_vector_complex,
    i: size_t,
) -> gsl_complex {
    return *(&mut *((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*v).stride)
                as isize,
        ) as *mut libc::c_double as *mut gsl_complex);
}
#[inline]
unsafe extern "C" fn gsl_vector_complex_set(
    mut v: *mut gsl_vector_complex,
    i: size_t,
    mut z: gsl_complex,
) {
    *(&mut *((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*v).stride)
                as isize,
        ) as *mut libc::c_double as *mut gsl_complex) = z;
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
pub unsafe extern "C" fn gsl_linalg_complex_QR_decomp(
    mut A: *mut gsl_matrix_complex,
    mut tau: *mut gsl_vector_complex,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if (*tau).size != N {
        gsl_error(
            b"size of tau must be N\0" as *const u8 as *const libc::c_char,
            b"qrc.c\0" as *const u8 as *const libc::c_char,
            68 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < (if M < N { M } else { N }) {
            let mut c: gsl_vector_complex_view = gsl_matrix_complex_subcolumn(
                A,
                i,
                i,
                M.wrapping_sub(i),
            );
            let mut tau_i: gsl_complex = gsl_linalg_complex_householder_transform(
                &mut c.vector,
            );
            gsl_vector_complex_set(tau, i, tau_i);
            if i.wrapping_add(1 as libc::c_int as libc::c_ulong) < N {
                let mut m: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
                    A,
                    i,
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    M.wrapping_sub(i),
                    N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                let mut tau_i_conj: gsl_complex = gsl_complex_conjugate(tau_i);
                let mut work: gsl_vector_complex_view = gsl_vector_complex_subvector(
                    tau,
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                gsl_linalg_complex_householder_left(
                    tau_i_conj,
                    &mut c.vector,
                    &mut m.matrix,
                    &mut work.vector,
                );
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_complex_QR_solve(
    mut QR: *const gsl_matrix_complex,
    mut tau: *const gsl_vector_complex,
    mut b: *const gsl_vector_complex,
    mut x: *mut gsl_vector_complex,
) -> libc::c_int {
    if (*QR).size1 != (*QR).size2 {
        gsl_error(
            b"QR matrix must be square\0" as *const u8 as *const libc::c_char,
            b"qrc.c\0" as *const u8 as *const libc::c_char,
            113 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*QR).size1 != (*b).size {
        gsl_error(
            b"matrix size must match b size\0" as *const u8 as *const libc::c_char,
            b"qrc.c\0" as *const u8 as *const libc::c_char,
            117 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*QR).size2 != (*x).size {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8
                as *const libc::c_char,
            b"qrc.c\0" as *const u8 as *const libc::c_char,
            121 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        gsl_vector_complex_memcpy(x, b);
        gsl_linalg_complex_QR_svx(QR, tau, x);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_complex_QR_svx(
    mut QR: *const gsl_matrix_complex,
    mut tau: *const gsl_vector_complex,
    mut x: *mut gsl_vector_complex,
) -> libc::c_int {
    if (*QR).size1 != (*QR).size2 {
        gsl_error(
            b"QR matrix must be square\0" as *const u8 as *const libc::c_char,
            b"qrc.c\0" as *const u8 as *const libc::c_char,
            148 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*QR).size1 != (*x).size {
        gsl_error(
            b"matrix size must match x/rhs size\0" as *const u8 as *const libc::c_char,
            b"qrc.c\0" as *const u8 as *const libc::c_char,
            152 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        gsl_linalg_complex_QR_QHvec(QR, tau, x);
        gsl_blas_ztrsv(CblasUpper, CblasNoTrans, CblasNonUnit, QR, x);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_complex_QR_lssolve(
    mut QR: *const gsl_matrix_complex,
    mut tau: *const gsl_vector_complex,
    mut b: *const gsl_vector_complex,
    mut x: *mut gsl_vector_complex,
    mut residual: *mut gsl_vector_complex,
) -> libc::c_int {
    let M: size_t = (*QR).size1;
    let N: size_t = (*QR).size2;
    if M < N {
        gsl_error(
            b"QR matrix must have M>=N\0" as *const u8 as *const libc::c_char,
            b"qrc.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if M != (*b).size {
        gsl_error(
            b"matrix size must match b size\0" as *const u8 as *const libc::c_char,
            b"qrc.c\0" as *const u8 as *const libc::c_char,
            188 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if N != (*x).size {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8
                as *const libc::c_char,
            b"qrc.c\0" as *const u8 as *const libc::c_char,
            192 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if M != (*residual).size {
        gsl_error(
            b"matrix size must match residual size\0" as *const u8
                as *const libc::c_char,
            b"qrc.c\0" as *const u8 as *const libc::c_char,
            196 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let R: gsl_matrix_complex_const_view = gsl_matrix_complex_const_submatrix(
            QR,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N,
            N,
        );
        let mut c: gsl_vector_complex_view = gsl_vector_complex_subvector(
            residual,
            0 as libc::c_int as size_t,
            N,
        );
        gsl_vector_complex_memcpy(residual, b);
        gsl_linalg_complex_QR_QHvec(QR, tau, residual);
        gsl_vector_complex_memcpy(x, &mut c.vector);
        gsl_blas_ztrsv(CblasUpper, CblasNoTrans, CblasNonUnit, &R.matrix, x);
        gsl_vector_complex_set_zero(&mut c.vector);
        gsl_linalg_complex_QR_Qvec(QR, tau, residual);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_complex_QR_QHvec(
    mut QR: *const gsl_matrix_complex,
    mut tau: *const gsl_vector_complex,
    mut v: *mut gsl_vector_complex,
) -> libc::c_int {
    let M: size_t = (*QR).size1;
    let N: size_t = (*QR).size2;
    if (*tau).size != N {
        gsl_error(
            b"size of tau must be N\0" as *const u8 as *const libc::c_char,
            b"qrc.c\0" as *const u8 as *const libc::c_char,
            230 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*v).size != M {
        gsl_error(
            b"vector size must be M\0" as *const u8 as *const libc::c_char,
            b"qrc.c\0" as *const u8 as *const libc::c_char,
            234 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < (if M < N { M } else { N }) {
            let h: gsl_vector_complex_const_view = gsl_matrix_complex_const_subcolumn(
                QR,
                i,
                i,
                M.wrapping_sub(i),
            );
            let mut w: gsl_vector_complex_view = gsl_vector_complex_subvector(
                v,
                i,
                M.wrapping_sub(i),
            );
            let mut ti: gsl_complex = gsl_vector_complex_get(tau, i);
            let mut ti_conj: gsl_complex = gsl_complex_conjugate(ti);
            gsl_linalg_complex_householder_hv(ti_conj, &h.vector, &mut w.vector);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_complex_QR_Qvec(
    mut QR: *const gsl_matrix_complex,
    mut tau: *const gsl_vector_complex,
    mut v: *mut gsl_vector_complex,
) -> libc::c_int {
    let M: size_t = (*QR).size1;
    let N: size_t = (*QR).size2;
    if (*tau).size != (if M < N { M } else { N }) {
        gsl_error(
            b"size of tau must be MIN(M,N)\0" as *const u8 as *const libc::c_char,
            b"qrc.c\0" as *const u8 as *const libc::c_char,
            264 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*v).size != M {
        gsl_error(
            b"vector size must be M\0" as *const u8 as *const libc::c_char,
            b"qrc.c\0" as *const u8 as *const libc::c_char,
            268 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = if M < N { M } else { N };
        loop {
            let fresh0 = i;
            i = i.wrapping_sub(1);
            if !(fresh0 > 0 as libc::c_int as libc::c_ulong) {
                break;
            }
            let c: gsl_vector_complex_const_view = gsl_matrix_complex_const_column(
                QR,
                i,
            );
            let h: gsl_vector_complex_const_view = gsl_vector_complex_const_subvector(
                &c.vector,
                i,
                M.wrapping_sub(i),
            );
            let mut w: gsl_vector_complex_view = gsl_vector_complex_subvector(
                v,
                i,
                M.wrapping_sub(i),
            );
            let mut ti: gsl_complex = gsl_vector_complex_get(tau, i);
            gsl_linalg_complex_householder_hv(ti, &h.vector, &mut w.vector);
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_complex_QR_unpack(
    mut QR: *const gsl_matrix_complex,
    mut tau: *const gsl_vector_complex,
    mut Q: *mut gsl_matrix_complex,
    mut R: *mut gsl_matrix_complex,
) -> libc::c_int {
    let M: size_t = (*QR).size1;
    let N: size_t = (*QR).size2;
    if (*Q).size1 != M || (*Q).size2 != M {
        gsl_error(
            b"Q matrix must be M x M\0" as *const u8 as *const libc::c_char,
            b"qrc.c\0" as *const u8 as *const libc::c_char,
            300 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*R).size1 != M || (*R).size2 != N {
        gsl_error(
            b"R matrix must be M x N\0" as *const u8 as *const libc::c_char,
            b"qrc.c\0" as *const u8 as *const libc::c_char,
            304 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*tau).size != N {
        gsl_error(
            b"size of tau must be N\0" as *const u8 as *const libc::c_char,
            b"qrc.c\0" as *const u8 as *const libc::c_char,
            308 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        gsl_matrix_complex_set_identity(Q);
        i = if M < N { M } else { N };
        loop {
            let fresh1 = i;
            i = i.wrapping_sub(1);
            if !(fresh1 > 0 as libc::c_int as libc::c_ulong) {
                break;
            }
            let c: gsl_vector_complex_const_view = gsl_matrix_complex_const_column(
                QR,
                i,
            );
            let h: gsl_vector_complex_const_view = gsl_vector_complex_const_subvector(
                &c.vector,
                i,
                M.wrapping_sub(i),
            );
            let mut m: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
                Q,
                i,
                i,
                M.wrapping_sub(i),
                M.wrapping_sub(i),
            );
            let mut ti: gsl_complex = gsl_vector_complex_get(tau, i);
            let mut work: gsl_vector_complex_view = gsl_matrix_complex_subcolumn(
                R,
                0 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                M.wrapping_sub(i),
            );
            gsl_linalg_complex_householder_left(
                ti,
                &h.vector,
                &mut m.matrix,
                &mut work.vector,
            );
        }
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < i && j < N {
                gsl_matrix_complex_set(R, i, j, gsl_complex_rect(0.0f64, 0.0f64));
                j = j.wrapping_add(1);
                j;
            }
            j = i;
            while j < N {
                gsl_matrix_complex_set(R, i, j, gsl_matrix_complex_get(QR, i, j));
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
