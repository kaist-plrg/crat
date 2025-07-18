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
pub struct gsl_odeiv_system {
    pub function: Option::<
        unsafe extern "C" fn(
            libc::c_double,
            *const libc::c_double,
            *mut libc::c_double,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub jacobian: Option::<
        unsafe extern "C" fn(
            libc::c_double,
            *const libc::c_double,
            *mut libc::c_double,
            *mut libc::c_double,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub dimension: size_t,
    pub params: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_odeiv_step_type {
    pub name: *const libc::c_char,
    pub can_use_dydt_in: libc::c_int,
    pub gives_exact_dydt_out: libc::c_int,
    pub alloc: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub apply: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            size_t,
            libc::c_double,
            libc::c_double,
            *mut libc::c_double,
            *mut libc::c_double,
            *const libc::c_double,
            *mut libc::c_double,
            *const gsl_odeiv_system,
        ) -> libc::c_int,
    >,
    pub reset: Option::<unsafe extern "C" fn(*mut libc::c_void, size_t) -> libc::c_int>,
    pub order: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint>,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_odeiv_step {
    pub type_0: *const gsl_odeiv_step_type,
    pub dimension: size_t,
    pub state: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_odeiv_control_type {
    pub name: *const libc::c_char,
    pub alloc: Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
    pub init: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_double,
            libc::c_double,
            libc::c_double,
            libc::c_double,
        ) -> libc::c_int,
    >,
    pub hadjust: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            size_t,
            libc::c_uint,
            *const libc::c_double,
            *const libc::c_double,
            *const libc::c_double,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_odeiv_control {
    pub type_0: *const gsl_odeiv_control_type,
    pub state: *mut libc::c_void,
}
pub unsafe extern "C" fn gsl_odeiv_control_alloc(
    mut T: *const gsl_odeiv_control_type,
) -> *mut gsl_odeiv_control {
    let mut c: *mut gsl_odeiv_control = malloc(
        ::std::mem::size_of::<gsl_odeiv_control>() as libc::c_ulong,
    ) as *mut gsl_odeiv_control;
    if c.is_null() {
        gsl_error(
            b"failed to allocate space for control struct\0" as *const u8
                as *const libc::c_char,
            b"control.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_odeiv_control;
    }
    (*c).type_0 = T;
    (*c).state = ((*(*c).type_0).alloc).unwrap()();
    if ((*c).state).is_null() {
        free(c as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for control state\0" as *const u8
                as *const libc::c_char,
            b"control.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_odeiv_control;
    }
    return c;
}
pub unsafe extern "C" fn gsl_odeiv_control_init(
    mut c: *mut gsl_odeiv_control,
    mut eps_abs: libc::c_double,
    mut eps_rel: libc::c_double,
    mut a_y: libc::c_double,
    mut a_dydt: libc::c_double,
) -> libc::c_int {
    return ((*(*c).type_0).init).unwrap()((*c).state, eps_abs, eps_rel, a_y, a_dydt);
}
pub unsafe extern "C" fn gsl_odeiv_control_free(mut c: *mut gsl_odeiv_control) {
    if c.is_null() {
        return;
    }
    ((*(*c).type_0).free).unwrap()((*c).state);
    free(c as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_odeiv_control_name(
    mut c: *const gsl_odeiv_control,
) -> *const libc::c_char {
    return (*(*c).type_0).name;
}
pub unsafe extern "C" fn gsl_odeiv_control_hadjust(
    mut c: *mut gsl_odeiv_control,
    mut s: *mut gsl_odeiv_step,
    mut y: *const libc::c_double,
    mut yerr: *const libc::c_double,
    mut dydt: *const libc::c_double,
    mut h: *mut libc::c_double,
) -> libc::c_int {
    return ((*(*c).type_0).hadjust)
        .unwrap()(
        (*c).state,
        (*s).dimension,
        ((*(*s).type_0).order).unwrap()((*s).state),
        y,
        yerr,
        dydt,
        h,
    );
}
