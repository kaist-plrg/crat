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
    fn gsl_matrix_memcpy(dest: *mut gsl_matrix, src: *const gsl_matrix) -> libc::c_int;
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
    fn gsl_permutation_free(p: *mut gsl_permutation);
    fn gsl_permutation_calloc(n: size_t) -> *mut gsl_permutation;
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
pub struct gsl_multiroot_function_fdf_struct {
    pub f: Option::<
        unsafe extern "C" fn(
            *const gsl_vector,
            *mut libc::c_void,
            *mut gsl_vector,
        ) -> libc::c_int,
    >,
    pub df: Option::<
        unsafe extern "C" fn(
            *const gsl_vector,
            *mut libc::c_void,
            *mut gsl_matrix,
        ) -> libc::c_int,
    >,
    pub fdf: Option::<
        unsafe extern "C" fn(
            *const gsl_vector,
            *mut libc::c_void,
            *mut gsl_vector,
            *mut gsl_matrix,
        ) -> libc::c_int,
    >,
    pub n: size_t,
    pub params: *mut libc::c_void,
}
pub type gsl_multiroot_function_fdf = gsl_multiroot_function_fdf_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multiroot_fdfsolver_type {
    pub name: *const libc::c_char,
    pub size: size_t,
    pub alloc: Option::<unsafe extern "C" fn(*mut libc::c_void, size_t) -> libc::c_int>,
    pub set: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut gsl_multiroot_function_fdf,
            *mut gsl_vector,
            *mut gsl_vector,
            *mut gsl_matrix,
            *mut gsl_vector,
        ) -> libc::c_int,
    >,
    pub iterate: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut gsl_multiroot_function_fdf,
            *mut gsl_vector,
            *mut gsl_vector,
            *mut gsl_matrix,
            *mut gsl_vector,
        ) -> libc::c_int,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
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
pub struct gnewton_state_t {
    pub phi: libc::c_double,
    pub x_trial: *mut gsl_vector,
    pub d: *mut gsl_vector,
    pub lu: *mut gsl_matrix,
    pub permutation: *mut gsl_permutation,
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
unsafe extern "C" fn gnewton_alloc(
    mut vstate: *mut libc::c_void,
    mut n: size_t,
) -> libc::c_int {
    let mut state: *mut gnewton_state_t = vstate as *mut gnewton_state_t;
    let mut d: *mut gsl_vector = 0 as *mut gsl_vector;
    let mut x_trial: *mut gsl_vector = 0 as *mut gsl_vector;
    let mut p: *mut gsl_permutation = 0 as *mut gsl_permutation;
    let mut m: *mut gsl_matrix = 0 as *mut gsl_matrix;
    m = gsl_matrix_calloc(n, n);
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for lu\0" as *const u8 as *const libc::c_char,
            b"gnewton.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).lu = m;
    p = gsl_permutation_calloc(n);
    if p.is_null() {
        gsl_matrix_free(m);
        gsl_error(
            b"failed to allocate space for permutation\0" as *const u8
                as *const libc::c_char,
            b"gnewton.c\0" as *const u8 as *const libc::c_char,
            75 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).permutation = p;
    d = gsl_vector_calloc(n);
    if d.is_null() {
        gsl_permutation_free(p);
        gsl_matrix_free(m);
        gsl_error(
            b"failed to allocate space for d\0" as *const u8 as *const libc::c_char,
            b"gnewton.c\0" as *const u8 as *const libc::c_char,
            87 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).d = d;
    x_trial = gsl_vector_calloc(n);
    if x_trial.is_null() {
        gsl_vector_free(d);
        gsl_permutation_free(p);
        gsl_matrix_free(m);
        gsl_error(
            b"failed to allocate space for x_trial\0" as *const u8
                as *const libc::c_char,
            b"gnewton.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).x_trial = x_trial;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn gnewton_set(
    mut vstate: *mut libc::c_void,
    mut FDF: *mut gsl_multiroot_function_fdf,
    mut x: *mut gsl_vector,
    mut f: *mut gsl_vector,
    mut J: *mut gsl_matrix,
    mut dx: *mut gsl_vector,
) -> libc::c_int {
    let mut state: *mut gnewton_state_t = vstate as *mut gnewton_state_t;
    let mut i: size_t = 0;
    let mut n: size_t = (*FDF).n;
    (Some(((*FDF).fdf).unwrap())).unwrap()(x, (*FDF).params, f, J);
    i = 0 as libc::c_int as size_t;
    while i < n {
        gsl_vector_set(dx, i, 0.0f64);
        i = i.wrapping_add(1);
        i;
    }
    (*state).phi = enorm(f);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn gnewton_iterate(
    mut vstate: *mut libc::c_void,
    mut fdf: *mut gsl_multiroot_function_fdf,
    mut x: *mut gsl_vector,
    mut f: *mut gsl_vector,
    mut J: *mut gsl_matrix,
    mut dx: *mut gsl_vector,
) -> libc::c_int {
    let mut state: *mut gnewton_state_t = vstate as *mut gnewton_state_t;
    let mut signum: libc::c_int = 0;
    let mut t: libc::c_double = 0.;
    let mut phi0: libc::c_double = 0.;
    let mut phi1: libc::c_double = 0.;
    let mut i: size_t = 0;
    let mut n: size_t = (*fdf).n;
    gsl_matrix_memcpy((*state).lu, J);
    gsl_linalg_LU_decomp((*state).lu, (*state).permutation, &mut signum);
    let mut status: libc::c_int = gsl_linalg_LU_solve(
        (*state).lu,
        (*state).permutation,
        f,
        (*state).d,
    );
    if status != 0 {
        return status;
    }
    t = 1 as libc::c_int as libc::c_double;
    phi0 = (*state).phi;
    loop {
        i = 0 as libc::c_int as size_t;
        while i < n {
            let mut di: libc::c_double = gsl_vector_get((*state).d, i);
            let mut xi: libc::c_double = gsl_vector_get(x, i);
            gsl_vector_set((*state).x_trial, i, xi - t * di);
            i = i.wrapping_add(1);
            i;
        }
        let mut status_0: libc::c_int = (Some(((*fdf).f).unwrap()))
            .unwrap()((*state).x_trial, (*fdf).params, f);
        if status_0 != GSL_SUCCESS as libc::c_int {
            return GSL_EBADFUNC as libc::c_int;
        }
        phi1 = enorm(f);
        if !(phi1 > phi0 && t > 2.2204460492503131e-16f64) {
            break;
        }
        let mut theta: libc::c_double = phi1 / phi0;
        let mut u: libc::c_double = (sqrt(1.0f64 + 6.0f64 * theta) - 1.0f64)
            / (3.0f64 * theta);
        t *= u;
    }
    gsl_vector_memcpy(x, (*state).x_trial);
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut di_0: libc::c_double = gsl_vector_get((*state).d, i);
        gsl_vector_set(dx, i, -t * di_0);
        i = i.wrapping_add(1);
        i;
    }
    let mut status_1: libc::c_int = (Some(((*fdf).df).unwrap()))
        .unwrap()(x, (*fdf).params, J);
    if status_1 != GSL_SUCCESS as libc::c_int {
        return GSL_EBADFUNC as libc::c_int;
    }
    (*state).phi = phi1;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn gnewton_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut gnewton_state_t = vstate as *mut gnewton_state_t;
    gsl_vector_free((*state).d);
    gsl_vector_free((*state).x_trial);
    gsl_matrix_free((*state).lu);
    gsl_permutation_free((*state).permutation);
}
static mut gnewton_type: gsl_multiroot_fdfsolver_type = {
    let mut init = gsl_multiroot_fdfsolver_type {
        name: b"gnewton\0" as *const u8 as *const libc::c_char,
        size: ::std::mem::size_of::<gnewton_state_t>() as libc::c_ulong,
        alloc: Some(
            gnewton_alloc
                as unsafe extern "C" fn(*mut libc::c_void, size_t) -> libc::c_int,
        ),
        set: Some(
            gnewton_set
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut gsl_multiroot_function_fdf,
                    *mut gsl_vector,
                    *mut gsl_vector,
                    *mut gsl_matrix,
                    *mut gsl_vector,
                ) -> libc::c_int,
        ),
        iterate: Some(
            gnewton_iterate
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut gsl_multiroot_function_fdf,
                    *mut gsl_vector,
                    *mut gsl_vector,
                    *mut gsl_matrix,
                    *mut gsl_vector,
                ) -> libc::c_int,
        ),
        free: Some(gnewton_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    };
    init
};
pub static mut gsl_multiroot_fdfsolver_gnewton: *const gsl_multiroot_fdfsolver_type = unsafe {
    &gnewton_type as *const gsl_multiroot_fdfsolver_type
};
