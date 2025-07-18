use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    static mut gsl_movstat_accum_mad: *const gsl_movstat_accum;
    static mut gsl_movstat_accum_mean: *const gsl_movstat_accum;
    static mut gsl_movstat_accum_min: *const gsl_movstat_accum;
    static mut gsl_movstat_accum_sum: *const gsl_movstat_accum;
    static mut gsl_movstat_accum_median: *const gsl_movstat_accum;
    static mut gsl_movstat_accum_Qn: *const gsl_movstat_accum;
    static mut gsl_movstat_accum_qqr: *const gsl_movstat_accum;
    static mut gsl_movstat_accum_Sn: *const gsl_movstat_accum;
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
pub struct gsl_movstat_accum {
    pub size: Option::<unsafe extern "C" fn(size_t) -> size_t>,
    pub init: Option::<unsafe extern "C" fn(size_t, *mut libc::c_void) -> libc::c_int>,
    pub insert: Option::<
        unsafe extern "C" fn(libc::c_double, *mut libc::c_void) -> libc::c_int,
    >,
    pub delete_oldest: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub get: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_double,
            *const libc::c_void,
        ) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_movstat_workspace {
    pub H: size_t,
    pub J: size_t,
    pub K: size_t,
    pub work: *mut libc::c_double,
    pub state: *mut libc::c_void,
    pub state_size: size_t,
}
pub unsafe extern "C" fn gsl_movstat_alloc(K: size_t) -> *mut gsl_movstat_workspace {
    let H: size_t = K.wrapping_div(2 as libc::c_int as libc::c_ulong);
    return gsl_movstat_alloc_with_size(0 as libc::c_int as size_t, H, H);
}
pub unsafe extern "C" fn gsl_movstat_alloc2(
    H: size_t,
    J: size_t,
) -> *mut gsl_movstat_workspace {
    return gsl_movstat_alloc_with_size(0 as libc::c_int as size_t, H, J);
}
pub unsafe extern "C" fn gsl_movstat_alloc_with_size(
    accum_state_size: size_t,
    H: size_t,
    J: size_t,
) -> *mut gsl_movstat_workspace {
    let mut w: *mut gsl_movstat_workspace = 0 as *mut gsl_movstat_workspace;
    let mut state_size: size_t = accum_state_size;
    w = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gsl_movstat_workspace>() as libc::c_ulong,
    ) as *mut gsl_movstat_workspace;
    if w.is_null() {
        gsl_error(
            b"failed to allocate space for workspace\0" as *const u8
                as *const libc::c_char,
            b"alloc.c\0" as *const u8 as *const libc::c_char,
            102 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_movstat_workspace;
    }
    (*w).H = H;
    (*w).J = J;
    (*w).K = H.wrapping_add(J).wrapping_add(1 as libc::c_int as libc::c_ulong);
    if state_size == 0 as libc::c_int as libc::c_ulong {
        state_size = if state_size > ((*gsl_movstat_accum_mad).size).unwrap()((*w).K) {
            state_size
        } else {
            ((*gsl_movstat_accum_mad).size).unwrap()((*w).K)
        };
        state_size = if state_size > ((*gsl_movstat_accum_mean).size).unwrap()((*w).K) {
            state_size
        } else {
            ((*gsl_movstat_accum_mean).size).unwrap()((*w).K)
        };
        state_size = if state_size > ((*gsl_movstat_accum_min).size).unwrap()((*w).K) {
            state_size
        } else {
            ((*gsl_movstat_accum_min).size).unwrap()((*w).K)
        };
        state_size = if state_size > ((*gsl_movstat_accum_sum).size).unwrap()((*w).K) {
            state_size
        } else {
            ((*gsl_movstat_accum_sum).size).unwrap()((*w).K)
        };
        state_size = if state_size > ((*gsl_movstat_accum_median).size).unwrap()((*w).K)
        {
            state_size
        } else {
            ((*gsl_movstat_accum_median).size).unwrap()((*w).K)
        };
        state_size = if state_size > ((*gsl_movstat_accum_Qn).size).unwrap()((*w).K) {
            state_size
        } else {
            ((*gsl_movstat_accum_Qn).size).unwrap()((*w).K)
        };
        state_size = if state_size > ((*gsl_movstat_accum_qqr).size).unwrap()((*w).K) {
            state_size
        } else {
            ((*gsl_movstat_accum_qqr).size).unwrap()((*w).K)
        };
        state_size = if state_size > ((*gsl_movstat_accum_Sn).size).unwrap()((*w).K) {
            state_size
        } else {
            ((*gsl_movstat_accum_Sn).size).unwrap()((*w).K)
        };
    }
    (*w).state = malloc(state_size);
    if ((*w).state).is_null() {
        gsl_movstat_free(w);
        gsl_error(
            b"failed to allocate space for accumulator state\0" as *const u8
                as *const libc::c_char,
            b"alloc.c\0" as *const u8 as *const libc::c_char,
            129 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_movstat_workspace;
    }
    (*w)
        .work = malloc(
        ((*w).K).wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*w).work).is_null() {
        gsl_movstat_free(w);
        gsl_error(
            b"failed to allocate space for work\0" as *const u8 as *const libc::c_char,
            b"alloc.c\0" as *const u8 as *const libc::c_char,
            136 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_movstat_workspace;
    }
    (*w).state_size = state_size;
    return w;
}
pub unsafe extern "C" fn gsl_movstat_free(mut w: *mut gsl_movstat_workspace) {
    if !((*w).work).is_null() {
        free((*w).work as *mut libc::c_void);
    }
    if !((*w).state).is_null() {
        free((*w).state);
    }
    free(w as *mut libc::c_void);
}
