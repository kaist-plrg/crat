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
pub struct gsl_rng_type {
    pub name: *const libc::c_char,
    pub max: libc::c_ulong,
    pub min: libc::c_ulong,
    pub size: size_t,
    pub set: Option::<unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> ()>,
    pub get: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong>,
    pub get_double: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_rng {
    pub type_0: *const gsl_rng_type,
    pub state: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_ran_discrete_t {
    pub K: size_t,
    pub A: *mut size_t,
    pub F: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_stack_t {
    pub N: size_t,
    pub v: *mut size_t,
    pub i: size_t,
}
#[inline]
unsafe extern "C" fn gsl_rng_uniform(mut r: *const gsl_rng) -> libc::c_double {
    return ((*(*r).type_0).get_double).unwrap()((*r).state);
}
unsafe extern "C" fn new_stack(mut N: size_t) -> *mut gsl_stack_t {
    let mut s: *mut gsl_stack_t = 0 as *mut gsl_stack_t;
    s = malloc(::std::mem::size_of::<gsl_stack_t>() as libc::c_ulong)
        as *mut gsl_stack_t;
    (*s).N = N;
    (*s).i = 0 as libc::c_int as size_t;
    (*s)
        .v = malloc((::std::mem::size_of::<size_t>() as libc::c_ulong).wrapping_mul(N))
        as *mut size_t;
    return s;
}
unsafe extern "C" fn push_stack(mut s: *mut gsl_stack_t, mut v: size_t) -> libc::c_int {
    if (*s).i >= (*s).N {
        return -(1 as libc::c_int);
    }
    *((*s).v).offset((*s).i as isize) = v;
    (*s)
        .i = ((*s).i as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong)
        as size_t as size_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn pop_stack(mut s: *mut gsl_stack_t) -> size_t {
    if (*s).i == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"internal error - stack exhausted\0" as *const u8 as *const libc::c_char,
            b"discrete.c\0" as *const u8 as *const libc::c_char,
            191 as libc::c_int,
            GSL_ESANITY as libc::c_int,
        );
        return GSL_ESANITY as libc::c_int as size_t;
    }
    (*s)
        .i = ((*s).i as libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as size_t as size_t;
    return *((*s).v).offset((*s).i as isize);
}
#[inline]
unsafe extern "C" fn size_stack(mut s: *const gsl_stack_t) -> size_t {
    return (*s).i;
}
unsafe extern "C" fn free_stack(mut s: *mut gsl_stack_t) {
    free((*s).v as *mut libc::c_char as *mut libc::c_void);
    free(s as *mut libc::c_char as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_ran_discrete_preproc(
    mut Kevents: size_t,
    mut ProbArray: *const libc::c_double,
) -> *mut gsl_ran_discrete_t {
    let mut k: size_t = 0;
    let mut b: size_t = 0;
    let mut s: size_t = 0;
    let mut g: *mut gsl_ran_discrete_t = 0 as *mut gsl_ran_discrete_t;
    let mut nBigs: size_t = 0;
    let mut nSmalls: size_t = 0;
    let mut Bigs: *mut gsl_stack_t = 0 as *mut gsl_stack_t;
    let mut Smalls: *mut gsl_stack_t = 0 as *mut gsl_stack_t;
    let mut E: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut pTotal: libc::c_double = 0.0f64;
    let mut mean: libc::c_double = 0.;
    let mut d: libc::c_double = 0.;
    if Kevents < 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"number of events must be a positive integer\0" as *const u8
                as *const libc::c_char,
            b"discrete.c\0" as *const u8 as *const libc::c_char,
            228 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_ran_discrete_t;
    }
    k = 0 as libc::c_int as size_t;
    while k < Kevents {
        if *ProbArray.offset(k as isize) < 0 as libc::c_int as libc::c_double {
            gsl_error(
                b"probabilities must be non-negative\0" as *const u8
                    as *const libc::c_char,
                b"discrete.c\0" as *const u8 as *const libc::c_char,
                238 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut gsl_ran_discrete_t;
        }
        pTotal += *ProbArray.offset(k as isize);
        k = k.wrapping_add(1);
        k;
    }
    g = malloc(::std::mem::size_of::<gsl_ran_discrete_t>() as libc::c_ulong)
        as *mut gsl_ran_discrete_t;
    (*g).K = Kevents;
    (*g)
        .F = malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong).wrapping_mul(Kevents),
    ) as *mut libc::c_double;
    (*g)
        .A = malloc(
        (::std::mem::size_of::<size_t>() as libc::c_ulong).wrapping_mul(Kevents),
    ) as *mut size_t;
    E = malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong).wrapping_mul(Kevents),
    ) as *mut libc::c_double;
    if E.is_null() {
        gsl_error(
            b"Cannot allocate memory for randevent\0" as *const u8
                as *const libc::c_char,
            b"discrete.c\0" as *const u8 as *const libc::c_char,
            252 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_ran_discrete_t;
    }
    k = 0 as libc::c_int as size_t;
    while k < Kevents {
        *E.offset(k as isize) = *ProbArray.offset(k as isize) / pTotal;
        k = k.wrapping_add(1);
        k;
    }
    mean = 1.0f64 / Kevents as libc::c_double;
    nBigs = 0 as libc::c_int as size_t;
    nSmalls = nBigs;
    let which: *mut size_t = (*g).A;
    k = 0 as libc::c_int as size_t;
    while k < Kevents {
        if *E.offset(k as isize) < mean {
            nSmalls = nSmalls.wrapping_add(1);
            nSmalls;
            *which.offset(k as isize) = 0 as libc::c_int as size_t;
        } else {
            nBigs = nBigs.wrapping_add(1);
            nBigs;
            *which.offset(k as isize) = 1 as libc::c_int as size_t;
        }
        k = k.wrapping_add(1);
        k;
    }
    Bigs = new_stack(nBigs);
    Smalls = new_stack(nSmalls);
    k = 0 as libc::c_int as size_t;
    while k < Kevents {
        let mut Dest: *mut gsl_stack_t = if *which.offset(k as isize) != 0 {
            Bigs
        } else {
            Smalls
        };
        let mut status: libc::c_int = push_stack(Dest, k);
        if status != 0 {
            gsl_error(
                b"failed to build stacks\0" as *const u8 as *const libc::c_char,
                b"discrete.c\0" as *const u8 as *const libc::c_char,
                280 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return 0 as *mut gsl_ran_discrete_t;
        }
        k = k.wrapping_add(1);
        k;
    }
    while size_stack(Smalls) > 0 as libc::c_int as libc::c_ulong {
        s = pop_stack(Smalls);
        if size_stack(Bigs) == 0 as libc::c_int as libc::c_ulong {
            *((*g).A).offset(s as isize) = s;
            *((*g).F).offset(s as isize) = 1.0f64;
        } else {
            b = pop_stack(Bigs);
            *((*g).A).offset(s as isize) = b;
            *((*g).F)
                .offset(s as isize) = Kevents as libc::c_double * *E.offset(s as isize);
            d = mean - *E.offset(s as isize);
            *E.offset(s as isize) += d;
            *E.offset(b as isize) -= d;
            if *E.offset(b as isize) < mean {
                push_stack(Smalls, b);
            } else if *E.offset(b as isize) > mean {
                push_stack(Bigs, b);
            } else {
                *((*g).A).offset(b as isize) = b;
                *((*g).F).offset(b as isize) = 1.0f64;
            }
        }
    }
    while size_stack(Bigs) > 0 as libc::c_int as libc::c_ulong {
        b = pop_stack(Bigs);
        *((*g).A).offset(b as isize) = b;
        *((*g).F).offset(b as isize) = 1.0f64;
    }
    if size_stack(Smalls) != 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"Smalls stack has not been emptied\0" as *const u8 as *const libc::c_char,
            b"discrete.c\0" as *const u8 as *const libc::c_char,
            322 as libc::c_int,
            GSL_ESANITY as libc::c_int,
        );
        return 0 as *mut gsl_ran_discrete_t;
    }
    k = 0 as libc::c_int as size_t;
    while k < Kevents {
        *((*g).F).offset(k as isize) += k as libc::c_double;
        *((*g).F).offset(k as isize) /= Kevents as libc::c_double;
        k = k.wrapping_add(1);
        k;
    }
    free_stack(Bigs);
    free_stack(Smalls);
    free(E as *mut libc::c_char as *mut libc::c_void);
    return g;
}
pub unsafe extern "C" fn gsl_ran_discrete(
    mut r: *const gsl_rng,
    mut g: *const gsl_ran_discrete_t,
) -> size_t {
    let mut c: size_t = 0 as libc::c_int as size_t;
    let mut u: libc::c_double = 0.;
    let mut f: libc::c_double = 0.;
    u = gsl_rng_uniform(r);
    c = (u * (*g).K as libc::c_double) as size_t;
    f = *((*g).F).offset(c as isize);
    if f == 1.0f64 {
        return c;
    }
    if u < f { return c } else { return *((*g).A).offset(c as isize) };
}
pub unsafe extern "C" fn gsl_ran_discrete_free(mut g: *mut gsl_ran_discrete_t) {
    if g.is_null() {
        return;
    }
    free((*g).A as *mut libc::c_char as *mut libc::c_void);
    free((*g).F as *mut libc::c_char as *mut libc::c_void);
    free(g as *mut libc::c_char as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_ran_discrete_pdf(
    mut k: size_t,
    mut g: *const gsl_ran_discrete_t,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut K: size_t = 0;
    let mut f: libc::c_double = 0.;
    let mut p: libc::c_double = 0 as libc::c_int as libc::c_double;
    K = (*g).K;
    if k > K {
        return 0 as libc::c_int as libc::c_double;
    }
    i = 0 as libc::c_int as size_t;
    while i < K {
        f = *((*g).F).offset(i as isize);
        f = K as libc::c_double * f - i as libc::c_double;
        if i == k {
            p += f;
        } else if k == *((*g).A).offset(i as isize) {
            p += 1.0f64 - f;
        }
        i = i.wrapping_add(1);
        i;
    }
    return p / K as libc::c_double;
}
