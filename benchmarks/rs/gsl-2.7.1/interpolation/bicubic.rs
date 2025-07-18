use ::libc;
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn gsl_spline_free(spline: *mut gsl_spline);
    static mut gsl_interp_cspline: *const gsl_interp_type;
    fn gsl_interp_accel_alloc() -> *mut gsl_interp_accel;
    fn gsl_interp_accel_reset(a: *mut gsl_interp_accel) -> libc::c_int;
    fn gsl_interp_accel_free(a: *mut gsl_interp_accel);
    fn gsl_spline_alloc(T: *const gsl_interp_type, size: size_t) -> *mut gsl_spline;
    fn gsl_spline_init(
        spline: *mut gsl_spline,
        xa: *const libc::c_double,
        ya: *const libc::c_double,
        size: size_t,
    ) -> libc::c_int;
    fn gsl_spline_eval_deriv(
        spline: *const gsl_spline,
        x: libc::c_double,
        a: *mut gsl_interp_accel,
    ) -> libc::c_double;
    fn gsl_vector_alloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_free(v: *mut gsl_vector);
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
pub struct gsl_interp_accel {
    pub cache: size_t,
    pub miss_count: size_t,
    pub hit_count: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_interp_type {
    pub name: *const libc::c_char,
    pub min_size: libc::c_uint,
    pub alloc: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub init: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_double,
            *const libc::c_double,
            size_t,
        ) -> libc::c_int,
    >,
    pub eval: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            *const libc::c_double,
            *const libc::c_double,
            size_t,
            libc::c_double,
            *mut gsl_interp_accel,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    pub eval_deriv: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            *const libc::c_double,
            *const libc::c_double,
            size_t,
            libc::c_double,
            *mut gsl_interp_accel,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    pub eval_deriv2: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            *const libc::c_double,
            *const libc::c_double,
            size_t,
            libc::c_double,
            *mut gsl_interp_accel,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    pub eval_integ: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            *const libc::c_double,
            *const libc::c_double,
            size_t,
            *mut gsl_interp_accel,
            libc::c_double,
            libc::c_double,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_interp {
    pub type_0: *const gsl_interp_type,
    pub xmin: libc::c_double,
    pub xmax: libc::c_double,
    pub size: size_t,
    pub state: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_spline {
    pub interp: *mut gsl_interp,
    pub x: *mut libc::c_double,
    pub y: *mut libc::c_double,
    pub size: size_t,
}
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
pub struct gsl_interp2d_type {
    pub name: *const libc::c_char,
    pub min_size: libc::c_uint,
    pub alloc: Option::<unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void>,
    pub init: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_double,
            *const libc::c_double,
            *const libc::c_double,
            size_t,
            size_t,
        ) -> libc::c_int,
    >,
    pub eval: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            *const libc::c_double,
            *const libc::c_double,
            *const libc::c_double,
            size_t,
            size_t,
            libc::c_double,
            libc::c_double,
            *mut gsl_interp_accel,
            *mut gsl_interp_accel,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    pub eval_deriv_x: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            *const libc::c_double,
            *const libc::c_double,
            *const libc::c_double,
            size_t,
            size_t,
            libc::c_double,
            libc::c_double,
            *mut gsl_interp_accel,
            *mut gsl_interp_accel,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    pub eval_deriv_y: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            *const libc::c_double,
            *const libc::c_double,
            *const libc::c_double,
            size_t,
            size_t,
            libc::c_double,
            libc::c_double,
            *mut gsl_interp_accel,
            *mut gsl_interp_accel,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    pub eval_deriv_xx: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            *const libc::c_double,
            *const libc::c_double,
            *const libc::c_double,
            size_t,
            size_t,
            libc::c_double,
            libc::c_double,
            *mut gsl_interp_accel,
            *mut gsl_interp_accel,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    pub eval_deriv_xy: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            *const libc::c_double,
            *const libc::c_double,
            *const libc::c_double,
            size_t,
            size_t,
            libc::c_double,
            libc::c_double,
            *mut gsl_interp_accel,
            *mut gsl_interp_accel,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    pub eval_deriv_yy: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            *const libc::c_double,
            *const libc::c_double,
            *const libc::c_double,
            size_t,
            size_t,
            libc::c_double,
            libc::c_double,
            *mut gsl_interp_accel,
            *mut gsl_interp_accel,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bicubic_state_t {
    pub zx: *mut libc::c_double,
    pub zy: *mut libc::c_double,
    pub zxy: *mut libc::c_double,
    pub xsize: size_t,
    pub ysize: size_t,
}
#[inline]
unsafe extern "C" fn gsl_interp_bsearch(
    mut x_array: *const libc::c_double,
    mut x: libc::c_double,
    mut index_lo: size_t,
    mut index_hi: size_t,
) -> size_t {
    let mut ilo: size_t = index_lo;
    let mut ihi: size_t = index_hi;
    while ihi > ilo.wrapping_add(1 as libc::c_int as libc::c_ulong) {
        let mut i: size_t = ihi
            .wrapping_add(ilo)
            .wrapping_div(2 as libc::c_int as libc::c_ulong);
        if *x_array.offset(i as isize) > x {
            ihi = i;
        } else {
            ilo = i;
        }
    }
    return ilo;
}
#[inline]
unsafe extern "C" fn gsl_interp_accel_find(
    mut a: *mut gsl_interp_accel,
    mut xa: *const libc::c_double,
    mut len: size_t,
    mut x: libc::c_double,
) -> size_t {
    let mut x_index: size_t = (*a).cache;
    if x < *xa.offset(x_index as isize) {
        (*a).miss_count = ((*a).miss_count).wrapping_add(1);
        (*a).miss_count;
        (*a).cache = gsl_interp_bsearch(xa, x, 0 as libc::c_int as size_t, x_index);
    } else if x
        >= *xa.offset(x_index.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
    {
        (*a).miss_count = ((*a).miss_count).wrapping_add(1);
        (*a).miss_count;
        (*a)
            .cache = gsl_interp_bsearch(
            xa,
            x,
            x_index,
            len.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    } else {
        (*a).hit_count = ((*a).hit_count).wrapping_add(1);
        (*a).hit_count;
    }
    return (*a).cache;
}
#[inline]
unsafe extern "C" fn gsl_vector_set(
    mut v: *mut gsl_vector,
    i: size_t,
    mut x: libc::c_double,
) {
    *((*v).data).offset(i.wrapping_mul((*v).stride) as isize) = x;
}
unsafe extern "C" fn bicubic_alloc(
    mut xsize: size_t,
    mut ysize: size_t,
) -> *mut libc::c_void {
    let mut state: *mut bicubic_state_t = 0 as *mut bicubic_state_t;
    state = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<bicubic_state_t>() as libc::c_ulong,
    ) as *mut bicubic_state_t;
    if state.is_null() {
        gsl_error(
            b"failed to allocate space for state\0" as *const u8 as *const libc::c_char,
            b"bicubic.c\0" as *const u8 as *const libc::c_char,
            49 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .zx = malloc(
        xsize
            .wrapping_mul(ysize)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).zx).is_null() {
        bicubic_free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for zx\0" as *const u8 as *const libc::c_char,
            b"bicubic.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .zy = malloc(
        xsize
            .wrapping_mul(ysize)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).zy).is_null() {
        bicubic_free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for zy\0" as *const u8 as *const libc::c_char,
            b"bicubic.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .zxy = malloc(
        xsize
            .wrapping_mul(ysize)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).zxy).is_null() {
        bicubic_free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for zxy\0" as *const u8 as *const libc::c_char,
            b"bicubic.c\0" as *const u8 as *const libc::c_char,
            70 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).xsize = xsize;
    (*state).ysize = ysize;
    return state as *mut libc::c_void;
}
unsafe extern "C" fn bicubic_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut bicubic_state_t = vstate as *mut bicubic_state_t;
    if state.is_null() {
        return;
    }
    if !((*state).zx).is_null() {
        free((*state).zx as *mut libc::c_void);
    }
    if !((*state).zy).is_null() {
        free((*state).zy as *mut libc::c_void);
    }
    if !((*state).zxy).is_null() {
        free((*state).zxy as *mut libc::c_void);
    }
    free(state as *mut libc::c_void);
}
unsafe extern "C" fn bicubic_init(
    mut vstate: *mut libc::c_void,
    mut xa: *const libc::c_double,
    mut ya: *const libc::c_double,
    mut za: *const libc::c_double,
    mut xsize: size_t,
    mut ysize: size_t,
) -> libc::c_int {
    let mut state: *mut bicubic_state_t = vstate as *mut bicubic_state_t;
    let mut acc: *mut gsl_interp_accel = gsl_interp_accel_alloc();
    let mut spline: *mut gsl_spline = 0 as *mut gsl_spline;
    let mut x: *mut gsl_vector = 0 as *mut gsl_vector;
    let mut y: *mut gsl_vector = 0 as *mut gsl_vector;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    x = gsl_vector_alloc(xsize);
    y = gsl_vector_alloc(xsize);
    spline = gsl_spline_alloc(gsl_interp_cspline, xsize);
    j = 0 as libc::c_int as size_t;
    while j <= ysize.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        i = 0 as libc::c_int as size_t;
        while i <= xsize.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            gsl_vector_set(x, i, *xa.offset(i as isize));
            gsl_vector_set(
                y,
                i,
                *za.offset(j.wrapping_mul((*state).xsize).wrapping_add(i) as isize),
            );
            i = i.wrapping_add(1);
            i;
        }
        gsl_spline_init(
            spline,
            (*x).data as *const libc::c_double,
            (*y).data as *const libc::c_double,
            xsize,
        );
        i = 0 as libc::c_int as size_t;
        while i <= xsize.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            *((*state).zx)
                .offset(
                    j.wrapping_mul((*state).xsize).wrapping_add(i) as isize,
                ) = gsl_spline_eval_deriv(spline, *xa.offset(i as isize), acc);
            i = i.wrapping_add(1);
            i;
        }
        j = j.wrapping_add(1);
        j;
    }
    gsl_vector_free(x);
    gsl_vector_free(y);
    gsl_spline_free(spline);
    gsl_interp_accel_reset(acc);
    x = gsl_vector_alloc(ysize);
    y = gsl_vector_alloc(ysize);
    spline = gsl_spline_alloc(gsl_interp_cspline, ysize);
    i = 0 as libc::c_int as size_t;
    while i <= xsize.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        j = 0 as libc::c_int as size_t;
        while j <= ysize.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            gsl_vector_set(x, j, *ya.offset(j as isize));
            gsl_vector_set(
                y,
                j,
                *za.offset(j.wrapping_mul((*state).xsize).wrapping_add(i) as isize),
            );
            j = j.wrapping_add(1);
            j;
        }
        gsl_spline_init(
            spline,
            (*x).data as *const libc::c_double,
            (*y).data as *const libc::c_double,
            ysize,
        );
        j = 0 as libc::c_int as size_t;
        while j <= ysize.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            *((*state).zy)
                .offset(
                    j.wrapping_mul((*state).xsize).wrapping_add(i) as isize,
                ) = gsl_spline_eval_deriv(spline, *ya.offset(j as isize), acc);
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    gsl_vector_free(x);
    gsl_vector_free(y);
    gsl_spline_free(spline);
    gsl_interp_accel_reset(acc);
    x = gsl_vector_alloc(xsize);
    y = gsl_vector_alloc(xsize);
    spline = gsl_spline_alloc(gsl_interp_cspline, xsize);
    j = 0 as libc::c_int as size_t;
    while j <= ysize.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        i = 0 as libc::c_int as size_t;
        while i <= xsize.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            gsl_vector_set(x, i, *xa.offset(i as isize));
            gsl_vector_set(
                y,
                i,
                *((*state).zy)
                    .offset(j.wrapping_mul((*state).xsize).wrapping_add(i) as isize),
            );
            i = i.wrapping_add(1);
            i;
        }
        gsl_spline_init(
            spline,
            (*x).data as *const libc::c_double,
            (*y).data as *const libc::c_double,
            xsize,
        );
        i = 0 as libc::c_int as size_t;
        while i <= xsize.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            *((*state).zxy)
                .offset(
                    j.wrapping_mul((*state).xsize).wrapping_add(i) as isize,
                ) = gsl_spline_eval_deriv(spline, *xa.offset(i as isize), acc);
            i = i.wrapping_add(1);
            i;
        }
        j = j.wrapping_add(1);
        j;
    }
    gsl_vector_free(x);
    gsl_vector_free(y);
    gsl_spline_free(spline);
    gsl_interp_accel_free(acc);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn bicubic_eval(
    mut vstate: *const libc::c_void,
    mut xarr: *const libc::c_double,
    mut yarr: *const libc::c_double,
    mut zarr: *const libc::c_double,
    mut xsize: size_t,
    mut ysize: size_t,
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut xa: *mut gsl_interp_accel,
    mut ya: *mut gsl_interp_accel,
    mut z: *mut libc::c_double,
) -> libc::c_int {
    let mut state: *mut bicubic_state_t = vstate as *mut bicubic_state_t;
    let mut xmin: libc::c_double = 0.;
    let mut xmax: libc::c_double = 0.;
    let mut ymin: libc::c_double = 0.;
    let mut ymax: libc::c_double = 0.;
    let mut zminmin: libc::c_double = 0.;
    let mut zminmax: libc::c_double = 0.;
    let mut zmaxmin: libc::c_double = 0.;
    let mut zmaxmax: libc::c_double = 0.;
    let mut zxminmin: libc::c_double = 0.;
    let mut zxminmax: libc::c_double = 0.;
    let mut zxmaxmin: libc::c_double = 0.;
    let mut zxmaxmax: libc::c_double = 0.;
    let mut zyminmin: libc::c_double = 0.;
    let mut zyminmax: libc::c_double = 0.;
    let mut zymaxmin: libc::c_double = 0.;
    let mut zymaxmax: libc::c_double = 0.;
    let mut zxyminmin: libc::c_double = 0.;
    let mut zxyminmax: libc::c_double = 0.;
    let mut zxymaxmin: libc::c_double = 0.;
    let mut zxymaxmax: libc::c_double = 0.;
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    let mut dt: libc::c_double = 0.;
    let mut du: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut u: libc::c_double = 0.;
    let mut t0: libc::c_double = 0.;
    let mut t1: libc::c_double = 0.;
    let mut t2: libc::c_double = 0.;
    let mut t3: libc::c_double = 0.;
    let mut u0: libc::c_double = 0.;
    let mut u1: libc::c_double = 0.;
    let mut u2: libc::c_double = 0.;
    let mut u3: libc::c_double = 0.;
    let mut v: libc::c_double = 0.;
    let mut xi: size_t = 0;
    let mut yi: size_t = 0;
    if !xa.is_null() {
        xi = gsl_interp_accel_find(xa, xarr, xsize, x);
    } else {
        xi = gsl_interp_bsearch(
            xarr,
            x,
            0 as libc::c_int as size_t,
            xsize.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    if !ya.is_null() {
        yi = gsl_interp_accel_find(ya, yarr, ysize, y);
    } else {
        yi = gsl_interp_bsearch(
            yarr,
            y,
            0 as libc::c_int as size_t,
            ysize.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    xmin = *xarr.offset(xi as isize);
    xmax = *xarr.offset(xi.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
    ymin = *yarr.offset(yi as isize);
    ymax = *yarr.offset(yi.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
    zminmin = *zarr.offset(yi.wrapping_mul((*state).xsize).wrapping_add(xi) as isize);
    zminmax = *zarr
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi) as isize,
        );
    zmaxmin = *zarr
        .offset(
            yi
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        );
    zmaxmax = *zarr
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        );
    dx = xmax - xmin;
    dy = ymax - ymin;
    t = (x - xmin) / dx;
    u = (y - ymin) / dy;
    dt = 1.0f64 / dx;
    du = 1.0f64 / dy;
    zxminmin = *((*state).zx)
        .offset(yi.wrapping_mul((*state).xsize).wrapping_add(xi) as isize) / dt;
    zxminmax = *((*state).zx)
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi) as isize,
        ) / dt;
    zxmaxmin = *((*state).zx)
        .offset(
            yi
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        ) / dt;
    zxmaxmax = *((*state).zx)
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        ) / dt;
    zyminmin = *((*state).zy)
        .offset(yi.wrapping_mul((*state).xsize).wrapping_add(xi) as isize) / du;
    zyminmax = *((*state).zy)
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi) as isize,
        ) / du;
    zymaxmin = *((*state).zy)
        .offset(
            yi
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        ) / du;
    zymaxmax = *((*state).zy)
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        ) / du;
    zxyminmin = *((*state).zxy)
        .offset(yi.wrapping_mul((*state).xsize).wrapping_add(xi) as isize) / (dt * du);
    zxyminmax = *((*state).zxy)
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi) as isize,
        ) / (dt * du);
    zxymaxmin = *((*state).zxy)
        .offset(
            yi
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        ) / (dt * du);
    zxymaxmax = *((*state).zxy)
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        ) / (dt * du);
    t0 = 1 as libc::c_int as libc::c_double;
    t1 = t;
    t2 = t * t;
    t3 = t * t2;
    u0 = 1 as libc::c_int as libc::c_double;
    u1 = u;
    u2 = u * u;
    u3 = u * u2;
    *z = 0 as libc::c_int as libc::c_double;
    v = zminmin;
    *z += v * t0 * u0;
    v = zyminmin;
    *z += v * t0 * u1;
    v = -(3 as libc::c_int) as libc::c_double * zminmin
        + 3 as libc::c_int as libc::c_double * zminmax
        - 2 as libc::c_int as libc::c_double * zyminmin - zyminmax;
    *z += v * t0 * u2;
    v = 2 as libc::c_int as libc::c_double * zminmin
        - 2 as libc::c_int as libc::c_double * zminmax + zyminmin + zyminmax;
    *z += v * t0 * u3;
    v = zxminmin;
    *z += v * t1 * u0;
    v = zxyminmin;
    *z += v * t1 * u1;
    v = -(3 as libc::c_int) as libc::c_double * zxminmin
        + 3 as libc::c_int as libc::c_double * zxminmax
        - 2 as libc::c_int as libc::c_double * zxyminmin - zxyminmax;
    *z += v * t1 * u2;
    v = 2 as libc::c_int as libc::c_double * zxminmin
        - 2 as libc::c_int as libc::c_double * zxminmax + zxyminmin + zxyminmax;
    *z += v * t1 * u3;
    v = -(3 as libc::c_int) as libc::c_double * zminmin
        + 3 as libc::c_int as libc::c_double * zmaxmin
        - 2 as libc::c_int as libc::c_double * zxminmin - zxmaxmin;
    *z += v * t2 * u0;
    v = -(3 as libc::c_int) as libc::c_double * zyminmin
        + 3 as libc::c_int as libc::c_double * zymaxmin
        - 2 as libc::c_int as libc::c_double * zxyminmin - zxymaxmin;
    *z += v * t2 * u1;
    v = 9 as libc::c_int as libc::c_double * zminmin
        - 9 as libc::c_int as libc::c_double * zmaxmin
        + 9 as libc::c_int as libc::c_double * zmaxmax
        - 9 as libc::c_int as libc::c_double * zminmax
        + 6 as libc::c_int as libc::c_double * zxminmin
        + 3 as libc::c_int as libc::c_double * zxmaxmin
        - 3 as libc::c_int as libc::c_double * zxmaxmax
        - 6 as libc::c_int as libc::c_double * zxminmax
        + 6 as libc::c_int as libc::c_double * zyminmin
        - 6 as libc::c_int as libc::c_double * zymaxmin
        - 3 as libc::c_int as libc::c_double * zymaxmax
        + 3 as libc::c_int as libc::c_double * zyminmax
        + 4 as libc::c_int as libc::c_double * zxyminmin
        + 2 as libc::c_int as libc::c_double * zxymaxmin + zxymaxmax
        + 2 as libc::c_int as libc::c_double * zxyminmax;
    *z += v * t2 * u2;
    v = -(6 as libc::c_int) as libc::c_double * zminmin
        + 6 as libc::c_int as libc::c_double * zmaxmin
        - 6 as libc::c_int as libc::c_double * zmaxmax
        + 6 as libc::c_int as libc::c_double * zminmax
        - 4 as libc::c_int as libc::c_double * zxminmin
        - 2 as libc::c_int as libc::c_double * zxmaxmin
        + 2 as libc::c_int as libc::c_double * zxmaxmax
        + 4 as libc::c_int as libc::c_double * zxminmax
        - 3 as libc::c_int as libc::c_double * zyminmin
        + 3 as libc::c_int as libc::c_double * zymaxmin
        + 3 as libc::c_int as libc::c_double * zymaxmax
        - 3 as libc::c_int as libc::c_double * zyminmax
        - 2 as libc::c_int as libc::c_double * zxyminmin - zxymaxmin - zxymaxmax
        - 2 as libc::c_int as libc::c_double * zxyminmax;
    *z += v * t2 * u3;
    v = 2 as libc::c_int as libc::c_double * zminmin
        - 2 as libc::c_int as libc::c_double * zmaxmin + zxminmin + zxmaxmin;
    *z += v * t3 * u0;
    v = 2 as libc::c_int as libc::c_double * zyminmin
        - 2 as libc::c_int as libc::c_double * zymaxmin + zxyminmin + zxymaxmin;
    *z += v * t3 * u1;
    v = -(6 as libc::c_int) as libc::c_double * zminmin
        + 6 as libc::c_int as libc::c_double * zmaxmin
        - 6 as libc::c_int as libc::c_double * zmaxmax
        + 6 as libc::c_int as libc::c_double * zminmax
        - 3 as libc::c_int as libc::c_double * zxminmin
        - 3 as libc::c_int as libc::c_double * zxmaxmin
        + 3 as libc::c_int as libc::c_double * zxmaxmax
        + 3 as libc::c_int as libc::c_double * zxminmax
        - 4 as libc::c_int as libc::c_double * zyminmin
        + 4 as libc::c_int as libc::c_double * zymaxmin
        + 2 as libc::c_int as libc::c_double * zymaxmax
        - 2 as libc::c_int as libc::c_double * zyminmax
        - 2 as libc::c_int as libc::c_double * zxyminmin
        - 2 as libc::c_int as libc::c_double * zxymaxmin - zxymaxmax - zxyminmax;
    *z += v * t3 * u2;
    v = 4 as libc::c_int as libc::c_double * zminmin
        - 4 as libc::c_int as libc::c_double * zmaxmin
        + 4 as libc::c_int as libc::c_double * zmaxmax
        - 4 as libc::c_int as libc::c_double * zminmax
        + 2 as libc::c_int as libc::c_double * zxminmin
        + 2 as libc::c_int as libc::c_double * zxmaxmin
        - 2 as libc::c_int as libc::c_double * zxmaxmax
        - 2 as libc::c_int as libc::c_double * zxminmax
        + 2 as libc::c_int as libc::c_double * zyminmin
        - 2 as libc::c_int as libc::c_double * zymaxmin
        - 2 as libc::c_int as libc::c_double * zymaxmax
        + 2 as libc::c_int as libc::c_double * zyminmax + zxyminmin + zxymaxmin
        + zxymaxmax + zxyminmax;
    *z += v * t3 * u3;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn bicubic_deriv_x(
    mut vstate: *const libc::c_void,
    mut xarr: *const libc::c_double,
    mut yarr: *const libc::c_double,
    mut zarr: *const libc::c_double,
    mut xsize: size_t,
    mut ysize: size_t,
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut xa: *mut gsl_interp_accel,
    mut ya: *mut gsl_interp_accel,
    mut z_p: *mut libc::c_double,
) -> libc::c_int {
    let mut state: *mut bicubic_state_t = vstate as *mut bicubic_state_t;
    let mut xmin: libc::c_double = 0.;
    let mut xmax: libc::c_double = 0.;
    let mut ymin: libc::c_double = 0.;
    let mut ymax: libc::c_double = 0.;
    let mut zminmin: libc::c_double = 0.;
    let mut zminmax: libc::c_double = 0.;
    let mut zmaxmin: libc::c_double = 0.;
    let mut zmaxmax: libc::c_double = 0.;
    let mut zxminmin: libc::c_double = 0.;
    let mut zxminmax: libc::c_double = 0.;
    let mut zxmaxmin: libc::c_double = 0.;
    let mut zxmaxmax: libc::c_double = 0.;
    let mut zyminmin: libc::c_double = 0.;
    let mut zyminmax: libc::c_double = 0.;
    let mut zymaxmin: libc::c_double = 0.;
    let mut zymaxmax: libc::c_double = 0.;
    let mut zxyminmin: libc::c_double = 0.;
    let mut zxyminmax: libc::c_double = 0.;
    let mut zxymaxmin: libc::c_double = 0.;
    let mut zxymaxmax: libc::c_double = 0.;
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    let mut dt: libc::c_double = 0.;
    let mut du: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut u: libc::c_double = 0.;
    let mut t0: libc::c_double = 0.;
    let mut t1: libc::c_double = 0.;
    let mut t2: libc::c_double = 0.;
    let mut u0: libc::c_double = 0.;
    let mut u1: libc::c_double = 0.;
    let mut u2: libc::c_double = 0.;
    let mut u3: libc::c_double = 0.;
    let mut v: libc::c_double = 0.;
    let mut xi: size_t = 0;
    let mut yi: size_t = 0;
    if !xa.is_null() {
        xi = gsl_interp_accel_find(xa, xarr, xsize, x);
    } else {
        xi = gsl_interp_bsearch(
            xarr,
            x,
            0 as libc::c_int as size_t,
            xsize.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    if !ya.is_null() {
        yi = gsl_interp_accel_find(ya, yarr, ysize, y);
    } else {
        yi = gsl_interp_bsearch(
            yarr,
            y,
            0 as libc::c_int as size_t,
            ysize.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    xmin = *xarr.offset(xi as isize);
    xmax = *xarr.offset(xi.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
    ymin = *yarr.offset(yi as isize);
    ymax = *yarr.offset(yi.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
    zminmin = *zarr.offset(yi.wrapping_mul((*state).xsize).wrapping_add(xi) as isize);
    zminmax = *zarr
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi) as isize,
        );
    zmaxmin = *zarr
        .offset(
            yi
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        );
    zmaxmax = *zarr
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        );
    dx = xmax - xmin;
    dy = ymax - ymin;
    t = (x - xmin) / dx;
    u = (y - ymin) / dy;
    dt = 1.0f64 / dx;
    du = 1.0f64 / dy;
    zxminmin = *((*state).zx)
        .offset(yi.wrapping_mul((*state).xsize).wrapping_add(xi) as isize) / dt;
    zxminmax = *((*state).zx)
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi) as isize,
        ) / dt;
    zxmaxmin = *((*state).zx)
        .offset(
            yi
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        ) / dt;
    zxmaxmax = *((*state).zx)
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        ) / dt;
    zyminmin = *((*state).zy)
        .offset(yi.wrapping_mul((*state).xsize).wrapping_add(xi) as isize) / du;
    zyminmax = *((*state).zy)
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi) as isize,
        ) / du;
    zymaxmin = *((*state).zy)
        .offset(
            yi
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        ) / du;
    zymaxmax = *((*state).zy)
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        ) / du;
    zxyminmin = *((*state).zxy)
        .offset(yi.wrapping_mul((*state).xsize).wrapping_add(xi) as isize) / (dt * du);
    zxyminmax = *((*state).zxy)
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi) as isize,
        ) / (dt * du);
    zxymaxmin = *((*state).zxy)
        .offset(
            yi
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        ) / (dt * du);
    zxymaxmax = *((*state).zxy)
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        ) / (dt * du);
    t0 = 1 as libc::c_int as libc::c_double;
    t1 = t;
    t2 = t * t;
    u0 = 1 as libc::c_int as libc::c_double;
    u1 = u;
    u2 = u * u;
    u3 = u * u2;
    *z_p = 0 as libc::c_int as libc::c_double;
    v = zxminmin;
    *z_p += v * t0 * u0;
    v = zxyminmin;
    *z_p += v * t0 * u1;
    v = -(3 as libc::c_int) as libc::c_double * zxminmin
        + 3 as libc::c_int as libc::c_double * zxminmax
        - 2 as libc::c_int as libc::c_double * zxyminmin - zxyminmax;
    *z_p += v * t0 * u2;
    v = 2 as libc::c_int as libc::c_double * zxminmin
        - 2 as libc::c_int as libc::c_double * zxminmax + zxyminmin + zxyminmax;
    *z_p += v * t0 * u3;
    v = -(3 as libc::c_int) as libc::c_double * zminmin
        + 3 as libc::c_int as libc::c_double * zmaxmin
        - 2 as libc::c_int as libc::c_double * zxminmin - zxmaxmin;
    *z_p += 2 as libc::c_int as libc::c_double * v * t1 * u0;
    v = -(3 as libc::c_int) as libc::c_double * zyminmin
        + 3 as libc::c_int as libc::c_double * zymaxmin
        - 2 as libc::c_int as libc::c_double * zxyminmin - zxymaxmin;
    *z_p += 2 as libc::c_int as libc::c_double * v * t1 * u1;
    v = 9 as libc::c_int as libc::c_double * zminmin
        - 9 as libc::c_int as libc::c_double * zmaxmin
        + 9 as libc::c_int as libc::c_double * zmaxmax
        - 9 as libc::c_int as libc::c_double * zminmax
        + 6 as libc::c_int as libc::c_double * zxminmin
        + 3 as libc::c_int as libc::c_double * zxmaxmin
        - 3 as libc::c_int as libc::c_double * zxmaxmax
        - 6 as libc::c_int as libc::c_double * zxminmax
        + 6 as libc::c_int as libc::c_double * zyminmin
        - 6 as libc::c_int as libc::c_double * zymaxmin
        - 3 as libc::c_int as libc::c_double * zymaxmax
        + 3 as libc::c_int as libc::c_double * zyminmax
        + 4 as libc::c_int as libc::c_double * zxyminmin
        + 2 as libc::c_int as libc::c_double * zxymaxmin + zxymaxmax
        + 2 as libc::c_int as libc::c_double * zxyminmax;
    *z_p += 2 as libc::c_int as libc::c_double * v * t1 * u2;
    v = -(6 as libc::c_int) as libc::c_double * zminmin
        + 6 as libc::c_int as libc::c_double * zmaxmin
        - 6 as libc::c_int as libc::c_double * zmaxmax
        + 6 as libc::c_int as libc::c_double * zminmax
        - 4 as libc::c_int as libc::c_double * zxminmin
        - 2 as libc::c_int as libc::c_double * zxmaxmin
        + 2 as libc::c_int as libc::c_double * zxmaxmax
        + 4 as libc::c_int as libc::c_double * zxminmax
        - 3 as libc::c_int as libc::c_double * zyminmin
        + 3 as libc::c_int as libc::c_double * zymaxmin
        + 3 as libc::c_int as libc::c_double * zymaxmax
        - 3 as libc::c_int as libc::c_double * zyminmax
        - 2 as libc::c_int as libc::c_double * zxyminmin - zxymaxmin - zxymaxmax
        - 2 as libc::c_int as libc::c_double * zxyminmax;
    *z_p += 2 as libc::c_int as libc::c_double * v * t1 * u3;
    v = 2 as libc::c_int as libc::c_double * zminmin
        - 2 as libc::c_int as libc::c_double * zmaxmin + zxminmin + zxmaxmin;
    *z_p += 3 as libc::c_int as libc::c_double * v * t2 * u0;
    v = 2 as libc::c_int as libc::c_double * zyminmin
        - 2 as libc::c_int as libc::c_double * zymaxmin + zxyminmin + zxymaxmin;
    *z_p += 3 as libc::c_int as libc::c_double * v * t2 * u1;
    v = -(6 as libc::c_int) as libc::c_double * zminmin
        + 6 as libc::c_int as libc::c_double * zmaxmin
        - 6 as libc::c_int as libc::c_double * zmaxmax
        + 6 as libc::c_int as libc::c_double * zminmax
        - 3 as libc::c_int as libc::c_double * zxminmin
        - 3 as libc::c_int as libc::c_double * zxmaxmin
        + 3 as libc::c_int as libc::c_double * zxmaxmax
        + 3 as libc::c_int as libc::c_double * zxminmax
        - 4 as libc::c_int as libc::c_double * zyminmin
        + 4 as libc::c_int as libc::c_double * zymaxmin
        + 2 as libc::c_int as libc::c_double * zymaxmax
        - 2 as libc::c_int as libc::c_double * zyminmax
        - 2 as libc::c_int as libc::c_double * zxyminmin
        - 2 as libc::c_int as libc::c_double * zxymaxmin - zxymaxmax - zxyminmax;
    *z_p += 3 as libc::c_int as libc::c_double * v * t2 * u2;
    v = 4 as libc::c_int as libc::c_double * zminmin
        - 4 as libc::c_int as libc::c_double * zmaxmin
        + 4 as libc::c_int as libc::c_double * zmaxmax
        - 4 as libc::c_int as libc::c_double * zminmax
        + 2 as libc::c_int as libc::c_double * zxminmin
        + 2 as libc::c_int as libc::c_double * zxmaxmin
        - 2 as libc::c_int as libc::c_double * zxmaxmax
        - 2 as libc::c_int as libc::c_double * zxminmax
        + 2 as libc::c_int as libc::c_double * zyminmin
        - 2 as libc::c_int as libc::c_double * zymaxmin
        - 2 as libc::c_int as libc::c_double * zymaxmax
        + 2 as libc::c_int as libc::c_double * zyminmax + zxyminmin + zxymaxmin
        + zxymaxmax + zxyminmax;
    *z_p += 3 as libc::c_int as libc::c_double * v * t2 * u3;
    *z_p *= dt;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn bicubic_deriv_y(
    mut vstate: *const libc::c_void,
    mut xarr: *const libc::c_double,
    mut yarr: *const libc::c_double,
    mut zarr: *const libc::c_double,
    mut xsize: size_t,
    mut ysize: size_t,
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut xa: *mut gsl_interp_accel,
    mut ya: *mut gsl_interp_accel,
    mut z_p: *mut libc::c_double,
) -> libc::c_int {
    let mut state: *mut bicubic_state_t = vstate as *mut bicubic_state_t;
    let mut xmin: libc::c_double = 0.;
    let mut xmax: libc::c_double = 0.;
    let mut ymin: libc::c_double = 0.;
    let mut ymax: libc::c_double = 0.;
    let mut zminmin: libc::c_double = 0.;
    let mut zminmax: libc::c_double = 0.;
    let mut zmaxmin: libc::c_double = 0.;
    let mut zmaxmax: libc::c_double = 0.;
    let mut zxminmin: libc::c_double = 0.;
    let mut zxminmax: libc::c_double = 0.;
    let mut zxmaxmin: libc::c_double = 0.;
    let mut zxmaxmax: libc::c_double = 0.;
    let mut zyminmin: libc::c_double = 0.;
    let mut zyminmax: libc::c_double = 0.;
    let mut zymaxmin: libc::c_double = 0.;
    let mut zymaxmax: libc::c_double = 0.;
    let mut zxyminmin: libc::c_double = 0.;
    let mut zxyminmax: libc::c_double = 0.;
    let mut zxymaxmin: libc::c_double = 0.;
    let mut zxymaxmax: libc::c_double = 0.;
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    let mut dt: libc::c_double = 0.;
    let mut du: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut u: libc::c_double = 0.;
    let mut t0: libc::c_double = 0.;
    let mut t1: libc::c_double = 0.;
    let mut t2: libc::c_double = 0.;
    let mut t3: libc::c_double = 0.;
    let mut u0: libc::c_double = 0.;
    let mut u1: libc::c_double = 0.;
    let mut u2: libc::c_double = 0.;
    let mut v: libc::c_double = 0.;
    let mut xi: size_t = 0;
    let mut yi: size_t = 0;
    if !xa.is_null() {
        xi = gsl_interp_accel_find(xa, xarr, xsize, x);
    } else {
        xi = gsl_interp_bsearch(
            xarr,
            x,
            0 as libc::c_int as size_t,
            xsize.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    if !ya.is_null() {
        yi = gsl_interp_accel_find(ya, yarr, ysize, y);
    } else {
        yi = gsl_interp_bsearch(
            yarr,
            y,
            0 as libc::c_int as size_t,
            ysize.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    xmin = *xarr.offset(xi as isize);
    xmax = *xarr.offset(xi.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
    ymin = *yarr.offset(yi as isize);
    ymax = *yarr.offset(yi.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
    zminmin = *zarr.offset(yi.wrapping_mul((*state).xsize).wrapping_add(xi) as isize);
    zminmax = *zarr
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi) as isize,
        );
    zmaxmin = *zarr
        .offset(
            yi
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        );
    zmaxmax = *zarr
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        );
    dx = xmax - xmin;
    dy = ymax - ymin;
    t = (x - xmin) / dx;
    u = (y - ymin) / dy;
    dt = 1.0f64 / dx;
    du = 1.0f64 / dy;
    zxminmin = *((*state).zx)
        .offset(yi.wrapping_mul((*state).xsize).wrapping_add(xi) as isize) / dt;
    zxminmax = *((*state).zx)
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi) as isize,
        ) / dt;
    zxmaxmin = *((*state).zx)
        .offset(
            yi
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        ) / dt;
    zxmaxmax = *((*state).zx)
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        ) / dt;
    zyminmin = *((*state).zy)
        .offset(yi.wrapping_mul((*state).xsize).wrapping_add(xi) as isize) / du;
    zyminmax = *((*state).zy)
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi) as isize,
        ) / du;
    zymaxmin = *((*state).zy)
        .offset(
            yi
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        ) / du;
    zymaxmax = *((*state).zy)
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        ) / du;
    zxyminmin = *((*state).zxy)
        .offset(yi.wrapping_mul((*state).xsize).wrapping_add(xi) as isize) / (dt * du);
    zxyminmax = *((*state).zxy)
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi) as isize,
        ) / (dt * du);
    zxymaxmin = *((*state).zxy)
        .offset(
            yi
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        ) / (dt * du);
    zxymaxmax = *((*state).zxy)
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        ) / (dt * du);
    t0 = 1 as libc::c_int as libc::c_double;
    t1 = t;
    t2 = t * t;
    t3 = t * t2;
    u0 = 1 as libc::c_int as libc::c_double;
    u1 = u;
    u2 = u * u;
    *z_p = 0 as libc::c_int as libc::c_double;
    v = zyminmin;
    *z_p += v * t0 * u0;
    v = -(3 as libc::c_int) as libc::c_double * zminmin
        + 3 as libc::c_int as libc::c_double * zminmax
        - 2 as libc::c_int as libc::c_double * zyminmin - zyminmax;
    *z_p += 2 as libc::c_int as libc::c_double * v * t0 * u1;
    v = 2 as libc::c_int as libc::c_double * zminmin
        - 2 as libc::c_int as libc::c_double * zminmax + zyminmin + zyminmax;
    *z_p += 3 as libc::c_int as libc::c_double * v * t0 * u2;
    v = zxyminmin;
    *z_p += v * t1 * u0;
    v = -(3 as libc::c_int) as libc::c_double * zxminmin
        + 3 as libc::c_int as libc::c_double * zxminmax
        - 2 as libc::c_int as libc::c_double * zxyminmin - zxyminmax;
    *z_p += 2 as libc::c_int as libc::c_double * v * t1 * u1;
    v = 2 as libc::c_int as libc::c_double * zxminmin
        - 2 as libc::c_int as libc::c_double * zxminmax + zxyminmin + zxyminmax;
    *z_p += 3 as libc::c_int as libc::c_double * v * t1 * u2;
    v = -(3 as libc::c_int) as libc::c_double * zyminmin
        + 3 as libc::c_int as libc::c_double * zymaxmin
        - 2 as libc::c_int as libc::c_double * zxyminmin - zxymaxmin;
    *z_p += v * t2 * u0;
    v = 9 as libc::c_int as libc::c_double * zminmin
        - 9 as libc::c_int as libc::c_double * zmaxmin
        + 9 as libc::c_int as libc::c_double * zmaxmax
        - 9 as libc::c_int as libc::c_double * zminmax
        + 6 as libc::c_int as libc::c_double * zxminmin
        + 3 as libc::c_int as libc::c_double * zxmaxmin
        - 3 as libc::c_int as libc::c_double * zxmaxmax
        - 6 as libc::c_int as libc::c_double * zxminmax
        + 6 as libc::c_int as libc::c_double * zyminmin
        - 6 as libc::c_int as libc::c_double * zymaxmin
        - 3 as libc::c_int as libc::c_double * zymaxmax
        + 3 as libc::c_int as libc::c_double * zyminmax
        + 4 as libc::c_int as libc::c_double * zxyminmin
        + 2 as libc::c_int as libc::c_double * zxymaxmin + zxymaxmax
        + 2 as libc::c_int as libc::c_double * zxyminmax;
    *z_p += 2 as libc::c_int as libc::c_double * v * t2 * u1;
    v = -(6 as libc::c_int) as libc::c_double * zminmin
        + 6 as libc::c_int as libc::c_double * zmaxmin
        - 6 as libc::c_int as libc::c_double * zmaxmax
        + 6 as libc::c_int as libc::c_double * zminmax
        - 4 as libc::c_int as libc::c_double * zxminmin
        - 2 as libc::c_int as libc::c_double * zxmaxmin
        + 2 as libc::c_int as libc::c_double * zxmaxmax
        + 4 as libc::c_int as libc::c_double * zxminmax
        - 3 as libc::c_int as libc::c_double * zyminmin
        + 3 as libc::c_int as libc::c_double * zymaxmin
        + 3 as libc::c_int as libc::c_double * zymaxmax
        - 3 as libc::c_int as libc::c_double * zyminmax
        - 2 as libc::c_int as libc::c_double * zxyminmin - zxymaxmin - zxymaxmax
        - 2 as libc::c_int as libc::c_double * zxyminmax;
    *z_p += 3 as libc::c_int as libc::c_double * v * t2 * u2;
    v = 2 as libc::c_int as libc::c_double * zyminmin
        - 2 as libc::c_int as libc::c_double * zymaxmin + zxyminmin + zxymaxmin;
    *z_p += v * t3 * u0;
    v = -(6 as libc::c_int) as libc::c_double * zminmin
        + 6 as libc::c_int as libc::c_double * zmaxmin
        - 6 as libc::c_int as libc::c_double * zmaxmax
        + 6 as libc::c_int as libc::c_double * zminmax
        - 3 as libc::c_int as libc::c_double * zxminmin
        - 3 as libc::c_int as libc::c_double * zxmaxmin
        + 3 as libc::c_int as libc::c_double * zxmaxmax
        + 3 as libc::c_int as libc::c_double * zxminmax
        - 4 as libc::c_int as libc::c_double * zyminmin
        + 4 as libc::c_int as libc::c_double * zymaxmin
        + 2 as libc::c_int as libc::c_double * zymaxmax
        - 2 as libc::c_int as libc::c_double * zyminmax
        - 2 as libc::c_int as libc::c_double * zxyminmin
        - 2 as libc::c_int as libc::c_double * zxymaxmin - zxymaxmax - zxyminmax;
    *z_p += 2 as libc::c_int as libc::c_double * v * t3 * u1;
    v = 4 as libc::c_int as libc::c_double * zminmin
        - 4 as libc::c_int as libc::c_double * zmaxmin
        + 4 as libc::c_int as libc::c_double * zmaxmax
        - 4 as libc::c_int as libc::c_double * zminmax
        + 2 as libc::c_int as libc::c_double * zxminmin
        + 2 as libc::c_int as libc::c_double * zxmaxmin
        - 2 as libc::c_int as libc::c_double * zxmaxmax
        - 2 as libc::c_int as libc::c_double * zxminmax
        + 2 as libc::c_int as libc::c_double * zyminmin
        - 2 as libc::c_int as libc::c_double * zymaxmin
        - 2 as libc::c_int as libc::c_double * zymaxmax
        + 2 as libc::c_int as libc::c_double * zyminmax + zxyminmin + zxymaxmin
        + zxymaxmax + zxyminmax;
    *z_p += 3 as libc::c_int as libc::c_double * v * t3 * u2;
    *z_p *= du;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn bicubic_deriv_xx(
    mut vstate: *const libc::c_void,
    mut xarr: *const libc::c_double,
    mut yarr: *const libc::c_double,
    mut zarr: *const libc::c_double,
    mut xsize: size_t,
    mut ysize: size_t,
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut xa: *mut gsl_interp_accel,
    mut ya: *mut gsl_interp_accel,
    mut z_pp: *mut libc::c_double,
) -> libc::c_int {
    let mut state: *mut bicubic_state_t = vstate as *mut bicubic_state_t;
    let mut xmin: libc::c_double = 0.;
    let mut xmax: libc::c_double = 0.;
    let mut ymin: libc::c_double = 0.;
    let mut ymax: libc::c_double = 0.;
    let mut zminmin: libc::c_double = 0.;
    let mut zminmax: libc::c_double = 0.;
    let mut zmaxmin: libc::c_double = 0.;
    let mut zmaxmax: libc::c_double = 0.;
    let mut zxminmin: libc::c_double = 0.;
    let mut zxminmax: libc::c_double = 0.;
    let mut zxmaxmin: libc::c_double = 0.;
    let mut zxmaxmax: libc::c_double = 0.;
    let mut zyminmin: libc::c_double = 0.;
    let mut zyminmax: libc::c_double = 0.;
    let mut zymaxmin: libc::c_double = 0.;
    let mut zymaxmax: libc::c_double = 0.;
    let mut zxyminmin: libc::c_double = 0.;
    let mut zxyminmax: libc::c_double = 0.;
    let mut zxymaxmin: libc::c_double = 0.;
    let mut zxymaxmax: libc::c_double = 0.;
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    let mut dt: libc::c_double = 0.;
    let mut du: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut u: libc::c_double = 0.;
    let mut t0: libc::c_double = 0.;
    let mut t1: libc::c_double = 0.;
    let mut u0: libc::c_double = 0.;
    let mut u1: libc::c_double = 0.;
    let mut u2: libc::c_double = 0.;
    let mut u3: libc::c_double = 0.;
    let mut v: libc::c_double = 0.;
    let mut xi: size_t = 0;
    let mut yi: size_t = 0;
    if !xa.is_null() {
        xi = gsl_interp_accel_find(xa, xarr, xsize, x);
    } else {
        xi = gsl_interp_bsearch(
            xarr,
            x,
            0 as libc::c_int as size_t,
            xsize.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    if !ya.is_null() {
        yi = gsl_interp_accel_find(ya, yarr, ysize, y);
    } else {
        yi = gsl_interp_bsearch(
            yarr,
            y,
            0 as libc::c_int as size_t,
            ysize.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    xmin = *xarr.offset(xi as isize);
    xmax = *xarr.offset(xi.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
    ymin = *yarr.offset(yi as isize);
    ymax = *yarr.offset(yi.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
    zminmin = *zarr.offset(yi.wrapping_mul((*state).xsize).wrapping_add(xi) as isize);
    zminmax = *zarr
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi) as isize,
        );
    zmaxmin = *zarr
        .offset(
            yi
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        );
    zmaxmax = *zarr
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        );
    dx = xmax - xmin;
    dy = ymax - ymin;
    t = (x - xmin) / dx;
    u = (y - ymin) / dy;
    dt = 1.0f64 / dx;
    du = 1.0f64 / dy;
    zxminmin = *((*state).zx)
        .offset(yi.wrapping_mul((*state).xsize).wrapping_add(xi) as isize) / dt;
    zxminmax = *((*state).zx)
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi) as isize,
        ) / dt;
    zxmaxmin = *((*state).zx)
        .offset(
            yi
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        ) / dt;
    zxmaxmax = *((*state).zx)
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        ) / dt;
    zyminmin = *((*state).zy)
        .offset(yi.wrapping_mul((*state).xsize).wrapping_add(xi) as isize) / du;
    zyminmax = *((*state).zy)
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi) as isize,
        ) / du;
    zymaxmin = *((*state).zy)
        .offset(
            yi
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        ) / du;
    zymaxmax = *((*state).zy)
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        ) / du;
    zxyminmin = *((*state).zxy)
        .offset(yi.wrapping_mul((*state).xsize).wrapping_add(xi) as isize) / (dt * du);
    zxyminmax = *((*state).zxy)
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi) as isize,
        ) / (dt * du);
    zxymaxmin = *((*state).zxy)
        .offset(
            yi
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        ) / (dt * du);
    zxymaxmax = *((*state).zxy)
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        ) / (dt * du);
    t0 = 1 as libc::c_int as libc::c_double;
    t1 = t;
    u0 = 1 as libc::c_int as libc::c_double;
    u1 = u;
    u2 = u * u;
    u3 = u * u2;
    *z_pp = 0 as libc::c_int as libc::c_double;
    v = -(3 as libc::c_int) as libc::c_double * zminmin
        + 3 as libc::c_int as libc::c_double * zmaxmin
        - 2 as libc::c_int as libc::c_double * zxminmin - zxmaxmin;
    *z_pp += 2 as libc::c_int as libc::c_double * v * t0 * u0;
    v = -(3 as libc::c_int) as libc::c_double * zyminmin
        + 3 as libc::c_int as libc::c_double * zymaxmin
        - 2 as libc::c_int as libc::c_double * zxyminmin - zxymaxmin;
    *z_pp += 2 as libc::c_int as libc::c_double * v * t0 * u1;
    v = 9 as libc::c_int as libc::c_double * zminmin
        - 9 as libc::c_int as libc::c_double * zmaxmin
        + 9 as libc::c_int as libc::c_double * zmaxmax
        - 9 as libc::c_int as libc::c_double * zminmax
        + 6 as libc::c_int as libc::c_double * zxminmin
        + 3 as libc::c_int as libc::c_double * zxmaxmin
        - 3 as libc::c_int as libc::c_double * zxmaxmax
        - 6 as libc::c_int as libc::c_double * zxminmax
        + 6 as libc::c_int as libc::c_double * zyminmin
        - 6 as libc::c_int as libc::c_double * zymaxmin
        - 3 as libc::c_int as libc::c_double * zymaxmax
        + 3 as libc::c_int as libc::c_double * zyminmax
        + 4 as libc::c_int as libc::c_double * zxyminmin
        + 2 as libc::c_int as libc::c_double * zxymaxmin + zxymaxmax
        + 2 as libc::c_int as libc::c_double * zxyminmax;
    *z_pp += 2 as libc::c_int as libc::c_double * v * t0 * u2;
    v = -(6 as libc::c_int) as libc::c_double * zminmin
        + 6 as libc::c_int as libc::c_double * zmaxmin
        - 6 as libc::c_int as libc::c_double * zmaxmax
        + 6 as libc::c_int as libc::c_double * zminmax
        - 4 as libc::c_int as libc::c_double * zxminmin
        - 2 as libc::c_int as libc::c_double * zxmaxmin
        + 2 as libc::c_int as libc::c_double * zxmaxmax
        + 4 as libc::c_int as libc::c_double * zxminmax
        - 3 as libc::c_int as libc::c_double * zyminmin
        + 3 as libc::c_int as libc::c_double * zymaxmin
        + 3 as libc::c_int as libc::c_double * zymaxmax
        - 3 as libc::c_int as libc::c_double * zyminmax
        - 2 as libc::c_int as libc::c_double * zxyminmin - zxymaxmin - zxymaxmax
        - 2 as libc::c_int as libc::c_double * zxyminmax;
    *z_pp += 2 as libc::c_int as libc::c_double * v * t0 * u3;
    v = 2 as libc::c_int as libc::c_double * zminmin
        - 2 as libc::c_int as libc::c_double * zmaxmin + zxminmin + zxmaxmin;
    *z_pp += 6 as libc::c_int as libc::c_double * v * t1 * u0;
    v = 2 as libc::c_int as libc::c_double * zyminmin
        - 2 as libc::c_int as libc::c_double * zymaxmin + zxyminmin + zxymaxmin;
    *z_pp += 6 as libc::c_int as libc::c_double * v * t1 * u1;
    v = -(6 as libc::c_int) as libc::c_double * zminmin
        + 6 as libc::c_int as libc::c_double * zmaxmin
        - 6 as libc::c_int as libc::c_double * zmaxmax
        + 6 as libc::c_int as libc::c_double * zminmax
        - 3 as libc::c_int as libc::c_double * zxminmin
        - 3 as libc::c_int as libc::c_double * zxmaxmin
        + 3 as libc::c_int as libc::c_double * zxmaxmax
        + 3 as libc::c_int as libc::c_double * zxminmax
        - 4 as libc::c_int as libc::c_double * zyminmin
        + 4 as libc::c_int as libc::c_double * zymaxmin
        + 2 as libc::c_int as libc::c_double * zymaxmax
        - 2 as libc::c_int as libc::c_double * zyminmax
        - 2 as libc::c_int as libc::c_double * zxyminmin
        - 2 as libc::c_int as libc::c_double * zxymaxmin - zxymaxmax - zxyminmax;
    *z_pp += 6 as libc::c_int as libc::c_double * v * t1 * u2;
    v = 4 as libc::c_int as libc::c_double * zminmin
        - 4 as libc::c_int as libc::c_double * zmaxmin
        + 4 as libc::c_int as libc::c_double * zmaxmax
        - 4 as libc::c_int as libc::c_double * zminmax
        + 2 as libc::c_int as libc::c_double * zxminmin
        + 2 as libc::c_int as libc::c_double * zxmaxmin
        - 2 as libc::c_int as libc::c_double * zxmaxmax
        - 2 as libc::c_int as libc::c_double * zxminmax
        + 2 as libc::c_int as libc::c_double * zyminmin
        - 2 as libc::c_int as libc::c_double * zymaxmin
        - 2 as libc::c_int as libc::c_double * zymaxmax
        + 2 as libc::c_int as libc::c_double * zyminmax + zxyminmin + zxymaxmin
        + zxymaxmax + zxyminmax;
    *z_pp += 6 as libc::c_int as libc::c_double * v * t1 * u3;
    *z_pp *= dt * dt;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn bicubic_deriv_xy(
    mut vstate: *const libc::c_void,
    mut xarr: *const libc::c_double,
    mut yarr: *const libc::c_double,
    mut zarr: *const libc::c_double,
    mut xsize: size_t,
    mut ysize: size_t,
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut xa: *mut gsl_interp_accel,
    mut ya: *mut gsl_interp_accel,
    mut z_pp: *mut libc::c_double,
) -> libc::c_int {
    let mut state: *mut bicubic_state_t = vstate as *mut bicubic_state_t;
    let mut xmin: libc::c_double = 0.;
    let mut xmax: libc::c_double = 0.;
    let mut ymin: libc::c_double = 0.;
    let mut ymax: libc::c_double = 0.;
    let mut zminmin: libc::c_double = 0.;
    let mut zminmax: libc::c_double = 0.;
    let mut zmaxmin: libc::c_double = 0.;
    let mut zmaxmax: libc::c_double = 0.;
    let mut zxminmin: libc::c_double = 0.;
    let mut zxminmax: libc::c_double = 0.;
    let mut zxmaxmin: libc::c_double = 0.;
    let mut zxmaxmax: libc::c_double = 0.;
    let mut zyminmin: libc::c_double = 0.;
    let mut zyminmax: libc::c_double = 0.;
    let mut zymaxmin: libc::c_double = 0.;
    let mut zymaxmax: libc::c_double = 0.;
    let mut zxyminmin: libc::c_double = 0.;
    let mut zxyminmax: libc::c_double = 0.;
    let mut zxymaxmin: libc::c_double = 0.;
    let mut zxymaxmax: libc::c_double = 0.;
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    let mut dt: libc::c_double = 0.;
    let mut du: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut u: libc::c_double = 0.;
    let mut t0: libc::c_double = 0.;
    let mut t1: libc::c_double = 0.;
    let mut t2: libc::c_double = 0.;
    let mut u0: libc::c_double = 0.;
    let mut u1: libc::c_double = 0.;
    let mut u2: libc::c_double = 0.;
    let mut v: libc::c_double = 0.;
    let mut xi: size_t = 0;
    let mut yi: size_t = 0;
    if !xa.is_null() {
        xi = gsl_interp_accel_find(xa, xarr, xsize, x);
    } else {
        xi = gsl_interp_bsearch(
            xarr,
            x,
            0 as libc::c_int as size_t,
            xsize.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    if !ya.is_null() {
        yi = gsl_interp_accel_find(ya, yarr, ysize, y);
    } else {
        yi = gsl_interp_bsearch(
            yarr,
            y,
            0 as libc::c_int as size_t,
            ysize.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    xmin = *xarr.offset(xi as isize);
    xmax = *xarr.offset(xi.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
    ymin = *yarr.offset(yi as isize);
    ymax = *yarr.offset(yi.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
    zminmin = *zarr.offset(yi.wrapping_mul((*state).xsize).wrapping_add(xi) as isize);
    zminmax = *zarr
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi) as isize,
        );
    zmaxmin = *zarr
        .offset(
            yi
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        );
    zmaxmax = *zarr
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        );
    dx = xmax - xmin;
    dy = ymax - ymin;
    t = (x - xmin) / dx;
    u = (y - ymin) / dy;
    dt = 1.0f64 / dx;
    du = 1.0f64 / dy;
    zxminmin = *((*state).zx)
        .offset(yi.wrapping_mul((*state).xsize).wrapping_add(xi) as isize) / dt;
    zxminmax = *((*state).zx)
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi) as isize,
        ) / dt;
    zxmaxmin = *((*state).zx)
        .offset(
            yi
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        ) / dt;
    zxmaxmax = *((*state).zx)
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        ) / dt;
    zyminmin = *((*state).zy)
        .offset(yi.wrapping_mul((*state).xsize).wrapping_add(xi) as isize) / du;
    zyminmax = *((*state).zy)
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi) as isize,
        ) / du;
    zymaxmin = *((*state).zy)
        .offset(
            yi
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        ) / du;
    zymaxmax = *((*state).zy)
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        ) / du;
    zxyminmin = *((*state).zxy)
        .offset(yi.wrapping_mul((*state).xsize).wrapping_add(xi) as isize) / (dt * du);
    zxyminmax = *((*state).zxy)
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi) as isize,
        ) / (dt * du);
    zxymaxmin = *((*state).zxy)
        .offset(
            yi
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        ) / (dt * du);
    zxymaxmax = *((*state).zxy)
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        ) / (dt * du);
    t0 = 1 as libc::c_int as libc::c_double;
    t1 = t;
    t2 = t * t;
    u0 = 1 as libc::c_int as libc::c_double;
    u1 = u;
    u2 = u * u;
    *z_pp = 0 as libc::c_int as libc::c_double;
    v = zxyminmin;
    *z_pp += v * t0 * u0;
    v = -(3 as libc::c_int) as libc::c_double * zxminmin
        + 3 as libc::c_int as libc::c_double * zxminmax
        - 2 as libc::c_int as libc::c_double * zxyminmin - zxyminmax;
    *z_pp += 2 as libc::c_int as libc::c_double * v * t0 * u1;
    v = 2 as libc::c_int as libc::c_double * zxminmin
        - 2 as libc::c_int as libc::c_double * zxminmax + zxyminmin + zxyminmax;
    *z_pp += 3 as libc::c_int as libc::c_double * v * t0 * u2;
    v = -(3 as libc::c_int) as libc::c_double * zyminmin
        + 3 as libc::c_int as libc::c_double * zymaxmin
        - 2 as libc::c_int as libc::c_double * zxyminmin - zxymaxmin;
    *z_pp += 2 as libc::c_int as libc::c_double * v * t1 * u0;
    v = 9 as libc::c_int as libc::c_double * zminmin
        - 9 as libc::c_int as libc::c_double * zmaxmin
        + 9 as libc::c_int as libc::c_double * zmaxmax
        - 9 as libc::c_int as libc::c_double * zminmax
        + 6 as libc::c_int as libc::c_double * zxminmin
        + 3 as libc::c_int as libc::c_double * zxmaxmin
        - 3 as libc::c_int as libc::c_double * zxmaxmax
        - 6 as libc::c_int as libc::c_double * zxminmax
        + 6 as libc::c_int as libc::c_double * zyminmin
        - 6 as libc::c_int as libc::c_double * zymaxmin
        - 3 as libc::c_int as libc::c_double * zymaxmax
        + 3 as libc::c_int as libc::c_double * zyminmax
        + 4 as libc::c_int as libc::c_double * zxyminmin
        + 2 as libc::c_int as libc::c_double * zxymaxmin + zxymaxmax
        + 2 as libc::c_int as libc::c_double * zxyminmax;
    *z_pp += 4 as libc::c_int as libc::c_double * v * t1 * u1;
    v = -(6 as libc::c_int) as libc::c_double * zminmin
        + 6 as libc::c_int as libc::c_double * zmaxmin
        - 6 as libc::c_int as libc::c_double * zmaxmax
        + 6 as libc::c_int as libc::c_double * zminmax
        - 4 as libc::c_int as libc::c_double * zxminmin
        - 2 as libc::c_int as libc::c_double * zxmaxmin
        + 2 as libc::c_int as libc::c_double * zxmaxmax
        + 4 as libc::c_int as libc::c_double * zxminmax
        - 3 as libc::c_int as libc::c_double * zyminmin
        + 3 as libc::c_int as libc::c_double * zymaxmin
        + 3 as libc::c_int as libc::c_double * zymaxmax
        - 3 as libc::c_int as libc::c_double * zyminmax
        - 2 as libc::c_int as libc::c_double * zxyminmin - zxymaxmin - zxymaxmax
        - 2 as libc::c_int as libc::c_double * zxyminmax;
    *z_pp += 6 as libc::c_int as libc::c_double * v * t1 * u2;
    v = 2 as libc::c_int as libc::c_double * zyminmin
        - 2 as libc::c_int as libc::c_double * zymaxmin + zxyminmin + zxymaxmin;
    *z_pp += 3 as libc::c_int as libc::c_double * v * t2 * u0;
    v = -(6 as libc::c_int) as libc::c_double * zminmin
        + 6 as libc::c_int as libc::c_double * zmaxmin
        - 6 as libc::c_int as libc::c_double * zmaxmax
        + 6 as libc::c_int as libc::c_double * zminmax
        - 3 as libc::c_int as libc::c_double * zxminmin
        - 3 as libc::c_int as libc::c_double * zxmaxmin
        + 3 as libc::c_int as libc::c_double * zxmaxmax
        + 3 as libc::c_int as libc::c_double * zxminmax
        - 4 as libc::c_int as libc::c_double * zyminmin
        + 4 as libc::c_int as libc::c_double * zymaxmin
        + 2 as libc::c_int as libc::c_double * zymaxmax
        - 2 as libc::c_int as libc::c_double * zyminmax
        - 2 as libc::c_int as libc::c_double * zxyminmin
        - 2 as libc::c_int as libc::c_double * zxymaxmin - zxymaxmax - zxyminmax;
    *z_pp += 6 as libc::c_int as libc::c_double * v * t2 * u1;
    v = 4 as libc::c_int as libc::c_double * zminmin
        - 4 as libc::c_int as libc::c_double * zmaxmin
        + 4 as libc::c_int as libc::c_double * zmaxmax
        - 4 as libc::c_int as libc::c_double * zminmax
        + 2 as libc::c_int as libc::c_double * zxminmin
        + 2 as libc::c_int as libc::c_double * zxmaxmin
        - 2 as libc::c_int as libc::c_double * zxmaxmax
        - 2 as libc::c_int as libc::c_double * zxminmax
        + 2 as libc::c_int as libc::c_double * zyminmin
        - 2 as libc::c_int as libc::c_double * zymaxmin
        - 2 as libc::c_int as libc::c_double * zymaxmax
        + 2 as libc::c_int as libc::c_double * zyminmax + zxyminmin + zxymaxmin
        + zxymaxmax + zxyminmax;
    *z_pp += 9 as libc::c_int as libc::c_double * v * t2 * u2;
    *z_pp *= dt * du;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn bicubic_deriv_yy(
    mut vstate: *const libc::c_void,
    mut xarr: *const libc::c_double,
    mut yarr: *const libc::c_double,
    mut zarr: *const libc::c_double,
    mut xsize: size_t,
    mut ysize: size_t,
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut xa: *mut gsl_interp_accel,
    mut ya: *mut gsl_interp_accel,
    mut z_pp: *mut libc::c_double,
) -> libc::c_int {
    let mut state: *mut bicubic_state_t = vstate as *mut bicubic_state_t;
    let mut xmin: libc::c_double = 0.;
    let mut xmax: libc::c_double = 0.;
    let mut ymin: libc::c_double = 0.;
    let mut ymax: libc::c_double = 0.;
    let mut zminmin: libc::c_double = 0.;
    let mut zminmax: libc::c_double = 0.;
    let mut zmaxmin: libc::c_double = 0.;
    let mut zmaxmax: libc::c_double = 0.;
    let mut zxminmin: libc::c_double = 0.;
    let mut zxminmax: libc::c_double = 0.;
    let mut zxmaxmin: libc::c_double = 0.;
    let mut zxmaxmax: libc::c_double = 0.;
    let mut zyminmin: libc::c_double = 0.;
    let mut zyminmax: libc::c_double = 0.;
    let mut zymaxmin: libc::c_double = 0.;
    let mut zymaxmax: libc::c_double = 0.;
    let mut zxyminmin: libc::c_double = 0.;
    let mut zxyminmax: libc::c_double = 0.;
    let mut zxymaxmin: libc::c_double = 0.;
    let mut zxymaxmax: libc::c_double = 0.;
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    let mut dt: libc::c_double = 0.;
    let mut du: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut u: libc::c_double = 0.;
    let mut t0: libc::c_double = 0.;
    let mut t1: libc::c_double = 0.;
    let mut t2: libc::c_double = 0.;
    let mut t3: libc::c_double = 0.;
    let mut u0: libc::c_double = 0.;
    let mut u1: libc::c_double = 0.;
    let mut v: libc::c_double = 0.;
    let mut xi: size_t = 0;
    let mut yi: size_t = 0;
    if !xa.is_null() {
        xi = gsl_interp_accel_find(xa, xarr, xsize, x);
    } else {
        xi = gsl_interp_bsearch(
            xarr,
            x,
            0 as libc::c_int as size_t,
            xsize.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    if !ya.is_null() {
        yi = gsl_interp_accel_find(ya, yarr, ysize, y);
    } else {
        yi = gsl_interp_bsearch(
            yarr,
            y,
            0 as libc::c_int as size_t,
            ysize.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    xmin = *xarr.offset(xi as isize);
    xmax = *xarr.offset(xi.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
    ymin = *yarr.offset(yi as isize);
    ymax = *yarr.offset(yi.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
    zminmin = *zarr.offset(yi.wrapping_mul((*state).xsize).wrapping_add(xi) as isize);
    zminmax = *zarr
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi) as isize,
        );
    zmaxmin = *zarr
        .offset(
            yi
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        );
    zmaxmax = *zarr
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        );
    dx = xmax - xmin;
    dy = ymax - ymin;
    t = (x - xmin) / dx;
    u = (y - ymin) / dy;
    dt = 1.0f64 / dx;
    du = 1.0f64 / dy;
    zxminmin = *((*state).zx)
        .offset(yi.wrapping_mul((*state).xsize).wrapping_add(xi) as isize) / dt;
    zxminmax = *((*state).zx)
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi) as isize,
        ) / dt;
    zxmaxmin = *((*state).zx)
        .offset(
            yi
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        ) / dt;
    zxmaxmax = *((*state).zx)
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        ) / dt;
    zyminmin = *((*state).zy)
        .offset(yi.wrapping_mul((*state).xsize).wrapping_add(xi) as isize) / du;
    zyminmax = *((*state).zy)
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi) as isize,
        ) / du;
    zymaxmin = *((*state).zy)
        .offset(
            yi
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        ) / du;
    zymaxmax = *((*state).zy)
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        ) / du;
    zxyminmin = *((*state).zxy)
        .offset(yi.wrapping_mul((*state).xsize).wrapping_add(xi) as isize) / (dt * du);
    zxyminmax = *((*state).zxy)
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi) as isize,
        ) / (dt * du);
    zxymaxmin = *((*state).zxy)
        .offset(
            yi
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        ) / (dt * du);
    zxymaxmax = *((*state).zxy)
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        ) / (dt * du);
    t0 = 1 as libc::c_int as libc::c_double;
    t1 = t;
    t2 = t * t;
    t3 = t * t2;
    u0 = 1 as libc::c_int as libc::c_double;
    u1 = u;
    *z_pp = 0 as libc::c_int as libc::c_double;
    v = -(3 as libc::c_int) as libc::c_double * zminmin
        + 3 as libc::c_int as libc::c_double * zminmax
        - 2 as libc::c_int as libc::c_double * zyminmin - zyminmax;
    *z_pp += 2 as libc::c_int as libc::c_double * v * t0 * u0;
    v = 2 as libc::c_int as libc::c_double * zminmin
        - 2 as libc::c_int as libc::c_double * zminmax + zyminmin + zyminmax;
    *z_pp += 6 as libc::c_int as libc::c_double * v * t0 * u1;
    v = -(3 as libc::c_int) as libc::c_double * zxminmin
        + 3 as libc::c_int as libc::c_double * zxminmax
        - 2 as libc::c_int as libc::c_double * zxyminmin - zxyminmax;
    *z_pp += 2 as libc::c_int as libc::c_double * v * t1 * u0;
    v = 2 as libc::c_int as libc::c_double * zxminmin
        - 2 as libc::c_int as libc::c_double * zxminmax + zxyminmin + zxyminmax;
    *z_pp += 6 as libc::c_int as libc::c_double * v * t1 * u1;
    v = 9 as libc::c_int as libc::c_double * zminmin
        - 9 as libc::c_int as libc::c_double * zmaxmin
        + 9 as libc::c_int as libc::c_double * zmaxmax
        - 9 as libc::c_int as libc::c_double * zminmax
        + 6 as libc::c_int as libc::c_double * zxminmin
        + 3 as libc::c_int as libc::c_double * zxmaxmin
        - 3 as libc::c_int as libc::c_double * zxmaxmax
        - 6 as libc::c_int as libc::c_double * zxminmax
        + 6 as libc::c_int as libc::c_double * zyminmin
        - 6 as libc::c_int as libc::c_double * zymaxmin
        - 3 as libc::c_int as libc::c_double * zymaxmax
        + 3 as libc::c_int as libc::c_double * zyminmax
        + 4 as libc::c_int as libc::c_double * zxyminmin
        + 2 as libc::c_int as libc::c_double * zxymaxmin + zxymaxmax
        + 2 as libc::c_int as libc::c_double * zxyminmax;
    *z_pp += 2 as libc::c_int as libc::c_double * v * t2 * u0;
    v = -(6 as libc::c_int) as libc::c_double * zminmin
        + 6 as libc::c_int as libc::c_double * zmaxmin
        - 6 as libc::c_int as libc::c_double * zmaxmax
        + 6 as libc::c_int as libc::c_double * zminmax
        - 4 as libc::c_int as libc::c_double * zxminmin
        - 2 as libc::c_int as libc::c_double * zxmaxmin
        + 2 as libc::c_int as libc::c_double * zxmaxmax
        + 4 as libc::c_int as libc::c_double * zxminmax
        - 3 as libc::c_int as libc::c_double * zyminmin
        + 3 as libc::c_int as libc::c_double * zymaxmin
        + 3 as libc::c_int as libc::c_double * zymaxmax
        - 3 as libc::c_int as libc::c_double * zyminmax
        - 2 as libc::c_int as libc::c_double * zxyminmin - zxymaxmin - zxymaxmax
        - 2 as libc::c_int as libc::c_double * zxyminmax;
    *z_pp += 6 as libc::c_int as libc::c_double * v * t2 * u1;
    v = -(6 as libc::c_int) as libc::c_double * zminmin
        + 6 as libc::c_int as libc::c_double * zmaxmin
        - 6 as libc::c_int as libc::c_double * zmaxmax
        + 6 as libc::c_int as libc::c_double * zminmax
        - 3 as libc::c_int as libc::c_double * zxminmin
        - 3 as libc::c_int as libc::c_double * zxmaxmin
        + 3 as libc::c_int as libc::c_double * zxmaxmax
        + 3 as libc::c_int as libc::c_double * zxminmax
        - 4 as libc::c_int as libc::c_double * zyminmin
        + 4 as libc::c_int as libc::c_double * zymaxmin
        + 2 as libc::c_int as libc::c_double * zymaxmax
        - 2 as libc::c_int as libc::c_double * zyminmax
        - 2 as libc::c_int as libc::c_double * zxyminmin
        - 2 as libc::c_int as libc::c_double * zxymaxmin - zxymaxmax - zxyminmax;
    *z_pp += 2 as libc::c_int as libc::c_double * v * t3 * u0;
    v = 4 as libc::c_int as libc::c_double * zminmin
        - 4 as libc::c_int as libc::c_double * zmaxmin
        + 4 as libc::c_int as libc::c_double * zmaxmax
        - 4 as libc::c_int as libc::c_double * zminmax
        + 2 as libc::c_int as libc::c_double * zxminmin
        + 2 as libc::c_int as libc::c_double * zxmaxmin
        - 2 as libc::c_int as libc::c_double * zxmaxmax
        - 2 as libc::c_int as libc::c_double * zxminmax
        + 2 as libc::c_int as libc::c_double * zyminmin
        - 2 as libc::c_int as libc::c_double * zymaxmin
        - 2 as libc::c_int as libc::c_double * zymaxmax
        + 2 as libc::c_int as libc::c_double * zyminmax + zxyminmin + zxymaxmin
        + zxymaxmax + zxyminmax;
    *z_pp += 6 as libc::c_int as libc::c_double * v * t3 * u1;
    *z_pp *= du * du;
    return GSL_SUCCESS as libc::c_int;
}
static mut bicubic_type: gsl_interp2d_type = {
    let mut init = gsl_interp2d_type {
        name: b"bicubic\0" as *const u8 as *const libc::c_char,
        min_size: 4 as libc::c_int as libc::c_uint,
        alloc: Some(
            bicubic_alloc as unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void,
        ),
        init: Some(
            bicubic_init
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_double,
                    *const libc::c_double,
                    *const libc::c_double,
                    size_t,
                    size_t,
                ) -> libc::c_int,
        ),
        eval: Some(
            bicubic_eval
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_double,
                    *const libc::c_double,
                    *const libc::c_double,
                    size_t,
                    size_t,
                    libc::c_double,
                    libc::c_double,
                    *mut gsl_interp_accel,
                    *mut gsl_interp_accel,
                    *mut libc::c_double,
                ) -> libc::c_int,
        ),
        eval_deriv_x: Some(
            bicubic_deriv_x
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_double,
                    *const libc::c_double,
                    *const libc::c_double,
                    size_t,
                    size_t,
                    libc::c_double,
                    libc::c_double,
                    *mut gsl_interp_accel,
                    *mut gsl_interp_accel,
                    *mut libc::c_double,
                ) -> libc::c_int,
        ),
        eval_deriv_y: Some(
            bicubic_deriv_y
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_double,
                    *const libc::c_double,
                    *const libc::c_double,
                    size_t,
                    size_t,
                    libc::c_double,
                    libc::c_double,
                    *mut gsl_interp_accel,
                    *mut gsl_interp_accel,
                    *mut libc::c_double,
                ) -> libc::c_int,
        ),
        eval_deriv_xx: Some(
            bicubic_deriv_xx
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_double,
                    *const libc::c_double,
                    *const libc::c_double,
                    size_t,
                    size_t,
                    libc::c_double,
                    libc::c_double,
                    *mut gsl_interp_accel,
                    *mut gsl_interp_accel,
                    *mut libc::c_double,
                ) -> libc::c_int,
        ),
        eval_deriv_xy: Some(
            bicubic_deriv_xy
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_double,
                    *const libc::c_double,
                    *const libc::c_double,
                    size_t,
                    size_t,
                    libc::c_double,
                    libc::c_double,
                    *mut gsl_interp_accel,
                    *mut gsl_interp_accel,
                    *mut libc::c_double,
                ) -> libc::c_int,
        ),
        eval_deriv_yy: Some(
            bicubic_deriv_yy
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_double,
                    *const libc::c_double,
                    *const libc::c_double,
                    size_t,
                    size_t,
                    libc::c_double,
                    libc::c_double,
                    *mut gsl_interp_accel,
                    *mut gsl_interp_accel,
                    *mut libc::c_double,
                ) -> libc::c_int,
        ),
        free: Some(bicubic_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    };
    init
};
pub static mut gsl_interp2d_bicubic: *const gsl_interp2d_type = unsafe {
    &bicubic_type as *const gsl_interp2d_type
};
