use ::libc;
use ::f128;
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_spmatrix_char_ptr(
        m: *const gsl_spmatrix_char,
        i: size_t,
        j: size_t,
    ) -> *mut libc::c_char;
    fn gsl_spmatrix_complex_long_double_ptr(
        m: *const gsl_spmatrix_complex_long_double,
        i: size_t,
        j: size_t,
    ) -> *mut gsl_complex_long_double;
    fn gsl_spmatrix_complex_ptr(
        m: *const gsl_spmatrix_complex,
        i: size_t,
        j: size_t,
    ) -> *mut gsl_complex;
    fn gsl_spmatrix_complex_float_ptr(
        m: *const gsl_spmatrix_complex_float,
        i: size_t,
        j: size_t,
    ) -> *mut gsl_complex_float;
    fn gsl_spmatrix_long_double_ptr(
        m: *const gsl_spmatrix_long_double,
        i: size_t,
        j: size_t,
    ) -> *mut f128::f128;
    fn gsl_spmatrix_ptr(
        m: *const gsl_spmatrix,
        i: size_t,
        j: size_t,
    ) -> *mut libc::c_double;
    fn gsl_spmatrix_float_ptr(
        m: *const gsl_spmatrix_float,
        i: size_t,
        j: size_t,
    ) -> *mut libc::c_float;
    fn gsl_spmatrix_ulong_ptr(
        m: *const gsl_spmatrix_ulong,
        i: size_t,
        j: size_t,
    ) -> *mut libc::c_ulong;
    fn gsl_spmatrix_long_ptr(
        m: *const gsl_spmatrix_long,
        i: size_t,
        j: size_t,
    ) -> *mut libc::c_long;
    fn gsl_spmatrix_uint_ptr(
        m: *const gsl_spmatrix_uint,
        i: size_t,
        j: size_t,
    ) -> *mut libc::c_uint;
    fn gsl_spmatrix_int_ptr(
        m: *const gsl_spmatrix_int,
        i: size_t,
        j: size_t,
    ) -> *mut libc::c_int;
    fn gsl_spmatrix_ushort_ptr(
        m: *const gsl_spmatrix_ushort,
        i: size_t,
        j: size_t,
    ) -> *mut libc::c_ushort;
    fn gsl_spmatrix_short_ptr(
        m: *const gsl_spmatrix_short,
        i: size_t,
        j: size_t,
    ) -> *mut libc::c_short;
    fn gsl_spmatrix_uchar_ptr(
        m: *const gsl_spmatrix_uchar,
        i: size_t,
        j: size_t,
    ) -> *mut libc::c_uchar;
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
pub unsafe extern "C" fn gsl_spmatrix_complex_float_equal(
    mut a: *const gsl_spmatrix_complex_float,
    mut b: *const gsl_spmatrix_complex_float,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if (*a).sptype != (*b).sptype {
        gsl_error(
            b"trying to compare different sparse matrix types\0" as *const u8
                as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else {
        let nz: size_t = (*a).nz;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if nz != (*b).nz {
            return 0 as libc::c_int;
        }
        if (*a).sptype == GSL_SPMATRIX_COO as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                let mut bptr: *mut libc::c_float = gsl_spmatrix_complex_float_ptr(
                    b,
                    *((*a).i).offset(n as isize) as size_t,
                    *((*a).p).offset(n as isize) as size_t,
                ) as *mut libc::c_float;
                if bptr.is_null() {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 2 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        ) != *bptr.offset(r as isize)
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*a).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                if *((*a).i).offset(n as isize) != *((*b).i).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 2 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        )
                        != *((*b).data)
                            .offset(
                                (2 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(n)
                                    .wrapping_add(r) as isize,
                            )
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*a).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                if *((*a).p).offset(n as isize) != *((*b).p).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*a).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                if *((*a).i).offset(n as isize) != *((*b).i).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 2 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        )
                        != *((*b).data)
                            .offset(
                                (2 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(n)
                                    .wrapping_add(r) as isize,
                            )
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*a).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                if *((*a).p).offset(n as isize) != *((*b).p).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./prop_source.c\0" as *const u8 as *const libc::c_char,
                119 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int;
        }
        return 1 as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_float_equal(
    mut a: *const gsl_spmatrix_float,
    mut b: *const gsl_spmatrix_float,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if (*a).sptype != (*b).sptype {
        gsl_error(
            b"trying to compare different sparse matrix types\0" as *const u8
                as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else {
        let nz: size_t = (*a).nz;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if nz != (*b).nz {
            return 0 as libc::c_int;
        }
        if (*a).sptype == GSL_SPMATRIX_COO as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                let mut bptr: *mut libc::c_float = gsl_spmatrix_float_ptr(
                    b,
                    *((*a).i).offset(n as isize) as size_t,
                    *((*a).p).offset(n as isize) as size_t,
                );
                if bptr.is_null() {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        ) != *bptr.offset(r as isize)
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*a).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                if *((*a).i).offset(n as isize) != *((*b).i).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        )
                        != *((*b).data)
                            .offset(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(n)
                                    .wrapping_add(r) as isize,
                            )
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*a).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                if *((*a).p).offset(n as isize) != *((*b).p).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*a).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                if *((*a).i).offset(n as isize) != *((*b).i).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        )
                        != *((*b).data)
                            .offset(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(n)
                                    .wrapping_add(r) as isize,
                            )
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*a).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                if *((*a).p).offset(n as isize) != *((*b).p).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./prop_source.c\0" as *const u8 as *const libc::c_char,
                119 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int;
        }
        return 1 as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_equal(
    mut a: *const gsl_spmatrix_complex,
    mut b: *const gsl_spmatrix_complex,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if (*a).sptype != (*b).sptype {
        gsl_error(
            b"trying to compare different sparse matrix types\0" as *const u8
                as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else {
        let nz: size_t = (*a).nz;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if nz != (*b).nz {
            return 0 as libc::c_int;
        }
        if (*a).sptype == GSL_SPMATRIX_COO as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                let mut bptr: *mut libc::c_double = gsl_spmatrix_complex_ptr(
                    b,
                    *((*a).i).offset(n as isize) as size_t,
                    *((*a).p).offset(n as isize) as size_t,
                ) as *mut libc::c_double;
                if bptr.is_null() {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 2 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        ) != *bptr.offset(r as isize)
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*a).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                if *((*a).i).offset(n as isize) != *((*b).i).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 2 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        )
                        != *((*b).data)
                            .offset(
                                (2 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(n)
                                    .wrapping_add(r) as isize,
                            )
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*a).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                if *((*a).p).offset(n as isize) != *((*b).p).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*a).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                if *((*a).i).offset(n as isize) != *((*b).i).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 2 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        )
                        != *((*b).data)
                            .offset(
                                (2 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(n)
                                    .wrapping_add(r) as isize,
                            )
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*a).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                if *((*a).p).offset(n as isize) != *((*b).p).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./prop_source.c\0" as *const u8 as *const libc::c_char,
                119 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int;
        }
        return 1 as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_ulong_equal(
    mut a: *const gsl_spmatrix_ulong,
    mut b: *const gsl_spmatrix_ulong,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if (*a).sptype != (*b).sptype {
        gsl_error(
            b"trying to compare different sparse matrix types\0" as *const u8
                as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else {
        let nz: size_t = (*a).nz;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if nz != (*b).nz {
            return 0 as libc::c_int;
        }
        if (*a).sptype == GSL_SPMATRIX_COO as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                let mut bptr: *mut libc::c_ulong = gsl_spmatrix_ulong_ptr(
                    b,
                    *((*a).i).offset(n as isize) as size_t,
                    *((*a).p).offset(n as isize) as size_t,
                );
                if bptr.is_null() {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        ) != *bptr.offset(r as isize)
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*a).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                if *((*a).i).offset(n as isize) != *((*b).i).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        )
                        != *((*b).data)
                            .offset(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(n)
                                    .wrapping_add(r) as isize,
                            )
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*a).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                if *((*a).p).offset(n as isize) != *((*b).p).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*a).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                if *((*a).i).offset(n as isize) != *((*b).i).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        )
                        != *((*b).data)
                            .offset(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(n)
                                    .wrapping_add(r) as isize,
                            )
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*a).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                if *((*a).p).offset(n as isize) != *((*b).p).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./prop_source.c\0" as *const u8 as *const libc::c_char,
                119 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int;
        }
        return 1 as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_char_equal(
    mut a: *const gsl_spmatrix_char,
    mut b: *const gsl_spmatrix_char,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if (*a).sptype != (*b).sptype {
        gsl_error(
            b"trying to compare different sparse matrix types\0" as *const u8
                as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else {
        let nz: size_t = (*a).nz;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if nz != (*b).nz {
            return 0 as libc::c_int;
        }
        if (*a).sptype == GSL_SPMATRIX_COO as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                let mut bptr: *mut libc::c_char = gsl_spmatrix_char_ptr(
                    b,
                    *((*a).i).offset(n as isize) as size_t,
                    *((*a).p).offset(n as isize) as size_t,
                );
                if bptr.is_null() {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        ) as libc::c_int != *bptr.offset(r as isize) as libc::c_int
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*a).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                if *((*a).i).offset(n as isize) != *((*b).i).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        ) as libc::c_int
                        != *((*b).data)
                            .offset(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(n)
                                    .wrapping_add(r) as isize,
                            ) as libc::c_int
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*a).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                if *((*a).p).offset(n as isize) != *((*b).p).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*a).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                if *((*a).i).offset(n as isize) != *((*b).i).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        ) as libc::c_int
                        != *((*b).data)
                            .offset(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(n)
                                    .wrapping_add(r) as isize,
                            ) as libc::c_int
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*a).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                if *((*a).p).offset(n as isize) != *((*b).p).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./prop_source.c\0" as *const u8 as *const libc::c_char,
                119 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int;
        }
        return 1 as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_long_equal(
    mut a: *const gsl_spmatrix_long,
    mut b: *const gsl_spmatrix_long,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if (*a).sptype != (*b).sptype {
        gsl_error(
            b"trying to compare different sparse matrix types\0" as *const u8
                as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else {
        let nz: size_t = (*a).nz;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if nz != (*b).nz {
            return 0 as libc::c_int;
        }
        if (*a).sptype == GSL_SPMATRIX_COO as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                let mut bptr: *mut libc::c_long = gsl_spmatrix_long_ptr(
                    b,
                    *((*a).i).offset(n as isize) as size_t,
                    *((*a).p).offset(n as isize) as size_t,
                );
                if bptr.is_null() {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        ) != *bptr.offset(r as isize)
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*a).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                if *((*a).i).offset(n as isize) != *((*b).i).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        )
                        != *((*b).data)
                            .offset(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(n)
                                    .wrapping_add(r) as isize,
                            )
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*a).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                if *((*a).p).offset(n as isize) != *((*b).p).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*a).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                if *((*a).i).offset(n as isize) != *((*b).i).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        )
                        != *((*b).data)
                            .offset(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(n)
                                    .wrapping_add(r) as isize,
                            )
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*a).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                if *((*a).p).offset(n as isize) != *((*b).p).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./prop_source.c\0" as *const u8 as *const libc::c_char,
                119 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int;
        }
        return 1 as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_short_equal(
    mut a: *const gsl_spmatrix_short,
    mut b: *const gsl_spmatrix_short,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if (*a).sptype != (*b).sptype {
        gsl_error(
            b"trying to compare different sparse matrix types\0" as *const u8
                as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else {
        let nz: size_t = (*a).nz;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if nz != (*b).nz {
            return 0 as libc::c_int;
        }
        if (*a).sptype == GSL_SPMATRIX_COO as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                let mut bptr: *mut libc::c_short = gsl_spmatrix_short_ptr(
                    b,
                    *((*a).i).offset(n as isize) as size_t,
                    *((*a).p).offset(n as isize) as size_t,
                );
                if bptr.is_null() {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        ) as libc::c_int != *bptr.offset(r as isize) as libc::c_int
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*a).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                if *((*a).i).offset(n as isize) != *((*b).i).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        ) as libc::c_int
                        != *((*b).data)
                            .offset(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(n)
                                    .wrapping_add(r) as isize,
                            ) as libc::c_int
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*a).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                if *((*a).p).offset(n as isize) != *((*b).p).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*a).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                if *((*a).i).offset(n as isize) != *((*b).i).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        ) as libc::c_int
                        != *((*b).data)
                            .offset(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(n)
                                    .wrapping_add(r) as isize,
                            ) as libc::c_int
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*a).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                if *((*a).p).offset(n as isize) != *((*b).p).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./prop_source.c\0" as *const u8 as *const libc::c_char,
                119 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int;
        }
        return 1 as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uint_equal(
    mut a: *const gsl_spmatrix_uint,
    mut b: *const gsl_spmatrix_uint,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if (*a).sptype != (*b).sptype {
        gsl_error(
            b"trying to compare different sparse matrix types\0" as *const u8
                as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else {
        let nz: size_t = (*a).nz;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if nz != (*b).nz {
            return 0 as libc::c_int;
        }
        if (*a).sptype == GSL_SPMATRIX_COO as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                let mut bptr: *mut libc::c_uint = gsl_spmatrix_uint_ptr(
                    b,
                    *((*a).i).offset(n as isize) as size_t,
                    *((*a).p).offset(n as isize) as size_t,
                );
                if bptr.is_null() {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        ) != *bptr.offset(r as isize)
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*a).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                if *((*a).i).offset(n as isize) != *((*b).i).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        )
                        != *((*b).data)
                            .offset(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(n)
                                    .wrapping_add(r) as isize,
                            )
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*a).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                if *((*a).p).offset(n as isize) != *((*b).p).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*a).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                if *((*a).i).offset(n as isize) != *((*b).i).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        )
                        != *((*b).data)
                            .offset(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(n)
                                    .wrapping_add(r) as isize,
                            )
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*a).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                if *((*a).p).offset(n as isize) != *((*b).p).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./prop_source.c\0" as *const u8 as *const libc::c_char,
                119 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int;
        }
        return 1 as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_int_equal(
    mut a: *const gsl_spmatrix_int,
    mut b: *const gsl_spmatrix_int,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if (*a).sptype != (*b).sptype {
        gsl_error(
            b"trying to compare different sparse matrix types\0" as *const u8
                as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else {
        let nz: size_t = (*a).nz;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if nz != (*b).nz {
            return 0 as libc::c_int;
        }
        if (*a).sptype == GSL_SPMATRIX_COO as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                let mut bptr: *mut libc::c_int = gsl_spmatrix_int_ptr(
                    b,
                    *((*a).i).offset(n as isize) as size_t,
                    *((*a).p).offset(n as isize) as size_t,
                );
                if bptr.is_null() {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        ) != *bptr.offset(r as isize)
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*a).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                if *((*a).i).offset(n as isize) != *((*b).i).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        )
                        != *((*b).data)
                            .offset(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(n)
                                    .wrapping_add(r) as isize,
                            )
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*a).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                if *((*a).p).offset(n as isize) != *((*b).p).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*a).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                if *((*a).i).offset(n as isize) != *((*b).i).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        )
                        != *((*b).data)
                            .offset(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(n)
                                    .wrapping_add(r) as isize,
                            )
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*a).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                if *((*a).p).offset(n as isize) != *((*b).p).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./prop_source.c\0" as *const u8 as *const libc::c_char,
                119 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int;
        }
        return 1 as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_long_double_equal(
    mut a: *const gsl_spmatrix_complex_long_double,
    mut b: *const gsl_spmatrix_complex_long_double,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if (*a).sptype != (*b).sptype {
        gsl_error(
            b"trying to compare different sparse matrix types\0" as *const u8
                as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else {
        let nz: size_t = (*a).nz;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if nz != (*b).nz {
            return 0 as libc::c_int;
        }
        if (*a).sptype == GSL_SPMATRIX_COO as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                let mut bptr: *mut f128::f128 = gsl_spmatrix_complex_long_double_ptr(
                    b,
                    *((*a).i).offset(n as isize) as size_t,
                    *((*a).p).offset(n as isize) as size_t,
                ) as *mut f128::f128;
                if bptr.is_null() {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 2 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        ) != *bptr.offset(r as isize)
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*a).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                if *((*a).i).offset(n as isize) != *((*b).i).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 2 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        )
                        != *((*b).data)
                            .offset(
                                (2 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(n)
                                    .wrapping_add(r) as isize,
                            )
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*a).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                if *((*a).p).offset(n as isize) != *((*b).p).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*a).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                if *((*a).i).offset(n as isize) != *((*b).i).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 2 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        )
                        != *((*b).data)
                            .offset(
                                (2 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(n)
                                    .wrapping_add(r) as isize,
                            )
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*a).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                if *((*a).p).offset(n as isize) != *((*b).p).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./prop_source.c\0" as *const u8 as *const libc::c_char,
                119 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int;
        }
        return 1 as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_ushort_equal(
    mut a: *const gsl_spmatrix_ushort,
    mut b: *const gsl_spmatrix_ushort,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if (*a).sptype != (*b).sptype {
        gsl_error(
            b"trying to compare different sparse matrix types\0" as *const u8
                as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else {
        let nz: size_t = (*a).nz;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if nz != (*b).nz {
            return 0 as libc::c_int;
        }
        if (*a).sptype == GSL_SPMATRIX_COO as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                let mut bptr: *mut libc::c_ushort = gsl_spmatrix_ushort_ptr(
                    b,
                    *((*a).i).offset(n as isize) as size_t,
                    *((*a).p).offset(n as isize) as size_t,
                );
                if bptr.is_null() {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        ) as libc::c_int != *bptr.offset(r as isize) as libc::c_int
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*a).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                if *((*a).i).offset(n as isize) != *((*b).i).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        ) as libc::c_int
                        != *((*b).data)
                            .offset(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(n)
                                    .wrapping_add(r) as isize,
                            ) as libc::c_int
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*a).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                if *((*a).p).offset(n as isize) != *((*b).p).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*a).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                if *((*a).i).offset(n as isize) != *((*b).i).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        ) as libc::c_int
                        != *((*b).data)
                            .offset(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(n)
                                    .wrapping_add(r) as isize,
                            ) as libc::c_int
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*a).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                if *((*a).p).offset(n as isize) != *((*b).p).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./prop_source.c\0" as *const u8 as *const libc::c_char,
                119 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int;
        }
        return 1 as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_equal(
    mut a: *const gsl_spmatrix_long_double,
    mut b: *const gsl_spmatrix_long_double,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if (*a).sptype != (*b).sptype {
        gsl_error(
            b"trying to compare different sparse matrix types\0" as *const u8
                as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else {
        let nz: size_t = (*a).nz;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if nz != (*b).nz {
            return 0 as libc::c_int;
        }
        if (*a).sptype == GSL_SPMATRIX_COO as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                let mut bptr: *mut f128::f128 = gsl_spmatrix_long_double_ptr(
                    b,
                    *((*a).i).offset(n as isize) as size_t,
                    *((*a).p).offset(n as isize) as size_t,
                );
                if bptr.is_null() {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        ) != *bptr.offset(r as isize)
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*a).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                if *((*a).i).offset(n as isize) != *((*b).i).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        )
                        != *((*b).data)
                            .offset(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(n)
                                    .wrapping_add(r) as isize,
                            )
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*a).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                if *((*a).p).offset(n as isize) != *((*b).p).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*a).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                if *((*a).i).offset(n as isize) != *((*b).i).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        )
                        != *((*b).data)
                            .offset(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(n)
                                    .wrapping_add(r) as isize,
                            )
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*a).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                if *((*a).p).offset(n as isize) != *((*b).p).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./prop_source.c\0" as *const u8 as *const libc::c_char,
                119 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int;
        }
        return 1 as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_equal(
    mut a: *const gsl_spmatrix,
    mut b: *const gsl_spmatrix,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if (*a).sptype != (*b).sptype {
        gsl_error(
            b"trying to compare different sparse matrix types\0" as *const u8
                as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else {
        let nz: size_t = (*a).nz;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if nz != (*b).nz {
            return 0 as libc::c_int;
        }
        if (*a).sptype == GSL_SPMATRIX_COO as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                let mut bptr: *mut libc::c_double = gsl_spmatrix_ptr(
                    b,
                    *((*a).i).offset(n as isize) as size_t,
                    *((*a).p).offset(n as isize) as size_t,
                );
                if bptr.is_null() {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        ) != *bptr.offset(r as isize)
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*a).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                if *((*a).i).offset(n as isize) != *((*b).i).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        )
                        != *((*b).data)
                            .offset(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(n)
                                    .wrapping_add(r) as isize,
                            )
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*a).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                if *((*a).p).offset(n as isize) != *((*b).p).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*a).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                if *((*a).i).offset(n as isize) != *((*b).i).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        )
                        != *((*b).data)
                            .offset(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(n)
                                    .wrapping_add(r) as isize,
                            )
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*a).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                if *((*a).p).offset(n as isize) != *((*b).p).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./prop_source.c\0" as *const u8 as *const libc::c_char,
                119 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int;
        }
        return 1 as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uchar_equal(
    mut a: *const gsl_spmatrix_uchar,
    mut b: *const gsl_spmatrix_uchar,
) -> libc::c_int {
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    if (*b).size1 != M || (*b).size2 != N {
        gsl_error(
            b"matrices must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return 0 as libc::c_int;
    } else if (*a).sptype != (*b).sptype {
        gsl_error(
            b"trying to compare different sparse matrix types\0" as *const u8
                as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int;
    } else {
        let nz: size_t = (*a).nz;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if nz != (*b).nz {
            return 0 as libc::c_int;
        }
        if (*a).sptype == GSL_SPMATRIX_COO as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                let mut bptr: *mut libc::c_uchar = gsl_spmatrix_uchar_ptr(
                    b,
                    *((*a).i).offset(n as isize) as size_t,
                    *((*a).p).offset(n as isize) as size_t,
                );
                if bptr.is_null() {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        ) as libc::c_int != *bptr.offset(r as isize) as libc::c_int
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*a).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                if *((*a).i).offset(n as isize) != *((*b).i).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        ) as libc::c_int
                        != *((*b).data)
                            .offset(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(n)
                                    .wrapping_add(r) as isize,
                            ) as libc::c_int
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*a).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                if *((*a).p).offset(n as isize) != *((*b).p).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*a).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < nz {
                if *((*a).i).offset(n as isize) != *((*b).i).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                r = 0 as libc::c_int as size_t;
                while r < 1 as libc::c_int as libc::c_ulong {
                    if *((*a).data)
                        .offset(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(n)
                                .wrapping_add(r) as isize,
                        ) as libc::c_int
                        != *((*b).data)
                            .offset(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(n)
                                    .wrapping_add(r) as isize,
                            ) as libc::c_int
                    {
                        return 0 as libc::c_int;
                    }
                    r = r.wrapping_add(1);
                    r;
                }
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*a).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                if *((*a).p).offset(n as isize) != *((*b).p).offset(n as isize) {
                    return 0 as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else {
            gsl_error(
                b"unknown sparse matrix type\0" as *const u8 as *const libc::c_char,
                b"./prop_source.c\0" as *const u8 as *const libc::c_char,
                119 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return 0 as libc::c_int;
        }
        return 1 as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_float_norm1(
    mut A: *const gsl_spmatrix_float,
) -> libc::c_float {
    let N: size_t = (*A).size2;
    let mut value: libc::c_float = 0 as libc::c_int as libc::c_float;
    if (*A).nz == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as libc::c_float
    } else if (*A).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        let mut Ap: *mut libc::c_int = (*A).p;
        let mut Ad: *mut libc::c_float = (*A).data;
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < N {
            let mut sum: libc::c_float = 0 as libc::c_int as libc::c_float;
            let mut p: libc::c_int = 0;
            p = *Ap.offset(j as isize);
            while p
                < *Ap.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                sum
                    += if *Ad.offset(p as isize) >= 0 as libc::c_int as libc::c_float {
                        *Ad.offset(p as isize)
                    } else {
                        -*Ad.offset(p as isize)
                    };
                p += 1;
                p;
            }
            if sum > value {
                value = sum;
            }
            j = j.wrapping_add(1);
            j;
        }
    } else {
        let mut colsum: *mut libc::c_float = (*A).work.work_atomic;
        let mut Ad_0: *mut libc::c_float = (*A).data;
        let mut j_0: size_t = 0;
        j_0 = 0 as libc::c_int as size_t;
        while j_0 < N {
            *colsum.offset(j_0 as isize) = 0 as libc::c_int as libc::c_float;
            j_0 = j_0.wrapping_add(1);
            j_0;
        }
        if (*A).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut Ap_0: *mut libc::c_int = (*A).p;
            j_0 = 0 as libc::c_int as size_t;
            while j_0 < (*A).nz {
                *colsum.offset(*Ap_0.offset(j_0 as isize) as isize)
                    += if *Ad_0.offset(j_0 as isize) >= 0 as libc::c_int as libc::c_float
                    {
                        *Ad_0.offset(j_0 as isize)
                    } else {
                        -*Ad_0.offset(j_0 as isize)
                    };
                j_0 = j_0.wrapping_add(1);
                j_0;
            }
        } else if (*A).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Aj: *mut libc::c_int = (*A).i;
            j_0 = 0 as libc::c_int as size_t;
            while j_0 < (*A).nz {
                *colsum.offset(*Aj.offset(j_0 as isize) as isize)
                    += if *Ad_0.offset(j_0 as isize) >= 0 as libc::c_int as libc::c_float
                    {
                        *Ad_0.offset(j_0 as isize)
                    } else {
                        -*Ad_0.offset(j_0 as isize)
                    };
                j_0 = j_0.wrapping_add(1);
                j_0;
            }
        }
        j_0 = 0 as libc::c_int as size_t;
        while j_0 < N {
            if *colsum.offset(j_0 as isize) > value {
                value = *colsum.offset(j_0 as isize);
            }
            j_0 = j_0.wrapping_add(1);
            j_0;
        }
    }
    return value;
}
pub unsafe extern "C" fn gsl_spmatrix_long_norm1(
    mut A: *const gsl_spmatrix_long,
) -> libc::c_long {
    let N: size_t = (*A).size2;
    let mut value: libc::c_long = 0 as libc::c_int as libc::c_long;
    if (*A).nz == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as libc::c_long
    } else if (*A).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        let mut Ap: *mut libc::c_int = (*A).p;
        let mut Ad: *mut libc::c_long = (*A).data;
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < N {
            let mut sum: libc::c_long = 0 as libc::c_int as libc::c_long;
            let mut p: libc::c_int = 0;
            p = *Ap.offset(j as isize);
            while p
                < *Ap.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                sum
                    += if *Ad.offset(p as isize) >= 0 as libc::c_int as libc::c_long {
                        *Ad.offset(p as isize)
                    } else {
                        -*Ad.offset(p as isize)
                    };
                p += 1;
                p;
            }
            if sum > value {
                value = sum;
            }
            j = j.wrapping_add(1);
            j;
        }
    } else {
        let mut colsum: *mut libc::c_long = (*A).work.work_atomic;
        let mut Ad_0: *mut libc::c_long = (*A).data;
        let mut j_0: size_t = 0;
        j_0 = 0 as libc::c_int as size_t;
        while j_0 < N {
            *colsum.offset(j_0 as isize) = 0 as libc::c_int as libc::c_long;
            j_0 = j_0.wrapping_add(1);
            j_0;
        }
        if (*A).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut Ap_0: *mut libc::c_int = (*A).p;
            j_0 = 0 as libc::c_int as size_t;
            while j_0 < (*A).nz {
                *colsum.offset(*Ap_0.offset(j_0 as isize) as isize)
                    += if *Ad_0.offset(j_0 as isize) >= 0 as libc::c_int as libc::c_long
                    {
                        *Ad_0.offset(j_0 as isize)
                    } else {
                        -*Ad_0.offset(j_0 as isize)
                    };
                j_0 = j_0.wrapping_add(1);
                j_0;
            }
        } else if (*A).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Aj: *mut libc::c_int = (*A).i;
            j_0 = 0 as libc::c_int as size_t;
            while j_0 < (*A).nz {
                *colsum.offset(*Aj.offset(j_0 as isize) as isize)
                    += if *Ad_0.offset(j_0 as isize) >= 0 as libc::c_int as libc::c_long
                    {
                        *Ad_0.offset(j_0 as isize)
                    } else {
                        -*Ad_0.offset(j_0 as isize)
                    };
                j_0 = j_0.wrapping_add(1);
                j_0;
            }
        }
        j_0 = 0 as libc::c_int as size_t;
        while j_0 < N {
            if *colsum.offset(j_0 as isize) > value {
                value = *colsum.offset(j_0 as isize);
            }
            j_0 = j_0.wrapping_add(1);
            j_0;
        }
    }
    return value;
}
pub unsafe extern "C" fn gsl_spmatrix_char_norm1(
    mut A: *const gsl_spmatrix_char,
) -> libc::c_char {
    let N: size_t = (*A).size2;
    let mut value: libc::c_char = 0 as libc::c_int as libc::c_char;
    if (*A).nz == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as libc::c_char
    } else if (*A).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        let mut Ap: *mut libc::c_int = (*A).p;
        let mut Ad: *mut libc::c_char = (*A).data;
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < N {
            let mut sum: libc::c_char = 0 as libc::c_int as libc::c_char;
            let mut p: libc::c_int = 0;
            p = *Ap.offset(j as isize);
            while p
                < *Ap.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                sum = (sum as libc::c_int
                    + if *Ad.offset(p as isize) as libc::c_int
                        >= 0 as libc::c_int as libc::c_char as libc::c_int
                    {
                        *Ad.offset(p as isize) as libc::c_int
                    } else {
                        -(*Ad.offset(p as isize) as libc::c_int)
                    }) as libc::c_char;
                p += 1;
                p;
            }
            if sum as libc::c_int > value as libc::c_int {
                value = sum;
            }
            j = j.wrapping_add(1);
            j;
        }
    } else {
        let mut colsum: *mut libc::c_char = (*A).work.work_atomic;
        let mut Ad_0: *mut libc::c_char = (*A).data;
        let mut j_0: size_t = 0;
        j_0 = 0 as libc::c_int as size_t;
        while j_0 < N {
            *colsum.offset(j_0 as isize) = 0 as libc::c_int as libc::c_char;
            j_0 = j_0.wrapping_add(1);
            j_0;
        }
        if (*A).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut Ap_0: *mut libc::c_int = (*A).p;
            j_0 = 0 as libc::c_int as size_t;
            while j_0 < (*A).nz {
                let ref mut fresh0 = *colsum.offset(*Ap_0.offset(j_0 as isize) as isize);
                *fresh0 = (*fresh0 as libc::c_int
                    + if *Ad_0.offset(j_0 as isize) as libc::c_int
                        >= 0 as libc::c_int as libc::c_char as libc::c_int
                    {
                        *Ad_0.offset(j_0 as isize) as libc::c_int
                    } else {
                        -(*Ad_0.offset(j_0 as isize) as libc::c_int)
                    }) as libc::c_char;
                j_0 = j_0.wrapping_add(1);
                j_0;
            }
        } else if (*A).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Aj: *mut libc::c_int = (*A).i;
            j_0 = 0 as libc::c_int as size_t;
            while j_0 < (*A).nz {
                let ref mut fresh1 = *colsum.offset(*Aj.offset(j_0 as isize) as isize);
                *fresh1 = (*fresh1 as libc::c_int
                    + if *Ad_0.offset(j_0 as isize) as libc::c_int
                        >= 0 as libc::c_int as libc::c_char as libc::c_int
                    {
                        *Ad_0.offset(j_0 as isize) as libc::c_int
                    } else {
                        -(*Ad_0.offset(j_0 as isize) as libc::c_int)
                    }) as libc::c_char;
                j_0 = j_0.wrapping_add(1);
                j_0;
            }
        }
        j_0 = 0 as libc::c_int as size_t;
        while j_0 < N {
            if *colsum.offset(j_0 as isize) as libc::c_int > value as libc::c_int {
                value = *colsum.offset(j_0 as isize);
            }
            j_0 = j_0.wrapping_add(1);
            j_0;
        }
    }
    return value;
}
pub unsafe extern "C" fn gsl_spmatrix_int_norm1(
    mut A: *const gsl_spmatrix_int,
) -> libc::c_int {
    let N: size_t = (*A).size2;
    let mut value: libc::c_int = 0 as libc::c_int;
    if (*A).nz == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int
    } else if (*A).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        let mut Ap: *mut libc::c_int = (*A).p;
        let mut Ad: *mut libc::c_int = (*A).data;
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < N {
            let mut sum: libc::c_int = 0 as libc::c_int;
            let mut p: libc::c_int = 0;
            p = *Ap.offset(j as isize);
            while p
                < *Ap.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                sum
                    += if *Ad.offset(p as isize) >= 0 as libc::c_int {
                        *Ad.offset(p as isize)
                    } else {
                        -*Ad.offset(p as isize)
                    };
                p += 1;
                p;
            }
            if sum > value {
                value = sum;
            }
            j = j.wrapping_add(1);
            j;
        }
    } else {
        let mut colsum: *mut libc::c_int = (*A).work.work_atomic;
        let mut Ad_0: *mut libc::c_int = (*A).data;
        let mut j_0: size_t = 0;
        j_0 = 0 as libc::c_int as size_t;
        while j_0 < N {
            *colsum.offset(j_0 as isize) = 0 as libc::c_int;
            j_0 = j_0.wrapping_add(1);
            j_0;
        }
        if (*A).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut Ap_0: *mut libc::c_int = (*A).p;
            j_0 = 0 as libc::c_int as size_t;
            while j_0 < (*A).nz {
                *colsum.offset(*Ap_0.offset(j_0 as isize) as isize)
                    += if *Ad_0.offset(j_0 as isize) >= 0 as libc::c_int {
                        *Ad_0.offset(j_0 as isize)
                    } else {
                        -*Ad_0.offset(j_0 as isize)
                    };
                j_0 = j_0.wrapping_add(1);
                j_0;
            }
        } else if (*A).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Aj: *mut libc::c_int = (*A).i;
            j_0 = 0 as libc::c_int as size_t;
            while j_0 < (*A).nz {
                *colsum.offset(*Aj.offset(j_0 as isize) as isize)
                    += if *Ad_0.offset(j_0 as isize) >= 0 as libc::c_int {
                        *Ad_0.offset(j_0 as isize)
                    } else {
                        -*Ad_0.offset(j_0 as isize)
                    };
                j_0 = j_0.wrapping_add(1);
                j_0;
            }
        }
        j_0 = 0 as libc::c_int as size_t;
        while j_0 < N {
            if *colsum.offset(j_0 as isize) > value {
                value = *colsum.offset(j_0 as isize);
            }
            j_0 = j_0.wrapping_add(1);
            j_0;
        }
    }
    return value;
}
pub unsafe extern "C" fn gsl_spmatrix_norm1(
    mut A: *const gsl_spmatrix,
) -> libc::c_double {
    let N: size_t = (*A).size2;
    let mut value: libc::c_double = 0 as libc::c_int as libc::c_double;
    if (*A).nz == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as libc::c_double
    } else if (*A).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        let mut Ap: *mut libc::c_int = (*A).p;
        let mut Ad: *mut libc::c_double = (*A).data;
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < N {
            let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
            let mut p: libc::c_int = 0;
            p = *Ap.offset(j as isize);
            while p
                < *Ap.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                sum
                    += if *Ad.offset(p as isize) >= 0 as libc::c_int as libc::c_double {
                        *Ad.offset(p as isize)
                    } else {
                        -*Ad.offset(p as isize)
                    };
                p += 1;
                p;
            }
            if sum > value {
                value = sum;
            }
            j = j.wrapping_add(1);
            j;
        }
    } else {
        let mut colsum: *mut libc::c_double = (*A).work.work_atomic;
        let mut Ad_0: *mut libc::c_double = (*A).data;
        let mut j_0: size_t = 0;
        j_0 = 0 as libc::c_int as size_t;
        while j_0 < N {
            *colsum.offset(j_0 as isize) = 0 as libc::c_int as libc::c_double;
            j_0 = j_0.wrapping_add(1);
            j_0;
        }
        if (*A).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut Ap_0: *mut libc::c_int = (*A).p;
            j_0 = 0 as libc::c_int as size_t;
            while j_0 < (*A).nz {
                *colsum.offset(*Ap_0.offset(j_0 as isize) as isize)
                    += if *Ad_0.offset(j_0 as isize)
                        >= 0 as libc::c_int as libc::c_double
                    {
                        *Ad_0.offset(j_0 as isize)
                    } else {
                        -*Ad_0.offset(j_0 as isize)
                    };
                j_0 = j_0.wrapping_add(1);
                j_0;
            }
        } else if (*A).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Aj: *mut libc::c_int = (*A).i;
            j_0 = 0 as libc::c_int as size_t;
            while j_0 < (*A).nz {
                *colsum.offset(*Aj.offset(j_0 as isize) as isize)
                    += if *Ad_0.offset(j_0 as isize)
                        >= 0 as libc::c_int as libc::c_double
                    {
                        *Ad_0.offset(j_0 as isize)
                    } else {
                        -*Ad_0.offset(j_0 as isize)
                    };
                j_0 = j_0.wrapping_add(1);
                j_0;
            }
        }
        j_0 = 0 as libc::c_int as size_t;
        while j_0 < N {
            if *colsum.offset(j_0 as isize) > value {
                value = *colsum.offset(j_0 as isize);
            }
            j_0 = j_0.wrapping_add(1);
            j_0;
        }
    }
    return value;
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_norm1(
    mut A: *const gsl_spmatrix_long_double,
) -> f128::f128 {
    let N: size_t = (*A).size2;
    let mut value: f128::f128 = f128::f128::new(0 as libc::c_int);
    if (*A).nz == 0 as libc::c_int as libc::c_ulong {
        return f128::f128::new(0 as libc::c_int)
    } else if (*A).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        let mut Ap: *mut libc::c_int = (*A).p;
        let mut Ad: *mut f128::f128 = (*A).data;
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < N {
            let mut sum: f128::f128 = f128::f128::new(0 as libc::c_int);
            let mut p: libc::c_int = 0;
            p = *Ap.offset(j as isize);
            while p
                < *Ap.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                sum
                    += if *Ad.offset(p as isize) >= f128::f128::new(0 as libc::c_int) {
                        *Ad.offset(p as isize)
                    } else {
                        -*Ad.offset(p as isize)
                    };
                p += 1;
                p;
            }
            if sum > value {
                value = sum;
            }
            j = j.wrapping_add(1);
            j;
        }
    } else {
        let mut colsum: *mut f128::f128 = (*A).work.work_atomic;
        let mut Ad_0: *mut f128::f128 = (*A).data;
        let mut j_0: size_t = 0;
        j_0 = 0 as libc::c_int as size_t;
        while j_0 < N {
            *colsum.offset(j_0 as isize) = f128::f128::new(0 as libc::c_int);
            j_0 = j_0.wrapping_add(1);
            j_0;
        }
        if (*A).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut Ap_0: *mut libc::c_int = (*A).p;
            j_0 = 0 as libc::c_int as size_t;
            while j_0 < (*A).nz {
                *colsum.offset(*Ap_0.offset(j_0 as isize) as isize)
                    += if *Ad_0.offset(j_0 as isize) >= f128::f128::new(0 as libc::c_int)
                    {
                        *Ad_0.offset(j_0 as isize)
                    } else {
                        -*Ad_0.offset(j_0 as isize)
                    };
                j_0 = j_0.wrapping_add(1);
                j_0;
            }
        } else if (*A).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Aj: *mut libc::c_int = (*A).i;
            j_0 = 0 as libc::c_int as size_t;
            while j_0 < (*A).nz {
                *colsum.offset(*Aj.offset(j_0 as isize) as isize)
                    += if *Ad_0.offset(j_0 as isize) >= f128::f128::new(0 as libc::c_int)
                    {
                        *Ad_0.offset(j_0 as isize)
                    } else {
                        -*Ad_0.offset(j_0 as isize)
                    };
                j_0 = j_0.wrapping_add(1);
                j_0;
            }
        }
        j_0 = 0 as libc::c_int as size_t;
        while j_0 < N {
            if *colsum.offset(j_0 as isize) > value {
                value = *colsum.offset(j_0 as isize);
            }
            j_0 = j_0.wrapping_add(1);
            j_0;
        }
    }
    return value;
}
pub unsafe extern "C" fn gsl_spmatrix_short_norm1(
    mut A: *const gsl_spmatrix_short,
) -> libc::c_short {
    let N: size_t = (*A).size2;
    let mut value: libc::c_short = 0 as libc::c_int as libc::c_short;
    if (*A).nz == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as libc::c_short
    } else if (*A).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        let mut Ap: *mut libc::c_int = (*A).p;
        let mut Ad: *mut libc::c_short = (*A).data;
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < N {
            let mut sum: libc::c_short = 0 as libc::c_int as libc::c_short;
            let mut p: libc::c_int = 0;
            p = *Ap.offset(j as isize);
            while p
                < *Ap.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                sum = (sum as libc::c_int
                    + if *Ad.offset(p as isize) as libc::c_int
                        >= 0 as libc::c_int as libc::c_short as libc::c_int
                    {
                        *Ad.offset(p as isize) as libc::c_int
                    } else {
                        -(*Ad.offset(p as isize) as libc::c_int)
                    }) as libc::c_short;
                p += 1;
                p;
            }
            if sum as libc::c_int > value as libc::c_int {
                value = sum;
            }
            j = j.wrapping_add(1);
            j;
        }
    } else {
        let mut colsum: *mut libc::c_short = (*A).work.work_atomic;
        let mut Ad_0: *mut libc::c_short = (*A).data;
        let mut j_0: size_t = 0;
        j_0 = 0 as libc::c_int as size_t;
        while j_0 < N {
            *colsum.offset(j_0 as isize) = 0 as libc::c_int as libc::c_short;
            j_0 = j_0.wrapping_add(1);
            j_0;
        }
        if (*A).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut Ap_0: *mut libc::c_int = (*A).p;
            j_0 = 0 as libc::c_int as size_t;
            while j_0 < (*A).nz {
                let ref mut fresh2 = *colsum.offset(*Ap_0.offset(j_0 as isize) as isize);
                *fresh2 = (*fresh2 as libc::c_int
                    + if *Ad_0.offset(j_0 as isize) as libc::c_int
                        >= 0 as libc::c_int as libc::c_short as libc::c_int
                    {
                        *Ad_0.offset(j_0 as isize) as libc::c_int
                    } else {
                        -(*Ad_0.offset(j_0 as isize) as libc::c_int)
                    }) as libc::c_short;
                j_0 = j_0.wrapping_add(1);
                j_0;
            }
        } else if (*A).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            let mut Aj: *mut libc::c_int = (*A).i;
            j_0 = 0 as libc::c_int as size_t;
            while j_0 < (*A).nz {
                let ref mut fresh3 = *colsum.offset(*Aj.offset(j_0 as isize) as isize);
                *fresh3 = (*fresh3 as libc::c_int
                    + if *Ad_0.offset(j_0 as isize) as libc::c_int
                        >= 0 as libc::c_int as libc::c_short as libc::c_int
                    {
                        *Ad_0.offset(j_0 as isize) as libc::c_int
                    } else {
                        -(*Ad_0.offset(j_0 as isize) as libc::c_int)
                    }) as libc::c_short;
                j_0 = j_0.wrapping_add(1);
                j_0;
            }
        }
        j_0 = 0 as libc::c_int as size_t;
        while j_0 < N {
            if *colsum.offset(j_0 as isize) as libc::c_int > value as libc::c_int {
                value = *colsum.offset(j_0 as isize);
            }
            j_0 = j_0.wrapping_add(1);
            j_0;
        }
    }
    return value;
}
