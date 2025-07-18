use ::libc;
use ::f128;
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_matrix_complex_long_double_set_zero(m: *mut gsl_matrix_complex_long_double);
    fn gsl_spmatrix_complex_long_double_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_complex_long_double,
    ) -> libc::c_int;
    fn gsl_spmatrix_complex_long_double_set_zero(
        m: *mut gsl_spmatrix_complex_long_double,
    ) -> libc::c_int;
    fn gsl_spmatrix_complex_long_double_set(
        m: *mut gsl_spmatrix_complex_long_double,
        i: size_t,
        j: size_t,
        x: gsl_complex_long_double,
    ) -> libc::c_int;
    fn gsl_matrix_complex_set_zero(m: *mut gsl_matrix_complex);
    fn gsl_spmatrix_complex_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_complex,
    ) -> libc::c_int;
    fn gsl_spmatrix_complex_set_zero(m: *mut gsl_spmatrix_complex) -> libc::c_int;
    fn gsl_spmatrix_complex_set(
        m: *mut gsl_spmatrix_complex,
        i: size_t,
        j: size_t,
        x: gsl_complex,
    ) -> libc::c_int;
    fn gsl_matrix_complex_float_set_zero(m: *mut gsl_matrix_complex_float);
    fn gsl_spmatrix_complex_float_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_complex_float,
    ) -> libc::c_int;
    fn gsl_spmatrix_complex_float_set_zero(
        m: *mut gsl_spmatrix_complex_float,
    ) -> libc::c_int;
    fn gsl_spmatrix_complex_float_set(
        m: *mut gsl_spmatrix_complex_float,
        i: size_t,
        j: size_t,
        x: gsl_complex_float,
    ) -> libc::c_int;
    fn gsl_matrix_long_double_set_zero(m: *mut gsl_matrix_long_double);
    fn gsl_spmatrix_long_double_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_long_double,
    ) -> libc::c_int;
    fn gsl_spmatrix_long_double_set_zero(
        m: *mut gsl_spmatrix_long_double,
    ) -> libc::c_int;
    fn gsl_spmatrix_long_double_set(
        m: *mut gsl_spmatrix_long_double,
        i: size_t,
        j: size_t,
        x: f128::f128,
    ) -> libc::c_int;
    fn gsl_matrix_set_zero(m: *mut gsl_matrix);
    fn gsl_spmatrix_realloc(nzmax: size_t, m: *mut gsl_spmatrix) -> libc::c_int;
    fn gsl_spmatrix_set_zero(m: *mut gsl_spmatrix) -> libc::c_int;
    fn gsl_spmatrix_set(
        m: *mut gsl_spmatrix,
        i: size_t,
        j: size_t,
        x: libc::c_double,
    ) -> libc::c_int;
    fn gsl_matrix_float_set_zero(m: *mut gsl_matrix_float);
    fn gsl_spmatrix_float_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_float,
    ) -> libc::c_int;
    fn gsl_spmatrix_float_set_zero(m: *mut gsl_spmatrix_float) -> libc::c_int;
    fn gsl_spmatrix_float_set(
        m: *mut gsl_spmatrix_float,
        i: size_t,
        j: size_t,
        x: libc::c_float,
    ) -> libc::c_int;
    fn gsl_matrix_ulong_set_zero(m: *mut gsl_matrix_ulong);
    fn gsl_spmatrix_ulong_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_ulong,
    ) -> libc::c_int;
    fn gsl_spmatrix_ulong_set_zero(m: *mut gsl_spmatrix_ulong) -> libc::c_int;
    fn gsl_spmatrix_ulong_set(
        m: *mut gsl_spmatrix_ulong,
        i: size_t,
        j: size_t,
        x: libc::c_ulong,
    ) -> libc::c_int;
    fn gsl_matrix_long_set_zero(m: *mut gsl_matrix_long);
    fn gsl_spmatrix_long_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_long,
    ) -> libc::c_int;
    fn gsl_spmatrix_long_set_zero(m: *mut gsl_spmatrix_long) -> libc::c_int;
    fn gsl_spmatrix_long_set(
        m: *mut gsl_spmatrix_long,
        i: size_t,
        j: size_t,
        x: libc::c_long,
    ) -> libc::c_int;
    fn gsl_matrix_uint_set_zero(m: *mut gsl_matrix_uint);
    fn gsl_spmatrix_uint_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_uint,
    ) -> libc::c_int;
    fn gsl_spmatrix_uint_set_zero(m: *mut gsl_spmatrix_uint) -> libc::c_int;
    fn gsl_spmatrix_uint_set(
        m: *mut gsl_spmatrix_uint,
        i: size_t,
        j: size_t,
        x: libc::c_uint,
    ) -> libc::c_int;
    fn gsl_matrix_int_set_zero(m: *mut gsl_matrix_int);
    fn gsl_spmatrix_int_realloc(nzmax: size_t, m: *mut gsl_spmatrix_int) -> libc::c_int;
    fn gsl_spmatrix_int_set_zero(m: *mut gsl_spmatrix_int) -> libc::c_int;
    fn gsl_spmatrix_int_set(
        m: *mut gsl_spmatrix_int,
        i: size_t,
        j: size_t,
        x: libc::c_int,
    ) -> libc::c_int;
    fn gsl_matrix_ushort_set_zero(m: *mut gsl_matrix_ushort);
    fn gsl_spmatrix_ushort_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_ushort,
    ) -> libc::c_int;
    fn gsl_spmatrix_ushort_set_zero(m: *mut gsl_spmatrix_ushort) -> libc::c_int;
    fn gsl_spmatrix_ushort_set(
        m: *mut gsl_spmatrix_ushort,
        i: size_t,
        j: size_t,
        x: libc::c_ushort,
    ) -> libc::c_int;
    fn gsl_matrix_short_set_zero(m: *mut gsl_matrix_short);
    fn gsl_spmatrix_short_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_short,
    ) -> libc::c_int;
    fn gsl_spmatrix_short_set_zero(m: *mut gsl_spmatrix_short) -> libc::c_int;
    fn gsl_spmatrix_short_set(
        m: *mut gsl_spmatrix_short,
        i: size_t,
        j: size_t,
        x: libc::c_short,
    ) -> libc::c_int;
    fn gsl_spmatrix_char_set(
        m: *mut gsl_spmatrix_char,
        i: size_t,
        j: size_t,
        x: libc::c_char,
    ) -> libc::c_int;
    fn gsl_matrix_uchar_set_zero(m: *mut gsl_matrix_uchar);
    fn gsl_spmatrix_uchar_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_uchar,
    ) -> libc::c_int;
    fn gsl_spmatrix_uchar_set_zero(m: *mut gsl_spmatrix_uchar) -> libc::c_int;
    fn gsl_spmatrix_uchar_set(
        m: *mut gsl_spmatrix_uchar,
        i: size_t,
        j: size_t,
        x: libc::c_uchar,
    ) -> libc::c_int;
    fn gsl_matrix_char_set_zero(m: *mut gsl_matrix_char);
    fn gsl_spmatrix_char_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_char,
    ) -> libc::c_int;
    fn gsl_spmatrix_char_set_zero(m: *mut gsl_spmatrix_char) -> libc::c_int;
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
pub struct gsl_block_long_double_struct {
    pub size: size_t,
    pub data: *mut f128::f128,
}
pub type gsl_block_long_double = gsl_block_long_double_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_long_double {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut f128::f128,
    pub block: *mut gsl_block_long_double,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_complex_long_double_struct {
    pub size: size_t,
    pub data: *mut f128::f128,
}
pub type gsl_block_complex_long_double = gsl_block_complex_long_double_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_complex_long_double {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut f128::f128,
    pub block: *mut gsl_block_complex_long_double,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_complex_long_double {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut f128::f128,
    pub block: *mut gsl_block_complex_long_double,
    pub owner: libc::c_int,
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
pub struct gsl_block_complex_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
}
pub type gsl_block_complex = gsl_block_complex_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_complex {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block_complex,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_complex {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block_complex,
    pub owner: libc::c_int,
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
pub struct gsl_block_float_struct {
    pub size: size_t,
    pub data: *mut libc::c_float,
}
pub type gsl_block_float = gsl_block_float_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_float {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_float,
    pub block: *mut gsl_block_float,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_complex_float_struct {
    pub size: size_t,
    pub data: *mut libc::c_float,
}
pub type gsl_block_complex_float = gsl_block_complex_float_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_complex_float {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_float,
    pub block: *mut gsl_block_complex_float,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_complex_float {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_float,
    pub block: *mut gsl_block_complex_float,
    pub owner: libc::c_int,
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
pub struct gsl_matrix_long_double {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut f128::f128,
    pub block: *mut gsl_block_long_double,
    pub owner: libc::c_int,
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
pub struct gsl_matrix {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: libc::c_int,
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
pub struct gsl_matrix_float {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_float,
    pub block: *mut gsl_block_float,
    pub owner: libc::c_int,
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
pub struct gsl_block_ulong_struct {
    pub size: size_t,
    pub data: *mut libc::c_ulong,
}
pub type gsl_block_ulong = gsl_block_ulong_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_ulong {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_ulong,
    pub block: *mut gsl_block_ulong,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_ulong {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_ulong,
    pub block: *mut gsl_block_ulong,
    pub owner: libc::c_int,
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
pub struct gsl_block_long_struct {
    pub size: size_t,
    pub data: *mut libc::c_long,
}
pub type gsl_block_long = gsl_block_long_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_long {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_long,
    pub block: *mut gsl_block_long,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_long {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_long,
    pub block: *mut gsl_block_long,
    pub owner: libc::c_int,
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
pub struct gsl_block_uint_struct {
    pub size: size_t,
    pub data: *mut libc::c_uint,
}
pub type gsl_block_uint = gsl_block_uint_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_uint {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_uint,
    pub block: *mut gsl_block_uint,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_uint {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_uint,
    pub block: *mut gsl_block_uint,
    pub owner: libc::c_int,
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
pub struct gsl_block_int_struct {
    pub size: size_t,
    pub data: *mut libc::c_int,
}
pub type gsl_block_int = gsl_block_int_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_int {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_int,
    pub block: *mut gsl_block_int,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_int {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_int,
    pub block: *mut gsl_block_int,
    pub owner: libc::c_int,
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
pub struct gsl_block_ushort_struct {
    pub size: size_t,
    pub data: *mut libc::c_ushort,
}
pub type gsl_block_ushort = gsl_block_ushort_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_ushort {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_ushort,
    pub block: *mut gsl_block_ushort,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_ushort {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_ushort,
    pub block: *mut gsl_block_ushort,
    pub owner: libc::c_int,
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
pub struct gsl_block_short_struct {
    pub size: size_t,
    pub data: *mut libc::c_short,
}
pub type gsl_block_short = gsl_block_short_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_short {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_short,
    pub block: *mut gsl_block_short,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_short {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_short,
    pub block: *mut gsl_block_short,
    pub owner: libc::c_int,
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
pub struct gsl_block_uchar_struct {
    pub size: size_t,
    pub data: *mut libc::c_uchar,
}
pub type gsl_block_uchar = gsl_block_uchar_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_uchar {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_uchar,
    pub block: *mut gsl_block_uchar,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_uchar {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_uchar,
    pub block: *mut gsl_block_uchar,
    pub owner: libc::c_int,
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
pub struct gsl_block_char_struct {
    pub size: size_t,
    pub data: *mut libc::c_char,
}
pub type gsl_block_char = gsl_block_char_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_char {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_char,
    pub block: *mut gsl_block_char,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_char {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_char,
    pub block: *mut gsl_block_char,
    pub owner: libc::c_int,
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
#[inline]
unsafe extern "C" fn gsl_vector_long_double_get(
    mut v: *const gsl_vector_long_double,
    i: size_t,
) -> f128::f128 {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_vector_complex_long_double_get(
    mut v: *const gsl_vector_complex_long_double,
    i: size_t,
) -> gsl_complex_long_double {
    return *(&mut *((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*v).stride)
                as isize,
        ) as *mut f128::f128 as *mut gsl_complex_long_double);
}
#[inline]
unsafe extern "C" fn gsl_matrix_complex_long_double_get(
    mut m: *const gsl_matrix_complex_long_double,
    i: size_t,
    j: size_t,
) -> gsl_complex_long_double {
    return *(((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        ) as *mut gsl_complex_long_double);
}
#[inline]
unsafe extern "C" fn gsl_matrix_complex_long_double_set(
    mut m: *mut gsl_matrix_complex_long_double,
    i: size_t,
    j: size_t,
    x: gsl_complex_long_double,
) {
    *(((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        ) as *mut gsl_complex_long_double) = x;
}
#[inline]
unsafe extern "C" fn gsl_vector_get(
    mut v: *const gsl_vector,
    i: size_t,
) -> libc::c_double {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_vector_complex_get(
    mut v: *const gsl_vector_complex,
    i: size_t,
) -> gsl_complex {
    return *(&mut *((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*v).stride)
                as isize,
        ) as *mut libc::c_double as *mut gsl_complex);
}
#[inline]
unsafe extern "C" fn gsl_matrix_complex_get(
    mut m: *const gsl_matrix_complex,
    i: size_t,
    j: size_t,
) -> gsl_complex {
    return *(((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        ) as *mut gsl_complex);
}
#[inline]
unsafe extern "C" fn gsl_matrix_complex_set(
    mut m: *mut gsl_matrix_complex,
    i: size_t,
    j: size_t,
    x: gsl_complex,
) {
    *(((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        ) as *mut gsl_complex) = x;
}
#[inline]
unsafe extern "C" fn gsl_vector_float_get(
    mut v: *const gsl_vector_float,
    i: size_t,
) -> libc::c_float {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_vector_complex_float_get(
    mut v: *const gsl_vector_complex_float,
    i: size_t,
) -> gsl_complex_float {
    return *(&mut *((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*v).stride)
                as isize,
        ) as *mut libc::c_float as *mut gsl_complex_float);
}
#[inline]
unsafe extern "C" fn gsl_matrix_complex_float_get(
    mut m: *const gsl_matrix_complex_float,
    i: size_t,
    j: size_t,
) -> gsl_complex_float {
    return *(((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        ) as *mut gsl_complex_float);
}
#[inline]
unsafe extern "C" fn gsl_matrix_complex_float_set(
    mut m: *mut gsl_matrix_complex_float,
    i: size_t,
    j: size_t,
    x: gsl_complex_float,
) {
    *(((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        ) as *mut gsl_complex_float) = x;
}
#[inline]
unsafe extern "C" fn gsl_matrix_long_double_get(
    mut m: *const gsl_matrix_long_double,
    i: size_t,
    j: size_t,
) -> f128::f128 {
    return *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
#[inline]
unsafe extern "C" fn gsl_matrix_long_double_set(
    mut m: *mut gsl_matrix_long_double,
    i: size_t,
    j: size_t,
    x: f128::f128,
) {
    *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize) = x;
}
#[inline]
unsafe extern "C" fn gsl_matrix_get(
    mut m: *const gsl_matrix,
    i: size_t,
    j: size_t,
) -> libc::c_double {
    return *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
#[inline]
unsafe extern "C" fn gsl_matrix_set(
    mut m: *mut gsl_matrix,
    i: size_t,
    j: size_t,
    x: libc::c_double,
) {
    *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize) = x;
}
#[inline]
unsafe extern "C" fn gsl_matrix_float_get(
    mut m: *const gsl_matrix_float,
    i: size_t,
    j: size_t,
) -> libc::c_float {
    return *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
#[inline]
unsafe extern "C" fn gsl_matrix_float_set(
    mut m: *mut gsl_matrix_float,
    i: size_t,
    j: size_t,
    x: libc::c_float,
) {
    *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize) = x;
}
#[inline]
unsafe extern "C" fn gsl_vector_ulong_get(
    mut v: *const gsl_vector_ulong,
    i: size_t,
) -> libc::c_ulong {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_matrix_ulong_get(
    mut m: *const gsl_matrix_ulong,
    i: size_t,
    j: size_t,
) -> libc::c_ulong {
    return *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
#[inline]
unsafe extern "C" fn gsl_matrix_ulong_set(
    mut m: *mut gsl_matrix_ulong,
    i: size_t,
    j: size_t,
    x: libc::c_ulong,
) {
    *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize) = x;
}
#[inline]
unsafe extern "C" fn gsl_vector_long_get(
    mut v: *const gsl_vector_long,
    i: size_t,
) -> libc::c_long {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_matrix_long_get(
    mut m: *const gsl_matrix_long,
    i: size_t,
    j: size_t,
) -> libc::c_long {
    return *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
#[inline]
unsafe extern "C" fn gsl_matrix_long_set(
    mut m: *mut gsl_matrix_long,
    i: size_t,
    j: size_t,
    x: libc::c_long,
) {
    *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize) = x;
}
#[inline]
unsafe extern "C" fn gsl_vector_uint_get(
    mut v: *const gsl_vector_uint,
    i: size_t,
) -> libc::c_uint {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_matrix_uint_get(
    mut m: *const gsl_matrix_uint,
    i: size_t,
    j: size_t,
) -> libc::c_uint {
    return *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
#[inline]
unsafe extern "C" fn gsl_matrix_uint_set(
    mut m: *mut gsl_matrix_uint,
    i: size_t,
    j: size_t,
    x: libc::c_uint,
) {
    *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize) = x;
}
#[inline]
unsafe extern "C" fn gsl_vector_int_get(
    mut v: *const gsl_vector_int,
    i: size_t,
) -> libc::c_int {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_matrix_int_get(
    mut m: *const gsl_matrix_int,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    return *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
#[inline]
unsafe extern "C" fn gsl_matrix_int_set(
    mut m: *mut gsl_matrix_int,
    i: size_t,
    j: size_t,
    x: libc::c_int,
) {
    *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize) = x;
}
#[inline]
unsafe extern "C" fn gsl_vector_ushort_get(
    mut v: *const gsl_vector_ushort,
    i: size_t,
) -> libc::c_ushort {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_matrix_ushort_get(
    mut m: *const gsl_matrix_ushort,
    i: size_t,
    j: size_t,
) -> libc::c_ushort {
    return *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
#[inline]
unsafe extern "C" fn gsl_matrix_ushort_set(
    mut m: *mut gsl_matrix_ushort,
    i: size_t,
    j: size_t,
    x: libc::c_ushort,
) {
    *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize) = x;
}
#[inline]
unsafe extern "C" fn gsl_vector_short_get(
    mut v: *const gsl_vector_short,
    i: size_t,
) -> libc::c_short {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_vector_uchar_get(
    mut v: *const gsl_vector_uchar,
    i: size_t,
) -> libc::c_uchar {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_matrix_uchar_get(
    mut m: *const gsl_matrix_uchar,
    i: size_t,
    j: size_t,
) -> libc::c_uchar {
    return *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
#[inline]
unsafe extern "C" fn gsl_matrix_uchar_set(
    mut m: *mut gsl_matrix_uchar,
    i: size_t,
    j: size_t,
    x: libc::c_uchar,
) {
    *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize) = x;
}
#[inline]
unsafe extern "C" fn gsl_vector_char_get(
    mut v: *const gsl_vector_char,
    i: size_t,
) -> libc::c_char {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_matrix_char_get(
    mut m: *const gsl_matrix_char,
    i: size_t,
    j: size_t,
) -> libc::c_char {
    return *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
#[inline]
unsafe extern "C" fn gsl_matrix_char_set(
    mut m: *mut gsl_matrix_char,
    i: size_t,
    j: size_t,
    x: libc::c_char,
) {
    *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize) = x;
}
#[inline]
unsafe extern "C" fn gsl_matrix_short_get(
    mut m: *const gsl_matrix_short,
    i: size_t,
    j: size_t,
) -> libc::c_short {
    return *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
#[inline]
unsafe extern "C" fn gsl_matrix_short_set(
    mut m: *mut gsl_matrix_short,
    i: size_t,
    j: size_t,
    x: libc::c_short,
) {
    *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize) = x;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_scale(
    mut m: *mut gsl_spmatrix_complex,
    x: gsl_complex,
) -> libc::c_int {
    let xr: libc::c_double = x.dat[0 as libc::c_int as usize];
    let xi: libc::c_double = x.dat[1 as libc::c_int as usize];
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*m).nz {
        let mut mr: libc::c_double = *((*m).data)
            .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize);
        let mut mi: libc::c_double = *((*m).data)
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            );
        *((*m).data)
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize,
            ) = mr * xr - mi * xi;
        *((*m).data)
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = mi * xr + mr * xi;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_long_double_scale(
    mut m: *mut gsl_spmatrix_complex_long_double,
    x: gsl_complex_long_double,
) -> libc::c_int {
    let xr: f128::f128 = x.dat[0 as libc::c_int as usize];
    let xi: f128::f128 = x.dat[1 as libc::c_int as usize];
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*m).nz {
        let mut mr: f128::f128 = *((*m).data)
            .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize);
        let mut mi: f128::f128 = *((*m).data)
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            );
        *((*m).data)
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize,
            ) = mr * xr - mi * xi;
        *((*m).data)
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = mi * xr + mr * xi;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_float_scale(
    mut m: *mut gsl_spmatrix_complex_float,
    x: gsl_complex_float,
) -> libc::c_int {
    let xr: libc::c_float = x.dat[0 as libc::c_int as usize];
    let xi: libc::c_float = x.dat[1 as libc::c_int as usize];
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*m).nz {
        let mut mr: libc::c_float = *((*m).data)
            .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize);
        let mut mi: libc::c_float = *((*m).data)
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            );
        *((*m).data)
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize,
            ) = mr * xr - mi * xi;
        *((*m).data)
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = mi * xr + mr * xi;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_scale_columns(
    mut m: *mut gsl_spmatrix_complex,
    mut x: *const gsl_vector_complex,
) -> libc::c_int {
    if (*m).size2 != (*x).size {
        gsl_error(
            b"x vector length does not match matrix\0" as *const u8
                as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            49 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut Ad: *mut libc::c_double = (*m).data;
        if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < (*m).size2 {
                let mut xj: gsl_complex = gsl_vector_complex_get(x, j);
                let mut xr: libc::c_double = xj.dat[0 as libc::c_int as usize];
                let mut xi: libc::c_double = xj.dat[1 as libc::c_int as usize];
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let mut ar: libc::c_double = *Ad
                        .offset((2 as libc::c_int * p) as isize);
                    let mut ai: libc::c_double = *Ad
                        .offset((2 as libc::c_int * p + 1 as libc::c_int) as isize);
                    *Ad.offset((2 as libc::c_int * p) as isize) = ar * xr - ai * xi;
                    *Ad
                        .offset(
                            (2 as libc::c_int * p + 1 as libc::c_int) as isize,
                        ) = ai * xr + ar * xi;
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Aj: *const libc::c_int = (*m).i;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).nz {
                let mut y: gsl_complex = gsl_vector_complex_get(
                    x,
                    *Aj.offset(i as isize) as size_t,
                );
                let mut yr: libc::c_double = y.dat[0 as libc::c_int as usize];
                let mut yi: libc::c_double = y.dat[1 as libc::c_int as usize];
                let mut ar_0: libc::c_double = *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize,
                    );
                let mut ai_0: libc::c_double = *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize,
                    ) = ar_0 * yr - ai_0 * yi;
                *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = ai_0 * yr + ar_0 * yi;
                i = i.wrapping_add(1);
                i;
            }
        } else if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut Aj_0: *const libc::c_int = (*m).p;
            let mut i_0: size_t = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*m).nz {
                let mut y_0: gsl_complex = gsl_vector_complex_get(
                    x,
                    *Aj_0.offset(i_0 as isize) as size_t,
                );
                let mut yr_0: libc::c_double = y_0.dat[0 as libc::c_int as usize];
                let mut yi_0: libc::c_double = y_0.dat[1 as libc::c_int as usize];
                let mut ar_1: libc::c_double = *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong).wrapping_mul(i_0) as isize,
                    );
                let mut ai_1: libc::c_double = *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i_0)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong).wrapping_mul(i_0) as isize,
                    ) = ar_1 * yr_0 - ai_1 * yi_0;
                *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i_0)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = ai_1 * yr_0 + ar_1 * yi_0;
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
                113 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_long_double_scale_columns(
    mut m: *mut gsl_spmatrix_complex_long_double,
    mut x: *const gsl_vector_complex_long_double,
) -> libc::c_int {
    if (*m).size2 != (*x).size {
        gsl_error(
            b"x vector length does not match matrix\0" as *const u8
                as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            49 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut Ad: *mut f128::f128 = (*m).data;
        if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < (*m).size2 {
                let mut xj: gsl_complex_long_double = gsl_vector_complex_long_double_get(
                    x,
                    j,
                );
                let mut xr: f128::f128 = xj.dat[0 as libc::c_int as usize];
                let mut xi: f128::f128 = xj.dat[1 as libc::c_int as usize];
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let mut ar: f128::f128 = *Ad.offset((2 as libc::c_int * p) as isize);
                    let mut ai: f128::f128 = *Ad
                        .offset((2 as libc::c_int * p + 1 as libc::c_int) as isize);
                    *Ad.offset((2 as libc::c_int * p) as isize) = ar * xr - ai * xi;
                    *Ad
                        .offset(
                            (2 as libc::c_int * p + 1 as libc::c_int) as isize,
                        ) = ai * xr + ar * xi;
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Aj: *const libc::c_int = (*m).i;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).nz {
                let mut y: gsl_complex_long_double = gsl_vector_complex_long_double_get(
                    x,
                    *Aj.offset(i as isize) as size_t,
                );
                let mut yr: f128::f128 = y.dat[0 as libc::c_int as usize];
                let mut yi: f128::f128 = y.dat[1 as libc::c_int as usize];
                let mut ar_0: f128::f128 = *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize,
                    );
                let mut ai_0: f128::f128 = *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize,
                    ) = ar_0 * yr - ai_0 * yi;
                *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = ai_0 * yr + ar_0 * yi;
                i = i.wrapping_add(1);
                i;
            }
        } else if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut Aj_0: *const libc::c_int = (*m).p;
            let mut i_0: size_t = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*m).nz {
                let mut y_0: gsl_complex_long_double = gsl_vector_complex_long_double_get(
                    x,
                    *Aj_0.offset(i_0 as isize) as size_t,
                );
                let mut yr_0: f128::f128 = y_0.dat[0 as libc::c_int as usize];
                let mut yi_0: f128::f128 = y_0.dat[1 as libc::c_int as usize];
                let mut ar_1: f128::f128 = *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong).wrapping_mul(i_0) as isize,
                    );
                let mut ai_1: f128::f128 = *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i_0)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong).wrapping_mul(i_0) as isize,
                    ) = ar_1 * yr_0 - ai_1 * yi_0;
                *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i_0)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = ai_1 * yr_0 + ar_1 * yi_0;
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
                113 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_float_scale_columns(
    mut m: *mut gsl_spmatrix_complex_float,
    mut x: *const gsl_vector_complex_float,
) -> libc::c_int {
    if (*m).size2 != (*x).size {
        gsl_error(
            b"x vector length does not match matrix\0" as *const u8
                as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            49 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut Ad: *mut libc::c_float = (*m).data;
        if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < (*m).size2 {
                let mut xj: gsl_complex_float = gsl_vector_complex_float_get(x, j);
                let mut xr: libc::c_float = xj.dat[0 as libc::c_int as usize];
                let mut xi: libc::c_float = xj.dat[1 as libc::c_int as usize];
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let mut ar: libc::c_float = *Ad
                        .offset((2 as libc::c_int * p) as isize);
                    let mut ai: libc::c_float = *Ad
                        .offset((2 as libc::c_int * p + 1 as libc::c_int) as isize);
                    *Ad.offset((2 as libc::c_int * p) as isize) = ar * xr - ai * xi;
                    *Ad
                        .offset(
                            (2 as libc::c_int * p + 1 as libc::c_int) as isize,
                        ) = ai * xr + ar * xi;
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Aj: *const libc::c_int = (*m).i;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).nz {
                let mut y: gsl_complex_float = gsl_vector_complex_float_get(
                    x,
                    *Aj.offset(i as isize) as size_t,
                );
                let mut yr: libc::c_float = y.dat[0 as libc::c_int as usize];
                let mut yi: libc::c_float = y.dat[1 as libc::c_int as usize];
                let mut ar_0: libc::c_float = *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize,
                    );
                let mut ai_0: libc::c_float = *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize,
                    ) = ar_0 * yr - ai_0 * yi;
                *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = ai_0 * yr + ar_0 * yi;
                i = i.wrapping_add(1);
                i;
            }
        } else if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut Aj_0: *const libc::c_int = (*m).p;
            let mut i_0: size_t = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*m).nz {
                let mut y_0: gsl_complex_float = gsl_vector_complex_float_get(
                    x,
                    *Aj_0.offset(i_0 as isize) as size_t,
                );
                let mut yr_0: libc::c_float = y_0.dat[0 as libc::c_int as usize];
                let mut yi_0: libc::c_float = y_0.dat[1 as libc::c_int as usize];
                let mut ar_1: libc::c_float = *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong).wrapping_mul(i_0) as isize,
                    );
                let mut ai_1: libc::c_float = *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i_0)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong).wrapping_mul(i_0) as isize,
                    ) = ar_1 * yr_0 - ai_1 * yi_0;
                *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i_0)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = ai_1 * yr_0 + ar_1 * yi_0;
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
                113 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_long_double_scale_rows(
    mut m: *mut gsl_spmatrix_complex_long_double,
    mut x: *const gsl_vector_complex_long_double,
) -> libc::c_int {
    if (*m).size1 != (*x).size {
        gsl_error(
            b"x vector length does not match matrix\0" as *const u8
                as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut Ad: *mut f128::f128 = (*m).data;
        if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ai: *const libc::c_int = (*m).i;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).nz {
                let mut y: gsl_complex_long_double = gsl_vector_complex_long_double_get(
                    x,
                    *Ai.offset(i as isize) as size_t,
                );
                let mut yr: f128::f128 = y.dat[0 as libc::c_int as usize];
                let mut yi: f128::f128 = y.dat[1 as libc::c_int as usize];
                let mut ar: f128::f128 = *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize,
                    );
                let mut ai: f128::f128 = *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize,
                    ) = ar * yr - ai * yi;
                *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = ai * yr + ar * yi;
                i = i.wrapping_add(1);
                i;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut i_0: size_t = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*m).size1 {
                let mut y_0: gsl_complex_long_double = gsl_vector_complex_long_double_get(
                    x,
                    i_0,
                );
                let mut yr_0: f128::f128 = y_0.dat[0 as libc::c_int as usize];
                let mut yi_0: f128::f128 = y_0.dat[1 as libc::c_int as usize];
                p = *Ap.offset(i_0 as isize);
                while p
                    < *Ap
                        .offset(
                            i_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let mut ar_0: f128::f128 = *Ad
                        .offset((2 as libc::c_int * p) as isize);
                    let mut ai_0: f128::f128 = *Ad
                        .offset((2 as libc::c_int * p + 1 as libc::c_int) as isize);
                    *Ad
                        .offset(
                            (2 as libc::c_int * p) as isize,
                        ) = ar_0 * yr_0 - ai_0 * yi_0;
                    *Ad
                        .offset(
                            (2 as libc::c_int * p + 1 as libc::c_int) as isize,
                        ) = ai_0 * yr_0 + ar_0 * yi_0;
                    p += 1;
                    p;
                }
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut Ai_0: *const libc::c_int = (*m).i;
            let mut i_1: size_t = 0;
            i_1 = 0 as libc::c_int as size_t;
            while i_1 < (*m).nz {
                let mut y_1: gsl_complex_long_double = gsl_vector_complex_long_double_get(
                    x,
                    *Ai_0.offset(i_1 as isize) as size_t,
                );
                let mut yr_1: f128::f128 = y_1.dat[0 as libc::c_int as usize];
                let mut yi_1: f128::f128 = y_1.dat[1 as libc::c_int as usize];
                let mut ar_1: f128::f128 = *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong).wrapping_mul(i_1) as isize,
                    );
                let mut ai_1: f128::f128 = *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i_1)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong).wrapping_mul(i_1) as isize,
                    ) = ar_1 * yr_1 - ai_1 * yi_1;
                *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i_1)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = ai_1 * yr_1 + ar_1 * yi_1;
                i_1 = i_1.wrapping_add(1);
                i_1;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
                189 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_float_scale_rows(
    mut m: *mut gsl_spmatrix_complex_float,
    mut x: *const gsl_vector_complex_float,
) -> libc::c_int {
    if (*m).size1 != (*x).size {
        gsl_error(
            b"x vector length does not match matrix\0" as *const u8
                as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut Ad: *mut libc::c_float = (*m).data;
        if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ai: *const libc::c_int = (*m).i;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).nz {
                let mut y: gsl_complex_float = gsl_vector_complex_float_get(
                    x,
                    *Ai.offset(i as isize) as size_t,
                );
                let mut yr: libc::c_float = y.dat[0 as libc::c_int as usize];
                let mut yi: libc::c_float = y.dat[1 as libc::c_int as usize];
                let mut ar: libc::c_float = *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize,
                    );
                let mut ai: libc::c_float = *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize,
                    ) = ar * yr - ai * yi;
                *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = ai * yr + ar * yi;
                i = i.wrapping_add(1);
                i;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut i_0: size_t = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*m).size1 {
                let mut y_0: gsl_complex_float = gsl_vector_complex_float_get(x, i_0);
                let mut yr_0: libc::c_float = y_0.dat[0 as libc::c_int as usize];
                let mut yi_0: libc::c_float = y_0.dat[1 as libc::c_int as usize];
                p = *Ap.offset(i_0 as isize);
                while p
                    < *Ap
                        .offset(
                            i_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let mut ar_0: libc::c_float = *Ad
                        .offset((2 as libc::c_int * p) as isize);
                    let mut ai_0: libc::c_float = *Ad
                        .offset((2 as libc::c_int * p + 1 as libc::c_int) as isize);
                    *Ad
                        .offset(
                            (2 as libc::c_int * p) as isize,
                        ) = ar_0 * yr_0 - ai_0 * yi_0;
                    *Ad
                        .offset(
                            (2 as libc::c_int * p + 1 as libc::c_int) as isize,
                        ) = ai_0 * yr_0 + ar_0 * yi_0;
                    p += 1;
                    p;
                }
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut Ai_0: *const libc::c_int = (*m).i;
            let mut i_1: size_t = 0;
            i_1 = 0 as libc::c_int as size_t;
            while i_1 < (*m).nz {
                let mut y_1: gsl_complex_float = gsl_vector_complex_float_get(
                    x,
                    *Ai_0.offset(i_1 as isize) as size_t,
                );
                let mut yr_1: libc::c_float = y_1.dat[0 as libc::c_int as usize];
                let mut yi_1: libc::c_float = y_1.dat[1 as libc::c_int as usize];
                let mut ar_1: libc::c_float = *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong).wrapping_mul(i_1) as isize,
                    );
                let mut ai_1: libc::c_float = *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i_1)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong).wrapping_mul(i_1) as isize,
                    ) = ar_1 * yr_1 - ai_1 * yi_1;
                *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i_1)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = ai_1 * yr_1 + ar_1 * yi_1;
                i_1 = i_1.wrapping_add(1);
                i_1;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
                189 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_scale_rows(
    mut m: *mut gsl_spmatrix_complex,
    mut x: *const gsl_vector_complex,
) -> libc::c_int {
    if (*m).size1 != (*x).size {
        gsl_error(
            b"x vector length does not match matrix\0" as *const u8
                as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut Ad: *mut libc::c_double = (*m).data;
        if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ai: *const libc::c_int = (*m).i;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).nz {
                let mut y: gsl_complex = gsl_vector_complex_get(
                    x,
                    *Ai.offset(i as isize) as size_t,
                );
                let mut yr: libc::c_double = y.dat[0 as libc::c_int as usize];
                let mut yi: libc::c_double = y.dat[1 as libc::c_int as usize];
                let mut ar: libc::c_double = *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize,
                    );
                let mut ai: libc::c_double = *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize,
                    ) = ar * yr - ai * yi;
                *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = ai * yr + ar * yi;
                i = i.wrapping_add(1);
                i;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut i_0: size_t = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*m).size1 {
                let mut y_0: gsl_complex = gsl_vector_complex_get(x, i_0);
                let mut yr_0: libc::c_double = y_0.dat[0 as libc::c_int as usize];
                let mut yi_0: libc::c_double = y_0.dat[1 as libc::c_int as usize];
                p = *Ap.offset(i_0 as isize);
                while p
                    < *Ap
                        .offset(
                            i_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let mut ar_0: libc::c_double = *Ad
                        .offset((2 as libc::c_int * p) as isize);
                    let mut ai_0: libc::c_double = *Ad
                        .offset((2 as libc::c_int * p + 1 as libc::c_int) as isize);
                    *Ad
                        .offset(
                            (2 as libc::c_int * p) as isize,
                        ) = ar_0 * yr_0 - ai_0 * yi_0;
                    *Ad
                        .offset(
                            (2 as libc::c_int * p + 1 as libc::c_int) as isize,
                        ) = ai_0 * yr_0 + ar_0 * yi_0;
                    p += 1;
                    p;
                }
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut Ai_0: *const libc::c_int = (*m).i;
            let mut i_1: size_t = 0;
            i_1 = 0 as libc::c_int as size_t;
            while i_1 < (*m).nz {
                let mut y_1: gsl_complex = gsl_vector_complex_get(
                    x,
                    *Ai_0.offset(i_1 as isize) as size_t,
                );
                let mut yr_1: libc::c_double = y_1.dat[0 as libc::c_int as usize];
                let mut yi_1: libc::c_double = y_1.dat[1 as libc::c_int as usize];
                let mut ar_1: libc::c_double = *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong).wrapping_mul(i_1) as isize,
                    );
                let mut ai_1: libc::c_double = *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i_1)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong).wrapping_mul(i_1) as isize,
                    ) = ar_1 * yr_1 - ai_1 * yi_1;
                *Ad
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i_1)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = ai_1 * yr_1 + ar_1 * yi_1;
                i_1 = i_1.wrapping_add(1);
                i_1;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
                189 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_float_add(
    mut c: *mut gsl_spmatrix_complex_float,
    mut a: *const gsl_spmatrix_complex_float,
    mut b: *const gsl_spmatrix_complex_float,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N || (*c).size1 != M || (*c).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            216 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*a).sptype != (*b).sptype || (*a).sptype != (*c).sptype {
        gsl_error(
            b"matrices must have same sparse storage format\0" as *const u8
                as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            221 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*a).sptype == GSL_SPMATRIX_COO as libc::c_int {
        gsl_error(
            b"COO format not yet supported\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            225 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut w: *mut libc::c_int = (*a).work.work_int;
        let mut x: *mut libc::c_float = (*c).work.work_atomic;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut Ci: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut Cd: *mut libc::c_float = 0 as *mut libc::c_float;
        let mut p: libc::c_int = 0;
        let mut j: size_t = 0;
        let mut nz: size_t = 0 as libc::c_int as size_t;
        let mut inner_size: size_t = 0;
        let mut outer_size: size_t = 0;
        if (*a).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            inner_size = M;
            outer_size = N;
        } else if (*a).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            inner_size = N;
            outer_size = M;
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
                251 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        if (*c).nzmax < ((*a).nz).wrapping_add((*b).nz) {
            status = gsl_spmatrix_complex_float_realloc(
                ((*a).nz).wrapping_add((*b).nz),
                c,
            );
            if status != 0 {
                return status;
            }
        }
        j = 0 as libc::c_int as size_t;
        while j < inner_size {
            *w.offset(j as isize) = 0 as libc::c_int;
            j = j.wrapping_add(1);
            j;
        }
        Ci = (*c).i;
        Cp = (*c).p;
        Cd = (*c).data;
        j = 0 as libc::c_int as size_t;
        while j < outer_size {
            *Cp.offset(j as isize) = nz as libc::c_int;
            nz = spmatrix_complex_float_scatter(
                a,
                j,
                w,
                x,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                c,
                nz,
            );
            nz = spmatrix_complex_float_scatter(
                b,
                j,
                w,
                x,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                c,
                nz,
            );
            p = *Cp.offset(j as isize);
            while p < nz as libc::c_int {
                *Cd
                    .offset(
                        (2 as libc::c_int * p) as isize,
                    ) = *x.offset((2 as libc::c_int * *Ci.offset(p as isize)) as isize);
                *Cd
                    .offset(
                        (2 as libc::c_int * p + 1 as libc::c_int) as isize,
                    ) = *x
                    .offset(
                        (2 as libc::c_int * *Ci.offset(p as isize) + 1 as libc::c_int)
                            as isize,
                    );
                p += 1;
                p;
            }
            j = j.wrapping_add(1);
            j;
        }
        *Cp.offset(j as isize) = nz as libc::c_int;
        (*c).nz = nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_long_double_add(
    mut c: *mut gsl_spmatrix_complex_long_double,
    mut a: *const gsl_spmatrix_complex_long_double,
    mut b: *const gsl_spmatrix_complex_long_double,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N || (*c).size1 != M || (*c).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            216 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*a).sptype != (*b).sptype || (*a).sptype != (*c).sptype {
        gsl_error(
            b"matrices must have same sparse storage format\0" as *const u8
                as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            221 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*a).sptype == GSL_SPMATRIX_COO as libc::c_int {
        gsl_error(
            b"COO format not yet supported\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            225 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut w: *mut libc::c_int = (*a).work.work_int;
        let mut x: *mut f128::f128 = (*c).work.work_atomic;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut Ci: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut Cd: *mut f128::f128 = 0 as *mut f128::f128;
        let mut p: libc::c_int = 0;
        let mut j: size_t = 0;
        let mut nz: size_t = 0 as libc::c_int as size_t;
        let mut inner_size: size_t = 0;
        let mut outer_size: size_t = 0;
        if (*a).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            inner_size = M;
            outer_size = N;
        } else if (*a).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            inner_size = N;
            outer_size = M;
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
                251 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        if (*c).nzmax < ((*a).nz).wrapping_add((*b).nz) {
            status = gsl_spmatrix_complex_long_double_realloc(
                ((*a).nz).wrapping_add((*b).nz),
                c,
            );
            if status != 0 {
                return status;
            }
        }
        j = 0 as libc::c_int as size_t;
        while j < inner_size {
            *w.offset(j as isize) = 0 as libc::c_int;
            j = j.wrapping_add(1);
            j;
        }
        Ci = (*c).i;
        Cp = (*c).p;
        Cd = (*c).data;
        j = 0 as libc::c_int as size_t;
        while j < outer_size {
            *Cp.offset(j as isize) = nz as libc::c_int;
            nz = spmatrix_complex_long_double_scatter(
                a,
                j,
                w,
                x,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                c,
                nz,
            );
            nz = spmatrix_complex_long_double_scatter(
                b,
                j,
                w,
                x,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                c,
                nz,
            );
            p = *Cp.offset(j as isize);
            while p < nz as libc::c_int {
                *Cd
                    .offset(
                        (2 as libc::c_int * p) as isize,
                    ) = *x.offset((2 as libc::c_int * *Ci.offset(p as isize)) as isize);
                *Cd
                    .offset(
                        (2 as libc::c_int * p + 1 as libc::c_int) as isize,
                    ) = *x
                    .offset(
                        (2 as libc::c_int * *Ci.offset(p as isize) + 1 as libc::c_int)
                            as isize,
                    );
                p += 1;
                p;
            }
            j = j.wrapping_add(1);
            j;
        }
        *Cp.offset(j as isize) = nz as libc::c_int;
        (*c).nz = nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_add(
    mut c: *mut gsl_spmatrix_complex,
    mut a: *const gsl_spmatrix_complex,
    mut b: *const gsl_spmatrix_complex,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N || (*c).size1 != M || (*c).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            216 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*a).sptype != (*b).sptype || (*a).sptype != (*c).sptype {
        gsl_error(
            b"matrices must have same sparse storage format\0" as *const u8
                as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            221 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*a).sptype == GSL_SPMATRIX_COO as libc::c_int {
        gsl_error(
            b"COO format not yet supported\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            225 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut w: *mut libc::c_int = (*a).work.work_int;
        let mut x: *mut libc::c_double = (*c).work.work_atomic;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut Ci: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut Cd: *mut libc::c_double = 0 as *mut libc::c_double;
        let mut p: libc::c_int = 0;
        let mut j: size_t = 0;
        let mut nz: size_t = 0 as libc::c_int as size_t;
        let mut inner_size: size_t = 0;
        let mut outer_size: size_t = 0;
        if (*a).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            inner_size = M;
            outer_size = N;
        } else if (*a).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            inner_size = N;
            outer_size = M;
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
                251 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        if (*c).nzmax < ((*a).nz).wrapping_add((*b).nz) {
            status = gsl_spmatrix_complex_realloc(((*a).nz).wrapping_add((*b).nz), c);
            if status != 0 {
                return status;
            }
        }
        j = 0 as libc::c_int as size_t;
        while j < inner_size {
            *w.offset(j as isize) = 0 as libc::c_int;
            j = j.wrapping_add(1);
            j;
        }
        Ci = (*c).i;
        Cp = (*c).p;
        Cd = (*c).data;
        j = 0 as libc::c_int as size_t;
        while j < outer_size {
            *Cp.offset(j as isize) = nz as libc::c_int;
            nz = spmatrix_complex_scatter(
                a,
                j,
                w,
                x,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                c,
                nz,
            );
            nz = spmatrix_complex_scatter(
                b,
                j,
                w,
                x,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                c,
                nz,
            );
            p = *Cp.offset(j as isize);
            while p < nz as libc::c_int {
                *Cd
                    .offset(
                        (2 as libc::c_int * p) as isize,
                    ) = *x.offset((2 as libc::c_int * *Ci.offset(p as isize)) as isize);
                *Cd
                    .offset(
                        (2 as libc::c_int * p + 1 as libc::c_int) as isize,
                    ) = *x
                    .offset(
                        (2 as libc::c_int * *Ci.offset(p as isize) + 1 as libc::c_int)
                            as isize,
                    );
                p += 1;
                p;
            }
            j = j.wrapping_add(1);
            j;
        }
        *Cp.offset(j as isize) = nz as libc::c_int;
        (*c).nz = nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_float_dense_add(
    mut a: *mut gsl_matrix_complex_float,
    mut b: *const gsl_spmatrix_complex_float,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            312 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let mut bd: *const libc::c_float = (*b).data;
        if (*b).nz == 0 as libc::c_int as libc::c_ulong {
            return GSL_SUCCESS as libc::c_int;
        }
        if (*b).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut bi: *const libc::c_int = (*b).i;
            let mut bj: *const libc::c_int = (*b).p;
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*b).nz {
                let idx: size_t = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        (*bi.offset(n as isize) as libc::c_ulong)
                            .wrapping_mul(tda_a)
                            .wrapping_add(*bj.offset(n as isize) as libc::c_ulong),
                    );
                *((*a).data).offset(idx as isize)
                    += *bd
                        .offset(
                            (2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize,
                        );
                *((*a).data)
                    .offset(idx.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                    += *bd
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                n = n.wrapping_add(1);
                n;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut bi_0: *const libc::c_int = (*b).i;
            let mut bp: *const libc::c_int = (*b).p;
            let mut j: size_t = 0;
            let mut p: libc::c_int = 0;
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *bp.offset(j as isize);
                while p
                    < *bp
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let idx_0: size_t = (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            (*bi_0.offset(p as isize) as libc::c_ulong)
                                .wrapping_mul(tda_a)
                                .wrapping_add(j),
                        );
                    *((*a).data).offset(idx_0 as isize)
                        += *bd.offset((2 as libc::c_int * p) as isize);
                    *((*a).data)
                        .offset(
                            idx_0.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        )
                        += *bd
                            .offset((2 as libc::c_int * p + 1 as libc::c_int) as isize);
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut bj_0: *const libc::c_int = (*b).i;
            let mut bp_0: *const libc::c_int = (*b).p;
            let mut i: size_t = 0;
            let mut p_0: libc::c_int = 0;
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *bp_0.offset(i as isize);
                while p_0
                    < *bp_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let idx_1: size_t = (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(*bj_0.offset(p_0 as isize) as libc::c_ulong),
                        );
                    *((*a).data).offset(idx_1 as isize)
                        += *bd.offset((2 as libc::c_int * p_0) as isize);
                    *((*a).data)
                        .offset(
                            idx_1.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        )
                        += *bd
                            .offset(
                                (2 as libc::c_int * p_0 + 1 as libc::c_int) as isize,
                            );
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_long_double_dense_add(
    mut a: *mut gsl_matrix_complex_long_double,
    mut b: *const gsl_spmatrix_complex_long_double,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            312 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let mut bd: *const f128::f128 = (*b).data;
        if (*b).nz == 0 as libc::c_int as libc::c_ulong {
            return GSL_SUCCESS as libc::c_int;
        }
        if (*b).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut bi: *const libc::c_int = (*b).i;
            let mut bj: *const libc::c_int = (*b).p;
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*b).nz {
                let idx: size_t = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        (*bi.offset(n as isize) as libc::c_ulong)
                            .wrapping_mul(tda_a)
                            .wrapping_add(*bj.offset(n as isize) as libc::c_ulong),
                    );
                *((*a).data).offset(idx as isize)
                    += *bd
                        .offset(
                            (2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize,
                        );
                *((*a).data)
                    .offset(idx.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                    += *bd
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                n = n.wrapping_add(1);
                n;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut bi_0: *const libc::c_int = (*b).i;
            let mut bp: *const libc::c_int = (*b).p;
            let mut j: size_t = 0;
            let mut p: libc::c_int = 0;
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *bp.offset(j as isize);
                while p
                    < *bp
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let idx_0: size_t = (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            (*bi_0.offset(p as isize) as libc::c_ulong)
                                .wrapping_mul(tda_a)
                                .wrapping_add(j),
                        );
                    *((*a).data).offset(idx_0 as isize)
                        += *bd.offset((2 as libc::c_int * p) as isize);
                    *((*a).data)
                        .offset(
                            idx_0.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        )
                        += *bd
                            .offset((2 as libc::c_int * p + 1 as libc::c_int) as isize);
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut bj_0: *const libc::c_int = (*b).i;
            let mut bp_0: *const libc::c_int = (*b).p;
            let mut i: size_t = 0;
            let mut p_0: libc::c_int = 0;
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *bp_0.offset(i as isize);
                while p_0
                    < *bp_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let idx_1: size_t = (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(*bj_0.offset(p_0 as isize) as libc::c_ulong),
                        );
                    *((*a).data).offset(idx_1 as isize)
                        += *bd.offset((2 as libc::c_int * p_0) as isize);
                    *((*a).data)
                        .offset(
                            idx_1.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        )
                        += *bd
                            .offset(
                                (2 as libc::c_int * p_0 + 1 as libc::c_int) as isize,
                            );
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_dense_add(
    mut a: *mut gsl_matrix_complex,
    mut b: *const gsl_spmatrix_complex,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            312 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let mut bd: *const libc::c_double = (*b).data;
        if (*b).nz == 0 as libc::c_int as libc::c_ulong {
            return GSL_SUCCESS as libc::c_int;
        }
        if (*b).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut bi: *const libc::c_int = (*b).i;
            let mut bj: *const libc::c_int = (*b).p;
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*b).nz {
                let idx: size_t = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        (*bi.offset(n as isize) as libc::c_ulong)
                            .wrapping_mul(tda_a)
                            .wrapping_add(*bj.offset(n as isize) as libc::c_ulong),
                    );
                *((*a).data).offset(idx as isize)
                    += *bd
                        .offset(
                            (2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize,
                        );
                *((*a).data)
                    .offset(idx.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                    += *bd
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                n = n.wrapping_add(1);
                n;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut bi_0: *const libc::c_int = (*b).i;
            let mut bp: *const libc::c_int = (*b).p;
            let mut j: size_t = 0;
            let mut p: libc::c_int = 0;
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *bp.offset(j as isize);
                while p
                    < *bp
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let idx_0: size_t = (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            (*bi_0.offset(p as isize) as libc::c_ulong)
                                .wrapping_mul(tda_a)
                                .wrapping_add(j),
                        );
                    *((*a).data).offset(idx_0 as isize)
                        += *bd.offset((2 as libc::c_int * p) as isize);
                    *((*a).data)
                        .offset(
                            idx_0.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        )
                        += *bd
                            .offset((2 as libc::c_int * p + 1 as libc::c_int) as isize);
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut bj_0: *const libc::c_int = (*b).i;
            let mut bp_0: *const libc::c_int = (*b).p;
            let mut i: size_t = 0;
            let mut p_0: libc::c_int = 0;
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *bp_0.offset(i as isize);
                while p_0
                    < *bp_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let idx_1: size_t = (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(*bj_0.offset(p_0 as isize) as libc::c_ulong),
                        );
                    *((*a).data).offset(idx_1 as isize)
                        += *bd.offset((2 as libc::c_int * p_0) as isize);
                    *((*a).data)
                        .offset(
                            idx_1.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        )
                        += *bd
                            .offset(
                                (2 as libc::c_int * p_0 + 1 as libc::c_int) as isize,
                            );
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_long_double_add_to_dense(
    mut a: *mut gsl_matrix_complex_long_double,
    mut b: *const gsl_spmatrix_complex_long_double,
) -> libc::c_int {
    return gsl_spmatrix_complex_long_double_dense_add(a, b);
}
pub unsafe extern "C" fn gsl_spmatrix_complex_add_to_dense(
    mut a: *mut gsl_matrix_complex,
    mut b: *const gsl_spmatrix_complex,
) -> libc::c_int {
    return gsl_spmatrix_complex_dense_add(a, b);
}
pub unsafe extern "C" fn gsl_spmatrix_complex_float_add_to_dense(
    mut a: *mut gsl_matrix_complex_float,
    mut b: *const gsl_spmatrix_complex_float,
) -> libc::c_int {
    return gsl_spmatrix_complex_float_dense_add(a, b);
}
pub unsafe extern "C" fn gsl_spmatrix_complex_long_double_dense_sub(
    mut a: *mut gsl_matrix_complex_long_double,
    mut b: *const gsl_spmatrix_complex_long_double,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            403 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let mut bd: *const f128::f128 = (*b).data;
        if (*b).nz == 0 as libc::c_int as libc::c_ulong {
            return GSL_SUCCESS as libc::c_int;
        }
        if (*b).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut bi: *const libc::c_int = (*b).i;
            let mut bj: *const libc::c_int = (*b).p;
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*b).nz {
                let idx: size_t = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        (*bi.offset(n as isize) as libc::c_ulong)
                            .wrapping_mul(tda_a)
                            .wrapping_add(*bj.offset(n as isize) as libc::c_ulong),
                    );
                *((*a).data).offset(idx as isize)
                    -= *bd
                        .offset(
                            (2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize,
                        );
                *((*a).data)
                    .offset(idx.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                    -= *bd
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                n = n.wrapping_add(1);
                n;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut bi_0: *const libc::c_int = (*b).i;
            let mut bp: *const libc::c_int = (*b).p;
            let mut j: size_t = 0;
            let mut p: libc::c_int = 0;
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *bp.offset(j as isize);
                while p
                    < *bp
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let idx_0: size_t = (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            (*bi_0.offset(p as isize) as libc::c_ulong)
                                .wrapping_mul(tda_a)
                                .wrapping_add(j),
                        );
                    *((*a).data).offset(idx_0 as isize)
                        -= *bd.offset((2 as libc::c_int * p) as isize);
                    *((*a).data)
                        .offset(
                            idx_0.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        )
                        -= *bd
                            .offset((2 as libc::c_int * p + 1 as libc::c_int) as isize);
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut bj_0: *const libc::c_int = (*b).i;
            let mut bp_0: *const libc::c_int = (*b).p;
            let mut i: size_t = 0;
            let mut p_0: libc::c_int = 0;
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *bp_0.offset(i as isize);
                while p_0
                    < *bp_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let idx_1: size_t = (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(*bj_0.offset(p_0 as isize) as libc::c_ulong),
                        );
                    *((*a).data).offset(idx_1 as isize)
                        -= *bd.offset((2 as libc::c_int * p_0) as isize);
                    *((*a).data)
                        .offset(
                            idx_1.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        )
                        -= *bd
                            .offset(
                                (2 as libc::c_int * p_0 + 1 as libc::c_int) as isize,
                            );
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_dense_sub(
    mut a: *mut gsl_matrix_complex,
    mut b: *const gsl_spmatrix_complex,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            403 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let mut bd: *const libc::c_double = (*b).data;
        if (*b).nz == 0 as libc::c_int as libc::c_ulong {
            return GSL_SUCCESS as libc::c_int;
        }
        if (*b).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut bi: *const libc::c_int = (*b).i;
            let mut bj: *const libc::c_int = (*b).p;
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*b).nz {
                let idx: size_t = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        (*bi.offset(n as isize) as libc::c_ulong)
                            .wrapping_mul(tda_a)
                            .wrapping_add(*bj.offset(n as isize) as libc::c_ulong),
                    );
                *((*a).data).offset(idx as isize)
                    -= *bd
                        .offset(
                            (2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize,
                        );
                *((*a).data)
                    .offset(idx.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                    -= *bd
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                n = n.wrapping_add(1);
                n;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut bi_0: *const libc::c_int = (*b).i;
            let mut bp: *const libc::c_int = (*b).p;
            let mut j: size_t = 0;
            let mut p: libc::c_int = 0;
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *bp.offset(j as isize);
                while p
                    < *bp
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let idx_0: size_t = (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            (*bi_0.offset(p as isize) as libc::c_ulong)
                                .wrapping_mul(tda_a)
                                .wrapping_add(j),
                        );
                    *((*a).data).offset(idx_0 as isize)
                        -= *bd.offset((2 as libc::c_int * p) as isize);
                    *((*a).data)
                        .offset(
                            idx_0.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        )
                        -= *bd
                            .offset((2 as libc::c_int * p + 1 as libc::c_int) as isize);
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut bj_0: *const libc::c_int = (*b).i;
            let mut bp_0: *const libc::c_int = (*b).p;
            let mut i: size_t = 0;
            let mut p_0: libc::c_int = 0;
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *bp_0.offset(i as isize);
                while p_0
                    < *bp_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let idx_1: size_t = (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(*bj_0.offset(p_0 as isize) as libc::c_ulong),
                        );
                    *((*a).data).offset(idx_1 as isize)
                        -= *bd.offset((2 as libc::c_int * p_0) as isize);
                    *((*a).data)
                        .offset(
                            idx_1.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        )
                        -= *bd
                            .offset(
                                (2 as libc::c_int * p_0 + 1 as libc::c_int) as isize,
                            );
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_float_dense_sub(
    mut a: *mut gsl_matrix_complex_float,
    mut b: *const gsl_spmatrix_complex_float,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            403 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let mut bd: *const libc::c_float = (*b).data;
        if (*b).nz == 0 as libc::c_int as libc::c_ulong {
            return GSL_SUCCESS as libc::c_int;
        }
        if (*b).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut bi: *const libc::c_int = (*b).i;
            let mut bj: *const libc::c_int = (*b).p;
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*b).nz {
                let idx: size_t = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        (*bi.offset(n as isize) as libc::c_ulong)
                            .wrapping_mul(tda_a)
                            .wrapping_add(*bj.offset(n as isize) as libc::c_ulong),
                    );
                *((*a).data).offset(idx as isize)
                    -= *bd
                        .offset(
                            (2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize,
                        );
                *((*a).data)
                    .offset(idx.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                    -= *bd
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                n = n.wrapping_add(1);
                n;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut bi_0: *const libc::c_int = (*b).i;
            let mut bp: *const libc::c_int = (*b).p;
            let mut j: size_t = 0;
            let mut p: libc::c_int = 0;
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *bp.offset(j as isize);
                while p
                    < *bp
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let idx_0: size_t = (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            (*bi_0.offset(p as isize) as libc::c_ulong)
                                .wrapping_mul(tda_a)
                                .wrapping_add(j),
                        );
                    *((*a).data).offset(idx_0 as isize)
                        -= *bd.offset((2 as libc::c_int * p) as isize);
                    *((*a).data)
                        .offset(
                            idx_0.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        )
                        -= *bd
                            .offset((2 as libc::c_int * p + 1 as libc::c_int) as isize);
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut bj_0: *const libc::c_int = (*b).i;
            let mut bp_0: *const libc::c_int = (*b).p;
            let mut i: size_t = 0;
            let mut p_0: libc::c_int = 0;
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *bp_0.offset(i as isize);
                while p_0
                    < *bp_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let idx_1: size_t = (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(*bj_0.offset(p_0 as isize) as libc::c_ulong),
                        );
                    *((*a).data).offset(idx_1 as isize)
                        -= *bd.offset((2 as libc::c_int * p_0) as isize);
                    *((*a).data)
                        .offset(
                            idx_1.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        )
                        -= *bd
                            .offset(
                                (2 as libc::c_int * p_0 + 1 as libc::c_int) as isize,
                            );
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_long_double_d2sp(
    mut T: *mut gsl_spmatrix_complex_long_double,
    mut A: *const gsl_matrix_complex_long_double,
) -> libc::c_int {
    if (*T).size1 != (*A).size1 || (*T).size2 != (*A).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            479 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if !((*T).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"sparse matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            483 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        gsl_spmatrix_complex_long_double_set_zero(T);
        i = 0 as libc::c_int as size_t;
        while i < (*A).size1 {
            j = 0 as libc::c_int as size_t;
            while j < (*A).size2 {
                let mut x: gsl_complex_long_double = gsl_matrix_complex_long_double_get(
                    A,
                    i,
                    j,
                );
                if !(x.dat[0 as libc::c_int as usize]
                    == f128::f128::new(0 as libc::c_int)
                    && x.dat[1 as libc::c_int as usize]
                        == f128::f128::new(0 as libc::c_int))
                {
                    gsl_spmatrix_complex_long_double_set(T, i, j, x);
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_d2sp(
    mut T: *mut gsl_spmatrix_complex,
    mut A: *const gsl_matrix_complex,
) -> libc::c_int {
    if (*T).size1 != (*A).size1 || (*T).size2 != (*A).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            479 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if !((*T).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"sparse matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            483 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        gsl_spmatrix_complex_set_zero(T);
        i = 0 as libc::c_int as size_t;
        while i < (*A).size1 {
            j = 0 as libc::c_int as size_t;
            while j < (*A).size2 {
                let mut x: gsl_complex = gsl_matrix_complex_get(A, i, j);
                if !(x.dat[0 as libc::c_int as usize]
                    == 0 as libc::c_int as libc::c_double
                    && x.dat[1 as libc::c_int as usize]
                        == 0 as libc::c_int as libc::c_double)
                {
                    gsl_spmatrix_complex_set(T, i, j, x);
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_float_d2sp(
    mut T: *mut gsl_spmatrix_complex_float,
    mut A: *const gsl_matrix_complex_float,
) -> libc::c_int {
    if (*T).size1 != (*A).size1 || (*T).size2 != (*A).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            479 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if !((*T).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"sparse matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            483 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        gsl_spmatrix_complex_float_set_zero(T);
        i = 0 as libc::c_int as size_t;
        while i < (*A).size1 {
            j = 0 as libc::c_int as size_t;
            while j < (*A).size2 {
                let mut x: gsl_complex_float = gsl_matrix_complex_float_get(A, i, j);
                if !(x.dat[0 as libc::c_int as usize]
                    == 0 as libc::c_int as libc::c_float
                    && x.dat[1 as libc::c_int as usize]
                        == 0 as libc::c_int as libc::c_float)
                {
                    gsl_spmatrix_complex_float_set(T, i, j, x);
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_long_double_sp2d(
    mut A: *mut gsl_matrix_complex_long_double,
    mut S: *const gsl_spmatrix_complex_long_double,
) -> libc::c_int {
    if (*A).size1 != (*S).size1 || (*A).size2 != (*S).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            516 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        gsl_matrix_complex_long_double_set_zero(A);
        if (*S).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*S).nz {
                let mut i: libc::c_int = *((*S).i).offset(n as isize);
                let mut j: libc::c_int = *((*S).p).offset(n as isize);
                let mut x: gsl_complex_long_double = *(&mut *((*S).data)
                    .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
                    as *mut f128::f128 as *mut gsl_complex_long_double);
                gsl_matrix_complex_long_double_set(A, i as size_t, j as size_t, x);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*S).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Sj: *const libc::c_int = (*S).i;
            let mut Sp: *const libc::c_int = (*S).p;
            let mut Sd: *const f128::f128 = (*S).data;
            let mut i_0: size_t = 0;
            let mut p: libc::c_int = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*S).size1 {
                p = *Sp.offset(i_0 as isize);
                while p
                    < *Sp
                        .offset(
                            i_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let mut x_0: gsl_complex_long_double = gsl_complex_long_double {
                        dat: [f128::f128::ZERO; 2],
                    };
                    x_0
                        .dat[0 as libc::c_int
                        as usize] = *Sd.offset((2 as libc::c_int * p) as isize);
                    x_0
                        .dat[1 as libc::c_int
                        as usize] = *Sd
                        .offset((2 as libc::c_int * p + 1 as libc::c_int) as isize);
                    gsl_matrix_complex_long_double_set(
                        A,
                        i_0,
                        *Sj.offset(p as isize) as size_t,
                        x_0,
                    );
                    p += 1;
                    p;
                }
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else if (*S).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Si: *const libc::c_int = (*S).i;
            let mut Sp_0: *const libc::c_int = (*S).p;
            let mut Sd_0: *const f128::f128 = (*S).data;
            let mut j_0: size_t = 0;
            let mut p_0: libc::c_int = 0;
            j_0 = 0 as libc::c_int as size_t;
            while j_0 < (*S).size2 {
                p_0 = *Sp_0.offset(j_0 as isize);
                while p_0
                    < *Sp_0
                        .offset(
                            j_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let mut x_1: gsl_complex_long_double = gsl_complex_long_double {
                        dat: [f128::f128::ZERO; 2],
                    };
                    x_1
                        .dat[0 as libc::c_int
                        as usize] = *Sd_0.offset((2 as libc::c_int * p_0) as isize);
                    x_1
                        .dat[1 as libc::c_int
                        as usize] = *Sd_0
                        .offset((2 as libc::c_int * p_0 + 1 as libc::c_int) as isize);
                    gsl_matrix_complex_long_double_set(
                        A,
                        *Si.offset(p_0 as isize) as size_t,
                        j_0,
                        x_1,
                    );
                    p_0 += 1;
                    p_0;
                }
                j_0 = j_0.wrapping_add(1);
                j_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
                573 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_float_sp2d(
    mut A: *mut gsl_matrix_complex_float,
    mut S: *const gsl_spmatrix_complex_float,
) -> libc::c_int {
    if (*A).size1 != (*S).size1 || (*A).size2 != (*S).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            516 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        gsl_matrix_complex_float_set_zero(A);
        if (*S).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*S).nz {
                let mut i: libc::c_int = *((*S).i).offset(n as isize);
                let mut j: libc::c_int = *((*S).p).offset(n as isize);
                let mut x: gsl_complex_float = *(&mut *((*S).data)
                    .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
                    as *mut libc::c_float as *mut gsl_complex_float);
                gsl_matrix_complex_float_set(A, i as size_t, j as size_t, x);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*S).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Sj: *const libc::c_int = (*S).i;
            let mut Sp: *const libc::c_int = (*S).p;
            let mut Sd: *const libc::c_float = (*S).data;
            let mut i_0: size_t = 0;
            let mut p: libc::c_int = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*S).size1 {
                p = *Sp.offset(i_0 as isize);
                while p
                    < *Sp
                        .offset(
                            i_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let mut x_0: gsl_complex_float = gsl_complex_float { dat: [0.; 2] };
                    x_0
                        .dat[0 as libc::c_int
                        as usize] = *Sd.offset((2 as libc::c_int * p) as isize);
                    x_0
                        .dat[1 as libc::c_int
                        as usize] = *Sd
                        .offset((2 as libc::c_int * p + 1 as libc::c_int) as isize);
                    gsl_matrix_complex_float_set(
                        A,
                        i_0,
                        *Sj.offset(p as isize) as size_t,
                        x_0,
                    );
                    p += 1;
                    p;
                }
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else if (*S).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Si: *const libc::c_int = (*S).i;
            let mut Sp_0: *const libc::c_int = (*S).p;
            let mut Sd_0: *const libc::c_float = (*S).data;
            let mut j_0: size_t = 0;
            let mut p_0: libc::c_int = 0;
            j_0 = 0 as libc::c_int as size_t;
            while j_0 < (*S).size2 {
                p_0 = *Sp_0.offset(j_0 as isize);
                while p_0
                    < *Sp_0
                        .offset(
                            j_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let mut x_1: gsl_complex_float = gsl_complex_float { dat: [0.; 2] };
                    x_1
                        .dat[0 as libc::c_int
                        as usize] = *Sd_0.offset((2 as libc::c_int * p_0) as isize);
                    x_1
                        .dat[1 as libc::c_int
                        as usize] = *Sd_0
                        .offset((2 as libc::c_int * p_0 + 1 as libc::c_int) as isize);
                    gsl_matrix_complex_float_set(
                        A,
                        *Si.offset(p_0 as isize) as size_t,
                        j_0,
                        x_1,
                    );
                    p_0 += 1;
                    p_0;
                }
                j_0 = j_0.wrapping_add(1);
                j_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
                573 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_sp2d(
    mut A: *mut gsl_matrix_complex,
    mut S: *const gsl_spmatrix_complex,
) -> libc::c_int {
    if (*A).size1 != (*S).size1 || (*A).size2 != (*S).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            516 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        gsl_matrix_complex_set_zero(A);
        if (*S).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*S).nz {
                let mut i: libc::c_int = *((*S).i).offset(n as isize);
                let mut j: libc::c_int = *((*S).p).offset(n as isize);
                let mut x: gsl_complex = *(&mut *((*S).data)
                    .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
                    as *mut libc::c_double as *mut gsl_complex);
                gsl_matrix_complex_set(A, i as size_t, j as size_t, x);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*S).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Sj: *const libc::c_int = (*S).i;
            let mut Sp: *const libc::c_int = (*S).p;
            let mut Sd: *const libc::c_double = (*S).data;
            let mut i_0: size_t = 0;
            let mut p: libc::c_int = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*S).size1 {
                p = *Sp.offset(i_0 as isize);
                while p
                    < *Sp
                        .offset(
                            i_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let mut x_0: gsl_complex = gsl_complex { dat: [0.; 2] };
                    x_0
                        .dat[0 as libc::c_int
                        as usize] = *Sd.offset((2 as libc::c_int * p) as isize);
                    x_0
                        .dat[1 as libc::c_int
                        as usize] = *Sd
                        .offset((2 as libc::c_int * p + 1 as libc::c_int) as isize);
                    gsl_matrix_complex_set(
                        A,
                        i_0,
                        *Sj.offset(p as isize) as size_t,
                        x_0,
                    );
                    p += 1;
                    p;
                }
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else if (*S).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Si: *const libc::c_int = (*S).i;
            let mut Sp_0: *const libc::c_int = (*S).p;
            let mut Sd_0: *const libc::c_double = (*S).data;
            let mut j_0: size_t = 0;
            let mut p_0: libc::c_int = 0;
            j_0 = 0 as libc::c_int as size_t;
            while j_0 < (*S).size2 {
                p_0 = *Sp_0.offset(j_0 as isize);
                while p_0
                    < *Sp_0
                        .offset(
                            j_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let mut x_1: gsl_complex = gsl_complex { dat: [0.; 2] };
                    x_1
                        .dat[0 as libc::c_int
                        as usize] = *Sd_0.offset((2 as libc::c_int * p_0) as isize);
                    x_1
                        .dat[1 as libc::c_int
                        as usize] = *Sd_0
                        .offset((2 as libc::c_int * p_0 + 1 as libc::c_int) as isize);
                    gsl_matrix_complex_set(
                        A,
                        *Si.offset(p_0 as isize) as size_t,
                        j_0,
                        x_1,
                    );
                    p_0 += 1;
                    p_0;
                }
                j_0 = j_0.wrapping_add(1);
                j_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
                573 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn spmatrix_complex_long_double_scatter(
    mut A: *const gsl_spmatrix_complex_long_double,
    j: size_t,
    mut w: *mut libc::c_int,
    mut x: *mut f128::f128,
    mark: libc::c_int,
    mut C: *mut gsl_spmatrix_complex_long_double,
    mut nz: size_t,
) -> size_t {
    let mut p: libc::c_int = 0;
    let mut Ai: *mut libc::c_int = (*A).i;
    let mut Ap: *mut libc::c_int = (*A).p;
    let mut Ad: *mut f128::f128 = (*A).data;
    let mut Ci: *mut libc::c_int = (*C).i;
    p = *Ap.offset(j as isize);
    while p < *Ap.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize) {
        let mut i: libc::c_int = *Ai.offset(p as isize);
        if *w.offset(i as isize) < mark {
            *w.offset(i as isize) = mark;
            let fresh0 = nz;
            nz = nz.wrapping_add(1);
            *Ci.offset(fresh0 as isize) = i;
            *x
                .offset(
                    (2 as libc::c_int * i) as isize,
                ) = *Ad.offset((2 as libc::c_int * p) as isize);
            *x
                .offset(
                    (2 as libc::c_int * i + 1 as libc::c_int) as isize,
                ) = *Ad.offset((2 as libc::c_int * p + 1 as libc::c_int) as isize);
        } else {
            *x.offset((2 as libc::c_int * i) as isize)
                += *Ad.offset((2 as libc::c_int * p) as isize);
            *x.offset((2 as libc::c_int * i + 1 as libc::c_int) as isize)
                += *Ad.offset((2 as libc::c_int * p + 1 as libc::c_int) as isize);
        }
        p += 1;
        p;
    }
    return nz;
}
unsafe extern "C" fn spmatrix_complex_float_scatter(
    mut A: *const gsl_spmatrix_complex_float,
    j: size_t,
    mut w: *mut libc::c_int,
    mut x: *mut libc::c_float,
    mark: libc::c_int,
    mut C: *mut gsl_spmatrix_complex_float,
    mut nz: size_t,
) -> size_t {
    let mut p: libc::c_int = 0;
    let mut Ai: *mut libc::c_int = (*A).i;
    let mut Ap: *mut libc::c_int = (*A).p;
    let mut Ad: *mut libc::c_float = (*A).data;
    let mut Ci: *mut libc::c_int = (*C).i;
    p = *Ap.offset(j as isize);
    while p < *Ap.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize) {
        let mut i: libc::c_int = *Ai.offset(p as isize);
        if *w.offset(i as isize) < mark {
            *w.offset(i as isize) = mark;
            let fresh1 = nz;
            nz = nz.wrapping_add(1);
            *Ci.offset(fresh1 as isize) = i;
            *x
                .offset(
                    (2 as libc::c_int * i) as isize,
                ) = *Ad.offset((2 as libc::c_int * p) as isize);
            *x
                .offset(
                    (2 as libc::c_int * i + 1 as libc::c_int) as isize,
                ) = *Ad.offset((2 as libc::c_int * p + 1 as libc::c_int) as isize);
        } else {
            *x.offset((2 as libc::c_int * i) as isize)
                += *Ad.offset((2 as libc::c_int * p) as isize);
            *x.offset((2 as libc::c_int * i + 1 as libc::c_int) as isize)
                += *Ad.offset((2 as libc::c_int * p + 1 as libc::c_int) as isize);
        }
        p += 1;
        p;
    }
    return nz;
}
unsafe extern "C" fn spmatrix_complex_scatter(
    mut A: *const gsl_spmatrix_complex,
    j: size_t,
    mut w: *mut libc::c_int,
    mut x: *mut libc::c_double,
    mark: libc::c_int,
    mut C: *mut gsl_spmatrix_complex,
    mut nz: size_t,
) -> size_t {
    let mut p: libc::c_int = 0;
    let mut Ai: *mut libc::c_int = (*A).i;
    let mut Ap: *mut libc::c_int = (*A).p;
    let mut Ad: *mut libc::c_double = (*A).data;
    let mut Ci: *mut libc::c_int = (*C).i;
    p = *Ap.offset(j as isize);
    while p < *Ap.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize) {
        let mut i: libc::c_int = *Ai.offset(p as isize);
        if *w.offset(i as isize) < mark {
            *w.offset(i as isize) = mark;
            let fresh2 = nz;
            nz = nz.wrapping_add(1);
            *Ci.offset(fresh2 as isize) = i;
            *x
                .offset(
                    (2 as libc::c_int * i) as isize,
                ) = *Ad.offset((2 as libc::c_int * p) as isize);
            *x
                .offset(
                    (2 as libc::c_int * i + 1 as libc::c_int) as isize,
                ) = *Ad.offset((2 as libc::c_int * p + 1 as libc::c_int) as isize);
        } else {
            *x.offset((2 as libc::c_int * i) as isize)
                += *Ad.offset((2 as libc::c_int * p) as isize);
            *x.offset((2 as libc::c_int * i + 1 as libc::c_int) as isize)
                += *Ad.offset((2 as libc::c_int * p + 1 as libc::c_int) as isize);
        }
        p += 1;
        p;
    }
    return nz;
}
pub unsafe extern "C" fn gsl_spmatrix_ulong_scale(
    mut m: *mut gsl_spmatrix_ulong,
    x: libc::c_ulong,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*m).nz {
        let ref mut fresh3 = *((*m).data).offset(i as isize);
        *fresh3 = (*fresh3).wrapping_mul(x);
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_short_scale(
    mut m: *mut gsl_spmatrix_short,
    x: libc::c_short,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*m).nz {
        let ref mut fresh4 = *((*m).data).offset(i as isize);
        *fresh4 = (*fresh4 as libc::c_int * x as libc::c_int) as libc::c_short;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_int_scale(
    mut m: *mut gsl_spmatrix_int,
    x: libc::c_int,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*m).nz {
        *((*m).data).offset(i as isize) *= x;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_long_scale(
    mut m: *mut gsl_spmatrix_long,
    x: libc::c_long,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*m).nz {
        *((*m).data).offset(i as isize) *= x;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_uchar_scale(
    mut m: *mut gsl_spmatrix_uchar,
    x: libc::c_uchar,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*m).nz {
        let ref mut fresh5 = *((*m).data).offset(i as isize);
        *fresh5 = (*fresh5 as libc::c_int * x as libc::c_int) as libc::c_uchar;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_scale(
    mut m: *mut gsl_spmatrix_long_double,
    x: f128::f128,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*m).nz {
        *((*m).data).offset(i as isize) *= x;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_float_scale(
    mut m: *mut gsl_spmatrix_float,
    x: libc::c_float,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*m).nz {
        *((*m).data).offset(i as isize) *= x;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_char_scale(
    mut m: *mut gsl_spmatrix_char,
    x: libc::c_char,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*m).nz {
        let ref mut fresh6 = *((*m).data).offset(i as isize);
        *fresh6 = (*fresh6 as libc::c_int * x as libc::c_int) as libc::c_char;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_uint_scale(
    mut m: *mut gsl_spmatrix_uint,
    x: libc::c_uint,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*m).nz {
        let ref mut fresh7 = *((*m).data).offset(i as isize);
        *fresh7 = (*fresh7).wrapping_mul(x);
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_scale(
    mut m: *mut gsl_spmatrix,
    x: libc::c_double,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*m).nz {
        *((*m).data).offset(i as isize) *= x;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_ushort_scale(
    mut m: *mut gsl_spmatrix_ushort,
    x: libc::c_ushort,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*m).nz {
        let ref mut fresh8 = *((*m).data).offset(i as isize);
        *fresh8 = (*fresh8 as libc::c_int * x as libc::c_int) as libc::c_ushort;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_scale_columns(
    mut m: *mut gsl_spmatrix,
    mut x: *const gsl_vector,
) -> libc::c_int {
    if (*m).size2 != (*x).size {
        gsl_error(
            b"x vector length does not match matrix\0" as *const u8
                as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut Ad: *mut libc::c_double = (*m).data;
        if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < (*m).size2 {
                let mut xj: libc::c_double = gsl_vector_get(x, j);
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    *Ad.offset(p as isize) *= xj;
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Aj: *const libc::c_int = (*m).i;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).nz {
                let mut y: libc::c_double = gsl_vector_get(
                    x,
                    *Aj.offset(i as isize) as size_t,
                );
                *Ad.offset(i as isize) *= y;
                i = i.wrapping_add(1);
                i;
            }
        } else if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut Aj_0: *const libc::c_int = (*m).p;
            let mut i_0: size_t = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*m).nz {
                let mut y_0: libc::c_double = gsl_vector_get(
                    x,
                    *Aj_0.offset(i_0 as isize) as size_t,
                );
                *Ad.offset(i_0 as isize) *= y_0;
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                85 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_int_scale_columns(
    mut m: *mut gsl_spmatrix_int,
    mut x: *const gsl_vector_int,
) -> libc::c_int {
    if (*m).size2 != (*x).size {
        gsl_error(
            b"x vector length does not match matrix\0" as *const u8
                as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut Ad: *mut libc::c_int = (*m).data;
        if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < (*m).size2 {
                let mut xj: libc::c_int = gsl_vector_int_get(x, j);
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    *Ad.offset(p as isize) *= xj;
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Aj: *const libc::c_int = (*m).i;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).nz {
                let mut y: libc::c_int = gsl_vector_int_get(
                    x,
                    *Aj.offset(i as isize) as size_t,
                );
                *Ad.offset(i as isize) *= y;
                i = i.wrapping_add(1);
                i;
            }
        } else if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut Aj_0: *const libc::c_int = (*m).p;
            let mut i_0: size_t = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*m).nz {
                let mut y_0: libc::c_int = gsl_vector_int_get(
                    x,
                    *Aj_0.offset(i_0 as isize) as size_t,
                );
                *Ad.offset(i_0 as isize) *= y_0;
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                85 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uint_scale_columns(
    mut m: *mut gsl_spmatrix_uint,
    mut x: *const gsl_vector_uint,
) -> libc::c_int {
    if (*m).size2 != (*x).size {
        gsl_error(
            b"x vector length does not match matrix\0" as *const u8
                as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut Ad: *mut libc::c_uint = (*m).data;
        if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < (*m).size2 {
                let mut xj: libc::c_uint = gsl_vector_uint_get(x, j);
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh9 = *Ad.offset(p as isize);
                    *fresh9 = (*fresh9).wrapping_mul(xj);
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Aj: *const libc::c_int = (*m).i;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).nz {
                let mut y: libc::c_uint = gsl_vector_uint_get(
                    x,
                    *Aj.offset(i as isize) as size_t,
                );
                let ref mut fresh10 = *Ad.offset(i as isize);
                *fresh10 = (*fresh10).wrapping_mul(y);
                i = i.wrapping_add(1);
                i;
            }
        } else if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut Aj_0: *const libc::c_int = (*m).p;
            let mut i_0: size_t = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*m).nz {
                let mut y_0: libc::c_uint = gsl_vector_uint_get(
                    x,
                    *Aj_0.offset(i_0 as isize) as size_t,
                );
                let ref mut fresh11 = *Ad.offset(i_0 as isize);
                *fresh11 = (*fresh11).wrapping_mul(y_0);
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                85 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uchar_scale_columns(
    mut m: *mut gsl_spmatrix_uchar,
    mut x: *const gsl_vector_uchar,
) -> libc::c_int {
    if (*m).size2 != (*x).size {
        gsl_error(
            b"x vector length does not match matrix\0" as *const u8
                as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut Ad: *mut libc::c_uchar = (*m).data;
        if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < (*m).size2 {
                let mut xj: libc::c_uchar = gsl_vector_uchar_get(x, j);
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh12 = *Ad.offset(p as isize);
                    *fresh12 = (*fresh12 as libc::c_int * xj as libc::c_int)
                        as libc::c_uchar;
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Aj: *const libc::c_int = (*m).i;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).nz {
                let mut y: libc::c_uchar = gsl_vector_uchar_get(
                    x,
                    *Aj.offset(i as isize) as size_t,
                );
                let ref mut fresh13 = *Ad.offset(i as isize);
                *fresh13 = (*fresh13 as libc::c_int * y as libc::c_int) as libc::c_uchar;
                i = i.wrapping_add(1);
                i;
            }
        } else if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut Aj_0: *const libc::c_int = (*m).p;
            let mut i_0: size_t = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*m).nz {
                let mut y_0: libc::c_uchar = gsl_vector_uchar_get(
                    x,
                    *Aj_0.offset(i_0 as isize) as size_t,
                );
                let ref mut fresh14 = *Ad.offset(i_0 as isize);
                *fresh14 = (*fresh14 as libc::c_int * y_0 as libc::c_int)
                    as libc::c_uchar;
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                85 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_char_scale_columns(
    mut m: *mut gsl_spmatrix_char,
    mut x: *const gsl_vector_char,
) -> libc::c_int {
    if (*m).size2 != (*x).size {
        gsl_error(
            b"x vector length does not match matrix\0" as *const u8
                as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut Ad: *mut libc::c_char = (*m).data;
        if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < (*m).size2 {
                let mut xj: libc::c_char = gsl_vector_char_get(x, j);
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh15 = *Ad.offset(p as isize);
                    *fresh15 = (*fresh15 as libc::c_int * xj as libc::c_int)
                        as libc::c_char;
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Aj: *const libc::c_int = (*m).i;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).nz {
                let mut y: libc::c_char = gsl_vector_char_get(
                    x,
                    *Aj.offset(i as isize) as size_t,
                );
                let ref mut fresh16 = *Ad.offset(i as isize);
                *fresh16 = (*fresh16 as libc::c_int * y as libc::c_int) as libc::c_char;
                i = i.wrapping_add(1);
                i;
            }
        } else if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut Aj_0: *const libc::c_int = (*m).p;
            let mut i_0: size_t = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*m).nz {
                let mut y_0: libc::c_char = gsl_vector_char_get(
                    x,
                    *Aj_0.offset(i_0 as isize) as size_t,
                );
                let ref mut fresh17 = *Ad.offset(i_0 as isize);
                *fresh17 = (*fresh17 as libc::c_int * y_0 as libc::c_int)
                    as libc::c_char;
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                85 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_float_scale_columns(
    mut m: *mut gsl_spmatrix_float,
    mut x: *const gsl_vector_float,
) -> libc::c_int {
    if (*m).size2 != (*x).size {
        gsl_error(
            b"x vector length does not match matrix\0" as *const u8
                as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut Ad: *mut libc::c_float = (*m).data;
        if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < (*m).size2 {
                let mut xj: libc::c_float = gsl_vector_float_get(x, j);
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    *Ad.offset(p as isize) *= xj;
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Aj: *const libc::c_int = (*m).i;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).nz {
                let mut y: libc::c_float = gsl_vector_float_get(
                    x,
                    *Aj.offset(i as isize) as size_t,
                );
                *Ad.offset(i as isize) *= y;
                i = i.wrapping_add(1);
                i;
            }
        } else if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut Aj_0: *const libc::c_int = (*m).p;
            let mut i_0: size_t = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*m).nz {
                let mut y_0: libc::c_float = gsl_vector_float_get(
                    x,
                    *Aj_0.offset(i_0 as isize) as size_t,
                );
                *Ad.offset(i_0 as isize) *= y_0;
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                85 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_ulong_scale_columns(
    mut m: *mut gsl_spmatrix_ulong,
    mut x: *const gsl_vector_ulong,
) -> libc::c_int {
    if (*m).size2 != (*x).size {
        gsl_error(
            b"x vector length does not match matrix\0" as *const u8
                as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut Ad: *mut libc::c_ulong = (*m).data;
        if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < (*m).size2 {
                let mut xj: libc::c_ulong = gsl_vector_ulong_get(x, j);
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh18 = *Ad.offset(p as isize);
                    *fresh18 = (*fresh18).wrapping_mul(xj);
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Aj: *const libc::c_int = (*m).i;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).nz {
                let mut y: libc::c_ulong = gsl_vector_ulong_get(
                    x,
                    *Aj.offset(i as isize) as size_t,
                );
                let ref mut fresh19 = *Ad.offset(i as isize);
                *fresh19 = (*fresh19).wrapping_mul(y);
                i = i.wrapping_add(1);
                i;
            }
        } else if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut Aj_0: *const libc::c_int = (*m).p;
            let mut i_0: size_t = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*m).nz {
                let mut y_0: libc::c_ulong = gsl_vector_ulong_get(
                    x,
                    *Aj_0.offset(i_0 as isize) as size_t,
                );
                let ref mut fresh20 = *Ad.offset(i_0 as isize);
                *fresh20 = (*fresh20).wrapping_mul(y_0);
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                85 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_ushort_scale_columns(
    mut m: *mut gsl_spmatrix_ushort,
    mut x: *const gsl_vector_ushort,
) -> libc::c_int {
    if (*m).size2 != (*x).size {
        gsl_error(
            b"x vector length does not match matrix\0" as *const u8
                as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut Ad: *mut libc::c_ushort = (*m).data;
        if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < (*m).size2 {
                let mut xj: libc::c_ushort = gsl_vector_ushort_get(x, j);
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh21 = *Ad.offset(p as isize);
                    *fresh21 = (*fresh21 as libc::c_int * xj as libc::c_int)
                        as libc::c_ushort;
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Aj: *const libc::c_int = (*m).i;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).nz {
                let mut y: libc::c_ushort = gsl_vector_ushort_get(
                    x,
                    *Aj.offset(i as isize) as size_t,
                );
                let ref mut fresh22 = *Ad.offset(i as isize);
                *fresh22 = (*fresh22 as libc::c_int * y as libc::c_int)
                    as libc::c_ushort;
                i = i.wrapping_add(1);
                i;
            }
        } else if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut Aj_0: *const libc::c_int = (*m).p;
            let mut i_0: size_t = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*m).nz {
                let mut y_0: libc::c_ushort = gsl_vector_ushort_get(
                    x,
                    *Aj_0.offset(i_0 as isize) as size_t,
                );
                let ref mut fresh23 = *Ad.offset(i_0 as isize);
                *fresh23 = (*fresh23 as libc::c_int * y_0 as libc::c_int)
                    as libc::c_ushort;
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                85 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_scale_columns(
    mut m: *mut gsl_spmatrix_long_double,
    mut x: *const gsl_vector_long_double,
) -> libc::c_int {
    if (*m).size2 != (*x).size {
        gsl_error(
            b"x vector length does not match matrix\0" as *const u8
                as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut Ad: *mut f128::f128 = (*m).data;
        if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < (*m).size2 {
                let mut xj: f128::f128 = gsl_vector_long_double_get(x, j);
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    *Ad.offset(p as isize) *= xj;
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Aj: *const libc::c_int = (*m).i;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).nz {
                let mut y: f128::f128 = gsl_vector_long_double_get(
                    x,
                    *Aj.offset(i as isize) as size_t,
                );
                *Ad.offset(i as isize) *= y;
                i = i.wrapping_add(1);
                i;
            }
        } else if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut Aj_0: *const libc::c_int = (*m).p;
            let mut i_0: size_t = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*m).nz {
                let mut y_0: f128::f128 = gsl_vector_long_double_get(
                    x,
                    *Aj_0.offset(i_0 as isize) as size_t,
                );
                *Ad.offset(i_0 as isize) *= y_0;
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                85 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_short_scale_columns(
    mut m: *mut gsl_spmatrix_short,
    mut x: *const gsl_vector_short,
) -> libc::c_int {
    if (*m).size2 != (*x).size {
        gsl_error(
            b"x vector length does not match matrix\0" as *const u8
                as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut Ad: *mut libc::c_short = (*m).data;
        if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < (*m).size2 {
                let mut xj: libc::c_short = gsl_vector_short_get(x, j);
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh24 = *Ad.offset(p as isize);
                    *fresh24 = (*fresh24 as libc::c_int * xj as libc::c_int)
                        as libc::c_short;
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Aj: *const libc::c_int = (*m).i;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).nz {
                let mut y: libc::c_short = gsl_vector_short_get(
                    x,
                    *Aj.offset(i as isize) as size_t,
                );
                let ref mut fresh25 = *Ad.offset(i as isize);
                *fresh25 = (*fresh25 as libc::c_int * y as libc::c_int) as libc::c_short;
                i = i.wrapping_add(1);
                i;
            }
        } else if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut Aj_0: *const libc::c_int = (*m).p;
            let mut i_0: size_t = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*m).nz {
                let mut y_0: libc::c_short = gsl_vector_short_get(
                    x,
                    *Aj_0.offset(i_0 as isize) as size_t,
                );
                let ref mut fresh26 = *Ad.offset(i_0 as isize);
                *fresh26 = (*fresh26 as libc::c_int * y_0 as libc::c_int)
                    as libc::c_short;
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                85 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_long_scale_columns(
    mut m: *mut gsl_spmatrix_long,
    mut x: *const gsl_vector_long,
) -> libc::c_int {
    if (*m).size2 != (*x).size {
        gsl_error(
            b"x vector length does not match matrix\0" as *const u8
                as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut Ad: *mut libc::c_long = (*m).data;
        if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut j: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < (*m).size2 {
                let mut xj: libc::c_long = gsl_vector_long_get(x, j);
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    *Ad.offset(p as isize) *= xj;
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Aj: *const libc::c_int = (*m).i;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).nz {
                let mut y: libc::c_long = gsl_vector_long_get(
                    x,
                    *Aj.offset(i as isize) as size_t,
                );
                *Ad.offset(i as isize) *= y;
                i = i.wrapping_add(1);
                i;
            }
        } else if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut Aj_0: *const libc::c_int = (*m).p;
            let mut i_0: size_t = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*m).nz {
                let mut y_0: libc::c_long = gsl_vector_long_get(
                    x,
                    *Aj_0.offset(i_0 as isize) as size_t,
                );
                *Ad.offset(i_0 as isize) *= y_0;
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                85 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_scale_rows(
    mut m: *mut gsl_spmatrix,
    mut x: *const gsl_vector,
) -> libc::c_int {
    if (*m).size1 != (*x).size {
        gsl_error(
            b"x vector length does not match matrix\0" as *const u8
                as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            98 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut Ad: *mut libc::c_double = (*m).data;
        if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ai: *const libc::c_int = (*m).i;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).nz {
                let mut y: libc::c_double = gsl_vector_get(
                    x,
                    *Ai.offset(i as isize) as size_t,
                );
                *Ad.offset(i as isize) *= y;
                i = i.wrapping_add(1);
                i;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut i_0: size_t = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*m).size1 {
                let mut xi: libc::c_double = gsl_vector_get(x, i_0);
                p = *Ap.offset(i_0 as isize);
                while p
                    < *Ap
                        .offset(
                            i_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    *Ad.offset(p as isize) *= xi;
                    p += 1;
                    p;
                }
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut Ai_0: *const libc::c_int = (*m).i;
            let mut i_1: size_t = 0;
            i_1 = 0 as libc::c_int as size_t;
            while i_1 < (*m).nz {
                let mut y_0: libc::c_double = gsl_vector_get(
                    x,
                    *Ai_0.offset(i_1 as isize) as size_t,
                );
                *Ad.offset(i_1 as isize) *= y_0;
                i_1 = i_1.wrapping_add(1);
                i_1;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                142 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_int_scale_rows(
    mut m: *mut gsl_spmatrix_int,
    mut x: *const gsl_vector_int,
) -> libc::c_int {
    if (*m).size1 != (*x).size {
        gsl_error(
            b"x vector length does not match matrix\0" as *const u8
                as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            98 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut Ad: *mut libc::c_int = (*m).data;
        if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ai: *const libc::c_int = (*m).i;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).nz {
                let mut y: libc::c_int = gsl_vector_int_get(
                    x,
                    *Ai.offset(i as isize) as size_t,
                );
                *Ad.offset(i as isize) *= y;
                i = i.wrapping_add(1);
                i;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut i_0: size_t = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*m).size1 {
                let mut xi: libc::c_int = gsl_vector_int_get(x, i_0);
                p = *Ap.offset(i_0 as isize);
                while p
                    < *Ap
                        .offset(
                            i_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    *Ad.offset(p as isize) *= xi;
                    p += 1;
                    p;
                }
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut Ai_0: *const libc::c_int = (*m).i;
            let mut i_1: size_t = 0;
            i_1 = 0 as libc::c_int as size_t;
            while i_1 < (*m).nz {
                let mut y_0: libc::c_int = gsl_vector_int_get(
                    x,
                    *Ai_0.offset(i_1 as isize) as size_t,
                );
                *Ad.offset(i_1 as isize) *= y_0;
                i_1 = i_1.wrapping_add(1);
                i_1;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                142 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_scale_rows(
    mut m: *mut gsl_spmatrix_long_double,
    mut x: *const gsl_vector_long_double,
) -> libc::c_int {
    if (*m).size1 != (*x).size {
        gsl_error(
            b"x vector length does not match matrix\0" as *const u8
                as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            98 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut Ad: *mut f128::f128 = (*m).data;
        if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ai: *const libc::c_int = (*m).i;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).nz {
                let mut y: f128::f128 = gsl_vector_long_double_get(
                    x,
                    *Ai.offset(i as isize) as size_t,
                );
                *Ad.offset(i as isize) *= y;
                i = i.wrapping_add(1);
                i;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut i_0: size_t = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*m).size1 {
                let mut xi: f128::f128 = gsl_vector_long_double_get(x, i_0);
                p = *Ap.offset(i_0 as isize);
                while p
                    < *Ap
                        .offset(
                            i_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    *Ad.offset(p as isize) *= xi;
                    p += 1;
                    p;
                }
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut Ai_0: *const libc::c_int = (*m).i;
            let mut i_1: size_t = 0;
            i_1 = 0 as libc::c_int as size_t;
            while i_1 < (*m).nz {
                let mut y_0: f128::f128 = gsl_vector_long_double_get(
                    x,
                    *Ai_0.offset(i_1 as isize) as size_t,
                );
                *Ad.offset(i_1 as isize) *= y_0;
                i_1 = i_1.wrapping_add(1);
                i_1;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                142 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uint_scale_rows(
    mut m: *mut gsl_spmatrix_uint,
    mut x: *const gsl_vector_uint,
) -> libc::c_int {
    if (*m).size1 != (*x).size {
        gsl_error(
            b"x vector length does not match matrix\0" as *const u8
                as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            98 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut Ad: *mut libc::c_uint = (*m).data;
        if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ai: *const libc::c_int = (*m).i;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).nz {
                let mut y: libc::c_uint = gsl_vector_uint_get(
                    x,
                    *Ai.offset(i as isize) as size_t,
                );
                let ref mut fresh27 = *Ad.offset(i as isize);
                *fresh27 = (*fresh27).wrapping_mul(y);
                i = i.wrapping_add(1);
                i;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut i_0: size_t = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*m).size1 {
                let mut xi: libc::c_uint = gsl_vector_uint_get(x, i_0);
                p = *Ap.offset(i_0 as isize);
                while p
                    < *Ap
                        .offset(
                            i_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh28 = *Ad.offset(p as isize);
                    *fresh28 = (*fresh28).wrapping_mul(xi);
                    p += 1;
                    p;
                }
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut Ai_0: *const libc::c_int = (*m).i;
            let mut i_1: size_t = 0;
            i_1 = 0 as libc::c_int as size_t;
            while i_1 < (*m).nz {
                let mut y_0: libc::c_uint = gsl_vector_uint_get(
                    x,
                    *Ai_0.offset(i_1 as isize) as size_t,
                );
                let ref mut fresh29 = *Ad.offset(i_1 as isize);
                *fresh29 = (*fresh29).wrapping_mul(y_0);
                i_1 = i_1.wrapping_add(1);
                i_1;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                142 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_char_scale_rows(
    mut m: *mut gsl_spmatrix_char,
    mut x: *const gsl_vector_char,
) -> libc::c_int {
    if (*m).size1 != (*x).size {
        gsl_error(
            b"x vector length does not match matrix\0" as *const u8
                as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            98 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut Ad: *mut libc::c_char = (*m).data;
        if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ai: *const libc::c_int = (*m).i;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).nz {
                let mut y: libc::c_char = gsl_vector_char_get(
                    x,
                    *Ai.offset(i as isize) as size_t,
                );
                let ref mut fresh30 = *Ad.offset(i as isize);
                *fresh30 = (*fresh30 as libc::c_int * y as libc::c_int) as libc::c_char;
                i = i.wrapping_add(1);
                i;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut i_0: size_t = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*m).size1 {
                let mut xi: libc::c_char = gsl_vector_char_get(x, i_0);
                p = *Ap.offset(i_0 as isize);
                while p
                    < *Ap
                        .offset(
                            i_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh31 = *Ad.offset(p as isize);
                    *fresh31 = (*fresh31 as libc::c_int * xi as libc::c_int)
                        as libc::c_char;
                    p += 1;
                    p;
                }
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut Ai_0: *const libc::c_int = (*m).i;
            let mut i_1: size_t = 0;
            i_1 = 0 as libc::c_int as size_t;
            while i_1 < (*m).nz {
                let mut y_0: libc::c_char = gsl_vector_char_get(
                    x,
                    *Ai_0.offset(i_1 as isize) as size_t,
                );
                let ref mut fresh32 = *Ad.offset(i_1 as isize);
                *fresh32 = (*fresh32 as libc::c_int * y_0 as libc::c_int)
                    as libc::c_char;
                i_1 = i_1.wrapping_add(1);
                i_1;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                142 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_long_scale_rows(
    mut m: *mut gsl_spmatrix_long,
    mut x: *const gsl_vector_long,
) -> libc::c_int {
    if (*m).size1 != (*x).size {
        gsl_error(
            b"x vector length does not match matrix\0" as *const u8
                as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            98 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut Ad: *mut libc::c_long = (*m).data;
        if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ai: *const libc::c_int = (*m).i;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).nz {
                let mut y: libc::c_long = gsl_vector_long_get(
                    x,
                    *Ai.offset(i as isize) as size_t,
                );
                *Ad.offset(i as isize) *= y;
                i = i.wrapping_add(1);
                i;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut i_0: size_t = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*m).size1 {
                let mut xi: libc::c_long = gsl_vector_long_get(x, i_0);
                p = *Ap.offset(i_0 as isize);
                while p
                    < *Ap
                        .offset(
                            i_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    *Ad.offset(p as isize) *= xi;
                    p += 1;
                    p;
                }
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut Ai_0: *const libc::c_int = (*m).i;
            let mut i_1: size_t = 0;
            i_1 = 0 as libc::c_int as size_t;
            while i_1 < (*m).nz {
                let mut y_0: libc::c_long = gsl_vector_long_get(
                    x,
                    *Ai_0.offset(i_1 as isize) as size_t,
                );
                *Ad.offset(i_1 as isize) *= y_0;
                i_1 = i_1.wrapping_add(1);
                i_1;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                142 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uchar_scale_rows(
    mut m: *mut gsl_spmatrix_uchar,
    mut x: *const gsl_vector_uchar,
) -> libc::c_int {
    if (*m).size1 != (*x).size {
        gsl_error(
            b"x vector length does not match matrix\0" as *const u8
                as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            98 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut Ad: *mut libc::c_uchar = (*m).data;
        if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ai: *const libc::c_int = (*m).i;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).nz {
                let mut y: libc::c_uchar = gsl_vector_uchar_get(
                    x,
                    *Ai.offset(i as isize) as size_t,
                );
                let ref mut fresh33 = *Ad.offset(i as isize);
                *fresh33 = (*fresh33 as libc::c_int * y as libc::c_int) as libc::c_uchar;
                i = i.wrapping_add(1);
                i;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut i_0: size_t = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*m).size1 {
                let mut xi: libc::c_uchar = gsl_vector_uchar_get(x, i_0);
                p = *Ap.offset(i_0 as isize);
                while p
                    < *Ap
                        .offset(
                            i_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh34 = *Ad.offset(p as isize);
                    *fresh34 = (*fresh34 as libc::c_int * xi as libc::c_int)
                        as libc::c_uchar;
                    p += 1;
                    p;
                }
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut Ai_0: *const libc::c_int = (*m).i;
            let mut i_1: size_t = 0;
            i_1 = 0 as libc::c_int as size_t;
            while i_1 < (*m).nz {
                let mut y_0: libc::c_uchar = gsl_vector_uchar_get(
                    x,
                    *Ai_0.offset(i_1 as isize) as size_t,
                );
                let ref mut fresh35 = *Ad.offset(i_1 as isize);
                *fresh35 = (*fresh35 as libc::c_int * y_0 as libc::c_int)
                    as libc::c_uchar;
                i_1 = i_1.wrapping_add(1);
                i_1;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                142 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_short_scale_rows(
    mut m: *mut gsl_spmatrix_short,
    mut x: *const gsl_vector_short,
) -> libc::c_int {
    if (*m).size1 != (*x).size {
        gsl_error(
            b"x vector length does not match matrix\0" as *const u8
                as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            98 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut Ad: *mut libc::c_short = (*m).data;
        if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ai: *const libc::c_int = (*m).i;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).nz {
                let mut y: libc::c_short = gsl_vector_short_get(
                    x,
                    *Ai.offset(i as isize) as size_t,
                );
                let ref mut fresh36 = *Ad.offset(i as isize);
                *fresh36 = (*fresh36 as libc::c_int * y as libc::c_int) as libc::c_short;
                i = i.wrapping_add(1);
                i;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut i_0: size_t = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*m).size1 {
                let mut xi: libc::c_short = gsl_vector_short_get(x, i_0);
                p = *Ap.offset(i_0 as isize);
                while p
                    < *Ap
                        .offset(
                            i_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh37 = *Ad.offset(p as isize);
                    *fresh37 = (*fresh37 as libc::c_int * xi as libc::c_int)
                        as libc::c_short;
                    p += 1;
                    p;
                }
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut Ai_0: *const libc::c_int = (*m).i;
            let mut i_1: size_t = 0;
            i_1 = 0 as libc::c_int as size_t;
            while i_1 < (*m).nz {
                let mut y_0: libc::c_short = gsl_vector_short_get(
                    x,
                    *Ai_0.offset(i_1 as isize) as size_t,
                );
                let ref mut fresh38 = *Ad.offset(i_1 as isize);
                *fresh38 = (*fresh38 as libc::c_int * y_0 as libc::c_int)
                    as libc::c_short;
                i_1 = i_1.wrapping_add(1);
                i_1;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                142 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_ushort_scale_rows(
    mut m: *mut gsl_spmatrix_ushort,
    mut x: *const gsl_vector_ushort,
) -> libc::c_int {
    if (*m).size1 != (*x).size {
        gsl_error(
            b"x vector length does not match matrix\0" as *const u8
                as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            98 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut Ad: *mut libc::c_ushort = (*m).data;
        if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ai: *const libc::c_int = (*m).i;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).nz {
                let mut y: libc::c_ushort = gsl_vector_ushort_get(
                    x,
                    *Ai.offset(i as isize) as size_t,
                );
                let ref mut fresh39 = *Ad.offset(i as isize);
                *fresh39 = (*fresh39 as libc::c_int * y as libc::c_int)
                    as libc::c_ushort;
                i = i.wrapping_add(1);
                i;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut i_0: size_t = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*m).size1 {
                let mut xi: libc::c_ushort = gsl_vector_ushort_get(x, i_0);
                p = *Ap.offset(i_0 as isize);
                while p
                    < *Ap
                        .offset(
                            i_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh40 = *Ad.offset(p as isize);
                    *fresh40 = (*fresh40 as libc::c_int * xi as libc::c_int)
                        as libc::c_ushort;
                    p += 1;
                    p;
                }
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut Ai_0: *const libc::c_int = (*m).i;
            let mut i_1: size_t = 0;
            i_1 = 0 as libc::c_int as size_t;
            while i_1 < (*m).nz {
                let mut y_0: libc::c_ushort = gsl_vector_ushort_get(
                    x,
                    *Ai_0.offset(i_1 as isize) as size_t,
                );
                let ref mut fresh41 = *Ad.offset(i_1 as isize);
                *fresh41 = (*fresh41 as libc::c_int * y_0 as libc::c_int)
                    as libc::c_ushort;
                i_1 = i_1.wrapping_add(1);
                i_1;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                142 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_float_scale_rows(
    mut m: *mut gsl_spmatrix_float,
    mut x: *const gsl_vector_float,
) -> libc::c_int {
    if (*m).size1 != (*x).size {
        gsl_error(
            b"x vector length does not match matrix\0" as *const u8
                as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            98 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut Ad: *mut libc::c_float = (*m).data;
        if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ai: *const libc::c_int = (*m).i;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).nz {
                let mut y: libc::c_float = gsl_vector_float_get(
                    x,
                    *Ai.offset(i as isize) as size_t,
                );
                *Ad.offset(i as isize) *= y;
                i = i.wrapping_add(1);
                i;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut i_0: size_t = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*m).size1 {
                let mut xi: libc::c_float = gsl_vector_float_get(x, i_0);
                p = *Ap.offset(i_0 as isize);
                while p
                    < *Ap
                        .offset(
                            i_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    *Ad.offset(p as isize) *= xi;
                    p += 1;
                    p;
                }
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut Ai_0: *const libc::c_int = (*m).i;
            let mut i_1: size_t = 0;
            i_1 = 0 as libc::c_int as size_t;
            while i_1 < (*m).nz {
                let mut y_0: libc::c_float = gsl_vector_float_get(
                    x,
                    *Ai_0.offset(i_1 as isize) as size_t,
                );
                *Ad.offset(i_1 as isize) *= y_0;
                i_1 = i_1.wrapping_add(1);
                i_1;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                142 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_ulong_scale_rows(
    mut m: *mut gsl_spmatrix_ulong,
    mut x: *const gsl_vector_ulong,
) -> libc::c_int {
    if (*m).size1 != (*x).size {
        gsl_error(
            b"x vector length does not match matrix\0" as *const u8
                as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            98 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut Ad: *mut libc::c_ulong = (*m).data;
        if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Ai: *const libc::c_int = (*m).i;
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*m).nz {
                let mut y: libc::c_ulong = gsl_vector_ulong_get(
                    x,
                    *Ai.offset(i as isize) as size_t,
                );
                let ref mut fresh42 = *Ad.offset(i as isize);
                *fresh42 = (*fresh42).wrapping_mul(y);
                i = i.wrapping_add(1);
                i;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Ap: *const libc::c_int = (*m).p;
            let mut p: libc::c_int = 0;
            let mut i_0: size_t = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*m).size1 {
                let mut xi: libc::c_ulong = gsl_vector_ulong_get(x, i_0);
                p = *Ap.offset(i_0 as isize);
                while p
                    < *Ap
                        .offset(
                            i_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh43 = *Ad.offset(p as isize);
                    *fresh43 = (*fresh43).wrapping_mul(xi);
                    p += 1;
                    p;
                }
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut Ai_0: *const libc::c_int = (*m).i;
            let mut i_1: size_t = 0;
            i_1 = 0 as libc::c_int as size_t;
            while i_1 < (*m).nz {
                let mut y_0: libc::c_ulong = gsl_vector_ulong_get(
                    x,
                    *Ai_0.offset(i_1 as isize) as size_t,
                );
                let ref mut fresh44 = *Ad.offset(i_1 as isize);
                *fresh44 = (*fresh44).wrapping_mul(y_0);
                i_1 = i_1.wrapping_add(1);
                i_1;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                142 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_ulong_add(
    mut c: *mut gsl_spmatrix_ulong,
    mut a: *const gsl_spmatrix_ulong,
    mut b: *const gsl_spmatrix_ulong,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N || (*c).size1 != M || (*c).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            169 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*a).sptype != (*b).sptype || (*a).sptype != (*c).sptype {
        gsl_error(
            b"matrices must have same sparse storage format\0" as *const u8
                as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*a).sptype == GSL_SPMATRIX_COO as libc::c_int {
        gsl_error(
            b"COO format not yet supported\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            178 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut w: *mut libc::c_int = (*a).work.work_int;
        let mut x: *mut libc::c_ulong = (*c).work.work_atomic;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut Ci: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut Cd: *mut libc::c_ulong = 0 as *mut libc::c_ulong;
        let mut p: libc::c_int = 0;
        let mut j: size_t = 0;
        let mut nz: size_t = 0 as libc::c_int as size_t;
        let mut inner_size: size_t = 0;
        let mut outer_size: size_t = 0;
        if (*a).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            inner_size = M;
            outer_size = N;
        } else if (*a).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            inner_size = N;
            outer_size = M;
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                204 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        if (*c).nzmax < ((*a).nz).wrapping_add((*b).nz) {
            status = gsl_spmatrix_ulong_realloc(((*a).nz).wrapping_add((*b).nz), c);
            if status != 0 {
                return status;
            }
        }
        j = 0 as libc::c_int as size_t;
        while j < inner_size {
            *w.offset(j as isize) = 0 as libc::c_int;
            j = j.wrapping_add(1);
            j;
        }
        Ci = (*c).i;
        Cp = (*c).p;
        Cd = (*c).data;
        j = 0 as libc::c_int as size_t;
        while j < outer_size {
            *Cp.offset(j as isize) = nz as libc::c_int;
            nz = spmatrix_ulong_scatter(
                a,
                j,
                w,
                x,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                c,
                nz,
            );
            nz = spmatrix_ulong_scatter(
                b,
                j,
                w,
                x,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                c,
                nz,
            );
            p = *Cp.offset(j as isize);
            while p < nz as libc::c_int {
                *Cd.offset(p as isize) = *x.offset(*Ci.offset(p as isize) as isize);
                p += 1;
                p;
            }
            j = j.wrapping_add(1);
            j;
        }
        *Cp.offset(j as isize) = nz as libc::c_int;
        (*c).nz = nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_int_add(
    mut c: *mut gsl_spmatrix_int,
    mut a: *const gsl_spmatrix_int,
    mut b: *const gsl_spmatrix_int,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N || (*c).size1 != M || (*c).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            169 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*a).sptype != (*b).sptype || (*a).sptype != (*c).sptype {
        gsl_error(
            b"matrices must have same sparse storage format\0" as *const u8
                as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*a).sptype == GSL_SPMATRIX_COO as libc::c_int {
        gsl_error(
            b"COO format not yet supported\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            178 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut w: *mut libc::c_int = (*a).work.work_int;
        let mut x: *mut libc::c_int = (*c).work.work_atomic;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut Ci: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut Cd: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut p: libc::c_int = 0;
        let mut j: size_t = 0;
        let mut nz: size_t = 0 as libc::c_int as size_t;
        let mut inner_size: size_t = 0;
        let mut outer_size: size_t = 0;
        if (*a).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            inner_size = M;
            outer_size = N;
        } else if (*a).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            inner_size = N;
            outer_size = M;
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                204 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        if (*c).nzmax < ((*a).nz).wrapping_add((*b).nz) {
            status = gsl_spmatrix_int_realloc(((*a).nz).wrapping_add((*b).nz), c);
            if status != 0 {
                return status;
            }
        }
        j = 0 as libc::c_int as size_t;
        while j < inner_size {
            *w.offset(j as isize) = 0 as libc::c_int;
            j = j.wrapping_add(1);
            j;
        }
        Ci = (*c).i;
        Cp = (*c).p;
        Cd = (*c).data;
        j = 0 as libc::c_int as size_t;
        while j < outer_size {
            *Cp.offset(j as isize) = nz as libc::c_int;
            nz = spmatrix_int_scatter(
                a,
                j,
                w,
                x,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                c,
                nz,
            );
            nz = spmatrix_int_scatter(
                b,
                j,
                w,
                x,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                c,
                nz,
            );
            p = *Cp.offset(j as isize);
            while p < nz as libc::c_int {
                *Cd.offset(p as isize) = *x.offset(*Ci.offset(p as isize) as isize);
                p += 1;
                p;
            }
            j = j.wrapping_add(1);
            j;
        }
        *Cp.offset(j as isize) = nz as libc::c_int;
        (*c).nz = nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_short_add(
    mut c: *mut gsl_spmatrix_short,
    mut a: *const gsl_spmatrix_short,
    mut b: *const gsl_spmatrix_short,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N || (*c).size1 != M || (*c).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            169 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*a).sptype != (*b).sptype || (*a).sptype != (*c).sptype {
        gsl_error(
            b"matrices must have same sparse storage format\0" as *const u8
                as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*a).sptype == GSL_SPMATRIX_COO as libc::c_int {
        gsl_error(
            b"COO format not yet supported\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            178 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut w: *mut libc::c_int = (*a).work.work_int;
        let mut x: *mut libc::c_short = (*c).work.work_atomic;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut Ci: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut Cd: *mut libc::c_short = 0 as *mut libc::c_short;
        let mut p: libc::c_int = 0;
        let mut j: size_t = 0;
        let mut nz: size_t = 0 as libc::c_int as size_t;
        let mut inner_size: size_t = 0;
        let mut outer_size: size_t = 0;
        if (*a).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            inner_size = M;
            outer_size = N;
        } else if (*a).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            inner_size = N;
            outer_size = M;
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                204 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        if (*c).nzmax < ((*a).nz).wrapping_add((*b).nz) {
            status = gsl_spmatrix_short_realloc(((*a).nz).wrapping_add((*b).nz), c);
            if status != 0 {
                return status;
            }
        }
        j = 0 as libc::c_int as size_t;
        while j < inner_size {
            *w.offset(j as isize) = 0 as libc::c_int;
            j = j.wrapping_add(1);
            j;
        }
        Ci = (*c).i;
        Cp = (*c).p;
        Cd = (*c).data;
        j = 0 as libc::c_int as size_t;
        while j < outer_size {
            *Cp.offset(j as isize) = nz as libc::c_int;
            nz = spmatrix_short_scatter(
                a,
                j,
                w,
                x,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                c,
                nz,
            );
            nz = spmatrix_short_scatter(
                b,
                j,
                w,
                x,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                c,
                nz,
            );
            p = *Cp.offset(j as isize);
            while p < nz as libc::c_int {
                *Cd.offset(p as isize) = *x.offset(*Ci.offset(p as isize) as isize);
                p += 1;
                p;
            }
            j = j.wrapping_add(1);
            j;
        }
        *Cp.offset(j as isize) = nz as libc::c_int;
        (*c).nz = nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uchar_add(
    mut c: *mut gsl_spmatrix_uchar,
    mut a: *const gsl_spmatrix_uchar,
    mut b: *const gsl_spmatrix_uchar,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N || (*c).size1 != M || (*c).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            169 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*a).sptype != (*b).sptype || (*a).sptype != (*c).sptype {
        gsl_error(
            b"matrices must have same sparse storage format\0" as *const u8
                as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*a).sptype == GSL_SPMATRIX_COO as libc::c_int {
        gsl_error(
            b"COO format not yet supported\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            178 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut w: *mut libc::c_int = (*a).work.work_int;
        let mut x: *mut libc::c_uchar = (*c).work.work_atomic;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut Ci: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut Cd: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut p: libc::c_int = 0;
        let mut j: size_t = 0;
        let mut nz: size_t = 0 as libc::c_int as size_t;
        let mut inner_size: size_t = 0;
        let mut outer_size: size_t = 0;
        if (*a).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            inner_size = M;
            outer_size = N;
        } else if (*a).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            inner_size = N;
            outer_size = M;
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                204 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        if (*c).nzmax < ((*a).nz).wrapping_add((*b).nz) {
            status = gsl_spmatrix_uchar_realloc(((*a).nz).wrapping_add((*b).nz), c);
            if status != 0 {
                return status;
            }
        }
        j = 0 as libc::c_int as size_t;
        while j < inner_size {
            *w.offset(j as isize) = 0 as libc::c_int;
            j = j.wrapping_add(1);
            j;
        }
        Ci = (*c).i;
        Cp = (*c).p;
        Cd = (*c).data;
        j = 0 as libc::c_int as size_t;
        while j < outer_size {
            *Cp.offset(j as isize) = nz as libc::c_int;
            nz = spmatrix_uchar_scatter(
                a,
                j,
                w,
                x,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                c,
                nz,
            );
            nz = spmatrix_uchar_scatter(
                b,
                j,
                w,
                x,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                c,
                nz,
            );
            p = *Cp.offset(j as isize);
            while p < nz as libc::c_int {
                *Cd.offset(p as isize) = *x.offset(*Ci.offset(p as isize) as isize);
                p += 1;
                p;
            }
            j = j.wrapping_add(1);
            j;
        }
        *Cp.offset(j as isize) = nz as libc::c_int;
        (*c).nz = nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_add(
    mut c: *mut gsl_spmatrix_long_double,
    mut a: *const gsl_spmatrix_long_double,
    mut b: *const gsl_spmatrix_long_double,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N || (*c).size1 != M || (*c).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            169 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*a).sptype != (*b).sptype || (*a).sptype != (*c).sptype {
        gsl_error(
            b"matrices must have same sparse storage format\0" as *const u8
                as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*a).sptype == GSL_SPMATRIX_COO as libc::c_int {
        gsl_error(
            b"COO format not yet supported\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            178 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut w: *mut libc::c_int = (*a).work.work_int;
        let mut x: *mut f128::f128 = (*c).work.work_atomic;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut Ci: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut Cd: *mut f128::f128 = 0 as *mut f128::f128;
        let mut p: libc::c_int = 0;
        let mut j: size_t = 0;
        let mut nz: size_t = 0 as libc::c_int as size_t;
        let mut inner_size: size_t = 0;
        let mut outer_size: size_t = 0;
        if (*a).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            inner_size = M;
            outer_size = N;
        } else if (*a).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            inner_size = N;
            outer_size = M;
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                204 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        if (*c).nzmax < ((*a).nz).wrapping_add((*b).nz) {
            status = gsl_spmatrix_long_double_realloc(
                ((*a).nz).wrapping_add((*b).nz),
                c,
            );
            if status != 0 {
                return status;
            }
        }
        j = 0 as libc::c_int as size_t;
        while j < inner_size {
            *w.offset(j as isize) = 0 as libc::c_int;
            j = j.wrapping_add(1);
            j;
        }
        Ci = (*c).i;
        Cp = (*c).p;
        Cd = (*c).data;
        j = 0 as libc::c_int as size_t;
        while j < outer_size {
            *Cp.offset(j as isize) = nz as libc::c_int;
            nz = spmatrix_long_double_scatter(
                a,
                j,
                w,
                x,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                c,
                nz,
            );
            nz = spmatrix_long_double_scatter(
                b,
                j,
                w,
                x,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                c,
                nz,
            );
            p = *Cp.offset(j as isize);
            while p < nz as libc::c_int {
                *Cd.offset(p as isize) = *x.offset(*Ci.offset(p as isize) as isize);
                p += 1;
                p;
            }
            j = j.wrapping_add(1);
            j;
        }
        *Cp.offset(j as isize) = nz as libc::c_int;
        (*c).nz = nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uint_add(
    mut c: *mut gsl_spmatrix_uint,
    mut a: *const gsl_spmatrix_uint,
    mut b: *const gsl_spmatrix_uint,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N || (*c).size1 != M || (*c).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            169 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*a).sptype != (*b).sptype || (*a).sptype != (*c).sptype {
        gsl_error(
            b"matrices must have same sparse storage format\0" as *const u8
                as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*a).sptype == GSL_SPMATRIX_COO as libc::c_int {
        gsl_error(
            b"COO format not yet supported\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            178 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut w: *mut libc::c_int = (*a).work.work_int;
        let mut x: *mut libc::c_uint = (*c).work.work_atomic;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut Ci: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut Cd: *mut libc::c_uint = 0 as *mut libc::c_uint;
        let mut p: libc::c_int = 0;
        let mut j: size_t = 0;
        let mut nz: size_t = 0 as libc::c_int as size_t;
        let mut inner_size: size_t = 0;
        let mut outer_size: size_t = 0;
        if (*a).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            inner_size = M;
            outer_size = N;
        } else if (*a).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            inner_size = N;
            outer_size = M;
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                204 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        if (*c).nzmax < ((*a).nz).wrapping_add((*b).nz) {
            status = gsl_spmatrix_uint_realloc(((*a).nz).wrapping_add((*b).nz), c);
            if status != 0 {
                return status;
            }
        }
        j = 0 as libc::c_int as size_t;
        while j < inner_size {
            *w.offset(j as isize) = 0 as libc::c_int;
            j = j.wrapping_add(1);
            j;
        }
        Ci = (*c).i;
        Cp = (*c).p;
        Cd = (*c).data;
        j = 0 as libc::c_int as size_t;
        while j < outer_size {
            *Cp.offset(j as isize) = nz as libc::c_int;
            nz = spmatrix_uint_scatter(
                a,
                j,
                w,
                x,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                c,
                nz,
            );
            nz = spmatrix_uint_scatter(
                b,
                j,
                w,
                x,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                c,
                nz,
            );
            p = *Cp.offset(j as isize);
            while p < nz as libc::c_int {
                *Cd.offset(p as isize) = *x.offset(*Ci.offset(p as isize) as isize);
                p += 1;
                p;
            }
            j = j.wrapping_add(1);
            j;
        }
        *Cp.offset(j as isize) = nz as libc::c_int;
        (*c).nz = nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_float_add(
    mut c: *mut gsl_spmatrix_float,
    mut a: *const gsl_spmatrix_float,
    mut b: *const gsl_spmatrix_float,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N || (*c).size1 != M || (*c).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            169 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*a).sptype != (*b).sptype || (*a).sptype != (*c).sptype {
        gsl_error(
            b"matrices must have same sparse storage format\0" as *const u8
                as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*a).sptype == GSL_SPMATRIX_COO as libc::c_int {
        gsl_error(
            b"COO format not yet supported\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            178 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut w: *mut libc::c_int = (*a).work.work_int;
        let mut x: *mut libc::c_float = (*c).work.work_atomic;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut Ci: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut Cd: *mut libc::c_float = 0 as *mut libc::c_float;
        let mut p: libc::c_int = 0;
        let mut j: size_t = 0;
        let mut nz: size_t = 0 as libc::c_int as size_t;
        let mut inner_size: size_t = 0;
        let mut outer_size: size_t = 0;
        if (*a).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            inner_size = M;
            outer_size = N;
        } else if (*a).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            inner_size = N;
            outer_size = M;
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                204 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        if (*c).nzmax < ((*a).nz).wrapping_add((*b).nz) {
            status = gsl_spmatrix_float_realloc(((*a).nz).wrapping_add((*b).nz), c);
            if status != 0 {
                return status;
            }
        }
        j = 0 as libc::c_int as size_t;
        while j < inner_size {
            *w.offset(j as isize) = 0 as libc::c_int;
            j = j.wrapping_add(1);
            j;
        }
        Ci = (*c).i;
        Cp = (*c).p;
        Cd = (*c).data;
        j = 0 as libc::c_int as size_t;
        while j < outer_size {
            *Cp.offset(j as isize) = nz as libc::c_int;
            nz = spmatrix_float_scatter(
                a,
                j,
                w,
                x,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                c,
                nz,
            );
            nz = spmatrix_float_scatter(
                b,
                j,
                w,
                x,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                c,
                nz,
            );
            p = *Cp.offset(j as isize);
            while p < nz as libc::c_int {
                *Cd.offset(p as isize) = *x.offset(*Ci.offset(p as isize) as isize);
                p += 1;
                p;
            }
            j = j.wrapping_add(1);
            j;
        }
        *Cp.offset(j as isize) = nz as libc::c_int;
        (*c).nz = nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_add(
    mut c: *mut gsl_spmatrix,
    mut a: *const gsl_spmatrix,
    mut b: *const gsl_spmatrix,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N || (*c).size1 != M || (*c).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            169 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*a).sptype != (*b).sptype || (*a).sptype != (*c).sptype {
        gsl_error(
            b"matrices must have same sparse storage format\0" as *const u8
                as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*a).sptype == GSL_SPMATRIX_COO as libc::c_int {
        gsl_error(
            b"COO format not yet supported\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            178 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut w: *mut libc::c_int = (*a).work.work_int;
        let mut x: *mut libc::c_double = (*c).work.work_atomic;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut Ci: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut Cd: *mut libc::c_double = 0 as *mut libc::c_double;
        let mut p: libc::c_int = 0;
        let mut j: size_t = 0;
        let mut nz: size_t = 0 as libc::c_int as size_t;
        let mut inner_size: size_t = 0;
        let mut outer_size: size_t = 0;
        if (*a).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            inner_size = M;
            outer_size = N;
        } else if (*a).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            inner_size = N;
            outer_size = M;
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                204 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        if (*c).nzmax < ((*a).nz).wrapping_add((*b).nz) {
            status = gsl_spmatrix_realloc(((*a).nz).wrapping_add((*b).nz), c);
            if status != 0 {
                return status;
            }
        }
        j = 0 as libc::c_int as size_t;
        while j < inner_size {
            *w.offset(j as isize) = 0 as libc::c_int;
            j = j.wrapping_add(1);
            j;
        }
        Ci = (*c).i;
        Cp = (*c).p;
        Cd = (*c).data;
        j = 0 as libc::c_int as size_t;
        while j < outer_size {
            *Cp.offset(j as isize) = nz as libc::c_int;
            nz = spmatrix_scatter(
                a,
                j,
                w,
                x,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                c,
                nz,
            );
            nz = spmatrix_scatter(
                b,
                j,
                w,
                x,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                c,
                nz,
            );
            p = *Cp.offset(j as isize);
            while p < nz as libc::c_int {
                *Cd.offset(p as isize) = *x.offset(*Ci.offset(p as isize) as isize);
                p += 1;
                p;
            }
            j = j.wrapping_add(1);
            j;
        }
        *Cp.offset(j as isize) = nz as libc::c_int;
        (*c).nz = nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_char_add(
    mut c: *mut gsl_spmatrix_char,
    mut a: *const gsl_spmatrix_char,
    mut b: *const gsl_spmatrix_char,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N || (*c).size1 != M || (*c).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            169 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*a).sptype != (*b).sptype || (*a).sptype != (*c).sptype {
        gsl_error(
            b"matrices must have same sparse storage format\0" as *const u8
                as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*a).sptype == GSL_SPMATRIX_COO as libc::c_int {
        gsl_error(
            b"COO format not yet supported\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            178 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut w: *mut libc::c_int = (*a).work.work_int;
        let mut x: *mut libc::c_char = (*c).work.work_atomic;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut Ci: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut Cd: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut p: libc::c_int = 0;
        let mut j: size_t = 0;
        let mut nz: size_t = 0 as libc::c_int as size_t;
        let mut inner_size: size_t = 0;
        let mut outer_size: size_t = 0;
        if (*a).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            inner_size = M;
            outer_size = N;
        } else if (*a).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            inner_size = N;
            outer_size = M;
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                204 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        if (*c).nzmax < ((*a).nz).wrapping_add((*b).nz) {
            status = gsl_spmatrix_char_realloc(((*a).nz).wrapping_add((*b).nz), c);
            if status != 0 {
                return status;
            }
        }
        j = 0 as libc::c_int as size_t;
        while j < inner_size {
            *w.offset(j as isize) = 0 as libc::c_int;
            j = j.wrapping_add(1);
            j;
        }
        Ci = (*c).i;
        Cp = (*c).p;
        Cd = (*c).data;
        j = 0 as libc::c_int as size_t;
        while j < outer_size {
            *Cp.offset(j as isize) = nz as libc::c_int;
            nz = spmatrix_char_scatter(
                a,
                j,
                w,
                x,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                c,
                nz,
            );
            nz = spmatrix_char_scatter(
                b,
                j,
                w,
                x,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                c,
                nz,
            );
            p = *Cp.offset(j as isize);
            while p < nz as libc::c_int {
                *Cd.offset(p as isize) = *x.offset(*Ci.offset(p as isize) as isize);
                p += 1;
                p;
            }
            j = j.wrapping_add(1);
            j;
        }
        *Cp.offset(j as isize) = nz as libc::c_int;
        (*c).nz = nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_ushort_add(
    mut c: *mut gsl_spmatrix_ushort,
    mut a: *const gsl_spmatrix_ushort,
    mut b: *const gsl_spmatrix_ushort,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N || (*c).size1 != M || (*c).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            169 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*a).sptype != (*b).sptype || (*a).sptype != (*c).sptype {
        gsl_error(
            b"matrices must have same sparse storage format\0" as *const u8
                as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*a).sptype == GSL_SPMATRIX_COO as libc::c_int {
        gsl_error(
            b"COO format not yet supported\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            178 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut w: *mut libc::c_int = (*a).work.work_int;
        let mut x: *mut libc::c_ushort = (*c).work.work_atomic;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut Ci: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut Cd: *mut libc::c_ushort = 0 as *mut libc::c_ushort;
        let mut p: libc::c_int = 0;
        let mut j: size_t = 0;
        let mut nz: size_t = 0 as libc::c_int as size_t;
        let mut inner_size: size_t = 0;
        let mut outer_size: size_t = 0;
        if (*a).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            inner_size = M;
            outer_size = N;
        } else if (*a).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            inner_size = N;
            outer_size = M;
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                204 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        if (*c).nzmax < ((*a).nz).wrapping_add((*b).nz) {
            status = gsl_spmatrix_ushort_realloc(((*a).nz).wrapping_add((*b).nz), c);
            if status != 0 {
                return status;
            }
        }
        j = 0 as libc::c_int as size_t;
        while j < inner_size {
            *w.offset(j as isize) = 0 as libc::c_int;
            j = j.wrapping_add(1);
            j;
        }
        Ci = (*c).i;
        Cp = (*c).p;
        Cd = (*c).data;
        j = 0 as libc::c_int as size_t;
        while j < outer_size {
            *Cp.offset(j as isize) = nz as libc::c_int;
            nz = spmatrix_ushort_scatter(
                a,
                j,
                w,
                x,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                c,
                nz,
            );
            nz = spmatrix_ushort_scatter(
                b,
                j,
                w,
                x,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                c,
                nz,
            );
            p = *Cp.offset(j as isize);
            while p < nz as libc::c_int {
                *Cd.offset(p as isize) = *x.offset(*Ci.offset(p as isize) as isize);
                p += 1;
                p;
            }
            j = j.wrapping_add(1);
            j;
        }
        *Cp.offset(j as isize) = nz as libc::c_int;
        (*c).nz = nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_long_add(
    mut c: *mut gsl_spmatrix_long,
    mut a: *const gsl_spmatrix_long,
    mut b: *const gsl_spmatrix_long,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N || (*c).size1 != M || (*c).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            169 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*a).sptype != (*b).sptype || (*a).sptype != (*c).sptype {
        gsl_error(
            b"matrices must have same sparse storage format\0" as *const u8
                as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*a).sptype == GSL_SPMATRIX_COO as libc::c_int {
        gsl_error(
            b"COO format not yet supported\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            178 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut w: *mut libc::c_int = (*a).work.work_int;
        let mut x: *mut libc::c_long = (*c).work.work_atomic;
        let mut Cp: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut Ci: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut Cd: *mut libc::c_long = 0 as *mut libc::c_long;
        let mut p: libc::c_int = 0;
        let mut j: size_t = 0;
        let mut nz: size_t = 0 as libc::c_int as size_t;
        let mut inner_size: size_t = 0;
        let mut outer_size: size_t = 0;
        if (*a).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            inner_size = M;
            outer_size = N;
        } else if (*a).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            inner_size = N;
            outer_size = M;
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                204 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        if (*c).nzmax < ((*a).nz).wrapping_add((*b).nz) {
            status = gsl_spmatrix_long_realloc(((*a).nz).wrapping_add((*b).nz), c);
            if status != 0 {
                return status;
            }
        }
        j = 0 as libc::c_int as size_t;
        while j < inner_size {
            *w.offset(j as isize) = 0 as libc::c_int;
            j = j.wrapping_add(1);
            j;
        }
        Ci = (*c).i;
        Cp = (*c).p;
        Cd = (*c).data;
        j = 0 as libc::c_int as size_t;
        while j < outer_size {
            *Cp.offset(j as isize) = nz as libc::c_int;
            nz = spmatrix_long_scatter(
                a,
                j,
                w,
                x,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                c,
                nz,
            );
            nz = spmatrix_long_scatter(
                b,
                j,
                w,
                x,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                c,
                nz,
            );
            p = *Cp.offset(j as isize);
            while p < nz as libc::c_int {
                *Cd.offset(p as isize) = *x.offset(*Ci.offset(p as isize) as isize);
                p += 1;
                p;
            }
            j = j.wrapping_add(1);
            j;
        }
        *Cp.offset(j as isize) = nz as libc::c_int;
        (*c).nz = nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_int_dense_add(
    mut a: *mut gsl_matrix_int,
    mut b: *const gsl_spmatrix_int,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            262 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let mut bd: *const libc::c_int = (*b).data;
        if (*b).nz == 0 as libc::c_int as libc::c_ulong {
            return GSL_SUCCESS as libc::c_int;
        }
        if (*b).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut bi: *const libc::c_int = (*b).i;
            let mut bj: *const libc::c_int = (*b).p;
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*b).nz {
                *((*a).data)
                    .offset(
                        (*bi.offset(n as isize) as libc::c_ulong)
                            .wrapping_mul(tda_a)
                            .wrapping_add(*bj.offset(n as isize) as libc::c_ulong)
                            as isize,
                    ) += *bd.offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut bi_0: *const libc::c_int = (*b).i;
            let mut bp: *const libc::c_int = (*b).p;
            let mut j: size_t = 0;
            let mut p: libc::c_int = 0;
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *bp.offset(j as isize);
                while p
                    < *bp
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    *((*a).data)
                        .offset(
                            (*bi_0.offset(p as isize) as libc::c_ulong)
                                .wrapping_mul(tda_a)
                                .wrapping_add(j) as isize,
                        ) += *bd.offset(p as isize);
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut bj_0: *const libc::c_int = (*b).i;
            let mut bp_0: *const libc::c_int = (*b).p;
            let mut i: size_t = 0;
            let mut p_0: libc::c_int = 0;
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *bp_0.offset(i as isize);
                while p_0
                    < *bp_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    *((*a).data)
                        .offset(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(*bj_0.offset(p_0 as isize) as libc::c_ulong)
                                as isize,
                        ) += *bd.offset(p_0 as isize);
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_char_dense_add(
    mut a: *mut gsl_matrix_char,
    mut b: *const gsl_spmatrix_char,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            262 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let mut bd: *const libc::c_char = (*b).data;
        if (*b).nz == 0 as libc::c_int as libc::c_ulong {
            return GSL_SUCCESS as libc::c_int;
        }
        if (*b).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut bi: *const libc::c_int = (*b).i;
            let mut bj: *const libc::c_int = (*b).p;
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*b).nz {
                let ref mut fresh45 = *((*a).data)
                    .offset(
                        (*bi.offset(n as isize) as libc::c_ulong)
                            .wrapping_mul(tda_a)
                            .wrapping_add(*bj.offset(n as isize) as libc::c_ulong)
                            as isize,
                    );
                *fresh45 = (*fresh45 as libc::c_int
                    + *bd.offset(n as isize) as libc::c_int) as libc::c_char;
                n = n.wrapping_add(1);
                n;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut bi_0: *const libc::c_int = (*b).i;
            let mut bp: *const libc::c_int = (*b).p;
            let mut j: size_t = 0;
            let mut p: libc::c_int = 0;
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *bp.offset(j as isize);
                while p
                    < *bp
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh46 = *((*a).data)
                        .offset(
                            (*bi_0.offset(p as isize) as libc::c_ulong)
                                .wrapping_mul(tda_a)
                                .wrapping_add(j) as isize,
                        );
                    *fresh46 = (*fresh46 as libc::c_int
                        + *bd.offset(p as isize) as libc::c_int) as libc::c_char;
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut bj_0: *const libc::c_int = (*b).i;
            let mut bp_0: *const libc::c_int = (*b).p;
            let mut i: size_t = 0;
            let mut p_0: libc::c_int = 0;
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *bp_0.offset(i as isize);
                while p_0
                    < *bp_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh47 = *((*a).data)
                        .offset(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(*bj_0.offset(p_0 as isize) as libc::c_ulong)
                                as isize,
                        );
                    *fresh47 = (*fresh47 as libc::c_int
                        + *bd.offset(p_0 as isize) as libc::c_int) as libc::c_char;
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_dense_add(
    mut a: *mut gsl_matrix,
    mut b: *const gsl_spmatrix,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            262 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let mut bd: *const libc::c_double = (*b).data;
        if (*b).nz == 0 as libc::c_int as libc::c_ulong {
            return GSL_SUCCESS as libc::c_int;
        }
        if (*b).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut bi: *const libc::c_int = (*b).i;
            let mut bj: *const libc::c_int = (*b).p;
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*b).nz {
                *((*a).data)
                    .offset(
                        (*bi.offset(n as isize) as libc::c_ulong)
                            .wrapping_mul(tda_a)
                            .wrapping_add(*bj.offset(n as isize) as libc::c_ulong)
                            as isize,
                    ) += *bd.offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut bi_0: *const libc::c_int = (*b).i;
            let mut bp: *const libc::c_int = (*b).p;
            let mut j: size_t = 0;
            let mut p: libc::c_int = 0;
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *bp.offset(j as isize);
                while p
                    < *bp
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    *((*a).data)
                        .offset(
                            (*bi_0.offset(p as isize) as libc::c_ulong)
                                .wrapping_mul(tda_a)
                                .wrapping_add(j) as isize,
                        ) += *bd.offset(p as isize);
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut bj_0: *const libc::c_int = (*b).i;
            let mut bp_0: *const libc::c_int = (*b).p;
            let mut i: size_t = 0;
            let mut p_0: libc::c_int = 0;
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *bp_0.offset(i as isize);
                while p_0
                    < *bp_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    *((*a).data)
                        .offset(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(*bj_0.offset(p_0 as isize) as libc::c_ulong)
                                as isize,
                        ) += *bd.offset(p_0 as isize);
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_ulong_dense_add(
    mut a: *mut gsl_matrix_ulong,
    mut b: *const gsl_spmatrix_ulong,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            262 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let mut bd: *const libc::c_ulong = (*b).data;
        if (*b).nz == 0 as libc::c_int as libc::c_ulong {
            return GSL_SUCCESS as libc::c_int;
        }
        if (*b).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut bi: *const libc::c_int = (*b).i;
            let mut bj: *const libc::c_int = (*b).p;
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*b).nz {
                let ref mut fresh48 = *((*a).data)
                    .offset(
                        (*bi.offset(n as isize) as libc::c_ulong)
                            .wrapping_mul(tda_a)
                            .wrapping_add(*bj.offset(n as isize) as libc::c_ulong)
                            as isize,
                    );
                *fresh48 = (*fresh48).wrapping_add(*bd.offset(n as isize));
                n = n.wrapping_add(1);
                n;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut bi_0: *const libc::c_int = (*b).i;
            let mut bp: *const libc::c_int = (*b).p;
            let mut j: size_t = 0;
            let mut p: libc::c_int = 0;
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *bp.offset(j as isize);
                while p
                    < *bp
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh49 = *((*a).data)
                        .offset(
                            (*bi_0.offset(p as isize) as libc::c_ulong)
                                .wrapping_mul(tda_a)
                                .wrapping_add(j) as isize,
                        );
                    *fresh49 = (*fresh49).wrapping_add(*bd.offset(p as isize));
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut bj_0: *const libc::c_int = (*b).i;
            let mut bp_0: *const libc::c_int = (*b).p;
            let mut i: size_t = 0;
            let mut p_0: libc::c_int = 0;
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *bp_0.offset(i as isize);
                while p_0
                    < *bp_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh50 = *((*a).data)
                        .offset(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(*bj_0.offset(p_0 as isize) as libc::c_ulong)
                                as isize,
                        );
                    *fresh50 = (*fresh50).wrapping_add(*bd.offset(p_0 as isize));
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_ushort_dense_add(
    mut a: *mut gsl_matrix_ushort,
    mut b: *const gsl_spmatrix_ushort,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            262 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let mut bd: *const libc::c_ushort = (*b).data;
        if (*b).nz == 0 as libc::c_int as libc::c_ulong {
            return GSL_SUCCESS as libc::c_int;
        }
        if (*b).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut bi: *const libc::c_int = (*b).i;
            let mut bj: *const libc::c_int = (*b).p;
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*b).nz {
                let ref mut fresh51 = *((*a).data)
                    .offset(
                        (*bi.offset(n as isize) as libc::c_ulong)
                            .wrapping_mul(tda_a)
                            .wrapping_add(*bj.offset(n as isize) as libc::c_ulong)
                            as isize,
                    );
                *fresh51 = (*fresh51 as libc::c_int
                    + *bd.offset(n as isize) as libc::c_int) as libc::c_ushort;
                n = n.wrapping_add(1);
                n;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut bi_0: *const libc::c_int = (*b).i;
            let mut bp: *const libc::c_int = (*b).p;
            let mut j: size_t = 0;
            let mut p: libc::c_int = 0;
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *bp.offset(j as isize);
                while p
                    < *bp
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh52 = *((*a).data)
                        .offset(
                            (*bi_0.offset(p as isize) as libc::c_ulong)
                                .wrapping_mul(tda_a)
                                .wrapping_add(j) as isize,
                        );
                    *fresh52 = (*fresh52 as libc::c_int
                        + *bd.offset(p as isize) as libc::c_int) as libc::c_ushort;
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut bj_0: *const libc::c_int = (*b).i;
            let mut bp_0: *const libc::c_int = (*b).p;
            let mut i: size_t = 0;
            let mut p_0: libc::c_int = 0;
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *bp_0.offset(i as isize);
                while p_0
                    < *bp_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh53 = *((*a).data)
                        .offset(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(*bj_0.offset(p_0 as isize) as libc::c_ulong)
                                as isize,
                        );
                    *fresh53 = (*fresh53 as libc::c_int
                        + *bd.offset(p_0 as isize) as libc::c_int) as libc::c_ushort;
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_long_dense_add(
    mut a: *mut gsl_matrix_long,
    mut b: *const gsl_spmatrix_long,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            262 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let mut bd: *const libc::c_long = (*b).data;
        if (*b).nz == 0 as libc::c_int as libc::c_ulong {
            return GSL_SUCCESS as libc::c_int;
        }
        if (*b).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut bi: *const libc::c_int = (*b).i;
            let mut bj: *const libc::c_int = (*b).p;
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*b).nz {
                *((*a).data)
                    .offset(
                        (*bi.offset(n as isize) as libc::c_ulong)
                            .wrapping_mul(tda_a)
                            .wrapping_add(*bj.offset(n as isize) as libc::c_ulong)
                            as isize,
                    ) += *bd.offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut bi_0: *const libc::c_int = (*b).i;
            let mut bp: *const libc::c_int = (*b).p;
            let mut j: size_t = 0;
            let mut p: libc::c_int = 0;
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *bp.offset(j as isize);
                while p
                    < *bp
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    *((*a).data)
                        .offset(
                            (*bi_0.offset(p as isize) as libc::c_ulong)
                                .wrapping_mul(tda_a)
                                .wrapping_add(j) as isize,
                        ) += *bd.offset(p as isize);
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut bj_0: *const libc::c_int = (*b).i;
            let mut bp_0: *const libc::c_int = (*b).p;
            let mut i: size_t = 0;
            let mut p_0: libc::c_int = 0;
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *bp_0.offset(i as isize);
                while p_0
                    < *bp_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    *((*a).data)
                        .offset(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(*bj_0.offset(p_0 as isize) as libc::c_ulong)
                                as isize,
                        ) += *bd.offset(p_0 as isize);
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_float_dense_add(
    mut a: *mut gsl_matrix_float,
    mut b: *const gsl_spmatrix_float,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            262 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let mut bd: *const libc::c_float = (*b).data;
        if (*b).nz == 0 as libc::c_int as libc::c_ulong {
            return GSL_SUCCESS as libc::c_int;
        }
        if (*b).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut bi: *const libc::c_int = (*b).i;
            let mut bj: *const libc::c_int = (*b).p;
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*b).nz {
                *((*a).data)
                    .offset(
                        (*bi.offset(n as isize) as libc::c_ulong)
                            .wrapping_mul(tda_a)
                            .wrapping_add(*bj.offset(n as isize) as libc::c_ulong)
                            as isize,
                    ) += *bd.offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut bi_0: *const libc::c_int = (*b).i;
            let mut bp: *const libc::c_int = (*b).p;
            let mut j: size_t = 0;
            let mut p: libc::c_int = 0;
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *bp.offset(j as isize);
                while p
                    < *bp
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    *((*a).data)
                        .offset(
                            (*bi_0.offset(p as isize) as libc::c_ulong)
                                .wrapping_mul(tda_a)
                                .wrapping_add(j) as isize,
                        ) += *bd.offset(p as isize);
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut bj_0: *const libc::c_int = (*b).i;
            let mut bp_0: *const libc::c_int = (*b).p;
            let mut i: size_t = 0;
            let mut p_0: libc::c_int = 0;
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *bp_0.offset(i as isize);
                while p_0
                    < *bp_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    *((*a).data)
                        .offset(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(*bj_0.offset(p_0 as isize) as libc::c_ulong)
                                as isize,
                        ) += *bd.offset(p_0 as isize);
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uint_dense_add(
    mut a: *mut gsl_matrix_uint,
    mut b: *const gsl_spmatrix_uint,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            262 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let mut bd: *const libc::c_uint = (*b).data;
        if (*b).nz == 0 as libc::c_int as libc::c_ulong {
            return GSL_SUCCESS as libc::c_int;
        }
        if (*b).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut bi: *const libc::c_int = (*b).i;
            let mut bj: *const libc::c_int = (*b).p;
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*b).nz {
                let ref mut fresh54 = *((*a).data)
                    .offset(
                        (*bi.offset(n as isize) as libc::c_ulong)
                            .wrapping_mul(tda_a)
                            .wrapping_add(*bj.offset(n as isize) as libc::c_ulong)
                            as isize,
                    );
                *fresh54 = (*fresh54).wrapping_add(*bd.offset(n as isize));
                n = n.wrapping_add(1);
                n;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut bi_0: *const libc::c_int = (*b).i;
            let mut bp: *const libc::c_int = (*b).p;
            let mut j: size_t = 0;
            let mut p: libc::c_int = 0;
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *bp.offset(j as isize);
                while p
                    < *bp
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh55 = *((*a).data)
                        .offset(
                            (*bi_0.offset(p as isize) as libc::c_ulong)
                                .wrapping_mul(tda_a)
                                .wrapping_add(j) as isize,
                        );
                    *fresh55 = (*fresh55).wrapping_add(*bd.offset(p as isize));
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut bj_0: *const libc::c_int = (*b).i;
            let mut bp_0: *const libc::c_int = (*b).p;
            let mut i: size_t = 0;
            let mut p_0: libc::c_int = 0;
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *bp_0.offset(i as isize);
                while p_0
                    < *bp_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh56 = *((*a).data)
                        .offset(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(*bj_0.offset(p_0 as isize) as libc::c_ulong)
                                as isize,
                        );
                    *fresh56 = (*fresh56).wrapping_add(*bd.offset(p_0 as isize));
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_dense_add(
    mut a: *mut gsl_matrix_long_double,
    mut b: *const gsl_spmatrix_long_double,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            262 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let mut bd: *const f128::f128 = (*b).data;
        if (*b).nz == 0 as libc::c_int as libc::c_ulong {
            return GSL_SUCCESS as libc::c_int;
        }
        if (*b).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut bi: *const libc::c_int = (*b).i;
            let mut bj: *const libc::c_int = (*b).p;
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*b).nz {
                *((*a).data)
                    .offset(
                        (*bi.offset(n as isize) as libc::c_ulong)
                            .wrapping_mul(tda_a)
                            .wrapping_add(*bj.offset(n as isize) as libc::c_ulong)
                            as isize,
                    ) += *bd.offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut bi_0: *const libc::c_int = (*b).i;
            let mut bp: *const libc::c_int = (*b).p;
            let mut j: size_t = 0;
            let mut p: libc::c_int = 0;
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *bp.offset(j as isize);
                while p
                    < *bp
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    *((*a).data)
                        .offset(
                            (*bi_0.offset(p as isize) as libc::c_ulong)
                                .wrapping_mul(tda_a)
                                .wrapping_add(j) as isize,
                        ) += *bd.offset(p as isize);
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut bj_0: *const libc::c_int = (*b).i;
            let mut bp_0: *const libc::c_int = (*b).p;
            let mut i: size_t = 0;
            let mut p_0: libc::c_int = 0;
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *bp_0.offset(i as isize);
                while p_0
                    < *bp_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    *((*a).data)
                        .offset(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(*bj_0.offset(p_0 as isize) as libc::c_ulong)
                                as isize,
                        ) += *bd.offset(p_0 as isize);
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uchar_dense_add(
    mut a: *mut gsl_matrix_uchar,
    mut b: *const gsl_spmatrix_uchar,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            262 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let mut bd: *const libc::c_uchar = (*b).data;
        if (*b).nz == 0 as libc::c_int as libc::c_ulong {
            return GSL_SUCCESS as libc::c_int;
        }
        if (*b).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut bi: *const libc::c_int = (*b).i;
            let mut bj: *const libc::c_int = (*b).p;
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*b).nz {
                let ref mut fresh57 = *((*a).data)
                    .offset(
                        (*bi.offset(n as isize) as libc::c_ulong)
                            .wrapping_mul(tda_a)
                            .wrapping_add(*bj.offset(n as isize) as libc::c_ulong)
                            as isize,
                    );
                *fresh57 = (*fresh57 as libc::c_int
                    + *bd.offset(n as isize) as libc::c_int) as libc::c_uchar;
                n = n.wrapping_add(1);
                n;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut bi_0: *const libc::c_int = (*b).i;
            let mut bp: *const libc::c_int = (*b).p;
            let mut j: size_t = 0;
            let mut p: libc::c_int = 0;
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *bp.offset(j as isize);
                while p
                    < *bp
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh58 = *((*a).data)
                        .offset(
                            (*bi_0.offset(p as isize) as libc::c_ulong)
                                .wrapping_mul(tda_a)
                                .wrapping_add(j) as isize,
                        );
                    *fresh58 = (*fresh58 as libc::c_int
                        + *bd.offset(p as isize) as libc::c_int) as libc::c_uchar;
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut bj_0: *const libc::c_int = (*b).i;
            let mut bp_0: *const libc::c_int = (*b).p;
            let mut i: size_t = 0;
            let mut p_0: libc::c_int = 0;
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *bp_0.offset(i as isize);
                while p_0
                    < *bp_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh59 = *((*a).data)
                        .offset(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(*bj_0.offset(p_0 as isize) as libc::c_ulong)
                                as isize,
                        );
                    *fresh59 = (*fresh59 as libc::c_int
                        + *bd.offset(p_0 as isize) as libc::c_int) as libc::c_uchar;
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_short_dense_add(
    mut a: *mut gsl_matrix_short,
    mut b: *const gsl_spmatrix_short,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            262 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let mut bd: *const libc::c_short = (*b).data;
        if (*b).nz == 0 as libc::c_int as libc::c_ulong {
            return GSL_SUCCESS as libc::c_int;
        }
        if (*b).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut bi: *const libc::c_int = (*b).i;
            let mut bj: *const libc::c_int = (*b).p;
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*b).nz {
                let ref mut fresh60 = *((*a).data)
                    .offset(
                        (*bi.offset(n as isize) as libc::c_ulong)
                            .wrapping_mul(tda_a)
                            .wrapping_add(*bj.offset(n as isize) as libc::c_ulong)
                            as isize,
                    );
                *fresh60 = (*fresh60 as libc::c_int
                    + *bd.offset(n as isize) as libc::c_int) as libc::c_short;
                n = n.wrapping_add(1);
                n;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut bi_0: *const libc::c_int = (*b).i;
            let mut bp: *const libc::c_int = (*b).p;
            let mut j: size_t = 0;
            let mut p: libc::c_int = 0;
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *bp.offset(j as isize);
                while p
                    < *bp
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh61 = *((*a).data)
                        .offset(
                            (*bi_0.offset(p as isize) as libc::c_ulong)
                                .wrapping_mul(tda_a)
                                .wrapping_add(j) as isize,
                        );
                    *fresh61 = (*fresh61 as libc::c_int
                        + *bd.offset(p as isize) as libc::c_int) as libc::c_short;
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut bj_0: *const libc::c_int = (*b).i;
            let mut bp_0: *const libc::c_int = (*b).p;
            let mut i: size_t = 0;
            let mut p_0: libc::c_int = 0;
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *bp_0.offset(i as isize);
                while p_0
                    < *bp_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh62 = *((*a).data)
                        .offset(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(*bj_0.offset(p_0 as isize) as libc::c_ulong)
                                as isize,
                        );
                    *fresh62 = (*fresh62 as libc::c_int
                        + *bd.offset(p_0 as isize) as libc::c_int) as libc::c_short;
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uint_add_to_dense(
    mut a: *mut gsl_matrix_uint,
    mut b: *const gsl_spmatrix_uint,
) -> libc::c_int {
    return gsl_spmatrix_uint_dense_add(a, b);
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_add_to_dense(
    mut a: *mut gsl_matrix_long_double,
    mut b: *const gsl_spmatrix_long_double,
) -> libc::c_int {
    return gsl_spmatrix_long_double_dense_add(a, b);
}
pub unsafe extern "C" fn gsl_spmatrix_int_add_to_dense(
    mut a: *mut gsl_matrix_int,
    mut b: *const gsl_spmatrix_int,
) -> libc::c_int {
    return gsl_spmatrix_int_dense_add(a, b);
}
pub unsafe extern "C" fn gsl_spmatrix_uchar_add_to_dense(
    mut a: *mut gsl_matrix_uchar,
    mut b: *const gsl_spmatrix_uchar,
) -> libc::c_int {
    return gsl_spmatrix_uchar_dense_add(a, b);
}
pub unsafe extern "C" fn gsl_spmatrix_long_add_to_dense(
    mut a: *mut gsl_matrix_long,
    mut b: *const gsl_spmatrix_long,
) -> libc::c_int {
    return gsl_spmatrix_long_dense_add(a, b);
}
pub unsafe extern "C" fn gsl_spmatrix_ushort_add_to_dense(
    mut a: *mut gsl_matrix_ushort,
    mut b: *const gsl_spmatrix_ushort,
) -> libc::c_int {
    return gsl_spmatrix_ushort_dense_add(a, b);
}
pub unsafe extern "C" fn gsl_spmatrix_short_add_to_dense(
    mut a: *mut gsl_matrix_short,
    mut b: *const gsl_spmatrix_short,
) -> libc::c_int {
    return gsl_spmatrix_short_dense_add(a, b);
}
pub unsafe extern "C" fn gsl_spmatrix_add_to_dense(
    mut a: *mut gsl_matrix,
    mut b: *const gsl_spmatrix,
) -> libc::c_int {
    return gsl_spmatrix_dense_add(a, b);
}
pub unsafe extern "C" fn gsl_spmatrix_char_add_to_dense(
    mut a: *mut gsl_matrix_char,
    mut b: *const gsl_spmatrix_char,
) -> libc::c_int {
    return gsl_spmatrix_char_dense_add(a, b);
}
pub unsafe extern "C" fn gsl_spmatrix_float_add_to_dense(
    mut a: *mut gsl_matrix_float,
    mut b: *const gsl_spmatrix_float,
) -> libc::c_int {
    return gsl_spmatrix_float_dense_add(a, b);
}
pub unsafe extern "C" fn gsl_spmatrix_ulong_add_to_dense(
    mut a: *mut gsl_matrix_ulong,
    mut b: *const gsl_spmatrix_ulong,
) -> libc::c_int {
    return gsl_spmatrix_ulong_dense_add(a, b);
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_dense_sub(
    mut a: *mut gsl_matrix_long_double,
    mut b: *const gsl_spmatrix_long_double,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            347 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let mut bd: *const f128::f128 = (*b).data;
        if (*b).nz == 0 as libc::c_int as libc::c_ulong {
            return GSL_SUCCESS as libc::c_int;
        }
        if (*b).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut bi: *const libc::c_int = (*b).i;
            let mut bj: *const libc::c_int = (*b).p;
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*b).nz {
                *((*a).data)
                    .offset(
                        (*bi.offset(n as isize) as libc::c_ulong)
                            .wrapping_mul(tda_a)
                            .wrapping_add(*bj.offset(n as isize) as libc::c_ulong)
                            as isize,
                    ) -= *bd.offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut bi_0: *const libc::c_int = (*b).i;
            let mut bp: *const libc::c_int = (*b).p;
            let mut j: size_t = 0;
            let mut p: libc::c_int = 0;
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *bp.offset(j as isize);
                while p
                    < *bp
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    *((*a).data)
                        .offset(
                            (*bi_0.offset(p as isize) as libc::c_ulong)
                                .wrapping_mul(tda_a)
                                .wrapping_add(j) as isize,
                        ) -= *bd.offset(p as isize);
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut bj_0: *const libc::c_int = (*b).i;
            let mut bp_0: *const libc::c_int = (*b).p;
            let mut i: size_t = 0;
            let mut p_0: libc::c_int = 0;
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *bp_0.offset(i as isize);
                while p_0
                    < *bp_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    *((*a).data)
                        .offset(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(*bj_0.offset(p_0 as isize) as libc::c_ulong)
                                as isize,
                        ) -= *bd.offset(p_0 as isize);
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_float_dense_sub(
    mut a: *mut gsl_matrix_float,
    mut b: *const gsl_spmatrix_float,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            347 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let mut bd: *const libc::c_float = (*b).data;
        if (*b).nz == 0 as libc::c_int as libc::c_ulong {
            return GSL_SUCCESS as libc::c_int;
        }
        if (*b).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut bi: *const libc::c_int = (*b).i;
            let mut bj: *const libc::c_int = (*b).p;
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*b).nz {
                *((*a).data)
                    .offset(
                        (*bi.offset(n as isize) as libc::c_ulong)
                            .wrapping_mul(tda_a)
                            .wrapping_add(*bj.offset(n as isize) as libc::c_ulong)
                            as isize,
                    ) -= *bd.offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut bi_0: *const libc::c_int = (*b).i;
            let mut bp: *const libc::c_int = (*b).p;
            let mut j: size_t = 0;
            let mut p: libc::c_int = 0;
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *bp.offset(j as isize);
                while p
                    < *bp
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    *((*a).data)
                        .offset(
                            (*bi_0.offset(p as isize) as libc::c_ulong)
                                .wrapping_mul(tda_a)
                                .wrapping_add(j) as isize,
                        ) -= *bd.offset(p as isize);
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut bj_0: *const libc::c_int = (*b).i;
            let mut bp_0: *const libc::c_int = (*b).p;
            let mut i: size_t = 0;
            let mut p_0: libc::c_int = 0;
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *bp_0.offset(i as isize);
                while p_0
                    < *bp_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    *((*a).data)
                        .offset(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(*bj_0.offset(p_0 as isize) as libc::c_ulong)
                                as isize,
                        ) -= *bd.offset(p_0 as isize);
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_int_dense_sub(
    mut a: *mut gsl_matrix_int,
    mut b: *const gsl_spmatrix_int,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            347 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let mut bd: *const libc::c_int = (*b).data;
        if (*b).nz == 0 as libc::c_int as libc::c_ulong {
            return GSL_SUCCESS as libc::c_int;
        }
        if (*b).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut bi: *const libc::c_int = (*b).i;
            let mut bj: *const libc::c_int = (*b).p;
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*b).nz {
                *((*a).data)
                    .offset(
                        (*bi.offset(n as isize) as libc::c_ulong)
                            .wrapping_mul(tda_a)
                            .wrapping_add(*bj.offset(n as isize) as libc::c_ulong)
                            as isize,
                    ) -= *bd.offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut bi_0: *const libc::c_int = (*b).i;
            let mut bp: *const libc::c_int = (*b).p;
            let mut j: size_t = 0;
            let mut p: libc::c_int = 0;
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *bp.offset(j as isize);
                while p
                    < *bp
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    *((*a).data)
                        .offset(
                            (*bi_0.offset(p as isize) as libc::c_ulong)
                                .wrapping_mul(tda_a)
                                .wrapping_add(j) as isize,
                        ) -= *bd.offset(p as isize);
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut bj_0: *const libc::c_int = (*b).i;
            let mut bp_0: *const libc::c_int = (*b).p;
            let mut i: size_t = 0;
            let mut p_0: libc::c_int = 0;
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *bp_0.offset(i as isize);
                while p_0
                    < *bp_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    *((*a).data)
                        .offset(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(*bj_0.offset(p_0 as isize) as libc::c_ulong)
                                as isize,
                        ) -= *bd.offset(p_0 as isize);
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_long_dense_sub(
    mut a: *mut gsl_matrix_long,
    mut b: *const gsl_spmatrix_long,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            347 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let mut bd: *const libc::c_long = (*b).data;
        if (*b).nz == 0 as libc::c_int as libc::c_ulong {
            return GSL_SUCCESS as libc::c_int;
        }
        if (*b).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut bi: *const libc::c_int = (*b).i;
            let mut bj: *const libc::c_int = (*b).p;
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*b).nz {
                *((*a).data)
                    .offset(
                        (*bi.offset(n as isize) as libc::c_ulong)
                            .wrapping_mul(tda_a)
                            .wrapping_add(*bj.offset(n as isize) as libc::c_ulong)
                            as isize,
                    ) -= *bd.offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut bi_0: *const libc::c_int = (*b).i;
            let mut bp: *const libc::c_int = (*b).p;
            let mut j: size_t = 0;
            let mut p: libc::c_int = 0;
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *bp.offset(j as isize);
                while p
                    < *bp
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    *((*a).data)
                        .offset(
                            (*bi_0.offset(p as isize) as libc::c_ulong)
                                .wrapping_mul(tda_a)
                                .wrapping_add(j) as isize,
                        ) -= *bd.offset(p as isize);
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut bj_0: *const libc::c_int = (*b).i;
            let mut bp_0: *const libc::c_int = (*b).p;
            let mut i: size_t = 0;
            let mut p_0: libc::c_int = 0;
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *bp_0.offset(i as isize);
                while p_0
                    < *bp_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    *((*a).data)
                        .offset(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(*bj_0.offset(p_0 as isize) as libc::c_ulong)
                                as isize,
                        ) -= *bd.offset(p_0 as isize);
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_dense_sub(
    mut a: *mut gsl_matrix,
    mut b: *const gsl_spmatrix,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            347 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let mut bd: *const libc::c_double = (*b).data;
        if (*b).nz == 0 as libc::c_int as libc::c_ulong {
            return GSL_SUCCESS as libc::c_int;
        }
        if (*b).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut bi: *const libc::c_int = (*b).i;
            let mut bj: *const libc::c_int = (*b).p;
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*b).nz {
                *((*a).data)
                    .offset(
                        (*bi.offset(n as isize) as libc::c_ulong)
                            .wrapping_mul(tda_a)
                            .wrapping_add(*bj.offset(n as isize) as libc::c_ulong)
                            as isize,
                    ) -= *bd.offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut bi_0: *const libc::c_int = (*b).i;
            let mut bp: *const libc::c_int = (*b).p;
            let mut j: size_t = 0;
            let mut p: libc::c_int = 0;
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *bp.offset(j as isize);
                while p
                    < *bp
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    *((*a).data)
                        .offset(
                            (*bi_0.offset(p as isize) as libc::c_ulong)
                                .wrapping_mul(tda_a)
                                .wrapping_add(j) as isize,
                        ) -= *bd.offset(p as isize);
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut bj_0: *const libc::c_int = (*b).i;
            let mut bp_0: *const libc::c_int = (*b).p;
            let mut i: size_t = 0;
            let mut p_0: libc::c_int = 0;
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *bp_0.offset(i as isize);
                while p_0
                    < *bp_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    *((*a).data)
                        .offset(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(*bj_0.offset(p_0 as isize) as libc::c_ulong)
                                as isize,
                        ) -= *bd.offset(p_0 as isize);
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uchar_dense_sub(
    mut a: *mut gsl_matrix_uchar,
    mut b: *const gsl_spmatrix_uchar,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            347 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let mut bd: *const libc::c_uchar = (*b).data;
        if (*b).nz == 0 as libc::c_int as libc::c_ulong {
            return GSL_SUCCESS as libc::c_int;
        }
        if (*b).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut bi: *const libc::c_int = (*b).i;
            let mut bj: *const libc::c_int = (*b).p;
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*b).nz {
                let ref mut fresh63 = *((*a).data)
                    .offset(
                        (*bi.offset(n as isize) as libc::c_ulong)
                            .wrapping_mul(tda_a)
                            .wrapping_add(*bj.offset(n as isize) as libc::c_ulong)
                            as isize,
                    );
                *fresh63 = (*fresh63 as libc::c_int
                    - *bd.offset(n as isize) as libc::c_int) as libc::c_uchar;
                n = n.wrapping_add(1);
                n;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut bi_0: *const libc::c_int = (*b).i;
            let mut bp: *const libc::c_int = (*b).p;
            let mut j: size_t = 0;
            let mut p: libc::c_int = 0;
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *bp.offset(j as isize);
                while p
                    < *bp
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh64 = *((*a).data)
                        .offset(
                            (*bi_0.offset(p as isize) as libc::c_ulong)
                                .wrapping_mul(tda_a)
                                .wrapping_add(j) as isize,
                        );
                    *fresh64 = (*fresh64 as libc::c_int
                        - *bd.offset(p as isize) as libc::c_int) as libc::c_uchar;
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut bj_0: *const libc::c_int = (*b).i;
            let mut bp_0: *const libc::c_int = (*b).p;
            let mut i: size_t = 0;
            let mut p_0: libc::c_int = 0;
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *bp_0.offset(i as isize);
                while p_0
                    < *bp_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh65 = *((*a).data)
                        .offset(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(*bj_0.offset(p_0 as isize) as libc::c_ulong)
                                as isize,
                        );
                    *fresh65 = (*fresh65 as libc::c_int
                        - *bd.offset(p_0 as isize) as libc::c_int) as libc::c_uchar;
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uint_dense_sub(
    mut a: *mut gsl_matrix_uint,
    mut b: *const gsl_spmatrix_uint,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            347 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let mut bd: *const libc::c_uint = (*b).data;
        if (*b).nz == 0 as libc::c_int as libc::c_ulong {
            return GSL_SUCCESS as libc::c_int;
        }
        if (*b).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut bi: *const libc::c_int = (*b).i;
            let mut bj: *const libc::c_int = (*b).p;
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*b).nz {
                let ref mut fresh66 = *((*a).data)
                    .offset(
                        (*bi.offset(n as isize) as libc::c_ulong)
                            .wrapping_mul(tda_a)
                            .wrapping_add(*bj.offset(n as isize) as libc::c_ulong)
                            as isize,
                    );
                *fresh66 = (*fresh66).wrapping_sub(*bd.offset(n as isize));
                n = n.wrapping_add(1);
                n;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut bi_0: *const libc::c_int = (*b).i;
            let mut bp: *const libc::c_int = (*b).p;
            let mut j: size_t = 0;
            let mut p: libc::c_int = 0;
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *bp.offset(j as isize);
                while p
                    < *bp
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh67 = *((*a).data)
                        .offset(
                            (*bi_0.offset(p as isize) as libc::c_ulong)
                                .wrapping_mul(tda_a)
                                .wrapping_add(j) as isize,
                        );
                    *fresh67 = (*fresh67).wrapping_sub(*bd.offset(p as isize));
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut bj_0: *const libc::c_int = (*b).i;
            let mut bp_0: *const libc::c_int = (*b).p;
            let mut i: size_t = 0;
            let mut p_0: libc::c_int = 0;
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *bp_0.offset(i as isize);
                while p_0
                    < *bp_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh68 = *((*a).data)
                        .offset(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(*bj_0.offset(p_0 as isize) as libc::c_ulong)
                                as isize,
                        );
                    *fresh68 = (*fresh68).wrapping_sub(*bd.offset(p_0 as isize));
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_short_dense_sub(
    mut a: *mut gsl_matrix_short,
    mut b: *const gsl_spmatrix_short,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            347 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let mut bd: *const libc::c_short = (*b).data;
        if (*b).nz == 0 as libc::c_int as libc::c_ulong {
            return GSL_SUCCESS as libc::c_int;
        }
        if (*b).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut bi: *const libc::c_int = (*b).i;
            let mut bj: *const libc::c_int = (*b).p;
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*b).nz {
                let ref mut fresh69 = *((*a).data)
                    .offset(
                        (*bi.offset(n as isize) as libc::c_ulong)
                            .wrapping_mul(tda_a)
                            .wrapping_add(*bj.offset(n as isize) as libc::c_ulong)
                            as isize,
                    );
                *fresh69 = (*fresh69 as libc::c_int
                    - *bd.offset(n as isize) as libc::c_int) as libc::c_short;
                n = n.wrapping_add(1);
                n;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut bi_0: *const libc::c_int = (*b).i;
            let mut bp: *const libc::c_int = (*b).p;
            let mut j: size_t = 0;
            let mut p: libc::c_int = 0;
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *bp.offset(j as isize);
                while p
                    < *bp
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh70 = *((*a).data)
                        .offset(
                            (*bi_0.offset(p as isize) as libc::c_ulong)
                                .wrapping_mul(tda_a)
                                .wrapping_add(j) as isize,
                        );
                    *fresh70 = (*fresh70 as libc::c_int
                        - *bd.offset(p as isize) as libc::c_int) as libc::c_short;
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut bj_0: *const libc::c_int = (*b).i;
            let mut bp_0: *const libc::c_int = (*b).p;
            let mut i: size_t = 0;
            let mut p_0: libc::c_int = 0;
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *bp_0.offset(i as isize);
                while p_0
                    < *bp_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh71 = *((*a).data)
                        .offset(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(*bj_0.offset(p_0 as isize) as libc::c_ulong)
                                as isize,
                        );
                    *fresh71 = (*fresh71 as libc::c_int
                        - *bd.offset(p_0 as isize) as libc::c_int) as libc::c_short;
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_ushort_dense_sub(
    mut a: *mut gsl_matrix_ushort,
    mut b: *const gsl_spmatrix_ushort,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            347 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let mut bd: *const libc::c_ushort = (*b).data;
        if (*b).nz == 0 as libc::c_int as libc::c_ulong {
            return GSL_SUCCESS as libc::c_int;
        }
        if (*b).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut bi: *const libc::c_int = (*b).i;
            let mut bj: *const libc::c_int = (*b).p;
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*b).nz {
                let ref mut fresh72 = *((*a).data)
                    .offset(
                        (*bi.offset(n as isize) as libc::c_ulong)
                            .wrapping_mul(tda_a)
                            .wrapping_add(*bj.offset(n as isize) as libc::c_ulong)
                            as isize,
                    );
                *fresh72 = (*fresh72 as libc::c_int
                    - *bd.offset(n as isize) as libc::c_int) as libc::c_ushort;
                n = n.wrapping_add(1);
                n;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut bi_0: *const libc::c_int = (*b).i;
            let mut bp: *const libc::c_int = (*b).p;
            let mut j: size_t = 0;
            let mut p: libc::c_int = 0;
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *bp.offset(j as isize);
                while p
                    < *bp
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh73 = *((*a).data)
                        .offset(
                            (*bi_0.offset(p as isize) as libc::c_ulong)
                                .wrapping_mul(tda_a)
                                .wrapping_add(j) as isize,
                        );
                    *fresh73 = (*fresh73 as libc::c_int
                        - *bd.offset(p as isize) as libc::c_int) as libc::c_ushort;
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut bj_0: *const libc::c_int = (*b).i;
            let mut bp_0: *const libc::c_int = (*b).p;
            let mut i: size_t = 0;
            let mut p_0: libc::c_int = 0;
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *bp_0.offset(i as isize);
                while p_0
                    < *bp_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh74 = *((*a).data)
                        .offset(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(*bj_0.offset(p_0 as isize) as libc::c_ulong)
                                as isize,
                        );
                    *fresh74 = (*fresh74 as libc::c_int
                        - *bd.offset(p_0 as isize) as libc::c_int) as libc::c_ushort;
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_ulong_dense_sub(
    mut a: *mut gsl_matrix_ulong,
    mut b: *const gsl_spmatrix_ulong,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            347 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let mut bd: *const libc::c_ulong = (*b).data;
        if (*b).nz == 0 as libc::c_int as libc::c_ulong {
            return GSL_SUCCESS as libc::c_int;
        }
        if (*b).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut bi: *const libc::c_int = (*b).i;
            let mut bj: *const libc::c_int = (*b).p;
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*b).nz {
                let ref mut fresh75 = *((*a).data)
                    .offset(
                        (*bi.offset(n as isize) as libc::c_ulong)
                            .wrapping_mul(tda_a)
                            .wrapping_add(*bj.offset(n as isize) as libc::c_ulong)
                            as isize,
                    );
                *fresh75 = (*fresh75).wrapping_sub(*bd.offset(n as isize));
                n = n.wrapping_add(1);
                n;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut bi_0: *const libc::c_int = (*b).i;
            let mut bp: *const libc::c_int = (*b).p;
            let mut j: size_t = 0;
            let mut p: libc::c_int = 0;
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *bp.offset(j as isize);
                while p
                    < *bp
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh76 = *((*a).data)
                        .offset(
                            (*bi_0.offset(p as isize) as libc::c_ulong)
                                .wrapping_mul(tda_a)
                                .wrapping_add(j) as isize,
                        );
                    *fresh76 = (*fresh76).wrapping_sub(*bd.offset(p as isize));
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut bj_0: *const libc::c_int = (*b).i;
            let mut bp_0: *const libc::c_int = (*b).p;
            let mut i: size_t = 0;
            let mut p_0: libc::c_int = 0;
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *bp_0.offset(i as isize);
                while p_0
                    < *bp_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh77 = *((*a).data)
                        .offset(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(*bj_0.offset(p_0 as isize) as libc::c_ulong)
                                as isize,
                        );
                    *fresh77 = (*fresh77).wrapping_sub(*bd.offset(p_0 as isize));
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_char_dense_sub(
    mut a: *mut gsl_matrix_char,
    mut b: *const gsl_spmatrix_char,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            347 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let tda_a: size_t = (*a).tda;
        let mut bd: *const libc::c_char = (*b).data;
        if (*b).nz == 0 as libc::c_int as libc::c_ulong {
            return GSL_SUCCESS as libc::c_int;
        }
        if (*b).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut bi: *const libc::c_int = (*b).i;
            let mut bj: *const libc::c_int = (*b).p;
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*b).nz {
                let ref mut fresh78 = *((*a).data)
                    .offset(
                        (*bi.offset(n as isize) as libc::c_ulong)
                            .wrapping_mul(tda_a)
                            .wrapping_add(*bj.offset(n as isize) as libc::c_ulong)
                            as isize,
                    );
                *fresh78 = (*fresh78 as libc::c_int
                    - *bd.offset(n as isize) as libc::c_int) as libc::c_char;
                n = n.wrapping_add(1);
                n;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut bi_0: *const libc::c_int = (*b).i;
            let mut bp: *const libc::c_int = (*b).p;
            let mut j: size_t = 0;
            let mut p: libc::c_int = 0;
            j = 0 as libc::c_int as size_t;
            while j < N {
                p = *bp.offset(j as isize);
                while p
                    < *bp
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh79 = *((*a).data)
                        .offset(
                            (*bi_0.offset(p as isize) as libc::c_ulong)
                                .wrapping_mul(tda_a)
                                .wrapping_add(j) as isize,
                        );
                    *fresh79 = (*fresh79 as libc::c_int
                        - *bd.offset(p as isize) as libc::c_int) as libc::c_char;
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*b).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut bj_0: *const libc::c_int = (*b).i;
            let mut bp_0: *const libc::c_int = (*b).p;
            let mut i: size_t = 0;
            let mut p_0: libc::c_int = 0;
            i = 0 as libc::c_int as size_t;
            while i < M {
                p_0 = *bp_0.offset(i as isize);
                while p_0
                    < *bp_0
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    let ref mut fresh80 = *((*a).data)
                        .offset(
                            i
                                .wrapping_mul(tda_a)
                                .wrapping_add(*bj_0.offset(p_0 as isize) as libc::c_ulong)
                                as isize,
                        );
                    *fresh80 = (*fresh80 as libc::c_int
                        - *bd.offset(p_0 as isize) as libc::c_int) as libc::c_char;
                    p_0 += 1;
                    p_0;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_short_d2sp(
    mut T: *mut gsl_spmatrix_short,
    mut A: *const gsl_matrix_short,
) -> libc::c_int {
    if (*T).size1 != (*A).size1 || (*T).size2 != (*A).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            417 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if !((*T).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"sparse matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            421 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        gsl_spmatrix_short_set_zero(T);
        i = 0 as libc::c_int as size_t;
        while i < (*A).size1 {
            j = 0 as libc::c_int as size_t;
            while j < (*A).size2 {
                let mut x: libc::c_short = gsl_matrix_short_get(A, i, j);
                if x as libc::c_int != 0 as libc::c_int as libc::c_short as libc::c_int {
                    gsl_spmatrix_short_set(T, i, j, x);
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_float_d2sp(
    mut T: *mut gsl_spmatrix_float,
    mut A: *const gsl_matrix_float,
) -> libc::c_int {
    if (*T).size1 != (*A).size1 || (*T).size2 != (*A).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            417 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if !((*T).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"sparse matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            421 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        gsl_spmatrix_float_set_zero(T);
        i = 0 as libc::c_int as size_t;
        while i < (*A).size1 {
            j = 0 as libc::c_int as size_t;
            while j < (*A).size2 {
                let mut x: libc::c_float = gsl_matrix_float_get(A, i, j);
                if x != 0.0f32 {
                    gsl_spmatrix_float_set(T, i, j, x);
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_d2sp(
    mut T: *mut gsl_spmatrix_long_double,
    mut A: *const gsl_matrix_long_double,
) -> libc::c_int {
    if (*T).size1 != (*A).size1 || (*T).size2 != (*A).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            417 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if !((*T).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"sparse matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            421 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        gsl_spmatrix_long_double_set_zero(T);
        i = 0 as libc::c_int as size_t;
        while i < (*A).size1 {
            j = 0 as libc::c_int as size_t;
            while j < (*A).size2 {
                let mut x: f128::f128 = gsl_matrix_long_double_get(A, i, j);
                if x != f128::f128::new(0.0) {
                    gsl_spmatrix_long_double_set(T, i, j, x);
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_char_d2sp(
    mut T: *mut gsl_spmatrix_char,
    mut A: *const gsl_matrix_char,
) -> libc::c_int {
    if (*T).size1 != (*A).size1 || (*T).size2 != (*A).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            417 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if !((*T).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"sparse matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            421 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        gsl_spmatrix_char_set_zero(T);
        i = 0 as libc::c_int as size_t;
        while i < (*A).size1 {
            j = 0 as libc::c_int as size_t;
            while j < (*A).size2 {
                let mut x: libc::c_char = gsl_matrix_char_get(A, i, j);
                if x as libc::c_int != 0 as libc::c_int as libc::c_char as libc::c_int {
                    gsl_spmatrix_char_set(T, i, j, x);
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_ushort_d2sp(
    mut T: *mut gsl_spmatrix_ushort,
    mut A: *const gsl_matrix_ushort,
) -> libc::c_int {
    if (*T).size1 != (*A).size1 || (*T).size2 != (*A).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            417 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if !((*T).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"sparse matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            421 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        gsl_spmatrix_ushort_set_zero(T);
        i = 0 as libc::c_int as size_t;
        while i < (*A).size1 {
            j = 0 as libc::c_int as size_t;
            while j < (*A).size2 {
                let mut x: libc::c_ushort = gsl_matrix_ushort_get(A, i, j);
                if x as libc::c_int != 0 as libc::c_uint as libc::c_ushort as libc::c_int
                {
                    gsl_spmatrix_ushort_set(T, i, j, x);
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uchar_d2sp(
    mut T: *mut gsl_spmatrix_uchar,
    mut A: *const gsl_matrix_uchar,
) -> libc::c_int {
    if (*T).size1 != (*A).size1 || (*T).size2 != (*A).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            417 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if !((*T).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"sparse matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            421 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        gsl_spmatrix_uchar_set_zero(T);
        i = 0 as libc::c_int as size_t;
        while i < (*A).size1 {
            j = 0 as libc::c_int as size_t;
            while j < (*A).size2 {
                let mut x: libc::c_uchar = gsl_matrix_uchar_get(A, i, j);
                if x as libc::c_int != 0 as libc::c_uint as libc::c_uchar as libc::c_int
                {
                    gsl_spmatrix_uchar_set(T, i, j, x);
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_int_d2sp(
    mut T: *mut gsl_spmatrix_int,
    mut A: *const gsl_matrix_int,
) -> libc::c_int {
    if (*T).size1 != (*A).size1 || (*T).size2 != (*A).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            417 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if !((*T).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"sparse matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            421 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        gsl_spmatrix_int_set_zero(T);
        i = 0 as libc::c_int as size_t;
        while i < (*A).size1 {
            j = 0 as libc::c_int as size_t;
            while j < (*A).size2 {
                let mut x: libc::c_int = gsl_matrix_int_get(A, i, j);
                if x != 0 as libc::c_int {
                    gsl_spmatrix_int_set(T, i, j, x);
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_ulong_d2sp(
    mut T: *mut gsl_spmatrix_ulong,
    mut A: *const gsl_matrix_ulong,
) -> libc::c_int {
    if (*T).size1 != (*A).size1 || (*T).size2 != (*A).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            417 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if !((*T).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"sparse matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            421 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        gsl_spmatrix_ulong_set_zero(T);
        i = 0 as libc::c_int as size_t;
        while i < (*A).size1 {
            j = 0 as libc::c_int as size_t;
            while j < (*A).size2 {
                let mut x: libc::c_ulong = gsl_matrix_ulong_get(A, i, j);
                if x != 0 as libc::c_ulong {
                    gsl_spmatrix_ulong_set(T, i, j, x);
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_long_d2sp(
    mut T: *mut gsl_spmatrix_long,
    mut A: *const gsl_matrix_long,
) -> libc::c_int {
    if (*T).size1 != (*A).size1 || (*T).size2 != (*A).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            417 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if !((*T).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"sparse matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            421 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        gsl_spmatrix_long_set_zero(T);
        i = 0 as libc::c_int as size_t;
        while i < (*A).size1 {
            j = 0 as libc::c_int as size_t;
            while j < (*A).size2 {
                let mut x: libc::c_long = gsl_matrix_long_get(A, i, j);
                if x != 0 as libc::c_long {
                    gsl_spmatrix_long_set(T, i, j, x);
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_d2sp(
    mut T: *mut gsl_spmatrix,
    mut A: *const gsl_matrix,
) -> libc::c_int {
    if (*T).size1 != (*A).size1 || (*T).size2 != (*A).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            417 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if !((*T).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"sparse matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            421 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        gsl_spmatrix_set_zero(T);
        i = 0 as libc::c_int as size_t;
        while i < (*A).size1 {
            j = 0 as libc::c_int as size_t;
            while j < (*A).size2 {
                let mut x: libc::c_double = gsl_matrix_get(A, i, j);
                if x != 0.0f64 {
                    gsl_spmatrix_set(T, i, j, x);
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uint_d2sp(
    mut T: *mut gsl_spmatrix_uint,
    mut A: *const gsl_matrix_uint,
) -> libc::c_int {
    if (*T).size1 != (*A).size1 || (*T).size2 != (*A).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            417 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if !((*T).sptype == GSL_SPMATRIX_COO as libc::c_int) {
        gsl_error(
            b"sparse matrix must be in COO format\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            421 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        gsl_spmatrix_uint_set_zero(T);
        i = 0 as libc::c_int as size_t;
        while i < (*A).size1 {
            j = 0 as libc::c_int as size_t;
            while j < (*A).size2 {
                let mut x: libc::c_uint = gsl_matrix_uint_get(A, i, j);
                if x != 0 as libc::c_uint {
                    gsl_spmatrix_uint_set(T, i, j, x);
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_ulong_sp2d(
    mut A: *mut gsl_matrix_ulong,
    mut S: *const gsl_spmatrix_ulong,
) -> libc::c_int {
    if (*A).size1 != (*S).size1 || (*A).size2 != (*S).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            454 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        gsl_matrix_ulong_set_zero(A);
        if (*S).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*S).nz {
                let mut i: libc::c_int = *((*S).i).offset(n as isize);
                let mut j: libc::c_int = *((*S).p).offset(n as isize);
                let mut x: libc::c_ulong = *((*S).data).offset(n as isize);
                gsl_matrix_ulong_set(A, i as size_t, j as size_t, x);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*S).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Sj: *const libc::c_int = (*S).i;
            let mut Sp: *const libc::c_int = (*S).p;
            let mut Sd: *const libc::c_ulong = (*S).data;
            let mut i_0: size_t = 0;
            let mut p: libc::c_int = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*S).size1 {
                p = *Sp.offset(i_0 as isize);
                while p
                    < *Sp
                        .offset(
                            i_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    gsl_matrix_ulong_set(
                        A,
                        i_0,
                        *Sj.offset(p as isize) as size_t,
                        *Sd.offset(p as isize),
                    );
                    p += 1;
                    p;
                }
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else if (*S).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Si: *const libc::c_int = (*S).i;
            let mut Sp_0: *const libc::c_int = (*S).p;
            let mut Sd_0: *const libc::c_ulong = (*S).data;
            let mut j_0: size_t = 0;
            let mut p_0: libc::c_int = 0;
            j_0 = 0 as libc::c_int as size_t;
            while j_0 < (*S).size2 {
                p_0 = *Sp_0.offset(j_0 as isize);
                while p_0
                    < *Sp_0
                        .offset(
                            j_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    gsl_matrix_ulong_set(
                        A,
                        *Si.offset(p_0 as isize) as size_t,
                        j_0,
                        *Sd_0.offset(p_0 as isize),
                    );
                    p_0 += 1;
                    p_0;
                }
                j_0 = j_0.wrapping_add(1);
                j_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                503 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_float_sp2d(
    mut A: *mut gsl_matrix_float,
    mut S: *const gsl_spmatrix_float,
) -> libc::c_int {
    if (*A).size1 != (*S).size1 || (*A).size2 != (*S).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            454 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        gsl_matrix_float_set_zero(A);
        if (*S).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*S).nz {
                let mut i: libc::c_int = *((*S).i).offset(n as isize);
                let mut j: libc::c_int = *((*S).p).offset(n as isize);
                let mut x: libc::c_float = *((*S).data).offset(n as isize);
                gsl_matrix_float_set(A, i as size_t, j as size_t, x);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*S).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Sj: *const libc::c_int = (*S).i;
            let mut Sp: *const libc::c_int = (*S).p;
            let mut Sd: *const libc::c_float = (*S).data;
            let mut i_0: size_t = 0;
            let mut p: libc::c_int = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*S).size1 {
                p = *Sp.offset(i_0 as isize);
                while p
                    < *Sp
                        .offset(
                            i_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    gsl_matrix_float_set(
                        A,
                        i_0,
                        *Sj.offset(p as isize) as size_t,
                        *Sd.offset(p as isize),
                    );
                    p += 1;
                    p;
                }
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else if (*S).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Si: *const libc::c_int = (*S).i;
            let mut Sp_0: *const libc::c_int = (*S).p;
            let mut Sd_0: *const libc::c_float = (*S).data;
            let mut j_0: size_t = 0;
            let mut p_0: libc::c_int = 0;
            j_0 = 0 as libc::c_int as size_t;
            while j_0 < (*S).size2 {
                p_0 = *Sp_0.offset(j_0 as isize);
                while p_0
                    < *Sp_0
                        .offset(
                            j_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    gsl_matrix_float_set(
                        A,
                        *Si.offset(p_0 as isize) as size_t,
                        j_0,
                        *Sd_0.offset(p_0 as isize),
                    );
                    p_0 += 1;
                    p_0;
                }
                j_0 = j_0.wrapping_add(1);
                j_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                503 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uint_sp2d(
    mut A: *mut gsl_matrix_uint,
    mut S: *const gsl_spmatrix_uint,
) -> libc::c_int {
    if (*A).size1 != (*S).size1 || (*A).size2 != (*S).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            454 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        gsl_matrix_uint_set_zero(A);
        if (*S).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*S).nz {
                let mut i: libc::c_int = *((*S).i).offset(n as isize);
                let mut j: libc::c_int = *((*S).p).offset(n as isize);
                let mut x: libc::c_uint = *((*S).data).offset(n as isize);
                gsl_matrix_uint_set(A, i as size_t, j as size_t, x);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*S).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Sj: *const libc::c_int = (*S).i;
            let mut Sp: *const libc::c_int = (*S).p;
            let mut Sd: *const libc::c_uint = (*S).data;
            let mut i_0: size_t = 0;
            let mut p: libc::c_int = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*S).size1 {
                p = *Sp.offset(i_0 as isize);
                while p
                    < *Sp
                        .offset(
                            i_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    gsl_matrix_uint_set(
                        A,
                        i_0,
                        *Sj.offset(p as isize) as size_t,
                        *Sd.offset(p as isize),
                    );
                    p += 1;
                    p;
                }
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else if (*S).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Si: *const libc::c_int = (*S).i;
            let mut Sp_0: *const libc::c_int = (*S).p;
            let mut Sd_0: *const libc::c_uint = (*S).data;
            let mut j_0: size_t = 0;
            let mut p_0: libc::c_int = 0;
            j_0 = 0 as libc::c_int as size_t;
            while j_0 < (*S).size2 {
                p_0 = *Sp_0.offset(j_0 as isize);
                while p_0
                    < *Sp_0
                        .offset(
                            j_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    gsl_matrix_uint_set(
                        A,
                        *Si.offset(p_0 as isize) as size_t,
                        j_0,
                        *Sd_0.offset(p_0 as isize),
                    );
                    p_0 += 1;
                    p_0;
                }
                j_0 = j_0.wrapping_add(1);
                j_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                503 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_short_sp2d(
    mut A: *mut gsl_matrix_short,
    mut S: *const gsl_spmatrix_short,
) -> libc::c_int {
    if (*A).size1 != (*S).size1 || (*A).size2 != (*S).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            454 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        gsl_matrix_short_set_zero(A);
        if (*S).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*S).nz {
                let mut i: libc::c_int = *((*S).i).offset(n as isize);
                let mut j: libc::c_int = *((*S).p).offset(n as isize);
                let mut x: libc::c_short = *((*S).data).offset(n as isize);
                gsl_matrix_short_set(A, i as size_t, j as size_t, x);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*S).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Sj: *const libc::c_int = (*S).i;
            let mut Sp: *const libc::c_int = (*S).p;
            let mut Sd: *const libc::c_short = (*S).data;
            let mut i_0: size_t = 0;
            let mut p: libc::c_int = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*S).size1 {
                p = *Sp.offset(i_0 as isize);
                while p
                    < *Sp
                        .offset(
                            i_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    gsl_matrix_short_set(
                        A,
                        i_0,
                        *Sj.offset(p as isize) as size_t,
                        *Sd.offset(p as isize),
                    );
                    p += 1;
                    p;
                }
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else if (*S).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Si: *const libc::c_int = (*S).i;
            let mut Sp_0: *const libc::c_int = (*S).p;
            let mut Sd_0: *const libc::c_short = (*S).data;
            let mut j_0: size_t = 0;
            let mut p_0: libc::c_int = 0;
            j_0 = 0 as libc::c_int as size_t;
            while j_0 < (*S).size2 {
                p_0 = *Sp_0.offset(j_0 as isize);
                while p_0
                    < *Sp_0
                        .offset(
                            j_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    gsl_matrix_short_set(
                        A,
                        *Si.offset(p_0 as isize) as size_t,
                        j_0,
                        *Sd_0.offset(p_0 as isize),
                    );
                    p_0 += 1;
                    p_0;
                }
                j_0 = j_0.wrapping_add(1);
                j_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                503 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_sp2d(
    mut A: *mut gsl_matrix_long_double,
    mut S: *const gsl_spmatrix_long_double,
) -> libc::c_int {
    if (*A).size1 != (*S).size1 || (*A).size2 != (*S).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            454 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        gsl_matrix_long_double_set_zero(A);
        if (*S).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*S).nz {
                let mut i: libc::c_int = *((*S).i).offset(n as isize);
                let mut j: libc::c_int = *((*S).p).offset(n as isize);
                let mut x: f128::f128 = *((*S).data).offset(n as isize);
                gsl_matrix_long_double_set(A, i as size_t, j as size_t, x);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*S).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Sj: *const libc::c_int = (*S).i;
            let mut Sp: *const libc::c_int = (*S).p;
            let mut Sd: *const f128::f128 = (*S).data;
            let mut i_0: size_t = 0;
            let mut p: libc::c_int = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*S).size1 {
                p = *Sp.offset(i_0 as isize);
                while p
                    < *Sp
                        .offset(
                            i_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    gsl_matrix_long_double_set(
                        A,
                        i_0,
                        *Sj.offset(p as isize) as size_t,
                        *Sd.offset(p as isize),
                    );
                    p += 1;
                    p;
                }
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else if (*S).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Si: *const libc::c_int = (*S).i;
            let mut Sp_0: *const libc::c_int = (*S).p;
            let mut Sd_0: *const f128::f128 = (*S).data;
            let mut j_0: size_t = 0;
            let mut p_0: libc::c_int = 0;
            j_0 = 0 as libc::c_int as size_t;
            while j_0 < (*S).size2 {
                p_0 = *Sp_0.offset(j_0 as isize);
                while p_0
                    < *Sp_0
                        .offset(
                            j_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    gsl_matrix_long_double_set(
                        A,
                        *Si.offset(p_0 as isize) as size_t,
                        j_0,
                        *Sd_0.offset(p_0 as isize),
                    );
                    p_0 += 1;
                    p_0;
                }
                j_0 = j_0.wrapping_add(1);
                j_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                503 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_long_sp2d(
    mut A: *mut gsl_matrix_long,
    mut S: *const gsl_spmatrix_long,
) -> libc::c_int {
    if (*A).size1 != (*S).size1 || (*A).size2 != (*S).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            454 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        gsl_matrix_long_set_zero(A);
        if (*S).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*S).nz {
                let mut i: libc::c_int = *((*S).i).offset(n as isize);
                let mut j: libc::c_int = *((*S).p).offset(n as isize);
                let mut x: libc::c_long = *((*S).data).offset(n as isize);
                gsl_matrix_long_set(A, i as size_t, j as size_t, x);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*S).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Sj: *const libc::c_int = (*S).i;
            let mut Sp: *const libc::c_int = (*S).p;
            let mut Sd: *const libc::c_long = (*S).data;
            let mut i_0: size_t = 0;
            let mut p: libc::c_int = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*S).size1 {
                p = *Sp.offset(i_0 as isize);
                while p
                    < *Sp
                        .offset(
                            i_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    gsl_matrix_long_set(
                        A,
                        i_0,
                        *Sj.offset(p as isize) as size_t,
                        *Sd.offset(p as isize),
                    );
                    p += 1;
                    p;
                }
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else if (*S).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Si: *const libc::c_int = (*S).i;
            let mut Sp_0: *const libc::c_int = (*S).p;
            let mut Sd_0: *const libc::c_long = (*S).data;
            let mut j_0: size_t = 0;
            let mut p_0: libc::c_int = 0;
            j_0 = 0 as libc::c_int as size_t;
            while j_0 < (*S).size2 {
                p_0 = *Sp_0.offset(j_0 as isize);
                while p_0
                    < *Sp_0
                        .offset(
                            j_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    gsl_matrix_long_set(
                        A,
                        *Si.offset(p_0 as isize) as size_t,
                        j_0,
                        *Sd_0.offset(p_0 as isize),
                    );
                    p_0 += 1;
                    p_0;
                }
                j_0 = j_0.wrapping_add(1);
                j_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                503 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_int_sp2d(
    mut A: *mut gsl_matrix_int,
    mut S: *const gsl_spmatrix_int,
) -> libc::c_int {
    if (*A).size1 != (*S).size1 || (*A).size2 != (*S).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            454 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        gsl_matrix_int_set_zero(A);
        if (*S).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*S).nz {
                let mut i: libc::c_int = *((*S).i).offset(n as isize);
                let mut j: libc::c_int = *((*S).p).offset(n as isize);
                let mut x: libc::c_int = *((*S).data).offset(n as isize);
                gsl_matrix_int_set(A, i as size_t, j as size_t, x);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*S).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Sj: *const libc::c_int = (*S).i;
            let mut Sp: *const libc::c_int = (*S).p;
            let mut Sd: *const libc::c_int = (*S).data;
            let mut i_0: size_t = 0;
            let mut p: libc::c_int = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*S).size1 {
                p = *Sp.offset(i_0 as isize);
                while p
                    < *Sp
                        .offset(
                            i_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    gsl_matrix_int_set(
                        A,
                        i_0,
                        *Sj.offset(p as isize) as size_t,
                        *Sd.offset(p as isize),
                    );
                    p += 1;
                    p;
                }
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else if (*S).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Si: *const libc::c_int = (*S).i;
            let mut Sp_0: *const libc::c_int = (*S).p;
            let mut Sd_0: *const libc::c_int = (*S).data;
            let mut j_0: size_t = 0;
            let mut p_0: libc::c_int = 0;
            j_0 = 0 as libc::c_int as size_t;
            while j_0 < (*S).size2 {
                p_0 = *Sp_0.offset(j_0 as isize);
                while p_0
                    < *Sp_0
                        .offset(
                            j_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    gsl_matrix_int_set(
                        A,
                        *Si.offset(p_0 as isize) as size_t,
                        j_0,
                        *Sd_0.offset(p_0 as isize),
                    );
                    p_0 += 1;
                    p_0;
                }
                j_0 = j_0.wrapping_add(1);
                j_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                503 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_sp2d(
    mut A: *mut gsl_matrix,
    mut S: *const gsl_spmatrix,
) -> libc::c_int {
    if (*A).size1 != (*S).size1 || (*A).size2 != (*S).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            454 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        gsl_matrix_set_zero(A);
        if (*S).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*S).nz {
                let mut i: libc::c_int = *((*S).i).offset(n as isize);
                let mut j: libc::c_int = *((*S).p).offset(n as isize);
                let mut x: libc::c_double = *((*S).data).offset(n as isize);
                gsl_matrix_set(A, i as size_t, j as size_t, x);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*S).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Sj: *const libc::c_int = (*S).i;
            let mut Sp: *const libc::c_int = (*S).p;
            let mut Sd: *const libc::c_double = (*S).data;
            let mut i_0: size_t = 0;
            let mut p: libc::c_int = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*S).size1 {
                p = *Sp.offset(i_0 as isize);
                while p
                    < *Sp
                        .offset(
                            i_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    gsl_matrix_set(
                        A,
                        i_0,
                        *Sj.offset(p as isize) as size_t,
                        *Sd.offset(p as isize),
                    );
                    p += 1;
                    p;
                }
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else if (*S).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Si: *const libc::c_int = (*S).i;
            let mut Sp_0: *const libc::c_int = (*S).p;
            let mut Sd_0: *const libc::c_double = (*S).data;
            let mut j_0: size_t = 0;
            let mut p_0: libc::c_int = 0;
            j_0 = 0 as libc::c_int as size_t;
            while j_0 < (*S).size2 {
                p_0 = *Sp_0.offset(j_0 as isize);
                while p_0
                    < *Sp_0
                        .offset(
                            j_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    gsl_matrix_set(
                        A,
                        *Si.offset(p_0 as isize) as size_t,
                        j_0,
                        *Sd_0.offset(p_0 as isize),
                    );
                    p_0 += 1;
                    p_0;
                }
                j_0 = j_0.wrapping_add(1);
                j_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                503 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_char_sp2d(
    mut A: *mut gsl_matrix_char,
    mut S: *const gsl_spmatrix_char,
) -> libc::c_int {
    if (*A).size1 != (*S).size1 || (*A).size2 != (*S).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            454 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        gsl_matrix_char_set_zero(A);
        if (*S).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*S).nz {
                let mut i: libc::c_int = *((*S).i).offset(n as isize);
                let mut j: libc::c_int = *((*S).p).offset(n as isize);
                let mut x: libc::c_char = *((*S).data).offset(n as isize);
                gsl_matrix_char_set(A, i as size_t, j as size_t, x);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*S).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Sj: *const libc::c_int = (*S).i;
            let mut Sp: *const libc::c_int = (*S).p;
            let mut Sd: *const libc::c_char = (*S).data;
            let mut i_0: size_t = 0;
            let mut p: libc::c_int = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*S).size1 {
                p = *Sp.offset(i_0 as isize);
                while p
                    < *Sp
                        .offset(
                            i_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    gsl_matrix_char_set(
                        A,
                        i_0,
                        *Sj.offset(p as isize) as size_t,
                        *Sd.offset(p as isize),
                    );
                    p += 1;
                    p;
                }
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else if (*S).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Si: *const libc::c_int = (*S).i;
            let mut Sp_0: *const libc::c_int = (*S).p;
            let mut Sd_0: *const libc::c_char = (*S).data;
            let mut j_0: size_t = 0;
            let mut p_0: libc::c_int = 0;
            j_0 = 0 as libc::c_int as size_t;
            while j_0 < (*S).size2 {
                p_0 = *Sp_0.offset(j_0 as isize);
                while p_0
                    < *Sp_0
                        .offset(
                            j_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    gsl_matrix_char_set(
                        A,
                        *Si.offset(p_0 as isize) as size_t,
                        j_0,
                        *Sd_0.offset(p_0 as isize),
                    );
                    p_0 += 1;
                    p_0;
                }
                j_0 = j_0.wrapping_add(1);
                j_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                503 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uchar_sp2d(
    mut A: *mut gsl_matrix_uchar,
    mut S: *const gsl_spmatrix_uchar,
) -> libc::c_int {
    if (*A).size1 != (*S).size1 || (*A).size2 != (*S).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            454 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        gsl_matrix_uchar_set_zero(A);
        if (*S).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*S).nz {
                let mut i: libc::c_int = *((*S).i).offset(n as isize);
                let mut j: libc::c_int = *((*S).p).offset(n as isize);
                let mut x: libc::c_uchar = *((*S).data).offset(n as isize);
                gsl_matrix_uchar_set(A, i as size_t, j as size_t, x);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*S).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Sj: *const libc::c_int = (*S).i;
            let mut Sp: *const libc::c_int = (*S).p;
            let mut Sd: *const libc::c_uchar = (*S).data;
            let mut i_0: size_t = 0;
            let mut p: libc::c_int = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*S).size1 {
                p = *Sp.offset(i_0 as isize);
                while p
                    < *Sp
                        .offset(
                            i_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    gsl_matrix_uchar_set(
                        A,
                        i_0,
                        *Sj.offset(p as isize) as size_t,
                        *Sd.offset(p as isize),
                    );
                    p += 1;
                    p;
                }
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else if (*S).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Si: *const libc::c_int = (*S).i;
            let mut Sp_0: *const libc::c_int = (*S).p;
            let mut Sd_0: *const libc::c_uchar = (*S).data;
            let mut j_0: size_t = 0;
            let mut p_0: libc::c_int = 0;
            j_0 = 0 as libc::c_int as size_t;
            while j_0 < (*S).size2 {
                p_0 = *Sp_0.offset(j_0 as isize);
                while p_0
                    < *Sp_0
                        .offset(
                            j_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    gsl_matrix_uchar_set(
                        A,
                        *Si.offset(p_0 as isize) as size_t,
                        j_0,
                        *Sd_0.offset(p_0 as isize),
                    );
                    p_0 += 1;
                    p_0;
                }
                j_0 = j_0.wrapping_add(1);
                j_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                503 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_ushort_sp2d(
    mut A: *mut gsl_matrix_ushort,
    mut S: *const gsl_spmatrix_ushort,
) -> libc::c_int {
    if (*A).size1 != (*S).size1 || (*A).size2 != (*S).size2 {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            454 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        gsl_matrix_ushort_set_zero(A);
        if (*S).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < (*S).nz {
                let mut i: libc::c_int = *((*S).i).offset(n as isize);
                let mut j: libc::c_int = *((*S).p).offset(n as isize);
                let mut x: libc::c_ushort = *((*S).data).offset(n as isize);
                gsl_matrix_ushort_set(A, i as size_t, j as size_t, x);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*S).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Sj: *const libc::c_int = (*S).i;
            let mut Sp: *const libc::c_int = (*S).p;
            let mut Sd: *const libc::c_ushort = (*S).data;
            let mut i_0: size_t = 0;
            let mut p: libc::c_int = 0;
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < (*S).size1 {
                p = *Sp.offset(i_0 as isize);
                while p
                    < *Sp
                        .offset(
                            i_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    gsl_matrix_ushort_set(
                        A,
                        i_0,
                        *Sj.offset(p as isize) as size_t,
                        *Sd.offset(p as isize),
                    );
                    p += 1;
                    p;
                }
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else if (*S).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            let mut Si: *const libc::c_int = (*S).i;
            let mut Sp_0: *const libc::c_int = (*S).p;
            let mut Sd_0: *const libc::c_ushort = (*S).data;
            let mut j_0: size_t = 0;
            let mut p_0: libc::c_int = 0;
            j_0 = 0 as libc::c_int as size_t;
            while j_0 < (*S).size2 {
                p_0 = *Sp_0.offset(j_0 as isize);
                while p_0
                    < *Sp_0
                        .offset(
                            j_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    gsl_matrix_ushort_set(
                        A,
                        *Si.offset(p_0 as isize) as size_t,
                        j_0,
                        *Sd_0.offset(p_0 as isize),
                    );
                    p_0 += 1;
                    p_0;
                }
                j_0 = j_0.wrapping_add(1);
                j_0;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./oper_source.c\0" as *const u8 as *const libc::c_char,
                503 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn spmatrix_short_scatter(
    mut A: *const gsl_spmatrix_short,
    j: size_t,
    mut w: *mut libc::c_int,
    mut x: *mut libc::c_short,
    mark: libc::c_int,
    mut C: *mut gsl_spmatrix_short,
    mut nz: size_t,
) -> size_t {
    let mut p: libc::c_int = 0;
    let mut Ai: *mut libc::c_int = (*A).i;
    let mut Ap: *mut libc::c_int = (*A).p;
    let mut Ad: *mut libc::c_short = (*A).data;
    let mut Ci: *mut libc::c_int = (*C).i;
    p = *Ap.offset(j as isize);
    while p < *Ap.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize) {
        let mut i: libc::c_int = *Ai.offset(p as isize);
        if *w.offset(i as isize) < mark {
            *w.offset(i as isize) = mark;
            let fresh81 = nz;
            nz = nz.wrapping_add(1);
            *Ci.offset(fresh81 as isize) = i;
            *x.offset(i as isize) = *Ad.offset(p as isize);
        } else {
            let ref mut fresh82 = *x.offset(i as isize);
            *fresh82 = (*fresh82 as libc::c_int + *Ad.offset(p as isize) as libc::c_int)
                as libc::c_short;
        }
        p += 1;
        p;
    }
    return nz;
}
unsafe extern "C" fn spmatrix_long_scatter(
    mut A: *const gsl_spmatrix_long,
    j: size_t,
    mut w: *mut libc::c_int,
    mut x: *mut libc::c_long,
    mark: libc::c_int,
    mut C: *mut gsl_spmatrix_long,
    mut nz: size_t,
) -> size_t {
    let mut p: libc::c_int = 0;
    let mut Ai: *mut libc::c_int = (*A).i;
    let mut Ap: *mut libc::c_int = (*A).p;
    let mut Ad: *mut libc::c_long = (*A).data;
    let mut Ci: *mut libc::c_int = (*C).i;
    p = *Ap.offset(j as isize);
    while p < *Ap.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize) {
        let mut i: libc::c_int = *Ai.offset(p as isize);
        if *w.offset(i as isize) < mark {
            *w.offset(i as isize) = mark;
            let fresh83 = nz;
            nz = nz.wrapping_add(1);
            *Ci.offset(fresh83 as isize) = i;
            *x.offset(i as isize) = *Ad.offset(p as isize);
        } else {
            *x.offset(i as isize) += *Ad.offset(p as isize);
        }
        p += 1;
        p;
    }
    return nz;
}
unsafe extern "C" fn spmatrix_ushort_scatter(
    mut A: *const gsl_spmatrix_ushort,
    j: size_t,
    mut w: *mut libc::c_int,
    mut x: *mut libc::c_ushort,
    mark: libc::c_int,
    mut C: *mut gsl_spmatrix_ushort,
    mut nz: size_t,
) -> size_t {
    let mut p: libc::c_int = 0;
    let mut Ai: *mut libc::c_int = (*A).i;
    let mut Ap: *mut libc::c_int = (*A).p;
    let mut Ad: *mut libc::c_ushort = (*A).data;
    let mut Ci: *mut libc::c_int = (*C).i;
    p = *Ap.offset(j as isize);
    while p < *Ap.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize) {
        let mut i: libc::c_int = *Ai.offset(p as isize);
        if *w.offset(i as isize) < mark {
            *w.offset(i as isize) = mark;
            let fresh84 = nz;
            nz = nz.wrapping_add(1);
            *Ci.offset(fresh84 as isize) = i;
            *x.offset(i as isize) = *Ad.offset(p as isize);
        } else {
            let ref mut fresh85 = *x.offset(i as isize);
            *fresh85 = (*fresh85 as libc::c_int + *Ad.offset(p as isize) as libc::c_int)
                as libc::c_ushort;
        }
        p += 1;
        p;
    }
    return nz;
}
unsafe extern "C" fn spmatrix_uint_scatter(
    mut A: *const gsl_spmatrix_uint,
    j: size_t,
    mut w: *mut libc::c_int,
    mut x: *mut libc::c_uint,
    mark: libc::c_int,
    mut C: *mut gsl_spmatrix_uint,
    mut nz: size_t,
) -> size_t {
    let mut p: libc::c_int = 0;
    let mut Ai: *mut libc::c_int = (*A).i;
    let mut Ap: *mut libc::c_int = (*A).p;
    let mut Ad: *mut libc::c_uint = (*A).data;
    let mut Ci: *mut libc::c_int = (*C).i;
    p = *Ap.offset(j as isize);
    while p < *Ap.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize) {
        let mut i: libc::c_int = *Ai.offset(p as isize);
        if *w.offset(i as isize) < mark {
            *w.offset(i as isize) = mark;
            let fresh86 = nz;
            nz = nz.wrapping_add(1);
            *Ci.offset(fresh86 as isize) = i;
            *x.offset(i as isize) = *Ad.offset(p as isize);
        } else {
            let ref mut fresh87 = *x.offset(i as isize);
            *fresh87 = (*fresh87).wrapping_add(*Ad.offset(p as isize));
        }
        p += 1;
        p;
    }
    return nz;
}
unsafe extern "C" fn spmatrix_char_scatter(
    mut A: *const gsl_spmatrix_char,
    j: size_t,
    mut w: *mut libc::c_int,
    mut x: *mut libc::c_char,
    mark: libc::c_int,
    mut C: *mut gsl_spmatrix_char,
    mut nz: size_t,
) -> size_t {
    let mut p: libc::c_int = 0;
    let mut Ai: *mut libc::c_int = (*A).i;
    let mut Ap: *mut libc::c_int = (*A).p;
    let mut Ad: *mut libc::c_char = (*A).data;
    let mut Ci: *mut libc::c_int = (*C).i;
    p = *Ap.offset(j as isize);
    while p < *Ap.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize) {
        let mut i: libc::c_int = *Ai.offset(p as isize);
        if *w.offset(i as isize) < mark {
            *w.offset(i as isize) = mark;
            let fresh88 = nz;
            nz = nz.wrapping_add(1);
            *Ci.offset(fresh88 as isize) = i;
            *x.offset(i as isize) = *Ad.offset(p as isize);
        } else {
            let ref mut fresh89 = *x.offset(i as isize);
            *fresh89 = (*fresh89 as libc::c_int + *Ad.offset(p as isize) as libc::c_int)
                as libc::c_char;
        }
        p += 1;
        p;
    }
    return nz;
}
unsafe extern "C" fn spmatrix_ulong_scatter(
    mut A: *const gsl_spmatrix_ulong,
    j: size_t,
    mut w: *mut libc::c_int,
    mut x: *mut libc::c_ulong,
    mark: libc::c_int,
    mut C: *mut gsl_spmatrix_ulong,
    mut nz: size_t,
) -> size_t {
    let mut p: libc::c_int = 0;
    let mut Ai: *mut libc::c_int = (*A).i;
    let mut Ap: *mut libc::c_int = (*A).p;
    let mut Ad: *mut libc::c_ulong = (*A).data;
    let mut Ci: *mut libc::c_int = (*C).i;
    p = *Ap.offset(j as isize);
    while p < *Ap.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize) {
        let mut i: libc::c_int = *Ai.offset(p as isize);
        if *w.offset(i as isize) < mark {
            *w.offset(i as isize) = mark;
            let fresh90 = nz;
            nz = nz.wrapping_add(1);
            *Ci.offset(fresh90 as isize) = i;
            *x.offset(i as isize) = *Ad.offset(p as isize);
        } else {
            let ref mut fresh91 = *x.offset(i as isize);
            *fresh91 = (*fresh91).wrapping_add(*Ad.offset(p as isize));
        }
        p += 1;
        p;
    }
    return nz;
}
unsafe extern "C" fn spmatrix_scatter(
    mut A: *const gsl_spmatrix,
    j: size_t,
    mut w: *mut libc::c_int,
    mut x: *mut libc::c_double,
    mark: libc::c_int,
    mut C: *mut gsl_spmatrix,
    mut nz: size_t,
) -> size_t {
    let mut p: libc::c_int = 0;
    let mut Ai: *mut libc::c_int = (*A).i;
    let mut Ap: *mut libc::c_int = (*A).p;
    let mut Ad: *mut libc::c_double = (*A).data;
    let mut Ci: *mut libc::c_int = (*C).i;
    p = *Ap.offset(j as isize);
    while p < *Ap.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize) {
        let mut i: libc::c_int = *Ai.offset(p as isize);
        if *w.offset(i as isize) < mark {
            *w.offset(i as isize) = mark;
            let fresh92 = nz;
            nz = nz.wrapping_add(1);
            *Ci.offset(fresh92 as isize) = i;
            *x.offset(i as isize) = *Ad.offset(p as isize);
        } else {
            *x.offset(i as isize) += *Ad.offset(p as isize);
        }
        p += 1;
        p;
    }
    return nz;
}
unsafe extern "C" fn spmatrix_int_scatter(
    mut A: *const gsl_spmatrix_int,
    j: size_t,
    mut w: *mut libc::c_int,
    mut x: *mut libc::c_int,
    mark: libc::c_int,
    mut C: *mut gsl_spmatrix_int,
    mut nz: size_t,
) -> size_t {
    let mut p: libc::c_int = 0;
    let mut Ai: *mut libc::c_int = (*A).i;
    let mut Ap: *mut libc::c_int = (*A).p;
    let mut Ad: *mut libc::c_int = (*A).data;
    let mut Ci: *mut libc::c_int = (*C).i;
    p = *Ap.offset(j as isize);
    while p < *Ap.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize) {
        let mut i: libc::c_int = *Ai.offset(p as isize);
        if *w.offset(i as isize) < mark {
            *w.offset(i as isize) = mark;
            let fresh93 = nz;
            nz = nz.wrapping_add(1);
            *Ci.offset(fresh93 as isize) = i;
            *x.offset(i as isize) = *Ad.offset(p as isize);
        } else {
            *x.offset(i as isize) += *Ad.offset(p as isize);
        }
        p += 1;
        p;
    }
    return nz;
}
unsafe extern "C" fn spmatrix_uchar_scatter(
    mut A: *const gsl_spmatrix_uchar,
    j: size_t,
    mut w: *mut libc::c_int,
    mut x: *mut libc::c_uchar,
    mark: libc::c_int,
    mut C: *mut gsl_spmatrix_uchar,
    mut nz: size_t,
) -> size_t {
    let mut p: libc::c_int = 0;
    let mut Ai: *mut libc::c_int = (*A).i;
    let mut Ap: *mut libc::c_int = (*A).p;
    let mut Ad: *mut libc::c_uchar = (*A).data;
    let mut Ci: *mut libc::c_int = (*C).i;
    p = *Ap.offset(j as isize);
    while p < *Ap.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize) {
        let mut i: libc::c_int = *Ai.offset(p as isize);
        if *w.offset(i as isize) < mark {
            *w.offset(i as isize) = mark;
            let fresh94 = nz;
            nz = nz.wrapping_add(1);
            *Ci.offset(fresh94 as isize) = i;
            *x.offset(i as isize) = *Ad.offset(p as isize);
        } else {
            let ref mut fresh95 = *x.offset(i as isize);
            *fresh95 = (*fresh95 as libc::c_int + *Ad.offset(p as isize) as libc::c_int)
                as libc::c_uchar;
        }
        p += 1;
        p;
    }
    return nz;
}
unsafe extern "C" fn spmatrix_float_scatter(
    mut A: *const gsl_spmatrix_float,
    j: size_t,
    mut w: *mut libc::c_int,
    mut x: *mut libc::c_float,
    mark: libc::c_int,
    mut C: *mut gsl_spmatrix_float,
    mut nz: size_t,
) -> size_t {
    let mut p: libc::c_int = 0;
    let mut Ai: *mut libc::c_int = (*A).i;
    let mut Ap: *mut libc::c_int = (*A).p;
    let mut Ad: *mut libc::c_float = (*A).data;
    let mut Ci: *mut libc::c_int = (*C).i;
    p = *Ap.offset(j as isize);
    while p < *Ap.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize) {
        let mut i: libc::c_int = *Ai.offset(p as isize);
        if *w.offset(i as isize) < mark {
            *w.offset(i as isize) = mark;
            let fresh96 = nz;
            nz = nz.wrapping_add(1);
            *Ci.offset(fresh96 as isize) = i;
            *x.offset(i as isize) = *Ad.offset(p as isize);
        } else {
            *x.offset(i as isize) += *Ad.offset(p as isize);
        }
        p += 1;
        p;
    }
    return nz;
}
unsafe extern "C" fn spmatrix_long_double_scatter(
    mut A: *const gsl_spmatrix_long_double,
    j: size_t,
    mut w: *mut libc::c_int,
    mut x: *mut f128::f128,
    mark: libc::c_int,
    mut C: *mut gsl_spmatrix_long_double,
    mut nz: size_t,
) -> size_t {
    let mut p: libc::c_int = 0;
    let mut Ai: *mut libc::c_int = (*A).i;
    let mut Ap: *mut libc::c_int = (*A).p;
    let mut Ad: *mut f128::f128 = (*A).data;
    let mut Ci: *mut libc::c_int = (*C).i;
    p = *Ap.offset(j as isize);
    while p < *Ap.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize) {
        let mut i: libc::c_int = *Ai.offset(p as isize);
        if *w.offset(i as isize) < mark {
            *w.offset(i as isize) = mark;
            let fresh97 = nz;
            nz = nz.wrapping_add(1);
            *Ci.offset(fresh97 as isize) = i;
            *x.offset(i as isize) = *Ad.offset(p as isize);
        } else {
            *x.offset(i as isize) += *Ad.offset(p as isize);
        }
        p += 1;
        p;
    }
    return nz;
}
