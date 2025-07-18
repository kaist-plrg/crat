use ::libc;
extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_movstat_alloc(K: size_t) -> *mut gsl_movstat_workspace;
    fn gsl_movstat_median(
        endtype: gsl_movstat_end_t,
        x: *const gsl_vector,
        y: *mut gsl_vector,
        w: *mut gsl_movstat_workspace,
    ) -> libc::c_int;
    fn gsl_movstat_mad(
        endtype: gsl_movstat_end_t,
        x: *const gsl_vector,
        xmedian: *mut gsl_vector,
        xmad: *mut gsl_vector,
        w: *mut gsl_movstat_workspace,
    ) -> libc::c_int;
    fn gsl_movstat_qqr(
        endtype: gsl_movstat_end_t,
        x: *const gsl_vector,
        q: libc::c_double,
        xqqr: *mut gsl_vector,
        w: *mut gsl_movstat_workspace,
    ) -> libc::c_int;
    fn gsl_movstat_Sn(
        endtype: gsl_movstat_end_t,
        x: *const gsl_vector,
        xscale: *mut gsl_vector,
        w: *mut gsl_movstat_workspace,
    ) -> libc::c_int;
    fn gsl_movstat_Qn(
        endtype: gsl_movstat_end_t,
        x: *const gsl_vector,
        xscale: *mut gsl_vector,
        w: *mut gsl_movstat_workspace,
    ) -> libc::c_int;
    fn gsl_movstat_free(w: *mut gsl_movstat_workspace);
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
pub struct gsl_block_int_struct {
    pub size: size_t,
    pub data: *mut libc::c_int,
}
pub type gsl_block_int = gsl_block_int_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_int {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_int,
    pub block: *mut gsl_block_int,
    pub owner: libc::c_int,
}
pub type gsl_movstat_end_t = libc::c_uint;
pub const GSL_MOVSTAT_END_TRUNCATE: gsl_movstat_end_t = 2;
pub const GSL_MOVSTAT_END_PADVALUE: gsl_movstat_end_t = 1;
pub const GSL_MOVSTAT_END_PADZERO: gsl_movstat_end_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_movstat_workspace {
    pub H: size_t,
    pub J: size_t,
    pub K: size_t,
    pub work: *mut libc::c_double,
    pub state: *mut libc::c_void,
    pub state_size: size_t,
}
pub type gsl_filter_end_t = libc::c_uint;
pub const GSL_FILTER_END_TRUNCATE: gsl_filter_end_t = 2;
pub const GSL_FILTER_END_PADVALUE: gsl_filter_end_t = 1;
pub const GSL_FILTER_END_PADZERO: gsl_filter_end_t = 0;
pub type gsl_filter_scale_t = libc::c_uint;
pub const GSL_FILTER_SCALE_QN: gsl_filter_scale_t = 3;
pub const GSL_FILTER_SCALE_SN: gsl_filter_scale_t = 2;
pub const GSL_FILTER_SCALE_IQR: gsl_filter_scale_t = 1;
pub const GSL_FILTER_SCALE_MAD: gsl_filter_scale_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_filter_impulse_workspace {
    pub movstat_workspace_p: *mut gsl_movstat_workspace,
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
unsafe extern "C" fn gsl_vector_ptr(
    mut v: *mut gsl_vector,
    i: size_t,
) -> *mut libc::c_double {
    return ((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_vector_int_set(
    mut v: *mut gsl_vector_int,
    i: size_t,
    mut x: libc::c_int,
) {
    *((*v).data).offset(i.wrapping_mul((*v).stride) as isize) = x;
}
pub unsafe extern "C" fn gsl_filter_impulse_alloc(
    K: size_t,
) -> *mut gsl_filter_impulse_workspace {
    let mut w: *mut gsl_filter_impulse_workspace = 0
        as *mut gsl_filter_impulse_workspace;
    w = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gsl_filter_impulse_workspace>() as libc::c_ulong,
    ) as *mut gsl_filter_impulse_workspace;
    if w.is_null() {
        gsl_error(
            b"failed to allocate space for workspace\0" as *const u8
                as *const libc::c_char,
            b"impulse.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_filter_impulse_workspace;
    }
    (*w).movstat_workspace_p = gsl_movstat_alloc(K);
    if ((*w).movstat_workspace_p).is_null() {
        gsl_filter_impulse_free(w);
        return 0 as *mut gsl_filter_impulse_workspace;
    }
    return w;
}
pub unsafe extern "C" fn gsl_filter_impulse_free(
    mut w: *mut gsl_filter_impulse_workspace,
) {
    if !((*w).movstat_workspace_p).is_null() {
        gsl_movstat_free((*w).movstat_workspace_p);
    }
    free(w as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_filter_impulse(
    endtype: gsl_filter_end_t,
    scale_type: gsl_filter_scale_t,
    t: libc::c_double,
    mut x: *const gsl_vector,
    mut y: *mut gsl_vector,
    mut xmedian: *mut gsl_vector,
    mut xsigma: *mut gsl_vector,
    mut noutlier: *mut size_t,
    mut ioutlier: *mut gsl_vector_int,
    mut w: *mut gsl_filter_impulse_workspace,
) -> libc::c_int {
    let n: size_t = (*x).size;
    if n != (*y).size {
        gsl_error(
            b"input and output vectors must have same length\0" as *const u8
                as *const libc::c_char,
            b"impulse.c\0" as *const u8 as *const libc::c_char,
            108 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*xmedian).size != n {
        gsl_error(
            b"xmedian vector must match input size\0" as *const u8
                as *const libc::c_char,
            b"impulse.c\0" as *const u8 as *const libc::c_char,
            112 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*xsigma).size != n {
        gsl_error(
            b"xsigma vector must match input size\0" as *const u8 as *const libc::c_char,
            b"impulse.c\0" as *const u8 as *const libc::c_char,
            116 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if !ioutlier.is_null() && (*ioutlier).size != n {
        gsl_error(
            b"ioutlier vector must match input size\0" as *const u8
                as *const libc::c_char,
            b"impulse.c\0" as *const u8 as *const libc::c_char,
            120 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if t < 0.0f64 {
        gsl_error(
            b"t must be non-negative\0" as *const u8 as *const libc::c_char,
            b"impulse.c\0" as *const u8 as *const libc::c_char,
            124 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut scale: libc::c_double = 1.0f64;
        match scale_type as libc::c_uint {
            0 => {
                gsl_movstat_mad(
                    endtype as gsl_movstat_end_t,
                    x,
                    xmedian,
                    xsigma,
                    (*w).movstat_workspace_p,
                );
            }
            1 => {
                scale = 0.741301109252801f64;
                gsl_movstat_median(
                    endtype as gsl_movstat_end_t,
                    x,
                    xmedian,
                    (*w).movstat_workspace_p,
                );
                gsl_movstat_qqr(
                    endtype as gsl_movstat_end_t,
                    x,
                    0.25f64,
                    xsigma,
                    (*w).movstat_workspace_p,
                );
            }
            2 => {
                gsl_movstat_median(
                    endtype as gsl_movstat_end_t,
                    x,
                    xmedian,
                    (*w).movstat_workspace_p,
                );
                gsl_movstat_Sn(
                    endtype as gsl_movstat_end_t,
                    x,
                    xsigma,
                    (*w).movstat_workspace_p,
                );
            }
            3 => {
                gsl_movstat_median(
                    endtype as gsl_movstat_end_t,
                    x,
                    xmedian,
                    (*w).movstat_workspace_p,
                );
                gsl_movstat_Qn(
                    endtype as gsl_movstat_end_t,
                    x,
                    xsigma,
                    (*w).movstat_workspace_p,
                );
            }
            _ => {
                gsl_error(
                    b"unknown scale type\0" as *const u8 as *const libc::c_char,
                    b"impulse.c\0" as *const u8 as *const libc::c_char,
                    178 as libc::c_int,
                    GSL_EDOM as libc::c_int,
                );
                return GSL_EDOM as libc::c_int;
            }
        }
        status = filter_impulse(
            scale,
            0.0f64,
            t,
            x,
            xmedian,
            y,
            xsigma,
            noutlier,
            ioutlier,
        );
        return status;
    };
}
unsafe extern "C" fn filter_impulse(
    scale: libc::c_double,
    epsilon: libc::c_double,
    t: libc::c_double,
    mut x: *const gsl_vector,
    mut xmedian: *const gsl_vector,
    mut y: *mut gsl_vector,
    mut xsigma: *mut gsl_vector,
    mut noutlier: *mut size_t,
    mut ioutlier: *mut gsl_vector_int,
) -> libc::c_int {
    let n: size_t = (*x).size;
    if n != (*y).size {
        gsl_error(
            b"input and output vectors must have same length\0" as *const u8
                as *const libc::c_char,
            b"impulse.c\0" as *const u8 as *const libc::c_char,
            226 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*xmedian).size != n {
        gsl_error(
            b"xmedian vector must match input size\0" as *const u8
                as *const libc::c_char,
            b"impulse.c\0" as *const u8 as *const libc::c_char,
            230 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*xsigma).size != n {
        gsl_error(
            b"xsigma vector must match input size\0" as *const u8 as *const libc::c_char,
            b"impulse.c\0" as *const u8 as *const libc::c_char,
            234 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if !ioutlier.is_null() && (*ioutlier).size != n {
        gsl_error(
            b"ioutlier vector must match input size\0" as *const u8
                as *const libc::c_char,
            b"impulse.c\0" as *const u8 as *const libc::c_char,
            238 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if t < 0.0f64 {
        gsl_error(
            b"t must be non-negative\0" as *const u8 as *const libc::c_char,
            b"impulse.c\0" as *const u8 as *const libc::c_char,
            242 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        let mut i: size_t = 0;
        *noutlier = 0 as libc::c_int as size_t;
        i = 0 as libc::c_int as size_t;
        while i < n {
            let mut xi: libc::c_double = gsl_vector_get(x, i);
            let mut xmedi: libc::c_double = gsl_vector_get(xmedian, i);
            let mut absdevi: libc::c_double = fabs(xi - xmedi);
            let mut xsigmai: *mut libc::c_double = gsl_vector_ptr(xsigma, i);
            *xsigmai *= scale;
            if *xsigmai >= epsilon && absdevi > t * *xsigmai {
                gsl_vector_set(y, i, xmedi);
                *noutlier = (*noutlier).wrapping_add(1);
                *noutlier;
                if !ioutlier.is_null() {
                    gsl_vector_int_set(ioutlier, i, 1 as libc::c_int);
                }
            } else {
                gsl_vector_set(y, i, xi);
                if !ioutlier.is_null() {
                    gsl_vector_int_set(ioutlier, i, 0 as libc::c_int);
                }
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
