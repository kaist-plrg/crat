use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_calloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
    fn gsl_matrix_calloc(n1: size_t, n2: size_t) -> *mut gsl_matrix;
    fn gsl_matrix_free(m: *mut gsl_matrix);
    fn gsl_multiroot_fdjacobian(
        F: *mut gsl_multiroot_function,
        x: *const gsl_vector,
        f: *const gsl_vector,
        epsrel: libc::c_double,
        jacobian: *mut gsl_matrix,
    ) -> libc::c_int;
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
    fn gsl_linalg_LU_invert(
        LU: *const gsl_matrix,
        p: *const gsl_permutation,
        inverse: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_permutation_calloc(n: size_t) -> *mut gsl_permutation;
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
pub struct gsl_block_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
}
pub type gsl_block = gsl_block_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: libc::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multiroot_function_struct {
    pub f: Option::<
        unsafe extern "C" fn(
            *const gsl_vector,
            *mut libc::c_void,
            *mut gsl_vector,
        ) -> libc::c_int,
    >,
    pub n: size_t,
    pub params: *mut libc::c_void,
}
pub type gsl_multiroot_function = gsl_multiroot_function_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multiroot_fsolver_type {
    pub name: *const libc::c_char,
    pub size: size_t,
    pub alloc: Option::<unsafe extern "C" fn(*mut libc::c_void, size_t) -> libc::c_int>,
    pub set: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut gsl_multiroot_function,
            *mut gsl_vector,
            *mut gsl_vector,
            *mut gsl_vector,
        ) -> libc::c_int,
    >,
    pub iterate: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut gsl_multiroot_function,
            *mut gsl_vector,
            *mut gsl_vector,
            *mut gsl_vector,
        ) -> libc::c_int,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct broyden_state_t {
    pub H: *mut gsl_matrix,
    pub lu: *mut gsl_matrix,
    pub permutation: *mut gsl_permutation,
    pub v: *mut gsl_vector,
    pub w: *mut gsl_vector,
    pub y: *mut gsl_vector,
    pub p: *mut gsl_vector,
    pub fnew: *mut gsl_vector,
    pub x_trial: *mut gsl_vector,
    pub phi: libc::c_double,
}
pub type gsl_permutation = gsl_permutation_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_permutation_struct {
    pub size: size_t,
    pub data: *mut size_t,
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
unsafe extern "C" fn enorm(mut f: *const gsl_vector) -> libc::c_double {
    let mut e2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: size_t = 0;
    let mut n: size_t = (*f).size;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut fi: libc::c_double = gsl_vector_get(f, i);
        e2 += fi * fi;
        i = i.wrapping_add(1);
        i;
    }
    return sqrt(e2);
}
unsafe extern "C" fn broyden_alloc(
    mut vstate: *mut libc::c_void,
    mut n: size_t,
) -> libc::c_int {
    let mut state: *mut broyden_state_t = vstate as *mut broyden_state_t;
    let mut v: *mut gsl_vector = 0 as *mut gsl_vector;
    let mut w: *mut gsl_vector = 0 as *mut gsl_vector;
    let mut y: *mut gsl_vector = 0 as *mut gsl_vector;
    let mut fnew: *mut gsl_vector = 0 as *mut gsl_vector;
    let mut x_trial: *mut gsl_vector = 0 as *mut gsl_vector;
    let mut p: *mut gsl_vector = 0 as *mut gsl_vector;
    let mut perm: *mut gsl_permutation = 0 as *mut gsl_permutation;
    let mut m: *mut gsl_matrix = 0 as *mut gsl_matrix;
    let mut H: *mut gsl_matrix = 0 as *mut gsl_matrix;
    m = gsl_matrix_calloc(n, n);
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for lu\0" as *const u8 as *const libc::c_char,
            b"broyden.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).lu = m;
    perm = gsl_permutation_calloc(n);
    if perm.is_null() {
        gsl_matrix_free(m);
        gsl_error(
            b"failed to allocate space for permutation\0" as *const u8
                as *const libc::c_char,
            b"broyden.c\0" as *const u8 as *const libc::c_char,
            88 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).permutation = perm;
    H = gsl_matrix_calloc(n, n);
    if H.is_null() {
        gsl_permutation_free(perm);
        gsl_matrix_free(m);
        gsl_error(
            b"failed to allocate space for d\0" as *const u8 as *const libc::c_char,
            b"broyden.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).H = H;
    v = gsl_vector_calloc(n);
    if v.is_null() {
        gsl_matrix_free(H);
        gsl_permutation_free(perm);
        gsl_matrix_free(m);
        gsl_error(
            b"failed to allocate space for v\0" as *const u8 as *const libc::c_char,
            b"broyden.c\0" as *const u8 as *const libc::c_char,
            113 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).v = v;
    w = gsl_vector_calloc(n);
    if w.is_null() {
        gsl_vector_free(v);
        gsl_matrix_free(H);
        gsl_permutation_free(perm);
        gsl_matrix_free(m);
        gsl_error(
            b"failed to allocate space for w\0" as *const u8 as *const libc::c_char,
            b"broyden.c\0" as *const u8 as *const libc::c_char,
            127 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).w = w;
    y = gsl_vector_calloc(n);
    if y.is_null() {
        gsl_vector_free(w);
        gsl_vector_free(v);
        gsl_matrix_free(H);
        gsl_permutation_free(perm);
        gsl_matrix_free(m);
        gsl_error(
            b"failed to allocate space for y\0" as *const u8 as *const libc::c_char,
            b"broyden.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).y = y;
    fnew = gsl_vector_calloc(n);
    if fnew.is_null() {
        gsl_vector_free(y);
        gsl_vector_free(w);
        gsl_vector_free(v);
        gsl_matrix_free(H);
        gsl_permutation_free(perm);
        gsl_matrix_free(m);
        gsl_error(
            b"failed to allocate space for fnew\0" as *const u8 as *const libc::c_char,
            b"broyden.c\0" as *const u8 as *const libc::c_char,
            158 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).fnew = fnew;
    x_trial = gsl_vector_calloc(n);
    if x_trial.is_null() {
        gsl_vector_free(fnew);
        gsl_vector_free(y);
        gsl_vector_free(w);
        gsl_vector_free(v);
        gsl_matrix_free(H);
        gsl_permutation_free(perm);
        gsl_matrix_free(m);
        gsl_error(
            b"failed to allocate space for x_trial\0" as *const u8
                as *const libc::c_char,
            b"broyden.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).x_trial = x_trial;
    p = gsl_vector_calloc(n);
    if p.is_null() {
        gsl_vector_free(x_trial);
        gsl_vector_free(fnew);
        gsl_vector_free(y);
        gsl_vector_free(w);
        gsl_vector_free(v);
        gsl_matrix_free(H);
        gsl_permutation_free(perm);
        gsl_matrix_free(m);
        gsl_error(
            b"failed to allocate space for p\0" as *const u8 as *const libc::c_char,
            b"broyden.c\0" as *const u8 as *const libc::c_char,
            193 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).p = p;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn broyden_set(
    mut vstate: *mut libc::c_void,
    mut function: *mut gsl_multiroot_function,
    mut x: *mut gsl_vector,
    mut f: *mut gsl_vector,
    mut dx: *mut gsl_vector,
) -> libc::c_int {
    let mut state: *mut broyden_state_t = vstate as *mut broyden_state_t;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut n: size_t = (*function).n;
    let mut signum: libc::c_int = 0 as libc::c_int;
    (Some(((*function).f).unwrap())).unwrap()(x, (*function).params, f);
    gsl_multiroot_fdjacobian(function, x, f, 1.4901161193847656e-08f64, (*state).lu);
    gsl_linalg_LU_decomp((*state).lu, (*state).permutation, &mut signum);
    gsl_linalg_LU_invert((*state).lu, (*state).permutation, (*state).H);
    i = 0 as libc::c_int as size_t;
    while i < n {
        j = 0 as libc::c_int as size_t;
        while j < n {
            gsl_matrix_set((*state).H, i, j, -gsl_matrix_get((*state).H, i, j));
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        gsl_vector_set(dx, i, 0.0f64);
        i = i.wrapping_add(1);
        i;
    }
    (*state).phi = enorm(f);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn broyden_iterate(
    mut vstate: *mut libc::c_void,
    mut function: *mut gsl_multiroot_function,
    mut x: *mut gsl_vector,
    mut f: *mut gsl_vector,
    mut dx: *mut gsl_vector,
) -> libc::c_int {
    let mut state: *mut broyden_state_t = vstate as *mut broyden_state_t;
    let mut phi0: libc::c_double = 0.;
    let mut phi1: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut lambda: libc::c_double = 0.;
    let mut H: *mut gsl_matrix = (*state).H;
    let mut p: *mut gsl_vector = (*state).p;
    let mut y: *mut gsl_vector = (*state).y;
    let mut v: *mut gsl_vector = (*state).v;
    let mut w: *mut gsl_vector = (*state).w;
    let mut fnew: *mut gsl_vector = (*state).fnew;
    let mut x_trial: *mut gsl_vector = (*state).x_trial;
    let mut lu: *mut gsl_matrix = (*state).lu;
    let mut perm: *mut gsl_permutation = (*state).permutation;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut iter: size_t = 0;
    let mut n: size_t = (*function).n;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
        j = 0 as libc::c_int as size_t;
        while j < n {
            sum += gsl_matrix_get(H, i, j) * gsl_vector_get(f, j);
            j = j.wrapping_add(1);
            j;
        }
        gsl_vector_set(p, i, sum);
        i = i.wrapping_add(1);
        i;
    }
    t = 1 as libc::c_int as libc::c_double;
    iter = 0 as libc::c_int as size_t;
    phi0 = (*state).phi;
    loop {
        i = 0 as libc::c_int as size_t;
        while i < n {
            let mut pi: libc::c_double = gsl_vector_get(p, i);
            let mut xi: libc::c_double = gsl_vector_get(x, i);
            gsl_vector_set(x_trial, i, xi + t * pi);
            i = i.wrapping_add(1);
            i;
        }
        let mut status: libc::c_int = (Some(((*function).f).unwrap()))
            .unwrap()(x_trial, (*function).params, fnew);
        if status != GSL_SUCCESS as libc::c_int {
            return GSL_EBADFUNC as libc::c_int;
        }
        phi1 = enorm(fnew);
        iter = iter.wrapping_add(1);
        iter;
        if !(phi1 > phi0 && iter < 10 as libc::c_int as libc::c_ulong && t > 0.1f64) {
            break;
        }
        let mut theta: libc::c_double = phi1 / phi0;
        t *= (sqrt(1.0f64 + 6.0f64 * theta) - 1.0f64) / (3.0f64 * theta);
    }
    if phi1 > phi0 {
        let mut signum: libc::c_int = 0 as libc::c_int;
        gsl_multiroot_fdjacobian(function, x, f, 1.4901161193847656e-08f64, lu);
        i = 0 as libc::c_int as size_t;
        while i < n {
            j = 0 as libc::c_int as size_t;
            while j < n {
                gsl_matrix_set(lu, i, j, -gsl_matrix_get(lu, i, j));
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        gsl_linalg_LU_decomp(lu, perm, &mut signum);
        gsl_linalg_LU_invert(lu, perm, H);
        gsl_linalg_LU_solve(lu, perm, f, p);
        t = 1 as libc::c_int as libc::c_double;
        i = 0 as libc::c_int as size_t;
        while i < n {
            let mut pi_0: libc::c_double = gsl_vector_get(p, i);
            let mut xi_0: libc::c_double = gsl_vector_get(x, i);
            gsl_vector_set(x_trial, i, xi_0 + t * pi_0);
            i = i.wrapping_add(1);
            i;
        }
        let mut status_0: libc::c_int = (Some(((*function).f).unwrap()))
            .unwrap()(x_trial, (*function).params, fnew);
        if status_0 != GSL_SUCCESS as libc::c_int {
            return GSL_EBADFUNC as libc::c_int;
        }
        phi1 = enorm(fnew);
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut yi: libc::c_double = gsl_vector_get(fnew, i) - gsl_vector_get(f, i);
        gsl_vector_set(y, i, yi);
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut sum_0: libc::c_double = 0 as libc::c_int as libc::c_double;
        j = 0 as libc::c_int as size_t;
        while j < n {
            sum_0 += gsl_matrix_get(H, i, j) * gsl_vector_get(y, j);
            j = j.wrapping_add(1);
            j;
        }
        gsl_vector_set(v, i, sum_0);
        i = i.wrapping_add(1);
        i;
    }
    lambda = 0 as libc::c_int as libc::c_double;
    i = 0 as libc::c_int as size_t;
    while i < n {
        lambda += gsl_vector_get(p, i) * gsl_vector_get(v, i);
        i = i.wrapping_add(1);
        i;
    }
    if lambda == 0 as libc::c_int as libc::c_double {
        gsl_error(
            b"approximation to Jacobian has collapsed\0" as *const u8
                as *const libc::c_char,
            b"broyden.c\0" as *const u8 as *const libc::c_char,
            368 as libc::c_int,
            GSL_EZERODIV as libc::c_int,
        );
        return GSL_EZERODIV as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut vi: libc::c_double = gsl_vector_get(v, i) + t * gsl_vector_get(p, i);
        gsl_vector_set(v, i, vi);
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut sum_1: libc::c_double = 0 as libc::c_int as libc::c_double;
        j = 0 as libc::c_int as size_t;
        while j < n {
            sum_1 += gsl_matrix_get(H, j, i) * gsl_vector_get(p, j);
            j = j.wrapping_add(1);
            j;
        }
        gsl_vector_set(w, i, sum_1);
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut vi_0: libc::c_double = gsl_vector_get(v, i);
        j = 0 as libc::c_int as size_t;
        while j < n {
            let mut wj: libc::c_double = gsl_vector_get(w, j);
            let mut Hij: libc::c_double = gsl_matrix_get(H, i, j) - vi_0 * wj / lambda;
            gsl_matrix_set(H, i, j, Hij);
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    gsl_vector_memcpy(f, fnew);
    gsl_vector_memcpy(x, x_trial);
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut pi_1: libc::c_double = gsl_vector_get(p, i);
        gsl_vector_set(dx, i, t * pi_1);
        i = i.wrapping_add(1);
        i;
    }
    (*state).phi = phi1;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn broyden_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut broyden_state_t = vstate as *mut broyden_state_t;
    gsl_matrix_free((*state).H);
    gsl_matrix_free((*state).lu);
    gsl_permutation_free((*state).permutation);
    gsl_vector_free((*state).v);
    gsl_vector_free((*state).w);
    gsl_vector_free((*state).y);
    gsl_vector_free((*state).p);
    gsl_vector_free((*state).fnew);
    gsl_vector_free((*state).x_trial);
}
static mut broyden_type: gsl_multiroot_fsolver_type = {
    let mut init = gsl_multiroot_fsolver_type {
        name: b"broyden\0" as *const u8 as *const libc::c_char,
        size: ::std::mem::size_of::<broyden_state_t>() as libc::c_ulong,
        alloc: Some(
            broyden_alloc
                as unsafe extern "C" fn(*mut libc::c_void, size_t) -> libc::c_int,
        ),
        set: Some(
            broyden_set
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut gsl_multiroot_function,
                    *mut gsl_vector,
                    *mut gsl_vector,
                    *mut gsl_vector,
                ) -> libc::c_int,
        ),
        iterate: Some(
            broyden_iterate
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut gsl_multiroot_function,
                    *mut gsl_vector,
                    *mut gsl_vector,
                    *mut gsl_vector,
                ) -> libc::c_int,
        ),
        free: Some(broyden_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    };
    init
};
pub static mut gsl_multiroot_fsolver_broyden: *const gsl_multiroot_fsolver_type = unsafe {
    &broyden_type as *const gsl_multiroot_fsolver_type
};
