use ::libc;
extern "C" {
    fn gsl_matrix_set_zero(m: *mut gsl_matrix);
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
pub struct gsl_multilarge_nlinear_solver {
    pub name: *const libc::c_char,
    pub alloc: Option::<unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void>,
    pub init: Option::<
        unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> libc::c_int,
    >,
    pub presolve: Option::<
        unsafe extern "C" fn(
            libc::c_double,
            *const libc::c_void,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub solve: Option::<
        unsafe extern "C" fn(
            *const gsl_vector,
            *mut gsl_vector,
            *const libc::c_void,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub rcond: Option::<
        unsafe extern "C" fn(
            *mut libc::c_double,
            *const gsl_matrix,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub covar: Option::<
        unsafe extern "C" fn(
            *const gsl_matrix,
            *mut gsl_matrix,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
unsafe extern "C" fn dummy_alloc(n: size_t, p: size_t) -> *mut libc::c_void {
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn dummy_free(mut vstate: *mut libc::c_void) {}
unsafe extern "C" fn dummy_init(
    mut vtrust_state: *const libc::c_void,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn dummy_presolve(
    mu: libc::c_double,
    mut vtrust_state: *const libc::c_void,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn dummy_solve(
    mut g: *const gsl_vector,
    mut x: *mut gsl_vector,
    mut vtrust_state: *const libc::c_void,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn dummy_rcond(
    mut rcond: *mut libc::c_double,
    mut JTJ: *const gsl_matrix,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    *rcond = 0.0f64;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn dummy_covar(
    mut JTJ: *const gsl_matrix,
    mut covar: *mut gsl_matrix,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    gsl_matrix_set_zero(covar);
    return GSL_SUCCESS as libc::c_int;
}
static mut dummy_type: gsl_multilarge_nlinear_solver = unsafe {
    {
        let mut init = gsl_multilarge_nlinear_solver {
            name: b"dummy\0" as *const u8 as *const libc::c_char,
            alloc: Some(
                dummy_alloc as unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void,
            ),
            init: Some(
                dummy_init
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            presolve: Some(
                dummy_presolve
                    as unsafe extern "C" fn(
                        libc::c_double,
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            solve: Some(
                dummy_solve
                    as unsafe extern "C" fn(
                        *const gsl_vector,
                        *mut gsl_vector,
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            rcond: Some(
                dummy_rcond
                    as unsafe extern "C" fn(
                        *mut libc::c_double,
                        *const gsl_matrix,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            covar: Some(
                dummy_covar
                    as unsafe extern "C" fn(
                        *const gsl_matrix,
                        *mut gsl_matrix,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            free: Some(dummy_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        };
        init
    }
};
pub static mut gsl_multilarge_nlinear_solver_none: *const gsl_multilarge_nlinear_solver = unsafe {
    &dummy_type as *const gsl_multilarge_nlinear_solver
};
