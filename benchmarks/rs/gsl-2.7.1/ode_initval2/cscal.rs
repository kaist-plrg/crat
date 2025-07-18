use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_odeiv2_control_alloc(
        T: *const gsl_odeiv2_control_type,
    ) -> *mut gsl_odeiv2_control;
    fn gsl_odeiv2_control_init(
        c: *mut gsl_odeiv2_control,
        eps_abs: libc::c_double,
        eps_rel: libc::c_double,
        a_y: libc::c_double,
        a_dydt: libc::c_double,
    ) -> libc::c_int;
    fn gsl_odeiv2_control_free(c: *mut gsl_odeiv2_control);
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
pub struct sc_control_state_t {
    pub eps_abs: libc::c_double,
    pub eps_rel: libc::c_double,
    pub a_y: libc::c_double,
    pub a_dydt: libc::c_double,
    pub scale_abs: *mut libc::c_double,
}
#[inline]
unsafe extern "C" fn GSL_MAX_DBL(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return if a > b { a } else { b };
}
unsafe extern "C" fn control_set_driver_null(
    mut vstate: *mut libc::c_void,
    mut d: *const gsl_odeiv2_driver,
) -> libc::c_int {
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn sc_control_alloc() -> *mut libc::c_void {
    let mut s: *mut sc_control_state_t = malloc(
        ::std::mem::size_of::<sc_control_state_t>() as libc::c_ulong,
    ) as *mut sc_control_state_t;
    if s.is_null() {
        gsl_error(
            b"failed to allocate space for sc_control_state\0" as *const u8
                as *const libc::c_char,
            b"cscal.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    return s as *mut libc::c_void;
}
unsafe extern "C" fn sc_control_init(
    mut vstate: *mut libc::c_void,
    mut eps_abs: libc::c_double,
    mut eps_rel: libc::c_double,
    mut a_y: libc::c_double,
    mut a_dydt: libc::c_double,
) -> libc::c_int {
    let mut s: *mut sc_control_state_t = vstate as *mut sc_control_state_t;
    if eps_abs < 0 as libc::c_int as libc::c_double {
        gsl_error(
            b"eps_abs is negative\0" as *const u8 as *const libc::c_char,
            b"cscal.c\0" as *const u8 as *const libc::c_char,
            62 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if eps_rel < 0 as libc::c_int as libc::c_double {
        gsl_error(
            b"eps_rel is negative\0" as *const u8 as *const libc::c_char,
            b"cscal.c\0" as *const u8 as *const libc::c_char,
            66 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if a_y < 0 as libc::c_int as libc::c_double {
        gsl_error(
            b"a_y is negative\0" as *const u8 as *const libc::c_char,
            b"cscal.c\0" as *const u8 as *const libc::c_char,
            70 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if a_dydt < 0 as libc::c_int as libc::c_double {
        gsl_error(
            b"a_dydt is negative\0" as *const u8 as *const libc::c_char,
            b"cscal.c\0" as *const u8 as *const libc::c_char,
            74 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    (*s).eps_rel = eps_rel;
    (*s).eps_abs = eps_abs;
    (*s).a_y = a_y;
    (*s).a_dydt = a_dydt;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn sc_control_hadjust(
    mut vstate: *mut libc::c_void,
    mut dim: size_t,
    mut ord: libc::c_uint,
    mut y: *const libc::c_double,
    mut yerr: *const libc::c_double,
    mut yp: *const libc::c_double,
    mut h: *mut libc::c_double,
) -> libc::c_int {
    let mut state: *mut sc_control_state_t = vstate as *mut sc_control_state_t;
    let eps_abs: libc::c_double = (*state).eps_abs;
    let eps_rel: libc::c_double = (*state).eps_rel;
    let a_y: libc::c_double = (*state).a_y;
    let a_dydt: libc::c_double = (*state).a_dydt;
    let mut scale_abs: *const libc::c_double = (*state).scale_abs;
    let S: libc::c_double = 0.9f64;
    let h_old: libc::c_double = *h;
    let mut rmax: libc::c_double = 2.2250738585072014e-308f64;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < dim {
        let D0: libc::c_double = eps_rel
            * (a_y * fabs(*y.offset(i as isize))
                + a_dydt * fabs(h_old * *yp.offset(i as isize)))
            + eps_abs * *scale_abs.offset(i as isize);
        let r: libc::c_double = fabs(*yerr.offset(i as isize)) / fabs(D0);
        rmax = GSL_MAX_DBL(r, rmax);
        i = i.wrapping_add(1);
        i;
    }
    if rmax > 1.1f64 {
        let mut r_0: libc::c_double = S / pow(rmax, 1.0f64 / ord as libc::c_double);
        if r_0 < 0.2f64 {
            r_0 = 0.2f64;
        }
        *h = r_0 * h_old;
        return -(1 as libc::c_int);
    } else if rmax < 0.5f64 {
        let mut r_1: libc::c_double = S
            / pow(rmax, 1.0f64 / (ord as libc::c_double + 1.0f64));
        let maxscale: libc::c_double = 4.9f64;
        if r_1 > maxscale {
            r_1 = maxscale;
        }
        if r_1 < 1.0f64 {
            r_1 = 1.0f64;
        }
        *h = r_1 * h_old;
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn sc_control_errlevel(
    mut vstate: *mut libc::c_void,
    y: libc::c_double,
    dydt: libc::c_double,
    h: libc::c_double,
    ind: size_t,
    mut errlev: *mut libc::c_double,
) -> libc::c_int {
    let mut state: *mut sc_control_state_t = vstate as *mut sc_control_state_t;
    let eps_abs: libc::c_double = (*state).eps_abs;
    let eps_rel: libc::c_double = (*state).eps_rel;
    let a_y: libc::c_double = (*state).a_y;
    let a_dydt: libc::c_double = (*state).a_dydt;
    let mut scale_abs: *const libc::c_double = (*state).scale_abs;
    *errlev = eps_rel * (a_y * fabs(y) + a_dydt * fabs(h * dydt))
        + eps_abs * *scale_abs.offset(ind as isize);
    if *errlev <= 0.0f64 {
        gsl_error(
            b"errlev <= zero\0" as *const u8 as *const libc::c_char,
            b"cscal.c\0" as *const u8 as *const libc::c_char,
            166 as libc::c_int,
            GSL_ESANITY as libc::c_int,
        );
        return GSL_ESANITY as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn sc_control_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut sc_control_state_t = vstate as *mut sc_control_state_t;
    free((*state).scale_abs as *mut libc::c_void);
    free(state as *mut libc::c_void);
}
static mut sc_control_type: gsl_odeiv2_control_type = {
    let mut init = gsl_odeiv2_control_type {
        name: b"scaled\0" as *const u8 as *const libc::c_char,
        alloc: Some(sc_control_alloc as unsafe extern "C" fn() -> *mut libc::c_void),
        init: Some(
            sc_control_init
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    libc::c_double,
                    libc::c_double,
                    libc::c_double,
                    libc::c_double,
                ) -> libc::c_int,
        ),
        hadjust: Some(
            sc_control_hadjust
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    size_t,
                    libc::c_uint,
                    *const libc::c_double,
                    *const libc::c_double,
                    *const libc::c_double,
                    *mut libc::c_double,
                ) -> libc::c_int,
        ),
        errlevel: Some(
            sc_control_errlevel
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    libc::c_double,
                    libc::c_double,
                    libc::c_double,
                    size_t,
                    *mut libc::c_double,
                ) -> libc::c_int,
        ),
        set_driver: Some(
            control_set_driver_null
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const gsl_odeiv2_driver,
                ) -> libc::c_int,
        ),
        free: Some(sc_control_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    };
    init
};
pub static mut gsl_odeiv2_control_scaled: *const gsl_odeiv2_control_type = unsafe {
    &sc_control_type as *const gsl_odeiv2_control_type
};
pub unsafe extern "C" fn gsl_odeiv2_control_scaled_new(
    mut eps_abs: libc::c_double,
    mut eps_rel: libc::c_double,
    mut a_y: libc::c_double,
    mut a_dydt: libc::c_double,
    mut scale_abs: *const libc::c_double,
    mut dim: size_t,
) -> *mut gsl_odeiv2_control {
    let mut c: *mut gsl_odeiv2_control = gsl_odeiv2_control_alloc(
        gsl_odeiv2_control_scaled,
    );
    let mut status: libc::c_int = gsl_odeiv2_control_init(
        c,
        eps_abs,
        eps_rel,
        a_y,
        a_dydt,
    );
    if status != GSL_SUCCESS as libc::c_int {
        gsl_odeiv2_control_free(c);
        gsl_error(
            b"error trying to initialize control\0" as *const u8 as *const libc::c_char,
            b"cscal.c\0" as *const u8 as *const libc::c_char,
            206 as libc::c_int,
            status,
        );
        return 0 as *mut gsl_odeiv2_control;
    }
    let mut s: *mut sc_control_state_t = (*c).state as *mut sc_control_state_t;
    (*s)
        .scale_abs = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*s).scale_abs).is_null() {
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for scale_abs\0" as *const u8
                as *const libc::c_char,
            b"cscal.c\0" as *const u8 as *const libc::c_char,
            217 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_odeiv2_control;
    }
    memcpy(
        (*s).scale_abs as *mut libc::c_void,
        scale_abs as *const libc::c_void,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    return c;
}
