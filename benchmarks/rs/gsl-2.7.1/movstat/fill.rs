use ::libc;
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
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
pub type gsl_movstat_end_t = libc::c_uint;
pub const GSL_MOVSTAT_END_TRUNCATE: gsl_movstat_end_t = 2;
pub const GSL_MOVSTAT_END_PADVALUE: gsl_movstat_end_t = 1;
pub const GSL_MOVSTAT_END_PADZERO: gsl_movstat_end_t = 0;
#[inline]
unsafe extern "C" fn gsl_vector_get(
    mut v: *const gsl_vector,
    i: size_t,
) -> libc::c_double {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
pub unsafe extern "C" fn gsl_movstat_fill(
    endtype: gsl_movstat_end_t,
    mut x: *const gsl_vector,
    idx: size_t,
    H: size_t,
    J: size_t,
    mut window: *mut libc::c_double,
) -> size_t {
    if idx >= (*x).size {
        gsl_error(
            b"window center index must be between 0 and n - 1\0" as *const u8
                as *const libc::c_char,
            b"fill.c\0" as *const u8 as *const libc::c_char,
            49 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return 0 as libc::c_int as size_t;
    } else {
        let n: libc::c_int = (*x).size as libc::c_int;
        let iidx: libc::c_int = idx as libc::c_int;
        let iH: libc::c_int = H as libc::c_int;
        let iJ: libc::c_int = J as libc::c_int;
        let mut idx1: libc::c_int = 0;
        let mut idx2: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        let mut window_size: size_t = 0;
        if endtype as libc::c_uint
            == GSL_MOVSTAT_END_TRUNCATE as libc::c_int as libc::c_uint
        {
            idx1 = if iidx - iH > 0 as libc::c_int {
                iidx - iH
            } else {
                0 as libc::c_int
            };
            idx2 = if iidx + iJ < n - 1 as libc::c_int {
                iidx + iJ
            } else {
                n - 1 as libc::c_int
            };
        } else {
            idx1 = iidx - iH;
            idx2 = iidx + iJ;
        }
        window_size = (idx2 - idx1 + 1 as libc::c_int) as size_t;
        j = idx1;
        while j <= idx2 {
            let mut widx: libc::c_int = j - idx1;
            if j < 0 as libc::c_int {
                if endtype as libc::c_uint
                    == GSL_MOVSTAT_END_PADZERO as libc::c_int as libc::c_uint
                {
                    *window.offset(widx as isize) = 0.0f64;
                } else if endtype as libc::c_uint
                    == GSL_MOVSTAT_END_PADVALUE as libc::c_int as libc::c_uint
                {
                    *window
                        .offset(
                            widx as isize,
                        ) = gsl_vector_get(x, 0 as libc::c_int as size_t);
                }
            } else if j >= n {
                if endtype as libc::c_uint
                    == GSL_MOVSTAT_END_PADZERO as libc::c_int as libc::c_uint
                {
                    *window.offset(widx as isize) = 0.0f64;
                } else if endtype as libc::c_uint
                    == GSL_MOVSTAT_END_PADVALUE as libc::c_int as libc::c_uint
                {
                    *window
                        .offset(
                            widx as isize,
                        ) = gsl_vector_get(x, (n - 1 as libc::c_int) as size_t);
                }
            } else {
                *window.offset(widx as isize) = gsl_vector_get(x, j as size_t);
            }
            j += 1;
            j;
        }
        return window_size;
    };
}
