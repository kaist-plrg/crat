use ::libc;
extern "C" {
    fn gsl_multifit_nlinear_eval_f(
        fdf: *mut gsl_multifit_nlinear_fdf,
        x: *const gsl_vector,
        swts: *const gsl_vector,
        y: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_matrix_const_row(m: *const gsl_matrix, i: size_t) -> _gsl_vector_const_view;
    fn gsl_blas_ddot(
        X: *const gsl_vector,
        Y: *const gsl_vector,
        result: *mut libc::c_double,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
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
pub struct gsl_multifit_nlinear_fdf {
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
    pub fvv: Option::<
        unsafe extern "C" fn(
            *const gsl_vector,
            *const gsl_vector,
            *mut libc::c_void,
            *mut gsl_vector,
        ) -> libc::c_int,
    >,
    pub n: size_t,
    pub p: size_t,
    pub params: *mut libc::c_void,
    pub nevalf: size_t,
    pub nevaldf: size_t,
    pub nevalfvv: size_t,
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
unsafe extern "C" fn fdfvv(
    h: libc::c_double,
    mut x: *const gsl_vector,
    mut v: *const gsl_vector,
    mut f: *const gsl_vector,
    mut J: *const gsl_matrix,
    mut swts: *const gsl_vector,
    mut fdf: *mut gsl_multifit_nlinear_fdf,
    mut fvv: *mut gsl_vector,
    mut work: *mut gsl_vector,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let n: size_t = (*fdf).n;
    let p: size_t = (*fdf).p;
    let hinv: libc::c_double = 1.0f64 / h;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < p {
        let mut xi: libc::c_double = gsl_vector_get(x, i);
        let mut vi: libc::c_double = gsl_vector_get(v, i);
        gsl_vector_set(work, i, xi + h * vi);
        i = i.wrapping_add(1);
        i;
    }
    status = gsl_multifit_nlinear_eval_f(fdf, work, swts, fvv);
    if status != 0 {
        return status;
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut fi: libc::c_double = gsl_vector_get(f, i);
        let mut fip: libc::c_double = gsl_vector_get(fvv, i);
        let row: gsl_vector_const_view = gsl_matrix_const_row(J, i);
        let mut u: libc::c_double = 0.;
        let mut fvvi: libc::c_double = 0.;
        gsl_blas_ddot(&row.vector, v, &mut u);
        fvvi = 2.0f64 * hinv * ((fip - fi) * hinv - u);
        gsl_vector_set(fvv, i, fvvi);
        i = i.wrapping_add(1);
        i;
    }
    return status;
}
pub unsafe extern "C" fn gsl_multifit_nlinear_fdfvv(
    h: libc::c_double,
    mut x: *const gsl_vector,
    mut v: *const gsl_vector,
    mut f: *const gsl_vector,
    mut J: *const gsl_matrix,
    mut swts: *const gsl_vector,
    mut fdf: *mut gsl_multifit_nlinear_fdf,
    mut fvv: *mut gsl_vector,
    mut work: *mut gsl_vector,
) -> libc::c_int {
    return fdfvv(h, x, v, f, J, swts, fdf, fvv, work);
}
