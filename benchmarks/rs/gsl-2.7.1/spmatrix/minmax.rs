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
pub type C2RustUnnamed = libc::c_uint;
pub const GSL_SPMATRIX_CRS: C2RustUnnamed = 2;
pub const GSL_SPMATRIX_CCS: C2RustUnnamed = 1;
pub const GSL_SPMATRIX_TRIPLET: C2RustUnnamed = 0;
pub const GSL_SPMATRIX_CSR: C2RustUnnamed = 2;
pub const GSL_SPMATRIX_CSC: C2RustUnnamed = 1;
pub const GSL_SPMATRIX_COO: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_spmatrix_pool_node {
    pub next: *mut gsl_spmatrix_pool_node,
    pub block_ptr: *mut libc::c_void,
    pub free_slot: *mut libc::c_uchar,
}
pub type gsl_spmatrix_pool = gsl_spmatrix_pool_node;
pub type gsl_bst_cmp_function = unsafe extern "C" fn(
    *const libc::c_void,
    *const libc::c_void,
    *mut libc::c_void,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_bst_allocator {
    pub alloc: Option::<
        unsafe extern "C" fn(size_t, *mut libc::c_void) -> *mut libc::c_void,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_bst_avl_node {
    pub avl_link: [*mut gsl_bst_avl_node; 2],
    pub avl_data: *mut libc::c_void,
    pub avl_balance: libc::c_schar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_bst_avl_table {
    pub avl_root: *mut gsl_bst_avl_node,
    pub avl_compare: Option::<gsl_bst_cmp_function>,
    pub avl_param: *mut libc::c_void,
    pub avl_alloc: *const gsl_bst_allocator,
    pub avl_count: size_t,
    pub avl_generation: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_bst_rb_node {
    pub rb_link: [*mut gsl_bst_rb_node; 2],
    pub rb_data: *mut libc::c_void,
    pub rb_color: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_bst_rb_table {
    pub rb_root: *mut gsl_bst_rb_node,
    pub rb_compare: Option::<gsl_bst_cmp_function>,
    pub rb_param: *mut libc::c_void,
    pub rb_alloc: *const gsl_bst_allocator,
    pub rb_count: size_t,
    pub rb_generation: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_bst_type {
    pub name: *const libc::c_char,
    pub node_size: size_t,
    pub init: Option::<
        unsafe extern "C" fn(
            *const gsl_bst_allocator,
            Option::<gsl_bst_cmp_function>,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub nodes: Option::<unsafe extern "C" fn(*const libc::c_void) -> size_t>,
    pub insert: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> *mut libc::c_void,
    >,
    pub find: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            *const libc::c_void,
        ) -> *mut libc::c_void,
    >,
    pub remove: Option::<
        unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> *mut libc::c_void,
    >,
    pub empty: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub trav_init: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_void) -> libc::c_int,
    >,
    pub trav_first: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_void) -> *mut libc::c_void,
    >,
    pub trav_last: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_void) -> *mut libc::c_void,
    >,
    pub trav_find: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            *mut libc::c_void,
            *const libc::c_void,
        ) -> *mut libc::c_void,
    >,
    pub trav_insert: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> *mut libc::c_void,
    >,
    pub trav_copy: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_void) -> *mut libc::c_void,
    >,
    pub trav_next: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
    >,
    pub trav_prev: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
    >,
    pub trav_cur: Option::<
        unsafe extern "C" fn(*const libc::c_void) -> *mut libc::c_void,
    >,
    pub trav_replace: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> *mut libc::c_void,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_bst_workspace {
    pub type_0: *const gsl_bst_type,
    pub table: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub avl_table: gsl_bst_avl_table,
    pub rb_table: gsl_bst_rb_table,
}
pub type C2RustUnnamed_1 = libc::c_int;
pub const GSL_EOF: C2RustUnnamed_1 = 32;
pub const GSL_ETOLG: C2RustUnnamed_1 = 31;
pub const GSL_ETOLX: C2RustUnnamed_1 = 30;
pub const GSL_ETOLF: C2RustUnnamed_1 = 29;
pub const GSL_ENOPROGJ: C2RustUnnamed_1 = 28;
pub const GSL_ENOPROG: C2RustUnnamed_1 = 27;
pub const GSL_ETABLE: C2RustUnnamed_1 = 26;
pub const GSL_ECACHE: C2RustUnnamed_1 = 25;
pub const GSL_EUNIMPL: C2RustUnnamed_1 = 24;
pub const GSL_EUNSUP: C2RustUnnamed_1 = 23;
pub const GSL_EDIVERGE: C2RustUnnamed_1 = 22;
pub const GSL_ESING: C2RustUnnamed_1 = 21;
pub const GSL_ENOTSQR: C2RustUnnamed_1 = 20;
pub const GSL_EBADLEN: C2RustUnnamed_1 = 19;
pub const GSL_EROUND: C2RustUnnamed_1 = 18;
pub const GSL_ELOSS: C2RustUnnamed_1 = 17;
pub const GSL_EOVRFLW: C2RustUnnamed_1 = 16;
pub const GSL_EUNDRFLW: C2RustUnnamed_1 = 15;
pub const GSL_ETOL: C2RustUnnamed_1 = 14;
pub const GSL_EBADTOL: C2RustUnnamed_1 = 13;
pub const GSL_EZERODIV: C2RustUnnamed_1 = 12;
pub const GSL_EMAXITER: C2RustUnnamed_1 = 11;
pub const GSL_ERUNAWAY: C2RustUnnamed_1 = 10;
pub const GSL_EBADFUNC: C2RustUnnamed_1 = 9;
pub const GSL_ENOMEM: C2RustUnnamed_1 = 8;
pub const GSL_ESANITY: C2RustUnnamed_1 = 7;
pub const GSL_EFACTOR: C2RustUnnamed_1 = 6;
pub const GSL_EFAILED: C2RustUnnamed_1 = 5;
pub const GSL_EINVAL: C2RustUnnamed_1 = 4;
pub const GSL_EFAULT: C2RustUnnamed_1 = 3;
pub const GSL_ERANGE: C2RustUnnamed_1 = 2;
pub const GSL_EDOM: C2RustUnnamed_1 = 1;
pub const GSL_CONTINUE: C2RustUnnamed_1 = -2;
pub const GSL_FAILURE: C2RustUnnamed_1 = -1;
pub const GSL_SUCCESS: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_spmatrix_long_double {
    pub size1: size_t,
    pub size2: size_t,
    pub i: *mut libc::c_int,
    pub data: *mut f128::f128,
    pub p: *mut libc::c_int,
    pub nzmax: size_t,
    pub nz: size_t,
    pub tree: *mut gsl_bst_workspace,
    pub pool: *mut gsl_spmatrix_pool,
    pub node_size: size_t,
    pub work: C2RustUnnamed_2,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub work_void: *mut libc::c_void,
    pub work_int: *mut libc::c_int,
    pub work_atomic: *mut f128::f128,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_spmatrix {
    pub size1: size_t,
    pub size2: size_t,
    pub i: *mut libc::c_int,
    pub data: *mut libc::c_double,
    pub p: *mut libc::c_int,
    pub nzmax: size_t,
    pub nz: size_t,
    pub tree: *mut gsl_bst_workspace,
    pub pool: *mut gsl_spmatrix_pool,
    pub node_size: size_t,
    pub work: C2RustUnnamed_3,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub work_void: *mut libc::c_void,
    pub work_int: *mut libc::c_int,
    pub work_atomic: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_spmatrix_float {
    pub size1: size_t,
    pub size2: size_t,
    pub i: *mut libc::c_int,
    pub data: *mut libc::c_float,
    pub p: *mut libc::c_int,
    pub nzmax: size_t,
    pub nz: size_t,
    pub tree: *mut gsl_bst_workspace,
    pub pool: *mut gsl_spmatrix_pool,
    pub node_size: size_t,
    pub work: C2RustUnnamed_4,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub work_void: *mut libc::c_void,
    pub work_int: *mut libc::c_int,
    pub work_atomic: *mut libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_spmatrix_ulong {
    pub size1: size_t,
    pub size2: size_t,
    pub i: *mut libc::c_int,
    pub data: *mut libc::c_ulong,
    pub p: *mut libc::c_int,
    pub nzmax: size_t,
    pub nz: size_t,
    pub tree: *mut gsl_bst_workspace,
    pub pool: *mut gsl_spmatrix_pool,
    pub node_size: size_t,
    pub work: C2RustUnnamed_5,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub work_void: *mut libc::c_void,
    pub work_int: *mut libc::c_int,
    pub work_atomic: *mut libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_spmatrix_long {
    pub size1: size_t,
    pub size2: size_t,
    pub i: *mut libc::c_int,
    pub data: *mut libc::c_long,
    pub p: *mut libc::c_int,
    pub nzmax: size_t,
    pub nz: size_t,
    pub tree: *mut gsl_bst_workspace,
    pub pool: *mut gsl_spmatrix_pool,
    pub node_size: size_t,
    pub work: C2RustUnnamed_6,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub work_void: *mut libc::c_void,
    pub work_int: *mut libc::c_int,
    pub work_atomic: *mut libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_spmatrix_uint {
    pub size1: size_t,
    pub size2: size_t,
    pub i: *mut libc::c_int,
    pub data: *mut libc::c_uint,
    pub p: *mut libc::c_int,
    pub nzmax: size_t,
    pub nz: size_t,
    pub tree: *mut gsl_bst_workspace,
    pub pool: *mut gsl_spmatrix_pool,
    pub node_size: size_t,
    pub work: C2RustUnnamed_7,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub work_void: *mut libc::c_void,
    pub work_int: *mut libc::c_int,
    pub work_atomic: *mut libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_spmatrix_int {
    pub size1: size_t,
    pub size2: size_t,
    pub i: *mut libc::c_int,
    pub data: *mut libc::c_int,
    pub p: *mut libc::c_int,
    pub nzmax: size_t,
    pub nz: size_t,
    pub tree: *mut gsl_bst_workspace,
    pub pool: *mut gsl_spmatrix_pool,
    pub node_size: size_t,
    pub work: C2RustUnnamed_8,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub work_void: *mut libc::c_void,
    pub work_int: *mut libc::c_int,
    pub work_atomic: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_spmatrix_ushort {
    pub size1: size_t,
    pub size2: size_t,
    pub i: *mut libc::c_int,
    pub data: *mut libc::c_ushort,
    pub p: *mut libc::c_int,
    pub nzmax: size_t,
    pub nz: size_t,
    pub tree: *mut gsl_bst_workspace,
    pub pool: *mut gsl_spmatrix_pool,
    pub node_size: size_t,
    pub work: C2RustUnnamed_9,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub work_void: *mut libc::c_void,
    pub work_int: *mut libc::c_int,
    pub work_atomic: *mut libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_spmatrix_short {
    pub size1: size_t,
    pub size2: size_t,
    pub i: *mut libc::c_int,
    pub data: *mut libc::c_short,
    pub p: *mut libc::c_int,
    pub nzmax: size_t,
    pub nz: size_t,
    pub tree: *mut gsl_bst_workspace,
    pub pool: *mut gsl_spmatrix_pool,
    pub node_size: size_t,
    pub work: C2RustUnnamed_10,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub work_void: *mut libc::c_void,
    pub work_int: *mut libc::c_int,
    pub work_atomic: *mut libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_spmatrix_uchar {
    pub size1: size_t,
    pub size2: size_t,
    pub i: *mut libc::c_int,
    pub data: *mut libc::c_uchar,
    pub p: *mut libc::c_int,
    pub nzmax: size_t,
    pub nz: size_t,
    pub tree: *mut gsl_bst_workspace,
    pub pool: *mut gsl_spmatrix_pool,
    pub node_size: size_t,
    pub work: C2RustUnnamed_11,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub work_void: *mut libc::c_void,
    pub work_int: *mut libc::c_int,
    pub work_atomic: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_spmatrix_char {
    pub size1: size_t,
    pub size2: size_t,
    pub i: *mut libc::c_int,
    pub data: *mut libc::c_char,
    pub p: *mut libc::c_int,
    pub nzmax: size_t,
    pub nz: size_t,
    pub tree: *mut gsl_bst_workspace,
    pub pool: *mut gsl_spmatrix_pool,
    pub node_size: size_t,
    pub work: C2RustUnnamed_12,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub work_void: *mut libc::c_void,
    pub work_int: *mut libc::c_int,
    pub work_atomic: *mut libc::c_char,
}
pub unsafe extern "C" fn gsl_spmatrix_minmax(
    mut m: *const gsl_spmatrix,
    mut min_out: *mut libc::c_double,
    mut max_out: *mut libc::c_double,
) -> libc::c_int {
    let mut min: libc::c_double = 0.;
    let mut max: libc::c_double = 0.;
    let mut n: size_t = 0;
    if (*m).nz == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix is empty\0" as *const u8 as *const libc::c_char,
            b"./minmax_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    min = *((*m).data).offset(0 as libc::c_int as isize);
    max = *((*m).data).offset(0 as libc::c_int as isize);
    n = 1 as libc::c_int as size_t;
    while n < (*m).nz {
        let mut x: libc::c_double = *((*m).data).offset(n as isize);
        if x < min {
            min = x;
        }
        if x > max {
            max = x;
        }
        n = n.wrapping_add(1);
        n;
    }
    *min_out = min;
    *max_out = max;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_ushort_minmax(
    mut m: *const gsl_spmatrix_ushort,
    mut min_out: *mut libc::c_ushort,
    mut max_out: *mut libc::c_ushort,
) -> libc::c_int {
    let mut min: libc::c_ushort = 0;
    let mut max: libc::c_ushort = 0;
    let mut n: size_t = 0;
    if (*m).nz == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix is empty\0" as *const u8 as *const libc::c_char,
            b"./minmax_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    min = *((*m).data).offset(0 as libc::c_int as isize);
    max = *((*m).data).offset(0 as libc::c_int as isize);
    n = 1 as libc::c_int as size_t;
    while n < (*m).nz {
        let mut x: libc::c_ushort = *((*m).data).offset(n as isize);
        if (x as libc::c_int) < min as libc::c_int {
            min = x;
        }
        if x as libc::c_int > max as libc::c_int {
            max = x;
        }
        n = n.wrapping_add(1);
        n;
    }
    *min_out = min;
    *max_out = max;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_char_minmax(
    mut m: *const gsl_spmatrix_char,
    mut min_out: *mut libc::c_char,
    mut max_out: *mut libc::c_char,
) -> libc::c_int {
    let mut min: libc::c_char = 0;
    let mut max: libc::c_char = 0;
    let mut n: size_t = 0;
    if (*m).nz == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix is empty\0" as *const u8 as *const libc::c_char,
            b"./minmax_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    min = *((*m).data).offset(0 as libc::c_int as isize);
    max = *((*m).data).offset(0 as libc::c_int as isize);
    n = 1 as libc::c_int as size_t;
    while n < (*m).nz {
        let mut x: libc::c_char = *((*m).data).offset(n as isize);
        if (x as libc::c_int) < min as libc::c_int {
            min = x;
        }
        if x as libc::c_int > max as libc::c_int {
            max = x;
        }
        n = n.wrapping_add(1);
        n;
    }
    *min_out = min;
    *max_out = max;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_minmax(
    mut m: *const gsl_spmatrix_long_double,
    mut min_out: *mut f128::f128,
    mut max_out: *mut f128::f128,
) -> libc::c_int {
    let mut min: f128::f128 = f128::f128::ZERO;
    let mut max: f128::f128 = f128::f128::ZERO;
    let mut n: size_t = 0;
    if (*m).nz == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix is empty\0" as *const u8 as *const libc::c_char,
            b"./minmax_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    min = *((*m).data).offset(0 as libc::c_int as isize);
    max = *((*m).data).offset(0 as libc::c_int as isize);
    n = 1 as libc::c_int as size_t;
    while n < (*m).nz {
        let mut x: f128::f128 = *((*m).data).offset(n as isize);
        if x < min {
            min = x;
        }
        if x > max {
            max = x;
        }
        n = n.wrapping_add(1);
        n;
    }
    *min_out = min;
    *max_out = max;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_uchar_minmax(
    mut m: *const gsl_spmatrix_uchar,
    mut min_out: *mut libc::c_uchar,
    mut max_out: *mut libc::c_uchar,
) -> libc::c_int {
    let mut min: libc::c_uchar = 0;
    let mut max: libc::c_uchar = 0;
    let mut n: size_t = 0;
    if (*m).nz == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix is empty\0" as *const u8 as *const libc::c_char,
            b"./minmax_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    min = *((*m).data).offset(0 as libc::c_int as isize);
    max = *((*m).data).offset(0 as libc::c_int as isize);
    n = 1 as libc::c_int as size_t;
    while n < (*m).nz {
        let mut x: libc::c_uchar = *((*m).data).offset(n as isize);
        if (x as libc::c_int) < min as libc::c_int {
            min = x;
        }
        if x as libc::c_int > max as libc::c_int {
            max = x;
        }
        n = n.wrapping_add(1);
        n;
    }
    *min_out = min;
    *max_out = max;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_short_minmax(
    mut m: *const gsl_spmatrix_short,
    mut min_out: *mut libc::c_short,
    mut max_out: *mut libc::c_short,
) -> libc::c_int {
    let mut min: libc::c_short = 0;
    let mut max: libc::c_short = 0;
    let mut n: size_t = 0;
    if (*m).nz == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix is empty\0" as *const u8 as *const libc::c_char,
            b"./minmax_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    min = *((*m).data).offset(0 as libc::c_int as isize);
    max = *((*m).data).offset(0 as libc::c_int as isize);
    n = 1 as libc::c_int as size_t;
    while n < (*m).nz {
        let mut x: libc::c_short = *((*m).data).offset(n as isize);
        if (x as libc::c_int) < min as libc::c_int {
            min = x;
        }
        if x as libc::c_int > max as libc::c_int {
            max = x;
        }
        n = n.wrapping_add(1);
        n;
    }
    *min_out = min;
    *max_out = max;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_float_minmax(
    mut m: *const gsl_spmatrix_float,
    mut min_out: *mut libc::c_float,
    mut max_out: *mut libc::c_float,
) -> libc::c_int {
    let mut min: libc::c_float = 0.;
    let mut max: libc::c_float = 0.;
    let mut n: size_t = 0;
    if (*m).nz == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix is empty\0" as *const u8 as *const libc::c_char,
            b"./minmax_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    min = *((*m).data).offset(0 as libc::c_int as isize);
    max = *((*m).data).offset(0 as libc::c_int as isize);
    n = 1 as libc::c_int as size_t;
    while n < (*m).nz {
        let mut x: libc::c_float = *((*m).data).offset(n as isize);
        if x < min {
            min = x;
        }
        if x > max {
            max = x;
        }
        n = n.wrapping_add(1);
        n;
    }
    *min_out = min;
    *max_out = max;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_int_minmax(
    mut m: *const gsl_spmatrix_int,
    mut min_out: *mut libc::c_int,
    mut max_out: *mut libc::c_int,
) -> libc::c_int {
    let mut min: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    let mut n: size_t = 0;
    if (*m).nz == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix is empty\0" as *const u8 as *const libc::c_char,
            b"./minmax_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    min = *((*m).data).offset(0 as libc::c_int as isize);
    max = *((*m).data).offset(0 as libc::c_int as isize);
    n = 1 as libc::c_int as size_t;
    while n < (*m).nz {
        let mut x: libc::c_int = *((*m).data).offset(n as isize);
        if x < min {
            min = x;
        }
        if x > max {
            max = x;
        }
        n = n.wrapping_add(1);
        n;
    }
    *min_out = min;
    *max_out = max;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_ulong_minmax(
    mut m: *const gsl_spmatrix_ulong,
    mut min_out: *mut libc::c_ulong,
    mut max_out: *mut libc::c_ulong,
) -> libc::c_int {
    let mut min: libc::c_ulong = 0;
    let mut max: libc::c_ulong = 0;
    let mut n: size_t = 0;
    if (*m).nz == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix is empty\0" as *const u8 as *const libc::c_char,
            b"./minmax_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    min = *((*m).data).offset(0 as libc::c_int as isize);
    max = *((*m).data).offset(0 as libc::c_int as isize);
    n = 1 as libc::c_int as size_t;
    while n < (*m).nz {
        let mut x: libc::c_ulong = *((*m).data).offset(n as isize);
        if x < min {
            min = x;
        }
        if x > max {
            max = x;
        }
        n = n.wrapping_add(1);
        n;
    }
    *min_out = min;
    *max_out = max;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_uint_minmax(
    mut m: *const gsl_spmatrix_uint,
    mut min_out: *mut libc::c_uint,
    mut max_out: *mut libc::c_uint,
) -> libc::c_int {
    let mut min: libc::c_uint = 0;
    let mut max: libc::c_uint = 0;
    let mut n: size_t = 0;
    if (*m).nz == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix is empty\0" as *const u8 as *const libc::c_char,
            b"./minmax_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    min = *((*m).data).offset(0 as libc::c_int as isize);
    max = *((*m).data).offset(0 as libc::c_int as isize);
    n = 1 as libc::c_int as size_t;
    while n < (*m).nz {
        let mut x: libc::c_uint = *((*m).data).offset(n as isize);
        if x < min {
            min = x;
        }
        if x > max {
            max = x;
        }
        n = n.wrapping_add(1);
        n;
    }
    *min_out = min;
    *max_out = max;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_long_minmax(
    mut m: *const gsl_spmatrix_long,
    mut min_out: *mut libc::c_long,
    mut max_out: *mut libc::c_long,
) -> libc::c_int {
    let mut min: libc::c_long = 0;
    let mut max: libc::c_long = 0;
    let mut n: size_t = 0;
    if (*m).nz == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix is empty\0" as *const u8 as *const libc::c_char,
            b"./minmax_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    min = *((*m).data).offset(0 as libc::c_int as isize);
    max = *((*m).data).offset(0 as libc::c_int as isize);
    n = 1 as libc::c_int as size_t;
    while n < (*m).nz {
        let mut x: libc::c_long = *((*m).data).offset(n as isize);
        if x < min {
            min = x;
        }
        if x > max {
            max = x;
        }
        n = n.wrapping_add(1);
        n;
    }
    *min_out = min;
    *max_out = max;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_long_min_index(
    mut m: *const gsl_spmatrix_long,
    mut imin_out: *mut size_t,
    mut jmin_out: *mut size_t,
) -> libc::c_int {
    if (*m).nz == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix is empty\0" as *const u8 as *const libc::c_char,
            b"./minmax_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut Ad: *mut libc::c_long = (*m).data;
        let mut min: libc::c_long = *Ad.offset(0 as libc::c_int as isize);
        let mut imin: libc::c_int = 0 as libc::c_int;
        let mut jmin: libc::c_int = 0 as libc::c_int;
        let mut n: size_t = 0;
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            imin = *((*m).i).offset(0 as libc::c_int as isize);
            jmin = *((*m).p).offset(0 as libc::c_int as isize);
            n = 1 as libc::c_int as size_t;
            while n < (*m).nz {
                let mut x: libc::c_long = *((*m).data).offset(n as isize);
                if x < min {
                    min = x;
                    imin = *((*m).i).offset(n as isize);
                    jmin = *((*m).p).offset(n as isize);
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < (*m).size2 {
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    if *Ad.offset(p as isize) < min {
                        min = *Ad.offset(p as isize);
                        imin = *((*m).i).offset(p as isize);
                        jmin = j as libc::c_int;
                    }
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Ap_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).size1 {
                p_0 = *Ap_0.offset(i as isize);
                while p_0
                    < *Ap_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    if *Ad.offset(p_0 as isize) < min {
                        min = *Ad.offset(p_0 as isize);
                        imin = i as libc::c_int;
                        jmin = *((*m).i).offset(p_0 as isize);
                    }
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./minmax_source.c\0" as *const u8 as *const libc::c_char,
                125 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        *imin_out = imin as size_t;
        *jmin_out = jmin as size_t;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_ulong_min_index(
    mut m: *const gsl_spmatrix_ulong,
    mut imin_out: *mut size_t,
    mut jmin_out: *mut size_t,
) -> libc::c_int {
    if (*m).nz == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix is empty\0" as *const u8 as *const libc::c_char,
            b"./minmax_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut Ad: *mut libc::c_ulong = (*m).data;
        let mut min: libc::c_ulong = *Ad.offset(0 as libc::c_int as isize);
        let mut imin: libc::c_int = 0 as libc::c_int;
        let mut jmin: libc::c_int = 0 as libc::c_int;
        let mut n: size_t = 0;
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            imin = *((*m).i).offset(0 as libc::c_int as isize);
            jmin = *((*m).p).offset(0 as libc::c_int as isize);
            n = 1 as libc::c_int as size_t;
            while n < (*m).nz {
                let mut x: libc::c_ulong = *((*m).data).offset(n as isize);
                if x < min {
                    min = x;
                    imin = *((*m).i).offset(n as isize);
                    jmin = *((*m).p).offset(n as isize);
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < (*m).size2 {
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    if *Ad.offset(p as isize) < min {
                        min = *Ad.offset(p as isize);
                        imin = *((*m).i).offset(p as isize);
                        jmin = j as libc::c_int;
                    }
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Ap_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).size1 {
                p_0 = *Ap_0.offset(i as isize);
                while p_0
                    < *Ap_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    if *Ad.offset(p_0 as isize) < min {
                        min = *Ad.offset(p_0 as isize);
                        imin = i as libc::c_int;
                        jmin = *((*m).i).offset(p_0 as isize);
                    }
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./minmax_source.c\0" as *const u8 as *const libc::c_char,
                125 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        *imin_out = imin as size_t;
        *jmin_out = jmin as size_t;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uint_min_index(
    mut m: *const gsl_spmatrix_uint,
    mut imin_out: *mut size_t,
    mut jmin_out: *mut size_t,
) -> libc::c_int {
    if (*m).nz == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix is empty\0" as *const u8 as *const libc::c_char,
            b"./minmax_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut Ad: *mut libc::c_uint = (*m).data;
        let mut min: libc::c_uint = *Ad.offset(0 as libc::c_int as isize);
        let mut imin: libc::c_int = 0 as libc::c_int;
        let mut jmin: libc::c_int = 0 as libc::c_int;
        let mut n: size_t = 0;
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            imin = *((*m).i).offset(0 as libc::c_int as isize);
            jmin = *((*m).p).offset(0 as libc::c_int as isize);
            n = 1 as libc::c_int as size_t;
            while n < (*m).nz {
                let mut x: libc::c_uint = *((*m).data).offset(n as isize);
                if x < min {
                    min = x;
                    imin = *((*m).i).offset(n as isize);
                    jmin = *((*m).p).offset(n as isize);
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < (*m).size2 {
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    if *Ad.offset(p as isize) < min {
                        min = *Ad.offset(p as isize);
                        imin = *((*m).i).offset(p as isize);
                        jmin = j as libc::c_int;
                    }
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Ap_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).size1 {
                p_0 = *Ap_0.offset(i as isize);
                while p_0
                    < *Ap_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    if *Ad.offset(p_0 as isize) < min {
                        min = *Ad.offset(p_0 as isize);
                        imin = i as libc::c_int;
                        jmin = *((*m).i).offset(p_0 as isize);
                    }
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./minmax_source.c\0" as *const u8 as *const libc::c_char,
                125 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        *imin_out = imin as size_t;
        *jmin_out = jmin as size_t;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_float_min_index(
    mut m: *const gsl_spmatrix_float,
    mut imin_out: *mut size_t,
    mut jmin_out: *mut size_t,
) -> libc::c_int {
    if (*m).nz == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix is empty\0" as *const u8 as *const libc::c_char,
            b"./minmax_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut Ad: *mut libc::c_float = (*m).data;
        let mut min: libc::c_float = *Ad.offset(0 as libc::c_int as isize);
        let mut imin: libc::c_int = 0 as libc::c_int;
        let mut jmin: libc::c_int = 0 as libc::c_int;
        let mut n: size_t = 0;
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            imin = *((*m).i).offset(0 as libc::c_int as isize);
            jmin = *((*m).p).offset(0 as libc::c_int as isize);
            n = 1 as libc::c_int as size_t;
            while n < (*m).nz {
                let mut x: libc::c_float = *((*m).data).offset(n as isize);
                if x < min {
                    min = x;
                    imin = *((*m).i).offset(n as isize);
                    jmin = *((*m).p).offset(n as isize);
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < (*m).size2 {
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    if *Ad.offset(p as isize) < min {
                        min = *Ad.offset(p as isize);
                        imin = *((*m).i).offset(p as isize);
                        jmin = j as libc::c_int;
                    }
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Ap_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).size1 {
                p_0 = *Ap_0.offset(i as isize);
                while p_0
                    < *Ap_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    if *Ad.offset(p_0 as isize) < min {
                        min = *Ad.offset(p_0 as isize);
                        imin = i as libc::c_int;
                        jmin = *((*m).i).offset(p_0 as isize);
                    }
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./minmax_source.c\0" as *const u8 as *const libc::c_char,
                125 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        *imin_out = imin as size_t;
        *jmin_out = jmin as size_t;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_int_min_index(
    mut m: *const gsl_spmatrix_int,
    mut imin_out: *mut size_t,
    mut jmin_out: *mut size_t,
) -> libc::c_int {
    if (*m).nz == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix is empty\0" as *const u8 as *const libc::c_char,
            b"./minmax_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut Ad: *mut libc::c_int = (*m).data;
        let mut min: libc::c_int = *Ad.offset(0 as libc::c_int as isize);
        let mut imin: libc::c_int = 0 as libc::c_int;
        let mut jmin: libc::c_int = 0 as libc::c_int;
        let mut n: size_t = 0;
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            imin = *((*m).i).offset(0 as libc::c_int as isize);
            jmin = *((*m).p).offset(0 as libc::c_int as isize);
            n = 1 as libc::c_int as size_t;
            while n < (*m).nz {
                let mut x: libc::c_int = *((*m).data).offset(n as isize);
                if x < min {
                    min = x;
                    imin = *((*m).i).offset(n as isize);
                    jmin = *((*m).p).offset(n as isize);
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < (*m).size2 {
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    if *Ad.offset(p as isize) < min {
                        min = *Ad.offset(p as isize);
                        imin = *((*m).i).offset(p as isize);
                        jmin = j as libc::c_int;
                    }
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Ap_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).size1 {
                p_0 = *Ap_0.offset(i as isize);
                while p_0
                    < *Ap_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    if *Ad.offset(p_0 as isize) < min {
                        min = *Ad.offset(p_0 as isize);
                        imin = i as libc::c_int;
                        jmin = *((*m).i).offset(p_0 as isize);
                    }
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./minmax_source.c\0" as *const u8 as *const libc::c_char,
                125 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        *imin_out = imin as size_t;
        *jmin_out = jmin as size_t;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_ushort_min_index(
    mut m: *const gsl_spmatrix_ushort,
    mut imin_out: *mut size_t,
    mut jmin_out: *mut size_t,
) -> libc::c_int {
    if (*m).nz == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix is empty\0" as *const u8 as *const libc::c_char,
            b"./minmax_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut Ad: *mut libc::c_ushort = (*m).data;
        let mut min: libc::c_ushort = *Ad.offset(0 as libc::c_int as isize);
        let mut imin: libc::c_int = 0 as libc::c_int;
        let mut jmin: libc::c_int = 0 as libc::c_int;
        let mut n: size_t = 0;
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            imin = *((*m).i).offset(0 as libc::c_int as isize);
            jmin = *((*m).p).offset(0 as libc::c_int as isize);
            n = 1 as libc::c_int as size_t;
            while n < (*m).nz {
                let mut x: libc::c_ushort = *((*m).data).offset(n as isize);
                if (x as libc::c_int) < min as libc::c_int {
                    min = x;
                    imin = *((*m).i).offset(n as isize);
                    jmin = *((*m).p).offset(n as isize);
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < (*m).size2 {
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    if (*Ad.offset(p as isize) as libc::c_int) < min as libc::c_int {
                        min = *Ad.offset(p as isize);
                        imin = *((*m).i).offset(p as isize);
                        jmin = j as libc::c_int;
                    }
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Ap_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).size1 {
                p_0 = *Ap_0.offset(i as isize);
                while p_0
                    < *Ap_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    if (*Ad.offset(p_0 as isize) as libc::c_int) < min as libc::c_int {
                        min = *Ad.offset(p_0 as isize);
                        imin = i as libc::c_int;
                        jmin = *((*m).i).offset(p_0 as isize);
                    }
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./minmax_source.c\0" as *const u8 as *const libc::c_char,
                125 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        *imin_out = imin as size_t;
        *jmin_out = jmin as size_t;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_min_index(
    mut m: *const gsl_spmatrix_long_double,
    mut imin_out: *mut size_t,
    mut jmin_out: *mut size_t,
) -> libc::c_int {
    if (*m).nz == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix is empty\0" as *const u8 as *const libc::c_char,
            b"./minmax_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut Ad: *mut f128::f128 = (*m).data;
        let mut min: f128::f128 = *Ad.offset(0 as libc::c_int as isize);
        let mut imin: libc::c_int = 0 as libc::c_int;
        let mut jmin: libc::c_int = 0 as libc::c_int;
        let mut n: size_t = 0;
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            imin = *((*m).i).offset(0 as libc::c_int as isize);
            jmin = *((*m).p).offset(0 as libc::c_int as isize);
            n = 1 as libc::c_int as size_t;
            while n < (*m).nz {
                let mut x: f128::f128 = *((*m).data).offset(n as isize);
                if x < min {
                    min = x;
                    imin = *((*m).i).offset(n as isize);
                    jmin = *((*m).p).offset(n as isize);
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < (*m).size2 {
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    if *Ad.offset(p as isize) < min {
                        min = *Ad.offset(p as isize);
                        imin = *((*m).i).offset(p as isize);
                        jmin = j as libc::c_int;
                    }
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Ap_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).size1 {
                p_0 = *Ap_0.offset(i as isize);
                while p_0
                    < *Ap_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    if *Ad.offset(p_0 as isize) < min {
                        min = *Ad.offset(p_0 as isize);
                        imin = i as libc::c_int;
                        jmin = *((*m).i).offset(p_0 as isize);
                    }
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./minmax_source.c\0" as *const u8 as *const libc::c_char,
                125 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        *imin_out = imin as size_t;
        *jmin_out = jmin as size_t;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_min_index(
    mut m: *const gsl_spmatrix,
    mut imin_out: *mut size_t,
    mut jmin_out: *mut size_t,
) -> libc::c_int {
    if (*m).nz == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix is empty\0" as *const u8 as *const libc::c_char,
            b"./minmax_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut Ad: *mut libc::c_double = (*m).data;
        let mut min: libc::c_double = *Ad.offset(0 as libc::c_int as isize);
        let mut imin: libc::c_int = 0 as libc::c_int;
        let mut jmin: libc::c_int = 0 as libc::c_int;
        let mut n: size_t = 0;
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            imin = *((*m).i).offset(0 as libc::c_int as isize);
            jmin = *((*m).p).offset(0 as libc::c_int as isize);
            n = 1 as libc::c_int as size_t;
            while n < (*m).nz {
                let mut x: libc::c_double = *((*m).data).offset(n as isize);
                if x < min {
                    min = x;
                    imin = *((*m).i).offset(n as isize);
                    jmin = *((*m).p).offset(n as isize);
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < (*m).size2 {
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    if *Ad.offset(p as isize) < min {
                        min = *Ad.offset(p as isize);
                        imin = *((*m).i).offset(p as isize);
                        jmin = j as libc::c_int;
                    }
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Ap_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).size1 {
                p_0 = *Ap_0.offset(i as isize);
                while p_0
                    < *Ap_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    if *Ad.offset(p_0 as isize) < min {
                        min = *Ad.offset(p_0 as isize);
                        imin = i as libc::c_int;
                        jmin = *((*m).i).offset(p_0 as isize);
                    }
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./minmax_source.c\0" as *const u8 as *const libc::c_char,
                125 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        *imin_out = imin as size_t;
        *jmin_out = jmin as size_t;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_short_min_index(
    mut m: *const gsl_spmatrix_short,
    mut imin_out: *mut size_t,
    mut jmin_out: *mut size_t,
) -> libc::c_int {
    if (*m).nz == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix is empty\0" as *const u8 as *const libc::c_char,
            b"./minmax_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut Ad: *mut libc::c_short = (*m).data;
        let mut min: libc::c_short = *Ad.offset(0 as libc::c_int as isize);
        let mut imin: libc::c_int = 0 as libc::c_int;
        let mut jmin: libc::c_int = 0 as libc::c_int;
        let mut n: size_t = 0;
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            imin = *((*m).i).offset(0 as libc::c_int as isize);
            jmin = *((*m).p).offset(0 as libc::c_int as isize);
            n = 1 as libc::c_int as size_t;
            while n < (*m).nz {
                let mut x: libc::c_short = *((*m).data).offset(n as isize);
                if (x as libc::c_int) < min as libc::c_int {
                    min = x;
                    imin = *((*m).i).offset(n as isize);
                    jmin = *((*m).p).offset(n as isize);
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < (*m).size2 {
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    if (*Ad.offset(p as isize) as libc::c_int) < min as libc::c_int {
                        min = *Ad.offset(p as isize);
                        imin = *((*m).i).offset(p as isize);
                        jmin = j as libc::c_int;
                    }
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Ap_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).size1 {
                p_0 = *Ap_0.offset(i as isize);
                while p_0
                    < *Ap_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    if (*Ad.offset(p_0 as isize) as libc::c_int) < min as libc::c_int {
                        min = *Ad.offset(p_0 as isize);
                        imin = i as libc::c_int;
                        jmin = *((*m).i).offset(p_0 as isize);
                    }
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./minmax_source.c\0" as *const u8 as *const libc::c_char,
                125 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        *imin_out = imin as size_t;
        *jmin_out = jmin as size_t;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_char_min_index(
    mut m: *const gsl_spmatrix_char,
    mut imin_out: *mut size_t,
    mut jmin_out: *mut size_t,
) -> libc::c_int {
    if (*m).nz == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix is empty\0" as *const u8 as *const libc::c_char,
            b"./minmax_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut Ad: *mut libc::c_char = (*m).data;
        let mut min: libc::c_char = *Ad.offset(0 as libc::c_int as isize);
        let mut imin: libc::c_int = 0 as libc::c_int;
        let mut jmin: libc::c_int = 0 as libc::c_int;
        let mut n: size_t = 0;
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            imin = *((*m).i).offset(0 as libc::c_int as isize);
            jmin = *((*m).p).offset(0 as libc::c_int as isize);
            n = 1 as libc::c_int as size_t;
            while n < (*m).nz {
                let mut x: libc::c_char = *((*m).data).offset(n as isize);
                if (x as libc::c_int) < min as libc::c_int {
                    min = x;
                    imin = *((*m).i).offset(n as isize);
                    jmin = *((*m).p).offset(n as isize);
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < (*m).size2 {
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    if (*Ad.offset(p as isize) as libc::c_int) < min as libc::c_int {
                        min = *Ad.offset(p as isize);
                        imin = *((*m).i).offset(p as isize);
                        jmin = j as libc::c_int;
                    }
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Ap_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).size1 {
                p_0 = *Ap_0.offset(i as isize);
                while p_0
                    < *Ap_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    if (*Ad.offset(p_0 as isize) as libc::c_int) < min as libc::c_int {
                        min = *Ad.offset(p_0 as isize);
                        imin = i as libc::c_int;
                        jmin = *((*m).i).offset(p_0 as isize);
                    }
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./minmax_source.c\0" as *const u8 as *const libc::c_char,
                125 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        *imin_out = imin as size_t;
        *jmin_out = jmin as size_t;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uchar_min_index(
    mut m: *const gsl_spmatrix_uchar,
    mut imin_out: *mut size_t,
    mut jmin_out: *mut size_t,
) -> libc::c_int {
    if (*m).nz == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix is empty\0" as *const u8 as *const libc::c_char,
            b"./minmax_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut Ad: *mut libc::c_uchar = (*m).data;
        let mut min: libc::c_uchar = *Ad.offset(0 as libc::c_int as isize);
        let mut imin: libc::c_int = 0 as libc::c_int;
        let mut jmin: libc::c_int = 0 as libc::c_int;
        let mut n: size_t = 0;
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            imin = *((*m).i).offset(0 as libc::c_int as isize);
            jmin = *((*m).p).offset(0 as libc::c_int as isize);
            n = 1 as libc::c_int as size_t;
            while n < (*m).nz {
                let mut x: libc::c_uchar = *((*m).data).offset(n as isize);
                if (x as libc::c_int) < min as libc::c_int {
                    min = x;
                    imin = *((*m).i).offset(n as isize);
                    jmin = *((*m).p).offset(n as isize);
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < (*m).size2 {
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    if (*Ad.offset(p as isize) as libc::c_int) < min as libc::c_int {
                        min = *Ad.offset(p as isize);
                        imin = *((*m).i).offset(p as isize);
                        jmin = j as libc::c_int;
                    }
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Ap_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).size1 {
                p_0 = *Ap_0.offset(i as isize);
                while p_0
                    < *Ap_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    if (*Ad.offset(p_0 as isize) as libc::c_int) < min as libc::c_int {
                        min = *Ad.offset(p_0 as isize);
                        imin = i as libc::c_int;
                        jmin = *((*m).i).offset(p_0 as isize);
                    }
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./minmax_source.c\0" as *const u8 as *const libc::c_char,
                125 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        *imin_out = imin as size_t;
        *jmin_out = jmin as size_t;
        return GSL_SUCCESS as libc::c_int;
    };
}
