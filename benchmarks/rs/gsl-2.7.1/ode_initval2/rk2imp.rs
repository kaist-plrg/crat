use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_odeiv2_control_errlevel(
        c: *mut gsl_odeiv2_control,
        y: libc::c_double,
        dydt: libc::c_double,
        h: libc::c_double,
        ind: size_t,
        errlev: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_matrix_free(m: *mut gsl_matrix);
    fn gsl_permutation_free(p: *mut gsl_permutation);
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_blas_dnrm2(X: *const gsl_vector) -> libc::c_double;
    fn gsl_linalg_LU_solve(
        LU: *const gsl_matrix,
        p: *const gsl_permutation,
        b: *const gsl_vector,
        x: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_vector_set_zero(v: *mut gsl_vector);
    fn gsl_linalg_LU_decomp(
        A: *mut gsl_matrix,
        p: *mut gsl_permutation,
        signum: *mut libc::c_int,
    ) -> libc::c_int;
    fn gsl_vector_alloc(n: size_t) -> *mut gsl_vector;
    fn gsl_permutation_alloc(n: size_t) -> *mut gsl_permutation;
    fn gsl_matrix_alloc(n1: size_t, n2: size_t) -> *mut gsl_matrix;
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
pub struct gsl_odeiv2_system {
    pub function: Option::<
        unsafe extern "C" fn(
            libc::c_double,
            *const libc::c_double,
            *mut libc::c_double,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub jacobian: Option::<
        unsafe extern "C" fn(
            libc::c_double,
            *const libc::c_double,
            *mut libc::c_double,
            *mut libc::c_double,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub dimension: size_t,
    pub params: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_odeiv2_step_struct {
    pub type_0: *const gsl_odeiv2_step_type,
    pub dimension: size_t,
    pub state: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_odeiv2_step_type {
    pub name: *const libc::c_char,
    pub can_use_dydt_in: libc::c_int,
    pub gives_exact_dydt_out: libc::c_int,
    pub alloc: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub apply: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            size_t,
            libc::c_double,
            libc::c_double,
            *mut libc::c_double,
            *mut libc::c_double,
            *const libc::c_double,
            *mut libc::c_double,
            *const gsl_odeiv2_system,
        ) -> libc::c_int,
    >,
    pub set_driver: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const gsl_odeiv2_driver) -> libc::c_int,
    >,
    pub reset: Option::<unsafe extern "C" fn(*mut libc::c_void, size_t) -> libc::c_int>,
    pub order: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint>,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
pub type gsl_odeiv2_driver = gsl_odeiv2_driver_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_odeiv2_driver_struct {
    pub sys: *const gsl_odeiv2_system,
    pub s: *mut gsl_odeiv2_step,
    pub c: *mut gsl_odeiv2_control,
    pub e: *mut gsl_odeiv2_evolve,
    pub h: libc::c_double,
    pub hmin: libc::c_double,
    pub hmax: libc::c_double,
    pub n: libc::c_ulong,
    pub nmax: libc::c_ulong,
}
pub type gsl_odeiv2_evolve = gsl_odeiv2_evolve_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_odeiv2_evolve_struct {
    pub dimension: size_t,
    pub y0: *mut libc::c_double,
    pub yerr: *mut libc::c_double,
    pub dydt_in: *mut libc::c_double,
    pub dydt_out: *mut libc::c_double,
    pub last_step: libc::c_double,
    pub count: libc::c_ulong,
    pub failed_steps: libc::c_ulong,
    pub driver: *const gsl_odeiv2_driver,
}
pub type gsl_odeiv2_control = gsl_odeiv2_control_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_odeiv2_control_struct {
    pub type_0: *const gsl_odeiv2_control_type,
    pub state: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_odeiv2_control_type {
    pub name: *const libc::c_char,
    pub alloc: Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
    pub init: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_double,
            libc::c_double,
            libc::c_double,
            libc::c_double,
        ) -> libc::c_int,
    >,
    pub hadjust: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            size_t,
            libc::c_uint,
            *const libc::c_double,
            *const libc::c_double,
            *const libc::c_double,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    pub errlevel: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_double,
            libc::c_double,
            libc::c_double,
            size_t,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    pub set_driver: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const gsl_odeiv2_driver) -> libc::c_int,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
pub type gsl_odeiv2_step = gsl_odeiv2_step_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rk2imp_state_t {
    pub A: *mut gsl_matrix,
    pub y_onestep: *mut libc::c_double,
    pub y_twostep: *mut libc::c_double,
    pub ytmp: *mut libc::c_double,
    pub y_save: *mut libc::c_double,
    pub YZ: *mut libc::c_double,
    pub fYZ: *mut libc::c_double,
    pub dfdy: *mut gsl_matrix,
    pub dfdt: *mut libc::c_double,
    pub esol: *mut modnewton1_state_t,
    pub errlev: *mut libc::c_double,
    pub driver: *const gsl_odeiv2_driver,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct modnewton1_state_t {
    pub IhAJ: *mut gsl_matrix,
    pub p: *mut gsl_permutation,
    pub dYk: *mut gsl_vector,
    pub dScal: *mut gsl_vector,
    pub Yk: *mut libc::c_double,
    pub fYk: *mut libc::c_double,
    pub rhs: *mut gsl_vector,
    pub eeta_prev: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: libc::c_int,
}
pub type gsl_block = gsl_block_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
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
pub struct gsl_matrix {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: libc::c_int,
}
#[inline]
unsafe extern "C" fn GSL_MAX_DBL(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return if a > b { a } else { b };
}
unsafe extern "C" fn rksubs(
    mut y: *mut libc::c_double,
    h: libc::c_double,
    mut y0: *const libc::c_double,
    mut fY: *const libc::c_double,
    mut b: *const libc::c_double,
    stage: size_t,
    dim: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < dim {
        *y.offset(i as isize) = 0.0f64;
        j = 0 as libc::c_int as size_t;
        while j < stage {
            *y.offset(i as isize)
                += *b.offset(j as isize)
                    * *fY.offset(j.wrapping_mul(dim).wrapping_add(i) as isize);
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < dim {
        *y.offset(i as isize) *= h;
        *y.offset(i as isize) += *y0.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn modnewton1_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut modnewton1_state_t = vstate as *mut modnewton1_state_t;
    gsl_vector_free((*state).rhs);
    free((*state).fYk as *mut libc::c_void);
    free((*state).Yk as *mut libc::c_void);
    gsl_vector_free((*state).dScal);
    gsl_vector_free((*state).dYk);
    gsl_permutation_free((*state).p);
    gsl_matrix_free((*state).IhAJ);
    free(state as *mut libc::c_void);
}
unsafe extern "C" fn modnewton1_solve(
    mut vstate: *mut libc::c_void,
    mut A: *const gsl_matrix,
    mut c: *const libc::c_double,
    t: libc::c_double,
    h: libc::c_double,
    mut y0: *const libc::c_double,
    mut sys: *const gsl_odeiv2_system,
    mut YZ: *mut libc::c_double,
    mut errlev: *const libc::c_double,
) -> libc::c_int {
    let mut state: *mut modnewton1_state_t = vstate as *mut modnewton1_state_t;
    let IhAJ: *mut gsl_matrix = (*state).IhAJ;
    let p: *mut gsl_permutation = (*state).p;
    let dYk: *mut gsl_vector = (*state).dYk;
    let Yk: *mut libc::c_double = (*state).Yk;
    let fYk: *mut libc::c_double = (*state).fYk;
    let rhs: *mut gsl_vector = (*state).rhs;
    let eeta_prev: *mut libc::c_double = &mut (*state).eeta_prev;
    let dScal: *mut gsl_vector = (*state).dScal;
    let dim: size_t = (*sys).dimension;
    let stage: size_t = (*A).size1;
    let mut status: libc::c_int = GSL_CONTINUE as libc::c_int;
    let mut j: size_t = 0;
    let mut m: size_t = 0;
    gsl_vector_set_zero(dYk);
    j = 0 as libc::c_int as size_t;
    while j < stage {
        m = 0 as libc::c_int as size_t;
        while m < dim {
            *Yk
                .offset(
                    j.wrapping_mul(dim).wrapping_add(m) as isize,
                ) = *y0.offset(m as isize);
            m = m.wrapping_add(1);
            m;
        }
        j = j.wrapping_add(1);
        j;
    }
    let mut iter: size_t = 0 as libc::c_int as size_t;
    let mut j_0: size_t = 0;
    let mut m_0: size_t = 0;
    let mut n: size_t = 0;
    let max_iter: size_t = 7 as libc::c_int as size_t;
    let mut dScal_norm: libc::c_double = 0.0f64;
    let mut dScal_norm_prev: libc::c_double = 0.0f64;
    while status == GSL_CONTINUE as libc::c_int && iter < max_iter {
        iter = iter.wrapping_add(1);
        iter;
        j_0 = 0 as libc::c_int as size_t;
        while j_0 < stage {
            m_0 = 0 as libc::c_int as size_t;
            while m_0 < dim {
                *Yk.offset(j_0.wrapping_mul(dim).wrapping_add(m_0) as isize)
                    += gsl_vector_get(dYk, j_0.wrapping_mul(dim).wrapping_add(m_0));
                m_0 = m_0.wrapping_add(1);
                m_0;
            }
            let mut s: libc::c_int = (Some(((*sys).function).unwrap()))
                .unwrap()(
                t + *c.offset(j_0 as isize) * h,
                &mut *Yk.offset(j_0.wrapping_mul(dim) as isize) as *mut libc::c_double
                    as *const libc::c_double,
                &mut *fYk.offset(j_0.wrapping_mul(dim) as isize),
                (*sys).params,
            );
            if s != GSL_SUCCESS as libc::c_int {
                return s;
            }
            j_0 = j_0.wrapping_add(1);
            j_0;
        }
        j_0 = 0 as libc::c_int as size_t;
        while j_0 < stage {
            m_0 = 0 as libc::c_int as size_t;
            while m_0 < dim {
                let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
                n = 0 as libc::c_int as size_t;
                while n < stage {
                    sum
                        += gsl_matrix_get(A, j_0, n)
                            * *fYk
                                .offset(n.wrapping_mul(dim).wrapping_add(m_0) as isize);
                    n = n.wrapping_add(1);
                    n;
                }
                gsl_vector_set(
                    rhs,
                    j_0.wrapping_mul(dim).wrapping_add(m_0),
                    -1.0f64
                        * *Yk.offset(j_0.wrapping_mul(dim).wrapping_add(m_0) as isize)
                        + *y0.offset(m_0 as isize) + h * sum,
                );
                m_0 = m_0.wrapping_add(1);
                m_0;
            }
            j_0 = j_0.wrapping_add(1);
            j_0;
        }
        let mut s_0: libc::c_int = gsl_linalg_LU_solve(IhAJ, p, rhs, dYk);
        if s_0 != GSL_SUCCESS as libc::c_int {
            return s_0;
        }
        let mut theta_k: libc::c_double = 0.;
        let mut eeta_k: libc::c_double = 0.0f64;
        j_0 = 0 as libc::c_int as size_t;
        while j_0 < stage {
            m_0 = 0 as libc::c_int as size_t;
            while m_0 < dim {
                gsl_vector_set(
                    dScal,
                    j_0.wrapping_mul(dim).wrapping_add(m_0),
                    gsl_vector_get(dYk, j_0.wrapping_mul(dim).wrapping_add(m_0))
                        / *errlev.offset(m_0 as isize),
                );
                m_0 = m_0.wrapping_add(1);
                m_0;
            }
            j_0 = j_0.wrapping_add(1);
            j_0;
        }
        dScal_norm_prev = dScal_norm;
        dScal_norm = gsl_blas_dnrm2(dScal);
        theta_k = dScal_norm / dScal_norm_prev;
        if iter > 1 as libc::c_int as libc::c_ulong {
            if theta_k >= 1.0f64 {
                return GSL_FAILURE as libc::c_int;
            }
            eeta_k = theta_k / (1.0f64 - theta_k);
        } else {
            eeta_k = pow(GSL_MAX_DBL(*eeta_prev, 2.2204460492503131e-16f64), 0.8f64);
        }
        *eeta_prev = eeta_k;
        if eeta_k * dScal_norm < 1.0f64 {
            status = GSL_SUCCESS as libc::c_int;
        }
    }
    if status != GSL_SUCCESS as libc::c_int {
        return GSL_FAILURE as libc::c_int
    } else {
        let mut j_1: size_t = 0;
        let mut m_1: size_t = 0;
        j_1 = 0 as libc::c_int as size_t;
        while j_1 < stage {
            m_1 = 0 as libc::c_int as size_t;
            while m_1 < dim {
                *YZ
                    .offset(
                        j_1.wrapping_mul(dim).wrapping_add(m_1) as isize,
                    ) = *Yk.offset(j_1.wrapping_mul(dim).wrapping_add(m_1) as isize)
                    + gsl_vector_get(dYk, j_1.wrapping_mul(dim).wrapping_add(m_1));
                m_1 = m_1.wrapping_add(1);
                m_1;
            }
            j_1 = j_1.wrapping_add(1);
            j_1;
        }
        return status;
    };
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
unsafe extern "C" fn modnewton1_init(
    mut vstate: *mut libc::c_void,
    mut A: *const gsl_matrix,
    h: libc::c_double,
    mut dfdy: *const gsl_matrix,
    mut sys: *const gsl_odeiv2_system,
) -> libc::c_int {
    let mut state: *mut modnewton1_state_t = vstate as *mut modnewton1_state_t;
    let IhAJ: *mut gsl_matrix = (*state).IhAJ;
    let p: *mut gsl_permutation = (*state).p;
    let dim: size_t = (*sys).dimension;
    let stage: size_t = (*A).size1;
    (*state).eeta_prev = 1.7976931348623157e+308f64;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut m: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < dim {
        j = 0 as libc::c_int as size_t;
        while j < dim {
            k = 0 as libc::c_int as size_t;
            while k < stage {
                m = 0 as libc::c_int as size_t;
                while m < stage {
                    let mut x: size_t = dim.wrapping_mul(k).wrapping_add(i);
                    let mut y: size_t = dim.wrapping_mul(m).wrapping_add(j);
                    if x != y {
                        gsl_matrix_set(
                            IhAJ,
                            x,
                            y,
                            -h * gsl_matrix_get(A, k, m) * gsl_matrix_get(dfdy, i, j),
                        );
                    } else {
                        gsl_matrix_set(
                            IhAJ,
                            x,
                            y,
                            1.0f64
                                - h * gsl_matrix_get(A, k, m) * gsl_matrix_get(dfdy, i, j),
                        );
                    }
                    m = m.wrapping_add(1);
                    m;
                }
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    let mut signum: libc::c_int = 0;
    let mut s: libc::c_int = gsl_linalg_LU_decomp(IhAJ, p, &mut signum);
    if s != GSL_SUCCESS as libc::c_int {
        return s;
    }
    return GSL_SUCCESS as libc::c_int;
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
unsafe extern "C" fn modnewton1_alloc(
    mut dim: size_t,
    mut stage: size_t,
) -> *mut libc::c_void {
    let mut state: *mut modnewton1_state_t = malloc(
        ::std::mem::size_of::<modnewton1_state_t>() as libc::c_ulong,
    ) as *mut modnewton1_state_t;
    if state.is_null() {
        gsl_error(
            b"failed to allocate space for modnewton1_state_t\0" as *const u8
                as *const libc::c_char,
            b"./modnewton1.c\0" as *const u8 as *const libc::c_char,
            83 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).IhAJ = gsl_matrix_alloc(dim.wrapping_mul(stage), dim.wrapping_mul(stage));
    if ((*state).IhAJ).is_null() {
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for IhAJ\0" as *const u8 as *const libc::c_char,
            b"./modnewton1.c\0" as *const u8 as *const libc::c_char,
            91 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).p = gsl_permutation_alloc(dim.wrapping_mul(stage));
    if ((*state).p).is_null() {
        gsl_matrix_free((*state).IhAJ);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for p\0" as *const u8 as *const libc::c_char,
            b"./modnewton1.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).dYk = gsl_vector_alloc(dim.wrapping_mul(stage));
    if ((*state).dYk).is_null() {
        gsl_permutation_free((*state).p);
        gsl_matrix_free((*state).IhAJ);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for dYk\0" as *const u8 as *const libc::c_char,
            b"./modnewton1.c\0" as *const u8 as *const libc::c_char,
            110 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).dScal = gsl_vector_alloc(dim.wrapping_mul(stage));
    if ((*state).dScal).is_null() {
        gsl_vector_free((*state).dYk);
        gsl_permutation_free((*state).p);
        gsl_matrix_free((*state).IhAJ);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for dScal\0" as *const u8 as *const libc::c_char,
            b"./modnewton1.c\0" as *const u8 as *const libc::c_char,
            121 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .Yk = malloc(
        dim
            .wrapping_mul(stage)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).Yk).is_null() {
        gsl_vector_free((*state).dScal);
        gsl_vector_free((*state).dYk);
        gsl_permutation_free((*state).p);
        gsl_matrix_free((*state).IhAJ);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for Yk\0" as *const u8 as *const libc::c_char,
            b"./modnewton1.c\0" as *const u8 as *const libc::c_char,
            133 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .fYk = malloc(
        dim
            .wrapping_mul(stage)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).fYk).is_null() {
        free((*state).Yk as *mut libc::c_void);
        gsl_vector_free((*state).dScal);
        gsl_vector_free((*state).dYk);
        gsl_permutation_free((*state).p);
        gsl_matrix_free((*state).IhAJ);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for Yk\0" as *const u8 as *const libc::c_char,
            b"./modnewton1.c\0" as *const u8 as *const libc::c_char,
            146 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).rhs = gsl_vector_alloc(dim.wrapping_mul(stage));
    if ((*state).rhs).is_null() {
        free((*state).fYk as *mut libc::c_void);
        free((*state).Yk as *mut libc::c_void);
        gsl_vector_free((*state).dScal);
        gsl_vector_free((*state).dYk);
        gsl_permutation_free((*state).p);
        gsl_matrix_free((*state).IhAJ);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for rhs\0" as *const u8 as *const libc::c_char,
            b"./modnewton1.c\0" as *const u8 as *const libc::c_char,
            160 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).eeta_prev = 1.7976931348623157e+308f64;
    return state as *mut libc::c_void;
}
unsafe extern "C" fn rk2imp_alloc(mut dim: size_t) -> *mut libc::c_void {
    let mut state: *mut rk2imp_state_t = malloc(
        ::std::mem::size_of::<rk2imp_state_t>() as libc::c_ulong,
    ) as *mut rk2imp_state_t;
    if state.is_null() {
        gsl_error(
            b"failed to allocate space for rk2imp_state\0" as *const u8
                as *const libc::c_char,
            b"rk2imp.c\0" as *const u8 as *const libc::c_char,
            71 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .A = gsl_matrix_alloc(1 as libc::c_int as size_t, 1 as libc::c_int as size_t);
    if ((*state).A).is_null() {
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for A\0" as *const u8 as *const libc::c_char,
            b"rk2imp.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .y_onestep = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).y_onestep).is_null() {
        gsl_matrix_free((*state).A);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for y_onestep\0" as *const u8
                as *const libc::c_char,
            b"rk2imp.c\0" as *const u8 as *const libc::c_char,
            88 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .y_twostep = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).y_twostep).is_null() {
        free((*state).y_onestep as *mut libc::c_void);
        gsl_matrix_free((*state).A);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for y_onestep\0" as *const u8
                as *const libc::c_char,
            b"rk2imp.c\0" as *const u8 as *const libc::c_char,
            98 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .ytmp = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).ytmp).is_null() {
        free((*state).y_twostep as *mut libc::c_void);
        free((*state).y_onestep as *mut libc::c_void);
        gsl_matrix_free((*state).A);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for ytmp\0" as *const u8 as *const libc::c_char,
            b"rk2imp.c\0" as *const u8 as *const libc::c_char,
            109 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .y_save = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).y_save).is_null() {
        free((*state).ytmp as *mut libc::c_void);
        free((*state).y_twostep as *mut libc::c_void);
        free((*state).y_onestep as *mut libc::c_void);
        gsl_matrix_free((*state).A);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for y_save\0" as *const u8 as *const libc::c_char,
            b"rk2imp.c\0" as *const u8 as *const libc::c_char,
            121 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .YZ = malloc(
        dim
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).YZ).is_null() {
        free((*state).y_save as *mut libc::c_void);
        free((*state).ytmp as *mut libc::c_void);
        free((*state).y_twostep as *mut libc::c_void);
        free((*state).y_onestep as *mut libc::c_void);
        gsl_matrix_free((*state).A);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for YZ\0" as *const u8 as *const libc::c_char,
            b"rk2imp.c\0" as *const u8 as *const libc::c_char,
            134 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .fYZ = malloc(
        dim
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).fYZ).is_null() {
        free((*state).YZ as *mut libc::c_void);
        free((*state).y_save as *mut libc::c_void);
        free((*state).ytmp as *mut libc::c_void);
        free((*state).y_twostep as *mut libc::c_void);
        free((*state).y_onestep as *mut libc::c_void);
        gsl_matrix_free((*state).A);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for fYZ\0" as *const u8 as *const libc::c_char,
            b"rk2imp.c\0" as *const u8 as *const libc::c_char,
            148 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .dfdt = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).dfdt).is_null() {
        free((*state).fYZ as *mut libc::c_void);
        free((*state).YZ as *mut libc::c_void);
        free((*state).y_save as *mut libc::c_void);
        free((*state).ytmp as *mut libc::c_void);
        free((*state).y_twostep as *mut libc::c_void);
        free((*state).y_onestep as *mut libc::c_void);
        gsl_matrix_free((*state).A);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for dfdt\0" as *const u8 as *const libc::c_char,
            b"rk2imp.c\0" as *const u8 as *const libc::c_char,
            163 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).dfdy = gsl_matrix_alloc(dim, dim);
    if ((*state).dfdy).is_null() {
        free((*state).dfdt as *mut libc::c_void);
        free((*state).fYZ as *mut libc::c_void);
        free((*state).YZ as *mut libc::c_void);
        free((*state).y_save as *mut libc::c_void);
        free((*state).ytmp as *mut libc::c_void);
        free((*state).y_twostep as *mut libc::c_void);
        free((*state).y_onestep as *mut libc::c_void);
        gsl_matrix_free((*state).A);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for dfdy\0" as *const u8 as *const libc::c_char,
            b"rk2imp.c\0" as *const u8 as *const libc::c_char,
            179 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .esol = modnewton1_alloc(dim, 1 as libc::c_int as size_t)
        as *mut modnewton1_state_t;
    if ((*state).esol).is_null() {
        gsl_matrix_free((*state).dfdy);
        free((*state).dfdt as *mut libc::c_void);
        free((*state).fYZ as *mut libc::c_void);
        free((*state).YZ as *mut libc::c_void);
        free((*state).y_save as *mut libc::c_void);
        free((*state).ytmp as *mut libc::c_void);
        free((*state).y_twostep as *mut libc::c_void);
        free((*state).y_onestep as *mut libc::c_void);
        gsl_matrix_free((*state).A);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for esol\0" as *const u8 as *const libc::c_char,
            b"rk2imp.c\0" as *const u8 as *const libc::c_char,
            196 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .errlev = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).errlev).is_null() {
        modnewton1_free((*state).esol as *mut libc::c_void);
        gsl_matrix_free((*state).dfdy);
        free((*state).dfdt as *mut libc::c_void);
        free((*state).fYZ as *mut libc::c_void);
        free((*state).YZ as *mut libc::c_void);
        free((*state).y_save as *mut libc::c_void);
        free((*state).ytmp as *mut libc::c_void);
        free((*state).y_twostep as *mut libc::c_void);
        free((*state).y_onestep as *mut libc::c_void);
        gsl_matrix_free((*state).A);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for errlev\0" as *const u8 as *const libc::c_char,
            b"rk2imp.c\0" as *const u8 as *const libc::c_char,
            214 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).driver = 0 as *const gsl_odeiv2_driver;
    return state as *mut libc::c_void;
}
unsafe extern "C" fn rk2imp_apply(
    mut vstate: *mut libc::c_void,
    mut dim: size_t,
    mut t: libc::c_double,
    mut h: libc::c_double,
    mut y: *mut libc::c_double,
    mut yerr: *mut libc::c_double,
    mut dydt_in: *const libc::c_double,
    mut dydt_out: *mut libc::c_double,
    mut sys: *const gsl_odeiv2_system,
) -> libc::c_int {
    let mut state: *mut rk2imp_state_t = vstate as *mut rk2imp_state_t;
    let y_onestep: *mut libc::c_double = (*state).y_onestep;
    let y_twostep: *mut libc::c_double = (*state).y_twostep;
    let ytmp: *mut libc::c_double = (*state).ytmp;
    let y_save: *mut libc::c_double = (*state).y_save;
    let YZ: *mut libc::c_double = (*state).YZ;
    let fYZ: *mut libc::c_double = (*state).fYZ;
    let dfdy: *mut gsl_matrix = (*state).dfdy;
    let dfdt: *mut libc::c_double = (*state).dfdt;
    let errlev: *mut libc::c_double = (*state).errlev;
    let mut esol: *const modnewton1_state_t = (*state).esol;
    let mut A: *mut gsl_matrix = (*state).A;
    let b: [libc::c_double; 1] = [1.0f64];
    let c: [libc::c_double; 1] = [0.5f64];
    gsl_matrix_set(A, 0 as libc::c_int as size_t, 0 as libc::c_int as size_t, 0.5f64);
    if esol.is_null() {
        gsl_error(
            b"no non-linear equation solver speficied\0" as *const u8
                as *const libc::c_char,
            b"rk2imp.c\0" as *const u8 as *const libc::c_char,
            268 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if ((*state).driver).is_null() {
        return GSL_EFAULT as libc::c_int
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < dim {
            if !dydt_in.is_null() {
                gsl_odeiv2_control_errlevel(
                    (*(*state).driver).c,
                    *y.offset(i as isize),
                    *dydt_in.offset(i as isize),
                    h,
                    i,
                    &mut *errlev.offset(i as isize),
                );
            } else {
                gsl_odeiv2_control_errlevel(
                    (*(*state).driver).c,
                    *y.offset(i as isize),
                    0.0f64,
                    h,
                    i,
                    &mut *errlev.offset(i as isize),
                );
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    let mut s: libc::c_int = (Some(((*sys).jacobian).unwrap()))
        .unwrap()(t, y as *const libc::c_double, (*dfdy).data, dfdt, (*sys).params);
    if s != GSL_SUCCESS as libc::c_int {
        return s;
    }
    let mut s_0: libc::c_int = modnewton1_init(
        esol as *mut libc::c_void,
        A,
        h,
        dfdy,
        sys,
    );
    if s_0 != GSL_SUCCESS as libc::c_int {
        return s_0;
    }
    let mut s_1: libc::c_int = modnewton1_solve(
        esol as *mut libc::c_void,
        A,
        c.as_ptr(),
        t,
        h,
        y as *const libc::c_double,
        sys,
        YZ,
        errlev as *const libc::c_double,
    );
    if s_1 != GSL_SUCCESS as libc::c_int {
        return s_1;
    }
    let mut s_2: libc::c_int = (Some(((*sys).function).unwrap()))
        .unwrap()(
        t + c[0 as libc::c_int as usize] * h,
        YZ as *const libc::c_double,
        fYZ,
        (*sys).params,
    );
    if s_2 != GSL_SUCCESS as libc::c_int {
        return s_2;
    }
    let mut s_3: libc::c_int = rksubs(
        y_onestep,
        h,
        y as *const libc::c_double,
        fYZ as *const libc::c_double,
        b.as_ptr(),
        1 as libc::c_int as size_t,
        dim,
    );
    if s_3 != GSL_SUCCESS as libc::c_int {
        return s_3;
    }
    let mut s_4: libc::c_int = modnewton1_init(
        esol as *mut libc::c_void,
        A,
        h / 2.0f64,
        dfdy,
        sys,
    );
    if s_4 != GSL_SUCCESS as libc::c_int {
        return s_4;
    }
    let mut s_5: libc::c_int = modnewton1_solve(
        esol as *mut libc::c_void,
        A,
        c.as_ptr(),
        t,
        h / 2.0f64,
        y as *const libc::c_double,
        sys,
        YZ,
        errlev as *const libc::c_double,
    );
    if s_5 != GSL_SUCCESS as libc::c_int {
        return s_5;
    }
    let mut s_6: libc::c_int = (Some(((*sys).function).unwrap()))
        .unwrap()(
        t + c[0 as libc::c_int as usize] * h / 2.0f64,
        YZ as *const libc::c_double,
        fYZ,
        (*sys).params,
    );
    if s_6 != GSL_SUCCESS as libc::c_int {
        return s_6;
    }
    let mut s_7: libc::c_int = rksubs(
        ytmp,
        h / 2.0f64,
        y as *const libc::c_double,
        fYZ as *const libc::c_double,
        b.as_ptr(),
        1 as libc::c_int as size_t,
        dim,
    );
    if s_7 != GSL_SUCCESS as libc::c_int {
        return s_7;
    }
    memcpy(
        y_save as *mut libc::c_void,
        y as *const libc::c_void,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    let mut s_8: libc::c_int = modnewton1_solve(
        esol as *mut libc::c_void,
        A,
        c.as_ptr(),
        t + h / 2.0f64,
        h / 2.0f64,
        ytmp as *const libc::c_double,
        sys,
        YZ,
        errlev as *const libc::c_double,
    );
    if s_8 != GSL_SUCCESS as libc::c_int {
        return s_8;
    }
    let mut s_9: libc::c_int = (Some(((*sys).function).unwrap()))
        .unwrap()(
        t + h / 2.0f64 + c[0 as libc::c_int as usize] * h / 2.0f64,
        YZ as *const libc::c_double,
        fYZ,
        (*sys).params,
    );
    if s_9 != GSL_SUCCESS as libc::c_int {
        return s_9;
    }
    let mut s_10: libc::c_int = rksubs(
        y_twostep,
        h / 2.0f64,
        ytmp as *const libc::c_double,
        fYZ as *const libc::c_double,
        b.as_ptr(),
        1 as libc::c_int as size_t,
        dim,
    );
    if s_10 != GSL_SUCCESS as libc::c_int {
        memcpy(
            y as *mut libc::c_void,
            y_save as *const libc::c_void,
            dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
        return s_10;
    }
    memcpy(
        y as *mut libc::c_void,
        y_twostep as *const libc::c_void,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    let mut i_0: size_t = 0;
    i_0 = 0 as libc::c_int as size_t;
    while i_0 < dim {
        *yerr
            .offset(
                i_0 as isize,
            ) = 8.0f64 * 0.5f64
            * fabs(*y_twostep.offset(i_0 as isize) - *y_onestep.offset(i_0 as isize))
            / 3.0f64;
        i_0 = i_0.wrapping_add(1);
        i_0;
    }
    if !dydt_out.is_null() {
        let mut s_11: libc::c_int = (Some(((*sys).function).unwrap()))
            .unwrap()(t + h, y as *const libc::c_double, dydt_out, (*sys).params);
        if s_11 != GSL_SUCCESS as libc::c_int {
            memcpy(
                y as *mut libc::c_void,
                y_save as *const libc::c_void,
                dim
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    ),
            );
            return s_11;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn rk2imp_set_driver(
    mut vstate: *mut libc::c_void,
    mut d: *const gsl_odeiv2_driver,
) -> libc::c_int {
    let mut state: *mut rk2imp_state_t = vstate as *mut rk2imp_state_t;
    (*state).driver = d;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn rk2imp_reset(
    mut vstate: *mut libc::c_void,
    mut dim: size_t,
) -> libc::c_int {
    let mut state: *mut rk2imp_state_t = vstate as *mut rk2imp_state_t;
    memset(
        (*state).y_onestep as *mut libc::c_void,
        0 as libc::c_int,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memset(
        (*state).y_twostep as *mut libc::c_void,
        0 as libc::c_int,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memset(
        (*state).ytmp as *mut libc::c_void,
        0 as libc::c_int,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memset(
        (*state).y_save as *mut libc::c_void,
        0 as libc::c_int,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memset(
        (*state).YZ as *mut libc::c_void,
        0 as libc::c_int,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memset(
        (*state).fYZ as *mut libc::c_void,
        0 as libc::c_int,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn rk2imp_order(mut vstate: *mut libc::c_void) -> libc::c_uint {
    let mut state: *mut rk2imp_state_t = vstate as *mut rk2imp_state_t;
    state = 0 as *mut rk2imp_state_t;
    return 2 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn rk2imp_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut rk2imp_state_t = vstate as *mut rk2imp_state_t;
    free((*state).errlev as *mut libc::c_void);
    modnewton1_free((*state).esol as *mut libc::c_void);
    gsl_matrix_free((*state).dfdy);
    free((*state).dfdt as *mut libc::c_void);
    free((*state).fYZ as *mut libc::c_void);
    free((*state).YZ as *mut libc::c_void);
    free((*state).y_save as *mut libc::c_void);
    free((*state).ytmp as *mut libc::c_void);
    free((*state).y_twostep as *mut libc::c_void);
    free((*state).y_onestep as *mut libc::c_void);
    gsl_matrix_free((*state).A);
    free(state as *mut libc::c_void);
}
static mut rk2imp_type: gsl_odeiv2_step_type = {
    let mut init = gsl_odeiv2_step_type {
        name: b"rk2imp\0" as *const u8 as *const libc::c_char,
        can_use_dydt_in: 1 as libc::c_int,
        gives_exact_dydt_out: 1 as libc::c_int,
        alloc: Some(rk2imp_alloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
        apply: Some(
            rk2imp_apply
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    size_t,
                    libc::c_double,
                    libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *const libc::c_double,
                    *mut libc::c_double,
                    *const gsl_odeiv2_system,
                ) -> libc::c_int,
        ),
        set_driver: Some(
            rk2imp_set_driver
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const gsl_odeiv2_driver,
                ) -> libc::c_int,
        ),
        reset: Some(
            rk2imp_reset
                as unsafe extern "C" fn(*mut libc::c_void, size_t) -> libc::c_int,
        ),
        order: Some(
            rk2imp_order as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint,
        ),
        free: Some(rk2imp_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    };
    init
};
pub static mut gsl_odeiv2_step_rk2imp: *const gsl_odeiv2_step_type = unsafe {
    &rk2imp_type as *const gsl_odeiv2_step_type
};
