use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_calloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_vector_set_all(v: *mut gsl_vector, x: libc::c_double);
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
    fn gsl_matrix_calloc(n1: size_t, n2: size_t) -> *mut gsl_matrix;
    fn gsl_matrix_free(m: *mut gsl_matrix);
    fn gsl_linalg_QR_decomp(A: *mut gsl_matrix, tau: *mut gsl_vector) -> libc::c_int;
    fn gsl_linalg_QR_update(
        Q: *mut gsl_matrix,
        R: *mut gsl_matrix,
        w: *mut gsl_vector,
        v: *const gsl_vector,
    ) -> libc::c_int;
    fn gsl_linalg_QR_unpack(
        QR: *const gsl_matrix,
        tau: *const gsl_vector,
        Q: *mut gsl_matrix,
        R: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_linalg_R_solve(
        R: *const gsl_matrix,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hybridj_state_t {
    pub iter: size_t,
    pub ncfail: size_t,
    pub ncsuc: size_t,
    pub nslow1: size_t,
    pub nslow2: size_t,
    pub fnorm: libc::c_double,
    pub delta: libc::c_double,
    pub q: *mut gsl_matrix,
    pub r: *mut gsl_matrix,
    pub tau: *mut gsl_vector,
    pub diag: *mut gsl_vector,
    pub qtf: *mut gsl_vector,
    pub newton: *mut gsl_vector,
    pub gradient: *mut gsl_vector,
    pub x_trial: *mut gsl_vector,
    pub f_trial: *mut gsl_vector,
    pub df: *mut gsl_vector,
    pub qtdf: *mut gsl_vector,
    pub rdx: *mut gsl_vector,
    pub w: *mut gsl_vector,
    pub v: *mut gsl_vector,
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
unsafe extern "C" fn scaled_enorm(
    mut d: *const gsl_vector,
    mut f: *const gsl_vector,
) -> libc::c_double {
    let mut e2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: size_t = 0;
    let mut n: size_t = (*f).size;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut fi: libc::c_double = gsl_vector_get(f, i);
        let mut di: libc::c_double = gsl_vector_get(d, i);
        let mut u: libc::c_double = di * fi;
        e2 += u * u;
        i = i.wrapping_add(1);
        i;
    }
    return sqrt(e2);
}
unsafe extern "C" fn enorm_sum(
    mut a: *const gsl_vector,
    mut b: *const gsl_vector,
) -> libc::c_double {
    let mut e2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: size_t = 0;
    let mut n: size_t = (*a).size;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut ai: libc::c_double = gsl_vector_get(a, i);
        let mut bi: libc::c_double = gsl_vector_get(b, i);
        let mut u: libc::c_double = ai + bi;
        e2 += u * u;
        i = i.wrapping_add(1);
        i;
    }
    return sqrt(e2);
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
unsafe extern "C" fn gradient_direction(
    mut r: *const gsl_matrix,
    mut qtf: *const gsl_vector,
    mut diag: *const gsl_vector,
    mut g: *mut gsl_vector,
) {
    let M: size_t = (*r).size1;
    let N: size_t = (*r).size2;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < M {
        let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut dj: libc::c_double = 0.;
        i = 0 as libc::c_int as size_t;
        while i < N {
            sum += gsl_matrix_get(r, i, j) * gsl_vector_get(qtf, i);
            i = i.wrapping_add(1);
            i;
        }
        dj = gsl_vector_get(diag, j);
        gsl_vector_set(g, j, -sum / dj);
        j = j.wrapping_add(1);
        j;
    }
}
unsafe extern "C" fn scaled_addition(
    mut alpha: libc::c_double,
    mut newton: *mut gsl_vector,
    mut beta: libc::c_double,
    mut gradient: *mut gsl_vector,
    mut p: *mut gsl_vector,
) {
    let N: size_t = (*p).size;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut ni: libc::c_double = gsl_vector_get(newton, i);
        let mut gi: libc::c_double = gsl_vector_get(gradient, i);
        gsl_vector_set(p, i, alpha * ni + beta * gi);
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn dogleg(
    mut r: *const gsl_matrix,
    mut qtf: *const gsl_vector,
    mut diag: *const gsl_vector,
    mut delta: libc::c_double,
    mut newton: *mut gsl_vector,
    mut gradient: *mut gsl_vector,
    mut p: *mut gsl_vector,
) -> libc::c_int {
    let mut qnorm: libc::c_double = 0.;
    let mut gnorm: libc::c_double = 0.;
    let mut sgnorm: libc::c_double = 0.;
    let mut bnorm: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    newton_direction(r, qtf, newton);
    qnorm = scaled_enorm(diag, newton);
    if qnorm <= delta {
        gsl_vector_memcpy(p, newton);
        return GSL_SUCCESS as libc::c_int;
    }
    gradient_direction(r, qtf, diag, gradient);
    gnorm = enorm(gradient);
    if gnorm == 0 as libc::c_int as libc::c_double {
        let mut alpha: libc::c_double = delta / qnorm;
        let mut beta: libc::c_double = 0 as libc::c_int as libc::c_double;
        scaled_addition(alpha, newton, beta, gradient, p);
        return GSL_SUCCESS as libc::c_int;
    }
    minimum_step(gnorm, diag, gradient);
    compute_Rg(r, gradient, p);
    temp = enorm(p);
    sgnorm = gnorm / temp / temp;
    if sgnorm > delta {
        let mut alpha_0: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut beta_0: libc::c_double = delta;
        scaled_addition(alpha_0, newton, beta_0, gradient, p);
        return GSL_SUCCESS as libc::c_int;
    }
    bnorm = enorm(qtf);
    let mut bg: libc::c_double = bnorm / gnorm;
    let mut bq: libc::c_double = bnorm / qnorm;
    let mut dq: libc::c_double = delta / qnorm;
    let mut dq2: libc::c_double = dq * dq;
    let mut sd: libc::c_double = sgnorm / delta;
    let mut sd2: libc::c_double = sd * sd;
    let mut t1: libc::c_double = bg * bq * sd;
    let mut u: libc::c_double = t1 - dq;
    let mut t2: libc::c_double = t1 - dq * sd2
        + sqrt(
            u * u
                + (1 as libc::c_int as libc::c_double - dq2)
                    * (1 as libc::c_int as libc::c_double - sd2),
        );
    let mut alpha_1: libc::c_double = dq * (1 as libc::c_int as libc::c_double - sd2)
        / t2;
    let mut beta_1: libc::c_double = (1 as libc::c_int as libc::c_double - alpha_1)
        * sgnorm;
    scaled_addition(alpha_1, newton, beta_1, gradient, p);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn compute_trial_step(
    mut x: *mut gsl_vector,
    mut dx: *mut gsl_vector,
    mut x_trial: *mut gsl_vector,
) {
    let mut i: size_t = 0;
    let mut N: size_t = (*x).size;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut pi: libc::c_double = gsl_vector_get(dx, i);
        let mut xi: libc::c_double = gsl_vector_get(x, i);
        gsl_vector_set(x_trial, i, xi + pi);
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn compute_df(
    mut f_trial: *const gsl_vector,
    mut f: *const gsl_vector,
    mut df: *mut gsl_vector,
) {
    let mut i: size_t = 0;
    let mut n: size_t = (*f).size;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut dfi: libc::c_double = gsl_vector_get(f_trial, i) - gsl_vector_get(f, i);
        gsl_vector_set(df, i, dfi);
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn minimum_step(
    mut gnorm: libc::c_double,
    mut diag: *const gsl_vector,
    mut g: *mut gsl_vector,
) {
    let N: size_t = (*g).size;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut gi: libc::c_double = gsl_vector_get(g, i);
        let mut di: libc::c_double = gsl_vector_get(diag, i);
        gsl_vector_set(g, i, gi / gnorm / di);
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn compute_wv(
    mut qtdf: *const gsl_vector,
    mut rdx: *const gsl_vector,
    mut dx: *const gsl_vector,
    mut diag: *const gsl_vector,
    mut pnorm: libc::c_double,
    mut w: *mut gsl_vector,
    mut v: *mut gsl_vector,
) {
    let mut i: size_t = 0;
    let mut n: size_t = (*qtdf).size;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut qtdfi: libc::c_double = gsl_vector_get(qtdf, i);
        let mut rdxi: libc::c_double = gsl_vector_get(rdx, i);
        let mut dxi: libc::c_double = gsl_vector_get(dx, i);
        let mut diagi: libc::c_double = gsl_vector_get(diag, i);
        gsl_vector_set(w, i, (qtdfi - rdxi) / pnorm);
        gsl_vector_set(v, i, diagi * diagi * dxi / pnorm);
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn compute_qtf(
    mut q: *const gsl_matrix,
    mut f: *const gsl_vector,
    mut qtf: *mut gsl_vector,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut N: size_t = (*f).size;
    j = 0 as libc::c_int as size_t;
    while j < N {
        let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
        i = 0 as libc::c_int as size_t;
        while i < N {
            sum += gsl_matrix_get(q, i, j) * gsl_vector_get(f, i);
            i = i.wrapping_add(1);
            i;
        }
        gsl_vector_set(qtf, j, sum);
        j = j.wrapping_add(1);
        j;
    }
}
unsafe extern "C" fn update_diag(mut J: *const gsl_matrix, mut diag: *mut gsl_vector) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut n: size_t = (*diag).size;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut cnorm: libc::c_double = 0.;
        let mut diagj: libc::c_double = 0.;
        let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
        i = 0 as libc::c_int as size_t;
        while i < n {
            let mut Jij: libc::c_double = gsl_matrix_get(J, i, j);
            sum += Jij * Jij;
            i = i.wrapping_add(1);
            i;
        }
        if sum == 0 as libc::c_int as libc::c_double {
            sum = 1.0f64;
        }
        cnorm = sqrt(sum);
        diagj = gsl_vector_get(diag, j);
        if cnorm > diagj {
            gsl_vector_set(diag, j, cnorm);
        }
        j = j.wrapping_add(1);
        j;
    }
}
unsafe extern "C" fn compute_delta(
    mut diag: *mut gsl_vector,
    mut x: *mut gsl_vector,
) -> libc::c_double {
    let mut Dx: libc::c_double = scaled_enorm(diag, x);
    let mut factor: libc::c_double = 100 as libc::c_int as libc::c_double;
    return if Dx > 0 as libc::c_int as libc::c_double { factor * Dx } else { factor };
}
unsafe extern "C" fn newton_direction(
    mut r: *const gsl_matrix,
    mut qtf: *const gsl_vector,
    mut p: *mut gsl_vector,
) -> libc::c_int {
    let N: size_t = (*r).size2;
    let mut i: size_t = 0;
    let mut status: libc::c_int = 0;
    status = gsl_linalg_R_solve(r, qtf, p);
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut pi: libc::c_double = gsl_vector_get(p, i);
        gsl_vector_set(p, i, -pi);
        i = i.wrapping_add(1);
        i;
    }
    return status;
}
unsafe extern "C" fn compute_diag(mut J: *const gsl_matrix, mut diag: *mut gsl_vector) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut n: size_t = (*diag).size;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
        i = 0 as libc::c_int as size_t;
        while i < n {
            let mut Jij: libc::c_double = gsl_matrix_get(J, i, j);
            sum += Jij * Jij;
            i = i.wrapping_add(1);
            i;
        }
        if sum == 0 as libc::c_int as libc::c_double {
            sum = 1.0f64;
        }
        gsl_vector_set(diag, j, sqrt(sum));
        j = j.wrapping_add(1);
        j;
    }
}
unsafe extern "C" fn compute_predicted_reduction(
    mut fnorm: libc::c_double,
    mut fnorm1: libc::c_double,
) -> libc::c_double {
    let mut prered: libc::c_double = 0.;
    if fnorm1 < fnorm {
        let mut u: libc::c_double = fnorm1 / fnorm;
        prered = 1 as libc::c_int as libc::c_double - u * u;
    } else {
        prered = 0 as libc::c_int as libc::c_double;
    }
    return prered;
}
unsafe extern "C" fn compute_Rg(
    mut r: *const gsl_matrix,
    mut gradient: *const gsl_vector,
    mut Rg: *mut gsl_vector,
) {
    let N: size_t = (*r).size2;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
        j = i;
        while j < N {
            let mut gj: libc::c_double = gsl_vector_get(gradient, j);
            let mut rij: libc::c_double = gsl_matrix_get(r, i, j);
            sum += rij * gj;
            j = j.wrapping_add(1);
            j;
        }
        gsl_vector_set(Rg, i, sum);
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn compute_rdx(
    mut r: *const gsl_matrix,
    mut dx: *const gsl_vector,
    mut rdx: *mut gsl_vector,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut N: size_t = (*dx).size;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
        j = i;
        while j < N {
            sum += gsl_matrix_get(r, i, j) * gsl_vector_get(dx, j);
            j = j.wrapping_add(1);
            j;
        }
        gsl_vector_set(rdx, i, sum);
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn compute_actual_reduction(
    mut fnorm: libc::c_double,
    mut fnorm1: libc::c_double,
) -> libc::c_double {
    let mut actred: libc::c_double = 0.;
    if fnorm1 < fnorm {
        let mut u: libc::c_double = fnorm1 / fnorm;
        actred = 1 as libc::c_int as libc::c_double - u * u;
    } else {
        actred = -(1 as libc::c_int) as libc::c_double;
    }
    return actred;
}
unsafe extern "C" fn hybridj_alloc(
    mut vstate: *mut libc::c_void,
    mut n: size_t,
) -> libc::c_int {
    let mut state: *mut hybridj_state_t = vstate as *mut hybridj_state_t;
    let mut q: *mut gsl_matrix = 0 as *mut gsl_matrix;
    let mut r: *mut gsl_matrix = 0 as *mut gsl_matrix;
    let mut tau: *mut gsl_vector = 0 as *mut gsl_vector;
    let mut diag: *mut gsl_vector = 0 as *mut gsl_vector;
    let mut qtf: *mut gsl_vector = 0 as *mut gsl_vector;
    let mut newton: *mut gsl_vector = 0 as *mut gsl_vector;
    let mut gradient: *mut gsl_vector = 0 as *mut gsl_vector;
    let mut x_trial: *mut gsl_vector = 0 as *mut gsl_vector;
    let mut f_trial: *mut gsl_vector = 0 as *mut gsl_vector;
    let mut df: *mut gsl_vector = 0 as *mut gsl_vector;
    let mut qtdf: *mut gsl_vector = 0 as *mut gsl_vector;
    let mut rdx: *mut gsl_vector = 0 as *mut gsl_vector;
    let mut w: *mut gsl_vector = 0 as *mut gsl_vector;
    let mut v: *mut gsl_vector = 0 as *mut gsl_vector;
    q = gsl_matrix_calloc(n, n);
    if q.is_null() {
        gsl_error(
            b"failed to allocate space for q\0" as *const u8 as *const libc::c_char,
            b"hybridj.c\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).q = q;
    r = gsl_matrix_calloc(n, n);
    if r.is_null() {
        gsl_matrix_free(q);
        gsl_error(
            b"failed to allocate space for r\0" as *const u8 as *const libc::c_char,
            b"hybridj.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).r = r;
    tau = gsl_vector_calloc(n);
    if tau.is_null() {
        gsl_matrix_free(q);
        gsl_matrix_free(r);
        gsl_error(
            b"failed to allocate space for tau\0" as *const u8 as *const libc::c_char,
            b"hybridj.c\0" as *const u8 as *const libc::c_char,
            104 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).tau = tau;
    diag = gsl_vector_calloc(n);
    if diag.is_null() {
        gsl_matrix_free(q);
        gsl_matrix_free(r);
        gsl_vector_free(tau);
        gsl_error(
            b"failed to allocate space for diag\0" as *const u8 as *const libc::c_char,
            b"hybridj.c\0" as *const u8 as *const libc::c_char,
            117 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).diag = diag;
    qtf = gsl_vector_calloc(n);
    if qtf.is_null() {
        gsl_matrix_free(q);
        gsl_matrix_free(r);
        gsl_vector_free(tau);
        gsl_vector_free(diag);
        gsl_error(
            b"failed to allocate space for qtf\0" as *const u8 as *const libc::c_char,
            b"hybridj.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).qtf = qtf;
    newton = gsl_vector_calloc(n);
    if newton.is_null() {
        gsl_matrix_free(q);
        gsl_matrix_free(r);
        gsl_vector_free(tau);
        gsl_vector_free(diag);
        gsl_vector_free(qtf);
        gsl_error(
            b"failed to allocate space for newton\0" as *const u8 as *const libc::c_char,
            b"hybridj.c\0" as *const u8 as *const libc::c_char,
            146 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).newton = newton;
    gradient = gsl_vector_calloc(n);
    if gradient.is_null() {
        gsl_matrix_free(q);
        gsl_matrix_free(r);
        gsl_vector_free(tau);
        gsl_vector_free(diag);
        gsl_vector_free(qtf);
        gsl_vector_free(newton);
        gsl_error(
            b"failed to allocate space for gradient\0" as *const u8
                as *const libc::c_char,
            b"hybridj.c\0" as *const u8 as *const libc::c_char,
            162 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).gradient = gradient;
    x_trial = gsl_vector_calloc(n);
    if x_trial.is_null() {
        gsl_matrix_free(q);
        gsl_matrix_free(r);
        gsl_vector_free(tau);
        gsl_vector_free(diag);
        gsl_vector_free(qtf);
        gsl_vector_free(newton);
        gsl_vector_free(gradient);
        gsl_error(
            b"failed to allocate space for x_trial\0" as *const u8
                as *const libc::c_char,
            b"hybridj.c\0" as *const u8 as *const libc::c_char,
            179 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).x_trial = x_trial;
    f_trial = gsl_vector_calloc(n);
    if f_trial.is_null() {
        gsl_matrix_free(q);
        gsl_matrix_free(r);
        gsl_vector_free(tau);
        gsl_vector_free(diag);
        gsl_vector_free(qtf);
        gsl_vector_free(newton);
        gsl_vector_free(gradient);
        gsl_vector_free(x_trial);
        gsl_error(
            b"failed to allocate space for f_trial\0" as *const u8
                as *const libc::c_char,
            b"hybridj.c\0" as *const u8 as *const libc::c_char,
            197 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).f_trial = f_trial;
    df = gsl_vector_calloc(n);
    if df.is_null() {
        gsl_matrix_free(q);
        gsl_matrix_free(r);
        gsl_vector_free(tau);
        gsl_vector_free(diag);
        gsl_vector_free(qtf);
        gsl_vector_free(newton);
        gsl_vector_free(gradient);
        gsl_vector_free(x_trial);
        gsl_vector_free(f_trial);
        gsl_error(
            b"failed to allocate space for df\0" as *const u8 as *const libc::c_char,
            b"hybridj.c\0" as *const u8 as *const libc::c_char,
            216 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).df = df;
    qtdf = gsl_vector_calloc(n);
    if qtdf.is_null() {
        gsl_matrix_free(q);
        gsl_matrix_free(r);
        gsl_vector_free(tau);
        gsl_vector_free(diag);
        gsl_vector_free(qtf);
        gsl_vector_free(newton);
        gsl_vector_free(gradient);
        gsl_vector_free(x_trial);
        gsl_vector_free(f_trial);
        gsl_vector_free(df);
        gsl_error(
            b"failed to allocate space for qtdf\0" as *const u8 as *const libc::c_char,
            b"hybridj.c\0" as *const u8 as *const libc::c_char,
            236 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).qtdf = qtdf;
    rdx = gsl_vector_calloc(n);
    if rdx.is_null() {
        gsl_matrix_free(q);
        gsl_matrix_free(r);
        gsl_vector_free(tau);
        gsl_vector_free(diag);
        gsl_vector_free(qtf);
        gsl_vector_free(newton);
        gsl_vector_free(gradient);
        gsl_vector_free(x_trial);
        gsl_vector_free(f_trial);
        gsl_vector_free(df);
        gsl_vector_free(qtdf);
        gsl_error(
            b"failed to allocate space for rdx\0" as *const u8 as *const libc::c_char,
            b"hybridj.c\0" as *const u8 as *const libc::c_char,
            258 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).rdx = rdx;
    w = gsl_vector_calloc(n);
    if w.is_null() {
        gsl_matrix_free(q);
        gsl_matrix_free(r);
        gsl_vector_free(tau);
        gsl_vector_free(diag);
        gsl_vector_free(qtf);
        gsl_vector_free(newton);
        gsl_vector_free(gradient);
        gsl_vector_free(x_trial);
        gsl_vector_free(f_trial);
        gsl_vector_free(df);
        gsl_vector_free(qtdf);
        gsl_vector_free(rdx);
        gsl_error(
            b"failed to allocate space for w\0" as *const u8 as *const libc::c_char,
            b"hybridj.c\0" as *const u8 as *const libc::c_char,
            280 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).w = w;
    v = gsl_vector_calloc(n);
    if v.is_null() {
        gsl_matrix_free(q);
        gsl_matrix_free(r);
        gsl_vector_free(tau);
        gsl_vector_free(diag);
        gsl_vector_free(qtf);
        gsl_vector_free(newton);
        gsl_vector_free(gradient);
        gsl_vector_free(x_trial);
        gsl_vector_free(f_trial);
        gsl_vector_free(df);
        gsl_vector_free(qtdf);
        gsl_vector_free(rdx);
        gsl_vector_free(w);
        gsl_error(
            b"failed to allocate space for v\0" as *const u8 as *const libc::c_char,
            b"hybridj.c\0" as *const u8 as *const libc::c_char,
            303 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).v = v;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn hybridj_set(
    mut vstate: *mut libc::c_void,
    mut fdf: *mut gsl_multiroot_function_fdf,
    mut x: *mut gsl_vector,
    mut f: *mut gsl_vector,
    mut J: *mut gsl_matrix,
    mut dx: *mut gsl_vector,
) -> libc::c_int {
    let mut status: libc::c_int = hybridj_set_impl(
        vstate,
        fdf,
        x,
        f,
        J,
        dx,
        0 as libc::c_int,
    );
    return status;
}
unsafe extern "C" fn hybridsj_set(
    mut vstate: *mut libc::c_void,
    mut fdf: *mut gsl_multiroot_function_fdf,
    mut x: *mut gsl_vector,
    mut f: *mut gsl_vector,
    mut J: *mut gsl_matrix,
    mut dx: *mut gsl_vector,
) -> libc::c_int {
    let mut status: libc::c_int = hybridj_set_impl(
        vstate,
        fdf,
        x,
        f,
        J,
        dx,
        1 as libc::c_int,
    );
    return status;
}
unsafe extern "C" fn hybridj_set_impl(
    mut vstate: *mut libc::c_void,
    mut fdf: *mut gsl_multiroot_function_fdf,
    mut x: *mut gsl_vector,
    mut f: *mut gsl_vector,
    mut J: *mut gsl_matrix,
    mut dx: *mut gsl_vector,
    mut scale: libc::c_int,
) -> libc::c_int {
    let mut state: *mut hybridj_state_t = vstate as *mut hybridj_state_t;
    let mut q: *mut gsl_matrix = (*state).q;
    let mut r: *mut gsl_matrix = (*state).r;
    let mut tau: *mut gsl_vector = (*state).tau;
    let mut diag: *mut gsl_vector = (*state).diag;
    (Some(((*fdf).fdf).unwrap())).unwrap()(x, (*fdf).params, f, J);
    (*state).iter = 1 as libc::c_int as size_t;
    (*state).fnorm = enorm(f);
    (*state).ncfail = 0 as libc::c_int as size_t;
    (*state).ncsuc = 0 as libc::c_int as size_t;
    (*state).nslow1 = 0 as libc::c_int as size_t;
    (*state).nslow2 = 0 as libc::c_int as size_t;
    gsl_vector_set_all(dx, 0.0f64);
    if scale != 0 {
        compute_diag(J, diag);
    } else {
        gsl_vector_set_all(diag, 1.0f64);
    }
    (*state).delta = compute_delta(diag, x);
    gsl_linalg_QR_decomp(J, tau);
    gsl_linalg_QR_unpack(J, tau, q, r);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn hybridj_iterate(
    mut vstate: *mut libc::c_void,
    mut fdf: *mut gsl_multiroot_function_fdf,
    mut x: *mut gsl_vector,
    mut f: *mut gsl_vector,
    mut J: *mut gsl_matrix,
    mut dx: *mut gsl_vector,
) -> libc::c_int {
    let mut status: libc::c_int = hybridj_iterate_impl(
        vstate,
        fdf,
        x,
        f,
        J,
        dx,
        0 as libc::c_int,
    );
    return status;
}
unsafe extern "C" fn hybridsj_iterate(
    mut vstate: *mut libc::c_void,
    mut fdf: *mut gsl_multiroot_function_fdf,
    mut x: *mut gsl_vector,
    mut f: *mut gsl_vector,
    mut J: *mut gsl_matrix,
    mut dx: *mut gsl_vector,
) -> libc::c_int {
    let mut status: libc::c_int = hybridj_iterate_impl(
        vstate,
        fdf,
        x,
        f,
        J,
        dx,
        1 as libc::c_int,
    );
    return status;
}
unsafe extern "C" fn hybridj_iterate_impl(
    mut vstate: *mut libc::c_void,
    mut fdf: *mut gsl_multiroot_function_fdf,
    mut x: *mut gsl_vector,
    mut f: *mut gsl_vector,
    mut J: *mut gsl_matrix,
    mut dx: *mut gsl_vector,
    mut scale: libc::c_int,
) -> libc::c_int {
    let mut state: *mut hybridj_state_t = vstate as *mut hybridj_state_t;
    let fnorm: libc::c_double = (*state).fnorm;
    let mut q: *mut gsl_matrix = (*state).q;
    let mut r: *mut gsl_matrix = (*state).r;
    let mut tau: *mut gsl_vector = (*state).tau;
    let mut diag: *mut gsl_vector = (*state).diag;
    let mut qtf: *mut gsl_vector = (*state).qtf;
    let mut x_trial: *mut gsl_vector = (*state).x_trial;
    let mut f_trial: *mut gsl_vector = (*state).f_trial;
    let mut df: *mut gsl_vector = (*state).df;
    let mut qtdf: *mut gsl_vector = (*state).qtdf;
    let mut rdx: *mut gsl_vector = (*state).rdx;
    let mut w: *mut gsl_vector = (*state).w;
    let mut v: *mut gsl_vector = (*state).v;
    let mut prered: libc::c_double = 0.;
    let mut actred: libc::c_double = 0.;
    let mut pnorm: libc::c_double = 0.;
    let mut fnorm1: libc::c_double = 0.;
    let mut fnorm1p: libc::c_double = 0.;
    let mut ratio: libc::c_double = 0.;
    let mut p1: libc::c_double = 0.1f64;
    let mut p5: libc::c_double = 0.5f64;
    let mut p001: libc::c_double = 0.001f64;
    let mut p0001: libc::c_double = 0.0001f64;
    compute_qtf(q, f, qtf);
    dogleg(r, qtf, diag, (*state).delta, (*state).newton, (*state).gradient, dx);
    compute_trial_step(x, dx, (*state).x_trial);
    pnorm = scaled_enorm(diag, dx);
    if (*state).iter == 1 as libc::c_int as libc::c_ulong {
        if pnorm < (*state).delta {
            (*state).delta = pnorm;
        }
    }
    let mut status: libc::c_int = (Some(((*fdf).f).unwrap()))
        .unwrap()(x_trial, (*fdf).params, f_trial);
    if status != GSL_SUCCESS as libc::c_int {
        return GSL_EBADFUNC as libc::c_int;
    }
    compute_df(f_trial, f, df);
    fnorm1 = enorm(f_trial);
    actred = compute_actual_reduction(fnorm, fnorm1);
    compute_rdx(r, dx, rdx);
    fnorm1p = enorm_sum(qtf, rdx);
    prered = compute_predicted_reduction(fnorm, fnorm1p);
    if prered > 0 as libc::c_int as libc::c_double {
        ratio = actred / prered;
    } else {
        ratio = 0 as libc::c_int as libc::c_double;
    }
    if ratio < p1 {
        (*state).ncsuc = 0 as libc::c_int as size_t;
        (*state).ncfail = ((*state).ncfail).wrapping_add(1);
        (*state).ncfail;
        (*state).delta *= p5;
    } else {
        (*state).ncfail = 0 as libc::c_int as size_t;
        (*state).ncsuc = ((*state).ncsuc).wrapping_add(1);
        (*state).ncsuc;
        if ratio >= p5 || (*state).ncsuc > 1 as libc::c_int as libc::c_ulong {
            (*state)
                .delta = if (*state).delta > pnorm / p5 {
                (*state).delta
            } else {
                pnorm / p5
            };
        }
        if fabs(ratio - 1 as libc::c_int as libc::c_double) <= p1 {
            (*state).delta = pnorm / p5;
        }
    }
    if ratio >= p0001 {
        gsl_vector_memcpy(x, x_trial);
        gsl_vector_memcpy(f, f_trial);
        (*state).fnorm = fnorm1;
        (*state).iter = ((*state).iter).wrapping_add(1);
        (*state).iter;
    }
    (*state).nslow1 = ((*state).nslow1).wrapping_add(1);
    (*state).nslow1;
    if actred >= p001 {
        (*state).nslow1 = 0 as libc::c_int as size_t;
    }
    if actred >= p1 {
        (*state).nslow2 = 0 as libc::c_int as size_t;
    }
    if (*state).ncfail == 2 as libc::c_int as libc::c_ulong {
        let mut status_0: libc::c_int = (Some(((*fdf).df).unwrap()))
            .unwrap()(x, (*fdf).params, J);
        if status_0 != GSL_SUCCESS as libc::c_int {
            return GSL_EBADFUNC as libc::c_int;
        }
        (*state).nslow2 = ((*state).nslow2).wrapping_add(1);
        (*state).nslow2;
        if (*state).iter == 1 as libc::c_int as libc::c_ulong {
            if scale != 0 {
                compute_diag(J, diag);
            }
            (*state).delta = compute_delta(diag, x);
        } else if scale != 0 {
            update_diag(J, diag);
        }
        gsl_linalg_QR_decomp(J, tau);
        gsl_linalg_QR_unpack(J, tau, q, r);
        return GSL_SUCCESS as libc::c_int;
    }
    compute_qtf(q, df, qtdf);
    compute_wv(qtdf, rdx, dx, diag, pnorm, w, v);
    gsl_linalg_QR_update(q, r, w, v);
    if (*state).nslow2 == 5 as libc::c_int as libc::c_ulong {
        return GSL_ENOPROGJ as libc::c_int;
    }
    if (*state).nslow1 == 10 as libc::c_int as libc::c_ulong {
        return GSL_ENOPROG as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn hybridj_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut hybridj_state_t = vstate as *mut hybridj_state_t;
    gsl_vector_free((*state).v);
    gsl_vector_free((*state).w);
    gsl_vector_free((*state).rdx);
    gsl_vector_free((*state).qtdf);
    gsl_vector_free((*state).df);
    gsl_vector_free((*state).f_trial);
    gsl_vector_free((*state).x_trial);
    gsl_vector_free((*state).gradient);
    gsl_vector_free((*state).newton);
    gsl_vector_free((*state).qtf);
    gsl_vector_free((*state).diag);
    gsl_vector_free((*state).tau);
    gsl_matrix_free((*state).r);
    gsl_matrix_free((*state).q);
}
static mut hybridj_type: gsl_multiroot_fdfsolver_type = {
    let mut init = gsl_multiroot_fdfsolver_type {
        name: b"hybridj\0" as *const u8 as *const libc::c_char,
        size: ::std::mem::size_of::<hybridj_state_t>() as libc::c_ulong,
        alloc: Some(
            hybridj_alloc
                as unsafe extern "C" fn(*mut libc::c_void, size_t) -> libc::c_int,
        ),
        set: Some(
            hybridj_set
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
            hybridj_iterate
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut gsl_multiroot_function_fdf,
                    *mut gsl_vector,
                    *mut gsl_vector,
                    *mut gsl_matrix,
                    *mut gsl_vector,
                ) -> libc::c_int,
        ),
        free: Some(hybridj_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    };
    init
};
static mut hybridsj_type: gsl_multiroot_fdfsolver_type = {
    let mut init = gsl_multiroot_fdfsolver_type {
        name: b"hybridsj\0" as *const u8 as *const libc::c_char,
        size: ::std::mem::size_of::<hybridj_state_t>() as libc::c_ulong,
        alloc: Some(
            hybridj_alloc
                as unsafe extern "C" fn(*mut libc::c_void, size_t) -> libc::c_int,
        ),
        set: Some(
            hybridsj_set
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
            hybridsj_iterate
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut gsl_multiroot_function_fdf,
                    *mut gsl_vector,
                    *mut gsl_vector,
                    *mut gsl_matrix,
                    *mut gsl_vector,
                ) -> libc::c_int,
        ),
        free: Some(hybridj_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    };
    init
};
pub static mut gsl_multiroot_fdfsolver_hybridj: *const gsl_multiroot_fdfsolver_type = unsafe {
    &hybridj_type as *const gsl_multiroot_fdfsolver_type
};
pub static mut gsl_multiroot_fdfsolver_hybridsj: *const gsl_multiroot_fdfsolver_type = unsafe {
    &hybridsj_type as *const gsl_multiroot_fdfsolver_type
};
