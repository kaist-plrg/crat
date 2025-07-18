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
    fn gsl_blas_dnrm2(X: *const gsl_vector) -> libc::c_double;
    fn gsl_matrix_scale(a: *mut gsl_matrix, x: libc::c_double) -> libc::c_int;
    fn gsl_matrix_memcpy(dest: *mut gsl_matrix, src: *const gsl_matrix) -> libc::c_int;
    fn gsl_vector_set_zero(v: *mut gsl_vector);
    fn gsl_vector_alloc(n: size_t) -> *mut gsl_vector;
    fn gsl_matrix_alloc(n1: size_t, n2: size_t) -> *mut gsl_matrix;
    fn gsl_matrix_free(m: *mut gsl_matrix);
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_linalg_LU_decomp(
        A: *mut gsl_matrix,
        p: *mut gsl_permutation,
        signum: *mut libc::c_int,
    ) -> libc::c_int;
    fn gsl_linalg_LU_solve(
        LU: *const gsl_matrix,
        p: *const gsl_permutation,
        b: *const gsl_vector,
        x: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_permutation_alloc(n: size_t) -> *mut gsl_permutation;
    fn gsl_permutation_free(p: *mut gsl_permutation);
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
pub struct msbdf_state_t {
    pub z: *mut libc::c_double,
    pub zbackup: *mut libc::c_double,
    pub ytmp: *mut libc::c_double,
    pub ytmp2: *mut libc::c_double,
    pub l: *mut libc::c_double,
    pub hprev: *mut libc::c_double,
    pub hprevbackup: *mut libc::c_double,
    pub ordprev: *mut size_t,
    pub ordprevbackup: *mut size_t,
    pub errlev: *mut libc::c_double,
    pub abscor: *mut gsl_vector,
    pub abscorscaled: *mut gsl_vector,
    pub relcor: *mut gsl_vector,
    pub svec: *mut gsl_vector,
    pub tempvec: *mut gsl_vector,
    pub driver: *const gsl_odeiv2_driver,
    pub dfdy: *mut gsl_matrix,
    pub dfdt: *mut libc::c_double,
    pub M: *mut gsl_matrix,
    pub p: *mut gsl_permutation,
    pub rhs: *mut gsl_vector,
    pub ni: libc::c_long,
    pub ord: size_t,
    pub tprev: libc::c_double,
    pub ordwait: size_t,
    pub ordwaitbackup: size_t,
    pub failord: size_t,
    pub failt: libc::c_double,
    pub ordp1coeffprev: libc::c_double,
    pub nJ: size_t,
    pub nM: size_t,
    pub gammaprev: libc::c_double,
    pub gammaprevbackup: libc::c_double,
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
pub type gsl_permutation = gsl_permutation_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_permutation_struct {
    pub size: size_t,
    pub data: *mut size_t,
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
unsafe extern "C" fn gsl_matrix_get(
    mut m: *const gsl_matrix,
    i: size_t,
    j: size_t,
) -> libc::c_double {
    return *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
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
unsafe extern "C" fn msbdf_alloc(mut dim: size_t) -> *mut libc::c_void {
    let mut state: *mut msbdf_state_t = malloc(
        ::std::mem::size_of::<msbdf_state_t>() as libc::c_ulong,
    ) as *mut msbdf_state_t;
    if state.is_null() {
        gsl_error(
            b"failed to allocate space for msbdf_state\0" as *const u8
                as *const libc::c_char,
            b"msbdf.c\0" as *const u8 as *const libc::c_char,
            122 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .z = malloc(
        ((5 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(dim)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).z).is_null() {
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for z\0" as *const u8 as *const libc::c_char,
            b"msbdf.c\0" as *const u8 as *const libc::c_char,
            130 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .zbackup = malloc(
        ((5 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(dim)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).zbackup).is_null() {
        free((*state).z as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for zbackup\0" as *const u8
                as *const libc::c_char,
            b"msbdf.c\0" as *const u8 as *const libc::c_char,
            140 as libc::c_int,
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
            b"msbdf.c\0" as *const u8 as *const libc::c_char,
            150 as libc::c_int,
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
            b"msbdf.c\0" as *const u8 as *const libc::c_char,
            161 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .l = malloc(
        ((5 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).l).is_null() {
        free((*state).ytmp as *mut libc::c_void);
        free((*state).zbackup as *mut libc::c_void);
        free((*state).z as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for l\0" as *const u8 as *const libc::c_char,
            b"msbdf.c\0" as *const u8 as *const libc::c_char,
            172 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .hprev = malloc(
        (5 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).hprev).is_null() {
        free((*state).l as *mut libc::c_void);
        free((*state).ytmp2 as *mut libc::c_void);
        free((*state).ytmp as *mut libc::c_void);
        free((*state).zbackup as *mut libc::c_void);
        free((*state).z as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for hprev\0" as *const u8 as *const libc::c_char,
            b"msbdf.c\0" as *const u8 as *const libc::c_char,
            185 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .hprevbackup = malloc(
        (5 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).hprevbackup).is_null() {
        free((*state).hprev as *mut libc::c_void);
        free((*state).l as *mut libc::c_void);
        free((*state).ytmp2 as *mut libc::c_void);
        free((*state).ytmp as *mut libc::c_void);
        free((*state).zbackup as *mut libc::c_void);
        free((*state).z as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for hprevbackup\0" as *const u8
                as *const libc::c_char,
            b"msbdf.c\0" as *const u8 as *const libc::c_char,
            199 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .ordprev = malloc(
        (5 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong),
    ) as *mut size_t;
    if ((*state).ordprev).is_null() {
        free((*state).hprevbackup as *mut libc::c_void);
        free((*state).hprev as *mut libc::c_void);
        free((*state).l as *mut libc::c_void);
        free((*state).ytmp2 as *mut libc::c_void);
        free((*state).ytmp as *mut libc::c_void);
        free((*state).zbackup as *mut libc::c_void);
        free((*state).z as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for ordprev\0" as *const u8
                as *const libc::c_char,
            b"msbdf.c\0" as *const u8 as *const libc::c_char,
            214 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .ordprevbackup = malloc(
        (5 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong),
    ) as *mut size_t;
    if ((*state).ordprevbackup).is_null() {
        free((*state).ordprev as *mut libc::c_void);
        free((*state).hprevbackup as *mut libc::c_void);
        free((*state).hprev as *mut libc::c_void);
        free((*state).l as *mut libc::c_void);
        free((*state).ytmp2 as *mut libc::c_void);
        free((*state).ytmp as *mut libc::c_void);
        free((*state).zbackup as *mut libc::c_void);
        free((*state).z as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for ordprevbackup\0" as *const u8
                as *const libc::c_char,
            b"msbdf.c\0" as *const u8 as *const libc::c_char,
            231 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .errlev = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).errlev).is_null() {
        free((*state).ordprevbackup as *mut libc::c_void);
        free((*state).ordprev as *mut libc::c_void);
        free((*state).hprevbackup as *mut libc::c_void);
        free((*state).hprev as *mut libc::c_void);
        free((*state).l as *mut libc::c_void);
        free((*state).ytmp2 as *mut libc::c_void);
        free((*state).ytmp as *mut libc::c_void);
        free((*state).zbackup as *mut libc::c_void);
        free((*state).z as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for errlev\0" as *const u8 as *const libc::c_char,
            b"msbdf.c\0" as *const u8 as *const libc::c_char,
            248 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).abscor = gsl_vector_alloc(dim);
    if ((*state).abscor).is_null() {
        free((*state).errlev as *mut libc::c_void);
        free((*state).ordprevbackup as *mut libc::c_void);
        free((*state).ordprev as *mut libc::c_void);
        free((*state).hprevbackup as *mut libc::c_void);
        free((*state).hprev as *mut libc::c_void);
        free((*state).l as *mut libc::c_void);
        free((*state).ytmp2 as *mut libc::c_void);
        free((*state).ytmp as *mut libc::c_void);
        free((*state).zbackup as *mut libc::c_void);
        free((*state).z as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for abscor\0" as *const u8 as *const libc::c_char,
            b"msbdf.c\0" as *const u8 as *const libc::c_char,
            266 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).relcor = gsl_vector_alloc(dim);
    if ((*state).relcor).is_null() {
        gsl_vector_free((*state).abscor);
        free((*state).errlev as *mut libc::c_void);
        free((*state).ordprevbackup as *mut libc::c_void);
        free((*state).ordprev as *mut libc::c_void);
        free((*state).hprevbackup as *mut libc::c_void);
        free((*state).hprev as *mut libc::c_void);
        free((*state).l as *mut libc::c_void);
        free((*state).ytmp2 as *mut libc::c_void);
        free((*state).ytmp as *mut libc::c_void);
        free((*state).zbackup as *mut libc::c_void);
        free((*state).z as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for relcor\0" as *const u8 as *const libc::c_char,
            b"msbdf.c\0" as *const u8 as *const libc::c_char,
            285 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).svec = gsl_vector_alloc(dim);
    if ((*state).svec).is_null() {
        gsl_vector_free((*state).relcor);
        gsl_vector_free((*state).abscor);
        free((*state).errlev as *mut libc::c_void);
        free((*state).ordprevbackup as *mut libc::c_void);
        free((*state).ordprev as *mut libc::c_void);
        free((*state).hprevbackup as *mut libc::c_void);
        free((*state).hprev as *mut libc::c_void);
        free((*state).l as *mut libc::c_void);
        free((*state).ytmp2 as *mut libc::c_void);
        free((*state).ytmp as *mut libc::c_void);
        free((*state).zbackup as *mut libc::c_void);
        free((*state).z as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for svec\0" as *const u8 as *const libc::c_char,
            b"msbdf.c\0" as *const u8 as *const libc::c_char,
            305 as libc::c_int,
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
        free((*state).ordprevbackup as *mut libc::c_void);
        free((*state).ordprev as *mut libc::c_void);
        free((*state).hprevbackup as *mut libc::c_void);
        free((*state).hprev as *mut libc::c_void);
        free((*state).l as *mut libc::c_void);
        free((*state).ytmp2 as *mut libc::c_void);
        free((*state).ytmp as *mut libc::c_void);
        free((*state).zbackup as *mut libc::c_void);
        free((*state).z as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for tempvec\0" as *const u8
                as *const libc::c_char,
            b"msbdf.c\0" as *const u8 as *const libc::c_char,
            326 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).dfdy = gsl_matrix_alloc(dim, dim);
    if ((*state).dfdy).is_null() {
        gsl_vector_free((*state).tempvec);
        gsl_vector_free((*state).svec);
        gsl_vector_free((*state).relcor);
        gsl_vector_free((*state).abscor);
        free((*state).errlev as *mut libc::c_void);
        free((*state).ordprevbackup as *mut libc::c_void);
        free((*state).ordprev as *mut libc::c_void);
        free((*state).hprevbackup as *mut libc::c_void);
        free((*state).hprev as *mut libc::c_void);
        free((*state).l as *mut libc::c_void);
        free((*state).ytmp2 as *mut libc::c_void);
        free((*state).ytmp as *mut libc::c_void);
        free((*state).zbackup as *mut libc::c_void);
        free((*state).z as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for dfdy\0" as *const u8 as *const libc::c_char,
            b"msbdf.c\0" as *const u8 as *const libc::c_char,
            348 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .dfdt = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).dfdt).is_null() {
        gsl_matrix_free((*state).dfdy);
        gsl_vector_free((*state).tempvec);
        gsl_vector_free((*state).svec);
        gsl_vector_free((*state).relcor);
        gsl_vector_free((*state).abscor);
        free((*state).errlev as *mut libc::c_void);
        free((*state).ordprevbackup as *mut libc::c_void);
        free((*state).ordprev as *mut libc::c_void);
        free((*state).hprevbackup as *mut libc::c_void);
        free((*state).hprev as *mut libc::c_void);
        free((*state).l as *mut libc::c_void);
        free((*state).ytmp2 as *mut libc::c_void);
        free((*state).ytmp as *mut libc::c_void);
        free((*state).zbackup as *mut libc::c_void);
        free((*state).z as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for dfdt\0" as *const u8 as *const libc::c_char,
            b"msbdf.c\0" as *const u8 as *const libc::c_char,
            371 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).M = gsl_matrix_alloc(dim, dim);
    if ((*state).M).is_null() {
        free((*state).dfdt as *mut libc::c_void);
        gsl_matrix_free((*state).dfdy);
        gsl_vector_free((*state).tempvec);
        gsl_vector_free((*state).svec);
        gsl_vector_free((*state).relcor);
        gsl_vector_free((*state).abscor);
        free((*state).errlev as *mut libc::c_void);
        free((*state).ordprevbackup as *mut libc::c_void);
        free((*state).ordprev as *mut libc::c_void);
        free((*state).hprevbackup as *mut libc::c_void);
        free((*state).hprev as *mut libc::c_void);
        free((*state).l as *mut libc::c_void);
        free((*state).ytmp2 as *mut libc::c_void);
        free((*state).ytmp as *mut libc::c_void);
        free((*state).zbackup as *mut libc::c_void);
        free((*state).z as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for M\0" as *const u8 as *const libc::c_char,
            b"msbdf.c\0" as *const u8 as *const libc::c_char,
            395 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).p = gsl_permutation_alloc(dim);
    if ((*state).p).is_null() {
        gsl_matrix_free((*state).M);
        free((*state).dfdt as *mut libc::c_void);
        gsl_matrix_free((*state).dfdy);
        gsl_vector_free((*state).tempvec);
        gsl_vector_free((*state).svec);
        gsl_vector_free((*state).relcor);
        gsl_vector_free((*state).abscor);
        free((*state).errlev as *mut libc::c_void);
        free((*state).ordprevbackup as *mut libc::c_void);
        free((*state).ordprev as *mut libc::c_void);
        free((*state).hprevbackup as *mut libc::c_void);
        free((*state).hprev as *mut libc::c_void);
        free((*state).l as *mut libc::c_void);
        free((*state).ytmp2 as *mut libc::c_void);
        free((*state).ytmp as *mut libc::c_void);
        free((*state).zbackup as *mut libc::c_void);
        free((*state).z as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for p\0" as *const u8 as *const libc::c_char,
            b"msbdf.c\0" as *const u8 as *const libc::c_char,
            420 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).rhs = gsl_vector_alloc(dim);
    if ((*state).rhs).is_null() {
        gsl_permutation_free((*state).p);
        gsl_matrix_free((*state).M);
        free((*state).dfdt as *mut libc::c_void);
        gsl_matrix_free((*state).dfdy);
        gsl_vector_free((*state).tempvec);
        gsl_vector_free((*state).svec);
        gsl_vector_free((*state).relcor);
        gsl_vector_free((*state).abscor);
        free((*state).errlev as *mut libc::c_void);
        free((*state).ordprevbackup as *mut libc::c_void);
        free((*state).ordprev as *mut libc::c_void);
        free((*state).hprevbackup as *mut libc::c_void);
        free((*state).hprev as *mut libc::c_void);
        free((*state).l as *mut libc::c_void);
        free((*state).ytmp2 as *mut libc::c_void);
        free((*state).ytmp as *mut libc::c_void);
        free((*state).zbackup as *mut libc::c_void);
        free((*state).z as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for rhs\0" as *const u8 as *const libc::c_char,
            b"msbdf.c\0" as *const u8 as *const libc::c_char,
            446 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).abscorscaled = gsl_vector_alloc(dim);
    if ((*state).abscorscaled).is_null() {
        gsl_vector_free((*state).rhs);
        gsl_permutation_free((*state).p);
        gsl_matrix_free((*state).M);
        free((*state).dfdt as *mut libc::c_void);
        gsl_matrix_free((*state).dfdy);
        gsl_vector_free((*state).tempvec);
        gsl_vector_free((*state).svec);
        gsl_vector_free((*state).relcor);
        gsl_vector_free((*state).abscor);
        free((*state).errlev as *mut libc::c_void);
        free((*state).ordprevbackup as *mut libc::c_void);
        free((*state).ordprev as *mut libc::c_void);
        free((*state).hprevbackup as *mut libc::c_void);
        free((*state).hprev as *mut libc::c_void);
        free((*state).l as *mut libc::c_void);
        free((*state).ytmp2 as *mut libc::c_void);
        free((*state).ytmp as *mut libc::c_void);
        free((*state).zbackup as *mut libc::c_void);
        free((*state).z as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for abscorscaled\0" as *const u8
                as *const libc::c_char,
            b"msbdf.c\0" as *const u8 as *const libc::c_char,
            473 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    msbdf_reset(state as *mut libc::c_void, dim);
    (*state).driver = 0 as *const gsl_odeiv2_driver;
    return state as *mut libc::c_void;
}
unsafe extern "C" fn msbdf_failurehandler(
    mut vstate: *mut libc::c_void,
    dim: size_t,
    t: libc::c_double,
) -> libc::c_int {
    let mut state: *mut msbdf_state_t = vstate as *mut msbdf_state_t;
    let ord: size_t = (*state).ord;
    if ord > 1 as libc::c_int as libc::c_ulong
        && ord.wrapping_sub(*((*state).ordprev).offset(0 as libc::c_int as isize))
            == 0 as libc::c_int as libc::c_ulong && ord == (*state).failord
        && t == (*state).failt
    {
        (*state).ord = ((*state).ord).wrapping_sub(1);
        (*state).ord;
    }
    (*state).failord = ord;
    (*state).failt = t;
    (*state).ni += 1;
    (*state).ni;
    if ord == 1 as libc::c_int as libc::c_ulong {
        msbdf_reset(vstate, dim);
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn msbdf_calccoeffs(
    ord: size_t,
    ordwait: size_t,
    h: libc::c_double,
    mut hprev: *const libc::c_double,
    mut l: *mut libc::c_double,
    mut errcoeff: *mut libc::c_double,
    mut ordm1coeff: *mut libc::c_double,
    mut ordp1coeff: *mut libc::c_double,
    mut ordp2coeff: *mut libc::c_double,
    mut gamma: *mut libc::c_double,
) -> libc::c_int {
    if ord == 1 as libc::c_int as libc::c_ulong {
        *l.offset(0 as libc::c_int as isize) = 1.0f64;
        *l.offset(1 as libc::c_int as isize) = 1.0f64;
        *errcoeff = 0.5f64;
        *ordp1coeff = 2.0f64;
        let hsum: libc::c_double = h + *hprev.offset(0 as libc::c_int as isize);
        let a5: libc::c_double = -1.5f64;
        let a6: libc::c_double = -1.0f64 - h / hsum;
        let c2: libc::c_double = 2.0f64 / (1.0f64 - a6 + a5);
        *ordp2coeff = fabs(c2 * (h / hsum) * 3.0f64 * a5);
    } else {
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut hsum_0: libc::c_double = h;
        let mut coeff1: libc::c_double = -1.0f64;
        let mut x: libc::c_double = 0.;
        memset(
            l as *mut libc::c_void,
            0 as libc::c_int,
            ((5 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
        *l.offset(0 as libc::c_int as isize) = 1.0f64;
        *l.offset(1 as libc::c_int as isize) = 1.0f64;
        i = 2 as libc::c_int as size_t;
        while i < ord {
            hsum_0
                += *hprev
                    .offset(i.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize);
            coeff1 += -1.0f64 / i as libc::c_double;
            j = i;
            while j > 0 as libc::c_int as libc::c_ulong {
                *l.offset(j as isize)
                    += h / hsum_0
                        * *l
                            .offset(
                                j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            );
                j = j.wrapping_sub(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        coeff1 += -1.0f64 / ord as libc::c_double;
        x = -*l.offset(1 as libc::c_int as isize) - coeff1;
        i = ord;
        while i > 0 as libc::c_int as libc::c_ulong {
            *l.offset(i as isize)
                += *l.offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    * x;
            i = i.wrapping_sub(1);
            i;
        }
        hsum_0
            += *hprev
                .offset(ord.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize);
        let coeff2: libc::c_double = -*l.offset(1 as libc::c_int as isize) - h / hsum_0;
        let a1: libc::c_double = 1.0f64 - coeff2 + coeff1;
        let a2: libc::c_double = 1.0f64 + ord as libc::c_double * a1;
        *errcoeff = fabs(a1 / (coeff1 * a2));
        if ordwait < 2 as libc::c_int as libc::c_ulong {
            let a3: libc::c_double = coeff1 + 1.0f64 / ord as libc::c_double;
            let a4: libc::c_double = coeff2 + h / hsum_0;
            let c1: libc::c_double = a3 / (1.0f64 - a4 + a3);
            *ordm1coeff = fabs(c1 / (x / *l.offset(ord as isize)));
            *ordp1coeff = fabs(a2 / (*l.offset(ord as isize) * (h / hsum_0) / x));
            hsum_0
                += *hprev
                    .offset(
                        ord.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    );
            let a5_0: libc::c_double = coeff1
                - 1.0f64 / (ord as libc::c_double + 1.0f64);
            let a6_0: libc::c_double = coeff2 - h / hsum_0;
            let c2_0: libc::c_double = a2 / (1.0f64 - a6_0 + a5_0);
            *ordp2coeff = fabs(
                c2_0 * (h / hsum_0)
                    * ord.wrapping_add(2 as libc::c_int as libc::c_ulong)
                        as libc::c_double * a5_0,
            );
        }
    }
    *gamma = h / *l.offset(1 as libc::c_int as isize);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn msbdf_update(
    mut vstate: *mut libc::c_void,
    dim: size_t,
    mut dfdy: *mut gsl_matrix,
    mut dfdt: *mut libc::c_double,
    t: libc::c_double,
    mut y: *const libc::c_double,
    mut sys: *const gsl_odeiv2_system,
    mut M: *mut gsl_matrix,
    mut p: *mut gsl_permutation,
    iter: size_t,
    mut nJ: *mut size_t,
    mut nM: *mut size_t,
    tprev: libc::c_double,
    failt: libc::c_double,
    gamma: libc::c_double,
    gammaprev: libc::c_double,
    hratio: libc::c_double,
) -> libc::c_int {
    let c: libc::c_double = 0.2f64;
    let gammarel: libc::c_double = fabs(gamma / gammaprev - 1.0f64);
    if *nJ == 0 as libc::c_int as libc::c_ulong
        || *nJ > 50 as libc::c_int as libc::c_ulong
        || t == failt && (gammarel < c || hratio < 1.0f64)
    {
        let mut s: libc::c_int = (Some(((*sys).jacobian).unwrap()))
            .unwrap()(t, y, (*dfdy).data, dfdt, (*sys).params);
        if s == GSL_EBADFUNC as libc::c_int {
            return s;
        }
        if s != GSL_SUCCESS as libc::c_int {
            msbdf_failurehandler(vstate, dim, t);
            return s;
        }
        *nJ = 0 as libc::c_int as size_t;
    }
    if *nM == 0 as libc::c_int as libc::c_ulong
        || *nM > 20 as libc::c_int as libc::c_ulong || gammarel >= c || t == tprev
        || t == failt
    {
        let mut i: size_t = 0;
        gsl_matrix_memcpy(M, dfdy);
        gsl_matrix_scale(M, -gamma);
        i = 0 as libc::c_int as size_t;
        while i < dim {
            gsl_matrix_set(M, i, i, gsl_matrix_get(M, i, i) + 1.0f64);
            i = i.wrapping_add(1);
            i;
        }
        let mut signum: libc::c_int = 0;
        let mut s_0: libc::c_int = gsl_linalg_LU_decomp(M, p, &mut signum);
        if s_0 != GSL_SUCCESS as libc::c_int {
            return GSL_FAILURE as libc::c_int;
        }
        *nM = 0 as libc::c_int as size_t;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn msbdf_corrector(
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
    mut dfdy: *mut gsl_matrix,
    mut dfdt: *mut libc::c_double,
    mut M: *mut gsl_matrix,
    mut p: *mut gsl_permutation,
    mut rhs: *mut gsl_vector,
    mut nJ: *mut size_t,
    mut nM: *mut size_t,
    tprev: libc::c_double,
    failt: libc::c_double,
    gamma: libc::c_double,
    gammaprev: libc::c_double,
    hprev0: libc::c_double,
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
        msbdf_failurehandler(vstate, dim, t);
        return s;
    }
    gsl_vector_set_zero(abscor);
    mi = 0 as libc::c_int as size_t;
    while mi < max_iter {
        let safety: libc::c_double = 0.3f64;
        let safety2: libc::c_double = 0.1f64;
        if mi == 0 as libc::c_int as libc::c_ulong {
            let mut s_0: libc::c_int = msbdf_update(
                vstate,
                dim,
                dfdy,
                dfdt,
                t + h,
                z,
                sys,
                M,
                p,
                mi,
                nJ,
                nM,
                tprev,
                failt,
                gamma,
                gammaprev,
                h / hprev0,
            );
            if s_0 != GSL_SUCCESS as libc::c_int {
                return s_0;
            }
        }
        i = 0 as libc::c_int as size_t;
        while i < dim {
            let r: libc::c_double = -1.0f64 * gsl_vector_get(abscor, i)
                - *z
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(dim)
                            .wrapping_add(i) as isize,
                    ) / *l.offset(1 as libc::c_int as isize)
                + gamma * *ytmp.offset(i as isize);
            gsl_vector_set(rhs, i, r);
            i = i.wrapping_add(1);
            i;
        }
        let mut s_1: libc::c_int = gsl_linalg_LU_solve(M, p, rhs, relcor);
        if s_1 != GSL_SUCCESS as libc::c_int {
            msbdf_failurehandler(vstate, dim, t);
            return GSL_FAILURE as libc::c_int;
        }
        i = 0 as libc::c_int as size_t;
        while i < dim {
            let r_0: libc::c_double = gsl_vector_get(abscor, i)
                + gsl_vector_get(relcor, i);
            gsl_vector_set(abscor, i, r_0);
            *ytmp2.offset(i as isize) = *z.offset(i as isize) + r_0;
            gsl_vector_set(
                relcor,
                i,
                gsl_vector_get(relcor, i) / *errlev.offset(i as isize),
            );
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
            msbdf_failurehandler(vstate, dim, t);
            return GSL_FAILURE as libc::c_int;
        }
        let mut s_2: libc::c_int = (Some(((*sys).function).unwrap()))
            .unwrap()(t + h, ytmp2 as *const libc::c_double, ytmp, (*sys).params);
        if s_2 == GSL_EBADFUNC as libc::c_int {
            return s_2;
        }
        if s_2 != GSL_SUCCESS as libc::c_int {
            msbdf_failurehandler(vstate, dim, t);
            return s_2;
        }
        stepnormprev = stepnorm;
        mi = mi.wrapping_add(1);
        mi;
    }
    if mi == max_iter {
        msbdf_failurehandler(vstate, dim, t);
        return GSL_FAILURE as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn msbdf_eval_order(
    mut abscorscaled: *mut gsl_vector,
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
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut ordest: libc::c_double = 0.0f64;
    let mut ordm1est: libc::c_double = 0.0f64;
    let mut ordp1est: libc::c_double = 0.0f64;
    let safety: libc::c_double = 1e-6f64;
    let bias: libc::c_double = 6.0f64;
    let bias2: libc::c_double = 10.0f64;
    let min_incr: libc::c_double = 1.5f64;
    ordest = 1.0f64
        / (pow(
            bias * gsl_blas_dnrm2(abscorscaled) / sqrt(dim as libc::c_double) * errcoeff,
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
    if *ord < 5 as libc::c_int as libc::c_ulong {
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
                gsl_vector_get(svec, i) * c + gsl_vector_get(abscorscaled, i),
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
    if ordm1est > ordest && ordm1est > ordp1est && ordm1est > min_incr {
        *ord = (*ord as libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    } else if ordp1est > ordest && ordp1est > ordm1est && ordp1est > min_incr {
        *ord = (*ord as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn msbdf_check_no_order_decrease(
    mut ordprev: *const size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (5 as libc::c_int - 1 as libc::c_int) as libc::c_ulong {
        if *ordprev.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            > *ordprev.offset(i as isize)
        {
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn msbdf_check_step_size_decrease(
    mut hprev: *const libc::c_double,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut max: libc::c_double = fabs(*hprev.offset(0 as libc::c_int as isize));
    let min: libc::c_double = fabs(*hprev.offset(0 as libc::c_int as isize));
    let decrease_limit: libc::c_double = 0.5f64;
    i = 1 as libc::c_int as size_t;
    while i < 5 as libc::c_int as libc::c_ulong {
        let h: libc::c_double = fabs(*hprev.offset(i as isize));
        if h > min && h > max {
            max = h;
        }
        i = i.wrapping_add(1);
        i;
    }
    if min / max < decrease_limit {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn msbdf_apply(
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
    let mut state: *mut msbdf_state_t = vstate as *mut msbdf_state_t;
    let z: *mut libc::c_double = (*state).z;
    let zbackup: *mut libc::c_double = (*state).zbackup;
    let ytmp: *mut libc::c_double = (*state).ytmp;
    let ytmp2: *mut libc::c_double = (*state).ytmp2;
    let l: *mut libc::c_double = (*state).l;
    let hprev: *mut libc::c_double = (*state).hprev;
    let hprevbackup: *mut libc::c_double = (*state).hprevbackup;
    let ordprev: *mut size_t = (*state).ordprev;
    let ordprevbackup: *mut size_t = (*state).ordprevbackup;
    let errlev: *mut libc::c_double = (*state).errlev;
    let abscor: *mut gsl_vector = (*state).abscor;
    let abscorscaled: *mut gsl_vector = (*state).abscorscaled;
    let relcor: *mut gsl_vector = (*state).relcor;
    let svec: *mut gsl_vector = (*state).svec;
    let tempvec: *mut gsl_vector = (*state).tempvec;
    let mut ord: size_t = (*state).ord;
    let mut ordm1coeff: libc::c_double = 0.0f64;
    let mut ordp1coeff: libc::c_double = 0.0f64;
    let mut ordp2coeff: libc::c_double = 0.0f64;
    let mut errcoeff: libc::c_double = 0.0f64;
    let mut gamma: libc::c_double = 0.0f64;
    let max_failcount: size_t = 3 as libc::c_int as size_t;
    let mut i: size_t = 0;
    if (*state).ni > 0 as libc::c_int as libc::c_long
        && (t == (*state).tprev || t == (*state).failt)
    {
        if (*state).ni == 1 as libc::c_int as libc::c_long {
            msbdf_reset(vstate, dim);
        } else {
            if ord > *ordprev.offset(0 as libc::c_int as isize) {
                (*state).ord = *ordprev.offset(0 as libc::c_int as isize);
                ord = (*state).ord;
            }
            memcpy(
                z as *mut libc::c_void,
                zbackup as *const libc::c_void,
                ((5 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(dim)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    ),
            );
            memcpy(
                hprev as *mut libc::c_void,
                hprevbackup as *const libc::c_void,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    ),
            );
            i = 0 as libc::c_int as size_t;
            while i < 5 as libc::c_int as libc::c_ulong {
                *ordprev.offset(i as isize) = *ordprevbackup.offset(i as isize);
                i = i.wrapping_add(1);
                i;
            }
            (*state).ordwait = (*state).ordwaitbackup;
            (*state).gammaprev = (*state).gammaprevbackup;
        }
        (*state).failcount = ((*state).failcount).wrapping_add(1);
        (*state).failcount;
        if (*state).failcount > max_failcount
            && (*state).ni > 1 as libc::c_int as libc::c_long
        {
            msbdf_reset(vstate, dim);
            ord = (*state).ord;
        }
    } else {
        memcpy(
            zbackup as *mut libc::c_void,
            z as *const libc::c_void,
            ((5 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(dim)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
        memcpy(
            hprevbackup as *mut libc::c_void,
            hprev as *const libc::c_void,
            (5 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
        i = 0 as libc::c_int as size_t;
        while i < 5 as libc::c_int as libc::c_ulong {
            *ordprevbackup.offset(i as isize) = *ordprev.offset(i as isize);
            i = i.wrapping_add(1);
            i;
        }
        (*state).ordwaitbackup = (*state).ordwait;
        (*state).gammaprevbackup = (*state).gammaprev;
        (*state).failcount = 0 as libc::c_int as size_t;
    }
    if ((*state).driver).is_null() {
        return GSL_EFAULT as libc::c_int
    } else {
        let mut i_0: size_t = 0;
        i_0 = 0 as libc::c_int as size_t;
        while i_0 < dim {
            if !dydt_in.is_null() {
                gsl_odeiv2_control_errlevel(
                    (*(*state).driver).c,
                    *y.offset(i_0 as isize),
                    *dydt_in.offset(i_0 as isize),
                    h,
                    i_0,
                    &mut *errlev.offset(i_0 as isize),
                );
            } else {
                gsl_odeiv2_control_errlevel(
                    (*(*state).driver).c,
                    *y.offset(i_0 as isize),
                    0.0f64,
                    h,
                    i_0,
                    &mut *errlev.offset(i_0 as isize),
                );
            }
            i_0 = i_0.wrapping_add(1);
            i_0;
        }
    }
    if (*state).ni == 0 as libc::c_int as libc::c_long {
        let mut i_1: size_t = 0;
        memset(
            z as *mut libc::c_void,
            0 as libc::c_int,
            ((5 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
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
        i_1 = 0 as libc::c_int as size_t;
        while i_1 < dim {
            *z
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dim)
                        .wrapping_add(i_1) as isize,
                ) *= h;
            i_1 = i_1.wrapping_add(1);
            i_1;
        }
    }
    if ord > 1 as libc::c_int as libc::c_ulong
        && ord.wrapping_sub(*ordprev.offset(0 as libc::c_int as isize))
            == 0 as libc::c_int as libc::c_ulong
        && msbdf_check_no_order_decrease(ordprev as *const size_t) != 0
        && msbdf_check_step_size_decrease(hprev as *const libc::c_double) != 0
    {
        (*state).ord = ((*state).ord).wrapping_sub(1);
        (*state).ord;
        (*state).ordwait = ord.wrapping_add(2 as libc::c_int as libc::c_ulong);
        ord = (*state).ord;
    }
    let deltaord: libc::c_int = ord
        .wrapping_sub(*ordprev.offset(0 as libc::c_int as isize)) as libc::c_int;
    if deltaord > 1 as libc::c_int || deltaord < -(1 as libc::c_int) {
        printf(b"-- order change %d\n\0" as *const u8 as *const libc::c_char, deltaord);
        gsl_error(
            b"msbdf_apply too large order change\0" as *const u8 as *const libc::c_char,
            b"msbdf.c\0" as *const u8 as *const libc::c_char,
            1362 as libc::c_int,
            GSL_ESANITY as libc::c_int,
        );
        return GSL_ESANITY as libc::c_int;
    }
    if deltaord == 1 as libc::c_int {
        if ord > 2 as libc::c_int as libc::c_ulong {
            let mut i_2: size_t = 0;
            let mut j: size_t = 0;
            let mut hsum: libc::c_double = h;
            let mut coeff1: libc::c_double = -1.0f64;
            let mut coeff2: libc::c_double = 1.0f64;
            let mut hrelprev: libc::c_double = 1.0f64;
            let mut hrelprod: libc::c_double = 1.0f64;
            let mut hrel: libc::c_double = 0.0f64;
            memset(
                l as *mut libc::c_void,
                0 as libc::c_int,
                ((5 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    ),
            );
            *l.offset(2 as libc::c_int as isize) = 1.0f64;
            i_2 = 1 as libc::c_int as size_t;
            while i_2 < ord.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                hsum += *hprev.offset(i_2 as isize);
                hrel = hsum / h;
                hrelprod *= hrel;
                coeff1
                    -= 1.0f64
                        / i_2.wrapping_add(1 as libc::c_int as libc::c_ulong)
                            as libc::c_double;
                coeff2 += 1.0f64 / hrel;
                j = i_2.wrapping_add(2 as libc::c_int as libc::c_ulong);
                while j > 1 as libc::c_int as libc::c_ulong {
                    *l.offset(j as isize) *= hrelprev;
                    *l.offset(j as isize)
                        += *l
                            .offset(
                                j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            );
                    j = j.wrapping_sub(1);
                    j;
                }
                hrelprev = hrel;
                i_2 = i_2.wrapping_add(1);
                i_2;
            }
            let c: libc::c_double = (-coeff1 - coeff2) / hrelprod;
            i_2 = 0 as libc::c_int as size_t;
            while i_2 < dim {
                *z
                    .offset(
                        ord.wrapping_mul(dim).wrapping_add(i_2) as isize,
                    ) = c * gsl_vector_get(abscor, i_2);
                i_2 = i_2.wrapping_add(1);
                i_2;
            }
            i_2 = 2 as libc::c_int as size_t;
            while i_2 < ord {
                j = 0 as libc::c_int as size_t;
                while j < dim {
                    *z.offset(i_2.wrapping_mul(dim).wrapping_add(j) as isize)
                        += *l.offset(i_2 as isize)
                            * *z.offset(ord.wrapping_mul(dim).wrapping_add(j) as isize);
                    j = j.wrapping_add(1);
                    j;
                }
                i_2 = i_2.wrapping_add(1);
                i_2;
            }
        } else {
            memset(
                &mut *z.offset(ord.wrapping_mul(dim) as isize) as *mut libc::c_double
                    as *mut libc::c_void,
                0 as libc::c_int,
                dim
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    ),
            );
        }
    }
    if deltaord == -(1 as libc::c_int) {
        let mut i_3: size_t = 0;
        let mut j_0: size_t = 0;
        let mut hsum_0: libc::c_double = 0.0f64;
        memset(
            l as *mut libc::c_void,
            0 as libc::c_int,
            ((5 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
        *l.offset(2 as libc::c_int as isize) = 1.0f64;
        i_3 = 1 as libc::c_int as size_t;
        while i_3 < ord {
            hsum_0
                += *hprev
                    .offset(
                        i_3.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    );
            j_0 = i_3.wrapping_add(2 as libc::c_int as libc::c_ulong);
            while j_0 > 1 as libc::c_int as libc::c_ulong {
                *l.offset(j_0 as isize) *= hsum_0 / h;
                *l.offset(j_0 as isize)
                    += *l
                        .offset(
                            j_0.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                j_0 = j_0.wrapping_sub(1);
                j_0;
            }
            i_3 = i_3.wrapping_add(1);
            i_3;
        }
        i_3 = 2 as libc::c_int as size_t;
        while i_3 < ord.wrapping_add(1 as libc::c_int as libc::c_ulong) {
            j_0 = 0 as libc::c_int as size_t;
            while j_0 < dim {
                *z.offset(i_3.wrapping_mul(dim).wrapping_add(j_0) as isize)
                    += -*l.offset(i_3 as isize)
                        * *z
                            .offset(
                                ord
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(dim)
                                    .wrapping_add(j_0) as isize,
                            );
                j_0 = j_0.wrapping_add(1);
                j_0;
            }
            i_3 = i_3.wrapping_add(1);
            i_3;
        }
    }
    if (*state).ni > 0 as libc::c_int as libc::c_long
        && h != *hprev.offset(0 as libc::c_int as isize)
    {
        let mut i_4: size_t = 0;
        let mut j_1: size_t = 0;
        let hrel_0: libc::c_double = h / *hprev.offset(0 as libc::c_int as isize);
        let mut coeff: libc::c_double = hrel_0;
        i_4 = 1 as libc::c_int as size_t;
        while i_4 < ord.wrapping_add(1 as libc::c_int as libc::c_ulong) {
            j_1 = 0 as libc::c_int as size_t;
            while j_1 < dim {
                *z.offset(i_4.wrapping_mul(dim).wrapping_add(j_1) as isize) *= coeff;
                j_1 = j_1.wrapping_add(1);
                j_1;
            }
            coeff *= hrel_0;
            i_4 = i_4.wrapping_add(1);
            i_4;
        }
    }
    msbdf_calccoeffs(
        ord,
        (*state).ordwait,
        h,
        hprev as *const libc::c_double,
        l,
        &mut errcoeff,
        &mut ordm1coeff,
        &mut ordp1coeff,
        &mut ordp2coeff,
        &mut gamma,
    );
    let mut i_5: size_t = 0;
    let mut j_2: size_t = 0;
    let mut k: size_t = 0;
    i_5 = 1 as libc::c_int as size_t;
    while i_5 < ord.wrapping_add(1 as libc::c_int as libc::c_ulong) {
        j_2 = ord;
        while j_2 > i_5.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            k = 0 as libc::c_int as size_t;
            while k < dim {
                *z
                    .offset(
                        j_2
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(dim)
                            .wrapping_add(k) as isize,
                    ) += *z.offset(j_2.wrapping_mul(dim).wrapping_add(k) as isize);
                k = k.wrapping_add(1);
                k;
            }
            j_2 = j_2.wrapping_sub(1);
            j_2;
        }
        i_5 = i_5.wrapping_add(1);
        i_5;
    }
    let mut s_0: libc::c_int = 0;
    s_0 = msbdf_corrector(
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
        (*state).dfdy,
        (*state).dfdt,
        (*state).M,
        (*state).p,
        (*state).rhs,
        &mut (*state).nJ,
        &mut (*state).nM,
        (*state).tprev,
        (*state).failt,
        gamma,
        (*state).gammaprev,
        *hprev.offset(0 as libc::c_int as isize),
    );
    if s_0 != GSL_SUCCESS as libc::c_int {
        return s_0;
    }
    let mut i_6: size_t = 0;
    let mut j_3: size_t = 0;
    i_6 = 0 as libc::c_int as size_t;
    while i_6 < ord.wrapping_add(1 as libc::c_int as libc::c_ulong) {
        j_3 = 0 as libc::c_int as size_t;
        while j_3 < dim {
            *z.offset(i_6.wrapping_mul(dim).wrapping_add(j_3) as isize)
                += *l.offset(i_6 as isize) * gsl_vector_get(abscor, j_3);
            j_3 = j_3.wrapping_add(1);
            j_3;
        }
        i_6 = i_6.wrapping_add(1);
        i_6;
    }
    if !dydt_out.is_null() {
        let mut s_1: libc::c_int = (Some(((*sys).function).unwrap()))
            .unwrap()(t + h, z as *const libc::c_double, dydt_out, (*sys).params);
        if s_1 == GSL_EBADFUNC as libc::c_int {
            return s_1;
        }
        if s_1 != GSL_SUCCESS as libc::c_int {
            msbdf_failurehandler(vstate, dim, t);
            return s_1;
        }
    }
    i_6 = 0 as libc::c_int as size_t;
    while i_6 < dim {
        *yerr.offset(i_6 as isize) = fabs(gsl_vector_get(abscor, i_6)) * errcoeff;
        i_6 = i_6.wrapping_add(1);
        i_6;
    }
    i_6 = 0 as libc::c_int as size_t;
    while i_6 < dim {
        *y
            .offset(
                i_6 as isize,
            ) = *z
            .offset(
                (0 as libc::c_int as libc::c_ulong).wrapping_mul(dim).wrapping_add(i_6)
                    as isize,
            );
        i_6 = i_6.wrapping_add(1);
        i_6;
    }
    let mut i_7: size_t = 0;
    i_7 = 0 as libc::c_int as size_t;
    while i_7 < dim {
        gsl_vector_set(
            abscorscaled,
            i_7,
            gsl_vector_get(abscor, i_7) / *errlev.offset(i_7 as isize),
        );
        i_7 = i_7.wrapping_add(1);
        i_7;
    }
    if (*state).ordwait == 1 as libc::c_int as libc::c_ulong
        && ord < 5 as libc::c_int as libc::c_ulong
    {
        let mut i_8: size_t = 0;
        (*state).ordp1coeffprev = ordp1coeff;
        i_8 = 0 as libc::c_int as size_t;
        while i_8 < dim {
            gsl_vector_set(svec, i_8, gsl_vector_get(abscorscaled, i_8));
            i_8 = i_8.wrapping_add(1);
            i_8;
        }
    }
    if (*state).ordwait == 0 as libc::c_int as libc::c_ulong {
        if ord.wrapping_sub(*ordprev.offset(0 as libc::c_int as isize))
            == 0 as libc::c_int as libc::c_ulong
        {
            msbdf_eval_order(
                abscorscaled,
                tempvec,
                svec,
                errcoeff,
                dim,
                errlev as *const libc::c_double,
                ordm1coeff,
                ordp1coeff,
                (*state).ordp1coeffprev,
                ordp2coeff,
                hprev as *const libc::c_double,
                h,
                z as *const libc::c_double,
                &mut (*state).ord,
            );
            (*state)
                .ordwait = ((*state).ord)
                .wrapping_add(2 as libc::c_int as libc::c_ulong);
        } else {
            (*state).ordwait = 2 as libc::c_int as size_t;
        }
    }
    let mut i_9: size_t = 0;
    i_9 = (5 as libc::c_int - 1 as libc::c_int) as size_t;
    while i_9 > 0 as libc::c_int as libc::c_ulong {
        *hprev
            .offset(
                i_9 as isize,
            ) = *hprev
            .offset(i_9.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
        *ordprev
            .offset(
                i_9 as isize,
            ) = *ordprev
            .offset(i_9.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
        i_9 = i_9.wrapping_sub(1);
        i_9;
    }
    *hprev.offset(0 as libc::c_int as isize) = h;
    *ordprev.offset(0 as libc::c_int as isize) = ord;
    (*state).tprev = t;
    (*state).ordwait = ((*state).ordwait).wrapping_sub(1);
    (*state).ordwait;
    (*state).ni += 1;
    (*state).ni;
    (*state).gammaprev = gamma;
    (*state).nJ = ((*state).nJ).wrapping_add(1);
    (*state).nJ;
    (*state).nM = ((*state).nM).wrapping_add(1);
    (*state).nM;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn msbdf_set_driver(
    mut vstate: *mut libc::c_void,
    mut d: *const gsl_odeiv2_driver,
) -> libc::c_int {
    let mut state: *mut msbdf_state_t = vstate as *mut msbdf_state_t;
    (*state).driver = d;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn msbdf_reset(
    mut vstate: *mut libc::c_void,
    mut dim: size_t,
) -> libc::c_int {
    let mut state: *mut msbdf_state_t = vstate as *mut msbdf_state_t;
    let mut i: size_t = 0;
    (*state).ni = 0 as libc::c_int as libc::c_long;
    (*state).ord = 1 as libc::c_int as size_t;
    (*state).ordwait = 2 as libc::c_int as size_t;
    (*state).ordwaitbackup = 2 as libc::c_int as size_t;
    (*state).failord = 0 as libc::c_int as size_t;
    (*state).failt = ::std::f32::NAN as libc::c_double;
    (*state).tprev = ::std::f32::NAN as libc::c_double;
    (*state).gammaprev = 1.0f64;
    (*state).gammaprevbackup = 1.0f64;
    (*state).nJ = 0 as libc::c_int as size_t;
    (*state).nM = 0 as libc::c_int as size_t;
    (*state).failcount = 0 as libc::c_int as size_t;
    (*state).ordp1coeffprev = 0.0f64;
    memset(
        (*state).hprev as *mut libc::c_void,
        0 as libc::c_int,
        (5 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memset(
        (*state).hprevbackup as *mut libc::c_void,
        0 as libc::c_int,
        (5 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memset(
        (*state).z as *mut libc::c_void,
        0 as libc::c_int,
        ((5 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(dim)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memset(
        (*state).zbackup as *mut libc::c_void,
        0 as libc::c_int,
        ((5 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(dim)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < 5 as libc::c_int as libc::c_ulong {
        *((*state).ordprev).offset(i as isize) = 1 as libc::c_int as size_t;
        *((*state).ordprevbackup).offset(i as isize) = 1 as libc::c_int as size_t;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn msbdf_order(mut vstate: *mut libc::c_void) -> libc::c_uint {
    let mut state: *mut msbdf_state_t = vstate as *mut msbdf_state_t;
    return (*state).ord as libc::c_uint;
}
unsafe extern "C" fn msbdf_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut msbdf_state_t = vstate as *mut msbdf_state_t;
    gsl_vector_free((*state).rhs);
    gsl_permutation_free((*state).p);
    gsl_matrix_free((*state).M);
    free((*state).dfdt as *mut libc::c_void);
    gsl_matrix_free((*state).dfdy);
    gsl_vector_free((*state).tempvec);
    gsl_vector_free((*state).svec);
    gsl_vector_free((*state).relcor);
    gsl_vector_free((*state).abscor);
    gsl_vector_free((*state).abscorscaled);
    free((*state).errlev as *mut libc::c_void);
    free((*state).ordprevbackup as *mut libc::c_void);
    free((*state).ordprev as *mut libc::c_void);
    free((*state).hprevbackup as *mut libc::c_void);
    free((*state).hprev as *mut libc::c_void);
    free((*state).l as *mut libc::c_void);
    free((*state).ytmp2 as *mut libc::c_void);
    free((*state).ytmp as *mut libc::c_void);
    free((*state).zbackup as *mut libc::c_void);
    free((*state).z as *mut libc::c_void);
    free(state as *mut libc::c_void);
}
static mut msbdf_type: gsl_odeiv2_step_type = {
    let mut init = gsl_odeiv2_step_type {
        name: b"msbdf\0" as *const u8 as *const libc::c_char,
        can_use_dydt_in: 1 as libc::c_int,
        gives_exact_dydt_out: 1 as libc::c_int,
        alloc: Some(msbdf_alloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
        apply: Some(
            msbdf_apply
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
            msbdf_set_driver
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const gsl_odeiv2_driver,
                ) -> libc::c_int,
        ),
        reset: Some(
            msbdf_reset as unsafe extern "C" fn(*mut libc::c_void, size_t) -> libc::c_int,
        ),
        order: Some(
            msbdf_order as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint,
        ),
        free: Some(msbdf_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    };
    init
};
pub static mut gsl_odeiv2_step_msbdf: *const gsl_odeiv2_step_type = unsafe {
    &msbdf_type as *const gsl_odeiv2_step_type
};
