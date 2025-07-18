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
    fn gsl_vector_scale(a: *mut gsl_vector, x: libc::c_double) -> libc::c_int;
    fn gsl_matrix_submatrix(
        m: *mut gsl_matrix,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_view;
    fn gsl_matrix_diagonal(m: *mut gsl_matrix) -> _gsl_vector_view;
    fn gsl_matrix_subcolumn(
        m: *mut gsl_matrix,
        j: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
    fn gsl_matrix_transpose_tricpy(
        Uplo_src: CBLAS_UPLO_t,
        Diag: CBLAS_DIAG_t,
        dest: *mut gsl_matrix,
        src: *const gsl_matrix,
    ) -> libc::c_int;
    fn gsl_blas_dsyr(
        Uplo: CBLAS_UPLO_t,
        alpha: libc::c_double,
        X: *const gsl_vector,
        A: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_linalg_pcholesky_solve(
        LDLT: *const gsl_matrix,
        p: *const gsl_permutation,
        b: *const gsl_vector,
        x: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_linalg_pcholesky_svx(
        LDLT: *const gsl_matrix,
        p: *const gsl_permutation,
        x: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_linalg_pcholesky_invert(
        LDLT: *const gsl_matrix,
        p: *const gsl_permutation,
        Ainv: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_linalg_pcholesky_rcond(
        LDLT: *const gsl_matrix,
        p: *const gsl_permutation,
        rcond: *mut libc::c_double,
        work: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_permutation_init(p: *mut gsl_permutation);
    fn gsl_permutation_swap(
        p: *mut gsl_permutation,
        i: size_t,
        j: size_t,
    ) -> libc::c_int;
    fn gsl_permute_vector_inverse(
        p: *const gsl_permutation,
        v: *mut gsl_vector,
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
pub struct gsl_permutation_struct {
    pub size: size_t,
    pub data: *mut size_t,
}
pub type gsl_permutation = gsl_permutation_struct;
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
pub unsafe extern "C" fn gsl_linalg_mcholesky_decomp(
    mut A: *mut gsl_matrix,
    mut p: *mut gsl_permutation,
    mut E: *mut gsl_vector,
) -> libc::c_int {
    let N: size_t = (*A).size1;
    if N != (*A).size2 {
        gsl_error(
            b"LDLT decomposition requires square matrix\0" as *const u8
                as *const libc::c_char,
            b"mcholesky.c\0" as *const u8 as *const libc::c_char,
            83 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*p).size != N {
        gsl_error(
            b"permutation length must match matrix size\0" as *const u8
                as *const libc::c_char,
            b"mcholesky.c\0" as *const u8 as *const libc::c_char,
            87 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let delta: libc::c_double = 2.2204460492503131e-16f64;
        let mut beta: libc::c_double = 0.;
        let mut gamma: libc::c_double = 0.0f64;
        let mut xi: libc::c_double = 0.0f64;
        let mut diag: gsl_vector_view = gsl_matrix_diagonal(A);
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        gsl_matrix_transpose_tricpy(CblasLower, CblasUnit, A, A);
        gsl_permutation_init(p);
        i = 0 as libc::c_int as size_t;
        while i < N {
            let mut aii: libc::c_double = gsl_matrix_get(A, i, i);
            gamma = if gamma > fabs(aii) { gamma } else { fabs(aii) };
            j = 0 as libc::c_int as size_t;
            while j < i {
                let mut aij: libc::c_double = gsl_matrix_get(A, i, j);
                xi = if xi > fabs(aij) { xi } else { fabs(aij) };
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        if N == 1 as libc::c_int as libc::c_ulong {
            beta = if (if gamma > xi { gamma } else { xi }) > 2.2204460492503131e-16f64 {
                if gamma > xi { gamma } else { xi }
            } else {
                2.2204460492503131e-16f64
            };
        } else {
            let mut nu: libc::c_double = sqrt(
                N.wrapping_mul(N) as libc::c_double - 1.0f64,
            );
            beta = if (if gamma > xi / nu { gamma } else { xi / nu })
                > 2.2204460492503131e-16f64
            {
                if gamma > xi / nu { gamma } else { xi / nu }
            } else {
                2.2204460492503131e-16f64
            };
        }
        beta = sqrt(beta);
        j = 0 as libc::c_int as size_t;
        while j < N {
            let mut ajj: libc::c_double = 0.;
            let mut thetaj: libc::c_double = 0.;
            let mut u: libc::c_double = 0.;
            let mut alpha: libc::c_double = 0.;
            let mut alphainv: libc::c_double = 0.;
            let mut w: gsl_vector_view = gsl_vector_view {
                vector: gsl_vector {
                    size: 0,
                    stride: 0,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block,
                    owner: 0,
                },
            };
            let mut q: size_t = 0;
            w = gsl_vector_subvector(&mut diag.vector, j, N.wrapping_sub(j));
            q = (mcholesky_maxabs(&mut w.vector, 0 as *mut libc::c_double))
                .wrapping_add(j);
            gsl_permutation_swap(p, q, j);
            cholesky_swap_rowcol(A, q, j);
            if j < N.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                w = gsl_matrix_subcolumn(
                    A,
                    j,
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    N.wrapping_sub(j).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                mcholesky_maxabs(&mut w.vector, &mut thetaj);
            } else {
                thetaj = 0.0f64;
            }
            u = thetaj / beta;
            ajj = gsl_matrix_get(A, j, j);
            alpha = if (if delta > fabs(ajj) { delta } else { fabs(ajj) }) > u * u {
                if delta > fabs(ajj) { delta } else { fabs(ajj) }
            } else {
                u * u
            };
            alphainv = 1.0f64 / alpha;
            if j < N.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                let mut v: gsl_vector_view = gsl_matrix_subcolumn(
                    A,
                    j,
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    N.wrapping_sub(j).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                let mut m: gsl_matrix_view = gsl_matrix_submatrix(
                    A,
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    N.wrapping_sub(j).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    N.wrapping_sub(j).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                gsl_blas_dsyr(CblasLower, -alphainv, &mut v.vector, &mut m.matrix);
                gsl_vector_scale(&mut v.vector, alphainv);
            }
            if !E.is_null() {
                gsl_vector_set(E, j, alpha - ajj);
            }
            gsl_matrix_set(A, j, j, alpha);
            j = j.wrapping_add(1);
            j;
        }
        if !E.is_null() {
            gsl_permute_vector_inverse(p, E);
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_mcholesky_solve(
    mut LDLT: *const gsl_matrix,
    mut p: *const gsl_permutation,
    mut b: *const gsl_vector,
    mut x: *mut gsl_vector,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_linalg_pcholesky_solve(LDLT, p, b, x);
    return status;
}
pub unsafe extern "C" fn gsl_linalg_mcholesky_svx(
    mut LDLT: *const gsl_matrix,
    mut p: *const gsl_permutation,
    mut x: *mut gsl_vector,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_linalg_pcholesky_svx(LDLT, p, x);
    return status;
}
pub unsafe extern "C" fn gsl_linalg_mcholesky_rcond(
    mut LDLT: *const gsl_matrix,
    mut p: *const gsl_permutation,
    mut rcond: *mut libc::c_double,
    mut work: *mut gsl_vector,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_linalg_pcholesky_rcond(LDLT, p, rcond, work);
    return status;
}
pub unsafe extern "C" fn gsl_linalg_mcholesky_invert(
    mut LDLT: *const gsl_matrix,
    mut p: *const gsl_permutation,
    mut Ainv: *mut gsl_matrix,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_linalg_pcholesky_invert(LDLT, p, Ainv);
    return status;
}
unsafe extern "C" fn mcholesky_maxabs(
    mut v: *const gsl_vector,
    mut maxabs: *mut libc::c_double,
) -> size_t {
    let n: size_t = (*v).size;
    let mut i: size_t = 0;
    let mut idx: size_t = 0 as libc::c_int as size_t;
    let mut max: libc::c_double = gsl_vector_get(v, idx);
    i = 1 as libc::c_int as size_t;
    while i < n {
        let mut vi: libc::c_double = gsl_vector_get(v, i);
        let mut absvi: libc::c_double = fabs(vi);
        if absvi > max {
            max = absvi;
            idx = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    if !maxabs.is_null() {
        *maxabs = max;
    }
    return idx;
}
