use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn log(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn gsl_pow_int(x: libc::c_double, n: libc::c_int) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
pub struct gsl_monte_function_struct {
    pub f: Option::<
        unsafe extern "C" fn(
            *mut libc::c_double,
            size_t,
            *mut libc::c_void,
        ) -> libc::c_double,
    >,
    pub dim: size_t,
    pub params: *mut libc::c_void,
}
pub type gsl_monte_function = gsl_monte_function_struct;
pub type C2RustUnnamed_0 = libc::c_int;
pub const GSL_VEGAS_MODE_STRATIFIED: C2RustUnnamed_0 = -1;
pub const GSL_VEGAS_MODE_IMPORTANCE_ONLY: C2RustUnnamed_0 = 0;
pub const GSL_VEGAS_MODE_IMPORTANCE: C2RustUnnamed_0 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_monte_vegas_state {
    pub dim: size_t,
    pub bins_max: size_t,
    pub bins: libc::c_uint,
    pub boxes: libc::c_uint,
    pub xi: *mut libc::c_double,
    pub xin: *mut libc::c_double,
    pub delx: *mut libc::c_double,
    pub weight: *mut libc::c_double,
    pub vol: libc::c_double,
    pub x: *mut libc::c_double,
    pub bin: *mut libc::c_int,
    pub box_0: *mut libc::c_int,
    pub d: *mut libc::c_double,
    pub alpha: libc::c_double,
    pub mode: libc::c_int,
    pub verbose: libc::c_int,
    pub iterations: libc::c_uint,
    pub stage: libc::c_int,
    pub jac: libc::c_double,
    pub wtd_int_sum: libc::c_double,
    pub sum_wgts: libc::c_double,
    pub chi_sum: libc::c_double,
    pub chisq: libc::c_double,
    pub result: libc::c_double,
    pub sigma: libc::c_double,
    pub it_start: libc::c_uint,
    pub it_num: libc::c_uint,
    pub samples: libc::c_uint,
    pub calls_per_box: libc::c_uint,
    pub ostream: *mut FILE,
}
pub type coord = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_monte_vegas_params {
    pub alpha: libc::c_double,
    pub iterations: size_t,
    pub stage: libc::c_int,
    pub mode: libc::c_int,
    pub verbose: libc::c_int,
    pub ostream: *mut FILE,
}
#[inline]
unsafe extern "C" fn gsl_rng_uniform_pos(mut r: *const gsl_rng) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    loop {
        x = ((*(*r).type_0).get_double).unwrap()((*r).state);
        if !(x == 0 as libc::c_int as libc::c_double) {
            break;
        }
    }
    return x;
}
pub unsafe extern "C" fn gsl_monte_vegas_integrate(
    mut f: *mut gsl_monte_function,
    mut xl: *mut libc::c_double,
    mut xu: *mut libc::c_double,
    mut dim: size_t,
    mut calls: size_t,
    mut r: *mut gsl_rng,
    mut state: *mut gsl_monte_vegas_state,
    mut result: *mut libc::c_double,
    mut abserr: *mut libc::c_double,
) -> libc::c_int {
    let mut cum_int: libc::c_double = 0.;
    let mut cum_sig: libc::c_double = 0.;
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    let mut it: size_t = 0;
    if dim != (*state).dim {
        gsl_error(
            b"number of dimensions must match allocated size\0" as *const u8
                as *const libc::c_char,
            b"vegas.c\0" as *const u8 as *const libc::c_char,
            121 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < dim {
        if *xu.offset(i as isize) <= *xl.offset(i as isize) {
            gsl_error(
                b"xu must be greater than xl\0" as *const u8 as *const libc::c_char,
                b"vegas.c\0" as *const u8 as *const libc::c_char,
                128 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        if *xu.offset(i as isize) - *xl.offset(i as isize) > 1.7976931348623157e+308f64 {
            gsl_error(
                b"Range of integration is too large, please rescale\0" as *const u8
                    as *const libc::c_char,
                b"vegas.c\0" as *const u8 as *const libc::c_char,
                134 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    if (*state).stage == 0 as libc::c_int {
        init_grid(state, xl, xu, dim);
        if (*state).verbose >= 0 as libc::c_int {
            print_lim(state, xl, xu, dim);
        }
    }
    if (*state).stage <= 1 as libc::c_int {
        (*state).wtd_int_sum = 0 as libc::c_int as libc::c_double;
        (*state).sum_wgts = 0 as libc::c_int as libc::c_double;
        (*state).chi_sum = 0 as libc::c_int as libc::c_double;
        (*state).it_num = 1 as libc::c_int as libc::c_uint;
        (*state).samples = 0 as libc::c_int as libc::c_uint;
        (*state).chisq = 0 as libc::c_int as libc::c_double;
    }
    if (*state).stage <= 2 as libc::c_int {
        let mut bins: libc::c_uint = (*state).bins_max as libc::c_uint;
        let mut boxes: libc::c_uint = 1 as libc::c_int as libc::c_uint;
        if (*state).mode != GSL_VEGAS_MODE_IMPORTANCE_ONLY as libc::c_int {
            boxes = floor(
                pow(calls as libc::c_double / 2.0f64, 1.0f64 / dim as libc::c_double),
            ) as libc::c_uint;
            (*state).mode = GSL_VEGAS_MODE_IMPORTANCE as libc::c_int;
            if (2 as libc::c_int as libc::c_uint).wrapping_mul(boxes) as libc::c_ulong
                >= (*state).bins_max
            {
                let mut box_per_bin: libc::c_int = (if (boxes as libc::c_ulong)
                    .wrapping_div((*state).bins_max) > 1 as libc::c_int as libc::c_ulong
                {
                    (boxes as libc::c_ulong).wrapping_div((*state).bins_max)
                } else {
                    1 as libc::c_int as libc::c_ulong
                }) as libc::c_int;
                bins = (if (boxes.wrapping_div(box_per_bin as libc::c_uint)
                    as libc::c_ulong) < (*state).bins_max
                {
                    boxes.wrapping_div(box_per_bin as libc::c_uint) as libc::c_ulong
                } else {
                    (*state).bins_max
                }) as libc::c_uint;
                boxes = (box_per_bin as libc::c_uint).wrapping_mul(bins);
                (*state).mode = GSL_VEGAS_MODE_STRATIFIED as libc::c_int;
            }
        }
        let mut tot_boxes: libc::c_double = gsl_pow_int(
            boxes as libc::c_double,
            dim as libc::c_int,
        );
        (*state)
            .calls_per_box = (if calls as libc::c_double / tot_boxes
            > 2 as libc::c_int as libc::c_double
        {
            calls as libc::c_double / tot_boxes
        } else {
            2 as libc::c_int as libc::c_double
        }) as libc::c_uint;
        calls = ((*state).calls_per_box as libc::c_double * tot_boxes) as size_t;
        (*state)
            .jac = (*state).vol * pow(bins as libc::c_double, dim as libc::c_double)
            / calls as libc::c_double;
        (*state).boxes = boxes;
        if bins != (*state).bins {
            resize_grid(state, bins);
            if (*state).verbose > 1 as libc::c_int {
                print_grid(state, dim);
            }
        }
        if (*state).verbose >= 0 as libc::c_int {
            print_head(
                state,
                dim,
                calls,
                (*state).it_num,
                (*state).bins,
                (*state).boxes,
            );
        }
    }
    (*state).it_start = (*state).it_num;
    cum_int = 0.0f64;
    cum_sig = 0.0f64;
    it = 0 as libc::c_int as size_t;
    while it < (*state).iterations as libc::c_ulong {
        let mut intgrl: libc::c_double = 0.0f64;
        let mut intgrl_sq: libc::c_double = 0.0f64;
        let mut tss: libc::c_double = 0.0f64;
        let mut wgt: libc::c_double = 0.;
        let mut var: libc::c_double = 0.;
        let mut sig: libc::c_double = 0.;
        let mut calls_per_box: size_t = (*state).calls_per_box as size_t;
        let mut jacbin: libc::c_double = (*state).jac;
        let mut x: *mut libc::c_double = (*state).x;
        let mut bin: *mut coord = (*state).bin;
        (*state)
            .it_num = ((*state).it_start as libc::c_ulong).wrapping_add(it)
            as libc::c_uint;
        reset_grid_values(state);
        init_box_coord(state, (*state).box_0);
        loop {
            let mut m: libc::c_double = 0 as libc::c_int as libc::c_double;
            let mut q: libc::c_double = 0 as libc::c_int as libc::c_double;
            let mut f_sq_sum: libc::c_double = 0.0f64;
            k = 0 as libc::c_int as size_t;
            while k < calls_per_box {
                let mut fval: libc::c_double = 0.;
                let mut bin_vol: libc::c_double = 0.;
                random_point(
                    x,
                    bin,
                    &mut bin_vol,
                    (*state).box_0 as *const coord,
                    xl as *const libc::c_double,
                    xu as *const libc::c_double,
                    state,
                    r,
                );
                ::std::ptr::write_volatile(
                    &mut fval as *mut libc::c_double,
                    jacbin * bin_vol
                        * (Some(((*f).f).unwrap())).unwrap()(x, (*f).dim, (*f).params),
                );
                let mut d: libc::c_double = fval - m;
                ::std::ptr::write_volatile(
                    &mut m as *mut libc::c_double,
                    ::std::ptr::read_volatile::<
                        libc::c_double,
                    >(&m as *const libc::c_double) + d / (k as libc::c_double + 1.0f64),
                );
                ::std::ptr::write_volatile(
                    &mut q as *mut libc::c_double,
                    ::std::ptr::read_volatile::<
                        libc::c_double,
                    >(&q as *const libc::c_double)
                        + d * d * (k as libc::c_double / (k as libc::c_double + 1.0f64)),
                );
                if (*state).mode != GSL_VEGAS_MODE_STRATIFIED as libc::c_int {
                    let mut f_sq: libc::c_double = fval * fval;
                    accumulate_distribution(state, bin, f_sq);
                }
                k = k.wrapping_add(1);
                k;
            }
            intgrl += m * calls_per_box as libc::c_double;
            f_sq_sum = q * calls_per_box as libc::c_double;
            tss += f_sq_sum;
            if (*state).mode == GSL_VEGAS_MODE_STRATIFIED as libc::c_int {
                accumulate_distribution(state, bin, f_sq_sum);
            }
            if !(change_box_coord(state, (*state).box_0) != 0) {
                break;
            }
        }
        var = tss / (calls_per_box as libc::c_double - 1.0f64);
        if var > 0 as libc::c_int as libc::c_double {
            wgt = 1.0f64 / var;
        } else if (*state).sum_wgts > 0 as libc::c_int as libc::c_double {
            wgt = (*state).sum_wgts / (*state).samples as libc::c_double;
        } else {
            wgt = 0.0f64;
        }
        intgrl_sq = intgrl * intgrl;
        sig = sqrt(var);
        (*state).result = intgrl;
        (*state).sigma = sig;
        if wgt > 0.0f64 {
            let mut sum_wgts: libc::c_double = (*state).sum_wgts;
            let mut wtd_int_sum: libc::c_double = (*state).wtd_int_sum;
            let mut m_0: libc::c_double = if sum_wgts
                > 0 as libc::c_int as libc::c_double
            {
                wtd_int_sum / sum_wgts
            } else {
                0 as libc::c_int as libc::c_double
            };
            let mut q_0: libc::c_double = intgrl - m_0;
            (*state).samples = ((*state).samples).wrapping_add(1);
            (*state).samples;
            (*state).sum_wgts += wgt;
            (*state).wtd_int_sum += intgrl * wgt;
            (*state).chi_sum += intgrl_sq * wgt;
            cum_int = (*state).wtd_int_sum / (*state).sum_wgts;
            cum_sig = sqrt(1 as libc::c_int as libc::c_double / (*state).sum_wgts);
            if (*state).samples == 1 as libc::c_int as libc::c_uint {
                (*state).chisq = 0 as libc::c_int as libc::c_double;
            } else {
                (*state).chisq *= (*state).samples as libc::c_double - 2.0f64;
                (*state).chisq
                    += wgt / (1 as libc::c_int as libc::c_double + wgt / sum_wgts) * q_0
                        * q_0;
                (*state).chisq /= (*state).samples as libc::c_double - 1.0f64;
            }
        } else {
            cum_int += (intgrl - cum_int) / (it as libc::c_double + 1.0f64);
            cum_sig = 0.0f64;
        }
        if (*state).verbose >= 0 as libc::c_int {
            print_res(
                state,
                (*state).it_num,
                intgrl,
                sig,
                cum_int,
                cum_sig,
                (*state).chisq,
            );
            if it.wrapping_add(1 as libc::c_int as libc::c_ulong)
                == (*state).iterations as libc::c_ulong
                && (*state).verbose > 0 as libc::c_int
            {
                print_grid(state, dim);
            }
        }
        if (*state).verbose > 1 as libc::c_int {
            print_dist(state, dim);
        }
        refine_grid(state);
        if (*state).verbose > 1 as libc::c_int {
            print_grid(state, dim);
        }
        it = it.wrapping_add(1);
        it;
    }
    (*state).stage = 1 as libc::c_int;
    *result = cum_int;
    *abserr = cum_sig;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_monte_vegas_alloc(
    mut dim: size_t,
) -> *mut gsl_monte_vegas_state {
    let mut s: *mut gsl_monte_vegas_state = malloc(
        ::std::mem::size_of::<gsl_monte_vegas_state>() as libc::c_ulong,
    ) as *mut gsl_monte_vegas_state;
    if s.is_null() {
        gsl_error(
            b"failed to allocate space for vegas state struct\0" as *const u8
                as *const libc::c_char,
            b"vegas.c\0" as *const u8 as *const libc::c_char,
            390 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_monte_vegas_state;
    }
    (*s)
        .delx = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*s).delx).is_null() {
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for delx\0" as *const u8 as *const libc::c_char,
            b"vegas.c\0" as *const u8 as *const libc::c_char,
            398 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_monte_vegas_state;
    }
    (*s)
        .d = malloc(
        (50 as libc::c_int as libc::c_ulong)
            .wrapping_mul(dim)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*s).d).is_null() {
        free((*s).delx as *mut libc::c_void);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for d\0" as *const u8 as *const libc::c_char,
            b"vegas.c\0" as *const u8 as *const libc::c_char,
            407 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_monte_vegas_state;
    }
    (*s)
        .xi = malloc(
        ((50 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(dim)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*s).xi).is_null() {
        free((*s).d as *mut libc::c_void);
        free((*s).delx as *mut libc::c_void);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for xi\0" as *const u8 as *const libc::c_char,
            b"vegas.c\0" as *const u8 as *const libc::c_char,
            417 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_monte_vegas_state;
    }
    (*s)
        .xin = malloc(
        ((50 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*s).xin).is_null() {
        free((*s).xi as *mut libc::c_void);
        free((*s).d as *mut libc::c_void);
        free((*s).delx as *mut libc::c_void);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for xin\0" as *const u8 as *const libc::c_char,
            b"vegas.c\0" as *const u8 as *const libc::c_char,
            428 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_monte_vegas_state;
    }
    (*s)
        .weight = malloc(
        (50 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*s).weight).is_null() {
        free((*s).xin as *mut libc::c_void);
        free((*s).xi as *mut libc::c_void);
        free((*s).d as *mut libc::c_void);
        free((*s).delx as *mut libc::c_void);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for xin\0" as *const u8 as *const libc::c_char,
            b"vegas.c\0" as *const u8 as *const libc::c_char,
            440 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_monte_vegas_state;
    }
    (*s)
        .box_0 = malloc(
        dim.wrapping_mul(::std::mem::size_of::<coord>() as libc::c_ulong),
    ) as *mut coord;
    if ((*s).box_0).is_null() {
        free((*s).weight as *mut libc::c_void);
        free((*s).xin as *mut libc::c_void);
        free((*s).xi as *mut libc::c_void);
        free((*s).d as *mut libc::c_void);
        free((*s).delx as *mut libc::c_void);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for box\0" as *const u8 as *const libc::c_char,
            b"vegas.c\0" as *const u8 as *const libc::c_char,
            453 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_monte_vegas_state;
    }
    (*s)
        .bin = malloc(dim.wrapping_mul(::std::mem::size_of::<coord>() as libc::c_ulong))
        as *mut coord;
    if ((*s).bin).is_null() {
        free((*s).box_0 as *mut libc::c_void);
        free((*s).weight as *mut libc::c_void);
        free((*s).xin as *mut libc::c_void);
        free((*s).xi as *mut libc::c_void);
        free((*s).d as *mut libc::c_void);
        free((*s).delx as *mut libc::c_void);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for bin\0" as *const u8 as *const libc::c_char,
            b"vegas.c\0" as *const u8 as *const libc::c_char,
            467 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_monte_vegas_state;
    }
    (*s)
        .x = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*s).x).is_null() {
        free((*s).bin as *mut libc::c_void);
        free((*s).box_0 as *mut libc::c_void);
        free((*s).weight as *mut libc::c_void);
        free((*s).xin as *mut libc::c_void);
        free((*s).xi as *mut libc::c_void);
        free((*s).d as *mut libc::c_void);
        free((*s).delx as *mut libc::c_void);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for x\0" as *const u8 as *const libc::c_char,
            b"vegas.c\0" as *const u8 as *const libc::c_char,
            482 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_monte_vegas_state;
    }
    (*s).dim = dim;
    (*s).bins_max = 50 as libc::c_int as size_t;
    gsl_monte_vegas_init(s);
    return s;
}
pub unsafe extern "C" fn gsl_monte_vegas_init(
    mut state: *mut gsl_monte_vegas_state,
) -> libc::c_int {
    (*state).stage = 0 as libc::c_int;
    (*state).alpha = 1.5f64;
    (*state).verbose = -(1 as libc::c_int);
    (*state).iterations = 5 as libc::c_int as libc::c_uint;
    (*state).mode = GSL_VEGAS_MODE_IMPORTANCE as libc::c_int;
    (*state).chisq = 0 as libc::c_int as libc::c_double;
    (*state).bins = (*state).bins_max as libc::c_uint;
    (*state).ostream = stdout;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_monte_vegas_free(mut s: *mut gsl_monte_vegas_state) {
    if s.is_null() {
        return;
    }
    free((*s).x as *mut libc::c_void);
    free((*s).delx as *mut libc::c_void);
    free((*s).d as *mut libc::c_void);
    free((*s).xi as *mut libc::c_void);
    free((*s).xin as *mut libc::c_void);
    free((*s).weight as *mut libc::c_void);
    free((*s).box_0 as *mut libc::c_void);
    free((*s).bin as *mut libc::c_void);
    free(s as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_monte_vegas_chisq(
    mut s: *const gsl_monte_vegas_state,
) -> libc::c_double {
    return (*s).chisq;
}
pub unsafe extern "C" fn gsl_monte_vegas_runval(
    mut s: *const gsl_monte_vegas_state,
    mut result: *mut libc::c_double,
    mut sigma: *mut libc::c_double,
) {
    *result = (*s).result;
    *sigma = (*s).sigma;
}
pub unsafe extern "C" fn gsl_monte_vegas_params_get(
    mut s: *const gsl_monte_vegas_state,
    mut p: *mut gsl_monte_vegas_params,
) {
    (*p).alpha = (*s).alpha;
    (*p).iterations = (*s).iterations as size_t;
    (*p).stage = (*s).stage;
    (*p).mode = (*s).mode;
    (*p).verbose = (*s).verbose;
    (*p).ostream = (*s).ostream;
}
pub unsafe extern "C" fn gsl_monte_vegas_params_set(
    mut s: *mut gsl_monte_vegas_state,
    mut p: *const gsl_monte_vegas_params,
) {
    (*s).alpha = (*p).alpha;
    (*s).iterations = (*p).iterations as libc::c_uint;
    (*s).stage = (*p).stage;
    (*s).mode = (*p).mode;
    (*s).verbose = (*p).verbose;
    (*s).ostream = (*p).ostream;
}
unsafe extern "C" fn init_box_coord(
    mut s: *mut gsl_monte_vegas_state,
    mut box_0: *mut coord,
) {
    let mut i: size_t = 0;
    let mut dim: size_t = (*s).dim;
    i = 0 as libc::c_int as size_t;
    while i < dim {
        *box_0.offset(i as isize) = 0 as libc::c_int;
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn change_box_coord(
    mut s: *mut gsl_monte_vegas_state,
    mut box_0: *mut coord,
) -> libc::c_int {
    let mut j: libc::c_int = ((*s).dim).wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as libc::c_int;
    let mut ng: libc::c_int = (*s).boxes as libc::c_int;
    while j >= 0 as libc::c_int {
        *box_0.offset(j as isize) = (*box_0.offset(j as isize) + 1 as libc::c_int) % ng;
        if *box_0.offset(j as isize) != 0 as libc::c_int {
            return 1 as libc::c_int;
        }
        j -= 1;
        j;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn init_grid(
    mut s: *mut gsl_monte_vegas_state,
    mut xl: *mut libc::c_double,
    mut xu: *mut libc::c_double,
    mut dim: size_t,
) {
    let mut j: size_t = 0;
    let mut vol: libc::c_double = 1.0f64;
    (*s).bins = 1 as libc::c_int as libc::c_uint;
    j = 0 as libc::c_int as size_t;
    while j < dim {
        let mut dx: libc::c_double = *xu.offset(j as isize) - *xl.offset(j as isize);
        *((*s).delx).offset(j as isize) = dx;
        vol *= dx;
        *((*s).xi)
            .offset(
                (0 as libc::c_int as libc::c_ulong)
                    .wrapping_mul((*s).dim)
                    .wrapping_add(j) as isize,
            ) = 0.0f64;
        *((*s).xi)
            .offset(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul((*s).dim)
                    .wrapping_add(j) as isize,
            ) = 1.0f64;
        j = j.wrapping_add(1);
        j;
    }
    (*s).vol = vol;
}
unsafe extern "C" fn reset_grid_values(mut s: *mut gsl_monte_vegas_state) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut dim: size_t = (*s).dim;
    let mut bins: size_t = (*s).bins as size_t;
    i = 0 as libc::c_int as size_t;
    while i < bins {
        j = 0 as libc::c_int as size_t;
        while j < dim {
            *((*s).d).offset(i.wrapping_mul((*s).dim).wrapping_add(j) as isize) = 0.0f64;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn accumulate_distribution(
    mut s: *mut gsl_monte_vegas_state,
    mut bin: *mut coord,
    mut y: libc::c_double,
) {
    let mut j: size_t = 0;
    let mut dim: size_t = (*s).dim;
    j = 0 as libc::c_int as size_t;
    while j < dim {
        let mut i: libc::c_int = *bin.offset(j as isize);
        *((*s).d)
            .offset((i as libc::c_ulong).wrapping_mul((*s).dim).wrapping_add(j) as isize)
            += y;
        j = j.wrapping_add(1);
        j;
    }
}
unsafe extern "C" fn random_point(
    mut x: *mut libc::c_double,
    mut bin: *mut coord,
    mut bin_vol: *mut libc::c_double,
    mut box_0: *const coord,
    mut xl: *const libc::c_double,
    mut xu: *const libc::c_double,
    mut s: *mut gsl_monte_vegas_state,
    mut r: *mut gsl_rng,
) {
    let mut vol: libc::c_double = 1.0f64;
    let mut j: size_t = 0;
    let mut dim: size_t = (*s).dim;
    let mut bins: size_t = (*s).bins as size_t;
    let mut boxes: size_t = (*s).boxes as size_t;
    while if !xu.is_null() { 0 as libc::c_int } else { 0 as libc::c_int } != 0 {}
    j = 0 as libc::c_int as size_t;
    while j < dim {
        let mut z: libc::c_double = (*box_0.offset(j as isize) as libc::c_double
            + gsl_rng_uniform_pos(r)) / boxes as libc::c_double * bins as libc::c_double;
        let mut k: libc::c_int = z as libc::c_int;
        let mut y: libc::c_double = 0.;
        let mut bin_width: libc::c_double = 0.;
        *bin.offset(j as isize) = k;
        if k == 0 as libc::c_int {
            bin_width = *((*s).xi)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul((*s).dim)
                        .wrapping_add(j) as isize,
                );
            y = z * bin_width;
        } else {
            bin_width = *((*s).xi)
                .offset(
                    ((k + 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul((*s).dim)
                        .wrapping_add(j) as isize,
                )
                - *((*s).xi)
                    .offset(
                        (k as libc::c_ulong).wrapping_mul((*s).dim).wrapping_add(j)
                            as isize,
                    );
            y = *((*s).xi)
                .offset(
                    (k as libc::c_ulong).wrapping_mul((*s).dim).wrapping_add(j) as isize,
                ) + (z - k as libc::c_double) * bin_width;
        }
        *x
            .offset(
                j as isize,
            ) = *xl.offset(j as isize) + y * *((*s).delx).offset(j as isize);
        vol *= bin_width;
        j = j.wrapping_add(1);
        j;
    }
    *bin_vol = vol;
}
unsafe extern "C" fn resize_grid(
    mut s: *mut gsl_monte_vegas_state,
    mut bins: libc::c_uint,
) {
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut dim: size_t = (*s).dim;
    let mut pts_per_bin: libc::c_double = (*s).bins as libc::c_double
        / bins as libc::c_double;
    j = 0 as libc::c_int as size_t;
    while j < dim {
        let mut xold: libc::c_double = 0.;
        let mut xnew: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut dw: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut i: libc::c_int = 1 as libc::c_int;
        k = 1 as libc::c_int as size_t;
        while k <= (*s).bins as libc::c_ulong {
            dw += 1.0f64;
            xold = xnew;
            xnew = *((*s).xi).offset(k.wrapping_mul((*s).dim).wrapping_add(j) as isize);
            while dw > pts_per_bin {
                dw -= pts_per_bin;
                *((*s).xin).offset(i as isize) = xnew - (xnew - xold) * dw;
                i += 1;
                i;
            }
            k = k.wrapping_add(1);
            k;
        }
        k = 1 as libc::c_int as size_t;
        while k < bins as libc::c_ulong {
            *((*s).xi)
                .offset(
                    k.wrapping_mul((*s).dim).wrapping_add(j) as isize,
                ) = *((*s).xin).offset(k as isize);
            k = k.wrapping_add(1);
            k;
        }
        *((*s).xi)
            .offset(
                (bins as libc::c_ulong).wrapping_mul((*s).dim).wrapping_add(j) as isize,
            ) = 1 as libc::c_int as libc::c_double;
        j = j.wrapping_add(1);
        j;
    }
    (*s).bins = bins;
}
unsafe extern "C" fn refine_grid(mut s: *mut gsl_monte_vegas_state) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut dim: size_t = (*s).dim;
    let mut bins: size_t = (*s).bins as size_t;
    j = 0 as libc::c_int as size_t;
    while j < dim {
        let mut grid_tot_j: libc::c_double = 0.;
        let mut tot_weight: libc::c_double = 0.;
        let mut weight: *mut libc::c_double = (*s).weight;
        let mut oldg: libc::c_double = *((*s).d)
            .offset(
                (0 as libc::c_int as libc::c_ulong)
                    .wrapping_mul((*s).dim)
                    .wrapping_add(j) as isize,
            );
        let mut newg: libc::c_double = *((*s).d)
            .offset(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul((*s).dim)
                    .wrapping_add(j) as isize,
            );
        *((*s).d)
            .offset(
                (0 as libc::c_int as libc::c_ulong)
                    .wrapping_mul((*s).dim)
                    .wrapping_add(j) as isize,
            ) = (oldg + newg) / 2 as libc::c_int as libc::c_double;
        grid_tot_j = *((*s).d)
            .offset(
                (0 as libc::c_int as libc::c_ulong)
                    .wrapping_mul((*s).dim)
                    .wrapping_add(j) as isize,
            );
        i = 1 as libc::c_int as size_t;
        while i < bins.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            let mut rc: libc::c_double = oldg + newg;
            oldg = newg;
            newg = *((*s).d)
                .offset(
                    i
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul((*s).dim)
                        .wrapping_add(j) as isize,
                );
            *((*s).d)
                .offset(
                    i.wrapping_mul((*s).dim).wrapping_add(j) as isize,
                ) = (rc + newg) / 3 as libc::c_int as libc::c_double;
            grid_tot_j
                += *((*s).d).offset(i.wrapping_mul((*s).dim).wrapping_add(j) as isize);
            i = i.wrapping_add(1);
            i;
        }
        *((*s).d)
            .offset(
                bins
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul((*s).dim)
                    .wrapping_add(j) as isize,
            ) = (newg + oldg) / 2 as libc::c_int as libc::c_double;
        grid_tot_j
            += *((*s).d)
                .offset(
                    bins
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul((*s).dim)
                        .wrapping_add(j) as isize,
                );
        tot_weight = 0 as libc::c_int as libc::c_double;
        i = 0 as libc::c_int as size_t;
        while i < bins {
            *weight.offset(i as isize) = 0 as libc::c_int as libc::c_double;
            if *((*s).d).offset(i.wrapping_mul((*s).dim).wrapping_add(j) as isize)
                > 0 as libc::c_int as libc::c_double
            {
                oldg = grid_tot_j
                    / *((*s).d)
                        .offset(i.wrapping_mul((*s).dim).wrapping_add(j) as isize);
                *weight
                    .offset(
                        i as isize,
                    ) = pow(
                    (oldg - 1 as libc::c_int as libc::c_double) / oldg / log(oldg),
                    (*s).alpha,
                );
            }
            tot_weight += *weight.offset(i as isize);
            i = i.wrapping_add(1);
            i;
        }
        let mut pts_per_bin: libc::c_double = tot_weight / bins as libc::c_double;
        let mut xold: libc::c_double = 0.;
        let mut xnew: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut dw: libc::c_double = 0 as libc::c_int as libc::c_double;
        i = 1 as libc::c_int as size_t;
        k = 0 as libc::c_int as size_t;
        while k < bins {
            dw += *weight.offset(k as isize);
            xold = xnew;
            xnew = *((*s).xi)
                .offset(
                    k
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul((*s).dim)
                        .wrapping_add(j) as isize,
                );
            while dw > pts_per_bin {
                dw -= pts_per_bin;
                *((*s).xin)
                    .offset(
                        i as isize,
                    ) = xnew - (xnew - xold) * dw / *weight.offset(k as isize);
                i = i.wrapping_add(1);
                i;
            }
            k = k.wrapping_add(1);
            k;
        }
        k = 1 as libc::c_int as size_t;
        while k < bins {
            *((*s).xi)
                .offset(
                    k.wrapping_mul((*s).dim).wrapping_add(j) as isize,
                ) = *((*s).xin).offset(k as isize);
            k = k.wrapping_add(1);
            k;
        }
        *((*s).xi)
            .offset(
                bins.wrapping_mul((*s).dim).wrapping_add(j) as isize,
            ) = 1 as libc::c_int as libc::c_double;
        j = j.wrapping_add(1);
        j;
    }
}
unsafe extern "C" fn print_lim(
    mut state: *mut gsl_monte_vegas_state,
    mut xl: *mut libc::c_double,
    mut xu: *mut libc::c_double,
    mut dim: libc::c_ulong,
) {
    let mut j: libc::c_ulong = 0;
    fprintf(
        (*state).ostream,
        b"The limits of integration are:\n\0" as *const u8 as *const libc::c_char,
    );
    j = 0 as libc::c_int as libc::c_ulong;
    while j < dim {
        fprintf(
            (*state).ostream,
            b"\nxl[%lu]=%f    xu[%lu]=%f\0" as *const u8 as *const libc::c_char,
            j,
            *xl.offset(j as isize),
            j,
            *xu.offset(j as isize),
        );
        j = j.wrapping_add(1);
        j;
    }
    fprintf((*state).ostream, b"\n\0" as *const u8 as *const libc::c_char);
    fflush((*state).ostream);
}
unsafe extern "C" fn print_head(
    mut state: *mut gsl_monte_vegas_state,
    mut num_dim: libc::c_ulong,
    mut calls: libc::c_ulong,
    mut it_num: libc::c_uint,
    mut bins: libc::c_uint,
    mut boxes: libc::c_uint,
) {
    fprintf(
        (*state).ostream,
        b"\nnum_dim=%lu, calls=%lu, it_num=%d, max_it_num=%d \0" as *const u8
            as *const libc::c_char,
        num_dim,
        calls,
        it_num,
        (*state).iterations,
    );
    fprintf(
        (*state).ostream,
        b"verb=%d, alph=%.2f,\nmode=%d, bins=%d, boxes=%d\n\0" as *const u8
            as *const libc::c_char,
        (*state).verbose,
        (*state).alpha,
        (*state).mode,
        bins,
        boxes,
    );
    fprintf(
        (*state).ostream,
        b"\n       single.......iteration                   \0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        (*state).ostream,
        b"accumulated......results   \n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        (*state).ostream,
        b"iteration     integral    sigma             integral   \0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        (*state).ostream,
        b"      sigma     chi-sq/it\n\n\0" as *const u8 as *const libc::c_char,
    );
    fflush((*state).ostream);
}
unsafe extern "C" fn print_res(
    mut state: *mut gsl_monte_vegas_state,
    mut itr: libc::c_uint,
    mut res: libc::c_double,
    mut err: libc::c_double,
    mut cum_res: libc::c_double,
    mut cum_err: libc::c_double,
    mut chi_sq: libc::c_double,
) {
    fprintf(
        (*state).ostream,
        b"%4d        %6.4e %10.2e          %6.4e      %8.2e  %10.2e\n\0" as *const u8
            as *const libc::c_char,
        itr,
        res,
        err,
        cum_res,
        cum_err,
        chi_sq,
    );
    fflush((*state).ostream);
}
unsafe extern "C" fn print_dist(
    mut state: *mut gsl_monte_vegas_state,
    mut dim: libc::c_ulong,
) {
    let mut i: libc::c_ulong = 0;
    let mut j: libc::c_ulong = 0;
    let mut p: libc::c_int = (*state).verbose;
    if p < 1 as libc::c_int {
        return;
    }
    j = 0 as libc::c_int as libc::c_ulong;
    while j < dim {
        fprintf(
            (*state).ostream,
            b"\n axis %lu \n\0" as *const u8 as *const libc::c_char,
            j,
        );
        fprintf(
            (*state).ostream,
            b"      x   g\n\0" as *const u8 as *const libc::c_char,
        );
        i = 0 as libc::c_int as libc::c_ulong;
        while i < (*state).bins as libc::c_ulong {
            fprintf(
                (*state).ostream,
                b"weight [%11.2e , %11.2e] = \0" as *const u8 as *const libc::c_char,
                *((*state).xi)
                    .offset(i.wrapping_mul((*state).dim).wrapping_add(j) as isize),
                *((*state).xi)
                    .offset(
                        i
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul((*state).dim)
                            .wrapping_add(j) as isize,
                    ),
            );
            fprintf(
                (*state).ostream,
                b" %11.2e\n\0" as *const u8 as *const libc::c_char,
                *((*state).d)
                    .offset(i.wrapping_mul((*state).dim).wrapping_add(j) as isize),
            );
            i = i.wrapping_add(1);
            i;
        }
        fprintf((*state).ostream, b"\n\0" as *const u8 as *const libc::c_char);
        j = j.wrapping_add(1);
        j;
    }
    fprintf((*state).ostream, b"\n\0" as *const u8 as *const libc::c_char);
    fflush((*state).ostream);
}
unsafe extern "C" fn print_grid(
    mut state: *mut gsl_monte_vegas_state,
    mut dim: libc::c_ulong,
) {
    let mut i: libc::c_ulong = 0;
    let mut j: libc::c_ulong = 0;
    let mut p: libc::c_int = (*state).verbose;
    if p < 1 as libc::c_int {
        return;
    }
    j = 0 as libc::c_int as libc::c_ulong;
    while j < dim {
        fprintf(
            (*state).ostream,
            b"\n axis %lu \n\0" as *const u8 as *const libc::c_char,
            j,
        );
        fprintf((*state).ostream, b"      x   \n\0" as *const u8 as *const libc::c_char);
        i = 0 as libc::c_int as libc::c_ulong;
        while i <= (*state).bins as libc::c_ulong {
            fprintf(
                (*state).ostream,
                b"%11.2e\0" as *const u8 as *const libc::c_char,
                *((*state).xi)
                    .offset(i.wrapping_mul((*state).dim).wrapping_add(j) as isize),
            );
            if i.wrapping_rem(5 as libc::c_int as libc::c_ulong)
                == 4 as libc::c_int as libc::c_ulong
            {
                fprintf((*state).ostream, b"\n\0" as *const u8 as *const libc::c_char);
            }
            i = i.wrapping_add(1);
            i;
        }
        fprintf((*state).ostream, b"\n\0" as *const u8 as *const libc::c_char);
        j = j.wrapping_add(1);
        j;
    }
    fprintf((*state).ostream, b"\n\0" as *const u8 as *const libc::c_char);
    fflush((*state).ostream);
}
