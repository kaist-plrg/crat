use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_odeiv2_control_errlevel(
        c: *mut gsl_odeiv2_control,
        y: libc::c_double,
        dydt: libc::c_double,
        h: libc::c_double,
        ind: size_t,
        errlev: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_vector_alloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_set_zero(v: *mut gsl_vector);
    fn gsl_blas_dnrm2(X: *const gsl_vector) -> libc::c_double;
    fn gsl_vector_free(v: *mut gsl_vector);
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
pub struct gsl_odeiv2_system {
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
pub struct gsl_odeiv2_step_struct {
    pub type_0: *const gsl_odeiv2_step_type,
    pub dimension: size_t,
    pub state: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_odeiv2_step_type {
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
            *const gsl_odeiv2_system,
        ) -> libc::c_int,
    >,
    pub set_driver: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const gsl_odeiv2_driver) -> libc::c_int,
    >,
    pub reset: Option::<unsafe extern "C" fn(*mut libc::c_void, size_t) -> libc::c_int>,
    pub order: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint>,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
pub type gsl_odeiv2_driver = gsl_odeiv2_driver_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_odeiv2_driver_struct {
    pub sys: *const gsl_odeiv2_system,
    pub s: *mut gsl_odeiv2_step,
    pub c: *mut gsl_odeiv2_control,
    pub e: *mut gsl_odeiv2_evolve,
    pub h: libc::c_double,
    pub hmin: libc::c_double,
    pub hmax: libc::c_double,
    pub n: libc::c_ulong,
    pub nmax: libc::c_ulong,
}
pub type gsl_odeiv2_evolve = gsl_odeiv2_evolve_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_odeiv2_evolve_struct {
    pub dimension: size_t,
    pub y0: *mut libc::c_double,
    pub yerr: *mut libc::c_double,
    pub dydt_in: *mut libc::c_double,
    pub dydt_out: *mut libc::c_double,
    pub last_step: libc::c_double,
    pub count: libc::c_ulong,
    pub failed_steps: libc::c_ulong,
    pub driver: *const gsl_odeiv2_driver,
}
pub type gsl_odeiv2_control = gsl_odeiv2_control_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_odeiv2_control_struct {
    pub type_0: *const gsl_odeiv2_control_type,
    pub state: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_odeiv2_control_type {
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
    pub errlevel: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_double,
            libc::c_double,
            libc::c_double,
            size_t,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    pub set_driver: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const gsl_odeiv2_driver) -> libc::c_int,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
pub type gsl_odeiv2_step = gsl_odeiv2_step_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msadams_state_t {
    pub z: *mut libc::c_double,
    pub zbackup: *mut libc::c_double,
    pub ytmp: *mut libc::c_double,
    pub ytmp2: *mut libc::c_double,
    pub pc: *mut libc::c_double,
    pub l: *mut libc::c_double,
    pub hprev: *mut libc::c_double,
    pub hprevbackup: *mut libc::c_double,
    pub errlev: *mut libc::c_double,
    pub abscor: *mut gsl_vector,
    pub relcor: *mut gsl_vector,
    pub svec: *mut gsl_vector,
    pub tempvec: *mut gsl_vector,
    pub driver: *const gsl_odeiv2_driver,
    pub ni: libc::c_long,
    pub ord: size_t,
    pub ordprev: size_t,
    pub ordprevbackup: size_t,
    pub tprev: libc::c_double,
    pub ordwait: size_t,
    pub ordwaitbackup: size_t,
    pub failord: size_t,
    pub failt: libc::c_double,
    pub ordm1coeff: libc::c_double,
    pub ordp1coeffprev: libc::c_double,
    pub failcount: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: libc::c_int,
}
pub type gsl_block = gsl_block_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
}
#[inline]
unsafe extern "C" fn GSL_MAX_DBL(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn GSL_MIN_DBL(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return if a < b { a } else { b };
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
unsafe extern "C" fn gsl_vector_get(
    mut v: *const gsl_vector,
    i: size_t,
) -> libc::c_double {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
unsafe extern "C" fn msadams_alloc(mut dim: size_t) -> *mut libc::c_void {
    let mut state: *mut msadams_state_t = malloc(
        ::std::mem::size_of::<msadams_state_t>() as libc::c_ulong,
    ) as *mut msadams_state_t;
    if state.is_null() {
        gsl_error(
            b"failed to allocate space for msadams_state\0" as *const u8
                as *const libc::c_char,
            b"msadams.c\0" as *const u8 as *const libc::c_char,
            108 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .z = malloc(
        ((12 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(dim)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).z).is_null() {
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for z\0" as *const u8 as *const libc::c_char,
            b"msadams.c\0" as *const u8 as *const libc::c_char,
            117 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .zbackup = malloc(
        ((12 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(dim)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).zbackup).is_null() {
        free((*state).z as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for zbackup\0" as *const u8
                as *const libc::c_char,
            b"msadams.c\0" as *const u8 as *const libc::c_char,
            127 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .ytmp = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).ytmp).is_null() {
        free((*state).zbackup as *mut libc::c_void);
        free((*state).z as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for ytmp\0" as *const u8 as *const libc::c_char,
            b"msadams.c\0" as *const u8 as *const libc::c_char,
            137 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .ytmp2 = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).ytmp2).is_null() {
        free((*state).ytmp as *mut libc::c_void);
        free((*state).zbackup as *mut libc::c_void);
        free((*state).z as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for ytmp2\0" as *const u8 as *const libc::c_char,
            b"msadams.c\0" as *const u8 as *const libc::c_char,
            148 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .pc = malloc(
        ((12 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).pc).is_null() {
        free((*state).ytmp2 as *mut libc::c_void);
        free((*state).ytmp as *mut libc::c_void);
        free((*state).zbackup as *mut libc::c_void);
        free((*state).z as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for pc\0" as *const u8 as *const libc::c_char,
            b"msadams.c\0" as *const u8 as *const libc::c_char,
            160 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .l = malloc(
        ((12 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).l).is_null() {
        free((*state).pc as *mut libc::c_void);
        free((*state).ytmp as *mut libc::c_void);
        free((*state).zbackup as *mut libc::c_void);
        free((*state).z as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for l\0" as *const u8 as *const libc::c_char,
            b"msadams.c\0" as *const u8 as *const libc::c_char,
            172 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .hprev = malloc(
        (12 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).hprev).is_null() {
        free((*state).l as *mut libc::c_void);
        free((*state).pc as *mut libc::c_void);
        free((*state).ytmp2 as *mut libc::c_void);
        free((*state).ytmp as *mut libc::c_void);
        free((*state).zbackup as *mut libc::c_void);
        free((*state).z as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for hprev\0" as *const u8 as *const libc::c_char,
            b"msadams.c\0" as *const u8 as *const libc::c_char,
            186 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .hprevbackup = malloc(
        (12 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).hprevbackup).is_null() {
        free((*state).hprev as *mut libc::c_void);
        free((*state).l as *mut libc::c_void);
        free((*state).pc as *mut libc::c_void);
        free((*state).ytmp2 as *mut libc::c_void);
        free((*state).ytmp as *mut libc::c_void);
        free((*state).zbackup as *mut libc::c_void);
        free((*state).z as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for hprevbackup\0" as *const u8
                as *const libc::c_char,
            b"msadams.c\0" as *const u8 as *const libc::c_char,
            201 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .errlev = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).errlev).is_null() {
        free((*state).hprevbackup as *mut libc::c_void);
        free((*state).hprev as *mut libc::c_void);
        free((*state).l as *mut libc::c_void);
        free((*state).pc as *mut libc::c_void);
        free((*state).ytmp2 as *mut libc::c_void);
        free((*state).ytmp as *mut libc::c_void);
        free((*state).zbackup as *mut libc::c_void);
        free((*state).z as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for errlev\0" as *const u8 as *const libc::c_char,
            b"msadams.c\0" as *const u8 as *const libc::c_char,
            217 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).abscor = gsl_vector_alloc(dim);
    if ((*state).abscor).is_null() {
        free((*state).errlev as *mut libc::c_void);
        free((*state).hprevbackup as *mut libc::c_void);
        free((*state).hprev as *mut libc::c_void);
        free((*state).l as *mut libc::c_void);
        free((*state).pc as *mut libc::c_void);
        free((*state).ytmp2 as *mut libc::c_void);
        free((*state).ytmp as *mut libc::c_void);
        free((*state).zbackup as *mut libc::c_void);
        free((*state).z as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for abscor\0" as *const u8 as *const libc::c_char,
            b"msadams.c\0" as *const u8 as *const libc::c_char,
            234 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).relcor = gsl_vector_alloc(dim);
    if ((*state).relcor).is_null() {
        gsl_vector_free((*state).abscor);
        free((*state).errlev as *mut libc::c_void);
        free((*state).hprevbackup as *mut libc::c_void);
        free((*state).hprev as *mut libc::c_void);
        free((*state).l as *mut libc::c_void);
        free((*state).pc as *mut libc::c_void);
        free((*state).ytmp2 as *mut libc::c_void);
        free((*state).ytmp as *mut libc::c_void);
        free((*state).zbackup as *mut libc::c_void);
        free((*state).z as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for relcor\0" as *const u8 as *const libc::c_char,
            b"msadams.c\0" as *const u8 as *const libc::c_char,
            252 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).svec = gsl_vector_alloc(dim);
    if ((*state).svec).is_null() {
        gsl_vector_free((*state).relcor);
        gsl_vector_free((*state).abscor);
        free((*state).errlev as *mut libc::c_void);
        free((*state).hprevbackup as *mut libc::c_void);
        free((*state).hprev as *mut libc::c_void);
        free((*state).l as *mut libc::c_void);
        free((*state).pc as *mut libc::c_void);
        free((*state).ytmp2 as *mut libc::c_void);
        free((*state).ytmp as *mut libc::c_void);
        free((*state).zbackup as *mut libc::c_void);
        free((*state).z as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for svec\0" as *const u8 as *const libc::c_char,
            b"msadams.c\0" as *const u8 as *const libc::c_char,
            271 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).tempvec = gsl_vector_alloc(dim);
    if ((*state).tempvec).is_null() {
        gsl_vector_free((*state).svec);
        gsl_vector_free((*state).relcor);
        gsl_vector_free((*state).abscor);
        free((*state).errlev as *mut libc::c_void);
        free((*state).hprevbackup as *mut libc::c_void);
        free((*state).hprev as *mut libc::c_void);
        free((*state).l as *mut libc::c_void);
        free((*state).pc as *mut libc::c_void);
        free((*state).ytmp2 as *mut libc::c_void);
        free((*state).ytmp as *mut libc::c_void);
        free((*state).zbackup as *mut libc::c_void);
        free((*state).z as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for tempvec\0" as *const u8
                as *const libc::c_char,
            b"msadams.c\0" as *const u8 as *const libc::c_char,
            291 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    msadams_reset(state as *mut libc::c_void, dim);
    (*state).driver = 0 as *const gsl_odeiv2_driver;
    return state as *mut libc::c_void;
}
unsafe extern "C" fn msadams_failurehandler(
    mut vstate: *mut libc::c_void,
    dim: size_t,
    t: libc::c_double,
) -> libc::c_int {
    let mut state: *mut msadams_state_t = vstate as *mut msadams_state_t;
    let ord: size_t = (*state).ord;
    if ord > 1 as libc::c_int as libc::c_ulong
        && ord.wrapping_sub((*state).ordprev) == 0 as libc::c_int as libc::c_ulong
        && ord == (*state).failord && t == (*state).failt
    {
        (*state).ord = ((*state).ord).wrapping_sub(1);
        (*state).ord;
    }
    (*state).failord = ord;
    (*state).failt = t;
    (*state).ni += 1;
    (*state).ni;
    if ord == 1 as libc::c_int as libc::c_ulong {
        msadams_reset(vstate, dim);
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn msadams_calccoeffs(
    ord: size_t,
    ordwait: size_t,
    h: libc::c_double,
    mut hprev: *const libc::c_double,
    mut pc: *mut libc::c_double,
    mut l: *mut libc::c_double,
    mut errcoeff: *mut libc::c_double,
    mut ordm1coeff: *mut libc::c_double,
    mut ordp1coeff: *mut libc::c_double,
    mut ordp2coeff: *mut libc::c_double,
) -> libc::c_int {
    if ord == 1 as libc::c_int as libc::c_ulong {
        *l.offset(0 as libc::c_int as isize) = 1.0f64;
        *l.offset(1 as libc::c_int as isize) = 1.0f64;
        *errcoeff = 0.5f64;
        *ordp1coeff = 1.0f64;
        *ordp2coeff = 12.0f64;
    } else {
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut hsum: libc::c_double = h;
        let mut st1: libc::c_double = 0.0f64;
        let mut st2: libc::c_double = 0.0f64;
        memset(
            pc as *mut libc::c_void,
            0 as libc::c_int,
            ((12 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
        *pc.offset(0 as libc::c_int as isize) = 1.0f64;
        i = 1 as libc::c_int as size_t;
        while i < ord {
            if i == ord.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                && ordwait == 1 as libc::c_int as libc::c_ulong
            {
                let mut s: libc::c_int = 1 as libc::c_int;
                *ordm1coeff = 0.0f64;
                j = 0 as libc::c_int as size_t;
                while j < ord.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                    *ordm1coeff
                        += s as libc::c_double * *pc.offset(j as isize)
                            / j.wrapping_add(2 as libc::c_int as libc::c_ulong)
                                as libc::c_double;
                    s = -s;
                    j = j.wrapping_add(1);
                    j;
                }
                *ordm1coeff = *pc
                    .offset(ord.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
                    / (ord as libc::c_double * *ordm1coeff);
            }
            j = i;
            while j > 0 as libc::c_int as libc::c_ulong {
                *pc.offset(j as isize)
                    += *pc
                        .offset(
                            j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ) * h / hsum;
                j = j.wrapping_sub(1);
                j;
            }
            hsum
                += *hprev
                    .offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
            i = i.wrapping_add(1);
            i;
        }
        let mut s_0: libc::c_int = 1 as libc::c_int;
        i = 0 as libc::c_int as size_t;
        while i < ord {
            st1
                += s_0 as libc::c_double * *pc.offset(i as isize)
                    / (i as libc::c_double + 1.0f64);
            s_0 = -s_0;
            i = i.wrapping_add(1);
            i;
        }
        let mut s_1: libc::c_int = 1 as libc::c_int;
        i = 0 as libc::c_int as size_t;
        while i < ord {
            st2
                += s_1 as libc::c_double * *pc.offset(i as isize)
                    / (i as libc::c_double + 2.0f64);
            s_1 = -s_1;
            i = i.wrapping_add(1);
            i;
        }
        memset(
            l as *mut libc::c_void,
            0 as libc::c_int,
            ((12 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
        *l.offset(0 as libc::c_int as isize) = 1.0f64;
        i = 1 as libc::c_int as size_t;
        while i < ord.wrapping_add(1 as libc::c_int as libc::c_ulong) {
            *l
                .offset(
                    i as isize,
                ) = *pc
                .offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                / (i as libc::c_double * st1);
            i = i.wrapping_add(1);
            i;
        }
        *errcoeff = h / hsum * (st2 / st1);
        if ordwait < 2 as libc::c_int as libc::c_ulong {
            let mut s_2: libc::c_int = 1 as libc::c_int;
            *ordp1coeff = hsum / (h * *l.offset(ord as isize));
            *ordp2coeff = 0.0f64;
            i = ord;
            while i > 0 as libc::c_int as libc::c_ulong {
                *pc.offset(i as isize)
                    += *pc
                        .offset(
                            i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ) * (h / hsum);
                i = i.wrapping_sub(1);
                i;
            }
            i = 0 as libc::c_int as size_t;
            while i < ord.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *ordp2coeff
                    += s_2 as libc::c_double * *pc.offset(i as isize)
                        / i.wrapping_add(2 as libc::c_int as libc::c_ulong)
                            as libc::c_double;
                s_2 = -s_2;
                i = i.wrapping_add(1);
                i;
            }
            *ordp2coeff = ord.wrapping_add(1 as libc::c_int as libc::c_ulong)
                as libc::c_double * st1 / *ordp2coeff;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn msadams_corrector(
    mut vstate: *mut libc::c_void,
    mut sys: *const gsl_odeiv2_system,
    t: libc::c_double,
    h: libc::c_double,
    dim: size_t,
    mut z: *const libc::c_double,
    mut errlev: *const libc::c_double,
    mut l: *const libc::c_double,
    errcoeff: libc::c_double,
    mut abscor: *mut gsl_vector,
    mut relcor: *mut gsl_vector,
    mut ytmp: *mut libc::c_double,
    mut ytmp2: *mut libc::c_double,
) -> libc::c_int {
    let mut mi: size_t = 0;
    let mut i: size_t = 0;
    let max_iter: size_t = 3 as libc::c_int as size_t;
    let mut convrate: libc::c_double = 1.0f64;
    let mut stepnorm: libc::c_double = 0.0f64;
    let mut stepnormprev: libc::c_double = 0.0f64;
    let mut s: libc::c_int = (Some(((*sys).function).unwrap()))
        .unwrap()(t + h, z, ytmp, (*sys).params);
    if s == GSL_EBADFUNC as libc::c_int {
        return s;
    }
    if s != GSL_SUCCESS as libc::c_int {
        msadams_failurehandler(vstate, dim, t);
        return s;
    }
    gsl_vector_set_zero(abscor);
    mi = 0 as libc::c_int as size_t;
    while mi < max_iter {
        let safety: libc::c_double = 0.3f64;
        let safety2: libc::c_double = 0.1f64;
        i = 0 as libc::c_int as size_t;
        while i < dim {
            *ytmp.offset(i as isize) *= h;
            *ytmp.offset(i as isize)
                -= *z
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(dim)
                            .wrapping_add(i) as isize,
                    );
            *ytmp.offset(i as isize) /= *l.offset(1 as libc::c_int as isize);
            *ytmp2
                .offset(
                    i as isize,
                ) = *z
                .offset(
                    (0 as libc::c_int as libc::c_ulong).wrapping_mul(dim).wrapping_add(i)
                        as isize,
                ) + *ytmp.offset(i as isize);
            i = i.wrapping_add(1);
            i;
        }
        i = 0 as libc::c_int as size_t;
        while i < dim {
            gsl_vector_set(
                relcor,
                i,
                (*ytmp.offset(i as isize) - gsl_vector_get(abscor, i))
                    / *errlev.offset(i as isize),
            );
            gsl_vector_set(abscor, i, *ytmp.offset(i as isize));
            i = i.wrapping_add(1);
            i;
        }
        stepnorm = gsl_blas_dnrm2(relcor) / sqrt(dim as libc::c_double);
        if mi > 0 as libc::c_int as libc::c_ulong {
            convrate = GSL_MAX_DBL(safety * convrate, stepnorm / stepnormprev);
        } else {
            convrate = 1.0f64;
        }
        let convtest: libc::c_double = GSL_MIN_DBL(convrate, 1.0f64) * stepnorm
            * errcoeff / safety2;
        if convtest <= 1.0f64 {
            break;
        }
        let div_const: libc::c_double = 2.0f64;
        if mi > 1 as libc::c_int as libc::c_ulong && stepnorm > div_const * stepnormprev
        {
            msadams_failurehandler(vstate, dim, t);
            return GSL_FAILURE as libc::c_int;
        }
        let mut s_0: libc::c_int = (Some(((*sys).function).unwrap()))
            .unwrap()(t + h, ytmp2 as *const libc::c_double, ytmp, (*sys).params);
        if s_0 == GSL_EBADFUNC as libc::c_int {
            return s_0;
        }
        if s_0 != GSL_SUCCESS as libc::c_int {
            msadams_failurehandler(vstate, dim, t);
            return s_0;
        }
        stepnormprev = stepnorm;
        mi = mi.wrapping_add(1);
        mi;
    }
    if mi == max_iter {
        msadams_failurehandler(vstate, dim, t);
        return GSL_FAILURE as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn msadams_eval_order(
    mut abscor: *mut gsl_vector,
    mut tempvec: *mut gsl_vector,
    mut svec: *mut gsl_vector,
    errcoeff: libc::c_double,
    dim: size_t,
    mut errlev: *const libc::c_double,
    ordm1coeff: libc::c_double,
    ordp1coeff: libc::c_double,
    ordp1coeffprev: libc::c_double,
    ordp2coeff: libc::c_double,
    mut hprev: *const libc::c_double,
    h: libc::c_double,
    mut z: *const libc::c_double,
    mut ord: *mut size_t,
    mut ordwait: *mut size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut ordest: libc::c_double = 0.0f64;
    let mut ordm1est: libc::c_double = 0.0f64;
    let mut ordp1est: libc::c_double = 0.0f64;
    let safety: libc::c_double = 1e-6f64;
    let bias: libc::c_double = 6.0f64;
    let bias2: libc::c_double = 10.0f64;
    ordest = 1.0f64
        / (pow(
            bias * gsl_blas_dnrm2(abscor) / sqrt(dim as libc::c_double) * errcoeff,
            1.0f64
                / (*ord).wrapping_add(1 as libc::c_int as libc::c_ulong)
                    as libc::c_double,
        ) + safety);
    if *ord > 1 as libc::c_int as libc::c_ulong {
        i = 0 as libc::c_int as size_t;
        while i < dim {
            gsl_vector_set(
                tempvec,
                i,
                *z.offset((*ord).wrapping_mul(dim).wrapping_add(i) as isize)
                    / *errlev.offset(i as isize),
            );
            i = i.wrapping_add(1);
            i;
        }
        ordm1est = 1.0f64
            / (pow(
                bias * gsl_blas_dnrm2(tempvec) / sqrt(dim as libc::c_double)
                    / ordm1coeff,
                1.0f64 / *ord as libc::c_double,
            ) + safety);
    } else {
        ordm1est = 0.0f64;
    }
    if *ord < 12 as libc::c_int as libc::c_ulong {
        let c: libc::c_double = -ordp1coeff / ordp1coeffprev
            * pow(
                h / *hprev.offset(1 as libc::c_int as isize),
                (*ord).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_double,
            );
        i = 0 as libc::c_int as size_t;
        while i < dim {
            gsl_vector_set(
                svec,
                i,
                gsl_vector_get(svec, i) * c + gsl_vector_get(abscor, i),
            );
            i = i.wrapping_add(1);
            i;
        }
        ordp1est = 1.0f64
            / (pow(
                bias2 * gsl_blas_dnrm2(svec) / sqrt(dim as libc::c_double) / ordp2coeff,
                1.0f64
                    / (*ord).wrapping_add(2 as libc::c_int as libc::c_ulong)
                        as libc::c_double,
            ) + safety);
    } else {
        ordp1est = 0.0f64;
    }
    let min_incr: libc::c_double = 1.5f64;
    if ordm1est > ordest && ordm1est > ordp1est && ordm1est > min_incr {
        *ord = (*ord as libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    } else if ordp1est > ordest && ordp1est > ordm1est && ordp1est > min_incr {
        *ord = (*ord as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    *ordwait = (*ord).wrapping_add(2 as libc::c_int as libc::c_ulong);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn msadams_apply(
    mut vstate: *mut libc::c_void,
    mut dim: size_t,
    mut t: libc::c_double,
    mut h: libc::c_double,
    mut y: *mut libc::c_double,
    mut yerr: *mut libc::c_double,
    mut dydt_in: *const libc::c_double,
    mut dydt_out: *mut libc::c_double,
    mut sys: *const gsl_odeiv2_system,
) -> libc::c_int {
    let mut state: *mut msadams_state_t = vstate as *mut msadams_state_t;
    let z: *mut libc::c_double = (*state).z;
    let zbackup: *mut libc::c_double = (*state).zbackup;
    let ytmp: *mut libc::c_double = (*state).ytmp;
    let ytmp2: *mut libc::c_double = (*state).ytmp2;
    let pc: *mut libc::c_double = (*state).pc;
    let l: *mut libc::c_double = (*state).l;
    let hprev: *mut libc::c_double = (*state).hprev;
    let hprevbackup: *mut libc::c_double = (*state).hprevbackup;
    let errlev: *mut libc::c_double = (*state).errlev;
    let abscor: *mut gsl_vector = (*state).abscor;
    let relcor: *mut gsl_vector = (*state).relcor;
    let svec: *mut gsl_vector = (*state).svec;
    let tempvec: *mut gsl_vector = (*state).tempvec;
    let mut ord: size_t = (*state).ord;
    let mut ordm1coeff: libc::c_double = 0.0f64;
    let mut ordp1coeff: libc::c_double = 0.0f64;
    let mut ordp2coeff: libc::c_double = 0.0f64;
    let mut errcoeff: libc::c_double = 0.0f64;
    let mut deltaord: libc::c_int = 0;
    if (*state).ni > 0 as libc::c_int as libc::c_long
        && (t == (*state).tprev || t == (*state).failt)
    {
        if (*state).ni == 1 as libc::c_int as libc::c_long {
            msadams_reset(vstate, dim);
        } else {
            if ord > (*state).ordprev {
                (*state).ord = (*state).ordprev;
                ord = (*state).ord;
            }
            memcpy(
                z as *mut libc::c_void,
                zbackup as *const libc::c_void,
                ((12 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(dim)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    ),
            );
            memcpy(
                hprev as *mut libc::c_void,
                hprevbackup as *const libc::c_void,
                (12 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    ),
            );
            (*state).ordprev = (*state).ordprevbackup;
            (*state).ordwait = (*state).ordwaitbackup;
        }
        (*state).failcount = ((*state).failcount).wrapping_add(1);
        (*state).failcount;
        let max_failcount: size_t = 3 as libc::c_int as size_t;
        if (*state).failcount > max_failcount
            && (*state).ni > 1 as libc::c_int as libc::c_long
        {
            msadams_reset(vstate, dim);
            ord = (*state).ord;
        } else if (*state).ordprev as libc::c_int - ord as libc::c_int
            >= 2 as libc::c_int
        {
            msadams_reset(vstate, dim);
            ord = (*state).ord;
        }
    } else {
        memcpy(
            zbackup as *mut libc::c_void,
            z as *const libc::c_void,
            ((12 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(dim)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
        memcpy(
            hprevbackup as *mut libc::c_void,
            hprev as *const libc::c_void,
            (12 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
        (*state).ordprevbackup = (*state).ordprev;
        (*state).ordwaitbackup = (*state).ordwait;
        (*state).failcount = 0 as libc::c_int as size_t;
    }
    if ((*state).driver).is_null() {
        return GSL_EFAULT as libc::c_int
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < dim {
            if !dydt_in.is_null() {
                gsl_odeiv2_control_errlevel(
                    (*(*state).driver).c,
                    *y.offset(i as isize),
                    *dydt_in.offset(i as isize),
                    h,
                    i,
                    &mut *errlev.offset(i as isize),
                );
            } else {
                gsl_odeiv2_control_errlevel(
                    (*(*state).driver).c,
                    *y.offset(i as isize),
                    0.0f64,
                    h,
                    i,
                    &mut *errlev.offset(i as isize),
                );
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    if (*state).ni == 0 as libc::c_int as libc::c_long {
        let mut i_0: size_t = 0;
        memset(
            z as *mut libc::c_void,
            0 as libc::c_int,
            ((12 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(dim)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
        if !dydt_in.is_null() {
            memcpy(
                ytmp as *mut libc::c_void,
                dydt_in as *const libc::c_void,
                dim
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    ),
            );
        } else {
            let mut s: libc::c_int = (Some(((*sys).function).unwrap()))
                .unwrap()(t, y as *const libc::c_double, ytmp, (*sys).params);
            if s != GSL_SUCCESS as libc::c_int {
                return s;
            }
        }
        memcpy(
            &mut *z
                .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(dim) as isize)
                as *mut libc::c_double as *mut libc::c_void,
            y as *const libc::c_void,
            dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
        memcpy(
            &mut *z
                .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(dim) as isize)
                as *mut libc::c_double as *mut libc::c_void,
            ytmp as *const libc::c_void,
            dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
        i_0 = 0 as libc::c_int as size_t;
        while i_0 < dim {
            *z
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dim)
                        .wrapping_add(i_0) as isize,
                ) *= h;
            i_0 = i_0.wrapping_add(1);
            i_0;
        }
    }
    deltaord = ord.wrapping_sub((*state).ordprev) as libc::c_int;
    if deltaord > 1 as libc::c_int || deltaord < -(1 as libc::c_int) {
        printf(b"-- order change %d\n\0" as *const u8 as *const libc::c_char, deltaord);
        gsl_error(
            b"msadams_apply too large order change\0" as *const u8
                as *const libc::c_char,
            b"msadams.c\0" as *const u8 as *const libc::c_char,
            986 as libc::c_int,
            GSL_ESANITY as libc::c_int,
        );
        return GSL_ESANITY as libc::c_int;
    }
    if deltaord == 1 as libc::c_int {
        memset(
            &mut *z.offset(ord.wrapping_mul(dim) as isize) as *mut libc::c_double
                as *mut libc::c_void,
            0 as libc::c_int,
            dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
    }
    if deltaord == -(1 as libc::c_int) {
        let mut hsum: libc::c_double = 0.0f64;
        let mut i_1: size_t = 0;
        let mut j: size_t = 0;
        memset(
            l as *mut libc::c_void,
            0 as libc::c_int,
            ((12 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
        *l.offset(1 as libc::c_int as isize) = 1.0f64;
        i_1 = 1 as libc::c_int as size_t;
        while i_1 < ord {
            hsum
                += *hprev
                    .offset(
                        i_1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    );
            j = i_1.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while j > 0 as libc::c_int as libc::c_ulong {
                *l.offset(j as isize) *= hsum / *hprev.offset(0 as libc::c_int as isize);
                *l.offset(j as isize)
                    += *l
                        .offset(
                            j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                j = j.wrapping_sub(1);
                j;
            }
            i_1 = i_1.wrapping_add(1);
            i_1;
        }
        i_1 = 1 as libc::c_int as size_t;
        while i_1 < ord {
            *l
                .offset(
                    i_1.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = ord.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_double
                * *l.offset(i_1 as isize)
                / i_1.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_double;
            i_1 = i_1.wrapping_add(1);
            i_1;
        }
        i_1 = 2 as libc::c_int as size_t;
        while i_1 < ord.wrapping_add(1 as libc::c_int as libc::c_ulong) {
            j = 0 as libc::c_int as size_t;
            while j < dim {
                *z.offset(i_1.wrapping_mul(dim).wrapping_add(j) as isize)
                    += -*l.offset(i_1 as isize)
                        * *z
                            .offset(
                                ord
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(dim)
                                    .wrapping_add(j) as isize,
                            );
                j = j.wrapping_add(1);
                j;
            }
            i_1 = i_1.wrapping_add(1);
            i_1;
        }
    }
    if (*state).ni > 0 as libc::c_int as libc::c_long
        && h != *hprev.offset(0 as libc::c_int as isize)
    {
        let mut i_2: size_t = 0;
        let mut j_0: size_t = 0;
        let hrel: libc::c_double = h / *hprev.offset(0 as libc::c_int as isize);
        let mut coeff: libc::c_double = hrel;
        i_2 = 1 as libc::c_int as size_t;
        while i_2 < ord.wrapping_add(1 as libc::c_int as libc::c_ulong) {
            j_0 = 0 as libc::c_int as size_t;
            while j_0 < dim {
                *z.offset(i_2.wrapping_mul(dim).wrapping_add(j_0) as isize) *= coeff;
                j_0 = j_0.wrapping_add(1);
                j_0;
            }
            coeff *= hrel;
            i_2 = i_2.wrapping_add(1);
            i_2;
        }
    }
    msadams_calccoeffs(
        ord,
        (*state).ordwait,
        h,
        hprev as *const libc::c_double,
        pc,
        l,
        &mut errcoeff,
        &mut ordm1coeff,
        &mut ordp1coeff,
        &mut ordp2coeff,
    );
    let mut i_3: size_t = 0;
    let mut j_1: size_t = 0;
    let mut k: size_t = 0;
    i_3 = 1 as libc::c_int as size_t;
    while i_3 < ord.wrapping_add(1 as libc::c_int as libc::c_ulong) {
        j_1 = ord;
        while j_1 > i_3.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            k = 0 as libc::c_int as size_t;
            while k < dim {
                *z
                    .offset(
                        j_1
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(dim)
                            .wrapping_add(k) as isize,
                    ) += *z.offset(j_1.wrapping_mul(dim).wrapping_add(k) as isize);
                k = k.wrapping_add(1);
                k;
            }
            j_1 = j_1.wrapping_sub(1);
            j_1;
        }
        i_3 = i_3.wrapping_add(1);
        i_3;
    }
    let mut s_0: libc::c_int = 0;
    s_0 = msadams_corrector(
        vstate,
        sys,
        t,
        h,
        dim,
        z as *const libc::c_double,
        errlev as *const libc::c_double,
        l as *const libc::c_double,
        errcoeff,
        abscor,
        relcor,
        ytmp,
        ytmp2,
    );
    if s_0 != GSL_SUCCESS as libc::c_int {
        return s_0;
    }
    let mut i_4: size_t = 0;
    let mut j_2: size_t = 0;
    i_4 = 0 as libc::c_int as size_t;
    while i_4 < ord.wrapping_add(1 as libc::c_int as libc::c_ulong) {
        j_2 = 0 as libc::c_int as size_t;
        while j_2 < dim {
            *z.offset(i_4.wrapping_mul(dim).wrapping_add(j_2) as isize)
                += *l.offset(i_4 as isize) * gsl_vector_get(abscor, j_2);
            j_2 = j_2.wrapping_add(1);
            j_2;
        }
        i_4 = i_4.wrapping_add(1);
        i_4;
    }
    if !dydt_out.is_null() {
        let mut s_1: libc::c_int = (Some(((*sys).function).unwrap()))
            .unwrap()(t + h, z as *const libc::c_double, dydt_out, (*sys).params);
        if s_1 == GSL_EBADFUNC as libc::c_int {
            return s_1;
        }
        if s_1 != GSL_SUCCESS as libc::c_int {
            msadams_failurehandler(vstate, dim, t);
            return s_1;
        }
    }
    i_4 = 0 as libc::c_int as size_t;
    while i_4 < dim {
        *yerr.offset(i_4 as isize) = fabs(gsl_vector_get(abscor, i_4)) * errcoeff;
        i_4 = i_4.wrapping_add(1);
        i_4;
    }
    i_4 = 0 as libc::c_int as size_t;
    while i_4 < dim {
        *y
            .offset(
                i_4 as isize,
            ) = *z
            .offset(
                (0 as libc::c_int as libc::c_ulong).wrapping_mul(dim).wrapping_add(i_4)
                    as isize,
            );
        i_4 = i_4.wrapping_add(1);
        i_4;
    }
    let mut i_5: size_t = 0;
    i_5 = 0 as libc::c_int as size_t;
    while i_5 < dim {
        gsl_vector_set(
            abscor,
            i_5,
            gsl_vector_get(abscor, i_5) / *errlev.offset(i_5 as isize),
        );
        i_5 = i_5.wrapping_add(1);
        i_5;
    }
    if (*state).ordwait == 1 as libc::c_int as libc::c_ulong
        && ord < 12 as libc::c_int as libc::c_ulong
    {
        let mut i_6: size_t = 0;
        (*state).ordp1coeffprev = ordp1coeff;
        (*state).ordm1coeff = ordm1coeff;
        i_6 = 0 as libc::c_int as size_t;
        while i_6 < dim {
            gsl_vector_set(svec, i_6, gsl_vector_get(abscor, i_6));
            i_6 = i_6.wrapping_add(1);
            i_6;
        }
    }
    if (*state).ordwait == 0 as libc::c_int as libc::c_ulong {
        msadams_eval_order(
            abscor,
            tempvec,
            svec,
            errcoeff,
            dim,
            errlev as *const libc::c_double,
            (*state).ordm1coeff,
            ordp1coeff,
            (*state).ordp1coeffprev,
            ordp2coeff,
            hprev as *const libc::c_double,
            h,
            z as *const libc::c_double,
            &mut (*state).ord,
            &mut (*state).ordwait,
        );
    }
    let mut i_7: size_t = 0;
    (*state).ordprev = ord;
    i_7 = (12 as libc::c_int - 1 as libc::c_int) as size_t;
    while i_7 > 0 as libc::c_int as libc::c_ulong {
        *hprev
            .offset(
                i_7 as isize,
            ) = *hprev
            .offset(i_7.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
        i_7 = i_7.wrapping_sub(1);
        i_7;
    }
    *hprev.offset(0 as libc::c_int as isize) = h;
    (*state).tprev = t;
    (*state).ordwait = ((*state).ordwait).wrapping_sub(1);
    (*state).ordwait;
    (*state).ni += 1;
    (*state).ni;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn msadams_set_driver(
    mut vstate: *mut libc::c_void,
    mut d: *const gsl_odeiv2_driver,
) -> libc::c_int {
    let mut state: *mut msadams_state_t = vstate as *mut msadams_state_t;
    (*state).driver = d;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn msadams_reset(
    mut vstate: *mut libc::c_void,
    mut dim: size_t,
) -> libc::c_int {
    let mut state: *mut msadams_state_t = vstate as *mut msadams_state_t;
    (*state).ni = 0 as libc::c_int as libc::c_long;
    (*state).ord = 1 as libc::c_int as size_t;
    (*state).ordprev = 1 as libc::c_int as size_t;
    (*state).ordprevbackup = 1 as libc::c_int as size_t;
    (*state).ordwait = 2 as libc::c_int as size_t;
    (*state).ordwaitbackup = 2 as libc::c_int as size_t;
    (*state).failord = 0 as libc::c_int as size_t;
    (*state).failt = ::std::f32::NAN as libc::c_double;
    (*state).failcount = 0 as libc::c_int as size_t;
    memset(
        (*state).hprev as *mut libc::c_void,
        0 as libc::c_int,
        (12 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memset(
        (*state).z as *mut libc::c_void,
        0 as libc::c_int,
        ((12 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(dim)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn msadams_order(mut vstate: *mut libc::c_void) -> libc::c_uint {
    let mut state: *mut msadams_state_t = vstate as *mut msadams_state_t;
    return (*state).ord as libc::c_uint;
}
unsafe extern "C" fn msadams_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut msadams_state_t = vstate as *mut msadams_state_t;
    gsl_vector_free((*state).tempvec);
    gsl_vector_free((*state).svec);
    gsl_vector_free((*state).relcor);
    gsl_vector_free((*state).abscor);
    free((*state).errlev as *mut libc::c_void);
    free((*state).hprevbackup as *mut libc::c_void);
    free((*state).hprev as *mut libc::c_void);
    free((*state).l as *mut libc::c_void);
    free((*state).pc as *mut libc::c_void);
    free((*state).ytmp2 as *mut libc::c_void);
    free((*state).ytmp as *mut libc::c_void);
    free((*state).zbackup as *mut libc::c_void);
    free((*state).z as *mut libc::c_void);
    free(state as *mut libc::c_void);
}
static mut msadams_type: gsl_odeiv2_step_type = {
    let mut init = gsl_odeiv2_step_type {
        name: b"msadams\0" as *const u8 as *const libc::c_char,
        can_use_dydt_in: 1 as libc::c_int,
        gives_exact_dydt_out: 1 as libc::c_int,
        alloc: Some(msadams_alloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
        apply: Some(
            msadams_apply
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    size_t,
                    libc::c_double,
                    libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *const libc::c_double,
                    *mut libc::c_double,
                    *const gsl_odeiv2_system,
                ) -> libc::c_int,
        ),
        set_driver: Some(
            msadams_set_driver
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const gsl_odeiv2_driver,
                ) -> libc::c_int,
        ),
        reset: Some(
            msadams_reset
                as unsafe extern "C" fn(*mut libc::c_void, size_t) -> libc::c_int,
        ),
        order: Some(
            msadams_order as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint,
        ),
        free: Some(msadams_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    };
    init
};
pub static mut gsl_odeiv2_step_msadams: *const gsl_odeiv2_step_type = unsafe {
    &msadams_type as *const gsl_odeiv2_step_type
};
