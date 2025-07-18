use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_subvector(
        v: *mut gsl_vector,
        i: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
    fn gsl_vector_const_subvector(
        v: *const gsl_vector,
        i: size_t,
        n: size_t,
    ) -> _gsl_vector_const_view;
    fn gsl_movstat_fill(
        endtype: gsl_movstat_end_t,
        x: *const gsl_vector,
        idx: size_t,
        H: size_t,
        J: size_t,
        window: *mut libc::c_double,
    ) -> size_t;
    fn gsl_movstat_apply_accum(
        endtype: gsl_movstat_end_t,
        x: *const gsl_vector,
        accum: *const gsl_movstat_accum,
        accum_params: *mut libc::c_void,
        y: *mut gsl_vector,
        z: *mut gsl_vector,
        w: *mut gsl_movstat_workspace,
    ) -> libc::c_int;
    fn gsl_movstat_free(w: *mut gsl_movstat_workspace);
    fn gsl_movstat_alloc_with_size(
        accum_state_size: size_t,
        H: size_t,
        J: size_t,
    ) -> *mut gsl_movstat_workspace;
    static mut gsl_movstat_accum_minmax: *const gsl_movstat_accum;
    fn gsl_stats_median(
        sorted_data: *mut libc::c_double,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
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
pub struct _gsl_vector_view {
    pub vector: gsl_vector,
}
pub type gsl_vector_view = _gsl_vector_view;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_vector_const_view {
    pub vector: gsl_vector,
}
pub type gsl_vector_const_view = _gsl_vector_const_view;
pub type gsl_movstat_end_t = libc::c_uint;
pub const GSL_MOVSTAT_END_TRUNCATE: gsl_movstat_end_t = 2;
pub const GSL_MOVSTAT_END_PADVALUE: gsl_movstat_end_t = 1;
pub const GSL_MOVSTAT_END_PADZERO: gsl_movstat_end_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_movstat_accum {
    pub size: Option::<unsafe extern "C" fn(size_t) -> size_t>,
    pub init: Option::<unsafe extern "C" fn(size_t, *mut libc::c_void) -> libc::c_int>,
    pub insert: Option::<
        unsafe extern "C" fn(libc::c_double, *mut libc::c_void) -> libc::c_int,
    >,
    pub delete_oldest: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub get: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_double,
            *const libc::c_void,
        ) -> libc::c_int,
    >,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_filter_rmedian_workspace {
    pub H: size_t,
    pub K: size_t,
    pub state: *mut libc::c_void,
    pub window: *mut libc::c_double,
    pub minmaxacc: *const gsl_movstat_accum,
    pub movstat_workspace_p: *mut gsl_movstat_workspace,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rmedian_state_t {
    pub minmax_acc: *const gsl_movstat_accum,
    pub minmax_state: *mut libc::c_void,
}
#[inline]
unsafe extern "C" fn gsl_vector_set(
    mut v: *mut gsl_vector,
    i: size_t,
    mut x: libc::c_double,
) {
    *((*v).data).offset(i.wrapping_mul((*v).stride) as isize) = x;
}
pub unsafe extern "C" fn gsl_filter_rmedian_alloc(
    K: size_t,
) -> *mut gsl_filter_rmedian_workspace {
    let mut w: *mut gsl_filter_rmedian_workspace = 0
        as *mut gsl_filter_rmedian_workspace;
    let mut state_size: size_t = 0;
    w = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gsl_filter_rmedian_workspace>() as libc::c_ulong,
    ) as *mut gsl_filter_rmedian_workspace;
    if w.is_null() {
        gsl_error(
            b"failed to allocate space for workspace\0" as *const u8
                as *const libc::c_char,
            b"rmedian.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_filter_rmedian_workspace;
    }
    (*w).H = K.wrapping_div(2 as libc::c_int as libc::c_ulong);
    (*w)
        .K = (2 as libc::c_int as libc::c_ulong)
        .wrapping_mul((*w).H)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    (*w).minmaxacc = gsl_movstat_accum_minmax;
    (*w)
        .window = malloc(
        ((*w).K).wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*w).window).is_null() {
        gsl_filter_rmedian_free(w);
        gsl_error(
            b"failed to allocate space for window\0" as *const u8 as *const libc::c_char,
            b"rmedian.c\0" as *const u8 as *const libc::c_char,
            74 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_filter_rmedian_workspace;
    }
    state_size = rmedian_size(((*w).H).wrapping_add(1 as libc::c_int as libc::c_ulong));
    (*w).state = malloc(state_size);
    if ((*w).state).is_null() {
        gsl_filter_rmedian_free(w);
        gsl_error(
            b"failed to allocate space for min/max state\0" as *const u8
                as *const libc::c_char,
            b"rmedian.c\0" as *const u8 as *const libc::c_char,
            83 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_filter_rmedian_workspace;
    }
    (*w)
        .movstat_workspace_p = gsl_movstat_alloc_with_size(
        state_size,
        0 as libc::c_int as size_t,
        (*w).H,
    );
    if ((*w).movstat_workspace_p).is_null() {
        gsl_filter_rmedian_free(w);
        gsl_error(
            b"failed to allocate space for movstat workspace\0" as *const u8
                as *const libc::c_char,
            b"rmedian.c\0" as *const u8 as *const libc::c_char,
            90 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_filter_rmedian_workspace;
    }
    return w;
}
pub unsafe extern "C" fn gsl_filter_rmedian_free(
    mut w: *mut gsl_filter_rmedian_workspace,
) {
    if !((*w).state).is_null() {
        free((*w).state);
    }
    if !((*w).window).is_null() {
        free((*w).window as *mut libc::c_void);
    }
    if !((*w).movstat_workspace_p).is_null() {
        gsl_movstat_free((*w).movstat_workspace_p);
    }
    free(w as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_filter_rmedian(
    endtype: gsl_filter_end_t,
    mut x: *const gsl_vector,
    mut y: *mut gsl_vector,
    mut w: *mut gsl_filter_rmedian_workspace,
) -> libc::c_int {
    if (*x).size != (*y).size {
        gsl_error(
            b"input and output vectors must have same length\0" as *const u8
                as *const libc::c_char,
            b"rmedian.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let n: size_t = (*x).size;
        let H: libc::c_int = (*w).H as libc::c_int;
        let mut yprev: libc::c_double = 0.;
        let mut wsize: libc::c_int = 0;
        wsize = gsl_movstat_fill(
            endtype as gsl_movstat_end_t,
            x,
            0 as libc::c_int as size_t,
            H as size_t,
            H as size_t,
            (*w).window,
        ) as libc::c_int;
        yprev = gsl_stats_median(
            (*w).window,
            1 as libc::c_int as size_t,
            wsize as size_t,
        );
        gsl_vector_set(y, 0 as libc::c_int as size_t, yprev);
        if (*x).size > 1 as libc::c_int as libc::c_ulong {
            let xv: gsl_vector_const_view = gsl_vector_const_subvector(
                x,
                1 as libc::c_int as size_t,
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            let mut yv: gsl_vector_view = gsl_vector_subvector(
                y,
                1 as libc::c_int as size_t,
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            status = gsl_movstat_apply_accum(
                endtype as gsl_movstat_end_t,
                &xv.vector,
                &rmedian_accum_type,
                &mut yprev as *mut libc::c_double as *mut libc::c_void,
                &mut yv.vector,
                0 as *mut gsl_vector,
                (*w).movstat_workspace_p,
            );
        }
        return status;
    };
}
unsafe extern "C" fn rmedian_size(n: size_t) -> size_t {
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut acc: *const gsl_movstat_accum = gsl_movstat_accum_minmax;
    size = (size as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<rmedian_state_t>() as libc::c_ulong)
        as size_t as size_t;
    size = (size as libc::c_ulong).wrapping_add(((*acc).size).unwrap()(n)) as size_t
        as size_t;
    return size;
}
unsafe extern "C" fn rmedian_init(
    n: size_t,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut state: *mut rmedian_state_t = vstate as *mut rmedian_state_t;
    (*state).minmax_acc = gsl_movstat_accum_minmax;
    (*state)
        .minmax_state = (vstate as *mut libc::c_uchar)
        .offset(::std::mem::size_of::<rmedian_state_t>() as libc::c_ulong as isize)
        as *mut libc::c_void;
    ((*(*state).minmax_acc).init).unwrap()(n, (*state).minmax_state);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn rmedian_insert(
    x: libc::c_double,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut state: *mut rmedian_state_t = vstate as *mut rmedian_state_t;
    return ((*(*state).minmax_acc).insert).unwrap()(x, (*state).minmax_state);
}
unsafe extern "C" fn rmedian_delete(mut vstate: *mut libc::c_void) -> libc::c_int {
    let mut state: *mut rmedian_state_t = vstate as *mut rmedian_state_t;
    return ((*(*state).minmax_acc).delete_oldest).unwrap()((*state).minmax_state);
}
unsafe extern "C" fn rmedian_get(
    mut params: *mut libc::c_void,
    mut result: *mut libc::c_double,
    mut vstate: *const libc::c_void,
) -> libc::c_int {
    let mut state: *const rmedian_state_t = vstate as *const rmedian_state_t;
    let mut yprev: *mut libc::c_double = params as *mut libc::c_double;
    let mut y: libc::c_double = 0.;
    let mut xminmax: [libc::c_double; 2] = [0.; 2];
    ((*(*state).minmax_acc).get)
        .unwrap()(0 as *mut libc::c_void, xminmax.as_mut_ptr(), (*state).minmax_state);
    if *yprev <= xminmax[0 as libc::c_int as usize] {
        y = xminmax[0 as libc::c_int as usize];
    } else if *yprev <= xminmax[1 as libc::c_int as usize] {
        y = *yprev;
    } else {
        y = xminmax[1 as libc::c_int as usize];
    }
    *result = y;
    *yprev = y;
    return GSL_SUCCESS as libc::c_int;
}
static mut rmedian_accum_type: gsl_movstat_accum = unsafe {
    {
        let mut init = gsl_movstat_accum {
            size: Some(rmedian_size as unsafe extern "C" fn(size_t) -> size_t),
            init: Some(
                rmedian_init
                    as unsafe extern "C" fn(size_t, *mut libc::c_void) -> libc::c_int,
            ),
            insert: Some(
                rmedian_insert
                    as unsafe extern "C" fn(
                        libc::c_double,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            delete_oldest: Some(
                rmedian_delete as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
            ),
            get: Some(
                rmedian_get
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_double,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        };
        init
    }
};
