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
    fn gsl_matrix_superdiagonal(m: *mut gsl_matrix, k: size_t) -> _gsl_vector_view;
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
    fn gsl_linalg_householder_transform(v: *mut gsl_vector) -> libc::c_double;
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
pub unsafe extern "C" fn gsl_linalg_QR_band_decomp_L2(
    M: size_t,
    p: size_t,
    q: size_t,
    mut AB: *mut gsl_matrix,
    mut tau: *mut gsl_vector,
) -> libc::c_int {
    let N: size_t = (*AB).size1;
    if (*tau).size != N {
        gsl_error(
            b"tau must have length N\0" as *const u8 as *const libc::c_char,
            b"qr_band.c\0" as *const u8 as *const libc::c_char,
            75 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*AB).size2
        != (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(p)
            .wrapping_add(q)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        gsl_error(
            b"dimensions of AB are inconsistent with (p,q)\0" as *const u8
                as *const libc::c_char,
            b"qr_band.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let minMN: size_t = if M < N { M } else { N };
        let mut j: size_t = 0;
        if p > 0 as libc::c_int as libc::c_ulong {
            let mut m: gsl_matrix_view = gsl_matrix_submatrix(
                AB,
                0 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                N,
                p,
            );
            gsl_matrix_set_zero(&mut m.matrix);
        }
        j = 0 as libc::c_int as size_t;
        while j < minMN {
            let mut k1: size_t = if p.wrapping_add(1 as libc::c_int as libc::c_ulong)
                < M.wrapping_sub(j)
            {
                p.wrapping_add(1 as libc::c_int as libc::c_ulong)
            } else {
                M.wrapping_sub(j)
            };
            let mut k2: size_t = if p.wrapping_add(q)
                < N.wrapping_sub(j).wrapping_sub(1 as libc::c_int as libc::c_ulong)
            {
                p.wrapping_add(q)
            } else {
                N.wrapping_sub(j).wrapping_sub(1 as libc::c_int as libc::c_ulong)
            };
            let mut c: gsl_vector_view = gsl_matrix_subrow(AB, j, p.wrapping_add(q), k1);
            let mut tau_j: libc::c_double = gsl_linalg_householder_transform(
                &mut c.vector,
            );
            let mut ptr: *mut libc::c_double = gsl_vector_ptr(
                &mut c.vector,
                0 as libc::c_int as size_t,
            );
            gsl_vector_set(tau, j, tau_j);
            if k2 > 0 as libc::c_int as libc::c_ulong {
                let mut m_0: gsl_matrix_view = gsl_matrix_submatrix(
                    AB,
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    p.wrapping_add(q).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    k2,
                    k1,
                );
                let mut work: gsl_vector_view = gsl_vector_subvector(
                    tau,
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    k2,
                );
                let mut tmp: libc::c_double = *ptr;
                m_0
                    .matrix
                    .tda = (m_0.matrix.tda as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as size_t as size_t;
                *ptr = 1.0f64;
                gsl_linalg_householder_right(
                    tau_j,
                    &mut c.vector,
                    &mut m_0.matrix,
                    &mut work.vector,
                );
                *ptr = tmp;
            }
            j = j.wrapping_add(1);
            j;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_QR_band_unpack_L2(
    p: size_t,
    q: size_t,
    mut QRB: *const gsl_matrix,
    mut tau: *const gsl_vector,
    mut Q: *mut gsl_matrix,
    mut R: *mut gsl_matrix,
) -> libc::c_int {
    let M: size_t = (*Q).size1;
    let N: size_t = (*QRB).size1;
    if (*Q).size2 != M {
        gsl_error(
            b"Q matrix must be square\0" as *const u8 as *const libc::c_char,
            b"qr_band.c\0" as *const u8 as *const libc::c_char,
            140 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*R).size1 != M || (*R).size2 != N {
        gsl_error(
            b"R matrix must be M x N\0" as *const u8 as *const libc::c_char,
            b"qr_band.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*tau).size < (if M < N { M } else { N }) {
        gsl_error(
            b"size of tau must be at least MIN(M,N)\0" as *const u8
                as *const libc::c_char,
            b"qr_band.c\0" as *const u8 as *const libc::c_char,
            148 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*QRB).size2
        != (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(p)
            .wrapping_add(q)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        gsl_error(
            b"dimensions of QRB are inconsistent with (p,q)\0" as *const u8
                as *const libc::c_char,
            b"qr_band.c\0" as *const u8 as *const libc::c_char,
            152 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        gsl_matrix_set_identity(Q);
        i = if M < N { M } else { N };
        loop {
            let fresh0 = i;
            i = i.wrapping_sub(1);
            if !(fresh0 > 0 as libc::c_int as libc::c_ulong) {
                break;
            }
            let mut k1: size_t = if p.wrapping_add(1 as libc::c_int as libc::c_ulong)
                < M.wrapping_sub(i)
            {
                p.wrapping_add(1 as libc::c_int as libc::c_ulong)
            } else {
                M.wrapping_sub(i)
            };
            let h: gsl_vector_const_view = gsl_matrix_const_subrow(
                QRB,
                i,
                p.wrapping_add(q),
                k1,
            );
            let mut m: gsl_matrix_view = gsl_matrix_submatrix(
                Q,
                i,
                i,
                k1,
                M.wrapping_sub(i),
            );
            let mut ti: libc::c_double = gsl_vector_get(tau, i);
            let mut work: gsl_vector_view = gsl_matrix_subcolumn(
                R,
                0 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                M.wrapping_sub(i),
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
        gsl_matrix_set_zero(R);
        i = 0 as libc::c_int as size_t;
        while i
            <= (if p.wrapping_add(q) < N.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            {
                p.wrapping_add(q)
            } else {
                N.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            })
        {
            let src: gsl_vector_const_view = gsl_matrix_const_subcolumn(
                QRB,
                p.wrapping_add(q).wrapping_sub(i),
                i,
                if M < N.wrapping_sub(i) { M } else { N.wrapping_sub(i) },
            );
            let mut dest: gsl_vector_view = gsl_matrix_superdiagonal(R, i);
            gsl_vector_memcpy(&mut dest.vector, &src.vector);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
