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
pub struct gsl_function_fdf_struct {
    pub f: Option::<
        unsafe extern "C" fn(libc::c_double, *mut libc::c_void) -> libc::c_double,
    >,
    pub df: Option::<
        unsafe extern "C" fn(libc::c_double, *mut libc::c_void) -> libc::c_double,
    >,
    pub fdf: Option::<
        unsafe extern "C" fn(
            libc::c_double,
            *mut libc::c_void,
            *mut libc::c_double,
            *mut libc::c_double,
        ) -> (),
    >,
    pub params: *mut libc::c_void,
}
pub type gsl_function_fdf = gsl_function_fdf_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_root_fdfsolver_type {
    pub name: *const libc::c_char,
    pub size: size_t,
    pub set: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut gsl_function_fdf,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    pub iterate: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut gsl_function_fdf,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_root_fdfsolver {
    pub type_0: *const gsl_root_fdfsolver_type,
    pub fdf: *mut gsl_function_fdf,
    pub root: libc::c_double,
    pub state: *mut libc::c_void,
}
pub unsafe extern "C" fn gsl_root_fdfsolver_alloc(
    mut T: *const gsl_root_fdfsolver_type,
) -> *mut gsl_root_fdfsolver {
    let mut s: *mut gsl_root_fdfsolver = malloc(
        ::std::mem::size_of::<gsl_root_fdfsolver>() as libc::c_ulong,
    ) as *mut gsl_root_fdfsolver;
    if s.is_null() {
        gsl_error(
            b"failed to allocate space for root solver struct\0" as *const u8
                as *const libc::c_char,
            b"fdfsolver.c\0" as *const u8 as *const libc::c_char,
            35 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_root_fdfsolver;
    }
    (*s).state = malloc((*T).size);
    if ((*s).state).is_null() {
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for root solver state\0" as *const u8
                as *const libc::c_char,
            b"fdfsolver.c\0" as *const u8 as *const libc::c_char,
            45 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_root_fdfsolver;
    }
    (*s).type_0 = T;
    (*s).fdf = 0 as *mut gsl_function_fdf;
    return s;
}
pub unsafe extern "C" fn gsl_root_fdfsolver_set(
    mut s: *mut gsl_root_fdfsolver,
    mut f: *mut gsl_function_fdf,
    mut root: libc::c_double,
) -> libc::c_int {
    (*s).fdf = f;
    (*s).root = root;
    return ((*(*s).type_0).set).unwrap()((*s).state, (*s).fdf, &mut (*s).root);
}
pub unsafe extern "C" fn gsl_root_fdfsolver_iterate(
    mut s: *mut gsl_root_fdfsolver,
) -> libc::c_int {
    return ((*(*s).type_0).iterate).unwrap()((*s).state, (*s).fdf, &mut (*s).root);
}
pub unsafe extern "C" fn gsl_root_fdfsolver_free(mut s: *mut gsl_root_fdfsolver) {
    if s.is_null() {
        return;
    }
    free((*s).state);
    free(s as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_root_fdfsolver_name(
    mut s: *const gsl_root_fdfsolver,
) -> *const libc::c_char {
    return (*(*s).type_0).name;
}
pub unsafe extern "C" fn gsl_root_fdfsolver_root(
    mut s: *const gsl_root_fdfsolver,
) -> libc::c_double {
    return (*s).root;
}
