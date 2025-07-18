use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_alloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_isnull(v: *const gsl_vector) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multiroot_function_struct {
    pub f: Option::<
        unsafe extern "C" fn(
            *const gsl_vector,
            *mut libc::c_void,
            *mut gsl_vector,
        ) -> libc::c_int,
    >,
    pub n: size_t,
    pub params: *mut libc::c_void,
}
pub type gsl_multiroot_function = gsl_multiroot_function_struct;
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
pub unsafe extern "C" fn gsl_multiroot_fdjacobian(
    mut F: *mut gsl_multiroot_function,
    mut x: *const gsl_vector,
    mut f: *const gsl_vector,
    mut epsrel: libc::c_double,
    mut jacobian: *mut gsl_matrix,
) -> libc::c_int {
    let n: size_t = (*x).size;
    let m: size_t = (*f).size;
    let n1: size_t = (*jacobian).size1;
    let n2: size_t = (*jacobian).size2;
    let mut status: libc::c_int = 0 as libc::c_int;
    if m != n1 || n != n2 {
        gsl_error(
            b"function and jacobian are not conformant\0" as *const u8
                as *const libc::c_char,
            b"fdjac.c\0" as *const u8 as *const libc::c_char,
            36 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut x1: *mut gsl_vector = 0 as *mut gsl_vector;
    let mut f1: *mut gsl_vector = 0 as *mut gsl_vector;
    x1 = gsl_vector_alloc(n);
    if x1.is_null() {
        gsl_error(
            b"failed to allocate space for x1 workspace\0" as *const u8
                as *const libc::c_char,
            b"fdjac.c\0" as *const u8 as *const libc::c_char,
            47 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    f1 = gsl_vector_alloc(m);
    if f1.is_null() {
        gsl_vector_free(x1);
        gsl_error(
            b"failed to allocate space for f1 workspace\0" as *const u8
                as *const libc::c_char,
            b"fdjac.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    gsl_vector_memcpy(x1, x);
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut xj: libc::c_double = gsl_vector_get(x, j);
        let mut dx: libc::c_double = epsrel * fabs(xj);
        if dx == 0 as libc::c_int as libc::c_double {
            dx = epsrel;
        }
        gsl_vector_set(x1, j, xj + dx);
        let mut f_stat: libc::c_int = (Some(((*F).f).unwrap()))
            .unwrap()(x1, (*F).params, f1);
        if f_stat != GSL_SUCCESS as libc::c_int {
            status = GSL_EBADFUNC as libc::c_int;
            break;
        } else {
            gsl_vector_set(x1, j, xj);
            i = 0 as libc::c_int as size_t;
            while i < m {
                let mut g1: libc::c_double = gsl_vector_get(f1, i);
                let mut g0: libc::c_double = gsl_vector_get(f, i);
                gsl_matrix_set(jacobian, i, j, (g1 - g0) / dx);
                i = i.wrapping_add(1);
                i;
            }
            let mut col: gsl_vector_view = gsl_matrix_column(jacobian, j);
            let mut null_col: libc::c_int = gsl_vector_isnull(&mut col.vector);
            if null_col != 0 {
                status = GSL_ESING as libc::c_int;
            }
            j = j.wrapping_add(1);
            j;
        }
    }
    gsl_vector_free(x1);
    gsl_vector_free(f1);
    if status != 0 { return status } else { return GSL_SUCCESS as libc::c_int };
}
