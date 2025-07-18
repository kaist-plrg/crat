use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn floor(_: libc::c_double) -> libc::c_double;
    static mut gsl_bst_avl: *const gsl_bst_type;
    fn gsl_bst_alloc(
        T: *const gsl_bst_type,
        allocator: *const gsl_bst_allocator,
        compare: Option::<gsl_bst_cmp_function>,
        params: *mut libc::c_void,
    ) -> *mut gsl_bst_workspace;
    fn gsl_bst_free(w: *mut gsl_bst_workspace);
    fn gsl_bst_empty(w: *mut gsl_bst_workspace) -> libc::c_int;
    fn gsl_bst_insert(
        item: *mut libc::c_void,
        w: *mut gsl_bst_workspace,
    ) -> *mut libc::c_void;
    fn gsl_bst_node_size(w: *const gsl_bst_workspace) -> size_t;
    fn gsl_bst_trav_first(
        trav: *mut gsl_bst_trav,
        w: *const gsl_bst_workspace,
    ) -> *mut libc::c_void;
    fn gsl_bst_trav_next(trav: *mut gsl_bst_trav) -> *mut libc::c_void;
    fn gsl_bst_trav_replace(
        trav: *mut gsl_bst_trav,
        new_item: *mut libc::c_void,
    ) -> *mut libc::c_void;
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
pub struct gsl_bst_avl_traverser {
    pub avl_table: *const gsl_bst_avl_table,
    pub avl_node: *mut gsl_bst_avl_node,
    pub avl_stack: [*mut gsl_bst_avl_node; 32],
    pub avl_height: size_t,
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
pub struct gsl_bst_rb_traverser {
    pub rb_table: *const gsl_bst_rb_table,
    pub rb_node: *mut gsl_bst_rb_node,
    pub rb_stack: [*mut gsl_bst_rb_node; 48],
    pub rb_height: size_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_bst_trav {
    pub type_0: *const gsl_bst_type,
    pub trav_data: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub avl_trav: gsl_bst_avl_traverser,
    pub rb_trav: gsl_bst_rb_traverser,
}
pub type C2RustUnnamed_2 = libc::c_int;
pub const GSL_EOF: C2RustUnnamed_2 = 32;
pub const GSL_ETOLG: C2RustUnnamed_2 = 31;
pub const GSL_ETOLX: C2RustUnnamed_2 = 30;
pub const GSL_ETOLF: C2RustUnnamed_2 = 29;
pub const GSL_ENOPROGJ: C2RustUnnamed_2 = 28;
pub const GSL_ENOPROG: C2RustUnnamed_2 = 27;
pub const GSL_ETABLE: C2RustUnnamed_2 = 26;
pub const GSL_ECACHE: C2RustUnnamed_2 = 25;
pub const GSL_EUNIMPL: C2RustUnnamed_2 = 24;
pub const GSL_EUNSUP: C2RustUnnamed_2 = 23;
pub const GSL_EDIVERGE: C2RustUnnamed_2 = 22;
pub const GSL_ESING: C2RustUnnamed_2 = 21;
pub const GSL_ENOTSQR: C2RustUnnamed_2 = 20;
pub const GSL_EBADLEN: C2RustUnnamed_2 = 19;
pub const GSL_EROUND: C2RustUnnamed_2 = 18;
pub const GSL_ELOSS: C2RustUnnamed_2 = 17;
pub const GSL_EOVRFLW: C2RustUnnamed_2 = 16;
pub const GSL_EUNDRFLW: C2RustUnnamed_2 = 15;
pub const GSL_ETOL: C2RustUnnamed_2 = 14;
pub const GSL_EBADTOL: C2RustUnnamed_2 = 13;
pub const GSL_EZERODIV: C2RustUnnamed_2 = 12;
pub const GSL_EMAXITER: C2RustUnnamed_2 = 11;
pub const GSL_ERUNAWAY: C2RustUnnamed_2 = 10;
pub const GSL_EBADFUNC: C2RustUnnamed_2 = 9;
pub const GSL_ENOMEM: C2RustUnnamed_2 = 8;
pub const GSL_ESANITY: C2RustUnnamed_2 = 7;
pub const GSL_EFACTOR: C2RustUnnamed_2 = 6;
pub const GSL_EFAILED: C2RustUnnamed_2 = 5;
pub const GSL_EINVAL: C2RustUnnamed_2 = 4;
pub const GSL_EFAULT: C2RustUnnamed_2 = 3;
pub const GSL_ERANGE: C2RustUnnamed_2 = 2;
pub const GSL_EDOM: C2RustUnnamed_2 = 1;
pub const GSL_CONTINUE: C2RustUnnamed_2 = -2;
pub const GSL_FAILURE: C2RustUnnamed_2 = -1;
pub const GSL_SUCCESS: C2RustUnnamed_2 = 0;
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
    pub work: C2RustUnnamed_3,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
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
    pub work: C2RustUnnamed_4,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
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
    pub work: C2RustUnnamed_5,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
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
    pub work: C2RustUnnamed_6,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
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
    pub work: C2RustUnnamed_7,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
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
    pub work: C2RustUnnamed_8,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
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
    pub work: C2RustUnnamed_9,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
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
    pub work: C2RustUnnamed_10,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
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
    pub work: C2RustUnnamed_11,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
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
    pub work: C2RustUnnamed_12,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
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
    pub work: C2RustUnnamed_13,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
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
    pub work: C2RustUnnamed_14,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_14 {
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
    pub work: C2RustUnnamed_15,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_15 {
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
    pub work: C2RustUnnamed_16,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_16 {
    pub work_void: *mut libc::c_void,
    pub work_int: *mut libc::c_int,
    pub work_atomic: *mut libc::c_char,
}
static mut spmatrix_char_allocator: gsl_bst_allocator = unsafe {
    {
        let mut init = gsl_bst_allocator {
            alloc: Some(
                spmatrix_char_malloc
                    as unsafe extern "C" fn(
                        size_t,
                        *mut libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            free: Some(
                spmatrix_char_free
                    as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
            ),
        };
        init
    }
};
static mut spmatrix_ulong_allocator: gsl_bst_allocator = unsafe {
    {
        let mut init = gsl_bst_allocator {
            alloc: Some(
                spmatrix_ulong_malloc
                    as unsafe extern "C" fn(
                        size_t,
                        *mut libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            free: Some(
                spmatrix_ulong_free
                    as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
            ),
        };
        init
    }
};
static mut spmatrix_int_allocator: gsl_bst_allocator = unsafe {
    {
        let mut init = gsl_bst_allocator {
            alloc: Some(
                spmatrix_int_malloc
                    as unsafe extern "C" fn(
                        size_t,
                        *mut libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            free: Some(
                spmatrix_int_free
                    as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
            ),
        };
        init
    }
};
static mut spmatrix_ushort_allocator: gsl_bst_allocator = unsafe {
    {
        let mut init = gsl_bst_allocator {
            alloc: Some(
                spmatrix_ushort_malloc
                    as unsafe extern "C" fn(
                        size_t,
                        *mut libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            free: Some(
                spmatrix_ushort_free
                    as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
            ),
        };
        init
    }
};
static mut spmatrix_long_double_allocator: gsl_bst_allocator = unsafe {
    {
        let mut init = gsl_bst_allocator {
            alloc: Some(
                spmatrix_long_double_malloc
                    as unsafe extern "C" fn(
                        size_t,
                        *mut libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            free: Some(
                spmatrix_long_double_free
                    as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
            ),
        };
        init
    }
};
static mut spmatrix_complex_long_double_allocator: gsl_bst_allocator = unsafe {
    {
        let mut init = gsl_bst_allocator {
            alloc: Some(
                spmatrix_complex_long_double_malloc
                    as unsafe extern "C" fn(
                        size_t,
                        *mut libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            free: Some(
                spmatrix_complex_long_double_free
                    as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
            ),
        };
        init
    }
};
static mut spmatrix_float_allocator: gsl_bst_allocator = unsafe {
    {
        let mut init = gsl_bst_allocator {
            alloc: Some(
                spmatrix_float_malloc
                    as unsafe extern "C" fn(
                        size_t,
                        *mut libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            free: Some(
                spmatrix_float_free
                    as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
            ),
        };
        init
    }
};
static mut spmatrix_complex_allocator: gsl_bst_allocator = unsafe {
    {
        let mut init = gsl_bst_allocator {
            alloc: Some(
                spmatrix_complex_malloc
                    as unsafe extern "C" fn(
                        size_t,
                        *mut libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            free: Some(
                spmatrix_complex_free
                    as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
            ),
        };
        init
    }
};
static mut spmatrix_allocator: gsl_bst_allocator = unsafe {
    {
        let mut init = gsl_bst_allocator {
            alloc: Some(
                spmatrix_malloc
                    as unsafe extern "C" fn(
                        size_t,
                        *mut libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            free: Some(
                spmatrix_free
                    as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
            ),
        };
        init
    }
};
static mut spmatrix_complex_float_allocator: gsl_bst_allocator = unsafe {
    {
        let mut init = gsl_bst_allocator {
            alloc: Some(
                spmatrix_complex_float_malloc
                    as unsafe extern "C" fn(
                        size_t,
                        *mut libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            free: Some(
                spmatrix_complex_float_free
                    as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
            ),
        };
        init
    }
};
static mut spmatrix_uint_allocator: gsl_bst_allocator = unsafe {
    {
        let mut init = gsl_bst_allocator {
            alloc: Some(
                spmatrix_uint_malloc
                    as unsafe extern "C" fn(
                        size_t,
                        *mut libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            free: Some(
                spmatrix_uint_free
                    as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
            ),
        };
        init
    }
};
static mut spmatrix_uchar_allocator: gsl_bst_allocator = unsafe {
    {
        let mut init = gsl_bst_allocator {
            alloc: Some(
                spmatrix_uchar_malloc
                    as unsafe extern "C" fn(
                        size_t,
                        *mut libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            free: Some(
                spmatrix_uchar_free
                    as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
            ),
        };
        init
    }
};
static mut spmatrix_short_allocator: gsl_bst_allocator = unsafe {
    {
        let mut init = gsl_bst_allocator {
            alloc: Some(
                spmatrix_short_malloc
                    as unsafe extern "C" fn(
                        size_t,
                        *mut libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            free: Some(
                spmatrix_short_free
                    as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
            ),
        };
        init
    }
};
static mut spmatrix_long_allocator: gsl_bst_allocator = unsafe {
    {
        let mut init = gsl_bst_allocator {
            alloc: Some(
                spmatrix_long_malloc
                    as unsafe extern "C" fn(
                        size_t,
                        *mut libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            free: Some(
                spmatrix_long_free
                    as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
            ),
        };
        init
    }
};
pub unsafe extern "C" fn gsl_spmatrix_complex_long_double_alloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_spmatrix_complex_long_double {
    let density: libc::c_double = 0.1f64;
    let mut nzmax: size_t = floor(n1.wrapping_mul(n2) as libc::c_double * density)
        as size_t;
    if nzmax == 0 as libc::c_int as libc::c_ulong {
        nzmax = 10 as libc::c_int as size_t;
    }
    return gsl_spmatrix_complex_long_double_alloc_nzmax(
        n1,
        n2,
        nzmax,
        GSL_SPMATRIX_COO as libc::c_int,
    );
}
pub unsafe extern "C" fn gsl_spmatrix_uchar_alloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_spmatrix_uchar {
    let density: libc::c_double = 0.1f64;
    let mut nzmax: size_t = floor(n1.wrapping_mul(n2) as libc::c_double * density)
        as size_t;
    if nzmax == 0 as libc::c_int as libc::c_ulong {
        nzmax = 10 as libc::c_int as size_t;
    }
    return gsl_spmatrix_uchar_alloc_nzmax(
        n1,
        n2,
        nzmax,
        GSL_SPMATRIX_COO as libc::c_int,
    );
}
pub unsafe extern "C" fn gsl_spmatrix_ulong_alloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_spmatrix_ulong {
    let density: libc::c_double = 0.1f64;
    let mut nzmax: size_t = floor(n1.wrapping_mul(n2) as libc::c_double * density)
        as size_t;
    if nzmax == 0 as libc::c_int as libc::c_ulong {
        nzmax = 10 as libc::c_int as size_t;
    }
    return gsl_spmatrix_ulong_alloc_nzmax(
        n1,
        n2,
        nzmax,
        GSL_SPMATRIX_COO as libc::c_int,
    );
}
pub unsafe extern "C" fn gsl_spmatrix_complex_float_alloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_spmatrix_complex_float {
    let density: libc::c_double = 0.1f64;
    let mut nzmax: size_t = floor(n1.wrapping_mul(n2) as libc::c_double * density)
        as size_t;
    if nzmax == 0 as libc::c_int as libc::c_ulong {
        nzmax = 10 as libc::c_int as size_t;
    }
    return gsl_spmatrix_complex_float_alloc_nzmax(
        n1,
        n2,
        nzmax,
        GSL_SPMATRIX_COO as libc::c_int,
    );
}
pub unsafe extern "C" fn gsl_spmatrix_char_alloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_spmatrix_char {
    let density: libc::c_double = 0.1f64;
    let mut nzmax: size_t = floor(n1.wrapping_mul(n2) as libc::c_double * density)
        as size_t;
    if nzmax == 0 as libc::c_int as libc::c_ulong {
        nzmax = 10 as libc::c_int as size_t;
    }
    return gsl_spmatrix_char_alloc_nzmax(n1, n2, nzmax, GSL_SPMATRIX_COO as libc::c_int);
}
pub unsafe extern "C" fn gsl_spmatrix_long_alloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_spmatrix_long {
    let density: libc::c_double = 0.1f64;
    let mut nzmax: size_t = floor(n1.wrapping_mul(n2) as libc::c_double * density)
        as size_t;
    if nzmax == 0 as libc::c_int as libc::c_ulong {
        nzmax = 10 as libc::c_int as size_t;
    }
    return gsl_spmatrix_long_alloc_nzmax(n1, n2, nzmax, GSL_SPMATRIX_COO as libc::c_int);
}
pub unsafe extern "C" fn gsl_spmatrix_uint_alloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_spmatrix_uint {
    let density: libc::c_double = 0.1f64;
    let mut nzmax: size_t = floor(n1.wrapping_mul(n2) as libc::c_double * density)
        as size_t;
    if nzmax == 0 as libc::c_int as libc::c_ulong {
        nzmax = 10 as libc::c_int as size_t;
    }
    return gsl_spmatrix_uint_alloc_nzmax(n1, n2, nzmax, GSL_SPMATRIX_COO as libc::c_int);
}
pub unsafe extern "C" fn gsl_spmatrix_int_alloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_spmatrix_int {
    let density: libc::c_double = 0.1f64;
    let mut nzmax: size_t = floor(n1.wrapping_mul(n2) as libc::c_double * density)
        as size_t;
    if nzmax == 0 as libc::c_int as libc::c_ulong {
        nzmax = 10 as libc::c_int as size_t;
    }
    return gsl_spmatrix_int_alloc_nzmax(n1, n2, nzmax, GSL_SPMATRIX_COO as libc::c_int);
}
pub unsafe extern "C" fn gsl_spmatrix_ushort_alloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_spmatrix_ushort {
    let density: libc::c_double = 0.1f64;
    let mut nzmax: size_t = floor(n1.wrapping_mul(n2) as libc::c_double * density)
        as size_t;
    if nzmax == 0 as libc::c_int as libc::c_ulong {
        nzmax = 10 as libc::c_int as size_t;
    }
    return gsl_spmatrix_ushort_alloc_nzmax(
        n1,
        n2,
        nzmax,
        GSL_SPMATRIX_COO as libc::c_int,
    );
}
pub unsafe extern "C" fn gsl_spmatrix_float_alloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_spmatrix_float {
    let density: libc::c_double = 0.1f64;
    let mut nzmax: size_t = floor(n1.wrapping_mul(n2) as libc::c_double * density)
        as size_t;
    if nzmax == 0 as libc::c_int as libc::c_ulong {
        nzmax = 10 as libc::c_int as size_t;
    }
    return gsl_spmatrix_float_alloc_nzmax(
        n1,
        n2,
        nzmax,
        GSL_SPMATRIX_COO as libc::c_int,
    );
}
pub unsafe extern "C" fn gsl_spmatrix_short_alloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_spmatrix_short {
    let density: libc::c_double = 0.1f64;
    let mut nzmax: size_t = floor(n1.wrapping_mul(n2) as libc::c_double * density)
        as size_t;
    if nzmax == 0 as libc::c_int as libc::c_ulong {
        nzmax = 10 as libc::c_int as size_t;
    }
    return gsl_spmatrix_short_alloc_nzmax(
        n1,
        n2,
        nzmax,
        GSL_SPMATRIX_COO as libc::c_int,
    );
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_alloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_spmatrix_long_double {
    let density: libc::c_double = 0.1f64;
    let mut nzmax: size_t = floor(n1.wrapping_mul(n2) as libc::c_double * density)
        as size_t;
    if nzmax == 0 as libc::c_int as libc::c_ulong {
        nzmax = 10 as libc::c_int as size_t;
    }
    return gsl_spmatrix_long_double_alloc_nzmax(
        n1,
        n2,
        nzmax,
        GSL_SPMATRIX_COO as libc::c_int,
    );
}
pub unsafe extern "C" fn gsl_spmatrix_complex_alloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_spmatrix_complex {
    let density: libc::c_double = 0.1f64;
    let mut nzmax: size_t = floor(n1.wrapping_mul(n2) as libc::c_double * density)
        as size_t;
    if nzmax == 0 as libc::c_int as libc::c_ulong {
        nzmax = 10 as libc::c_int as size_t;
    }
    return gsl_spmatrix_complex_alloc_nzmax(
        n1,
        n2,
        nzmax,
        GSL_SPMATRIX_COO as libc::c_int,
    );
}
pub unsafe extern "C" fn gsl_spmatrix_alloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_spmatrix {
    let density: libc::c_double = 0.1f64;
    let mut nzmax: size_t = floor(n1.wrapping_mul(n2) as libc::c_double * density)
        as size_t;
    if nzmax == 0 as libc::c_int as libc::c_ulong {
        nzmax = 10 as libc::c_int as size_t;
    }
    return gsl_spmatrix_alloc_nzmax(n1, n2, nzmax, GSL_SPMATRIX_COO as libc::c_int);
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_alloc_nzmax(
    n1: size_t,
    n2: size_t,
    nzmax: size_t,
    sptype: libc::c_int,
) -> *mut gsl_spmatrix_long_double {
    let mut m: *mut gsl_spmatrix_long_double = 0 as *mut gsl_spmatrix_long_double;
    if n1 == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix dimension n1 must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_long_double;
    } else if n2 == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix dimension n2 must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_long_double;
    }
    m = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gsl_spmatrix_long_double>() as libc::c_ulong,
    ) as *mut gsl_spmatrix_long_double;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for spmatrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            65 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_long_double;
    }
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).nz = 0 as libc::c_int as size_t;
    (*m)
        .nzmax = if nzmax > 1 as libc::c_int as libc::c_ulong {
        nzmax
    } else {
        1 as libc::c_int as libc::c_ulong
    };
    (*m).sptype = sptype;
    if n1 == 1 as libc::c_int as libc::c_ulong && n2 == 1 as libc::c_int as libc::c_ulong
    {
        (*m).spflags = ((1 as libc::c_int) << 0 as libc::c_int) as size_t;
    } else {
        (*m).spflags = 0 as libc::c_int as size_t;
    }
    (*m)
        .i = malloc(
        ((*m).nzmax).wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if ((*m).i).is_null() {
        gsl_spmatrix_long_double_free(m);
        gsl_error(
            b"failed to allocate space for row indices\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_long_double;
    }
    (*m)
        .work
        .work_void = malloc(
        (if n1 > n2 { n1 } else { n2 })
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(
                (if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    > ::std::mem::size_of::<f128::f128>() as libc::c_ulong
                {
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                } else {
                    ::std::mem::size_of::<f128::f128>() as libc::c_ulong
                }),
            ),
    );
    if ((*m).work.work_void).is_null() {
        gsl_spmatrix_long_double_free(m);
        gsl_error(
            b"failed to allocate space for work\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_long_double;
    }
    if sptype == GSL_SPMATRIX_COO as libc::c_int {
        (*m)
            .tree = gsl_bst_alloc(
            gsl_bst_avl,
            &spmatrix_long_double_allocator,
            Some(
                compare_long_double_func
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            m as *mut libc::c_void,
        );
        if ((*m).tree).is_null() {
            gsl_spmatrix_long_double_free(m);
            gsl_error(
                b"failed to allocate space for binary tree\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                102 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_long_double;
        }
        (*m).node_size = gsl_bst_node_size((*m).tree);
        spmatrix_long_double_pool_init(m);
        (*m)
            .p = malloc(
            ((*m).nzmax)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_long_double_free(m);
            gsl_error(
                b"failed to allocate space for column indices\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                115 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_long_double;
        }
    } else if sptype == GSL_SPMATRIX_CSC as libc::c_int {
        (*m)
            .p = malloc(
            n2
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_long_double_free(m);
            gsl_error(
                b"failed to allocate space for column pointers\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                125 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_long_double;
        }
    } else if sptype == GSL_SPMATRIX_CSR as libc::c_int {
        (*m)
            .p = malloc(
            n1
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_long_double_free(m);
            gsl_error(
                b"failed to allocate space for row pointers\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                135 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_long_double;
        }
    }
    (*m)
        .data = malloc(
        ((*m).nzmax)
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<f128::f128>() as libc::c_ulong),
    ) as *mut f128::f128;
    if ((*m).data).is_null() {
        gsl_spmatrix_long_double_free(m);
        gsl_error(
            b"failed to allocate space for data\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_long_double;
    }
    return m;
}
pub unsafe extern "C" fn gsl_spmatrix_ulong_alloc_nzmax(
    n1: size_t,
    n2: size_t,
    nzmax: size_t,
    sptype: libc::c_int,
) -> *mut gsl_spmatrix_ulong {
    let mut m: *mut gsl_spmatrix_ulong = 0 as *mut gsl_spmatrix_ulong;
    if n1 == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix dimension n1 must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_ulong;
    } else if n2 == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix dimension n2 must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_ulong;
    }
    m = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gsl_spmatrix_ulong>() as libc::c_ulong,
    ) as *mut gsl_spmatrix_ulong;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for spmatrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            65 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_ulong;
    }
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).nz = 0 as libc::c_int as size_t;
    (*m)
        .nzmax = if nzmax > 1 as libc::c_int as libc::c_ulong {
        nzmax
    } else {
        1 as libc::c_int as libc::c_ulong
    };
    (*m).sptype = sptype;
    if n1 == 1 as libc::c_int as libc::c_ulong && n2 == 1 as libc::c_int as libc::c_ulong
    {
        (*m).spflags = ((1 as libc::c_int) << 0 as libc::c_int) as size_t;
    } else {
        (*m).spflags = 0 as libc::c_int as size_t;
    }
    (*m)
        .i = malloc(
        ((*m).nzmax).wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if ((*m).i).is_null() {
        gsl_spmatrix_ulong_free(m);
        gsl_error(
            b"failed to allocate space for row indices\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_ulong;
    }
    (*m)
        .work
        .work_void = malloc(
        (if n1 > n2 { n1 } else { n2 })
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(
                (if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    > ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong
                {
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                } else {
                    ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong
                }),
            ),
    );
    if ((*m).work.work_void).is_null() {
        gsl_spmatrix_ulong_free(m);
        gsl_error(
            b"failed to allocate space for work\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_ulong;
    }
    if sptype == GSL_SPMATRIX_COO as libc::c_int {
        (*m)
            .tree = gsl_bst_alloc(
            gsl_bst_avl,
            &spmatrix_ulong_allocator,
            Some(
                compare_ulong_func
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            m as *mut libc::c_void,
        );
        if ((*m).tree).is_null() {
            gsl_spmatrix_ulong_free(m);
            gsl_error(
                b"failed to allocate space for binary tree\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                102 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_ulong;
        }
        (*m).node_size = gsl_bst_node_size((*m).tree);
        spmatrix_ulong_pool_init(m);
        (*m)
            .p = malloc(
            ((*m).nzmax)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_ulong_free(m);
            gsl_error(
                b"failed to allocate space for column indices\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                115 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_ulong;
        }
    } else if sptype == GSL_SPMATRIX_CSC as libc::c_int {
        (*m)
            .p = malloc(
            n2
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_ulong_free(m);
            gsl_error(
                b"failed to allocate space for column pointers\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                125 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_ulong;
        }
    } else if sptype == GSL_SPMATRIX_CSR as libc::c_int {
        (*m)
            .p = malloc(
            n1
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_ulong_free(m);
            gsl_error(
                b"failed to allocate space for row pointers\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                135 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_ulong;
        }
    }
    (*m)
        .data = malloc(
        ((*m).nzmax)
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong),
    ) as *mut libc::c_ulong;
    if ((*m).data).is_null() {
        gsl_spmatrix_ulong_free(m);
        gsl_error(
            b"failed to allocate space for data\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_ulong;
    }
    return m;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_float_alloc_nzmax(
    n1: size_t,
    n2: size_t,
    nzmax: size_t,
    sptype: libc::c_int,
) -> *mut gsl_spmatrix_complex_float {
    let mut m: *mut gsl_spmatrix_complex_float = 0 as *mut gsl_spmatrix_complex_float;
    if n1 == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix dimension n1 must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_complex_float;
    } else if n2 == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix dimension n2 must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_complex_float;
    }
    m = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gsl_spmatrix_complex_float>() as libc::c_ulong,
    ) as *mut gsl_spmatrix_complex_float;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for spmatrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            65 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_complex_float;
    }
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).nz = 0 as libc::c_int as size_t;
    (*m)
        .nzmax = if nzmax > 1 as libc::c_int as libc::c_ulong {
        nzmax
    } else {
        1 as libc::c_int as libc::c_ulong
    };
    (*m).sptype = sptype;
    if n1 == 1 as libc::c_int as libc::c_ulong && n2 == 1 as libc::c_int as libc::c_ulong
    {
        (*m).spflags = ((1 as libc::c_int) << 0 as libc::c_int) as size_t;
    } else {
        (*m).spflags = 0 as libc::c_int as size_t;
    }
    (*m)
        .i = malloc(
        ((*m).nzmax).wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if ((*m).i).is_null() {
        gsl_spmatrix_complex_float_free(m);
        gsl_error(
            b"failed to allocate space for row indices\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_complex_float;
    }
    (*m)
        .work
        .work_void = malloc(
        (if n1 > n2 { n1 } else { n2 })
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(
                (if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    > ::std::mem::size_of::<libc::c_float>() as libc::c_ulong
                {
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                } else {
                    ::std::mem::size_of::<libc::c_float>() as libc::c_ulong
                }),
            ),
    );
    if ((*m).work.work_void).is_null() {
        gsl_spmatrix_complex_float_free(m);
        gsl_error(
            b"failed to allocate space for work\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_complex_float;
    }
    if sptype == GSL_SPMATRIX_COO as libc::c_int {
        (*m)
            .tree = gsl_bst_alloc(
            gsl_bst_avl,
            &spmatrix_complex_float_allocator,
            Some(
                compare_complex_float_func
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            m as *mut libc::c_void,
        );
        if ((*m).tree).is_null() {
            gsl_spmatrix_complex_float_free(m);
            gsl_error(
                b"failed to allocate space for binary tree\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                102 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_complex_float;
        }
        (*m).node_size = gsl_bst_node_size((*m).tree);
        spmatrix_complex_float_pool_init(m);
        (*m)
            .p = malloc(
            ((*m).nzmax)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_complex_float_free(m);
            gsl_error(
                b"failed to allocate space for column indices\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                115 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_complex_float;
        }
    } else if sptype == GSL_SPMATRIX_CSC as libc::c_int {
        (*m)
            .p = malloc(
            n2
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_complex_float_free(m);
            gsl_error(
                b"failed to allocate space for column pointers\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                125 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_complex_float;
        }
    } else if sptype == GSL_SPMATRIX_CSR as libc::c_int {
        (*m)
            .p = malloc(
            n1
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_complex_float_free(m);
            gsl_error(
                b"failed to allocate space for row pointers\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                135 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_complex_float;
        }
    }
    (*m)
        .data = malloc(
        ((*m).nzmax)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
    ) as *mut libc::c_float;
    if ((*m).data).is_null() {
        gsl_spmatrix_complex_float_free(m);
        gsl_error(
            b"failed to allocate space for data\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_complex_float;
    }
    return m;
}
pub unsafe extern "C" fn gsl_spmatrix_int_alloc_nzmax(
    n1: size_t,
    n2: size_t,
    nzmax: size_t,
    sptype: libc::c_int,
) -> *mut gsl_spmatrix_int {
    let mut m: *mut gsl_spmatrix_int = 0 as *mut gsl_spmatrix_int;
    if n1 == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix dimension n1 must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_int;
    } else if n2 == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix dimension n2 must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_int;
    }
    m = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gsl_spmatrix_int>() as libc::c_ulong,
    ) as *mut gsl_spmatrix_int;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for spmatrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            65 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_int;
    }
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).nz = 0 as libc::c_int as size_t;
    (*m)
        .nzmax = if nzmax > 1 as libc::c_int as libc::c_ulong {
        nzmax
    } else {
        1 as libc::c_int as libc::c_ulong
    };
    (*m).sptype = sptype;
    if n1 == 1 as libc::c_int as libc::c_ulong && n2 == 1 as libc::c_int as libc::c_ulong
    {
        (*m).spflags = ((1 as libc::c_int) << 0 as libc::c_int) as size_t;
    } else {
        (*m).spflags = 0 as libc::c_int as size_t;
    }
    (*m)
        .i = malloc(
        ((*m).nzmax).wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if ((*m).i).is_null() {
        gsl_spmatrix_int_free(m);
        gsl_error(
            b"failed to allocate space for row indices\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_int;
    }
    (*m)
        .work
        .work_void = malloc(
        (if n1 > n2 { n1 } else { n2 })
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(
                (if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    > ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                {
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                } else {
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                }),
            ),
    );
    if ((*m).work.work_void).is_null() {
        gsl_spmatrix_int_free(m);
        gsl_error(
            b"failed to allocate space for work\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_int;
    }
    if sptype == GSL_SPMATRIX_COO as libc::c_int {
        (*m)
            .tree = gsl_bst_alloc(
            gsl_bst_avl,
            &spmatrix_int_allocator,
            Some(
                compare_int_func
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            m as *mut libc::c_void,
        );
        if ((*m).tree).is_null() {
            gsl_spmatrix_int_free(m);
            gsl_error(
                b"failed to allocate space for binary tree\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                102 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_int;
        }
        (*m).node_size = gsl_bst_node_size((*m).tree);
        spmatrix_int_pool_init(m);
        (*m)
            .p = malloc(
            ((*m).nzmax)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_int_free(m);
            gsl_error(
                b"failed to allocate space for column indices\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                115 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_int;
        }
    } else if sptype == GSL_SPMATRIX_CSC as libc::c_int {
        (*m)
            .p = malloc(
            n2
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_int_free(m);
            gsl_error(
                b"failed to allocate space for column pointers\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                125 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_int;
        }
    } else if sptype == GSL_SPMATRIX_CSR as libc::c_int {
        (*m)
            .p = malloc(
            n1
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_int_free(m);
            gsl_error(
                b"failed to allocate space for row pointers\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                135 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_int;
        }
    }
    (*m)
        .data = malloc(
        ((*m).nzmax)
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if ((*m).data).is_null() {
        gsl_spmatrix_int_free(m);
        gsl_error(
            b"failed to allocate space for data\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_int;
    }
    return m;
}
pub unsafe extern "C" fn gsl_spmatrix_float_alloc_nzmax(
    n1: size_t,
    n2: size_t,
    nzmax: size_t,
    sptype: libc::c_int,
) -> *mut gsl_spmatrix_float {
    let mut m: *mut gsl_spmatrix_float = 0 as *mut gsl_spmatrix_float;
    if n1 == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix dimension n1 must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_float;
    } else if n2 == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix dimension n2 must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_float;
    }
    m = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gsl_spmatrix_float>() as libc::c_ulong,
    ) as *mut gsl_spmatrix_float;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for spmatrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            65 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_float;
    }
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).nz = 0 as libc::c_int as size_t;
    (*m)
        .nzmax = if nzmax > 1 as libc::c_int as libc::c_ulong {
        nzmax
    } else {
        1 as libc::c_int as libc::c_ulong
    };
    (*m).sptype = sptype;
    if n1 == 1 as libc::c_int as libc::c_ulong && n2 == 1 as libc::c_int as libc::c_ulong
    {
        (*m).spflags = ((1 as libc::c_int) << 0 as libc::c_int) as size_t;
    } else {
        (*m).spflags = 0 as libc::c_int as size_t;
    }
    (*m)
        .i = malloc(
        ((*m).nzmax).wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if ((*m).i).is_null() {
        gsl_spmatrix_float_free(m);
        gsl_error(
            b"failed to allocate space for row indices\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_float;
    }
    (*m)
        .work
        .work_void = malloc(
        (if n1 > n2 { n1 } else { n2 })
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(
                (if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    > ::std::mem::size_of::<libc::c_float>() as libc::c_ulong
                {
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                } else {
                    ::std::mem::size_of::<libc::c_float>() as libc::c_ulong
                }),
            ),
    );
    if ((*m).work.work_void).is_null() {
        gsl_spmatrix_float_free(m);
        gsl_error(
            b"failed to allocate space for work\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_float;
    }
    if sptype == GSL_SPMATRIX_COO as libc::c_int {
        (*m)
            .tree = gsl_bst_alloc(
            gsl_bst_avl,
            &spmatrix_float_allocator,
            Some(
                compare_float_func
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            m as *mut libc::c_void,
        );
        if ((*m).tree).is_null() {
            gsl_spmatrix_float_free(m);
            gsl_error(
                b"failed to allocate space for binary tree\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                102 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_float;
        }
        (*m).node_size = gsl_bst_node_size((*m).tree);
        spmatrix_float_pool_init(m);
        (*m)
            .p = malloc(
            ((*m).nzmax)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_float_free(m);
            gsl_error(
                b"failed to allocate space for column indices\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                115 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_float;
        }
    } else if sptype == GSL_SPMATRIX_CSC as libc::c_int {
        (*m)
            .p = malloc(
            n2
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_float_free(m);
            gsl_error(
                b"failed to allocate space for column pointers\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                125 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_float;
        }
    } else if sptype == GSL_SPMATRIX_CSR as libc::c_int {
        (*m)
            .p = malloc(
            n1
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_float_free(m);
            gsl_error(
                b"failed to allocate space for row pointers\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                135 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_float;
        }
    }
    (*m)
        .data = malloc(
        ((*m).nzmax)
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
    ) as *mut libc::c_float;
    if ((*m).data).is_null() {
        gsl_spmatrix_float_free(m);
        gsl_error(
            b"failed to allocate space for data\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_float;
    }
    return m;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_long_double_alloc_nzmax(
    n1: size_t,
    n2: size_t,
    nzmax: size_t,
    sptype: libc::c_int,
) -> *mut gsl_spmatrix_complex_long_double {
    let mut m: *mut gsl_spmatrix_complex_long_double = 0
        as *mut gsl_spmatrix_complex_long_double;
    if n1 == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix dimension n1 must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_complex_long_double;
    } else if n2 == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix dimension n2 must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_complex_long_double;
    }
    m = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gsl_spmatrix_complex_long_double>() as libc::c_ulong,
    ) as *mut gsl_spmatrix_complex_long_double;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for spmatrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            65 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_complex_long_double;
    }
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).nz = 0 as libc::c_int as size_t;
    (*m)
        .nzmax = if nzmax > 1 as libc::c_int as libc::c_ulong {
        nzmax
    } else {
        1 as libc::c_int as libc::c_ulong
    };
    (*m).sptype = sptype;
    if n1 == 1 as libc::c_int as libc::c_ulong && n2 == 1 as libc::c_int as libc::c_ulong
    {
        (*m).spflags = ((1 as libc::c_int) << 0 as libc::c_int) as size_t;
    } else {
        (*m).spflags = 0 as libc::c_int as size_t;
    }
    (*m)
        .i = malloc(
        ((*m).nzmax).wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if ((*m).i).is_null() {
        gsl_spmatrix_complex_long_double_free(m);
        gsl_error(
            b"failed to allocate space for row indices\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_complex_long_double;
    }
    (*m)
        .work
        .work_void = malloc(
        (if n1 > n2 { n1 } else { n2 })
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(
                (if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    > ::std::mem::size_of::<f128::f128>() as libc::c_ulong
                {
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                } else {
                    ::std::mem::size_of::<f128::f128>() as libc::c_ulong
                }),
            ),
    );
    if ((*m).work.work_void).is_null() {
        gsl_spmatrix_complex_long_double_free(m);
        gsl_error(
            b"failed to allocate space for work\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_complex_long_double;
    }
    if sptype == GSL_SPMATRIX_COO as libc::c_int {
        (*m)
            .tree = gsl_bst_alloc(
            gsl_bst_avl,
            &spmatrix_complex_long_double_allocator,
            Some(
                compare_complex_long_double_func
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            m as *mut libc::c_void,
        );
        if ((*m).tree).is_null() {
            gsl_spmatrix_complex_long_double_free(m);
            gsl_error(
                b"failed to allocate space for binary tree\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                102 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_complex_long_double;
        }
        (*m).node_size = gsl_bst_node_size((*m).tree);
        spmatrix_complex_long_double_pool_init(m);
        (*m)
            .p = malloc(
            ((*m).nzmax)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_complex_long_double_free(m);
            gsl_error(
                b"failed to allocate space for column indices\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                115 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_complex_long_double;
        }
    } else if sptype == GSL_SPMATRIX_CSC as libc::c_int {
        (*m)
            .p = malloc(
            n2
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_complex_long_double_free(m);
            gsl_error(
                b"failed to allocate space for column pointers\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                125 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_complex_long_double;
        }
    } else if sptype == GSL_SPMATRIX_CSR as libc::c_int {
        (*m)
            .p = malloc(
            n1
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_complex_long_double_free(m);
            gsl_error(
                b"failed to allocate space for row pointers\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                135 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_complex_long_double;
        }
    }
    (*m)
        .data = malloc(
        ((*m).nzmax)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<f128::f128>() as libc::c_ulong),
    ) as *mut f128::f128;
    if ((*m).data).is_null() {
        gsl_spmatrix_complex_long_double_free(m);
        gsl_error(
            b"failed to allocate space for data\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_complex_long_double;
    }
    return m;
}
pub unsafe extern "C" fn gsl_spmatrix_uint_alloc_nzmax(
    n1: size_t,
    n2: size_t,
    nzmax: size_t,
    sptype: libc::c_int,
) -> *mut gsl_spmatrix_uint {
    let mut m: *mut gsl_spmatrix_uint = 0 as *mut gsl_spmatrix_uint;
    if n1 == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix dimension n1 must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_uint;
    } else if n2 == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix dimension n2 must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_uint;
    }
    m = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gsl_spmatrix_uint>() as libc::c_ulong,
    ) as *mut gsl_spmatrix_uint;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for spmatrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            65 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_uint;
    }
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).nz = 0 as libc::c_int as size_t;
    (*m)
        .nzmax = if nzmax > 1 as libc::c_int as libc::c_ulong {
        nzmax
    } else {
        1 as libc::c_int as libc::c_ulong
    };
    (*m).sptype = sptype;
    if n1 == 1 as libc::c_int as libc::c_ulong && n2 == 1 as libc::c_int as libc::c_ulong
    {
        (*m).spflags = ((1 as libc::c_int) << 0 as libc::c_int) as size_t;
    } else {
        (*m).spflags = 0 as libc::c_int as size_t;
    }
    (*m)
        .i = malloc(
        ((*m).nzmax).wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if ((*m).i).is_null() {
        gsl_spmatrix_uint_free(m);
        gsl_error(
            b"failed to allocate space for row indices\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_uint;
    }
    (*m)
        .work
        .work_void = malloc(
        (if n1 > n2 { n1 } else { n2 })
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(
                (if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    > ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong
                {
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                } else {
                    ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong
                }),
            ),
    );
    if ((*m).work.work_void).is_null() {
        gsl_spmatrix_uint_free(m);
        gsl_error(
            b"failed to allocate space for work\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_uint;
    }
    if sptype == GSL_SPMATRIX_COO as libc::c_int {
        (*m)
            .tree = gsl_bst_alloc(
            gsl_bst_avl,
            &spmatrix_uint_allocator,
            Some(
                compare_uint_func
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            m as *mut libc::c_void,
        );
        if ((*m).tree).is_null() {
            gsl_spmatrix_uint_free(m);
            gsl_error(
                b"failed to allocate space for binary tree\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                102 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_uint;
        }
        (*m).node_size = gsl_bst_node_size((*m).tree);
        spmatrix_uint_pool_init(m);
        (*m)
            .p = malloc(
            ((*m).nzmax)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_uint_free(m);
            gsl_error(
                b"failed to allocate space for column indices\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                115 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_uint;
        }
    } else if sptype == GSL_SPMATRIX_CSC as libc::c_int {
        (*m)
            .p = malloc(
            n2
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_uint_free(m);
            gsl_error(
                b"failed to allocate space for column pointers\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                125 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_uint;
        }
    } else if sptype == GSL_SPMATRIX_CSR as libc::c_int {
        (*m)
            .p = malloc(
            n1
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_uint_free(m);
            gsl_error(
                b"failed to allocate space for row pointers\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                135 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_uint;
        }
    }
    (*m)
        .data = malloc(
        ((*m).nzmax)
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong),
    ) as *mut libc::c_uint;
    if ((*m).data).is_null() {
        gsl_spmatrix_uint_free(m);
        gsl_error(
            b"failed to allocate space for data\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_uint;
    }
    return m;
}
pub unsafe extern "C" fn gsl_spmatrix_ushort_alloc_nzmax(
    n1: size_t,
    n2: size_t,
    nzmax: size_t,
    sptype: libc::c_int,
) -> *mut gsl_spmatrix_ushort {
    let mut m: *mut gsl_spmatrix_ushort = 0 as *mut gsl_spmatrix_ushort;
    if n1 == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix dimension n1 must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_ushort;
    } else if n2 == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix dimension n2 must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_ushort;
    }
    m = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gsl_spmatrix_ushort>() as libc::c_ulong,
    ) as *mut gsl_spmatrix_ushort;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for spmatrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            65 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_ushort;
    }
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).nz = 0 as libc::c_int as size_t;
    (*m)
        .nzmax = if nzmax > 1 as libc::c_int as libc::c_ulong {
        nzmax
    } else {
        1 as libc::c_int as libc::c_ulong
    };
    (*m).sptype = sptype;
    if n1 == 1 as libc::c_int as libc::c_ulong && n2 == 1 as libc::c_int as libc::c_ulong
    {
        (*m).spflags = ((1 as libc::c_int) << 0 as libc::c_int) as size_t;
    } else {
        (*m).spflags = 0 as libc::c_int as size_t;
    }
    (*m)
        .i = malloc(
        ((*m).nzmax).wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if ((*m).i).is_null() {
        gsl_spmatrix_ushort_free(m);
        gsl_error(
            b"failed to allocate space for row indices\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_ushort;
    }
    (*m)
        .work
        .work_void = malloc(
        (if n1 > n2 { n1 } else { n2 })
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(
                (if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    > ::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong
                {
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                } else {
                    ::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong
                }),
            ),
    );
    if ((*m).work.work_void).is_null() {
        gsl_spmatrix_ushort_free(m);
        gsl_error(
            b"failed to allocate space for work\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_ushort;
    }
    if sptype == GSL_SPMATRIX_COO as libc::c_int {
        (*m)
            .tree = gsl_bst_alloc(
            gsl_bst_avl,
            &spmatrix_ushort_allocator,
            Some(
                compare_ushort_func
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            m as *mut libc::c_void,
        );
        if ((*m).tree).is_null() {
            gsl_spmatrix_ushort_free(m);
            gsl_error(
                b"failed to allocate space for binary tree\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                102 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_ushort;
        }
        (*m).node_size = gsl_bst_node_size((*m).tree);
        spmatrix_ushort_pool_init(m);
        (*m)
            .p = malloc(
            ((*m).nzmax)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_ushort_free(m);
            gsl_error(
                b"failed to allocate space for column indices\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                115 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_ushort;
        }
    } else if sptype == GSL_SPMATRIX_CSC as libc::c_int {
        (*m)
            .p = malloc(
            n2
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_ushort_free(m);
            gsl_error(
                b"failed to allocate space for column pointers\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                125 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_ushort;
        }
    } else if sptype == GSL_SPMATRIX_CSR as libc::c_int {
        (*m)
            .p = malloc(
            n1
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_ushort_free(m);
            gsl_error(
                b"failed to allocate space for row pointers\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                135 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_ushort;
        }
    }
    (*m)
        .data = malloc(
        ((*m).nzmax)
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong),
    ) as *mut libc::c_ushort;
    if ((*m).data).is_null() {
        gsl_spmatrix_ushort_free(m);
        gsl_error(
            b"failed to allocate space for data\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_ushort;
    }
    return m;
}
pub unsafe extern "C" fn gsl_spmatrix_short_alloc_nzmax(
    n1: size_t,
    n2: size_t,
    nzmax: size_t,
    sptype: libc::c_int,
) -> *mut gsl_spmatrix_short {
    let mut m: *mut gsl_spmatrix_short = 0 as *mut gsl_spmatrix_short;
    if n1 == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix dimension n1 must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_short;
    } else if n2 == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix dimension n2 must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_short;
    }
    m = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gsl_spmatrix_short>() as libc::c_ulong,
    ) as *mut gsl_spmatrix_short;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for spmatrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            65 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_short;
    }
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).nz = 0 as libc::c_int as size_t;
    (*m)
        .nzmax = if nzmax > 1 as libc::c_int as libc::c_ulong {
        nzmax
    } else {
        1 as libc::c_int as libc::c_ulong
    };
    (*m).sptype = sptype;
    if n1 == 1 as libc::c_int as libc::c_ulong && n2 == 1 as libc::c_int as libc::c_ulong
    {
        (*m).spflags = ((1 as libc::c_int) << 0 as libc::c_int) as size_t;
    } else {
        (*m).spflags = 0 as libc::c_int as size_t;
    }
    (*m)
        .i = malloc(
        ((*m).nzmax).wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if ((*m).i).is_null() {
        gsl_spmatrix_short_free(m);
        gsl_error(
            b"failed to allocate space for row indices\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_short;
    }
    (*m)
        .work
        .work_void = malloc(
        (if n1 > n2 { n1 } else { n2 })
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(
                (if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    > ::std::mem::size_of::<libc::c_short>() as libc::c_ulong
                {
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                } else {
                    ::std::mem::size_of::<libc::c_short>() as libc::c_ulong
                }),
            ),
    );
    if ((*m).work.work_void).is_null() {
        gsl_spmatrix_short_free(m);
        gsl_error(
            b"failed to allocate space for work\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_short;
    }
    if sptype == GSL_SPMATRIX_COO as libc::c_int {
        (*m)
            .tree = gsl_bst_alloc(
            gsl_bst_avl,
            &spmatrix_short_allocator,
            Some(
                compare_short_func
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            m as *mut libc::c_void,
        );
        if ((*m).tree).is_null() {
            gsl_spmatrix_short_free(m);
            gsl_error(
                b"failed to allocate space for binary tree\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                102 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_short;
        }
        (*m).node_size = gsl_bst_node_size((*m).tree);
        spmatrix_short_pool_init(m);
        (*m)
            .p = malloc(
            ((*m).nzmax)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_short_free(m);
            gsl_error(
                b"failed to allocate space for column indices\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                115 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_short;
        }
    } else if sptype == GSL_SPMATRIX_CSC as libc::c_int {
        (*m)
            .p = malloc(
            n2
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_short_free(m);
            gsl_error(
                b"failed to allocate space for column pointers\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                125 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_short;
        }
    } else if sptype == GSL_SPMATRIX_CSR as libc::c_int {
        (*m)
            .p = malloc(
            n1
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_short_free(m);
            gsl_error(
                b"failed to allocate space for row pointers\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                135 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_short;
        }
    }
    (*m)
        .data = malloc(
        ((*m).nzmax)
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_short>() as libc::c_ulong),
    ) as *mut libc::c_short;
    if ((*m).data).is_null() {
        gsl_spmatrix_short_free(m);
        gsl_error(
            b"failed to allocate space for data\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_short;
    }
    return m;
}
pub unsafe extern "C" fn gsl_spmatrix_long_alloc_nzmax(
    n1: size_t,
    n2: size_t,
    nzmax: size_t,
    sptype: libc::c_int,
) -> *mut gsl_spmatrix_long {
    let mut m: *mut gsl_spmatrix_long = 0 as *mut gsl_spmatrix_long;
    if n1 == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix dimension n1 must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_long;
    } else if n2 == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix dimension n2 must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_long;
    }
    m = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gsl_spmatrix_long>() as libc::c_ulong,
    ) as *mut gsl_spmatrix_long;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for spmatrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            65 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_long;
    }
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).nz = 0 as libc::c_int as size_t;
    (*m)
        .nzmax = if nzmax > 1 as libc::c_int as libc::c_ulong {
        nzmax
    } else {
        1 as libc::c_int as libc::c_ulong
    };
    (*m).sptype = sptype;
    if n1 == 1 as libc::c_int as libc::c_ulong && n2 == 1 as libc::c_int as libc::c_ulong
    {
        (*m).spflags = ((1 as libc::c_int) << 0 as libc::c_int) as size_t;
    } else {
        (*m).spflags = 0 as libc::c_int as size_t;
    }
    (*m)
        .i = malloc(
        ((*m).nzmax).wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if ((*m).i).is_null() {
        gsl_spmatrix_long_free(m);
        gsl_error(
            b"failed to allocate space for row indices\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_long;
    }
    (*m)
        .work
        .work_void = malloc(
        (if n1 > n2 { n1 } else { n2 })
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(
                (if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    > ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
                {
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                } else {
                    ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
                }),
            ),
    );
    if ((*m).work.work_void).is_null() {
        gsl_spmatrix_long_free(m);
        gsl_error(
            b"failed to allocate space for work\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_long;
    }
    if sptype == GSL_SPMATRIX_COO as libc::c_int {
        (*m)
            .tree = gsl_bst_alloc(
            gsl_bst_avl,
            &spmatrix_long_allocator,
            Some(
                compare_long_func
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            m as *mut libc::c_void,
        );
        if ((*m).tree).is_null() {
            gsl_spmatrix_long_free(m);
            gsl_error(
                b"failed to allocate space for binary tree\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                102 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_long;
        }
        (*m).node_size = gsl_bst_node_size((*m).tree);
        spmatrix_long_pool_init(m);
        (*m)
            .p = malloc(
            ((*m).nzmax)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_long_free(m);
            gsl_error(
                b"failed to allocate space for column indices\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                115 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_long;
        }
    } else if sptype == GSL_SPMATRIX_CSC as libc::c_int {
        (*m)
            .p = malloc(
            n2
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_long_free(m);
            gsl_error(
                b"failed to allocate space for column pointers\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                125 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_long;
        }
    } else if sptype == GSL_SPMATRIX_CSR as libc::c_int {
        (*m)
            .p = malloc(
            n1
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_long_free(m);
            gsl_error(
                b"failed to allocate space for row pointers\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                135 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_long;
        }
    }
    (*m)
        .data = malloc(
        ((*m).nzmax)
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_long>() as libc::c_ulong),
    ) as *mut libc::c_long;
    if ((*m).data).is_null() {
        gsl_spmatrix_long_free(m);
        gsl_error(
            b"failed to allocate space for data\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_long;
    }
    return m;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_alloc_nzmax(
    n1: size_t,
    n2: size_t,
    nzmax: size_t,
    sptype: libc::c_int,
) -> *mut gsl_spmatrix_complex {
    let mut m: *mut gsl_spmatrix_complex = 0 as *mut gsl_spmatrix_complex;
    if n1 == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix dimension n1 must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_complex;
    } else if n2 == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix dimension n2 must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_complex;
    }
    m = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gsl_spmatrix_complex>() as libc::c_ulong,
    ) as *mut gsl_spmatrix_complex;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for spmatrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            65 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_complex;
    }
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).nz = 0 as libc::c_int as size_t;
    (*m)
        .nzmax = if nzmax > 1 as libc::c_int as libc::c_ulong {
        nzmax
    } else {
        1 as libc::c_int as libc::c_ulong
    };
    (*m).sptype = sptype;
    if n1 == 1 as libc::c_int as libc::c_ulong && n2 == 1 as libc::c_int as libc::c_ulong
    {
        (*m).spflags = ((1 as libc::c_int) << 0 as libc::c_int) as size_t;
    } else {
        (*m).spflags = 0 as libc::c_int as size_t;
    }
    (*m)
        .i = malloc(
        ((*m).nzmax).wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if ((*m).i).is_null() {
        gsl_spmatrix_complex_free(m);
        gsl_error(
            b"failed to allocate space for row indices\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_complex;
    }
    (*m)
        .work
        .work_void = malloc(
        (if n1 > n2 { n1 } else { n2 })
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(
                (if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    > ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
                {
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                } else {
                    ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
                }),
            ),
    );
    if ((*m).work.work_void).is_null() {
        gsl_spmatrix_complex_free(m);
        gsl_error(
            b"failed to allocate space for work\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_complex;
    }
    if sptype == GSL_SPMATRIX_COO as libc::c_int {
        (*m)
            .tree = gsl_bst_alloc(
            gsl_bst_avl,
            &spmatrix_complex_allocator,
            Some(
                compare_complex_func
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            m as *mut libc::c_void,
        );
        if ((*m).tree).is_null() {
            gsl_spmatrix_complex_free(m);
            gsl_error(
                b"failed to allocate space for binary tree\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                102 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_complex;
        }
        (*m).node_size = gsl_bst_node_size((*m).tree);
        spmatrix_complex_pool_init(m);
        (*m)
            .p = malloc(
            ((*m).nzmax)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_complex_free(m);
            gsl_error(
                b"failed to allocate space for column indices\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                115 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_complex;
        }
    } else if sptype == GSL_SPMATRIX_CSC as libc::c_int {
        (*m)
            .p = malloc(
            n2
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_complex_free(m);
            gsl_error(
                b"failed to allocate space for column pointers\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                125 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_complex;
        }
    } else if sptype == GSL_SPMATRIX_CSR as libc::c_int {
        (*m)
            .p = malloc(
            n1
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_complex_free(m);
            gsl_error(
                b"failed to allocate space for row pointers\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                135 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_complex;
        }
    }
    (*m)
        .data = malloc(
        ((*m).nzmax)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*m).data).is_null() {
        gsl_spmatrix_complex_free(m);
        gsl_error(
            b"failed to allocate space for data\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_complex;
    }
    return m;
}
pub unsafe extern "C" fn gsl_spmatrix_char_alloc_nzmax(
    n1: size_t,
    n2: size_t,
    nzmax: size_t,
    sptype: libc::c_int,
) -> *mut gsl_spmatrix_char {
    let mut m: *mut gsl_spmatrix_char = 0 as *mut gsl_spmatrix_char;
    if n1 == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix dimension n1 must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_char;
    } else if n2 == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix dimension n2 must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_char;
    }
    m = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gsl_spmatrix_char>() as libc::c_ulong,
    ) as *mut gsl_spmatrix_char;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for spmatrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            65 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_char;
    }
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).nz = 0 as libc::c_int as size_t;
    (*m)
        .nzmax = if nzmax > 1 as libc::c_int as libc::c_ulong {
        nzmax
    } else {
        1 as libc::c_int as libc::c_ulong
    };
    (*m).sptype = sptype;
    if n1 == 1 as libc::c_int as libc::c_ulong && n2 == 1 as libc::c_int as libc::c_ulong
    {
        (*m).spflags = ((1 as libc::c_int) << 0 as libc::c_int) as size_t;
    } else {
        (*m).spflags = 0 as libc::c_int as size_t;
    }
    (*m)
        .i = malloc(
        ((*m).nzmax).wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if ((*m).i).is_null() {
        gsl_spmatrix_char_free(m);
        gsl_error(
            b"failed to allocate space for row indices\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_char;
    }
    (*m)
        .work
        .work_void = malloc(
        (if n1 > n2 { n1 } else { n2 })
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(
                (if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    > ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                {
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                } else {
                    ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                }),
            ),
    );
    if ((*m).work.work_void).is_null() {
        gsl_spmatrix_char_free(m);
        gsl_error(
            b"failed to allocate space for work\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_char;
    }
    if sptype == GSL_SPMATRIX_COO as libc::c_int {
        (*m)
            .tree = gsl_bst_alloc(
            gsl_bst_avl,
            &spmatrix_char_allocator,
            Some(
                compare_char_func
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            m as *mut libc::c_void,
        );
        if ((*m).tree).is_null() {
            gsl_spmatrix_char_free(m);
            gsl_error(
                b"failed to allocate space for binary tree\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                102 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_char;
        }
        (*m).node_size = gsl_bst_node_size((*m).tree);
        spmatrix_char_pool_init(m);
        (*m)
            .p = malloc(
            ((*m).nzmax)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_char_free(m);
            gsl_error(
                b"failed to allocate space for column indices\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                115 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_char;
        }
    } else if sptype == GSL_SPMATRIX_CSC as libc::c_int {
        (*m)
            .p = malloc(
            n2
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_char_free(m);
            gsl_error(
                b"failed to allocate space for column pointers\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                125 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_char;
        }
    } else if sptype == GSL_SPMATRIX_CSR as libc::c_int {
        (*m)
            .p = malloc(
            n1
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_char_free(m);
            gsl_error(
                b"failed to allocate space for row pointers\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                135 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_char;
        }
    }
    (*m)
        .data = malloc(
        ((*m).nzmax)
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    if ((*m).data).is_null() {
        gsl_spmatrix_char_free(m);
        gsl_error(
            b"failed to allocate space for data\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_char;
    }
    return m;
}
pub unsafe extern "C" fn gsl_spmatrix_uchar_alloc_nzmax(
    n1: size_t,
    n2: size_t,
    nzmax: size_t,
    sptype: libc::c_int,
) -> *mut gsl_spmatrix_uchar {
    let mut m: *mut gsl_spmatrix_uchar = 0 as *mut gsl_spmatrix_uchar;
    if n1 == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix dimension n1 must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_uchar;
    } else if n2 == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix dimension n2 must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_uchar;
    }
    m = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gsl_spmatrix_uchar>() as libc::c_ulong,
    ) as *mut gsl_spmatrix_uchar;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for spmatrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            65 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_uchar;
    }
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).nz = 0 as libc::c_int as size_t;
    (*m)
        .nzmax = if nzmax > 1 as libc::c_int as libc::c_ulong {
        nzmax
    } else {
        1 as libc::c_int as libc::c_ulong
    };
    (*m).sptype = sptype;
    if n1 == 1 as libc::c_int as libc::c_ulong && n2 == 1 as libc::c_int as libc::c_ulong
    {
        (*m).spflags = ((1 as libc::c_int) << 0 as libc::c_int) as size_t;
    } else {
        (*m).spflags = 0 as libc::c_int as size_t;
    }
    (*m)
        .i = malloc(
        ((*m).nzmax).wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if ((*m).i).is_null() {
        gsl_spmatrix_uchar_free(m);
        gsl_error(
            b"failed to allocate space for row indices\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_uchar;
    }
    (*m)
        .work
        .work_void = malloc(
        (if n1 > n2 { n1 } else { n2 })
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(
                (if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    > ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                {
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                } else {
                    ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                }),
            ),
    );
    if ((*m).work.work_void).is_null() {
        gsl_spmatrix_uchar_free(m);
        gsl_error(
            b"failed to allocate space for work\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_uchar;
    }
    if sptype == GSL_SPMATRIX_COO as libc::c_int {
        (*m)
            .tree = gsl_bst_alloc(
            gsl_bst_avl,
            &spmatrix_uchar_allocator,
            Some(
                compare_uchar_func
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            m as *mut libc::c_void,
        );
        if ((*m).tree).is_null() {
            gsl_spmatrix_uchar_free(m);
            gsl_error(
                b"failed to allocate space for binary tree\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                102 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_uchar;
        }
        (*m).node_size = gsl_bst_node_size((*m).tree);
        spmatrix_uchar_pool_init(m);
        (*m)
            .p = malloc(
            ((*m).nzmax)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_uchar_free(m);
            gsl_error(
                b"failed to allocate space for column indices\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                115 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_uchar;
        }
    } else if sptype == GSL_SPMATRIX_CSC as libc::c_int {
        (*m)
            .p = malloc(
            n2
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_uchar_free(m);
            gsl_error(
                b"failed to allocate space for column pointers\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                125 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_uchar;
        }
    } else if sptype == GSL_SPMATRIX_CSR as libc::c_int {
        (*m)
            .p = malloc(
            n1
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_uchar_free(m);
            gsl_error(
                b"failed to allocate space for row pointers\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                135 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_uchar;
        }
    }
    (*m)
        .data = malloc(
        ((*m).nzmax)
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
    ) as *mut libc::c_uchar;
    if ((*m).data).is_null() {
        gsl_spmatrix_uchar_free(m);
        gsl_error(
            b"failed to allocate space for data\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_uchar;
    }
    return m;
}
pub unsafe extern "C" fn gsl_spmatrix_alloc_nzmax(
    n1: size_t,
    n2: size_t,
    nzmax: size_t,
    sptype: libc::c_int,
) -> *mut gsl_spmatrix {
    let mut m: *mut gsl_spmatrix = 0 as *mut gsl_spmatrix;
    if n1 == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix dimension n1 must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix;
    } else if n2 == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix dimension n2 must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix;
    }
    m = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gsl_spmatrix>() as libc::c_ulong,
    ) as *mut gsl_spmatrix;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for spmatrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            65 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix;
    }
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).nz = 0 as libc::c_int as size_t;
    (*m)
        .nzmax = if nzmax > 1 as libc::c_int as libc::c_ulong {
        nzmax
    } else {
        1 as libc::c_int as libc::c_ulong
    };
    (*m).sptype = sptype;
    if n1 == 1 as libc::c_int as libc::c_ulong && n2 == 1 as libc::c_int as libc::c_ulong
    {
        (*m).spflags = ((1 as libc::c_int) << 0 as libc::c_int) as size_t;
    } else {
        (*m).spflags = 0 as libc::c_int as size_t;
    }
    (*m)
        .i = malloc(
        ((*m).nzmax).wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if ((*m).i).is_null() {
        gsl_spmatrix_free(m);
        gsl_error(
            b"failed to allocate space for row indices\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix;
    }
    (*m)
        .work
        .work_void = malloc(
        (if n1 > n2 { n1 } else { n2 })
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(
                (if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    > ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
                {
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                } else {
                    ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
                }),
            ),
    );
    if ((*m).work.work_void).is_null() {
        gsl_spmatrix_free(m);
        gsl_error(
            b"failed to allocate space for work\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix;
    }
    if sptype == GSL_SPMATRIX_COO as libc::c_int {
        (*m)
            .tree = gsl_bst_alloc(
            gsl_bst_avl,
            &spmatrix_allocator,
            Some(
                compare_func
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            m as *mut libc::c_void,
        );
        if ((*m).tree).is_null() {
            gsl_spmatrix_free(m);
            gsl_error(
                b"failed to allocate space for binary tree\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                102 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix;
        }
        (*m).node_size = gsl_bst_node_size((*m).tree);
        spmatrix_pool_init(m);
        (*m)
            .p = malloc(
            ((*m).nzmax)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_free(m);
            gsl_error(
                b"failed to allocate space for column indices\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                115 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix;
        }
    } else if sptype == GSL_SPMATRIX_CSC as libc::c_int {
        (*m)
            .p = malloc(
            n2
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_free(m);
            gsl_error(
                b"failed to allocate space for column pointers\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                125 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix;
        }
    } else if sptype == GSL_SPMATRIX_CSR as libc::c_int {
        (*m)
            .p = malloc(
            n1
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*m).p).is_null() {
            gsl_spmatrix_free(m);
            gsl_error(
                b"failed to allocate space for row pointers\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                135 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix;
        }
    }
    (*m)
        .data = malloc(
        ((*m).nzmax)
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*m).data).is_null() {
        gsl_spmatrix_free(m);
        gsl_error(
            b"failed to allocate space for data\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix;
    }
    return m;
}
pub unsafe extern "C" fn gsl_spmatrix_long_free(mut m: *mut gsl_spmatrix_long) {
    if !((*m).i).is_null() {
        free((*m).i as *mut libc::c_void);
    }
    if !((*m).p).is_null() {
        free((*m).p as *mut libc::c_void);
    }
    if !((*m).data).is_null() {
        free((*m).data as *mut libc::c_void);
    }
    if !((*m).work.work_void).is_null() {
        free((*m).work.work_void);
    }
    if !((*m).tree).is_null() {
        gsl_bst_free((*m).tree);
    }
    spmatrix_long_pool_free(m);
    free(m as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_spmatrix_complex_free(mut m: *mut gsl_spmatrix_complex) {
    if !((*m).i).is_null() {
        free((*m).i as *mut libc::c_void);
    }
    if !((*m).p).is_null() {
        free((*m).p as *mut libc::c_void);
    }
    if !((*m).data).is_null() {
        free((*m).data as *mut libc::c_void);
    }
    if !((*m).work.work_void).is_null() {
        free((*m).work.work_void);
    }
    if !((*m).tree).is_null() {
        gsl_bst_free((*m).tree);
    }
    spmatrix_complex_pool_free(m);
    free(m as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_spmatrix_ulong_free(mut m: *mut gsl_spmatrix_ulong) {
    if !((*m).i).is_null() {
        free((*m).i as *mut libc::c_void);
    }
    if !((*m).p).is_null() {
        free((*m).p as *mut libc::c_void);
    }
    if !((*m).data).is_null() {
        free((*m).data as *mut libc::c_void);
    }
    if !((*m).work.work_void).is_null() {
        free((*m).work.work_void);
    }
    if !((*m).tree).is_null() {
        gsl_bst_free((*m).tree);
    }
    spmatrix_ulong_pool_free(m);
    free(m as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_spmatrix_free(mut m: *mut gsl_spmatrix) {
    if !((*m).i).is_null() {
        free((*m).i as *mut libc::c_void);
    }
    if !((*m).p).is_null() {
        free((*m).p as *mut libc::c_void);
    }
    if !((*m).data).is_null() {
        free((*m).data as *mut libc::c_void);
    }
    if !((*m).work.work_void).is_null() {
        free((*m).work.work_void);
    }
    if !((*m).tree).is_null() {
        gsl_bst_free((*m).tree);
    }
    spmatrix_pool_free(m);
    free(m as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_spmatrix_float_free(mut m: *mut gsl_spmatrix_float) {
    if !((*m).i).is_null() {
        free((*m).i as *mut libc::c_void);
    }
    if !((*m).p).is_null() {
        free((*m).p as *mut libc::c_void);
    }
    if !((*m).data).is_null() {
        free((*m).data as *mut libc::c_void);
    }
    if !((*m).work.work_void).is_null() {
        free((*m).work.work_void);
    }
    if !((*m).tree).is_null() {
        gsl_bst_free((*m).tree);
    }
    spmatrix_float_pool_free(m);
    free(m as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_spmatrix_int_free(mut m: *mut gsl_spmatrix_int) {
    if !((*m).i).is_null() {
        free((*m).i as *mut libc::c_void);
    }
    if !((*m).p).is_null() {
        free((*m).p as *mut libc::c_void);
    }
    if !((*m).data).is_null() {
        free((*m).data as *mut libc::c_void);
    }
    if !((*m).work.work_void).is_null() {
        free((*m).work.work_void);
    }
    if !((*m).tree).is_null() {
        gsl_bst_free((*m).tree);
    }
    spmatrix_int_pool_free(m);
    free(m as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_spmatrix_char_free(mut m: *mut gsl_spmatrix_char) {
    if !((*m).i).is_null() {
        free((*m).i as *mut libc::c_void);
    }
    if !((*m).p).is_null() {
        free((*m).p as *mut libc::c_void);
    }
    if !((*m).data).is_null() {
        free((*m).data as *mut libc::c_void);
    }
    if !((*m).work.work_void).is_null() {
        free((*m).work.work_void);
    }
    if !((*m).tree).is_null() {
        gsl_bst_free((*m).tree);
    }
    spmatrix_char_pool_free(m);
    free(m as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_spmatrix_uchar_free(mut m: *mut gsl_spmatrix_uchar) {
    if !((*m).i).is_null() {
        free((*m).i as *mut libc::c_void);
    }
    if !((*m).p).is_null() {
        free((*m).p as *mut libc::c_void);
    }
    if !((*m).data).is_null() {
        free((*m).data as *mut libc::c_void);
    }
    if !((*m).work.work_void).is_null() {
        free((*m).work.work_void);
    }
    if !((*m).tree).is_null() {
        gsl_bst_free((*m).tree);
    }
    spmatrix_uchar_pool_free(m);
    free(m as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_spmatrix_uint_free(mut m: *mut gsl_spmatrix_uint) {
    if !((*m).i).is_null() {
        free((*m).i as *mut libc::c_void);
    }
    if !((*m).p).is_null() {
        free((*m).p as *mut libc::c_void);
    }
    if !((*m).data).is_null() {
        free((*m).data as *mut libc::c_void);
    }
    if !((*m).work.work_void).is_null() {
        free((*m).work.work_void);
    }
    if !((*m).tree).is_null() {
        gsl_bst_free((*m).tree);
    }
    spmatrix_uint_pool_free(m);
    free(m as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_free(
    mut m: *mut gsl_spmatrix_long_double,
) {
    if !((*m).i).is_null() {
        free((*m).i as *mut libc::c_void);
    }
    if !((*m).p).is_null() {
        free((*m).p as *mut libc::c_void);
    }
    if !((*m).data).is_null() {
        free((*m).data as *mut libc::c_void);
    }
    if !((*m).work.work_void).is_null() {
        free((*m).work.work_void);
    }
    if !((*m).tree).is_null() {
        gsl_bst_free((*m).tree);
    }
    spmatrix_long_double_pool_free(m);
    free(m as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_spmatrix_ushort_free(mut m: *mut gsl_spmatrix_ushort) {
    if !((*m).i).is_null() {
        free((*m).i as *mut libc::c_void);
    }
    if !((*m).p).is_null() {
        free((*m).p as *mut libc::c_void);
    }
    if !((*m).data).is_null() {
        free((*m).data as *mut libc::c_void);
    }
    if !((*m).work.work_void).is_null() {
        free((*m).work.work_void);
    }
    if !((*m).tree).is_null() {
        gsl_bst_free((*m).tree);
    }
    spmatrix_ushort_pool_free(m);
    free(m as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_spmatrix_complex_long_double_free(
    mut m: *mut gsl_spmatrix_complex_long_double,
) {
    if !((*m).i).is_null() {
        free((*m).i as *mut libc::c_void);
    }
    if !((*m).p).is_null() {
        free((*m).p as *mut libc::c_void);
    }
    if !((*m).data).is_null() {
        free((*m).data as *mut libc::c_void);
    }
    if !((*m).work.work_void).is_null() {
        free((*m).work.work_void);
    }
    if !((*m).tree).is_null() {
        gsl_bst_free((*m).tree);
    }
    spmatrix_complex_long_double_pool_free(m);
    free(m as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_spmatrix_complex_float_free(
    mut m: *mut gsl_spmatrix_complex_float,
) {
    if !((*m).i).is_null() {
        free((*m).i as *mut libc::c_void);
    }
    if !((*m).p).is_null() {
        free((*m).p as *mut libc::c_void);
    }
    if !((*m).data).is_null() {
        free((*m).data as *mut libc::c_void);
    }
    if !((*m).work.work_void).is_null() {
        free((*m).work.work_void);
    }
    if !((*m).tree).is_null() {
        gsl_bst_free((*m).tree);
    }
    spmatrix_complex_float_pool_free(m);
    free(m as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_spmatrix_short_free(mut m: *mut gsl_spmatrix_short) {
    if !((*m).i).is_null() {
        free((*m).i as *mut libc::c_void);
    }
    if !((*m).p).is_null() {
        free((*m).p as *mut libc::c_void);
    }
    if !((*m).data).is_null() {
        free((*m).data as *mut libc::c_void);
    }
    if !((*m).work.work_void).is_null() {
        free((*m).work.work_void);
    }
    if !((*m).tree).is_null() {
        gsl_bst_free((*m).tree);
    }
    spmatrix_short_pool_free(m);
    free(m as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_spmatrix_short_realloc(
    nzmax: size_t,
    mut m: *mut gsl_spmatrix_short,
) -> libc::c_int {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ptr_atomic: *mut libc::c_short = 0 as *mut libc::c_short;
    if nzmax < (*m).nz {
        gsl_error(
            b"new nzmax is less than current nz\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            183 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    ptr = realloc(
        (*m).i as *mut libc::c_void,
        nzmax.wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    if ptr.is_null() {
        gsl_error(
            b"failed to allocate space for row indices\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            189 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*m).i = ptr as *mut libc::c_int;
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        ptr = realloc(
            (*m).p as *mut libc::c_void,
            nzmax.wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        if ptr.is_null() {
            gsl_error(
                b"failed to allocate space for column indices\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                199 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*m).p = ptr as *mut libc::c_int;
    }
    ptr_atomic = realloc(
        (*m).data as *mut libc::c_void,
        nzmax
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_short>() as libc::c_ulong),
    ) as *mut libc::c_short;
    if ptr_atomic.is_null() {
        gsl_error(
            b"failed to allocate space for data\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let nnew: size_t = nzmax.wrapping_sub((*m).nz);
        let mut node: *mut gsl_spmatrix_pool = 0 as *mut gsl_spmatrix_pool;
        if ptr_atomic != (*m).data {
            let mut trav: gsl_bst_trav = gsl_bst_trav {
                type_0: 0 as *const gsl_bst_type,
                trav_data: C2RustUnnamed_1 {
                    avl_trav: gsl_bst_avl_traverser {
                        avl_table: 0 as *const gsl_bst_avl_table,
                        avl_node: 0 as *mut gsl_bst_avl_node,
                        avl_stack: [0 as *mut gsl_bst_avl_node; 32],
                        avl_height: 0,
                        avl_generation: 0,
                    },
                },
            };
            ptr = gsl_bst_trav_first(&mut trav, (*m).tree);
            while !ptr.is_null() {
                let mut q: *mut libc::c_short = ptr as *mut libc::c_short;
                let mut idx: size_t = q.offset_from((*m).data) as libc::c_long as size_t;
                gsl_bst_trav_replace(
                    &mut trav,
                    &mut *ptr_atomic.offset(idx as isize) as *mut libc::c_short
                        as *mut libc::c_void,
                );
                ptr = gsl_bst_trav_next(&mut trav);
            }
        }
        node = malloc(::std::mem::size_of::<gsl_spmatrix_pool>() as libc::c_ulong)
            as *mut gsl_spmatrix_pool;
        if node.is_null() {
            gsl_error(
                b"failed to allocate space for memory pool node\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                239 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*node).block_ptr = malloc(nnew.wrapping_mul((*m).node_size));
        if ((*node).block_ptr).is_null() {
            gsl_error(
                b"failed to allocate space for memory pool block\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                245 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*node).free_slot = (*node).block_ptr as *mut libc::c_uchar;
        (*node).next = (*m).pool;
        (*m).pool = node;
    }
    (*m).data = ptr_atomic;
    (*m).nzmax = nzmax;
    return status;
}
pub unsafe extern "C" fn gsl_spmatrix_ulong_realloc(
    nzmax: size_t,
    mut m: *mut gsl_spmatrix_ulong,
) -> libc::c_int {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ptr_atomic: *mut libc::c_ulong = 0 as *mut libc::c_ulong;
    if nzmax < (*m).nz {
        gsl_error(
            b"new nzmax is less than current nz\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            183 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    ptr = realloc(
        (*m).i as *mut libc::c_void,
        nzmax.wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    if ptr.is_null() {
        gsl_error(
            b"failed to allocate space for row indices\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            189 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*m).i = ptr as *mut libc::c_int;
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        ptr = realloc(
            (*m).p as *mut libc::c_void,
            nzmax.wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        if ptr.is_null() {
            gsl_error(
                b"failed to allocate space for column indices\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                199 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*m).p = ptr as *mut libc::c_int;
    }
    ptr_atomic = realloc(
        (*m).data as *mut libc::c_void,
        nzmax
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong),
    ) as *mut libc::c_ulong;
    if ptr_atomic.is_null() {
        gsl_error(
            b"failed to allocate space for data\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let nnew: size_t = nzmax.wrapping_sub((*m).nz);
        let mut node: *mut gsl_spmatrix_pool = 0 as *mut gsl_spmatrix_pool;
        if ptr_atomic != (*m).data {
            let mut trav: gsl_bst_trav = gsl_bst_trav {
                type_0: 0 as *const gsl_bst_type,
                trav_data: C2RustUnnamed_1 {
                    avl_trav: gsl_bst_avl_traverser {
                        avl_table: 0 as *const gsl_bst_avl_table,
                        avl_node: 0 as *mut gsl_bst_avl_node,
                        avl_stack: [0 as *mut gsl_bst_avl_node; 32],
                        avl_height: 0,
                        avl_generation: 0,
                    },
                },
            };
            ptr = gsl_bst_trav_first(&mut trav, (*m).tree);
            while !ptr.is_null() {
                let mut q: *mut libc::c_ulong = ptr as *mut libc::c_ulong;
                let mut idx: size_t = q.offset_from((*m).data) as libc::c_long as size_t;
                gsl_bst_trav_replace(
                    &mut trav,
                    &mut *ptr_atomic.offset(idx as isize) as *mut libc::c_ulong
                        as *mut libc::c_void,
                );
                ptr = gsl_bst_trav_next(&mut trav);
            }
        }
        node = malloc(::std::mem::size_of::<gsl_spmatrix_pool>() as libc::c_ulong)
            as *mut gsl_spmatrix_pool;
        if node.is_null() {
            gsl_error(
                b"failed to allocate space for memory pool node\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                239 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*node).block_ptr = malloc(nnew.wrapping_mul((*m).node_size));
        if ((*node).block_ptr).is_null() {
            gsl_error(
                b"failed to allocate space for memory pool block\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                245 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*node).free_slot = (*node).block_ptr as *mut libc::c_uchar;
        (*node).next = (*m).pool;
        (*m).pool = node;
    }
    (*m).data = ptr_atomic;
    (*m).nzmax = nzmax;
    return status;
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_realloc(
    nzmax: size_t,
    mut m: *mut gsl_spmatrix_long_double,
) -> libc::c_int {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ptr_atomic: *mut f128::f128 = 0 as *mut f128::f128;
    if nzmax < (*m).nz {
        gsl_error(
            b"new nzmax is less than current nz\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            183 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    ptr = realloc(
        (*m).i as *mut libc::c_void,
        nzmax.wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    if ptr.is_null() {
        gsl_error(
            b"failed to allocate space for row indices\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            189 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*m).i = ptr as *mut libc::c_int;
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        ptr = realloc(
            (*m).p as *mut libc::c_void,
            nzmax.wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        if ptr.is_null() {
            gsl_error(
                b"failed to allocate space for column indices\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                199 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*m).p = ptr as *mut libc::c_int;
    }
    ptr_atomic = realloc(
        (*m).data as *mut libc::c_void,
        nzmax
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<f128::f128>() as libc::c_ulong),
    ) as *mut f128::f128;
    if ptr_atomic.is_null() {
        gsl_error(
            b"failed to allocate space for data\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let nnew: size_t = nzmax.wrapping_sub((*m).nz);
        let mut node: *mut gsl_spmatrix_pool = 0 as *mut gsl_spmatrix_pool;
        if ptr_atomic != (*m).data {
            let mut trav: gsl_bst_trav = gsl_bst_trav {
                type_0: 0 as *const gsl_bst_type,
                trav_data: C2RustUnnamed_1 {
                    avl_trav: gsl_bst_avl_traverser {
                        avl_table: 0 as *const gsl_bst_avl_table,
                        avl_node: 0 as *mut gsl_bst_avl_node,
                        avl_stack: [0 as *mut gsl_bst_avl_node; 32],
                        avl_height: 0,
                        avl_generation: 0,
                    },
                },
            };
            ptr = gsl_bst_trav_first(&mut trav, (*m).tree);
            while !ptr.is_null() {
                let mut q: *mut f128::f128 = ptr as *mut f128::f128;
                let mut idx: size_t = q.offset_from((*m).data) as libc::c_long as size_t;
                gsl_bst_trav_replace(
                    &mut trav,
                    &mut *ptr_atomic.offset(idx as isize) as *mut f128::f128
                        as *mut libc::c_void,
                );
                ptr = gsl_bst_trav_next(&mut trav);
            }
        }
        node = malloc(::std::mem::size_of::<gsl_spmatrix_pool>() as libc::c_ulong)
            as *mut gsl_spmatrix_pool;
        if node.is_null() {
            gsl_error(
                b"failed to allocate space for memory pool node\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                239 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*node).block_ptr = malloc(nnew.wrapping_mul((*m).node_size));
        if ((*node).block_ptr).is_null() {
            gsl_error(
                b"failed to allocate space for memory pool block\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                245 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*node).free_slot = (*node).block_ptr as *mut libc::c_uchar;
        (*node).next = (*m).pool;
        (*m).pool = node;
    }
    (*m).data = ptr_atomic;
    (*m).nzmax = nzmax;
    return status;
}
pub unsafe extern "C" fn gsl_spmatrix_float_realloc(
    nzmax: size_t,
    mut m: *mut gsl_spmatrix_float,
) -> libc::c_int {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ptr_atomic: *mut libc::c_float = 0 as *mut libc::c_float;
    if nzmax < (*m).nz {
        gsl_error(
            b"new nzmax is less than current nz\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            183 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    ptr = realloc(
        (*m).i as *mut libc::c_void,
        nzmax.wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    if ptr.is_null() {
        gsl_error(
            b"failed to allocate space for row indices\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            189 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*m).i = ptr as *mut libc::c_int;
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        ptr = realloc(
            (*m).p as *mut libc::c_void,
            nzmax.wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        if ptr.is_null() {
            gsl_error(
                b"failed to allocate space for column indices\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                199 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*m).p = ptr as *mut libc::c_int;
    }
    ptr_atomic = realloc(
        (*m).data as *mut libc::c_void,
        nzmax
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
    ) as *mut libc::c_float;
    if ptr_atomic.is_null() {
        gsl_error(
            b"failed to allocate space for data\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let nnew: size_t = nzmax.wrapping_sub((*m).nz);
        let mut node: *mut gsl_spmatrix_pool = 0 as *mut gsl_spmatrix_pool;
        if ptr_atomic != (*m).data {
            let mut trav: gsl_bst_trav = gsl_bst_trav {
                type_0: 0 as *const gsl_bst_type,
                trav_data: C2RustUnnamed_1 {
                    avl_trav: gsl_bst_avl_traverser {
                        avl_table: 0 as *const gsl_bst_avl_table,
                        avl_node: 0 as *mut gsl_bst_avl_node,
                        avl_stack: [0 as *mut gsl_bst_avl_node; 32],
                        avl_height: 0,
                        avl_generation: 0,
                    },
                },
            };
            ptr = gsl_bst_trav_first(&mut trav, (*m).tree);
            while !ptr.is_null() {
                let mut q: *mut libc::c_float = ptr as *mut libc::c_float;
                let mut idx: size_t = q.offset_from((*m).data) as libc::c_long as size_t;
                gsl_bst_trav_replace(
                    &mut trav,
                    &mut *ptr_atomic.offset(idx as isize) as *mut libc::c_float
                        as *mut libc::c_void,
                );
                ptr = gsl_bst_trav_next(&mut trav);
            }
        }
        node = malloc(::std::mem::size_of::<gsl_spmatrix_pool>() as libc::c_ulong)
            as *mut gsl_spmatrix_pool;
        if node.is_null() {
            gsl_error(
                b"failed to allocate space for memory pool node\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                239 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*node).block_ptr = malloc(nnew.wrapping_mul((*m).node_size));
        if ((*node).block_ptr).is_null() {
            gsl_error(
                b"failed to allocate space for memory pool block\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                245 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*node).free_slot = (*node).block_ptr as *mut libc::c_uchar;
        (*node).next = (*m).pool;
        (*m).pool = node;
    }
    (*m).data = ptr_atomic;
    (*m).nzmax = nzmax;
    return status;
}
pub unsafe extern "C" fn gsl_spmatrix_realloc(
    nzmax: size_t,
    mut m: *mut gsl_spmatrix,
) -> libc::c_int {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ptr_atomic: *mut libc::c_double = 0 as *mut libc::c_double;
    if nzmax < (*m).nz {
        gsl_error(
            b"new nzmax is less than current nz\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            183 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    ptr = realloc(
        (*m).i as *mut libc::c_void,
        nzmax.wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    if ptr.is_null() {
        gsl_error(
            b"failed to allocate space for row indices\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            189 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*m).i = ptr as *mut libc::c_int;
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        ptr = realloc(
            (*m).p as *mut libc::c_void,
            nzmax.wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        if ptr.is_null() {
            gsl_error(
                b"failed to allocate space for column indices\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                199 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*m).p = ptr as *mut libc::c_int;
    }
    ptr_atomic = realloc(
        (*m).data as *mut libc::c_void,
        nzmax
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ptr_atomic.is_null() {
        gsl_error(
            b"failed to allocate space for data\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let nnew: size_t = nzmax.wrapping_sub((*m).nz);
        let mut node: *mut gsl_spmatrix_pool = 0 as *mut gsl_spmatrix_pool;
        if ptr_atomic != (*m).data {
            let mut trav: gsl_bst_trav = gsl_bst_trav {
                type_0: 0 as *const gsl_bst_type,
                trav_data: C2RustUnnamed_1 {
                    avl_trav: gsl_bst_avl_traverser {
                        avl_table: 0 as *const gsl_bst_avl_table,
                        avl_node: 0 as *mut gsl_bst_avl_node,
                        avl_stack: [0 as *mut gsl_bst_avl_node; 32],
                        avl_height: 0,
                        avl_generation: 0,
                    },
                },
            };
            ptr = gsl_bst_trav_first(&mut trav, (*m).tree);
            while !ptr.is_null() {
                let mut q: *mut libc::c_double = ptr as *mut libc::c_double;
                let mut idx: size_t = q.offset_from((*m).data) as libc::c_long as size_t;
                gsl_bst_trav_replace(
                    &mut trav,
                    &mut *ptr_atomic.offset(idx as isize) as *mut libc::c_double
                        as *mut libc::c_void,
                );
                ptr = gsl_bst_trav_next(&mut trav);
            }
        }
        node = malloc(::std::mem::size_of::<gsl_spmatrix_pool>() as libc::c_ulong)
            as *mut gsl_spmatrix_pool;
        if node.is_null() {
            gsl_error(
                b"failed to allocate space for memory pool node\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                239 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*node).block_ptr = malloc(nnew.wrapping_mul((*m).node_size));
        if ((*node).block_ptr).is_null() {
            gsl_error(
                b"failed to allocate space for memory pool block\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                245 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*node).free_slot = (*node).block_ptr as *mut libc::c_uchar;
        (*node).next = (*m).pool;
        (*m).pool = node;
    }
    (*m).data = ptr_atomic;
    (*m).nzmax = nzmax;
    return status;
}
pub unsafe extern "C" fn gsl_spmatrix_int_realloc(
    nzmax: size_t,
    mut m: *mut gsl_spmatrix_int,
) -> libc::c_int {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ptr_atomic: *mut libc::c_int = 0 as *mut libc::c_int;
    if nzmax < (*m).nz {
        gsl_error(
            b"new nzmax is less than current nz\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            183 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    ptr = realloc(
        (*m).i as *mut libc::c_void,
        nzmax.wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    if ptr.is_null() {
        gsl_error(
            b"failed to allocate space for row indices\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            189 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*m).i = ptr as *mut libc::c_int;
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        ptr = realloc(
            (*m).p as *mut libc::c_void,
            nzmax.wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        if ptr.is_null() {
            gsl_error(
                b"failed to allocate space for column indices\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                199 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*m).p = ptr as *mut libc::c_int;
    }
    ptr_atomic = realloc(
        (*m).data as *mut libc::c_void,
        nzmax
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if ptr_atomic.is_null() {
        gsl_error(
            b"failed to allocate space for data\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let nnew: size_t = nzmax.wrapping_sub((*m).nz);
        let mut node: *mut gsl_spmatrix_pool = 0 as *mut gsl_spmatrix_pool;
        if ptr_atomic != (*m).data {
            let mut trav: gsl_bst_trav = gsl_bst_trav {
                type_0: 0 as *const gsl_bst_type,
                trav_data: C2RustUnnamed_1 {
                    avl_trav: gsl_bst_avl_traverser {
                        avl_table: 0 as *const gsl_bst_avl_table,
                        avl_node: 0 as *mut gsl_bst_avl_node,
                        avl_stack: [0 as *mut gsl_bst_avl_node; 32],
                        avl_height: 0,
                        avl_generation: 0,
                    },
                },
            };
            ptr = gsl_bst_trav_first(&mut trav, (*m).tree);
            while !ptr.is_null() {
                let mut q: *mut libc::c_int = ptr as *mut libc::c_int;
                let mut idx: size_t = q.offset_from((*m).data) as libc::c_long as size_t;
                gsl_bst_trav_replace(
                    &mut trav,
                    &mut *ptr_atomic.offset(idx as isize) as *mut libc::c_int
                        as *mut libc::c_void,
                );
                ptr = gsl_bst_trav_next(&mut trav);
            }
        }
        node = malloc(::std::mem::size_of::<gsl_spmatrix_pool>() as libc::c_ulong)
            as *mut gsl_spmatrix_pool;
        if node.is_null() {
            gsl_error(
                b"failed to allocate space for memory pool node\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                239 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*node).block_ptr = malloc(nnew.wrapping_mul((*m).node_size));
        if ((*node).block_ptr).is_null() {
            gsl_error(
                b"failed to allocate space for memory pool block\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                245 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*node).free_slot = (*node).block_ptr as *mut libc::c_uchar;
        (*node).next = (*m).pool;
        (*m).pool = node;
    }
    (*m).data = ptr_atomic;
    (*m).nzmax = nzmax;
    return status;
}
pub unsafe extern "C" fn gsl_spmatrix_uint_realloc(
    nzmax: size_t,
    mut m: *mut gsl_spmatrix_uint,
) -> libc::c_int {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ptr_atomic: *mut libc::c_uint = 0 as *mut libc::c_uint;
    if nzmax < (*m).nz {
        gsl_error(
            b"new nzmax is less than current nz\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            183 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    ptr = realloc(
        (*m).i as *mut libc::c_void,
        nzmax.wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    if ptr.is_null() {
        gsl_error(
            b"failed to allocate space for row indices\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            189 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*m).i = ptr as *mut libc::c_int;
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        ptr = realloc(
            (*m).p as *mut libc::c_void,
            nzmax.wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        if ptr.is_null() {
            gsl_error(
                b"failed to allocate space for column indices\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                199 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*m).p = ptr as *mut libc::c_int;
    }
    ptr_atomic = realloc(
        (*m).data as *mut libc::c_void,
        nzmax
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong),
    ) as *mut libc::c_uint;
    if ptr_atomic.is_null() {
        gsl_error(
            b"failed to allocate space for data\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let nnew: size_t = nzmax.wrapping_sub((*m).nz);
        let mut node: *mut gsl_spmatrix_pool = 0 as *mut gsl_spmatrix_pool;
        if ptr_atomic != (*m).data {
            let mut trav: gsl_bst_trav = gsl_bst_trav {
                type_0: 0 as *const gsl_bst_type,
                trav_data: C2RustUnnamed_1 {
                    avl_trav: gsl_bst_avl_traverser {
                        avl_table: 0 as *const gsl_bst_avl_table,
                        avl_node: 0 as *mut gsl_bst_avl_node,
                        avl_stack: [0 as *mut gsl_bst_avl_node; 32],
                        avl_height: 0,
                        avl_generation: 0,
                    },
                },
            };
            ptr = gsl_bst_trav_first(&mut trav, (*m).tree);
            while !ptr.is_null() {
                let mut q: *mut libc::c_uint = ptr as *mut libc::c_uint;
                let mut idx: size_t = q.offset_from((*m).data) as libc::c_long as size_t;
                gsl_bst_trav_replace(
                    &mut trav,
                    &mut *ptr_atomic.offset(idx as isize) as *mut libc::c_uint
                        as *mut libc::c_void,
                );
                ptr = gsl_bst_trav_next(&mut trav);
            }
        }
        node = malloc(::std::mem::size_of::<gsl_spmatrix_pool>() as libc::c_ulong)
            as *mut gsl_spmatrix_pool;
        if node.is_null() {
            gsl_error(
                b"failed to allocate space for memory pool node\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                239 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*node).block_ptr = malloc(nnew.wrapping_mul((*m).node_size));
        if ((*node).block_ptr).is_null() {
            gsl_error(
                b"failed to allocate space for memory pool block\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                245 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*node).free_slot = (*node).block_ptr as *mut libc::c_uchar;
        (*node).next = (*m).pool;
        (*m).pool = node;
    }
    (*m).data = ptr_atomic;
    (*m).nzmax = nzmax;
    return status;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_float_realloc(
    nzmax: size_t,
    mut m: *mut gsl_spmatrix_complex_float,
) -> libc::c_int {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ptr_atomic: *mut libc::c_float = 0 as *mut libc::c_float;
    if nzmax < (*m).nz {
        gsl_error(
            b"new nzmax is less than current nz\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            183 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    ptr = realloc(
        (*m).i as *mut libc::c_void,
        nzmax.wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    if ptr.is_null() {
        gsl_error(
            b"failed to allocate space for row indices\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            189 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*m).i = ptr as *mut libc::c_int;
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        ptr = realloc(
            (*m).p as *mut libc::c_void,
            nzmax.wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        if ptr.is_null() {
            gsl_error(
                b"failed to allocate space for column indices\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                199 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*m).p = ptr as *mut libc::c_int;
    }
    ptr_atomic = realloc(
        (*m).data as *mut libc::c_void,
        nzmax
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
    ) as *mut libc::c_float;
    if ptr_atomic.is_null() {
        gsl_error(
            b"failed to allocate space for data\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let nnew: size_t = nzmax.wrapping_sub((*m).nz);
        let mut node: *mut gsl_spmatrix_pool = 0 as *mut gsl_spmatrix_pool;
        if ptr_atomic != (*m).data {
            let mut trav: gsl_bst_trav = gsl_bst_trav {
                type_0: 0 as *const gsl_bst_type,
                trav_data: C2RustUnnamed_1 {
                    avl_trav: gsl_bst_avl_traverser {
                        avl_table: 0 as *const gsl_bst_avl_table,
                        avl_node: 0 as *mut gsl_bst_avl_node,
                        avl_stack: [0 as *mut gsl_bst_avl_node; 32],
                        avl_height: 0,
                        avl_generation: 0,
                    },
                },
            };
            ptr = gsl_bst_trav_first(&mut trav, (*m).tree);
            while !ptr.is_null() {
                let mut q: *mut libc::c_float = ptr as *mut libc::c_float;
                let mut idx: size_t = q.offset_from((*m).data) as libc::c_long as size_t;
                gsl_bst_trav_replace(
                    &mut trav,
                    &mut *ptr_atomic.offset(idx as isize) as *mut libc::c_float
                        as *mut libc::c_void,
                );
                ptr = gsl_bst_trav_next(&mut trav);
            }
        }
        node = malloc(::std::mem::size_of::<gsl_spmatrix_pool>() as libc::c_ulong)
            as *mut gsl_spmatrix_pool;
        if node.is_null() {
            gsl_error(
                b"failed to allocate space for memory pool node\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                239 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*node).block_ptr = malloc(nnew.wrapping_mul((*m).node_size));
        if ((*node).block_ptr).is_null() {
            gsl_error(
                b"failed to allocate space for memory pool block\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                245 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*node).free_slot = (*node).block_ptr as *mut libc::c_uchar;
        (*node).next = (*m).pool;
        (*m).pool = node;
    }
    (*m).data = ptr_atomic;
    (*m).nzmax = nzmax;
    return status;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_realloc(
    nzmax: size_t,
    mut m: *mut gsl_spmatrix_complex,
) -> libc::c_int {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ptr_atomic: *mut libc::c_double = 0 as *mut libc::c_double;
    if nzmax < (*m).nz {
        gsl_error(
            b"new nzmax is less than current nz\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            183 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    ptr = realloc(
        (*m).i as *mut libc::c_void,
        nzmax.wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    if ptr.is_null() {
        gsl_error(
            b"failed to allocate space for row indices\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            189 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*m).i = ptr as *mut libc::c_int;
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        ptr = realloc(
            (*m).p as *mut libc::c_void,
            nzmax.wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        if ptr.is_null() {
            gsl_error(
                b"failed to allocate space for column indices\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                199 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*m).p = ptr as *mut libc::c_int;
    }
    ptr_atomic = realloc(
        (*m).data as *mut libc::c_void,
        nzmax
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ptr_atomic.is_null() {
        gsl_error(
            b"failed to allocate space for data\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let nnew: size_t = nzmax.wrapping_sub((*m).nz);
        let mut node: *mut gsl_spmatrix_pool = 0 as *mut gsl_spmatrix_pool;
        if ptr_atomic != (*m).data {
            let mut trav: gsl_bst_trav = gsl_bst_trav {
                type_0: 0 as *const gsl_bst_type,
                trav_data: C2RustUnnamed_1 {
                    avl_trav: gsl_bst_avl_traverser {
                        avl_table: 0 as *const gsl_bst_avl_table,
                        avl_node: 0 as *mut gsl_bst_avl_node,
                        avl_stack: [0 as *mut gsl_bst_avl_node; 32],
                        avl_height: 0,
                        avl_generation: 0,
                    },
                },
            };
            ptr = gsl_bst_trav_first(&mut trav, (*m).tree);
            while !ptr.is_null() {
                let mut q: *mut libc::c_double = ptr as *mut libc::c_double;
                let mut idx: size_t = q.offset_from((*m).data) as libc::c_long as size_t;
                gsl_bst_trav_replace(
                    &mut trav,
                    &mut *ptr_atomic.offset(idx as isize) as *mut libc::c_double
                        as *mut libc::c_void,
                );
                ptr = gsl_bst_trav_next(&mut trav);
            }
        }
        node = malloc(::std::mem::size_of::<gsl_spmatrix_pool>() as libc::c_ulong)
            as *mut gsl_spmatrix_pool;
        if node.is_null() {
            gsl_error(
                b"failed to allocate space for memory pool node\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                239 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*node).block_ptr = malloc(nnew.wrapping_mul((*m).node_size));
        if ((*node).block_ptr).is_null() {
            gsl_error(
                b"failed to allocate space for memory pool block\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                245 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*node).free_slot = (*node).block_ptr as *mut libc::c_uchar;
        (*node).next = (*m).pool;
        (*m).pool = node;
    }
    (*m).data = ptr_atomic;
    (*m).nzmax = nzmax;
    return status;
}
pub unsafe extern "C" fn gsl_spmatrix_long_realloc(
    nzmax: size_t,
    mut m: *mut gsl_spmatrix_long,
) -> libc::c_int {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ptr_atomic: *mut libc::c_long = 0 as *mut libc::c_long;
    if nzmax < (*m).nz {
        gsl_error(
            b"new nzmax is less than current nz\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            183 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    ptr = realloc(
        (*m).i as *mut libc::c_void,
        nzmax.wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    if ptr.is_null() {
        gsl_error(
            b"failed to allocate space for row indices\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            189 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*m).i = ptr as *mut libc::c_int;
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        ptr = realloc(
            (*m).p as *mut libc::c_void,
            nzmax.wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        if ptr.is_null() {
            gsl_error(
                b"failed to allocate space for column indices\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                199 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*m).p = ptr as *mut libc::c_int;
    }
    ptr_atomic = realloc(
        (*m).data as *mut libc::c_void,
        nzmax
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_long>() as libc::c_ulong),
    ) as *mut libc::c_long;
    if ptr_atomic.is_null() {
        gsl_error(
            b"failed to allocate space for data\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let nnew: size_t = nzmax.wrapping_sub((*m).nz);
        let mut node: *mut gsl_spmatrix_pool = 0 as *mut gsl_spmatrix_pool;
        if ptr_atomic != (*m).data {
            let mut trav: gsl_bst_trav = gsl_bst_trav {
                type_0: 0 as *const gsl_bst_type,
                trav_data: C2RustUnnamed_1 {
                    avl_trav: gsl_bst_avl_traverser {
                        avl_table: 0 as *const gsl_bst_avl_table,
                        avl_node: 0 as *mut gsl_bst_avl_node,
                        avl_stack: [0 as *mut gsl_bst_avl_node; 32],
                        avl_height: 0,
                        avl_generation: 0,
                    },
                },
            };
            ptr = gsl_bst_trav_first(&mut trav, (*m).tree);
            while !ptr.is_null() {
                let mut q: *mut libc::c_long = ptr as *mut libc::c_long;
                let mut idx: size_t = q.offset_from((*m).data) as libc::c_long as size_t;
                gsl_bst_trav_replace(
                    &mut trav,
                    &mut *ptr_atomic.offset(idx as isize) as *mut libc::c_long
                        as *mut libc::c_void,
                );
                ptr = gsl_bst_trav_next(&mut trav);
            }
        }
        node = malloc(::std::mem::size_of::<gsl_spmatrix_pool>() as libc::c_ulong)
            as *mut gsl_spmatrix_pool;
        if node.is_null() {
            gsl_error(
                b"failed to allocate space for memory pool node\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                239 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*node).block_ptr = malloc(nnew.wrapping_mul((*m).node_size));
        if ((*node).block_ptr).is_null() {
            gsl_error(
                b"failed to allocate space for memory pool block\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                245 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*node).free_slot = (*node).block_ptr as *mut libc::c_uchar;
        (*node).next = (*m).pool;
        (*m).pool = node;
    }
    (*m).data = ptr_atomic;
    (*m).nzmax = nzmax;
    return status;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_long_double_realloc(
    nzmax: size_t,
    mut m: *mut gsl_spmatrix_complex_long_double,
) -> libc::c_int {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ptr_atomic: *mut f128::f128 = 0 as *mut f128::f128;
    if nzmax < (*m).nz {
        gsl_error(
            b"new nzmax is less than current nz\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            183 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    ptr = realloc(
        (*m).i as *mut libc::c_void,
        nzmax.wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    if ptr.is_null() {
        gsl_error(
            b"failed to allocate space for row indices\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            189 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*m).i = ptr as *mut libc::c_int;
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        ptr = realloc(
            (*m).p as *mut libc::c_void,
            nzmax.wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        if ptr.is_null() {
            gsl_error(
                b"failed to allocate space for column indices\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                199 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*m).p = ptr as *mut libc::c_int;
    }
    ptr_atomic = realloc(
        (*m).data as *mut libc::c_void,
        nzmax
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<f128::f128>() as libc::c_ulong),
    ) as *mut f128::f128;
    if ptr_atomic.is_null() {
        gsl_error(
            b"failed to allocate space for data\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let nnew: size_t = nzmax.wrapping_sub((*m).nz);
        let mut node: *mut gsl_spmatrix_pool = 0 as *mut gsl_spmatrix_pool;
        if ptr_atomic != (*m).data {
            let mut trav: gsl_bst_trav = gsl_bst_trav {
                type_0: 0 as *const gsl_bst_type,
                trav_data: C2RustUnnamed_1 {
                    avl_trav: gsl_bst_avl_traverser {
                        avl_table: 0 as *const gsl_bst_avl_table,
                        avl_node: 0 as *mut gsl_bst_avl_node,
                        avl_stack: [0 as *mut gsl_bst_avl_node; 32],
                        avl_height: 0,
                        avl_generation: 0,
                    },
                },
            };
            ptr = gsl_bst_trav_first(&mut trav, (*m).tree);
            while !ptr.is_null() {
                let mut q: *mut f128::f128 = ptr as *mut f128::f128;
                let mut idx: size_t = q.offset_from((*m).data) as libc::c_long as size_t;
                gsl_bst_trav_replace(
                    &mut trav,
                    &mut *ptr_atomic.offset(idx as isize) as *mut f128::f128
                        as *mut libc::c_void,
                );
                ptr = gsl_bst_trav_next(&mut trav);
            }
        }
        node = malloc(::std::mem::size_of::<gsl_spmatrix_pool>() as libc::c_ulong)
            as *mut gsl_spmatrix_pool;
        if node.is_null() {
            gsl_error(
                b"failed to allocate space for memory pool node\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                239 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*node).block_ptr = malloc(nnew.wrapping_mul((*m).node_size));
        if ((*node).block_ptr).is_null() {
            gsl_error(
                b"failed to allocate space for memory pool block\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                245 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*node).free_slot = (*node).block_ptr as *mut libc::c_uchar;
        (*node).next = (*m).pool;
        (*m).pool = node;
    }
    (*m).data = ptr_atomic;
    (*m).nzmax = nzmax;
    return status;
}
pub unsafe extern "C" fn gsl_spmatrix_ushort_realloc(
    nzmax: size_t,
    mut m: *mut gsl_spmatrix_ushort,
) -> libc::c_int {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ptr_atomic: *mut libc::c_ushort = 0 as *mut libc::c_ushort;
    if nzmax < (*m).nz {
        gsl_error(
            b"new nzmax is less than current nz\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            183 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    ptr = realloc(
        (*m).i as *mut libc::c_void,
        nzmax.wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    if ptr.is_null() {
        gsl_error(
            b"failed to allocate space for row indices\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            189 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*m).i = ptr as *mut libc::c_int;
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        ptr = realloc(
            (*m).p as *mut libc::c_void,
            nzmax.wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        if ptr.is_null() {
            gsl_error(
                b"failed to allocate space for column indices\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                199 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*m).p = ptr as *mut libc::c_int;
    }
    ptr_atomic = realloc(
        (*m).data as *mut libc::c_void,
        nzmax
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong),
    ) as *mut libc::c_ushort;
    if ptr_atomic.is_null() {
        gsl_error(
            b"failed to allocate space for data\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let nnew: size_t = nzmax.wrapping_sub((*m).nz);
        let mut node: *mut gsl_spmatrix_pool = 0 as *mut gsl_spmatrix_pool;
        if ptr_atomic != (*m).data {
            let mut trav: gsl_bst_trav = gsl_bst_trav {
                type_0: 0 as *const gsl_bst_type,
                trav_data: C2RustUnnamed_1 {
                    avl_trav: gsl_bst_avl_traverser {
                        avl_table: 0 as *const gsl_bst_avl_table,
                        avl_node: 0 as *mut gsl_bst_avl_node,
                        avl_stack: [0 as *mut gsl_bst_avl_node; 32],
                        avl_height: 0,
                        avl_generation: 0,
                    },
                },
            };
            ptr = gsl_bst_trav_first(&mut trav, (*m).tree);
            while !ptr.is_null() {
                let mut q: *mut libc::c_ushort = ptr as *mut libc::c_ushort;
                let mut idx: size_t = q.offset_from((*m).data) as libc::c_long as size_t;
                gsl_bst_trav_replace(
                    &mut trav,
                    &mut *ptr_atomic.offset(idx as isize) as *mut libc::c_ushort
                        as *mut libc::c_void,
                );
                ptr = gsl_bst_trav_next(&mut trav);
            }
        }
        node = malloc(::std::mem::size_of::<gsl_spmatrix_pool>() as libc::c_ulong)
            as *mut gsl_spmatrix_pool;
        if node.is_null() {
            gsl_error(
                b"failed to allocate space for memory pool node\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                239 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*node).block_ptr = malloc(nnew.wrapping_mul((*m).node_size));
        if ((*node).block_ptr).is_null() {
            gsl_error(
                b"failed to allocate space for memory pool block\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                245 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*node).free_slot = (*node).block_ptr as *mut libc::c_uchar;
        (*node).next = (*m).pool;
        (*m).pool = node;
    }
    (*m).data = ptr_atomic;
    (*m).nzmax = nzmax;
    return status;
}
pub unsafe extern "C" fn gsl_spmatrix_uchar_realloc(
    nzmax: size_t,
    mut m: *mut gsl_spmatrix_uchar,
) -> libc::c_int {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ptr_atomic: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if nzmax < (*m).nz {
        gsl_error(
            b"new nzmax is less than current nz\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            183 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    ptr = realloc(
        (*m).i as *mut libc::c_void,
        nzmax.wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    if ptr.is_null() {
        gsl_error(
            b"failed to allocate space for row indices\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            189 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*m).i = ptr as *mut libc::c_int;
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        ptr = realloc(
            (*m).p as *mut libc::c_void,
            nzmax.wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        if ptr.is_null() {
            gsl_error(
                b"failed to allocate space for column indices\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                199 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*m).p = ptr as *mut libc::c_int;
    }
    ptr_atomic = realloc(
        (*m).data as *mut libc::c_void,
        nzmax
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
    ) as *mut libc::c_uchar;
    if ptr_atomic.is_null() {
        gsl_error(
            b"failed to allocate space for data\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let nnew: size_t = nzmax.wrapping_sub((*m).nz);
        let mut node: *mut gsl_spmatrix_pool = 0 as *mut gsl_spmatrix_pool;
        if ptr_atomic != (*m).data {
            let mut trav: gsl_bst_trav = gsl_bst_trav {
                type_0: 0 as *const gsl_bst_type,
                trav_data: C2RustUnnamed_1 {
                    avl_trav: gsl_bst_avl_traverser {
                        avl_table: 0 as *const gsl_bst_avl_table,
                        avl_node: 0 as *mut gsl_bst_avl_node,
                        avl_stack: [0 as *mut gsl_bst_avl_node; 32],
                        avl_height: 0,
                        avl_generation: 0,
                    },
                },
            };
            ptr = gsl_bst_trav_first(&mut trav, (*m).tree);
            while !ptr.is_null() {
                let mut q: *mut libc::c_uchar = ptr as *mut libc::c_uchar;
                let mut idx: size_t = q.offset_from((*m).data) as libc::c_long as size_t;
                gsl_bst_trav_replace(
                    &mut trav,
                    &mut *ptr_atomic.offset(idx as isize) as *mut libc::c_uchar
                        as *mut libc::c_void,
                );
                ptr = gsl_bst_trav_next(&mut trav);
            }
        }
        node = malloc(::std::mem::size_of::<gsl_spmatrix_pool>() as libc::c_ulong)
            as *mut gsl_spmatrix_pool;
        if node.is_null() {
            gsl_error(
                b"failed to allocate space for memory pool node\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                239 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*node).block_ptr = malloc(nnew.wrapping_mul((*m).node_size));
        if ((*node).block_ptr).is_null() {
            gsl_error(
                b"failed to allocate space for memory pool block\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                245 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*node).free_slot = (*node).block_ptr as *mut libc::c_uchar;
        (*node).next = (*m).pool;
        (*m).pool = node;
    }
    (*m).data = ptr_atomic;
    (*m).nzmax = nzmax;
    return status;
}
pub unsafe extern "C" fn gsl_spmatrix_char_realloc(
    nzmax: size_t,
    mut m: *mut gsl_spmatrix_char,
) -> libc::c_int {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ptr_atomic: *mut libc::c_char = 0 as *mut libc::c_char;
    if nzmax < (*m).nz {
        gsl_error(
            b"new nzmax is less than current nz\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            183 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    ptr = realloc(
        (*m).i as *mut libc::c_void,
        nzmax.wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    if ptr.is_null() {
        gsl_error(
            b"failed to allocate space for row indices\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            189 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*m).i = ptr as *mut libc::c_int;
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        ptr = realloc(
            (*m).p as *mut libc::c_void,
            nzmax.wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        if ptr.is_null() {
            gsl_error(
                b"failed to allocate space for column indices\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                199 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*m).p = ptr as *mut libc::c_int;
    }
    ptr_atomic = realloc(
        (*m).data as *mut libc::c_void,
        nzmax
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    if ptr_atomic.is_null() {
        gsl_error(
            b"failed to allocate space for data\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let nnew: size_t = nzmax.wrapping_sub((*m).nz);
        let mut node: *mut gsl_spmatrix_pool = 0 as *mut gsl_spmatrix_pool;
        if ptr_atomic != (*m).data {
            let mut trav: gsl_bst_trav = gsl_bst_trav {
                type_0: 0 as *const gsl_bst_type,
                trav_data: C2RustUnnamed_1 {
                    avl_trav: gsl_bst_avl_traverser {
                        avl_table: 0 as *const gsl_bst_avl_table,
                        avl_node: 0 as *mut gsl_bst_avl_node,
                        avl_stack: [0 as *mut gsl_bst_avl_node; 32],
                        avl_height: 0,
                        avl_generation: 0,
                    },
                },
            };
            ptr = gsl_bst_trav_first(&mut trav, (*m).tree);
            while !ptr.is_null() {
                let mut q: *mut libc::c_char = ptr as *mut libc::c_char;
                let mut idx: size_t = q.offset_from((*m).data) as libc::c_long as size_t;
                gsl_bst_trav_replace(
                    &mut trav,
                    &mut *ptr_atomic.offset(idx as isize) as *mut libc::c_char
                        as *mut libc::c_void,
                );
                ptr = gsl_bst_trav_next(&mut trav);
            }
        }
        node = malloc(::std::mem::size_of::<gsl_spmatrix_pool>() as libc::c_ulong)
            as *mut gsl_spmatrix_pool;
        if node.is_null() {
            gsl_error(
                b"failed to allocate space for memory pool node\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                239 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*node).block_ptr = malloc(nnew.wrapping_mul((*m).node_size));
        if ((*node).block_ptr).is_null() {
            gsl_error(
                b"failed to allocate space for memory pool block\0" as *const u8
                    as *const libc::c_char,
                b"./init_source.c\0" as *const u8 as *const libc::c_char,
                245 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        (*node).free_slot = (*node).block_ptr as *mut libc::c_uchar;
        (*node).next = (*m).pool;
        (*m).pool = node;
    }
    (*m).data = ptr_atomic;
    (*m).nzmax = nzmax;
    return status;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_long_double_nnz(
    mut m: *const gsl_spmatrix_complex_long_double,
) -> size_t {
    return (*m).nz;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_float_nnz(
    mut m: *const gsl_spmatrix_complex_float,
) -> size_t {
    return (*m).nz;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_nnz(
    mut m: *const gsl_spmatrix_complex,
) -> size_t {
    return (*m).nz;
}
pub unsafe extern "C" fn gsl_spmatrix_uchar_nnz(
    mut m: *const gsl_spmatrix_uchar,
) -> size_t {
    return (*m).nz;
}
pub unsafe extern "C" fn gsl_spmatrix_uint_nnz(
    mut m: *const gsl_spmatrix_uint,
) -> size_t {
    return (*m).nz;
}
pub unsafe extern "C" fn gsl_spmatrix_ulong_nnz(
    mut m: *const gsl_spmatrix_ulong,
) -> size_t {
    return (*m).nz;
}
pub unsafe extern "C" fn gsl_spmatrix_char_nnz(
    mut m: *const gsl_spmatrix_char,
) -> size_t {
    return (*m).nz;
}
pub unsafe extern "C" fn gsl_spmatrix_ushort_nnz(
    mut m: *const gsl_spmatrix_ushort,
) -> size_t {
    return (*m).nz;
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_nnz(
    mut m: *const gsl_spmatrix_long_double,
) -> size_t {
    return (*m).nz;
}
pub unsafe extern "C" fn gsl_spmatrix_nnz(mut m: *const gsl_spmatrix) -> size_t {
    return (*m).nz;
}
pub unsafe extern "C" fn gsl_spmatrix_short_nnz(
    mut m: *const gsl_spmatrix_short,
) -> size_t {
    return (*m).nz;
}
pub unsafe extern "C" fn gsl_spmatrix_int_nnz(mut m: *const gsl_spmatrix_int) -> size_t {
    return (*m).nz;
}
pub unsafe extern "C" fn gsl_spmatrix_float_nnz(
    mut m: *const gsl_spmatrix_float,
) -> size_t {
    return (*m).nz;
}
pub unsafe extern "C" fn gsl_spmatrix_long_nnz(
    mut m: *const gsl_spmatrix_long,
) -> size_t {
    return (*m).nz;
}
pub unsafe extern "C" fn gsl_spmatrix_float_type(
    mut m: *const gsl_spmatrix_float,
) -> *const libc::c_char {
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        return b"COO\0" as *const u8 as *const libc::c_char
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        return b"CSR\0" as *const u8 as *const libc::c_char
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        return b"CSC\0" as *const u8 as *const libc::c_char
    } else {
        return b"unknown\0" as *const u8 as *const libc::c_char
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_float_type(
    mut m: *const gsl_spmatrix_complex_float,
) -> *const libc::c_char {
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        return b"COO\0" as *const u8 as *const libc::c_char
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        return b"CSR\0" as *const u8 as *const libc::c_char
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        return b"CSC\0" as *const u8 as *const libc::c_char
    } else {
        return b"unknown\0" as *const u8 as *const libc::c_char
    };
}
pub unsafe extern "C" fn gsl_spmatrix_long_type(
    mut m: *const gsl_spmatrix_long,
) -> *const libc::c_char {
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        return b"COO\0" as *const u8 as *const libc::c_char
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        return b"CSR\0" as *const u8 as *const libc::c_char
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        return b"CSC\0" as *const u8 as *const libc::c_char
    } else {
        return b"unknown\0" as *const u8 as *const libc::c_char
    };
}
pub unsafe extern "C" fn gsl_spmatrix_type(
    mut m: *const gsl_spmatrix,
) -> *const libc::c_char {
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        return b"COO\0" as *const u8 as *const libc::c_char
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        return b"CSR\0" as *const u8 as *const libc::c_char
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        return b"CSC\0" as *const u8 as *const libc::c_char
    } else {
        return b"unknown\0" as *const u8 as *const libc::c_char
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_type(
    mut m: *const gsl_spmatrix_complex,
) -> *const libc::c_char {
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        return b"COO\0" as *const u8 as *const libc::c_char
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        return b"CSR\0" as *const u8 as *const libc::c_char
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        return b"CSC\0" as *const u8 as *const libc::c_char
    } else {
        return b"unknown\0" as *const u8 as *const libc::c_char
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_long_double_type(
    mut m: *const gsl_spmatrix_complex_long_double,
) -> *const libc::c_char {
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        return b"COO\0" as *const u8 as *const libc::c_char
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        return b"CSR\0" as *const u8 as *const libc::c_char
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        return b"CSC\0" as *const u8 as *const libc::c_char
    } else {
        return b"unknown\0" as *const u8 as *const libc::c_char
    };
}
pub unsafe extern "C" fn gsl_spmatrix_ushort_type(
    mut m: *const gsl_spmatrix_ushort,
) -> *const libc::c_char {
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        return b"COO\0" as *const u8 as *const libc::c_char
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        return b"CSR\0" as *const u8 as *const libc::c_char
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        return b"CSC\0" as *const u8 as *const libc::c_char
    } else {
        return b"unknown\0" as *const u8 as *const libc::c_char
    };
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_type(
    mut m: *const gsl_spmatrix_long_double,
) -> *const libc::c_char {
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        return b"COO\0" as *const u8 as *const libc::c_char
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        return b"CSR\0" as *const u8 as *const libc::c_char
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        return b"CSC\0" as *const u8 as *const libc::c_char
    } else {
        return b"unknown\0" as *const u8 as *const libc::c_char
    };
}
pub unsafe extern "C" fn gsl_spmatrix_short_type(
    mut m: *const gsl_spmatrix_short,
) -> *const libc::c_char {
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        return b"COO\0" as *const u8 as *const libc::c_char
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        return b"CSR\0" as *const u8 as *const libc::c_char
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        return b"CSC\0" as *const u8 as *const libc::c_char
    } else {
        return b"unknown\0" as *const u8 as *const libc::c_char
    };
}
pub unsafe extern "C" fn gsl_spmatrix_int_type(
    mut m: *const gsl_spmatrix_int,
) -> *const libc::c_char {
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        return b"COO\0" as *const u8 as *const libc::c_char
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        return b"CSR\0" as *const u8 as *const libc::c_char
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        return b"CSC\0" as *const u8 as *const libc::c_char
    } else {
        return b"unknown\0" as *const u8 as *const libc::c_char
    };
}
pub unsafe extern "C" fn gsl_spmatrix_ulong_type(
    mut m: *const gsl_spmatrix_ulong,
) -> *const libc::c_char {
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        return b"COO\0" as *const u8 as *const libc::c_char
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        return b"CSR\0" as *const u8 as *const libc::c_char
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        return b"CSC\0" as *const u8 as *const libc::c_char
    } else {
        return b"unknown\0" as *const u8 as *const libc::c_char
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uchar_type(
    mut m: *const gsl_spmatrix_uchar,
) -> *const libc::c_char {
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        return b"COO\0" as *const u8 as *const libc::c_char
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        return b"CSR\0" as *const u8 as *const libc::c_char
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        return b"CSC\0" as *const u8 as *const libc::c_char
    } else {
        return b"unknown\0" as *const u8 as *const libc::c_char
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uint_type(
    mut m: *const gsl_spmatrix_uint,
) -> *const libc::c_char {
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        return b"COO\0" as *const u8 as *const libc::c_char
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        return b"CSR\0" as *const u8 as *const libc::c_char
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        return b"CSC\0" as *const u8 as *const libc::c_char
    } else {
        return b"unknown\0" as *const u8 as *const libc::c_char
    };
}
pub unsafe extern "C" fn gsl_spmatrix_char_type(
    mut m: *const gsl_spmatrix_char,
) -> *const libc::c_char {
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        return b"COO\0" as *const u8 as *const libc::c_char
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        return b"CSR\0" as *const u8 as *const libc::c_char
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        return b"CSC\0" as *const u8 as *const libc::c_char
    } else {
        return b"unknown\0" as *const u8 as *const libc::c_char
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uchar_set_zero(
    mut m: *mut gsl_spmatrix_uchar,
) -> libc::c_int {
    (*m).nz = 0 as libc::c_int as size_t;
    if !((*m).tree).is_null() {
        gsl_bst_empty((*m).tree);
        spmatrix_uchar_pool_free(m);
        spmatrix_uchar_pool_init(m);
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_set_zero(mut m: *mut gsl_spmatrix) -> libc::c_int {
    (*m).nz = 0 as libc::c_int as size_t;
    if !((*m).tree).is_null() {
        gsl_bst_empty((*m).tree);
        spmatrix_pool_free(m);
        spmatrix_pool_init(m);
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_ulong_set_zero(
    mut m: *mut gsl_spmatrix_ulong,
) -> libc::c_int {
    (*m).nz = 0 as libc::c_int as size_t;
    if !((*m).tree).is_null() {
        gsl_bst_empty((*m).tree);
        spmatrix_ulong_pool_free(m);
        spmatrix_ulong_pool_init(m);
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_short_set_zero(
    mut m: *mut gsl_spmatrix_short,
) -> libc::c_int {
    (*m).nz = 0 as libc::c_int as size_t;
    if !((*m).tree).is_null() {
        gsl_bst_empty((*m).tree);
        spmatrix_short_pool_free(m);
        spmatrix_short_pool_init(m);
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_float_set_zero(
    mut m: *mut gsl_spmatrix_float,
) -> libc::c_int {
    (*m).nz = 0 as libc::c_int as size_t;
    if !((*m).tree).is_null() {
        gsl_bst_empty((*m).tree);
        spmatrix_float_pool_free(m);
        spmatrix_float_pool_init(m);
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_long_double_set_zero(
    mut m: *mut gsl_spmatrix_complex_long_double,
) -> libc::c_int {
    (*m).nz = 0 as libc::c_int as size_t;
    if !((*m).tree).is_null() {
        gsl_bst_empty((*m).tree);
        spmatrix_complex_long_double_pool_free(m);
        spmatrix_complex_long_double_pool_init(m);
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_set_zero(
    mut m: *mut gsl_spmatrix_long_double,
) -> libc::c_int {
    (*m).nz = 0 as libc::c_int as size_t;
    if !((*m).tree).is_null() {
        gsl_bst_empty((*m).tree);
        spmatrix_long_double_pool_free(m);
        spmatrix_long_double_pool_init(m);
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_long_set_zero(
    mut m: *mut gsl_spmatrix_long,
) -> libc::c_int {
    (*m).nz = 0 as libc::c_int as size_t;
    if !((*m).tree).is_null() {
        gsl_bst_empty((*m).tree);
        spmatrix_long_pool_free(m);
        spmatrix_long_pool_init(m);
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_uint_set_zero(
    mut m: *mut gsl_spmatrix_uint,
) -> libc::c_int {
    (*m).nz = 0 as libc::c_int as size_t;
    if !((*m).tree).is_null() {
        gsl_bst_empty((*m).tree);
        spmatrix_uint_pool_free(m);
        spmatrix_uint_pool_init(m);
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_ushort_set_zero(
    mut m: *mut gsl_spmatrix_ushort,
) -> libc::c_int {
    (*m).nz = 0 as libc::c_int as size_t;
    if !((*m).tree).is_null() {
        gsl_bst_empty((*m).tree);
        spmatrix_ushort_pool_free(m);
        spmatrix_ushort_pool_init(m);
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_char_set_zero(
    mut m: *mut gsl_spmatrix_char,
) -> libc::c_int {
    (*m).nz = 0 as libc::c_int as size_t;
    if !((*m).tree).is_null() {
        gsl_bst_empty((*m).tree);
        spmatrix_char_pool_free(m);
        spmatrix_char_pool_init(m);
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_set_zero(
    mut m: *mut gsl_spmatrix_complex,
) -> libc::c_int {
    (*m).nz = 0 as libc::c_int as size_t;
    if !((*m).tree).is_null() {
        gsl_bst_empty((*m).tree);
        spmatrix_complex_pool_free(m);
        spmatrix_complex_pool_init(m);
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_float_set_zero(
    mut m: *mut gsl_spmatrix_complex_float,
) -> libc::c_int {
    (*m).nz = 0 as libc::c_int as size_t;
    if !((*m).tree).is_null() {
        gsl_bst_empty((*m).tree);
        spmatrix_complex_float_pool_free(m);
        spmatrix_complex_float_pool_init(m);
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_int_set_zero(
    mut m: *mut gsl_spmatrix_int,
) -> libc::c_int {
    (*m).nz = 0 as libc::c_int as size_t;
    if !((*m).tree).is_null() {
        gsl_bst_empty((*m).tree);
        spmatrix_int_pool_free(m);
        spmatrix_int_pool_init(m);
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_long_double_tree_rebuild(
    mut m: *mut gsl_spmatrix_complex_long_double,
) -> libc::c_int {
    if !((*m).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            312 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut n: size_t = 0;
        gsl_bst_empty((*m).tree);
        spmatrix_complex_long_double_pool_free(m);
        spmatrix_complex_long_double_pool_init(m);
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            let mut ptr: *mut libc::c_void = gsl_bst_insert(
                &mut *((*m).data)
                    .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
                    as *mut f128::f128 as *mut libc::c_void,
                (*m).tree,
            );
            if !ptr.is_null() {
                gsl_error(
                    b"detected duplicate entry\0" as *const u8 as *const libc::c_char,
                    b"./init_source.c\0" as *const u8 as *const libc::c_char,
                    331 as libc::c_int,
                    GSL_EINVAL as libc::c_int,
                );
                return GSL_EINVAL as libc::c_int;
            }
            n = n.wrapping_add(1);
            n;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_int_tree_rebuild(
    mut m: *mut gsl_spmatrix_int,
) -> libc::c_int {
    if !((*m).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            312 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut n: size_t = 0;
        gsl_bst_empty((*m).tree);
        spmatrix_int_pool_free(m);
        spmatrix_int_pool_init(m);
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            let mut ptr: *mut libc::c_void = gsl_bst_insert(
                &mut *((*m).data)
                    .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
                    as *mut libc::c_int as *mut libc::c_void,
                (*m).tree,
            );
            if !ptr.is_null() {
                gsl_error(
                    b"detected duplicate entry\0" as *const u8 as *const libc::c_char,
                    b"./init_source.c\0" as *const u8 as *const libc::c_char,
                    331 as libc::c_int,
                    GSL_EINVAL as libc::c_int,
                );
                return GSL_EINVAL as libc::c_int;
            }
            n = n.wrapping_add(1);
            n;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_tree_rebuild(
    mut m: *mut gsl_spmatrix_complex,
) -> libc::c_int {
    if !((*m).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            312 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut n: size_t = 0;
        gsl_bst_empty((*m).tree);
        spmatrix_complex_pool_free(m);
        spmatrix_complex_pool_init(m);
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            let mut ptr: *mut libc::c_void = gsl_bst_insert(
                &mut *((*m).data)
                    .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
                    as *mut libc::c_double as *mut libc::c_void,
                (*m).tree,
            );
            if !ptr.is_null() {
                gsl_error(
                    b"detected duplicate entry\0" as *const u8 as *const libc::c_char,
                    b"./init_source.c\0" as *const u8 as *const libc::c_char,
                    331 as libc::c_int,
                    GSL_EINVAL as libc::c_int,
                );
                return GSL_EINVAL as libc::c_int;
            }
            n = n.wrapping_add(1);
            n;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_tree_rebuild(
    mut m: *mut gsl_spmatrix_long_double,
) -> libc::c_int {
    if !((*m).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            312 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut n: size_t = 0;
        gsl_bst_empty((*m).tree);
        spmatrix_long_double_pool_free(m);
        spmatrix_long_double_pool_init(m);
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            let mut ptr: *mut libc::c_void = gsl_bst_insert(
                &mut *((*m).data)
                    .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
                    as *mut f128::f128 as *mut libc::c_void,
                (*m).tree,
            );
            if !ptr.is_null() {
                gsl_error(
                    b"detected duplicate entry\0" as *const u8 as *const libc::c_char,
                    b"./init_source.c\0" as *const u8 as *const libc::c_char,
                    331 as libc::c_int,
                    GSL_EINVAL as libc::c_int,
                );
                return GSL_EINVAL as libc::c_int;
            }
            n = n.wrapping_add(1);
            n;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_float_tree_rebuild(
    mut m: *mut gsl_spmatrix_float,
) -> libc::c_int {
    if !((*m).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            312 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut n: size_t = 0;
        gsl_bst_empty((*m).tree);
        spmatrix_float_pool_free(m);
        spmatrix_float_pool_init(m);
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            let mut ptr: *mut libc::c_void = gsl_bst_insert(
                &mut *((*m).data)
                    .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
                    as *mut libc::c_float as *mut libc::c_void,
                (*m).tree,
            );
            if !ptr.is_null() {
                gsl_error(
                    b"detected duplicate entry\0" as *const u8 as *const libc::c_char,
                    b"./init_source.c\0" as *const u8 as *const libc::c_char,
                    331 as libc::c_int,
                    GSL_EINVAL as libc::c_int,
                );
                return GSL_EINVAL as libc::c_int;
            }
            n = n.wrapping_add(1);
            n;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_tree_rebuild(
    mut m: *mut gsl_spmatrix,
) -> libc::c_int {
    if !((*m).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            312 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut n: size_t = 0;
        gsl_bst_empty((*m).tree);
        spmatrix_pool_free(m);
        spmatrix_pool_init(m);
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            let mut ptr: *mut libc::c_void = gsl_bst_insert(
                &mut *((*m).data)
                    .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
                    as *mut libc::c_double as *mut libc::c_void,
                (*m).tree,
            );
            if !ptr.is_null() {
                gsl_error(
                    b"detected duplicate entry\0" as *const u8 as *const libc::c_char,
                    b"./init_source.c\0" as *const u8 as *const libc::c_char,
                    331 as libc::c_int,
                    GSL_EINVAL as libc::c_int,
                );
                return GSL_EINVAL as libc::c_int;
            }
            n = n.wrapping_add(1);
            n;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uint_tree_rebuild(
    mut m: *mut gsl_spmatrix_uint,
) -> libc::c_int {
    if !((*m).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            312 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut n: size_t = 0;
        gsl_bst_empty((*m).tree);
        spmatrix_uint_pool_free(m);
        spmatrix_uint_pool_init(m);
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            let mut ptr: *mut libc::c_void = gsl_bst_insert(
                &mut *((*m).data)
                    .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
                    as *mut libc::c_uint as *mut libc::c_void,
                (*m).tree,
            );
            if !ptr.is_null() {
                gsl_error(
                    b"detected duplicate entry\0" as *const u8 as *const libc::c_char,
                    b"./init_source.c\0" as *const u8 as *const libc::c_char,
                    331 as libc::c_int,
                    GSL_EINVAL as libc::c_int,
                );
                return GSL_EINVAL as libc::c_int;
            }
            n = n.wrapping_add(1);
            n;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_ulong_tree_rebuild(
    mut m: *mut gsl_spmatrix_ulong,
) -> libc::c_int {
    if !((*m).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            312 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut n: size_t = 0;
        gsl_bst_empty((*m).tree);
        spmatrix_ulong_pool_free(m);
        spmatrix_ulong_pool_init(m);
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            let mut ptr: *mut libc::c_void = gsl_bst_insert(
                &mut *((*m).data)
                    .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
                    as *mut libc::c_ulong as *mut libc::c_void,
                (*m).tree,
            );
            if !ptr.is_null() {
                gsl_error(
                    b"detected duplicate entry\0" as *const u8 as *const libc::c_char,
                    b"./init_source.c\0" as *const u8 as *const libc::c_char,
                    331 as libc::c_int,
                    GSL_EINVAL as libc::c_int,
                );
                return GSL_EINVAL as libc::c_int;
            }
            n = n.wrapping_add(1);
            n;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_char_tree_rebuild(
    mut m: *mut gsl_spmatrix_char,
) -> libc::c_int {
    if !((*m).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            312 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut n: size_t = 0;
        gsl_bst_empty((*m).tree);
        spmatrix_char_pool_free(m);
        spmatrix_char_pool_init(m);
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            let mut ptr: *mut libc::c_void = gsl_bst_insert(
                &mut *((*m).data)
                    .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
                    as *mut libc::c_char as *mut libc::c_void,
                (*m).tree,
            );
            if !ptr.is_null() {
                gsl_error(
                    b"detected duplicate entry\0" as *const u8 as *const libc::c_char,
                    b"./init_source.c\0" as *const u8 as *const libc::c_char,
                    331 as libc::c_int,
                    GSL_EINVAL as libc::c_int,
                );
                return GSL_EINVAL as libc::c_int;
            }
            n = n.wrapping_add(1);
            n;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_ushort_tree_rebuild(
    mut m: *mut gsl_spmatrix_ushort,
) -> libc::c_int {
    if !((*m).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            312 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut n: size_t = 0;
        gsl_bst_empty((*m).tree);
        spmatrix_ushort_pool_free(m);
        spmatrix_ushort_pool_init(m);
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            let mut ptr: *mut libc::c_void = gsl_bst_insert(
                &mut *((*m).data)
                    .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
                    as *mut libc::c_ushort as *mut libc::c_void,
                (*m).tree,
            );
            if !ptr.is_null() {
                gsl_error(
                    b"detected duplicate entry\0" as *const u8 as *const libc::c_char,
                    b"./init_source.c\0" as *const u8 as *const libc::c_char,
                    331 as libc::c_int,
                    GSL_EINVAL as libc::c_int,
                );
                return GSL_EINVAL as libc::c_int;
            }
            n = n.wrapping_add(1);
            n;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uchar_tree_rebuild(
    mut m: *mut gsl_spmatrix_uchar,
) -> libc::c_int {
    if !((*m).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            312 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut n: size_t = 0;
        gsl_bst_empty((*m).tree);
        spmatrix_uchar_pool_free(m);
        spmatrix_uchar_pool_init(m);
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            let mut ptr: *mut libc::c_void = gsl_bst_insert(
                &mut *((*m).data)
                    .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
                    as *mut libc::c_uchar as *mut libc::c_void,
                (*m).tree,
            );
            if !ptr.is_null() {
                gsl_error(
                    b"detected duplicate entry\0" as *const u8 as *const libc::c_char,
                    b"./init_source.c\0" as *const u8 as *const libc::c_char,
                    331 as libc::c_int,
                    GSL_EINVAL as libc::c_int,
                );
                return GSL_EINVAL as libc::c_int;
            }
            n = n.wrapping_add(1);
            n;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_short_tree_rebuild(
    mut m: *mut gsl_spmatrix_short,
) -> libc::c_int {
    if !((*m).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            312 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut n: size_t = 0;
        gsl_bst_empty((*m).tree);
        spmatrix_short_pool_free(m);
        spmatrix_short_pool_init(m);
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            let mut ptr: *mut libc::c_void = gsl_bst_insert(
                &mut *((*m).data)
                    .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
                    as *mut libc::c_short as *mut libc::c_void,
                (*m).tree,
            );
            if !ptr.is_null() {
                gsl_error(
                    b"detected duplicate entry\0" as *const u8 as *const libc::c_char,
                    b"./init_source.c\0" as *const u8 as *const libc::c_char,
                    331 as libc::c_int,
                    GSL_EINVAL as libc::c_int,
                );
                return GSL_EINVAL as libc::c_int;
            }
            n = n.wrapping_add(1);
            n;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_float_tree_rebuild(
    mut m: *mut gsl_spmatrix_complex_float,
) -> libc::c_int {
    if !((*m).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            312 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut n: size_t = 0;
        gsl_bst_empty((*m).tree);
        spmatrix_complex_float_pool_free(m);
        spmatrix_complex_float_pool_init(m);
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            let mut ptr: *mut libc::c_void = gsl_bst_insert(
                &mut *((*m).data)
                    .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
                    as *mut libc::c_float as *mut libc::c_void,
                (*m).tree,
            );
            if !ptr.is_null() {
                gsl_error(
                    b"detected duplicate entry\0" as *const u8 as *const libc::c_char,
                    b"./init_source.c\0" as *const u8 as *const libc::c_char,
                    331 as libc::c_int,
                    GSL_EINVAL as libc::c_int,
                );
                return GSL_EINVAL as libc::c_int;
            }
            n = n.wrapping_add(1);
            n;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_long_tree_rebuild(
    mut m: *mut gsl_spmatrix_long,
) -> libc::c_int {
    if !((*m).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            312 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut n: size_t = 0;
        gsl_bst_empty((*m).tree);
        spmatrix_long_pool_free(m);
        spmatrix_long_pool_init(m);
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            let mut ptr: *mut libc::c_void = gsl_bst_insert(
                &mut *((*m).data)
                    .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
                    as *mut libc::c_long as *mut libc::c_void,
                (*m).tree,
            );
            if !ptr.is_null() {
                gsl_error(
                    b"detected duplicate entry\0" as *const u8 as *const libc::c_char,
                    b"./init_source.c\0" as *const u8 as *const libc::c_char,
                    331 as libc::c_int,
                    GSL_EINVAL as libc::c_int,
                );
                return GSL_EINVAL as libc::c_int;
            }
            n = n.wrapping_add(1);
            n;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn compare_uchar_func(
    mut pa: *const libc::c_void,
    mut pb: *const libc::c_void,
    mut param: *mut libc::c_void,
) -> libc::c_int {
    let mut m: *mut gsl_spmatrix_uchar = param as *mut gsl_spmatrix_uchar;
    let idxa: size_t = (pa as *const libc::c_uchar).offset_from((*m).data)
        as libc::c_long as size_t;
    let idxb: size_t = (pb as *const libc::c_uchar).offset_from((*m).data)
        as libc::c_long as size_t;
    return if *((*m).i).offset(idxa as isize) < *((*m).i).offset(idxb as isize) {
        -(1 as libc::c_int)
    } else if *((*m).i).offset(idxa as isize) > *((*m).i).offset(idxb as isize) {
        1 as libc::c_int
    } else if *((*m).p).offset(idxa as isize) < *((*m).p).offset(idxb as isize) {
        -(1 as libc::c_int)
    } else {
        (*((*m).p).offset(idxa as isize) > *((*m).p).offset(idxb as isize))
            as libc::c_int
    };
}
unsafe extern "C" fn compare_func(
    mut pa: *const libc::c_void,
    mut pb: *const libc::c_void,
    mut param: *mut libc::c_void,
) -> libc::c_int {
    let mut m: *mut gsl_spmatrix = param as *mut gsl_spmatrix;
    let idxa: size_t = (pa as *const libc::c_double).offset_from((*m).data)
        as libc::c_long as size_t;
    let idxb: size_t = (pb as *const libc::c_double).offset_from((*m).data)
        as libc::c_long as size_t;
    return if *((*m).i).offset(idxa as isize) < *((*m).i).offset(idxb as isize) {
        -(1 as libc::c_int)
    } else if *((*m).i).offset(idxa as isize) > *((*m).i).offset(idxb as isize) {
        1 as libc::c_int
    } else if *((*m).p).offset(idxa as isize) < *((*m).p).offset(idxb as isize) {
        -(1 as libc::c_int)
    } else {
        (*((*m).p).offset(idxa as isize) > *((*m).p).offset(idxb as isize))
            as libc::c_int
    };
}
unsafe extern "C" fn compare_ulong_func(
    mut pa: *const libc::c_void,
    mut pb: *const libc::c_void,
    mut param: *mut libc::c_void,
) -> libc::c_int {
    let mut m: *mut gsl_spmatrix_ulong = param as *mut gsl_spmatrix_ulong;
    let idxa: size_t = (pa as *const libc::c_ulong).offset_from((*m).data)
        as libc::c_long as size_t;
    let idxb: size_t = (pb as *const libc::c_ulong).offset_from((*m).data)
        as libc::c_long as size_t;
    return if *((*m).i).offset(idxa as isize) < *((*m).i).offset(idxb as isize) {
        -(1 as libc::c_int)
    } else if *((*m).i).offset(idxa as isize) > *((*m).i).offset(idxb as isize) {
        1 as libc::c_int
    } else if *((*m).p).offset(idxa as isize) < *((*m).p).offset(idxb as isize) {
        -(1 as libc::c_int)
    } else {
        (*((*m).p).offset(idxa as isize) > *((*m).p).offset(idxb as isize))
            as libc::c_int
    };
}
unsafe extern "C" fn compare_int_func(
    mut pa: *const libc::c_void,
    mut pb: *const libc::c_void,
    mut param: *mut libc::c_void,
) -> libc::c_int {
    let mut m: *mut gsl_spmatrix_int = param as *mut gsl_spmatrix_int;
    let idxa: size_t = (pa as *const libc::c_int).offset_from((*m).data) as libc::c_long
        as size_t;
    let idxb: size_t = (pb as *const libc::c_int).offset_from((*m).data) as libc::c_long
        as size_t;
    return if *((*m).i).offset(idxa as isize) < *((*m).i).offset(idxb as isize) {
        -(1 as libc::c_int)
    } else if *((*m).i).offset(idxa as isize) > *((*m).i).offset(idxb as isize) {
        1 as libc::c_int
    } else if *((*m).p).offset(idxa as isize) < *((*m).p).offset(idxb as isize) {
        -(1 as libc::c_int)
    } else {
        (*((*m).p).offset(idxa as isize) > *((*m).p).offset(idxb as isize))
            as libc::c_int
    };
}
unsafe extern "C" fn compare_float_func(
    mut pa: *const libc::c_void,
    mut pb: *const libc::c_void,
    mut param: *mut libc::c_void,
) -> libc::c_int {
    let mut m: *mut gsl_spmatrix_float = param as *mut gsl_spmatrix_float;
    let idxa: size_t = (pa as *const libc::c_float).offset_from((*m).data)
        as libc::c_long as size_t;
    let idxb: size_t = (pb as *const libc::c_float).offset_from((*m).data)
        as libc::c_long as size_t;
    return if *((*m).i).offset(idxa as isize) < *((*m).i).offset(idxb as isize) {
        -(1 as libc::c_int)
    } else if *((*m).i).offset(idxa as isize) > *((*m).i).offset(idxb as isize) {
        1 as libc::c_int
    } else if *((*m).p).offset(idxa as isize) < *((*m).p).offset(idxb as isize) {
        -(1 as libc::c_int)
    } else {
        (*((*m).p).offset(idxa as isize) > *((*m).p).offset(idxb as isize))
            as libc::c_int
    };
}
unsafe extern "C" fn compare_ushort_func(
    mut pa: *const libc::c_void,
    mut pb: *const libc::c_void,
    mut param: *mut libc::c_void,
) -> libc::c_int {
    let mut m: *mut gsl_spmatrix_ushort = param as *mut gsl_spmatrix_ushort;
    let idxa: size_t = (pa as *const libc::c_ushort).offset_from((*m).data)
        as libc::c_long as size_t;
    let idxb: size_t = (pb as *const libc::c_ushort).offset_from((*m).data)
        as libc::c_long as size_t;
    return if *((*m).i).offset(idxa as isize) < *((*m).i).offset(idxb as isize) {
        -(1 as libc::c_int)
    } else if *((*m).i).offset(idxa as isize) > *((*m).i).offset(idxb as isize) {
        1 as libc::c_int
    } else if *((*m).p).offset(idxa as isize) < *((*m).p).offset(idxb as isize) {
        -(1 as libc::c_int)
    } else {
        (*((*m).p).offset(idxa as isize) > *((*m).p).offset(idxb as isize))
            as libc::c_int
    };
}
unsafe extern "C" fn compare_long_double_func(
    mut pa: *const libc::c_void,
    mut pb: *const libc::c_void,
    mut param: *mut libc::c_void,
) -> libc::c_int {
    let mut m: *mut gsl_spmatrix_long_double = param as *mut gsl_spmatrix_long_double;
    let idxa: size_t = (pa as *const f128::f128).offset_from((*m).data) as libc::c_long
        as size_t;
    let idxb: size_t = (pb as *const f128::f128).offset_from((*m).data) as libc::c_long
        as size_t;
    return if *((*m).i).offset(idxa as isize) < *((*m).i).offset(idxb as isize) {
        -(1 as libc::c_int)
    } else if *((*m).i).offset(idxa as isize) > *((*m).i).offset(idxb as isize) {
        1 as libc::c_int
    } else if *((*m).p).offset(idxa as isize) < *((*m).p).offset(idxb as isize) {
        -(1 as libc::c_int)
    } else {
        (*((*m).p).offset(idxa as isize) > *((*m).p).offset(idxb as isize))
            as libc::c_int
    };
}
unsafe extern "C" fn compare_complex_func(
    mut pa: *const libc::c_void,
    mut pb: *const libc::c_void,
    mut param: *mut libc::c_void,
) -> libc::c_int {
    let mut m: *mut gsl_spmatrix_complex = param as *mut gsl_spmatrix_complex;
    let idxa: size_t = ((pa as *const libc::c_double).offset_from((*m).data)
        as libc::c_long >> 1 as libc::c_int) as size_t;
    let idxb: size_t = ((pb as *const libc::c_double).offset_from((*m).data)
        as libc::c_long >> 1 as libc::c_int) as size_t;
    return if *((*m).i).offset(idxa as isize) < *((*m).i).offset(idxb as isize) {
        -(1 as libc::c_int)
    } else if *((*m).i).offset(idxa as isize) > *((*m).i).offset(idxb as isize) {
        1 as libc::c_int
    } else if *((*m).p).offset(idxa as isize) < *((*m).p).offset(idxb as isize) {
        -(1 as libc::c_int)
    } else {
        (*((*m).p).offset(idxa as isize) > *((*m).p).offset(idxb as isize))
            as libc::c_int
    };
}
unsafe extern "C" fn compare_char_func(
    mut pa: *const libc::c_void,
    mut pb: *const libc::c_void,
    mut param: *mut libc::c_void,
) -> libc::c_int {
    let mut m: *mut gsl_spmatrix_char = param as *mut gsl_spmatrix_char;
    let idxa: size_t = (pa as *const libc::c_char).offset_from((*m).data) as libc::c_long
        as size_t;
    let idxb: size_t = (pb as *const libc::c_char).offset_from((*m).data) as libc::c_long
        as size_t;
    return if *((*m).i).offset(idxa as isize) < *((*m).i).offset(idxb as isize) {
        -(1 as libc::c_int)
    } else if *((*m).i).offset(idxa as isize) > *((*m).i).offset(idxb as isize) {
        1 as libc::c_int
    } else if *((*m).p).offset(idxa as isize) < *((*m).p).offset(idxb as isize) {
        -(1 as libc::c_int)
    } else {
        (*((*m).p).offset(idxa as isize) > *((*m).p).offset(idxb as isize))
            as libc::c_int
    };
}
unsafe extern "C" fn compare_complex_float_func(
    mut pa: *const libc::c_void,
    mut pb: *const libc::c_void,
    mut param: *mut libc::c_void,
) -> libc::c_int {
    let mut m: *mut gsl_spmatrix_complex_float = param
        as *mut gsl_spmatrix_complex_float;
    let idxa: size_t = ((pa as *const libc::c_float).offset_from((*m).data)
        as libc::c_long >> 1 as libc::c_int) as size_t;
    let idxb: size_t = ((pb as *const libc::c_float).offset_from((*m).data)
        as libc::c_long >> 1 as libc::c_int) as size_t;
    return if *((*m).i).offset(idxa as isize) < *((*m).i).offset(idxb as isize) {
        -(1 as libc::c_int)
    } else if *((*m).i).offset(idxa as isize) > *((*m).i).offset(idxb as isize) {
        1 as libc::c_int
    } else if *((*m).p).offset(idxa as isize) < *((*m).p).offset(idxb as isize) {
        -(1 as libc::c_int)
    } else {
        (*((*m).p).offset(idxa as isize) > *((*m).p).offset(idxb as isize))
            as libc::c_int
    };
}
unsafe extern "C" fn compare_uint_func(
    mut pa: *const libc::c_void,
    mut pb: *const libc::c_void,
    mut param: *mut libc::c_void,
) -> libc::c_int {
    let mut m: *mut gsl_spmatrix_uint = param as *mut gsl_spmatrix_uint;
    let idxa: size_t = (pa as *const libc::c_uint).offset_from((*m).data) as libc::c_long
        as size_t;
    let idxb: size_t = (pb as *const libc::c_uint).offset_from((*m).data) as libc::c_long
        as size_t;
    return if *((*m).i).offset(idxa as isize) < *((*m).i).offset(idxb as isize) {
        -(1 as libc::c_int)
    } else if *((*m).i).offset(idxa as isize) > *((*m).i).offset(idxb as isize) {
        1 as libc::c_int
    } else if *((*m).p).offset(idxa as isize) < *((*m).p).offset(idxb as isize) {
        -(1 as libc::c_int)
    } else {
        (*((*m).p).offset(idxa as isize) > *((*m).p).offset(idxb as isize))
            as libc::c_int
    };
}
unsafe extern "C" fn compare_short_func(
    mut pa: *const libc::c_void,
    mut pb: *const libc::c_void,
    mut param: *mut libc::c_void,
) -> libc::c_int {
    let mut m: *mut gsl_spmatrix_short = param as *mut gsl_spmatrix_short;
    let idxa: size_t = (pa as *const libc::c_short).offset_from((*m).data)
        as libc::c_long as size_t;
    let idxb: size_t = (pb as *const libc::c_short).offset_from((*m).data)
        as libc::c_long as size_t;
    return if *((*m).i).offset(idxa as isize) < *((*m).i).offset(idxb as isize) {
        -(1 as libc::c_int)
    } else if *((*m).i).offset(idxa as isize) > *((*m).i).offset(idxb as isize) {
        1 as libc::c_int
    } else if *((*m).p).offset(idxa as isize) < *((*m).p).offset(idxb as isize) {
        -(1 as libc::c_int)
    } else {
        (*((*m).p).offset(idxa as isize) > *((*m).p).offset(idxb as isize))
            as libc::c_int
    };
}
unsafe extern "C" fn compare_long_func(
    mut pa: *const libc::c_void,
    mut pb: *const libc::c_void,
    mut param: *mut libc::c_void,
) -> libc::c_int {
    let mut m: *mut gsl_spmatrix_long = param as *mut gsl_spmatrix_long;
    let idxa: size_t = (pa as *const libc::c_long).offset_from((*m).data) as libc::c_long
        as size_t;
    let idxb: size_t = (pb as *const libc::c_long).offset_from((*m).data) as libc::c_long
        as size_t;
    return if *((*m).i).offset(idxa as isize) < *((*m).i).offset(idxb as isize) {
        -(1 as libc::c_int)
    } else if *((*m).i).offset(idxa as isize) > *((*m).i).offset(idxb as isize) {
        1 as libc::c_int
    } else if *((*m).p).offset(idxa as isize) < *((*m).p).offset(idxb as isize) {
        -(1 as libc::c_int)
    } else {
        (*((*m).p).offset(idxa as isize) > *((*m).p).offset(idxb as isize))
            as libc::c_int
    };
}
unsafe extern "C" fn compare_complex_long_double_func(
    mut pa: *const libc::c_void,
    mut pb: *const libc::c_void,
    mut param: *mut libc::c_void,
) -> libc::c_int {
    let mut m: *mut gsl_spmatrix_complex_long_double = param
        as *mut gsl_spmatrix_complex_long_double;
    let idxa: size_t = ((pa as *const f128::f128).offset_from((*m).data) as libc::c_long
        >> 1 as libc::c_int) as size_t;
    let idxb: size_t = ((pb as *const f128::f128).offset_from((*m).data) as libc::c_long
        >> 1 as libc::c_int) as size_t;
    return if *((*m).i).offset(idxa as isize) < *((*m).i).offset(idxb as isize) {
        -(1 as libc::c_int)
    } else if *((*m).i).offset(idxa as isize) > *((*m).i).offset(idxb as isize) {
        1 as libc::c_int
    } else if *((*m).p).offset(idxa as isize) < *((*m).p).offset(idxb as isize) {
        -(1 as libc::c_int)
    } else {
        (*((*m).p).offset(idxa as isize) > *((*m).p).offset(idxb as isize))
            as libc::c_int
    };
}
unsafe extern "C" fn spmatrix_pool_init(mut m: *mut gsl_spmatrix) -> libc::c_int {
    (*m)
        .pool = malloc(::std::mem::size_of::<gsl_spmatrix_pool>() as libc::c_ulong)
        as *mut gsl_spmatrix_pool;
    if ((*m).pool).is_null() {
        gsl_error(
            b"failed to allocate space for memory pool\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            387 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*(*m).pool).block_ptr = malloc(((*m).nzmax).wrapping_mul((*m).node_size));
    if ((*(*m).pool).block_ptr).is_null() {
        gsl_error(
            b"failed to allocate space for memory block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            393 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*(*m).pool).next = 0 as *mut gsl_spmatrix_pool_node;
    (*(*m).pool).free_slot = (*(*m).pool).block_ptr as *mut libc::c_uchar;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn spmatrix_complex_pool_init(
    mut m: *mut gsl_spmatrix_complex,
) -> libc::c_int {
    (*m)
        .pool = malloc(::std::mem::size_of::<gsl_spmatrix_pool>() as libc::c_ulong)
        as *mut gsl_spmatrix_pool;
    if ((*m).pool).is_null() {
        gsl_error(
            b"failed to allocate space for memory pool\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            387 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*(*m).pool).block_ptr = malloc(((*m).nzmax).wrapping_mul((*m).node_size));
    if ((*(*m).pool).block_ptr).is_null() {
        gsl_error(
            b"failed to allocate space for memory block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            393 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*(*m).pool).next = 0 as *mut gsl_spmatrix_pool_node;
    (*(*m).pool).free_slot = (*(*m).pool).block_ptr as *mut libc::c_uchar;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn spmatrix_long_pool_init(
    mut m: *mut gsl_spmatrix_long,
) -> libc::c_int {
    (*m)
        .pool = malloc(::std::mem::size_of::<gsl_spmatrix_pool>() as libc::c_ulong)
        as *mut gsl_spmatrix_pool;
    if ((*m).pool).is_null() {
        gsl_error(
            b"failed to allocate space for memory pool\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            387 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*(*m).pool).block_ptr = malloc(((*m).nzmax).wrapping_mul((*m).node_size));
    if ((*(*m).pool).block_ptr).is_null() {
        gsl_error(
            b"failed to allocate space for memory block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            393 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*(*m).pool).next = 0 as *mut gsl_spmatrix_pool_node;
    (*(*m).pool).free_slot = (*(*m).pool).block_ptr as *mut libc::c_uchar;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn spmatrix_complex_long_double_pool_init(
    mut m: *mut gsl_spmatrix_complex_long_double,
) -> libc::c_int {
    (*m)
        .pool = malloc(::std::mem::size_of::<gsl_spmatrix_pool>() as libc::c_ulong)
        as *mut gsl_spmatrix_pool;
    if ((*m).pool).is_null() {
        gsl_error(
            b"failed to allocate space for memory pool\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            387 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*(*m).pool).block_ptr = malloc(((*m).nzmax).wrapping_mul((*m).node_size));
    if ((*(*m).pool).block_ptr).is_null() {
        gsl_error(
            b"failed to allocate space for memory block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            393 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*(*m).pool).next = 0 as *mut gsl_spmatrix_pool_node;
    (*(*m).pool).free_slot = (*(*m).pool).block_ptr as *mut libc::c_uchar;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn spmatrix_long_double_pool_init(
    mut m: *mut gsl_spmatrix_long_double,
) -> libc::c_int {
    (*m)
        .pool = malloc(::std::mem::size_of::<gsl_spmatrix_pool>() as libc::c_ulong)
        as *mut gsl_spmatrix_pool;
    if ((*m).pool).is_null() {
        gsl_error(
            b"failed to allocate space for memory pool\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            387 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*(*m).pool).block_ptr = malloc(((*m).nzmax).wrapping_mul((*m).node_size));
    if ((*(*m).pool).block_ptr).is_null() {
        gsl_error(
            b"failed to allocate space for memory block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            393 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*(*m).pool).next = 0 as *mut gsl_spmatrix_pool_node;
    (*(*m).pool).free_slot = (*(*m).pool).block_ptr as *mut libc::c_uchar;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn spmatrix_uint_pool_init(
    mut m: *mut gsl_spmatrix_uint,
) -> libc::c_int {
    (*m)
        .pool = malloc(::std::mem::size_of::<gsl_spmatrix_pool>() as libc::c_ulong)
        as *mut gsl_spmatrix_pool;
    if ((*m).pool).is_null() {
        gsl_error(
            b"failed to allocate space for memory pool\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            387 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*(*m).pool).block_ptr = malloc(((*m).nzmax).wrapping_mul((*m).node_size));
    if ((*(*m).pool).block_ptr).is_null() {
        gsl_error(
            b"failed to allocate space for memory block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            393 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*(*m).pool).next = 0 as *mut gsl_spmatrix_pool_node;
    (*(*m).pool).free_slot = (*(*m).pool).block_ptr as *mut libc::c_uchar;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn spmatrix_uchar_pool_init(
    mut m: *mut gsl_spmatrix_uchar,
) -> libc::c_int {
    (*m)
        .pool = malloc(::std::mem::size_of::<gsl_spmatrix_pool>() as libc::c_ulong)
        as *mut gsl_spmatrix_pool;
    if ((*m).pool).is_null() {
        gsl_error(
            b"failed to allocate space for memory pool\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            387 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*(*m).pool).block_ptr = malloc(((*m).nzmax).wrapping_mul((*m).node_size));
    if ((*(*m).pool).block_ptr).is_null() {
        gsl_error(
            b"failed to allocate space for memory block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            393 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*(*m).pool).next = 0 as *mut gsl_spmatrix_pool_node;
    (*(*m).pool).free_slot = (*(*m).pool).block_ptr as *mut libc::c_uchar;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn spmatrix_short_pool_init(
    mut m: *mut gsl_spmatrix_short,
) -> libc::c_int {
    (*m)
        .pool = malloc(::std::mem::size_of::<gsl_spmatrix_pool>() as libc::c_ulong)
        as *mut gsl_spmatrix_pool;
    if ((*m).pool).is_null() {
        gsl_error(
            b"failed to allocate space for memory pool\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            387 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*(*m).pool).block_ptr = malloc(((*m).nzmax).wrapping_mul((*m).node_size));
    if ((*(*m).pool).block_ptr).is_null() {
        gsl_error(
            b"failed to allocate space for memory block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            393 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*(*m).pool).next = 0 as *mut gsl_spmatrix_pool_node;
    (*(*m).pool).free_slot = (*(*m).pool).block_ptr as *mut libc::c_uchar;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn spmatrix_complex_float_pool_init(
    mut m: *mut gsl_spmatrix_complex_float,
) -> libc::c_int {
    (*m)
        .pool = malloc(::std::mem::size_of::<gsl_spmatrix_pool>() as libc::c_ulong)
        as *mut gsl_spmatrix_pool;
    if ((*m).pool).is_null() {
        gsl_error(
            b"failed to allocate space for memory pool\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            387 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*(*m).pool).block_ptr = malloc(((*m).nzmax).wrapping_mul((*m).node_size));
    if ((*(*m).pool).block_ptr).is_null() {
        gsl_error(
            b"failed to allocate space for memory block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            393 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*(*m).pool).next = 0 as *mut gsl_spmatrix_pool_node;
    (*(*m).pool).free_slot = (*(*m).pool).block_ptr as *mut libc::c_uchar;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn spmatrix_ushort_pool_init(
    mut m: *mut gsl_spmatrix_ushort,
) -> libc::c_int {
    (*m)
        .pool = malloc(::std::mem::size_of::<gsl_spmatrix_pool>() as libc::c_ulong)
        as *mut gsl_spmatrix_pool;
    if ((*m).pool).is_null() {
        gsl_error(
            b"failed to allocate space for memory pool\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            387 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*(*m).pool).block_ptr = malloc(((*m).nzmax).wrapping_mul((*m).node_size));
    if ((*(*m).pool).block_ptr).is_null() {
        gsl_error(
            b"failed to allocate space for memory block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            393 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*(*m).pool).next = 0 as *mut gsl_spmatrix_pool_node;
    (*(*m).pool).free_slot = (*(*m).pool).block_ptr as *mut libc::c_uchar;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn spmatrix_int_pool_init(
    mut m: *mut gsl_spmatrix_int,
) -> libc::c_int {
    (*m)
        .pool = malloc(::std::mem::size_of::<gsl_spmatrix_pool>() as libc::c_ulong)
        as *mut gsl_spmatrix_pool;
    if ((*m).pool).is_null() {
        gsl_error(
            b"failed to allocate space for memory pool\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            387 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*(*m).pool).block_ptr = malloc(((*m).nzmax).wrapping_mul((*m).node_size));
    if ((*(*m).pool).block_ptr).is_null() {
        gsl_error(
            b"failed to allocate space for memory block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            393 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*(*m).pool).next = 0 as *mut gsl_spmatrix_pool_node;
    (*(*m).pool).free_slot = (*(*m).pool).block_ptr as *mut libc::c_uchar;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn spmatrix_float_pool_init(
    mut m: *mut gsl_spmatrix_float,
) -> libc::c_int {
    (*m)
        .pool = malloc(::std::mem::size_of::<gsl_spmatrix_pool>() as libc::c_ulong)
        as *mut gsl_spmatrix_pool;
    if ((*m).pool).is_null() {
        gsl_error(
            b"failed to allocate space for memory pool\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            387 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*(*m).pool).block_ptr = malloc(((*m).nzmax).wrapping_mul((*m).node_size));
    if ((*(*m).pool).block_ptr).is_null() {
        gsl_error(
            b"failed to allocate space for memory block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            393 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*(*m).pool).next = 0 as *mut gsl_spmatrix_pool_node;
    (*(*m).pool).free_slot = (*(*m).pool).block_ptr as *mut libc::c_uchar;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn spmatrix_ulong_pool_init(
    mut m: *mut gsl_spmatrix_ulong,
) -> libc::c_int {
    (*m)
        .pool = malloc(::std::mem::size_of::<gsl_spmatrix_pool>() as libc::c_ulong)
        as *mut gsl_spmatrix_pool;
    if ((*m).pool).is_null() {
        gsl_error(
            b"failed to allocate space for memory pool\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            387 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*(*m).pool).block_ptr = malloc(((*m).nzmax).wrapping_mul((*m).node_size));
    if ((*(*m).pool).block_ptr).is_null() {
        gsl_error(
            b"failed to allocate space for memory block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            393 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*(*m).pool).next = 0 as *mut gsl_spmatrix_pool_node;
    (*(*m).pool).free_slot = (*(*m).pool).block_ptr as *mut libc::c_uchar;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn spmatrix_char_pool_init(
    mut m: *mut gsl_spmatrix_char,
) -> libc::c_int {
    (*m)
        .pool = malloc(::std::mem::size_of::<gsl_spmatrix_pool>() as libc::c_ulong)
        as *mut gsl_spmatrix_pool;
    if ((*m).pool).is_null() {
        gsl_error(
            b"failed to allocate space for memory pool\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            387 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*(*m).pool).block_ptr = malloc(((*m).nzmax).wrapping_mul((*m).node_size));
    if ((*(*m).pool).block_ptr).is_null() {
        gsl_error(
            b"failed to allocate space for memory block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            393 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*(*m).pool).next = 0 as *mut gsl_spmatrix_pool_node;
    (*(*m).pool).free_slot = (*(*m).pool).block_ptr as *mut libc::c_uchar;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn spmatrix_ulong_pool_free(
    mut m: *mut gsl_spmatrix_ulong,
) -> libc::c_int {
    while !((*m).pool).is_null() {
        let mut next: *mut gsl_spmatrix_pool = (*(*m).pool).next;
        free((*(*m).pool).block_ptr);
        free((*m).pool as *mut libc::c_void);
        (*m).pool = next;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn spmatrix_char_pool_free(
    mut m: *mut gsl_spmatrix_char,
) -> libc::c_int {
    while !((*m).pool).is_null() {
        let mut next: *mut gsl_spmatrix_pool = (*(*m).pool).next;
        free((*(*m).pool).block_ptr);
        free((*m).pool as *mut libc::c_void);
        (*m).pool = next;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn spmatrix_long_pool_free(
    mut m: *mut gsl_spmatrix_long,
) -> libc::c_int {
    while !((*m).pool).is_null() {
        let mut next: *mut gsl_spmatrix_pool = (*(*m).pool).next;
        free((*(*m).pool).block_ptr);
        free((*m).pool as *mut libc::c_void);
        (*m).pool = next;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn spmatrix_uint_pool_free(
    mut m: *mut gsl_spmatrix_uint,
) -> libc::c_int {
    while !((*m).pool).is_null() {
        let mut next: *mut gsl_spmatrix_pool = (*(*m).pool).next;
        free((*(*m).pool).block_ptr);
        free((*m).pool as *mut libc::c_void);
        (*m).pool = next;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn spmatrix_complex_long_double_pool_free(
    mut m: *mut gsl_spmatrix_complex_long_double,
) -> libc::c_int {
    while !((*m).pool).is_null() {
        let mut next: *mut gsl_spmatrix_pool = (*(*m).pool).next;
        free((*(*m).pool).block_ptr);
        free((*m).pool as *mut libc::c_void);
        (*m).pool = next;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn spmatrix_float_pool_free(
    mut m: *mut gsl_spmatrix_float,
) -> libc::c_int {
    while !((*m).pool).is_null() {
        let mut next: *mut gsl_spmatrix_pool = (*(*m).pool).next;
        free((*(*m).pool).block_ptr);
        free((*m).pool as *mut libc::c_void);
        (*m).pool = next;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn spmatrix_short_pool_free(
    mut m: *mut gsl_spmatrix_short,
) -> libc::c_int {
    while !((*m).pool).is_null() {
        let mut next: *mut gsl_spmatrix_pool = (*(*m).pool).next;
        free((*(*m).pool).block_ptr);
        free((*m).pool as *mut libc::c_void);
        (*m).pool = next;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn spmatrix_complex_pool_free(
    mut m: *mut gsl_spmatrix_complex,
) -> libc::c_int {
    while !((*m).pool).is_null() {
        let mut next: *mut gsl_spmatrix_pool = (*(*m).pool).next;
        free((*(*m).pool).block_ptr);
        free((*m).pool as *mut libc::c_void);
        (*m).pool = next;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn spmatrix_complex_float_pool_free(
    mut m: *mut gsl_spmatrix_complex_float,
) -> libc::c_int {
    while !((*m).pool).is_null() {
        let mut next: *mut gsl_spmatrix_pool = (*(*m).pool).next;
        free((*(*m).pool).block_ptr);
        free((*m).pool as *mut libc::c_void);
        (*m).pool = next;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn spmatrix_int_pool_free(
    mut m: *mut gsl_spmatrix_int,
) -> libc::c_int {
    while !((*m).pool).is_null() {
        let mut next: *mut gsl_spmatrix_pool = (*(*m).pool).next;
        free((*(*m).pool).block_ptr);
        free((*m).pool as *mut libc::c_void);
        (*m).pool = next;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn spmatrix_uchar_pool_free(
    mut m: *mut gsl_spmatrix_uchar,
) -> libc::c_int {
    while !((*m).pool).is_null() {
        let mut next: *mut gsl_spmatrix_pool = (*(*m).pool).next;
        free((*(*m).pool).block_ptr);
        free((*m).pool as *mut libc::c_void);
        (*m).pool = next;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn spmatrix_pool_free(mut m: *mut gsl_spmatrix) -> libc::c_int {
    while !((*m).pool).is_null() {
        let mut next: *mut gsl_spmatrix_pool = (*(*m).pool).next;
        free((*(*m).pool).block_ptr);
        free((*m).pool as *mut libc::c_void);
        (*m).pool = next;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn spmatrix_long_double_pool_free(
    mut m: *mut gsl_spmatrix_long_double,
) -> libc::c_int {
    while !((*m).pool).is_null() {
        let mut next: *mut gsl_spmatrix_pool = (*(*m).pool).next;
        free((*(*m).pool).block_ptr);
        free((*m).pool as *mut libc::c_void);
        (*m).pool = next;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn spmatrix_ushort_pool_free(
    mut m: *mut gsl_spmatrix_ushort,
) -> libc::c_int {
    while !((*m).pool).is_null() {
        let mut next: *mut gsl_spmatrix_pool = (*(*m).pool).next;
        free((*(*m).pool).block_ptr);
        free((*m).pool as *mut libc::c_void);
        (*m).pool = next;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn spmatrix_complex_float_malloc(
    mut size: size_t,
    mut params: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut m: *mut gsl_spmatrix_complex_float = params
        as *mut gsl_spmatrix_complex_float;
    let mut pool: *mut gsl_spmatrix_pool = (*m).pool;
    let mut ptr: *mut libc::c_void = (*pool).free_slot as *mut libc::c_void;
    (*pool).free_slot = ((*pool).free_slot).offset(size as isize);
    return ptr;
}
unsafe extern "C" fn spmatrix_ulong_malloc(
    mut size: size_t,
    mut params: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut m: *mut gsl_spmatrix_ulong = params as *mut gsl_spmatrix_ulong;
    let mut pool: *mut gsl_spmatrix_pool = (*m).pool;
    let mut ptr: *mut libc::c_void = (*pool).free_slot as *mut libc::c_void;
    (*pool).free_slot = ((*pool).free_slot).offset(size as isize);
    return ptr;
}
unsafe extern "C" fn spmatrix_long_malloc(
    mut size: size_t,
    mut params: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut m: *mut gsl_spmatrix_long = params as *mut gsl_spmatrix_long;
    let mut pool: *mut gsl_spmatrix_pool = (*m).pool;
    let mut ptr: *mut libc::c_void = (*pool).free_slot as *mut libc::c_void;
    (*pool).free_slot = ((*pool).free_slot).offset(size as isize);
    return ptr;
}
unsafe extern "C" fn spmatrix_int_malloc(
    mut size: size_t,
    mut params: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut m: *mut gsl_spmatrix_int = params as *mut gsl_spmatrix_int;
    let mut pool: *mut gsl_spmatrix_pool = (*m).pool;
    let mut ptr: *mut libc::c_void = (*pool).free_slot as *mut libc::c_void;
    (*pool).free_slot = ((*pool).free_slot).offset(size as isize);
    return ptr;
}
unsafe extern "C" fn spmatrix_uint_malloc(
    mut size: size_t,
    mut params: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut m: *mut gsl_spmatrix_uint = params as *mut gsl_spmatrix_uint;
    let mut pool: *mut gsl_spmatrix_pool = (*m).pool;
    let mut ptr: *mut libc::c_void = (*pool).free_slot as *mut libc::c_void;
    (*pool).free_slot = ((*pool).free_slot).offset(size as isize);
    return ptr;
}
unsafe extern "C" fn spmatrix_uchar_malloc(
    mut size: size_t,
    mut params: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut m: *mut gsl_spmatrix_uchar = params as *mut gsl_spmatrix_uchar;
    let mut pool: *mut gsl_spmatrix_pool = (*m).pool;
    let mut ptr: *mut libc::c_void = (*pool).free_slot as *mut libc::c_void;
    (*pool).free_slot = ((*pool).free_slot).offset(size as isize);
    return ptr;
}
unsafe extern "C" fn spmatrix_malloc(
    mut size: size_t,
    mut params: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut m: *mut gsl_spmatrix = params as *mut gsl_spmatrix;
    let mut pool: *mut gsl_spmatrix_pool = (*m).pool;
    let mut ptr: *mut libc::c_void = (*pool).free_slot as *mut libc::c_void;
    (*pool).free_slot = ((*pool).free_slot).offset(size as isize);
    return ptr;
}
unsafe extern "C" fn spmatrix_long_double_malloc(
    mut size: size_t,
    mut params: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut m: *mut gsl_spmatrix_long_double = params as *mut gsl_spmatrix_long_double;
    let mut pool: *mut gsl_spmatrix_pool = (*m).pool;
    let mut ptr: *mut libc::c_void = (*pool).free_slot as *mut libc::c_void;
    (*pool).free_slot = ((*pool).free_slot).offset(size as isize);
    return ptr;
}
unsafe extern "C" fn spmatrix_complex_long_double_malloc(
    mut size: size_t,
    mut params: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut m: *mut gsl_spmatrix_complex_long_double = params
        as *mut gsl_spmatrix_complex_long_double;
    let mut pool: *mut gsl_spmatrix_pool = (*m).pool;
    let mut ptr: *mut libc::c_void = (*pool).free_slot as *mut libc::c_void;
    (*pool).free_slot = ((*pool).free_slot).offset(size as isize);
    return ptr;
}
unsafe extern "C" fn spmatrix_ushort_malloc(
    mut size: size_t,
    mut params: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut m: *mut gsl_spmatrix_ushort = params as *mut gsl_spmatrix_ushort;
    let mut pool: *mut gsl_spmatrix_pool = (*m).pool;
    let mut ptr: *mut libc::c_void = (*pool).free_slot as *mut libc::c_void;
    (*pool).free_slot = ((*pool).free_slot).offset(size as isize);
    return ptr;
}
unsafe extern "C" fn spmatrix_char_malloc(
    mut size: size_t,
    mut params: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut m: *mut gsl_spmatrix_char = params as *mut gsl_spmatrix_char;
    let mut pool: *mut gsl_spmatrix_pool = (*m).pool;
    let mut ptr: *mut libc::c_void = (*pool).free_slot as *mut libc::c_void;
    (*pool).free_slot = ((*pool).free_slot).offset(size as isize);
    return ptr;
}
unsafe extern "C" fn spmatrix_complex_malloc(
    mut size: size_t,
    mut params: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut m: *mut gsl_spmatrix_complex = params as *mut gsl_spmatrix_complex;
    let mut pool: *mut gsl_spmatrix_pool = (*m).pool;
    let mut ptr: *mut libc::c_void = (*pool).free_slot as *mut libc::c_void;
    (*pool).free_slot = ((*pool).free_slot).offset(size as isize);
    return ptr;
}
unsafe extern "C" fn spmatrix_short_malloc(
    mut size: size_t,
    mut params: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut m: *mut gsl_spmatrix_short = params as *mut gsl_spmatrix_short;
    let mut pool: *mut gsl_spmatrix_pool = (*m).pool;
    let mut ptr: *mut libc::c_void = (*pool).free_slot as *mut libc::c_void;
    (*pool).free_slot = ((*pool).free_slot).offset(size as isize);
    return ptr;
}
unsafe extern "C" fn spmatrix_float_malloc(
    mut size: size_t,
    mut params: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut m: *mut gsl_spmatrix_float = params as *mut gsl_spmatrix_float;
    let mut pool: *mut gsl_spmatrix_pool = (*m).pool;
    let mut ptr: *mut libc::c_void = (*pool).free_slot as *mut libc::c_void;
    (*pool).free_slot = ((*pool).free_slot).offset(size as isize);
    return ptr;
}
unsafe extern "C" fn spmatrix_long_double_free(
    mut block: *mut libc::c_void,
    mut params: *mut libc::c_void,
) {}
unsafe extern "C" fn spmatrix_uint_free(
    mut block: *mut libc::c_void,
    mut params: *mut libc::c_void,
) {}
unsafe extern "C" fn spmatrix_short_free(
    mut block: *mut libc::c_void,
    mut params: *mut libc::c_void,
) {}
unsafe extern "C" fn spmatrix_complex_long_double_free(
    mut block: *mut libc::c_void,
    mut params: *mut libc::c_void,
) {}
unsafe extern "C" fn spmatrix_ushort_free(
    mut block: *mut libc::c_void,
    mut params: *mut libc::c_void,
) {}
unsafe extern "C" fn spmatrix_char_free(
    mut block: *mut libc::c_void,
    mut params: *mut libc::c_void,
) {}
unsafe extern "C" fn spmatrix_complex_float_free(
    mut block: *mut libc::c_void,
    mut params: *mut libc::c_void,
) {}
unsafe extern "C" fn spmatrix_complex_free(
    mut block: *mut libc::c_void,
    mut params: *mut libc::c_void,
) {}
unsafe extern "C" fn spmatrix_free(
    mut block: *mut libc::c_void,
    mut params: *mut libc::c_void,
) {}
unsafe extern "C" fn spmatrix_uchar_free(
    mut block: *mut libc::c_void,
    mut params: *mut libc::c_void,
) {}
unsafe extern "C" fn spmatrix_long_free(
    mut block: *mut libc::c_void,
    mut params: *mut libc::c_void,
) {}
unsafe extern "C" fn spmatrix_float_free(
    mut block: *mut libc::c_void,
    mut params: *mut libc::c_void,
) {}
unsafe extern "C" fn spmatrix_int_free(
    mut block: *mut libc::c_void,
    mut params: *mut libc::c_void,
) {}
unsafe extern "C" fn spmatrix_ulong_free(
    mut block: *mut libc::c_void,
    mut params: *mut libc::c_void,
) {}
