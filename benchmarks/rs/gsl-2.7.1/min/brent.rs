use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
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
pub struct gsl_function_struct {
    pub function: Option::<
        unsafe extern "C" fn(libc::c_double, *mut libc::c_void) -> libc::c_double,
    >,
    pub params: *mut libc::c_void,
}
pub type gsl_function = gsl_function_struct;
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
pub struct gsl_min_fminimizer_type {
    pub name: *const libc::c_char,
    pub size: size_t,
    pub set: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut gsl_function,
            libc::c_double,
            libc::c_double,
            libc::c_double,
            libc::c_double,
            libc::c_double,
            libc::c_double,
        ) -> libc::c_int,
    >,
    pub iterate: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut gsl_function,
            *mut libc::c_double,
            *mut libc::c_double,
            *mut libc::c_double,
            *mut libc::c_double,
            *mut libc::c_double,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brent_state_t {
    pub d: libc::c_double,
    pub e: libc::c_double,
    pub v: libc::c_double,
    pub w: libc::c_double,
    pub f_v: libc::c_double,
    pub f_w: libc::c_double,
}
unsafe extern "C" fn brent_init(
    mut vstate: *mut libc::c_void,
    mut f: *mut gsl_function,
    mut x_minimum: libc::c_double,
    mut f_minimum: libc::c_double,
    mut x_lower: libc::c_double,
    mut f_lower: libc::c_double,
    mut x_upper: libc::c_double,
    mut f_upper: libc::c_double,
) -> libc::c_int {
    let mut state: *mut brent_state_t = vstate as *mut brent_state_t;
    let golden: libc::c_double = 0.3819660f64;
    let mut v: libc::c_double = x_lower + golden * (x_upper - x_lower);
    let mut w: libc::c_double = v;
    let mut f_vw: libc::c_double = 0.;
    x_minimum = 0 as libc::c_int as libc::c_double;
    f_minimum = 0 as libc::c_int as libc::c_double;
    f_lower = 0 as libc::c_int as libc::c_double;
    f_upper = 0 as libc::c_int as libc::c_double;
    (*state).v = v;
    (*state).w = w;
    (*state).d = 0 as libc::c_int as libc::c_double;
    (*state).e = 0 as libc::c_int as libc::c_double;
    f_vw = (Some(((*f).function).unwrap())).unwrap()(v, (*f).params);
    if gsl_finite(f_vw) == 0 {
        gsl_error(
            b"computed function value is infinite or NaN\0" as *const u8
                as *const libc::c_char,
            b"brent.c\0" as *const u8 as *const libc::c_char,
            69 as libc::c_int,
            GSL_EBADFUNC as libc::c_int,
        );
        return GSL_EBADFUNC as libc::c_int;
    }
    (*state).f_v = f_vw;
    (*state).f_w = f_vw;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn brent_iterate(
    mut vstate: *mut libc::c_void,
    mut f: *mut gsl_function,
    mut x_minimum: *mut libc::c_double,
    mut f_minimum: *mut libc::c_double,
    mut x_lower: *mut libc::c_double,
    mut f_lower: *mut libc::c_double,
    mut x_upper: *mut libc::c_double,
    mut f_upper: *mut libc::c_double,
) -> libc::c_int {
    let mut state: *mut brent_state_t = vstate as *mut brent_state_t;
    let x_left: libc::c_double = *x_lower;
    let x_right: libc::c_double = *x_upper;
    let z: libc::c_double = *x_minimum;
    let mut d: libc::c_double = (*state).e;
    let mut e: libc::c_double = (*state).d;
    let mut u: libc::c_double = 0.;
    let mut f_u: libc::c_double = 0.;
    let v: libc::c_double = (*state).v;
    let w: libc::c_double = (*state).w;
    let f_v: libc::c_double = (*state).f_v;
    let f_w: libc::c_double = (*state).f_w;
    let f_z: libc::c_double = *f_minimum;
    let golden: libc::c_double = 0.3819660f64;
    let w_lower: libc::c_double = z - x_left;
    let w_upper: libc::c_double = x_right - z;
    let tolerance: libc::c_double = 1.4901161193847656e-08f64 * fabs(z);
    let mut p: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut q: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut r: libc::c_double = 0 as libc::c_int as libc::c_double;
    let midpoint: libc::c_double = 0.5f64 * (x_left + x_right);
    if fabs(e) > tolerance {
        r = (z - w) * (f_z - f_v);
        q = (z - v) * (f_z - f_w);
        p = (z - v) * q - (z - w) * r;
        q = 2 as libc::c_int as libc::c_double * (q - r);
        if q > 0 as libc::c_int as libc::c_double {
            p = -p;
        } else {
            q = -q;
        }
        r = e;
        e = d;
    }
    if fabs(p) < fabs(0.5f64 * q * r) && p < q * w_lower && p < q * w_upper {
        let mut t2: libc::c_double = 2 as libc::c_int as libc::c_double * tolerance;
        d = p / q;
        u = z + d;
        if u - x_left < t2 || x_right - u < t2 {
            d = if z < midpoint { tolerance } else { -tolerance };
        }
    } else {
        e = if z < midpoint { x_right - z } else { -(z - x_left) };
        d = golden * e;
    }
    if fabs(d) >= tolerance {
        u = z + d;
    } else {
        u = z
            + (if d > 0 as libc::c_int as libc::c_double {
                tolerance
            } else {
                -tolerance
            });
    }
    (*state).e = e;
    (*state).d = d;
    f_u = (Some(((*f).function).unwrap())).unwrap()(u, (*f).params);
    if gsl_finite(f_u) == 0 {
        gsl_error(
            b"computed function value is infinite or NaN\0" as *const u8
                as *const libc::c_char,
            b"brent.c\0" as *const u8 as *const libc::c_char,
            159 as libc::c_int,
            GSL_EBADFUNC as libc::c_int,
        );
        return GSL_EBADFUNC as libc::c_int;
    }
    if f_u <= f_z {
        if u < z {
            *x_upper = z;
            *f_upper = f_z;
        } else {
            *x_lower = z;
            *f_lower = f_z;
        }
        (*state).v = w;
        (*state).f_v = f_w;
        (*state).w = z;
        (*state).f_w = f_z;
        *x_minimum = u;
        *f_minimum = f_u;
        return GSL_SUCCESS as libc::c_int;
    } else {
        if u < z {
            *x_lower = u;
            *f_lower = f_u;
        } else {
            *x_upper = u;
            *f_upper = f_u;
        }
        if f_u <= f_w || w == z {
            (*state).v = w;
            (*state).f_v = f_w;
            (*state).w = u;
            (*state).f_w = f_u;
            return GSL_SUCCESS as libc::c_int;
        } else if f_u <= f_v || v == z || v == w {
            (*state).v = u;
            (*state).f_v = f_u;
            return GSL_SUCCESS as libc::c_int;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
static mut brent_type: gsl_min_fminimizer_type = {
    let mut init = gsl_min_fminimizer_type {
        name: b"brent\0" as *const u8 as *const libc::c_char,
        size: ::std::mem::size_of::<brent_state_t>() as libc::c_ulong,
        set: Some(
            brent_init
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut gsl_function,
                    libc::c_double,
                    libc::c_double,
                    libc::c_double,
                    libc::c_double,
                    libc::c_double,
                    libc::c_double,
                ) -> libc::c_int,
        ),
        iterate: Some(
            brent_iterate
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut gsl_function,
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_double,
                ) -> libc::c_int,
        ),
    };
    init
};
pub static mut gsl_min_fminimizer_brent: *const gsl_min_fminimizer_type = unsafe {
    &brent_type as *const gsl_min_fminimizer_type
};
