use ::libc;
extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
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
    fn gsl_vector_set_all(v: *mut gsl_vector, x: libc::c_double);
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_mul(a: *mut gsl_vector, b: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_scale(a: *mut gsl_vector, x: libc::c_double) -> libc::c_int;
    fn gsl_matrix_submatrix(
        m: *mut gsl_matrix,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_view;
    fn gsl_matrix_diagonal(m: *mut gsl_matrix) -> _gsl_vector_view;
    fn gsl_matrix_set_zero(m: *mut gsl_matrix);
    fn gsl_matrix_memcpy(dest: *mut gsl_matrix, src: *const gsl_matrix) -> libc::c_int;
    fn gsl_multifit_fdfsolver_alloc(
        T: *const gsl_multifit_fdfsolver_type,
        n: size_t,
        p: size_t,
    ) -> *mut gsl_multifit_fdfsolver;
    fn gsl_multifit_fdfsolver_wset(
        s: *mut gsl_multifit_fdfsolver,
        f: *mut gsl_multifit_function_fdf,
        x: *const gsl_vector,
        wts: *const gsl_vector,
    ) -> libc::c_int;
    fn gsl_multifit_fdfsolver_iterate(s: *mut gsl_multifit_fdfsolver) -> libc::c_int;
    fn gsl_multifit_fdfsolver_driver(
        s: *mut gsl_multifit_fdfsolver,
        maxiter: size_t,
        xtol: libc::c_double,
        gtol: libc::c_double,
        ftol: libc::c_double,
        info: *mut libc::c_int,
    ) -> libc::c_int;
    fn gsl_multifit_fdfsolver_residual(
        s: *const gsl_multifit_fdfsolver,
    ) -> *mut gsl_vector;
    fn gsl_multifit_eval_wf(
        fdf: *mut gsl_multifit_function_fdf,
        x: *const gsl_vector,
        wts: *const gsl_vector,
        y: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_multifit_fdfsolver_free(s: *mut gsl_multifit_fdfsolver);
    fn gsl_multifit_fdfsolver_name(
        s: *const gsl_multifit_fdfsolver,
    ) -> *const libc::c_char;
    fn gsl_multifit_fdfsolver_position(
        s: *const gsl_multifit_fdfsolver,
    ) -> *mut gsl_vector;
    fn gsl_multifit_eval_wdf(
        fdf: *mut gsl_multifit_function_fdf,
        x: *const gsl_vector,
        wts: *const gsl_vector,
        dy: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_multifit_fdfsolver_dif_df(
        x: *const gsl_vector,
        wts: *const gsl_vector,
        fdf: *mut gsl_multifit_function_fdf,
        f: *const gsl_vector,
        J: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_blas_dgemv(
        TransA: CBLAS_TRANSPOSE_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        X: *const gsl_vector,
        beta: libc::c_double,
        Y: *mut gsl_vector,
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
pub type CBLAS_TRANSPOSE_t = CBLAS_TRANSPOSE;
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
pub struct gsl_multifit_function_fdf_struct {
    pub f: Option::<
        unsafe extern "C" fn(
            *const gsl_vector,
            *mut libc::c_void,
            *mut gsl_vector,
        ) -> libc::c_int,
    >,
    pub df: Option::<
        unsafe extern "C" fn(
            *const gsl_vector,
            *mut libc::c_void,
            *mut gsl_matrix,
        ) -> libc::c_int,
    >,
    pub fdf: Option::<
        unsafe extern "C" fn(
            *const gsl_vector,
            *mut libc::c_void,
            *mut gsl_vector,
            *mut gsl_matrix,
        ) -> libc::c_int,
    >,
    pub n: size_t,
    pub p: size_t,
    pub params: *mut libc::c_void,
    pub nevalf: size_t,
    pub nevaldf: size_t,
}
pub type gsl_multifit_function_fdf = gsl_multifit_function_fdf_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_fdfsolver_type {
    pub name: *const libc::c_char,
    pub size: size_t,
    pub alloc: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t, size_t) -> libc::c_int,
    >,
    pub set: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const gsl_vector,
            *mut gsl_multifit_function_fdf,
            *mut gsl_vector,
            *mut gsl_vector,
            *mut gsl_vector,
        ) -> libc::c_int,
    >,
    pub iterate: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const gsl_vector,
            *mut gsl_multifit_function_fdf,
            *mut gsl_vector,
            *mut gsl_vector,
            *mut gsl_vector,
        ) -> libc::c_int,
    >,
    pub gradient: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut gsl_vector) -> libc::c_int,
    >,
    pub jac: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut gsl_matrix) -> libc::c_int,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_fdfsolver {
    pub type_0: *const gsl_multifit_fdfsolver_type,
    pub fdf: *mut gsl_multifit_function_fdf,
    pub x: *mut gsl_vector,
    pub f: *mut gsl_vector,
    pub dx: *mut gsl_vector,
    pub g: *mut gsl_vector,
    pub sqrt_wts: *mut gsl_vector,
    pub niter: size_t,
    pub state: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_fdfridge {
    pub n: size_t,
    pub p: size_t,
    pub lambda: libc::c_double,
    pub L_diag: *const gsl_vector,
    pub L: *const gsl_matrix,
    pub f: *mut gsl_vector,
    pub wts: *mut gsl_vector,
    pub s: *mut gsl_multifit_fdfsolver,
    pub fdf: *mut gsl_multifit_function_fdf,
    pub fdftik: gsl_multifit_function_fdf,
}
pub unsafe extern "C" fn gsl_multifit_fdfridge_alloc(
    mut T: *const gsl_multifit_fdfsolver_type,
    n: size_t,
    p: size_t,
) -> *mut gsl_multifit_fdfridge {
    let mut work: *mut gsl_multifit_fdfridge = 0 as *mut gsl_multifit_fdfridge;
    work = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gsl_multifit_fdfridge>() as libc::c_ulong,
    ) as *mut gsl_multifit_fdfridge;
    if work.is_null() {
        gsl_error(
            b"failed to allocate workspace\0" as *const u8 as *const libc::c_char,
            b"fdfridge.c\0" as *const u8 as *const libc::c_char,
            42 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_fdfridge;
    }
    (*work).s = gsl_multifit_fdfsolver_alloc(T, n.wrapping_add(p), p);
    if ((*work).s).is_null() {
        gsl_multifit_fdfridge_free(work);
        gsl_error(
            b"failed to allocate space for fdfsolver\0" as *const u8
                as *const libc::c_char,
            b"fdfridge.c\0" as *const u8 as *const libc::c_char,
            50 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_fdfridge;
    }
    (*work).wts = gsl_vector_alloc(n.wrapping_add(p));
    if ((*work).wts).is_null() {
        gsl_multifit_fdfridge_free(work);
        gsl_error(
            b"failed to allocate space for weight vector\0" as *const u8
                as *const libc::c_char,
            b"fdfridge.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_fdfridge;
    }
    (*work).f = gsl_vector_alloc(n);
    if ((*work).f).is_null() {
        gsl_multifit_fdfridge_free(work);
        gsl_error(
            b"failed to allocate space for f vector\0" as *const u8
                as *const libc::c_char,
            b"fdfridge.c\0" as *const u8 as *const libc::c_char,
            66 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_fdfridge;
    }
    (*work).n = n;
    (*work).p = p;
    (*work).lambda = 0.0f64;
    gsl_vector_set_all((*work).wts, 1.0f64);
    return work;
}
pub unsafe extern "C" fn gsl_multifit_fdfridge_free(
    mut work: *mut gsl_multifit_fdfridge,
) {
    if !((*work).s).is_null() {
        gsl_multifit_fdfsolver_free((*work).s);
    }
    if !((*work).wts).is_null() {
        gsl_vector_free((*work).wts);
    }
    if !((*work).f).is_null() {
        gsl_vector_free((*work).f);
    }
    free(work as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_multifit_fdfridge_name(
    mut w: *const gsl_multifit_fdfridge,
) -> *const libc::c_char {
    return gsl_multifit_fdfsolver_name((*w).s);
}
pub unsafe extern "C" fn gsl_multifit_fdfridge_position(
    mut w: *const gsl_multifit_fdfridge,
) -> *mut gsl_vector {
    return gsl_multifit_fdfsolver_position((*w).s);
}
pub unsafe extern "C" fn gsl_multifit_fdfridge_residual(
    mut w: *const gsl_multifit_fdfridge,
) -> *mut gsl_vector {
    return gsl_multifit_fdfsolver_residual((*w).s);
}
pub unsafe extern "C" fn gsl_multifit_fdfridge_niter(
    mut w: *const gsl_multifit_fdfridge,
) -> size_t {
    return (*(*w).s).niter;
}
pub unsafe extern "C" fn gsl_multifit_fdfridge_set(
    mut w: *mut gsl_multifit_fdfridge,
    mut f: *mut gsl_multifit_function_fdf,
    mut x: *const gsl_vector,
    lambda: libc::c_double,
) -> libc::c_int {
    return gsl_multifit_fdfridge_wset(w, f, x, lambda, 0 as *const gsl_vector);
}
pub unsafe extern "C" fn gsl_multifit_fdfridge_wset(
    mut w: *mut gsl_multifit_fdfridge,
    mut f: *mut gsl_multifit_function_fdf,
    mut x: *const gsl_vector,
    lambda: libc::c_double,
    mut wts: *const gsl_vector,
) -> libc::c_int {
    let n: size_t = (*w).n;
    let p: size_t = (*w).p;
    if n != (*f).n || p != (*f).p {
        gsl_error(
            b"function size does not match solver\0" as *const u8 as *const libc::c_char,
            b"fdfridge.c\0" as *const u8 as *const libc::c_char,
            139 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if p != (*x).size {
        gsl_error(
            b"vector length does not match solver\0" as *const u8 as *const libc::c_char,
            b"fdfridge.c\0" as *const u8 as *const libc::c_char,
            143 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if !wts.is_null() && n != (*wts).size {
        gsl_error(
            b"weight vector length does not match solver\0" as *const u8
                as *const libc::c_char,
            b"fdfridge.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut wv: gsl_vector_view = gsl_vector_subvector(
            (*w).wts,
            0 as libc::c_int as size_t,
            n,
        );
        (*w).fdf = f;
        (*w)
            .fdftik
            .f = Some(
            fdfridge_f
                as unsafe extern "C" fn(
                    *const gsl_vector,
                    *mut libc::c_void,
                    *mut gsl_vector,
                ) -> libc::c_int,
        );
        (*w)
            .fdftik
            .df = Some(
            fdfridge_df
                as unsafe extern "C" fn(
                    *const gsl_vector,
                    *mut libc::c_void,
                    *mut gsl_matrix,
                ) -> libc::c_int,
        );
        (*w).fdftik.n = n.wrapping_add(p);
        (*w).fdftik.p = p;
        (*w).fdftik.params = w as *mut libc::c_void;
        (*w).lambda = lambda;
        (*w).L = 0 as *const gsl_matrix;
        if !wts.is_null() {
            gsl_vector_memcpy(&mut wv.vector, wts);
            status = gsl_multifit_fdfsolver_wset((*w).s, &mut (*w).fdftik, x, (*w).wts);
        } else {
            status = gsl_multifit_fdfsolver_wset(
                (*w).s,
                &mut (*w).fdftik,
                x,
                0 as *const gsl_vector,
            );
        }
        (*f).nevalf = (*w).fdftik.nevalf;
        (*f).nevaldf = (*w).fdftik.nevaldf;
        return status;
    };
}
pub unsafe extern "C" fn gsl_multifit_fdfridge_set2(
    mut w: *mut gsl_multifit_fdfridge,
    mut f: *mut gsl_multifit_function_fdf,
    mut x: *const gsl_vector,
    mut lambda: *const gsl_vector,
) -> libc::c_int {
    return gsl_multifit_fdfridge_wset2(w, f, x, lambda, 0 as *const gsl_vector);
}
pub unsafe extern "C" fn gsl_multifit_fdfridge_wset2(
    mut w: *mut gsl_multifit_fdfridge,
    mut f: *mut gsl_multifit_function_fdf,
    mut x: *const gsl_vector,
    mut lambda: *const gsl_vector,
    mut wts: *const gsl_vector,
) -> libc::c_int {
    let n: size_t = (*w).n;
    let p: size_t = (*w).p;
    if n != (*f).n || p != (*f).p {
        gsl_error(
            b"function size does not match solver\0" as *const u8 as *const libc::c_char,
            b"fdfridge.c\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if p != (*x).size {
        gsl_error(
            b"vector length does not match solver\0" as *const u8 as *const libc::c_char,
            b"fdfridge.c\0" as *const u8 as *const libc::c_char,
            212 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*lambda).size != p {
        gsl_error(
            b"lambda vector size does not match solver\0" as *const u8
                as *const libc::c_char,
            b"fdfridge.c\0" as *const u8 as *const libc::c_char,
            216 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if !wts.is_null() && n != (*wts).size {
        gsl_error(
            b"weight vector length does not match solver\0" as *const u8
                as *const libc::c_char,
            b"fdfridge.c\0" as *const u8 as *const libc::c_char,
            220 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut wv: gsl_vector_view = gsl_vector_subvector(
            (*w).wts,
            0 as libc::c_int as size_t,
            n,
        );
        (*w).fdf = f;
        (*(*w).fdf).nevalf = 0 as libc::c_int as size_t;
        (*(*w).fdf).nevaldf = 0 as libc::c_int as size_t;
        (*w)
            .fdftik
            .f = Some(
            fdfridge_f
                as unsafe extern "C" fn(
                    *const gsl_vector,
                    *mut libc::c_void,
                    *mut gsl_vector,
                ) -> libc::c_int,
        );
        (*w)
            .fdftik
            .df = Some(
            fdfridge_df
                as unsafe extern "C" fn(
                    *const gsl_vector,
                    *mut libc::c_void,
                    *mut gsl_matrix,
                ) -> libc::c_int,
        );
        (*w).fdftik.n = n.wrapping_add(p);
        (*w).fdftik.p = p;
        (*w).fdftik.params = w as *mut libc::c_void;
        (*w).lambda = 0.0f64;
        (*w).L_diag = lambda;
        (*w).L = 0 as *const gsl_matrix;
        if !wts.is_null() {
            gsl_vector_memcpy(&mut wv.vector, wts);
            status = gsl_multifit_fdfsolver_wset((*w).s, &mut (*w).fdftik, x, (*w).wts);
        } else {
            status = gsl_multifit_fdfsolver_wset(
                (*w).s,
                &mut (*w).fdftik,
                x,
                0 as *const gsl_vector,
            );
        }
        (*f).nevalf = (*w).fdftik.nevalf;
        (*f).nevaldf = (*w).fdftik.nevaldf;
        return status;
    };
}
pub unsafe extern "C" fn gsl_multifit_fdfridge_set3(
    mut w: *mut gsl_multifit_fdfridge,
    mut f: *mut gsl_multifit_function_fdf,
    mut x: *const gsl_vector,
    mut L: *const gsl_matrix,
) -> libc::c_int {
    return gsl_multifit_fdfridge_wset3(w, f, x, L, 0 as *const gsl_vector);
}
pub unsafe extern "C" fn gsl_multifit_fdfridge_wset3(
    mut w: *mut gsl_multifit_fdfridge,
    mut f: *mut gsl_multifit_function_fdf,
    mut x: *const gsl_vector,
    mut L: *const gsl_matrix,
    mut wts: *const gsl_vector,
) -> libc::c_int {
    let n: size_t = (*w).n;
    let p: size_t = (*w).p;
    if n != (*f).n || p != (*f).p {
        gsl_error(
            b"function size does not match solver\0" as *const u8 as *const libc::c_char,
            b"fdfridge.c\0" as *const u8 as *const libc::c_char,
            284 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if p != (*x).size {
        gsl_error(
            b"vector length does not match solver\0" as *const u8 as *const libc::c_char,
            b"fdfridge.c\0" as *const u8 as *const libc::c_char,
            288 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*L).size2 != p {
        gsl_error(
            b"L matrix size2 does not match solver\0" as *const u8
                as *const libc::c_char,
            b"fdfridge.c\0" as *const u8 as *const libc::c_char,
            292 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if !wts.is_null() && n != (*wts).size {
        gsl_error(
            b"weight vector length does not match solver\0" as *const u8
                as *const libc::c_char,
            b"fdfridge.c\0" as *const u8 as *const libc::c_char,
            296 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut wv: gsl_vector_view = gsl_vector_subvector(
            (*w).wts,
            0 as libc::c_int as size_t,
            n,
        );
        (*w).fdf = f;
        (*(*w).fdf).nevalf = 0 as libc::c_int as size_t;
        (*(*w).fdf).nevaldf = 0 as libc::c_int as size_t;
        (*w)
            .fdftik
            .f = Some(
            fdfridge_f
                as unsafe extern "C" fn(
                    *const gsl_vector,
                    *mut libc::c_void,
                    *mut gsl_vector,
                ) -> libc::c_int,
        );
        (*w)
            .fdftik
            .df = Some(
            fdfridge_df
                as unsafe extern "C" fn(
                    *const gsl_vector,
                    *mut libc::c_void,
                    *mut gsl_matrix,
                ) -> libc::c_int,
        );
        (*w).fdftik.n = n.wrapping_add(p);
        (*w).fdftik.p = p;
        (*w).fdftik.params = w as *mut libc::c_void;
        (*w).lambda = 0.0f64;
        (*w).L_diag = 0 as *const gsl_vector;
        (*w).L = L;
        if !wts.is_null() {
            gsl_vector_memcpy(&mut wv.vector, wts);
            status = gsl_multifit_fdfsolver_wset((*w).s, &mut (*w).fdftik, x, (*w).wts);
        } else {
            status = gsl_multifit_fdfsolver_wset(
                (*w).s,
                &mut (*w).fdftik,
                x,
                0 as *const gsl_vector,
            );
        }
        (*f).nevalf = (*w).fdftik.nevalf;
        (*f).nevaldf = (*w).fdftik.nevaldf;
        return status;
    };
}
pub unsafe extern "C" fn gsl_multifit_fdfridge_iterate(
    mut w: *mut gsl_multifit_fdfridge,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_multifit_fdfsolver_iterate((*w).s);
    (*(*w).fdf).nevalf = (*w).fdftik.nevalf;
    (*(*w).fdf).nevaldf = (*w).fdftik.nevaldf;
    return status;
}
pub unsafe extern "C" fn gsl_multifit_fdfridge_driver(
    mut w: *mut gsl_multifit_fdfridge,
    maxiter: size_t,
    xtol: libc::c_double,
    gtol: libc::c_double,
    ftol: libc::c_double,
    mut info: *mut libc::c_int,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_multifit_fdfsolver_driver(
        (*w).s,
        maxiter,
        xtol,
        gtol,
        ftol,
        info,
    );
    return status;
}
unsafe extern "C" fn fdfridge_f(
    mut x: *const gsl_vector,
    mut params: *mut libc::c_void,
    mut f: *mut gsl_vector,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut w: *mut gsl_multifit_fdfridge = params as *mut gsl_multifit_fdfridge;
    let n: size_t = (*w).n;
    let p: size_t = (*w).p;
    let mut f_user: gsl_vector_view = gsl_vector_subvector(
        f,
        0 as libc::c_int as size_t,
        n,
    );
    let mut f_tik: gsl_vector_view = gsl_vector_subvector(f, n, p);
    status = gsl_multifit_eval_wf(
        (*w).fdf,
        x,
        0 as *const gsl_vector,
        &mut f_user.vector,
    );
    if status != 0 {
        return status;
    }
    if !((*w).L_diag).is_null() {
        gsl_vector_memcpy(&mut f_tik.vector, x);
        gsl_vector_mul(&mut f_tik.vector, (*w).L_diag);
    } else if !((*w).L).is_null() {
        gsl_blas_dgemv(CblasNoTrans, 1.0f64, (*w).L, x, 0.0f64, &mut f_tik.vector);
    } else {
        gsl_vector_memcpy(&mut f_tik.vector, x);
        gsl_vector_scale(&mut f_tik.vector, (*w).lambda);
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn fdfridge_df(
    mut x: *const gsl_vector,
    mut params: *mut libc::c_void,
    mut J: *mut gsl_matrix,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut w: *mut gsl_multifit_fdfridge = params as *mut gsl_multifit_fdfridge;
    let n: size_t = (*w).n;
    let p: size_t = (*w).p;
    let mut J_user: gsl_matrix_view = gsl_matrix_submatrix(
        J,
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
        n,
        p,
    );
    let mut J_tik: gsl_matrix_view = gsl_matrix_submatrix(
        J,
        n,
        0 as libc::c_int as size_t,
        p,
        p,
    );
    let mut diag: gsl_vector_view = gsl_matrix_diagonal(&mut J_tik.matrix);
    if ((*(*w).fdf).df).is_some() {
        status = gsl_multifit_eval_wdf(
            (*w).fdf,
            x,
            0 as *const gsl_vector,
            &mut J_user.matrix,
        );
    } else {
        status = gsl_multifit_eval_wf((*w).fdf, x, 0 as *const gsl_vector, (*w).f);
        status
            += gsl_multifit_fdfsolver_dif_df(
                x,
                0 as *const gsl_vector,
                (*w).fdf,
                (*w).f,
                &mut J_user.matrix,
            );
    }
    if status != 0 {
        return status;
    }
    if !((*w).L_diag).is_null() {
        gsl_matrix_set_zero(&mut J_tik.matrix);
        gsl_vector_memcpy(&mut diag.vector, (*w).L_diag);
    } else if !((*w).L).is_null() {
        gsl_matrix_memcpy(&mut J_tik.matrix, (*w).L);
    } else {
        gsl_matrix_set_zero(&mut J_tik.matrix);
        gsl_vector_set_all(&mut diag.vector, (*w).lambda);
    }
    return GSL_SUCCESS as libc::c_int;
}
