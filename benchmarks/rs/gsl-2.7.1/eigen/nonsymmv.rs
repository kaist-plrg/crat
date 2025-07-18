use ::libc;
extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_alloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_vector_view_array(v: *mut libc::c_double, n: size_t) -> _gsl_vector_view;
    fn gsl_vector_subvector(
        v: *mut gsl_vector,
        i: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
    fn gsl_vector_complex_view_array(
        base: *mut libc::c_double,
        n: size_t,
    ) -> _gsl_vector_complex_view;
    fn gsl_vector_complex_real(v: *mut gsl_vector_complex) -> _gsl_vector_view;
    fn gsl_vector_complex_imag(v: *mut gsl_vector_complex) -> _gsl_vector_view;
    fn gsl_matrix_complex_column(
        m: *mut gsl_matrix_complex,
        j: size_t,
    ) -> _gsl_vector_complex_view;
    fn gsl_matrix_submatrix(
        m: *mut gsl_matrix,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_view;
    fn gsl_matrix_column(m: *mut gsl_matrix, j: size_t) -> _gsl_vector_view;
    fn gsl_matrix_subcolumn(
        m: *mut gsl_matrix,
        j: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
    fn gsl_matrix_memcpy(dest: *mut gsl_matrix, src: *const gsl_matrix) -> libc::c_int;
    fn gsl_eigen_nonsymm_alloc(n: size_t) -> *mut gsl_eigen_nonsymm_workspace;
    fn gsl_eigen_nonsymm_free(w: *mut gsl_eigen_nonsymm_workspace);
    fn gsl_eigen_nonsymm_params(
        compute_t: libc::c_int,
        balance: libc::c_int,
        w: *mut gsl_eigen_nonsymm_workspace,
    );
    fn gsl_eigen_nonsymm_Z(
        A: *mut gsl_matrix,
        eval: *mut gsl_vector_complex,
        Z: *mut gsl_matrix,
        w: *mut gsl_eigen_nonsymm_workspace,
    ) -> libc::c_int;
    fn gsl_schur_solve_equation(
        ca: libc::c_double,
        A: *const gsl_matrix,
        z: libc::c_double,
        d1: libc::c_double,
        d2: libc::c_double,
        b: *const gsl_vector,
        x: *mut gsl_vector,
        s: *mut libc::c_double,
        xnorm: *mut libc::c_double,
        smin: libc::c_double,
    ) -> libc::c_int;
    fn gsl_schur_solve_equation_z(
        ca: libc::c_double,
        A: *const gsl_matrix,
        z: *mut gsl_complex,
        d1: libc::c_double,
        d2: libc::c_double,
        b: *const gsl_vector_complex,
        x: *mut gsl_vector_complex,
        s: *mut libc::c_double,
        xnorm: *mut libc::c_double,
        smin: libc::c_double,
    ) -> libc::c_int;
    fn gsl_blas_dnrm2(X: *const gsl_vector) -> libc::c_double;
    fn gsl_blas_daxpy(
        alpha: libc::c_double,
        X: *const gsl_vector,
        Y: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_blas_dscal(alpha: libc::c_double, X: *mut gsl_vector);
    fn gsl_blas_zdscal(alpha: libc::c_double, X: *mut gsl_vector_complex);
    fn gsl_blas_dgemv(
        TransA: CBLAS_TRANSPOSE_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        X: *const gsl_vector,
        beta: libc::c_double,
        Y: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_hypot(x: libc::c_double, y: libc::c_double) -> libc::c_double;
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
pub type CBLAS_TRANSPOSE = libc::c_uint;
pub const CblasConjTrans: CBLAS_TRANSPOSE = 113;
pub const CblasTrans: CBLAS_TRANSPOSE = 112;
pub const CblasNoTrans: CBLAS_TRANSPOSE = 111;
pub type CBLAS_TRANSPOSE_t = CBLAS_TRANSPOSE;
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
pub struct gsl_eigen_francis_workspace {
    pub size: size_t,
    pub max_iterations: size_t,
    pub n_iter: size_t,
    pub n_evals: size_t,
    pub compute_t: libc::c_int,
    pub H: *mut gsl_matrix,
    pub Z: *mut gsl_matrix,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_eigen_nonsymm_workspace {
    pub size: size_t,
    pub diag: *mut gsl_vector,
    pub tau: *mut gsl_vector,
    pub Z: *mut gsl_matrix,
    pub do_balance: libc::c_int,
    pub n_evals: size_t,
    pub francis_workspace_p: *mut gsl_eigen_francis_workspace,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_eigen_nonsymmv_workspace {
    pub size: size_t,
    pub work: *mut gsl_vector,
    pub work2: *mut gsl_vector,
    pub work3: *mut gsl_vector,
    pub Z: *mut gsl_matrix,
    pub nonsymm_workspace_p: *mut gsl_eigen_nonsymm_workspace,
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
unsafe extern "C" fn gsl_matrix_get(
    mut m: *const gsl_matrix,
    i: size_t,
    j: size_t,
) -> libc::c_double {
    return *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
pub unsafe extern "C" fn gsl_eigen_nonsymmv_alloc(
    n: size_t,
) -> *mut gsl_eigen_nonsymmv_workspace {
    let mut w: *mut gsl_eigen_nonsymmv_workspace = 0
        as *mut gsl_eigen_nonsymmv_workspace;
    if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix dimension must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"nonsymmv.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_eigen_nonsymmv_workspace;
    }
    w = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gsl_eigen_nonsymmv_workspace>() as libc::c_ulong,
    ) as *mut gsl_eigen_nonsymmv_workspace;
    if w.is_null() {
        gsl_error(
            b"failed to allocate space for workspace\0" as *const u8
                as *const libc::c_char,
            b"nonsymmv.c\0" as *const u8 as *const libc::c_char,
            80 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_eigen_nonsymmv_workspace;
    }
    (*w).size = n;
    (*w).Z = 0 as *mut gsl_matrix;
    (*w).nonsymm_workspace_p = gsl_eigen_nonsymm_alloc(n);
    if ((*w).nonsymm_workspace_p).is_null() {
        gsl_eigen_nonsymmv_free(w);
        gsl_error(
            b"failed to allocate space for nonsymm workspace\0" as *const u8
                as *const libc::c_char,
            b"nonsymmv.c\0" as *const u8 as *const libc::c_char,
            90 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_eigen_nonsymmv_workspace;
    }
    gsl_eigen_nonsymm_params(
        1 as libc::c_int,
        0 as libc::c_int,
        (*w).nonsymm_workspace_p,
    );
    (*w).work = gsl_vector_alloc(n);
    (*w).work2 = gsl_vector_alloc(n);
    (*w).work3 = gsl_vector_alloc(n);
    if ((*w).work).is_null() || ((*w).work2).is_null() || ((*w).work3).is_null() {
        gsl_eigen_nonsymmv_free(w);
        gsl_error(
            b"failed to allocate space for nonsymmv additional workspace\0" as *const u8
                as *const libc::c_char,
            b"nonsymmv.c\0" as *const u8 as *const libc::c_char,
            105 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_eigen_nonsymmv_workspace;
    }
    return w;
}
pub unsafe extern "C" fn gsl_eigen_nonsymmv_free(
    mut w: *mut gsl_eigen_nonsymmv_workspace,
) {
    if w.is_null() {
        return;
    }
    if !((*w).nonsymm_workspace_p).is_null() {
        gsl_eigen_nonsymm_free((*w).nonsymm_workspace_p);
    }
    if !((*w).work).is_null() {
        gsl_vector_free((*w).work);
    }
    if !((*w).work2).is_null() {
        gsl_vector_free((*w).work2);
    }
    if !((*w).work3).is_null() {
        gsl_vector_free((*w).work3);
    }
    free(w as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_eigen_nonsymmv_params(
    balance: libc::c_int,
    mut w: *mut gsl_eigen_nonsymmv_workspace,
) {
    gsl_eigen_nonsymm_params(1 as libc::c_int, balance, (*w).nonsymm_workspace_p);
}
pub unsafe extern "C" fn gsl_eigen_nonsymmv(
    mut A: *mut gsl_matrix,
    mut eval: *mut gsl_vector_complex,
    mut evec: *mut gsl_matrix_complex,
    mut w: *mut gsl_eigen_nonsymmv_workspace,
) -> libc::c_int {
    let N: size_t = (*A).size1;
    if N != (*A).size2 {
        gsl_error(
            b"matrix must be square to compute eigenvalues\0" as *const u8
                as *const libc::c_char,
            b"nonsymmv.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*eval).size != N {
        gsl_error(
            b"eigenvalue vector must match matrix size\0" as *const u8
                as *const libc::c_char,
            b"nonsymmv.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*evec).size1 != (*evec).size2 {
        gsl_error(
            b"eigenvector matrix must be square\0" as *const u8 as *const libc::c_char,
            b"nonsymmv.c\0" as *const u8 as *const libc::c_char,
            188 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*evec).size1 != N {
        gsl_error(
            b"eigenvector matrix has wrong size\0" as *const u8 as *const libc::c_char,
            b"nonsymmv.c\0" as *const u8 as *const libc::c_char,
            192 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut s: libc::c_int = 0;
        let mut Z: gsl_matrix = gsl_matrix {
            size1: 0,
            size2: 0,
            tda: 0,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0,
        };
        Z.size1 = N;
        Z.size2 = N;
        Z.tda = (2 as libc::c_int as libc::c_ulong).wrapping_mul(N);
        Z.data = (*evec).data;
        Z.block = 0 as *mut gsl_block;
        Z.owner = 0 as libc::c_int;
        s = gsl_eigen_nonsymm_Z(A, eval, &mut Z, (*w).nonsymm_workspace_p);
        if !((*w).Z).is_null() {
            gsl_matrix_memcpy((*w).Z, &mut Z);
        }
        if s == GSL_SUCCESS as libc::c_int {
            nonsymmv_get_right_eigenvectors(A, &mut Z, eval, evec, w);
            nonsymmv_normalize_eigenvectors(eval, evec);
        }
        return s;
    };
}
pub unsafe extern "C" fn gsl_eigen_nonsymmv_Z(
    mut A: *mut gsl_matrix,
    mut eval: *mut gsl_vector_complex,
    mut evec: *mut gsl_matrix_complex,
    mut Z: *mut gsl_matrix,
    mut w: *mut gsl_eigen_nonsymmv_workspace,
) -> libc::c_int {
    if (*A).size1 != (*A).size2 {
        gsl_error(
            b"matrix must be square to compute eigenvalues/eigenvectors\0" as *const u8
                as *const libc::c_char,
            b"nonsymmv.c\0" as *const u8 as *const libc::c_char,
            262 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*eval).size != (*A).size1 {
        gsl_error(
            b"eigenvalue vector must match matrix size\0" as *const u8
                as *const libc::c_char,
            b"nonsymmv.c\0" as *const u8 as *const libc::c_char,
            266 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*evec).size1 != (*evec).size2 {
        gsl_error(
            b"eigenvector matrix must be square\0" as *const u8 as *const libc::c_char,
            b"nonsymmv.c\0" as *const u8 as *const libc::c_char,
            270 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*evec).size1 != (*A).size1 {
        gsl_error(
            b"eigenvector matrix has wrong size\0" as *const u8 as *const libc::c_char,
            b"nonsymmv.c\0" as *const u8 as *const libc::c_char,
            274 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*Z).size1 != (*Z).size2 || (*Z).size1 != (*A).size1 {
        gsl_error(
            b"Z matrix has wrong dimensions\0" as *const u8 as *const libc::c_char,
            b"nonsymmv.c\0" as *const u8 as *const libc::c_char,
            278 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut s: libc::c_int = 0;
        (*w).Z = Z;
        s = gsl_eigen_nonsymmv(A, eval, evec, w);
        (*w).Z = 0 as *mut gsl_matrix;
        return s;
    };
}
unsafe extern "C" fn nonsymmv_get_right_eigenvectors(
    mut T: *mut gsl_matrix,
    mut Z: *mut gsl_matrix,
    mut eval: *mut gsl_vector_complex,
    mut evec: *mut gsl_matrix_complex,
    mut w: *mut gsl_eigen_nonsymmv_workspace,
) {
    let N: size_t = (*T).size1;
    let smlnum: libc::c_double = 2.2250738585072014e-308f64 * N as libc::c_double
        / 2.2204460492503131e-16f64;
    let bignum: libc::c_double = (1.0f64 - 2.2204460492503131e-16f64) / smlnum;
    let mut i: libc::c_int = 0;
    let mut iu: size_t = 0;
    let mut ju: size_t = 0;
    let mut ii: size_t = 0;
    let mut lambda: gsl_complex = gsl_complex { dat: [0.; 2] };
    let mut lambda_re: libc::c_double = 0.;
    let mut lambda_im: libc::c_double = 0.;
    let mut Tv: gsl_matrix_view = gsl_matrix_view {
        matrix: gsl_matrix {
            size1: 0,
            size2: 0,
            tda: 0,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0,
        },
    };
    let mut Zv: gsl_matrix_view = gsl_matrix_view {
        matrix: gsl_matrix {
            size1: 0,
            size2: 0,
            tda: 0,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0,
        },
    };
    let mut y: gsl_vector_view = gsl_vector_view {
        vector: gsl_vector {
            size: 0,
            stride: 0,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0,
        },
    };
    let mut y2: gsl_vector_view = gsl_vector_view {
        vector: gsl_vector {
            size: 0,
            stride: 0,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0,
        },
    };
    let mut ev: gsl_vector_view = gsl_vector_view {
        vector: gsl_vector {
            size: 0,
            stride: 0,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0,
        },
    };
    let mut ev2: gsl_vector_view = gsl_vector_view {
        vector: gsl_vector {
            size: 0,
            stride: 0,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0,
        },
    };
    let mut dat: [libc::c_double; 4] = [0.; 4];
    let mut dat_X: [libc::c_double; 4] = [0.; 4];
    let mut scale: libc::c_double = 0.;
    let mut xnorm: libc::c_double = 0.;
    let mut ecol: gsl_vector_complex_view = gsl_vector_complex_view {
        vector: gsl_vector_complex {
            size: 0,
            stride: 0,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block_complex,
            owner: 0,
        },
    };
    let mut ecol2: gsl_vector_complex_view = gsl_vector_complex_view {
        vector: gsl_vector_complex {
            size: 0,
            stride: 0,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block_complex,
            owner: 0,
        },
    };
    let mut complex_pair: libc::c_int = 0;
    let mut smin: libc::c_double = 0.;
    gsl_vector_set((*w).work3, 0 as libc::c_int as size_t, 0.0f64);
    ju = 1 as libc::c_int as size_t;
    while ju < N {
        gsl_vector_set((*w).work3, ju, 0.0f64);
        iu = 0 as libc::c_int as size_t;
        while iu < ju {
            gsl_vector_set(
                (*w).work3,
                ju,
                gsl_vector_get((*w).work3, ju) + fabs(gsl_matrix_get(T, iu, ju)),
            );
            iu = iu.wrapping_add(1);
            iu;
        }
        ju = ju.wrapping_add(1);
        ju;
    }
    i = N as libc::c_int - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        iu = i as size_t;
        lambda_re = gsl_matrix_get(T, iu, iu);
        if iu != 0 as libc::c_int as libc::c_ulong
            && gsl_matrix_get(T, iu, iu.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                != 0.0f64
        {
            lambda_im = sqrt(
                fabs(
                    gsl_matrix_get(
                        T,
                        iu,
                        iu.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ),
                ),
            )
                * sqrt(
                    fabs(
                        gsl_matrix_get(
                            T,
                            iu.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            iu,
                        ),
                    ),
                );
        } else {
            lambda_im = 0.0f64;
        }
        lambda.dat[0 as libc::c_int as usize] = lambda_re;
        lambda.dat[1 as libc::c_int as usize] = lambda_im;
        smin = if 2.2204460492503131e-16f64 * (fabs(lambda_re) + fabs(lambda_im))
            > smlnum
        {
            2.2204460492503131e-16f64 * (fabs(lambda_re) + fabs(lambda_im))
        } else {
            smlnum
        };
        smin = if smin > 2.0f64 * 2.2250738585072014e-308f64 {
            smin
        } else {
            2.0f64 * 2.2250738585072014e-308f64
        };
        if lambda_im == 0.0f64 {
            let mut k: libc::c_int = 0;
            let mut l: libc::c_int = 0;
            let mut bv: gsl_vector_view = gsl_vector_view {
                vector: gsl_vector {
                    size: 0,
                    stride: 0,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block,
                    owner: 0,
                },
            };
            let mut xv: gsl_vector_view = gsl_vector_view {
                vector: gsl_vector {
                    size: 0,
                    stride: 0,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block,
                    owner: 0,
                },
            };
            gsl_vector_complex_set(eval, iu, lambda);
            k = 0 as libc::c_int;
            while k < i {
                gsl_vector_set(
                    (*w).work,
                    k as size_t,
                    -gsl_matrix_get(T, k as size_t, iu),
                );
                k += 1;
                k;
            }
            gsl_vector_set((*w).work, iu, 1.0f64);
            l = i - 1 as libc::c_int;
            while l >= 0 as libc::c_int {
                let mut lu: size_t = l as size_t;
                if lu == 0 as libc::c_int as libc::c_ulong {
                    complex_pair = 0 as libc::c_int;
                } else {
                    complex_pair = (gsl_matrix_get(
                        T,
                        lu,
                        lu.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ) != 0.0f64) as libc::c_int;
                }
                if complex_pair == 0 {
                    let mut x: libc::c_double = 0.;
                    Tv = gsl_matrix_submatrix(
                        T,
                        lu,
                        lu,
                        1 as libc::c_int as size_t,
                        1 as libc::c_int as size_t,
                    );
                    bv = gsl_vector_view_array(
                        dat.as_mut_ptr(),
                        1 as libc::c_int as size_t,
                    );
                    gsl_vector_set(
                        &mut bv.vector,
                        0 as libc::c_int as size_t,
                        gsl_vector_get((*w).work, lu),
                    );
                    xv = gsl_vector_view_array(
                        dat_X.as_mut_ptr(),
                        1 as libc::c_int as size_t,
                    );
                    gsl_schur_solve_equation(
                        1.0f64,
                        &mut Tv.matrix,
                        lambda_re,
                        1.0f64,
                        1.0f64,
                        &mut bv.vector,
                        &mut xv.vector,
                        &mut scale,
                        &mut xnorm,
                        smin,
                    );
                    x = gsl_vector_get(&mut xv.vector, 0 as libc::c_int as size_t);
                    if xnorm > 1.0f64 {
                        if gsl_vector_get((*w).work3, lu) > bignum / xnorm {
                            x /= xnorm;
                            scale /= xnorm;
                        }
                    }
                    if scale != 1.0f64 {
                        let mut wv: gsl_vector_view = gsl_vector_view {
                            vector: gsl_vector {
                                size: 0,
                                stride: 0,
                                data: 0 as *mut libc::c_double,
                                block: 0 as *mut gsl_block,
                                owner: 0,
                            },
                        };
                        wv = gsl_vector_subvector(
                            (*w).work,
                            0 as libc::c_int as size_t,
                            iu.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        );
                        gsl_blas_dscal(scale, &mut wv.vector);
                    }
                    gsl_vector_set((*w).work, lu, x);
                    if lu > 0 as libc::c_int as libc::c_ulong {
                        let mut v1: gsl_vector_view = gsl_vector_view {
                            vector: gsl_vector {
                                size: 0,
                                stride: 0,
                                data: 0 as *mut libc::c_double,
                                block: 0 as *mut gsl_block,
                                owner: 0,
                            },
                        };
                        let mut v2: gsl_vector_view = gsl_vector_view {
                            vector: gsl_vector {
                                size: 0,
                                stride: 0,
                                data: 0 as *mut libc::c_double,
                                block: 0 as *mut gsl_block,
                                owner: 0,
                            },
                        };
                        v1 = gsl_matrix_subcolumn(T, lu, 0 as libc::c_int as size_t, lu);
                        v2 = gsl_vector_subvector(
                            (*w).work,
                            0 as libc::c_int as size_t,
                            lu,
                        );
                        gsl_blas_daxpy(-x, &mut v1.vector, &mut v2.vector);
                    }
                } else {
                    let mut x11: libc::c_double = 0.;
                    let mut x21: libc::c_double = 0.;
                    Tv = gsl_matrix_submatrix(
                        T,
                        lu.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        lu.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        2 as libc::c_int as size_t,
                        2 as libc::c_int as size_t,
                    );
                    bv = gsl_vector_view_array(
                        dat.as_mut_ptr(),
                        2 as libc::c_int as size_t,
                    );
                    gsl_vector_set(
                        &mut bv.vector,
                        0 as libc::c_int as size_t,
                        gsl_vector_get(
                            (*w).work,
                            lu.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        ),
                    );
                    gsl_vector_set(
                        &mut bv.vector,
                        1 as libc::c_int as size_t,
                        gsl_vector_get((*w).work, lu),
                    );
                    xv = gsl_vector_view_array(
                        dat_X.as_mut_ptr(),
                        2 as libc::c_int as size_t,
                    );
                    gsl_schur_solve_equation(
                        1.0f64,
                        &mut Tv.matrix,
                        lambda_re,
                        1.0f64,
                        1.0f64,
                        &mut bv.vector,
                        &mut xv.vector,
                        &mut scale,
                        &mut xnorm,
                        smin,
                    );
                    x11 = gsl_vector_get(&mut xv.vector, 0 as libc::c_int as size_t);
                    x21 = gsl_vector_get(&mut xv.vector, 1 as libc::c_int as size_t);
                    if xnorm > 1.0f64 {
                        let mut beta: libc::c_double = 0.;
                        beta = if gsl_vector_get(
                            (*w).work3,
                            lu.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        ) > gsl_vector_get((*w).work3, lu)
                        {
                            gsl_vector_get(
                                (*w).work3,
                                lu.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            )
                        } else {
                            gsl_vector_get((*w).work3, lu)
                        };
                        if beta > bignum / xnorm {
                            x11 /= xnorm;
                            x21 /= xnorm;
                            scale /= xnorm;
                        }
                    }
                    if scale != 1.0f64 {
                        let mut wv_0: gsl_vector_view = gsl_vector_view {
                            vector: gsl_vector {
                                size: 0,
                                stride: 0,
                                data: 0 as *mut libc::c_double,
                                block: 0 as *mut gsl_block,
                                owner: 0,
                            },
                        };
                        wv_0 = gsl_vector_subvector(
                            (*w).work,
                            0 as libc::c_int as size_t,
                            iu.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        );
                        gsl_blas_dscal(scale, &mut wv_0.vector);
                    }
                    gsl_vector_set(
                        (*w).work,
                        lu.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        x11,
                    );
                    gsl_vector_set((*w).work, lu, x21);
                    if lu > 1 as libc::c_int as libc::c_ulong {
                        let mut v1_0: gsl_vector_view = gsl_vector_view {
                            vector: gsl_vector {
                                size: 0,
                                stride: 0,
                                data: 0 as *mut libc::c_double,
                                block: 0 as *mut gsl_block,
                                owner: 0,
                            },
                        };
                        let mut v2_0: gsl_vector_view = gsl_vector_view {
                            vector: gsl_vector {
                                size: 0,
                                stride: 0,
                                data: 0 as *mut libc::c_double,
                                block: 0 as *mut gsl_block,
                                owner: 0,
                            },
                        };
                        v1_0 = gsl_matrix_subcolumn(
                            T,
                            lu.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            0 as libc::c_int as size_t,
                            lu.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        );
                        v2_0 = gsl_vector_subvector(
                            (*w).work,
                            0 as libc::c_int as size_t,
                            lu.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        );
                        gsl_blas_daxpy(-x11, &mut v1_0.vector, &mut v2_0.vector);
                        v1_0 = gsl_matrix_subcolumn(
                            T,
                            lu,
                            0 as libc::c_int as size_t,
                            lu.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        );
                        gsl_blas_daxpy(-x21, &mut v1_0.vector, &mut v2_0.vector);
                    }
                    l -= 1;
                    l;
                }
                l -= 1;
                l;
            }
            ecol = gsl_matrix_complex_column(evec, iu);
            y = gsl_matrix_column(Z, iu);
            if iu > 0 as libc::c_int as libc::c_ulong {
                let mut x_0: gsl_vector_view = gsl_vector_view {
                    vector: gsl_vector {
                        size: 0,
                        stride: 0,
                        data: 0 as *mut libc::c_double,
                        block: 0 as *mut gsl_block,
                        owner: 0,
                    },
                };
                Zv = gsl_matrix_submatrix(
                    Z,
                    0 as libc::c_int as size_t,
                    0 as libc::c_int as size_t,
                    N,
                    iu,
                );
                x_0 = gsl_vector_subvector((*w).work, 0 as libc::c_int as size_t, iu);
                gsl_blas_dgemv(
                    CblasNoTrans,
                    1.0f64,
                    &mut Zv.matrix,
                    &mut x_0.vector,
                    gsl_vector_get((*w).work, iu),
                    &mut y.vector,
                );
            }
            ev = gsl_vector_complex_real(&mut ecol.vector);
            ev2 = gsl_vector_complex_imag(&mut ecol.vector);
            scale = 0.0f64;
            ii = 0 as libc::c_int as size_t;
            while ii < N {
                let mut a: libc::c_double = gsl_vector_get(&mut y.vector, ii);
                gsl_vector_set(&mut ev.vector, ii, a);
                gsl_vector_set(&mut ev2.vector, ii, 0.0f64);
                if fabs(a) > scale {
                    scale = fabs(a);
                }
                ii = ii.wrapping_add(1);
                ii;
            }
            if scale != 0.0f64 {
                scale = 1.0f64 / scale;
            }
            gsl_blas_dscal(scale, &mut ev.vector);
        } else {
            let mut bv_0: gsl_vector_complex_view = gsl_vector_complex_view {
                vector: gsl_vector_complex {
                    size: 0,
                    stride: 0,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block_complex,
                    owner: 0,
                },
            };
            let mut xv_0: gsl_vector_complex_view = gsl_vector_complex_view {
                vector: gsl_vector_complex {
                    size: 0,
                    stride: 0,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block_complex,
                    owner: 0,
                },
            };
            let mut k_0: size_t = 0;
            let mut l_0: libc::c_int = 0;
            let mut lambda2: gsl_complex = gsl_complex { dat: [0.; 2] };
            lambda2
                .dat[0 as libc::c_int as usize] = lambda.dat[0 as libc::c_int as usize];
            lambda2
                .dat[1 as libc::c_int as usize] = -lambda.dat[1 as libc::c_int as usize];
            gsl_vector_complex_set(
                eval,
                iu.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                lambda,
            );
            gsl_vector_complex_set(eval, iu, lambda2);
            if fabs(
                gsl_matrix_get(T, iu.wrapping_sub(1 as libc::c_int as libc::c_ulong), iu),
            )
                >= fabs(
                    gsl_matrix_get(
                        T,
                        iu,
                        iu.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ),
                )
            {
                gsl_vector_set(
                    (*w).work,
                    iu.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    1.0f64,
                );
                gsl_vector_set(
                    (*w).work2,
                    iu,
                    lambda_im
                        / gsl_matrix_get(
                            T,
                            iu.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            iu,
                        ),
                );
            } else {
                gsl_vector_set(
                    (*w).work,
                    iu.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    -lambda_im
                        / gsl_matrix_get(
                            T,
                            iu,
                            iu.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        ),
                );
                gsl_vector_set((*w).work2, iu, 1.0f64);
            }
            gsl_vector_set((*w).work, iu, 0.0f64);
            gsl_vector_set(
                (*w).work2,
                iu.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                0.0f64,
            );
            k_0 = 0 as libc::c_int as size_t;
            while k_0 < iu.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                gsl_vector_set(
                    (*w).work,
                    k_0,
                    -gsl_vector_get(
                        (*w).work,
                        iu.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    )
                        * gsl_matrix_get(
                            T,
                            k_0,
                            iu.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        ),
                );
                gsl_vector_set(
                    (*w).work2,
                    k_0,
                    -gsl_vector_get((*w).work2, iu) * gsl_matrix_get(T, k_0, iu),
                );
                k_0 = k_0.wrapping_add(1);
                k_0;
            }
            l_0 = i - 2 as libc::c_int;
            while l_0 >= 0 as libc::c_int {
                let mut lu_0: size_t = l_0 as size_t;
                if lu_0 == 0 as libc::c_int as libc::c_ulong {
                    complex_pair = 0 as libc::c_int;
                } else {
                    complex_pair = (gsl_matrix_get(
                        T,
                        lu_0,
                        lu_0.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ) != 0.0f64) as libc::c_int;
                }
                if complex_pair == 0 {
                    let mut bval: gsl_complex = gsl_complex { dat: [0.; 2] };
                    let mut x_1: gsl_complex = gsl_complex { dat: [0.; 2] };
                    Tv = gsl_matrix_submatrix(
                        T,
                        lu_0,
                        lu_0,
                        1 as libc::c_int as size_t,
                        1 as libc::c_int as size_t,
                    );
                    bv_0 = gsl_vector_complex_view_array(
                        dat.as_mut_ptr(),
                        1 as libc::c_int as size_t,
                    );
                    xv_0 = gsl_vector_complex_view_array(
                        dat_X.as_mut_ptr(),
                        1 as libc::c_int as size_t,
                    );
                    bval
                        .dat[0 as libc::c_int
                        as usize] = gsl_vector_get((*w).work, lu_0);
                    bval
                        .dat[1 as libc::c_int
                        as usize] = gsl_vector_get((*w).work2, lu_0);
                    gsl_vector_complex_set(
                        &mut bv_0.vector,
                        0 as libc::c_int as size_t,
                        bval,
                    );
                    gsl_schur_solve_equation_z(
                        1.0f64,
                        &mut Tv.matrix,
                        &mut lambda,
                        1.0f64,
                        1.0f64,
                        &mut bv_0.vector,
                        &mut xv_0.vector,
                        &mut scale,
                        &mut xnorm,
                        smin,
                    );
                    if xnorm > 1.0f64 {
                        if gsl_vector_get((*w).work3, lu_0) > bignum / xnorm {
                            gsl_blas_zdscal(1.0f64 / xnorm, &mut xv_0.vector);
                            scale /= xnorm;
                        }
                    }
                    if scale != 1.0f64 {
                        let mut wv_1: gsl_vector_view = gsl_vector_view {
                            vector: gsl_vector {
                                size: 0,
                                stride: 0,
                                data: 0 as *mut libc::c_double,
                                block: 0 as *mut gsl_block,
                                owner: 0,
                            },
                        };
                        wv_1 = gsl_vector_subvector(
                            (*w).work,
                            0 as libc::c_int as size_t,
                            iu.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        );
                        gsl_blas_dscal(scale, &mut wv_1.vector);
                        wv_1 = gsl_vector_subvector(
                            (*w).work2,
                            0 as libc::c_int as size_t,
                            iu.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        );
                        gsl_blas_dscal(scale, &mut wv_1.vector);
                    }
                    x_1 = gsl_vector_complex_get(
                        &mut xv_0.vector,
                        0 as libc::c_int as size_t,
                    );
                    gsl_vector_set((*w).work, lu_0, x_1.dat[0 as libc::c_int as usize]);
                    gsl_vector_set((*w).work2, lu_0, x_1.dat[1 as libc::c_int as usize]);
                    if lu_0 > 0 as libc::c_int as libc::c_ulong {
                        let mut v1_1: gsl_vector_view = gsl_vector_view {
                            vector: gsl_vector {
                                size: 0,
                                stride: 0,
                                data: 0 as *mut libc::c_double,
                                block: 0 as *mut gsl_block,
                                owner: 0,
                            },
                        };
                        let mut v2_1: gsl_vector_view = gsl_vector_view {
                            vector: gsl_vector {
                                size: 0,
                                stride: 0,
                                data: 0 as *mut libc::c_double,
                                block: 0 as *mut gsl_block,
                                owner: 0,
                            },
                        };
                        v1_1 = gsl_matrix_subcolumn(
                            T,
                            lu_0,
                            0 as libc::c_int as size_t,
                            lu_0,
                        );
                        v2_1 = gsl_vector_subvector(
                            (*w).work,
                            0 as libc::c_int as size_t,
                            lu_0,
                        );
                        gsl_blas_daxpy(
                            -x_1.dat[0 as libc::c_int as usize],
                            &mut v1_1.vector,
                            &mut v2_1.vector,
                        );
                        v2_1 = gsl_vector_subvector(
                            (*w).work2,
                            0 as libc::c_int as size_t,
                            lu_0,
                        );
                        gsl_blas_daxpy(
                            -x_1.dat[1 as libc::c_int as usize],
                            &mut v1_1.vector,
                            &mut v2_1.vector,
                        );
                    }
                } else {
                    let mut b1: gsl_complex = gsl_complex { dat: [0.; 2] };
                    let mut b2: gsl_complex = gsl_complex { dat: [0.; 2] };
                    let mut x1: gsl_complex = gsl_complex { dat: [0.; 2] };
                    let mut x2: gsl_complex = gsl_complex { dat: [0.; 2] };
                    Tv = gsl_matrix_submatrix(
                        T,
                        lu_0.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        lu_0.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        2 as libc::c_int as size_t,
                        2 as libc::c_int as size_t,
                    );
                    bv_0 = gsl_vector_complex_view_array(
                        dat.as_mut_ptr(),
                        2 as libc::c_int as size_t,
                    );
                    xv_0 = gsl_vector_complex_view_array(
                        dat_X.as_mut_ptr(),
                        2 as libc::c_int as size_t,
                    );
                    b1
                        .dat[0 as libc::c_int
                        as usize] = gsl_vector_get(
                        (*w).work,
                        lu_0.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    );
                    b1
                        .dat[1 as libc::c_int
                        as usize] = gsl_vector_get(
                        (*w).work2,
                        lu_0.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    );
                    b2.dat[0 as libc::c_int as usize] = gsl_vector_get((*w).work, lu_0);
                    b2.dat[1 as libc::c_int as usize] = gsl_vector_get((*w).work2, lu_0);
                    gsl_vector_complex_set(
                        &mut bv_0.vector,
                        0 as libc::c_int as size_t,
                        b1,
                    );
                    gsl_vector_complex_set(
                        &mut bv_0.vector,
                        1 as libc::c_int as size_t,
                        b2,
                    );
                    gsl_schur_solve_equation_z(
                        1.0f64,
                        &mut Tv.matrix,
                        &mut lambda,
                        1.0f64,
                        1.0f64,
                        &mut bv_0.vector,
                        &mut xv_0.vector,
                        &mut scale,
                        &mut xnorm,
                        smin,
                    );
                    x1 = gsl_vector_complex_get(
                        &mut xv_0.vector,
                        0 as libc::c_int as size_t,
                    );
                    x2 = gsl_vector_complex_get(
                        &mut xv_0.vector,
                        1 as libc::c_int as size_t,
                    );
                    if xnorm > 1.0f64 {
                        let mut beta_0: libc::c_double = 0.;
                        beta_0 = if gsl_vector_get(
                            (*w).work3,
                            lu_0.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        ) > gsl_vector_get((*w).work3, lu_0)
                        {
                            gsl_vector_get(
                                (*w).work3,
                                lu_0.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            )
                        } else {
                            gsl_vector_get((*w).work3, lu_0)
                        };
                        if beta_0 > bignum / xnorm {
                            gsl_blas_zdscal(1.0f64 / xnorm, &mut xv_0.vector);
                            scale /= xnorm;
                        }
                    }
                    if scale != 1.0f64 {
                        let mut wv_2: gsl_vector_view = gsl_vector_view {
                            vector: gsl_vector {
                                size: 0,
                                stride: 0,
                                data: 0 as *mut libc::c_double,
                                block: 0 as *mut gsl_block,
                                owner: 0,
                            },
                        };
                        wv_2 = gsl_vector_subvector(
                            (*w).work,
                            0 as libc::c_int as size_t,
                            iu.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        );
                        gsl_blas_dscal(scale, &mut wv_2.vector);
                        wv_2 = gsl_vector_subvector(
                            (*w).work2,
                            0 as libc::c_int as size_t,
                            iu.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        );
                        gsl_blas_dscal(scale, &mut wv_2.vector);
                    }
                    gsl_vector_set(
                        (*w).work,
                        lu_0.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        x1.dat[0 as libc::c_int as usize],
                    );
                    gsl_vector_set((*w).work, lu_0, x2.dat[0 as libc::c_int as usize]);
                    gsl_vector_set(
                        (*w).work2,
                        lu_0.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        x1.dat[1 as libc::c_int as usize],
                    );
                    gsl_vector_set((*w).work2, lu_0, x2.dat[1 as libc::c_int as usize]);
                    if lu_0 > 1 as libc::c_int as libc::c_ulong {
                        let mut v1_2: gsl_vector_view = gsl_vector_view {
                            vector: gsl_vector {
                                size: 0,
                                stride: 0,
                                data: 0 as *mut libc::c_double,
                                block: 0 as *mut gsl_block,
                                owner: 0,
                            },
                        };
                        let mut v2_2: gsl_vector_view = gsl_vector_view {
                            vector: gsl_vector {
                                size: 0,
                                stride: 0,
                                data: 0 as *mut libc::c_double,
                                block: 0 as *mut gsl_block,
                                owner: 0,
                            },
                        };
                        let mut v3: gsl_vector_view = gsl_vector_view {
                            vector: gsl_vector {
                                size: 0,
                                stride: 0,
                                data: 0 as *mut libc::c_double,
                                block: 0 as *mut gsl_block,
                                owner: 0,
                            },
                        };
                        let mut v4: gsl_vector_view = gsl_vector_view {
                            vector: gsl_vector {
                                size: 0,
                                stride: 0,
                                data: 0 as *mut libc::c_double,
                                block: 0 as *mut gsl_block,
                                owner: 0,
                            },
                        };
                        v1_2 = gsl_matrix_subcolumn(
                            T,
                            lu_0.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            0 as libc::c_int as size_t,
                            lu_0.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        );
                        v4 = gsl_matrix_subcolumn(
                            T,
                            lu_0,
                            0 as libc::c_int as size_t,
                            lu_0.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        );
                        v2_2 = gsl_vector_subvector(
                            (*w).work,
                            0 as libc::c_int as size_t,
                            lu_0.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        );
                        v3 = gsl_vector_subvector(
                            (*w).work2,
                            0 as libc::c_int as size_t,
                            lu_0.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        );
                        gsl_blas_daxpy(
                            -x1.dat[0 as libc::c_int as usize],
                            &mut v1_2.vector,
                            &mut v2_2.vector,
                        );
                        gsl_blas_daxpy(
                            -x2.dat[0 as libc::c_int as usize],
                            &mut v4.vector,
                            &mut v2_2.vector,
                        );
                        gsl_blas_daxpy(
                            -x1.dat[1 as libc::c_int as usize],
                            &mut v1_2.vector,
                            &mut v3.vector,
                        );
                        gsl_blas_daxpy(
                            -x2.dat[1 as libc::c_int as usize],
                            &mut v4.vector,
                            &mut v3.vector,
                        );
                    }
                    l_0 -= 1;
                    l_0;
                }
                l_0 -= 1;
                l_0;
            }
            y = gsl_matrix_column(Z, iu.wrapping_sub(1 as libc::c_int as libc::c_ulong));
            y2 = gsl_matrix_column(Z, iu);
            if iu > 1 as libc::c_int as libc::c_ulong {
                let mut x_2: gsl_vector_view = gsl_vector_view {
                    vector: gsl_vector {
                        size: 0,
                        stride: 0,
                        data: 0 as *mut libc::c_double,
                        block: 0 as *mut gsl_block,
                        owner: 0,
                    },
                };
                Zv = gsl_matrix_submatrix(
                    Z,
                    0 as libc::c_int as size_t,
                    0 as libc::c_int as size_t,
                    N,
                    iu.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                x_2 = gsl_vector_subvector(
                    (*w).work,
                    0 as libc::c_int as size_t,
                    iu.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                gsl_blas_dgemv(
                    CblasNoTrans,
                    1.0f64,
                    &mut Zv.matrix,
                    &mut x_2.vector,
                    gsl_vector_get(
                        (*w).work,
                        iu.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ),
                    &mut y.vector,
                );
                x_2 = gsl_vector_subvector(
                    (*w).work2,
                    0 as libc::c_int as size_t,
                    iu.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                gsl_blas_dgemv(
                    CblasNoTrans,
                    1.0f64,
                    &mut Zv.matrix,
                    &mut x_2.vector,
                    gsl_vector_get((*w).work2, iu),
                    &mut y2.vector,
                );
            } else {
                gsl_blas_dscal(
                    gsl_vector_get(
                        (*w).work,
                        iu.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ),
                    &mut y.vector,
                );
                gsl_blas_dscal(gsl_vector_get((*w).work2, iu), &mut y2.vector);
            }
            ecol = gsl_matrix_complex_column(
                evec,
                iu.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            ecol2 = gsl_matrix_complex_column(evec, iu);
            ev = gsl_vector_complex_imag(&mut ecol.vector);
            ev2 = gsl_vector_complex_imag(&mut ecol2.vector);
            scale = 0.0f64;
            ii = 0 as libc::c_int as size_t;
            while ii < N {
                let mut a_0: libc::c_double = gsl_vector_get(&mut y2.vector, ii);
                scale = if scale > fabs(a_0) + fabs(gsl_vector_get(&mut y.vector, ii)) {
                    scale
                } else {
                    fabs(a_0) + fabs(gsl_vector_get(&mut y.vector, ii))
                };
                gsl_vector_set(&mut ev.vector, ii, a_0);
                gsl_vector_set(&mut ev2.vector, ii, -a_0);
                ii = ii.wrapping_add(1);
                ii;
            }
            ev = gsl_vector_complex_real(&mut ecol.vector);
            ev2 = gsl_vector_complex_real(&mut ecol2.vector);
            ii = 0 as libc::c_int as size_t;
            while ii < N {
                let mut a_1: libc::c_double = gsl_vector_get(&mut y.vector, ii);
                gsl_vector_set(&mut ev.vector, ii, a_1);
                gsl_vector_set(&mut ev2.vector, ii, a_1);
                ii = ii.wrapping_add(1);
                ii;
            }
            if scale != 0.0f64 {
                scale = 1.0f64 / scale;
            }
            gsl_blas_zdscal(scale, &mut ecol.vector);
            gsl_blas_zdscal(scale, &mut ecol2.vector);
            i -= 1;
            i;
        }
        i -= 1;
        i;
    }
}
unsafe extern "C" fn nonsymmv_normalize_eigenvectors(
    mut eval: *mut gsl_vector_complex,
    mut evec: *mut gsl_matrix_complex,
) {
    let N: size_t = (*evec).size1;
    let mut i: size_t = 0;
    let mut ei: gsl_complex = gsl_complex { dat: [0.; 2] };
    let mut vi: gsl_vector_complex_view = gsl_vector_complex_view {
        vector: gsl_vector_complex {
            size: 0,
            stride: 0,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block_complex,
            owner: 0,
        },
    };
    let mut re: gsl_vector_view = gsl_vector_view {
        vector: gsl_vector {
            size: 0,
            stride: 0,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0,
        },
    };
    let mut im: gsl_vector_view = gsl_vector_view {
        vector: gsl_vector {
            size: 0,
            stride: 0,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0,
        },
    };
    let mut scale: libc::c_double = 0.;
    i = 0 as libc::c_int as size_t;
    while i < N {
        ei = gsl_vector_complex_get(eval, i);
        vi = gsl_matrix_complex_column(evec, i);
        re = gsl_vector_complex_real(&mut vi.vector);
        if ei.dat[1 as libc::c_int as usize] == 0.0f64 {
            scale = 1.0f64 / gsl_blas_dnrm2(&mut re.vector);
            gsl_blas_dscal(scale, &mut re.vector);
        } else if ei.dat[1 as libc::c_int as usize] > 0.0f64 {
            im = gsl_vector_complex_imag(&mut vi.vector);
            scale = 1.0f64
                / gsl_hypot(
                    gsl_blas_dnrm2(&mut re.vector),
                    gsl_blas_dnrm2(&mut im.vector),
                );
            gsl_blas_zdscal(scale, &mut vi.vector);
            vi = gsl_matrix_complex_column(
                evec,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            gsl_blas_zdscal(scale, &mut vi.vector);
        }
        i = i.wrapping_add(1);
        i;
    }
}
