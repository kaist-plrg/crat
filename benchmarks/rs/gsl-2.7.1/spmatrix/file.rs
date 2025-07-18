use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn gsl_spmatrix_complex_float_set(
        m: *mut gsl_spmatrix_complex_float,
        i: size_t,
        j: size_t,
        x: gsl_complex_float,
    ) -> libc::c_int;
    fn gsl_spmatrix_char_set(
        m: *mut gsl_spmatrix_char,
        i: size_t,
        j: size_t,
        x: libc::c_char,
    ) -> libc::c_int;
    fn gsl_spmatrix_char_tree_rebuild(m: *mut gsl_spmatrix_char) -> libc::c_int;
    fn gsl_spmatrix_char_alloc_nzmax(
        n1: size_t,
        n2: size_t,
        nzmax: size_t,
        sptype: libc::c_int,
    ) -> *mut gsl_spmatrix_char;
    fn gsl_spmatrix_uchar_set(
        m: *mut gsl_spmatrix_uchar,
        i: size_t,
        j: size_t,
        x: libc::c_uchar,
    ) -> libc::c_int;
    fn gsl_spmatrix_uchar_tree_rebuild(m: *mut gsl_spmatrix_uchar) -> libc::c_int;
    fn gsl_spmatrix_uchar_alloc_nzmax(
        n1: size_t,
        n2: size_t,
        nzmax: size_t,
        sptype: libc::c_int,
    ) -> *mut gsl_spmatrix_uchar;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
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
    fn gsl_spmatrix_complex_long_double_tree_rebuild(
        m: *mut gsl_spmatrix_complex_long_double,
    ) -> libc::c_int;
    fn gsl_spmatrix_complex_long_double_set(
        m: *mut gsl_spmatrix_complex_long_double,
        i: size_t,
        j: size_t,
        x: gsl_complex_long_double,
    ) -> libc::c_int;
    fn gsl_spmatrix_complex_alloc_nzmax(
        n1: size_t,
        n2: size_t,
        nzmax: size_t,
        sptype: libc::c_int,
    ) -> *mut gsl_spmatrix_complex;
    fn gsl_spmatrix_complex_tree_rebuild(m: *mut gsl_spmatrix_complex) -> libc::c_int;
    fn gsl_spmatrix_complex_set(
        m: *mut gsl_spmatrix_complex,
        i: size_t,
        j: size_t,
        x: gsl_complex,
    ) -> libc::c_int;
    fn gsl_spmatrix_complex_float_alloc_nzmax(
        n1: size_t,
        n2: size_t,
        nzmax: size_t,
        sptype: libc::c_int,
    ) -> *mut gsl_spmatrix_complex_float;
    fn gsl_spmatrix_complex_float_tree_rebuild(
        m: *mut gsl_spmatrix_complex_float,
    ) -> libc::c_int;
    fn gsl_spmatrix_long_double_alloc_nzmax(
        n1: size_t,
        n2: size_t,
        nzmax: size_t,
        sptype: libc::c_int,
    ) -> *mut gsl_spmatrix_long_double;
    fn gsl_spmatrix_long_double_tree_rebuild(
        m: *mut gsl_spmatrix_long_double,
    ) -> libc::c_int;
    fn gsl_spmatrix_long_double_set(
        m: *mut gsl_spmatrix_long_double,
        i: size_t,
        j: size_t,
        x: f128::f128,
    ) -> libc::c_int;
    fn gsl_spmatrix_alloc_nzmax(
        n1: size_t,
        n2: size_t,
        nzmax: size_t,
        sptype: libc::c_int,
    ) -> *mut gsl_spmatrix;
    fn gsl_spmatrix_tree_rebuild(m: *mut gsl_spmatrix) -> libc::c_int;
    fn gsl_spmatrix_set(
        m: *mut gsl_spmatrix,
        i: size_t,
        j: size_t,
        x: libc::c_double,
    ) -> libc::c_int;
    fn gsl_spmatrix_float_alloc_nzmax(
        n1: size_t,
        n2: size_t,
        nzmax: size_t,
        sptype: libc::c_int,
    ) -> *mut gsl_spmatrix_float;
    fn gsl_spmatrix_float_tree_rebuild(m: *mut gsl_spmatrix_float) -> libc::c_int;
    fn gsl_spmatrix_float_set(
        m: *mut gsl_spmatrix_float,
        i: size_t,
        j: size_t,
        x: libc::c_float,
    ) -> libc::c_int;
    fn gsl_spmatrix_ulong_alloc_nzmax(
        n1: size_t,
        n2: size_t,
        nzmax: size_t,
        sptype: libc::c_int,
    ) -> *mut gsl_spmatrix_ulong;
    fn gsl_spmatrix_ulong_tree_rebuild(m: *mut gsl_spmatrix_ulong) -> libc::c_int;
    fn gsl_spmatrix_ulong_set(
        m: *mut gsl_spmatrix_ulong,
        i: size_t,
        j: size_t,
        x: libc::c_ulong,
    ) -> libc::c_int;
    fn gsl_spmatrix_long_alloc_nzmax(
        n1: size_t,
        n2: size_t,
        nzmax: size_t,
        sptype: libc::c_int,
    ) -> *mut gsl_spmatrix_long;
    fn gsl_spmatrix_long_tree_rebuild(m: *mut gsl_spmatrix_long) -> libc::c_int;
    fn gsl_spmatrix_long_set(
        m: *mut gsl_spmatrix_long,
        i: size_t,
        j: size_t,
        x: libc::c_long,
    ) -> libc::c_int;
    fn gsl_spmatrix_uint_alloc_nzmax(
        n1: size_t,
        n2: size_t,
        nzmax: size_t,
        sptype: libc::c_int,
    ) -> *mut gsl_spmatrix_uint;
    fn gsl_spmatrix_uint_tree_rebuild(m: *mut gsl_spmatrix_uint) -> libc::c_int;
    fn gsl_spmatrix_uint_set(
        m: *mut gsl_spmatrix_uint,
        i: size_t,
        j: size_t,
        x: libc::c_uint,
    ) -> libc::c_int;
    fn gsl_spmatrix_int_alloc_nzmax(
        n1: size_t,
        n2: size_t,
        nzmax: size_t,
        sptype: libc::c_int,
    ) -> *mut gsl_spmatrix_int;
    fn gsl_spmatrix_int_tree_rebuild(m: *mut gsl_spmatrix_int) -> libc::c_int;
    fn gsl_spmatrix_int_set(
        m: *mut gsl_spmatrix_int,
        i: size_t,
        j: size_t,
        x: libc::c_int,
    ) -> libc::c_int;
    fn gsl_spmatrix_short_set(
        m: *mut gsl_spmatrix_short,
        i: size_t,
        j: size_t,
        x: libc::c_short,
    ) -> libc::c_int;
    fn gsl_spmatrix_ushort_alloc_nzmax(
        n1: size_t,
        n2: size_t,
        nzmax: size_t,
        sptype: libc::c_int,
    ) -> *mut gsl_spmatrix_ushort;
    fn gsl_spmatrix_ushort_tree_rebuild(m: *mut gsl_spmatrix_ushort) -> libc::c_int;
    fn gsl_spmatrix_ushort_set(
        m: *mut gsl_spmatrix_ushort,
        i: size_t,
        j: size_t,
        x: libc::c_ushort,
    ) -> libc::c_int;
    fn gsl_spmatrix_short_alloc_nzmax(
        n1: size_t,
        n2: size_t,
        nzmax: size_t,
        sptype: libc::c_int,
    ) -> *mut gsl_spmatrix_short;
    fn gsl_spmatrix_short_tree_rebuild(m: *mut gsl_spmatrix_short) -> libc::c_int;
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
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
pub unsafe extern "C" fn gsl_spmatrix_long_fprintf(
    mut stream: *mut FILE,
    mut m: *const gsl_spmatrix_long,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    status = fprintf(
        stream,
        b"%%%%MatrixMarket matrix coordinate real general\n\0" as *const u8
            as *const libc::c_char,
    );
    if status < 0 as libc::c_int {
        gsl_error(
            b"fprintf failed for header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    status = fprintf(
        stream,
        b"%u\t%u\t%u\n\0" as *const u8 as *const libc::c_char,
        (*m).size1 as libc::c_uint,
        (*m).size2 as libc::c_uint,
        (*m).nz as libc::c_uint,
    );
    if status < 0 as libc::c_int {
        gsl_error(
            b"fprintf failed for dimension header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let mut n: size_t = 0;
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            status = fprintf(
                stream,
                b"%d\t%d\t\0" as *const u8 as *const libc::c_char,
                *((*m).i).offset(n as isize) + 1 as libc::c_int,
                *((*m).p).offset(n as isize) + 1 as libc::c_int,
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    70 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            status = fprintf(
                stream,
                format,
                *((*m).data)
                    .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize),
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    76 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            status = putc('\n' as i32, stream);
            if status == -(1 as libc::c_int) {
                gsl_error(
                    b"putc failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    98 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            n = n.wrapping_add(1);
            n;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        let mut j: size_t = 0;
        let mut p: libc::c_int = 0;
        j = 0 as libc::c_int as size_t;
        while j < (*m).size2 {
            p = *((*m).p).offset(j as isize);
            while p
                < *((*m).p)
                    .offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                status = fprintf(
                    stream,
                    b"%d\t%u\t\0" as *const u8 as *const libc::c_char,
                    *((*m).i).offset(p as isize) + 1 as libc::c_int,
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        114 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = fprintf(
                    stream,
                    format,
                    *((*m).data).offset((1 as libc::c_int * p) as isize),
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        120 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = putc('\n' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        142 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                p += 1;
                p;
            }
            j = j.wrapping_add(1);
            j;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        let mut i: size_t = 0;
        let mut p_0: libc::c_int = 0;
        i = 0 as libc::c_int as size_t;
        while i < (*m).size1 {
            p_0 = *((*m).p).offset(i as isize);
            while p_0
                < *((*m).p)
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                status = fprintf(
                    stream,
                    b"%u\t%d\t\0" as *const u8 as *const libc::c_char,
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
                    *((*m).i).offset(p_0 as isize) + 1 as libc::c_int,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        159 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = fprintf(
                    stream,
                    format,
                    *((*m).data).offset((1 as libc::c_int * p_0) as isize),
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        165 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = putc('\n' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        187 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
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
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            194 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_uchar_fprintf(
    mut stream: *mut FILE,
    mut m: *const gsl_spmatrix_uchar,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    status = fprintf(
        stream,
        b"%%%%MatrixMarket matrix coordinate real general\n\0" as *const u8
            as *const libc::c_char,
    );
    if status < 0 as libc::c_int {
        gsl_error(
            b"fprintf failed for header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    status = fprintf(
        stream,
        b"%u\t%u\t%u\n\0" as *const u8 as *const libc::c_char,
        (*m).size1 as libc::c_uint,
        (*m).size2 as libc::c_uint,
        (*m).nz as libc::c_uint,
    );
    if status < 0 as libc::c_int {
        gsl_error(
            b"fprintf failed for dimension header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let mut n: size_t = 0;
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            status = fprintf(
                stream,
                b"%d\t%d\t\0" as *const u8 as *const libc::c_char,
                *((*m).i).offset(n as isize) + 1 as libc::c_int,
                *((*m).p).offset(n as isize) + 1 as libc::c_int,
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    70 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            status = fprintf(
                stream,
                format,
                *((*m).data)
                    .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
                    as libc::c_int,
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    76 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            status = putc('\n' as i32, stream);
            if status == -(1 as libc::c_int) {
                gsl_error(
                    b"putc failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    98 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            n = n.wrapping_add(1);
            n;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        let mut j: size_t = 0;
        let mut p: libc::c_int = 0;
        j = 0 as libc::c_int as size_t;
        while j < (*m).size2 {
            p = *((*m).p).offset(j as isize);
            while p
                < *((*m).p)
                    .offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                status = fprintf(
                    stream,
                    b"%d\t%u\t\0" as *const u8 as *const libc::c_char,
                    *((*m).i).offset(p as isize) + 1 as libc::c_int,
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        114 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = fprintf(
                    stream,
                    format,
                    *((*m).data).offset((1 as libc::c_int * p) as isize) as libc::c_int,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        120 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = putc('\n' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        142 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                p += 1;
                p;
            }
            j = j.wrapping_add(1);
            j;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        let mut i: size_t = 0;
        let mut p_0: libc::c_int = 0;
        i = 0 as libc::c_int as size_t;
        while i < (*m).size1 {
            p_0 = *((*m).p).offset(i as isize);
            while p_0
                < *((*m).p)
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                status = fprintf(
                    stream,
                    b"%u\t%d\t\0" as *const u8 as *const libc::c_char,
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
                    *((*m).i).offset(p_0 as isize) + 1 as libc::c_int,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        159 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = fprintf(
                    stream,
                    format,
                    *((*m).data).offset((1 as libc::c_int * p_0) as isize) as libc::c_int,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        165 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = putc('\n' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        187 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
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
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            194 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_fprintf(
    mut stream: *mut FILE,
    mut m: *const gsl_spmatrix_complex,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    status = fprintf(
        stream,
        b"%%%%MatrixMarket matrix coordinate complex general\n\0" as *const u8
            as *const libc::c_char,
    );
    if status < 0 as libc::c_int {
        gsl_error(
            b"fprintf failed for header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    status = fprintf(
        stream,
        b"%u\t%u\t%u\n\0" as *const u8 as *const libc::c_char,
        (*m).size1 as libc::c_uint,
        (*m).size2 as libc::c_uint,
        (*m).nz as libc::c_uint,
    );
    if status < 0 as libc::c_int {
        gsl_error(
            b"fprintf failed for dimension header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let mut n: size_t = 0;
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            status = fprintf(
                stream,
                b"%d\t%d\t\0" as *const u8 as *const libc::c_char,
                *((*m).i).offset(n as isize) + 1 as libc::c_int,
                *((*m).p).offset(n as isize) + 1 as libc::c_int,
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    70 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            status = fprintf(
                stream,
                format,
                *((*m).data)
                    .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize),
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    76 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            status = putc('\t' as i32, stream);
            if status == -(1 as libc::c_int) {
                gsl_error(
                    b"putc failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    84 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            status = fprintf(
                stream,
                format,
                *((*m).data)
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(n)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ),
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    90 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            status = putc('\n' as i32, stream);
            if status == -(1 as libc::c_int) {
                gsl_error(
                    b"putc failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    98 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            n = n.wrapping_add(1);
            n;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        let mut j: size_t = 0;
        let mut p: libc::c_int = 0;
        j = 0 as libc::c_int as size_t;
        while j < (*m).size2 {
            p = *((*m).p).offset(j as isize);
            while p
                < *((*m).p)
                    .offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                status = fprintf(
                    stream,
                    b"%d\t%u\t\0" as *const u8 as *const libc::c_char,
                    *((*m).i).offset(p as isize) + 1 as libc::c_int,
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        114 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = fprintf(
                    stream,
                    format,
                    *((*m).data).offset((2 as libc::c_int * p) as isize),
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        120 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = putc('\t' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        128 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = fprintf(
                    stream,
                    format,
                    *((*m).data)
                        .offset((2 as libc::c_int * p + 1 as libc::c_int) as isize),
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        134 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = putc('\n' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        142 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                p += 1;
                p;
            }
            j = j.wrapping_add(1);
            j;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        let mut i: size_t = 0;
        let mut p_0: libc::c_int = 0;
        i = 0 as libc::c_int as size_t;
        while i < (*m).size1 {
            p_0 = *((*m).p).offset(i as isize);
            while p_0
                < *((*m).p)
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                status = fprintf(
                    stream,
                    b"%u\t%d\t\0" as *const u8 as *const libc::c_char,
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
                    *((*m).i).offset(p_0 as isize) + 1 as libc::c_int,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        159 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = fprintf(
                    stream,
                    format,
                    *((*m).data).offset((2 as libc::c_int * p_0) as isize),
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        165 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = putc('\t' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        173 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = fprintf(
                    stream,
                    format,
                    *((*m).data)
                        .offset((2 as libc::c_int * p_0 + 1 as libc::c_int) as isize),
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        179 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = putc('\n' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        187 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
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
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            194 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_fprintf(
    mut stream: *mut FILE,
    mut m: *const gsl_spmatrix,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    status = fprintf(
        stream,
        b"%%%%MatrixMarket matrix coordinate real general\n\0" as *const u8
            as *const libc::c_char,
    );
    if status < 0 as libc::c_int {
        gsl_error(
            b"fprintf failed for header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    status = fprintf(
        stream,
        b"%u\t%u\t%u\n\0" as *const u8 as *const libc::c_char,
        (*m).size1 as libc::c_uint,
        (*m).size2 as libc::c_uint,
        (*m).nz as libc::c_uint,
    );
    if status < 0 as libc::c_int {
        gsl_error(
            b"fprintf failed for dimension header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let mut n: size_t = 0;
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            status = fprintf(
                stream,
                b"%d\t%d\t\0" as *const u8 as *const libc::c_char,
                *((*m).i).offset(n as isize) + 1 as libc::c_int,
                *((*m).p).offset(n as isize) + 1 as libc::c_int,
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    70 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            status = fprintf(
                stream,
                format,
                *((*m).data)
                    .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize),
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    76 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            status = putc('\n' as i32, stream);
            if status == -(1 as libc::c_int) {
                gsl_error(
                    b"putc failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    98 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            n = n.wrapping_add(1);
            n;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        let mut j: size_t = 0;
        let mut p: libc::c_int = 0;
        j = 0 as libc::c_int as size_t;
        while j < (*m).size2 {
            p = *((*m).p).offset(j as isize);
            while p
                < *((*m).p)
                    .offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                status = fprintf(
                    stream,
                    b"%d\t%u\t\0" as *const u8 as *const libc::c_char,
                    *((*m).i).offset(p as isize) + 1 as libc::c_int,
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        114 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = fprintf(
                    stream,
                    format,
                    *((*m).data).offset((1 as libc::c_int * p) as isize),
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        120 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = putc('\n' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        142 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                p += 1;
                p;
            }
            j = j.wrapping_add(1);
            j;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        let mut i: size_t = 0;
        let mut p_0: libc::c_int = 0;
        i = 0 as libc::c_int as size_t;
        while i < (*m).size1 {
            p_0 = *((*m).p).offset(i as isize);
            while p_0
                < *((*m).p)
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                status = fprintf(
                    stream,
                    b"%u\t%d\t\0" as *const u8 as *const libc::c_char,
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
                    *((*m).i).offset(p_0 as isize) + 1 as libc::c_int,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        159 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = fprintf(
                    stream,
                    format,
                    *((*m).data).offset((1 as libc::c_int * p_0) as isize),
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        165 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = putc('\n' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        187 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
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
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            194 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_ulong_fprintf(
    mut stream: *mut FILE,
    mut m: *const gsl_spmatrix_ulong,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    status = fprintf(
        stream,
        b"%%%%MatrixMarket matrix coordinate real general\n\0" as *const u8
            as *const libc::c_char,
    );
    if status < 0 as libc::c_int {
        gsl_error(
            b"fprintf failed for header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    status = fprintf(
        stream,
        b"%u\t%u\t%u\n\0" as *const u8 as *const libc::c_char,
        (*m).size1 as libc::c_uint,
        (*m).size2 as libc::c_uint,
        (*m).nz as libc::c_uint,
    );
    if status < 0 as libc::c_int {
        gsl_error(
            b"fprintf failed for dimension header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let mut n: size_t = 0;
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            status = fprintf(
                stream,
                b"%d\t%d\t\0" as *const u8 as *const libc::c_char,
                *((*m).i).offset(n as isize) + 1 as libc::c_int,
                *((*m).p).offset(n as isize) + 1 as libc::c_int,
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    70 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            status = fprintf(
                stream,
                format,
                *((*m).data)
                    .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize),
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    76 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            status = putc('\n' as i32, stream);
            if status == -(1 as libc::c_int) {
                gsl_error(
                    b"putc failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    98 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            n = n.wrapping_add(1);
            n;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        let mut j: size_t = 0;
        let mut p: libc::c_int = 0;
        j = 0 as libc::c_int as size_t;
        while j < (*m).size2 {
            p = *((*m).p).offset(j as isize);
            while p
                < *((*m).p)
                    .offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                status = fprintf(
                    stream,
                    b"%d\t%u\t\0" as *const u8 as *const libc::c_char,
                    *((*m).i).offset(p as isize) + 1 as libc::c_int,
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        114 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = fprintf(
                    stream,
                    format,
                    *((*m).data).offset((1 as libc::c_int * p) as isize),
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        120 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = putc('\n' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        142 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                p += 1;
                p;
            }
            j = j.wrapping_add(1);
            j;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        let mut i: size_t = 0;
        let mut p_0: libc::c_int = 0;
        i = 0 as libc::c_int as size_t;
        while i < (*m).size1 {
            p_0 = *((*m).p).offset(i as isize);
            while p_0
                < *((*m).p)
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                status = fprintf(
                    stream,
                    b"%u\t%d\t\0" as *const u8 as *const libc::c_char,
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
                    *((*m).i).offset(p_0 as isize) + 1 as libc::c_int,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        159 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = fprintf(
                    stream,
                    format,
                    *((*m).data).offset((1 as libc::c_int * p_0) as isize),
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        165 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = putc('\n' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        187 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
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
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            194 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_ushort_fprintf(
    mut stream: *mut FILE,
    mut m: *const gsl_spmatrix_ushort,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    status = fprintf(
        stream,
        b"%%%%MatrixMarket matrix coordinate real general\n\0" as *const u8
            as *const libc::c_char,
    );
    if status < 0 as libc::c_int {
        gsl_error(
            b"fprintf failed for header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    status = fprintf(
        stream,
        b"%u\t%u\t%u\n\0" as *const u8 as *const libc::c_char,
        (*m).size1 as libc::c_uint,
        (*m).size2 as libc::c_uint,
        (*m).nz as libc::c_uint,
    );
    if status < 0 as libc::c_int {
        gsl_error(
            b"fprintf failed for dimension header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let mut n: size_t = 0;
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            status = fprintf(
                stream,
                b"%d\t%d\t\0" as *const u8 as *const libc::c_char,
                *((*m).i).offset(n as isize) + 1 as libc::c_int,
                *((*m).p).offset(n as isize) + 1 as libc::c_int,
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    70 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            status = fprintf(
                stream,
                format,
                *((*m).data)
                    .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
                    as libc::c_int,
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    76 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            status = putc('\n' as i32, stream);
            if status == -(1 as libc::c_int) {
                gsl_error(
                    b"putc failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    98 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            n = n.wrapping_add(1);
            n;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        let mut j: size_t = 0;
        let mut p: libc::c_int = 0;
        j = 0 as libc::c_int as size_t;
        while j < (*m).size2 {
            p = *((*m).p).offset(j as isize);
            while p
                < *((*m).p)
                    .offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                status = fprintf(
                    stream,
                    b"%d\t%u\t\0" as *const u8 as *const libc::c_char,
                    *((*m).i).offset(p as isize) + 1 as libc::c_int,
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        114 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = fprintf(
                    stream,
                    format,
                    *((*m).data).offset((1 as libc::c_int * p) as isize) as libc::c_int,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        120 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = putc('\n' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        142 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                p += 1;
                p;
            }
            j = j.wrapping_add(1);
            j;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        let mut i: size_t = 0;
        let mut p_0: libc::c_int = 0;
        i = 0 as libc::c_int as size_t;
        while i < (*m).size1 {
            p_0 = *((*m).p).offset(i as isize);
            while p_0
                < *((*m).p)
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                status = fprintf(
                    stream,
                    b"%u\t%d\t\0" as *const u8 as *const libc::c_char,
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
                    *((*m).i).offset(p_0 as isize) + 1 as libc::c_int,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        159 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = fprintf(
                    stream,
                    format,
                    *((*m).data).offset((1 as libc::c_int * p_0) as isize) as libc::c_int,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        165 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = putc('\n' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        187 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
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
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            194 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_float_fprintf(
    mut stream: *mut FILE,
    mut m: *const gsl_spmatrix_float,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    status = fprintf(
        stream,
        b"%%%%MatrixMarket matrix coordinate real general\n\0" as *const u8
            as *const libc::c_char,
    );
    if status < 0 as libc::c_int {
        gsl_error(
            b"fprintf failed for header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    status = fprintf(
        stream,
        b"%u\t%u\t%u\n\0" as *const u8 as *const libc::c_char,
        (*m).size1 as libc::c_uint,
        (*m).size2 as libc::c_uint,
        (*m).nz as libc::c_uint,
    );
    if status < 0 as libc::c_int {
        gsl_error(
            b"fprintf failed for dimension header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let mut n: size_t = 0;
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            status = fprintf(
                stream,
                b"%d\t%d\t\0" as *const u8 as *const libc::c_char,
                *((*m).i).offset(n as isize) + 1 as libc::c_int,
                *((*m).p).offset(n as isize) + 1 as libc::c_int,
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    70 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            status = fprintf(
                stream,
                format,
                *((*m).data)
                    .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
                    as libc::c_double,
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    76 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            status = putc('\n' as i32, stream);
            if status == -(1 as libc::c_int) {
                gsl_error(
                    b"putc failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    98 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            n = n.wrapping_add(1);
            n;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        let mut j: size_t = 0;
        let mut p: libc::c_int = 0;
        j = 0 as libc::c_int as size_t;
        while j < (*m).size2 {
            p = *((*m).p).offset(j as isize);
            while p
                < *((*m).p)
                    .offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                status = fprintf(
                    stream,
                    b"%d\t%u\t\0" as *const u8 as *const libc::c_char,
                    *((*m).i).offset(p as isize) + 1 as libc::c_int,
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        114 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = fprintf(
                    stream,
                    format,
                    *((*m).data).offset((1 as libc::c_int * p) as isize)
                        as libc::c_double,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        120 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = putc('\n' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        142 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                p += 1;
                p;
            }
            j = j.wrapping_add(1);
            j;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        let mut i: size_t = 0;
        let mut p_0: libc::c_int = 0;
        i = 0 as libc::c_int as size_t;
        while i < (*m).size1 {
            p_0 = *((*m).p).offset(i as isize);
            while p_0
                < *((*m).p)
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                status = fprintf(
                    stream,
                    b"%u\t%d\t\0" as *const u8 as *const libc::c_char,
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
                    *((*m).i).offset(p_0 as isize) + 1 as libc::c_int,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        159 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = fprintf(
                    stream,
                    format,
                    *((*m).data).offset((1 as libc::c_int * p_0) as isize)
                        as libc::c_double,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        165 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = putc('\n' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        187 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
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
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            194 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_float_fprintf(
    mut stream: *mut FILE,
    mut m: *const gsl_spmatrix_complex_float,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    status = fprintf(
        stream,
        b"%%%%MatrixMarket matrix coordinate complex general\n\0" as *const u8
            as *const libc::c_char,
    );
    if status < 0 as libc::c_int {
        gsl_error(
            b"fprintf failed for header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    status = fprintf(
        stream,
        b"%u\t%u\t%u\n\0" as *const u8 as *const libc::c_char,
        (*m).size1 as libc::c_uint,
        (*m).size2 as libc::c_uint,
        (*m).nz as libc::c_uint,
    );
    if status < 0 as libc::c_int {
        gsl_error(
            b"fprintf failed for dimension header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let mut n: size_t = 0;
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            status = fprintf(
                stream,
                b"%d\t%d\t\0" as *const u8 as *const libc::c_char,
                *((*m).i).offset(n as isize) + 1 as libc::c_int,
                *((*m).p).offset(n as isize) + 1 as libc::c_int,
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    70 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            status = fprintf(
                stream,
                format,
                *((*m).data)
                    .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
                    as libc::c_double,
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    76 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            status = putc('\t' as i32, stream);
            if status == -(1 as libc::c_int) {
                gsl_error(
                    b"putc failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    84 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            status = fprintf(
                stream,
                format,
                *((*m).data)
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(n)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) as libc::c_double,
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    90 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            status = putc('\n' as i32, stream);
            if status == -(1 as libc::c_int) {
                gsl_error(
                    b"putc failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    98 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            n = n.wrapping_add(1);
            n;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        let mut j: size_t = 0;
        let mut p: libc::c_int = 0;
        j = 0 as libc::c_int as size_t;
        while j < (*m).size2 {
            p = *((*m).p).offset(j as isize);
            while p
                < *((*m).p)
                    .offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                status = fprintf(
                    stream,
                    b"%d\t%u\t\0" as *const u8 as *const libc::c_char,
                    *((*m).i).offset(p as isize) + 1 as libc::c_int,
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        114 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = fprintf(
                    stream,
                    format,
                    *((*m).data).offset((2 as libc::c_int * p) as isize)
                        as libc::c_double,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        120 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = putc('\t' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        128 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = fprintf(
                    stream,
                    format,
                    *((*m).data)
                        .offset((2 as libc::c_int * p + 1 as libc::c_int) as isize)
                        as libc::c_double,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        134 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = putc('\n' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        142 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                p += 1;
                p;
            }
            j = j.wrapping_add(1);
            j;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        let mut i: size_t = 0;
        let mut p_0: libc::c_int = 0;
        i = 0 as libc::c_int as size_t;
        while i < (*m).size1 {
            p_0 = *((*m).p).offset(i as isize);
            while p_0
                < *((*m).p)
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                status = fprintf(
                    stream,
                    b"%u\t%d\t\0" as *const u8 as *const libc::c_char,
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
                    *((*m).i).offset(p_0 as isize) + 1 as libc::c_int,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        159 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = fprintf(
                    stream,
                    format,
                    *((*m).data).offset((2 as libc::c_int * p_0) as isize)
                        as libc::c_double,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        165 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = putc('\t' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        173 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = fprintf(
                    stream,
                    format,
                    *((*m).data)
                        .offset((2 as libc::c_int * p_0 + 1 as libc::c_int) as isize)
                        as libc::c_double,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        179 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = putc('\n' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        187 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
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
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            194 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_long_double_fprintf(
    mut stream: *mut FILE,
    mut m: *const gsl_spmatrix_complex_long_double,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    status = fprintf(
        stream,
        b"%%%%MatrixMarket matrix coordinate complex general\n\0" as *const u8
            as *const libc::c_char,
    );
    if status < 0 as libc::c_int {
        gsl_error(
            b"fprintf failed for header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    status = fprintf(
        stream,
        b"%u\t%u\t%u\n\0" as *const u8 as *const libc::c_char,
        (*m).size1 as libc::c_uint,
        (*m).size2 as libc::c_uint,
        (*m).nz as libc::c_uint,
    );
    if status < 0 as libc::c_int {
        gsl_error(
            b"fprintf failed for dimension header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let mut n: size_t = 0;
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            status = fprintf(
                stream,
                b"%d\t%d\t\0" as *const u8 as *const libc::c_char,
                *((*m).i).offset(n as isize) + 1 as libc::c_int,
                *((*m).p).offset(n as isize) + 1 as libc::c_int,
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    70 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            status = fprintf(
                stream,
                format,
                *((*m).data)
                    .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize),
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    76 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            status = putc('\t' as i32, stream);
            if status == -(1 as libc::c_int) {
                gsl_error(
                    b"putc failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    84 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            status = fprintf(
                stream,
                format,
                *((*m).data)
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(n)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ),
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    90 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            status = putc('\n' as i32, stream);
            if status == -(1 as libc::c_int) {
                gsl_error(
                    b"putc failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    98 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            n = n.wrapping_add(1);
            n;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        let mut j: size_t = 0;
        let mut p: libc::c_int = 0;
        j = 0 as libc::c_int as size_t;
        while j < (*m).size2 {
            p = *((*m).p).offset(j as isize);
            while p
                < *((*m).p)
                    .offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                status = fprintf(
                    stream,
                    b"%d\t%u\t\0" as *const u8 as *const libc::c_char,
                    *((*m).i).offset(p as isize) + 1 as libc::c_int,
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        114 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = fprintf(
                    stream,
                    format,
                    *((*m).data).offset((2 as libc::c_int * p) as isize),
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        120 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = putc('\t' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        128 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = fprintf(
                    stream,
                    format,
                    *((*m).data)
                        .offset((2 as libc::c_int * p + 1 as libc::c_int) as isize),
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        134 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = putc('\n' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        142 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                p += 1;
                p;
            }
            j = j.wrapping_add(1);
            j;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        let mut i: size_t = 0;
        let mut p_0: libc::c_int = 0;
        i = 0 as libc::c_int as size_t;
        while i < (*m).size1 {
            p_0 = *((*m).p).offset(i as isize);
            while p_0
                < *((*m).p)
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                status = fprintf(
                    stream,
                    b"%u\t%d\t\0" as *const u8 as *const libc::c_char,
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
                    *((*m).i).offset(p_0 as isize) + 1 as libc::c_int,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        159 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = fprintf(
                    stream,
                    format,
                    *((*m).data).offset((2 as libc::c_int * p_0) as isize),
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        165 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = putc('\t' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        173 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = fprintf(
                    stream,
                    format,
                    *((*m).data)
                        .offset((2 as libc::c_int * p_0 + 1 as libc::c_int) as isize),
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        179 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = putc('\n' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        187 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
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
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            194 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_short_fprintf(
    mut stream: *mut FILE,
    mut m: *const gsl_spmatrix_short,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    status = fprintf(
        stream,
        b"%%%%MatrixMarket matrix coordinate real general\n\0" as *const u8
            as *const libc::c_char,
    );
    if status < 0 as libc::c_int {
        gsl_error(
            b"fprintf failed for header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    status = fprintf(
        stream,
        b"%u\t%u\t%u\n\0" as *const u8 as *const libc::c_char,
        (*m).size1 as libc::c_uint,
        (*m).size2 as libc::c_uint,
        (*m).nz as libc::c_uint,
    );
    if status < 0 as libc::c_int {
        gsl_error(
            b"fprintf failed for dimension header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let mut n: size_t = 0;
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            status = fprintf(
                stream,
                b"%d\t%d\t\0" as *const u8 as *const libc::c_char,
                *((*m).i).offset(n as isize) + 1 as libc::c_int,
                *((*m).p).offset(n as isize) + 1 as libc::c_int,
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    70 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            status = fprintf(
                stream,
                format,
                *((*m).data)
                    .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
                    as libc::c_int,
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    76 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            status = putc('\n' as i32, stream);
            if status == -(1 as libc::c_int) {
                gsl_error(
                    b"putc failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    98 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            n = n.wrapping_add(1);
            n;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        let mut j: size_t = 0;
        let mut p: libc::c_int = 0;
        j = 0 as libc::c_int as size_t;
        while j < (*m).size2 {
            p = *((*m).p).offset(j as isize);
            while p
                < *((*m).p)
                    .offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                status = fprintf(
                    stream,
                    b"%d\t%u\t\0" as *const u8 as *const libc::c_char,
                    *((*m).i).offset(p as isize) + 1 as libc::c_int,
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        114 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = fprintf(
                    stream,
                    format,
                    *((*m).data).offset((1 as libc::c_int * p) as isize) as libc::c_int,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        120 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = putc('\n' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        142 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                p += 1;
                p;
            }
            j = j.wrapping_add(1);
            j;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        let mut i: size_t = 0;
        let mut p_0: libc::c_int = 0;
        i = 0 as libc::c_int as size_t;
        while i < (*m).size1 {
            p_0 = *((*m).p).offset(i as isize);
            while p_0
                < *((*m).p)
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                status = fprintf(
                    stream,
                    b"%u\t%d\t\0" as *const u8 as *const libc::c_char,
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
                    *((*m).i).offset(p_0 as isize) + 1 as libc::c_int,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        159 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = fprintf(
                    stream,
                    format,
                    *((*m).data).offset((1 as libc::c_int * p_0) as isize) as libc::c_int,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        165 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = putc('\n' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        187 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
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
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            194 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_fprintf(
    mut stream: *mut FILE,
    mut m: *const gsl_spmatrix_long_double,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    status = fprintf(
        stream,
        b"%%%%MatrixMarket matrix coordinate real general\n\0" as *const u8
            as *const libc::c_char,
    );
    if status < 0 as libc::c_int {
        gsl_error(
            b"fprintf failed for header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    status = fprintf(
        stream,
        b"%u\t%u\t%u\n\0" as *const u8 as *const libc::c_char,
        (*m).size1 as libc::c_uint,
        (*m).size2 as libc::c_uint,
        (*m).nz as libc::c_uint,
    );
    if status < 0 as libc::c_int {
        gsl_error(
            b"fprintf failed for dimension header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let mut n: size_t = 0;
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            status = fprintf(
                stream,
                b"%d\t%d\t\0" as *const u8 as *const libc::c_char,
                *((*m).i).offset(n as isize) + 1 as libc::c_int,
                *((*m).p).offset(n as isize) + 1 as libc::c_int,
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    70 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            status = fprintf(
                stream,
                format,
                *((*m).data)
                    .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize),
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    76 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            status = putc('\n' as i32, stream);
            if status == -(1 as libc::c_int) {
                gsl_error(
                    b"putc failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    98 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            n = n.wrapping_add(1);
            n;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        let mut j: size_t = 0;
        let mut p: libc::c_int = 0;
        j = 0 as libc::c_int as size_t;
        while j < (*m).size2 {
            p = *((*m).p).offset(j as isize);
            while p
                < *((*m).p)
                    .offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                status = fprintf(
                    stream,
                    b"%d\t%u\t\0" as *const u8 as *const libc::c_char,
                    *((*m).i).offset(p as isize) + 1 as libc::c_int,
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        114 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = fprintf(
                    stream,
                    format,
                    *((*m).data).offset((1 as libc::c_int * p) as isize),
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        120 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = putc('\n' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        142 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                p += 1;
                p;
            }
            j = j.wrapping_add(1);
            j;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        let mut i: size_t = 0;
        let mut p_0: libc::c_int = 0;
        i = 0 as libc::c_int as size_t;
        while i < (*m).size1 {
            p_0 = *((*m).p).offset(i as isize);
            while p_0
                < *((*m).p)
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                status = fprintf(
                    stream,
                    b"%u\t%d\t\0" as *const u8 as *const libc::c_char,
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
                    *((*m).i).offset(p_0 as isize) + 1 as libc::c_int,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        159 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = fprintf(
                    stream,
                    format,
                    *((*m).data).offset((1 as libc::c_int * p_0) as isize),
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        165 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = putc('\n' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        187 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
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
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            194 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_uint_fprintf(
    mut stream: *mut FILE,
    mut m: *const gsl_spmatrix_uint,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    status = fprintf(
        stream,
        b"%%%%MatrixMarket matrix coordinate real general\n\0" as *const u8
            as *const libc::c_char,
    );
    if status < 0 as libc::c_int {
        gsl_error(
            b"fprintf failed for header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    status = fprintf(
        stream,
        b"%u\t%u\t%u\n\0" as *const u8 as *const libc::c_char,
        (*m).size1 as libc::c_uint,
        (*m).size2 as libc::c_uint,
        (*m).nz as libc::c_uint,
    );
    if status < 0 as libc::c_int {
        gsl_error(
            b"fprintf failed for dimension header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let mut n: size_t = 0;
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            status = fprintf(
                stream,
                b"%d\t%d\t\0" as *const u8 as *const libc::c_char,
                *((*m).i).offset(n as isize) + 1 as libc::c_int,
                *((*m).p).offset(n as isize) + 1 as libc::c_int,
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    70 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            status = fprintf(
                stream,
                format,
                *((*m).data)
                    .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize),
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    76 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            status = putc('\n' as i32, stream);
            if status == -(1 as libc::c_int) {
                gsl_error(
                    b"putc failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    98 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            n = n.wrapping_add(1);
            n;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        let mut j: size_t = 0;
        let mut p: libc::c_int = 0;
        j = 0 as libc::c_int as size_t;
        while j < (*m).size2 {
            p = *((*m).p).offset(j as isize);
            while p
                < *((*m).p)
                    .offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                status = fprintf(
                    stream,
                    b"%d\t%u\t\0" as *const u8 as *const libc::c_char,
                    *((*m).i).offset(p as isize) + 1 as libc::c_int,
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        114 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = fprintf(
                    stream,
                    format,
                    *((*m).data).offset((1 as libc::c_int * p) as isize),
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        120 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = putc('\n' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        142 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                p += 1;
                p;
            }
            j = j.wrapping_add(1);
            j;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        let mut i: size_t = 0;
        let mut p_0: libc::c_int = 0;
        i = 0 as libc::c_int as size_t;
        while i < (*m).size1 {
            p_0 = *((*m).p).offset(i as isize);
            while p_0
                < *((*m).p)
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                status = fprintf(
                    stream,
                    b"%u\t%d\t\0" as *const u8 as *const libc::c_char,
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
                    *((*m).i).offset(p_0 as isize) + 1 as libc::c_int,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        159 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = fprintf(
                    stream,
                    format,
                    *((*m).data).offset((1 as libc::c_int * p_0) as isize),
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        165 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = putc('\n' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        187 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
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
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            194 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_int_fprintf(
    mut stream: *mut FILE,
    mut m: *const gsl_spmatrix_int,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    status = fprintf(
        stream,
        b"%%%%MatrixMarket matrix coordinate real general\n\0" as *const u8
            as *const libc::c_char,
    );
    if status < 0 as libc::c_int {
        gsl_error(
            b"fprintf failed for header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    status = fprintf(
        stream,
        b"%u\t%u\t%u\n\0" as *const u8 as *const libc::c_char,
        (*m).size1 as libc::c_uint,
        (*m).size2 as libc::c_uint,
        (*m).nz as libc::c_uint,
    );
    if status < 0 as libc::c_int {
        gsl_error(
            b"fprintf failed for dimension header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let mut n: size_t = 0;
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            status = fprintf(
                stream,
                b"%d\t%d\t\0" as *const u8 as *const libc::c_char,
                *((*m).i).offset(n as isize) + 1 as libc::c_int,
                *((*m).p).offset(n as isize) + 1 as libc::c_int,
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    70 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            status = fprintf(
                stream,
                format,
                *((*m).data)
                    .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize),
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    76 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            status = putc('\n' as i32, stream);
            if status == -(1 as libc::c_int) {
                gsl_error(
                    b"putc failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    98 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            n = n.wrapping_add(1);
            n;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        let mut j: size_t = 0;
        let mut p: libc::c_int = 0;
        j = 0 as libc::c_int as size_t;
        while j < (*m).size2 {
            p = *((*m).p).offset(j as isize);
            while p
                < *((*m).p)
                    .offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                status = fprintf(
                    stream,
                    b"%d\t%u\t\0" as *const u8 as *const libc::c_char,
                    *((*m).i).offset(p as isize) + 1 as libc::c_int,
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        114 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = fprintf(
                    stream,
                    format,
                    *((*m).data).offset((1 as libc::c_int * p) as isize),
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        120 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = putc('\n' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        142 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                p += 1;
                p;
            }
            j = j.wrapping_add(1);
            j;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        let mut i: size_t = 0;
        let mut p_0: libc::c_int = 0;
        i = 0 as libc::c_int as size_t;
        while i < (*m).size1 {
            p_0 = *((*m).p).offset(i as isize);
            while p_0
                < *((*m).p)
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                status = fprintf(
                    stream,
                    b"%u\t%d\t\0" as *const u8 as *const libc::c_char,
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
                    *((*m).i).offset(p_0 as isize) + 1 as libc::c_int,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        159 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = fprintf(
                    stream,
                    format,
                    *((*m).data).offset((1 as libc::c_int * p_0) as isize),
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        165 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = putc('\n' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        187 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
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
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            194 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_char_fprintf(
    mut stream: *mut FILE,
    mut m: *const gsl_spmatrix_char,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    status = fprintf(
        stream,
        b"%%%%MatrixMarket matrix coordinate real general\n\0" as *const u8
            as *const libc::c_char,
    );
    if status < 0 as libc::c_int {
        gsl_error(
            b"fprintf failed for header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    status = fprintf(
        stream,
        b"%u\t%u\t%u\n\0" as *const u8 as *const libc::c_char,
        (*m).size1 as libc::c_uint,
        (*m).size2 as libc::c_uint,
        (*m).nz as libc::c_uint,
    );
    if status < 0 as libc::c_int {
        gsl_error(
            b"fprintf failed for dimension header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        let mut n: size_t = 0;
        n = 0 as libc::c_int as size_t;
        while n < (*m).nz {
            status = fprintf(
                stream,
                b"%d\t%d\t\0" as *const u8 as *const libc::c_char,
                *((*m).i).offset(n as isize) + 1 as libc::c_int,
                *((*m).p).offset(n as isize) + 1 as libc::c_int,
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    70 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            status = fprintf(
                stream,
                format,
                *((*m).data)
                    .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
                    as libc::c_int,
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    76 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            status = putc('\n' as i32, stream);
            if status == -(1 as libc::c_int) {
                gsl_error(
                    b"putc failed\0" as *const u8 as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    98 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            n = n.wrapping_add(1);
            n;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        let mut j: size_t = 0;
        let mut p: libc::c_int = 0;
        j = 0 as libc::c_int as size_t;
        while j < (*m).size2 {
            p = *((*m).p).offset(j as isize);
            while p
                < *((*m).p)
                    .offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                status = fprintf(
                    stream,
                    b"%d\t%u\t\0" as *const u8 as *const libc::c_char,
                    *((*m).i).offset(p as isize) + 1 as libc::c_int,
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        114 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = fprintf(
                    stream,
                    format,
                    *((*m).data).offset((1 as libc::c_int * p) as isize) as libc::c_int,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        120 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = putc('\n' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        142 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                p += 1;
                p;
            }
            j = j.wrapping_add(1);
            j;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        let mut i: size_t = 0;
        let mut p_0: libc::c_int = 0;
        i = 0 as libc::c_int as size_t;
        while i < (*m).size1 {
            p_0 = *((*m).p).offset(i as isize);
            while p_0
                < *((*m).p)
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            {
                status = fprintf(
                    stream,
                    b"%u\t%d\t\0" as *const u8 as *const libc::c_char,
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
                    *((*m).i).offset(p_0 as isize) + 1 as libc::c_int,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        159 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = fprintf(
                    stream,
                    format,
                    *((*m).data).offset((1 as libc::c_int * p_0) as isize) as libc::c_int,
                );
                if status < 0 as libc::c_int {
                    gsl_error(
                        b"fprintf failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        165 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
                status = putc('\n' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./file_source.c\0" as *const u8 as *const libc::c_char,
                        187 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
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
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            194 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_uchar_fscanf(
    mut stream: *mut FILE,
) -> *mut gsl_spmatrix_uchar {
    let mut m: *mut gsl_spmatrix_uchar = 0 as *mut gsl_spmatrix_uchar;
    let mut size1: libc::c_uint = 0;
    let mut size2: libc::c_uint = 0;
    let mut nz: libc::c_uint = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut found_header: libc::c_int = 0 as libc::c_int;
    while !(fgets(buf.as_mut_ptr(), 1024 as libc::c_int, stream)).is_null() {
        let mut c: libc::c_int = 0;
        if *buf.as_mut_ptr() as libc::c_int == '%' as i32 {
            continue;
        }
        c = sscanf(
            buf.as_mut_ptr(),
            b"%u %u %u\0" as *const u8 as *const libc::c_char,
            &mut size1 as *mut libc::c_uint,
            &mut size2 as *mut libc::c_uint,
            &mut nz as *mut libc::c_uint,
        );
        if !(c == 3 as libc::c_int) {
            continue;
        }
        found_header = 1 as libc::c_int;
        break;
    }
    if found_header == 0 {
        gsl_error(
            b"fscanf failed reading header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            227 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_uchar;
    }
    m = gsl_spmatrix_uchar_alloc_nzmax(
        size1 as size_t,
        size2 as size_t,
        nz as size_t,
        GSL_SPMATRIX_COO as libc::c_int,
    );
    if m.is_null() {
        gsl_error(
            b"error allocating m\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            233 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_uchar;
    }
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut tmp: libc::c_uint = 0;
    while !(fgets(buf.as_mut_ptr(), 1024 as libc::c_int, stream)).is_null() {
        let mut c_0: libc::c_int = sscanf(
            buf.as_mut_ptr(),
            b"%u %u %u\0" as *const u8 as *const libc::c_char,
            &mut i as *mut libc::c_uint,
            &mut j as *mut libc::c_uint,
            &mut tmp as *mut libc::c_uint,
        );
        if c_0 < 3 as libc::c_int || i == 0 as libc::c_int as libc::c_uint
            || j == 0 as libc::c_int as libc::c_uint
        {
            gsl_error(
                b"error in input file format\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                275 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_uchar;
        } else if i > size1 || j > size2 {
            gsl_error(
                b"element exceeds matrix dimensions\0" as *const u8
                    as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                279 as libc::c_int,
                GSL_EBADLEN as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_uchar;
        } else {
            gsl_spmatrix_uchar_set(
                m,
                i.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                j.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                tmp as libc::c_uchar,
            );
        }
    }
    return m;
}
pub unsafe extern "C" fn gsl_spmatrix_char_fscanf(
    mut stream: *mut FILE,
) -> *mut gsl_spmatrix_char {
    let mut m: *mut gsl_spmatrix_char = 0 as *mut gsl_spmatrix_char;
    let mut size1: libc::c_uint = 0;
    let mut size2: libc::c_uint = 0;
    let mut nz: libc::c_uint = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut found_header: libc::c_int = 0 as libc::c_int;
    while !(fgets(buf.as_mut_ptr(), 1024 as libc::c_int, stream)).is_null() {
        let mut c: libc::c_int = 0;
        if *buf.as_mut_ptr() as libc::c_int == '%' as i32 {
            continue;
        }
        c = sscanf(
            buf.as_mut_ptr(),
            b"%u %u %u\0" as *const u8 as *const libc::c_char,
            &mut size1 as *mut libc::c_uint,
            &mut size2 as *mut libc::c_uint,
            &mut nz as *mut libc::c_uint,
        );
        if !(c == 3 as libc::c_int) {
            continue;
        }
        found_header = 1 as libc::c_int;
        break;
    }
    if found_header == 0 {
        gsl_error(
            b"fscanf failed reading header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            227 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_char;
    }
    m = gsl_spmatrix_char_alloc_nzmax(
        size1 as size_t,
        size2 as size_t,
        nz as size_t,
        GSL_SPMATRIX_COO as libc::c_int,
    );
    if m.is_null() {
        gsl_error(
            b"error allocating m\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            233 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_char;
    }
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut tmp: libc::c_int = 0;
    while !(fgets(buf.as_mut_ptr(), 1024 as libc::c_int, stream)).is_null() {
        let mut c_0: libc::c_int = sscanf(
            buf.as_mut_ptr(),
            b"%u %u %d\0" as *const u8 as *const libc::c_char,
            &mut i as *mut libc::c_uint,
            &mut j as *mut libc::c_uint,
            &mut tmp as *mut libc::c_int,
        );
        if c_0 < 3 as libc::c_int || i == 0 as libc::c_int as libc::c_uint
            || j == 0 as libc::c_int as libc::c_uint
        {
            gsl_error(
                b"error in input file format\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                275 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_char;
        } else if i > size1 || j > size2 {
            gsl_error(
                b"element exceeds matrix dimensions\0" as *const u8
                    as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                279 as libc::c_int,
                GSL_EBADLEN as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_char;
        } else {
            gsl_spmatrix_char_set(
                m,
                i.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                j.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                tmp as libc::c_char,
            );
        }
    }
    return m;
}
pub unsafe extern "C" fn gsl_spmatrix_int_fscanf(
    mut stream: *mut FILE,
) -> *mut gsl_spmatrix_int {
    let mut m: *mut gsl_spmatrix_int = 0 as *mut gsl_spmatrix_int;
    let mut size1: libc::c_uint = 0;
    let mut size2: libc::c_uint = 0;
    let mut nz: libc::c_uint = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut found_header: libc::c_int = 0 as libc::c_int;
    while !(fgets(buf.as_mut_ptr(), 1024 as libc::c_int, stream)).is_null() {
        let mut c: libc::c_int = 0;
        if *buf.as_mut_ptr() as libc::c_int == '%' as i32 {
            continue;
        }
        c = sscanf(
            buf.as_mut_ptr(),
            b"%u %u %u\0" as *const u8 as *const libc::c_char,
            &mut size1 as *mut libc::c_uint,
            &mut size2 as *mut libc::c_uint,
            &mut nz as *mut libc::c_uint,
        );
        if !(c == 3 as libc::c_int) {
            continue;
        }
        found_header = 1 as libc::c_int;
        break;
    }
    if found_header == 0 {
        gsl_error(
            b"fscanf failed reading header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            227 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_int;
    }
    m = gsl_spmatrix_int_alloc_nzmax(
        size1 as size_t,
        size2 as size_t,
        nz as size_t,
        GSL_SPMATRIX_COO as libc::c_int,
    );
    if m.is_null() {
        gsl_error(
            b"error allocating m\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            233 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_int;
    }
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut tmp: libc::c_int = 0;
    while !(fgets(buf.as_mut_ptr(), 1024 as libc::c_int, stream)).is_null() {
        let mut c_0: libc::c_int = sscanf(
            buf.as_mut_ptr(),
            b"%u %u %d\0" as *const u8 as *const libc::c_char,
            &mut i as *mut libc::c_uint,
            &mut j as *mut libc::c_uint,
            &mut tmp as *mut libc::c_int,
        );
        if c_0 < 3 as libc::c_int || i == 0 as libc::c_int as libc::c_uint
            || j == 0 as libc::c_int as libc::c_uint
        {
            gsl_error(
                b"error in input file format\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                275 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_int;
        } else if i > size1 || j > size2 {
            gsl_error(
                b"element exceeds matrix dimensions\0" as *const u8
                    as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                279 as libc::c_int,
                GSL_EBADLEN as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_int;
        } else {
            gsl_spmatrix_int_set(
                m,
                i.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                j.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                tmp,
            );
        }
    }
    return m;
}
pub unsafe extern "C" fn gsl_spmatrix_uint_fscanf(
    mut stream: *mut FILE,
) -> *mut gsl_spmatrix_uint {
    let mut m: *mut gsl_spmatrix_uint = 0 as *mut gsl_spmatrix_uint;
    let mut size1: libc::c_uint = 0;
    let mut size2: libc::c_uint = 0;
    let mut nz: libc::c_uint = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut found_header: libc::c_int = 0 as libc::c_int;
    while !(fgets(buf.as_mut_ptr(), 1024 as libc::c_int, stream)).is_null() {
        let mut c: libc::c_int = 0;
        if *buf.as_mut_ptr() as libc::c_int == '%' as i32 {
            continue;
        }
        c = sscanf(
            buf.as_mut_ptr(),
            b"%u %u %u\0" as *const u8 as *const libc::c_char,
            &mut size1 as *mut libc::c_uint,
            &mut size2 as *mut libc::c_uint,
            &mut nz as *mut libc::c_uint,
        );
        if !(c == 3 as libc::c_int) {
            continue;
        }
        found_header = 1 as libc::c_int;
        break;
    }
    if found_header == 0 {
        gsl_error(
            b"fscanf failed reading header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            227 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_uint;
    }
    m = gsl_spmatrix_uint_alloc_nzmax(
        size1 as size_t,
        size2 as size_t,
        nz as size_t,
        GSL_SPMATRIX_COO as libc::c_int,
    );
    if m.is_null() {
        gsl_error(
            b"error allocating m\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            233 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_uint;
    }
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut tmp: libc::c_uint = 0;
    while !(fgets(buf.as_mut_ptr(), 1024 as libc::c_int, stream)).is_null() {
        let mut c_0: libc::c_int = sscanf(
            buf.as_mut_ptr(),
            b"%u %u %u\0" as *const u8 as *const libc::c_char,
            &mut i as *mut libc::c_uint,
            &mut j as *mut libc::c_uint,
            &mut tmp as *mut libc::c_uint,
        );
        if c_0 < 3 as libc::c_int || i == 0 as libc::c_int as libc::c_uint
            || j == 0 as libc::c_int as libc::c_uint
        {
            gsl_error(
                b"error in input file format\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                275 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_uint;
        } else if i > size1 || j > size2 {
            gsl_error(
                b"element exceeds matrix dimensions\0" as *const u8
                    as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                279 as libc::c_int,
                GSL_EBADLEN as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_uint;
        } else {
            gsl_spmatrix_uint_set(
                m,
                i.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                j.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                tmp,
            );
        }
    }
    return m;
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_fscanf(
    mut stream: *mut FILE,
) -> *mut gsl_spmatrix_long_double {
    let mut m: *mut gsl_spmatrix_long_double = 0 as *mut gsl_spmatrix_long_double;
    let mut size1: libc::c_uint = 0;
    let mut size2: libc::c_uint = 0;
    let mut nz: libc::c_uint = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut found_header: libc::c_int = 0 as libc::c_int;
    while !(fgets(buf.as_mut_ptr(), 1024 as libc::c_int, stream)).is_null() {
        let mut c: libc::c_int = 0;
        if *buf.as_mut_ptr() as libc::c_int == '%' as i32 {
            continue;
        }
        c = sscanf(
            buf.as_mut_ptr(),
            b"%u %u %u\0" as *const u8 as *const libc::c_char,
            &mut size1 as *mut libc::c_uint,
            &mut size2 as *mut libc::c_uint,
            &mut nz as *mut libc::c_uint,
        );
        if !(c == 3 as libc::c_int) {
            continue;
        }
        found_header = 1 as libc::c_int;
        break;
    }
    if found_header == 0 {
        gsl_error(
            b"fscanf failed reading header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            227 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_long_double;
    }
    m = gsl_spmatrix_long_double_alloc_nzmax(
        size1 as size_t,
        size2 as size_t,
        nz as size_t,
        GSL_SPMATRIX_COO as libc::c_int,
    );
    if m.is_null() {
        gsl_error(
            b"error allocating m\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            233 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_long_double;
    }
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut tmp: f128::f128 = f128::f128::ZERO;
    while !(fgets(buf.as_mut_ptr(), 1024 as libc::c_int, stream)).is_null() {
        let mut c_0: libc::c_int = sscanf(
            buf.as_mut_ptr(),
            b"%u %u %Lg\0" as *const u8 as *const libc::c_char,
            &mut i as *mut libc::c_uint,
            &mut j as *mut libc::c_uint,
            &mut tmp as *mut f128::f128,
        );
        if c_0 < 3 as libc::c_int || i == 0 as libc::c_int as libc::c_uint
            || j == 0 as libc::c_int as libc::c_uint
        {
            gsl_error(
                b"error in input file format\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                275 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_long_double;
        } else if i > size1 || j > size2 {
            gsl_error(
                b"element exceeds matrix dimensions\0" as *const u8
                    as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                279 as libc::c_int,
                GSL_EBADLEN as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_long_double;
        } else {
            gsl_spmatrix_long_double_set(
                m,
                i.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                j.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                tmp,
            );
        }
    }
    return m;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_float_fscanf(
    mut stream: *mut FILE,
) -> *mut gsl_spmatrix_complex_float {
    let mut m: *mut gsl_spmatrix_complex_float = 0 as *mut gsl_spmatrix_complex_float;
    let mut size1: libc::c_uint = 0;
    let mut size2: libc::c_uint = 0;
    let mut nz: libc::c_uint = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut found_header: libc::c_int = 0 as libc::c_int;
    while !(fgets(buf.as_mut_ptr(), 1024 as libc::c_int, stream)).is_null() {
        let mut c: libc::c_int = 0;
        if *buf.as_mut_ptr() as libc::c_int == '%' as i32 {
            continue;
        }
        c = sscanf(
            buf.as_mut_ptr(),
            b"%u %u %u\0" as *const u8 as *const libc::c_char,
            &mut size1 as *mut libc::c_uint,
            &mut size2 as *mut libc::c_uint,
            &mut nz as *mut libc::c_uint,
        );
        if !(c == 3 as libc::c_int) {
            continue;
        }
        found_header = 1 as libc::c_int;
        break;
    }
    if found_header == 0 {
        gsl_error(
            b"fscanf failed reading header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            227 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_complex_float;
    }
    m = gsl_spmatrix_complex_float_alloc_nzmax(
        size1 as size_t,
        size2 as size_t,
        nz as size_t,
        GSL_SPMATRIX_COO as libc::c_int,
    );
    if m.is_null() {
        gsl_error(
            b"error allocating m\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            233 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_complex_float;
    }
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut xr: libc::c_float = 0.;
    let mut xi: libc::c_float = 0.;
    let mut x: gsl_complex_float = gsl_complex_float { dat: [0.; 2] };
    while !(fgets(buf.as_mut_ptr(), 1024 as libc::c_int, stream)).is_null() {
        let mut c_0: libc::c_int = sscanf(
            buf.as_mut_ptr(),
            b"%u %u %g %g\0" as *const u8 as *const libc::c_char,
            &mut i as *mut libc::c_uint,
            &mut j as *mut libc::c_uint,
            &mut xr as *mut libc::c_float,
            &mut xi as *mut libc::c_float,
        );
        if c_0 < 4 as libc::c_int || i == 0 as libc::c_int as libc::c_uint
            || j == 0 as libc::c_int as libc::c_uint
        {
            gsl_error(
                b"error in input file format\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                248 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_complex_float;
        } else if i > size1 || j > size2 {
            gsl_error(
                b"element exceeds matrix dimensions\0" as *const u8
                    as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                252 as libc::c_int,
                GSL_EBADLEN as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_complex_float;
        } else {
            x.dat[0 as libc::c_int as usize] = xr;
            x.dat[1 as libc::c_int as usize] = xi;
            gsl_spmatrix_complex_float_set(
                m,
                i.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                j.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                x,
            );
        }
    }
    return m;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_long_double_fscanf(
    mut stream: *mut FILE,
) -> *mut gsl_spmatrix_complex_long_double {
    let mut m: *mut gsl_spmatrix_complex_long_double = 0
        as *mut gsl_spmatrix_complex_long_double;
    let mut size1: libc::c_uint = 0;
    let mut size2: libc::c_uint = 0;
    let mut nz: libc::c_uint = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut found_header: libc::c_int = 0 as libc::c_int;
    while !(fgets(buf.as_mut_ptr(), 1024 as libc::c_int, stream)).is_null() {
        let mut c: libc::c_int = 0;
        if *buf.as_mut_ptr() as libc::c_int == '%' as i32 {
            continue;
        }
        c = sscanf(
            buf.as_mut_ptr(),
            b"%u %u %u\0" as *const u8 as *const libc::c_char,
            &mut size1 as *mut libc::c_uint,
            &mut size2 as *mut libc::c_uint,
            &mut nz as *mut libc::c_uint,
        );
        if !(c == 3 as libc::c_int) {
            continue;
        }
        found_header = 1 as libc::c_int;
        break;
    }
    if found_header == 0 {
        gsl_error(
            b"fscanf failed reading header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            227 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_complex_long_double;
    }
    m = gsl_spmatrix_complex_long_double_alloc_nzmax(
        size1 as size_t,
        size2 as size_t,
        nz as size_t,
        GSL_SPMATRIX_COO as libc::c_int,
    );
    if m.is_null() {
        gsl_error(
            b"error allocating m\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            233 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_complex_long_double;
    }
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut xr: f128::f128 = f128::f128::ZERO;
    let mut xi: f128::f128 = f128::f128::ZERO;
    let mut x: gsl_complex_long_double = gsl_complex_long_double {
        dat: [f128::f128::ZERO; 2],
    };
    while !(fgets(buf.as_mut_ptr(), 1024 as libc::c_int, stream)).is_null() {
        let mut c_0: libc::c_int = sscanf(
            buf.as_mut_ptr(),
            b"%u %u %Lg %Lg\0" as *const u8 as *const libc::c_char,
            &mut i as *mut libc::c_uint,
            &mut j as *mut libc::c_uint,
            &mut xr as *mut f128::f128,
            &mut xi as *mut f128::f128,
        );
        if c_0 < 4 as libc::c_int || i == 0 as libc::c_int as libc::c_uint
            || j == 0 as libc::c_int as libc::c_uint
        {
            gsl_error(
                b"error in input file format\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                248 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_complex_long_double;
        } else if i > size1 || j > size2 {
            gsl_error(
                b"element exceeds matrix dimensions\0" as *const u8
                    as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                252 as libc::c_int,
                GSL_EBADLEN as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_complex_long_double;
        } else {
            x.dat[0 as libc::c_int as usize] = xr;
            x.dat[1 as libc::c_int as usize] = xi;
            gsl_spmatrix_complex_long_double_set(
                m,
                i.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                j.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                x,
            );
        }
    }
    return m;
}
pub unsafe extern "C" fn gsl_spmatrix_float_fscanf(
    mut stream: *mut FILE,
) -> *mut gsl_spmatrix_float {
    let mut m: *mut gsl_spmatrix_float = 0 as *mut gsl_spmatrix_float;
    let mut size1: libc::c_uint = 0;
    let mut size2: libc::c_uint = 0;
    let mut nz: libc::c_uint = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut found_header: libc::c_int = 0 as libc::c_int;
    while !(fgets(buf.as_mut_ptr(), 1024 as libc::c_int, stream)).is_null() {
        let mut c: libc::c_int = 0;
        if *buf.as_mut_ptr() as libc::c_int == '%' as i32 {
            continue;
        }
        c = sscanf(
            buf.as_mut_ptr(),
            b"%u %u %u\0" as *const u8 as *const libc::c_char,
            &mut size1 as *mut libc::c_uint,
            &mut size2 as *mut libc::c_uint,
            &mut nz as *mut libc::c_uint,
        );
        if !(c == 3 as libc::c_int) {
            continue;
        }
        found_header = 1 as libc::c_int;
        break;
    }
    if found_header == 0 {
        gsl_error(
            b"fscanf failed reading header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            227 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_float;
    }
    m = gsl_spmatrix_float_alloc_nzmax(
        size1 as size_t,
        size2 as size_t,
        nz as size_t,
        GSL_SPMATRIX_COO as libc::c_int,
    );
    if m.is_null() {
        gsl_error(
            b"error allocating m\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            233 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_float;
    }
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut tmp: libc::c_float = 0.;
    while !(fgets(buf.as_mut_ptr(), 1024 as libc::c_int, stream)).is_null() {
        let mut c_0: libc::c_int = sscanf(
            buf.as_mut_ptr(),
            b"%u %u %g\0" as *const u8 as *const libc::c_char,
            &mut i as *mut libc::c_uint,
            &mut j as *mut libc::c_uint,
            &mut tmp as *mut libc::c_float,
        );
        if c_0 < 3 as libc::c_int || i == 0 as libc::c_int as libc::c_uint
            || j == 0 as libc::c_int as libc::c_uint
        {
            gsl_error(
                b"error in input file format\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                275 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_float;
        } else if i > size1 || j > size2 {
            gsl_error(
                b"element exceeds matrix dimensions\0" as *const u8
                    as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                279 as libc::c_int,
                GSL_EBADLEN as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_float;
        } else {
            gsl_spmatrix_float_set(
                m,
                i.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                j.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                tmp,
            );
        }
    }
    return m;
}
pub unsafe extern "C" fn gsl_spmatrix_short_fscanf(
    mut stream: *mut FILE,
) -> *mut gsl_spmatrix_short {
    let mut m: *mut gsl_spmatrix_short = 0 as *mut gsl_spmatrix_short;
    let mut size1: libc::c_uint = 0;
    let mut size2: libc::c_uint = 0;
    let mut nz: libc::c_uint = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut found_header: libc::c_int = 0 as libc::c_int;
    while !(fgets(buf.as_mut_ptr(), 1024 as libc::c_int, stream)).is_null() {
        let mut c: libc::c_int = 0;
        if *buf.as_mut_ptr() as libc::c_int == '%' as i32 {
            continue;
        }
        c = sscanf(
            buf.as_mut_ptr(),
            b"%u %u %u\0" as *const u8 as *const libc::c_char,
            &mut size1 as *mut libc::c_uint,
            &mut size2 as *mut libc::c_uint,
            &mut nz as *mut libc::c_uint,
        );
        if !(c == 3 as libc::c_int) {
            continue;
        }
        found_header = 1 as libc::c_int;
        break;
    }
    if found_header == 0 {
        gsl_error(
            b"fscanf failed reading header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            227 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_short;
    }
    m = gsl_spmatrix_short_alloc_nzmax(
        size1 as size_t,
        size2 as size_t,
        nz as size_t,
        GSL_SPMATRIX_COO as libc::c_int,
    );
    if m.is_null() {
        gsl_error(
            b"error allocating m\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            233 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_short;
    }
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut tmp: libc::c_short = 0;
    while !(fgets(buf.as_mut_ptr(), 1024 as libc::c_int, stream)).is_null() {
        let mut c_0: libc::c_int = sscanf(
            buf.as_mut_ptr(),
            b"%u %u %hd\0" as *const u8 as *const libc::c_char,
            &mut i as *mut libc::c_uint,
            &mut j as *mut libc::c_uint,
            &mut tmp as *mut libc::c_short,
        );
        if c_0 < 3 as libc::c_int || i == 0 as libc::c_int as libc::c_uint
            || j == 0 as libc::c_int as libc::c_uint
        {
            gsl_error(
                b"error in input file format\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                275 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_short;
        } else if i > size1 || j > size2 {
            gsl_error(
                b"element exceeds matrix dimensions\0" as *const u8
                    as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                279 as libc::c_int,
                GSL_EBADLEN as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_short;
        } else {
            gsl_spmatrix_short_set(
                m,
                i.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                j.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                tmp,
            );
        }
    }
    return m;
}
pub unsafe extern "C" fn gsl_spmatrix_ushort_fscanf(
    mut stream: *mut FILE,
) -> *mut gsl_spmatrix_ushort {
    let mut m: *mut gsl_spmatrix_ushort = 0 as *mut gsl_spmatrix_ushort;
    let mut size1: libc::c_uint = 0;
    let mut size2: libc::c_uint = 0;
    let mut nz: libc::c_uint = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut found_header: libc::c_int = 0 as libc::c_int;
    while !(fgets(buf.as_mut_ptr(), 1024 as libc::c_int, stream)).is_null() {
        let mut c: libc::c_int = 0;
        if *buf.as_mut_ptr() as libc::c_int == '%' as i32 {
            continue;
        }
        c = sscanf(
            buf.as_mut_ptr(),
            b"%u %u %u\0" as *const u8 as *const libc::c_char,
            &mut size1 as *mut libc::c_uint,
            &mut size2 as *mut libc::c_uint,
            &mut nz as *mut libc::c_uint,
        );
        if !(c == 3 as libc::c_int) {
            continue;
        }
        found_header = 1 as libc::c_int;
        break;
    }
    if found_header == 0 {
        gsl_error(
            b"fscanf failed reading header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            227 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_ushort;
    }
    m = gsl_spmatrix_ushort_alloc_nzmax(
        size1 as size_t,
        size2 as size_t,
        nz as size_t,
        GSL_SPMATRIX_COO as libc::c_int,
    );
    if m.is_null() {
        gsl_error(
            b"error allocating m\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            233 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_ushort;
    }
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut tmp: libc::c_ushort = 0;
    while !(fgets(buf.as_mut_ptr(), 1024 as libc::c_int, stream)).is_null() {
        let mut c_0: libc::c_int = sscanf(
            buf.as_mut_ptr(),
            b"%u %u %hu\0" as *const u8 as *const libc::c_char,
            &mut i as *mut libc::c_uint,
            &mut j as *mut libc::c_uint,
            &mut tmp as *mut libc::c_ushort,
        );
        if c_0 < 3 as libc::c_int || i == 0 as libc::c_int as libc::c_uint
            || j == 0 as libc::c_int as libc::c_uint
        {
            gsl_error(
                b"error in input file format\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                275 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_ushort;
        } else if i > size1 || j > size2 {
            gsl_error(
                b"element exceeds matrix dimensions\0" as *const u8
                    as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                279 as libc::c_int,
                GSL_EBADLEN as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_ushort;
        } else {
            gsl_spmatrix_ushort_set(
                m,
                i.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                j.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                tmp,
            );
        }
    }
    return m;
}
pub unsafe extern "C" fn gsl_spmatrix_long_fscanf(
    mut stream: *mut FILE,
) -> *mut gsl_spmatrix_long {
    let mut m: *mut gsl_spmatrix_long = 0 as *mut gsl_spmatrix_long;
    let mut size1: libc::c_uint = 0;
    let mut size2: libc::c_uint = 0;
    let mut nz: libc::c_uint = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut found_header: libc::c_int = 0 as libc::c_int;
    while !(fgets(buf.as_mut_ptr(), 1024 as libc::c_int, stream)).is_null() {
        let mut c: libc::c_int = 0;
        if *buf.as_mut_ptr() as libc::c_int == '%' as i32 {
            continue;
        }
        c = sscanf(
            buf.as_mut_ptr(),
            b"%u %u %u\0" as *const u8 as *const libc::c_char,
            &mut size1 as *mut libc::c_uint,
            &mut size2 as *mut libc::c_uint,
            &mut nz as *mut libc::c_uint,
        );
        if !(c == 3 as libc::c_int) {
            continue;
        }
        found_header = 1 as libc::c_int;
        break;
    }
    if found_header == 0 {
        gsl_error(
            b"fscanf failed reading header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            227 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_long;
    }
    m = gsl_spmatrix_long_alloc_nzmax(
        size1 as size_t,
        size2 as size_t,
        nz as size_t,
        GSL_SPMATRIX_COO as libc::c_int,
    );
    if m.is_null() {
        gsl_error(
            b"error allocating m\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            233 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_long;
    }
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut tmp: libc::c_long = 0;
    while !(fgets(buf.as_mut_ptr(), 1024 as libc::c_int, stream)).is_null() {
        let mut c_0: libc::c_int = sscanf(
            buf.as_mut_ptr(),
            b"%u %u %ld\0" as *const u8 as *const libc::c_char,
            &mut i as *mut libc::c_uint,
            &mut j as *mut libc::c_uint,
            &mut tmp as *mut libc::c_long,
        );
        if c_0 < 3 as libc::c_int || i == 0 as libc::c_int as libc::c_uint
            || j == 0 as libc::c_int as libc::c_uint
        {
            gsl_error(
                b"error in input file format\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                275 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_long;
        } else if i > size1 || j > size2 {
            gsl_error(
                b"element exceeds matrix dimensions\0" as *const u8
                    as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                279 as libc::c_int,
                GSL_EBADLEN as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_long;
        } else {
            gsl_spmatrix_long_set(
                m,
                i.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                j.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                tmp,
            );
        }
    }
    return m;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_fscanf(
    mut stream: *mut FILE,
) -> *mut gsl_spmatrix_complex {
    let mut m: *mut gsl_spmatrix_complex = 0 as *mut gsl_spmatrix_complex;
    let mut size1: libc::c_uint = 0;
    let mut size2: libc::c_uint = 0;
    let mut nz: libc::c_uint = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut found_header: libc::c_int = 0 as libc::c_int;
    while !(fgets(buf.as_mut_ptr(), 1024 as libc::c_int, stream)).is_null() {
        let mut c: libc::c_int = 0;
        if *buf.as_mut_ptr() as libc::c_int == '%' as i32 {
            continue;
        }
        c = sscanf(
            buf.as_mut_ptr(),
            b"%u %u %u\0" as *const u8 as *const libc::c_char,
            &mut size1 as *mut libc::c_uint,
            &mut size2 as *mut libc::c_uint,
            &mut nz as *mut libc::c_uint,
        );
        if !(c == 3 as libc::c_int) {
            continue;
        }
        found_header = 1 as libc::c_int;
        break;
    }
    if found_header == 0 {
        gsl_error(
            b"fscanf failed reading header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            227 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_complex;
    }
    m = gsl_spmatrix_complex_alloc_nzmax(
        size1 as size_t,
        size2 as size_t,
        nz as size_t,
        GSL_SPMATRIX_COO as libc::c_int,
    );
    if m.is_null() {
        gsl_error(
            b"error allocating m\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            233 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_complex;
    }
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut xr: libc::c_double = 0.;
    let mut xi: libc::c_double = 0.;
    let mut x: gsl_complex = gsl_complex { dat: [0.; 2] };
    while !(fgets(buf.as_mut_ptr(), 1024 as libc::c_int, stream)).is_null() {
        let mut c_0: libc::c_int = sscanf(
            buf.as_mut_ptr(),
            b"%u %u %lg %lg\0" as *const u8 as *const libc::c_char,
            &mut i as *mut libc::c_uint,
            &mut j as *mut libc::c_uint,
            &mut xr as *mut libc::c_double,
            &mut xi as *mut libc::c_double,
        );
        if c_0 < 4 as libc::c_int || i == 0 as libc::c_int as libc::c_uint
            || j == 0 as libc::c_int as libc::c_uint
        {
            gsl_error(
                b"error in input file format\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                248 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_complex;
        } else if i > size1 || j > size2 {
            gsl_error(
                b"element exceeds matrix dimensions\0" as *const u8
                    as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                252 as libc::c_int,
                GSL_EBADLEN as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_complex;
        } else {
            x.dat[0 as libc::c_int as usize] = xr;
            x.dat[1 as libc::c_int as usize] = xi;
            gsl_spmatrix_complex_set(
                m,
                i.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                j.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                x,
            );
        }
    }
    return m;
}
pub unsafe extern "C" fn gsl_spmatrix_ulong_fscanf(
    mut stream: *mut FILE,
) -> *mut gsl_spmatrix_ulong {
    let mut m: *mut gsl_spmatrix_ulong = 0 as *mut gsl_spmatrix_ulong;
    let mut size1: libc::c_uint = 0;
    let mut size2: libc::c_uint = 0;
    let mut nz: libc::c_uint = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut found_header: libc::c_int = 0 as libc::c_int;
    while !(fgets(buf.as_mut_ptr(), 1024 as libc::c_int, stream)).is_null() {
        let mut c: libc::c_int = 0;
        if *buf.as_mut_ptr() as libc::c_int == '%' as i32 {
            continue;
        }
        c = sscanf(
            buf.as_mut_ptr(),
            b"%u %u %u\0" as *const u8 as *const libc::c_char,
            &mut size1 as *mut libc::c_uint,
            &mut size2 as *mut libc::c_uint,
            &mut nz as *mut libc::c_uint,
        );
        if !(c == 3 as libc::c_int) {
            continue;
        }
        found_header = 1 as libc::c_int;
        break;
    }
    if found_header == 0 {
        gsl_error(
            b"fscanf failed reading header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            227 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_ulong;
    }
    m = gsl_spmatrix_ulong_alloc_nzmax(
        size1 as size_t,
        size2 as size_t,
        nz as size_t,
        GSL_SPMATRIX_COO as libc::c_int,
    );
    if m.is_null() {
        gsl_error(
            b"error allocating m\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            233 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix_ulong;
    }
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut tmp: libc::c_ulong = 0;
    while !(fgets(buf.as_mut_ptr(), 1024 as libc::c_int, stream)).is_null() {
        let mut c_0: libc::c_int = sscanf(
            buf.as_mut_ptr(),
            b"%u %u %lu\0" as *const u8 as *const libc::c_char,
            &mut i as *mut libc::c_uint,
            &mut j as *mut libc::c_uint,
            &mut tmp as *mut libc::c_ulong,
        );
        if c_0 < 3 as libc::c_int || i == 0 as libc::c_int as libc::c_uint
            || j == 0 as libc::c_int as libc::c_uint
        {
            gsl_error(
                b"error in input file format\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                275 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_ulong;
        } else if i > size1 || j > size2 {
            gsl_error(
                b"element exceeds matrix dimensions\0" as *const u8
                    as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                279 as libc::c_int,
                GSL_EBADLEN as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix_ulong;
        } else {
            gsl_spmatrix_ulong_set(
                m,
                i.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                j.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                tmp,
            );
        }
    }
    return m;
}
pub unsafe extern "C" fn gsl_spmatrix_fscanf(
    mut stream: *mut FILE,
) -> *mut gsl_spmatrix {
    let mut m: *mut gsl_spmatrix = 0 as *mut gsl_spmatrix;
    let mut size1: libc::c_uint = 0;
    let mut size2: libc::c_uint = 0;
    let mut nz: libc::c_uint = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut found_header: libc::c_int = 0 as libc::c_int;
    while !(fgets(buf.as_mut_ptr(), 1024 as libc::c_int, stream)).is_null() {
        let mut c: libc::c_int = 0;
        if *buf.as_mut_ptr() as libc::c_int == '%' as i32 {
            continue;
        }
        c = sscanf(
            buf.as_mut_ptr(),
            b"%u %u %u\0" as *const u8 as *const libc::c_char,
            &mut size1 as *mut libc::c_uint,
            &mut size2 as *mut libc::c_uint,
            &mut nz as *mut libc::c_uint,
        );
        if !(c == 3 as libc::c_int) {
            continue;
        }
        found_header = 1 as libc::c_int;
        break;
    }
    if found_header == 0 {
        gsl_error(
            b"fscanf failed reading header\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            227 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix;
    }
    m = gsl_spmatrix_alloc_nzmax(
        size1 as size_t,
        size2 as size_t,
        nz as size_t,
        GSL_SPMATRIX_COO as libc::c_int,
    );
    if m.is_null() {
        gsl_error(
            b"error allocating m\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            233 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_spmatrix;
    }
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut tmp: libc::c_double = 0.;
    while !(fgets(buf.as_mut_ptr(), 1024 as libc::c_int, stream)).is_null() {
        let mut c_0: libc::c_int = sscanf(
            buf.as_mut_ptr(),
            b"%u %u %lg\0" as *const u8 as *const libc::c_char,
            &mut i as *mut libc::c_uint,
            &mut j as *mut libc::c_uint,
            &mut tmp as *mut libc::c_double,
        );
        if c_0 < 3 as libc::c_int || i == 0 as libc::c_int as libc::c_uint
            || j == 0 as libc::c_int as libc::c_uint
        {
            gsl_error(
                b"error in input file format\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                275 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix;
        } else if i > size1 || j > size2 {
            gsl_error(
                b"element exceeds matrix dimensions\0" as *const u8
                    as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                279 as libc::c_int,
                GSL_EBADLEN as libc::c_int,
            );
            return 0 as *mut gsl_spmatrix;
        } else {
            gsl_spmatrix_set(
                m,
                i.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                j.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                tmp,
            );
        }
    }
    return m;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_float_fwrite(
    mut stream: *mut FILE,
    mut m: *const gsl_spmatrix_complex_float,
) -> libc::c_int {
    let mut items: size_t = 0;
    items = fwrite(
        &(*m).size1 as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            304 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        &(*m).size2 as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            310 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        &(*m).nz as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on nz\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            316 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        (*m).i as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        (*m).nz,
        stream,
    );
    if items != (*m).nz {
        gsl_error(
            b"fwrite failed on row indices\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            324 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        (*m).data as *const libc::c_void,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
        (*m).nz,
        stream,
    );
    if items != (*m).nz {
        gsl_error(
            b"fwrite failed on data\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            330 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            (*m).nz,
            stream,
        );
        if items != (*m).nz {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                338 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ((*m).size2).wrapping_add(1 as libc::c_int as libc::c_ulong),
            stream,
        );
        if items != ((*m).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                346 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ((*m).size1).wrapping_add(1 as libc::c_int as libc::c_ulong),
            stream,
        );
        if items != ((*m).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                354 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_fwrite(
    mut stream: *mut FILE,
    mut m: *const gsl_spmatrix_long_double,
) -> libc::c_int {
    let mut items: size_t = 0;
    items = fwrite(
        &(*m).size1 as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            304 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        &(*m).size2 as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            310 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        &(*m).nz as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on nz\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            316 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        (*m).i as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        (*m).nz,
        stream,
    );
    if items != (*m).nz {
        gsl_error(
            b"fwrite failed on row indices\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            324 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        (*m).data as *const libc::c_void,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<f128::f128>() as libc::c_ulong),
        (*m).nz,
        stream,
    );
    if items != (*m).nz {
        gsl_error(
            b"fwrite failed on data\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            330 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            (*m).nz,
            stream,
        );
        if items != (*m).nz {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                338 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ((*m).size2).wrapping_add(1 as libc::c_int as libc::c_ulong),
            stream,
        );
        if items != ((*m).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                346 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ((*m).size1).wrapping_add(1 as libc::c_int as libc::c_ulong),
            stream,
        );
        if items != ((*m).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                354 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_char_fwrite(
    mut stream: *mut FILE,
    mut m: *const gsl_spmatrix_char,
) -> libc::c_int {
    let mut items: size_t = 0;
    items = fwrite(
        &(*m).size1 as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            304 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        &(*m).size2 as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            310 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        &(*m).nz as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on nz\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            316 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        (*m).i as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        (*m).nz,
        stream,
    );
    if items != (*m).nz {
        gsl_error(
            b"fwrite failed on row indices\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            324 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        (*m).data as *const libc::c_void,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
        (*m).nz,
        stream,
    );
    if items != (*m).nz {
        gsl_error(
            b"fwrite failed on data\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            330 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            (*m).nz,
            stream,
        );
        if items != (*m).nz {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                338 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ((*m).size2).wrapping_add(1 as libc::c_int as libc::c_ulong),
            stream,
        );
        if items != ((*m).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                346 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ((*m).size1).wrapping_add(1 as libc::c_int as libc::c_ulong),
            stream,
        );
        if items != ((*m).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                354 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_long_double_fwrite(
    mut stream: *mut FILE,
    mut m: *const gsl_spmatrix_complex_long_double,
) -> libc::c_int {
    let mut items: size_t = 0;
    items = fwrite(
        &(*m).size1 as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            304 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        &(*m).size2 as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            310 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        &(*m).nz as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on nz\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            316 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        (*m).i as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        (*m).nz,
        stream,
    );
    if items != (*m).nz {
        gsl_error(
            b"fwrite failed on row indices\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            324 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        (*m).data as *const libc::c_void,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<f128::f128>() as libc::c_ulong),
        (*m).nz,
        stream,
    );
    if items != (*m).nz {
        gsl_error(
            b"fwrite failed on data\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            330 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            (*m).nz,
            stream,
        );
        if items != (*m).nz {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                338 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ((*m).size2).wrapping_add(1 as libc::c_int as libc::c_ulong),
            stream,
        );
        if items != ((*m).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                346 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ((*m).size1).wrapping_add(1 as libc::c_int as libc::c_ulong),
            stream,
        );
        if items != ((*m).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                354 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_ulong_fwrite(
    mut stream: *mut FILE,
    mut m: *const gsl_spmatrix_ulong,
) -> libc::c_int {
    let mut items: size_t = 0;
    items = fwrite(
        &(*m).size1 as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            304 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        &(*m).size2 as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            310 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        &(*m).nz as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on nz\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            316 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        (*m).i as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        (*m).nz,
        stream,
    );
    if items != (*m).nz {
        gsl_error(
            b"fwrite failed on row indices\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            324 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        (*m).data as *const libc::c_void,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong),
        (*m).nz,
        stream,
    );
    if items != (*m).nz {
        gsl_error(
            b"fwrite failed on data\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            330 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            (*m).nz,
            stream,
        );
        if items != (*m).nz {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                338 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ((*m).size2).wrapping_add(1 as libc::c_int as libc::c_ulong),
            stream,
        );
        if items != ((*m).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                346 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ((*m).size1).wrapping_add(1 as libc::c_int as libc::c_ulong),
            stream,
        );
        if items != ((*m).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                354 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_long_fwrite(
    mut stream: *mut FILE,
    mut m: *const gsl_spmatrix_long,
) -> libc::c_int {
    let mut items: size_t = 0;
    items = fwrite(
        &(*m).size1 as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            304 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        &(*m).size2 as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            310 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        &(*m).nz as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on nz\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            316 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        (*m).i as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        (*m).nz,
        stream,
    );
    if items != (*m).nz {
        gsl_error(
            b"fwrite failed on row indices\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            324 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        (*m).data as *const libc::c_void,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_long>() as libc::c_ulong),
        (*m).nz,
        stream,
    );
    if items != (*m).nz {
        gsl_error(
            b"fwrite failed on data\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            330 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            (*m).nz,
            stream,
        );
        if items != (*m).nz {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                338 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ((*m).size2).wrapping_add(1 as libc::c_int as libc::c_ulong),
            stream,
        );
        if items != ((*m).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                346 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ((*m).size1).wrapping_add(1 as libc::c_int as libc::c_ulong),
            stream,
        );
        if items != ((*m).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                354 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_fwrite(
    mut stream: *mut FILE,
    mut m: *const gsl_spmatrix_complex,
) -> libc::c_int {
    let mut items: size_t = 0;
    items = fwrite(
        &(*m).size1 as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            304 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        &(*m).size2 as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            310 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        &(*m).nz as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on nz\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            316 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        (*m).i as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        (*m).nz,
        stream,
    );
    if items != (*m).nz {
        gsl_error(
            b"fwrite failed on row indices\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            324 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        (*m).data as *const libc::c_void,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        (*m).nz,
        stream,
    );
    if items != (*m).nz {
        gsl_error(
            b"fwrite failed on data\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            330 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            (*m).nz,
            stream,
        );
        if items != (*m).nz {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                338 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ((*m).size2).wrapping_add(1 as libc::c_int as libc::c_ulong),
            stream,
        );
        if items != ((*m).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                346 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ((*m).size1).wrapping_add(1 as libc::c_int as libc::c_ulong),
            stream,
        );
        if items != ((*m).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                354 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_short_fwrite(
    mut stream: *mut FILE,
    mut m: *const gsl_spmatrix_short,
) -> libc::c_int {
    let mut items: size_t = 0;
    items = fwrite(
        &(*m).size1 as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            304 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        &(*m).size2 as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            310 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        &(*m).nz as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on nz\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            316 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        (*m).i as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        (*m).nz,
        stream,
    );
    if items != (*m).nz {
        gsl_error(
            b"fwrite failed on row indices\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            324 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        (*m).data as *const libc::c_void,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_short>() as libc::c_ulong),
        (*m).nz,
        stream,
    );
    if items != (*m).nz {
        gsl_error(
            b"fwrite failed on data\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            330 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            (*m).nz,
            stream,
        );
        if items != (*m).nz {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                338 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ((*m).size2).wrapping_add(1 as libc::c_int as libc::c_ulong),
            stream,
        );
        if items != ((*m).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                346 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ((*m).size1).wrapping_add(1 as libc::c_int as libc::c_ulong),
            stream,
        );
        if items != ((*m).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                354 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_float_fwrite(
    mut stream: *mut FILE,
    mut m: *const gsl_spmatrix_float,
) -> libc::c_int {
    let mut items: size_t = 0;
    items = fwrite(
        &(*m).size1 as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            304 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        &(*m).size2 as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            310 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        &(*m).nz as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on nz\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            316 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        (*m).i as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        (*m).nz,
        stream,
    );
    if items != (*m).nz {
        gsl_error(
            b"fwrite failed on row indices\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            324 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        (*m).data as *const libc::c_void,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
        (*m).nz,
        stream,
    );
    if items != (*m).nz {
        gsl_error(
            b"fwrite failed on data\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            330 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            (*m).nz,
            stream,
        );
        if items != (*m).nz {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                338 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ((*m).size2).wrapping_add(1 as libc::c_int as libc::c_ulong),
            stream,
        );
        if items != ((*m).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                346 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ((*m).size1).wrapping_add(1 as libc::c_int as libc::c_ulong),
            stream,
        );
        if items != ((*m).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                354 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_int_fwrite(
    mut stream: *mut FILE,
    mut m: *const gsl_spmatrix_int,
) -> libc::c_int {
    let mut items: size_t = 0;
    items = fwrite(
        &(*m).size1 as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            304 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        &(*m).size2 as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            310 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        &(*m).nz as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on nz\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            316 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        (*m).i as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        (*m).nz,
        stream,
    );
    if items != (*m).nz {
        gsl_error(
            b"fwrite failed on row indices\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            324 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        (*m).data as *const libc::c_void,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        (*m).nz,
        stream,
    );
    if items != (*m).nz {
        gsl_error(
            b"fwrite failed on data\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            330 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            (*m).nz,
            stream,
        );
        if items != (*m).nz {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                338 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ((*m).size2).wrapping_add(1 as libc::c_int as libc::c_ulong),
            stream,
        );
        if items != ((*m).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                346 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ((*m).size1).wrapping_add(1 as libc::c_int as libc::c_ulong),
            stream,
        );
        if items != ((*m).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                354 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_ushort_fwrite(
    mut stream: *mut FILE,
    mut m: *const gsl_spmatrix_ushort,
) -> libc::c_int {
    let mut items: size_t = 0;
    items = fwrite(
        &(*m).size1 as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            304 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        &(*m).size2 as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            310 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        &(*m).nz as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on nz\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            316 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        (*m).i as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        (*m).nz,
        stream,
    );
    if items != (*m).nz {
        gsl_error(
            b"fwrite failed on row indices\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            324 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        (*m).data as *const libc::c_void,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong),
        (*m).nz,
        stream,
    );
    if items != (*m).nz {
        gsl_error(
            b"fwrite failed on data\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            330 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            (*m).nz,
            stream,
        );
        if items != (*m).nz {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                338 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ((*m).size2).wrapping_add(1 as libc::c_int as libc::c_ulong),
            stream,
        );
        if items != ((*m).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                346 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ((*m).size1).wrapping_add(1 as libc::c_int as libc::c_ulong),
            stream,
        );
        if items != ((*m).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                354 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_uint_fwrite(
    mut stream: *mut FILE,
    mut m: *const gsl_spmatrix_uint,
) -> libc::c_int {
    let mut items: size_t = 0;
    items = fwrite(
        &(*m).size1 as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            304 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        &(*m).size2 as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            310 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        &(*m).nz as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on nz\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            316 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        (*m).i as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        (*m).nz,
        stream,
    );
    if items != (*m).nz {
        gsl_error(
            b"fwrite failed on row indices\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            324 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        (*m).data as *const libc::c_void,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong),
        (*m).nz,
        stream,
    );
    if items != (*m).nz {
        gsl_error(
            b"fwrite failed on data\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            330 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            (*m).nz,
            stream,
        );
        if items != (*m).nz {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                338 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ((*m).size2).wrapping_add(1 as libc::c_int as libc::c_ulong),
            stream,
        );
        if items != ((*m).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                346 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ((*m).size1).wrapping_add(1 as libc::c_int as libc::c_ulong),
            stream,
        );
        if items != ((*m).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                354 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_uchar_fwrite(
    mut stream: *mut FILE,
    mut m: *const gsl_spmatrix_uchar,
) -> libc::c_int {
    let mut items: size_t = 0;
    items = fwrite(
        &(*m).size1 as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            304 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        &(*m).size2 as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            310 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        &(*m).nz as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on nz\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            316 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        (*m).i as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        (*m).nz,
        stream,
    );
    if items != (*m).nz {
        gsl_error(
            b"fwrite failed on row indices\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            324 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        (*m).data as *const libc::c_void,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
        (*m).nz,
        stream,
    );
    if items != (*m).nz {
        gsl_error(
            b"fwrite failed on data\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            330 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            (*m).nz,
            stream,
        );
        if items != (*m).nz {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                338 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ((*m).size2).wrapping_add(1 as libc::c_int as libc::c_ulong),
            stream,
        );
        if items != ((*m).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                346 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ((*m).size1).wrapping_add(1 as libc::c_int as libc::c_ulong),
            stream,
        );
        if items != ((*m).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                354 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_fwrite(
    mut stream: *mut FILE,
    mut m: *const gsl_spmatrix,
) -> libc::c_int {
    let mut items: size_t = 0;
    items = fwrite(
        &(*m).size1 as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            304 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        &(*m).size2 as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            310 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        &(*m).nz as *const size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fwrite failed on nz\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            316 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        (*m).i as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        (*m).nz,
        stream,
    );
    if items != (*m).nz {
        gsl_error(
            b"fwrite failed on row indices\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            324 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fwrite(
        (*m).data as *const libc::c_void,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        (*m).nz,
        stream,
    );
    if items != (*m).nz {
        gsl_error(
            b"fwrite failed on data\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            330 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            (*m).nz,
            stream,
        );
        if items != (*m).nz {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                338 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ((*m).size2).wrapping_add(1 as libc::c_int as libc::c_ulong),
            stream,
        );
        if items != ((*m).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                346 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
        items = fwrite(
            (*m).p as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ((*m).size1).wrapping_add(1 as libc::c_int as libc::c_ulong),
            stream,
        );
        if items != ((*m).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
            gsl_error(
                b"fwrite failed on column indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                354 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_float_fread(
    mut stream: *mut FILE,
    mut m: *mut gsl_spmatrix_complex_float,
) -> libc::c_int {
    let mut size1: size_t = 0;
    let mut size2: size_t = 0;
    let mut nz: size_t = 0;
    let mut items: size_t = 0;
    items = fread(
        &mut size1 as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            372 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fread(
        &mut size2 as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            378 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fread(
        &mut nz as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on nz\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            384 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).size1 != size1 {
        gsl_error(
            b"matrix has wrong size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            389 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*m).size2 != size2 {
        gsl_error(
            b"matrix has wrong size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            393 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if nz > (*m).nzmax {
        gsl_error(
            b"matrix nzmax is too small\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            397 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        items = fread(
            (*m).i as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            nz,
            stream,
        );
        if items != nz {
            gsl_error(
                b"fread failed on row indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                406 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        items = fread(
            (*m).data as *mut libc::c_void,
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
            nz,
            stream,
        );
        if items != nz {
            gsl_error(
                b"fread failed on data\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                412 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        (*m).nz = nz;
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                nz,
                stream,
            );
            if items != nz {
                gsl_error(
                    b"fread failed on column indices\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    422 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            gsl_spmatrix_complex_float_tree_rebuild(m);
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                size2.wrapping_add(1 as libc::c_int as libc::c_ulong),
                stream,
            );
            if items != size2.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                gsl_error(
                    b"fread failed on row pointers\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    433 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                size1.wrapping_add(1 as libc::c_int as libc::c_ulong),
                stream,
            );
            if items != size1.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                gsl_error(
                    b"fread failed on column pointers\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    441 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_uint_fread(
    mut stream: *mut FILE,
    mut m: *mut gsl_spmatrix_uint,
) -> libc::c_int {
    let mut size1: size_t = 0;
    let mut size2: size_t = 0;
    let mut nz: size_t = 0;
    let mut items: size_t = 0;
    items = fread(
        &mut size1 as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            372 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fread(
        &mut size2 as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            378 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fread(
        &mut nz as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on nz\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            384 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).size1 != size1 {
        gsl_error(
            b"matrix has wrong size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            389 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*m).size2 != size2 {
        gsl_error(
            b"matrix has wrong size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            393 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if nz > (*m).nzmax {
        gsl_error(
            b"matrix nzmax is too small\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            397 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        items = fread(
            (*m).i as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            nz,
            stream,
        );
        if items != nz {
            gsl_error(
                b"fread failed on row indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                406 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        items = fread(
            (*m).data as *mut libc::c_void,
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong),
            nz,
            stream,
        );
        if items != nz {
            gsl_error(
                b"fread failed on data\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                412 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        (*m).nz = nz;
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                nz,
                stream,
            );
            if items != nz {
                gsl_error(
                    b"fread failed on column indices\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    422 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            gsl_spmatrix_uint_tree_rebuild(m);
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                size2.wrapping_add(1 as libc::c_int as libc::c_ulong),
                stream,
            );
            if items != size2.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                gsl_error(
                    b"fread failed on row pointers\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    433 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                size1.wrapping_add(1 as libc::c_int as libc::c_ulong),
                stream,
            );
            if items != size1.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                gsl_error(
                    b"fread failed on column pointers\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    441 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_fread(
    mut stream: *mut FILE,
    mut m: *mut gsl_spmatrix_long_double,
) -> libc::c_int {
    let mut size1: size_t = 0;
    let mut size2: size_t = 0;
    let mut nz: size_t = 0;
    let mut items: size_t = 0;
    items = fread(
        &mut size1 as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            372 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fread(
        &mut size2 as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            378 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fread(
        &mut nz as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on nz\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            384 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).size1 != size1 {
        gsl_error(
            b"matrix has wrong size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            389 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*m).size2 != size2 {
        gsl_error(
            b"matrix has wrong size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            393 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if nz > (*m).nzmax {
        gsl_error(
            b"matrix nzmax is too small\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            397 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        items = fread(
            (*m).i as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            nz,
            stream,
        );
        if items != nz {
            gsl_error(
                b"fread failed on row indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                406 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        items = fread(
            (*m).data as *mut libc::c_void,
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<f128::f128>() as libc::c_ulong),
            nz,
            stream,
        );
        if items != nz {
            gsl_error(
                b"fread failed on data\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                412 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        (*m).nz = nz;
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                nz,
                stream,
            );
            if items != nz {
                gsl_error(
                    b"fread failed on column indices\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    422 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            gsl_spmatrix_long_double_tree_rebuild(m);
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                size2.wrapping_add(1 as libc::c_int as libc::c_ulong),
                stream,
            );
            if items != size2.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                gsl_error(
                    b"fread failed on row pointers\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    433 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                size1.wrapping_add(1 as libc::c_int as libc::c_ulong),
                stream,
            );
            if items != size1.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                gsl_error(
                    b"fread failed on column pointers\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    441 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_short_fread(
    mut stream: *mut FILE,
    mut m: *mut gsl_spmatrix_short,
) -> libc::c_int {
    let mut size1: size_t = 0;
    let mut size2: size_t = 0;
    let mut nz: size_t = 0;
    let mut items: size_t = 0;
    items = fread(
        &mut size1 as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            372 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fread(
        &mut size2 as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            378 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fread(
        &mut nz as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on nz\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            384 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).size1 != size1 {
        gsl_error(
            b"matrix has wrong size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            389 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*m).size2 != size2 {
        gsl_error(
            b"matrix has wrong size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            393 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if nz > (*m).nzmax {
        gsl_error(
            b"matrix nzmax is too small\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            397 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        items = fread(
            (*m).i as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            nz,
            stream,
        );
        if items != nz {
            gsl_error(
                b"fread failed on row indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                406 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        items = fread(
            (*m).data as *mut libc::c_void,
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_short>() as libc::c_ulong),
            nz,
            stream,
        );
        if items != nz {
            gsl_error(
                b"fread failed on data\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                412 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        (*m).nz = nz;
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                nz,
                stream,
            );
            if items != nz {
                gsl_error(
                    b"fread failed on column indices\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    422 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            gsl_spmatrix_short_tree_rebuild(m);
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                size2.wrapping_add(1 as libc::c_int as libc::c_ulong),
                stream,
            );
            if items != size2.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                gsl_error(
                    b"fread failed on row pointers\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    433 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                size1.wrapping_add(1 as libc::c_int as libc::c_ulong),
                stream,
            );
            if items != size1.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                gsl_error(
                    b"fread failed on column pointers\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    441 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_fread(
    mut stream: *mut FILE,
    mut m: *mut gsl_spmatrix,
) -> libc::c_int {
    let mut size1: size_t = 0;
    let mut size2: size_t = 0;
    let mut nz: size_t = 0;
    let mut items: size_t = 0;
    items = fread(
        &mut size1 as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            372 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fread(
        &mut size2 as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            378 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fread(
        &mut nz as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on nz\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            384 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).size1 != size1 {
        gsl_error(
            b"matrix has wrong size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            389 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*m).size2 != size2 {
        gsl_error(
            b"matrix has wrong size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            393 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if nz > (*m).nzmax {
        gsl_error(
            b"matrix nzmax is too small\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            397 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        items = fread(
            (*m).i as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            nz,
            stream,
        );
        if items != nz {
            gsl_error(
                b"fread failed on row indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                406 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        items = fread(
            (*m).data as *mut libc::c_void,
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
            nz,
            stream,
        );
        if items != nz {
            gsl_error(
                b"fread failed on data\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                412 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        (*m).nz = nz;
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                nz,
                stream,
            );
            if items != nz {
                gsl_error(
                    b"fread failed on column indices\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    422 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            gsl_spmatrix_tree_rebuild(m);
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                size2.wrapping_add(1 as libc::c_int as libc::c_ulong),
                stream,
            );
            if items != size2.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                gsl_error(
                    b"fread failed on row pointers\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    433 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                size1.wrapping_add(1 as libc::c_int as libc::c_ulong),
                stream,
            );
            if items != size1.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                gsl_error(
                    b"fread failed on column pointers\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    441 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_int_fread(
    mut stream: *mut FILE,
    mut m: *mut gsl_spmatrix_int,
) -> libc::c_int {
    let mut size1: size_t = 0;
    let mut size2: size_t = 0;
    let mut nz: size_t = 0;
    let mut items: size_t = 0;
    items = fread(
        &mut size1 as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            372 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fread(
        &mut size2 as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            378 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fread(
        &mut nz as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on nz\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            384 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).size1 != size1 {
        gsl_error(
            b"matrix has wrong size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            389 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*m).size2 != size2 {
        gsl_error(
            b"matrix has wrong size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            393 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if nz > (*m).nzmax {
        gsl_error(
            b"matrix nzmax is too small\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            397 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        items = fread(
            (*m).i as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            nz,
            stream,
        );
        if items != nz {
            gsl_error(
                b"fread failed on row indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                406 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        items = fread(
            (*m).data as *mut libc::c_void,
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
            nz,
            stream,
        );
        if items != nz {
            gsl_error(
                b"fread failed on data\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                412 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        (*m).nz = nz;
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                nz,
                stream,
            );
            if items != nz {
                gsl_error(
                    b"fread failed on column indices\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    422 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            gsl_spmatrix_int_tree_rebuild(m);
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                size2.wrapping_add(1 as libc::c_int as libc::c_ulong),
                stream,
            );
            if items != size2.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                gsl_error(
                    b"fread failed on row pointers\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    433 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                size1.wrapping_add(1 as libc::c_int as libc::c_ulong),
                stream,
            );
            if items != size1.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                gsl_error(
                    b"fread failed on column pointers\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    441 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_uchar_fread(
    mut stream: *mut FILE,
    mut m: *mut gsl_spmatrix_uchar,
) -> libc::c_int {
    let mut size1: size_t = 0;
    let mut size2: size_t = 0;
    let mut nz: size_t = 0;
    let mut items: size_t = 0;
    items = fread(
        &mut size1 as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            372 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fread(
        &mut size2 as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            378 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fread(
        &mut nz as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on nz\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            384 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).size1 != size1 {
        gsl_error(
            b"matrix has wrong size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            389 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*m).size2 != size2 {
        gsl_error(
            b"matrix has wrong size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            393 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if nz > (*m).nzmax {
        gsl_error(
            b"matrix nzmax is too small\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            397 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        items = fread(
            (*m).i as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            nz,
            stream,
        );
        if items != nz {
            gsl_error(
                b"fread failed on row indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                406 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        items = fread(
            (*m).data as *mut libc::c_void,
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
            nz,
            stream,
        );
        if items != nz {
            gsl_error(
                b"fread failed on data\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                412 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        (*m).nz = nz;
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                nz,
                stream,
            );
            if items != nz {
                gsl_error(
                    b"fread failed on column indices\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    422 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            gsl_spmatrix_uchar_tree_rebuild(m);
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                size2.wrapping_add(1 as libc::c_int as libc::c_ulong),
                stream,
            );
            if items != size2.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                gsl_error(
                    b"fread failed on row pointers\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    433 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                size1.wrapping_add(1 as libc::c_int as libc::c_ulong),
                stream,
            );
            if items != size1.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                gsl_error(
                    b"fread failed on column pointers\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    441 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_ushort_fread(
    mut stream: *mut FILE,
    mut m: *mut gsl_spmatrix_ushort,
) -> libc::c_int {
    let mut size1: size_t = 0;
    let mut size2: size_t = 0;
    let mut nz: size_t = 0;
    let mut items: size_t = 0;
    items = fread(
        &mut size1 as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            372 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fread(
        &mut size2 as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            378 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fread(
        &mut nz as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on nz\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            384 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).size1 != size1 {
        gsl_error(
            b"matrix has wrong size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            389 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*m).size2 != size2 {
        gsl_error(
            b"matrix has wrong size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            393 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if nz > (*m).nzmax {
        gsl_error(
            b"matrix nzmax is too small\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            397 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        items = fread(
            (*m).i as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            nz,
            stream,
        );
        if items != nz {
            gsl_error(
                b"fread failed on row indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                406 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        items = fread(
            (*m).data as *mut libc::c_void,
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong),
            nz,
            stream,
        );
        if items != nz {
            gsl_error(
                b"fread failed on data\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                412 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        (*m).nz = nz;
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                nz,
                stream,
            );
            if items != nz {
                gsl_error(
                    b"fread failed on column indices\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    422 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            gsl_spmatrix_ushort_tree_rebuild(m);
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                size2.wrapping_add(1 as libc::c_int as libc::c_ulong),
                stream,
            );
            if items != size2.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                gsl_error(
                    b"fread failed on row pointers\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    433 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                size1.wrapping_add(1 as libc::c_int as libc::c_ulong),
                stream,
            );
            if items != size1.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                gsl_error(
                    b"fread failed on column pointers\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    441 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_char_fread(
    mut stream: *mut FILE,
    mut m: *mut gsl_spmatrix_char,
) -> libc::c_int {
    let mut size1: size_t = 0;
    let mut size2: size_t = 0;
    let mut nz: size_t = 0;
    let mut items: size_t = 0;
    items = fread(
        &mut size1 as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            372 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fread(
        &mut size2 as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            378 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fread(
        &mut nz as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on nz\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            384 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).size1 != size1 {
        gsl_error(
            b"matrix has wrong size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            389 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*m).size2 != size2 {
        gsl_error(
            b"matrix has wrong size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            393 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if nz > (*m).nzmax {
        gsl_error(
            b"matrix nzmax is too small\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            397 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        items = fread(
            (*m).i as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            nz,
            stream,
        );
        if items != nz {
            gsl_error(
                b"fread failed on row indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                406 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        items = fread(
            (*m).data as *mut libc::c_void,
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            nz,
            stream,
        );
        if items != nz {
            gsl_error(
                b"fread failed on data\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                412 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        (*m).nz = nz;
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                nz,
                stream,
            );
            if items != nz {
                gsl_error(
                    b"fread failed on column indices\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    422 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            gsl_spmatrix_char_tree_rebuild(m);
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                size2.wrapping_add(1 as libc::c_int as libc::c_ulong),
                stream,
            );
            if items != size2.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                gsl_error(
                    b"fread failed on row pointers\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    433 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                size1.wrapping_add(1 as libc::c_int as libc::c_ulong),
                stream,
            );
            if items != size1.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                gsl_error(
                    b"fread failed on column pointers\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    441 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_long_fread(
    mut stream: *mut FILE,
    mut m: *mut gsl_spmatrix_long,
) -> libc::c_int {
    let mut size1: size_t = 0;
    let mut size2: size_t = 0;
    let mut nz: size_t = 0;
    let mut items: size_t = 0;
    items = fread(
        &mut size1 as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            372 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fread(
        &mut size2 as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            378 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fread(
        &mut nz as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on nz\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            384 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).size1 != size1 {
        gsl_error(
            b"matrix has wrong size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            389 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*m).size2 != size2 {
        gsl_error(
            b"matrix has wrong size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            393 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if nz > (*m).nzmax {
        gsl_error(
            b"matrix nzmax is too small\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            397 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        items = fread(
            (*m).i as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            nz,
            stream,
        );
        if items != nz {
            gsl_error(
                b"fread failed on row indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                406 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        items = fread(
            (*m).data as *mut libc::c_void,
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_long>() as libc::c_ulong),
            nz,
            stream,
        );
        if items != nz {
            gsl_error(
                b"fread failed on data\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                412 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        (*m).nz = nz;
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                nz,
                stream,
            );
            if items != nz {
                gsl_error(
                    b"fread failed on column indices\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    422 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            gsl_spmatrix_long_tree_rebuild(m);
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                size2.wrapping_add(1 as libc::c_int as libc::c_ulong),
                stream,
            );
            if items != size2.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                gsl_error(
                    b"fread failed on row pointers\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    433 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                size1.wrapping_add(1 as libc::c_int as libc::c_ulong),
                stream,
            );
            if items != size1.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                gsl_error(
                    b"fread failed on column pointers\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    441 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_long_double_fread(
    mut stream: *mut FILE,
    mut m: *mut gsl_spmatrix_complex_long_double,
) -> libc::c_int {
    let mut size1: size_t = 0;
    let mut size2: size_t = 0;
    let mut nz: size_t = 0;
    let mut items: size_t = 0;
    items = fread(
        &mut size1 as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            372 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fread(
        &mut size2 as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            378 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fread(
        &mut nz as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on nz\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            384 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).size1 != size1 {
        gsl_error(
            b"matrix has wrong size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            389 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*m).size2 != size2 {
        gsl_error(
            b"matrix has wrong size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            393 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if nz > (*m).nzmax {
        gsl_error(
            b"matrix nzmax is too small\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            397 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        items = fread(
            (*m).i as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            nz,
            stream,
        );
        if items != nz {
            gsl_error(
                b"fread failed on row indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                406 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        items = fread(
            (*m).data as *mut libc::c_void,
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<f128::f128>() as libc::c_ulong),
            nz,
            stream,
        );
        if items != nz {
            gsl_error(
                b"fread failed on data\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                412 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        (*m).nz = nz;
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                nz,
                stream,
            );
            if items != nz {
                gsl_error(
                    b"fread failed on column indices\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    422 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            gsl_spmatrix_complex_long_double_tree_rebuild(m);
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                size2.wrapping_add(1 as libc::c_int as libc::c_ulong),
                stream,
            );
            if items != size2.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                gsl_error(
                    b"fread failed on row pointers\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    433 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                size1.wrapping_add(1 as libc::c_int as libc::c_ulong),
                stream,
            );
            if items != size1.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                gsl_error(
                    b"fread failed on column pointers\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    441 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_ulong_fread(
    mut stream: *mut FILE,
    mut m: *mut gsl_spmatrix_ulong,
) -> libc::c_int {
    let mut size1: size_t = 0;
    let mut size2: size_t = 0;
    let mut nz: size_t = 0;
    let mut items: size_t = 0;
    items = fread(
        &mut size1 as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            372 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fread(
        &mut size2 as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            378 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fread(
        &mut nz as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on nz\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            384 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).size1 != size1 {
        gsl_error(
            b"matrix has wrong size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            389 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*m).size2 != size2 {
        gsl_error(
            b"matrix has wrong size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            393 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if nz > (*m).nzmax {
        gsl_error(
            b"matrix nzmax is too small\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            397 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        items = fread(
            (*m).i as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            nz,
            stream,
        );
        if items != nz {
            gsl_error(
                b"fread failed on row indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                406 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        items = fread(
            (*m).data as *mut libc::c_void,
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong),
            nz,
            stream,
        );
        if items != nz {
            gsl_error(
                b"fread failed on data\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                412 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        (*m).nz = nz;
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                nz,
                stream,
            );
            if items != nz {
                gsl_error(
                    b"fread failed on column indices\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    422 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            gsl_spmatrix_ulong_tree_rebuild(m);
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                size2.wrapping_add(1 as libc::c_int as libc::c_ulong),
                stream,
            );
            if items != size2.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                gsl_error(
                    b"fread failed on row pointers\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    433 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                size1.wrapping_add(1 as libc::c_int as libc::c_ulong),
                stream,
            );
            if items != size1.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                gsl_error(
                    b"fread failed on column pointers\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    441 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_float_fread(
    mut stream: *mut FILE,
    mut m: *mut gsl_spmatrix_float,
) -> libc::c_int {
    let mut size1: size_t = 0;
    let mut size2: size_t = 0;
    let mut nz: size_t = 0;
    let mut items: size_t = 0;
    items = fread(
        &mut size1 as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            372 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fread(
        &mut size2 as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            378 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fread(
        &mut nz as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on nz\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            384 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).size1 != size1 {
        gsl_error(
            b"matrix has wrong size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            389 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*m).size2 != size2 {
        gsl_error(
            b"matrix has wrong size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            393 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if nz > (*m).nzmax {
        gsl_error(
            b"matrix nzmax is too small\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            397 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        items = fread(
            (*m).i as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            nz,
            stream,
        );
        if items != nz {
            gsl_error(
                b"fread failed on row indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                406 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        items = fread(
            (*m).data as *mut libc::c_void,
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
            nz,
            stream,
        );
        if items != nz {
            gsl_error(
                b"fread failed on data\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                412 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        (*m).nz = nz;
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                nz,
                stream,
            );
            if items != nz {
                gsl_error(
                    b"fread failed on column indices\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    422 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            gsl_spmatrix_float_tree_rebuild(m);
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                size2.wrapping_add(1 as libc::c_int as libc::c_ulong),
                stream,
            );
            if items != size2.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                gsl_error(
                    b"fread failed on row pointers\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    433 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                size1.wrapping_add(1 as libc::c_int as libc::c_ulong),
                stream,
            );
            if items != size1.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                gsl_error(
                    b"fread failed on column pointers\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    441 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_spmatrix_complex_fread(
    mut stream: *mut FILE,
    mut m: *mut gsl_spmatrix_complex,
) -> libc::c_int {
    let mut size1: size_t = 0;
    let mut size2: size_t = 0;
    let mut nz: size_t = 0;
    let mut items: size_t = 0;
    items = fread(
        &mut size1 as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            372 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fread(
        &mut size2 as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            378 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    items = fread(
        &mut nz as *mut size_t as *mut libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    );
    if items != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"fread failed on nz\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            384 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if (*m).size1 != size1 {
        gsl_error(
            b"matrix has wrong size1\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            389 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*m).size2 != size2 {
        gsl_error(
            b"matrix has wrong size2\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            393 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if nz > (*m).nzmax {
        gsl_error(
            b"matrix nzmax is too small\0" as *const u8 as *const libc::c_char,
            b"./file_source.c\0" as *const u8 as *const libc::c_char,
            397 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        items = fread(
            (*m).i as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            nz,
            stream,
        );
        if items != nz {
            gsl_error(
                b"fread failed on row indices\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                406 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        items = fread(
            (*m).data as *mut libc::c_void,
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
            nz,
            stream,
        );
        if items != nz {
            gsl_error(
                b"fread failed on data\0" as *const u8 as *const libc::c_char,
                b"./file_source.c\0" as *const u8 as *const libc::c_char,
                412 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        (*m).nz = nz;
        if (*m).sptype == GSL_SPMATRIX_COO as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                nz,
                stream,
            );
            if items != nz {
                gsl_error(
                    b"fread failed on column indices\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    422 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            gsl_spmatrix_complex_tree_rebuild(m);
        } else if (*m).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                size2.wrapping_add(1 as libc::c_int as libc::c_ulong),
                stream,
            );
            if items != size2.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                gsl_error(
                    b"fread failed on row pointers\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    433 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
        } else if (*m).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            items = fread(
                (*m).p as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                size1.wrapping_add(1 as libc::c_int as libc::c_ulong),
                stream,
            );
            if items != size1.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                gsl_error(
                    b"fread failed on column pointers\0" as *const u8
                        as *const libc::c_char,
                    b"./file_source.c\0" as *const u8 as *const libc::c_char,
                    441 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
