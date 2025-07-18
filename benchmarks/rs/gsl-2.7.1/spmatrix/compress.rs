use ::libc;
extern "C" {
    fn gsl_spmatrix_char_memcpy(
        dest: *mut gsl_spmatrix_char,
        src: *const gsl_spmatrix_char,
    ) -> libc::c_int;
    fn gsl_spmatrix_char_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_char,
    ) -> libc::c_int;
    fn gsl_spmatrix_char_free(m: *mut gsl_spmatrix_char);
    fn gsl_spmatrix_char_alloc_nzmax(
        n1: size_t,
        n2: size_t,
        nzmax: size_t,
        sptype: libc::c_int,
    ) -> *mut gsl_spmatrix_char;
    fn gsl_spmatrix_uchar_memcpy(
        dest: *mut gsl_spmatrix_uchar,
        src: *const gsl_spmatrix_uchar,
    ) -> libc::c_int;
    fn gsl_spmatrix_uchar_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_uchar,
    ) -> libc::c_int;
    fn gsl_spmatrix_uchar_free(m: *mut gsl_spmatrix_uchar);
    fn gsl_spmatrix_uchar_alloc_nzmax(
        n1: size_t,
        n2: size_t,
        nzmax: size_t,
        sptype: libc::c_int,
    ) -> *mut gsl_spmatrix_uchar;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_spmatrix_complex_long_double_alloc_nzmax(
        n1: size_t,
        n2: size_t,
        nzmax: size_t,
        sptype: libc::c_int,
    ) -> *mut gsl_spmatrix_complex_long_double;
    fn gsl_spmatrix_complex_long_double_free(m: *mut gsl_spmatrix_complex_long_double);
    fn gsl_spmatrix_complex_long_double_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_complex_long_double,
    ) -> libc::c_int;
    fn gsl_spmatrix_complex_long_double_memcpy(
        dest: *mut gsl_spmatrix_complex_long_double,
        src: *const gsl_spmatrix_complex_long_double,
    ) -> libc::c_int;
    fn gsl_spmatrix_complex_alloc_nzmax(
        n1: size_t,
        n2: size_t,
        nzmax: size_t,
        sptype: libc::c_int,
    ) -> *mut gsl_spmatrix_complex;
    fn gsl_spmatrix_complex_free(m: *mut gsl_spmatrix_complex);
    fn gsl_spmatrix_complex_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_complex,
    ) -> libc::c_int;
    fn gsl_spmatrix_complex_memcpy(
        dest: *mut gsl_spmatrix_complex,
        src: *const gsl_spmatrix_complex,
    ) -> libc::c_int;
    fn gsl_spmatrix_cumsum(n: size_t, c: *mut libc::c_int);
    fn gsl_spmatrix_complex_float_alloc_nzmax(
        n1: size_t,
        n2: size_t,
        nzmax: size_t,
        sptype: libc::c_int,
    ) -> *mut gsl_spmatrix_complex_float;
    fn gsl_spmatrix_complex_float_free(m: *mut gsl_spmatrix_complex_float);
    fn gsl_spmatrix_complex_float_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_complex_float,
    ) -> libc::c_int;
    fn gsl_spmatrix_complex_float_memcpy(
        dest: *mut gsl_spmatrix_complex_float,
        src: *const gsl_spmatrix_complex_float,
    ) -> libc::c_int;
    fn gsl_spmatrix_long_double_alloc_nzmax(
        n1: size_t,
        n2: size_t,
        nzmax: size_t,
        sptype: libc::c_int,
    ) -> *mut gsl_spmatrix_long_double;
    fn gsl_spmatrix_long_double_free(m: *mut gsl_spmatrix_long_double);
    fn gsl_spmatrix_long_double_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_long_double,
    ) -> libc::c_int;
    fn gsl_spmatrix_short_memcpy(
        dest: *mut gsl_spmatrix_short,
        src: *const gsl_spmatrix_short,
    ) -> libc::c_int;
    fn gsl_spmatrix_long_double_memcpy(
        dest: *mut gsl_spmatrix_long_double,
        src: *const gsl_spmatrix_long_double,
    ) -> libc::c_int;
    fn gsl_spmatrix_alloc_nzmax(
        n1: size_t,
        n2: size_t,
        nzmax: size_t,
        sptype: libc::c_int,
    ) -> *mut gsl_spmatrix;
    fn gsl_spmatrix_free(m: *mut gsl_spmatrix);
    fn gsl_spmatrix_realloc(nzmax: size_t, m: *mut gsl_spmatrix) -> libc::c_int;
    fn gsl_spmatrix_short_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_short,
    ) -> libc::c_int;
    fn gsl_spmatrix_short_free(m: *mut gsl_spmatrix_short);
    fn gsl_spmatrix_short_alloc_nzmax(
        n1: size_t,
        n2: size_t,
        nzmax: size_t,
        sptype: libc::c_int,
    ) -> *mut gsl_spmatrix_short;
    fn gsl_spmatrix_memcpy(
        dest: *mut gsl_spmatrix,
        src: *const gsl_spmatrix,
    ) -> libc::c_int;
    fn gsl_spmatrix_float_alloc_nzmax(
        n1: size_t,
        n2: size_t,
        nzmax: size_t,
        sptype: libc::c_int,
    ) -> *mut gsl_spmatrix_float;
    fn gsl_spmatrix_float_free(m: *mut gsl_spmatrix_float);
    fn gsl_spmatrix_float_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_float,
    ) -> libc::c_int;
    fn gsl_spmatrix_float_memcpy(
        dest: *mut gsl_spmatrix_float,
        src: *const gsl_spmatrix_float,
    ) -> libc::c_int;
    fn gsl_spmatrix_ulong_alloc_nzmax(
        n1: size_t,
        n2: size_t,
        nzmax: size_t,
        sptype: libc::c_int,
    ) -> *mut gsl_spmatrix_ulong;
    fn gsl_spmatrix_ulong_free(m: *mut gsl_spmatrix_ulong);
    fn gsl_spmatrix_ulong_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_ulong,
    ) -> libc::c_int;
    fn gsl_spmatrix_ulong_memcpy(
        dest: *mut gsl_spmatrix_ulong,
        src: *const gsl_spmatrix_ulong,
    ) -> libc::c_int;
    fn gsl_spmatrix_long_alloc_nzmax(
        n1: size_t,
        n2: size_t,
        nzmax: size_t,
        sptype: libc::c_int,
    ) -> *mut gsl_spmatrix_long;
    fn gsl_spmatrix_long_free(m: *mut gsl_spmatrix_long);
    fn gsl_spmatrix_long_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_long,
    ) -> libc::c_int;
    fn gsl_spmatrix_long_memcpy(
        dest: *mut gsl_spmatrix_long,
        src: *const gsl_spmatrix_long,
    ) -> libc::c_int;
    fn gsl_spmatrix_uint_alloc_nzmax(
        n1: size_t,
        n2: size_t,
        nzmax: size_t,
        sptype: libc::c_int,
    ) -> *mut gsl_spmatrix_uint;
    fn gsl_spmatrix_uint_free(m: *mut gsl_spmatrix_uint);
    fn gsl_spmatrix_uint_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_uint,
    ) -> libc::c_int;
    fn gsl_spmatrix_uint_memcpy(
        dest: *mut gsl_spmatrix_uint,
        src: *const gsl_spmatrix_uint,
    ) -> libc::c_int;
    fn gsl_spmatrix_int_alloc_nzmax(
        n1: size_t,
        n2: size_t,
        nzmax: size_t,
        sptype: libc::c_int,
    ) -> *mut gsl_spmatrix_int;
    fn gsl_spmatrix_int_free(m: *mut gsl_spmatrix_int);
    fn gsl_spmatrix_int_realloc(nzmax: size_t, m: *mut gsl_spmatrix_int) -> libc::c_int;
    fn gsl_spmatrix_int_memcpy(
        dest: *mut gsl_spmatrix_int,
        src: *const gsl_spmatrix_int,
    ) -> libc::c_int;
    fn gsl_spmatrix_ushort_alloc_nzmax(
        n1: size_t,
        n2: size_t,
        nzmax: size_t,
        sptype: libc::c_int,
    ) -> *mut gsl_spmatrix_ushort;
    fn gsl_spmatrix_ushort_free(m: *mut gsl_spmatrix_ushort);
    fn gsl_spmatrix_ushort_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_ushort,
    ) -> libc::c_int;
    fn gsl_spmatrix_ushort_memcpy(
        dest: *mut gsl_spmatrix_ushort,
        src: *const gsl_spmatrix_ushort,
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
pub unsafe extern "C" fn gsl_spmatrix_long_csc(
    mut dest: *mut gsl_spmatrix_long,
    mut src: *const gsl_spmatrix_long,
) -> libc::c_int {
    if !((*src).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"input matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if !((*dest).sptype == GSL_SPMATRIX_CSC as libc::c_int) {
        gsl_error(
            b"output matrix must be in CSC format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            38 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if (*src).size1 != (*dest).size1 || (*src).size2 != (*dest).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            42 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut Tj: *const libc::c_int = (*src).p;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut w: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_long_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        Cp = (*dest).p;
        n = 0 as libc::c_int as size_t;
        while n < ((*dest).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            *Cp.offset(n as isize) = 0 as libc::c_int;
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh0 = *Cp.offset(*Tj.offset(n as isize) as isize);
            *fresh0 += 1;
            *fresh0;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_cumsum((*dest).size2, Cp);
        w = (*dest).work.work_int;
        n = 0 as libc::c_int as size_t;
        while n < (*dest).size2 {
            *w.offset(n as isize) = *Cp.offset(n as isize);
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh1 = *w.offset(*Tj.offset(n as isize) as isize);
            let fresh2 = *fresh1;
            *fresh1 = *fresh1 + 1;
            let mut k: libc::c_int = fresh2;
            *((*dest).i).offset(k as isize) = *((*src).i).offset(n as isize);
            r = 0 as libc::c_int as size_t;
            while r < 1 as libc::c_int as libc::c_ulong {
                *((*dest).data)
                    .offset(
                        ((1 as libc::c_int * k) as libc::c_ulong).wrapping_add(r)
                            as isize,
                    ) = *((*src).data)
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(n)
                            .wrapping_add(r) as isize,
                    );
                r = r.wrapping_add(1);
                r;
            }
            n = n.wrapping_add(1);
            n;
        }
        (*dest).nz = (*src).nz;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_float_csc(
    mut dest: *mut gsl_spmatrix_float,
    mut src: *const gsl_spmatrix_float,
) -> libc::c_int {
    if !((*src).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"input matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if !((*dest).sptype == GSL_SPMATRIX_CSC as libc::c_int) {
        gsl_error(
            b"output matrix must be in CSC format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            38 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if (*src).size1 != (*dest).size1 || (*src).size2 != (*dest).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            42 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut Tj: *const libc::c_int = (*src).p;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut w: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_float_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        Cp = (*dest).p;
        n = 0 as libc::c_int as size_t;
        while n < ((*dest).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            *Cp.offset(n as isize) = 0 as libc::c_int;
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh3 = *Cp.offset(*Tj.offset(n as isize) as isize);
            *fresh3 += 1;
            *fresh3;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_cumsum((*dest).size2, Cp);
        w = (*dest).work.work_int;
        n = 0 as libc::c_int as size_t;
        while n < (*dest).size2 {
            *w.offset(n as isize) = *Cp.offset(n as isize);
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh4 = *w.offset(*Tj.offset(n as isize) as isize);
            let fresh5 = *fresh4;
            *fresh4 = *fresh4 + 1;
            let mut k: libc::c_int = fresh5;
            *((*dest).i).offset(k as isize) = *((*src).i).offset(n as isize);
            r = 0 as libc::c_int as size_t;
            while r < 1 as libc::c_int as libc::c_ulong {
                *((*dest).data)
                    .offset(
                        ((1 as libc::c_int * k) as libc::c_ulong).wrapping_add(r)
                            as isize,
                    ) = *((*src).data)
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(n)
                            .wrapping_add(r) as isize,
                    );
                r = r.wrapping_add(1);
                r;
            }
            n = n.wrapping_add(1);
            n;
        }
        (*dest).nz = (*src).nz;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_char_csc(
    mut dest: *mut gsl_spmatrix_char,
    mut src: *const gsl_spmatrix_char,
) -> libc::c_int {
    if !((*src).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"input matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if !((*dest).sptype == GSL_SPMATRIX_CSC as libc::c_int) {
        gsl_error(
            b"output matrix must be in CSC format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            38 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if (*src).size1 != (*dest).size1 || (*src).size2 != (*dest).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            42 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut Tj: *const libc::c_int = (*src).p;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut w: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_char_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        Cp = (*dest).p;
        n = 0 as libc::c_int as size_t;
        while n < ((*dest).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            *Cp.offset(n as isize) = 0 as libc::c_int;
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh6 = *Cp.offset(*Tj.offset(n as isize) as isize);
            *fresh6 += 1;
            *fresh6;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_cumsum((*dest).size2, Cp);
        w = (*dest).work.work_int;
        n = 0 as libc::c_int as size_t;
        while n < (*dest).size2 {
            *w.offset(n as isize) = *Cp.offset(n as isize);
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh7 = *w.offset(*Tj.offset(n as isize) as isize);
            let fresh8 = *fresh7;
            *fresh7 = *fresh7 + 1;
            let mut k: libc::c_int = fresh8;
            *((*dest).i).offset(k as isize) = *((*src).i).offset(n as isize);
            r = 0 as libc::c_int as size_t;
            while r < 1 as libc::c_int as libc::c_ulong {
                *((*dest).data)
                    .offset(
                        ((1 as libc::c_int * k) as libc::c_ulong).wrapping_add(r)
                            as isize,
                    ) = *((*src).data)
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(n)
                            .wrapping_add(r) as isize,
                    );
                r = r.wrapping_add(1);
                r;
            }
            n = n.wrapping_add(1);
            n;
        }
        (*dest).nz = (*src).nz;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_ushort_csc(
    mut dest: *mut gsl_spmatrix_ushort,
    mut src: *const gsl_spmatrix_ushort,
) -> libc::c_int {
    if !((*src).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"input matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if !((*dest).sptype == GSL_SPMATRIX_CSC as libc::c_int) {
        gsl_error(
            b"output matrix must be in CSC format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            38 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if (*src).size1 != (*dest).size1 || (*src).size2 != (*dest).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            42 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut Tj: *const libc::c_int = (*src).p;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut w: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_ushort_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        Cp = (*dest).p;
        n = 0 as libc::c_int as size_t;
        while n < ((*dest).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            *Cp.offset(n as isize) = 0 as libc::c_int;
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh9 = *Cp.offset(*Tj.offset(n as isize) as isize);
            *fresh9 += 1;
            *fresh9;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_cumsum((*dest).size2, Cp);
        w = (*dest).work.work_int;
        n = 0 as libc::c_int as size_t;
        while n < (*dest).size2 {
            *w.offset(n as isize) = *Cp.offset(n as isize);
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh10 = *w.offset(*Tj.offset(n as isize) as isize);
            let fresh11 = *fresh10;
            *fresh10 = *fresh10 + 1;
            let mut k: libc::c_int = fresh11;
            *((*dest).i).offset(k as isize) = *((*src).i).offset(n as isize);
            r = 0 as libc::c_int as size_t;
            while r < 1 as libc::c_int as libc::c_ulong {
                *((*dest).data)
                    .offset(
                        ((1 as libc::c_int * k) as libc::c_ulong).wrapping_add(r)
                            as isize,
                    ) = *((*src).data)
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(n)
                            .wrapping_add(r) as isize,
                    );
                r = r.wrapping_add(1);
                r;
            }
            n = n.wrapping_add(1);
            n;
        }
        (*dest).nz = (*src).nz;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_short_csc(
    mut dest: *mut gsl_spmatrix_short,
    mut src: *const gsl_spmatrix_short,
) -> libc::c_int {
    if !((*src).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"input matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if !((*dest).sptype == GSL_SPMATRIX_CSC as libc::c_int) {
        gsl_error(
            b"output matrix must be in CSC format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            38 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if (*src).size1 != (*dest).size1 || (*src).size2 != (*dest).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            42 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut Tj: *const libc::c_int = (*src).p;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut w: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_short_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        Cp = (*dest).p;
        n = 0 as libc::c_int as size_t;
        while n < ((*dest).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            *Cp.offset(n as isize) = 0 as libc::c_int;
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh12 = *Cp.offset(*Tj.offset(n as isize) as isize);
            *fresh12 += 1;
            *fresh12;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_cumsum((*dest).size2, Cp);
        w = (*dest).work.work_int;
        n = 0 as libc::c_int as size_t;
        while n < (*dest).size2 {
            *w.offset(n as isize) = *Cp.offset(n as isize);
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh13 = *w.offset(*Tj.offset(n as isize) as isize);
            let fresh14 = *fresh13;
            *fresh13 = *fresh13 + 1;
            let mut k: libc::c_int = fresh14;
            *((*dest).i).offset(k as isize) = *((*src).i).offset(n as isize);
            r = 0 as libc::c_int as size_t;
            while r < 1 as libc::c_int as libc::c_ulong {
                *((*dest).data)
                    .offset(
                        ((1 as libc::c_int * k) as libc::c_ulong).wrapping_add(r)
                            as isize,
                    ) = *((*src).data)
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(n)
                            .wrapping_add(r) as isize,
                    );
                r = r.wrapping_add(1);
                r;
            }
            n = n.wrapping_add(1);
            n;
        }
        (*dest).nz = (*src).nz;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_long_double_csc(
    mut dest: *mut gsl_spmatrix_complex_long_double,
    mut src: *const gsl_spmatrix_complex_long_double,
) -> libc::c_int {
    if !((*src).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"input matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if !((*dest).sptype == GSL_SPMATRIX_CSC as libc::c_int) {
        gsl_error(
            b"output matrix must be in CSC format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            38 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if (*src).size1 != (*dest).size1 || (*src).size2 != (*dest).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            42 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut Tj: *const libc::c_int = (*src).p;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut w: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_complex_long_double_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        Cp = (*dest).p;
        n = 0 as libc::c_int as size_t;
        while n < ((*dest).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            *Cp.offset(n as isize) = 0 as libc::c_int;
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh15 = *Cp.offset(*Tj.offset(n as isize) as isize);
            *fresh15 += 1;
            *fresh15;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_cumsum((*dest).size2, Cp);
        w = (*dest).work.work_int;
        n = 0 as libc::c_int as size_t;
        while n < (*dest).size2 {
            *w.offset(n as isize) = *Cp.offset(n as isize);
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh16 = *w.offset(*Tj.offset(n as isize) as isize);
            let fresh17 = *fresh16;
            *fresh16 = *fresh16 + 1;
            let mut k: libc::c_int = fresh17;
            *((*dest).i).offset(k as isize) = *((*src).i).offset(n as isize);
            r = 0 as libc::c_int as size_t;
            while r < 2 as libc::c_int as libc::c_ulong {
                *((*dest).data)
                    .offset(
                        ((2 as libc::c_int * k) as libc::c_ulong).wrapping_add(r)
                            as isize,
                    ) = *((*src).data)
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(n)
                            .wrapping_add(r) as isize,
                    );
                r = r.wrapping_add(1);
                r;
            }
            n = n.wrapping_add(1);
            n;
        }
        (*dest).nz = (*src).nz;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_csc(
    mut dest: *mut gsl_spmatrix,
    mut src: *const gsl_spmatrix,
) -> libc::c_int {
    if !((*src).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"input matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if !((*dest).sptype == GSL_SPMATRIX_CSC as libc::c_int) {
        gsl_error(
            b"output matrix must be in CSC format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            38 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if (*src).size1 != (*dest).size1 || (*src).size2 != (*dest).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            42 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut Tj: *const libc::c_int = (*src).p;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut w: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        Cp = (*dest).p;
        n = 0 as libc::c_int as size_t;
        while n < ((*dest).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            *Cp.offset(n as isize) = 0 as libc::c_int;
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh18 = *Cp.offset(*Tj.offset(n as isize) as isize);
            *fresh18 += 1;
            *fresh18;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_cumsum((*dest).size2, Cp);
        w = (*dest).work.work_int;
        n = 0 as libc::c_int as size_t;
        while n < (*dest).size2 {
            *w.offset(n as isize) = *Cp.offset(n as isize);
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh19 = *w.offset(*Tj.offset(n as isize) as isize);
            let fresh20 = *fresh19;
            *fresh19 = *fresh19 + 1;
            let mut k: libc::c_int = fresh20;
            *((*dest).i).offset(k as isize) = *((*src).i).offset(n as isize);
            r = 0 as libc::c_int as size_t;
            while r < 1 as libc::c_int as libc::c_ulong {
                *((*dest).data)
                    .offset(
                        ((1 as libc::c_int * k) as libc::c_ulong).wrapping_add(r)
                            as isize,
                    ) = *((*src).data)
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(n)
                            .wrapping_add(r) as isize,
                    );
                r = r.wrapping_add(1);
                r;
            }
            n = n.wrapping_add(1);
            n;
        }
        (*dest).nz = (*src).nz;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_int_csc(
    mut dest: *mut gsl_spmatrix_int,
    mut src: *const gsl_spmatrix_int,
) -> libc::c_int {
    if !((*src).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"input matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if !((*dest).sptype == GSL_SPMATRIX_CSC as libc::c_int) {
        gsl_error(
            b"output matrix must be in CSC format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            38 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if (*src).size1 != (*dest).size1 || (*src).size2 != (*dest).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            42 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut Tj: *const libc::c_int = (*src).p;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut w: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_int_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        Cp = (*dest).p;
        n = 0 as libc::c_int as size_t;
        while n < ((*dest).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            *Cp.offset(n as isize) = 0 as libc::c_int;
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh21 = *Cp.offset(*Tj.offset(n as isize) as isize);
            *fresh21 += 1;
            *fresh21;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_cumsum((*dest).size2, Cp);
        w = (*dest).work.work_int;
        n = 0 as libc::c_int as size_t;
        while n < (*dest).size2 {
            *w.offset(n as isize) = *Cp.offset(n as isize);
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh22 = *w.offset(*Tj.offset(n as isize) as isize);
            let fresh23 = *fresh22;
            *fresh22 = *fresh22 + 1;
            let mut k: libc::c_int = fresh23;
            *((*dest).i).offset(k as isize) = *((*src).i).offset(n as isize);
            r = 0 as libc::c_int as size_t;
            while r < 1 as libc::c_int as libc::c_ulong {
                *((*dest).data)
                    .offset(
                        ((1 as libc::c_int * k) as libc::c_ulong).wrapping_add(r)
                            as isize,
                    ) = *((*src).data)
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(n)
                            .wrapping_add(r) as isize,
                    );
                r = r.wrapping_add(1);
                r;
            }
            n = n.wrapping_add(1);
            n;
        }
        (*dest).nz = (*src).nz;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_csc(
    mut dest: *mut gsl_spmatrix_long_double,
    mut src: *const gsl_spmatrix_long_double,
) -> libc::c_int {
    if !((*src).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"input matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if !((*dest).sptype == GSL_SPMATRIX_CSC as libc::c_int) {
        gsl_error(
            b"output matrix must be in CSC format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            38 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if (*src).size1 != (*dest).size1 || (*src).size2 != (*dest).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            42 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut Tj: *const libc::c_int = (*src).p;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut w: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_long_double_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        Cp = (*dest).p;
        n = 0 as libc::c_int as size_t;
        while n < ((*dest).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            *Cp.offset(n as isize) = 0 as libc::c_int;
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh24 = *Cp.offset(*Tj.offset(n as isize) as isize);
            *fresh24 += 1;
            *fresh24;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_cumsum((*dest).size2, Cp);
        w = (*dest).work.work_int;
        n = 0 as libc::c_int as size_t;
        while n < (*dest).size2 {
            *w.offset(n as isize) = *Cp.offset(n as isize);
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh25 = *w.offset(*Tj.offset(n as isize) as isize);
            let fresh26 = *fresh25;
            *fresh25 = *fresh25 + 1;
            let mut k: libc::c_int = fresh26;
            *((*dest).i).offset(k as isize) = *((*src).i).offset(n as isize);
            r = 0 as libc::c_int as size_t;
            while r < 1 as libc::c_int as libc::c_ulong {
                *((*dest).data)
                    .offset(
                        ((1 as libc::c_int * k) as libc::c_ulong).wrapping_add(r)
                            as isize,
                    ) = *((*src).data)
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(n)
                            .wrapping_add(r) as isize,
                    );
                r = r.wrapping_add(1);
                r;
            }
            n = n.wrapping_add(1);
            n;
        }
        (*dest).nz = (*src).nz;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uchar_csc(
    mut dest: *mut gsl_spmatrix_uchar,
    mut src: *const gsl_spmatrix_uchar,
) -> libc::c_int {
    if !((*src).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"input matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if !((*dest).sptype == GSL_SPMATRIX_CSC as libc::c_int) {
        gsl_error(
            b"output matrix must be in CSC format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            38 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if (*src).size1 != (*dest).size1 || (*src).size2 != (*dest).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            42 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut Tj: *const libc::c_int = (*src).p;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut w: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_uchar_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        Cp = (*dest).p;
        n = 0 as libc::c_int as size_t;
        while n < ((*dest).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            *Cp.offset(n as isize) = 0 as libc::c_int;
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh27 = *Cp.offset(*Tj.offset(n as isize) as isize);
            *fresh27 += 1;
            *fresh27;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_cumsum((*dest).size2, Cp);
        w = (*dest).work.work_int;
        n = 0 as libc::c_int as size_t;
        while n < (*dest).size2 {
            *w.offset(n as isize) = *Cp.offset(n as isize);
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh28 = *w.offset(*Tj.offset(n as isize) as isize);
            let fresh29 = *fresh28;
            *fresh28 = *fresh28 + 1;
            let mut k: libc::c_int = fresh29;
            *((*dest).i).offset(k as isize) = *((*src).i).offset(n as isize);
            r = 0 as libc::c_int as size_t;
            while r < 1 as libc::c_int as libc::c_ulong {
                *((*dest).data)
                    .offset(
                        ((1 as libc::c_int * k) as libc::c_ulong).wrapping_add(r)
                            as isize,
                    ) = *((*src).data)
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(n)
                            .wrapping_add(r) as isize,
                    );
                r = r.wrapping_add(1);
                r;
            }
            n = n.wrapping_add(1);
            n;
        }
        (*dest).nz = (*src).nz;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_float_csc(
    mut dest: *mut gsl_spmatrix_complex_float,
    mut src: *const gsl_spmatrix_complex_float,
) -> libc::c_int {
    if !((*src).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"input matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if !((*dest).sptype == GSL_SPMATRIX_CSC as libc::c_int) {
        gsl_error(
            b"output matrix must be in CSC format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            38 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if (*src).size1 != (*dest).size1 || (*src).size2 != (*dest).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            42 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut Tj: *const libc::c_int = (*src).p;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut w: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_complex_float_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        Cp = (*dest).p;
        n = 0 as libc::c_int as size_t;
        while n < ((*dest).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            *Cp.offset(n as isize) = 0 as libc::c_int;
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh30 = *Cp.offset(*Tj.offset(n as isize) as isize);
            *fresh30 += 1;
            *fresh30;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_cumsum((*dest).size2, Cp);
        w = (*dest).work.work_int;
        n = 0 as libc::c_int as size_t;
        while n < (*dest).size2 {
            *w.offset(n as isize) = *Cp.offset(n as isize);
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh31 = *w.offset(*Tj.offset(n as isize) as isize);
            let fresh32 = *fresh31;
            *fresh31 = *fresh31 + 1;
            let mut k: libc::c_int = fresh32;
            *((*dest).i).offset(k as isize) = *((*src).i).offset(n as isize);
            r = 0 as libc::c_int as size_t;
            while r < 2 as libc::c_int as libc::c_ulong {
                *((*dest).data)
                    .offset(
                        ((2 as libc::c_int * k) as libc::c_ulong).wrapping_add(r)
                            as isize,
                    ) = *((*src).data)
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(n)
                            .wrapping_add(r) as isize,
                    );
                r = r.wrapping_add(1);
                r;
            }
            n = n.wrapping_add(1);
            n;
        }
        (*dest).nz = (*src).nz;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_csc(
    mut dest: *mut gsl_spmatrix_complex,
    mut src: *const gsl_spmatrix_complex,
) -> libc::c_int {
    if !((*src).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"input matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if !((*dest).sptype == GSL_SPMATRIX_CSC as libc::c_int) {
        gsl_error(
            b"output matrix must be in CSC format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            38 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if (*src).size1 != (*dest).size1 || (*src).size2 != (*dest).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            42 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut Tj: *const libc::c_int = (*src).p;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut w: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_complex_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        Cp = (*dest).p;
        n = 0 as libc::c_int as size_t;
        while n < ((*dest).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            *Cp.offset(n as isize) = 0 as libc::c_int;
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh33 = *Cp.offset(*Tj.offset(n as isize) as isize);
            *fresh33 += 1;
            *fresh33;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_cumsum((*dest).size2, Cp);
        w = (*dest).work.work_int;
        n = 0 as libc::c_int as size_t;
        while n < (*dest).size2 {
            *w.offset(n as isize) = *Cp.offset(n as isize);
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh34 = *w.offset(*Tj.offset(n as isize) as isize);
            let fresh35 = *fresh34;
            *fresh34 = *fresh34 + 1;
            let mut k: libc::c_int = fresh35;
            *((*dest).i).offset(k as isize) = *((*src).i).offset(n as isize);
            r = 0 as libc::c_int as size_t;
            while r < 2 as libc::c_int as libc::c_ulong {
                *((*dest).data)
                    .offset(
                        ((2 as libc::c_int * k) as libc::c_ulong).wrapping_add(r)
                            as isize,
                    ) = *((*src).data)
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(n)
                            .wrapping_add(r) as isize,
                    );
                r = r.wrapping_add(1);
                r;
            }
            n = n.wrapping_add(1);
            n;
        }
        (*dest).nz = (*src).nz;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_ulong_csc(
    mut dest: *mut gsl_spmatrix_ulong,
    mut src: *const gsl_spmatrix_ulong,
) -> libc::c_int {
    if !((*src).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"input matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if !((*dest).sptype == GSL_SPMATRIX_CSC as libc::c_int) {
        gsl_error(
            b"output matrix must be in CSC format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            38 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if (*src).size1 != (*dest).size1 || (*src).size2 != (*dest).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            42 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut Tj: *const libc::c_int = (*src).p;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut w: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_ulong_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        Cp = (*dest).p;
        n = 0 as libc::c_int as size_t;
        while n < ((*dest).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            *Cp.offset(n as isize) = 0 as libc::c_int;
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh36 = *Cp.offset(*Tj.offset(n as isize) as isize);
            *fresh36 += 1;
            *fresh36;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_cumsum((*dest).size2, Cp);
        w = (*dest).work.work_int;
        n = 0 as libc::c_int as size_t;
        while n < (*dest).size2 {
            *w.offset(n as isize) = *Cp.offset(n as isize);
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh37 = *w.offset(*Tj.offset(n as isize) as isize);
            let fresh38 = *fresh37;
            *fresh37 = *fresh37 + 1;
            let mut k: libc::c_int = fresh38;
            *((*dest).i).offset(k as isize) = *((*src).i).offset(n as isize);
            r = 0 as libc::c_int as size_t;
            while r < 1 as libc::c_int as libc::c_ulong {
                *((*dest).data)
                    .offset(
                        ((1 as libc::c_int * k) as libc::c_ulong).wrapping_add(r)
                            as isize,
                    ) = *((*src).data)
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(n)
                            .wrapping_add(r) as isize,
                    );
                r = r.wrapping_add(1);
                r;
            }
            n = n.wrapping_add(1);
            n;
        }
        (*dest).nz = (*src).nz;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uint_csc(
    mut dest: *mut gsl_spmatrix_uint,
    mut src: *const gsl_spmatrix_uint,
) -> libc::c_int {
    if !((*src).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"input matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if !((*dest).sptype == GSL_SPMATRIX_CSC as libc::c_int) {
        gsl_error(
            b"output matrix must be in CSC format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            38 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if (*src).size1 != (*dest).size1 || (*src).size2 != (*dest).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            42 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut Tj: *const libc::c_int = (*src).p;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut w: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_uint_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        Cp = (*dest).p;
        n = 0 as libc::c_int as size_t;
        while n < ((*dest).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            *Cp.offset(n as isize) = 0 as libc::c_int;
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh39 = *Cp.offset(*Tj.offset(n as isize) as isize);
            *fresh39 += 1;
            *fresh39;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_cumsum((*dest).size2, Cp);
        w = (*dest).work.work_int;
        n = 0 as libc::c_int as size_t;
        while n < (*dest).size2 {
            *w.offset(n as isize) = *Cp.offset(n as isize);
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh40 = *w.offset(*Tj.offset(n as isize) as isize);
            let fresh41 = *fresh40;
            *fresh40 = *fresh40 + 1;
            let mut k: libc::c_int = fresh41;
            *((*dest).i).offset(k as isize) = *((*src).i).offset(n as isize);
            r = 0 as libc::c_int as size_t;
            while r < 1 as libc::c_int as libc::c_ulong {
                *((*dest).data)
                    .offset(
                        ((1 as libc::c_int * k) as libc::c_ulong).wrapping_add(r)
                            as isize,
                    ) = *((*src).data)
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(n)
                            .wrapping_add(r) as isize,
                    );
                r = r.wrapping_add(1);
                r;
            }
            n = n.wrapping_add(1);
            n;
        }
        (*dest).nz = (*src).nz;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_compcol(
    mut src: *const gsl_spmatrix_complex,
) -> *mut gsl_spmatrix_complex {
    return gsl_spmatrix_complex_ccs(src);
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_compcol(
    mut src: *const gsl_spmatrix_long_double,
) -> *mut gsl_spmatrix_long_double {
    return gsl_spmatrix_long_double_ccs(src);
}
pub unsafe extern "C" fn gsl_spmatrix_ulong_compcol(
    mut src: *const gsl_spmatrix_ulong,
) -> *mut gsl_spmatrix_ulong {
    return gsl_spmatrix_ulong_ccs(src);
}
pub unsafe extern "C" fn gsl_spmatrix_long_compcol(
    mut src: *const gsl_spmatrix_long,
) -> *mut gsl_spmatrix_long {
    return gsl_spmatrix_long_ccs(src);
}
pub unsafe extern "C" fn gsl_spmatrix_float_compcol(
    mut src: *const gsl_spmatrix_float,
) -> *mut gsl_spmatrix_float {
    return gsl_spmatrix_float_ccs(src);
}
pub unsafe extern "C" fn gsl_spmatrix_short_compcol(
    mut src: *const gsl_spmatrix_short,
) -> *mut gsl_spmatrix_short {
    return gsl_spmatrix_short_ccs(src);
}
pub unsafe extern "C" fn gsl_spmatrix_complex_float_compcol(
    mut src: *const gsl_spmatrix_complex_float,
) -> *mut gsl_spmatrix_complex_float {
    return gsl_spmatrix_complex_float_ccs(src);
}
pub unsafe extern "C" fn gsl_spmatrix_uchar_compcol(
    mut src: *const gsl_spmatrix_uchar,
) -> *mut gsl_spmatrix_uchar {
    return gsl_spmatrix_uchar_ccs(src);
}
pub unsafe extern "C" fn gsl_spmatrix_uint_compcol(
    mut src: *const gsl_spmatrix_uint,
) -> *mut gsl_spmatrix_uint {
    return gsl_spmatrix_uint_ccs(src);
}
pub unsafe extern "C" fn gsl_spmatrix_complex_long_double_compcol(
    mut src: *const gsl_spmatrix_complex_long_double,
) -> *mut gsl_spmatrix_complex_long_double {
    return gsl_spmatrix_complex_long_double_ccs(src);
}
pub unsafe extern "C" fn gsl_spmatrix_char_compcol(
    mut src: *const gsl_spmatrix_char,
) -> *mut gsl_spmatrix_char {
    return gsl_spmatrix_char_ccs(src);
}
pub unsafe extern "C" fn gsl_spmatrix_int_compcol(
    mut src: *const gsl_spmatrix_int,
) -> *mut gsl_spmatrix_int {
    return gsl_spmatrix_int_ccs(src);
}
pub unsafe extern "C" fn gsl_spmatrix_ushort_compcol(
    mut src: *const gsl_spmatrix_ushort,
) -> *mut gsl_spmatrix_ushort {
    return gsl_spmatrix_ushort_ccs(src);
}
pub unsafe extern "C" fn gsl_spmatrix_compcol(
    mut src: *const gsl_spmatrix,
) -> *mut gsl_spmatrix {
    return gsl_spmatrix_ccs(src);
}
pub unsafe extern "C" fn gsl_spmatrix_long_ccs(
    mut src: *const gsl_spmatrix_long,
) -> *mut gsl_spmatrix_long {
    let mut dest: *mut gsl_spmatrix_long = gsl_spmatrix_long_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        GSL_SPMATRIX_CSC as libc::c_int,
    );
    gsl_spmatrix_long_csc(dest, src);
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_uint_ccs(
    mut src: *const gsl_spmatrix_uint,
) -> *mut gsl_spmatrix_uint {
    let mut dest: *mut gsl_spmatrix_uint = gsl_spmatrix_uint_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        GSL_SPMATRIX_CSC as libc::c_int,
    );
    gsl_spmatrix_uint_csc(dest, src);
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_ushort_ccs(
    mut src: *const gsl_spmatrix_ushort,
) -> *mut gsl_spmatrix_ushort {
    let mut dest: *mut gsl_spmatrix_ushort = gsl_spmatrix_ushort_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        GSL_SPMATRIX_CSC as libc::c_int,
    );
    gsl_spmatrix_ushort_csc(dest, src);
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_ulong_ccs(
    mut src: *const gsl_spmatrix_ulong,
) -> *mut gsl_spmatrix_ulong {
    let mut dest: *mut gsl_spmatrix_ulong = gsl_spmatrix_ulong_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        GSL_SPMATRIX_CSC as libc::c_int,
    );
    gsl_spmatrix_ulong_csc(dest, src);
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_float_ccs(
    mut src: *const gsl_spmatrix_complex_float,
) -> *mut gsl_spmatrix_complex_float {
    let mut dest: *mut gsl_spmatrix_complex_float = gsl_spmatrix_complex_float_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        GSL_SPMATRIX_CSC as libc::c_int,
    );
    gsl_spmatrix_complex_float_csc(dest, src);
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_short_ccs(
    mut src: *const gsl_spmatrix_short,
) -> *mut gsl_spmatrix_short {
    let mut dest: *mut gsl_spmatrix_short = gsl_spmatrix_short_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        GSL_SPMATRIX_CSC as libc::c_int,
    );
    gsl_spmatrix_short_csc(dest, src);
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_float_ccs(
    mut src: *const gsl_spmatrix_float,
) -> *mut gsl_spmatrix_float {
    let mut dest: *mut gsl_spmatrix_float = gsl_spmatrix_float_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        GSL_SPMATRIX_CSC as libc::c_int,
    );
    gsl_spmatrix_float_csc(dest, src);
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_char_ccs(
    mut src: *const gsl_spmatrix_char,
) -> *mut gsl_spmatrix_char {
    let mut dest: *mut gsl_spmatrix_char = gsl_spmatrix_char_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        GSL_SPMATRIX_CSC as libc::c_int,
    );
    gsl_spmatrix_char_csc(dest, src);
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_ccs(
    mut src: *const gsl_spmatrix_complex,
) -> *mut gsl_spmatrix_complex {
    let mut dest: *mut gsl_spmatrix_complex = gsl_spmatrix_complex_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        GSL_SPMATRIX_CSC as libc::c_int,
    );
    gsl_spmatrix_complex_csc(dest, src);
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_long_double_ccs(
    mut src: *const gsl_spmatrix_complex_long_double,
) -> *mut gsl_spmatrix_complex_long_double {
    let mut dest: *mut gsl_spmatrix_complex_long_double = gsl_spmatrix_complex_long_double_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        GSL_SPMATRIX_CSC as libc::c_int,
    );
    gsl_spmatrix_complex_long_double_csc(dest, src);
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_int_ccs(
    mut src: *const gsl_spmatrix_int,
) -> *mut gsl_spmatrix_int {
    let mut dest: *mut gsl_spmatrix_int = gsl_spmatrix_int_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        GSL_SPMATRIX_CSC as libc::c_int,
    );
    gsl_spmatrix_int_csc(dest, src);
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_uchar_ccs(
    mut src: *const gsl_spmatrix_uchar,
) -> *mut gsl_spmatrix_uchar {
    let mut dest: *mut gsl_spmatrix_uchar = gsl_spmatrix_uchar_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        GSL_SPMATRIX_CSC as libc::c_int,
    );
    gsl_spmatrix_uchar_csc(dest, src);
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_ccs(
    mut src: *const gsl_spmatrix_long_double,
) -> *mut gsl_spmatrix_long_double {
    let mut dest: *mut gsl_spmatrix_long_double = gsl_spmatrix_long_double_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        GSL_SPMATRIX_CSC as libc::c_int,
    );
    gsl_spmatrix_long_double_csc(dest, src);
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_ccs(
    mut src: *const gsl_spmatrix,
) -> *mut gsl_spmatrix {
    let mut dest: *mut gsl_spmatrix = gsl_spmatrix_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        GSL_SPMATRIX_CSC as libc::c_int,
    );
    gsl_spmatrix_csc(dest, src);
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_int_csr(
    mut dest: *mut gsl_spmatrix_int,
    mut src: *const gsl_spmatrix_int,
) -> libc::c_int {
    if !((*src).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"input matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            127 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if !((*dest).sptype == GSL_SPMATRIX_CSR as libc::c_int) {
        gsl_error(
            b"output matrix must be in CSR format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*src).size1 != (*dest).size1 || (*src).size2 != (*dest).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            135 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut Ti: *const libc::c_int = (*src).i;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut w: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_int_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        Cp = (*dest).p;
        n = 0 as libc::c_int as size_t;
        while n < ((*dest).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            *Cp.offset(n as isize) = 0 as libc::c_int;
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh42 = *Cp.offset(*Ti.offset(n as isize) as isize);
            *fresh42 += 1;
            *fresh42;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_cumsum((*dest).size1, Cp);
        w = (*dest).work.work_int;
        n = 0 as libc::c_int as size_t;
        while n < (*dest).size1 {
            *w.offset(n as isize) = *Cp.offset(n as isize);
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh43 = *w.offset(*Ti.offset(n as isize) as isize);
            let fresh44 = *fresh43;
            *fresh43 = *fresh43 + 1;
            let mut k: libc::c_int = fresh44;
            *((*dest).i).offset(k as isize) = *((*src).p).offset(n as isize);
            r = 0 as libc::c_int as size_t;
            while r < 1 as libc::c_int as libc::c_ulong {
                *((*dest).data)
                    .offset(
                        ((1 as libc::c_int * k) as libc::c_ulong).wrapping_add(r)
                            as isize,
                    ) = *((*src).data)
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(n)
                            .wrapping_add(r) as isize,
                    );
                r = r.wrapping_add(1);
                r;
            }
            n = n.wrapping_add(1);
            n;
        }
        (*dest).nz = (*src).nz;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_long_double_csr(
    mut dest: *mut gsl_spmatrix_complex_long_double,
    mut src: *const gsl_spmatrix_complex_long_double,
) -> libc::c_int {
    if !((*src).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"input matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            127 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if !((*dest).sptype == GSL_SPMATRIX_CSR as libc::c_int) {
        gsl_error(
            b"output matrix must be in CSR format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*src).size1 != (*dest).size1 || (*src).size2 != (*dest).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            135 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut Ti: *const libc::c_int = (*src).i;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut w: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_complex_long_double_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        Cp = (*dest).p;
        n = 0 as libc::c_int as size_t;
        while n < ((*dest).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            *Cp.offset(n as isize) = 0 as libc::c_int;
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh45 = *Cp.offset(*Ti.offset(n as isize) as isize);
            *fresh45 += 1;
            *fresh45;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_cumsum((*dest).size1, Cp);
        w = (*dest).work.work_int;
        n = 0 as libc::c_int as size_t;
        while n < (*dest).size1 {
            *w.offset(n as isize) = *Cp.offset(n as isize);
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh46 = *w.offset(*Ti.offset(n as isize) as isize);
            let fresh47 = *fresh46;
            *fresh46 = *fresh46 + 1;
            let mut k: libc::c_int = fresh47;
            *((*dest).i).offset(k as isize) = *((*src).p).offset(n as isize);
            r = 0 as libc::c_int as size_t;
            while r < 2 as libc::c_int as libc::c_ulong {
                *((*dest).data)
                    .offset(
                        ((2 as libc::c_int * k) as libc::c_ulong).wrapping_add(r)
                            as isize,
                    ) = *((*src).data)
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(n)
                            .wrapping_add(r) as isize,
                    );
                r = r.wrapping_add(1);
                r;
            }
            n = n.wrapping_add(1);
            n;
        }
        (*dest).nz = (*src).nz;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_csr(
    mut dest: *mut gsl_spmatrix,
    mut src: *const gsl_spmatrix,
) -> libc::c_int {
    if !((*src).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"input matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            127 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if !((*dest).sptype == GSL_SPMATRIX_CSR as libc::c_int) {
        gsl_error(
            b"output matrix must be in CSR format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*src).size1 != (*dest).size1 || (*src).size2 != (*dest).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            135 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut Ti: *const libc::c_int = (*src).i;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut w: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        Cp = (*dest).p;
        n = 0 as libc::c_int as size_t;
        while n < ((*dest).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            *Cp.offset(n as isize) = 0 as libc::c_int;
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh48 = *Cp.offset(*Ti.offset(n as isize) as isize);
            *fresh48 += 1;
            *fresh48;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_cumsum((*dest).size1, Cp);
        w = (*dest).work.work_int;
        n = 0 as libc::c_int as size_t;
        while n < (*dest).size1 {
            *w.offset(n as isize) = *Cp.offset(n as isize);
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh49 = *w.offset(*Ti.offset(n as isize) as isize);
            let fresh50 = *fresh49;
            *fresh49 = *fresh49 + 1;
            let mut k: libc::c_int = fresh50;
            *((*dest).i).offset(k as isize) = *((*src).p).offset(n as isize);
            r = 0 as libc::c_int as size_t;
            while r < 1 as libc::c_int as libc::c_ulong {
                *((*dest).data)
                    .offset(
                        ((1 as libc::c_int * k) as libc::c_ulong).wrapping_add(r)
                            as isize,
                    ) = *((*src).data)
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(n)
                            .wrapping_add(r) as isize,
                    );
                r = r.wrapping_add(1);
                r;
            }
            n = n.wrapping_add(1);
            n;
        }
        (*dest).nz = (*src).nz;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_float_csr(
    mut dest: *mut gsl_spmatrix_float,
    mut src: *const gsl_spmatrix_float,
) -> libc::c_int {
    if !((*src).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"input matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            127 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if !((*dest).sptype == GSL_SPMATRIX_CSR as libc::c_int) {
        gsl_error(
            b"output matrix must be in CSR format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*src).size1 != (*dest).size1 || (*src).size2 != (*dest).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            135 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut Ti: *const libc::c_int = (*src).i;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut w: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_float_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        Cp = (*dest).p;
        n = 0 as libc::c_int as size_t;
        while n < ((*dest).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            *Cp.offset(n as isize) = 0 as libc::c_int;
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh51 = *Cp.offset(*Ti.offset(n as isize) as isize);
            *fresh51 += 1;
            *fresh51;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_cumsum((*dest).size1, Cp);
        w = (*dest).work.work_int;
        n = 0 as libc::c_int as size_t;
        while n < (*dest).size1 {
            *w.offset(n as isize) = *Cp.offset(n as isize);
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh52 = *w.offset(*Ti.offset(n as isize) as isize);
            let fresh53 = *fresh52;
            *fresh52 = *fresh52 + 1;
            let mut k: libc::c_int = fresh53;
            *((*dest).i).offset(k as isize) = *((*src).p).offset(n as isize);
            r = 0 as libc::c_int as size_t;
            while r < 1 as libc::c_int as libc::c_ulong {
                *((*dest).data)
                    .offset(
                        ((1 as libc::c_int * k) as libc::c_ulong).wrapping_add(r)
                            as isize,
                    ) = *((*src).data)
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(n)
                            .wrapping_add(r) as isize,
                    );
                r = r.wrapping_add(1);
                r;
            }
            n = n.wrapping_add(1);
            n;
        }
        (*dest).nz = (*src).nz;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_char_csr(
    mut dest: *mut gsl_spmatrix_char,
    mut src: *const gsl_spmatrix_char,
) -> libc::c_int {
    if !((*src).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"input matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            127 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if !((*dest).sptype == GSL_SPMATRIX_CSR as libc::c_int) {
        gsl_error(
            b"output matrix must be in CSR format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*src).size1 != (*dest).size1 || (*src).size2 != (*dest).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            135 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut Ti: *const libc::c_int = (*src).i;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut w: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_char_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        Cp = (*dest).p;
        n = 0 as libc::c_int as size_t;
        while n < ((*dest).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            *Cp.offset(n as isize) = 0 as libc::c_int;
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh54 = *Cp.offset(*Ti.offset(n as isize) as isize);
            *fresh54 += 1;
            *fresh54;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_cumsum((*dest).size1, Cp);
        w = (*dest).work.work_int;
        n = 0 as libc::c_int as size_t;
        while n < (*dest).size1 {
            *w.offset(n as isize) = *Cp.offset(n as isize);
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh55 = *w.offset(*Ti.offset(n as isize) as isize);
            let fresh56 = *fresh55;
            *fresh55 = *fresh55 + 1;
            let mut k: libc::c_int = fresh56;
            *((*dest).i).offset(k as isize) = *((*src).p).offset(n as isize);
            r = 0 as libc::c_int as size_t;
            while r < 1 as libc::c_int as libc::c_ulong {
                *((*dest).data)
                    .offset(
                        ((1 as libc::c_int * k) as libc::c_ulong).wrapping_add(r)
                            as isize,
                    ) = *((*src).data)
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(n)
                            .wrapping_add(r) as isize,
                    );
                r = r.wrapping_add(1);
                r;
            }
            n = n.wrapping_add(1);
            n;
        }
        (*dest).nz = (*src).nz;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_short_csr(
    mut dest: *mut gsl_spmatrix_short,
    mut src: *const gsl_spmatrix_short,
) -> libc::c_int {
    if !((*src).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"input matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            127 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if !((*dest).sptype == GSL_SPMATRIX_CSR as libc::c_int) {
        gsl_error(
            b"output matrix must be in CSR format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*src).size1 != (*dest).size1 || (*src).size2 != (*dest).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            135 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut Ti: *const libc::c_int = (*src).i;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut w: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_short_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        Cp = (*dest).p;
        n = 0 as libc::c_int as size_t;
        while n < ((*dest).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            *Cp.offset(n as isize) = 0 as libc::c_int;
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh57 = *Cp.offset(*Ti.offset(n as isize) as isize);
            *fresh57 += 1;
            *fresh57;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_cumsum((*dest).size1, Cp);
        w = (*dest).work.work_int;
        n = 0 as libc::c_int as size_t;
        while n < (*dest).size1 {
            *w.offset(n as isize) = *Cp.offset(n as isize);
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh58 = *w.offset(*Ti.offset(n as isize) as isize);
            let fresh59 = *fresh58;
            *fresh58 = *fresh58 + 1;
            let mut k: libc::c_int = fresh59;
            *((*dest).i).offset(k as isize) = *((*src).p).offset(n as isize);
            r = 0 as libc::c_int as size_t;
            while r < 1 as libc::c_int as libc::c_ulong {
                *((*dest).data)
                    .offset(
                        ((1 as libc::c_int * k) as libc::c_ulong).wrapping_add(r)
                            as isize,
                    ) = *((*src).data)
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(n)
                            .wrapping_add(r) as isize,
                    );
                r = r.wrapping_add(1);
                r;
            }
            n = n.wrapping_add(1);
            n;
        }
        (*dest).nz = (*src).nz;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_ushort_csr(
    mut dest: *mut gsl_spmatrix_ushort,
    mut src: *const gsl_spmatrix_ushort,
) -> libc::c_int {
    if !((*src).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"input matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            127 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if !((*dest).sptype == GSL_SPMATRIX_CSR as libc::c_int) {
        gsl_error(
            b"output matrix must be in CSR format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*src).size1 != (*dest).size1 || (*src).size2 != (*dest).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            135 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut Ti: *const libc::c_int = (*src).i;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut w: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_ushort_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        Cp = (*dest).p;
        n = 0 as libc::c_int as size_t;
        while n < ((*dest).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            *Cp.offset(n as isize) = 0 as libc::c_int;
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh60 = *Cp.offset(*Ti.offset(n as isize) as isize);
            *fresh60 += 1;
            *fresh60;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_cumsum((*dest).size1, Cp);
        w = (*dest).work.work_int;
        n = 0 as libc::c_int as size_t;
        while n < (*dest).size1 {
            *w.offset(n as isize) = *Cp.offset(n as isize);
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh61 = *w.offset(*Ti.offset(n as isize) as isize);
            let fresh62 = *fresh61;
            *fresh61 = *fresh61 + 1;
            let mut k: libc::c_int = fresh62;
            *((*dest).i).offset(k as isize) = *((*src).p).offset(n as isize);
            r = 0 as libc::c_int as size_t;
            while r < 1 as libc::c_int as libc::c_ulong {
                *((*dest).data)
                    .offset(
                        ((1 as libc::c_int * k) as libc::c_ulong).wrapping_add(r)
                            as isize,
                    ) = *((*src).data)
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(n)
                            .wrapping_add(r) as isize,
                    );
                r = r.wrapping_add(1);
                r;
            }
            n = n.wrapping_add(1);
            n;
        }
        (*dest).nz = (*src).nz;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uchar_csr(
    mut dest: *mut gsl_spmatrix_uchar,
    mut src: *const gsl_spmatrix_uchar,
) -> libc::c_int {
    if !((*src).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"input matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            127 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if !((*dest).sptype == GSL_SPMATRIX_CSR as libc::c_int) {
        gsl_error(
            b"output matrix must be in CSR format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*src).size1 != (*dest).size1 || (*src).size2 != (*dest).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            135 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut Ti: *const libc::c_int = (*src).i;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut w: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_uchar_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        Cp = (*dest).p;
        n = 0 as libc::c_int as size_t;
        while n < ((*dest).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            *Cp.offset(n as isize) = 0 as libc::c_int;
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh63 = *Cp.offset(*Ti.offset(n as isize) as isize);
            *fresh63 += 1;
            *fresh63;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_cumsum((*dest).size1, Cp);
        w = (*dest).work.work_int;
        n = 0 as libc::c_int as size_t;
        while n < (*dest).size1 {
            *w.offset(n as isize) = *Cp.offset(n as isize);
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh64 = *w.offset(*Ti.offset(n as isize) as isize);
            let fresh65 = *fresh64;
            *fresh64 = *fresh64 + 1;
            let mut k: libc::c_int = fresh65;
            *((*dest).i).offset(k as isize) = *((*src).p).offset(n as isize);
            r = 0 as libc::c_int as size_t;
            while r < 1 as libc::c_int as libc::c_ulong {
                *((*dest).data)
                    .offset(
                        ((1 as libc::c_int * k) as libc::c_ulong).wrapping_add(r)
                            as isize,
                    ) = *((*src).data)
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(n)
                            .wrapping_add(r) as isize,
                    );
                r = r.wrapping_add(1);
                r;
            }
            n = n.wrapping_add(1);
            n;
        }
        (*dest).nz = (*src).nz;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_float_csr(
    mut dest: *mut gsl_spmatrix_complex_float,
    mut src: *const gsl_spmatrix_complex_float,
) -> libc::c_int {
    if !((*src).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"input matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            127 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if !((*dest).sptype == GSL_SPMATRIX_CSR as libc::c_int) {
        gsl_error(
            b"output matrix must be in CSR format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*src).size1 != (*dest).size1 || (*src).size2 != (*dest).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            135 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut Ti: *const libc::c_int = (*src).i;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut w: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_complex_float_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        Cp = (*dest).p;
        n = 0 as libc::c_int as size_t;
        while n < ((*dest).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            *Cp.offset(n as isize) = 0 as libc::c_int;
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh66 = *Cp.offset(*Ti.offset(n as isize) as isize);
            *fresh66 += 1;
            *fresh66;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_cumsum((*dest).size1, Cp);
        w = (*dest).work.work_int;
        n = 0 as libc::c_int as size_t;
        while n < (*dest).size1 {
            *w.offset(n as isize) = *Cp.offset(n as isize);
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh67 = *w.offset(*Ti.offset(n as isize) as isize);
            let fresh68 = *fresh67;
            *fresh67 = *fresh67 + 1;
            let mut k: libc::c_int = fresh68;
            *((*dest).i).offset(k as isize) = *((*src).p).offset(n as isize);
            r = 0 as libc::c_int as size_t;
            while r < 2 as libc::c_int as libc::c_ulong {
                *((*dest).data)
                    .offset(
                        ((2 as libc::c_int * k) as libc::c_ulong).wrapping_add(r)
                            as isize,
                    ) = *((*src).data)
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(n)
                            .wrapping_add(r) as isize,
                    );
                r = r.wrapping_add(1);
                r;
            }
            n = n.wrapping_add(1);
            n;
        }
        (*dest).nz = (*src).nz;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uint_csr(
    mut dest: *mut gsl_spmatrix_uint,
    mut src: *const gsl_spmatrix_uint,
) -> libc::c_int {
    if !((*src).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"input matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            127 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if !((*dest).sptype == GSL_SPMATRIX_CSR as libc::c_int) {
        gsl_error(
            b"output matrix must be in CSR format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*src).size1 != (*dest).size1 || (*src).size2 != (*dest).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            135 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut Ti: *const libc::c_int = (*src).i;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut w: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_uint_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        Cp = (*dest).p;
        n = 0 as libc::c_int as size_t;
        while n < ((*dest).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            *Cp.offset(n as isize) = 0 as libc::c_int;
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh69 = *Cp.offset(*Ti.offset(n as isize) as isize);
            *fresh69 += 1;
            *fresh69;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_cumsum((*dest).size1, Cp);
        w = (*dest).work.work_int;
        n = 0 as libc::c_int as size_t;
        while n < (*dest).size1 {
            *w.offset(n as isize) = *Cp.offset(n as isize);
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh70 = *w.offset(*Ti.offset(n as isize) as isize);
            let fresh71 = *fresh70;
            *fresh70 = *fresh70 + 1;
            let mut k: libc::c_int = fresh71;
            *((*dest).i).offset(k as isize) = *((*src).p).offset(n as isize);
            r = 0 as libc::c_int as size_t;
            while r < 1 as libc::c_int as libc::c_ulong {
                *((*dest).data)
                    .offset(
                        ((1 as libc::c_int * k) as libc::c_ulong).wrapping_add(r)
                            as isize,
                    ) = *((*src).data)
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(n)
                            .wrapping_add(r) as isize,
                    );
                r = r.wrapping_add(1);
                r;
            }
            n = n.wrapping_add(1);
            n;
        }
        (*dest).nz = (*src).nz;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_ulong_csr(
    mut dest: *mut gsl_spmatrix_ulong,
    mut src: *const gsl_spmatrix_ulong,
) -> libc::c_int {
    if !((*src).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"input matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            127 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if !((*dest).sptype == GSL_SPMATRIX_CSR as libc::c_int) {
        gsl_error(
            b"output matrix must be in CSR format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*src).size1 != (*dest).size1 || (*src).size2 != (*dest).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            135 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut Ti: *const libc::c_int = (*src).i;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut w: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_ulong_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        Cp = (*dest).p;
        n = 0 as libc::c_int as size_t;
        while n < ((*dest).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            *Cp.offset(n as isize) = 0 as libc::c_int;
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh72 = *Cp.offset(*Ti.offset(n as isize) as isize);
            *fresh72 += 1;
            *fresh72;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_cumsum((*dest).size1, Cp);
        w = (*dest).work.work_int;
        n = 0 as libc::c_int as size_t;
        while n < (*dest).size1 {
            *w.offset(n as isize) = *Cp.offset(n as isize);
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh73 = *w.offset(*Ti.offset(n as isize) as isize);
            let fresh74 = *fresh73;
            *fresh73 = *fresh73 + 1;
            let mut k: libc::c_int = fresh74;
            *((*dest).i).offset(k as isize) = *((*src).p).offset(n as isize);
            r = 0 as libc::c_int as size_t;
            while r < 1 as libc::c_int as libc::c_ulong {
                *((*dest).data)
                    .offset(
                        ((1 as libc::c_int * k) as libc::c_ulong).wrapping_add(r)
                            as isize,
                    ) = *((*src).data)
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(n)
                            .wrapping_add(r) as isize,
                    );
                r = r.wrapping_add(1);
                r;
            }
            n = n.wrapping_add(1);
            n;
        }
        (*dest).nz = (*src).nz;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_csr(
    mut dest: *mut gsl_spmatrix_complex,
    mut src: *const gsl_spmatrix_complex,
) -> libc::c_int {
    if !((*src).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"input matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            127 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if !((*dest).sptype == GSL_SPMATRIX_CSR as libc::c_int) {
        gsl_error(
            b"output matrix must be in CSR format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*src).size1 != (*dest).size1 || (*src).size2 != (*dest).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            135 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut Ti: *const libc::c_int = (*src).i;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut w: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_complex_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        Cp = (*dest).p;
        n = 0 as libc::c_int as size_t;
        while n < ((*dest).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            *Cp.offset(n as isize) = 0 as libc::c_int;
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh75 = *Cp.offset(*Ti.offset(n as isize) as isize);
            *fresh75 += 1;
            *fresh75;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_cumsum((*dest).size1, Cp);
        w = (*dest).work.work_int;
        n = 0 as libc::c_int as size_t;
        while n < (*dest).size1 {
            *w.offset(n as isize) = *Cp.offset(n as isize);
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh76 = *w.offset(*Ti.offset(n as isize) as isize);
            let fresh77 = *fresh76;
            *fresh76 = *fresh76 + 1;
            let mut k: libc::c_int = fresh77;
            *((*dest).i).offset(k as isize) = *((*src).p).offset(n as isize);
            r = 0 as libc::c_int as size_t;
            while r < 2 as libc::c_int as libc::c_ulong {
                *((*dest).data)
                    .offset(
                        ((2 as libc::c_int * k) as libc::c_ulong).wrapping_add(r)
                            as isize,
                    ) = *((*src).data)
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(n)
                            .wrapping_add(r) as isize,
                    );
                r = r.wrapping_add(1);
                r;
            }
            n = n.wrapping_add(1);
            n;
        }
        (*dest).nz = (*src).nz;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_long_csr(
    mut dest: *mut gsl_spmatrix_long,
    mut src: *const gsl_spmatrix_long,
) -> libc::c_int {
    if !((*src).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"input matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            127 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if !((*dest).sptype == GSL_SPMATRIX_CSR as libc::c_int) {
        gsl_error(
            b"output matrix must be in CSR format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*src).size1 != (*dest).size1 || (*src).size2 != (*dest).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            135 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut Ti: *const libc::c_int = (*src).i;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut w: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_long_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        Cp = (*dest).p;
        n = 0 as libc::c_int as size_t;
        while n < ((*dest).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            *Cp.offset(n as isize) = 0 as libc::c_int;
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh78 = *Cp.offset(*Ti.offset(n as isize) as isize);
            *fresh78 += 1;
            *fresh78;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_cumsum((*dest).size1, Cp);
        w = (*dest).work.work_int;
        n = 0 as libc::c_int as size_t;
        while n < (*dest).size1 {
            *w.offset(n as isize) = *Cp.offset(n as isize);
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh79 = *w.offset(*Ti.offset(n as isize) as isize);
            let fresh80 = *fresh79;
            *fresh79 = *fresh79 + 1;
            let mut k: libc::c_int = fresh80;
            *((*dest).i).offset(k as isize) = *((*src).p).offset(n as isize);
            r = 0 as libc::c_int as size_t;
            while r < 1 as libc::c_int as libc::c_ulong {
                *((*dest).data)
                    .offset(
                        ((1 as libc::c_int * k) as libc::c_ulong).wrapping_add(r)
                            as isize,
                    ) = *((*src).data)
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(n)
                            .wrapping_add(r) as isize,
                    );
                r = r.wrapping_add(1);
                r;
            }
            n = n.wrapping_add(1);
            n;
        }
        (*dest).nz = (*src).nz;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_csr(
    mut dest: *mut gsl_spmatrix_long_double,
    mut src: *const gsl_spmatrix_long_double,
) -> libc::c_int {
    if !((*src).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"input matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            127 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if !((*dest).sptype == GSL_SPMATRIX_CSR as libc::c_int) {
        gsl_error(
            b"output matrix must be in CSR format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*src).size1 != (*dest).size1 || (*src).size2 != (*dest).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            135 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut Ti: *const libc::c_int = (*src).i;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut w: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_long_double_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        Cp = (*dest).p;
        n = 0 as libc::c_int as size_t;
        while n < ((*dest).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            *Cp.offset(n as isize) = 0 as libc::c_int;
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh81 = *Cp.offset(*Ti.offset(n as isize) as isize);
            *fresh81 += 1;
            *fresh81;
            n = n.wrapping_add(1);
            n;
        }
        gsl_spmatrix_cumsum((*dest).size1, Cp);
        w = (*dest).work.work_int;
        n = 0 as libc::c_int as size_t;
        while n < (*dest).size1 {
            *w.offset(n as isize) = *Cp.offset(n as isize);
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as size_t;
        while n < (*src).nz {
            let ref mut fresh82 = *w.offset(*Ti.offset(n as isize) as isize);
            let fresh83 = *fresh82;
            *fresh82 = *fresh82 + 1;
            let mut k: libc::c_int = fresh83;
            *((*dest).i).offset(k as isize) = *((*src).p).offset(n as isize);
            r = 0 as libc::c_int as size_t;
            while r < 1 as libc::c_int as libc::c_ulong {
                *((*dest).data)
                    .offset(
                        ((1 as libc::c_int * k) as libc::c_ulong).wrapping_add(r)
                            as isize,
                    ) = *((*src).data)
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(n)
                            .wrapping_add(r) as isize,
                    );
                r = r.wrapping_add(1);
                r;
            }
            n = n.wrapping_add(1);
            n;
        }
        (*dest).nz = (*src).nz;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_long_double_crs(
    mut src: *const gsl_spmatrix_complex_long_double,
) -> *mut gsl_spmatrix_complex_long_double {
    let mut dest: *mut gsl_spmatrix_complex_long_double = gsl_spmatrix_complex_long_double_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        GSL_SPMATRIX_CSR as libc::c_int,
    );
    gsl_spmatrix_complex_long_double_csr(dest, src);
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_float_crs(
    mut src: *const gsl_spmatrix_float,
) -> *mut gsl_spmatrix_float {
    let mut dest: *mut gsl_spmatrix_float = gsl_spmatrix_float_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        GSL_SPMATRIX_CSR as libc::c_int,
    );
    gsl_spmatrix_float_csr(dest, src);
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_float_crs(
    mut src: *const gsl_spmatrix_complex_float,
) -> *mut gsl_spmatrix_complex_float {
    let mut dest: *mut gsl_spmatrix_complex_float = gsl_spmatrix_complex_float_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        GSL_SPMATRIX_CSR as libc::c_int,
    );
    gsl_spmatrix_complex_float_csr(dest, src);
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_ulong_crs(
    mut src: *const gsl_spmatrix_ulong,
) -> *mut gsl_spmatrix_ulong {
    let mut dest: *mut gsl_spmatrix_ulong = gsl_spmatrix_ulong_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        GSL_SPMATRIX_CSR as libc::c_int,
    );
    gsl_spmatrix_ulong_csr(dest, src);
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_short_crs(
    mut src: *const gsl_spmatrix_short,
) -> *mut gsl_spmatrix_short {
    let mut dest: *mut gsl_spmatrix_short = gsl_spmatrix_short_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        GSL_SPMATRIX_CSR as libc::c_int,
    );
    gsl_spmatrix_short_csr(dest, src);
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_char_crs(
    mut src: *const gsl_spmatrix_char,
) -> *mut gsl_spmatrix_char {
    let mut dest: *mut gsl_spmatrix_char = gsl_spmatrix_char_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        GSL_SPMATRIX_CSR as libc::c_int,
    );
    gsl_spmatrix_char_csr(dest, src);
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_crs(
    mut src: *const gsl_spmatrix_long_double,
) -> *mut gsl_spmatrix_long_double {
    let mut dest: *mut gsl_spmatrix_long_double = gsl_spmatrix_long_double_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        GSL_SPMATRIX_CSR as libc::c_int,
    );
    gsl_spmatrix_long_double_csr(dest, src);
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_long_crs(
    mut src: *const gsl_spmatrix_long,
) -> *mut gsl_spmatrix_long {
    let mut dest: *mut gsl_spmatrix_long = gsl_spmatrix_long_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        GSL_SPMATRIX_CSR as libc::c_int,
    );
    gsl_spmatrix_long_csr(dest, src);
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_ushort_crs(
    mut src: *const gsl_spmatrix_ushort,
) -> *mut gsl_spmatrix_ushort {
    let mut dest: *mut gsl_spmatrix_ushort = gsl_spmatrix_ushort_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        GSL_SPMATRIX_CSR as libc::c_int,
    );
    gsl_spmatrix_ushort_csr(dest, src);
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_int_crs(
    mut src: *const gsl_spmatrix_int,
) -> *mut gsl_spmatrix_int {
    let mut dest: *mut gsl_spmatrix_int = gsl_spmatrix_int_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        GSL_SPMATRIX_CSR as libc::c_int,
    );
    gsl_spmatrix_int_csr(dest, src);
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_uint_crs(
    mut src: *const gsl_spmatrix_uint,
) -> *mut gsl_spmatrix_uint {
    let mut dest: *mut gsl_spmatrix_uint = gsl_spmatrix_uint_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        GSL_SPMATRIX_CSR as libc::c_int,
    );
    gsl_spmatrix_uint_csr(dest, src);
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_crs(
    mut src: *const gsl_spmatrix_complex,
) -> *mut gsl_spmatrix_complex {
    let mut dest: *mut gsl_spmatrix_complex = gsl_spmatrix_complex_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        GSL_SPMATRIX_CSR as libc::c_int,
    );
    gsl_spmatrix_complex_csr(dest, src);
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_uchar_crs(
    mut src: *const gsl_spmatrix_uchar,
) -> *mut gsl_spmatrix_uchar {
    let mut dest: *mut gsl_spmatrix_uchar = gsl_spmatrix_uchar_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        GSL_SPMATRIX_CSR as libc::c_int,
    );
    gsl_spmatrix_uchar_csr(dest, src);
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_crs(
    mut src: *const gsl_spmatrix,
) -> *mut gsl_spmatrix {
    let mut dest: *mut gsl_spmatrix = gsl_spmatrix_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        GSL_SPMATRIX_CSR as libc::c_int,
    );
    gsl_spmatrix_csr(dest, src);
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_uint_compress(
    mut src: *const gsl_spmatrix_uint,
    sptype: libc::c_int,
) -> *mut gsl_spmatrix_uint {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut dest: *mut gsl_spmatrix_uint = gsl_spmatrix_uint_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        sptype,
    );
    if dest.is_null() {
        return 0 as *mut gsl_spmatrix_uint;
    }
    if sptype == GSL_SPMATRIX_CSC as libc::c_int {
        status = gsl_spmatrix_uint_csc(dest, src);
    } else if sptype == GSL_SPMATRIX_CSR as libc::c_int {
        status = gsl_spmatrix_uint_csr(dest, src);
    } else if sptype == GSL_SPMATRIX_COO as libc::c_int {
        status = gsl_spmatrix_uint_memcpy(dest, src);
    } else {
        status = GSL_EINVAL as libc::c_int;
        gsl_error(
            b"unknown sparse matrix format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            223 as libc::c_int,
            status,
        );
        return 0 as *mut gsl_spmatrix_uint;
    }
    if status != GSL_SUCCESS as libc::c_int {
        gsl_spmatrix_uint_free(dest);
        return 0 as *mut gsl_spmatrix_uint;
    }
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_ulong_compress(
    mut src: *const gsl_spmatrix_ulong,
    sptype: libc::c_int,
) -> *mut gsl_spmatrix_ulong {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut dest: *mut gsl_spmatrix_ulong = gsl_spmatrix_ulong_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        sptype,
    );
    if dest.is_null() {
        return 0 as *mut gsl_spmatrix_ulong;
    }
    if sptype == GSL_SPMATRIX_CSC as libc::c_int {
        status = gsl_spmatrix_ulong_csc(dest, src);
    } else if sptype == GSL_SPMATRIX_CSR as libc::c_int {
        status = gsl_spmatrix_ulong_csr(dest, src);
    } else if sptype == GSL_SPMATRIX_COO as libc::c_int {
        status = gsl_spmatrix_ulong_memcpy(dest, src);
    } else {
        status = GSL_EINVAL as libc::c_int;
        gsl_error(
            b"unknown sparse matrix format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            223 as libc::c_int,
            status,
        );
        return 0 as *mut gsl_spmatrix_ulong;
    }
    if status != GSL_SUCCESS as libc::c_int {
        gsl_spmatrix_ulong_free(dest);
        return 0 as *mut gsl_spmatrix_ulong;
    }
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_float_compress(
    mut src: *const gsl_spmatrix_complex_float,
    sptype: libc::c_int,
) -> *mut gsl_spmatrix_complex_float {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut dest: *mut gsl_spmatrix_complex_float = gsl_spmatrix_complex_float_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        sptype,
    );
    if dest.is_null() {
        return 0 as *mut gsl_spmatrix_complex_float;
    }
    if sptype == GSL_SPMATRIX_CSC as libc::c_int {
        status = gsl_spmatrix_complex_float_csc(dest, src);
    } else if sptype == GSL_SPMATRIX_CSR as libc::c_int {
        status = gsl_spmatrix_complex_float_csr(dest, src);
    } else if sptype == GSL_SPMATRIX_COO as libc::c_int {
        status = gsl_spmatrix_complex_float_memcpy(dest, src);
    } else {
        status = GSL_EINVAL as libc::c_int;
        gsl_error(
            b"unknown sparse matrix format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            223 as libc::c_int,
            status,
        );
        return 0 as *mut gsl_spmatrix_complex_float;
    }
    if status != GSL_SUCCESS as libc::c_int {
        gsl_spmatrix_complex_float_free(dest);
        return 0 as *mut gsl_spmatrix_complex_float;
    }
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_float_compress(
    mut src: *const gsl_spmatrix_float,
    sptype: libc::c_int,
) -> *mut gsl_spmatrix_float {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut dest: *mut gsl_spmatrix_float = gsl_spmatrix_float_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        sptype,
    );
    if dest.is_null() {
        return 0 as *mut gsl_spmatrix_float;
    }
    if sptype == GSL_SPMATRIX_CSC as libc::c_int {
        status = gsl_spmatrix_float_csc(dest, src);
    } else if sptype == GSL_SPMATRIX_CSR as libc::c_int {
        status = gsl_spmatrix_float_csr(dest, src);
    } else if sptype == GSL_SPMATRIX_COO as libc::c_int {
        status = gsl_spmatrix_float_memcpy(dest, src);
    } else {
        status = GSL_EINVAL as libc::c_int;
        gsl_error(
            b"unknown sparse matrix format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            223 as libc::c_int,
            status,
        );
        return 0 as *mut gsl_spmatrix_float;
    }
    if status != GSL_SUCCESS as libc::c_int {
        gsl_spmatrix_float_free(dest);
        return 0 as *mut gsl_spmatrix_float;
    }
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_char_compress(
    mut src: *const gsl_spmatrix_char,
    sptype: libc::c_int,
) -> *mut gsl_spmatrix_char {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut dest: *mut gsl_spmatrix_char = gsl_spmatrix_char_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        sptype,
    );
    if dest.is_null() {
        return 0 as *mut gsl_spmatrix_char;
    }
    if sptype == GSL_SPMATRIX_CSC as libc::c_int {
        status = gsl_spmatrix_char_csc(dest, src);
    } else if sptype == GSL_SPMATRIX_CSR as libc::c_int {
        status = gsl_spmatrix_char_csr(dest, src);
    } else if sptype == GSL_SPMATRIX_COO as libc::c_int {
        status = gsl_spmatrix_char_memcpy(dest, src);
    } else {
        status = GSL_EINVAL as libc::c_int;
        gsl_error(
            b"unknown sparse matrix format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            223 as libc::c_int,
            status,
        );
        return 0 as *mut gsl_spmatrix_char;
    }
    if status != GSL_SUCCESS as libc::c_int {
        gsl_spmatrix_char_free(dest);
        return 0 as *mut gsl_spmatrix_char;
    }
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_int_compress(
    mut src: *const gsl_spmatrix_int,
    sptype: libc::c_int,
) -> *mut gsl_spmatrix_int {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut dest: *mut gsl_spmatrix_int = gsl_spmatrix_int_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        sptype,
    );
    if dest.is_null() {
        return 0 as *mut gsl_spmatrix_int;
    }
    if sptype == GSL_SPMATRIX_CSC as libc::c_int {
        status = gsl_spmatrix_int_csc(dest, src);
    } else if sptype == GSL_SPMATRIX_CSR as libc::c_int {
        status = gsl_spmatrix_int_csr(dest, src);
    } else if sptype == GSL_SPMATRIX_COO as libc::c_int {
        status = gsl_spmatrix_int_memcpy(dest, src);
    } else {
        status = GSL_EINVAL as libc::c_int;
        gsl_error(
            b"unknown sparse matrix format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            223 as libc::c_int,
            status,
        );
        return 0 as *mut gsl_spmatrix_int;
    }
    if status != GSL_SUCCESS as libc::c_int {
        gsl_spmatrix_int_free(dest);
        return 0 as *mut gsl_spmatrix_int;
    }
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_uchar_compress(
    mut src: *const gsl_spmatrix_uchar,
    sptype: libc::c_int,
) -> *mut gsl_spmatrix_uchar {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut dest: *mut gsl_spmatrix_uchar = gsl_spmatrix_uchar_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        sptype,
    );
    if dest.is_null() {
        return 0 as *mut gsl_spmatrix_uchar;
    }
    if sptype == GSL_SPMATRIX_CSC as libc::c_int {
        status = gsl_spmatrix_uchar_csc(dest, src);
    } else if sptype == GSL_SPMATRIX_CSR as libc::c_int {
        status = gsl_spmatrix_uchar_csr(dest, src);
    } else if sptype == GSL_SPMATRIX_COO as libc::c_int {
        status = gsl_spmatrix_uchar_memcpy(dest, src);
    } else {
        status = GSL_EINVAL as libc::c_int;
        gsl_error(
            b"unknown sparse matrix format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            223 as libc::c_int,
            status,
        );
        return 0 as *mut gsl_spmatrix_uchar;
    }
    if status != GSL_SUCCESS as libc::c_int {
        gsl_spmatrix_uchar_free(dest);
        return 0 as *mut gsl_spmatrix_uchar;
    }
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_compress(
    mut src: *const gsl_spmatrix_complex,
    sptype: libc::c_int,
) -> *mut gsl_spmatrix_complex {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut dest: *mut gsl_spmatrix_complex = gsl_spmatrix_complex_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        sptype,
    );
    if dest.is_null() {
        return 0 as *mut gsl_spmatrix_complex;
    }
    if sptype == GSL_SPMATRIX_CSC as libc::c_int {
        status = gsl_spmatrix_complex_csc(dest, src);
    } else if sptype == GSL_SPMATRIX_CSR as libc::c_int {
        status = gsl_spmatrix_complex_csr(dest, src);
    } else if sptype == GSL_SPMATRIX_COO as libc::c_int {
        status = gsl_spmatrix_complex_memcpy(dest, src);
    } else {
        status = GSL_EINVAL as libc::c_int;
        gsl_error(
            b"unknown sparse matrix format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            223 as libc::c_int,
            status,
        );
        return 0 as *mut gsl_spmatrix_complex;
    }
    if status != GSL_SUCCESS as libc::c_int {
        gsl_spmatrix_complex_free(dest);
        return 0 as *mut gsl_spmatrix_complex;
    }
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_long_double_compress(
    mut src: *const gsl_spmatrix_complex_long_double,
    sptype: libc::c_int,
) -> *mut gsl_spmatrix_complex_long_double {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut dest: *mut gsl_spmatrix_complex_long_double = gsl_spmatrix_complex_long_double_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        sptype,
    );
    if dest.is_null() {
        return 0 as *mut gsl_spmatrix_complex_long_double;
    }
    if sptype == GSL_SPMATRIX_CSC as libc::c_int {
        status = gsl_spmatrix_complex_long_double_csc(dest, src);
    } else if sptype == GSL_SPMATRIX_CSR as libc::c_int {
        status = gsl_spmatrix_complex_long_double_csr(dest, src);
    } else if sptype == GSL_SPMATRIX_COO as libc::c_int {
        status = gsl_spmatrix_complex_long_double_memcpy(dest, src);
    } else {
        status = GSL_EINVAL as libc::c_int;
        gsl_error(
            b"unknown sparse matrix format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            223 as libc::c_int,
            status,
        );
        return 0 as *mut gsl_spmatrix_complex_long_double;
    }
    if status != GSL_SUCCESS as libc::c_int {
        gsl_spmatrix_complex_long_double_free(dest);
        return 0 as *mut gsl_spmatrix_complex_long_double;
    }
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_short_compress(
    mut src: *const gsl_spmatrix_short,
    sptype: libc::c_int,
) -> *mut gsl_spmatrix_short {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut dest: *mut gsl_spmatrix_short = gsl_spmatrix_short_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        sptype,
    );
    if dest.is_null() {
        return 0 as *mut gsl_spmatrix_short;
    }
    if sptype == GSL_SPMATRIX_CSC as libc::c_int {
        status = gsl_spmatrix_short_csc(dest, src);
    } else if sptype == GSL_SPMATRIX_CSR as libc::c_int {
        status = gsl_spmatrix_short_csr(dest, src);
    } else if sptype == GSL_SPMATRIX_COO as libc::c_int {
        status = gsl_spmatrix_short_memcpy(dest, src);
    } else {
        status = GSL_EINVAL as libc::c_int;
        gsl_error(
            b"unknown sparse matrix format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            223 as libc::c_int,
            status,
        );
        return 0 as *mut gsl_spmatrix_short;
    }
    if status != GSL_SUCCESS as libc::c_int {
        gsl_spmatrix_short_free(dest);
        return 0 as *mut gsl_spmatrix_short;
    }
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_ushort_compress(
    mut src: *const gsl_spmatrix_ushort,
    sptype: libc::c_int,
) -> *mut gsl_spmatrix_ushort {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut dest: *mut gsl_spmatrix_ushort = gsl_spmatrix_ushort_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        sptype,
    );
    if dest.is_null() {
        return 0 as *mut gsl_spmatrix_ushort;
    }
    if sptype == GSL_SPMATRIX_CSC as libc::c_int {
        status = gsl_spmatrix_ushort_csc(dest, src);
    } else if sptype == GSL_SPMATRIX_CSR as libc::c_int {
        status = gsl_spmatrix_ushort_csr(dest, src);
    } else if sptype == GSL_SPMATRIX_COO as libc::c_int {
        status = gsl_spmatrix_ushort_memcpy(dest, src);
    } else {
        status = GSL_EINVAL as libc::c_int;
        gsl_error(
            b"unknown sparse matrix format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            223 as libc::c_int,
            status,
        );
        return 0 as *mut gsl_spmatrix_ushort;
    }
    if status != GSL_SUCCESS as libc::c_int {
        gsl_spmatrix_ushort_free(dest);
        return 0 as *mut gsl_spmatrix_ushort;
    }
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_compress(
    mut src: *const gsl_spmatrix_long_double,
    sptype: libc::c_int,
) -> *mut gsl_spmatrix_long_double {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut dest: *mut gsl_spmatrix_long_double = gsl_spmatrix_long_double_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        sptype,
    );
    if dest.is_null() {
        return 0 as *mut gsl_spmatrix_long_double;
    }
    if sptype == GSL_SPMATRIX_CSC as libc::c_int {
        status = gsl_spmatrix_long_double_csc(dest, src);
    } else if sptype == GSL_SPMATRIX_CSR as libc::c_int {
        status = gsl_spmatrix_long_double_csr(dest, src);
    } else if sptype == GSL_SPMATRIX_COO as libc::c_int {
        status = gsl_spmatrix_long_double_memcpy(dest, src);
    } else {
        status = GSL_EINVAL as libc::c_int;
        gsl_error(
            b"unknown sparse matrix format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            223 as libc::c_int,
            status,
        );
        return 0 as *mut gsl_spmatrix_long_double;
    }
    if status != GSL_SUCCESS as libc::c_int {
        gsl_spmatrix_long_double_free(dest);
        return 0 as *mut gsl_spmatrix_long_double;
    }
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_long_compress(
    mut src: *const gsl_spmatrix_long,
    sptype: libc::c_int,
) -> *mut gsl_spmatrix_long {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut dest: *mut gsl_spmatrix_long = gsl_spmatrix_long_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        sptype,
    );
    if dest.is_null() {
        return 0 as *mut gsl_spmatrix_long;
    }
    if sptype == GSL_SPMATRIX_CSC as libc::c_int {
        status = gsl_spmatrix_long_csc(dest, src);
    } else if sptype == GSL_SPMATRIX_CSR as libc::c_int {
        status = gsl_spmatrix_long_csr(dest, src);
    } else if sptype == GSL_SPMATRIX_COO as libc::c_int {
        status = gsl_spmatrix_long_memcpy(dest, src);
    } else {
        status = GSL_EINVAL as libc::c_int;
        gsl_error(
            b"unknown sparse matrix format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            223 as libc::c_int,
            status,
        );
        return 0 as *mut gsl_spmatrix_long;
    }
    if status != GSL_SUCCESS as libc::c_int {
        gsl_spmatrix_long_free(dest);
        return 0 as *mut gsl_spmatrix_long;
    }
    return dest;
}
pub unsafe extern "C" fn gsl_spmatrix_compress(
    mut src: *const gsl_spmatrix,
    sptype: libc::c_int,
) -> *mut gsl_spmatrix {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut dest: *mut gsl_spmatrix = gsl_spmatrix_alloc_nzmax(
        (*src).size1,
        (*src).size2,
        (*src).nz,
        sptype,
    );
    if dest.is_null() {
        return 0 as *mut gsl_spmatrix;
    }
    if sptype == GSL_SPMATRIX_CSC as libc::c_int {
        status = gsl_spmatrix_csc(dest, src);
    } else if sptype == GSL_SPMATRIX_CSR as libc::c_int {
        status = gsl_spmatrix_csr(dest, src);
    } else if sptype == GSL_SPMATRIX_COO as libc::c_int {
        status = gsl_spmatrix_memcpy(dest, src);
    } else {
        status = GSL_EINVAL as libc::c_int;
        gsl_error(
            b"unknown sparse matrix format\0" as *const u8 as *const libc::c_char,
            b"./compress_source.c\0" as *const u8 as *const libc::c_char,
            223 as libc::c_int,
            status,
        );
        return 0 as *mut gsl_spmatrix;
    }
    if status != GSL_SUCCESS as libc::c_int {
        gsl_spmatrix_free(dest);
        return 0 as *mut gsl_spmatrix;
    }
    return dest;
}
