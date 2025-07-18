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
    fn gsl_vector_view_array(v: *mut libc::c_double, n: size_t) -> _gsl_vector_view;
    fn gsl_linalg_solve_symm_cyc_tridiag(
        diag: *const gsl_vector,
        offdiag: *const gsl_vector,
        b: *const gsl_vector,
        x: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_linalg_solve_symm_tridiag(
        diag: *const gsl_vector,
        offdiag: *const gsl_vector,
        b: *const gsl_vector,
        x: *mut gsl_vector,
    ) -> libc::c_int;
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
pub struct _gsl_vector_view {
    pub vector: gsl_vector,
}
pub type gsl_vector_view = _gsl_vector_view;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_interp_accel {
    pub cache: size_t,
    pub miss_count: size_t,
    pub hit_count: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_interp_type {
    pub name: *const libc::c_char,
    pub min_size: libc::c_uint,
    pub alloc: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub init: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_double,
            *const libc::c_double,
            size_t,
        ) -> libc::c_int,
    >,
    pub eval: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            *const libc::c_double,
            *const libc::c_double,
            size_t,
            libc::c_double,
            *mut gsl_interp_accel,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    pub eval_deriv: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            *const libc::c_double,
            *const libc::c_double,
            size_t,
            libc::c_double,
            *mut gsl_interp_accel,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    pub eval_deriv2: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            *const libc::c_double,
            *const libc::c_double,
            size_t,
            libc::c_double,
            *mut gsl_interp_accel,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    pub eval_integ: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            *const libc::c_double,
            *const libc::c_double,
            size_t,
            *mut gsl_interp_accel,
            libc::c_double,
            libc::c_double,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cspline_state_t {
    pub c: *mut libc::c_double,
    pub g: *mut libc::c_double,
    pub diag: *mut libc::c_double,
    pub offdiag: *mut libc::c_double,
}
#[inline]
unsafe extern "C" fn integ_eval(
    mut ai: libc::c_double,
    mut bi: libc::c_double,
    mut ci: libc::c_double,
    mut di: libc::c_double,
    mut xi: libc::c_double,
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    let r1: libc::c_double = a - xi;
    let r2: libc::c_double = b - xi;
    let r12: libc::c_double = r1 + r2;
    let bterm: libc::c_double = 0.5f64 * bi * r12;
    let cterm: libc::c_double = 1.0f64 / 3.0f64 * ci * (r1 * r1 + r2 * r2 + r1 * r2);
    let dterm: libc::c_double = 0.25f64 * di * r12 * (r1 * r1 + r2 * r2);
    return (b - a) * (ai + bterm + cterm + dterm);
}
#[inline]
unsafe extern "C" fn gsl_interp_bsearch(
    mut x_array: *const libc::c_double,
    mut x: libc::c_double,
    mut index_lo: size_t,
    mut index_hi: size_t,
) -> size_t {
    let mut ilo: size_t = index_lo;
    let mut ihi: size_t = index_hi;
    while ihi > ilo.wrapping_add(1 as libc::c_int as libc::c_ulong) {
        let mut i: size_t = ihi
            .wrapping_add(ilo)
            .wrapping_div(2 as libc::c_int as libc::c_ulong);
        if *x_array.offset(i as isize) > x {
            ihi = i;
        } else {
            ilo = i;
        }
    }
    return ilo;
}
#[inline]
unsafe extern "C" fn gsl_interp_accel_find(
    mut a: *mut gsl_interp_accel,
    mut xa: *const libc::c_double,
    mut len: size_t,
    mut x: libc::c_double,
) -> size_t {
    let mut x_index: size_t = (*a).cache;
    if x < *xa.offset(x_index as isize) {
        (*a).miss_count = ((*a).miss_count).wrapping_add(1);
        (*a).miss_count;
        (*a).cache = gsl_interp_bsearch(xa, x, 0 as libc::c_int as size_t, x_index);
    } else if x
        >= *xa.offset(x_index.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
    {
        (*a).miss_count = ((*a).miss_count).wrapping_add(1);
        (*a).miss_count;
        (*a)
            .cache = gsl_interp_bsearch(
            xa,
            x,
            x_index,
            len.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    } else {
        (*a).hit_count = ((*a).hit_count).wrapping_add(1);
        (*a).hit_count;
    }
    return (*a).cache;
}
unsafe extern "C" fn cspline_alloc(mut size: size_t) -> *mut libc::c_void {
    let mut state: *mut cspline_state_t = malloc(
        ::std::mem::size_of::<cspline_state_t>() as libc::c_ulong,
    ) as *mut cspline_state_t;
    if state.is_null() {
        gsl_error(
            b"failed to allocate space for state\0" as *const u8 as *const libc::c_char,
            b"cspline.c\0" as *const u8 as *const libc::c_char,
            47 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .c = malloc(
        size.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).c).is_null() {
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for c\0" as *const u8 as *const libc::c_char,
            b"cspline.c\0" as *const u8 as *const libc::c_char,
            55 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .g = malloc(
        size.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).g).is_null() {
        free((*state).c as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for g\0" as *const u8 as *const libc::c_char,
            b"cspline.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .diag = malloc(
        size.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).diag).is_null() {
        free((*state).g as *mut libc::c_void);
        free((*state).c as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for diag\0" as *const u8 as *const libc::c_char,
            b"cspline.c\0" as *const u8 as *const libc::c_char,
            74 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .offdiag = malloc(
        size.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).offdiag).is_null() {
        free((*state).diag as *mut libc::c_void);
        free((*state).g as *mut libc::c_void);
        free((*state).c as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for offdiag\0" as *const u8
                as *const libc::c_char,
            b"cspline.c\0" as *const u8 as *const libc::c_char,
            85 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    return state as *mut libc::c_void;
}
unsafe extern "C" fn cspline_init(
    mut vstate: *mut libc::c_void,
    mut xa: *const libc::c_double,
    mut ya: *const libc::c_double,
    mut size: size_t,
) -> libc::c_int {
    let mut state: *mut cspline_state_t = vstate as *mut cspline_state_t;
    let mut i: size_t = 0;
    let mut num_points: size_t = size;
    let mut max_index: size_t = num_points
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    let mut sys_size: size_t = max_index.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    *((*state).c).offset(0 as libc::c_int as isize) = 0.0f64;
    *((*state).c).offset(max_index as isize) = 0.0f64;
    i = 0 as libc::c_int as size_t;
    while i < sys_size {
        let h_i: libc::c_double = *xa
            .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            - *xa.offset(i as isize);
        let h_ip1: libc::c_double = *xa
            .offset(i.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize)
            - *xa.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
        let ydiff_i: libc::c_double = *ya
            .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            - *ya.offset(i as isize);
        let ydiff_ip1: libc::c_double = *ya
            .offset(i.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize)
            - *ya.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
        let g_i: libc::c_double = if h_i != 0.0f64 { 1.0f64 / h_i } else { 0.0f64 };
        let g_ip1: libc::c_double = if h_ip1 != 0.0f64 {
            1.0f64 / h_ip1
        } else {
            0.0f64
        };
        *((*state).offdiag).offset(i as isize) = h_ip1;
        *((*state).diag).offset(i as isize) = 2.0f64 * (h_ip1 + h_i);
        *((*state).g).offset(i as isize) = 3.0f64 * (ydiff_ip1 * g_ip1 - ydiff_i * g_i);
        i = i.wrapping_add(1);
        i;
    }
    if sys_size == 1 as libc::c_int as libc::c_ulong {
        *((*state).c)
            .offset(
                1 as libc::c_int as isize,
            ) = *((*state).g).offset(0 as libc::c_int as isize)
            / *((*state).diag).offset(0 as libc::c_int as isize);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut g_vec: gsl_vector_view = gsl_vector_view_array((*state).g, sys_size);
        let mut diag_vec: gsl_vector_view = gsl_vector_view_array(
            (*state).diag,
            sys_size,
        );
        let mut offdiag_vec: gsl_vector_view = gsl_vector_view_array(
            (*state).offdiag,
            sys_size.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        let mut solution_vec: gsl_vector_view = gsl_vector_view_array(
            ((*state).c).offset(1 as libc::c_int as isize),
            sys_size,
        );
        let mut status: libc::c_int = gsl_linalg_solve_symm_tridiag(
            &mut diag_vec.vector,
            &mut offdiag_vec.vector,
            &mut g_vec.vector,
            &mut solution_vec.vector,
        );
        return status;
    };
}
unsafe extern "C" fn cspline_init_periodic(
    mut vstate: *mut libc::c_void,
    mut xa: *const libc::c_double,
    mut ya: *const libc::c_double,
    mut size: size_t,
) -> libc::c_int {
    let mut state: *mut cspline_state_t = vstate as *mut cspline_state_t;
    let mut i: size_t = 0;
    let mut num_points: size_t = size;
    let mut max_index: size_t = num_points
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    let mut sys_size: size_t = max_index;
    if sys_size == 2 as libc::c_int as libc::c_ulong {
        let h0: libc::c_double = *xa.offset(1 as libc::c_int as isize)
            - *xa.offset(0 as libc::c_int as isize);
        let h1: libc::c_double = *xa.offset(2 as libc::c_int as isize)
            - *xa.offset(1 as libc::c_int as isize);
        let A: libc::c_double = 2.0f64 * (h0 + h1);
        let B: libc::c_double = h0 + h1;
        let mut g: [libc::c_double; 2] = [0.; 2];
        let mut det: libc::c_double = 0.;
        g[0 as libc::c_int
            as usize] = 3.0f64
            * ((*ya.offset(2 as libc::c_int as isize)
                - *ya.offset(1 as libc::c_int as isize)) / h1
                - (*ya.offset(1 as libc::c_int as isize)
                    - *ya.offset(0 as libc::c_int as isize)) / h0);
        g[1 as libc::c_int
            as usize] = 3.0f64
            * ((*ya.offset(1 as libc::c_int as isize)
                - *ya.offset(2 as libc::c_int as isize)) / h0
                - (*ya.offset(2 as libc::c_int as isize)
                    - *ya.offset(1 as libc::c_int as isize)) / h1);
        det = 3.0f64 * (h0 + h1) * (h0 + h1);
        *((*state).c)
            .offset(
                1 as libc::c_int as isize,
            ) = (A * g[0 as libc::c_int as usize] - B * g[1 as libc::c_int as usize])
            / det;
        *((*state).c)
            .offset(
                2 as libc::c_int as isize,
            ) = (-B * g[0 as libc::c_int as usize] + A * g[1 as libc::c_int as usize])
            / det;
        *((*state).c)
            .offset(
                0 as libc::c_int as isize,
            ) = *((*state).c).offset(2 as libc::c_int as isize);
        return GSL_SUCCESS as libc::c_int;
    } else {
        i = 0 as libc::c_int as size_t;
        while i < sys_size.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            let h_i: libc::c_double = *xa
                .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                - *xa.offset(i as isize);
            let h_ip1: libc::c_double = *xa
                .offset(i.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize)
                - *xa.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            let ydiff_i: libc::c_double = *ya
                .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                - *ya.offset(i as isize);
            let ydiff_ip1: libc::c_double = *ya
                .offset(i.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize)
                - *ya.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            let g_i: libc::c_double = if h_i != 0.0f64 { 1.0f64 / h_i } else { 0.0f64 };
            let g_ip1: libc::c_double = if h_ip1 != 0.0f64 {
                1.0f64 / h_ip1
            } else {
                0.0f64
            };
            *((*state).offdiag).offset(i as isize) = h_ip1;
            *((*state).diag).offset(i as isize) = 2.0f64 * (h_ip1 + h_i);
            *((*state).g)
                .offset(i as isize) = 3.0f64 * (ydiff_ip1 * g_ip1 - ydiff_i * g_i);
            i = i.wrapping_add(1);
            i;
        }
        i = sys_size.wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let h_i_0: libc::c_double = *xa
            .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            - *xa.offset(i as isize);
        let h_ip1_0: libc::c_double = *xa.offset(1 as libc::c_int as isize)
            - *xa.offset(0 as libc::c_int as isize);
        let ydiff_i_0: libc::c_double = *ya
            .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            - *ya.offset(i as isize);
        let ydiff_ip1_0: libc::c_double = *ya.offset(1 as libc::c_int as isize)
            - *ya.offset(0 as libc::c_int as isize);
        let g_i_0: libc::c_double = if h_i_0 != 0.0f64 {
            1.0f64 / h_i_0
        } else {
            0.0f64
        };
        let g_ip1_0: libc::c_double = if h_ip1_0 != 0.0f64 {
            1.0f64 / h_ip1_0
        } else {
            0.0f64
        };
        *((*state).offdiag).offset(i as isize) = h_ip1_0;
        *((*state).diag).offset(i as isize) = 2.0f64 * (h_ip1_0 + h_i_0);
        *((*state).g)
            .offset(i as isize) = 3.0f64 * (ydiff_ip1_0 * g_ip1_0 - ydiff_i_0 * g_i_0);
        let mut g_vec: gsl_vector_view = gsl_vector_view_array((*state).g, sys_size);
        let mut diag_vec: gsl_vector_view = gsl_vector_view_array(
            (*state).diag,
            sys_size,
        );
        let mut offdiag_vec: gsl_vector_view = gsl_vector_view_array(
            (*state).offdiag,
            sys_size,
        );
        let mut solution_vec: gsl_vector_view = gsl_vector_view_array(
            ((*state).c).offset(1 as libc::c_int as isize),
            sys_size,
        );
        let mut status: libc::c_int = gsl_linalg_solve_symm_cyc_tridiag(
            &mut diag_vec.vector,
            &mut offdiag_vec.vector,
            &mut g_vec.vector,
            &mut solution_vec.vector,
        );
        *((*state).c)
            .offset(
                0 as libc::c_int as isize,
            ) = *((*state).c).offset(max_index as isize);
        return status;
    };
}
unsafe extern "C" fn cspline_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut cspline_state_t = vstate as *mut cspline_state_t;
    free((*state).c as *mut libc::c_void);
    free((*state).g as *mut libc::c_void);
    free((*state).diag as *mut libc::c_void);
    free((*state).offdiag as *mut libc::c_void);
    free(state as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn coeff_calc(
    mut c_array: *const libc::c_double,
    mut dy: libc::c_double,
    mut dx: libc::c_double,
    mut index: size_t,
    mut b: *mut libc::c_double,
    mut c: *mut libc::c_double,
    mut d: *mut libc::c_double,
) {
    let c_i: libc::c_double = *c_array.offset(index as isize);
    let c_ip1: libc::c_double = *c_array
        .offset(index.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
    *b = dy / dx - dx * (c_ip1 + 2.0f64 * c_i) / 3.0f64;
    *c = c_i;
    *d = (c_ip1 - c_i) / (3.0f64 * dx);
}
unsafe extern "C" fn cspline_eval(
    mut vstate: *const libc::c_void,
    mut x_array: *const libc::c_double,
    mut y_array: *const libc::c_double,
    mut size: size_t,
    mut x: libc::c_double,
    mut a: *mut gsl_interp_accel,
    mut y: *mut libc::c_double,
) -> libc::c_int {
    let mut state: *const cspline_state_t = vstate as *const cspline_state_t;
    let mut x_lo: libc::c_double = 0.;
    let mut x_hi: libc::c_double = 0.;
    let mut dx: libc::c_double = 0.;
    let mut index: size_t = 0;
    if !a.is_null() {
        index = gsl_interp_accel_find(a, x_array, size, x);
    } else {
        index = gsl_interp_bsearch(
            x_array,
            x,
            0 as libc::c_int as size_t,
            size.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    x_hi = *x_array
        .offset(index.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
    x_lo = *x_array.offset(index as isize);
    dx = x_hi - x_lo;
    if dx > 0.0f64 {
        let y_lo: libc::c_double = *y_array.offset(index as isize);
        let y_hi: libc::c_double = *y_array
            .offset(index.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
        let dy: libc::c_double = y_hi - y_lo;
        let mut delx: libc::c_double = x - x_lo;
        let mut b_i: libc::c_double = 0.;
        let mut c_i: libc::c_double = 0.;
        let mut d_i: libc::c_double = 0.;
        coeff_calc(
            (*state).c as *const libc::c_double,
            dy,
            dx,
            index,
            &mut b_i,
            &mut c_i,
            &mut d_i,
        );
        *y = y_lo + delx * (b_i + delx * (c_i + delx * d_i));
        return GSL_SUCCESS as libc::c_int;
    } else {
        *y = 0.0f64;
        return GSL_EINVAL as libc::c_int;
    };
}
unsafe extern "C" fn cspline_eval_deriv(
    mut vstate: *const libc::c_void,
    mut x_array: *const libc::c_double,
    mut y_array: *const libc::c_double,
    mut size: size_t,
    mut x: libc::c_double,
    mut a: *mut gsl_interp_accel,
    mut dydx: *mut libc::c_double,
) -> libc::c_int {
    let mut state: *const cspline_state_t = vstate as *const cspline_state_t;
    let mut x_lo: libc::c_double = 0.;
    let mut x_hi: libc::c_double = 0.;
    let mut dx: libc::c_double = 0.;
    let mut index: size_t = 0;
    if !a.is_null() {
        index = gsl_interp_accel_find(a, x_array, size, x);
    } else {
        index = gsl_interp_bsearch(
            x_array,
            x,
            0 as libc::c_int as size_t,
            size.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    x_hi = *x_array
        .offset(index.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
    x_lo = *x_array.offset(index as isize);
    dx = x_hi - x_lo;
    if dx > 0.0f64 {
        let y_lo: libc::c_double = *y_array.offset(index as isize);
        let y_hi: libc::c_double = *y_array
            .offset(index.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
        let dy: libc::c_double = y_hi - y_lo;
        let mut delx: libc::c_double = x - x_lo;
        let mut b_i: libc::c_double = 0.;
        let mut c_i: libc::c_double = 0.;
        let mut d_i: libc::c_double = 0.;
        coeff_calc(
            (*state).c as *const libc::c_double,
            dy,
            dx,
            index,
            &mut b_i,
            &mut c_i,
            &mut d_i,
        );
        *dydx = b_i + delx * (2.0f64 * c_i + 3.0f64 * d_i * delx);
        return GSL_SUCCESS as libc::c_int;
    } else {
        *dydx = 0.0f64;
        return GSL_EINVAL as libc::c_int;
    };
}
unsafe extern "C" fn cspline_eval_deriv2(
    mut vstate: *const libc::c_void,
    mut x_array: *const libc::c_double,
    mut y_array: *const libc::c_double,
    mut size: size_t,
    mut x: libc::c_double,
    mut a: *mut gsl_interp_accel,
    mut y_pp: *mut libc::c_double,
) -> libc::c_int {
    let mut state: *const cspline_state_t = vstate as *const cspline_state_t;
    let mut x_lo: libc::c_double = 0.;
    let mut x_hi: libc::c_double = 0.;
    let mut dx: libc::c_double = 0.;
    let mut index: size_t = 0;
    if !a.is_null() {
        index = gsl_interp_accel_find(a, x_array, size, x);
    } else {
        index = gsl_interp_bsearch(
            x_array,
            x,
            0 as libc::c_int as size_t,
            size.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    x_hi = *x_array
        .offset(index.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
    x_lo = *x_array.offset(index as isize);
    dx = x_hi - x_lo;
    if dx > 0.0f64 {
        let y_lo: libc::c_double = *y_array.offset(index as isize);
        let y_hi: libc::c_double = *y_array
            .offset(index.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
        let dy: libc::c_double = y_hi - y_lo;
        let mut delx: libc::c_double = x - x_lo;
        let mut b_i: libc::c_double = 0.;
        let mut c_i: libc::c_double = 0.;
        let mut d_i: libc::c_double = 0.;
        coeff_calc(
            (*state).c as *const libc::c_double,
            dy,
            dx,
            index,
            &mut b_i,
            &mut c_i,
            &mut d_i,
        );
        *y_pp = 2.0f64 * c_i + 6.0f64 * d_i * delx;
        return GSL_SUCCESS as libc::c_int;
    } else {
        *y_pp = 0.0f64;
        return GSL_EINVAL as libc::c_int;
    };
}
unsafe extern "C" fn cspline_eval_integ(
    mut vstate: *const libc::c_void,
    mut x_array: *const libc::c_double,
    mut y_array: *const libc::c_double,
    mut size: size_t,
    mut acc: *mut gsl_interp_accel,
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut result: *mut libc::c_double,
) -> libc::c_int {
    let mut state: *const cspline_state_t = vstate as *const cspline_state_t;
    let mut i: size_t = 0;
    let mut index_a: size_t = 0;
    let mut index_b: size_t = 0;
    if !acc.is_null() {
        index_a = gsl_interp_accel_find(acc, x_array, size, a);
        index_b = gsl_interp_accel_find(acc, x_array, size, b);
    } else {
        index_a = gsl_interp_bsearch(
            x_array,
            a,
            0 as libc::c_int as size_t,
            size.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        index_b = gsl_interp_bsearch(
            x_array,
            b,
            0 as libc::c_int as size_t,
            size.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    *result = 0.0f64;
    i = index_a;
    while i <= index_b {
        let x_hi: libc::c_double = *x_array
            .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
        let x_lo: libc::c_double = *x_array.offset(i as isize);
        let y_lo: libc::c_double = *y_array.offset(i as isize);
        let y_hi: libc::c_double = *y_array
            .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
        let dx: libc::c_double = x_hi - x_lo;
        let dy: libc::c_double = y_hi - y_lo;
        if dx != 0.0f64 {
            let mut b_i: libc::c_double = 0.;
            let mut c_i: libc::c_double = 0.;
            let mut d_i: libc::c_double = 0.;
            coeff_calc(
                (*state).c as *const libc::c_double,
                dy,
                dx,
                i,
                &mut b_i,
                &mut c_i,
                &mut d_i,
            );
            if i == index_a || i == index_b {
                let mut x1: libc::c_double = if i == index_a { a } else { x_lo };
                let mut x2: libc::c_double = if i == index_b { b } else { x_hi };
                *result += integ_eval(y_lo, b_i, c_i, d_i, x_lo, x1, x2);
            } else {
                *result
                    += dx
                        * (y_lo
                            + dx
                                * (0.5f64 * b_i
                                    + dx * (c_i / 3.0f64 + 0.25f64 * d_i * dx)));
            }
        } else {
            *result = 0.0f64;
            return GSL_EINVAL as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
static mut cspline_type: gsl_interp_type = {
    let mut init = gsl_interp_type {
        name: b"cspline\0" as *const u8 as *const libc::c_char,
        min_size: 3 as libc::c_int as libc::c_uint,
        alloc: Some(cspline_alloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
        init: Some(
            cspline_init
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_double,
                    *const libc::c_double,
                    size_t,
                ) -> libc::c_int,
        ),
        eval: Some(
            cspline_eval
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_double,
                    *const libc::c_double,
                    size_t,
                    libc::c_double,
                    *mut gsl_interp_accel,
                    *mut libc::c_double,
                ) -> libc::c_int,
        ),
        eval_deriv: Some(
            cspline_eval_deriv
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_double,
                    *const libc::c_double,
                    size_t,
                    libc::c_double,
                    *mut gsl_interp_accel,
                    *mut libc::c_double,
                ) -> libc::c_int,
        ),
        eval_deriv2: Some(
            cspline_eval_deriv2
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_double,
                    *const libc::c_double,
                    size_t,
                    libc::c_double,
                    *mut gsl_interp_accel,
                    *mut libc::c_double,
                ) -> libc::c_int,
        ),
        eval_integ: Some(
            cspline_eval_integ
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_double,
                    *const libc::c_double,
                    size_t,
                    *mut gsl_interp_accel,
                    libc::c_double,
                    libc::c_double,
                    *mut libc::c_double,
                ) -> libc::c_int,
        ),
        free: Some(cspline_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    };
    init
};
pub static mut gsl_interp_cspline: *const gsl_interp_type = unsafe {
    &cspline_type as *const gsl_interp_type
};
static mut cspline_periodic_type: gsl_interp_type = {
    let mut init = gsl_interp_type {
        name: b"cspline-periodic\0" as *const u8 as *const libc::c_char,
        min_size: 2 as libc::c_int as libc::c_uint,
        alloc: Some(cspline_alloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
        init: Some(
            cspline_init_periodic
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_double,
                    *const libc::c_double,
                    size_t,
                ) -> libc::c_int,
        ),
        eval: Some(
            cspline_eval
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_double,
                    *const libc::c_double,
                    size_t,
                    libc::c_double,
                    *mut gsl_interp_accel,
                    *mut libc::c_double,
                ) -> libc::c_int,
        ),
        eval_deriv: Some(
            cspline_eval_deriv
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_double,
                    *const libc::c_double,
                    size_t,
                    libc::c_double,
                    *mut gsl_interp_accel,
                    *mut libc::c_double,
                ) -> libc::c_int,
        ),
        eval_deriv2: Some(
            cspline_eval_deriv2
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_double,
                    *const libc::c_double,
                    size_t,
                    libc::c_double,
                    *mut gsl_interp_accel,
                    *mut libc::c_double,
                ) -> libc::c_int,
        ),
        eval_integ: Some(
            cspline_eval_integ
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_double,
                    *const libc::c_double,
                    size_t,
                    *mut gsl_interp_accel,
                    libc::c_double,
                    libc::c_double,
                    *mut libc::c_double,
                ) -> libc::c_int,
        ),
        free: Some(cspline_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    };
    init
};
pub static mut gsl_interp_cspline_periodic: *const gsl_interp_type = unsafe {
    &cspline_periodic_type as *const gsl_interp_type
};
