use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_multifit_nlinear_eval_f(
        fdf: *mut gsl_multifit_nlinear_fdf,
        x: *const gsl_vector,
        swts: *const gsl_vector,
        y: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_matrix_column(m: *mut gsl_matrix, j: size_t) -> _gsl_vector_view;
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
pub struct gsl_matrix {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: libc::c_int,
}
pub type gsl_multifit_nlinear_fdtype = libc::c_uint;
pub const GSL_MULTIFIT_NLINEAR_CTRDIFF: gsl_multifit_nlinear_fdtype = 1;
pub const GSL_MULTIFIT_NLINEAR_FWDIFF: gsl_multifit_nlinear_fdtype = 0;
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
#[inline]
unsafe extern "C" fn gsl_matrix_set(
    mut m: *mut gsl_matrix,
    i: size_t,
    j: size_t,
    x: libc::c_double,
) {
    *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize) = x;
}
unsafe extern "C" fn forward_jac(
    h: libc::c_double,
    mut x: *const gsl_vector,
    mut wts: *const gsl_vector,
    mut fdf: *mut gsl_multifit_nlinear_fdf,
    mut f: *const gsl_vector,
    mut J: *mut gsl_matrix,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut delta: libc::c_double = 0.;
    j = 0 as libc::c_int as size_t;
    while j < (*fdf).p {
        let mut xj: libc::c_double = gsl_vector_get(x, j);
        let mut v: gsl_vector_view = gsl_matrix_column(J, j);
        delta = h * fabs(xj);
        if delta == 0.0f64 {
            delta = h;
        }
        gsl_vector_set(x as *mut gsl_vector, j, xj + delta);
        status += gsl_multifit_nlinear_eval_f(fdf, x, wts, &mut v.vector);
        if status != 0 {
            return status;
        }
        gsl_vector_set(x as *mut gsl_vector, j, xj);
        delta = 1.0f64 / delta;
        i = 0 as libc::c_int as size_t;
        while i < (*fdf).n {
            let mut fnext: libc::c_double = gsl_vector_get(&mut v.vector, i);
            let mut fi: libc::c_double = gsl_vector_get(f, i);
            gsl_matrix_set(J, i, j, (fnext - fi) * delta);
            i = i.wrapping_add(1);
            i;
        }
        j = j.wrapping_add(1);
        j;
    }
    return status;
}
unsafe extern "C" fn center_jac(
    h: libc::c_double,
    mut x: *const gsl_vector,
    mut wts: *const gsl_vector,
    mut fdf: *mut gsl_multifit_nlinear_fdf,
    mut J: *mut gsl_matrix,
    mut work: *mut gsl_vector,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut delta: libc::c_double = 0.;
    j = 0 as libc::c_int as size_t;
    while j < (*fdf).p {
        let mut xj: libc::c_double = gsl_vector_get(x, j);
        let mut v: gsl_vector_view = gsl_matrix_column(J, j);
        delta = h * fabs(xj);
        if delta == 0.0f64 {
            delta = h;
        }
        gsl_vector_set(x as *mut gsl_vector, j, xj + 0.5f64 * delta);
        status += gsl_multifit_nlinear_eval_f(fdf, x, wts, &mut v.vector);
        if status != 0 {
            return status;
        }
        gsl_vector_set(x as *mut gsl_vector, j, xj - 0.5f64 * delta);
        status += gsl_multifit_nlinear_eval_f(fdf, x, wts, work);
        if status != 0 {
            return status;
        }
        gsl_vector_set(x as *mut gsl_vector, j, xj);
        delta = 1.0f64 / delta;
        i = 0 as libc::c_int as size_t;
        while i < (*fdf).n {
            let mut fnext: libc::c_double = gsl_vector_get(&mut v.vector, i);
            let mut fprev: libc::c_double = gsl_vector_get(work, i);
            gsl_matrix_set(J, i, j, (fnext - fprev) * delta);
            i = i.wrapping_add(1);
            i;
        }
        j = j.wrapping_add(1);
        j;
    }
    return status;
}
pub unsafe extern "C" fn gsl_multifit_nlinear_df(
    h: libc::c_double,
    fdtype: gsl_multifit_nlinear_fdtype,
    mut x: *const gsl_vector,
    mut wts: *const gsl_vector,
    mut fdf: *mut gsl_multifit_nlinear_fdf,
    mut f: *const gsl_vector,
    mut J: *mut gsl_matrix,
    mut work: *mut gsl_vector,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    if fdtype as libc::c_uint
        == GSL_MULTIFIT_NLINEAR_FWDIFF as libc::c_int as libc::c_uint
    {
        status = forward_jac(h, x, wts, fdf, f, J);
    } else if fdtype as libc::c_uint
        == GSL_MULTIFIT_NLINEAR_CTRDIFF as libc::c_int as libc::c_uint
    {
        status = center_jac(h, x, wts, fdf, J, work);
    } else {
        gsl_error(
            b"invalid specified fdtype\0" as *const u8 as *const libc::c_char,
            b"fdjac.c\0" as *const u8 as *const libc::c_char,
            190 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    return status;
}
