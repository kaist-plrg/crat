use ::libc;
use ::f128;
extern "C" {
    fn gsl_bst_insert(
        item: *mut libc::c_void,
        w: *mut gsl_bst_workspace,
    ) -> *mut libc::c_void;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_spmatrix_complex_long_double_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_complex_long_double,
    ) -> libc::c_int;
    fn gsl_spmatrix_complex_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_complex,
    ) -> libc::c_int;
    fn gsl_spmatrix_complex_float_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_complex_float,
    ) -> libc::c_int;
    fn gsl_spmatrix_long_double_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_long_double,
    ) -> libc::c_int;
    fn gsl_spmatrix_realloc(nzmax: size_t, m: *mut gsl_spmatrix) -> libc::c_int;
    fn gsl_spmatrix_float_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_float,
    ) -> libc::c_int;
    fn gsl_spmatrix_ulong_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_ulong,
    ) -> libc::c_int;
    fn gsl_spmatrix_char_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_char,
    ) -> libc::c_int;
    fn gsl_spmatrix_long_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_long,
    ) -> libc::c_int;
    fn gsl_spmatrix_uint_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_uint,
    ) -> libc::c_int;
    fn gsl_spmatrix_int_realloc(nzmax: size_t, m: *mut gsl_spmatrix_int) -> libc::c_int;
    fn gsl_spmatrix_ushort_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_ushort,
    ) -> libc::c_int;
    fn gsl_spmatrix_short_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_short,
    ) -> libc::c_int;
    fn gsl_spmatrix_uchar_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_uchar,
    ) -> libc::c_int;
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
pub struct gsl_complex {
    pub dat: [libc::c_double; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_complex_long_double {
    pub dat: [f128::f128; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_complex_float {
    pub dat: [libc::c_float; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_spmatrix_complex_long_double {
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
pub struct gsl_spmatrix_complex {
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
pub struct gsl_spmatrix_complex_float {
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
    pub work: C2RustUnnamed_5,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
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
    pub work: C2RustUnnamed_6,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
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
    pub work: C2RustUnnamed_7,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
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
    pub work: C2RustUnnamed_8,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
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
    pub work: C2RustUnnamed_9,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
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
    pub work: C2RustUnnamed_10,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
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
    pub work: C2RustUnnamed_11,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
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
    pub work: C2RustUnnamed_12,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
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
    pub work: C2RustUnnamed_13,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
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
    pub work: C2RustUnnamed_14,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_14 {
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
    pub work: C2RustUnnamed_15,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_15 {
    pub work_void: *mut libc::c_void,
    pub work_int: *mut libc::c_int,
    pub work_atomic: *mut libc::c_char,
}
pub unsafe extern "C" fn gsl_spmatrix_complex_get(
    mut m: *const gsl_spmatrix_complex,
    i: size_t,
    j: size_t,
) -> gsl_complex {
    let mut zero: gsl_complex = {
        let mut init = gsl_complex {
            dat: [0.0f64, 0.0f64],
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"first index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_complex_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return zero;
    } else if j >= (*m).size2 {
        gsl_error(
            b"second index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_complex_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return zero;
    } else if (*m).nz == 0 as libc::c_int as libc::c_ulong {
        return zero
    } else {
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = tree_complex_find(m, i, j);
            let mut x: gsl_complex = if !ptr.is_null() {
                *(ptr as *mut gsl_complex)
            } else {
                zero
            };
            return x;
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut mi: *const libc::c_int = (*m).i;
            let mut mp: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            p = *mp.offset(j as isize);
            while p
                < *mp.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mi.offset(p as isize) == i as libc::c_int {
                    return *(&mut *((*m).data).offset((2 as libc::c_int * p) as isize)
                        as *mut libc::c_double as *mut gsl_complex);
                }
                p += 1;
                p;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut mj: *const libc::c_int = (*m).i;
            let mut mp_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            p_0 = *mp_0.offset(i as isize);
            while p_0
                < *mp_0
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mj.offset(p_0 as isize) == j as libc::c_int {
                    return *(&mut *((*m).data).offset((2 as libc::c_int * p_0) as isize)
                        as *mut libc::c_double as *mut gsl_complex);
                }
                p_0 += 1;
                p_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./getset_complex_source.c\0" as *const u8 as *const libc::c_char,
                77 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return zero;
        }
        return zero;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_float_get(
    mut m: *const gsl_spmatrix_complex_float,
    i: size_t,
    j: size_t,
) -> gsl_complex_float {
    let mut zero: gsl_complex_float = {
        let mut init = gsl_complex_float {
            dat: [0.0f32, 0.0f32],
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"first index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_complex_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return zero;
    } else if j >= (*m).size2 {
        gsl_error(
            b"second index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_complex_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return zero;
    } else if (*m).nz == 0 as libc::c_int as libc::c_ulong {
        return zero
    } else {
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = tree_complex_float_find(m, i, j);
            let mut x: gsl_complex_float = if !ptr.is_null() {
                *(ptr as *mut gsl_complex_float)
            } else {
                zero
            };
            return x;
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut mi: *const libc::c_int = (*m).i;
            let mut mp: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            p = *mp.offset(j as isize);
            while p
                < *mp.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mi.offset(p as isize) == i as libc::c_int {
                    return *(&mut *((*m).data).offset((2 as libc::c_int * p) as isize)
                        as *mut libc::c_float as *mut gsl_complex_float);
                }
                p += 1;
                p;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut mj: *const libc::c_int = (*m).i;
            let mut mp_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            p_0 = *mp_0.offset(i as isize);
            while p_0
                < *mp_0
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mj.offset(p_0 as isize) == j as libc::c_int {
                    return *(&mut *((*m).data).offset((2 as libc::c_int * p_0) as isize)
                        as *mut libc::c_float as *mut gsl_complex_float);
                }
                p_0 += 1;
                p_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./getset_complex_source.c\0" as *const u8 as *const libc::c_char,
                77 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return zero;
        }
        return zero;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_long_double_get(
    mut m: *const gsl_spmatrix_complex_long_double,
    i: size_t,
    j: size_t,
) -> gsl_complex_long_double {
    let mut zero: gsl_complex_long_double = {
        let mut init = gsl_complex_long_double {
            dat: [f128::f128::new(0.0), f128::f128::new(0.0)],
        };
        init
    };
    if i >= (*m).size1 {
        gsl_error(
            b"first index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_complex_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return zero;
    } else if j >= (*m).size2 {
        gsl_error(
            b"second index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_complex_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return zero;
    } else if (*m).nz == 0 as libc::c_int as libc::c_ulong {
        return zero
    } else {
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = tree_complex_long_double_find(m, i, j);
            let mut x: gsl_complex_long_double = if !ptr.is_null() {
                *(ptr as *mut gsl_complex_long_double)
            } else {
                zero
            };
            return x;
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut mi: *const libc::c_int = (*m).i;
            let mut mp: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            p = *mp.offset(j as isize);
            while p
                < *mp.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mi.offset(p as isize) == i as libc::c_int {
                    return *(&mut *((*m).data).offset((2 as libc::c_int * p) as isize)
                        as *mut f128::f128 as *mut gsl_complex_long_double);
                }
                p += 1;
                p;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut mj: *const libc::c_int = (*m).i;
            let mut mp_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            p_0 = *mp_0.offset(i as isize);
            while p_0
                < *mp_0
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mj.offset(p_0 as isize) == j as libc::c_int {
                    return *(&mut *((*m).data).offset((2 as libc::c_int * p_0) as isize)
                        as *mut f128::f128 as *mut gsl_complex_long_double);
                }
                p_0 += 1;
                p_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./getset_complex_source.c\0" as *const u8 as *const libc::c_char,
                77 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return zero;
        }
        return zero;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_long_double_set(
    mut m: *mut gsl_spmatrix_complex_long_double,
    i: size_t,
    j: size_t,
    x: gsl_complex_long_double,
) -> libc::c_int {
    if !((*m).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"matrix not in COO representation\0" as *const u8 as *const libc::c_char,
            b"./getset_complex_source.c\0" as *const u8 as *const libc::c_char,
            91 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*m).spflags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
        == 0 && (i >= (*m).size1 || j >= (*m).size2)
    {
        gsl_error(
            b"indices out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_complex_source.c\0" as *const u8 as *const libc::c_char,
            95 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*m).spflags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong
        != 0
    {
        let mut ptr: *mut libc::c_void = tree_complex_long_double_find(m, i, j);
        if ptr.is_null() {
            gsl_error(
                b"attempt to add new matrix element to fixed sparsity pattern\0"
                    as *const u8 as *const libc::c_char,
                b"./getset_complex_source.c\0" as *const u8 as *const libc::c_char,
                107 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        } else {
            *(ptr as *mut gsl_complex_long_double) = x;
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut ptr_0: *mut libc::c_void = 0 as *mut libc::c_void;
        if (*m).nz >= (*m).nzmax {
            status = gsl_spmatrix_complex_long_double_realloc(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul((*m).nzmax),
                m,
            );
            if status != 0 {
                return status;
            }
        }
        *((*m).i).offset((*m).nz as isize) = i as libc::c_int;
        *((*m).p).offset((*m).nz as isize) = j as libc::c_int;
        *((*m).data)
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul((*m).nz) as isize,
            ) = x.dat[0 as libc::c_int as usize];
        *((*m).data)
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul((*m).nz)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = x.dat[1 as libc::c_int as usize];
        ptr_0 = gsl_bst_insert(
            &mut *((*m).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong).wrapping_mul((*m).nz) as isize,
                ) as *mut f128::f128 as *mut libc::c_void,
            (*m).tree,
        );
        if !ptr_0.is_null() {
            *(ptr_0 as *mut gsl_complex_long_double) = x;
        } else {
            if (*m).spflags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
                != 0
            {
                (*m)
                    .size1 = if (*m).size1
                    > i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    (*m).size1
                } else {
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                };
                (*m)
                    .size2 = if (*m).size2
                    > j.wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    (*m).size2
                } else {
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong)
                };
            }
            (*m).nz = ((*m).nz).wrapping_add(1);
            (*m).nz;
        }
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_float_set(
    mut m: *mut gsl_spmatrix_complex_float,
    i: size_t,
    j: size_t,
    x: gsl_complex_float,
) -> libc::c_int {
    if !((*m).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"matrix not in COO representation\0" as *const u8 as *const libc::c_char,
            b"./getset_complex_source.c\0" as *const u8 as *const libc::c_char,
            91 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*m).spflags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
        == 0 && (i >= (*m).size1 || j >= (*m).size2)
    {
        gsl_error(
            b"indices out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_complex_source.c\0" as *const u8 as *const libc::c_char,
            95 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*m).spflags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong
        != 0
    {
        let mut ptr: *mut libc::c_void = tree_complex_float_find(m, i, j);
        if ptr.is_null() {
            gsl_error(
                b"attempt to add new matrix element to fixed sparsity pattern\0"
                    as *const u8 as *const libc::c_char,
                b"./getset_complex_source.c\0" as *const u8 as *const libc::c_char,
                107 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        } else {
            *(ptr as *mut gsl_complex_float) = x;
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut ptr_0: *mut libc::c_void = 0 as *mut libc::c_void;
        if (*m).nz >= (*m).nzmax {
            status = gsl_spmatrix_complex_float_realloc(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul((*m).nzmax),
                m,
            );
            if status != 0 {
                return status;
            }
        }
        *((*m).i).offset((*m).nz as isize) = i as libc::c_int;
        *((*m).p).offset((*m).nz as isize) = j as libc::c_int;
        *((*m).data)
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul((*m).nz) as isize,
            ) = x.dat[0 as libc::c_int as usize];
        *((*m).data)
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul((*m).nz)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = x.dat[1 as libc::c_int as usize];
        ptr_0 = gsl_bst_insert(
            &mut *((*m).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong).wrapping_mul((*m).nz) as isize,
                ) as *mut libc::c_float as *mut libc::c_void,
            (*m).tree,
        );
        if !ptr_0.is_null() {
            *(ptr_0 as *mut gsl_complex_float) = x;
        } else {
            if (*m).spflags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
                != 0
            {
                (*m)
                    .size1 = if (*m).size1
                    > i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    (*m).size1
                } else {
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                };
                (*m)
                    .size2 = if (*m).size2
                    > j.wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    (*m).size2
                } else {
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong)
                };
            }
            (*m).nz = ((*m).nz).wrapping_add(1);
            (*m).nz;
        }
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_set(
    mut m: *mut gsl_spmatrix_complex,
    i: size_t,
    j: size_t,
    x: gsl_complex,
) -> libc::c_int {
    if !((*m).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"matrix not in COO representation\0" as *const u8 as *const libc::c_char,
            b"./getset_complex_source.c\0" as *const u8 as *const libc::c_char,
            91 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*m).spflags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
        == 0 && (i >= (*m).size1 || j >= (*m).size2)
    {
        gsl_error(
            b"indices out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_complex_source.c\0" as *const u8 as *const libc::c_char,
            95 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*m).spflags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong
        != 0
    {
        let mut ptr: *mut libc::c_void = tree_complex_find(m, i, j);
        if ptr.is_null() {
            gsl_error(
                b"attempt to add new matrix element to fixed sparsity pattern\0"
                    as *const u8 as *const libc::c_char,
                b"./getset_complex_source.c\0" as *const u8 as *const libc::c_char,
                107 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        } else {
            *(ptr as *mut gsl_complex) = x;
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut ptr_0: *mut libc::c_void = 0 as *mut libc::c_void;
        if (*m).nz >= (*m).nzmax {
            status = gsl_spmatrix_complex_realloc(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul((*m).nzmax),
                m,
            );
            if status != 0 {
                return status;
            }
        }
        *((*m).i).offset((*m).nz as isize) = i as libc::c_int;
        *((*m).p).offset((*m).nz as isize) = j as libc::c_int;
        *((*m).data)
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul((*m).nz) as isize,
            ) = x.dat[0 as libc::c_int as usize];
        *((*m).data)
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul((*m).nz)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = x.dat[1 as libc::c_int as usize];
        ptr_0 = gsl_bst_insert(
            &mut *((*m).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong).wrapping_mul((*m).nz) as isize,
                ) as *mut libc::c_double as *mut libc::c_void,
            (*m).tree,
        );
        if !ptr_0.is_null() {
            *(ptr_0 as *mut gsl_complex) = x;
        } else {
            if (*m).spflags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
                != 0
            {
                (*m)
                    .size1 = if (*m).size1
                    > i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    (*m).size1
                } else {
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                };
                (*m)
                    .size2 = if (*m).size2
                    > j.wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    (*m).size2
                } else {
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong)
                };
            }
            (*m).nz = ((*m).nz).wrapping_add(1);
            (*m).nz;
        }
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_ptr(
    mut m: *const gsl_spmatrix_complex,
    i: size_t,
    j: size_t,
) -> *mut gsl_complex {
    if i >= (*m).size1 {
        gsl_error(
            b"first index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_complex_source.c\0" as *const u8 as *const libc::c_char,
            164 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_complex;
    } else if j >= (*m).size2 {
        gsl_error(
            b"second index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_complex_source.c\0" as *const u8 as *const libc::c_char,
            168 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_complex;
    } else {
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = tree_complex_find(m, i, j);
            return ptr as *mut gsl_complex;
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut mi: *const libc::c_int = (*m).i;
            let mut mp: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            p = *mp.offset(j as isize);
            while p
                < *mp.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mi.offset(p as isize) == i as libc::c_int {
                    return &mut *((*m).data).offset((2 as libc::c_int * p) as isize)
                        as *mut libc::c_double as *mut gsl_complex;
                }
                p += 1;
                p;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut mj: *const libc::c_int = (*m).i;
            let mut mp_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            p_0 = *mp_0.offset(i as isize);
            while p_0
                < *mp_0
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mj.offset(p_0 as isize) == j as libc::c_int {
                    return &mut *((*m).data).offset((2 as libc::c_int * p_0) as isize)
                        as *mut libc::c_double as *mut gsl_complex;
                }
                p_0 += 1;
                p_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./getset_complex_source.c\0" as *const u8 as *const libc::c_char,
                206 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut gsl_complex;
        }
        return 0 as *mut gsl_complex;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_long_double_ptr(
    mut m: *const gsl_spmatrix_complex_long_double,
    i: size_t,
    j: size_t,
) -> *mut gsl_complex_long_double {
    if i >= (*m).size1 {
        gsl_error(
            b"first index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_complex_source.c\0" as *const u8 as *const libc::c_char,
            164 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_complex_long_double;
    } else if j >= (*m).size2 {
        gsl_error(
            b"second index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_complex_source.c\0" as *const u8 as *const libc::c_char,
            168 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_complex_long_double;
    } else {
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = tree_complex_long_double_find(m, i, j);
            return ptr as *mut gsl_complex_long_double;
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut mi: *const libc::c_int = (*m).i;
            let mut mp: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            p = *mp.offset(j as isize);
            while p
                < *mp.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mi.offset(p as isize) == i as libc::c_int {
                    return &mut *((*m).data).offset((2 as libc::c_int * p) as isize)
                        as *mut f128::f128 as *mut gsl_complex_long_double;
                }
                p += 1;
                p;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut mj: *const libc::c_int = (*m).i;
            let mut mp_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            p_0 = *mp_0.offset(i as isize);
            while p_0
                < *mp_0
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mj.offset(p_0 as isize) == j as libc::c_int {
                    return &mut *((*m).data).offset((2 as libc::c_int * p_0) as isize)
                        as *mut f128::f128 as *mut gsl_complex_long_double;
                }
                p_0 += 1;
                p_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./getset_complex_source.c\0" as *const u8 as *const libc::c_char,
                206 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut gsl_complex_long_double;
        }
        return 0 as *mut gsl_complex_long_double;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_float_ptr(
    mut m: *const gsl_spmatrix_complex_float,
    i: size_t,
    j: size_t,
) -> *mut gsl_complex_float {
    if i >= (*m).size1 {
        gsl_error(
            b"first index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_complex_source.c\0" as *const u8 as *const libc::c_char,
            164 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_complex_float;
    } else if j >= (*m).size2 {
        gsl_error(
            b"second index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_complex_source.c\0" as *const u8 as *const libc::c_char,
            168 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_complex_float;
    } else {
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = tree_complex_float_find(m, i, j);
            return ptr as *mut gsl_complex_float;
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut mi: *const libc::c_int = (*m).i;
            let mut mp: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            p = *mp.offset(j as isize);
            while p
                < *mp.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mi.offset(p as isize) == i as libc::c_int {
                    return &mut *((*m).data).offset((2 as libc::c_int * p) as isize)
                        as *mut libc::c_float as *mut gsl_complex_float;
                }
                p += 1;
                p;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut mj: *const libc::c_int = (*m).i;
            let mut mp_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            p_0 = *mp_0.offset(i as isize);
            while p_0
                < *mp_0
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mj.offset(p_0 as isize) == j as libc::c_int {
                    return &mut *((*m).data).offset((2 as libc::c_int * p_0) as isize)
                        as *mut libc::c_float as *mut gsl_complex_float;
                }
                p_0 += 1;
                p_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./getset_complex_source.c\0" as *const u8 as *const libc::c_char,
                206 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut gsl_complex_float;
        }
        return 0 as *mut gsl_complex_float;
    };
}
unsafe extern "C" fn tree_complex_float_find(
    mut m: *const gsl_spmatrix_complex_float,
    i: size_t,
    j: size_t,
) -> *mut libc::c_void {
    let mut table: *const gsl_bst_avl_table = &mut (*(*m).tree).table.avl_table
        as *mut gsl_bst_avl_table as *const gsl_bst_avl_table;
    let mut p: *mut gsl_bst_avl_node = 0 as *mut gsl_bst_avl_node;
    p = (*table).avl_root;
    while !p.is_null() {
        let mut n: size_t = (((*p).avl_data as *mut libc::c_float).offset_from((*m).data)
            as libc::c_long >> 1 as libc::c_int) as size_t;
        let mut pi: libc::c_int = *((*m).i).offset(n as isize);
        let mut pj: libc::c_int = *((*m).p).offset(n as isize);
        let mut cmp: libc::c_int = if (i as libc::c_int) < pi {
            -(1 as libc::c_int)
        } else if i as libc::c_int > pi {
            1 as libc::c_int
        } else if (j as libc::c_int) < pj {
            -(1 as libc::c_int)
        } else {
            (j as libc::c_int > pj) as libc::c_int
        };
        if cmp < 0 as libc::c_int {
            p = (*p).avl_link[0 as libc::c_int as usize];
        } else if cmp > 0 as libc::c_int {
            p = (*p).avl_link[1 as libc::c_int as usize];
        } else {
            return (*p).avl_data
        }
    }
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn tree_complex_find(
    mut m: *const gsl_spmatrix_complex,
    i: size_t,
    j: size_t,
) -> *mut libc::c_void {
    let mut table: *const gsl_bst_avl_table = &mut (*(*m).tree).table.avl_table
        as *mut gsl_bst_avl_table as *const gsl_bst_avl_table;
    let mut p: *mut gsl_bst_avl_node = 0 as *mut gsl_bst_avl_node;
    p = (*table).avl_root;
    while !p.is_null() {
        let mut n: size_t = (((*p).avl_data as *mut libc::c_double)
            .offset_from((*m).data) as libc::c_long >> 1 as libc::c_int) as size_t;
        let mut pi: libc::c_int = *((*m).i).offset(n as isize);
        let mut pj: libc::c_int = *((*m).p).offset(n as isize);
        let mut cmp: libc::c_int = if (i as libc::c_int) < pi {
            -(1 as libc::c_int)
        } else if i as libc::c_int > pi {
            1 as libc::c_int
        } else if (j as libc::c_int) < pj {
            -(1 as libc::c_int)
        } else {
            (j as libc::c_int > pj) as libc::c_int
        };
        if cmp < 0 as libc::c_int {
            p = (*p).avl_link[0 as libc::c_int as usize];
        } else if cmp > 0 as libc::c_int {
            p = (*p).avl_link[1 as libc::c_int as usize];
        } else {
            return (*p).avl_data
        }
    }
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn tree_complex_long_double_find(
    mut m: *const gsl_spmatrix_complex_long_double,
    i: size_t,
    j: size_t,
) -> *mut libc::c_void {
    let mut table: *const gsl_bst_avl_table = &mut (*(*m).tree).table.avl_table
        as *mut gsl_bst_avl_table as *const gsl_bst_avl_table;
    let mut p: *mut gsl_bst_avl_node = 0 as *mut gsl_bst_avl_node;
    p = (*table).avl_root;
    while !p.is_null() {
        let mut n: size_t = (((*p).avl_data as *mut f128::f128).offset_from((*m).data)
            as libc::c_long >> 1 as libc::c_int) as size_t;
        let mut pi: libc::c_int = *((*m).i).offset(n as isize);
        let mut pj: libc::c_int = *((*m).p).offset(n as isize);
        let mut cmp: libc::c_int = if (i as libc::c_int) < pi {
            -(1 as libc::c_int)
        } else if i as libc::c_int > pi {
            1 as libc::c_int
        } else if (j as libc::c_int) < pj {
            -(1 as libc::c_int)
        } else {
            (j as libc::c_int > pj) as libc::c_int
        };
        if cmp < 0 as libc::c_int {
            p = (*p).avl_link[0 as libc::c_int as usize];
        } else if cmp > 0 as libc::c_int {
            p = (*p).avl_link[1 as libc::c_int as usize];
        } else {
            return (*p).avl_data
        }
    }
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn gsl_spmatrix_char_get(
    mut m: *const gsl_spmatrix_char,
    i: size_t,
    j: size_t,
) -> libc::c_char {
    if i >= (*m).size1 {
        gsl_error(
            b"first index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int as libc::c_char;
    } else if j >= (*m).size2 {
        gsl_error(
            b"second index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int as libc::c_char;
    } else if (*m).nz == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as libc::c_char
    } else {
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = tree_char_find(m, i, j);
            let mut x: libc::c_char = (if !ptr.is_null() {
                *(ptr as *mut libc::c_char) as libc::c_int
            } else {
                0 as libc::c_int as libc::c_char as libc::c_int
            }) as libc::c_char;
            return x;
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut mi: *const libc::c_int = (*m).i;
            let mut mp: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            p = *mp.offset(j as isize);
            while p
                < *mp.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mi.offset(p as isize) == i as libc::c_int {
                    return *((*m).data).offset(p as isize);
                }
                p += 1;
                p;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut mj: *const libc::c_int = (*m).i;
            let mut mp_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            p_0 = *mp_0.offset(i as isize);
            while p_0
                < *mp_0
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mj.offset(p_0 as isize) == j as libc::c_int {
                    return *((*m).data).offset(p_0 as isize);
                }
                p_0 += 1;
                p_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./getset_source.c\0" as *const u8 as *const libc::c_char,
                75 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int as libc::c_char;
        }
        return 0 as libc::c_int as libc::c_char;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_long_get(
    mut m: *const gsl_spmatrix_long,
    i: size_t,
    j: size_t,
) -> libc::c_long {
    if i >= (*m).size1 {
        gsl_error(
            b"first index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int as libc::c_long;
    } else if j >= (*m).size2 {
        gsl_error(
            b"second index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int as libc::c_long;
    } else if (*m).nz == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as libc::c_long
    } else {
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = tree_long_find(m, i, j);
            let mut x: libc::c_long = if !ptr.is_null() {
                *(ptr as *mut libc::c_long)
            } else {
                0 as libc::c_int as libc::c_long
            };
            return x;
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut mi: *const libc::c_int = (*m).i;
            let mut mp: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            p = *mp.offset(j as isize);
            while p
                < *mp.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mi.offset(p as isize) == i as libc::c_int {
                    return *((*m).data).offset(p as isize);
                }
                p += 1;
                p;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut mj: *const libc::c_int = (*m).i;
            let mut mp_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            p_0 = *mp_0.offset(i as isize);
            while p_0
                < *mp_0
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mj.offset(p_0 as isize) == j as libc::c_int {
                    return *((*m).data).offset(p_0 as isize);
                }
                p_0 += 1;
                p_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./getset_source.c\0" as *const u8 as *const libc::c_char,
                75 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int as libc::c_long;
        }
        return 0 as libc::c_int as libc::c_long;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_get(
    mut m: *const gsl_spmatrix,
    i: size_t,
    j: size_t,
) -> libc::c_double {
    if i >= (*m).size1 {
        gsl_error(
            b"first index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int as libc::c_double;
    } else if j >= (*m).size2 {
        gsl_error(
            b"second index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int as libc::c_double;
    } else if (*m).nz == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as libc::c_double
    } else {
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = tree_find(m, i, j);
            let mut x: libc::c_double = if !ptr.is_null() {
                *(ptr as *mut libc::c_double)
            } else {
                0 as libc::c_int as libc::c_double
            };
            return x;
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut mi: *const libc::c_int = (*m).i;
            let mut mp: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            p = *mp.offset(j as isize);
            while p
                < *mp.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mi.offset(p as isize) == i as libc::c_int {
                    return *((*m).data).offset(p as isize);
                }
                p += 1;
                p;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut mj: *const libc::c_int = (*m).i;
            let mut mp_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            p_0 = *mp_0.offset(i as isize);
            while p_0
                < *mp_0
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mj.offset(p_0 as isize) == j as libc::c_int {
                    return *((*m).data).offset(p_0 as isize);
                }
                p_0 += 1;
                p_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./getset_source.c\0" as *const u8 as *const libc::c_char,
                75 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int as libc::c_double;
        }
        return 0 as libc::c_int as libc::c_double;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uchar_get(
    mut m: *const gsl_spmatrix_uchar,
    i: size_t,
    j: size_t,
) -> libc::c_uchar {
    if i >= (*m).size1 {
        gsl_error(
            b"first index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int as libc::c_uchar;
    } else if j >= (*m).size2 {
        gsl_error(
            b"second index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int as libc::c_uchar;
    } else if (*m).nz == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as libc::c_uchar
    } else {
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = tree_uchar_find(m, i, j);
            let mut x: libc::c_uchar = (if !ptr.is_null() {
                *(ptr as *mut libc::c_uchar) as libc::c_int
            } else {
                0 as libc::c_int as libc::c_uchar as libc::c_int
            }) as libc::c_uchar;
            return x;
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut mi: *const libc::c_int = (*m).i;
            let mut mp: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            p = *mp.offset(j as isize);
            while p
                < *mp.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mi.offset(p as isize) == i as libc::c_int {
                    return *((*m).data).offset(p as isize);
                }
                p += 1;
                p;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut mj: *const libc::c_int = (*m).i;
            let mut mp_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            p_0 = *mp_0.offset(i as isize);
            while p_0
                < *mp_0
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mj.offset(p_0 as isize) == j as libc::c_int {
                    return *((*m).data).offset(p_0 as isize);
                }
                p_0 += 1;
                p_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./getset_source.c\0" as *const u8 as *const libc::c_char,
                75 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int as libc::c_uchar;
        }
        return 0 as libc::c_int as libc::c_uchar;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_ulong_get(
    mut m: *const gsl_spmatrix_ulong,
    i: size_t,
    j: size_t,
) -> libc::c_ulong {
    if i >= (*m).size1 {
        gsl_error(
            b"first index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int as libc::c_ulong;
    } else if j >= (*m).size2 {
        gsl_error(
            b"second index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int as libc::c_ulong;
    } else if (*m).nz == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as libc::c_ulong
    } else {
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = tree_ulong_find(m, i, j);
            let mut x: libc::c_ulong = if !ptr.is_null() {
                *(ptr as *mut libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            };
            return x;
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut mi: *const libc::c_int = (*m).i;
            let mut mp: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            p = *mp.offset(j as isize);
            while p
                < *mp.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mi.offset(p as isize) == i as libc::c_int {
                    return *((*m).data).offset(p as isize);
                }
                p += 1;
                p;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut mj: *const libc::c_int = (*m).i;
            let mut mp_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            p_0 = *mp_0.offset(i as isize);
            while p_0
                < *mp_0
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mj.offset(p_0 as isize) == j as libc::c_int {
                    return *((*m).data).offset(p_0 as isize);
                }
                p_0 += 1;
                p_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./getset_source.c\0" as *const u8 as *const libc::c_char,
                75 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int as libc::c_ulong;
        }
        return 0 as libc::c_int as libc::c_ulong;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_short_get(
    mut m: *const gsl_spmatrix_short,
    i: size_t,
    j: size_t,
) -> libc::c_short {
    if i >= (*m).size1 {
        gsl_error(
            b"first index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int as libc::c_short;
    } else if j >= (*m).size2 {
        gsl_error(
            b"second index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int as libc::c_short;
    } else if (*m).nz == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as libc::c_short
    } else {
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = tree_short_find(m, i, j);
            let mut x: libc::c_short = (if !ptr.is_null() {
                *(ptr as *mut libc::c_short) as libc::c_int
            } else {
                0 as libc::c_int as libc::c_short as libc::c_int
            }) as libc::c_short;
            return x;
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut mi: *const libc::c_int = (*m).i;
            let mut mp: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            p = *mp.offset(j as isize);
            while p
                < *mp.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mi.offset(p as isize) == i as libc::c_int {
                    return *((*m).data).offset(p as isize);
                }
                p += 1;
                p;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut mj: *const libc::c_int = (*m).i;
            let mut mp_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            p_0 = *mp_0.offset(i as isize);
            while p_0
                < *mp_0
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mj.offset(p_0 as isize) == j as libc::c_int {
                    return *((*m).data).offset(p_0 as isize);
                }
                p_0 += 1;
                p_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./getset_source.c\0" as *const u8 as *const libc::c_char,
                75 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int as libc::c_short;
        }
        return 0 as libc::c_int as libc::c_short;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_get(
    mut m: *const gsl_spmatrix_long_double,
    i: size_t,
    j: size_t,
) -> f128::f128 {
    if i >= (*m).size1 {
        gsl_error(
            b"first index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return f128::f128::new(0 as libc::c_int);
    } else if j >= (*m).size2 {
        gsl_error(
            b"second index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return f128::f128::new(0 as libc::c_int);
    } else if (*m).nz == 0 as libc::c_int as libc::c_ulong {
        return f128::f128::new(0 as libc::c_int)
    } else {
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = tree_long_double_find(m, i, j);
            let mut x: f128::f128 = if !ptr.is_null() {
                *(ptr as *mut f128::f128)
            } else {
                f128::f128::new(0 as libc::c_int)
            };
            return x;
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut mi: *const libc::c_int = (*m).i;
            let mut mp: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            p = *mp.offset(j as isize);
            while p
                < *mp.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mi.offset(p as isize) == i as libc::c_int {
                    return *((*m).data).offset(p as isize);
                }
                p += 1;
                p;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut mj: *const libc::c_int = (*m).i;
            let mut mp_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            p_0 = *mp_0.offset(i as isize);
            while p_0
                < *mp_0
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mj.offset(p_0 as isize) == j as libc::c_int {
                    return *((*m).data).offset(p_0 as isize);
                }
                p_0 += 1;
                p_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./getset_source.c\0" as *const u8 as *const libc::c_char,
                75 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return f128::f128::new(0 as libc::c_int);
        }
        return f128::f128::new(0 as libc::c_int);
    };
}
pub unsafe extern "C" fn gsl_spmatrix_float_get(
    mut m: *const gsl_spmatrix_float,
    i: size_t,
    j: size_t,
) -> libc::c_float {
    if i >= (*m).size1 {
        gsl_error(
            b"first index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int as libc::c_float;
    } else if j >= (*m).size2 {
        gsl_error(
            b"second index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int as libc::c_float;
    } else if (*m).nz == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as libc::c_float
    } else {
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = tree_float_find(m, i, j);
            let mut x: libc::c_float = if !ptr.is_null() {
                *(ptr as *mut libc::c_float)
            } else {
                0 as libc::c_int as libc::c_float
            };
            return x;
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut mi: *const libc::c_int = (*m).i;
            let mut mp: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            p = *mp.offset(j as isize);
            while p
                < *mp.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mi.offset(p as isize) == i as libc::c_int {
                    return *((*m).data).offset(p as isize);
                }
                p += 1;
                p;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut mj: *const libc::c_int = (*m).i;
            let mut mp_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            p_0 = *mp_0.offset(i as isize);
            while p_0
                < *mp_0
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mj.offset(p_0 as isize) == j as libc::c_int {
                    return *((*m).data).offset(p_0 as isize);
                }
                p_0 += 1;
                p_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./getset_source.c\0" as *const u8 as *const libc::c_char,
                75 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int as libc::c_float;
        }
        return 0 as libc::c_int as libc::c_float;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_ushort_get(
    mut m: *const gsl_spmatrix_ushort,
    i: size_t,
    j: size_t,
) -> libc::c_ushort {
    if i >= (*m).size1 {
        gsl_error(
            b"first index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int as libc::c_ushort;
    } else if j >= (*m).size2 {
        gsl_error(
            b"second index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int as libc::c_ushort;
    } else if (*m).nz == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as libc::c_ushort
    } else {
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = tree_ushort_find(m, i, j);
            let mut x: libc::c_ushort = (if !ptr.is_null() {
                *(ptr as *mut libc::c_ushort) as libc::c_int
            } else {
                0 as libc::c_int as libc::c_ushort as libc::c_int
            }) as libc::c_ushort;
            return x;
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut mi: *const libc::c_int = (*m).i;
            let mut mp: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            p = *mp.offset(j as isize);
            while p
                < *mp.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mi.offset(p as isize) == i as libc::c_int {
                    return *((*m).data).offset(p as isize);
                }
                p += 1;
                p;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut mj: *const libc::c_int = (*m).i;
            let mut mp_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            p_0 = *mp_0.offset(i as isize);
            while p_0
                < *mp_0
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mj.offset(p_0 as isize) == j as libc::c_int {
                    return *((*m).data).offset(p_0 as isize);
                }
                p_0 += 1;
                p_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./getset_source.c\0" as *const u8 as *const libc::c_char,
                75 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int as libc::c_ushort;
        }
        return 0 as libc::c_int as libc::c_ushort;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_int_get(
    mut m: *const gsl_spmatrix_int,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    if i >= (*m).size1 {
        gsl_error(
            b"first index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if j >= (*m).size2 {
        gsl_error(
            b"second index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if (*m).nz == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int
    } else {
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = tree_int_find(m, i, j);
            let mut x: libc::c_int = if !ptr.is_null() {
                *(ptr as *mut libc::c_int)
            } else {
                0 as libc::c_int
            };
            return x;
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut mi: *const libc::c_int = (*m).i;
            let mut mp: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            p = *mp.offset(j as isize);
            while p
                < *mp.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mi.offset(p as isize) == i as libc::c_int {
                    return *((*m).data).offset(p as isize);
                }
                p += 1;
                p;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut mj: *const libc::c_int = (*m).i;
            let mut mp_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            p_0 = *mp_0.offset(i as isize);
            while p_0
                < *mp_0
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mj.offset(p_0 as isize) == j as libc::c_int {
                    return *((*m).data).offset(p_0 as isize);
                }
                p_0 += 1;
                p_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./getset_source.c\0" as *const u8 as *const libc::c_char,
                75 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int;
        }
        return 0 as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uint_get(
    mut m: *const gsl_spmatrix_uint,
    i: size_t,
    j: size_t,
) -> libc::c_uint {
    if i >= (*m).size1 {
        gsl_error(
            b"first index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int as libc::c_uint;
    } else if j >= (*m).size2 {
        gsl_error(
            b"second index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int as libc::c_uint;
    } else if (*m).nz == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as libc::c_uint
    } else {
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = tree_uint_find(m, i, j);
            let mut x: libc::c_uint = if !ptr.is_null() {
                *(ptr as *mut libc::c_uint)
            } else {
                0 as libc::c_int as libc::c_uint
            };
            return x;
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut mi: *const libc::c_int = (*m).i;
            let mut mp: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            p = *mp.offset(j as isize);
            while p
                < *mp.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mi.offset(p as isize) == i as libc::c_int {
                    return *((*m).data).offset(p as isize);
                }
                p += 1;
                p;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut mj: *const libc::c_int = (*m).i;
            let mut mp_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            p_0 = *mp_0.offset(i as isize);
            while p_0
                < *mp_0
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mj.offset(p_0 as isize) == j as libc::c_int {
                    return *((*m).data).offset(p_0 as isize);
                }
                p_0 += 1;
                p_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./getset_source.c\0" as *const u8 as *const libc::c_char,
                75 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int as libc::c_uint;
        }
        return 0 as libc::c_int as libc::c_uint;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_set(
    mut m: *mut gsl_spmatrix,
    i: size_t,
    j: size_t,
    x: libc::c_double,
) -> libc::c_int {
    if !((*m).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"matrix not in COO representation\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*m).spflags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
        == 0 && (i >= (*m).size1 || j >= (*m).size2)
    {
        gsl_error(
            b"indices out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*m).spflags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong
        != 0
    {
        let mut ptr: *mut libc::c_void = tree_find(m, i, j);
        if ptr.is_null() {
            gsl_error(
                b"attempt to add new matrix element to fixed sparsity pattern\0"
                    as *const u8 as *const libc::c_char,
                b"./getset_source.c\0" as *const u8 as *const libc::c_char,
                105 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        } else {
            *(ptr as *mut libc::c_double) = x;
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut ptr_0: *mut libc::c_void = 0 as *mut libc::c_void;
        if (*m).nz >= (*m).nzmax {
            status = gsl_spmatrix_realloc(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul((*m).nzmax),
                m,
            );
            if status != 0 {
                return status;
            }
        }
        *((*m).i).offset((*m).nz as isize) = i as libc::c_int;
        *((*m).p).offset((*m).nz as isize) = j as libc::c_int;
        *((*m).data).offset((*m).nz as isize) = x;
        ptr_0 = gsl_bst_insert(
            &mut *((*m).data).offset((*m).nz as isize) as *mut libc::c_double
                as *mut libc::c_void,
            (*m).tree,
        );
        if !ptr_0.is_null() {
            *(ptr_0 as *mut libc::c_double) = x;
        } else {
            if (*m).spflags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
                != 0
            {
                (*m)
                    .size1 = if (*m).size1
                    > i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    (*m).size1
                } else {
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                };
                (*m)
                    .size2 = if (*m).size2
                    > j.wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    (*m).size2
                } else {
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong)
                };
            }
            (*m).nz = ((*m).nz).wrapping_add(1);
            (*m).nz;
        }
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_int_set(
    mut m: *mut gsl_spmatrix_int,
    i: size_t,
    j: size_t,
    x: libc::c_int,
) -> libc::c_int {
    if !((*m).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"matrix not in COO representation\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*m).spflags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
        == 0 && (i >= (*m).size1 || j >= (*m).size2)
    {
        gsl_error(
            b"indices out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*m).spflags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong
        != 0
    {
        let mut ptr: *mut libc::c_void = tree_int_find(m, i, j);
        if ptr.is_null() {
            gsl_error(
                b"attempt to add new matrix element to fixed sparsity pattern\0"
                    as *const u8 as *const libc::c_char,
                b"./getset_source.c\0" as *const u8 as *const libc::c_char,
                105 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        } else {
            *(ptr as *mut libc::c_int) = x;
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut ptr_0: *mut libc::c_void = 0 as *mut libc::c_void;
        if (*m).nz >= (*m).nzmax {
            status = gsl_spmatrix_int_realloc(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul((*m).nzmax),
                m,
            );
            if status != 0 {
                return status;
            }
        }
        *((*m).i).offset((*m).nz as isize) = i as libc::c_int;
        *((*m).p).offset((*m).nz as isize) = j as libc::c_int;
        *((*m).data).offset((*m).nz as isize) = x;
        ptr_0 = gsl_bst_insert(
            &mut *((*m).data).offset((*m).nz as isize) as *mut libc::c_int
                as *mut libc::c_void,
            (*m).tree,
        );
        if !ptr_0.is_null() {
            *(ptr_0 as *mut libc::c_int) = x;
        } else {
            if (*m).spflags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
                != 0
            {
                (*m)
                    .size1 = if (*m).size1
                    > i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    (*m).size1
                } else {
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                };
                (*m)
                    .size2 = if (*m).size2
                    > j.wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    (*m).size2
                } else {
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong)
                };
            }
            (*m).nz = ((*m).nz).wrapping_add(1);
            (*m).nz;
        }
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uchar_set(
    mut m: *mut gsl_spmatrix_uchar,
    i: size_t,
    j: size_t,
    x: libc::c_uchar,
) -> libc::c_int {
    if !((*m).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"matrix not in COO representation\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*m).spflags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
        == 0 && (i >= (*m).size1 || j >= (*m).size2)
    {
        gsl_error(
            b"indices out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*m).spflags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong
        != 0
    {
        let mut ptr: *mut libc::c_void = tree_uchar_find(m, i, j);
        if ptr.is_null() {
            gsl_error(
                b"attempt to add new matrix element to fixed sparsity pattern\0"
                    as *const u8 as *const libc::c_char,
                b"./getset_source.c\0" as *const u8 as *const libc::c_char,
                105 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        } else {
            *(ptr as *mut libc::c_uchar) = x;
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut ptr_0: *mut libc::c_void = 0 as *mut libc::c_void;
        if (*m).nz >= (*m).nzmax {
            status = gsl_spmatrix_uchar_realloc(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul((*m).nzmax),
                m,
            );
            if status != 0 {
                return status;
            }
        }
        *((*m).i).offset((*m).nz as isize) = i as libc::c_int;
        *((*m).p).offset((*m).nz as isize) = j as libc::c_int;
        *((*m).data).offset((*m).nz as isize) = x;
        ptr_0 = gsl_bst_insert(
            &mut *((*m).data).offset((*m).nz as isize) as *mut libc::c_uchar
                as *mut libc::c_void,
            (*m).tree,
        );
        if !ptr_0.is_null() {
            *(ptr_0 as *mut libc::c_uchar) = x;
        } else {
            if (*m).spflags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
                != 0
            {
                (*m)
                    .size1 = if (*m).size1
                    > i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    (*m).size1
                } else {
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                };
                (*m)
                    .size2 = if (*m).size2
                    > j.wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    (*m).size2
                } else {
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong)
                };
            }
            (*m).nz = ((*m).nz).wrapping_add(1);
            (*m).nz;
        }
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_short_set(
    mut m: *mut gsl_spmatrix_short,
    i: size_t,
    j: size_t,
    x: libc::c_short,
) -> libc::c_int {
    if !((*m).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"matrix not in COO representation\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*m).spflags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
        == 0 && (i >= (*m).size1 || j >= (*m).size2)
    {
        gsl_error(
            b"indices out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*m).spflags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong
        != 0
    {
        let mut ptr: *mut libc::c_void = tree_short_find(m, i, j);
        if ptr.is_null() {
            gsl_error(
                b"attempt to add new matrix element to fixed sparsity pattern\0"
                    as *const u8 as *const libc::c_char,
                b"./getset_source.c\0" as *const u8 as *const libc::c_char,
                105 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        } else {
            *(ptr as *mut libc::c_short) = x;
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut ptr_0: *mut libc::c_void = 0 as *mut libc::c_void;
        if (*m).nz >= (*m).nzmax {
            status = gsl_spmatrix_short_realloc(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul((*m).nzmax),
                m,
            );
            if status != 0 {
                return status;
            }
        }
        *((*m).i).offset((*m).nz as isize) = i as libc::c_int;
        *((*m).p).offset((*m).nz as isize) = j as libc::c_int;
        *((*m).data).offset((*m).nz as isize) = x;
        ptr_0 = gsl_bst_insert(
            &mut *((*m).data).offset((*m).nz as isize) as *mut libc::c_short
                as *mut libc::c_void,
            (*m).tree,
        );
        if !ptr_0.is_null() {
            *(ptr_0 as *mut libc::c_short) = x;
        } else {
            if (*m).spflags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
                != 0
            {
                (*m)
                    .size1 = if (*m).size1
                    > i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    (*m).size1
                } else {
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                };
                (*m)
                    .size2 = if (*m).size2
                    > j.wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    (*m).size2
                } else {
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong)
                };
            }
            (*m).nz = ((*m).nz).wrapping_add(1);
            (*m).nz;
        }
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_long_set(
    mut m: *mut gsl_spmatrix_long,
    i: size_t,
    j: size_t,
    x: libc::c_long,
) -> libc::c_int {
    if !((*m).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"matrix not in COO representation\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*m).spflags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
        == 0 && (i >= (*m).size1 || j >= (*m).size2)
    {
        gsl_error(
            b"indices out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*m).spflags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong
        != 0
    {
        let mut ptr: *mut libc::c_void = tree_long_find(m, i, j);
        if ptr.is_null() {
            gsl_error(
                b"attempt to add new matrix element to fixed sparsity pattern\0"
                    as *const u8 as *const libc::c_char,
                b"./getset_source.c\0" as *const u8 as *const libc::c_char,
                105 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        } else {
            *(ptr as *mut libc::c_long) = x;
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut ptr_0: *mut libc::c_void = 0 as *mut libc::c_void;
        if (*m).nz >= (*m).nzmax {
            status = gsl_spmatrix_long_realloc(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul((*m).nzmax),
                m,
            );
            if status != 0 {
                return status;
            }
        }
        *((*m).i).offset((*m).nz as isize) = i as libc::c_int;
        *((*m).p).offset((*m).nz as isize) = j as libc::c_int;
        *((*m).data).offset((*m).nz as isize) = x;
        ptr_0 = gsl_bst_insert(
            &mut *((*m).data).offset((*m).nz as isize) as *mut libc::c_long
                as *mut libc::c_void,
            (*m).tree,
        );
        if !ptr_0.is_null() {
            *(ptr_0 as *mut libc::c_long) = x;
        } else {
            if (*m).spflags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
                != 0
            {
                (*m)
                    .size1 = if (*m).size1
                    > i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    (*m).size1
                } else {
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                };
                (*m)
                    .size2 = if (*m).size2
                    > j.wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    (*m).size2
                } else {
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong)
                };
            }
            (*m).nz = ((*m).nz).wrapping_add(1);
            (*m).nz;
        }
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_ushort_set(
    mut m: *mut gsl_spmatrix_ushort,
    i: size_t,
    j: size_t,
    x: libc::c_ushort,
) -> libc::c_int {
    if !((*m).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"matrix not in COO representation\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*m).spflags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
        == 0 && (i >= (*m).size1 || j >= (*m).size2)
    {
        gsl_error(
            b"indices out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*m).spflags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong
        != 0
    {
        let mut ptr: *mut libc::c_void = tree_ushort_find(m, i, j);
        if ptr.is_null() {
            gsl_error(
                b"attempt to add new matrix element to fixed sparsity pattern\0"
                    as *const u8 as *const libc::c_char,
                b"./getset_source.c\0" as *const u8 as *const libc::c_char,
                105 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        } else {
            *(ptr as *mut libc::c_ushort) = x;
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut ptr_0: *mut libc::c_void = 0 as *mut libc::c_void;
        if (*m).nz >= (*m).nzmax {
            status = gsl_spmatrix_ushort_realloc(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul((*m).nzmax),
                m,
            );
            if status != 0 {
                return status;
            }
        }
        *((*m).i).offset((*m).nz as isize) = i as libc::c_int;
        *((*m).p).offset((*m).nz as isize) = j as libc::c_int;
        *((*m).data).offset((*m).nz as isize) = x;
        ptr_0 = gsl_bst_insert(
            &mut *((*m).data).offset((*m).nz as isize) as *mut libc::c_ushort
                as *mut libc::c_void,
            (*m).tree,
        );
        if !ptr_0.is_null() {
            *(ptr_0 as *mut libc::c_ushort) = x;
        } else {
            if (*m).spflags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
                != 0
            {
                (*m)
                    .size1 = if (*m).size1
                    > i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    (*m).size1
                } else {
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                };
                (*m)
                    .size2 = if (*m).size2
                    > j.wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    (*m).size2
                } else {
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong)
                };
            }
            (*m).nz = ((*m).nz).wrapping_add(1);
            (*m).nz;
        }
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_float_set(
    mut m: *mut gsl_spmatrix_float,
    i: size_t,
    j: size_t,
    x: libc::c_float,
) -> libc::c_int {
    if !((*m).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"matrix not in COO representation\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*m).spflags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
        == 0 && (i >= (*m).size1 || j >= (*m).size2)
    {
        gsl_error(
            b"indices out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*m).spflags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong
        != 0
    {
        let mut ptr: *mut libc::c_void = tree_float_find(m, i, j);
        if ptr.is_null() {
            gsl_error(
                b"attempt to add new matrix element to fixed sparsity pattern\0"
                    as *const u8 as *const libc::c_char,
                b"./getset_source.c\0" as *const u8 as *const libc::c_char,
                105 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        } else {
            *(ptr as *mut libc::c_float) = x;
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut ptr_0: *mut libc::c_void = 0 as *mut libc::c_void;
        if (*m).nz >= (*m).nzmax {
            status = gsl_spmatrix_float_realloc(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul((*m).nzmax),
                m,
            );
            if status != 0 {
                return status;
            }
        }
        *((*m).i).offset((*m).nz as isize) = i as libc::c_int;
        *((*m).p).offset((*m).nz as isize) = j as libc::c_int;
        *((*m).data).offset((*m).nz as isize) = x;
        ptr_0 = gsl_bst_insert(
            &mut *((*m).data).offset((*m).nz as isize) as *mut libc::c_float
                as *mut libc::c_void,
            (*m).tree,
        );
        if !ptr_0.is_null() {
            *(ptr_0 as *mut libc::c_float) = x;
        } else {
            if (*m).spflags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
                != 0
            {
                (*m)
                    .size1 = if (*m).size1
                    > i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    (*m).size1
                } else {
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                };
                (*m)
                    .size2 = if (*m).size2
                    > j.wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    (*m).size2
                } else {
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong)
                };
            }
            (*m).nz = ((*m).nz).wrapping_add(1);
            (*m).nz;
        }
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uint_set(
    mut m: *mut gsl_spmatrix_uint,
    i: size_t,
    j: size_t,
    x: libc::c_uint,
) -> libc::c_int {
    if !((*m).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"matrix not in COO representation\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*m).spflags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
        == 0 && (i >= (*m).size1 || j >= (*m).size2)
    {
        gsl_error(
            b"indices out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*m).spflags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong
        != 0
    {
        let mut ptr: *mut libc::c_void = tree_uint_find(m, i, j);
        if ptr.is_null() {
            gsl_error(
                b"attempt to add new matrix element to fixed sparsity pattern\0"
                    as *const u8 as *const libc::c_char,
                b"./getset_source.c\0" as *const u8 as *const libc::c_char,
                105 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        } else {
            *(ptr as *mut libc::c_uint) = x;
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut ptr_0: *mut libc::c_void = 0 as *mut libc::c_void;
        if (*m).nz >= (*m).nzmax {
            status = gsl_spmatrix_uint_realloc(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul((*m).nzmax),
                m,
            );
            if status != 0 {
                return status;
            }
        }
        *((*m).i).offset((*m).nz as isize) = i as libc::c_int;
        *((*m).p).offset((*m).nz as isize) = j as libc::c_int;
        *((*m).data).offset((*m).nz as isize) = x;
        ptr_0 = gsl_bst_insert(
            &mut *((*m).data).offset((*m).nz as isize) as *mut libc::c_uint
                as *mut libc::c_void,
            (*m).tree,
        );
        if !ptr_0.is_null() {
            *(ptr_0 as *mut libc::c_uint) = x;
        } else {
            if (*m).spflags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
                != 0
            {
                (*m)
                    .size1 = if (*m).size1
                    > i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    (*m).size1
                } else {
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                };
                (*m)
                    .size2 = if (*m).size2
                    > j.wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    (*m).size2
                } else {
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong)
                };
            }
            (*m).nz = ((*m).nz).wrapping_add(1);
            (*m).nz;
        }
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_char_set(
    mut m: *mut gsl_spmatrix_char,
    i: size_t,
    j: size_t,
    x: libc::c_char,
) -> libc::c_int {
    if !((*m).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"matrix not in COO representation\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*m).spflags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
        == 0 && (i >= (*m).size1 || j >= (*m).size2)
    {
        gsl_error(
            b"indices out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*m).spflags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong
        != 0
    {
        let mut ptr: *mut libc::c_void = tree_char_find(m, i, j);
        if ptr.is_null() {
            gsl_error(
                b"attempt to add new matrix element to fixed sparsity pattern\0"
                    as *const u8 as *const libc::c_char,
                b"./getset_source.c\0" as *const u8 as *const libc::c_char,
                105 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        } else {
            *(ptr as *mut libc::c_char) = x;
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut ptr_0: *mut libc::c_void = 0 as *mut libc::c_void;
        if (*m).nz >= (*m).nzmax {
            status = gsl_spmatrix_char_realloc(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul((*m).nzmax),
                m,
            );
            if status != 0 {
                return status;
            }
        }
        *((*m).i).offset((*m).nz as isize) = i as libc::c_int;
        *((*m).p).offset((*m).nz as isize) = j as libc::c_int;
        *((*m).data).offset((*m).nz as isize) = x;
        ptr_0 = gsl_bst_insert(
            &mut *((*m).data).offset((*m).nz as isize) as *mut libc::c_char
                as *mut libc::c_void,
            (*m).tree,
        );
        if !ptr_0.is_null() {
            *(ptr_0 as *mut libc::c_char) = x;
        } else {
            if (*m).spflags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
                != 0
            {
                (*m)
                    .size1 = if (*m).size1
                    > i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    (*m).size1
                } else {
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                };
                (*m)
                    .size2 = if (*m).size2
                    > j.wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    (*m).size2
                } else {
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong)
                };
            }
            (*m).nz = ((*m).nz).wrapping_add(1);
            (*m).nz;
        }
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_ulong_set(
    mut m: *mut gsl_spmatrix_ulong,
    i: size_t,
    j: size_t,
    x: libc::c_ulong,
) -> libc::c_int {
    if !((*m).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"matrix not in COO representation\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*m).spflags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
        == 0 && (i >= (*m).size1 || j >= (*m).size2)
    {
        gsl_error(
            b"indices out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*m).spflags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong
        != 0
    {
        let mut ptr: *mut libc::c_void = tree_ulong_find(m, i, j);
        if ptr.is_null() {
            gsl_error(
                b"attempt to add new matrix element to fixed sparsity pattern\0"
                    as *const u8 as *const libc::c_char,
                b"./getset_source.c\0" as *const u8 as *const libc::c_char,
                105 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        } else {
            *(ptr as *mut libc::c_ulong) = x;
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut ptr_0: *mut libc::c_void = 0 as *mut libc::c_void;
        if (*m).nz >= (*m).nzmax {
            status = gsl_spmatrix_ulong_realloc(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul((*m).nzmax),
                m,
            );
            if status != 0 {
                return status;
            }
        }
        *((*m).i).offset((*m).nz as isize) = i as libc::c_int;
        *((*m).p).offset((*m).nz as isize) = j as libc::c_int;
        *((*m).data).offset((*m).nz as isize) = x;
        ptr_0 = gsl_bst_insert(
            &mut *((*m).data).offset((*m).nz as isize) as *mut libc::c_ulong
                as *mut libc::c_void,
            (*m).tree,
        );
        if !ptr_0.is_null() {
            *(ptr_0 as *mut libc::c_ulong) = x;
        } else {
            if (*m).spflags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
                != 0
            {
                (*m)
                    .size1 = if (*m).size1
                    > i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    (*m).size1
                } else {
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                };
                (*m)
                    .size2 = if (*m).size2
                    > j.wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    (*m).size2
                } else {
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong)
                };
            }
            (*m).nz = ((*m).nz).wrapping_add(1);
            (*m).nz;
        }
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_set(
    mut m: *mut gsl_spmatrix_long_double,
    i: size_t,
    j: size_t,
    x: f128::f128,
) -> libc::c_int {
    if !((*m).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"matrix not in COO representation\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*m).spflags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
        == 0 && (i >= (*m).size1 || j >= (*m).size2)
    {
        gsl_error(
            b"indices out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*m).spflags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong
        != 0
    {
        let mut ptr: *mut libc::c_void = tree_long_double_find(m, i, j);
        if ptr.is_null() {
            gsl_error(
                b"attempt to add new matrix element to fixed sparsity pattern\0"
                    as *const u8 as *const libc::c_char,
                b"./getset_source.c\0" as *const u8 as *const libc::c_char,
                105 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        } else {
            *(ptr as *mut f128::f128) = x;
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut ptr_0: *mut libc::c_void = 0 as *mut libc::c_void;
        if (*m).nz >= (*m).nzmax {
            status = gsl_spmatrix_long_double_realloc(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul((*m).nzmax),
                m,
            );
            if status != 0 {
                return status;
            }
        }
        *((*m).i).offset((*m).nz as isize) = i as libc::c_int;
        *((*m).p).offset((*m).nz as isize) = j as libc::c_int;
        *((*m).data).offset((*m).nz as isize) = x;
        ptr_0 = gsl_bst_insert(
            &mut *((*m).data).offset((*m).nz as isize) as *mut f128::f128
                as *mut libc::c_void,
            (*m).tree,
        );
        if !ptr_0.is_null() {
            *(ptr_0 as *mut f128::f128) = x;
        } else {
            if (*m).spflags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
                != 0
            {
                (*m)
                    .size1 = if (*m).size1
                    > i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    (*m).size1
                } else {
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                };
                (*m)
                    .size2 = if (*m).size2
                    > j.wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    (*m).size2
                } else {
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong)
                };
            }
            (*m).nz = ((*m).nz).wrapping_add(1);
            (*m).nz;
        }
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_ptr(
    mut m: *const gsl_spmatrix_long_double,
    i: size_t,
    j: size_t,
) -> *mut f128::f128 {
    if i >= (*m).size1 {
        gsl_error(
            b"first index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            161 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut f128::f128;
    } else if j >= (*m).size2 {
        gsl_error(
            b"second index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            165 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut f128::f128;
    } else {
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = tree_long_double_find(m, i, j);
            return ptr as *mut f128::f128;
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut mi: *const libc::c_int = (*m).i;
            let mut mp: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            p = *mp.offset(j as isize);
            while p
                < *mp.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mi.offset(p as isize) == i as libc::c_int {
                    return &mut *((*m).data).offset(p as isize) as *mut f128::f128;
                }
                p += 1;
                p;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut mj: *const libc::c_int = (*m).i;
            let mut mp_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            p_0 = *mp_0.offset(i as isize);
            while p_0
                < *mp_0
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mj.offset(p_0 as isize) == j as libc::c_int {
                    return &mut *((*m).data).offset(p_0 as isize) as *mut f128::f128;
                }
                p_0 += 1;
                p_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./getset_source.c\0" as *const u8 as *const libc::c_char,
                203 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut f128::f128;
        }
        return 0 as *mut f128::f128;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_ushort_ptr(
    mut m: *const gsl_spmatrix_ushort,
    i: size_t,
    j: size_t,
) -> *mut libc::c_ushort {
    if i >= (*m).size1 {
        gsl_error(
            b"first index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            161 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut libc::c_ushort;
    } else if j >= (*m).size2 {
        gsl_error(
            b"second index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            165 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut libc::c_ushort;
    } else {
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = tree_ushort_find(m, i, j);
            return ptr as *mut libc::c_ushort;
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut mi: *const libc::c_int = (*m).i;
            let mut mp: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            p = *mp.offset(j as isize);
            while p
                < *mp.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mi.offset(p as isize) == i as libc::c_int {
                    return &mut *((*m).data).offset(p as isize) as *mut libc::c_ushort;
                }
                p += 1;
                p;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut mj: *const libc::c_int = (*m).i;
            let mut mp_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            p_0 = *mp_0.offset(i as isize);
            while p_0
                < *mp_0
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mj.offset(p_0 as isize) == j as libc::c_int {
                    return &mut *((*m).data).offset(p_0 as isize) as *mut libc::c_ushort;
                }
                p_0 += 1;
                p_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./getset_source.c\0" as *const u8 as *const libc::c_char,
                203 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut libc::c_ushort;
        }
        return 0 as *mut libc::c_ushort;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_float_ptr(
    mut m: *const gsl_spmatrix_float,
    i: size_t,
    j: size_t,
) -> *mut libc::c_float {
    if i >= (*m).size1 {
        gsl_error(
            b"first index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            161 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut libc::c_float;
    } else if j >= (*m).size2 {
        gsl_error(
            b"second index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            165 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut libc::c_float;
    } else {
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = tree_float_find(m, i, j);
            return ptr as *mut libc::c_float;
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut mi: *const libc::c_int = (*m).i;
            let mut mp: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            p = *mp.offset(j as isize);
            while p
                < *mp.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mi.offset(p as isize) == i as libc::c_int {
                    return &mut *((*m).data).offset(p as isize) as *mut libc::c_float;
                }
                p += 1;
                p;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut mj: *const libc::c_int = (*m).i;
            let mut mp_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            p_0 = *mp_0.offset(i as isize);
            while p_0
                < *mp_0
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mj.offset(p_0 as isize) == j as libc::c_int {
                    return &mut *((*m).data).offset(p_0 as isize) as *mut libc::c_float;
                }
                p_0 += 1;
                p_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./getset_source.c\0" as *const u8 as *const libc::c_char,
                203 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut libc::c_float;
        }
        return 0 as *mut libc::c_float;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uchar_ptr(
    mut m: *const gsl_spmatrix_uchar,
    i: size_t,
    j: size_t,
) -> *mut libc::c_uchar {
    if i >= (*m).size1 {
        gsl_error(
            b"first index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            161 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut libc::c_uchar;
    } else if j >= (*m).size2 {
        gsl_error(
            b"second index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            165 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut libc::c_uchar;
    } else {
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = tree_uchar_find(m, i, j);
            return ptr as *mut libc::c_uchar;
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut mi: *const libc::c_int = (*m).i;
            let mut mp: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            p = *mp.offset(j as isize);
            while p
                < *mp.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mi.offset(p as isize) == i as libc::c_int {
                    return &mut *((*m).data).offset(p as isize) as *mut libc::c_uchar;
                }
                p += 1;
                p;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut mj: *const libc::c_int = (*m).i;
            let mut mp_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            p_0 = *mp_0.offset(i as isize);
            while p_0
                < *mp_0
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mj.offset(p_0 as isize) == j as libc::c_int {
                    return &mut *((*m).data).offset(p_0 as isize) as *mut libc::c_uchar;
                }
                p_0 += 1;
                p_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./getset_source.c\0" as *const u8 as *const libc::c_char,
                203 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut libc::c_uchar;
        }
        return 0 as *mut libc::c_uchar;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_int_ptr(
    mut m: *const gsl_spmatrix_int,
    i: size_t,
    j: size_t,
) -> *mut libc::c_int {
    if i >= (*m).size1 {
        gsl_error(
            b"first index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            161 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut libc::c_int;
    } else if j >= (*m).size2 {
        gsl_error(
            b"second index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            165 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut libc::c_int;
    } else {
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = tree_int_find(m, i, j);
            return ptr as *mut libc::c_int;
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut mi: *const libc::c_int = (*m).i;
            let mut mp: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            p = *mp.offset(j as isize);
            while p
                < *mp.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mi.offset(p as isize) == i as libc::c_int {
                    return &mut *((*m).data).offset(p as isize) as *mut libc::c_int;
                }
                p += 1;
                p;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut mj: *const libc::c_int = (*m).i;
            let mut mp_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            p_0 = *mp_0.offset(i as isize);
            while p_0
                < *mp_0
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mj.offset(p_0 as isize) == j as libc::c_int {
                    return &mut *((*m).data).offset(p_0 as isize) as *mut libc::c_int;
                }
                p_0 += 1;
                p_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./getset_source.c\0" as *const u8 as *const libc::c_char,
                203 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut libc::c_int;
        }
        return 0 as *mut libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_ptr(
    mut m: *const gsl_spmatrix,
    i: size_t,
    j: size_t,
) -> *mut libc::c_double {
    if i >= (*m).size1 {
        gsl_error(
            b"first index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            161 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut libc::c_double;
    } else if j >= (*m).size2 {
        gsl_error(
            b"second index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            165 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut libc::c_double;
    } else {
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = tree_find(m, i, j);
            return ptr as *mut libc::c_double;
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut mi: *const libc::c_int = (*m).i;
            let mut mp: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            p = *mp.offset(j as isize);
            while p
                < *mp.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mi.offset(p as isize) == i as libc::c_int {
                    return &mut *((*m).data).offset(p as isize) as *mut libc::c_double;
                }
                p += 1;
                p;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut mj: *const libc::c_int = (*m).i;
            let mut mp_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            p_0 = *mp_0.offset(i as isize);
            while p_0
                < *mp_0
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mj.offset(p_0 as isize) == j as libc::c_int {
                    return &mut *((*m).data).offset(p_0 as isize) as *mut libc::c_double;
                }
                p_0 += 1;
                p_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./getset_source.c\0" as *const u8 as *const libc::c_char,
                203 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut libc::c_double;
        }
        return 0 as *mut libc::c_double;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_ulong_ptr(
    mut m: *const gsl_spmatrix_ulong,
    i: size_t,
    j: size_t,
) -> *mut libc::c_ulong {
    if i >= (*m).size1 {
        gsl_error(
            b"first index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            161 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut libc::c_ulong;
    } else if j >= (*m).size2 {
        gsl_error(
            b"second index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            165 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut libc::c_ulong;
    } else {
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = tree_ulong_find(m, i, j);
            return ptr as *mut libc::c_ulong;
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut mi: *const libc::c_int = (*m).i;
            let mut mp: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            p = *mp.offset(j as isize);
            while p
                < *mp.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mi.offset(p as isize) == i as libc::c_int {
                    return &mut *((*m).data).offset(p as isize) as *mut libc::c_ulong;
                }
                p += 1;
                p;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut mj: *const libc::c_int = (*m).i;
            let mut mp_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            p_0 = *mp_0.offset(i as isize);
            while p_0
                < *mp_0
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mj.offset(p_0 as isize) == j as libc::c_int {
                    return &mut *((*m).data).offset(p_0 as isize) as *mut libc::c_ulong;
                }
                p_0 += 1;
                p_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./getset_source.c\0" as *const u8 as *const libc::c_char,
                203 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut libc::c_ulong;
        }
        return 0 as *mut libc::c_ulong;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_short_ptr(
    mut m: *const gsl_spmatrix_short,
    i: size_t,
    j: size_t,
) -> *mut libc::c_short {
    if i >= (*m).size1 {
        gsl_error(
            b"first index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            161 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut libc::c_short;
    } else if j >= (*m).size2 {
        gsl_error(
            b"second index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            165 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut libc::c_short;
    } else {
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = tree_short_find(m, i, j);
            return ptr as *mut libc::c_short;
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut mi: *const libc::c_int = (*m).i;
            let mut mp: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            p = *mp.offset(j as isize);
            while p
                < *mp.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mi.offset(p as isize) == i as libc::c_int {
                    return &mut *((*m).data).offset(p as isize) as *mut libc::c_short;
                }
                p += 1;
                p;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut mj: *const libc::c_int = (*m).i;
            let mut mp_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            p_0 = *mp_0.offset(i as isize);
            while p_0
                < *mp_0
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mj.offset(p_0 as isize) == j as libc::c_int {
                    return &mut *((*m).data).offset(p_0 as isize) as *mut libc::c_short;
                }
                p_0 += 1;
                p_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./getset_source.c\0" as *const u8 as *const libc::c_char,
                203 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut libc::c_short;
        }
        return 0 as *mut libc::c_short;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_long_ptr(
    mut m: *const gsl_spmatrix_long,
    i: size_t,
    j: size_t,
) -> *mut libc::c_long {
    if i >= (*m).size1 {
        gsl_error(
            b"first index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            161 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut libc::c_long;
    } else if j >= (*m).size2 {
        gsl_error(
            b"second index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            165 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut libc::c_long;
    } else {
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = tree_long_find(m, i, j);
            return ptr as *mut libc::c_long;
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut mi: *const libc::c_int = (*m).i;
            let mut mp: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            p = *mp.offset(j as isize);
            while p
                < *mp.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mi.offset(p as isize) == i as libc::c_int {
                    return &mut *((*m).data).offset(p as isize) as *mut libc::c_long;
                }
                p += 1;
                p;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut mj: *const libc::c_int = (*m).i;
            let mut mp_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            p_0 = *mp_0.offset(i as isize);
            while p_0
                < *mp_0
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mj.offset(p_0 as isize) == j as libc::c_int {
                    return &mut *((*m).data).offset(p_0 as isize) as *mut libc::c_long;
                }
                p_0 += 1;
                p_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./getset_source.c\0" as *const u8 as *const libc::c_char,
                203 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut libc::c_long;
        }
        return 0 as *mut libc::c_long;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uint_ptr(
    mut m: *const gsl_spmatrix_uint,
    i: size_t,
    j: size_t,
) -> *mut libc::c_uint {
    if i >= (*m).size1 {
        gsl_error(
            b"first index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            161 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut libc::c_uint;
    } else if j >= (*m).size2 {
        gsl_error(
            b"second index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            165 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut libc::c_uint;
    } else {
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = tree_uint_find(m, i, j);
            return ptr as *mut libc::c_uint;
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut mi: *const libc::c_int = (*m).i;
            let mut mp: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            p = *mp.offset(j as isize);
            while p
                < *mp.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mi.offset(p as isize) == i as libc::c_int {
                    return &mut *((*m).data).offset(p as isize) as *mut libc::c_uint;
                }
                p += 1;
                p;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut mj: *const libc::c_int = (*m).i;
            let mut mp_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            p_0 = *mp_0.offset(i as isize);
            while p_0
                < *mp_0
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mj.offset(p_0 as isize) == j as libc::c_int {
                    return &mut *((*m).data).offset(p_0 as isize) as *mut libc::c_uint;
                }
                p_0 += 1;
                p_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./getset_source.c\0" as *const u8 as *const libc::c_char,
                203 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut libc::c_uint;
        }
        return 0 as *mut libc::c_uint;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_char_ptr(
    mut m: *const gsl_spmatrix_char,
    i: size_t,
    j: size_t,
) -> *mut libc::c_char {
    if i >= (*m).size1 {
        gsl_error(
            b"first index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            161 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut libc::c_char;
    } else if j >= (*m).size2 {
        gsl_error(
            b"second index out of range\0" as *const u8 as *const libc::c_char,
            b"./getset_source.c\0" as *const u8 as *const libc::c_char,
            165 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut libc::c_char;
    } else {
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = tree_char_find(m, i, j);
            return ptr as *mut libc::c_char;
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut mi: *const libc::c_int = (*m).i;
            let mut mp: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            p = *mp.offset(j as isize);
            while p
                < *mp.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mi.offset(p as isize) == i as libc::c_int {
                    return &mut *((*m).data).offset(p as isize) as *mut libc::c_char;
                }
                p += 1;
                p;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut mj: *const libc::c_int = (*m).i;
            let mut mp_0: *const libc::c_int = (*m).p;
            let mut p_0: libc::c_int = 0;
            p_0 = *mp_0.offset(i as isize);
            while p_0
                < *mp_0
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                if *mj.offset(p_0 as isize) == j as libc::c_int {
                    return &mut *((*m).data).offset(p_0 as isize) as *mut libc::c_char;
                }
                p_0 += 1;
                p_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./getset_source.c\0" as *const u8 as *const libc::c_char,
                203 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as *mut libc::c_char;
        }
        return 0 as *mut libc::c_char;
    };
}
unsafe extern "C" fn tree_int_find(
    mut m: *const gsl_spmatrix_int,
    i: size_t,
    j: size_t,
) -> *mut libc::c_void {
    let mut table: *const gsl_bst_avl_table = &mut (*(*m).tree).table.avl_table
        as *mut gsl_bst_avl_table as *const gsl_bst_avl_table;
    let mut p: *mut gsl_bst_avl_node = 0 as *mut gsl_bst_avl_node;
    p = (*table).avl_root;
    while !p.is_null() {
        let mut n: size_t = ((*p).avl_data as *mut libc::c_int).offset_from((*m).data)
            as libc::c_long as size_t;
        let mut pi: libc::c_int = *((*m).i).offset(n as isize);
        let mut pj: libc::c_int = *((*m).p).offset(n as isize);
        let mut cmp: libc::c_int = if (i as libc::c_int) < pi {
            -(1 as libc::c_int)
        } else if i as libc::c_int > pi {
            1 as libc::c_int
        } else if (j as libc::c_int) < pj {
            -(1 as libc::c_int)
        } else {
            (j as libc::c_int > pj) as libc::c_int
        };
        if cmp < 0 as libc::c_int {
            p = (*p).avl_link[0 as libc::c_int as usize];
        } else if cmp > 0 as libc::c_int {
            p = (*p).avl_link[1 as libc::c_int as usize];
        } else {
            return (*p).avl_data
        }
    }
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn tree_uint_find(
    mut m: *const gsl_spmatrix_uint,
    i: size_t,
    j: size_t,
) -> *mut libc::c_void {
    let mut table: *const gsl_bst_avl_table = &mut (*(*m).tree).table.avl_table
        as *mut gsl_bst_avl_table as *const gsl_bst_avl_table;
    let mut p: *mut gsl_bst_avl_node = 0 as *mut gsl_bst_avl_node;
    p = (*table).avl_root;
    while !p.is_null() {
        let mut n: size_t = ((*p).avl_data as *mut libc::c_uint).offset_from((*m).data)
            as libc::c_long as size_t;
        let mut pi: libc::c_int = *((*m).i).offset(n as isize);
        let mut pj: libc::c_int = *((*m).p).offset(n as isize);
        let mut cmp: libc::c_int = if (i as libc::c_int) < pi {
            -(1 as libc::c_int)
        } else if i as libc::c_int > pi {
            1 as libc::c_int
        } else if (j as libc::c_int) < pj {
            -(1 as libc::c_int)
        } else {
            (j as libc::c_int > pj) as libc::c_int
        };
        if cmp < 0 as libc::c_int {
            p = (*p).avl_link[0 as libc::c_int as usize];
        } else if cmp > 0 as libc::c_int {
            p = (*p).avl_link[1 as libc::c_int as usize];
        } else {
            return (*p).avl_data
        }
    }
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn tree_find(
    mut m: *const gsl_spmatrix,
    i: size_t,
    j: size_t,
) -> *mut libc::c_void {
    let mut table: *const gsl_bst_avl_table = &mut (*(*m).tree).table.avl_table
        as *mut gsl_bst_avl_table as *const gsl_bst_avl_table;
    let mut p: *mut gsl_bst_avl_node = 0 as *mut gsl_bst_avl_node;
    p = (*table).avl_root;
    while !p.is_null() {
        let mut n: size_t = ((*p).avl_data as *mut libc::c_double).offset_from((*m).data)
            as libc::c_long as size_t;
        let mut pi: libc::c_int = *((*m).i).offset(n as isize);
        let mut pj: libc::c_int = *((*m).p).offset(n as isize);
        let mut cmp: libc::c_int = if (i as libc::c_int) < pi {
            -(1 as libc::c_int)
        } else if i as libc::c_int > pi {
            1 as libc::c_int
        } else if (j as libc::c_int) < pj {
            -(1 as libc::c_int)
        } else {
            (j as libc::c_int > pj) as libc::c_int
        };
        if cmp < 0 as libc::c_int {
            p = (*p).avl_link[0 as libc::c_int as usize];
        } else if cmp > 0 as libc::c_int {
            p = (*p).avl_link[1 as libc::c_int as usize];
        } else {
            return (*p).avl_data
        }
    }
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn tree_short_find(
    mut m: *const gsl_spmatrix_short,
    i: size_t,
    j: size_t,
) -> *mut libc::c_void {
    let mut table: *const gsl_bst_avl_table = &mut (*(*m).tree).table.avl_table
        as *mut gsl_bst_avl_table as *const gsl_bst_avl_table;
    let mut p: *mut gsl_bst_avl_node = 0 as *mut gsl_bst_avl_node;
    p = (*table).avl_root;
    while !p.is_null() {
        let mut n: size_t = ((*p).avl_data as *mut libc::c_short).offset_from((*m).data)
            as libc::c_long as size_t;
        let mut pi: libc::c_int = *((*m).i).offset(n as isize);
        let mut pj: libc::c_int = *((*m).p).offset(n as isize);
        let mut cmp: libc::c_int = if (i as libc::c_int) < pi {
            -(1 as libc::c_int)
        } else if i as libc::c_int > pi {
            1 as libc::c_int
        } else if (j as libc::c_int) < pj {
            -(1 as libc::c_int)
        } else {
            (j as libc::c_int > pj) as libc::c_int
        };
        if cmp < 0 as libc::c_int {
            p = (*p).avl_link[0 as libc::c_int as usize];
        } else if cmp > 0 as libc::c_int {
            p = (*p).avl_link[1 as libc::c_int as usize];
        } else {
            return (*p).avl_data
        }
    }
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn tree_ushort_find(
    mut m: *const gsl_spmatrix_ushort,
    i: size_t,
    j: size_t,
) -> *mut libc::c_void {
    let mut table: *const gsl_bst_avl_table = &mut (*(*m).tree).table.avl_table
        as *mut gsl_bst_avl_table as *const gsl_bst_avl_table;
    let mut p: *mut gsl_bst_avl_node = 0 as *mut gsl_bst_avl_node;
    p = (*table).avl_root;
    while !p.is_null() {
        let mut n: size_t = ((*p).avl_data as *mut libc::c_ushort).offset_from((*m).data)
            as libc::c_long as size_t;
        let mut pi: libc::c_int = *((*m).i).offset(n as isize);
        let mut pj: libc::c_int = *((*m).p).offset(n as isize);
        let mut cmp: libc::c_int = if (i as libc::c_int) < pi {
            -(1 as libc::c_int)
        } else if i as libc::c_int > pi {
            1 as libc::c_int
        } else if (j as libc::c_int) < pj {
            -(1 as libc::c_int)
        } else {
            (j as libc::c_int > pj) as libc::c_int
        };
        if cmp < 0 as libc::c_int {
            p = (*p).avl_link[0 as libc::c_int as usize];
        } else if cmp > 0 as libc::c_int {
            p = (*p).avl_link[1 as libc::c_int as usize];
        } else {
            return (*p).avl_data
        }
    }
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn tree_long_double_find(
    mut m: *const gsl_spmatrix_long_double,
    i: size_t,
    j: size_t,
) -> *mut libc::c_void {
    let mut table: *const gsl_bst_avl_table = &mut (*(*m).tree).table.avl_table
        as *mut gsl_bst_avl_table as *const gsl_bst_avl_table;
    let mut p: *mut gsl_bst_avl_node = 0 as *mut gsl_bst_avl_node;
    p = (*table).avl_root;
    while !p.is_null() {
        let mut n: size_t = ((*p).avl_data as *mut f128::f128).offset_from((*m).data)
            as libc::c_long as size_t;
        let mut pi: libc::c_int = *((*m).i).offset(n as isize);
        let mut pj: libc::c_int = *((*m).p).offset(n as isize);
        let mut cmp: libc::c_int = if (i as libc::c_int) < pi {
            -(1 as libc::c_int)
        } else if i as libc::c_int > pi {
            1 as libc::c_int
        } else if (j as libc::c_int) < pj {
            -(1 as libc::c_int)
        } else {
            (j as libc::c_int > pj) as libc::c_int
        };
        if cmp < 0 as libc::c_int {
            p = (*p).avl_link[0 as libc::c_int as usize];
        } else if cmp > 0 as libc::c_int {
            p = (*p).avl_link[1 as libc::c_int as usize];
        } else {
            return (*p).avl_data
        }
    }
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn tree_char_find(
    mut m: *const gsl_spmatrix_char,
    i: size_t,
    j: size_t,
) -> *mut libc::c_void {
    let mut table: *const gsl_bst_avl_table = &mut (*(*m).tree).table.avl_table
        as *mut gsl_bst_avl_table as *const gsl_bst_avl_table;
    let mut p: *mut gsl_bst_avl_node = 0 as *mut gsl_bst_avl_node;
    p = (*table).avl_root;
    while !p.is_null() {
        let mut n: size_t = ((*p).avl_data as *mut libc::c_char).offset_from((*m).data)
            as libc::c_long as size_t;
        let mut pi: libc::c_int = *((*m).i).offset(n as isize);
        let mut pj: libc::c_int = *((*m).p).offset(n as isize);
        let mut cmp: libc::c_int = if (i as libc::c_int) < pi {
            -(1 as libc::c_int)
        } else if i as libc::c_int > pi {
            1 as libc::c_int
        } else if (j as libc::c_int) < pj {
            -(1 as libc::c_int)
        } else {
            (j as libc::c_int > pj) as libc::c_int
        };
        if cmp < 0 as libc::c_int {
            p = (*p).avl_link[0 as libc::c_int as usize];
        } else if cmp > 0 as libc::c_int {
            p = (*p).avl_link[1 as libc::c_int as usize];
        } else {
            return (*p).avl_data
        }
    }
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn tree_ulong_find(
    mut m: *const gsl_spmatrix_ulong,
    i: size_t,
    j: size_t,
) -> *mut libc::c_void {
    let mut table: *const gsl_bst_avl_table = &mut (*(*m).tree).table.avl_table
        as *mut gsl_bst_avl_table as *const gsl_bst_avl_table;
    let mut p: *mut gsl_bst_avl_node = 0 as *mut gsl_bst_avl_node;
    p = (*table).avl_root;
    while !p.is_null() {
        let mut n: size_t = ((*p).avl_data as *mut libc::c_ulong).offset_from((*m).data)
            as libc::c_long as size_t;
        let mut pi: libc::c_int = *((*m).i).offset(n as isize);
        let mut pj: libc::c_int = *((*m).p).offset(n as isize);
        let mut cmp: libc::c_int = if (i as libc::c_int) < pi {
            -(1 as libc::c_int)
        } else if i as libc::c_int > pi {
            1 as libc::c_int
        } else if (j as libc::c_int) < pj {
            -(1 as libc::c_int)
        } else {
            (j as libc::c_int > pj) as libc::c_int
        };
        if cmp < 0 as libc::c_int {
            p = (*p).avl_link[0 as libc::c_int as usize];
        } else if cmp > 0 as libc::c_int {
            p = (*p).avl_link[1 as libc::c_int as usize];
        } else {
            return (*p).avl_data
        }
    }
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn tree_uchar_find(
    mut m: *const gsl_spmatrix_uchar,
    i: size_t,
    j: size_t,
) -> *mut libc::c_void {
    let mut table: *const gsl_bst_avl_table = &mut (*(*m).tree).table.avl_table
        as *mut gsl_bst_avl_table as *const gsl_bst_avl_table;
    let mut p: *mut gsl_bst_avl_node = 0 as *mut gsl_bst_avl_node;
    p = (*table).avl_root;
    while !p.is_null() {
        let mut n: size_t = ((*p).avl_data as *mut libc::c_uchar).offset_from((*m).data)
            as libc::c_long as size_t;
        let mut pi: libc::c_int = *((*m).i).offset(n as isize);
        let mut pj: libc::c_int = *((*m).p).offset(n as isize);
        let mut cmp: libc::c_int = if (i as libc::c_int) < pi {
            -(1 as libc::c_int)
        } else if i as libc::c_int > pi {
            1 as libc::c_int
        } else if (j as libc::c_int) < pj {
            -(1 as libc::c_int)
        } else {
            (j as libc::c_int > pj) as libc::c_int
        };
        if cmp < 0 as libc::c_int {
            p = (*p).avl_link[0 as libc::c_int as usize];
        } else if cmp > 0 as libc::c_int {
            p = (*p).avl_link[1 as libc::c_int as usize];
        } else {
            return (*p).avl_data
        }
    }
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn tree_long_find(
    mut m: *const gsl_spmatrix_long,
    i: size_t,
    j: size_t,
) -> *mut libc::c_void {
    let mut table: *const gsl_bst_avl_table = &mut (*(*m).tree).table.avl_table
        as *mut gsl_bst_avl_table as *const gsl_bst_avl_table;
    let mut p: *mut gsl_bst_avl_node = 0 as *mut gsl_bst_avl_node;
    p = (*table).avl_root;
    while !p.is_null() {
        let mut n: size_t = ((*p).avl_data as *mut libc::c_long).offset_from((*m).data)
            as libc::c_long as size_t;
        let mut pi: libc::c_int = *((*m).i).offset(n as isize);
        let mut pj: libc::c_int = *((*m).p).offset(n as isize);
        let mut cmp: libc::c_int = if (i as libc::c_int) < pi {
            -(1 as libc::c_int)
        } else if i as libc::c_int > pi {
            1 as libc::c_int
        } else if (j as libc::c_int) < pj {
            -(1 as libc::c_int)
        } else {
            (j as libc::c_int > pj) as libc::c_int
        };
        if cmp < 0 as libc::c_int {
            p = (*p).avl_link[0 as libc::c_int as usize];
        } else if cmp > 0 as libc::c_int {
            p = (*p).avl_link[1 as libc::c_int as usize];
        } else {
            return (*p).avl_data
        }
    }
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn tree_float_find(
    mut m: *const gsl_spmatrix_float,
    i: size_t,
    j: size_t,
) -> *mut libc::c_void {
    let mut table: *const gsl_bst_avl_table = &mut (*(*m).tree).table.avl_table
        as *mut gsl_bst_avl_table as *const gsl_bst_avl_table;
    let mut p: *mut gsl_bst_avl_node = 0 as *mut gsl_bst_avl_node;
    p = (*table).avl_root;
    while !p.is_null() {
        let mut n: size_t = ((*p).avl_data as *mut libc::c_float).offset_from((*m).data)
            as libc::c_long as size_t;
        let mut pi: libc::c_int = *((*m).i).offset(n as isize);
        let mut pj: libc::c_int = *((*m).p).offset(n as isize);
        let mut cmp: libc::c_int = if (i as libc::c_int) < pi {
            -(1 as libc::c_int)
        } else if i as libc::c_int > pi {
            1 as libc::c_int
        } else if (j as libc::c_int) < pj {
            -(1 as libc::c_int)
        } else {
            (j as libc::c_int > pj) as libc::c_int
        };
        if cmp < 0 as libc::c_int {
            p = (*p).avl_link[0 as libc::c_int as usize];
        } else if cmp > 0 as libc::c_int {
            p = (*p).avl_link[1 as libc::c_int as usize];
        } else {
            return (*p).avl_data
        }
    }
    return 0 as *mut libc::c_void;
}
