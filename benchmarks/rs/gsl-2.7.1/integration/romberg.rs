use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn fabs(_: libc::c_double) -> libc::c_double;
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
pub struct gsl_integration_romberg_workspace {
    pub n: size_t,
    pub work1: *mut libc::c_double,
    pub work2: *mut libc::c_double,
}
pub unsafe extern "C" fn gsl_integration_romberg_alloc(
    n: size_t,
) -> *mut gsl_integration_romberg_workspace {
    let mut w: *mut gsl_integration_romberg_workspace = 0
        as *mut gsl_integration_romberg_workspace;
    if n < 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"workspace size n must be at least 1\0" as *const u8 as *const libc::c_char,
            b"romberg.c\0" as *const u8 as *const libc::c_char,
            45 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return 0 as *mut gsl_integration_romberg_workspace;
    }
    w = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gsl_integration_romberg_workspace>() as libc::c_ulong,
    ) as *mut gsl_integration_romberg_workspace;
    if w.is_null() {
        gsl_error(
            b"unable to allocate workspace\0" as *const u8 as *const libc::c_char,
            b"romberg.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_integration_romberg_workspace;
    }
    (*w)
        .n = if n < 30 as libc::c_int as libc::c_ulong {
        n
    } else {
        30 as libc::c_int as libc::c_ulong
    };
    (*w)
        .work1 = malloc(
        ((*w).n).wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*w).work1).is_null() {
        gsl_integration_romberg_free(w);
        gsl_error(
            b"unable to allocate previous row\0" as *const u8 as *const libc::c_char,
            b"romberg.c\0" as *const u8 as *const libc::c_char,
            61 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_integration_romberg_workspace;
    }
    (*w)
        .work2 = malloc(
        ((*w).n).wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*w).work2).is_null() {
        gsl_integration_romberg_free(w);
        gsl_error(
            b"unable to allocate current row\0" as *const u8 as *const libc::c_char,
            b"romberg.c\0" as *const u8 as *const libc::c_char,
            68 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_integration_romberg_workspace;
    }
    return w;
}
pub unsafe extern "C" fn gsl_integration_romberg_free(
    mut w: *mut gsl_integration_romberg_workspace,
) {
    if !((*w).work1).is_null() {
        free((*w).work1 as *mut libc::c_void);
    }
    if !((*w).work2).is_null() {
        free((*w).work2 as *mut libc::c_void);
    }
    free(w as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_integration_romberg(
    mut f: *const gsl_function,
    a: libc::c_double,
    b: libc::c_double,
    epsabs: libc::c_double,
    epsrel: libc::c_double,
    mut result: *mut libc::c_double,
    mut neval: *mut size_t,
    mut w: *mut gsl_integration_romberg_workspace,
) -> libc::c_int {
    if epsabs < 0.0f64 {
        gsl_error(
            b"epsabs must be non-negative\0" as *const u8 as *const libc::c_char,
            b"romberg.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if epsrel < 0.0f64 {
        gsl_error(
            b"epsrel must be non-negative\0" as *const u8 as *const libc::c_char,
            b"romberg.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        let n: size_t = (*w).n;
        let mut Rp: *mut libc::c_double = &mut *((*w).work1)
            .offset(0 as libc::c_int as isize) as *mut libc::c_double;
        let mut Rc: *mut libc::c_double = &mut *((*w).work2)
            .offset(0 as libc::c_int as isize) as *mut libc::c_double;
        let mut Rtmp: *mut libc::c_double = 0 as *mut libc::c_double;
        let mut h: libc::c_double = 0.5f64 * (b - a);
        let mut i: size_t = 0;
        *Rp
            .offset(
                0 as libc::c_int as isize,
            ) = h
            * ((Some(((*f).function).unwrap())).unwrap()(a, (*f).params)
                + (Some(((*f).function).unwrap())).unwrap()(b, (*f).params));
        *neval = 2 as libc::c_int as size_t;
        i = 1 as libc::c_int as size_t;
        while i < n {
            let mut j: size_t = 0;
            let mut sum: libc::c_double = 0.0f64;
            let mut err: libc::c_double = 0.;
            let mut four_j: libc::c_double = 0.;
            let mut two_i: size_t = ((1 as libc::c_int) << i) as size_t;
            j = 1 as libc::c_int as size_t;
            while j < two_i {
                sum
                    += (Some(((*f).function).unwrap()))
                        .unwrap()(a + j as libc::c_double * h, (*f).params);
                *neval = (*neval).wrapping_add(1);
                *neval;
                j = (j as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                    as size_t as size_t;
            }
            *Rc
                .offset(
                    0 as libc::c_int as isize,
                ) = sum * h + 0.5f64 * *Rp.offset(0 as libc::c_int as isize);
            four_j = 4.0f64;
            j = 1 as libc::c_int as size_t;
            while j <= i {
                *Rc
                    .offset(
                        j as isize,
                    ) = (four_j
                    * *Rc
                        .offset(
                            j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                    - *Rp
                        .offset(
                            j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        )) / (four_j - 1.0f64);
                four_j *= 4.0f64;
                j = j.wrapping_add(1);
                j;
            }
            err = fabs(
                *Rc.offset(i as isize)
                    - *Rp
                        .offset(
                            i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ),
            );
            if err < epsabs || err < epsrel * fabs(*Rc.offset(i as isize)) {
                *result = *Rc.offset(i as isize);
                return GSL_SUCCESS as libc::c_int;
            }
            Rtmp = Rp;
            Rp = Rc;
            Rc = Rtmp;
            h *= 0.5f64;
            i = i.wrapping_add(1);
            i;
        }
        *result = *Rp.offset(n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
        return GSL_EMAXITER as libc::c_int;
    };
}
