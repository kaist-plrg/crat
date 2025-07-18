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
    fn gsl_vector_view_array(v: *mut libc::c_double, n: size_t) -> _gsl_vector_view;
    fn gsl_permutation_alloc(n: size_t) -> *mut gsl_permutation;
    fn gsl_permutation_free(p: *mut gsl_permutation);
    fn gsl_matrix_alloc(n1: size_t, n2: size_t) -> *mut gsl_matrix;
    fn gsl_matrix_free(m: *mut gsl_matrix);
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
pub struct gsl_permutation_struct {
    pub size: size_t,
    pub data: *mut size_t,
}
pub type gsl_permutation = gsl_permutation_struct;
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
pub struct bsimp_state_t {
    pub d: *mut gsl_matrix,
    pub a_mat: *mut gsl_matrix,
    pub p_vec: *mut gsl_permutation,
    pub x: [libc::c_double; 7],
    pub k_current: size_t,
    pub k_choice: size_t,
    pub h_next: libc::c_double,
    pub eps: libc::c_double,
    pub yp: *mut libc::c_double,
    pub y_save: *mut libc::c_double,
    pub yerr_save: *mut libc::c_double,
    pub y_extrap_save: *mut libc::c_double,
    pub y_extrap_sequence: *mut libc::c_double,
    pub extrap_work: *mut libc::c_double,
    pub dfdt: *mut libc::c_double,
    pub y_temp: *mut libc::c_double,
    pub delta_temp: *mut libc::c_double,
    pub weight: *mut libc::c_double,
    pub dfdy: *mut gsl_matrix,
    pub rhs_temp: *mut libc::c_double,
    pub delta: *mut libc::c_double,
    pub order: size_t,
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
unsafe extern "C" fn stepper_set_driver_null(
    mut vstate: *mut libc::c_void,
    mut d: *const gsl_odeiv2_driver,
) -> libc::c_int {
    return GSL_SUCCESS as libc::c_int;
}
static mut bd_sequence: [libc::c_int; 8] = [
    2 as libc::c_int,
    6 as libc::c_int,
    10 as libc::c_int,
    14 as libc::c_int,
    22 as libc::c_int,
    34 as libc::c_int,
    50 as libc::c_int,
    70 as libc::c_int,
];
unsafe extern "C" fn compute_weights(
    mut y: *const libc::c_double,
    mut w: *mut libc::c_double,
    mut dim: size_t,
) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < dim {
        let mut u: libc::c_double = fabs(*y.offset(i as isize));
        *w.offset(i as isize) = if u > 0.0f64 { u } else { 1.0f64 };
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn bsimp_deuf_kchoice(
    mut eps: libc::c_double,
    mut dimension: size_t,
) -> size_t {
    let safety_f: libc::c_double = 0.25f64;
    let small_eps: libc::c_double = safety_f * eps;
    let mut a_work: [libc::c_double; 8] = [0.; 8];
    let mut alpha: [[libc::c_double; 7]; 7] = [[0.; 7]; 7];
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    a_work[0 as libc::c_int
        as usize] = bd_sequence[0 as libc::c_int as usize] as libc::c_double + 1.0f64;
    k = 0 as libc::c_int;
    while k < 7 as libc::c_int {
        a_work[(k + 1 as libc::c_int)
            as usize] = a_work[k as usize]
            + bd_sequence[(k + 1 as libc::c_int) as usize] as libc::c_double;
        k += 1;
        k;
    }
    i = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        alpha[i as usize][i as usize] = 1.0f64;
        k = 0 as libc::c_int;
        while k < i {
            let tmp1: libc::c_double = a_work[(k + 1 as libc::c_int) as usize]
                - a_work[(i + 1 as libc::c_int) as usize];
            let tmp2: libc::c_double = (a_work[(i + 1 as libc::c_int) as usize]
                - a_work[0 as libc::c_int as usize] + 1.0f64)
                * (2 as libc::c_int * k + 1 as libc::c_int) as libc::c_double;
            alpha[k as usize][i as usize] = pow(small_eps, tmp1 / tmp2);
            k += 1;
            k;
        }
        i += 1;
        i;
    }
    a_work[0 as libc::c_int as usize] += dimension as libc::c_double;
    k = 0 as libc::c_int;
    while k < 7 as libc::c_int {
        a_work[(k + 1 as libc::c_int)
            as usize] = a_work[k as usize]
            + bd_sequence[(k + 1 as libc::c_int) as usize] as libc::c_double;
        k += 1;
        k;
    }
    k = 0 as libc::c_int;
    while k < 7 as libc::c_int - 1 as libc::c_int {
        if a_work[(k + 2 as libc::c_int) as usize]
            > a_work[(k + 1 as libc::c_int) as usize]
                * alpha[k as usize][(k + 1 as libc::c_int) as usize]
        {
            break;
        }
        k += 1;
        k;
    }
    return k as size_t;
}
unsafe extern "C" fn poly_extrap(
    mut d: *mut gsl_matrix,
    mut x: *const libc::c_double,
    i_step: libc::c_uint,
    x_i: libc::c_double,
    mut y_i: *const libc::c_double,
    mut y_0: *mut libc::c_double,
    mut y_0_err: *mut libc::c_double,
    mut work: *mut libc::c_double,
    dim: size_t,
) {
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    memcpy(
        y_0_err as *mut libc::c_void,
        y_i as *const libc::c_void,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memcpy(
        y_0 as *mut libc::c_void,
        y_i as *const libc::c_void,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    if i_step == 0 as libc::c_int as libc::c_uint {
        j = 0 as libc::c_int as size_t;
        while j < dim {
            gsl_matrix_set(d, 0 as libc::c_int as size_t, j, *y_i.offset(j as isize));
            j = j.wrapping_add(1);
            j;
        }
    } else {
        memcpy(
            work as *mut libc::c_void,
            y_i as *const libc::c_void,
            dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
        k = 0 as libc::c_int as size_t;
        while k < i_step as libc::c_ulong {
            let mut delta: libc::c_double = 1.0f64
                / (*x
                    .offset(
                        (i_step as libc::c_ulong)
                            .wrapping_sub(k)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ) - x_i);
            let f1: libc::c_double = delta * x_i;
            let f2: libc::c_double = delta
                * *x
                    .offset(
                        (i_step as libc::c_ulong)
                            .wrapping_sub(k)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    );
            j = 0 as libc::c_int as size_t;
            while j < dim {
                let q_kj: libc::c_double = gsl_matrix_get(d, k, j);
                gsl_matrix_set(d, k, j, *y_0_err.offset(j as isize));
                delta = *work.offset(j as isize) - q_kj;
                *y_0_err.offset(j as isize) = f1 * delta;
                *work.offset(j as isize) = f2 * delta;
                *y_0.offset(j as isize) += *y_0_err.offset(j as isize);
                j = j.wrapping_add(1);
                j;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = 0 as libc::c_int as size_t;
        while j < dim {
            gsl_matrix_set(d, i_step as size_t, j, *y_0_err.offset(j as isize));
            j = j.wrapping_add(1);
            j;
        }
    };
}
unsafe extern "C" fn bsimp_step_local(
    mut vstate: *mut libc::c_void,
    mut dim: size_t,
    t0: libc::c_double,
    h_total: libc::c_double,
    n_step: libc::c_uint,
    mut y: *const libc::c_double,
    mut yp: *const libc::c_double,
    mut dfdt: *const libc::c_double,
    mut dfdy: *const gsl_matrix,
    mut y_out: *mut libc::c_double,
    mut sys: *const gsl_odeiv2_system,
) -> libc::c_int {
    let mut state: *mut bsimp_state_t = vstate as *mut bsimp_state_t;
    let a_mat: *mut gsl_matrix = (*state).a_mat;
    let p_vec: *mut gsl_permutation = (*state).p_vec;
    let delta: *mut libc::c_double = (*state).delta;
    let y_temp: *mut libc::c_double = (*state).y_temp;
    let delta_temp: *mut libc::c_double = (*state).delta_temp;
    let rhs_temp: *mut libc::c_double = (*state).rhs_temp;
    let w: *mut libc::c_double = (*state).weight;
    let mut y_temp_vec: gsl_vector_view = gsl_vector_view_array(y_temp, dim);
    let mut delta_temp_vec: gsl_vector_view = gsl_vector_view_array(delta_temp, dim);
    let mut rhs_temp_vec: gsl_vector_view = gsl_vector_view_array(rhs_temp, dim);
    let h: libc::c_double = h_total / n_step as libc::c_double;
    let mut t: libc::c_double = t0 + h;
    let mut sum: libc::c_double = 0.;
    let max_sum: libc::c_double = 100.0f64 * dim as libc::c_double;
    let mut signum: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut n_inter: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < dim {
        j = 0 as libc::c_int as size_t;
        while j < dim {
            gsl_matrix_set(a_mat, i, j, -h * gsl_matrix_get(dfdy, i, j));
            j = j.wrapping_add(1);
            j;
        }
        gsl_matrix_set(a_mat, i, i, gsl_matrix_get(a_mat, i, i) + 1.0f64);
        i = i.wrapping_add(1);
        i;
    }
    gsl_linalg_LU_decomp(a_mat, p_vec, &mut signum);
    compute_weights(y, w, dim);
    i = 0 as libc::c_int as size_t;
    while i < dim {
        *y_temp
            .offset(
                i as isize,
            ) = h * (*yp.offset(i as isize) + h * *dfdt.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    gsl_linalg_LU_solve(
        a_mat,
        p_vec,
        &mut y_temp_vec.vector,
        &mut delta_temp_vec.vector,
    );
    sum = 0.0f64;
    i = 0 as libc::c_int as size_t;
    while i < dim {
        let di: libc::c_double = *delta_temp.offset(i as isize);
        *delta.offset(i as isize) = di;
        *y_temp.offset(i as isize) = *y.offset(i as isize) + di;
        sum += fabs(di) / *w.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    if sum > max_sum {
        return GSL_EFAILED as libc::c_int;
    }
    status = (Some(((*sys).function).unwrap()))
        .unwrap()(t, y_temp as *const libc::c_double, y_out, (*sys).params);
    if status != 0 {
        return status;
    }
    n_inter = 1 as libc::c_int as size_t;
    while n_inter < n_step as libc::c_ulong {
        i = 0 as libc::c_int as size_t;
        while i < dim {
            *rhs_temp
                .offset(
                    i as isize,
                ) = h * *y_out.offset(i as isize) - *delta.offset(i as isize);
            i = i.wrapping_add(1);
            i;
        }
        gsl_linalg_LU_solve(
            a_mat,
            p_vec,
            &mut rhs_temp_vec.vector,
            &mut delta_temp_vec.vector,
        );
        sum = 0.0f64;
        i = 0 as libc::c_int as size_t;
        while i < dim {
            *delta.offset(i as isize) += 2.0f64 * *delta_temp.offset(i as isize);
            *y_temp.offset(i as isize) += *delta.offset(i as isize);
            sum += fabs(*delta.offset(i as isize)) / *w.offset(i as isize);
            i = i.wrapping_add(1);
            i;
        }
        if sum > max_sum {
            return GSL_EFAILED as libc::c_int;
        }
        t += h;
        status = (Some(((*sys).function).unwrap()))
            .unwrap()(t, y_temp as *const libc::c_double, y_out, (*sys).params);
        if status != 0 {
            return status;
        }
        n_inter = n_inter.wrapping_add(1);
        n_inter;
    }
    i = 0 as libc::c_int as size_t;
    while i < dim {
        *rhs_temp
            .offset(
                i as isize,
            ) = h * *y_out.offset(i as isize) - *delta.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    gsl_linalg_LU_solve(
        a_mat,
        p_vec,
        &mut rhs_temp_vec.vector,
        &mut delta_temp_vec.vector,
    );
    sum = 0.0f64;
    i = 0 as libc::c_int as size_t;
    while i < dim {
        *y_out
            .offset(
                i as isize,
            ) = *y_temp.offset(i as isize) + *delta_temp.offset(i as isize);
        sum += fabs(*delta_temp.offset(i as isize)) / *w.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    if sum > max_sum {
        return GSL_EFAILED as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn bsimp_alloc(mut dim: size_t) -> *mut libc::c_void {
    let mut state: *mut bsimp_state_t = malloc(
        ::std::mem::size_of::<bsimp_state_t>() as libc::c_ulong,
    ) as *mut bsimp_state_t;
    (*state).d = gsl_matrix_alloc(7 as libc::c_int as size_t, dim);
    (*state).a_mat = gsl_matrix_alloc(dim, dim);
    (*state).p_vec = gsl_permutation_alloc(dim);
    (*state)
        .yp = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    (*state)
        .y_save = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    (*state)
        .yerr_save = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    (*state)
        .y_extrap_save = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    (*state)
        .y_extrap_sequence = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    (*state)
        .extrap_work = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    (*state)
        .dfdt = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    (*state)
        .y_temp = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    (*state)
        .delta_temp = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    (*state)
        .weight = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    (*state).dfdy = gsl_matrix_alloc(dim, dim);
    (*state)
        .rhs_temp = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    (*state)
        .delta = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    let mut k_choice: size_t = bsimp_deuf_kchoice(1.4901161193847656e-08f64, dim);
    (*state).k_choice = k_choice;
    (*state).k_current = k_choice;
    (*state).order = (2 as libc::c_int as libc::c_ulong).wrapping_mul(k_choice);
    (*state).h_next = -1.3407807929942596e+154f64;
    return state as *mut libc::c_void;
}
unsafe extern "C" fn bsimp_apply(
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
    let mut state: *mut bsimp_state_t = vstate as *mut bsimp_state_t;
    let x: *mut libc::c_double = ((*state).x).as_mut_ptr();
    let yp: *mut libc::c_double = (*state).yp;
    let y_save: *mut libc::c_double = (*state).y_save;
    let yerr_save: *mut libc::c_double = (*state).yerr_save;
    let y_extrap_sequence: *mut libc::c_double = (*state).y_extrap_sequence;
    let y_extrap_save: *mut libc::c_double = (*state).y_extrap_save;
    let extrap_work: *mut libc::c_double = (*state).extrap_work;
    let dfdt: *mut libc::c_double = (*state).dfdt;
    let mut d: *mut gsl_matrix = (*state).d;
    let mut dfdy: *mut gsl_matrix = (*state).dfdy;
    let t_local: libc::c_double = t;
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    if h + t_local == t_local {
        return GSL_EUNDRFLW as libc::c_int;
    }
    memcpy(
        y_extrap_save as *mut libc::c_void,
        y as *const libc::c_void,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memcpy(
        y_save as *mut libc::c_void,
        y as *const libc::c_void,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memcpy(
        yerr_save as *mut libc::c_void,
        yerr as *const libc::c_void,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    if !dydt_in.is_null() {
        memcpy(
            yp as *mut libc::c_void,
            dydt_in as *const libc::c_void,
            dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
    } else {
        let mut s: libc::c_int = (Some(((*sys).function).unwrap()))
            .unwrap()(t_local, y as *const libc::c_double, yp, (*sys).params);
        if s != GSL_SUCCESS as libc::c_int {
            return s;
        }
    }
    let mut s_0: libc::c_int = (Some(((*sys).jacobian).unwrap()))
        .unwrap()(
        t_local,
        y as *const libc::c_double,
        (*dfdy).data,
        dfdt,
        (*sys).params,
    );
    if s_0 != GSL_SUCCESS as libc::c_int {
        return s_0;
    }
    k = 0 as libc::c_int as size_t;
    while k <= (*state).k_current {
        let N: libc::c_uint = bd_sequence[k as usize] as libc::c_uint;
        let r: libc::c_double = h / N as libc::c_double;
        let x_k: libc::c_double = r * r;
        let mut status: libc::c_int = bsimp_step_local(
            state as *mut libc::c_void,
            dim,
            t_local,
            h,
            N,
            y_extrap_save as *const libc::c_double,
            yp as *const libc::c_double,
            dfdt as *const libc::c_double,
            dfdy,
            y_extrap_sequence,
            sys,
        );
        if status == GSL_EFAILED as libc::c_int {
            i = 0 as libc::c_int as size_t;
            while i < dim {
                *yerr.offset(i as isize) = ::std::f32::INFINITY as libc::c_double;
                i = i.wrapping_add(1);
                i;
            }
            break;
        } else {
            if status != GSL_SUCCESS as libc::c_int {
                return status;
            }
            *x.offset(k as isize) = x_k;
            poly_extrap(
                d,
                x as *const libc::c_double,
                k as libc::c_uint,
                x_k,
                y_extrap_sequence as *const libc::c_double,
                y,
                yerr,
                extrap_work,
                dim,
            );
            k = k.wrapping_add(1);
            k;
        }
    }
    if !dydt_out.is_null() {
        let mut s_1: libc::c_int = (Some(((*sys).function).unwrap()))
            .unwrap()(t + h, y as *const libc::c_double, dydt_out, (*sys).params);
        if s_1 != GSL_SUCCESS as libc::c_int {
            memcpy(
                y as *mut libc::c_void,
                y_save as *const libc::c_void,
                dim
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    ),
            );
            memcpy(
                yerr as *mut libc::c_void,
                yerr_save as *const libc::c_void,
                dim
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    ),
            );
            return s_1;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn bsimp_order(mut vstate: *mut libc::c_void) -> libc::c_uint {
    let mut state: *mut bsimp_state_t = vstate as *mut bsimp_state_t;
    return (*state).order as libc::c_uint;
}
unsafe extern "C" fn bsimp_reset(
    mut vstate: *mut libc::c_void,
    mut dim: size_t,
) -> libc::c_int {
    let mut state: *mut bsimp_state_t = vstate as *mut bsimp_state_t;
    (*state).h_next = 0 as libc::c_int as libc::c_double;
    memset(
        (*state).yp as *mut libc::c_void,
        0 as libc::c_int,
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn bsimp_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut bsimp_state_t = vstate as *mut bsimp_state_t;
    free((*state).delta as *mut libc::c_void);
    free((*state).rhs_temp as *mut libc::c_void);
    gsl_matrix_free((*state).dfdy);
    free((*state).weight as *mut libc::c_void);
    free((*state).delta_temp as *mut libc::c_void);
    free((*state).y_temp as *mut libc::c_void);
    free((*state).dfdt as *mut libc::c_void);
    free((*state).extrap_work as *mut libc::c_void);
    free((*state).y_extrap_sequence as *mut libc::c_void);
    free((*state).y_extrap_save as *mut libc::c_void);
    free((*state).y_save as *mut libc::c_void);
    free((*state).yerr_save as *mut libc::c_void);
    free((*state).yp as *mut libc::c_void);
    gsl_permutation_free((*state).p_vec);
    gsl_matrix_free((*state).a_mat);
    gsl_matrix_free((*state).d);
    free(state as *mut libc::c_void);
}
static mut bsimp_type: gsl_odeiv2_step_type = {
    let mut init = gsl_odeiv2_step_type {
        name: b"bsimp\0" as *const u8 as *const libc::c_char,
        can_use_dydt_in: 1 as libc::c_int,
        gives_exact_dydt_out: 1 as libc::c_int,
        alloc: Some(bsimp_alloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
        apply: Some(
            bsimp_apply
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
            stepper_set_driver_null
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const gsl_odeiv2_driver,
                ) -> libc::c_int,
        ),
        reset: Some(
            bsimp_reset as unsafe extern "C" fn(*mut libc::c_void, size_t) -> libc::c_int,
        ),
        order: Some(
            bsimp_order as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint,
        ),
        free: Some(bsimp_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    };
    init
};
pub static mut gsl_odeiv2_step_bsimp: *const gsl_odeiv2_step_type = unsafe {
    &bsimp_type as *const gsl_odeiv2_step_type
};
