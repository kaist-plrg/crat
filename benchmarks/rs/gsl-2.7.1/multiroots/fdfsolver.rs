use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_calloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
    fn gsl_matrix_calloc(n1: size_t, n2: size_t) -> *mut gsl_matrix;
    fn gsl_matrix_free(m: *mut gsl_matrix);
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
pub struct gsl_multiroot_function_fdf_struct {
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
    pub params: *mut libc::c_void,
}
pub type gsl_multiroot_function_fdf = gsl_multiroot_function_fdf_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multiroot_fdfsolver_type {
    pub name: *const libc::c_char,
    pub size: size_t,
    pub alloc: Option::<unsafe extern "C" fn(*mut libc::c_void, size_t) -> libc::c_int>,
    pub set: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut gsl_multiroot_function_fdf,
            *mut gsl_vector,
            *mut gsl_vector,
            *mut gsl_matrix,
            *mut gsl_vector,
        ) -> libc::c_int,
    >,
    pub iterate: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut gsl_multiroot_function_fdf,
            *mut gsl_vector,
            *mut gsl_vector,
            *mut gsl_matrix,
            *mut gsl_vector,
        ) -> libc::c_int,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multiroot_fdfsolver {
    pub type_0: *const gsl_multiroot_fdfsolver_type,
    pub fdf: *mut gsl_multiroot_function_fdf,
    pub x: *mut gsl_vector,
    pub f: *mut gsl_vector,
    pub J: *mut gsl_matrix,
    pub dx: *mut gsl_vector,
    pub state: *mut libc::c_void,
}
pub unsafe extern "C" fn gsl_multiroot_fdfsolver_alloc(
    mut T: *const gsl_multiroot_fdfsolver_type,
    mut n: size_t,
) -> *mut gsl_multiroot_fdfsolver {
    let mut status: libc::c_int = 0;
    let mut s: *mut gsl_multiroot_fdfsolver = 0 as *mut gsl_multiroot_fdfsolver;
    s = malloc(::std::mem::size_of::<gsl_multiroot_fdfsolver>() as libc::c_ulong)
        as *mut gsl_multiroot_fdfsolver;
    if s.is_null() {
        gsl_error(
            b"failed to allocate space for multiroot solver struct\0" as *const u8
                as *const libc::c_char,
            b"fdfsolver.c\0" as *const u8 as *const libc::c_char,
            39 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multiroot_fdfsolver;
    }
    (*s).x = gsl_vector_calloc(n);
    if ((*s).x).is_null() {
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for x\0" as *const u8 as *const libc::c_char,
            b"fdfsolver.c\0" as *const u8 as *const libc::c_char,
            47 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multiroot_fdfsolver;
    }
    (*s).f = gsl_vector_calloc(n);
    if ((*s).f).is_null() {
        gsl_vector_free((*s).x);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for f\0" as *const u8 as *const libc::c_char,
            b"fdfsolver.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multiroot_fdfsolver;
    }
    (*s).J = gsl_matrix_calloc(n, n);
    if ((*s).J).is_null() {
        gsl_vector_free((*s).x);
        gsl_vector_free((*s).f);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for g\0" as *const u8 as *const libc::c_char,
            b"fdfsolver.c\0" as *const u8 as *const libc::c_char,
            66 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multiroot_fdfsolver;
    }
    (*s).dx = gsl_vector_calloc(n);
    if ((*s).dx).is_null() {
        gsl_matrix_free((*s).J);
        gsl_vector_free((*s).x);
        gsl_vector_free((*s).f);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for dx\0" as *const u8 as *const libc::c_char,
            b"fdfsolver.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multiroot_fdfsolver;
    }
    (*s).state = malloc((*T).size);
    if ((*s).state).is_null() {
        gsl_vector_free((*s).dx);
        gsl_vector_free((*s).x);
        gsl_vector_free((*s).f);
        gsl_matrix_free((*s).J);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for multiroot solver state\0" as *const u8
                as *const libc::c_char,
            b"fdfsolver.c\0" as *const u8 as *const libc::c_char,
            91 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multiroot_fdfsolver;
    }
    (*s).type_0 = T;
    status = ((*(*s).type_0).alloc).unwrap()((*s).state, n);
    if status != GSL_SUCCESS as libc::c_int {
        free((*s).state);
        gsl_vector_free((*s).dx);
        gsl_vector_free((*s).x);
        gsl_vector_free((*s).f);
        gsl_matrix_free((*s).J);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to set solver\0" as *const u8 as *const libc::c_char,
            b"fdfsolver.c\0" as *const u8 as *const libc::c_char,
            107 as libc::c_int,
            status,
        );
        return 0 as *mut gsl_multiroot_fdfsolver;
    }
    (*s).fdf = 0 as *mut gsl_multiroot_function_fdf;
    return s;
}
pub unsafe extern "C" fn gsl_multiroot_fdfsolver_set(
    mut s: *mut gsl_multiroot_fdfsolver,
    mut f: *mut gsl_multiroot_function_fdf,
    mut x: *const gsl_vector,
) -> libc::c_int {
    if (*(*s).x).size != (*f).n {
        gsl_error(
            b"function incompatible with solver size\0" as *const u8
                as *const libc::c_char,
            b"fdfsolver.c\0" as *const u8 as *const libc::c_char,
            122 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    if (*x).size != (*f).n {
        gsl_error(
            b"vector length not compatible with function\0" as *const u8
                as *const libc::c_char,
            b"fdfsolver.c\0" as *const u8 as *const libc::c_char,
            127 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    (*s).fdf = f;
    gsl_vector_memcpy((*s).x, x);
    return ((*(*s).type_0).set)
        .unwrap()((*s).state, (*s).fdf, (*s).x, (*s).f, (*s).J, (*s).dx);
}
pub unsafe extern "C" fn gsl_multiroot_fdfsolver_iterate(
    mut s: *mut gsl_multiroot_fdfsolver,
) -> libc::c_int {
    return ((*(*s).type_0).iterate)
        .unwrap()((*s).state, (*s).fdf, (*s).x, (*s).f, (*s).J, (*s).dx);
}
pub unsafe extern "C" fn gsl_multiroot_fdfsolver_free(
    mut s: *mut gsl_multiroot_fdfsolver,
) {
    if s.is_null() {
        return;
    }
    ((*(*s).type_0).free).unwrap()((*s).state);
    free((*s).state);
    gsl_vector_free((*s).dx);
    gsl_vector_free((*s).x);
    gsl_vector_free((*s).f);
    gsl_matrix_free((*s).J);
    free(s as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_multiroot_fdfsolver_name(
    mut s: *const gsl_multiroot_fdfsolver,
) -> *const libc::c_char {
    return (*(*s).type_0).name;
}
pub unsafe extern "C" fn gsl_multiroot_fdfsolver_root(
    mut s: *const gsl_multiroot_fdfsolver,
) -> *mut gsl_vector {
    return (*s).x;
}
pub unsafe extern "C" fn gsl_multiroot_fdfsolver_dx(
    mut s: *const gsl_multiroot_fdfsolver,
) -> *mut gsl_vector {
    return (*s).dx;
}
pub unsafe extern "C" fn gsl_multiroot_fdfsolver_f(
    mut s: *const gsl_multiroot_fdfsolver,
) -> *mut gsl_vector {
    return (*s).f;
}
