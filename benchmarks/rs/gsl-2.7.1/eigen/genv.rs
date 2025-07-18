use ::libc;
extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_alloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_vector_view_array_with_stride(
        base: *mut libc::c_double,
        stride: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
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
    fn gsl_schur_gen_eigvals(
        A: *const gsl_matrix,
        B: *const gsl_matrix,
        wr1: *mut libc::c_double,
        wr2: *mut libc::c_double,
        wi: *mut libc::c_double,
        scale1: *mut libc::c_double,
        scale2: *mut libc::c_double,
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
    fn gsl_eigen_gen_QZ(
        A: *mut gsl_matrix,
        B: *mut gsl_matrix,
        alpha: *mut gsl_vector_complex,
        beta: *mut gsl_vector,
        Q: *mut gsl_matrix,
        Z: *mut gsl_matrix,
        w: *mut gsl_eigen_gen_workspace,
    ) -> libc::c_int;
    fn gsl_eigen_gen_params(
        compute_s: libc::c_int,
        compute_t: libc::c_int,
        balance: libc::c_int,
        w: *mut gsl_eigen_gen_workspace,
    );
    fn gsl_eigen_gen_free(w: *mut gsl_eigen_gen_workspace);
    fn gsl_eigen_gen_alloc(n: size_t) -> *mut gsl_eigen_gen_workspace;
    fn gsl_matrix_const_submatrix(
        m: *const gsl_matrix,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_const_view;
    fn gsl_matrix_memcpy(dest: *mut gsl_matrix, src: *const gsl_matrix) -> libc::c_int;
    fn gsl_blas_dnrm2(X: *const gsl_vector) -> libc::c_double;
    fn gsl_blas_dscal(alpha: libc::c_double, X: *mut gsl_vector);
    fn gsl_blas_zdscal(alpha: libc::c_double, X: *mut gsl_vector_complex);
    fn gsl_hypot(x: libc::c_double, y: libc::c_double) -> libc::c_double;
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
pub struct _gsl_matrix_const_view {
    pub matrix: gsl_matrix,
}
pub type gsl_matrix_const_view = _gsl_matrix_const_view;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_eigen_gen_workspace {
    pub size: size_t,
    pub work: *mut gsl_vector,
    pub n_evals: size_t,
    pub max_iterations: size_t,
    pub n_iter: size_t,
    pub eshift: libc::c_double,
    pub needtop: libc::c_int,
    pub atol: libc::c_double,
    pub btol: libc::c_double,
    pub ascale: libc::c_double,
    pub bscale: libc::c_double,
    pub H: *mut gsl_matrix,
    pub R: *mut gsl_matrix,
    pub compute_s: libc::c_int,
    pub compute_t: libc::c_int,
    pub Q: *mut gsl_matrix,
    pub Z: *mut gsl_matrix,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_eigen_genv_workspace {
    pub size: size_t,
    pub work1: *mut gsl_vector,
    pub work2: *mut gsl_vector,
    pub work3: *mut gsl_vector,
    pub work4: *mut gsl_vector,
    pub work5: *mut gsl_vector,
    pub work6: *mut gsl_vector,
    pub Q: *mut gsl_matrix,
    pub Z: *mut gsl_matrix,
    pub gen_workspace_p: *mut gsl_eigen_gen_workspace,
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
#[inline]
unsafe extern "C" fn gsl_matrix_get(
    mut m: *const gsl_matrix,
    i: size_t,
    j: size_t,
) -> libc::c_double {
    return *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
pub unsafe extern "C" fn gsl_eigen_genv_alloc(
    n: size_t,
) -> *mut gsl_eigen_genv_workspace {
    let mut w: *mut gsl_eigen_genv_workspace = 0 as *mut gsl_eigen_genv_workspace;
    if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix dimension must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"genv.c\0" as *const u8 as *const libc::c_char,
            68 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_eigen_genv_workspace;
    }
    w = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gsl_eigen_genv_workspace>() as libc::c_ulong,
    ) as *mut gsl_eigen_genv_workspace;
    if w.is_null() {
        gsl_error(
            b"failed to allocate space for workspace\0" as *const u8
                as *const libc::c_char,
            b"genv.c\0" as *const u8 as *const libc::c_char,
            75 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_eigen_genv_workspace;
    }
    (*w).size = n;
    (*w).Q = 0 as *mut gsl_matrix;
    (*w).Z = 0 as *mut gsl_matrix;
    (*w).gen_workspace_p = gsl_eigen_gen_alloc(n);
    if ((*w).gen_workspace_p).is_null() {
        gsl_eigen_genv_free(w);
        gsl_error(
            b"failed to allocate space for gen workspace\0" as *const u8
                as *const libc::c_char,
            b"genv.c\0" as *const u8 as *const libc::c_char,
            87 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_eigen_genv_workspace;
    }
    gsl_eigen_gen_params(
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        (*w).gen_workspace_p,
    );
    (*w).work1 = gsl_vector_alloc(n);
    (*w).work2 = gsl_vector_alloc(n);
    (*w).work3 = gsl_vector_alloc(n);
    (*w).work4 = gsl_vector_alloc(n);
    (*w).work5 = gsl_vector_alloc(n);
    (*w).work6 = gsl_vector_alloc(n);
    if ((*w).work1).is_null() || ((*w).work2).is_null() || ((*w).work3).is_null()
        || ((*w).work4).is_null() || ((*w).work5).is_null() || ((*w).work6).is_null()
    {
        gsl_eigen_genv_free(w);
        gsl_error(
            b"failed to allocate space for additional workspace\0" as *const u8
                as *const libc::c_char,
            b"genv.c\0" as *const u8 as *const libc::c_char,
            104 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_eigen_genv_workspace;
    }
    return w;
}
pub unsafe extern "C" fn gsl_eigen_genv_free(mut w: *mut gsl_eigen_genv_workspace) {
    if w.is_null() {
        return;
    }
    if !((*w).gen_workspace_p).is_null() {
        gsl_eigen_gen_free((*w).gen_workspace_p);
    }
    if !((*w).work1).is_null() {
        gsl_vector_free((*w).work1);
    }
    if !((*w).work2).is_null() {
        gsl_vector_free((*w).work2);
    }
    if !((*w).work3).is_null() {
        gsl_vector_free((*w).work3);
    }
    if !((*w).work4).is_null() {
        gsl_vector_free((*w).work4);
    }
    if !((*w).work5).is_null() {
        gsl_vector_free((*w).work5);
    }
    if !((*w).work6).is_null() {
        gsl_vector_free((*w).work6);
    }
    free(w as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_eigen_genv(
    mut A: *mut gsl_matrix,
    mut B: *mut gsl_matrix,
    mut alpha: *mut gsl_vector_complex,
    mut beta: *mut gsl_vector,
    mut evec: *mut gsl_matrix_complex,
    mut w: *mut gsl_eigen_genv_workspace,
) -> libc::c_int {
    let N: size_t = (*A).size1;
    if N != (*A).size2 {
        gsl_error(
            b"matrix must be square to compute eigenvalues\0" as *const u8
                as *const libc::c_char,
            b"genv.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != (*B).size1 || N != (*B).size2 {
        gsl_error(
            b"B matrix dimensions must match A\0" as *const u8 as *const libc::c_char,
            b"genv.c\0" as *const u8 as *const libc::c_char,
            178 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*alpha).size != N || (*beta).size != N {
        gsl_error(
            b"eigenvalue vector must match matrix size\0" as *const u8
                as *const libc::c_char,
            b"genv.c\0" as *const u8 as *const libc::c_char,
            182 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*w).size != N {
        gsl_error(
            b"matrix size does not match workspace\0" as *const u8
                as *const libc::c_char,
            b"genv.c\0" as *const u8 as *const libc::c_char,
            186 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*evec).size1 != N {
        gsl_error(
            b"eigenvector matrix has wrong size\0" as *const u8 as *const libc::c_char,
            b"genv.c\0" as *const u8 as *const libc::c_char,
            190 as libc::c_int,
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
        s = gsl_eigen_gen_QZ(A, B, alpha, beta, (*w).Q, &mut Z, (*w).gen_workspace_p);
        if !((*w).Z).is_null() {
            gsl_matrix_memcpy((*w).Z, &mut Z);
        }
        if s == GSL_SUCCESS as libc::c_int {
            s = genv_get_right_eigenvectors(A, B, &mut Z, evec, w);
            if s == GSL_SUCCESS as libc::c_int {
                genv_normalize_eigenvectors(alpha, evec);
            }
        }
        return s;
    };
}
pub unsafe extern "C" fn gsl_eigen_genv_QZ(
    mut A: *mut gsl_matrix,
    mut B: *mut gsl_matrix,
    mut alpha: *mut gsl_vector_complex,
    mut beta: *mut gsl_vector,
    mut evec: *mut gsl_matrix_complex,
    mut Q: *mut gsl_matrix,
    mut Z: *mut gsl_matrix,
    mut w: *mut gsl_eigen_genv_workspace,
) -> libc::c_int {
    if !Q.is_null() && ((*A).size1 != (*Q).size1 || (*A).size1 != (*Q).size2) {
        gsl_error(
            b"Q matrix has wrong dimensions\0" as *const u8 as *const libc::c_char,
            b"genv.c\0" as *const u8 as *const libc::c_char,
            268 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if !Z.is_null() && ((*A).size1 != (*Z).size1 || (*A).size1 != (*Z).size2) {
        gsl_error(
            b"Z matrix has wrong dimensions\0" as *const u8 as *const libc::c_char,
            b"genv.c\0" as *const u8 as *const libc::c_char,
            272 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut s: libc::c_int = 0;
        (*w).Q = Q;
        (*w).Z = Z;
        s = gsl_eigen_genv(A, B, alpha, beta, evec, w);
        (*w).Q = 0 as *mut gsl_matrix;
        (*w).Z = 0 as *mut gsl_matrix;
        return s;
    };
}
unsafe extern "C" fn genv_get_right_eigenvectors(
    mut S: *const gsl_matrix,
    mut T: *const gsl_matrix,
    mut Z: *mut gsl_matrix,
    mut evec: *mut gsl_matrix_complex,
    mut w: *mut gsl_eigen_genv_workspace,
) -> libc::c_int {
    let N: size_t = (*w).size;
    let small: libc::c_double = 2.2250738585072014e-308f64 * N as libc::c_double
        / 2.2204460492503131e-16f64;
    let big: libc::c_double = 1.0f64 / small;
    let bignum: libc::c_double = 1.0f64
        / (2.2250738585072014e-308f64 * N as libc::c_double);
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut end: size_t = 0;
    let mut is: libc::c_int = 0;
    let mut anorm: libc::c_double = 0.;
    let mut bnorm: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    let mut temp2: libc::c_double = 0.;
    let mut temp2r: libc::c_double = 0.;
    let mut temp2i: libc::c_double = 0.;
    let mut ascale: libc::c_double = 0.;
    let mut bscale: libc::c_double = 0.;
    let mut salfar: libc::c_double = 0.;
    let mut sbeta: libc::c_double = 0.;
    let mut acoef: libc::c_double = 0.;
    let mut bcoefr: libc::c_double = 0.;
    let mut bcoefi: libc::c_double = 0.;
    let mut acoefa: libc::c_double = 0.;
    let mut bcoefa: libc::c_double = 0.;
    let mut creala: libc::c_double = 0.;
    let mut cimaga: libc::c_double = 0.;
    let mut crealb: libc::c_double = 0.;
    let mut cimagb: libc::c_double = 0.;
    let mut cre2a: libc::c_double = 0.;
    let mut cim2a: libc::c_double = 0.;
    let mut cre2b: libc::c_double = 0.;
    let mut cim2b: libc::c_double = 0.;
    let mut dmin: libc::c_double = 0.;
    let mut xmax: libc::c_double = 0.;
    let mut scale: libc::c_double = 0.;
    let mut nw: size_t = 0;
    let mut na: size_t = 0;
    let mut lsa: libc::c_int = 0;
    let mut lsb: libc::c_int = 0;
    let mut complex_pair: libc::c_int = 0;
    let mut z_zero: gsl_complex = gsl_complex { dat: [0.; 2] };
    let mut z_one: gsl_complex = gsl_complex { dat: [0.; 2] };
    let mut bdiag: [libc::c_double; 2] = [0.0f64, 0.0f64];
    let mut sum: [libc::c_double; 4] = [0.; 4];
    let mut il2by2: libc::c_int = 0;
    let mut jr: size_t = 0;
    let mut jc: size_t = 0;
    let mut ja: size_t = 0;
    let mut xscale: libc::c_double = 0.;
    let mut ecol: gsl_vector_complex_view = gsl_vector_complex_view {
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
    let mut re2: gsl_vector_view = gsl_vector_view {
        vector: gsl_vector {
            size: 0,
            stride: 0,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0,
        },
    };
    let mut im2: gsl_vector_view = gsl_vector_view {
        vector: gsl_vector {
            size: 0,
            stride: 0,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0,
        },
    };
    z_zero.dat[0 as libc::c_int as usize] = 0.0f64;
    z_zero.dat[1 as libc::c_int as usize] = 0.0f64;
    z_one.dat[0 as libc::c_int as usize] = 1.0f64;
    z_one.dat[1 as libc::c_int as usize] = 0.0f64;
    anorm = fabs(
        gsl_matrix_get(S, 0 as libc::c_int as size_t, 0 as libc::c_int as size_t),
    );
    if N > 1 as libc::c_int as libc::c_ulong {
        anorm
            += fabs(
                gsl_matrix_get(S, 1 as libc::c_int as size_t, 0 as libc::c_int as size_t),
            );
    }
    bnorm = fabs(
        gsl_matrix_get(T, 0 as libc::c_int as size_t, 0 as libc::c_int as size_t),
    );
    gsl_vector_set((*w).work1, 0 as libc::c_int as size_t, 0.0f64);
    gsl_vector_set((*w).work2, 0 as libc::c_int as size_t, 0.0f64);
    j = 1 as libc::c_int as size_t;
    while j < N {
        temp2 = 0.0f64;
        temp = temp2;
        if gsl_matrix_get(S, j, j.wrapping_sub(1 as libc::c_int as libc::c_ulong))
            == 0.0f64
        {
            end = j;
        } else {
            end = j.wrapping_sub(1 as libc::c_int as libc::c_ulong);
        }
        i = 0 as libc::c_int as size_t;
        while i < end {
            temp += fabs(gsl_matrix_get(S, i, j));
            temp2 += fabs(gsl_matrix_get(T, i, j));
            i = i.wrapping_add(1);
            i;
        }
        gsl_vector_set((*w).work1, j, temp);
        gsl_vector_set((*w).work2, j, temp2);
        i = end;
        while i
            < (if j.wrapping_add(2 as libc::c_int as libc::c_ulong) < N {
                j.wrapping_add(2 as libc::c_int as libc::c_ulong)
            } else {
                N
            })
        {
            temp += fabs(gsl_matrix_get(S, i, j));
            temp2 += fabs(gsl_matrix_get(T, i, j));
            i = i.wrapping_add(1);
            i;
        }
        anorm = if anorm > temp { anorm } else { temp };
        bnorm = if bnorm > temp2 { bnorm } else { temp2 };
        j = j.wrapping_add(1);
        j;
    }
    ascale = 1.0f64
        / (if anorm > 2.2250738585072014e-308f64 {
            anorm
        } else {
            2.2250738585072014e-308f64
        });
    bscale = 1.0f64
        / (if bnorm > 2.2250738585072014e-308f64 {
            bnorm
        } else {
            2.2250738585072014e-308f64
        });
    complex_pair = 0 as libc::c_int;
    let mut current_block_295: u64;
    k = 0 as libc::c_int as size_t;
    while k < N {
        let mut je: size_t = N
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_sub(k);
        if complex_pair != 0 {
            complex_pair = 0 as libc::c_int;
        } else {
            nw = 1 as libc::c_int as size_t;
            if je > 0 as libc::c_int as libc::c_ulong {
                if gsl_matrix_get(
                    S,
                    je,
                    je.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) != 0.0f64
                {
                    complex_pair = 1 as libc::c_int;
                    nw = 2 as libc::c_int as size_t;
                }
            }
            if complex_pair == 0 {
                if fabs(gsl_matrix_get(S, je, je)) <= 2.2250738585072014e-308f64
                    && fabs(gsl_matrix_get(T, je, je)) <= 2.2250738585072014e-308f64
                {
                    i = 0 as libc::c_int as size_t;
                    while i < N {
                        gsl_matrix_complex_set(evec, i, je, z_zero);
                        i = i.wrapping_add(1);
                        i;
                    }
                    gsl_matrix_complex_set(evec, je, je, z_one);
                    current_block_295 = 9007357115414505193;
                } else {
                    i = 0 as libc::c_int as size_t;
                    while i < N {
                        gsl_vector_set((*w).work3, i, 0.0f64);
                        i = i.wrapping_add(1);
                        i;
                    }
                    current_block_295 = 4888910987971495881;
                }
            } else {
                i = 0 as libc::c_int as size_t;
                while i < N {
                    gsl_vector_set((*w).work3, i, 0.0f64);
                    gsl_vector_set((*w).work4, i, 0.0f64);
                    i = i.wrapping_add(1);
                    i;
                }
                current_block_295 = 4888910987971495881;
            }
            match current_block_295 {
                9007357115414505193 => {}
                _ => {
                    if complex_pair == 0 {
                        temp = 1.0f64
                            / (if 2.2250738585072014e-308f64
                                > (if fabs(gsl_matrix_get(S, je, je)) * ascale
                                    > fabs(gsl_matrix_get(T, je, je)) * bscale
                                {
                                    fabs(gsl_matrix_get(S, je, je)) * ascale
                                } else {
                                    fabs(gsl_matrix_get(T, je, je)) * bscale
                                })
                            {
                                2.2250738585072014e-308f64
                            } else {
                                (if fabs(gsl_matrix_get(S, je, je)) * ascale
                                    > fabs(gsl_matrix_get(T, je, je)) * bscale
                                {
                                    fabs(gsl_matrix_get(S, je, je)) * ascale
                                } else {
                                    fabs(gsl_matrix_get(T, je, je)) * bscale
                                })
                            });
                        salfar = temp * gsl_matrix_get(S, je, je) * ascale;
                        sbeta = temp * gsl_matrix_get(T, je, je) * bscale;
                        acoef = sbeta * ascale;
                        bcoefr = salfar * bscale;
                        bcoefi = 0.0f64;
                        scale = 1.0f64;
                        lsa = (fabs(sbeta) >= 2.2250738585072014e-308f64
                            && fabs(acoef) < small) as libc::c_int;
                        lsb = (fabs(salfar) >= 2.2250738585072014e-308f64
                            && fabs(bcoefr) < small) as libc::c_int;
                        if lsa != 0 {
                            scale = small / fabs(sbeta)
                                * (if anorm < big { anorm } else { big });
                        }
                        if lsb != 0 {
                            scale = if scale
                                > small / fabs(salfar)
                                    * (if bnorm < big { bnorm } else { big })
                            {
                                scale
                            } else {
                                small / fabs(salfar)
                                    * (if bnorm < big { bnorm } else { big })
                            };
                        }
                        if lsa != 0 || lsb != 0 {
                            scale = if scale
                                < 1.0f64
                                    / (2.2250738585072014e-308f64
                                        * (if 1.0f64
                                            > (if fabs(acoef) > fabs(bcoefr) {
                                                fabs(acoef)
                                            } else {
                                                fabs(bcoefr)
                                            })
                                        {
                                            1.0f64
                                        } else {
                                            (if fabs(acoef) > fabs(bcoefr) {
                                                fabs(acoef)
                                            } else {
                                                fabs(bcoefr)
                                            })
                                        }))
                            {
                                scale
                            } else {
                                1.0f64
                                    / (2.2250738585072014e-308f64
                                        * (if 1.0f64
                                            > (if fabs(acoef) > fabs(bcoefr) {
                                                fabs(acoef)
                                            } else {
                                                fabs(bcoefr)
                                            })
                                        {
                                            1.0f64
                                        } else {
                                            (if fabs(acoef) > fabs(bcoefr) {
                                                fabs(acoef)
                                            } else {
                                                fabs(bcoefr)
                                            })
                                        }))
                            };
                            if lsa != 0 {
                                acoef = ascale * (scale * sbeta);
                            } else {
                                acoef *= scale;
                            }
                            if lsb != 0 {
                                bcoefr = bscale * (scale * salfar);
                            } else {
                                bcoefr *= scale;
                            }
                        }
                        acoefa = fabs(acoef);
                        bcoefa = fabs(bcoefr);
                        gsl_vector_set((*w).work3, je, 1.0f64);
                        xmax = 1.0f64;
                        i = 0 as libc::c_int as size_t;
                        while i < je {
                            gsl_vector_set(
                                (*w).work3,
                                i,
                                bcoefr * gsl_matrix_get(T, i, je)
                                    - acoef * gsl_matrix_get(S, i, je),
                            );
                            i = i.wrapping_add(1);
                            i;
                        }
                    } else {
                        let vs: gsl_matrix_const_view = gsl_matrix_const_submatrix(
                            S,
                            je.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            je.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            2 as libc::c_int as size_t,
                            2 as libc::c_int as size_t,
                        );
                        let vt: gsl_matrix_const_view = gsl_matrix_const_submatrix(
                            T,
                            je.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            je.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            2 as libc::c_int as size_t,
                            2 as libc::c_int as size_t,
                        );
                        gsl_schur_gen_eigvals(
                            &vs.matrix,
                            &vt.matrix,
                            &mut bcoefr,
                            &mut temp2,
                            &mut bcoefi,
                            &mut acoef,
                            &mut temp,
                        );
                        if bcoefi == 0.0f64 {
                            gsl_error(
                                b"gsl_schur_gen_eigvals failed on complex block\0"
                                    as *const u8 as *const libc::c_char,
                                b"genv.c\0" as *const u8 as *const libc::c_char,
                                514 as libc::c_int,
                                GSL_FAILURE as libc::c_int,
                            );
                            return GSL_FAILURE as libc::c_int;
                        }
                        acoefa = fabs(acoef);
                        bcoefa = fabs(bcoefr) + fabs(bcoefi);
                        scale = 1.0f64;
                        if acoefa * 2.2204460492503131e-16f64
                            < 2.2250738585072014e-308f64
                            && acoefa >= 2.2250738585072014e-308f64
                        {
                            scale = 2.2250738585072014e-308f64
                                / 2.2204460492503131e-16f64 / acoefa;
                        }
                        if bcoefa * 2.2204460492503131e-16f64
                            < 2.2250738585072014e-308f64
                            && bcoefa >= 2.2250738585072014e-308f64
                        {
                            scale = if scale
                                > 2.2250738585072014e-308f64 / 2.2204460492503131e-16f64
                                    / bcoefa
                            {
                                scale
                            } else {
                                2.2250738585072014e-308f64 / 2.2204460492503131e-16f64
                                    / bcoefa
                            };
                        }
                        if 2.2250738585072014e-308f64 * acoefa > ascale {
                            scale = ascale / (2.2250738585072014e-308f64 * acoefa);
                        }
                        if 2.2250738585072014e-308f64 * bcoefa > bscale {
                            scale = if scale
                                < bscale / (2.2250738585072014e-308f64 * bcoefa)
                            {
                                scale
                            } else {
                                bscale / (2.2250738585072014e-308f64 * bcoefa)
                            };
                        }
                        if scale != 1.0f64 {
                            acoef *= scale;
                            acoefa = fabs(acoef);
                            bcoefr *= scale;
                            bcoefi *= scale;
                            bcoefa = fabs(bcoefr) + fabs(bcoefi);
                        }
                        temp = acoef
                            * gsl_matrix_get(
                                S,
                                je,
                                je.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            );
                        temp2r = acoef * gsl_matrix_get(S, je, je)
                            - bcoefr * gsl_matrix_get(T, je, je);
                        temp2i = -bcoefi * gsl_matrix_get(T, je, je);
                        if fabs(temp) >= fabs(temp2r) + fabs(temp2i) {
                            gsl_vector_set((*w).work3, je, 1.0f64);
                            gsl_vector_set((*w).work4, je, 0.0f64);
                            gsl_vector_set(
                                (*w).work3,
                                je.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                -temp2r / temp,
                            );
                            gsl_vector_set(
                                (*w).work4,
                                je.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                -temp2i / temp,
                            );
                        } else {
                            gsl_vector_set(
                                (*w).work3,
                                je.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                1.0f64,
                            );
                            gsl_vector_set(
                                (*w).work4,
                                je.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                0.0f64,
                            );
                            temp = acoef
                                * gsl_matrix_get(
                                    S,
                                    je.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    je,
                                );
                            gsl_vector_set(
                                (*w).work3,
                                je,
                                (bcoefr
                                    * gsl_matrix_get(
                                        T,
                                        je.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                        je.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    )
                                    - acoef
                                        * gsl_matrix_get(
                                            S,
                                            je.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                            je.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                        )) / temp,
                            );
                            gsl_vector_set(
                                (*w).work4,
                                je,
                                bcoefi
                                    * gsl_matrix_get(
                                        T,
                                        je.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                        je.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    ) / temp,
                            );
                        }
                        xmax = if fabs(gsl_vector_get((*w).work3, je))
                            + fabs(gsl_vector_get((*w).work4, je))
                            > fabs(
                                gsl_vector_get(
                                    (*w).work3,
                                    je.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                ),
                            )
                                + fabs(
                                    gsl_vector_get(
                                        (*w).work4,
                                        je.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    ),
                                )
                        {
                            fabs(gsl_vector_get((*w).work3, je))
                                + fabs(gsl_vector_get((*w).work4, je))
                        } else {
                            fabs(
                                gsl_vector_get(
                                    (*w).work3,
                                    je.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                ),
                            )
                                + fabs(
                                    gsl_vector_get(
                                        (*w).work4,
                                        je.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    ),
                                )
                        };
                        creala = acoef
                            * gsl_vector_get(
                                (*w).work3,
                                je.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            );
                        cimaga = acoef
                            * gsl_vector_get(
                                (*w).work4,
                                je.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            );
                        crealb = bcoefr
                            * gsl_vector_get(
                                (*w).work3,
                                je.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            )
                            - bcoefi
                                * gsl_vector_get(
                                    (*w).work4,
                                    je.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                );
                        cimagb = bcoefi
                            * gsl_vector_get(
                                (*w).work3,
                                je.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            )
                            + bcoefr
                                * gsl_vector_get(
                                    (*w).work4,
                                    je.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                );
                        cre2a = acoef * gsl_vector_get((*w).work3, je);
                        cim2a = acoef * gsl_vector_get((*w).work4, je);
                        cre2b = bcoefr * gsl_vector_get((*w).work3, je)
                            - bcoefi * gsl_vector_get((*w).work4, je);
                        cim2b = bcoefi * gsl_vector_get((*w).work3, je)
                            + bcoefr * gsl_vector_get((*w).work4, je);
                        i = 0 as libc::c_int as size_t;
                        while i < je.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                            gsl_vector_set(
                                (*w).work3,
                                i,
                                -creala
                                    * gsl_matrix_get(
                                        S,
                                        i,
                                        je.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    )
                                    + crealb
                                        * gsl_matrix_get(
                                            T,
                                            i,
                                            je.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                        ) - cre2a * gsl_matrix_get(S, i, je)
                                    + cre2b * gsl_matrix_get(T, i, je),
                            );
                            gsl_vector_set(
                                (*w).work4,
                                i,
                                -cimaga
                                    * gsl_matrix_get(
                                        S,
                                        i,
                                        je.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    )
                                    + cimagb
                                        * gsl_matrix_get(
                                            T,
                                            i,
                                            je.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                        ) - cim2a * gsl_matrix_get(S, i, je)
                                    + cim2b * gsl_matrix_get(T, i, je),
                            );
                            i = i.wrapping_add(1);
                            i;
                        }
                    }
                    dmin = if 2.2250738585072014e-308f64
                        > (if 2.2204460492503131e-16f64 * acoefa * anorm
                            > 2.2204460492503131e-16f64 * bcoefa * bnorm
                        {
                            2.2204460492503131e-16f64 * acoefa * anorm
                        } else {
                            2.2204460492503131e-16f64 * bcoefa * bnorm
                        })
                    {
                        2.2250738585072014e-308f64
                    } else if 2.2204460492503131e-16f64 * acoefa * anorm
                        > 2.2204460492503131e-16f64 * bcoefa * bnorm
                    {
                        2.2204460492503131e-16f64 * acoefa * anorm
                    } else {
                        2.2204460492503131e-16f64 * bcoefa * bnorm
                    };
                    il2by2 = 0 as libc::c_int;
                    let mut current_block_230: u64;
                    is = je as libc::c_int - nw as libc::c_int;
                    while is >= 0 as libc::c_int {
                        j = is as size_t;
                        if il2by2 == 0 && j > 0 as libc::c_int as libc::c_ulong {
                            if gsl_matrix_get(
                                S,
                                j,
                                j.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            ) != 0.0f64
                            {
                                il2by2 = 1 as libc::c_int;
                                current_block_230 = 7244994750255146185;
                            } else {
                                current_block_230 = 10945915984064580713;
                            }
                        } else {
                            current_block_230 = 10945915984064580713;
                        }
                        match current_block_230 {
                            10945915984064580713 => {
                                bdiag[0 as libc::c_int as usize] = gsl_matrix_get(T, j, j);
                                if il2by2 != 0 {
                                    na = 2 as libc::c_int as size_t;
                                    bdiag[1 as libc::c_int
                                        as usize] = gsl_matrix_get(
                                        T,
                                        j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                        j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                    );
                                } else {
                                    na = 1 as libc::c_int as size_t;
                                }
                                if nw == 1 as libc::c_int as libc::c_ulong {
                                    let sv: gsl_matrix_const_view = gsl_matrix_const_submatrix(
                                        S,
                                        j,
                                        j,
                                        na,
                                        na,
                                    );
                                    let mut xv: gsl_vector_view = gsl_vector_view {
                                        vector: gsl_vector {
                                            size: 0,
                                            stride: 0,
                                            data: 0 as *mut libc::c_double,
                                            block: 0 as *mut gsl_block,
                                            owner: 0,
                                        },
                                    };
                                    let mut bv: gsl_vector_view = gsl_vector_view {
                                        vector: gsl_vector {
                                            size: 0,
                                            stride: 0,
                                            data: 0 as *mut libc::c_double,
                                            block: 0 as *mut gsl_block,
                                            owner: 0,
                                        },
                                    };
                                    bv = gsl_vector_subvector((*w).work3, j, na);
                                    xv = gsl_vector_view_array_with_stride(
                                        sum.as_mut_ptr(),
                                        2 as libc::c_int as size_t,
                                        na,
                                    );
                                    gsl_schur_solve_equation(
                                        acoef,
                                        &sv.matrix,
                                        bcoefr,
                                        bdiag[0 as libc::c_int as usize],
                                        bdiag[1 as libc::c_int as usize],
                                        &mut bv.vector,
                                        &mut xv.vector,
                                        &mut scale,
                                        &mut temp,
                                        dmin,
                                    );
                                } else {
                                    let mut bdat: [libc::c_double; 4] = [0.; 4];
                                    let sv_0: gsl_matrix_const_view = gsl_matrix_const_submatrix(
                                        S,
                                        j,
                                        j,
                                        na,
                                        na,
                                    );
                                    let mut xv_0: gsl_vector_complex_view = gsl_vector_complex_view_array(
                                        sum.as_mut_ptr(),
                                        na,
                                    );
                                    let mut bv_0: gsl_vector_complex_view = gsl_vector_complex_view_array(
                                        bdat.as_mut_ptr(),
                                        na,
                                    );
                                    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
                                    bdat[0 as libc::c_int
                                        as usize] = gsl_vector_get((*w).work3, j);
                                    bdat[1 as libc::c_int
                                        as usize] = gsl_vector_get((*w).work4, j);
                                    if na == 2 as libc::c_int as libc::c_ulong {
                                        bdat[2 as libc::c_int
                                            as usize] = gsl_vector_get(
                                            (*w).work3,
                                            j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                        );
                                        bdat[3 as libc::c_int
                                            as usize] = gsl_vector_get(
                                            (*w).work4,
                                            j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                        );
                                    }
                                    z.dat[0 as libc::c_int as usize] = bcoefr;
                                    z.dat[1 as libc::c_int as usize] = bcoefi;
                                    gsl_schur_solve_equation_z(
                                        acoef,
                                        &sv_0.matrix,
                                        &mut z,
                                        bdiag[0 as libc::c_int as usize],
                                        bdiag[1 as libc::c_int as usize],
                                        &mut bv_0.vector,
                                        &mut xv_0.vector,
                                        &mut scale,
                                        &mut temp,
                                        dmin,
                                    );
                                }
                                if scale < 1.0f64 {
                                    jr = 0 as libc::c_int as size_t;
                                    while jr <= je {
                                        gsl_vector_set(
                                            (*w).work3,
                                            jr,
                                            scale * gsl_vector_get((*w).work3, jr),
                                        );
                                        if nw == 2 as libc::c_int as libc::c_ulong {
                                            gsl_vector_set(
                                                (*w).work4,
                                                jr,
                                                scale * gsl_vector_get((*w).work4, jr),
                                            );
                                        }
                                        jr = jr.wrapping_add(1);
                                        jr;
                                    }
                                }
                                xmax = if scale * xmax > temp {
                                    scale * xmax
                                } else {
                                    temp
                                };
                                jr = 0 as libc::c_int as size_t;
                                while jr < na {
                                    gsl_vector_set(
                                        (*w).work3,
                                        j.wrapping_add(jr),
                                        sum[jr.wrapping_mul(na) as usize],
                                    );
                                    if nw == 2 as libc::c_int as libc::c_ulong {
                                        gsl_vector_set(
                                            (*w).work4,
                                            j.wrapping_add(jr),
                                            sum[jr
                                                .wrapping_mul(na)
                                                .wrapping_add(1 as libc::c_int as libc::c_ulong) as usize],
                                        );
                                    }
                                    jr = jr.wrapping_add(1);
                                    jr;
                                }
                                if j > 0 as libc::c_int as libc::c_ulong {
                                    xscale = 1.0f64
                                        / (if 1.0f64 > xmax { 1.0f64 } else { xmax });
                                    temp = acoefa * gsl_vector_get((*w).work1, j)
                                        + bcoefa * gsl_vector_get((*w).work2, j);
                                    if il2by2 != 0 {
                                        temp = if temp
                                            > acoefa
                                                * gsl_vector_get(
                                                    (*w).work1,
                                                    j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                                )
                                                + bcoefa
                                                    * gsl_vector_get(
                                                        (*w).work2,
                                                        j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                                    )
                                        {
                                            temp
                                        } else {
                                            acoefa
                                                * gsl_vector_get(
                                                    (*w).work1,
                                                    j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                                )
                                                + bcoefa
                                                    * gsl_vector_get(
                                                        (*w).work2,
                                                        j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                                    )
                                        };
                                    }
                                    temp = if temp
                                        > (if acoefa > bcoefa { acoefa } else { bcoefa })
                                    {
                                        temp
                                    } else if acoefa > bcoefa {
                                        acoefa
                                    } else {
                                        bcoefa
                                    };
                                    if temp > bignum * xscale {
                                        jr = 0 as libc::c_int as size_t;
                                        while jr <= je {
                                            gsl_vector_set(
                                                (*w).work3,
                                                jr,
                                                xscale * gsl_vector_get((*w).work3, jr),
                                            );
                                            if nw == 2 as libc::c_int as libc::c_ulong {
                                                gsl_vector_set(
                                                    (*w).work4,
                                                    jr,
                                                    xscale * gsl_vector_get((*w).work4, jr),
                                                );
                                            }
                                            jr = jr.wrapping_add(1);
                                            jr;
                                        }
                                        xmax *= xscale;
                                    }
                                    ja = 0 as libc::c_int as size_t;
                                    while ja < na {
                                        if complex_pair != 0 {
                                            creala = acoef
                                                * gsl_vector_get((*w).work3, j.wrapping_add(ja));
                                            cimaga = acoef
                                                * gsl_vector_get((*w).work4, j.wrapping_add(ja));
                                            crealb = bcoefr
                                                * gsl_vector_get((*w).work3, j.wrapping_add(ja))
                                                - bcoefi * gsl_vector_get((*w).work4, j.wrapping_add(ja));
                                            cimagb = bcoefi
                                                * gsl_vector_get((*w).work3, j.wrapping_add(ja))
                                                + bcoefr * gsl_vector_get((*w).work4, j.wrapping_add(ja));
                                            jr = 0 as libc::c_int as size_t;
                                            while jr
                                                <= j.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            {
                                                gsl_vector_set(
                                                    (*w).work3,
                                                    jr,
                                                    gsl_vector_get((*w).work3, jr)
                                                        - creala * gsl_matrix_get(S, jr, j.wrapping_add(ja))
                                                        + crealb * gsl_matrix_get(T, jr, j.wrapping_add(ja)),
                                                );
                                                gsl_vector_set(
                                                    (*w).work4,
                                                    jr,
                                                    gsl_vector_get((*w).work4, jr)
                                                        - cimaga * gsl_matrix_get(S, jr, j.wrapping_add(ja))
                                                        + cimagb * gsl_matrix_get(T, jr, j.wrapping_add(ja)),
                                                );
                                                jr = jr.wrapping_add(1);
                                                jr;
                                            }
                                        } else {
                                            creala = acoef
                                                * gsl_vector_get((*w).work3, j.wrapping_add(ja));
                                            crealb = bcoefr
                                                * gsl_vector_get((*w).work3, j.wrapping_add(ja));
                                            jr = 0 as libc::c_int as size_t;
                                            while jr
                                                <= j.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            {
                                                gsl_vector_set(
                                                    (*w).work3,
                                                    jr,
                                                    gsl_vector_get((*w).work3, jr)
                                                        - creala * gsl_matrix_get(S, jr, j.wrapping_add(ja))
                                                        + crealb * gsl_matrix_get(T, jr, j.wrapping_add(ja)),
                                                );
                                                jr = jr.wrapping_add(1);
                                                jr;
                                            }
                                        }
                                        ja = ja.wrapping_add(1);
                                        ja;
                                    }
                                }
                                il2by2 = 0 as libc::c_int;
                            }
                            _ => {}
                        }
                        is -= 1;
                        is;
                    }
                    jr = 0 as libc::c_int as size_t;
                    while jr < N {
                        gsl_vector_set(
                            (*w).work5,
                            jr,
                            gsl_vector_get((*w).work3, 0 as libc::c_int as size_t)
                                * gsl_matrix_get(Z, jr, 0 as libc::c_int as size_t),
                        );
                        if nw == 2 as libc::c_int as libc::c_ulong {
                            gsl_vector_set(
                                (*w).work6,
                                jr,
                                gsl_vector_get((*w).work4, 0 as libc::c_int as size_t)
                                    * gsl_matrix_get(Z, jr, 0 as libc::c_int as size_t),
                            );
                        }
                        jr = jr.wrapping_add(1);
                        jr;
                    }
                    jc = 1 as libc::c_int as size_t;
                    while jc <= je {
                        jr = 0 as libc::c_int as size_t;
                        while jr < N {
                            gsl_vector_set(
                                (*w).work5,
                                jr,
                                gsl_vector_get((*w).work5, jr)
                                    + gsl_vector_get((*w).work3, jc) * gsl_matrix_get(Z, jr, jc),
                            );
                            if nw == 2 as libc::c_int as libc::c_ulong {
                                gsl_vector_set(
                                    (*w).work6,
                                    jr,
                                    gsl_vector_get((*w).work6, jr)
                                        + gsl_vector_get((*w).work4, jc) * gsl_matrix_get(Z, jr, jc),
                                );
                            }
                            jr = jr.wrapping_add(1);
                            jr;
                        }
                        jc = jc.wrapping_add(1);
                        jc;
                    }
                    if complex_pair != 0 {
                        ecol = gsl_matrix_complex_column(
                            evec,
                            je.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        );
                        re = gsl_vector_complex_real(&mut ecol.vector);
                        im = gsl_vector_complex_imag(&mut ecol.vector);
                        ecol = gsl_matrix_complex_column(evec, je);
                        re2 = gsl_vector_complex_real(&mut ecol.vector);
                        im2 = gsl_vector_complex_imag(&mut ecol.vector);
                    } else {
                        ecol = gsl_matrix_complex_column(evec, je);
                        re = gsl_vector_complex_real(&mut ecol.vector);
                        im = gsl_vector_complex_imag(&mut ecol.vector);
                    }
                    jr = 0 as libc::c_int as size_t;
                    while jr < N {
                        gsl_vector_set(
                            &mut re.vector,
                            jr,
                            gsl_vector_get((*w).work5, jr),
                        );
                        if complex_pair != 0 {
                            gsl_vector_set(
                                &mut im.vector,
                                jr,
                                gsl_vector_get((*w).work6, jr),
                            );
                            gsl_vector_set(
                                &mut re2.vector,
                                jr,
                                gsl_vector_get((*w).work5, jr),
                            );
                            gsl_vector_set(
                                &mut im2.vector,
                                jr,
                                -gsl_vector_get((*w).work6, jr),
                            );
                        } else {
                            gsl_vector_set(&mut im.vector, jr, 0.0f64);
                        }
                        jr = jr.wrapping_add(1);
                        jr;
                    }
                    xmax = 0.0f64;
                    if complex_pair != 0 {
                        j = 0 as libc::c_int as size_t;
                        while j < N {
                            xmax = if xmax
                                > fabs(gsl_vector_get(&mut re.vector, j))
                                    + fabs(gsl_vector_get(&mut im.vector, j))
                            {
                                xmax
                            } else {
                                fabs(gsl_vector_get(&mut re.vector, j))
                                    + fabs(gsl_vector_get(&mut im.vector, j))
                            };
                            j = j.wrapping_add(1);
                            j;
                        }
                    } else {
                        j = 0 as libc::c_int as size_t;
                        while j < N {
                            xmax = if xmax > fabs(gsl_vector_get(&mut re.vector, j)) {
                                xmax
                            } else {
                                fabs(gsl_vector_get(&mut re.vector, j))
                            };
                            j = j.wrapping_add(1);
                            j;
                        }
                    }
                    if xmax > 2.2250738585072014e-308f64 {
                        xscale = 1.0f64 / xmax;
                        j = 0 as libc::c_int as size_t;
                        while j < N {
                            gsl_vector_set(
                                &mut re.vector,
                                j,
                                gsl_vector_get(&mut re.vector, j) * xscale,
                            );
                            if complex_pair != 0 {
                                gsl_vector_set(
                                    &mut im.vector,
                                    j,
                                    gsl_vector_get(&mut im.vector, j) * xscale,
                                );
                                gsl_vector_set(
                                    &mut re2.vector,
                                    j,
                                    gsl_vector_get(&mut re2.vector, j) * xscale,
                                );
                                gsl_vector_set(
                                    &mut im2.vector,
                                    j,
                                    gsl_vector_get(&mut im2.vector, j) * xscale,
                                );
                            }
                            j = j.wrapping_add(1);
                            j;
                        }
                    }
                }
            }
        }
        k = k.wrapping_add(1);
        k;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn genv_normalize_eigenvectors(
    mut alpha: *mut gsl_vector_complex,
    mut evec: *mut gsl_matrix_complex,
) {
    let N: size_t = (*evec).size1;
    let mut i: size_t = 0;
    let mut ai: gsl_complex = gsl_complex { dat: [0.; 2] };
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
        ai = gsl_vector_complex_get(alpha, i);
        vi = gsl_matrix_complex_column(evec, i);
        re = gsl_vector_complex_real(&mut vi.vector);
        if ai.dat[1 as libc::c_int as usize] == 0.0f64 {
            scale = 1.0f64 / gsl_blas_dnrm2(&mut re.vector);
            gsl_blas_dscal(scale, &mut re.vector);
        } else if ai.dat[1 as libc::c_int as usize] > 0.0f64 {
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
