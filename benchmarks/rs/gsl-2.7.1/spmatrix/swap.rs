use ::libc;
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
    fn gsl_spmatrix_complex_long_double_tree_rebuild(
        m: *mut gsl_spmatrix_complex_long_double,
    ) -> libc::c_int;
    fn gsl_spmatrix_complex_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_complex,
    ) -> libc::c_int;
    fn gsl_spmatrix_complex_tree_rebuild(m: *mut gsl_spmatrix_complex) -> libc::c_int;
    fn gsl_spmatrix_cumsum(n: size_t, c: *mut libc::c_int);
    fn gsl_spmatrix_complex_float_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_complex_float,
    ) -> libc::c_int;
    fn gsl_spmatrix_complex_float_tree_rebuild(
        m: *mut gsl_spmatrix_complex_float,
    ) -> libc::c_int;
    fn gsl_spmatrix_long_double_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_long_double,
    ) -> libc::c_int;
    fn gsl_spmatrix_long_double_tree_rebuild(
        m: *mut gsl_spmatrix_long_double,
    ) -> libc::c_int;
    fn gsl_spmatrix_char_tree_rebuild(m: *mut gsl_spmatrix_char) -> libc::c_int;
    fn gsl_spmatrix_realloc(nzmax: size_t, m: *mut gsl_spmatrix) -> libc::c_int;
    fn gsl_spmatrix_tree_rebuild(m: *mut gsl_spmatrix) -> libc::c_int;
    fn gsl_spmatrix_char_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_char,
    ) -> libc::c_int;
    fn gsl_spmatrix_float_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_float,
    ) -> libc::c_int;
    fn gsl_spmatrix_float_tree_rebuild(m: *mut gsl_spmatrix_float) -> libc::c_int;
    fn gsl_spmatrix_ulong_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_ulong,
    ) -> libc::c_int;
    fn gsl_spmatrix_ulong_tree_rebuild(m: *mut gsl_spmatrix_ulong) -> libc::c_int;
    fn gsl_spmatrix_long_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_long,
    ) -> libc::c_int;
    fn gsl_spmatrix_long_tree_rebuild(m: *mut gsl_spmatrix_long) -> libc::c_int;
    fn gsl_spmatrix_uint_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_uint,
    ) -> libc::c_int;
    fn gsl_spmatrix_uint_tree_rebuild(m: *mut gsl_spmatrix_uint) -> libc::c_int;
    fn gsl_spmatrix_int_realloc(nzmax: size_t, m: *mut gsl_spmatrix_int) -> libc::c_int;
    fn gsl_spmatrix_int_tree_rebuild(m: *mut gsl_spmatrix_int) -> libc::c_int;
    fn gsl_spmatrix_ushort_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_ushort,
    ) -> libc::c_int;
    fn gsl_spmatrix_ushort_tree_rebuild(m: *mut gsl_spmatrix_ushort) -> libc::c_int;
    fn gsl_spmatrix_short_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_short,
    ) -> libc::c_int;
    fn gsl_spmatrix_short_tree_rebuild(m: *mut gsl_spmatrix_short) -> libc::c_int;
    fn gsl_spmatrix_uchar_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_uchar,
    ) -> libc::c_int;
    fn gsl_spmatrix_uchar_tree_rebuild(m: *mut gsl_spmatrix_uchar) -> libc::c_int;
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
pub unsafe extern "C" fn gsl_spmatrix_short_transpose(
    mut m: *mut gsl_spmatrix_short,
) -> libc::c_int {
    if (*m).size1 != (*m).size2 {
        let mut tmp: size_t = (*m).size1;
        (*m).size1 = (*m).size2;
        (*m).size2 = tmp;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let mut n: size_t = 0;
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            let mut tmp_0: libc::c_int = *((*m).p).offset(n as isize);
            *((*m).p).offset(n as isize) = *((*m).i).offset(n as isize);
            *((*m).i).offset(n as isize) = tmp_0;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_short_tree_rebuild(m);
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        (*m).sptype = GSL_SPMATRIX_CSR as libc::c_int;
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        (*m).sptype = GSL_SPMATRIX_CSC as libc::c_int;
    } else {
        gsl_error(
            b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_transpose(
    mut m: *mut gsl_spmatrix,
) -> libc::c_int {
    if (*m).size1 != (*m).size2 {
        let mut tmp: size_t = (*m).size1;
        (*m).size1 = (*m).size2;
        (*m).size2 = tmp;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let mut n: size_t = 0;
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            let mut tmp_0: libc::c_int = *((*m).p).offset(n as isize);
            *((*m).p).offset(n as isize) = *((*m).i).offset(n as isize);
            *((*m).i).offset(n as isize) = tmp_0;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_tree_rebuild(m);
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        (*m).sptype = GSL_SPMATRIX_CSR as libc::c_int;
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        (*m).sptype = GSL_SPMATRIX_CSC as libc::c_int;
    } else {
        gsl_error(
            b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_uint_transpose(
    mut m: *mut gsl_spmatrix_uint,
) -> libc::c_int {
    if (*m).size1 != (*m).size2 {
        let mut tmp: size_t = (*m).size1;
        (*m).size1 = (*m).size2;
        (*m).size2 = tmp;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let mut n: size_t = 0;
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            let mut tmp_0: libc::c_int = *((*m).p).offset(n as isize);
            *((*m).p).offset(n as isize) = *((*m).i).offset(n as isize);
            *((*m).i).offset(n as isize) = tmp_0;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_uint_tree_rebuild(m);
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        (*m).sptype = GSL_SPMATRIX_CSR as libc::c_int;
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        (*m).sptype = GSL_SPMATRIX_CSC as libc::c_int;
    } else {
        gsl_error(
            b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_ushort_transpose(
    mut m: *mut gsl_spmatrix_ushort,
) -> libc::c_int {
    if (*m).size1 != (*m).size2 {
        let mut tmp: size_t = (*m).size1;
        (*m).size1 = (*m).size2;
        (*m).size2 = tmp;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let mut n: size_t = 0;
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            let mut tmp_0: libc::c_int = *((*m).p).offset(n as isize);
            *((*m).p).offset(n as isize) = *((*m).i).offset(n as isize);
            *((*m).i).offset(n as isize) = tmp_0;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_ushort_tree_rebuild(m);
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        (*m).sptype = GSL_SPMATRIX_CSR as libc::c_int;
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        (*m).sptype = GSL_SPMATRIX_CSC as libc::c_int;
    } else {
        gsl_error(
            b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_char_transpose(
    mut m: *mut gsl_spmatrix_char,
) -> libc::c_int {
    if (*m).size1 != (*m).size2 {
        let mut tmp: size_t = (*m).size1;
        (*m).size1 = (*m).size2;
        (*m).size2 = tmp;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let mut n: size_t = 0;
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            let mut tmp_0: libc::c_int = *((*m).p).offset(n as isize);
            *((*m).p).offset(n as isize) = *((*m).i).offset(n as isize);
            *((*m).i).offset(n as isize) = tmp_0;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_char_tree_rebuild(m);
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        (*m).sptype = GSL_SPMATRIX_CSR as libc::c_int;
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        (*m).sptype = GSL_SPMATRIX_CSC as libc::c_int;
    } else {
        gsl_error(
            b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_float_transpose(
    mut m: *mut gsl_spmatrix_float,
) -> libc::c_int {
    if (*m).size1 != (*m).size2 {
        let mut tmp: size_t = (*m).size1;
        (*m).size1 = (*m).size2;
        (*m).size2 = tmp;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let mut n: size_t = 0;
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            let mut tmp_0: libc::c_int = *((*m).p).offset(n as isize);
            *((*m).p).offset(n as isize) = *((*m).i).offset(n as isize);
            *((*m).i).offset(n as isize) = tmp_0;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_float_tree_rebuild(m);
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        (*m).sptype = GSL_SPMATRIX_CSR as libc::c_int;
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        (*m).sptype = GSL_SPMATRIX_CSC as libc::c_int;
    } else {
        gsl_error(
            b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_ulong_transpose(
    mut m: *mut gsl_spmatrix_ulong,
) -> libc::c_int {
    if (*m).size1 != (*m).size2 {
        let mut tmp: size_t = (*m).size1;
        (*m).size1 = (*m).size2;
        (*m).size2 = tmp;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let mut n: size_t = 0;
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            let mut tmp_0: libc::c_int = *((*m).p).offset(n as isize);
            *((*m).p).offset(n as isize) = *((*m).i).offset(n as isize);
            *((*m).i).offset(n as isize) = tmp_0;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_ulong_tree_rebuild(m);
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        (*m).sptype = GSL_SPMATRIX_CSR as libc::c_int;
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        (*m).sptype = GSL_SPMATRIX_CSC as libc::c_int;
    } else {
        gsl_error(
            b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_uchar_transpose(
    mut m: *mut gsl_spmatrix_uchar,
) -> libc::c_int {
    if (*m).size1 != (*m).size2 {
        let mut tmp: size_t = (*m).size1;
        (*m).size1 = (*m).size2;
        (*m).size2 = tmp;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let mut n: size_t = 0;
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            let mut tmp_0: libc::c_int = *((*m).p).offset(n as isize);
            *((*m).p).offset(n as isize) = *((*m).i).offset(n as isize);
            *((*m).i).offset(n as isize) = tmp_0;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_uchar_tree_rebuild(m);
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        (*m).sptype = GSL_SPMATRIX_CSR as libc::c_int;
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        (*m).sptype = GSL_SPMATRIX_CSC as libc::c_int;
    } else {
        gsl_error(
            b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_int_transpose(
    mut m: *mut gsl_spmatrix_int,
) -> libc::c_int {
    if (*m).size1 != (*m).size2 {
        let mut tmp: size_t = (*m).size1;
        (*m).size1 = (*m).size2;
        (*m).size2 = tmp;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let mut n: size_t = 0;
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            let mut tmp_0: libc::c_int = *((*m).p).offset(n as isize);
            *((*m).p).offset(n as isize) = *((*m).i).offset(n as isize);
            *((*m).i).offset(n as isize) = tmp_0;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_int_tree_rebuild(m);
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        (*m).sptype = GSL_SPMATRIX_CSR as libc::c_int;
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        (*m).sptype = GSL_SPMATRIX_CSC as libc::c_int;
    } else {
        gsl_error(
            b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_transpose(
    mut m: *mut gsl_spmatrix_long_double,
) -> libc::c_int {
    if (*m).size1 != (*m).size2 {
        let mut tmp: size_t = (*m).size1;
        (*m).size1 = (*m).size2;
        (*m).size2 = tmp;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let mut n: size_t = 0;
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            let mut tmp_0: libc::c_int = *((*m).p).offset(n as isize);
            *((*m).p).offset(n as isize) = *((*m).i).offset(n as isize);
            *((*m).i).offset(n as isize) = tmp_0;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_long_double_tree_rebuild(m);
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        (*m).sptype = GSL_SPMATRIX_CSR as libc::c_int;
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        (*m).sptype = GSL_SPMATRIX_CSC as libc::c_int;
    } else {
        gsl_error(
            b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_long_transpose(
    mut m: *mut gsl_spmatrix_long,
) -> libc::c_int {
    if (*m).size1 != (*m).size2 {
        let mut tmp: size_t = (*m).size1;
        (*m).size1 = (*m).size2;
        (*m).size2 = tmp;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let mut n: size_t = 0;
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            let mut tmp_0: libc::c_int = *((*m).p).offset(n as isize);
            *((*m).p).offset(n as isize) = *((*m).i).offset(n as isize);
            *((*m).i).offset(n as isize) = tmp_0;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_long_tree_rebuild(m);
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        (*m).sptype = GSL_SPMATRIX_CSR as libc::c_int;
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        (*m).sptype = GSL_SPMATRIX_CSC as libc::c_int;
    } else {
        gsl_error(
            b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_transpose(
    mut m: *mut gsl_spmatrix_complex,
) -> libc::c_int {
    if (*m).size1 != (*m).size2 {
        let mut tmp: size_t = (*m).size1;
        (*m).size1 = (*m).size2;
        (*m).size2 = tmp;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let mut n: size_t = 0;
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            let mut tmp_0: libc::c_int = *((*m).p).offset(n as isize);
            *((*m).p).offset(n as isize) = *((*m).i).offset(n as isize);
            *((*m).i).offset(n as isize) = tmp_0;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_complex_tree_rebuild(m);
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        (*m).sptype = GSL_SPMATRIX_CSR as libc::c_int;
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        (*m).sptype = GSL_SPMATRIX_CSC as libc::c_int;
    } else {
        gsl_error(
            b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_float_transpose(
    mut m: *mut gsl_spmatrix_complex_float,
) -> libc::c_int {
    if (*m).size1 != (*m).size2 {
        let mut tmp: size_t = (*m).size1;
        (*m).size1 = (*m).size2;
        (*m).size2 = tmp;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let mut n: size_t = 0;
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            let mut tmp_0: libc::c_int = *((*m).p).offset(n as isize);
            *((*m).p).offset(n as isize) = *((*m).i).offset(n as isize);
            *((*m).i).offset(n as isize) = tmp_0;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_complex_float_tree_rebuild(m);
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        (*m).sptype = GSL_SPMATRIX_CSR as libc::c_int;
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        (*m).sptype = GSL_SPMATRIX_CSC as libc::c_int;
    } else {
        gsl_error(
            b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_long_double_transpose(
    mut m: *mut gsl_spmatrix_complex_long_double,
) -> libc::c_int {
    if (*m).size1 != (*m).size2 {
        let mut tmp: size_t = (*m).size1;
        (*m).size1 = (*m).size2;
        (*m).size2 = tmp;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let mut n: size_t = 0;
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            let mut tmp_0: libc::c_int = *((*m).p).offset(n as isize);
            *((*m).p).offset(n as isize) = *((*m).i).offset(n as isize);
            *((*m).i).offset(n as isize) = tmp_0;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_complex_long_double_tree_rebuild(m);
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        (*m).sptype = GSL_SPMATRIX_CSR as libc::c_int;
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        (*m).sptype = GSL_SPMATRIX_CSC as libc::c_int;
    } else {
        gsl_error(
            b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_short_transpose2(
    mut m: *mut gsl_spmatrix_short,
) -> libc::c_int {
    return gsl_spmatrix_short_transpose(m);
}
pub unsafe extern "C" fn gsl_spmatrix_char_transpose2(
    mut m: *mut gsl_spmatrix_char,
) -> libc::c_int {
    return gsl_spmatrix_char_transpose(m);
}
pub unsafe extern "C" fn gsl_spmatrix_complex_long_double_transpose2(
    mut m: *mut gsl_spmatrix_complex_long_double,
) -> libc::c_int {
    return gsl_spmatrix_complex_long_double_transpose(m);
}
pub unsafe extern "C" fn gsl_spmatrix_complex_float_transpose2(
    mut m: *mut gsl_spmatrix_complex_float,
) -> libc::c_int {
    return gsl_spmatrix_complex_float_transpose(m);
}
pub unsafe extern "C" fn gsl_spmatrix_long_transpose2(
    mut m: *mut gsl_spmatrix_long,
) -> libc::c_int {
    return gsl_spmatrix_long_transpose(m);
}
pub unsafe extern "C" fn gsl_spmatrix_int_transpose2(
    mut m: *mut gsl_spmatrix_int,
) -> libc::c_int {
    return gsl_spmatrix_int_transpose(m);
}
pub unsafe extern "C" fn gsl_spmatrix_complex_transpose2(
    mut m: *mut gsl_spmatrix_complex,
) -> libc::c_int {
    return gsl_spmatrix_complex_transpose(m);
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_transpose2(
    mut m: *mut gsl_spmatrix_long_double,
) -> libc::c_int {
    return gsl_spmatrix_long_double_transpose(m);
}
pub unsafe extern "C" fn gsl_spmatrix_ulong_transpose2(
    mut m: *mut gsl_spmatrix_ulong,
) -> libc::c_int {
    return gsl_spmatrix_ulong_transpose(m);
}
pub unsafe extern "C" fn gsl_spmatrix_ushort_transpose2(
    mut m: *mut gsl_spmatrix_ushort,
) -> libc::c_int {
    return gsl_spmatrix_ushort_transpose(m);
}
pub unsafe extern "C" fn gsl_spmatrix_float_transpose2(
    mut m: *mut gsl_spmatrix_float,
) -> libc::c_int {
    return gsl_spmatrix_float_transpose(m);
}
pub unsafe extern "C" fn gsl_spmatrix_uchar_transpose2(
    mut m: *mut gsl_spmatrix_uchar,
) -> libc::c_int {
    return gsl_spmatrix_uchar_transpose(m);
}
pub unsafe extern "C" fn gsl_spmatrix_transpose2(
    mut m: *mut gsl_spmatrix,
) -> libc::c_int {
    return gsl_spmatrix_transpose(m);
}
pub unsafe extern "C" fn gsl_spmatrix_uint_transpose2(
    mut m: *mut gsl_spmatrix_uint,
) -> libc::c_int {
    return gsl_spmatrix_uint_transpose(m);
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_transpose_memcpy(
    mut dest: *mut gsl_spmatrix_long_double,
    mut src: *const gsl_spmatrix_long_double,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    if M != (*dest).size2 || N != (*dest).size1 {
        gsl_error(
            b"dimensions of dest must be transpose of src matrix\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*dest).sptype != (*src).sptype {
        gsl_error(
            b"cannot copy matrices of different storage formats\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let nz: size_t = (*src).nz;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_long_double_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        if (*src).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut n: size_t = 0;
            let mut r: size_t = 0;
            let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
            n = 0 as libc::c_int as size_t;
            while n < nz {
                *((*dest).i).offset(n as isize) = *((*src).p).offset(n as isize);
                *((*dest).p).offset(n as isize) = *((*src).i).offset(n as isize);
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    *((*dest).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        ) = *((*src).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        );
                    r = r.wrapping_add(1);
                    r;
                }
                ptr = gsl_bst_insert(
                    &mut *((*dest).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize,
                        ) as *mut f128::f128 as *mut libc::c_void,
                    (*dest).tree,
                );
                if !ptr.is_null() {
                    gsl_error(
                        b"detected duplicate entry\0" as *const u8
                            as *const libc::c_char,
                        b"./swap_source.c\0" as *const u8 as *const libc::c_char,
                        115 as libc::c_int,
                        GSL_EINVAL as libc::c_int,
                    );
                    return GSL_EINVAL as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ai: *mut libc::c_int = (*src).i;
            let mut Ap: *mut libc::c_int = (*src).p;
            let mut Ad: *mut f128::f128 = (*src).data;
            let mut ATi: *mut libc::c_int = (*dest).i;
            let mut ATp: *mut libc::c_int = (*dest).p;
            let mut ATd: *mut f128::f128 = (*dest).data;
            let mut w: *mut libc::c_int = (*dest).work.work_int;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            let mut r_0: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < M.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *ATp.offset(j as isize) = 0 as libc::c_int;
                j = j.wrapping_add(1);
                j;
            }
            j = 0 as libc::c_int as size_t;
            while j < nz {
                let ref mut fresh0 = *ATp.offset(*Ai.offset(j as isize) as isize);
                *fresh0 += 1;
                *fresh0;
                j = j.wrapping_add(1);
                j;
            }
            gsl_spmatrix_cumsum(M, ATp);
            j = 0 as libc::c_int as size_t;
            while j < M {
                *w.offset(j as isize) = *ATp.offset(j as isize);
                j = j.wrapping_add(1);
                j;
            }
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh1 = *w.offset(*Ai.offset(p as isize) as isize);
                    let fresh2 = *fresh1;
                    *fresh1 = *fresh1 + 1;
                    let mut k: libc::c_int = fresh2;
                    *ATi.offset(k as isize) = j as libc::c_int;
                    r_0 = 0 as libc::c_int as size_t;
                    while r_0 < 1 as libc::c_int as libc::c_ulong {
                        *ATd
                            .offset(
                                ((1 as libc::c_int * k) as libc::c_ulong).wrapping_add(r_0)
                                    as isize,
                            ) = *Ad
                            .offset(
                                ((1 as libc::c_int * p) as libc::c_ulong).wrapping_add(r_0)
                                    as isize,
                            );
                        r_0 = r_0.wrapping_add(1);
                        r_0;
                    }
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Aj: *mut libc::c_int = (*src).i;
            let mut Ap_0: *mut libc::c_int = (*src).p;
            let mut Ad_0: *mut f128::f128 = (*src).data;
            let mut ATj: *mut libc::c_int = (*dest).i;
            let mut ATp_0: *mut libc::c_int = (*dest).p;
            let mut ATd_0: *mut f128::f128 = (*dest).data;
            let mut w_0: *mut libc::c_int = (*dest).work.work_int;
            let mut p_0: libc::c_int = 0;
            let mut i: size_t = 0;
            let mut r_1: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < N.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *ATp_0.offset(i as isize) = 0 as libc::c_int;
                i = i.wrapping_add(1);
                i;
            }
            i = 0 as libc::c_int as size_t;
            while i < nz {
                let ref mut fresh3 = *ATp_0.offset(*Aj.offset(i as isize) as isize);
                *fresh3 += 1;
                *fresh3;
                i = i.wrapping_add(1);
                i;
            }
            gsl_spmatrix_cumsum(N, ATp_0);
            i = 0 as libc::c_int as size_t;
            while i < N {
                *w_0.offset(i as isize) = *ATp_0.offset(i as isize);
                i = i.wrapping_add(1);
                i;
            }
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *Ap_0.offset(i as isize);
                while p_0
                    < *Ap_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh4 = *w_0.offset(*Aj.offset(p_0 as isize) as isize);
                    let fresh5 = *fresh4;
                    *fresh4 = *fresh4 + 1;
                    let mut k_0: size_t = fresh5 as size_t;
                    *ATj.offset(k_0 as isize) = i as libc::c_int;
                    r_1 = 0 as libc::c_int as size_t;
                    while r_1 < 1 as libc::c_int as libc::c_ulong {
                        *ATd_0
                            .offset(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(k_0)
                                    .wrapping_add(r_1) as isize,
                            ) = *Ad_0
                            .offset(
                                ((1 as libc::c_int * p_0) as libc::c_ulong)
                                    .wrapping_add(r_1) as isize,
                            );
                        r_1 = r_1.wrapping_add(1);
                        r_1;
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
                b"./swap_source.c\0" as *const u8 as *const libc::c_char,
                199 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        (*dest).nz = nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_char_transpose_memcpy(
    mut dest: *mut gsl_spmatrix_char,
    mut src: *const gsl_spmatrix_char,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    if M != (*dest).size2 || N != (*dest).size1 {
        gsl_error(
            b"dimensions of dest must be transpose of src matrix\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*dest).sptype != (*src).sptype {
        gsl_error(
            b"cannot copy matrices of different storage formats\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let nz: size_t = (*src).nz;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_char_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        if (*src).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut n: size_t = 0;
            let mut r: size_t = 0;
            let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
            n = 0 as libc::c_int as size_t;
            while n < nz {
                *((*dest).i).offset(n as isize) = *((*src).p).offset(n as isize);
                *((*dest).p).offset(n as isize) = *((*src).i).offset(n as isize);
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    *((*dest).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        ) = *((*src).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        );
                    r = r.wrapping_add(1);
                    r;
                }
                ptr = gsl_bst_insert(
                    &mut *((*dest).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize,
                        ) as *mut libc::c_char as *mut libc::c_void,
                    (*dest).tree,
                );
                if !ptr.is_null() {
                    gsl_error(
                        b"detected duplicate entry\0" as *const u8
                            as *const libc::c_char,
                        b"./swap_source.c\0" as *const u8 as *const libc::c_char,
                        115 as libc::c_int,
                        GSL_EINVAL as libc::c_int,
                    );
                    return GSL_EINVAL as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ai: *mut libc::c_int = (*src).i;
            let mut Ap: *mut libc::c_int = (*src).p;
            let mut Ad: *mut libc::c_char = (*src).data;
            let mut ATi: *mut libc::c_int = (*dest).i;
            let mut ATp: *mut libc::c_int = (*dest).p;
            let mut ATd: *mut libc::c_char = (*dest).data;
            let mut w: *mut libc::c_int = (*dest).work.work_int;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            let mut r_0: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < M.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *ATp.offset(j as isize) = 0 as libc::c_int;
                j = j.wrapping_add(1);
                j;
            }
            j = 0 as libc::c_int as size_t;
            while j < nz {
                let ref mut fresh6 = *ATp.offset(*Ai.offset(j as isize) as isize);
                *fresh6 += 1;
                *fresh6;
                j = j.wrapping_add(1);
                j;
            }
            gsl_spmatrix_cumsum(M, ATp);
            j = 0 as libc::c_int as size_t;
            while j < M {
                *w.offset(j as isize) = *ATp.offset(j as isize);
                j = j.wrapping_add(1);
                j;
            }
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh7 = *w.offset(*Ai.offset(p as isize) as isize);
                    let fresh8 = *fresh7;
                    *fresh7 = *fresh7 + 1;
                    let mut k: libc::c_int = fresh8;
                    *ATi.offset(k as isize) = j as libc::c_int;
                    r_0 = 0 as libc::c_int as size_t;
                    while r_0 < 1 as libc::c_int as libc::c_ulong {
                        *ATd
                            .offset(
                                ((1 as libc::c_int * k) as libc::c_ulong).wrapping_add(r_0)
                                    as isize,
                            ) = *Ad
                            .offset(
                                ((1 as libc::c_int * p) as libc::c_ulong).wrapping_add(r_0)
                                    as isize,
                            );
                        r_0 = r_0.wrapping_add(1);
                        r_0;
                    }
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Aj: *mut libc::c_int = (*src).i;
            let mut Ap_0: *mut libc::c_int = (*src).p;
            let mut Ad_0: *mut libc::c_char = (*src).data;
            let mut ATj: *mut libc::c_int = (*dest).i;
            let mut ATp_0: *mut libc::c_int = (*dest).p;
            let mut ATd_0: *mut libc::c_char = (*dest).data;
            let mut w_0: *mut libc::c_int = (*dest).work.work_int;
            let mut p_0: libc::c_int = 0;
            let mut i: size_t = 0;
            let mut r_1: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < N.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *ATp_0.offset(i as isize) = 0 as libc::c_int;
                i = i.wrapping_add(1);
                i;
            }
            i = 0 as libc::c_int as size_t;
            while i < nz {
                let ref mut fresh9 = *ATp_0.offset(*Aj.offset(i as isize) as isize);
                *fresh9 += 1;
                *fresh9;
                i = i.wrapping_add(1);
                i;
            }
            gsl_spmatrix_cumsum(N, ATp_0);
            i = 0 as libc::c_int as size_t;
            while i < N {
                *w_0.offset(i as isize) = *ATp_0.offset(i as isize);
                i = i.wrapping_add(1);
                i;
            }
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *Ap_0.offset(i as isize);
                while p_0
                    < *Ap_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh10 = *w_0.offset(*Aj.offset(p_0 as isize) as isize);
                    let fresh11 = *fresh10;
                    *fresh10 = *fresh10 + 1;
                    let mut k_0: size_t = fresh11 as size_t;
                    *ATj.offset(k_0 as isize) = i as libc::c_int;
                    r_1 = 0 as libc::c_int as size_t;
                    while r_1 < 1 as libc::c_int as libc::c_ulong {
                        *ATd_0
                            .offset(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(k_0)
                                    .wrapping_add(r_1) as isize,
                            ) = *Ad_0
                            .offset(
                                ((1 as libc::c_int * p_0) as libc::c_ulong)
                                    .wrapping_add(r_1) as isize,
                            );
                        r_1 = r_1.wrapping_add(1);
                        r_1;
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
                b"./swap_source.c\0" as *const u8 as *const libc::c_char,
                199 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        (*dest).nz = nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_short_transpose_memcpy(
    mut dest: *mut gsl_spmatrix_short,
    mut src: *const gsl_spmatrix_short,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    if M != (*dest).size2 || N != (*dest).size1 {
        gsl_error(
            b"dimensions of dest must be transpose of src matrix\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*dest).sptype != (*src).sptype {
        gsl_error(
            b"cannot copy matrices of different storage formats\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let nz: size_t = (*src).nz;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_short_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        if (*src).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut n: size_t = 0;
            let mut r: size_t = 0;
            let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
            n = 0 as libc::c_int as size_t;
            while n < nz {
                *((*dest).i).offset(n as isize) = *((*src).p).offset(n as isize);
                *((*dest).p).offset(n as isize) = *((*src).i).offset(n as isize);
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    *((*dest).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        ) = *((*src).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        );
                    r = r.wrapping_add(1);
                    r;
                }
                ptr = gsl_bst_insert(
                    &mut *((*dest).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize,
                        ) as *mut libc::c_short as *mut libc::c_void,
                    (*dest).tree,
                );
                if !ptr.is_null() {
                    gsl_error(
                        b"detected duplicate entry\0" as *const u8
                            as *const libc::c_char,
                        b"./swap_source.c\0" as *const u8 as *const libc::c_char,
                        115 as libc::c_int,
                        GSL_EINVAL as libc::c_int,
                    );
                    return GSL_EINVAL as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ai: *mut libc::c_int = (*src).i;
            let mut Ap: *mut libc::c_int = (*src).p;
            let mut Ad: *mut libc::c_short = (*src).data;
            let mut ATi: *mut libc::c_int = (*dest).i;
            let mut ATp: *mut libc::c_int = (*dest).p;
            let mut ATd: *mut libc::c_short = (*dest).data;
            let mut w: *mut libc::c_int = (*dest).work.work_int;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            let mut r_0: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < M.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *ATp.offset(j as isize) = 0 as libc::c_int;
                j = j.wrapping_add(1);
                j;
            }
            j = 0 as libc::c_int as size_t;
            while j < nz {
                let ref mut fresh12 = *ATp.offset(*Ai.offset(j as isize) as isize);
                *fresh12 += 1;
                *fresh12;
                j = j.wrapping_add(1);
                j;
            }
            gsl_spmatrix_cumsum(M, ATp);
            j = 0 as libc::c_int as size_t;
            while j < M {
                *w.offset(j as isize) = *ATp.offset(j as isize);
                j = j.wrapping_add(1);
                j;
            }
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh13 = *w.offset(*Ai.offset(p as isize) as isize);
                    let fresh14 = *fresh13;
                    *fresh13 = *fresh13 + 1;
                    let mut k: libc::c_int = fresh14;
                    *ATi.offset(k as isize) = j as libc::c_int;
                    r_0 = 0 as libc::c_int as size_t;
                    while r_0 < 1 as libc::c_int as libc::c_ulong {
                        *ATd
                            .offset(
                                ((1 as libc::c_int * k) as libc::c_ulong).wrapping_add(r_0)
                                    as isize,
                            ) = *Ad
                            .offset(
                                ((1 as libc::c_int * p) as libc::c_ulong).wrapping_add(r_0)
                                    as isize,
                            );
                        r_0 = r_0.wrapping_add(1);
                        r_0;
                    }
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Aj: *mut libc::c_int = (*src).i;
            let mut Ap_0: *mut libc::c_int = (*src).p;
            let mut Ad_0: *mut libc::c_short = (*src).data;
            let mut ATj: *mut libc::c_int = (*dest).i;
            let mut ATp_0: *mut libc::c_int = (*dest).p;
            let mut ATd_0: *mut libc::c_short = (*dest).data;
            let mut w_0: *mut libc::c_int = (*dest).work.work_int;
            let mut p_0: libc::c_int = 0;
            let mut i: size_t = 0;
            let mut r_1: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < N.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *ATp_0.offset(i as isize) = 0 as libc::c_int;
                i = i.wrapping_add(1);
                i;
            }
            i = 0 as libc::c_int as size_t;
            while i < nz {
                let ref mut fresh15 = *ATp_0.offset(*Aj.offset(i as isize) as isize);
                *fresh15 += 1;
                *fresh15;
                i = i.wrapping_add(1);
                i;
            }
            gsl_spmatrix_cumsum(N, ATp_0);
            i = 0 as libc::c_int as size_t;
            while i < N {
                *w_0.offset(i as isize) = *ATp_0.offset(i as isize);
                i = i.wrapping_add(1);
                i;
            }
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *Ap_0.offset(i as isize);
                while p_0
                    < *Ap_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh16 = *w_0.offset(*Aj.offset(p_0 as isize) as isize);
                    let fresh17 = *fresh16;
                    *fresh16 = *fresh16 + 1;
                    let mut k_0: size_t = fresh17 as size_t;
                    *ATj.offset(k_0 as isize) = i as libc::c_int;
                    r_1 = 0 as libc::c_int as size_t;
                    while r_1 < 1 as libc::c_int as libc::c_ulong {
                        *ATd_0
                            .offset(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(k_0)
                                    .wrapping_add(r_1) as isize,
                            ) = *Ad_0
                            .offset(
                                ((1 as libc::c_int * p_0) as libc::c_ulong)
                                    .wrapping_add(r_1) as isize,
                            );
                        r_1 = r_1.wrapping_add(1);
                        r_1;
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
                b"./swap_source.c\0" as *const u8 as *const libc::c_char,
                199 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        (*dest).nz = nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uint_transpose_memcpy(
    mut dest: *mut gsl_spmatrix_uint,
    mut src: *const gsl_spmatrix_uint,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    if M != (*dest).size2 || N != (*dest).size1 {
        gsl_error(
            b"dimensions of dest must be transpose of src matrix\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*dest).sptype != (*src).sptype {
        gsl_error(
            b"cannot copy matrices of different storage formats\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let nz: size_t = (*src).nz;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_uint_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        if (*src).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut n: size_t = 0;
            let mut r: size_t = 0;
            let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
            n = 0 as libc::c_int as size_t;
            while n < nz {
                *((*dest).i).offset(n as isize) = *((*src).p).offset(n as isize);
                *((*dest).p).offset(n as isize) = *((*src).i).offset(n as isize);
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    *((*dest).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        ) = *((*src).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        );
                    r = r.wrapping_add(1);
                    r;
                }
                ptr = gsl_bst_insert(
                    &mut *((*dest).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize,
                        ) as *mut libc::c_uint as *mut libc::c_void,
                    (*dest).tree,
                );
                if !ptr.is_null() {
                    gsl_error(
                        b"detected duplicate entry\0" as *const u8
                            as *const libc::c_char,
                        b"./swap_source.c\0" as *const u8 as *const libc::c_char,
                        115 as libc::c_int,
                        GSL_EINVAL as libc::c_int,
                    );
                    return GSL_EINVAL as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ai: *mut libc::c_int = (*src).i;
            let mut Ap: *mut libc::c_int = (*src).p;
            let mut Ad: *mut libc::c_uint = (*src).data;
            let mut ATi: *mut libc::c_int = (*dest).i;
            let mut ATp: *mut libc::c_int = (*dest).p;
            let mut ATd: *mut libc::c_uint = (*dest).data;
            let mut w: *mut libc::c_int = (*dest).work.work_int;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            let mut r_0: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < M.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *ATp.offset(j as isize) = 0 as libc::c_int;
                j = j.wrapping_add(1);
                j;
            }
            j = 0 as libc::c_int as size_t;
            while j < nz {
                let ref mut fresh18 = *ATp.offset(*Ai.offset(j as isize) as isize);
                *fresh18 += 1;
                *fresh18;
                j = j.wrapping_add(1);
                j;
            }
            gsl_spmatrix_cumsum(M, ATp);
            j = 0 as libc::c_int as size_t;
            while j < M {
                *w.offset(j as isize) = *ATp.offset(j as isize);
                j = j.wrapping_add(1);
                j;
            }
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh19 = *w.offset(*Ai.offset(p as isize) as isize);
                    let fresh20 = *fresh19;
                    *fresh19 = *fresh19 + 1;
                    let mut k: libc::c_int = fresh20;
                    *ATi.offset(k as isize) = j as libc::c_int;
                    r_0 = 0 as libc::c_int as size_t;
                    while r_0 < 1 as libc::c_int as libc::c_ulong {
                        *ATd
                            .offset(
                                ((1 as libc::c_int * k) as libc::c_ulong).wrapping_add(r_0)
                                    as isize,
                            ) = *Ad
                            .offset(
                                ((1 as libc::c_int * p) as libc::c_ulong).wrapping_add(r_0)
                                    as isize,
                            );
                        r_0 = r_0.wrapping_add(1);
                        r_0;
                    }
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Aj: *mut libc::c_int = (*src).i;
            let mut Ap_0: *mut libc::c_int = (*src).p;
            let mut Ad_0: *mut libc::c_uint = (*src).data;
            let mut ATj: *mut libc::c_int = (*dest).i;
            let mut ATp_0: *mut libc::c_int = (*dest).p;
            let mut ATd_0: *mut libc::c_uint = (*dest).data;
            let mut w_0: *mut libc::c_int = (*dest).work.work_int;
            let mut p_0: libc::c_int = 0;
            let mut i: size_t = 0;
            let mut r_1: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < N.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *ATp_0.offset(i as isize) = 0 as libc::c_int;
                i = i.wrapping_add(1);
                i;
            }
            i = 0 as libc::c_int as size_t;
            while i < nz {
                let ref mut fresh21 = *ATp_0.offset(*Aj.offset(i as isize) as isize);
                *fresh21 += 1;
                *fresh21;
                i = i.wrapping_add(1);
                i;
            }
            gsl_spmatrix_cumsum(N, ATp_0);
            i = 0 as libc::c_int as size_t;
            while i < N {
                *w_0.offset(i as isize) = *ATp_0.offset(i as isize);
                i = i.wrapping_add(1);
                i;
            }
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *Ap_0.offset(i as isize);
                while p_0
                    < *Ap_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh22 = *w_0.offset(*Aj.offset(p_0 as isize) as isize);
                    let fresh23 = *fresh22;
                    *fresh22 = *fresh22 + 1;
                    let mut k_0: size_t = fresh23 as size_t;
                    *ATj.offset(k_0 as isize) = i as libc::c_int;
                    r_1 = 0 as libc::c_int as size_t;
                    while r_1 < 1 as libc::c_int as libc::c_ulong {
                        *ATd_0
                            .offset(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(k_0)
                                    .wrapping_add(r_1) as isize,
                            ) = *Ad_0
                            .offset(
                                ((1 as libc::c_int * p_0) as libc::c_ulong)
                                    .wrapping_add(r_1) as isize,
                            );
                        r_1 = r_1.wrapping_add(1);
                        r_1;
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
                b"./swap_source.c\0" as *const u8 as *const libc::c_char,
                199 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        (*dest).nz = nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_transpose_memcpy(
    mut dest: *mut gsl_spmatrix,
    mut src: *const gsl_spmatrix,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    if M != (*dest).size2 || N != (*dest).size1 {
        gsl_error(
            b"dimensions of dest must be transpose of src matrix\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*dest).sptype != (*src).sptype {
        gsl_error(
            b"cannot copy matrices of different storage formats\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let nz: size_t = (*src).nz;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        if (*src).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut n: size_t = 0;
            let mut r: size_t = 0;
            let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
            n = 0 as libc::c_int as size_t;
            while n < nz {
                *((*dest).i).offset(n as isize) = *((*src).p).offset(n as isize);
                *((*dest).p).offset(n as isize) = *((*src).i).offset(n as isize);
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    *((*dest).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        ) = *((*src).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        );
                    r = r.wrapping_add(1);
                    r;
                }
                ptr = gsl_bst_insert(
                    &mut *((*dest).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize,
                        ) as *mut libc::c_double as *mut libc::c_void,
                    (*dest).tree,
                );
                if !ptr.is_null() {
                    gsl_error(
                        b"detected duplicate entry\0" as *const u8
                            as *const libc::c_char,
                        b"./swap_source.c\0" as *const u8 as *const libc::c_char,
                        115 as libc::c_int,
                        GSL_EINVAL as libc::c_int,
                    );
                    return GSL_EINVAL as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ai: *mut libc::c_int = (*src).i;
            let mut Ap: *mut libc::c_int = (*src).p;
            let mut Ad: *mut libc::c_double = (*src).data;
            let mut ATi: *mut libc::c_int = (*dest).i;
            let mut ATp: *mut libc::c_int = (*dest).p;
            let mut ATd: *mut libc::c_double = (*dest).data;
            let mut w: *mut libc::c_int = (*dest).work.work_int;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            let mut r_0: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < M.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *ATp.offset(j as isize) = 0 as libc::c_int;
                j = j.wrapping_add(1);
                j;
            }
            j = 0 as libc::c_int as size_t;
            while j < nz {
                let ref mut fresh24 = *ATp.offset(*Ai.offset(j as isize) as isize);
                *fresh24 += 1;
                *fresh24;
                j = j.wrapping_add(1);
                j;
            }
            gsl_spmatrix_cumsum(M, ATp);
            j = 0 as libc::c_int as size_t;
            while j < M {
                *w.offset(j as isize) = *ATp.offset(j as isize);
                j = j.wrapping_add(1);
                j;
            }
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh25 = *w.offset(*Ai.offset(p as isize) as isize);
                    let fresh26 = *fresh25;
                    *fresh25 = *fresh25 + 1;
                    let mut k: libc::c_int = fresh26;
                    *ATi.offset(k as isize) = j as libc::c_int;
                    r_0 = 0 as libc::c_int as size_t;
                    while r_0 < 1 as libc::c_int as libc::c_ulong {
                        *ATd
                            .offset(
                                ((1 as libc::c_int * k) as libc::c_ulong).wrapping_add(r_0)
                                    as isize,
                            ) = *Ad
                            .offset(
                                ((1 as libc::c_int * p) as libc::c_ulong).wrapping_add(r_0)
                                    as isize,
                            );
                        r_0 = r_0.wrapping_add(1);
                        r_0;
                    }
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Aj: *mut libc::c_int = (*src).i;
            let mut Ap_0: *mut libc::c_int = (*src).p;
            let mut Ad_0: *mut libc::c_double = (*src).data;
            let mut ATj: *mut libc::c_int = (*dest).i;
            let mut ATp_0: *mut libc::c_int = (*dest).p;
            let mut ATd_0: *mut libc::c_double = (*dest).data;
            let mut w_0: *mut libc::c_int = (*dest).work.work_int;
            let mut p_0: libc::c_int = 0;
            let mut i: size_t = 0;
            let mut r_1: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < N.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *ATp_0.offset(i as isize) = 0 as libc::c_int;
                i = i.wrapping_add(1);
                i;
            }
            i = 0 as libc::c_int as size_t;
            while i < nz {
                let ref mut fresh27 = *ATp_0.offset(*Aj.offset(i as isize) as isize);
                *fresh27 += 1;
                *fresh27;
                i = i.wrapping_add(1);
                i;
            }
            gsl_spmatrix_cumsum(N, ATp_0);
            i = 0 as libc::c_int as size_t;
            while i < N {
                *w_0.offset(i as isize) = *ATp_0.offset(i as isize);
                i = i.wrapping_add(1);
                i;
            }
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *Ap_0.offset(i as isize);
                while p_0
                    < *Ap_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh28 = *w_0.offset(*Aj.offset(p_0 as isize) as isize);
                    let fresh29 = *fresh28;
                    *fresh28 = *fresh28 + 1;
                    let mut k_0: size_t = fresh29 as size_t;
                    *ATj.offset(k_0 as isize) = i as libc::c_int;
                    r_1 = 0 as libc::c_int as size_t;
                    while r_1 < 1 as libc::c_int as libc::c_ulong {
                        *ATd_0
                            .offset(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(k_0)
                                    .wrapping_add(r_1) as isize,
                            ) = *Ad_0
                            .offset(
                                ((1 as libc::c_int * p_0) as libc::c_ulong)
                                    .wrapping_add(r_1) as isize,
                            );
                        r_1 = r_1.wrapping_add(1);
                        r_1;
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
                b"./swap_source.c\0" as *const u8 as *const libc::c_char,
                199 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        (*dest).nz = nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uchar_transpose_memcpy(
    mut dest: *mut gsl_spmatrix_uchar,
    mut src: *const gsl_spmatrix_uchar,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    if M != (*dest).size2 || N != (*dest).size1 {
        gsl_error(
            b"dimensions of dest must be transpose of src matrix\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*dest).sptype != (*src).sptype {
        gsl_error(
            b"cannot copy matrices of different storage formats\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let nz: size_t = (*src).nz;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_uchar_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        if (*src).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut n: size_t = 0;
            let mut r: size_t = 0;
            let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
            n = 0 as libc::c_int as size_t;
            while n < nz {
                *((*dest).i).offset(n as isize) = *((*src).p).offset(n as isize);
                *((*dest).p).offset(n as isize) = *((*src).i).offset(n as isize);
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    *((*dest).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        ) = *((*src).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        );
                    r = r.wrapping_add(1);
                    r;
                }
                ptr = gsl_bst_insert(
                    &mut *((*dest).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize,
                        ) as *mut libc::c_uchar as *mut libc::c_void,
                    (*dest).tree,
                );
                if !ptr.is_null() {
                    gsl_error(
                        b"detected duplicate entry\0" as *const u8
                            as *const libc::c_char,
                        b"./swap_source.c\0" as *const u8 as *const libc::c_char,
                        115 as libc::c_int,
                        GSL_EINVAL as libc::c_int,
                    );
                    return GSL_EINVAL as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ai: *mut libc::c_int = (*src).i;
            let mut Ap: *mut libc::c_int = (*src).p;
            let mut Ad: *mut libc::c_uchar = (*src).data;
            let mut ATi: *mut libc::c_int = (*dest).i;
            let mut ATp: *mut libc::c_int = (*dest).p;
            let mut ATd: *mut libc::c_uchar = (*dest).data;
            let mut w: *mut libc::c_int = (*dest).work.work_int;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            let mut r_0: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < M.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *ATp.offset(j as isize) = 0 as libc::c_int;
                j = j.wrapping_add(1);
                j;
            }
            j = 0 as libc::c_int as size_t;
            while j < nz {
                let ref mut fresh30 = *ATp.offset(*Ai.offset(j as isize) as isize);
                *fresh30 += 1;
                *fresh30;
                j = j.wrapping_add(1);
                j;
            }
            gsl_spmatrix_cumsum(M, ATp);
            j = 0 as libc::c_int as size_t;
            while j < M {
                *w.offset(j as isize) = *ATp.offset(j as isize);
                j = j.wrapping_add(1);
                j;
            }
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh31 = *w.offset(*Ai.offset(p as isize) as isize);
                    let fresh32 = *fresh31;
                    *fresh31 = *fresh31 + 1;
                    let mut k: libc::c_int = fresh32;
                    *ATi.offset(k as isize) = j as libc::c_int;
                    r_0 = 0 as libc::c_int as size_t;
                    while r_0 < 1 as libc::c_int as libc::c_ulong {
                        *ATd
                            .offset(
                                ((1 as libc::c_int * k) as libc::c_ulong).wrapping_add(r_0)
                                    as isize,
                            ) = *Ad
                            .offset(
                                ((1 as libc::c_int * p) as libc::c_ulong).wrapping_add(r_0)
                                    as isize,
                            );
                        r_0 = r_0.wrapping_add(1);
                        r_0;
                    }
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Aj: *mut libc::c_int = (*src).i;
            let mut Ap_0: *mut libc::c_int = (*src).p;
            let mut Ad_0: *mut libc::c_uchar = (*src).data;
            let mut ATj: *mut libc::c_int = (*dest).i;
            let mut ATp_0: *mut libc::c_int = (*dest).p;
            let mut ATd_0: *mut libc::c_uchar = (*dest).data;
            let mut w_0: *mut libc::c_int = (*dest).work.work_int;
            let mut p_0: libc::c_int = 0;
            let mut i: size_t = 0;
            let mut r_1: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < N.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *ATp_0.offset(i as isize) = 0 as libc::c_int;
                i = i.wrapping_add(1);
                i;
            }
            i = 0 as libc::c_int as size_t;
            while i < nz {
                let ref mut fresh33 = *ATp_0.offset(*Aj.offset(i as isize) as isize);
                *fresh33 += 1;
                *fresh33;
                i = i.wrapping_add(1);
                i;
            }
            gsl_spmatrix_cumsum(N, ATp_0);
            i = 0 as libc::c_int as size_t;
            while i < N {
                *w_0.offset(i as isize) = *ATp_0.offset(i as isize);
                i = i.wrapping_add(1);
                i;
            }
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *Ap_0.offset(i as isize);
                while p_0
                    < *Ap_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh34 = *w_0.offset(*Aj.offset(p_0 as isize) as isize);
                    let fresh35 = *fresh34;
                    *fresh34 = *fresh34 + 1;
                    let mut k_0: size_t = fresh35 as size_t;
                    *ATj.offset(k_0 as isize) = i as libc::c_int;
                    r_1 = 0 as libc::c_int as size_t;
                    while r_1 < 1 as libc::c_int as libc::c_ulong {
                        *ATd_0
                            .offset(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(k_0)
                                    .wrapping_add(r_1) as isize,
                            ) = *Ad_0
                            .offset(
                                ((1 as libc::c_int * p_0) as libc::c_ulong)
                                    .wrapping_add(r_1) as isize,
                            );
                        r_1 = r_1.wrapping_add(1);
                        r_1;
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
                b"./swap_source.c\0" as *const u8 as *const libc::c_char,
                199 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        (*dest).nz = nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_ushort_transpose_memcpy(
    mut dest: *mut gsl_spmatrix_ushort,
    mut src: *const gsl_spmatrix_ushort,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    if M != (*dest).size2 || N != (*dest).size1 {
        gsl_error(
            b"dimensions of dest must be transpose of src matrix\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*dest).sptype != (*src).sptype {
        gsl_error(
            b"cannot copy matrices of different storage formats\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let nz: size_t = (*src).nz;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_ushort_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        if (*src).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut n: size_t = 0;
            let mut r: size_t = 0;
            let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
            n = 0 as libc::c_int as size_t;
            while n < nz {
                *((*dest).i).offset(n as isize) = *((*src).p).offset(n as isize);
                *((*dest).p).offset(n as isize) = *((*src).i).offset(n as isize);
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    *((*dest).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        ) = *((*src).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        );
                    r = r.wrapping_add(1);
                    r;
                }
                ptr = gsl_bst_insert(
                    &mut *((*dest).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize,
                        ) as *mut libc::c_ushort as *mut libc::c_void,
                    (*dest).tree,
                );
                if !ptr.is_null() {
                    gsl_error(
                        b"detected duplicate entry\0" as *const u8
                            as *const libc::c_char,
                        b"./swap_source.c\0" as *const u8 as *const libc::c_char,
                        115 as libc::c_int,
                        GSL_EINVAL as libc::c_int,
                    );
                    return GSL_EINVAL as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ai: *mut libc::c_int = (*src).i;
            let mut Ap: *mut libc::c_int = (*src).p;
            let mut Ad: *mut libc::c_ushort = (*src).data;
            let mut ATi: *mut libc::c_int = (*dest).i;
            let mut ATp: *mut libc::c_int = (*dest).p;
            let mut ATd: *mut libc::c_ushort = (*dest).data;
            let mut w: *mut libc::c_int = (*dest).work.work_int;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            let mut r_0: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < M.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *ATp.offset(j as isize) = 0 as libc::c_int;
                j = j.wrapping_add(1);
                j;
            }
            j = 0 as libc::c_int as size_t;
            while j < nz {
                let ref mut fresh36 = *ATp.offset(*Ai.offset(j as isize) as isize);
                *fresh36 += 1;
                *fresh36;
                j = j.wrapping_add(1);
                j;
            }
            gsl_spmatrix_cumsum(M, ATp);
            j = 0 as libc::c_int as size_t;
            while j < M {
                *w.offset(j as isize) = *ATp.offset(j as isize);
                j = j.wrapping_add(1);
                j;
            }
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh37 = *w.offset(*Ai.offset(p as isize) as isize);
                    let fresh38 = *fresh37;
                    *fresh37 = *fresh37 + 1;
                    let mut k: libc::c_int = fresh38;
                    *ATi.offset(k as isize) = j as libc::c_int;
                    r_0 = 0 as libc::c_int as size_t;
                    while r_0 < 1 as libc::c_int as libc::c_ulong {
                        *ATd
                            .offset(
                                ((1 as libc::c_int * k) as libc::c_ulong).wrapping_add(r_0)
                                    as isize,
                            ) = *Ad
                            .offset(
                                ((1 as libc::c_int * p) as libc::c_ulong).wrapping_add(r_0)
                                    as isize,
                            );
                        r_0 = r_0.wrapping_add(1);
                        r_0;
                    }
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Aj: *mut libc::c_int = (*src).i;
            let mut Ap_0: *mut libc::c_int = (*src).p;
            let mut Ad_0: *mut libc::c_ushort = (*src).data;
            let mut ATj: *mut libc::c_int = (*dest).i;
            let mut ATp_0: *mut libc::c_int = (*dest).p;
            let mut ATd_0: *mut libc::c_ushort = (*dest).data;
            let mut w_0: *mut libc::c_int = (*dest).work.work_int;
            let mut p_0: libc::c_int = 0;
            let mut i: size_t = 0;
            let mut r_1: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < N.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *ATp_0.offset(i as isize) = 0 as libc::c_int;
                i = i.wrapping_add(1);
                i;
            }
            i = 0 as libc::c_int as size_t;
            while i < nz {
                let ref mut fresh39 = *ATp_0.offset(*Aj.offset(i as isize) as isize);
                *fresh39 += 1;
                *fresh39;
                i = i.wrapping_add(1);
                i;
            }
            gsl_spmatrix_cumsum(N, ATp_0);
            i = 0 as libc::c_int as size_t;
            while i < N {
                *w_0.offset(i as isize) = *ATp_0.offset(i as isize);
                i = i.wrapping_add(1);
                i;
            }
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *Ap_0.offset(i as isize);
                while p_0
                    < *Ap_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh40 = *w_0.offset(*Aj.offset(p_0 as isize) as isize);
                    let fresh41 = *fresh40;
                    *fresh40 = *fresh40 + 1;
                    let mut k_0: size_t = fresh41 as size_t;
                    *ATj.offset(k_0 as isize) = i as libc::c_int;
                    r_1 = 0 as libc::c_int as size_t;
                    while r_1 < 1 as libc::c_int as libc::c_ulong {
                        *ATd_0
                            .offset(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(k_0)
                                    .wrapping_add(r_1) as isize,
                            ) = *Ad_0
                            .offset(
                                ((1 as libc::c_int * p_0) as libc::c_ulong)
                                    .wrapping_add(r_1) as isize,
                            );
                        r_1 = r_1.wrapping_add(1);
                        r_1;
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
                b"./swap_source.c\0" as *const u8 as *const libc::c_char,
                199 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        (*dest).nz = nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_transpose_memcpy(
    mut dest: *mut gsl_spmatrix_complex,
    mut src: *const gsl_spmatrix_complex,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    if M != (*dest).size2 || N != (*dest).size1 {
        gsl_error(
            b"dimensions of dest must be transpose of src matrix\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*dest).sptype != (*src).sptype {
        gsl_error(
            b"cannot copy matrices of different storage formats\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let nz: size_t = (*src).nz;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_complex_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        if (*src).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut n: size_t = 0;
            let mut r: size_t = 0;
            let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
            n = 0 as libc::c_int as size_t;
            while n < nz {
                *((*dest).i).offset(n as isize) = *((*src).p).offset(n as isize);
                *((*dest).p).offset(n as isize) = *((*src).i).offset(n as isize);
                r = 0 as libc::c_int as size_t;
                while r < 2 as libc::c_int as libc::c_ulong {
                    *((*dest).data)
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        ) = *((*src).data)
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        );
                    r = r.wrapping_add(1);
                    r;
                }
                ptr = gsl_bst_insert(
                    &mut *((*dest).data)
                        .offset(
                            (2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize,
                        ) as *mut libc::c_double as *mut libc::c_void,
                    (*dest).tree,
                );
                if !ptr.is_null() {
                    gsl_error(
                        b"detected duplicate entry\0" as *const u8
                            as *const libc::c_char,
                        b"./swap_source.c\0" as *const u8 as *const libc::c_char,
                        115 as libc::c_int,
                        GSL_EINVAL as libc::c_int,
                    );
                    return GSL_EINVAL as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ai: *mut libc::c_int = (*src).i;
            let mut Ap: *mut libc::c_int = (*src).p;
            let mut Ad: *mut libc::c_double = (*src).data;
            let mut ATi: *mut libc::c_int = (*dest).i;
            let mut ATp: *mut libc::c_int = (*dest).p;
            let mut ATd: *mut libc::c_double = (*dest).data;
            let mut w: *mut libc::c_int = (*dest).work.work_int;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            let mut r_0: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < M.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *ATp.offset(j as isize) = 0 as libc::c_int;
                j = j.wrapping_add(1);
                j;
            }
            j = 0 as libc::c_int as size_t;
            while j < nz {
                let ref mut fresh42 = *ATp.offset(*Ai.offset(j as isize) as isize);
                *fresh42 += 1;
                *fresh42;
                j = j.wrapping_add(1);
                j;
            }
            gsl_spmatrix_cumsum(M, ATp);
            j = 0 as libc::c_int as size_t;
            while j < M {
                *w.offset(j as isize) = *ATp.offset(j as isize);
                j = j.wrapping_add(1);
                j;
            }
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh43 = *w.offset(*Ai.offset(p as isize) as isize);
                    let fresh44 = *fresh43;
                    *fresh43 = *fresh43 + 1;
                    let mut k: libc::c_int = fresh44;
                    *ATi.offset(k as isize) = j as libc::c_int;
                    r_0 = 0 as libc::c_int as size_t;
                    while r_0 < 2 as libc::c_int as libc::c_ulong {
                        *ATd
                            .offset(
                                ((2 as libc::c_int * k) as libc::c_ulong).wrapping_add(r_0)
                                    as isize,
                            ) = *Ad
                            .offset(
                                ((2 as libc::c_int * p) as libc::c_ulong).wrapping_add(r_0)
                                    as isize,
                            );
                        r_0 = r_0.wrapping_add(1);
                        r_0;
                    }
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Aj: *mut libc::c_int = (*src).i;
            let mut Ap_0: *mut libc::c_int = (*src).p;
            let mut Ad_0: *mut libc::c_double = (*src).data;
            let mut ATj: *mut libc::c_int = (*dest).i;
            let mut ATp_0: *mut libc::c_int = (*dest).p;
            let mut ATd_0: *mut libc::c_double = (*dest).data;
            let mut w_0: *mut libc::c_int = (*dest).work.work_int;
            let mut p_0: libc::c_int = 0;
            let mut i: size_t = 0;
            let mut r_1: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < N.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *ATp_0.offset(i as isize) = 0 as libc::c_int;
                i = i.wrapping_add(1);
                i;
            }
            i = 0 as libc::c_int as size_t;
            while i < nz {
                let ref mut fresh45 = *ATp_0.offset(*Aj.offset(i as isize) as isize);
                *fresh45 += 1;
                *fresh45;
                i = i.wrapping_add(1);
                i;
            }
            gsl_spmatrix_cumsum(N, ATp_0);
            i = 0 as libc::c_int as size_t;
            while i < N {
                *w_0.offset(i as isize) = *ATp_0.offset(i as isize);
                i = i.wrapping_add(1);
                i;
            }
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *Ap_0.offset(i as isize);
                while p_0
                    < *Ap_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh46 = *w_0.offset(*Aj.offset(p_0 as isize) as isize);
                    let fresh47 = *fresh46;
                    *fresh46 = *fresh46 + 1;
                    let mut k_0: size_t = fresh47 as size_t;
                    *ATj.offset(k_0 as isize) = i as libc::c_int;
                    r_1 = 0 as libc::c_int as size_t;
                    while r_1 < 2 as libc::c_int as libc::c_ulong {
                        *ATd_0
                            .offset(
                                (2 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(k_0)
                                    .wrapping_add(r_1) as isize,
                            ) = *Ad_0
                            .offset(
                                ((2 as libc::c_int * p_0) as libc::c_ulong)
                                    .wrapping_add(r_1) as isize,
                            );
                        r_1 = r_1.wrapping_add(1);
                        r_1;
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
                b"./swap_source.c\0" as *const u8 as *const libc::c_char,
                199 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        (*dest).nz = nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_ulong_transpose_memcpy(
    mut dest: *mut gsl_spmatrix_ulong,
    mut src: *const gsl_spmatrix_ulong,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    if M != (*dest).size2 || N != (*dest).size1 {
        gsl_error(
            b"dimensions of dest must be transpose of src matrix\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*dest).sptype != (*src).sptype {
        gsl_error(
            b"cannot copy matrices of different storage formats\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let nz: size_t = (*src).nz;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_ulong_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        if (*src).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut n: size_t = 0;
            let mut r: size_t = 0;
            let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
            n = 0 as libc::c_int as size_t;
            while n < nz {
                *((*dest).i).offset(n as isize) = *((*src).p).offset(n as isize);
                *((*dest).p).offset(n as isize) = *((*src).i).offset(n as isize);
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    *((*dest).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        ) = *((*src).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        );
                    r = r.wrapping_add(1);
                    r;
                }
                ptr = gsl_bst_insert(
                    &mut *((*dest).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize,
                        ) as *mut libc::c_ulong as *mut libc::c_void,
                    (*dest).tree,
                );
                if !ptr.is_null() {
                    gsl_error(
                        b"detected duplicate entry\0" as *const u8
                            as *const libc::c_char,
                        b"./swap_source.c\0" as *const u8 as *const libc::c_char,
                        115 as libc::c_int,
                        GSL_EINVAL as libc::c_int,
                    );
                    return GSL_EINVAL as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ai: *mut libc::c_int = (*src).i;
            let mut Ap: *mut libc::c_int = (*src).p;
            let mut Ad: *mut libc::c_ulong = (*src).data;
            let mut ATi: *mut libc::c_int = (*dest).i;
            let mut ATp: *mut libc::c_int = (*dest).p;
            let mut ATd: *mut libc::c_ulong = (*dest).data;
            let mut w: *mut libc::c_int = (*dest).work.work_int;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            let mut r_0: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < M.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *ATp.offset(j as isize) = 0 as libc::c_int;
                j = j.wrapping_add(1);
                j;
            }
            j = 0 as libc::c_int as size_t;
            while j < nz {
                let ref mut fresh48 = *ATp.offset(*Ai.offset(j as isize) as isize);
                *fresh48 += 1;
                *fresh48;
                j = j.wrapping_add(1);
                j;
            }
            gsl_spmatrix_cumsum(M, ATp);
            j = 0 as libc::c_int as size_t;
            while j < M {
                *w.offset(j as isize) = *ATp.offset(j as isize);
                j = j.wrapping_add(1);
                j;
            }
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh49 = *w.offset(*Ai.offset(p as isize) as isize);
                    let fresh50 = *fresh49;
                    *fresh49 = *fresh49 + 1;
                    let mut k: libc::c_int = fresh50;
                    *ATi.offset(k as isize) = j as libc::c_int;
                    r_0 = 0 as libc::c_int as size_t;
                    while r_0 < 1 as libc::c_int as libc::c_ulong {
                        *ATd
                            .offset(
                                ((1 as libc::c_int * k) as libc::c_ulong).wrapping_add(r_0)
                                    as isize,
                            ) = *Ad
                            .offset(
                                ((1 as libc::c_int * p) as libc::c_ulong).wrapping_add(r_0)
                                    as isize,
                            );
                        r_0 = r_0.wrapping_add(1);
                        r_0;
                    }
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Aj: *mut libc::c_int = (*src).i;
            let mut Ap_0: *mut libc::c_int = (*src).p;
            let mut Ad_0: *mut libc::c_ulong = (*src).data;
            let mut ATj: *mut libc::c_int = (*dest).i;
            let mut ATp_0: *mut libc::c_int = (*dest).p;
            let mut ATd_0: *mut libc::c_ulong = (*dest).data;
            let mut w_0: *mut libc::c_int = (*dest).work.work_int;
            let mut p_0: libc::c_int = 0;
            let mut i: size_t = 0;
            let mut r_1: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < N.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *ATp_0.offset(i as isize) = 0 as libc::c_int;
                i = i.wrapping_add(1);
                i;
            }
            i = 0 as libc::c_int as size_t;
            while i < nz {
                let ref mut fresh51 = *ATp_0.offset(*Aj.offset(i as isize) as isize);
                *fresh51 += 1;
                *fresh51;
                i = i.wrapping_add(1);
                i;
            }
            gsl_spmatrix_cumsum(N, ATp_0);
            i = 0 as libc::c_int as size_t;
            while i < N {
                *w_0.offset(i as isize) = *ATp_0.offset(i as isize);
                i = i.wrapping_add(1);
                i;
            }
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *Ap_0.offset(i as isize);
                while p_0
                    < *Ap_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh52 = *w_0.offset(*Aj.offset(p_0 as isize) as isize);
                    let fresh53 = *fresh52;
                    *fresh52 = *fresh52 + 1;
                    let mut k_0: size_t = fresh53 as size_t;
                    *ATj.offset(k_0 as isize) = i as libc::c_int;
                    r_1 = 0 as libc::c_int as size_t;
                    while r_1 < 1 as libc::c_int as libc::c_ulong {
                        *ATd_0
                            .offset(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(k_0)
                                    .wrapping_add(r_1) as isize,
                            ) = *Ad_0
                            .offset(
                                ((1 as libc::c_int * p_0) as libc::c_ulong)
                                    .wrapping_add(r_1) as isize,
                            );
                        r_1 = r_1.wrapping_add(1);
                        r_1;
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
                b"./swap_source.c\0" as *const u8 as *const libc::c_char,
                199 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        (*dest).nz = nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_float_transpose_memcpy(
    mut dest: *mut gsl_spmatrix_complex_float,
    mut src: *const gsl_spmatrix_complex_float,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    if M != (*dest).size2 || N != (*dest).size1 {
        gsl_error(
            b"dimensions of dest must be transpose of src matrix\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*dest).sptype != (*src).sptype {
        gsl_error(
            b"cannot copy matrices of different storage formats\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let nz: size_t = (*src).nz;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_complex_float_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        if (*src).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut n: size_t = 0;
            let mut r: size_t = 0;
            let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
            n = 0 as libc::c_int as size_t;
            while n < nz {
                *((*dest).i).offset(n as isize) = *((*src).p).offset(n as isize);
                *((*dest).p).offset(n as isize) = *((*src).i).offset(n as isize);
                r = 0 as libc::c_int as size_t;
                while r < 2 as libc::c_int as libc::c_ulong {
                    *((*dest).data)
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        ) = *((*src).data)
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        );
                    r = r.wrapping_add(1);
                    r;
                }
                ptr = gsl_bst_insert(
                    &mut *((*dest).data)
                        .offset(
                            (2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize,
                        ) as *mut libc::c_float as *mut libc::c_void,
                    (*dest).tree,
                );
                if !ptr.is_null() {
                    gsl_error(
                        b"detected duplicate entry\0" as *const u8
                            as *const libc::c_char,
                        b"./swap_source.c\0" as *const u8 as *const libc::c_char,
                        115 as libc::c_int,
                        GSL_EINVAL as libc::c_int,
                    );
                    return GSL_EINVAL as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ai: *mut libc::c_int = (*src).i;
            let mut Ap: *mut libc::c_int = (*src).p;
            let mut Ad: *mut libc::c_float = (*src).data;
            let mut ATi: *mut libc::c_int = (*dest).i;
            let mut ATp: *mut libc::c_int = (*dest).p;
            let mut ATd: *mut libc::c_float = (*dest).data;
            let mut w: *mut libc::c_int = (*dest).work.work_int;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            let mut r_0: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < M.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *ATp.offset(j as isize) = 0 as libc::c_int;
                j = j.wrapping_add(1);
                j;
            }
            j = 0 as libc::c_int as size_t;
            while j < nz {
                let ref mut fresh54 = *ATp.offset(*Ai.offset(j as isize) as isize);
                *fresh54 += 1;
                *fresh54;
                j = j.wrapping_add(1);
                j;
            }
            gsl_spmatrix_cumsum(M, ATp);
            j = 0 as libc::c_int as size_t;
            while j < M {
                *w.offset(j as isize) = *ATp.offset(j as isize);
                j = j.wrapping_add(1);
                j;
            }
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh55 = *w.offset(*Ai.offset(p as isize) as isize);
                    let fresh56 = *fresh55;
                    *fresh55 = *fresh55 + 1;
                    let mut k: libc::c_int = fresh56;
                    *ATi.offset(k as isize) = j as libc::c_int;
                    r_0 = 0 as libc::c_int as size_t;
                    while r_0 < 2 as libc::c_int as libc::c_ulong {
                        *ATd
                            .offset(
                                ((2 as libc::c_int * k) as libc::c_ulong).wrapping_add(r_0)
                                    as isize,
                            ) = *Ad
                            .offset(
                                ((2 as libc::c_int * p) as libc::c_ulong).wrapping_add(r_0)
                                    as isize,
                            );
                        r_0 = r_0.wrapping_add(1);
                        r_0;
                    }
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Aj: *mut libc::c_int = (*src).i;
            let mut Ap_0: *mut libc::c_int = (*src).p;
            let mut Ad_0: *mut libc::c_float = (*src).data;
            let mut ATj: *mut libc::c_int = (*dest).i;
            let mut ATp_0: *mut libc::c_int = (*dest).p;
            let mut ATd_0: *mut libc::c_float = (*dest).data;
            let mut w_0: *mut libc::c_int = (*dest).work.work_int;
            let mut p_0: libc::c_int = 0;
            let mut i: size_t = 0;
            let mut r_1: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < N.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *ATp_0.offset(i as isize) = 0 as libc::c_int;
                i = i.wrapping_add(1);
                i;
            }
            i = 0 as libc::c_int as size_t;
            while i < nz {
                let ref mut fresh57 = *ATp_0.offset(*Aj.offset(i as isize) as isize);
                *fresh57 += 1;
                *fresh57;
                i = i.wrapping_add(1);
                i;
            }
            gsl_spmatrix_cumsum(N, ATp_0);
            i = 0 as libc::c_int as size_t;
            while i < N {
                *w_0.offset(i as isize) = *ATp_0.offset(i as isize);
                i = i.wrapping_add(1);
                i;
            }
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *Ap_0.offset(i as isize);
                while p_0
                    < *Ap_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh58 = *w_0.offset(*Aj.offset(p_0 as isize) as isize);
                    let fresh59 = *fresh58;
                    *fresh58 = *fresh58 + 1;
                    let mut k_0: size_t = fresh59 as size_t;
                    *ATj.offset(k_0 as isize) = i as libc::c_int;
                    r_1 = 0 as libc::c_int as size_t;
                    while r_1 < 2 as libc::c_int as libc::c_ulong {
                        *ATd_0
                            .offset(
                                (2 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(k_0)
                                    .wrapping_add(r_1) as isize,
                            ) = *Ad_0
                            .offset(
                                ((2 as libc::c_int * p_0) as libc::c_ulong)
                                    .wrapping_add(r_1) as isize,
                            );
                        r_1 = r_1.wrapping_add(1);
                        r_1;
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
                b"./swap_source.c\0" as *const u8 as *const libc::c_char,
                199 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        (*dest).nz = nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_int_transpose_memcpy(
    mut dest: *mut gsl_spmatrix_int,
    mut src: *const gsl_spmatrix_int,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    if M != (*dest).size2 || N != (*dest).size1 {
        gsl_error(
            b"dimensions of dest must be transpose of src matrix\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*dest).sptype != (*src).sptype {
        gsl_error(
            b"cannot copy matrices of different storage formats\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let nz: size_t = (*src).nz;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_int_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        if (*src).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut n: size_t = 0;
            let mut r: size_t = 0;
            let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
            n = 0 as libc::c_int as size_t;
            while n < nz {
                *((*dest).i).offset(n as isize) = *((*src).p).offset(n as isize);
                *((*dest).p).offset(n as isize) = *((*src).i).offset(n as isize);
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    *((*dest).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        ) = *((*src).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        );
                    r = r.wrapping_add(1);
                    r;
                }
                ptr = gsl_bst_insert(
                    &mut *((*dest).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize,
                        ) as *mut libc::c_int as *mut libc::c_void,
                    (*dest).tree,
                );
                if !ptr.is_null() {
                    gsl_error(
                        b"detected duplicate entry\0" as *const u8
                            as *const libc::c_char,
                        b"./swap_source.c\0" as *const u8 as *const libc::c_char,
                        115 as libc::c_int,
                        GSL_EINVAL as libc::c_int,
                    );
                    return GSL_EINVAL as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ai: *mut libc::c_int = (*src).i;
            let mut Ap: *mut libc::c_int = (*src).p;
            let mut Ad: *mut libc::c_int = (*src).data;
            let mut ATi: *mut libc::c_int = (*dest).i;
            let mut ATp: *mut libc::c_int = (*dest).p;
            let mut ATd: *mut libc::c_int = (*dest).data;
            let mut w: *mut libc::c_int = (*dest).work.work_int;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            let mut r_0: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < M.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *ATp.offset(j as isize) = 0 as libc::c_int;
                j = j.wrapping_add(1);
                j;
            }
            j = 0 as libc::c_int as size_t;
            while j < nz {
                let ref mut fresh60 = *ATp.offset(*Ai.offset(j as isize) as isize);
                *fresh60 += 1;
                *fresh60;
                j = j.wrapping_add(1);
                j;
            }
            gsl_spmatrix_cumsum(M, ATp);
            j = 0 as libc::c_int as size_t;
            while j < M {
                *w.offset(j as isize) = *ATp.offset(j as isize);
                j = j.wrapping_add(1);
                j;
            }
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh61 = *w.offset(*Ai.offset(p as isize) as isize);
                    let fresh62 = *fresh61;
                    *fresh61 = *fresh61 + 1;
                    let mut k: libc::c_int = fresh62;
                    *ATi.offset(k as isize) = j as libc::c_int;
                    r_0 = 0 as libc::c_int as size_t;
                    while r_0 < 1 as libc::c_int as libc::c_ulong {
                        *ATd
                            .offset(
                                ((1 as libc::c_int * k) as libc::c_ulong).wrapping_add(r_0)
                                    as isize,
                            ) = *Ad
                            .offset(
                                ((1 as libc::c_int * p) as libc::c_ulong).wrapping_add(r_0)
                                    as isize,
                            );
                        r_0 = r_0.wrapping_add(1);
                        r_0;
                    }
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Aj: *mut libc::c_int = (*src).i;
            let mut Ap_0: *mut libc::c_int = (*src).p;
            let mut Ad_0: *mut libc::c_int = (*src).data;
            let mut ATj: *mut libc::c_int = (*dest).i;
            let mut ATp_0: *mut libc::c_int = (*dest).p;
            let mut ATd_0: *mut libc::c_int = (*dest).data;
            let mut w_0: *mut libc::c_int = (*dest).work.work_int;
            let mut p_0: libc::c_int = 0;
            let mut i: size_t = 0;
            let mut r_1: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < N.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *ATp_0.offset(i as isize) = 0 as libc::c_int;
                i = i.wrapping_add(1);
                i;
            }
            i = 0 as libc::c_int as size_t;
            while i < nz {
                let ref mut fresh63 = *ATp_0.offset(*Aj.offset(i as isize) as isize);
                *fresh63 += 1;
                *fresh63;
                i = i.wrapping_add(1);
                i;
            }
            gsl_spmatrix_cumsum(N, ATp_0);
            i = 0 as libc::c_int as size_t;
            while i < N {
                *w_0.offset(i as isize) = *ATp_0.offset(i as isize);
                i = i.wrapping_add(1);
                i;
            }
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *Ap_0.offset(i as isize);
                while p_0
                    < *Ap_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh64 = *w_0.offset(*Aj.offset(p_0 as isize) as isize);
                    let fresh65 = *fresh64;
                    *fresh64 = *fresh64 + 1;
                    let mut k_0: size_t = fresh65 as size_t;
                    *ATj.offset(k_0 as isize) = i as libc::c_int;
                    r_1 = 0 as libc::c_int as size_t;
                    while r_1 < 1 as libc::c_int as libc::c_ulong {
                        *ATd_0
                            .offset(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(k_0)
                                    .wrapping_add(r_1) as isize,
                            ) = *Ad_0
                            .offset(
                                ((1 as libc::c_int * p_0) as libc::c_ulong)
                                    .wrapping_add(r_1) as isize,
                            );
                        r_1 = r_1.wrapping_add(1);
                        r_1;
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
                b"./swap_source.c\0" as *const u8 as *const libc::c_char,
                199 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        (*dest).nz = nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_long_double_transpose_memcpy(
    mut dest: *mut gsl_spmatrix_complex_long_double,
    mut src: *const gsl_spmatrix_complex_long_double,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    if M != (*dest).size2 || N != (*dest).size1 {
        gsl_error(
            b"dimensions of dest must be transpose of src matrix\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*dest).sptype != (*src).sptype {
        gsl_error(
            b"cannot copy matrices of different storage formats\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let nz: size_t = (*src).nz;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_complex_long_double_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        if (*src).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut n: size_t = 0;
            let mut r: size_t = 0;
            let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
            n = 0 as libc::c_int as size_t;
            while n < nz {
                *((*dest).i).offset(n as isize) = *((*src).p).offset(n as isize);
                *((*dest).p).offset(n as isize) = *((*src).i).offset(n as isize);
                r = 0 as libc::c_int as size_t;
                while r < 2 as libc::c_int as libc::c_ulong {
                    *((*dest).data)
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        ) = *((*src).data)
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        );
                    r = r.wrapping_add(1);
                    r;
                }
                ptr = gsl_bst_insert(
                    &mut *((*dest).data)
                        .offset(
                            (2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize,
                        ) as *mut f128::f128 as *mut libc::c_void,
                    (*dest).tree,
                );
                if !ptr.is_null() {
                    gsl_error(
                        b"detected duplicate entry\0" as *const u8
                            as *const libc::c_char,
                        b"./swap_source.c\0" as *const u8 as *const libc::c_char,
                        115 as libc::c_int,
                        GSL_EINVAL as libc::c_int,
                    );
                    return GSL_EINVAL as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ai: *mut libc::c_int = (*src).i;
            let mut Ap: *mut libc::c_int = (*src).p;
            let mut Ad: *mut f128::f128 = (*src).data;
            let mut ATi: *mut libc::c_int = (*dest).i;
            let mut ATp: *mut libc::c_int = (*dest).p;
            let mut ATd: *mut f128::f128 = (*dest).data;
            let mut w: *mut libc::c_int = (*dest).work.work_int;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            let mut r_0: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < M.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *ATp.offset(j as isize) = 0 as libc::c_int;
                j = j.wrapping_add(1);
                j;
            }
            j = 0 as libc::c_int as size_t;
            while j < nz {
                let ref mut fresh66 = *ATp.offset(*Ai.offset(j as isize) as isize);
                *fresh66 += 1;
                *fresh66;
                j = j.wrapping_add(1);
                j;
            }
            gsl_spmatrix_cumsum(M, ATp);
            j = 0 as libc::c_int as size_t;
            while j < M {
                *w.offset(j as isize) = *ATp.offset(j as isize);
                j = j.wrapping_add(1);
                j;
            }
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh67 = *w.offset(*Ai.offset(p as isize) as isize);
                    let fresh68 = *fresh67;
                    *fresh67 = *fresh67 + 1;
                    let mut k: libc::c_int = fresh68;
                    *ATi.offset(k as isize) = j as libc::c_int;
                    r_0 = 0 as libc::c_int as size_t;
                    while r_0 < 2 as libc::c_int as libc::c_ulong {
                        *ATd
                            .offset(
                                ((2 as libc::c_int * k) as libc::c_ulong).wrapping_add(r_0)
                                    as isize,
                            ) = *Ad
                            .offset(
                                ((2 as libc::c_int * p) as libc::c_ulong).wrapping_add(r_0)
                                    as isize,
                            );
                        r_0 = r_0.wrapping_add(1);
                        r_0;
                    }
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Aj: *mut libc::c_int = (*src).i;
            let mut Ap_0: *mut libc::c_int = (*src).p;
            let mut Ad_0: *mut f128::f128 = (*src).data;
            let mut ATj: *mut libc::c_int = (*dest).i;
            let mut ATp_0: *mut libc::c_int = (*dest).p;
            let mut ATd_0: *mut f128::f128 = (*dest).data;
            let mut w_0: *mut libc::c_int = (*dest).work.work_int;
            let mut p_0: libc::c_int = 0;
            let mut i: size_t = 0;
            let mut r_1: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < N.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *ATp_0.offset(i as isize) = 0 as libc::c_int;
                i = i.wrapping_add(1);
                i;
            }
            i = 0 as libc::c_int as size_t;
            while i < nz {
                let ref mut fresh69 = *ATp_0.offset(*Aj.offset(i as isize) as isize);
                *fresh69 += 1;
                *fresh69;
                i = i.wrapping_add(1);
                i;
            }
            gsl_spmatrix_cumsum(N, ATp_0);
            i = 0 as libc::c_int as size_t;
            while i < N {
                *w_0.offset(i as isize) = *ATp_0.offset(i as isize);
                i = i.wrapping_add(1);
                i;
            }
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *Ap_0.offset(i as isize);
                while p_0
                    < *Ap_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh70 = *w_0.offset(*Aj.offset(p_0 as isize) as isize);
                    let fresh71 = *fresh70;
                    *fresh70 = *fresh70 + 1;
                    let mut k_0: size_t = fresh71 as size_t;
                    *ATj.offset(k_0 as isize) = i as libc::c_int;
                    r_1 = 0 as libc::c_int as size_t;
                    while r_1 < 2 as libc::c_int as libc::c_ulong {
                        *ATd_0
                            .offset(
                                (2 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(k_0)
                                    .wrapping_add(r_1) as isize,
                            ) = *Ad_0
                            .offset(
                                ((2 as libc::c_int * p_0) as libc::c_ulong)
                                    .wrapping_add(r_1) as isize,
                            );
                        r_1 = r_1.wrapping_add(1);
                        r_1;
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
                b"./swap_source.c\0" as *const u8 as *const libc::c_char,
                199 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        (*dest).nz = nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_float_transpose_memcpy(
    mut dest: *mut gsl_spmatrix_float,
    mut src: *const gsl_spmatrix_float,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    if M != (*dest).size2 || N != (*dest).size1 {
        gsl_error(
            b"dimensions of dest must be transpose of src matrix\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*dest).sptype != (*src).sptype {
        gsl_error(
            b"cannot copy matrices of different storage formats\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let nz: size_t = (*src).nz;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_float_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        if (*src).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut n: size_t = 0;
            let mut r: size_t = 0;
            let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
            n = 0 as libc::c_int as size_t;
            while n < nz {
                *((*dest).i).offset(n as isize) = *((*src).p).offset(n as isize);
                *((*dest).p).offset(n as isize) = *((*src).i).offset(n as isize);
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    *((*dest).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        ) = *((*src).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        );
                    r = r.wrapping_add(1);
                    r;
                }
                ptr = gsl_bst_insert(
                    &mut *((*dest).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize,
                        ) as *mut libc::c_float as *mut libc::c_void,
                    (*dest).tree,
                );
                if !ptr.is_null() {
                    gsl_error(
                        b"detected duplicate entry\0" as *const u8
                            as *const libc::c_char,
                        b"./swap_source.c\0" as *const u8 as *const libc::c_char,
                        115 as libc::c_int,
                        GSL_EINVAL as libc::c_int,
                    );
                    return GSL_EINVAL as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ai: *mut libc::c_int = (*src).i;
            let mut Ap: *mut libc::c_int = (*src).p;
            let mut Ad: *mut libc::c_float = (*src).data;
            let mut ATi: *mut libc::c_int = (*dest).i;
            let mut ATp: *mut libc::c_int = (*dest).p;
            let mut ATd: *mut libc::c_float = (*dest).data;
            let mut w: *mut libc::c_int = (*dest).work.work_int;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            let mut r_0: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < M.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *ATp.offset(j as isize) = 0 as libc::c_int;
                j = j.wrapping_add(1);
                j;
            }
            j = 0 as libc::c_int as size_t;
            while j < nz {
                let ref mut fresh72 = *ATp.offset(*Ai.offset(j as isize) as isize);
                *fresh72 += 1;
                *fresh72;
                j = j.wrapping_add(1);
                j;
            }
            gsl_spmatrix_cumsum(M, ATp);
            j = 0 as libc::c_int as size_t;
            while j < M {
                *w.offset(j as isize) = *ATp.offset(j as isize);
                j = j.wrapping_add(1);
                j;
            }
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh73 = *w.offset(*Ai.offset(p as isize) as isize);
                    let fresh74 = *fresh73;
                    *fresh73 = *fresh73 + 1;
                    let mut k: libc::c_int = fresh74;
                    *ATi.offset(k as isize) = j as libc::c_int;
                    r_0 = 0 as libc::c_int as size_t;
                    while r_0 < 1 as libc::c_int as libc::c_ulong {
                        *ATd
                            .offset(
                                ((1 as libc::c_int * k) as libc::c_ulong).wrapping_add(r_0)
                                    as isize,
                            ) = *Ad
                            .offset(
                                ((1 as libc::c_int * p) as libc::c_ulong).wrapping_add(r_0)
                                    as isize,
                            );
                        r_0 = r_0.wrapping_add(1);
                        r_0;
                    }
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Aj: *mut libc::c_int = (*src).i;
            let mut Ap_0: *mut libc::c_int = (*src).p;
            let mut Ad_0: *mut libc::c_float = (*src).data;
            let mut ATj: *mut libc::c_int = (*dest).i;
            let mut ATp_0: *mut libc::c_int = (*dest).p;
            let mut ATd_0: *mut libc::c_float = (*dest).data;
            let mut w_0: *mut libc::c_int = (*dest).work.work_int;
            let mut p_0: libc::c_int = 0;
            let mut i: size_t = 0;
            let mut r_1: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < N.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *ATp_0.offset(i as isize) = 0 as libc::c_int;
                i = i.wrapping_add(1);
                i;
            }
            i = 0 as libc::c_int as size_t;
            while i < nz {
                let ref mut fresh75 = *ATp_0.offset(*Aj.offset(i as isize) as isize);
                *fresh75 += 1;
                *fresh75;
                i = i.wrapping_add(1);
                i;
            }
            gsl_spmatrix_cumsum(N, ATp_0);
            i = 0 as libc::c_int as size_t;
            while i < N {
                *w_0.offset(i as isize) = *ATp_0.offset(i as isize);
                i = i.wrapping_add(1);
                i;
            }
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *Ap_0.offset(i as isize);
                while p_0
                    < *Ap_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh76 = *w_0.offset(*Aj.offset(p_0 as isize) as isize);
                    let fresh77 = *fresh76;
                    *fresh76 = *fresh76 + 1;
                    let mut k_0: size_t = fresh77 as size_t;
                    *ATj.offset(k_0 as isize) = i as libc::c_int;
                    r_1 = 0 as libc::c_int as size_t;
                    while r_1 < 1 as libc::c_int as libc::c_ulong {
                        *ATd_0
                            .offset(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(k_0)
                                    .wrapping_add(r_1) as isize,
                            ) = *Ad_0
                            .offset(
                                ((1 as libc::c_int * p_0) as libc::c_ulong)
                                    .wrapping_add(r_1) as isize,
                            );
                        r_1 = r_1.wrapping_add(1);
                        r_1;
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
                b"./swap_source.c\0" as *const u8 as *const libc::c_char,
                199 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        (*dest).nz = nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_long_transpose_memcpy(
    mut dest: *mut gsl_spmatrix_long,
    mut src: *const gsl_spmatrix_long,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    if M != (*dest).size2 || N != (*dest).size1 {
        gsl_error(
            b"dimensions of dest must be transpose of src matrix\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*dest).sptype != (*src).sptype {
        gsl_error(
            b"cannot copy matrices of different storage formats\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let nz: size_t = (*src).nz;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_long_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        if (*src).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut n: size_t = 0;
            let mut r: size_t = 0;
            let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
            n = 0 as libc::c_int as size_t;
            while n < nz {
                *((*dest).i).offset(n as isize) = *((*src).p).offset(n as isize);
                *((*dest).p).offset(n as isize) = *((*src).i).offset(n as isize);
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    *((*dest).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        ) = *((*src).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        );
                    r = r.wrapping_add(1);
                    r;
                }
                ptr = gsl_bst_insert(
                    &mut *((*dest).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize,
                        ) as *mut libc::c_long as *mut libc::c_void,
                    (*dest).tree,
                );
                if !ptr.is_null() {
                    gsl_error(
                        b"detected duplicate entry\0" as *const u8
                            as *const libc::c_char,
                        b"./swap_source.c\0" as *const u8 as *const libc::c_char,
                        115 as libc::c_int,
                        GSL_EINVAL as libc::c_int,
                    );
                    return GSL_EINVAL as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ai: *mut libc::c_int = (*src).i;
            let mut Ap: *mut libc::c_int = (*src).p;
            let mut Ad: *mut libc::c_long = (*src).data;
            let mut ATi: *mut libc::c_int = (*dest).i;
            let mut ATp: *mut libc::c_int = (*dest).p;
            let mut ATd: *mut libc::c_long = (*dest).data;
            let mut w: *mut libc::c_int = (*dest).work.work_int;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            let mut r_0: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < M.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *ATp.offset(j as isize) = 0 as libc::c_int;
                j = j.wrapping_add(1);
                j;
            }
            j = 0 as libc::c_int as size_t;
            while j < nz {
                let ref mut fresh78 = *ATp.offset(*Ai.offset(j as isize) as isize);
                *fresh78 += 1;
                *fresh78;
                j = j.wrapping_add(1);
                j;
            }
            gsl_spmatrix_cumsum(M, ATp);
            j = 0 as libc::c_int as size_t;
            while j < M {
                *w.offset(j as isize) = *ATp.offset(j as isize);
                j = j.wrapping_add(1);
                j;
            }
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh79 = *w.offset(*Ai.offset(p as isize) as isize);
                    let fresh80 = *fresh79;
                    *fresh79 = *fresh79 + 1;
                    let mut k: libc::c_int = fresh80;
                    *ATi.offset(k as isize) = j as libc::c_int;
                    r_0 = 0 as libc::c_int as size_t;
                    while r_0 < 1 as libc::c_int as libc::c_ulong {
                        *ATd
                            .offset(
                                ((1 as libc::c_int * k) as libc::c_ulong).wrapping_add(r_0)
                                    as isize,
                            ) = *Ad
                            .offset(
                                ((1 as libc::c_int * p) as libc::c_ulong).wrapping_add(r_0)
                                    as isize,
                            );
                        r_0 = r_0.wrapping_add(1);
                        r_0;
                    }
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Aj: *mut libc::c_int = (*src).i;
            let mut Ap_0: *mut libc::c_int = (*src).p;
            let mut Ad_0: *mut libc::c_long = (*src).data;
            let mut ATj: *mut libc::c_int = (*dest).i;
            let mut ATp_0: *mut libc::c_int = (*dest).p;
            let mut ATd_0: *mut libc::c_long = (*dest).data;
            let mut w_0: *mut libc::c_int = (*dest).work.work_int;
            let mut p_0: libc::c_int = 0;
            let mut i: size_t = 0;
            let mut r_1: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < N.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *ATp_0.offset(i as isize) = 0 as libc::c_int;
                i = i.wrapping_add(1);
                i;
            }
            i = 0 as libc::c_int as size_t;
            while i < nz {
                let ref mut fresh81 = *ATp_0.offset(*Aj.offset(i as isize) as isize);
                *fresh81 += 1;
                *fresh81;
                i = i.wrapping_add(1);
                i;
            }
            gsl_spmatrix_cumsum(N, ATp_0);
            i = 0 as libc::c_int as size_t;
            while i < N {
                *w_0.offset(i as isize) = *ATp_0.offset(i as isize);
                i = i.wrapping_add(1);
                i;
            }
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *Ap_0.offset(i as isize);
                while p_0
                    < *Ap_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh82 = *w_0.offset(*Aj.offset(p_0 as isize) as isize);
                    let fresh83 = *fresh82;
                    *fresh82 = *fresh82 + 1;
                    let mut k_0: size_t = fresh83 as size_t;
                    *ATj.offset(k_0 as isize) = i as libc::c_int;
                    r_1 = 0 as libc::c_int as size_t;
                    while r_1 < 1 as libc::c_int as libc::c_ulong {
                        *ATd_0
                            .offset(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(k_0)
                                    .wrapping_add(r_1) as isize,
                            ) = *Ad_0
                            .offset(
                                ((1 as libc::c_int * p_0) as libc::c_ulong)
                                    .wrapping_add(r_1) as isize,
                            );
                        r_1 = r_1.wrapping_add(1);
                        r_1;
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
                b"./swap_source.c\0" as *const u8 as *const libc::c_char,
                199 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        (*dest).nz = nz;
        return status;
    };
}
