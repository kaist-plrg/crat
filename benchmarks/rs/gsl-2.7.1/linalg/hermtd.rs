use ::libc;
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_complex_alloc(n: size_t) -> *mut gsl_vector_complex;
    fn gsl_vector_complex_free(v: *mut gsl_vector_complex);
    fn gsl_vector_complex_subvector(
        base: *mut gsl_vector_complex,
        i: size_t,
        n: size_t,
    ) -> _gsl_vector_complex_view;
    fn gsl_vector_complex_const_real(
        v: *const gsl_vector_complex,
    ) -> _gsl_vector_const_view;
    fn gsl_matrix_complex_submatrix(
        m: *mut gsl_matrix_complex,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_complex_view;
    fn gsl_matrix_complex_column(
        m: *mut gsl_matrix_complex,
        j: size_t,
    ) -> _gsl_vector_complex_view;
    fn gsl_matrix_complex_const_diagonal(
        m: *const gsl_matrix_complex,
    ) -> _gsl_vector_complex_const_view;
    fn gsl_matrix_complex_const_subdiagonal(
        m: *const gsl_matrix_complex,
        k: size_t,
    ) -> _gsl_vector_complex_const_view;
    fn gsl_matrix_complex_const_subcolumn(
        m: *const gsl_matrix_complex,
        j: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_complex_const_view;
    fn gsl_matrix_complex_set_identity(m: *mut gsl_matrix_complex);
    fn gsl_blas_zdotc(
        X: *const gsl_vector_complex,
        Y: *const gsl_vector_complex,
        dotc: *mut gsl_complex,
    ) -> libc::c_int;
    fn gsl_blas_zaxpy(
        alpha: gsl_complex,
        X: *const gsl_vector_complex,
        Y: *mut gsl_vector_complex,
    ) -> libc::c_int;
    fn gsl_blas_zhemv(
        Uplo: CBLAS_UPLO_t,
        alpha: gsl_complex,
        A: *const gsl_matrix_complex,
        X: *const gsl_vector_complex,
        beta: gsl_complex,
        Y: *mut gsl_vector_complex,
    ) -> libc::c_int;
    fn gsl_blas_zher2(
        Uplo: CBLAS_UPLO_t,
        alpha: gsl_complex,
        X: *const gsl_vector_complex,
        Y: *const gsl_vector_complex,
        A: *mut gsl_matrix_complex,
    ) -> libc::c_int;
    fn gsl_complex_mul(a: gsl_complex, b: gsl_complex) -> gsl_complex;
    fn gsl_complex_mul_real(a: gsl_complex, x: libc::c_double) -> gsl_complex;
    fn gsl_linalg_complex_householder_transform(
        v: *mut gsl_vector_complex,
    ) -> gsl_complex;
    fn gsl_linalg_complex_householder_left(
        tau: gsl_complex,
        v: *const gsl_vector_complex,
        A: *mut gsl_matrix_complex,
        work: *mut gsl_vector_complex,
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
pub struct gsl_complex {
    pub dat: [libc::c_double; 2],
}
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
pub struct _gsl_vector_const_view {
    pub vector: gsl_vector,
}
pub type gsl_vector_const_view = _gsl_vector_const_view;
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
pub type CBLAS_UPLO = libc::c_uint;
pub const CblasLower: CBLAS_UPLO = 122;
pub const CblasUpper: CBLAS_UPLO = 121;
pub type CBLAS_UPLO_t = CBLAS_UPLO;
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
unsafe extern "C" fn gsl_complex_rect(
    mut x: libc::c_double,
    mut y: libc::c_double,
) -> gsl_complex {
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    z.dat[0 as libc::c_int as usize] = x;
    z.dat[1 as libc::c_int as usize] = y;
    return z;
}
pub unsafe extern "C" fn gsl_linalg_hermtd_decomp(
    mut A: *mut gsl_matrix_complex,
    mut tau: *mut gsl_vector_complex,
) -> libc::c_int {
    if (*A).size1 != (*A).size2 {
        gsl_error(
            b"hermitian tridiagonal decomposition requires square matrix\0" as *const u8
                as *const libc::c_char,
            b"hermtd.c\0" as *const u8 as *const libc::c_char,
            66 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if ((*tau).size).wrapping_add(1 as libc::c_int as libc::c_ulong) != (*A).size1
    {
        gsl_error(
            b"size of tau must be (matrix size - 1)\0" as *const u8
                as *const libc::c_char,
            b"hermtd.c\0" as *const u8 as *const libc::c_char,
            70 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let N: size_t = (*A).size1;
        let mut i: size_t = 0;
        let zero: gsl_complex = gsl_complex_rect(0.0f64, 0.0f64);
        let one: gsl_complex = gsl_complex_rect(1.0f64, 0.0f64);
        let neg_one: gsl_complex = gsl_complex_rect(-1.0f64, 0.0f64);
        i = 0 as libc::c_int as size_t;
        while i < N.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            let mut c: gsl_vector_complex_view = gsl_matrix_complex_column(A, i);
            let mut v: gsl_vector_complex_view = gsl_vector_complex_subvector(
                &mut c.vector,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(i.wrapping_add(1 as libc::c_int as libc::c_ulong)),
            );
            let mut tau_i: gsl_complex = gsl_linalg_complex_householder_transform(
                &mut v.vector,
            );
            if i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                < N.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                && !(tau_i.dat[0 as libc::c_int as usize] == 0.0f64
                    && tau_i.dat[1 as libc::c_int as usize] == 0.0f64)
            {
                let mut m: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
                    A,
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    N.wrapping_sub(i.wrapping_add(1 as libc::c_int as libc::c_ulong)),
                    N.wrapping_sub(i.wrapping_add(1 as libc::c_int as libc::c_ulong)),
                );
                let mut ei: gsl_complex = gsl_vector_complex_get(
                    &mut v.vector,
                    0 as libc::c_int as size_t,
                );
                let mut x: gsl_vector_complex_view = gsl_vector_complex_subvector(
                    tau,
                    i,
                    N.wrapping_sub(i.wrapping_add(1 as libc::c_int as libc::c_ulong)),
                );
                gsl_vector_complex_set(&mut v.vector, 0 as libc::c_int as size_t, one);
                gsl_blas_zhemv(
                    CblasLower,
                    tau_i,
                    &mut m.matrix,
                    &mut v.vector,
                    zero,
                    &mut x.vector,
                );
                let mut xv: gsl_complex = gsl_complex { dat: [0.; 2] };
                let mut txv: gsl_complex = gsl_complex { dat: [0.; 2] };
                let mut alpha: gsl_complex = gsl_complex { dat: [0.; 2] };
                gsl_blas_zdotc(&mut x.vector, &mut v.vector, &mut xv);
                txv = gsl_complex_mul(tau_i, xv);
                alpha = gsl_complex_mul_real(txv, -0.5f64);
                gsl_blas_zaxpy(alpha, &mut v.vector, &mut x.vector);
                gsl_blas_zher2(
                    CblasLower,
                    neg_one,
                    &mut v.vector,
                    &mut x.vector,
                    &mut m.matrix,
                );
                gsl_vector_complex_set(&mut v.vector, 0 as libc::c_int as size_t, ei);
            }
            gsl_vector_complex_set(tau, i, tau_i);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_hermtd_unpack(
    mut A: *const gsl_matrix_complex,
    mut tau: *const gsl_vector_complex,
    mut U: *mut gsl_matrix_complex,
    mut diag: *mut gsl_vector,
    mut sdiag: *mut gsl_vector,
) -> libc::c_int {
    if (*A).size1 != (*A).size2 {
        gsl_error(
            b"matrix A must be sqaure\0" as *const u8 as *const libc::c_char,
            b"hermtd.c\0" as *const u8 as *const libc::c_char,
            136 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if ((*tau).size).wrapping_add(1 as libc::c_int as libc::c_ulong) != (*A).size1
    {
        gsl_error(
            b"size of tau must be (matrix size - 1)\0" as *const u8
                as *const libc::c_char,
            b"hermtd.c\0" as *const u8 as *const libc::c_char,
            140 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*U).size1 != (*A).size1 || (*U).size2 != (*A).size1 {
        gsl_error(
            b"size of U must match size of A\0" as *const u8 as *const libc::c_char,
            b"hermtd.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*diag).size != (*A).size1 {
        gsl_error(
            b"size of diagonal must match size of A\0" as *const u8
                as *const libc::c_char,
            b"hermtd.c\0" as *const u8 as *const libc::c_char,
            148 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if ((*sdiag).size).wrapping_add(1 as libc::c_int as libc::c_ulong)
        != (*A).size1
    {
        gsl_error(
            b"size of subdiagonal must be (matrix size - 1)\0" as *const u8
                as *const libc::c_char,
            b"hermtd.c\0" as *const u8 as *const libc::c_char,
            152 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let N: size_t = (*A).size1;
        let zd: gsl_vector_complex_const_view = gsl_matrix_complex_const_diagonal(A);
        let zsd: gsl_vector_complex_const_view = gsl_matrix_complex_const_subdiagonal(
            A,
            1 as libc::c_int as size_t,
        );
        let d: gsl_vector_const_view = gsl_vector_complex_const_real(&zd.vector);
        let sd: gsl_vector_const_view = gsl_vector_complex_const_real(&zsd.vector);
        let mut work: *mut gsl_vector_complex = gsl_vector_complex_alloc(N);
        let mut i: size_t = 0;
        gsl_matrix_complex_set_identity(U);
        i = N.wrapping_sub(1 as libc::c_int as libc::c_ulong);
        loop {
            let fresh0 = i;
            i = i.wrapping_sub(1);
            if !(fresh0 > 0 as libc::c_int as libc::c_ulong) {
                break;
            }
            let mut ti: gsl_complex = gsl_vector_complex_get(tau, i);
            let h: gsl_vector_complex_const_view = gsl_matrix_complex_const_subcolumn(
                A,
                i,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            let mut m: gsl_matrix_complex_view = gsl_matrix_complex_submatrix(
                U,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            let mut w: gsl_vector_complex_view = gsl_vector_complex_subvector(
                work,
                0 as libc::c_int as size_t,
                N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            gsl_linalg_complex_householder_left(
                ti,
                &h.vector,
                &mut m.matrix,
                &mut w.vector,
            );
        }
        gsl_vector_memcpy(diag, &d.vector);
        gsl_vector_memcpy(sdiag, &sd.vector);
        gsl_vector_complex_free(work);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_hermtd_unpack_T(
    mut A: *const gsl_matrix_complex,
    mut diag: *mut gsl_vector,
    mut sdiag: *mut gsl_vector,
) -> libc::c_int {
    if (*A).size1 != (*A).size2 {
        gsl_error(
            b"matrix A must be sqaure\0" as *const u8 as *const libc::c_char,
            b"hermtd.c\0" as *const u8 as *const libc::c_char,
            198 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*diag).size != (*A).size1 {
        gsl_error(
            b"size of diagonal must match size of A\0" as *const u8
                as *const libc::c_char,
            b"hermtd.c\0" as *const u8 as *const libc::c_char,
            202 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if ((*sdiag).size).wrapping_add(1 as libc::c_int as libc::c_ulong)
        != (*A).size1
    {
        gsl_error(
            b"size of subdiagonal must be (matrix size - 1)\0" as *const u8
                as *const libc::c_char,
            b"hermtd.c\0" as *const u8 as *const libc::c_char,
            206 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let zd: gsl_vector_complex_const_view = gsl_matrix_complex_const_diagonal(A);
        let zsd: gsl_vector_complex_const_view = gsl_matrix_complex_const_subdiagonal(
            A,
            1 as libc::c_int as size_t,
        );
        let d: gsl_vector_const_view = gsl_vector_complex_const_real(&zd.vector);
        let sd: gsl_vector_const_view = gsl_vector_complex_const_real(&zsd.vector);
        gsl_vector_memcpy(diag, &d.vector);
        gsl_vector_memcpy(sdiag, &sd.vector);
        return GSL_SUCCESS as libc::c_int;
    };
}
