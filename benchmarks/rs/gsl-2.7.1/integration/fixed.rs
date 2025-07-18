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
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
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
pub struct gsl_function_struct {
    pub function: Option::<
        unsafe extern "C" fn(libc::c_double, *mut libc::c_void) -> libc::c_double,
    >,
    pub params: *mut libc::c_void,
}
pub type gsl_function = gsl_function_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_integration_fixed_params {
    pub alpha: libc::c_double,
    pub beta: libc::c_double,
    pub a: libc::c_double,
    pub b: libc::c_double,
    pub zemu: libc::c_double,
    pub shft: libc::c_double,
    pub slp: libc::c_double,
    pub al: libc::c_double,
    pub be: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_integration_fixed_type {
    pub check: Option::<
        unsafe extern "C" fn(size_t, *const gsl_integration_fixed_params) -> libc::c_int,
    >,
    pub init: Option::<
        unsafe extern "C" fn(
            size_t,
            *mut libc::c_double,
            *mut libc::c_double,
            *mut gsl_integration_fixed_params,
        ) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_integration_fixed_workspace {
    pub n: size_t,
    pub weights: *mut libc::c_double,
    pub x: *mut libc::c_double,
    pub diag: *mut libc::c_double,
    pub subdiag: *mut libc::c_double,
    pub type_0: *const gsl_integration_fixed_type,
}
pub unsafe extern "C" fn gsl_integration_fixed_alloc(
    mut type_0: *const gsl_integration_fixed_type,
    n: size_t,
    a: libc::c_double,
    b: libc::c_double,
    alpha: libc::c_double,
    beta: libc::c_double,
) -> *mut gsl_integration_fixed_workspace {
    let mut status: libc::c_int = 0;
    let mut w: *mut gsl_integration_fixed_workspace = 0
        as *mut gsl_integration_fixed_workspace;
    if n < 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"workspace size n must be at least 1\0" as *const u8 as *const libc::c_char,
            b"fixed.c\0" as *const u8 as *const libc::c_char,
            43 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return 0 as *mut gsl_integration_fixed_workspace;
    }
    w = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gsl_integration_fixed_workspace>() as libc::c_ulong,
    ) as *mut gsl_integration_fixed_workspace;
    if w.is_null() {
        gsl_error(
            b"unable to allocate workspace\0" as *const u8 as *const libc::c_char,
            b"fixed.c\0" as *const u8 as *const libc::c_char,
            49 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_integration_fixed_workspace;
    }
    (*w)
        .weights = malloc(
        n.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*w).weights).is_null() {
        gsl_integration_fixed_free(w);
        gsl_error(
            b"unable to allocate weights\0" as *const u8 as *const libc::c_char,
            b"fixed.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_integration_fixed_workspace;
    }
    (*w)
        .x = malloc(
        n.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*w).x).is_null() {
        gsl_integration_fixed_free(w);
        gsl_error(
            b"unable to allocate x\0" as *const u8 as *const libc::c_char,
            b"fixed.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_integration_fixed_workspace;
    }
    (*w)
        .diag = malloc(
        n.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*w).diag).is_null() {
        gsl_integration_fixed_free(w);
        gsl_error(
            b"unable to allocate diag\0" as *const u8 as *const libc::c_char,
            b"fixed.c\0" as *const u8 as *const libc::c_char,
            70 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_integration_fixed_workspace;
    }
    (*w)
        .subdiag = malloc(
        n.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*w).subdiag).is_null() {
        gsl_integration_fixed_free(w);
        gsl_error(
            b"unable to allocate subdiag\0" as *const u8 as *const libc::c_char,
            b"fixed.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_integration_fixed_workspace;
    }
    (*w).n = n;
    (*w).type_0 = type_0;
    status = fixed_compute(a, b, alpha, beta, w);
    if status != 0 {
        gsl_integration_fixed_free(w);
        gsl_error(
            b"error in integration parameters\0" as *const u8 as *const libc::c_char,
            b"fixed.c\0" as *const u8 as *const libc::c_char,
            88 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return 0 as *mut gsl_integration_fixed_workspace;
    }
    return w;
}
pub unsafe extern "C" fn gsl_integration_fixed_free(
    mut w: *mut gsl_integration_fixed_workspace,
) {
    if !((*w).weights).is_null() {
        free((*w).weights as *mut libc::c_void);
    }
    if !((*w).x).is_null() {
        free((*w).x as *mut libc::c_void);
    }
    if !((*w).diag).is_null() {
        free((*w).diag as *mut libc::c_void);
    }
    if !((*w).subdiag).is_null() {
        free((*w).subdiag as *mut libc::c_void);
    }
    free(w as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_integration_fixed_n(
    mut w: *const gsl_integration_fixed_workspace,
) -> size_t {
    return (*w).n;
}
pub unsafe extern "C" fn gsl_integration_fixed_nodes(
    mut w: *const gsl_integration_fixed_workspace,
) -> *mut libc::c_double {
    return (*w).x;
}
pub unsafe extern "C" fn gsl_integration_fixed_weights(
    mut w: *const gsl_integration_fixed_workspace,
) -> *mut libc::c_double {
    return (*w).weights;
}
pub unsafe extern "C" fn gsl_integration_fixed(
    mut func: *const gsl_function,
    mut result: *mut libc::c_double,
    mut w: *const gsl_integration_fixed_workspace,
) -> libc::c_int {
    let n: size_t = (*w).n;
    let mut i: size_t = 0;
    let mut sum: libc::c_double = 0.0f64;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut fi: libc::c_double = (Some(((*func).function).unwrap()))
            .unwrap()(*((*w).x).offset(i as isize), (*func).params);
        sum += *((*w).weights).offset(i as isize) * fi;
        i = i.wrapping_add(1);
        i;
    }
    *result = sum;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn fixed_compute(
    a: libc::c_double,
    b: libc::c_double,
    alpha: libc::c_double,
    beta: libc::c_double,
    mut w: *mut gsl_integration_fixed_workspace,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    let n: size_t = (*w).n;
    let mut params: gsl_integration_fixed_params = gsl_integration_fixed_params {
        alpha: 0.,
        beta: 0.,
        a: 0.,
        b: 0.,
        zemu: 0.,
        shft: 0.,
        slp: 0.,
        al: 0.,
        be: 0.,
    };
    let mut i: size_t = 0;
    params.a = a;
    params.b = b;
    params.alpha = alpha;
    params.beta = beta;
    s = ((*(*w).type_0).check).unwrap()(n, &mut params);
    if s != 0 {
        return s;
    }
    s = ((*(*w).type_0).init).unwrap()(n, (*w).diag, (*w).subdiag, &mut params);
    if s != 0 {
        return s;
    }
    if params.zemu <= 0.0f64 {
        gsl_error(
            b"zeroth moment must be positive\0" as *const u8 as *const libc::c_char,
            b"fixed.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        *((*w).x).offset(i as isize) = *((*w).diag).offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    *((*w).weights).offset(0 as libc::c_int as isize) = sqrt(params.zemu);
    i = 1 as libc::c_int as size_t;
    while i < n {
        *((*w).weights).offset(i as isize) = 0.0f64;
        i = i.wrapping_add(1);
        i;
    }
    s = imtqlx(n as libc::c_int, (*w).x, (*w).subdiag, (*w).weights);
    if s != 0 {
        return s;
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        *((*w).weights)
            .offset(
                i as isize,
            ) = *((*w).weights).offset(i as isize) * *((*w).weights).offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    let mut p: libc::c_double = pow(params.slp, params.al + params.be + 1.0f64);
    let mut k: size_t = 0;
    k = 0 as libc::c_int as size_t;
    while k < n {
        *((*w).x)
            .offset(
                k as isize,
            ) = params.shft + params.slp * *((*w).x).offset(k as isize);
        *((*w).weights).offset(k as isize) = *((*w).weights).offset(k as isize) * p;
        k = k.wrapping_add(1);
        k;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn imtqlx(
    n: libc::c_int,
    mut d: *mut libc::c_double,
    mut e: *mut libc::c_double,
    mut z: *mut libc::c_double,
) -> libc::c_int {
    let mut b: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    let mut f: libc::c_double = 0.;
    let mut g: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut itn: libc::c_int = 30 as libc::c_int;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut mml: libc::c_int = 0;
    let mut p: libc::c_double = 0.;
    let mut r: libc::c_double = 0.;
    let mut s: libc::c_double = 0.;
    if n == 1 as libc::c_int {
        return GSL_SUCCESS as libc::c_int;
    }
    *e.offset((n - 1 as libc::c_int) as isize) = 0.0f64;
    l = 1 as libc::c_int;
    while l <= n {
        j = 0 as libc::c_int;
        loop {
            m = l;
            while m <= n {
                if m == n {
                    break;
                }
                if fabs(*e.offset((m - 1 as libc::c_int) as isize))
                    <= 2.2204460492503131e-16f64
                        * (fabs(*d.offset((m - 1 as libc::c_int) as isize))
                            + fabs(*d.offset(m as isize)))
                {
                    break;
                }
                m += 1;
                m;
            }
            p = *d.offset((l - 1 as libc::c_int) as isize);
            if m == l {
                break;
            }
            if itn <= j {
                return GSL_EMAXITER as libc::c_int;
            }
            j = j + 1 as libc::c_int;
            g = (*d.offset(l as isize) - p)
                / (2.0f64 * *e.offset((l - 1 as libc::c_int) as isize));
            r = sqrt(g * g + 1.0f64);
            g = *d.offset((m - 1 as libc::c_int) as isize) - p
                + *e.offset((l - 1 as libc::c_int) as isize)
                    / (g
                        + fabs(r)
                            * (if g >= 0.0f64 {
                                1 as libc::c_int
                            } else {
                                -(1 as libc::c_int)
                            }) as libc::c_double);
            s = 1.0f64;
            c = 1.0f64;
            p = 0.0f64;
            mml = m - l;
            ii = 1 as libc::c_int;
            while ii <= mml {
                i = m - ii;
                f = s * *e.offset((i - 1 as libc::c_int) as isize);
                b = c * *e.offset((i - 1 as libc::c_int) as isize);
                if fabs(g) <= fabs(f) {
                    c = g / f;
                    r = sqrt(c * c + 1.0f64);
                    *e.offset(i as isize) = f * r;
                    s = 1.0f64 / r;
                    c = c * s;
                } else {
                    s = f / g;
                    r = sqrt(s * s + 1.0f64);
                    *e.offset(i as isize) = g * r;
                    c = 1.0f64 / r;
                    s = s * c;
                }
                g = *d.offset(i as isize) - p;
                r = (*d.offset((i - 1 as libc::c_int) as isize) - g) * s
                    + 2.0f64 * c * b;
                p = s * r;
                *d.offset(i as isize) = g + p;
                g = c * r - b;
                f = *z.offset(i as isize);
                *z
                    .offset(
                        i as isize,
                    ) = s * *z.offset((i - 1 as libc::c_int) as isize) + c * f;
                *z
                    .offset(
                        (i - 1 as libc::c_int) as isize,
                    ) = c * *z.offset((i - 1 as libc::c_int) as isize) - s * f;
                ii += 1;
                ii;
            }
            *d
                .offset(
                    (l - 1 as libc::c_int) as isize,
                ) = *d.offset((l - 1 as libc::c_int) as isize) - p;
            *e.offset((l - 1 as libc::c_int) as isize) = g;
            *e.offset((m - 1 as libc::c_int) as isize) = 0.0f64;
        }
        l += 1;
        l;
    }
    ii = 2 as libc::c_int;
    while ii <= m {
        i = ii - 1 as libc::c_int;
        k = i;
        p = *d.offset((i - 1 as libc::c_int) as isize);
        j = ii;
        while j <= n {
            if *d.offset((j - 1 as libc::c_int) as isize) < p {
                k = j;
                p = *d.offset((j - 1 as libc::c_int) as isize);
            }
            j += 1;
            j;
        }
        if k != i {
            *d
                .offset(
                    (k - 1 as libc::c_int) as isize,
                ) = *d.offset((i - 1 as libc::c_int) as isize);
            *d.offset((i - 1 as libc::c_int) as isize) = p;
            p = *z.offset((i - 1 as libc::c_int) as isize);
            *z
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = *z.offset((k - 1 as libc::c_int) as isize);
            *z.offset((k - 1 as libc::c_int) as isize) = p;
        }
        ii += 1;
        ii;
    }
    return GSL_SUCCESS as libc::c_int;
}
