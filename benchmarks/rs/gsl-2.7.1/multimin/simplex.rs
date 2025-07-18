use ::libc;
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_alloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_min_index(v: *const gsl_vector) -> size_t;
    fn gsl_matrix_alloc(n1: size_t, n2: size_t) -> *mut gsl_matrix;
    fn gsl_matrix_free(m: *mut gsl_matrix);
    fn gsl_matrix_get_row(
        v: *mut gsl_vector,
        m: *const gsl_matrix,
        i: size_t,
    ) -> libc::c_int;
    fn gsl_matrix_set_row(
        m: *mut gsl_matrix,
        i: size_t,
        v: *const gsl_vector,
    ) -> libc::c_int;
    fn gsl_blas_dnrm2(X: *const gsl_vector) -> libc::c_double;
    fn gsl_blas_daxpy(
        alpha: libc::c_double,
        X: *const gsl_vector,
        Y: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_finite(x: libc::c_double) -> libc::c_int;
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
pub struct gsl_multimin_function_struct {
    pub f: Option::<
        unsafe extern "C" fn(*const gsl_vector, *mut libc::c_void) -> libc::c_double,
    >,
    pub n: size_t,
    pub params: *mut libc::c_void,
}
pub type gsl_multimin_function = gsl_multimin_function_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multimin_fminimizer_type {
    pub name: *const libc::c_char,
    pub size: size_t,
    pub alloc: Option::<unsafe extern "C" fn(*mut libc::c_void, size_t) -> libc::c_int>,
    pub set: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut gsl_multimin_function,
            *const gsl_vector,
            *mut libc::c_double,
            *const gsl_vector,
        ) -> libc::c_int,
    >,
    pub iterate: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut gsl_multimin_function,
            *mut gsl_vector,
            *mut libc::c_double,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nmsimplex_state_t {
    pub x1: *mut gsl_matrix,
    pub y1: *mut gsl_vector,
    pub ws1: *mut gsl_vector,
    pub ws2: *mut gsl_vector,
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
unsafe extern "C" fn nmsimplex_move_corner(
    coeff: libc::c_double,
    mut state: *const nmsimplex_state_t,
    mut corner: size_t,
    mut xc: *mut gsl_vector,
    mut f: *const gsl_multimin_function,
) -> libc::c_double {
    let mut x1: *mut gsl_matrix = (*state).x1;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut newval: libc::c_double = 0.;
    let mut mp: libc::c_double = 0.;
    j = 0 as libc::c_int as size_t;
    while j < (*x1).size2 {
        mp = 0.0f64;
        i = 0 as libc::c_int as size_t;
        while i < (*x1).size1 {
            if i != corner {
                mp += gsl_matrix_get(x1, i, j);
            }
            i = i.wrapping_add(1);
            i;
        }
        mp
            /= ((*x1).size1).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                as libc::c_double;
        newval = mp - coeff * (mp - gsl_matrix_get(x1, corner, j));
        gsl_vector_set(xc, j, newval);
        j = j.wrapping_add(1);
        j;
    }
    newval = (Some(((*f).f).unwrap())).unwrap()(xc, (*f).params);
    return newval;
}
unsafe extern "C" fn nmsimplex_contract_by_best(
    mut state: *mut nmsimplex_state_t,
    mut best: size_t,
    mut xc: *mut gsl_vector,
    mut f: *mut gsl_multimin_function,
) -> libc::c_int {
    let mut x1: *mut gsl_matrix = (*state).x1;
    let mut y1: *mut gsl_vector = (*state).y1;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut newval: libc::c_double = 0.;
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    i = 0 as libc::c_int as size_t;
    while i < (*x1).size1 {
        if i != best {
            j = 0 as libc::c_int as size_t;
            while j < (*x1).size2 {
                newval = 0.5f64
                    * (gsl_matrix_get(x1, i, j) + gsl_matrix_get(x1, best, j));
                gsl_matrix_set(x1, i, j, newval);
                j = j.wrapping_add(1);
                j;
            }
            gsl_matrix_get_row(xc, x1, i);
            newval = (Some(((*f).f).unwrap())).unwrap()(xc, (*f).params);
            gsl_vector_set(y1, i, newval);
            if gsl_finite(newval) == 0 {
                status = GSL_EBADFUNC as libc::c_int;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return status;
}
unsafe extern "C" fn nmsimplex_calc_center(
    mut state: *const nmsimplex_state_t,
    mut mp: *mut gsl_vector,
) -> libc::c_int {
    let mut x1: *mut gsl_matrix = (*state).x1;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut val: libc::c_double = 0.;
    j = 0 as libc::c_int as size_t;
    while j < (*x1).size2 {
        val = 0.0f64;
        i = 0 as libc::c_int as size_t;
        while i < (*x1).size1 {
            val += gsl_matrix_get(x1, i, j);
            i = i.wrapping_add(1);
            i;
        }
        val /= (*x1).size1 as libc::c_double;
        gsl_vector_set(mp, j, val);
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn nmsimplex_size(
    mut state: *mut nmsimplex_state_t,
) -> libc::c_double {
    let mut s: *mut gsl_vector = (*state).ws1;
    let mut mp: *mut gsl_vector = (*state).ws2;
    let mut x1: *mut gsl_matrix = (*state).x1;
    let mut i: size_t = 0;
    let mut ss: libc::c_double = 0.0f64;
    nmsimplex_calc_center(state, mp);
    i = 0 as libc::c_int as size_t;
    while i < (*x1).size1 {
        gsl_matrix_get_row(s, x1, i);
        gsl_blas_daxpy(-1.0f64, mp, s);
        ss += gsl_blas_dnrm2(s);
        i = i.wrapping_add(1);
        i;
    }
    return ss / (*x1).size1 as libc::c_double;
}
unsafe extern "C" fn nmsimplex_alloc(
    mut vstate: *mut libc::c_void,
    mut n: size_t,
) -> libc::c_int {
    let mut state: *mut nmsimplex_state_t = vstate as *mut nmsimplex_state_t;
    if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"invalid number of parameters specified\0" as *const u8
                as *const libc::c_char,
            b"simplex.c\0" as *const u8 as *const libc::c_char,
            196 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    (*state).x1 = gsl_matrix_alloc(n.wrapping_add(1 as libc::c_int as libc::c_ulong), n);
    if ((*state).x1).is_null() {
        gsl_error(
            b"failed to allocate space for x1\0" as *const u8 as *const libc::c_char,
            b"simplex.c\0" as *const u8 as *const libc::c_char,
            203 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).y1 = gsl_vector_alloc(n.wrapping_add(1 as libc::c_int as libc::c_ulong));
    if ((*state).y1).is_null() {
        gsl_matrix_free((*state).x1);
        gsl_error(
            b"failed to allocate space for y\0" as *const u8 as *const libc::c_char,
            b"simplex.c\0" as *const u8 as *const libc::c_char,
            211 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).ws1 = gsl_vector_alloc(n);
    if ((*state).ws1).is_null() {
        gsl_matrix_free((*state).x1);
        gsl_vector_free((*state).y1);
        gsl_error(
            b"failed to allocate space for ws1\0" as *const u8 as *const libc::c_char,
            b"simplex.c\0" as *const u8 as *const libc::c_char,
            220 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).ws2 = gsl_vector_alloc(n);
    if ((*state).ws2).is_null() {
        gsl_matrix_free((*state).x1);
        gsl_vector_free((*state).y1);
        gsl_vector_free((*state).ws1);
        gsl_error(
            b"failed to allocate space for ws2\0" as *const u8 as *const libc::c_char,
            b"simplex.c\0" as *const u8 as *const libc::c_char,
            230 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn nmsimplex_set(
    mut vstate: *mut libc::c_void,
    mut f: *mut gsl_multimin_function,
    mut x: *const gsl_vector,
    mut size: *mut libc::c_double,
    mut step_size: *const gsl_vector,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut i: size_t = 0;
    let mut val: libc::c_double = 0.;
    let mut state: *mut nmsimplex_state_t = vstate as *mut nmsimplex_state_t;
    let mut xtemp: *mut gsl_vector = (*state).ws1;
    if (*xtemp).size != (*x).size {
        gsl_error(
            b"incompatible size of x\0" as *const u8 as *const libc::c_char,
            b"simplex.c\0" as *const u8 as *const libc::c_char,
            251 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if (*xtemp).size != (*step_size).size {
        gsl_error(
            b"incompatible size of step_size\0" as *const u8 as *const libc::c_char,
            b"simplex.c\0" as *const u8 as *const libc::c_char,
            256 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    val = (Some(((*f).f).unwrap())).unwrap()(x, (*f).params);
    if gsl_finite(val) == 0 {
        gsl_error(
            b"non-finite function value encountered\0" as *const u8
                as *const libc::c_char,
            b"simplex.c\0" as *const u8 as *const libc::c_char,
            265 as libc::c_int,
            GSL_EBADFUNC as libc::c_int,
        );
        return GSL_EBADFUNC as libc::c_int;
    }
    gsl_matrix_set_row((*state).x1, 0 as libc::c_int as size_t, x);
    gsl_vector_set((*state).y1, 0 as libc::c_int as size_t, val);
    i = 0 as libc::c_int as size_t;
    while i < (*x).size {
        status = gsl_vector_memcpy(xtemp, x);
        if status != 0 as libc::c_int {
            gsl_error(
                b"vector memcopy failed\0" as *const u8 as *const libc::c_char,
                b"simplex.c\0" as *const u8 as *const libc::c_char,
                279 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        val = gsl_vector_get(xtemp, i) + gsl_vector_get(step_size, i);
        gsl_vector_set(xtemp, i, val);
        val = (Some(((*f).f).unwrap())).unwrap()(xtemp, (*f).params);
        if gsl_finite(val) == 0 {
            gsl_error(
                b"non-finite function value encountered\0" as *const u8
                    as *const libc::c_char,
                b"simplex.c\0" as *const u8 as *const libc::c_char,
                288 as libc::c_int,
                GSL_EBADFUNC as libc::c_int,
            );
            return GSL_EBADFUNC as libc::c_int;
        }
        gsl_matrix_set_row(
            (*state).x1,
            i.wrapping_add(1 as libc::c_int as libc::c_ulong),
            xtemp,
        );
        gsl_vector_set(
            (*state).y1,
            i.wrapping_add(1 as libc::c_int as libc::c_ulong),
            val,
        );
        i = i.wrapping_add(1);
        i;
    }
    *size = nmsimplex_size(state);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn nmsimplex_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut nmsimplex_state_t = vstate as *mut nmsimplex_state_t;
    gsl_matrix_free((*state).x1);
    gsl_vector_free((*state).y1);
    gsl_vector_free((*state).ws1);
    gsl_vector_free((*state).ws2);
}
unsafe extern "C" fn nmsimplex_iterate(
    mut vstate: *mut libc::c_void,
    mut f: *mut gsl_multimin_function,
    mut x: *mut gsl_vector,
    mut size: *mut libc::c_double,
    mut fval: *mut libc::c_double,
) -> libc::c_int {
    let mut state: *mut nmsimplex_state_t = vstate as *mut nmsimplex_state_t;
    let mut xc: *mut gsl_vector = (*state).ws1;
    let mut xc2: *mut gsl_vector = (*state).ws2;
    let mut y1: *mut gsl_vector = (*state).y1;
    let mut x1: *mut gsl_matrix = (*state).x1;
    let mut n: size_t = (*y1).size;
    let mut i: size_t = 0;
    let mut hi: size_t = 0;
    let mut s_hi: size_t = 0;
    let mut lo: size_t = 0;
    let mut dhi: libc::c_double = 0.;
    let mut ds_hi: libc::c_double = 0.;
    let mut dlo: libc::c_double = 0.;
    let mut status: libc::c_int = 0;
    let mut val: libc::c_double = 0.;
    let mut val2: libc::c_double = 0.;
    if (*xc).size != (*x).size {
        gsl_error(
            b"incompatible size of x\0" as *const u8 as *const libc::c_char,
            b"simplex.c\0" as *const u8 as *const libc::c_char,
            340 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    dlo = gsl_vector_get(y1, 0 as libc::c_int as size_t);
    dhi = dlo;
    hi = 0 as libc::c_int as size_t;
    lo = 0 as libc::c_int as size_t;
    ds_hi = gsl_vector_get(y1, 1 as libc::c_int as size_t);
    s_hi = 1 as libc::c_int as size_t;
    i = 1 as libc::c_int as size_t;
    while i < n {
        val = gsl_vector_get(y1, i);
        if val < dlo {
            dlo = val;
            lo = i;
        } else if val > dhi {
            ds_hi = dhi;
            s_hi = hi;
            dhi = val;
            hi = i;
        } else if val > ds_hi {
            ds_hi = val;
            s_hi = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    val = nmsimplex_move_corner(-1.0f64, state, hi, xc, f);
    if gsl_finite(val) != 0 && val < gsl_vector_get(y1, lo) {
        val2 = nmsimplex_move_corner(-2.0f64, state, hi, xc2, f);
        if gsl_finite(val2) != 0 && val2 < gsl_vector_get(y1, lo) {
            gsl_matrix_set_row(x1, hi, xc2);
            gsl_vector_set(y1, hi, val2);
        } else {
            gsl_matrix_set_row(x1, hi, xc);
            gsl_vector_set(y1, hi, val);
        }
    } else if gsl_finite(val) == 0 || val > gsl_vector_get(y1, s_hi) {
        if gsl_finite(val) != 0 && val <= gsl_vector_get(y1, hi) {
            gsl_matrix_set_row(x1, hi, xc);
            gsl_vector_set(y1, hi, val);
        }
        val2 = nmsimplex_move_corner(0.5f64, state, hi, xc2, f);
        if gsl_finite(val2) != 0 && val2 <= gsl_vector_get(y1, hi) {
            gsl_matrix_set_row((*state).x1, hi, xc2);
            gsl_vector_set(y1, hi, val2);
        } else {
            status = nmsimplex_contract_by_best(state, lo, xc, f);
            if status != GSL_SUCCESS as libc::c_int {
                gsl_error(
                    b"nmsimplex_contract_by_best failed\0" as *const u8
                        as *const libc::c_char,
                    b"simplex.c\0" as *const u8 as *const libc::c_char,
                    430 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
        }
    } else {
        gsl_matrix_set_row(x1, hi, xc);
        gsl_vector_set(y1, hi, val);
    }
    lo = gsl_vector_min_index(y1);
    gsl_matrix_get_row(x, x1, lo);
    *fval = gsl_vector_get(y1, lo);
    *size = nmsimplex_size(state);
    return GSL_SUCCESS as libc::c_int;
}
static mut nmsimplex_type: gsl_multimin_fminimizer_type = {
    let mut init = gsl_multimin_fminimizer_type {
        name: b"nmsimplex\0" as *const u8 as *const libc::c_char,
        size: ::std::mem::size_of::<nmsimplex_state_t>() as libc::c_ulong,
        alloc: Some(
            nmsimplex_alloc
                as unsafe extern "C" fn(*mut libc::c_void, size_t) -> libc::c_int,
        ),
        set: Some(
            nmsimplex_set
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut gsl_multimin_function,
                    *const gsl_vector,
                    *mut libc::c_double,
                    *const gsl_vector,
                ) -> libc::c_int,
        ),
        iterate: Some(
            nmsimplex_iterate
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut gsl_multimin_function,
                    *mut gsl_vector,
                    *mut libc::c_double,
                    *mut libc::c_double,
                ) -> libc::c_int,
        ),
        free: Some(nmsimplex_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    };
    init
};
pub static mut gsl_multimin_fminimizer_nmsimplex: *const gsl_multimin_fminimizer_type = unsafe {
    &nmsimplex_type as *const gsl_multimin_fminimizer_type
};
