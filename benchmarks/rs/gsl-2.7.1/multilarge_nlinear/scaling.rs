use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn gsl_vector_set_zero(v: *mut gsl_vector);
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
pub struct gsl_multilarge_nlinear_scale {
    pub name: *const libc::c_char,
    pub init: Option::<
        unsafe extern "C" fn(*const gsl_matrix, *mut gsl_vector) -> libc::c_int,
    >,
    pub update: Option::<
        unsafe extern "C" fn(*const gsl_matrix, *mut gsl_vector) -> libc::c_int,
    >,
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
unsafe extern "C" fn gsl_vector_ptr(
    mut v: *mut gsl_vector,
    i: size_t,
) -> *mut libc::c_double {
    return ((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_matrix_get(
    mut m: *const gsl_matrix,
    i: size_t,
    j: size_t,
) -> libc::c_double {
    return *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
unsafe extern "C" fn init_diag_levenberg(
    mut JTJ: *const gsl_matrix,
    mut diag: *mut gsl_vector,
) -> libc::c_int {
    gsl_vector_set_all(diag, 1.0f64);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn update_diag_levenberg(
    mut JTJ: *const gsl_matrix,
    mut diag: *mut gsl_vector,
) -> libc::c_int {
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn init_diag_marquardt(
    mut JTJ: *const gsl_matrix,
    mut diag: *mut gsl_vector,
) -> libc::c_int {
    return update_diag_marquardt(JTJ, diag);
}
unsafe extern "C" fn update_diag_marquardt(
    mut JTJ: *const gsl_matrix,
    mut diag: *mut gsl_vector,
) -> libc::c_int {
    let p: size_t = (*JTJ).size2;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < p {
        let mut Jjj: libc::c_double = gsl_matrix_get(JTJ, j, j);
        let mut norm: libc::c_double = 0.;
        if Jjj <= 0.0f64 {
            norm = 1.0f64;
        } else {
            norm = sqrt(Jjj);
        }
        gsl_vector_set(diag, j, norm);
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn init_diag_more(
    mut JTJ: *const gsl_matrix,
    mut diag: *mut gsl_vector,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    gsl_vector_set_zero(diag);
    status = update_diag_more(JTJ, diag);
    return status;
}
unsafe extern "C" fn update_diag_more(
    mut JTJ: *const gsl_matrix,
    mut diag: *mut gsl_vector,
) -> libc::c_int {
    let p: size_t = (*JTJ).size2;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < p {
        let mut Jjj: libc::c_double = gsl_matrix_get(JTJ, j, j);
        let mut diagj: *mut libc::c_double = gsl_vector_ptr(diag, j);
        let mut norm: libc::c_double = 0.;
        if Jjj <= 0.0f64 {
            norm = 1.0f64;
        } else {
            norm = sqrt(Jjj);
        }
        *diagj = if *diagj > norm { *diagj } else { norm };
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as libc::c_int;
}
static mut levenberg_type: gsl_multilarge_nlinear_scale = unsafe {
    {
        let mut init = gsl_multilarge_nlinear_scale {
            name: b"levenberg\0" as *const u8 as *const libc::c_char,
            init: Some(
                init_diag_levenberg
                    as unsafe extern "C" fn(
                        *const gsl_matrix,
                        *mut gsl_vector,
                    ) -> libc::c_int,
            ),
            update: Some(
                update_diag_levenberg
                    as unsafe extern "C" fn(
                        *const gsl_matrix,
                        *mut gsl_vector,
                    ) -> libc::c_int,
            ),
        };
        init
    }
};
static mut marquardt_type: gsl_multilarge_nlinear_scale = unsafe {
    {
        let mut init = gsl_multilarge_nlinear_scale {
            name: b"marquardt\0" as *const u8 as *const libc::c_char,
            init: Some(
                init_diag_marquardt
                    as unsafe extern "C" fn(
                        *const gsl_matrix,
                        *mut gsl_vector,
                    ) -> libc::c_int,
            ),
            update: Some(
                update_diag_marquardt
                    as unsafe extern "C" fn(
                        *const gsl_matrix,
                        *mut gsl_vector,
                    ) -> libc::c_int,
            ),
        };
        init
    }
};
static mut more_type: gsl_multilarge_nlinear_scale = unsafe {
    {
        let mut init = gsl_multilarge_nlinear_scale {
            name: b"more\0" as *const u8 as *const libc::c_char,
            init: Some(
                init_diag_more
                    as unsafe extern "C" fn(
                        *const gsl_matrix,
                        *mut gsl_vector,
                    ) -> libc::c_int,
            ),
            update: Some(
                update_diag_more
                    as unsafe extern "C" fn(
                        *const gsl_matrix,
                        *mut gsl_vector,
                    ) -> libc::c_int,
            ),
        };
        init
    }
};
pub static mut gsl_multilarge_nlinear_scale_levenberg: *const gsl_multilarge_nlinear_scale = unsafe {
    &levenberg_type as *const gsl_multilarge_nlinear_scale
};
pub static mut gsl_multilarge_nlinear_scale_marquardt: *const gsl_multilarge_nlinear_scale = unsafe {
    &marquardt_type as *const gsl_multilarge_nlinear_scale
};
pub static mut gsl_multilarge_nlinear_scale_more: *const gsl_multilarge_nlinear_scale = unsafe {
    &more_type as *const gsl_multilarge_nlinear_scale
};
