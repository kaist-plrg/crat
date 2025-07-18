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
    fn gsl_vector_min_index(v: *const gsl_vector) -> size_t;
    fn gsl_vector_mul(a: *mut gsl_vector, b: *const gsl_vector) -> libc::c_int;
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
    fn gsl_multifit_linear_lreg(
        smin: libc::c_double,
        smax: libc::c_double,
        reg_param: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_blas_dnrm2(X: *const gsl_vector) -> libc::c_double;
    fn gsl_blas_dgemv(
        TransA: CBLAS_TRANSPOSE_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        X: *const gsl_vector,
        beta: libc::c_double,
        Y: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_min_fminimizer_alloc(
        T: *const gsl_min_fminimizer_type,
    ) -> *mut gsl_min_fminimizer;
    fn gsl_min_fminimizer_free(s: *mut gsl_min_fminimizer);
    fn gsl_min_fminimizer_set(
        s: *mut gsl_min_fminimizer,
        f: *mut gsl_function,
        x_minimum: libc::c_double,
        x_lower: libc::c_double,
        x_upper: libc::c_double,
    ) -> libc::c_int;
    fn gsl_min_fminimizer_iterate(s: *mut gsl_min_fminimizer) -> libc::c_int;
    fn gsl_min_fminimizer_x_lower(s: *const gsl_min_fminimizer) -> libc::c_double;
    fn gsl_min_fminimizer_x_upper(s: *const gsl_min_fminimizer) -> libc::c_double;
    fn gsl_min_fminimizer_minimum(s: *const gsl_min_fminimizer) -> libc::c_double;
    fn gsl_min_test_interval(
        x_lower: libc::c_double,
        x_upper: libc::c_double,
        epsabs: libc::c_double,
        epsrel: libc::c_double,
    ) -> libc::c_int;
    static mut gsl_min_fminimizer_brent: *const gsl_min_fminimizer_type;
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
pub struct gsl_function_struct {
    pub function: Option::<
        unsafe extern "C" fn(libc::c_double, *mut libc::c_void) -> libc::c_double,
    >,
    pub params: *mut libc::c_void,
}
pub type gsl_function = gsl_function_struct;
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
pub struct gsl_multifit_linear_workspace {
    pub nmax: size_t,
    pub pmax: size_t,
    pub n: size_t,
    pub p: size_t,
    pub A: *mut gsl_matrix,
    pub Q: *mut gsl_matrix,
    pub QSI: *mut gsl_matrix,
    pub S: *mut gsl_vector,
    pub t: *mut gsl_vector,
    pub xt: *mut gsl_vector,
    pub D: *mut gsl_vector,
    pub rcond: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gcv_params {
    pub S: *const gsl_vector,
    pub UTy: *const gsl_vector,
    pub delta0: libc::c_double,
    pub np: size_t,
    pub workp: *mut gsl_vector,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_min_fminimizer {
    pub type_0: *const gsl_min_fminimizer_type,
    pub function: *mut gsl_function,
    pub x_minimum: libc::c_double,
    pub x_lower: libc::c_double,
    pub x_upper: libc::c_double,
    pub f_minimum: libc::c_double,
    pub f_lower: libc::c_double,
    pub f_upper: libc::c_double,
    pub state: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_min_fminimizer_type {
    pub name: *const libc::c_char,
    pub size: size_t,
    pub set: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut gsl_function,
            libc::c_double,
            libc::c_double,
            libc::c_double,
            libc::c_double,
            libc::c_double,
            libc::c_double,
        ) -> libc::c_int,
    >,
    pub iterate: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut gsl_function,
            *mut libc::c_double,
            *mut libc::c_double,
            *mut libc::c_double,
            *mut libc::c_double,
            *mut libc::c_double,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
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
pub unsafe extern "C" fn gsl_multifit_linear_gcv_init(
    mut y: *const gsl_vector,
    mut reg_param: *mut gsl_vector,
    mut UTy: *mut gsl_vector,
    mut delta0: *mut libc::c_double,
    mut work: *mut gsl_multifit_linear_workspace,
) -> libc::c_int {
    let n: size_t = (*y).size;
    if n != (*work).n {
        gsl_error(
            b"y vector does not match workspace\0" as *const u8 as *const libc::c_char,
            b"gcv.c\0" as *const u8 as *const libc::c_char,
            69 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*UTy).size != (*work).p {
        gsl_error(
            b"UTy vector does not match workspace\0" as *const u8 as *const libc::c_char,
            b"gcv.c\0" as *const u8 as *const libc::c_char,
            73 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let p: size_t = (*work).p;
        let mut U: gsl_matrix_view = gsl_matrix_submatrix(
            (*work).A,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            n,
            p,
        );
        let mut S: gsl_vector_view = gsl_vector_subvector(
            (*work).S,
            0 as libc::c_int as size_t,
            p,
        );
        let smax: libc::c_double = gsl_vector_get(
            &mut S.vector,
            0 as libc::c_int as size_t,
        );
        let smin: libc::c_double = gsl_vector_get(
            &mut S.vector,
            p.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        let mut dr: libc::c_double = 0.;
        let mut normy: libc::c_double = gsl_blas_dnrm2(y);
        let mut normUTy: libc::c_double = 0.;
        gsl_blas_dgemv(CblasTrans, 1.0f64, &mut U.matrix, y, 0.0f64, UTy);
        normUTy = gsl_blas_dnrm2(UTy);
        dr = (normy + normUTy) * (normy - normUTy);
        gsl_multifit_linear_lreg(smin, smax, reg_param);
        if n > p && dr > 0.0f64 {
            *delta0 = dr;
        } else {
            *delta0 = 0.0f64;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_multifit_linear_gcv_curve(
    mut reg_param: *const gsl_vector,
    mut UTy: *const gsl_vector,
    delta0: libc::c_double,
    mut G: *mut gsl_vector,
    mut work: *mut gsl_multifit_linear_workspace,
) -> libc::c_int {
    let n: size_t = (*work).n;
    let p: size_t = (*work).p;
    let N: size_t = (*reg_param).size;
    if (*UTy).size != p {
        gsl_error(
            b"UTy vector does not match workspace\0" as *const u8 as *const libc::c_char,
            b"gcv.c\0" as *const u8 as *const libc::c_char,
            134 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*G).size != N {
        gsl_error(
            b"size of reg_param and G vectors do not match\0" as *const u8
                as *const libc::c_char,
            b"gcv.c\0" as *const u8 as *const libc::c_char,
            139 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        let mut S: gsl_vector_view = gsl_vector_subvector(
            (*work).S,
            0 as libc::c_int as size_t,
            p,
        );
        let mut workp: gsl_vector_view = gsl_matrix_subcolumn(
            (*work).QSI,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            p,
        );
        let mut params: gcv_params = gcv_params {
            S: 0 as *const gsl_vector,
            UTy: 0 as *const gsl_vector,
            delta0: 0.,
            np: 0,
            workp: 0 as *mut gsl_vector,
        };
        params.S = &mut S.vector;
        params.UTy = UTy;
        params.delta0 = delta0;
        params.np = n.wrapping_sub(p);
        params.workp = &mut workp.vector;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let mut lambdai: libc::c_double = gsl_vector_get(reg_param, i);
            let mut Gi: libc::c_double = gcv_func(
                lambdai,
                &mut params as *mut gcv_params as *mut libc::c_void,
            );
            gsl_vector_set(G, i, Gi);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_multifit_linear_gcv_min(
    mut reg_param: *const gsl_vector,
    mut UTy: *const gsl_vector,
    mut G: *const gsl_vector,
    delta0: libc::c_double,
    mut lambda: *mut libc::c_double,
    mut work: *mut gsl_multifit_linear_workspace,
) -> libc::c_int {
    let n: size_t = (*work).n;
    let p: size_t = (*work).p;
    let npts: size_t = (*reg_param).size;
    if (*UTy).size != p {
        gsl_error(
            b"UTy vector does not match workspace\0" as *const u8 as *const libc::c_char,
            b"gcv.c\0" as *const u8 as *const libc::c_char,
            194 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*G).size != npts {
        gsl_error(
            b"size of reg_param and G vectors do not match\0" as *const u8
                as *const libc::c_char,
            b"gcv.c\0" as *const u8 as *const libc::c_char,
            199 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let max_iter: size_t = 500 as libc::c_int as size_t;
        let tol: libc::c_double = 1.0e-4f64;
        let mut S: gsl_vector_view = gsl_vector_subvector(
            (*work).S,
            0 as libc::c_int as size_t,
            p,
        );
        let mut workp: gsl_vector_view = gsl_matrix_subcolumn(
            (*work).QSI,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            p,
        );
        let mut params: gcv_params = gcv_params {
            S: 0 as *const gsl_vector,
            UTy: 0 as *const gsl_vector,
            delta0: 0.,
            np: 0,
            workp: 0 as *mut gsl_vector,
        };
        let mut idxG: libc::c_int = gsl_vector_min_index(G) as libc::c_int;
        let mut a: libc::c_double = gsl_vector_get(
            reg_param,
            (if (idxG + 1 as libc::c_int) < npts as libc::c_int - 1 as libc::c_int {
                idxG + 1 as libc::c_int
            } else {
                npts as libc::c_int - 1 as libc::c_int
            }) as size_t,
        );
        let mut b: libc::c_double = gsl_vector_get(
            reg_param,
            (if idxG - 1 as libc::c_int > 0 as libc::c_int {
                idxG - 1 as libc::c_int
            } else {
                0 as libc::c_int
            }) as size_t,
        );
        let mut m: libc::c_double = gsl_vector_get(reg_param, idxG as size_t);
        let mut iter: size_t = 0 as libc::c_int as size_t;
        let mut F: gsl_function = gsl_function {
            function: None,
            params: 0 as *mut libc::c_void,
        };
        let mut min_workspace_p: *mut gsl_min_fminimizer = 0 as *mut gsl_min_fminimizer;
        if idxG == 0 as libc::c_int || idxG == npts as libc::c_int - 1 as libc::c_int {
            *lambda = m;
            return GSL_SUCCESS as libc::c_int;
        }
        min_workspace_p = gsl_min_fminimizer_alloc(gsl_min_fminimizer_brent);
        params.S = &mut S.vector;
        params.UTy = UTy;
        params.delta0 = delta0;
        params.np = n.wrapping_sub(p);
        params.workp = &mut workp.vector;
        F
            .function = Some(
            gcv_func
                as unsafe extern "C" fn(
                    libc::c_double,
                    *mut libc::c_void,
                ) -> libc::c_double,
        );
        F.params = &mut params as *mut gcv_params as *mut libc::c_void;
        gsl_min_fminimizer_set(min_workspace_p, &mut F, m, a, b);
        loop {
            iter = iter.wrapping_add(1);
            iter;
            status = gsl_min_fminimizer_iterate(min_workspace_p);
            a = gsl_min_fminimizer_x_lower(min_workspace_p);
            b = gsl_min_fminimizer_x_upper(min_workspace_p);
            status = gsl_min_test_interval(a, b, 0.0f64, tol);
            if !(status == GSL_CONTINUE as libc::c_int && iter < max_iter) {
                break;
            }
        }
        if status == GSL_SUCCESS as libc::c_int {
            *lambda = gsl_min_fminimizer_minimum(min_workspace_p);
        } else {
            status = GSL_EMAXITER as libc::c_int;
        }
        gsl_min_fminimizer_free(min_workspace_p);
        return status;
    };
}
pub unsafe extern "C" fn gsl_multifit_linear_gcv_calc(
    lambda: libc::c_double,
    mut UTy: *const gsl_vector,
    delta0: libc::c_double,
    mut work: *mut gsl_multifit_linear_workspace,
) -> libc::c_double {
    let n: size_t = (*work).n;
    let p: size_t = (*work).p;
    if (*UTy).size != p {
        gsl_error(
            b"UTy vector does not match workspace\0" as *const u8 as *const libc::c_char,
            b"gcv.c\0" as *const u8 as *const libc::c_char,
            285 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return 0.0f64;
    } else {
        let mut S: gsl_vector_view = gsl_vector_subvector(
            (*work).S,
            0 as libc::c_int as size_t,
            p,
        );
        let mut workp: gsl_vector_view = gsl_matrix_subcolumn(
            (*work).QSI,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            p,
        );
        let mut params: gcv_params = gcv_params {
            S: 0 as *const gsl_vector,
            UTy: 0 as *const gsl_vector,
            delta0: 0.,
            np: 0,
            workp: 0 as *mut gsl_vector,
        };
        let mut G: libc::c_double = 0.;
        params.S = &mut S.vector;
        params.UTy = UTy;
        params.delta0 = delta0;
        params.np = n.wrapping_sub(p);
        params.workp = &mut workp.vector;
        G = gcv_func(lambda, &mut params as *mut gcv_params as *mut libc::c_void);
        return G;
    };
}
pub unsafe extern "C" fn gsl_multifit_linear_gcv(
    mut y: *const gsl_vector,
    mut reg_param: *mut gsl_vector,
    mut G: *mut gsl_vector,
    mut lambda: *mut libc::c_double,
    mut G_lambda: *mut libc::c_double,
    mut work: *mut gsl_multifit_linear_workspace,
) -> libc::c_int {
    let n: size_t = (*y).size;
    let N: size_t = (*G).size;
    if n != (*work).n {
        gsl_error(
            b"y vector does not match workspace\0" as *const u8 as *const libc::c_char,
            b"gcv.c\0" as *const u8 as *const libc::c_char,
            333 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*reg_param).size != N {
        gsl_error(
            b"size of reg_param and G vectors do not match\0" as *const u8
                as *const libc::c_char,
            b"gcv.c\0" as *const u8 as *const libc::c_char,
            338 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let p: size_t = (*work).p;
        let mut UTy: gsl_vector_view = gsl_vector_subvector(
            (*work).xt,
            0 as libc::c_int as size_t,
            p,
        );
        let mut delta0: libc::c_double = 0.;
        status = gsl_multifit_linear_gcv_init(
            y,
            reg_param,
            &mut UTy.vector,
            &mut delta0,
            work,
        );
        if status != 0 {
            return status;
        }
        status = gsl_multifit_linear_gcv_curve(
            reg_param,
            &mut UTy.vector,
            delta0,
            G,
            work,
        );
        if status != 0 {
            return status;
        }
        status = gsl_multifit_linear_gcv_min(
            reg_param,
            &mut UTy.vector,
            G,
            delta0,
            lambda,
            work,
        );
        if status != 0 {
            return status;
        }
        *G_lambda = gsl_multifit_linear_gcv_calc(*lambda, &mut UTy.vector, delta0, work);
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn gcv_func(
    mut lambda: libc::c_double,
    mut params: *mut libc::c_void,
) -> libc::c_double {
    let mut par: *mut gcv_params = params as *mut gcv_params;
    let mut S: *const gsl_vector = (*par).S;
    let mut UTy: *const gsl_vector = (*par).UTy;
    let mut delta0: libc::c_double = (*par).delta0;
    let mut np: size_t = (*par).np;
    let mut workp: *mut gsl_vector = (*par).workp;
    let p: size_t = (*S).size;
    let mut i: size_t = 0;
    let mut lambda_sq: libc::c_double = lambda * lambda;
    let mut G: libc::c_double = 0.;
    let mut d: libc::c_double = 0.;
    let mut norm: libc::c_double = 0.;
    let mut sumf: libc::c_double = 0.0f64;
    i = 0 as libc::c_int as size_t;
    while i < p {
        let mut si: libc::c_double = gsl_vector_get(S, i);
        let mut fi: libc::c_double = lambda_sq / (si * si + lambda_sq);
        gsl_vector_set(workp, i, fi);
        sumf += fi;
        i = i.wrapping_add(1);
        i;
    }
    d = np as libc::c_double + sumf;
    gsl_vector_mul(workp, UTy);
    norm = gsl_blas_dnrm2(workp);
    G = (norm * norm + delta0) / (d * d);
    return G;
}
