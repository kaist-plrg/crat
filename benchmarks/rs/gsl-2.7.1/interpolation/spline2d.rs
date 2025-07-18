use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_interp2d_name(interp: *const gsl_interp2d) -> *const libc::c_char;
    fn gsl_interp2d_min_size(interp: *const gsl_interp2d) -> size_t;
    fn gsl_interp2d_set(
        interp: *const gsl_interp2d,
        zarr: *mut libc::c_double,
        i: size_t,
        j: size_t,
        z: libc::c_double,
    ) -> libc::c_int;
    fn gsl_interp2d_get(
        interp: *const gsl_interp2d,
        zarr: *const libc::c_double,
        i: size_t,
        j: size_t,
    ) -> libc::c_double;
    fn gsl_interp2d_init(
        interp: *mut gsl_interp2d,
        xa: *const libc::c_double,
        ya: *const libc::c_double,
        za: *const libc::c_double,
        xsize: size_t,
        ysize: size_t,
    ) -> libc::c_int;
    fn gsl_interp2d_eval(
        interp: *const gsl_interp2d,
        xarr: *const libc::c_double,
        yarr: *const libc::c_double,
        zarr: *const libc::c_double,
        x: libc::c_double,
        y: libc::c_double,
        xa: *mut gsl_interp_accel,
        ya: *mut gsl_interp_accel,
    ) -> libc::c_double;
    fn gsl_interp2d_eval_extrap(
        interp: *const gsl_interp2d,
        xarr: *const libc::c_double,
        yarr: *const libc::c_double,
        zarr: *const libc::c_double,
        x: libc::c_double,
        y: libc::c_double,
        xa: *mut gsl_interp_accel,
        ya: *mut gsl_interp_accel,
    ) -> libc::c_double;
    fn gsl_interp2d_eval_e(
        interp: *const gsl_interp2d,
        xarr: *const libc::c_double,
        yarr: *const libc::c_double,
        zarr: *const libc::c_double,
        x: libc::c_double,
        y: libc::c_double,
        xa: *mut gsl_interp_accel,
        ya: *mut gsl_interp_accel,
        z: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_interp2d_eval_extrap_e(
        interp: *const gsl_interp2d,
        xarr: *const libc::c_double,
        yarr: *const libc::c_double,
        zarr: *const libc::c_double,
        x: libc::c_double,
        y: libc::c_double,
        xa: *mut gsl_interp_accel,
        ya: *mut gsl_interp_accel,
        z: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_interp2d_eval_deriv_x(
        interp: *const gsl_interp2d,
        xarr: *const libc::c_double,
        yarr: *const libc::c_double,
        zarr: *const libc::c_double,
        x: libc::c_double,
        y: libc::c_double,
        xa: *mut gsl_interp_accel,
        ya: *mut gsl_interp_accel,
    ) -> libc::c_double;
    fn gsl_interp2d_eval_deriv_x_e(
        interp: *const gsl_interp2d,
        xarr: *const libc::c_double,
        yarr: *const libc::c_double,
        zarr: *const libc::c_double,
        x: libc::c_double,
        y: libc::c_double,
        xa: *mut gsl_interp_accel,
        ya: *mut gsl_interp_accel,
        z: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_interp2d_eval_deriv_y(
        interp: *const gsl_interp2d,
        xarr: *const libc::c_double,
        yarr: *const libc::c_double,
        zarr: *const libc::c_double,
        x: libc::c_double,
        y: libc::c_double,
        xa: *mut gsl_interp_accel,
        ya: *mut gsl_interp_accel,
    ) -> libc::c_double;
    fn gsl_interp2d_eval_deriv_y_e(
        interp: *const gsl_interp2d,
        xarr: *const libc::c_double,
        yarr: *const libc::c_double,
        zarr: *const libc::c_double,
        x: libc::c_double,
        y: libc::c_double,
        xa: *mut gsl_interp_accel,
        ya: *mut gsl_interp_accel,
        z: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_interp2d_eval_deriv_xx(
        interp: *const gsl_interp2d,
        xarr: *const libc::c_double,
        yarr: *const libc::c_double,
        zarr: *const libc::c_double,
        x: libc::c_double,
        y: libc::c_double,
        xa: *mut gsl_interp_accel,
        ya: *mut gsl_interp_accel,
    ) -> libc::c_double;
    fn gsl_interp2d_eval_deriv_xx_e(
        interp: *const gsl_interp2d,
        xarr: *const libc::c_double,
        yarr: *const libc::c_double,
        zarr: *const libc::c_double,
        x: libc::c_double,
        y: libc::c_double,
        xa: *mut gsl_interp_accel,
        ya: *mut gsl_interp_accel,
        z: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_interp2d_eval_deriv_yy(
        interp: *const gsl_interp2d,
        xarr: *const libc::c_double,
        yarr: *const libc::c_double,
        zarr: *const libc::c_double,
        x: libc::c_double,
        y: libc::c_double,
        xa: *mut gsl_interp_accel,
        ya: *mut gsl_interp_accel,
    ) -> libc::c_double;
    fn gsl_interp2d_eval_deriv_yy_e(
        interp: *const gsl_interp2d,
        xarr: *const libc::c_double,
        yarr: *const libc::c_double,
        zarr: *const libc::c_double,
        x: libc::c_double,
        y: libc::c_double,
        xa: *mut gsl_interp_accel,
        ya: *mut gsl_interp_accel,
        z: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_interp2d_eval_deriv_xy(
        interp: *const gsl_interp2d,
        xarr: *const libc::c_double,
        yarr: *const libc::c_double,
        zarr: *const libc::c_double,
        x: libc::c_double,
        y: libc::c_double,
        xa: *mut gsl_interp_accel,
        ya: *mut gsl_interp_accel,
    ) -> libc::c_double;
    fn gsl_interp2d_eval_deriv_xy_e(
        interp: *const gsl_interp2d,
        xarr: *const libc::c_double,
        yarr: *const libc::c_double,
        zarr: *const libc::c_double,
        x: libc::c_double,
        y: libc::c_double,
        xa: *mut gsl_interp_accel,
        ya: *mut gsl_interp_accel,
        z: *mut libc::c_double,
    ) -> libc::c_int;
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
pub struct gsl_interp2d {
    pub type_0: *const gsl_interp2d_type,
    pub xmin: libc::c_double,
    pub xmax: libc::c_double,
    pub ymin: libc::c_double,
    pub ymax: libc::c_double,
    pub xsize: size_t,
    pub ysize: size_t,
    pub state: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_spline2d {
    pub interp_object: gsl_interp2d,
    pub xarr: *mut libc::c_double,
    pub yarr: *mut libc::c_double,
    pub zarr: *mut libc::c_double,
}
pub unsafe extern "C" fn gsl_spline2d_alloc(
    mut T: *const gsl_interp2d_type,
    mut xsize: size_t,
    mut ysize: size_t,
) -> *mut gsl_spline2d {
    let mut array_mem: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut interp: *mut gsl_spline2d = 0 as *mut gsl_spline2d;
    if xsize < (*T).min_size as libc::c_ulong || ysize < (*T).min_size as libc::c_ulong {
        gsl_error(
            b"insufficient number of points for interpolation type\0" as *const u8
                as *const libc::c_char,
            b"spline2d.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_spline2d;
    }
    interp = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gsl_spline2d>() as libc::c_ulong,
    ) as *mut gsl_spline2d;
    if interp.is_null() {
        gsl_error(
            b"failed to allocate space for gsl_spline2d struct\0" as *const u8
                as *const libc::c_char,
            b"spline2d.c\0" as *const u8 as *const libc::c_char,
            43 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spline2d;
    }
    (*interp).interp_object.type_0 = T;
    (*interp).interp_object.xsize = xsize;
    (*interp).interp_object.ysize = ysize;
    if ((*(*interp).interp_object.type_0).alloc).is_none() {
        (*interp).interp_object.state = 0 as *mut libc::c_void;
    } else {
        (*interp)
            .interp_object
            .state = ((*(*interp).interp_object.type_0).alloc).unwrap()(xsize, ysize);
        if ((*interp).interp_object.state).is_null() {
            gsl_spline2d_free(interp);
            gsl_error(
                b"failed to allocate space for gsl_spline2d state\0" as *const u8
                    as *const libc::c_char,
                b"spline2d.c\0" as *const u8 as *const libc::c_char,
                59 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spline2d;
        }
    }
    array_mem = calloc(
        xsize.wrapping_add(ysize).wrapping_add(xsize.wrapping_mul(ysize)),
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    if array_mem.is_null() {
        gsl_spline2d_free(interp);
        gsl_error(
            b"failed to allocate space for data arrays\0" as *const u8
                as *const libc::c_char,
            b"spline2d.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spline2d;
    }
    (*interp).xarr = array_mem;
    (*interp).yarr = array_mem.offset(xsize as isize);
    (*interp).zarr = array_mem.offset(xsize as isize).offset(ysize as isize);
    return interp;
}
pub unsafe extern "C" fn gsl_spline2d_init(
    mut interp: *mut gsl_spline2d,
    mut xarr: *const libc::c_double,
    mut yarr: *const libc::c_double,
    mut zarr: *const libc::c_double,
    mut xsize: size_t,
    mut ysize: size_t,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_interp2d_init(
        &mut (*interp).interp_object,
        xarr,
        yarr,
        zarr,
        xsize,
        ysize,
    );
    memcpy(
        (*interp).xarr as *mut libc::c_void,
        xarr as *const libc::c_void,
        xsize.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memcpy(
        (*interp).yarr as *mut libc::c_void,
        yarr as *const libc::c_void,
        ysize.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memcpy(
        (*interp).zarr as *mut libc::c_void,
        zarr as *const libc::c_void,
        xsize
            .wrapping_mul(ysize)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    return status;
}
pub unsafe extern "C" fn gsl_spline2d_free(mut interp: *mut gsl_spline2d) {
    if interp.is_null() {
        return;
    }
    if ((*(*interp).interp_object.type_0).free).is_some() {
        ((*(*interp).interp_object.type_0).free).unwrap()((*interp).interp_object.state);
    }
    if !((*interp).xarr).is_null() {
        free((*interp).xarr as *mut libc::c_void);
    }
    free(interp as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_spline2d_eval(
    mut interp: *const gsl_spline2d,
    x: libc::c_double,
    y: libc::c_double,
    mut xa: *mut gsl_interp_accel,
    mut ya: *mut gsl_interp_accel,
) -> libc::c_double {
    return gsl_interp2d_eval(
        &(*interp).interp_object,
        (*interp).xarr as *const libc::c_double,
        (*interp).yarr as *const libc::c_double,
        (*interp).zarr as *const libc::c_double,
        x,
        y,
        xa,
        ya,
    );
}
pub unsafe extern "C" fn gsl_spline2d_eval_e(
    mut interp: *const gsl_spline2d,
    x: libc::c_double,
    y: libc::c_double,
    mut xa: *mut gsl_interp_accel,
    mut ya: *mut gsl_interp_accel,
    mut z: *mut libc::c_double,
) -> libc::c_int {
    return gsl_interp2d_eval_e(
        &(*interp).interp_object,
        (*interp).xarr as *const libc::c_double,
        (*interp).yarr as *const libc::c_double,
        (*interp).zarr as *const libc::c_double,
        x,
        y,
        xa,
        ya,
        z,
    );
}
pub unsafe extern "C" fn gsl_spline2d_eval_extrap(
    mut interp: *const gsl_spline2d,
    x: libc::c_double,
    y: libc::c_double,
    mut xa: *mut gsl_interp_accel,
    mut ya: *mut gsl_interp_accel,
) -> libc::c_double {
    return gsl_interp2d_eval_extrap(
        &(*interp).interp_object,
        (*interp).xarr as *const libc::c_double,
        (*interp).yarr as *const libc::c_double,
        (*interp).zarr as *const libc::c_double,
        x,
        y,
        xa,
        ya,
    );
}
pub unsafe extern "C" fn gsl_spline2d_eval_extrap_e(
    mut interp: *const gsl_spline2d,
    x: libc::c_double,
    y: libc::c_double,
    mut xa: *mut gsl_interp_accel,
    mut ya: *mut gsl_interp_accel,
    mut z: *mut libc::c_double,
) -> libc::c_int {
    return gsl_interp2d_eval_extrap_e(
        &(*interp).interp_object,
        (*interp).xarr as *const libc::c_double,
        (*interp).yarr as *const libc::c_double,
        (*interp).zarr as *const libc::c_double,
        x,
        y,
        xa,
        ya,
        z,
    );
}
pub unsafe extern "C" fn gsl_spline2d_eval_deriv_x(
    mut interp: *const gsl_spline2d,
    x: libc::c_double,
    y: libc::c_double,
    mut xa: *mut gsl_interp_accel,
    mut ya: *mut gsl_interp_accel,
) -> libc::c_double {
    return gsl_interp2d_eval_deriv_x(
        &(*interp).interp_object,
        (*interp).xarr as *const libc::c_double,
        (*interp).yarr as *const libc::c_double,
        (*interp).zarr as *const libc::c_double,
        x,
        y,
        xa,
        ya,
    );
}
pub unsafe extern "C" fn gsl_spline2d_eval_deriv_x_e(
    mut interp: *const gsl_spline2d,
    x: libc::c_double,
    y: libc::c_double,
    mut xa: *mut gsl_interp_accel,
    mut ya: *mut gsl_interp_accel,
    mut z: *mut libc::c_double,
) -> libc::c_int {
    return gsl_interp2d_eval_deriv_x_e(
        &(*interp).interp_object,
        (*interp).xarr as *const libc::c_double,
        (*interp).yarr as *const libc::c_double,
        (*interp).zarr as *const libc::c_double,
        x,
        y,
        xa,
        ya,
        z,
    );
}
pub unsafe extern "C" fn gsl_spline2d_eval_deriv_y(
    mut interp: *const gsl_spline2d,
    x: libc::c_double,
    y: libc::c_double,
    mut xa: *mut gsl_interp_accel,
    mut ya: *mut gsl_interp_accel,
) -> libc::c_double {
    return gsl_interp2d_eval_deriv_y(
        &(*interp).interp_object,
        (*interp).xarr as *const libc::c_double,
        (*interp).yarr as *const libc::c_double,
        (*interp).zarr as *const libc::c_double,
        x,
        y,
        xa,
        ya,
    );
}
pub unsafe extern "C" fn gsl_spline2d_eval_deriv_y_e(
    mut interp: *const gsl_spline2d,
    x: libc::c_double,
    y: libc::c_double,
    mut xa: *mut gsl_interp_accel,
    mut ya: *mut gsl_interp_accel,
    mut z: *mut libc::c_double,
) -> libc::c_int {
    return gsl_interp2d_eval_deriv_y_e(
        &(*interp).interp_object,
        (*interp).xarr as *const libc::c_double,
        (*interp).yarr as *const libc::c_double,
        (*interp).zarr as *const libc::c_double,
        x,
        y,
        xa,
        ya,
        z,
    );
}
pub unsafe extern "C" fn gsl_spline2d_eval_deriv_xx(
    mut interp: *const gsl_spline2d,
    x: libc::c_double,
    y: libc::c_double,
    mut xa: *mut gsl_interp_accel,
    mut ya: *mut gsl_interp_accel,
) -> libc::c_double {
    return gsl_interp2d_eval_deriv_xx(
        &(*interp).interp_object,
        (*interp).xarr as *const libc::c_double,
        (*interp).yarr as *const libc::c_double,
        (*interp).zarr as *const libc::c_double,
        x,
        y,
        xa,
        ya,
    );
}
pub unsafe extern "C" fn gsl_spline2d_eval_deriv_xx_e(
    mut interp: *const gsl_spline2d,
    x: libc::c_double,
    y: libc::c_double,
    mut xa: *mut gsl_interp_accel,
    mut ya: *mut gsl_interp_accel,
    mut z: *mut libc::c_double,
) -> libc::c_int {
    return gsl_interp2d_eval_deriv_xx_e(
        &(*interp).interp_object,
        (*interp).xarr as *const libc::c_double,
        (*interp).yarr as *const libc::c_double,
        (*interp).zarr as *const libc::c_double,
        x,
        y,
        xa,
        ya,
        z,
    );
}
pub unsafe extern "C" fn gsl_spline2d_eval_deriv_yy(
    mut interp: *const gsl_spline2d,
    x: libc::c_double,
    y: libc::c_double,
    mut xa: *mut gsl_interp_accel,
    mut ya: *mut gsl_interp_accel,
) -> libc::c_double {
    return gsl_interp2d_eval_deriv_yy(
        &(*interp).interp_object,
        (*interp).xarr as *const libc::c_double,
        (*interp).yarr as *const libc::c_double,
        (*interp).zarr as *const libc::c_double,
        x,
        y,
        xa,
        ya,
    );
}
pub unsafe extern "C" fn gsl_spline2d_eval_deriv_yy_e(
    mut interp: *const gsl_spline2d,
    x: libc::c_double,
    y: libc::c_double,
    mut xa: *mut gsl_interp_accel,
    mut ya: *mut gsl_interp_accel,
    mut z: *mut libc::c_double,
) -> libc::c_int {
    return gsl_interp2d_eval_deriv_yy_e(
        &(*interp).interp_object,
        (*interp).xarr as *const libc::c_double,
        (*interp).yarr as *const libc::c_double,
        (*interp).zarr as *const libc::c_double,
        x,
        y,
        xa,
        ya,
        z,
    );
}
pub unsafe extern "C" fn gsl_spline2d_eval_deriv_xy(
    mut interp: *const gsl_spline2d,
    x: libc::c_double,
    y: libc::c_double,
    mut xa: *mut gsl_interp_accel,
    mut ya: *mut gsl_interp_accel,
) -> libc::c_double {
    return gsl_interp2d_eval_deriv_xy(
        &(*interp).interp_object,
        (*interp).xarr as *const libc::c_double,
        (*interp).yarr as *const libc::c_double,
        (*interp).zarr as *const libc::c_double,
        x,
        y,
        xa,
        ya,
    );
}
pub unsafe extern "C" fn gsl_spline2d_eval_deriv_xy_e(
    mut interp: *const gsl_spline2d,
    x: libc::c_double,
    y: libc::c_double,
    mut xa: *mut gsl_interp_accel,
    mut ya: *mut gsl_interp_accel,
    mut z: *mut libc::c_double,
) -> libc::c_int {
    return gsl_interp2d_eval_deriv_xy_e(
        &(*interp).interp_object,
        (*interp).xarr as *const libc::c_double,
        (*interp).yarr as *const libc::c_double,
        (*interp).zarr as *const libc::c_double,
        x,
        y,
        xa,
        ya,
        z,
    );
}
pub unsafe extern "C" fn gsl_spline2d_min_size(
    mut interp: *const gsl_spline2d,
) -> size_t {
    return gsl_interp2d_min_size(&(*interp).interp_object);
}
pub unsafe extern "C" fn gsl_spline2d_name(
    mut interp: *const gsl_spline2d,
) -> *const libc::c_char {
    return gsl_interp2d_name(&(*interp).interp_object);
}
pub unsafe extern "C" fn gsl_spline2d_set(
    mut interp: *const gsl_spline2d,
    mut zarr: *mut libc::c_double,
    i: size_t,
    j: size_t,
    z: libc::c_double,
) -> libc::c_int {
    return gsl_interp2d_set(&(*interp).interp_object, zarr, i, j, z);
}
pub unsafe extern "C" fn gsl_spline2d_get(
    mut interp: *const gsl_spline2d,
    mut zarr: *const libc::c_double,
    i: size_t,
    j: size_t,
) -> libc::c_double {
    return gsl_interp2d_get(&(*interp).interp_object, zarr, i, j);
}
