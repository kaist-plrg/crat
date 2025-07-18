use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
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
pub struct gsl_histogram2d {
    pub nx: size_t,
    pub ny: size_t,
    pub xrange: *mut libc::c_double,
    pub yrange: *mut libc::c_double,
    pub bin: *mut libc::c_double,
}
pub unsafe extern "C" fn gsl_histogram2d_alloc(
    nx: size_t,
    ny: size_t,
) -> *mut gsl_histogram2d {
    let mut h: *mut gsl_histogram2d = 0 as *mut gsl_histogram2d;
    if nx == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"histogram2d length nx must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"init2d.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return 0 as *mut gsl_histogram2d;
    }
    if ny == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"histogram2d length ny must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"init2d.c\0" as *const u8 as *const libc::c_char,
            40 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return 0 as *mut gsl_histogram2d;
    }
    h = malloc(::std::mem::size_of::<gsl_histogram2d>() as libc::c_ulong)
        as *mut gsl_histogram2d;
    if h.is_null() {
        gsl_error(
            b"failed to allocate space for histogram2d struct\0" as *const u8
                as *const libc::c_char,
            b"init2d.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_histogram2d;
    }
    (*h)
        .xrange = malloc(
        nx
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*h).xrange).is_null() {
        free(h as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for histogram2d x ranges\0" as *const u8
                as *const libc::c_char,
            b"init2d.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_histogram2d;
    }
    (*h)
        .yrange = malloc(
        ny
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*h).yrange).is_null() {
        free((*h).xrange as *mut libc::c_void);
        free(h as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for histogram2d y ranges\0" as *const u8
                as *const libc::c_char,
            b"init2d.c\0" as *const u8 as *const libc::c_char,
            69 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_histogram2d;
    }
    (*h)
        .bin = malloc(
        nx
            .wrapping_mul(ny)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*h).bin).is_null() {
        free((*h).xrange as *mut libc::c_void);
        free((*h).yrange as *mut libc::c_void);
        free(h as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for histogram bins\0" as *const u8
                as *const libc::c_char,
            b"init2d.c\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_histogram2d;
    }
    (*h).nx = nx;
    (*h).ny = ny;
    return h;
}
unsafe extern "C" fn make_uniform(
    mut range: *mut libc::c_double,
    mut n: size_t,
    mut xmin: libc::c_double,
    mut xmax: libc::c_double,
) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i <= n {
        let mut f1: libc::c_double = n.wrapping_sub(i) as libc::c_double
            / n as libc::c_double;
        let mut f2: libc::c_double = i as libc::c_double / n as libc::c_double;
        *range.offset(i as isize) = f1 * xmin + f2 * xmax;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_histogram2d_calloc_uniform(
    nx: size_t,
    ny: size_t,
    xmin: libc::c_double,
    xmax: libc::c_double,
    ymin: libc::c_double,
    ymax: libc::c_double,
) -> *mut gsl_histogram2d {
    let mut h: *mut gsl_histogram2d = 0 as *mut gsl_histogram2d;
    if xmin >= xmax {
        gsl_error(
            b"xmin must be less than xmax\0" as *const u8 as *const libc::c_char,
            b"init2d.c\0" as *const u8 as *const libc::c_char,
            112 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_histogram2d;
    }
    if ymin >= ymax {
        gsl_error(
            b"ymin must be less than ymax\0" as *const u8 as *const libc::c_char,
            b"init2d.c\0" as *const u8 as *const libc::c_char,
            117 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_histogram2d;
    }
    h = gsl_histogram2d_calloc(nx, ny);
    if h.is_null() {
        return h;
    }
    make_uniform((*h).xrange, nx, xmin, xmax);
    make_uniform((*h).yrange, ny, ymin, ymax);
    return h;
}
pub unsafe extern "C" fn gsl_histogram2d_calloc(
    nx: size_t,
    ny: size_t,
) -> *mut gsl_histogram2d {
    let mut h: *mut gsl_histogram2d = 0 as *mut gsl_histogram2d;
    if nx == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"histogram2d length nx must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"init2d.c\0" as *const u8 as *const libc::c_char,
            141 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return 0 as *mut gsl_histogram2d;
    }
    if ny == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"histogram2d length ny must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"init2d.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return 0 as *mut gsl_histogram2d;
    }
    h = malloc(::std::mem::size_of::<gsl_histogram2d>() as libc::c_ulong)
        as *mut gsl_histogram2d;
    if h.is_null() {
        gsl_error(
            b"failed to allocate space for histogram2d struct\0" as *const u8
                as *const libc::c_char,
            b"init2d.c\0" as *const u8 as *const libc::c_char,
            155 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_histogram2d;
    }
    (*h)
        .xrange = malloc(
        nx
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*h).xrange).is_null() {
        free(h as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for histogram2d x ranges\0" as *const u8
                as *const libc::c_char,
            b"init2d.c\0" as *const u8 as *const libc::c_char,
            165 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_histogram2d;
    }
    (*h)
        .yrange = malloc(
        ny
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*h).yrange).is_null() {
        free((*h).xrange as *mut libc::c_void);
        free(h as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for histogram2d y ranges\0" as *const u8
                as *const libc::c_char,
            b"init2d.c\0" as *const u8 as *const libc::c_char,
            176 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_histogram2d;
    }
    (*h)
        .bin = malloc(
        nx
            .wrapping_mul(ny)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*h).bin).is_null() {
        free((*h).xrange as *mut libc::c_void);
        free((*h).yrange as *mut libc::c_void);
        free(h as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for histogram bins\0" as *const u8
                as *const libc::c_char,
            b"init2d.c\0" as *const u8 as *const libc::c_char,
            188 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_histogram2d;
    }
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < nx.wrapping_add(1 as libc::c_int as libc::c_ulong) {
        *((*h).xrange).offset(i as isize) = i as libc::c_double;
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < ny.wrapping_add(1 as libc::c_int as libc::c_ulong) {
        *((*h).yrange).offset(i as isize) = i as libc::c_double;
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < nx.wrapping_mul(ny) {
        *((*h).bin).offset(i as isize) = 0 as libc::c_int as libc::c_double;
        i = i.wrapping_add(1);
        i;
    }
    (*h).nx = nx;
    (*h).ny = ny;
    return h;
}
pub unsafe extern "C" fn gsl_histogram2d_free(mut h: *mut gsl_histogram2d) {
    if h.is_null() {
        return;
    }
    free((*h).xrange as *mut libc::c_void);
    free((*h).yrange as *mut libc::c_void);
    free((*h).bin as *mut libc::c_void);
    free(h as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_histogram2d_set_ranges_uniform(
    mut h: *mut gsl_histogram2d,
    mut xmin: libc::c_double,
    mut xmax: libc::c_double,
    mut ymin: libc::c_double,
    mut ymax: libc::c_double,
) -> libc::c_int {
    let mut i: size_t = 0;
    let nx: size_t = (*h).nx;
    let ny: size_t = (*h).ny;
    if xmin >= xmax {
        gsl_error(
            b"xmin must be less than xmax\0" as *const u8 as *const libc::c_char,
            b"init2d.c\0" as *const u8 as *const libc::c_char,
            238 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    }
    if ymin >= ymax {
        gsl_error(
            b"ymin must be less than ymax\0" as *const u8 as *const libc::c_char,
            b"init2d.c\0" as *const u8 as *const libc::c_char,
            243 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    }
    make_uniform((*h).xrange, nx, xmin, xmax);
    make_uniform((*h).yrange, ny, ymin, ymax);
    i = 0 as libc::c_int as size_t;
    while i < nx.wrapping_mul(ny) {
        *((*h).bin).offset(i as isize) = 0 as libc::c_int as libc::c_double;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_histogram2d_set_ranges(
    mut h: *mut gsl_histogram2d,
    mut xrange: *const libc::c_double,
    mut xsize: size_t,
    mut yrange: *const libc::c_double,
    mut ysize: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let nx: size_t = (*h).nx;
    let ny: size_t = (*h).ny;
    if xsize != nx.wrapping_add(1 as libc::c_int as libc::c_ulong) {
        gsl_error(
            b"size of xrange must match size of histogram\0" as *const u8
                as *const libc::c_char,
            b"init2d.c\0" as *const u8 as *const libc::c_char,
            272 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    }
    if ysize != ny.wrapping_add(1 as libc::c_int as libc::c_ulong) {
        gsl_error(
            b"size of yrange must match size of histogram\0" as *const u8
                as *const libc::c_char,
            b"init2d.c\0" as *const u8 as *const libc::c_char,
            278 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i <= nx {
        *((*h).xrange).offset(i as isize) = *xrange.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i <= ny {
        *((*h).yrange).offset(i as isize) = *yrange.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < nx.wrapping_mul(ny) {
        *((*h).bin).offset(i as isize) = 0 as libc::c_int as libc::c_double;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
