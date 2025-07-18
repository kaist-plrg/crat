use ::libc;
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_histogram2d_calloc_range(
        nx: size_t,
        ny: size_t,
        xrange: *mut libc::c_double,
        yrange: *mut libc::c_double,
    ) -> *mut gsl_histogram2d;
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
pub struct gsl_histogram2d {
    pub nx: size_t,
    pub ny: size_t,
    pub xrange: *mut libc::c_double,
    pub yrange: *mut libc::c_double,
    pub bin: *mut libc::c_double,
}
pub unsafe extern "C" fn gsl_histogram2d_memcpy(
    mut dest: *mut gsl_histogram2d,
    mut src: *const gsl_histogram2d,
) -> libc::c_int {
    let mut nx: size_t = (*src).nx;
    let mut ny: size_t = (*src).ny;
    let mut i: size_t = 0;
    if (*dest).nx != (*src).nx || (*dest).ny != (*src).ny {
        gsl_error(
            b"histograms have different sizes, cannot copy\0" as *const u8
                as *const libc::c_char,
            b"copy2d.c\0" as *const u8 as *const libc::c_char,
            46 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i <= nx {
        *((*dest).xrange).offset(i as isize) = *((*src).xrange).offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i <= ny {
        *((*dest).yrange).offset(i as isize) = *((*src).yrange).offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < nx.wrapping_mul(ny) {
        *((*dest).bin).offset(i as isize) = *((*src).bin).offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_histogram2d_clone(
    mut src: *const gsl_histogram2d,
) -> *mut gsl_histogram2d {
    let mut nx: size_t = (*src).nx;
    let mut ny: size_t = (*src).ny;
    let mut i: size_t = 0;
    let mut h: *mut gsl_histogram2d = 0 as *mut gsl_histogram2d;
    h = gsl_histogram2d_calloc_range(nx, ny, (*src).xrange, (*src).yrange);
    if h.is_null() {
        gsl_error(
            b"failed to allocate space for histogram struct\0" as *const u8
                as *const libc::c_char,
            b"copy2d.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_histogram2d;
    }
    i = 0 as libc::c_int as size_t;
    while i < nx.wrapping_mul(ny) {
        *((*h).bin).offset(i as isize) = *((*src).bin).offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    return h;
}
