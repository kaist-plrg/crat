use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_multifit_eval_wf(
        fdf: *mut gsl_multifit_function_fdf,
        x: *const gsl_vector,
        wts: *const gsl_vector,
        y: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_matrix_column(m: *mut gsl_matrix, j: size_t) -> _gsl_vector_view;
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
pub struct _gsl_vector_view {
    pub vector: gsl_vector,
}
pub type gsl_vector_view = _gsl_vector_view;
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
unsafe extern "C" fn gsl_matrix_set(
    mut m: *mut gsl_matrix,
    i: size_t,
    j: size_t,
    x: libc::c_double,
) {
    *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize) = x;
}
unsafe extern "C" fn fdjac(
    mut x: *const gsl_vector,
    mut wts: *const gsl_vector,
    mut fdf: *mut gsl_multifit_function_fdf,
    mut f: *const gsl_vector,
    mut J: *mut gsl_matrix,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut h: libc::c_double = 0.;
    let epsfcn: libc::c_double = 0.0f64;
    let mut eps: libc::c_double = sqrt(
        if epsfcn > 2.2204460492503131e-16f64 {
            epsfcn
        } else {
            2.2204460492503131e-16f64
        },
    );
    j = 0 as libc::c_int as size_t;
    while j < (*fdf).p {
        let mut xj: libc::c_double = gsl_vector_get(x, j);
        let mut v: gsl_vector_view = gsl_matrix_column(J, j);
        h = eps * fabs(xj);
        if h == 0.0f64 {
            h = eps;
        }
        gsl_vector_set(x as *mut gsl_vector, j, xj + h);
        status += gsl_multifit_eval_wf(fdf, x, wts, &mut v.vector);
        if status != 0 {
            return status;
        }
        gsl_vector_set(x as *mut gsl_vector, j, xj);
        h = 1.0f64 / h;
        i = 0 as libc::c_int as size_t;
        while i < (*fdf).n {
            let mut fnext: libc::c_double = gsl_vector_get(&mut v.vector, i);
            let mut fi: libc::c_double = gsl_vector_get(f, i);
            gsl_matrix_set(J, i, j, (fnext - fi) * h);
            i = i.wrapping_add(1);
            i;
        }
        j = j.wrapping_add(1);
        j;
    }
    return status;
}
pub unsafe extern "C" fn gsl_multifit_fdfsolver_dif_df(
    mut x: *const gsl_vector,
    mut wts: *const gsl_vector,
    mut fdf: *mut gsl_multifit_function_fdf,
    mut f: *const gsl_vector,
    mut J: *mut gsl_matrix,
) -> libc::c_int {
    return fdjac(x, wts, fdf, f, J);
}
pub unsafe extern "C" fn gsl_multifit_fdfsolver_dif_fdf(
    mut x: *const gsl_vector,
    mut fdf: *mut gsl_multifit_function_fdf,
    mut f: *mut gsl_vector,
    mut J: *mut gsl_matrix,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    status = gsl_multifit_eval_wf(fdf, x, 0 as *const gsl_vector, f);
    if status != 0 {
        return status;
    }
    status = fdjac(x, 0 as *const gsl_vector, fdf, f, J);
    if status != 0 {
        return status;
    }
    return status;
}
