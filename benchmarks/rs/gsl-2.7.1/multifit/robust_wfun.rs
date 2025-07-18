use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_vector_set_all(v: *mut gsl_vector, x: libc::c_double);
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
pub struct gsl_multifit_robust_type {
    pub name: *const libc::c_char,
    pub wfun: Option::<
        unsafe extern "C" fn(*const gsl_vector, *mut gsl_vector) -> libc::c_int,
    >,
    pub psi_deriv: Option::<
        unsafe extern "C" fn(*const gsl_vector, *mut gsl_vector) -> libc::c_int,
    >,
    pub tuning_default: libc::c_double,
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
unsafe extern "C" fn bisquare(
    mut r: *const gsl_vector,
    mut w: *mut gsl_vector,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut n: size_t = (*r).size;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut ri: libc::c_double = gsl_vector_get(r, i);
        if fabs(ri) < 1.0f64 {
            gsl_vector_set(w, i, (1.0f64 - ri * ri) * (1.0f64 - ri * ri));
        } else {
            gsl_vector_set(w, i, 0.0f64);
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn bisquare_dpsi(
    mut r: *const gsl_vector,
    mut dpsi: *mut gsl_vector,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut n: size_t = (*r).size;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut ri: libc::c_double = gsl_vector_get(r, i);
        if fabs(ri) < 1.0f64 {
            gsl_vector_set(dpsi, i, (1.0f64 - ri * ri) * (1.0f64 - 5.0f64 * ri * ri));
        } else {
            gsl_vector_set(dpsi, i, 0.0f64);
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
static mut bisquare_type: gsl_multifit_robust_type = {
    let mut init = gsl_multifit_robust_type {
        name: b"bisquare\0" as *const u8 as *const libc::c_char,
        wfun: Some(
            bisquare
                as unsafe extern "C" fn(
                    *const gsl_vector,
                    *mut gsl_vector,
                ) -> libc::c_int,
        ),
        psi_deriv: Some(
            bisquare_dpsi
                as unsafe extern "C" fn(
                    *const gsl_vector,
                    *mut gsl_vector,
                ) -> libc::c_int,
        ),
        tuning_default: 4.685f64,
    };
    init
};
unsafe extern "C" fn cauchy(
    mut r: *const gsl_vector,
    mut w: *mut gsl_vector,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut n: size_t = (*r).size;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut ri: libc::c_double = gsl_vector_get(r, i);
        gsl_vector_set(w, i, 1.0f64 / (1.0f64 + ri * ri));
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn cauchy_dpsi(
    mut r: *const gsl_vector,
    mut dpsi: *mut gsl_vector,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut n: size_t = (*r).size;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut ri: libc::c_double = gsl_vector_get(r, i);
        let mut rsq: libc::c_double = ri * ri;
        gsl_vector_set(
            dpsi,
            i,
            (1 as libc::c_int as libc::c_double - rsq) / (1.0f64 + rsq) / (1.0f64 + rsq),
        );
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
static mut cauchy_type: gsl_multifit_robust_type = {
    let mut init = gsl_multifit_robust_type {
        name: b"cauchy\0" as *const u8 as *const libc::c_char,
        wfun: Some(
            cauchy
                as unsafe extern "C" fn(
                    *const gsl_vector,
                    *mut gsl_vector,
                ) -> libc::c_int,
        ),
        psi_deriv: Some(
            cauchy_dpsi
                as unsafe extern "C" fn(
                    *const gsl_vector,
                    *mut gsl_vector,
                ) -> libc::c_int,
        ),
        tuning_default: 2.385f64,
    };
    init
};
unsafe extern "C" fn fair(
    mut r: *const gsl_vector,
    mut w: *mut gsl_vector,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut n: size_t = (*r).size;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut ri: libc::c_double = gsl_vector_get(r, i);
        gsl_vector_set(w, i, 1.0f64 / (1.0f64 + fabs(ri)));
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn fair_dpsi(
    mut r: *const gsl_vector,
    mut dpsi: *mut gsl_vector,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut n: size_t = (*r).size;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut ri: libc::c_double = gsl_vector_get(r, i);
        gsl_vector_set(dpsi, i, 1.0f64 / (1.0f64 + fabs(ri)) / (1.0f64 + fabs(ri)));
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
static mut fair_type: gsl_multifit_robust_type = {
    let mut init = gsl_multifit_robust_type {
        name: b"fair\0" as *const u8 as *const libc::c_char,
        wfun: Some(
            fair
                as unsafe extern "C" fn(
                    *const gsl_vector,
                    *mut gsl_vector,
                ) -> libc::c_int,
        ),
        psi_deriv: Some(
            fair_dpsi
                as unsafe extern "C" fn(
                    *const gsl_vector,
                    *mut gsl_vector,
                ) -> libc::c_int,
        ),
        tuning_default: 1.4f64,
    };
    init
};
unsafe extern "C" fn huber(
    mut r: *const gsl_vector,
    mut w: *mut gsl_vector,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut n: size_t = (*r).size;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut absri: libc::c_double = fabs(gsl_vector_get(r, i));
        if absri <= 1.0f64 {
            gsl_vector_set(w, i, 1.0f64);
        } else {
            gsl_vector_set(w, i, 1.0f64 / absri);
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn huber_dpsi(
    mut r: *const gsl_vector,
    mut dpsi: *mut gsl_vector,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut n: size_t = (*r).size;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut ri: libc::c_double = gsl_vector_get(r, i);
        if fabs(ri) <= 1.0f64 {
            gsl_vector_set(dpsi, i, 1.0f64);
        } else {
            gsl_vector_set(dpsi, i, 0.0f64);
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
static mut huber_type: gsl_multifit_robust_type = {
    let mut init = gsl_multifit_robust_type {
        name: b"huber\0" as *const u8 as *const libc::c_char,
        wfun: Some(
            huber
                as unsafe extern "C" fn(
                    *const gsl_vector,
                    *mut gsl_vector,
                ) -> libc::c_int,
        ),
        psi_deriv: Some(
            huber_dpsi
                as unsafe extern "C" fn(
                    *const gsl_vector,
                    *mut gsl_vector,
                ) -> libc::c_int,
        ),
        tuning_default: 1.345f64,
    };
    init
};
unsafe extern "C" fn ols(
    mut r: *const gsl_vector,
    mut w: *mut gsl_vector,
) -> libc::c_int {
    gsl_vector_set_all(w, 1.0f64);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn ols_dpsi(
    mut r: *const gsl_vector,
    mut dpsi: *mut gsl_vector,
) -> libc::c_int {
    gsl_vector_set_all(dpsi, 1.0f64);
    return GSL_SUCCESS as libc::c_int;
}
static mut ols_type: gsl_multifit_robust_type = {
    let mut init = gsl_multifit_robust_type {
        name: b"ols\0" as *const u8 as *const libc::c_char,
        wfun: Some(
            ols
                as unsafe extern "C" fn(
                    *const gsl_vector,
                    *mut gsl_vector,
                ) -> libc::c_int,
        ),
        psi_deriv: Some(
            ols_dpsi
                as unsafe extern "C" fn(
                    *const gsl_vector,
                    *mut gsl_vector,
                ) -> libc::c_int,
        ),
        tuning_default: 1.0f64,
    };
    init
};
unsafe extern "C" fn welsch(
    mut r: *const gsl_vector,
    mut w: *mut gsl_vector,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut n: size_t = (*r).size;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut ri: libc::c_double = gsl_vector_get(r, i);
        gsl_vector_set(w, i, exp(-ri * ri));
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn welsch_dpsi(
    mut r: *const gsl_vector,
    mut dpsi: *mut gsl_vector,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut n: size_t = (*r).size;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut ri: libc::c_double = gsl_vector_get(r, i);
        gsl_vector_set(dpsi, i, (1.0f64 - 2.0f64 * ri * ri) * exp(-ri * ri));
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
static mut welsch_type: gsl_multifit_robust_type = {
    let mut init = gsl_multifit_robust_type {
        name: b"welsch\0" as *const u8 as *const libc::c_char,
        wfun: Some(
            welsch
                as unsafe extern "C" fn(
                    *const gsl_vector,
                    *mut gsl_vector,
                ) -> libc::c_int,
        ),
        psi_deriv: Some(
            welsch_dpsi
                as unsafe extern "C" fn(
                    *const gsl_vector,
                    *mut gsl_vector,
                ) -> libc::c_int,
        ),
        tuning_default: 2.985f64,
    };
    init
};
pub static mut gsl_multifit_robust_default: *const gsl_multifit_robust_type = unsafe {
    &bisquare_type as *const gsl_multifit_robust_type
};
pub static mut gsl_multifit_robust_bisquare: *const gsl_multifit_robust_type = unsafe {
    &bisquare_type as *const gsl_multifit_robust_type
};
pub static mut gsl_multifit_robust_cauchy: *const gsl_multifit_robust_type = unsafe {
    &cauchy_type as *const gsl_multifit_robust_type
};
pub static mut gsl_multifit_robust_fair: *const gsl_multifit_robust_type = unsafe {
    &fair_type as *const gsl_multifit_robust_type
};
pub static mut gsl_multifit_robust_huber: *const gsl_multifit_robust_type = unsafe {
    &huber_type as *const gsl_multifit_robust_type
};
pub static mut gsl_multifit_robust_ols: *const gsl_multifit_robust_type = unsafe {
    &ols_type as *const gsl_multifit_robust_type
};
pub static mut gsl_multifit_robust_welsch: *const gsl_multifit_robust_type = unsafe {
    &welsch_type as *const gsl_multifit_robust_type
};
