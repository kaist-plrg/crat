use ::libc;
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_matrix_calloc(n1: size_t, n2: size_t) -> *mut gsl_matrix;
    fn gsl_matrix_free(m: *mut gsl_matrix);
    fn gsl_matrix_memcpy(dest: *mut gsl_matrix, src: *const gsl_matrix) -> libc::c_int;
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
pub type gsl_permutation = gsl_permutation_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_permutation_struct {
    pub size: size_t,
    pub data: *mut size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dnewton_state_t {
    pub J: *mut gsl_matrix,
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
unsafe extern "C" fn dnewton_alloc(
    mut vstate: *mut libc::c_void,
    mut n: size_t,
) -> libc::c_int {
    let mut state: *mut dnewton_state_t = vstate as *mut dnewton_state_t;
    let mut p: *mut gsl_permutation = 0 as *mut gsl_permutation;
    let mut m: *mut gsl_matrix = 0 as *mut gsl_matrix;
    let mut J: *mut gsl_matrix = 0 as *mut gsl_matrix;
    m = gsl_matrix_calloc(n, n);
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for lu\0" as *const u8 as *const libc::c_char,
            b"dnewton.c\0" as *const u8 as *const libc::c_char,
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
            b"dnewton.c\0" as *const u8 as *const libc::c_char,
            75 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).permutation = p;
    J = gsl_matrix_calloc(n, n);
    if J.is_null() {
        gsl_permutation_free(p);
        gsl_matrix_free(m);
        gsl_error(
            b"failed to allocate space for d\0" as *const u8 as *const libc::c_char,
            b"dnewton.c\0" as *const u8 as *const libc::c_char,
            87 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).J = J;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn dnewton_set(
    mut vstate: *mut libc::c_void,
    mut function: *mut gsl_multiroot_function,
    mut x: *mut gsl_vector,
    mut f: *mut gsl_vector,
    mut dx: *mut gsl_vector,
) -> libc::c_int {
    let mut state: *mut dnewton_state_t = vstate as *mut dnewton_state_t;
    let mut i: size_t = 0;
    let mut n: size_t = (*function).n;
    let mut status: libc::c_int = 0;
    status = (Some(((*function).f).unwrap())).unwrap()(x, (*function).params, f);
    if status != 0 {
        return status;
    }
    status = gsl_multiroot_fdjacobian(
        function,
        x,
        f,
        1.4901161193847656e-08f64,
        (*state).J,
    );
    if status != 0 {
        return status;
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        gsl_vector_set(dx, i, 0.0f64);
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn dnewton_iterate(
    mut vstate: *mut libc::c_void,
    mut function: *mut gsl_multiroot_function,
    mut x: *mut gsl_vector,
    mut f: *mut gsl_vector,
    mut dx: *mut gsl_vector,
) -> libc::c_int {
    let mut state: *mut dnewton_state_t = vstate as *mut dnewton_state_t;
    let mut signum: libc::c_int = 0;
    let mut i: size_t = 0;
    let mut n: size_t = (*function).n;
    gsl_matrix_memcpy((*state).lu, (*state).J);
    let mut status: libc::c_int = gsl_linalg_LU_decomp(
        (*state).lu,
        (*state).permutation,
        &mut signum,
    );
    if status != 0 {
        return status;
    }
    let mut status_0: libc::c_int = gsl_linalg_LU_solve(
        (*state).lu,
        (*state).permutation,
        f,
        dx,
    );
    if status_0 != 0 {
        return status_0;
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut e: libc::c_double = gsl_vector_get(dx, i);
        let mut y: libc::c_double = gsl_vector_get(x, i);
        gsl_vector_set(dx, i, -e);
        gsl_vector_set(x, i, y - e);
        i = i.wrapping_add(1);
        i;
    }
    let mut status_1: libc::c_int = (Some(((*function).f).unwrap()))
        .unwrap()(x, (*function).params, f);
    if status_1 != GSL_SUCCESS as libc::c_int {
        return GSL_EBADFUNC as libc::c_int;
    }
    gsl_multiroot_fdjacobian(function, x, f, 1.4901161193847656e-08f64, (*state).J);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn dnewton_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut dnewton_state_t = vstate as *mut dnewton_state_t;
    gsl_matrix_free((*state).J);
    gsl_matrix_free((*state).lu);
    gsl_permutation_free((*state).permutation);
}
static mut dnewton_type: gsl_multiroot_fsolver_type = {
    let mut init = gsl_multiroot_fsolver_type {
        name: b"dnewton\0" as *const u8 as *const libc::c_char,
        size: ::std::mem::size_of::<dnewton_state_t>() as libc::c_ulong,
        alloc: Some(
            dnewton_alloc
                as unsafe extern "C" fn(*mut libc::c_void, size_t) -> libc::c_int,
        ),
        set: Some(
            dnewton_set
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut gsl_multiroot_function,
                    *mut gsl_vector,
                    *mut gsl_vector,
                    *mut gsl_vector,
                ) -> libc::c_int,
        ),
        iterate: Some(
            dnewton_iterate
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut gsl_multiroot_function,
                    *mut gsl_vector,
                    *mut gsl_vector,
                    *mut gsl_vector,
                ) -> libc::c_int,
        ),
        free: Some(dnewton_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    };
    init
};
pub static mut gsl_multiroot_fsolver_dnewton: *const gsl_multiroot_fsolver_type = unsafe {
    &dnewton_type as *const gsl_multiroot_fsolver_type
};
