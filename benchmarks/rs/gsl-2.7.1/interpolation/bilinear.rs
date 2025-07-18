use ::libc;
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
unsafe extern "C" fn bilinear_init(
    mut state: *mut libc::c_void,
    mut xa: *const libc::c_double,
    mut ya: *const libc::c_double,
    mut za: *const libc::c_double,
    mut xsize: size_t,
    mut ysize: size_t,
) -> libc::c_int {
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn bilinear_eval(
    mut state: *const libc::c_void,
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
    let mut xmin: libc::c_double = 0.;
    let mut xmax: libc::c_double = 0.;
    let mut ymin: libc::c_double = 0.;
    let mut ymax: libc::c_double = 0.;
    let mut zminmin: libc::c_double = 0.;
    let mut zminmax: libc::c_double = 0.;
    let mut zmaxmin: libc::c_double = 0.;
    let mut zmaxmax: libc::c_double = 0.;
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut u: libc::c_double = 0.;
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
    zminmin = *zarr.offset(yi.wrapping_mul(xsize).wrapping_add(xi) as isize);
    zminmax = *zarr
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(xsize)
                .wrapping_add(xi) as isize,
        );
    zmaxmin = *zarr
        .offset(
            yi
                .wrapping_mul(xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        );
    zmaxmax = *zarr
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        );
    dx = xmax - xmin;
    dy = ymax - ymin;
    t = (x - xmin) / dx;
    u = (y - ymin) / dy;
    *z = (1.0f64 - t) * (1.0f64 - u) * zminmin + t * (1.0f64 - u) * zmaxmin
        + (1.0f64 - t) * u * zminmax + t * u * zmaxmax;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn bilinear_deriv_x(
    mut state: *const libc::c_void,
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
    let mut xmin: libc::c_double = 0.;
    let mut xmax: libc::c_double = 0.;
    let mut ymin: libc::c_double = 0.;
    let mut ymax: libc::c_double = 0.;
    let mut zminmin: libc::c_double = 0.;
    let mut zminmax: libc::c_double = 0.;
    let mut zmaxmin: libc::c_double = 0.;
    let mut zmaxmax: libc::c_double = 0.;
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    let mut dt: libc::c_double = 0.;
    let mut u: libc::c_double = 0.;
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
    zminmin = *zarr.offset(yi.wrapping_mul(xsize).wrapping_add(xi) as isize);
    zminmax = *zarr
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(xsize)
                .wrapping_add(xi) as isize,
        );
    zmaxmin = *zarr
        .offset(
            yi
                .wrapping_mul(xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        );
    zmaxmax = *zarr
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        );
    dx = xmax - xmin;
    dy = ymax - ymin;
    dt = 1.0f64 / dx;
    u = (y - ymin) / dy;
    *z_p = dt
        * (-(1.0f64 - u) * zminmin + (1.0f64 - u) * zmaxmin - u * zminmax + u * zmaxmax);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn bilinear_deriv_y(
    mut state: *const libc::c_void,
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
    let mut xmin: libc::c_double = 0.;
    let mut xmax: libc::c_double = 0.;
    let mut ymin: libc::c_double = 0.;
    let mut ymax: libc::c_double = 0.;
    let mut zminmin: libc::c_double = 0.;
    let mut zminmax: libc::c_double = 0.;
    let mut zmaxmin: libc::c_double = 0.;
    let mut zmaxmax: libc::c_double = 0.;
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut du: libc::c_double = 0.;
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
    zminmin = *zarr.offset(yi.wrapping_mul(xsize).wrapping_add(xi) as isize);
    zminmax = *zarr
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(xsize)
                .wrapping_add(xi) as isize,
        );
    zmaxmin = *zarr
        .offset(
            yi
                .wrapping_mul(xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        );
    zmaxmax = *zarr
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        );
    dx = xmax - xmin;
    dy = ymax - ymin;
    t = (x - xmin) / dx;
    du = 1.0f64 / dy;
    *z_p = du
        * (-(1.0f64 - t) * zminmin - t * zmaxmin + (1.0f64 - t) * zminmax + t * zmaxmax);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn bilinear_deriv2(
    mut state: *const libc::c_void,
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
    *z_pp = 0.0f64;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn bilinear_derivxy(
    mut state: *const libc::c_void,
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
    let mut xmin: libc::c_double = 0.;
    let mut xmax: libc::c_double = 0.;
    let mut ymin: libc::c_double = 0.;
    let mut ymax: libc::c_double = 0.;
    let mut zminmin: libc::c_double = 0.;
    let mut zminmax: libc::c_double = 0.;
    let mut zmaxmin: libc::c_double = 0.;
    let mut zmaxmax: libc::c_double = 0.;
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    let mut dt: libc::c_double = 0.;
    let mut du: libc::c_double = 0.;
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
    zminmin = *zarr.offset(yi.wrapping_mul(xsize).wrapping_add(xi) as isize);
    zminmax = *zarr
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(xsize)
                .wrapping_add(xi) as isize,
        );
    zmaxmin = *zarr
        .offset(
            yi
                .wrapping_mul(xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        );
    zmaxmax = *zarr
        .offset(
            yi
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(xsize)
                .wrapping_add(xi.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as isize,
        );
    dx = xmax - xmin;
    dy = ymax - ymin;
    dt = 1.0f64 / dx;
    du = 1.0f64 / dy;
    *z_pp = dt * du * (zminmin - zmaxmin - zminmax + zmaxmax);
    return GSL_SUCCESS as libc::c_int;
}
static mut bilinear_type: gsl_interp2d_type = {
    let mut init = gsl_interp2d_type {
        name: b"bilinear\0" as *const u8 as *const libc::c_char,
        min_size: 2 as libc::c_int as libc::c_uint,
        alloc: None,
        init: Some(
            bilinear_init
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
            bilinear_eval
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
            bilinear_deriv_x
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
            bilinear_deriv_y
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
            bilinear_deriv2
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
            bilinear_derivxy
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
            bilinear_deriv2
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
        free: None,
    };
    init
};
pub static mut gsl_interp2d_bilinear: *const gsl_interp2d_type = unsafe {
    &bilinear_type as *const gsl_interp2d_type
};
