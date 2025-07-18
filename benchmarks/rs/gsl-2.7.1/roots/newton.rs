use ::libc;
extern "C" {
    fn gsl_finite(x: libc::c_double) -> libc::c_int;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
}
pub type size_t = libc::c_ulong;
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
pub struct newton_state_t {
    pub f: libc::c_double,
    pub df: libc::c_double,
}
unsafe extern "C" fn newton_init(
    mut vstate: *mut libc::c_void,
    mut fdf: *mut gsl_function_fdf,
    mut root: *mut libc::c_double,
) -> libc::c_int {
    let mut state: *mut newton_state_t = vstate as *mut newton_state_t;
    let x: libc::c_double = *root;
    (*state).f = (Some(((*fdf).f).unwrap())).unwrap()(x, (*fdf).params);
    (*state).df = (Some(((*fdf).df).unwrap())).unwrap()(x, (*fdf).params);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn newton_iterate(
    mut vstate: *mut libc::c_void,
    mut fdf: *mut gsl_function_fdf,
    mut root: *mut libc::c_double,
) -> libc::c_int {
    let mut state: *mut newton_state_t = vstate as *mut newton_state_t;
    let mut root_new: libc::c_double = 0.;
    let mut f_new: libc::c_double = 0.;
    let mut df_new: libc::c_double = 0.;
    if (*state).df == 0.0f64 {
        gsl_error(
            b"derivative is zero\0" as *const u8 as *const libc::c_char,
            b"newton.c\0" as *const u8 as *const libc::c_char,
            74 as libc::c_int,
            GSL_EZERODIV as libc::c_int,
        );
        return GSL_EZERODIV as libc::c_int;
    }
    root_new = *root - (*state).f / (*state).df;
    *root = root_new;
    (Some(((*fdf).fdf).unwrap()))
        .unwrap()(root_new, (*fdf).params, &mut f_new, &mut df_new);
    (*state).f = f_new;
    (*state).df = df_new;
    if gsl_finite(f_new) == 0 {
        gsl_error(
            b"function value is not finite\0" as *const u8 as *const libc::c_char,
            b"newton.c\0" as *const u8 as *const libc::c_char,
            88 as libc::c_int,
            GSL_EBADFUNC as libc::c_int,
        );
        return GSL_EBADFUNC as libc::c_int;
    }
    if gsl_finite(df_new) == 0 {
        gsl_error(
            b"derivative value is not finite\0" as *const u8 as *const libc::c_char,
            b"newton.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int,
            GSL_EBADFUNC as libc::c_int,
        );
        return GSL_EBADFUNC as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
static mut newton_type: gsl_root_fdfsolver_type = {
    let mut init = gsl_root_fdfsolver_type {
        name: b"newton\0" as *const u8 as *const libc::c_char,
        size: ::std::mem::size_of::<newton_state_t>() as libc::c_ulong,
        set: Some(
            newton_init
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut gsl_function_fdf,
                    *mut libc::c_double,
                ) -> libc::c_int,
        ),
        iterate: Some(
            newton_iterate
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut gsl_function_fdf,
                    *mut libc::c_double,
                ) -> libc::c_int,
        ),
    };
    init
};
pub static mut gsl_root_fdfsolver_newton: *const gsl_root_fdfsolver_type = unsafe {
    &newton_type as *const gsl_root_fdfsolver_type
};
