use ::libc;
extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
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
pub unsafe extern "C" fn gsl_interp2d_alloc(
    mut T: *const gsl_interp2d_type,
    xsize: size_t,
    ysize: size_t,
) -> *mut gsl_interp2d {
    let mut interp: *mut gsl_interp2d = 0 as *mut gsl_interp2d;
    if xsize < (*T).min_size as libc::c_ulong || ysize < (*T).min_size as libc::c_ulong {
        gsl_error(
            b"insufficient number of points for interpolation type\0" as *const u8
                as *const libc::c_char,
            b"interp2d.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_interp2d;
    }
    interp = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gsl_interp2d>() as libc::c_ulong,
    ) as *mut gsl_interp2d;
    if interp.is_null() {
        gsl_error(
            b"failed to allocate space for gsl_interp2d struct\0" as *const u8
                as *const libc::c_char,
            b"interp2d.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_interp2d;
    }
    (*interp).type_0 = T;
    (*interp).xsize = xsize;
    (*interp).ysize = ysize;
    if ((*(*interp).type_0).alloc).is_none() {
        (*interp).state = 0 as *mut libc::c_void;
        return interp;
    }
    (*interp).state = ((*(*interp).type_0).alloc).unwrap()(xsize, ysize);
    if ((*interp).state).is_null() {
        free(interp as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for gsl_interp2d state\0" as *const u8
                as *const libc::c_char,
            b"interp2d.c\0" as *const u8 as *const libc::c_char,
            69 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_interp2d;
    }
    return interp;
}
pub unsafe extern "C" fn gsl_interp2d_free(mut interp: *mut gsl_interp2d) {
    if interp.is_null() {
        return;
    }
    if ((*(*interp).type_0).free).is_some() {
        ((*(*interp).type_0).free).unwrap()((*interp).state);
    }
    free(interp as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_interp2d_init(
    mut interp: *mut gsl_interp2d,
    mut xarr: *const libc::c_double,
    mut yarr: *const libc::c_double,
    mut zarr: *const libc::c_double,
    xsize: size_t,
    ysize: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    if xsize != (*interp).xsize || ysize != (*interp).ysize {
        gsl_error(
            b"data must match size of interpolation object\0" as *const u8
                as *const libc::c_char,
            b"interp2d.c\0" as *const u8 as *const libc::c_char,
            94 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    i = 1 as libc::c_int as size_t;
    while i < xsize {
        if *xarr.offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            >= *xarr.offset(i as isize)
        {
            gsl_error(
                b"x values must be strictly increasing\0" as *const u8
                    as *const libc::c_char,
                b"interp2d.c\0" as *const u8 as *const libc::c_char,
                101 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    i = 1 as libc::c_int as size_t;
    while i < ysize {
        if *yarr.offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            >= *yarr.offset(i as isize)
        {
            gsl_error(
                b"y values must be strictly increasing\0" as *const u8
                    as *const libc::c_char,
                b"interp2d.c\0" as *const u8 as *const libc::c_char,
                109 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    (*interp).xmin = *xarr.offset(0 as libc::c_int as isize);
    (*interp)
        .xmax = *xarr
        .offset(xsize.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
    (*interp).ymin = *yarr.offset(0 as libc::c_int as isize);
    (*interp)
        .ymax = *yarr
        .offset(ysize.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
    let mut status: libc::c_int = ((*(*interp).type_0).init)
        .unwrap()((*interp).state, xarr, yarr, zarr, xsize, ysize);
    return status;
}
unsafe extern "C" fn interp2d_eval(
    mut evaluator: Option::<
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
    mut interp: *const gsl_interp2d,
    mut xarr: *const libc::c_double,
    mut yarr: *const libc::c_double,
    mut zarr: *const libc::c_double,
    x: libc::c_double,
    y: libc::c_double,
    mut xa: *mut gsl_interp_accel,
    mut ya: *mut gsl_interp_accel,
    mut result: *mut libc::c_double,
) -> libc::c_int {
    if x < (*interp).xmin || x > (*interp).xmax {
        gsl_error(
            b"interpolation x value out of range\0" as *const u8 as *const libc::c_char,
            b"interp2d.c\0" as *const u8 as *const libc::c_char,
            143 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if y < (*interp).ymin || y > (*interp).ymax {
        gsl_error(
            b"interpolation y value out of range\0" as *const u8 as *const libc::c_char,
            b"interp2d.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    }
    return evaluator
        .unwrap()(
        (*interp).state,
        xarr,
        yarr,
        zarr,
        (*interp).xsize,
        (*interp).ysize,
        x,
        y,
        xa,
        ya,
        result,
    );
}
unsafe extern "C" fn interp2d_eval_extrap(
    mut evaluator: Option::<
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
    mut interp: *const gsl_interp2d,
    mut xarr: *const libc::c_double,
    mut yarr: *const libc::c_double,
    mut zarr: *const libc::c_double,
    x: libc::c_double,
    y: libc::c_double,
    mut xa: *mut gsl_interp_accel,
    mut ya: *mut gsl_interp_accel,
    mut result: *mut libc::c_double,
) -> libc::c_int {
    return evaluator
        .unwrap()(
        (*interp).state,
        xarr,
        yarr,
        zarr,
        (*interp).xsize,
        (*interp).ysize,
        x,
        y,
        xa,
        ya,
        result,
    );
}
pub unsafe extern "C" fn gsl_interp2d_eval(
    mut interp: *const gsl_interp2d,
    mut xarr: *const libc::c_double,
    mut yarr: *const libc::c_double,
    mut zarr: *const libc::c_double,
    x: libc::c_double,
    y: libc::c_double,
    mut xa: *mut gsl_interp_accel,
    mut ya: *mut gsl_interp_accel,
) -> libc::c_double {
    let mut z: libc::c_double = 0.;
    let mut status: libc::c_int = gsl_interp2d_eval_e(
        interp,
        xarr,
        yarr,
        zarr,
        x,
        y,
        xa,
        ya,
        &mut z,
    );
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"interpolation error\0" as *const u8 as *const libc::c_char,
            b"interp2d.c\0" as *const u8 as *const libc::c_char,
            185 as libc::c_int,
            status,
        );
        return ::std::f32::NAN as libc::c_double;
    }
    return z;
}
pub unsafe extern "C" fn gsl_interp2d_eval_extrap(
    mut interp: *const gsl_interp2d,
    mut xarr: *const libc::c_double,
    mut yarr: *const libc::c_double,
    mut zarr: *const libc::c_double,
    x: libc::c_double,
    y: libc::c_double,
    mut xa: *mut gsl_interp_accel,
    mut ya: *mut gsl_interp_accel,
) -> libc::c_double {
    let mut z: libc::c_double = 0.;
    let mut status: libc::c_int = interp2d_eval_extrap(
        (*(*interp).type_0).eval,
        interp,
        xarr,
        yarr,
        zarr,
        x,
        y,
        xa,
        ya,
        &mut z,
    );
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"interpolation error\0" as *const u8 as *const libc::c_char,
            b"interp2d.c\0" as *const u8 as *const libc::c_char,
            203 as libc::c_int,
            status,
        );
        return ::std::f32::NAN as libc::c_double;
    }
    return z;
}
pub unsafe extern "C" fn gsl_interp2d_eval_e(
    mut interp: *const gsl_interp2d,
    mut xarr: *const libc::c_double,
    mut yarr: *const libc::c_double,
    mut zarr: *const libc::c_double,
    x: libc::c_double,
    y: libc::c_double,
    mut xa: *mut gsl_interp_accel,
    mut ya: *mut gsl_interp_accel,
    mut z: *mut libc::c_double,
) -> libc::c_int {
    return interp2d_eval(
        (*(*interp).type_0).eval,
        interp,
        xarr,
        yarr,
        zarr,
        x,
        y,
        xa,
        ya,
        z,
    );
}
pub unsafe extern "C" fn gsl_interp2d_eval_e_extrap(
    mut interp: *const gsl_interp2d,
    mut xarr: *const libc::c_double,
    mut yarr: *const libc::c_double,
    mut zarr: *const libc::c_double,
    x: libc::c_double,
    y: libc::c_double,
    mut xa: *mut gsl_interp_accel,
    mut ya: *mut gsl_interp_accel,
    mut z: *mut libc::c_double,
) -> libc::c_int {
    return interp2d_eval_extrap(
        (*(*interp).type_0).eval,
        interp,
        xarr,
        yarr,
        zarr,
        x,
        y,
        xa,
        ya,
        z,
    );
}
pub unsafe extern "C" fn gsl_interp2d_eval_extrap_e(
    mut interp: *const gsl_interp2d,
    mut xarr: *const libc::c_double,
    mut yarr: *const libc::c_double,
    mut zarr: *const libc::c_double,
    x: libc::c_double,
    y: libc::c_double,
    mut xa: *mut gsl_interp_accel,
    mut ya: *mut gsl_interp_accel,
    mut z: *mut libc::c_double,
) -> libc::c_int {
    return interp2d_eval_extrap(
        (*(*interp).type_0).eval,
        interp,
        xarr,
        yarr,
        zarr,
        x,
        y,
        xa,
        ya,
        z,
    );
}
pub unsafe extern "C" fn gsl_interp2d_eval_deriv_x(
    mut interp: *const gsl_interp2d,
    mut xarr: *const libc::c_double,
    mut yarr: *const libc::c_double,
    mut zarr: *const libc::c_double,
    x: libc::c_double,
    y: libc::c_double,
    mut xa: *mut gsl_interp_accel,
    mut ya: *mut gsl_interp_accel,
) -> libc::c_double {
    let mut z: libc::c_double = 0.;
    let mut status: libc::c_int = gsl_interp2d_eval_deriv_x_e(
        interp,
        xarr,
        yarr,
        zarr,
        x,
        y,
        xa,
        ya,
        &mut z,
    );
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"interpolation error\0" as *const u8 as *const libc::c_char,
            b"interp2d.c\0" as *const u8 as *const libc::c_char,
            251 as libc::c_int,
            status,
        );
        return ::std::f32::NAN as libc::c_double;
    }
    return z;
}
pub unsafe extern "C" fn gsl_interp2d_eval_deriv_x_e(
    mut interp: *const gsl_interp2d,
    mut xarr: *const libc::c_double,
    mut yarr: *const libc::c_double,
    mut zarr: *const libc::c_double,
    x: libc::c_double,
    y: libc::c_double,
    mut xa: *mut gsl_interp_accel,
    mut ya: *mut gsl_interp_accel,
    mut z: *mut libc::c_double,
) -> libc::c_int {
    return interp2d_eval(
        (*(*interp).type_0).eval_deriv_x,
        interp,
        xarr,
        yarr,
        zarr,
        x,
        y,
        xa,
        ya,
        z,
    );
}
pub unsafe extern "C" fn gsl_interp2d_eval_deriv_y(
    mut interp: *const gsl_interp2d,
    mut xarr: *const libc::c_double,
    mut yarr: *const libc::c_double,
    mut zarr: *const libc::c_double,
    x: libc::c_double,
    y: libc::c_double,
    mut xa: *mut gsl_interp_accel,
    mut ya: *mut gsl_interp_accel,
) -> libc::c_double {
    let mut z: libc::c_double = 0.;
    let mut status: libc::c_int = gsl_interp2d_eval_deriv_y_e(
        interp,
        xarr,
        yarr,
        zarr,
        x,
        y,
        xa,
        ya,
        &mut z,
    );
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"interpolation error\0" as *const u8 as *const libc::c_char,
            b"interp2d.c\0" as *const u8 as *const libc::c_char,
            273 as libc::c_int,
            status,
        );
        return ::std::f32::NAN as libc::c_double;
    }
    return z;
}
pub unsafe extern "C" fn gsl_interp2d_eval_deriv_y_e(
    mut interp: *const gsl_interp2d,
    mut xarr: *const libc::c_double,
    mut yarr: *const libc::c_double,
    mut zarr: *const libc::c_double,
    x: libc::c_double,
    y: libc::c_double,
    mut xa: *mut gsl_interp_accel,
    mut ya: *mut gsl_interp_accel,
    mut z: *mut libc::c_double,
) -> libc::c_int {
    return interp2d_eval(
        (*(*interp).type_0).eval_deriv_y,
        interp,
        xarr,
        yarr,
        zarr,
        x,
        y,
        xa,
        ya,
        z,
    );
}
pub unsafe extern "C" fn gsl_interp2d_eval_deriv_xx(
    mut interp: *const gsl_interp2d,
    mut xarr: *const libc::c_double,
    mut yarr: *const libc::c_double,
    mut zarr: *const libc::c_double,
    x: libc::c_double,
    y: libc::c_double,
    mut xa: *mut gsl_interp_accel,
    mut ya: *mut gsl_interp_accel,
) -> libc::c_double {
    let mut z: libc::c_double = 0.;
    let mut status: libc::c_int = gsl_interp2d_eval_deriv_xx_e(
        interp,
        xarr,
        yarr,
        zarr,
        x,
        y,
        xa,
        ya,
        &mut z,
    );
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"interpolation error\0" as *const u8 as *const libc::c_char,
            b"interp2d.c\0" as *const u8 as *const libc::c_char,
            295 as libc::c_int,
            status,
        );
        return ::std::f32::NAN as libc::c_double;
    }
    return z;
}
pub unsafe extern "C" fn gsl_interp2d_eval_deriv_xx_e(
    mut interp: *const gsl_interp2d,
    mut xarr: *const libc::c_double,
    mut yarr: *const libc::c_double,
    mut zarr: *const libc::c_double,
    x: libc::c_double,
    y: libc::c_double,
    mut xa: *mut gsl_interp_accel,
    mut ya: *mut gsl_interp_accel,
    mut z: *mut libc::c_double,
) -> libc::c_int {
    return interp2d_eval(
        (*(*interp).type_0).eval_deriv_xx,
        interp,
        xarr,
        yarr,
        zarr,
        x,
        y,
        xa,
        ya,
        z,
    );
}
pub unsafe extern "C" fn gsl_interp2d_eval_deriv_yy(
    mut interp: *const gsl_interp2d,
    mut xarr: *const libc::c_double,
    mut yarr: *const libc::c_double,
    mut zarr: *const libc::c_double,
    x: libc::c_double,
    y: libc::c_double,
    mut xa: *mut gsl_interp_accel,
    mut ya: *mut gsl_interp_accel,
) -> libc::c_double {
    let mut z: libc::c_double = 0.;
    let mut status: libc::c_int = gsl_interp2d_eval_deriv_yy_e(
        interp,
        xarr,
        yarr,
        zarr,
        x,
        y,
        xa,
        ya,
        &mut z,
    );
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"interpolation error\0" as *const u8 as *const libc::c_char,
            b"interp2d.c\0" as *const u8 as *const libc::c_char,
            317 as libc::c_int,
            status,
        );
        return ::std::f32::NAN as libc::c_double;
    }
    return z;
}
pub unsafe extern "C" fn gsl_interp2d_eval_deriv_yy_e(
    mut interp: *const gsl_interp2d,
    mut xarr: *const libc::c_double,
    mut yarr: *const libc::c_double,
    mut zarr: *const libc::c_double,
    x: libc::c_double,
    y: libc::c_double,
    mut xa: *mut gsl_interp_accel,
    mut ya: *mut gsl_interp_accel,
    mut z: *mut libc::c_double,
) -> libc::c_int {
    return interp2d_eval(
        (*(*interp).type_0).eval_deriv_yy,
        interp,
        xarr,
        yarr,
        zarr,
        x,
        y,
        xa,
        ya,
        z,
    );
}
pub unsafe extern "C" fn gsl_interp2d_eval_deriv_xy(
    mut interp: *const gsl_interp2d,
    mut xarr: *const libc::c_double,
    mut yarr: *const libc::c_double,
    mut zarr: *const libc::c_double,
    x: libc::c_double,
    y: libc::c_double,
    mut xa: *mut gsl_interp_accel,
    mut ya: *mut gsl_interp_accel,
) -> libc::c_double {
    let mut z: libc::c_double = 0.;
    let mut status: libc::c_int = gsl_interp2d_eval_deriv_xy_e(
        interp,
        xarr,
        yarr,
        zarr,
        x,
        y,
        xa,
        ya,
        &mut z,
    );
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"interpolation error\0" as *const u8 as *const libc::c_char,
            b"interp2d.c\0" as *const u8 as *const libc::c_char,
            339 as libc::c_int,
            status,
        );
        return ::std::f32::NAN as libc::c_double;
    }
    return z;
}
pub unsafe extern "C" fn gsl_interp2d_eval_deriv_xy_e(
    mut interp: *const gsl_interp2d,
    mut xarr: *const libc::c_double,
    mut yarr: *const libc::c_double,
    mut zarr: *const libc::c_double,
    x: libc::c_double,
    y: libc::c_double,
    mut xa: *mut gsl_interp_accel,
    mut ya: *mut gsl_interp_accel,
    mut z: *mut libc::c_double,
) -> libc::c_int {
    return interp2d_eval(
        (*(*interp).type_0).eval_deriv_xy,
        interp,
        xarr,
        yarr,
        zarr,
        x,
        y,
        xa,
        ya,
        z,
    );
}
pub unsafe extern "C" fn gsl_interp2d_type_min_size(
    mut T: *const gsl_interp2d_type,
) -> size_t {
    return (*T).min_size as size_t;
}
pub unsafe extern "C" fn gsl_interp2d_min_size(
    mut interp: *const gsl_interp2d,
) -> size_t {
    return (*(*interp).type_0).min_size as size_t;
}
pub unsafe extern "C" fn gsl_interp2d_name(
    mut interp: *const gsl_interp2d,
) -> *const libc::c_char {
    return (*(*interp).type_0).name;
}
pub unsafe extern "C" fn gsl_interp2d_idx(
    mut interp: *const gsl_interp2d,
    i: size_t,
    j: size_t,
) -> size_t {
    if i >= (*interp).xsize {
        gsl_error(
            b"x index out of range\0" as *const u8 as *const libc::c_char,
            b"interp2d.c\0" as *const u8 as *const libc::c_char,
            377 as libc::c_int,
            GSL_ERANGE as libc::c_int,
        );
        return 0 as libc::c_int as size_t;
    } else if j >= (*interp).ysize {
        gsl_error(
            b"y index out of range\0" as *const u8 as *const libc::c_char,
            b"interp2d.c\0" as *const u8 as *const libc::c_char,
            381 as libc::c_int,
            GSL_ERANGE as libc::c_int,
        );
        return 0 as libc::c_int as size_t;
    } else {
        return j.wrapping_mul((*interp).xsize).wrapping_add(i)
    };
}
pub unsafe extern "C" fn gsl_interp2d_set(
    mut interp: *const gsl_interp2d,
    mut zarr: *mut libc::c_double,
    i: size_t,
    j: size_t,
    z: libc::c_double,
) -> libc::c_int {
    if i >= (*interp).xsize {
        gsl_error(
            b"x index out of range\0" as *const u8 as *const libc::c_char,
            b"interp2d.c\0" as *const u8 as *const libc::c_char,
            395 as libc::c_int,
            GSL_ERANGE as libc::c_int,
        );
        return GSL_ERANGE as libc::c_int;
    } else if j >= (*interp).ysize {
        gsl_error(
            b"y index out of range\0" as *const u8 as *const libc::c_char,
            b"interp2d.c\0" as *const u8 as *const libc::c_char,
            399 as libc::c_int,
            GSL_ERANGE as libc::c_int,
        );
        return GSL_ERANGE as libc::c_int;
    } else {
        *zarr.offset(j.wrapping_mul((*interp).xsize).wrapping_add(i) as isize) = z;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_interp2d_get(
    mut interp: *const gsl_interp2d,
    mut zarr: *const libc::c_double,
    i: size_t,
    j: size_t,
) -> libc::c_double {
    if i >= (*interp).xsize {
        gsl_error(
            b"x index out of range\0" as *const u8 as *const libc::c_char,
            b"interp2d.c\0" as *const u8 as *const libc::c_char,
            414 as libc::c_int,
            GSL_ERANGE as libc::c_int,
        );
        return 0 as libc::c_int as libc::c_double;
    } else if j >= (*interp).ysize {
        gsl_error(
            b"y index out of range\0" as *const u8 as *const libc::c_char,
            b"interp2d.c\0" as *const u8 as *const libc::c_char,
            418 as libc::c_int,
            GSL_ERANGE as libc::c_int,
        );
        return 0 as libc::c_int as libc::c_double;
    } else {
        return *zarr.offset(j.wrapping_mul((*interp).xsize).wrapping_add(i) as isize)
    };
}
