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
pub struct gsl_root_fsolver_type {
    pub name: *const libc::c_char,
    pub size: size_t,
    pub set: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut gsl_function,
            *mut libc::c_double,
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
        ) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brent_state_t {
    pub a: libc::c_double,
    pub b: libc::c_double,
    pub c: libc::c_double,
    pub d: libc::c_double,
    pub e: libc::c_double,
    pub fa: libc::c_double,
    pub fb: libc::c_double,
    pub fc: libc::c_double,
}
unsafe extern "C" fn brent_init(
    mut vstate: *mut libc::c_void,
    mut f: *mut gsl_function,
    mut root: *mut libc::c_double,
    mut x_lower: libc::c_double,
    mut x_upper: libc::c_double,
) -> libc::c_int {
    let mut state: *mut brent_state_t = vstate as *mut brent_state_t;
    let mut f_lower: libc::c_double = 0.;
    let mut f_upper: libc::c_double = 0.;
    *root = 0.5f64 * (x_lower + x_upper);
    f_lower = (Some(((*f).function).unwrap())).unwrap()(x_lower, (*f).params);
    if gsl_finite(f_lower) == 0 {
        gsl_error(
            b"function value is not finite\0" as *const u8 as *const libc::c_char,
            b"brent.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EBADFUNC as libc::c_int,
        );
        return GSL_EBADFUNC as libc::c_int;
    }
    f_upper = (Some(((*f).function).unwrap())).unwrap()(x_upper, (*f).params);
    if gsl_finite(f_upper) == 0 {
        gsl_error(
            b"function value is not finite\0" as *const u8 as *const libc::c_char,
            b"brent.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EBADFUNC as libc::c_int,
        );
        return GSL_EBADFUNC as libc::c_int;
    }
    (*state).a = x_lower;
    (*state).fa = f_lower;
    (*state).b = x_upper;
    (*state).fb = f_upper;
    (*state).c = x_upper;
    (*state).fc = f_upper;
    (*state).d = x_upper - x_lower;
    (*state).e = x_upper - x_lower;
    if f_lower < 0.0f64 && f_upper < 0.0f64 || f_lower > 0.0f64 && f_upper > 0.0f64 {
        gsl_error(
            b"endpoints do not straddle y=0\0" as *const u8 as *const libc::c_char,
            b"brent.c\0" as *const u8 as *const libc::c_char,
            74 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn brent_iterate(
    mut vstate: *mut libc::c_void,
    mut f: *mut gsl_function,
    mut root: *mut libc::c_double,
    mut x_lower: *mut libc::c_double,
    mut x_upper: *mut libc::c_double,
) -> libc::c_int {
    let mut state: *mut brent_state_t = vstate as *mut brent_state_t;
    let mut tol: libc::c_double = 0.;
    let mut m: libc::c_double = 0.;
    let mut ac_equal: libc::c_int = 0 as libc::c_int;
    let mut a: libc::c_double = (*state).a;
    let mut b: libc::c_double = (*state).b;
    let mut c: libc::c_double = (*state).c;
    let mut fa: libc::c_double = (*state).fa;
    let mut fb: libc::c_double = (*state).fb;
    let mut fc: libc::c_double = (*state).fc;
    let mut d: libc::c_double = (*state).d;
    let mut e: libc::c_double = (*state).e;
    if fb < 0 as libc::c_int as libc::c_double && fc < 0 as libc::c_int as libc::c_double
        || fb > 0 as libc::c_int as libc::c_double
            && fc > 0 as libc::c_int as libc::c_double
    {
        ac_equal = 1 as libc::c_int;
        c = a;
        fc = fa;
        d = b - a;
        e = b - a;
    }
    if fabs(fc) < fabs(fb) {
        ac_equal = 1 as libc::c_int;
        a = b;
        b = c;
        c = a;
        fa = fb;
        fb = fc;
        fc = fa;
    }
    tol = 0.5f64 * 2.2204460492503131e-16f64 * fabs(b);
    m = 0.5f64 * (c - b);
    if fb == 0 as libc::c_int as libc::c_double {
        *root = b;
        *x_lower = b;
        *x_upper = b;
        return GSL_SUCCESS as libc::c_int;
    }
    if fabs(m) <= tol {
        *root = b;
        if b < c {
            *x_lower = b;
            *x_upper = c;
        } else {
            *x_lower = c;
            *x_upper = b;
        }
        return GSL_SUCCESS as libc::c_int;
    }
    if fabs(e) < tol || fabs(fa) <= fabs(fb) {
        d = m;
        e = m;
    } else {
        let mut p: libc::c_double = 0.;
        let mut q: libc::c_double = 0.;
        let mut r: libc::c_double = 0.;
        let mut s: libc::c_double = fb / fa;
        if ac_equal != 0 {
            p = 2 as libc::c_int as libc::c_double * m * s;
            q = 1 as libc::c_int as libc::c_double - s;
        } else {
            q = fa / fc;
            r = fb / fc;
            p = s
                * (2 as libc::c_int as libc::c_double * m * q * (q - r)
                    - (b - a) * (r - 1 as libc::c_int as libc::c_double));
            q = (q - 1 as libc::c_int as libc::c_double)
                * (r - 1 as libc::c_int as libc::c_double)
                * (s - 1 as libc::c_int as libc::c_double);
        }
        if p > 0 as libc::c_int as libc::c_double {
            q = -q;
        } else {
            p = -p;
        }
        if 2 as libc::c_int as libc::c_double * p
            < (if 3 as libc::c_int as libc::c_double * m * q - fabs(tol * q)
                < fabs(e * q)
            {
                3 as libc::c_int as libc::c_double * m * q - fabs(tol * q)
            } else {
                fabs(e * q)
            })
        {
            e = d;
            d = p / q;
        } else {
            d = m;
            e = m;
        }
    }
    a = b;
    fa = fb;
    if fabs(d) > tol {
        b += d;
    } else {
        b += if m > 0 as libc::c_int as libc::c_double { tol } else { -tol };
    }
    fb = (Some(((*f).function).unwrap())).unwrap()(b, (*f).params);
    if gsl_finite(fb) == 0 {
        gsl_error(
            b"function value is not finite\0" as *const u8 as *const libc::c_char,
            b"brent.c\0" as *const u8 as *const libc::c_char,
            202 as libc::c_int,
            GSL_EBADFUNC as libc::c_int,
        );
        return GSL_EBADFUNC as libc::c_int;
    }
    (*state).a = a;
    (*state).b = b;
    (*state).c = c;
    (*state).d = d;
    (*state).e = e;
    (*state).fa = fa;
    (*state).fb = fb;
    (*state).fc = fc;
    *root = b;
    if fb < 0 as libc::c_int as libc::c_double && fc < 0 as libc::c_int as libc::c_double
        || fb > 0 as libc::c_int as libc::c_double
            && fc > 0 as libc::c_int as libc::c_double
    {
        c = a;
    }
    if b < c {
        *x_lower = b;
        *x_upper = c;
    } else {
        *x_lower = c;
        *x_upper = b;
    }
    return GSL_SUCCESS as libc::c_int;
}
static mut brent_type: gsl_root_fsolver_type = {
    let mut init = gsl_root_fsolver_type {
        name: b"brent\0" as *const u8 as *const libc::c_char,
        size: ::std::mem::size_of::<brent_state_t>() as libc::c_ulong,
        set: Some(
            brent_init
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut gsl_function,
                    *mut libc::c_double,
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
                ) -> libc::c_int,
        ),
    };
    init
};
pub static mut gsl_root_fsolver_brent: *const gsl_root_fsolver_type = unsafe {
    &brent_type as *const gsl_root_fsolver_type
};
