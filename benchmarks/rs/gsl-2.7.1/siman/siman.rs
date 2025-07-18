use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type size_t = libc::c_ulong;
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
pub type gsl_siman_Efunc_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
>;
pub type gsl_siman_step_t = Option::<
    unsafe extern "C" fn(*const gsl_rng, *mut libc::c_void, libc::c_double) -> (),
>;
pub type gsl_siman_metric_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_double,
>;
pub type gsl_siman_print_t = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type gsl_siman_copy_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
pub type gsl_siman_copy_construct_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
>;
pub type gsl_siman_destroy_t = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_siman_params_t {
    pub n_tries: libc::c_int,
    pub iters_fixed_T: libc::c_int,
    pub step_size: libc::c_double,
    pub k: libc::c_double,
    pub t_initial: libc::c_double,
    pub mu_t: libc::c_double,
    pub t_min: libc::c_double,
}
#[inline]
unsafe extern "C" fn gsl_rng_uniform(mut r: *const gsl_rng) -> libc::c_double {
    return ((*(*r).type_0).get_double).unwrap()((*r).state);
}
#[inline]
unsafe extern "C" fn boltzmann(
    mut E: libc::c_double,
    mut new_E: libc::c_double,
    mut T: libc::c_double,
    mut params: *mut gsl_siman_params_t,
) -> libc::c_double {
    let mut x: libc::c_double = -(new_E - E) / ((*params).k * T);
    return if x < -7.0839641853226408e+02f64 { 0.0f64 } else { exp(x) };
}
#[inline]
unsafe extern "C" fn copy_state(
    mut src: *mut libc::c_void,
    mut dst: *mut libc::c_void,
    mut size: size_t,
    mut copyfunc: gsl_siman_copy_t,
) {
    if copyfunc.is_some() {
        copyfunc.unwrap()(src, dst);
    } else {
        memcpy(dst, src, size);
    };
}
pub unsafe extern "C" fn gsl_siman_solve(
    mut r: *const gsl_rng,
    mut x0_p: *mut libc::c_void,
    mut Ef: gsl_siman_Efunc_t,
    mut take_step: gsl_siman_step_t,
    mut distance: gsl_siman_metric_t,
    mut print_position: gsl_siman_print_t,
    mut copyfunc: gsl_siman_copy_t,
    mut copy_constructor: gsl_siman_copy_construct_t,
    mut destructor: gsl_siman_destroy_t,
    mut element_size: size_t,
    mut params: gsl_siman_params_t,
) {
    let mut x: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut new_x: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut best_x: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut E: libc::c_double = 0.;
    let mut new_E: libc::c_double = 0.;
    let mut best_E: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut T: libc::c_double = 0.;
    let mut T_factor: libc::c_double = 0.;
    let mut n_evals: libc::c_int = 1 as libc::c_int;
    let mut n_iter: libc::c_int = 0 as libc::c_int;
    let mut n_accepts: libc::c_int = 0;
    let mut n_rejects: libc::c_int = 0;
    let mut n_eless: libc::c_int = 0;
    if copyfunc.is_some() && copy_constructor.is_some() && destructor.is_some()
        || element_size != 0 as libc::c_int as libc::c_ulong
    {} else {
        __assert_fail(
            b"(copyfunc != NULL && copy_constructor != NULL && destructor != NULL) || (element_size != 0)\0"
                as *const u8 as *const libc::c_char,
            b"siman.c\0" as *const u8 as *const libc::c_char,
            73 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 217],
                &[libc::c_char; 217],
            >(
                b"void gsl_siman_solve(const gsl_rng *, void *, gsl_siman_Efunc_t, gsl_siman_step_t, gsl_siman_metric_t, gsl_siman_print_t, gsl_siman_copy_t, gsl_siman_copy_construct_t, gsl_siman_destroy_t, size_t, gsl_siman_params_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4418: {
        if copyfunc.is_some() && copy_constructor.is_some() && destructor.is_some()
            || element_size != 0 as libc::c_int as libc::c_ulong
        {} else {
            __assert_fail(
                b"(copyfunc != NULL && copy_constructor != NULL && destructor != NULL) || (element_size != 0)\0"
                    as *const u8 as *const libc::c_char,
                b"siman.c\0" as *const u8 as *const libc::c_char,
                73 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 217],
                    &[libc::c_char; 217],
                >(
                    b"void gsl_siman_solve(const gsl_rng *, void *, gsl_siman_Efunc_t, gsl_siman_step_t, gsl_siman_metric_t, gsl_siman_print_t, gsl_siman_copy_t, gsl_siman_copy_construct_t, gsl_siman_destroy_t, size_t, gsl_siman_params_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    distance = None;
    E = Ef.unwrap()(x0_p);
    if copyfunc.is_some() {
        x = copy_constructor.unwrap()(x0_p);
        new_x = copy_constructor.unwrap()(x0_p);
        best_x = copy_constructor.unwrap()(x0_p);
    } else {
        x = malloc(element_size);
        memcpy(x, x0_p, element_size);
        new_x = malloc(element_size);
        best_x = malloc(element_size);
        memcpy(best_x, x0_p, element_size);
    }
    best_E = E;
    T = params.t_initial;
    T_factor = 1.0f64 / params.mu_t;
    if print_position.is_some() {
        printf(
            b"#-iter  #-evals   temperature     position   energy\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    loop {
        n_accepts = 0 as libc::c_int;
        n_rejects = 0 as libc::c_int;
        n_eless = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < params.iters_fixed_T {
            copy_state(x, new_x, element_size, copyfunc);
            take_step.unwrap()(r, new_x, params.step_size);
            new_E = Ef.unwrap()(new_x);
            if new_E <= best_E {
                if copyfunc.is_some() {
                    copyfunc.unwrap()(new_x, best_x);
                } else {
                    memcpy(best_x, new_x, element_size);
                }
                best_E = new_E;
            }
            n_evals += 1;
            n_evals;
            if new_E < E {
                if new_E < best_E {
                    copy_state(new_x, best_x, element_size, copyfunc);
                    best_E = new_E;
                }
                copy_state(new_x, x, element_size, copyfunc);
                E = new_E;
                n_eless += 1;
                n_eless;
            } else if gsl_rng_uniform(r) < boltzmann(E, new_E, T, &mut params) {
                copy_state(new_x, x, element_size, copyfunc);
                E = new_E;
                n_accepts += 1;
                n_accepts;
            } else {
                n_rejects += 1;
                n_rejects;
            }
            i += 1;
            i;
        }
        if print_position.is_some() {
            printf(
                b"%5d   %7d  %12g\0" as *const u8 as *const libc::c_char,
                n_iter,
                n_evals,
                T,
            );
            print_position.unwrap()(x);
            printf(b"  %12g  %12g\n\0" as *const u8 as *const libc::c_char, E, best_E);
        }
        T *= T_factor;
        n_iter += 1;
        n_iter;
        if T < params.t_min {
            break;
        }
    }
    copy_state(best_x, x0_p, element_size, copyfunc);
    if copyfunc.is_some() {
        destructor.unwrap()(x);
        destructor.unwrap()(new_x);
        destructor.unwrap()(best_x);
    } else {
        free(x);
        free(new_x);
        free(best_x);
    };
}
pub unsafe extern "C" fn gsl_siman_solve_many(
    mut r: *const gsl_rng,
    mut x0_p: *mut libc::c_void,
    mut Ef: gsl_siman_Efunc_t,
    mut take_step: gsl_siman_step_t,
    mut distance: gsl_siman_metric_t,
    mut print_position: gsl_siman_print_t,
    mut element_size: size_t,
    mut params: gsl_siman_params_t,
) {
    let mut x: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut new_x: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut energies: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut probs: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut sum_probs: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut Ex: libc::c_double = 0.;
    let mut T: libc::c_double = 0.;
    let mut T_factor: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut u: libc::c_double = 0.;
    let mut n_iter: libc::c_int = 0;
    if print_position.is_some() {
        printf(
            b"#-iter    temperature       position\0" as *const u8 as *const libc::c_char,
        );
        printf(
            b"         delta_pos        energy\n\0" as *const u8 as *const libc::c_char,
        );
    }
    x = malloc((params.n_tries as libc::c_ulong).wrapping_mul(element_size));
    new_x = malloc((params.n_tries as libc::c_ulong).wrapping_mul(element_size));
    energies = malloc(
        (params.n_tries as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    probs = malloc(
        (params.n_tries as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    sum_probs = malloc(
        (params.n_tries as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    T = params.t_initial;
    T_factor = 1.0f64 / params.mu_t;
    memcpy(x, x0_p, element_size);
    n_iter = 0 as libc::c_int;
    loop {
        Ex = Ef.unwrap()(x);
        i = 0 as libc::c_int;
        while i < params.n_tries - 1 as libc::c_int {
            *sum_probs.offset(i as isize) = 0 as libc::c_int as libc::c_double;
            memcpy(
                (new_x as *mut libc::c_char)
                    .offset((i as libc::c_ulong).wrapping_mul(element_size) as isize)
                    as *mut libc::c_void,
                x,
                element_size,
            );
            take_step
                .unwrap()(
                r,
                (new_x as *mut libc::c_char)
                    .offset((i as libc::c_ulong).wrapping_mul(element_size) as isize)
                    as *mut libc::c_void,
                params.step_size,
            );
            *energies
                .offset(
                    i as isize,
                ) = Ef
                .unwrap()(
                (new_x as *mut libc::c_char)
                    .offset((i as libc::c_ulong).wrapping_mul(element_size) as isize)
                    as *mut libc::c_void,
            );
            *probs
                .offset(
                    i as isize,
                ) = boltzmann(Ex, *energies.offset(i as isize), T, &mut params);
            i += 1;
            i;
        }
        memcpy(
            (new_x as *mut libc::c_char)
                .offset(
                    ((params.n_tries - 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(element_size) as isize,
                ) as *mut libc::c_void,
            x,
            element_size,
        );
        *energies.offset((params.n_tries - 1 as libc::c_int) as isize) = Ex;
        *probs
            .offset(
                (params.n_tries - 1 as libc::c_int) as isize,
            ) = boltzmann(Ex, *energies.offset(i as isize), T, &mut params);
        *sum_probs
            .offset(
                0 as libc::c_int as isize,
            ) = *probs.offset(0 as libc::c_int as isize);
        i = 1 as libc::c_int;
        while i < params.n_tries {
            *sum_probs
                .offset(
                    i as isize,
                ) = *sum_probs.offset((i - 1 as libc::c_int) as isize)
                + *probs.offset(i as isize);
            i += 1;
            i;
        }
        u = gsl_rng_uniform(r)
            * *sum_probs.offset((params.n_tries - 1 as libc::c_int) as isize);
        i = 0 as libc::c_int;
        while i < params.n_tries {
            if u < *sum_probs.offset(i as isize) {
                memcpy(
                    x,
                    (new_x as *mut libc::c_char)
                        .offset((i as libc::c_ulong).wrapping_mul(element_size) as isize)
                        as *const libc::c_void,
                    element_size,
                );
                break;
            } else {
                i += 1;
                i;
            }
        }
        if print_position.is_some() {
            printf(b"%5d\t%12g\t\0" as *const u8 as *const libc::c_char, n_iter, T);
            print_position.unwrap()(x);
            printf(
                b"\t%12g\t%12g\n\0" as *const u8 as *const libc::c_char,
                distance.unwrap()(x, x0_p),
                Ex,
            );
        }
        T *= T_factor;
        n_iter += 1;
        n_iter;
        if T < params.t_min {
            break;
        }
    }
    memcpy(x0_p, x, element_size);
    free(x);
    free(new_x);
    free(energies as *mut libc::c_void);
    free(probs as *mut libc::c_void);
    free(sum_probs as *mut libc::c_void);
}
