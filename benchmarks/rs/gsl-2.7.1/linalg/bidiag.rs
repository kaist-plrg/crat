use ::libc;
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_alloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_vector_subvector(
        v: *mut gsl_vector,
        i: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
    fn gsl_vector_const_subvector(
        v: *const gsl_vector,
        i: size_t,
        n: size_t,
    ) -> _gsl_vector_const_view;
    fn gsl_matrix_submatrix(
        m: *mut gsl_matrix,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_view;
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
    fn gsl_matrix_const_row(m: *const gsl_matrix, i: size_t) -> _gsl_vector_const_view;
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
    fn gsl_matrix_set_identity(m: *mut gsl_matrix);
    fn gsl_linalg_householder_transform(v: *mut gsl_vector) -> libc::c_double;
    fn gsl_linalg_householder_hm(
        tau: libc::c_double,
        v: *const gsl_vector,
        A: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_linalg_householder_left(
        tau: libc::c_double,
        v: *const gsl_vector,
        A: *mut gsl_matrix,
        work: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_linalg_householder_right(
        tau: libc::c_double,
        v: *const gsl_vector,
        A: *mut gsl_matrix,
        work: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_linalg_householder_hm1(
        tau: libc::c_double,
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
unsafe extern "C" fn gsl_matrix_get(
    mut m: *const gsl_matrix,
    i: size_t,
    j: size_t,
) -> libc::c_double {
    return *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
pub unsafe extern "C" fn gsl_linalg_bidiag_decomp(
    mut A: *mut gsl_matrix,
    mut tau_U: *mut gsl_vector,
    mut tau_V: *mut gsl_vector,
) -> libc::c_int {
    if (*A).size1 < (*A).size2 {
        gsl_error(
            b"bidiagonal decomposition requires M>=N\0" as *const u8
                as *const libc::c_char,
            b"bidiag.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*tau_U).size != (*A).size2 {
        gsl_error(
            b"size of tau_U must be N\0" as *const u8 as *const libc::c_char,
            b"bidiag.c\0" as *const u8 as *const libc::c_char,
            83 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if ((*tau_V).size).wrapping_add(1 as libc::c_int as libc::c_ulong)
        != (*A).size2
    {
        gsl_error(
            b"size of tau_V must be (N - 1)\0" as *const u8 as *const libc::c_char,
            b"bidiag.c\0" as *const u8 as *const libc::c_char,
            87 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let M: size_t = (*A).size1;
        let N: size_t = (*A).size2;
        let mut tmp: *mut gsl_vector = gsl_vector_alloc(M);
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < N {
            let mut v: gsl_vector_view = gsl_matrix_subcolumn(
                A,
                j,
                j,
                M.wrapping_sub(j),
            );
            let mut tau_j: libc::c_double = gsl_linalg_householder_transform(
                &mut v.vector,
            );
            if j.wrapping_add(1 as libc::c_int as libc::c_ulong) < N {
                let mut m: gsl_matrix_view = gsl_matrix_submatrix(
                    A,
                    j,
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    M.wrapping_sub(j),
                    N.wrapping_sub(j).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                let mut work: gsl_vector_view = gsl_vector_subvector(
                    tau_U,
                    j,
                    N.wrapping_sub(j).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                let mut ptr: *mut libc::c_double = gsl_vector_ptr(
                    &mut v.vector,
                    0 as libc::c_int as size_t,
                );
                let mut tmp_0: libc::c_double = *ptr;
                *ptr = 1.0f64;
                gsl_linalg_householder_left(
                    tau_j,
                    &mut v.vector,
                    &mut m.matrix,
                    &mut work.vector,
                );
                *ptr = tmp_0;
            }
            gsl_vector_set(tau_U, j, tau_j);
            if j.wrapping_add(1 as libc::c_int as libc::c_ulong) < N {
                v = gsl_matrix_subrow(
                    A,
                    j,
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    N.wrapping_sub(j).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                tau_j = gsl_linalg_householder_transform(&mut v.vector);
                if j.wrapping_add(1 as libc::c_int as libc::c_ulong) < M {
                    let mut m_0: gsl_matrix_view = gsl_matrix_submatrix(
                        A,
                        j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        M
                            .wrapping_sub(j)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        N.wrapping_sub(j).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    );
                    let mut work_0: gsl_vector_view = gsl_vector_subvector(
                        tmp,
                        0 as libc::c_int as size_t,
                        M.wrapping_sub(j).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    );
                    gsl_linalg_householder_right(
                        tau_j,
                        &mut v.vector,
                        &mut m_0.matrix,
                        &mut work_0.vector,
                    );
                }
                gsl_vector_set(tau_V, j, tau_j);
            }
            j = j.wrapping_add(1);
            j;
        }
        gsl_vector_free(tmp);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_bidiag_unpack(
    mut A: *const gsl_matrix,
    mut tau_U: *const gsl_vector,
    mut U: *mut gsl_matrix,
    mut tau_V: *const gsl_vector,
    mut V: *mut gsl_matrix,
    mut diag: *mut gsl_vector,
    mut superdiag: *mut gsl_vector,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    let K: size_t = if M < N { M } else { N };
    if M < N {
        gsl_error(
            b"matrix A must have M >= N\0" as *const u8 as *const libc::c_char,
            b"bidiag.c\0" as *const u8 as *const libc::c_char,
            160 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*tau_U).size != K {
        gsl_error(
            b"size of tau must be MIN(M,N)\0" as *const u8 as *const libc::c_char,
            b"bidiag.c\0" as *const u8 as *const libc::c_char,
            164 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if ((*tau_V).size).wrapping_add(1 as libc::c_int as libc::c_ulong) != K {
        gsl_error(
            b"size of tau must be MIN(M,N) - 1\0" as *const u8 as *const libc::c_char,
            b"bidiag.c\0" as *const u8 as *const libc::c_char,
            168 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*U).size1 != M || (*U).size2 != N {
        gsl_error(
            b"size of U must be M x N\0" as *const u8 as *const libc::c_char,
            b"bidiag.c\0" as *const u8 as *const libc::c_char,
            172 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*V).size1 != N || (*V).size2 != N {
        gsl_error(
            b"size of V must be N x N\0" as *const u8 as *const libc::c_char,
            b"bidiag.c\0" as *const u8 as *const libc::c_char,
            176 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*diag).size != K {
        gsl_error(
            b"size of diagonal must match size of A\0" as *const u8
                as *const libc::c_char,
            b"bidiag.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if ((*superdiag).size).wrapping_add(1 as libc::c_int as libc::c_ulong) != K {
        gsl_error(
            b"size of subdiagonal must be (diagonal size - 1)\0" as *const u8
                as *const libc::c_char,
            b"bidiag.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let mut Aii: libc::c_double = gsl_matrix_get(A, i, i);
            gsl_vector_set(diag, i, Aii);
            i = i.wrapping_add(1);
            i;
        }
        i = 0 as libc::c_int as size_t;
        while i < N.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            let mut Aij: libc::c_double = gsl_matrix_get(
                A,
                i,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            gsl_vector_set(superdiag, i, Aij);
            i = i.wrapping_add(1);
            i;
        }
        gsl_matrix_set_identity(V);
        i = N.wrapping_sub(1 as libc::c_int as libc::c_ulong);
        loop {
            let fresh0 = i;
            i = i.wrapping_sub(1);
            if !(fresh0 > 0 as libc::c_int as libc::c_ulong) {
                break;
            }
            let h: gsl_vector_const_view = gsl_matrix_const_subrow(
                A,
                i,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            let mut ti: libc::c_double = gsl_vector_get(tau_V, i);
            let mut m: gsl_matrix_view = gsl_matrix_submatrix(
                V,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            let mut work: gsl_vector_view = gsl_matrix_subrow(
                U,
                0 as libc::c_int as size_t,
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
        gsl_matrix_set_identity(U);
        j = N;
        loop {
            let fresh1 = j;
            j = j.wrapping_sub(1);
            if !(fresh1 > 0 as libc::c_int as libc::c_ulong) {
                break;
            }
            let h_0: gsl_vector_const_view = gsl_matrix_const_subcolumn(
                A,
                j,
                j,
                M.wrapping_sub(j),
            );
            let mut tj: libc::c_double = gsl_vector_get(tau_U, j);
            let mut m_0: gsl_matrix_view = gsl_matrix_submatrix(
                U,
                j,
                j,
                M.wrapping_sub(j),
                N.wrapping_sub(j),
            );
            gsl_linalg_householder_hm(tj, &h_0.vector, &mut m_0.matrix);
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_bidiag_unpack2(
    mut A: *mut gsl_matrix,
    mut tau_U: *mut gsl_vector,
    mut tau_V: *mut gsl_vector,
    mut V: *mut gsl_matrix,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    let K: size_t = if M < N { M } else { N };
    if M < N {
        gsl_error(
            b"matrix A must have M >= N\0" as *const u8 as *const libc::c_char,
            b"bidiag.c\0" as *const u8 as *const libc::c_char,
            256 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*tau_U).size != K {
        gsl_error(
            b"size of tau must be MIN(M,N)\0" as *const u8 as *const libc::c_char,
            b"bidiag.c\0" as *const u8 as *const libc::c_char,
            260 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if ((*tau_V).size).wrapping_add(1 as libc::c_int as libc::c_ulong) != K {
        gsl_error(
            b"size of tau must be MIN(M,N) - 1\0" as *const u8 as *const libc::c_char,
            b"bidiag.c\0" as *const u8 as *const libc::c_char,
            264 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*V).size1 != N || (*V).size2 != N {
        gsl_error(
            b"size of V must be N x N\0" as *const u8 as *const libc::c_char,
            b"bidiag.c\0" as *const u8 as *const libc::c_char,
            268 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        gsl_matrix_set_identity(V);
        i = N.wrapping_sub(1 as libc::c_int as libc::c_ulong);
        loop {
            let fresh2 = i;
            i = i.wrapping_sub(1);
            if !(fresh2 > 0 as libc::c_int as libc::c_ulong) {
                break;
            }
            let r: gsl_vector_const_view = gsl_matrix_const_row(A, i);
            let h: gsl_vector_const_view = gsl_vector_const_subvector(
                &r.vector,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(i.wrapping_add(1 as libc::c_int as libc::c_ulong)),
            );
            let mut ti: libc::c_double = gsl_vector_get(tau_V, i);
            let mut m: gsl_matrix_view = gsl_matrix_submatrix(
                V,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(i.wrapping_add(1 as libc::c_int as libc::c_ulong)),
                N.wrapping_sub(i.wrapping_add(1 as libc::c_int as libc::c_ulong)),
            );
            gsl_linalg_householder_hm(ti, &h.vector, &mut m.matrix);
        }
        i = 0 as libc::c_int as size_t;
        while i < N.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            let mut Aij: libc::c_double = gsl_matrix_get(
                A,
                i,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            gsl_vector_set(tau_V, i, Aij);
            i = i.wrapping_add(1);
            i;
        }
        j = N;
        loop {
            let fresh3 = j;
            j = j.wrapping_sub(1);
            if !(fresh3 > 0 as libc::c_int as libc::c_ulong) {
                break;
            }
            let mut tj: libc::c_double = gsl_vector_get(tau_U, j);
            let mut Ajj: libc::c_double = gsl_matrix_get(A, j, j);
            let mut m_0: gsl_matrix_view = gsl_matrix_submatrix(
                A,
                j,
                j,
                M.wrapping_sub(j),
                N.wrapping_sub(j),
            );
            gsl_vector_set(tau_U, j, Ajj);
            gsl_linalg_householder_hm1(tj, &mut m_0.matrix);
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_bidiag_unpack_B(
    mut A: *const gsl_matrix,
    mut diag: *mut gsl_vector,
    mut superdiag: *mut gsl_vector,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    let K: size_t = if M < N { M } else { N };
    if (*diag).size != K {
        gsl_error(
            b"size of diagonal must match size of A\0" as *const u8
                as *const libc::c_char,
            b"bidiag.c\0" as *const u8 as *const libc::c_char,
            332 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if ((*superdiag).size).wrapping_add(1 as libc::c_int as libc::c_ulong) != K {
        gsl_error(
            b"size of subdiagonal must be (matrix size - 1)\0" as *const u8
                as *const libc::c_char,
            b"bidiag.c\0" as *const u8 as *const libc::c_char,
            336 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < K {
            let mut Aii: libc::c_double = gsl_matrix_get(A, i, i);
            gsl_vector_set(diag, i, Aii);
            i = i.wrapping_add(1);
            i;
        }
        i = 0 as libc::c_int as size_t;
        while i < K.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            let mut Aij: libc::c_double = gsl_matrix_get(
                A,
                i,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            gsl_vector_set(superdiag, i, Aij);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
