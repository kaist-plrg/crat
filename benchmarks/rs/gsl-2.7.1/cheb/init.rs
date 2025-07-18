use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn cos(_: libc::c_double) -> libc::c_double;
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
pub struct gsl_cheb_series_struct {
    pub c: *mut libc::c_double,
    pub order: size_t,
    pub a: libc::c_double,
    pub b: libc::c_double,
    pub order_sp: size_t,
    pub f: *mut libc::c_double,
}
pub type gsl_cheb_series = gsl_cheb_series_struct;
pub unsafe extern "C" fn gsl_cheb_alloc(order: size_t) -> *mut gsl_cheb_series {
    let mut cs: *mut gsl_cheb_series = malloc(
        ::std::mem::size_of::<gsl_cheb_series>() as libc::c_ulong,
    ) as *mut gsl_cheb_series;
    if cs.is_null() {
        gsl_error(
            b"failed to allocate gsl_cheb_series struct\0" as *const u8
                as *const libc::c_char,
            b"init.c\0" as *const u8 as *const libc::c_char,
            35 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_cheb_series;
    }
    (*cs).order = order;
    (*cs).order_sp = order;
    (*cs)
        .c = malloc(
        order
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*cs).c).is_null() {
        gsl_error(
            b"failed to allocate cheb coefficients\0" as *const u8
                as *const libc::c_char,
            b"init.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_cheb_series;
    }
    (*cs)
        .f = malloc(
        order
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*cs).f).is_null() {
        gsl_error(
            b"failed to allocate cheb function space\0" as *const u8
                as *const libc::c_char,
            b"init.c\0" as *const u8 as *const libc::c_char,
            50 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_cheb_series;
    }
    return cs;
}
pub unsafe extern "C" fn gsl_cheb_free(mut cs: *mut gsl_cheb_series) {
    if cs.is_null() {
        return;
    }
    free((*cs).f as *mut libc::c_void);
    free((*cs).c as *mut libc::c_void);
    free(cs as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_cheb_init(
    mut cs: *mut gsl_cheb_series,
    mut func: *const gsl_function,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_int {
    let mut k: size_t = 0;
    let mut j: size_t = 0;
    if a >= b {
        gsl_error(
            b"null function interval [a,b]\0" as *const u8 as *const libc::c_char,
            b"init.c\0" as *const u8 as *const libc::c_char,
            73 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return 0 as libc::c_int;
    }
    (*cs).a = a;
    (*cs).b = b;
    let mut bma: libc::c_double = 0.5f64 * ((*cs).b - (*cs).a);
    let mut bpa: libc::c_double = 0.5f64 * ((*cs).b + (*cs).a);
    let mut fac: libc::c_double = 2.0f64 / ((*cs).order as libc::c_double + 1.0f64);
    k = 0 as libc::c_int as size_t;
    while k <= (*cs).order {
        let mut y: libc::c_double = cos(
            3.14159265358979323846f64 * (k as libc::c_double + 0.5f64)
                / ((*cs).order).wrapping_add(1 as libc::c_int as libc::c_ulong)
                    as libc::c_double,
        );
        *((*cs).f)
            .offset(
                k as isize,
            ) = (Some(((*func).function).unwrap()))
            .unwrap()(y * bma + bpa, (*func).params);
        k = k.wrapping_add(1);
        k;
    }
    j = 0 as libc::c_int as size_t;
    while j <= (*cs).order {
        let mut sum: libc::c_double = 0.0f64;
        k = 0 as libc::c_int as size_t;
        while k <= (*cs).order {
            sum
                += *((*cs).f).offset(k as isize)
                    * cos(
                        3.14159265358979323846f64 * j as libc::c_double
                            * (k as libc::c_double + 0.5f64)
                            / ((*cs).order)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as libc::c_double,
                    );
            k = k.wrapping_add(1);
            k;
        }
        *((*cs).c).offset(j as isize) = fac * sum;
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_cheb_order(mut cs: *const gsl_cheb_series) -> size_t {
    return (*cs).order;
}
pub unsafe extern "C" fn gsl_cheb_size(mut cs: *const gsl_cheb_series) -> size_t {
    return ((*cs).order).wrapping_add(1 as libc::c_int as libc::c_ulong);
}
pub unsafe extern "C" fn gsl_cheb_coeffs(
    mut cs: *const gsl_cheb_series,
) -> *mut libc::c_double {
    return (*cs).c;
}
