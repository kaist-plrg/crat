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
    fn gsl_spmatrix_char_realloc(
        nzmax: size_t,
        m: *mut gsl_spmatrix_char,
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
pub unsafe extern "C" fn gsl_spmatrix_char_memcpy(
    mut dest: *mut gsl_spmatrix_char,
    mut src: *const gsl_spmatrix_char,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    if M != (*dest).size1 || N != (*dest).size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*dest).sptype != (*src).sptype {
        gsl_error(
            b"cannot copy matrices of different storage formats\0" as *const u8
                as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_char_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        if (*src).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
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
                        b"./copy_source.c\0" as *const u8 as *const libc::c_char,
                        64 as libc::c_int,
                        GSL_EINVAL as libc::c_int,
                    );
                    return GSL_EINVAL as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
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
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*src).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
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
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*src).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else {
            gsl_error(
                b"invalid matrix type for src\0" as *const u8 as *const libc::c_char,
                b"./copy_source.c\0" as *const u8 as *const libc::c_char,
                100 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        (*dest).nz = (*src).nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_float_memcpy(
    mut dest: *mut gsl_spmatrix_complex_float,
    mut src: *const gsl_spmatrix_complex_float,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    if M != (*dest).size1 || N != (*dest).size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*dest).sptype != (*src).sptype {
        gsl_error(
            b"cannot copy matrices of different storage formats\0" as *const u8
                as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_complex_float_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        if (*src).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
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
                        b"./copy_source.c\0" as *const u8 as *const libc::c_char,
                        64 as libc::c_int,
                        GSL_EINVAL as libc::c_int,
                    );
                    return GSL_EINVAL as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
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
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*src).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
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
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*src).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else {
            gsl_error(
                b"invalid matrix type for src\0" as *const u8 as *const libc::c_char,
                b"./copy_source.c\0" as *const u8 as *const libc::c_char,
                100 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        (*dest).nz = (*src).nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_long_double_memcpy(
    mut dest: *mut gsl_spmatrix_long_double,
    mut src: *const gsl_spmatrix_long_double,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    if M != (*dest).size1 || N != (*dest).size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*dest).sptype != (*src).sptype {
        gsl_error(
            b"cannot copy matrices of different storage formats\0" as *const u8
                as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_long_double_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        if (*src).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
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
                        b"./copy_source.c\0" as *const u8 as *const libc::c_char,
                        64 as libc::c_int,
                        GSL_EINVAL as libc::c_int,
                    );
                    return GSL_EINVAL as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
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
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*src).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
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
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*src).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else {
            gsl_error(
                b"invalid matrix type for src\0" as *const u8 as *const libc::c_char,
                b"./copy_source.c\0" as *const u8 as *const libc::c_char,
                100 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        (*dest).nz = (*src).nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_memcpy(
    mut dest: *mut gsl_spmatrix,
    mut src: *const gsl_spmatrix,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    if M != (*dest).size1 || N != (*dest).size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*dest).sptype != (*src).sptype {
        gsl_error(
            b"cannot copy matrices of different storage formats\0" as *const u8
                as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        if (*src).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
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
                        b"./copy_source.c\0" as *const u8 as *const libc::c_char,
                        64 as libc::c_int,
                        GSL_EINVAL as libc::c_int,
                    );
                    return GSL_EINVAL as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
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
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*src).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
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
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*src).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else {
            gsl_error(
                b"invalid matrix type for src\0" as *const u8 as *const libc::c_char,
                b"./copy_source.c\0" as *const u8 as *const libc::c_char,
                100 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        (*dest).nz = (*src).nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_float_memcpy(
    mut dest: *mut gsl_spmatrix_float,
    mut src: *const gsl_spmatrix_float,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    if M != (*dest).size1 || N != (*dest).size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*dest).sptype != (*src).sptype {
        gsl_error(
            b"cannot copy matrices of different storage formats\0" as *const u8
                as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_float_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        if (*src).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
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
                        b"./copy_source.c\0" as *const u8 as *const libc::c_char,
                        64 as libc::c_int,
                        GSL_EINVAL as libc::c_int,
                    );
                    return GSL_EINVAL as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
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
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*src).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
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
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*src).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else {
            gsl_error(
                b"invalid matrix type for src\0" as *const u8 as *const libc::c_char,
                b"./copy_source.c\0" as *const u8 as *const libc::c_char,
                100 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        (*dest).nz = (*src).nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uchar_memcpy(
    mut dest: *mut gsl_spmatrix_uchar,
    mut src: *const gsl_spmatrix_uchar,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    if M != (*dest).size1 || N != (*dest).size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*dest).sptype != (*src).sptype {
        gsl_error(
            b"cannot copy matrices of different storage formats\0" as *const u8
                as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_uchar_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        if (*src).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
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
                        b"./copy_source.c\0" as *const u8 as *const libc::c_char,
                        64 as libc::c_int,
                        GSL_EINVAL as libc::c_int,
                    );
                    return GSL_EINVAL as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
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
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*src).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
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
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*src).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else {
            gsl_error(
                b"invalid matrix type for src\0" as *const u8 as *const libc::c_char,
                b"./copy_source.c\0" as *const u8 as *const libc::c_char,
                100 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        (*dest).nz = (*src).nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_ulong_memcpy(
    mut dest: *mut gsl_spmatrix_ulong,
    mut src: *const gsl_spmatrix_ulong,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    if M != (*dest).size1 || N != (*dest).size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*dest).sptype != (*src).sptype {
        gsl_error(
            b"cannot copy matrices of different storage formats\0" as *const u8
                as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_ulong_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        if (*src).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
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
                        b"./copy_source.c\0" as *const u8 as *const libc::c_char,
                        64 as libc::c_int,
                        GSL_EINVAL as libc::c_int,
                    );
                    return GSL_EINVAL as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
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
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*src).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
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
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*src).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else {
            gsl_error(
                b"invalid matrix type for src\0" as *const u8 as *const libc::c_char,
                b"./copy_source.c\0" as *const u8 as *const libc::c_char,
                100 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        (*dest).nz = (*src).nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_long_memcpy(
    mut dest: *mut gsl_spmatrix_long,
    mut src: *const gsl_spmatrix_long,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    if M != (*dest).size1 || N != (*dest).size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*dest).sptype != (*src).sptype {
        gsl_error(
            b"cannot copy matrices of different storage formats\0" as *const u8
                as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_long_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        if (*src).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
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
                        b"./copy_source.c\0" as *const u8 as *const libc::c_char,
                        64 as libc::c_int,
                        GSL_EINVAL as libc::c_int,
                    );
                    return GSL_EINVAL as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
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
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*src).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
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
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*src).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else {
            gsl_error(
                b"invalid matrix type for src\0" as *const u8 as *const libc::c_char,
                b"./copy_source.c\0" as *const u8 as *const libc::c_char,
                100 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        (*dest).nz = (*src).nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_uint_memcpy(
    mut dest: *mut gsl_spmatrix_uint,
    mut src: *const gsl_spmatrix_uint,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    if M != (*dest).size1 || N != (*dest).size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*dest).sptype != (*src).sptype {
        gsl_error(
            b"cannot copy matrices of different storage formats\0" as *const u8
                as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_uint_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        if (*src).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
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
                        b"./copy_source.c\0" as *const u8 as *const libc::c_char,
                        64 as libc::c_int,
                        GSL_EINVAL as libc::c_int,
                    );
                    return GSL_EINVAL as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
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
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*src).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
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
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*src).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else {
            gsl_error(
                b"invalid matrix type for src\0" as *const u8 as *const libc::c_char,
                b"./copy_source.c\0" as *const u8 as *const libc::c_char,
                100 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        (*dest).nz = (*src).nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_int_memcpy(
    mut dest: *mut gsl_spmatrix_int,
    mut src: *const gsl_spmatrix_int,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    if M != (*dest).size1 || N != (*dest).size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*dest).sptype != (*src).sptype {
        gsl_error(
            b"cannot copy matrices of different storage formats\0" as *const u8
                as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_int_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        if (*src).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
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
                        b"./copy_source.c\0" as *const u8 as *const libc::c_char,
                        64 as libc::c_int,
                        GSL_EINVAL as libc::c_int,
                    );
                    return GSL_EINVAL as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
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
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*src).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
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
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*src).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else {
            gsl_error(
                b"invalid matrix type for src\0" as *const u8 as *const libc::c_char,
                b"./copy_source.c\0" as *const u8 as *const libc::c_char,
                100 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        (*dest).nz = (*src).nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_ushort_memcpy(
    mut dest: *mut gsl_spmatrix_ushort,
    mut src: *const gsl_spmatrix_ushort,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    if M != (*dest).size1 || N != (*dest).size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*dest).sptype != (*src).sptype {
        gsl_error(
            b"cannot copy matrices of different storage formats\0" as *const u8
                as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_ushort_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        if (*src).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
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
                        b"./copy_source.c\0" as *const u8 as *const libc::c_char,
                        64 as libc::c_int,
                        GSL_EINVAL as libc::c_int,
                    );
                    return GSL_EINVAL as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
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
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*src).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
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
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*src).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else {
            gsl_error(
                b"invalid matrix type for src\0" as *const u8 as *const libc::c_char,
                b"./copy_source.c\0" as *const u8 as *const libc::c_char,
                100 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        (*dest).nz = (*src).nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_short_memcpy(
    mut dest: *mut gsl_spmatrix_short,
    mut src: *const gsl_spmatrix_short,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    if M != (*dest).size1 || N != (*dest).size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*dest).sptype != (*src).sptype {
        gsl_error(
            b"cannot copy matrices of different storage formats\0" as *const u8
                as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_short_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        if (*src).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
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
                        b"./copy_source.c\0" as *const u8 as *const libc::c_char,
                        64 as libc::c_int,
                        GSL_EINVAL as libc::c_int,
                    );
                    return GSL_EINVAL as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
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
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*src).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
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
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*src).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else {
            gsl_error(
                b"invalid matrix type for src\0" as *const u8 as *const libc::c_char,
                b"./copy_source.c\0" as *const u8 as *const libc::c_char,
                100 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        (*dest).nz = (*src).nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_memcpy(
    mut dest: *mut gsl_spmatrix_complex,
    mut src: *const gsl_spmatrix_complex,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    if M != (*dest).size1 || N != (*dest).size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*dest).sptype != (*src).sptype {
        gsl_error(
            b"cannot copy matrices of different storage formats\0" as *const u8
                as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_complex_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        if (*src).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
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
                        b"./copy_source.c\0" as *const u8 as *const libc::c_char,
                        64 as libc::c_int,
                        GSL_EINVAL as libc::c_int,
                    );
                    return GSL_EINVAL as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
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
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*src).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
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
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*src).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else {
            gsl_error(
                b"invalid matrix type for src\0" as *const u8 as *const libc::c_char,
                b"./copy_source.c\0" as *const u8 as *const libc::c_char,
                100 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        (*dest).nz = (*src).nz;
        return status;
    };
}
pub unsafe extern "C" fn gsl_spmatrix_complex_long_double_memcpy(
    mut dest: *mut gsl_spmatrix_complex_long_double,
    mut src: *const gsl_spmatrix_complex_long_double,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    if M != (*dest).size1 || N != (*dest).size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*dest).sptype != (*src).sptype {
        gsl_error(
            b"cannot copy matrices of different storage formats\0" as *const u8
                as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut n: size_t = 0;
        let mut r: size_t = 0;
        if (*dest).nzmax < (*src).nz {
            status = gsl_spmatrix_complex_long_double_realloc((*src).nz, dest);
            if status != 0 {
                return status;
            }
        }
        if (*src).sptype == GSL_SPMATRIX_COO as libc::c_int {
            let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
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
                        b"./copy_source.c\0" as *const u8 as *const libc::c_char,
                        64 as libc::c_int,
                        GSL_EINVAL as libc::c_int,
                    );
                    return GSL_EINVAL as libc::c_int;
                }
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSC as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
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
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*src).size2).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else if (*src).sptype == GSL_SPMATRIX_CSR as libc::c_int {
            n = 0 as libc::c_int as size_t;
            while n < (*src).nz {
                *((*dest).i).offset(n as isize) = *((*src).i).offset(n as isize);
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
                n = n.wrapping_add(1);
                n;
            }
            n = 0 as libc::c_int as size_t;
            while n < ((*src).size1).wrapping_add(1 as libc::c_int as libc::c_ulong) {
                *((*dest).p).offset(n as isize) = *((*src).p).offset(n as isize);
                n = n.wrapping_add(1);
                n;
            }
        } else {
            gsl_error(
                b"invalid matrix type for src\0" as *const u8 as *const libc::c_char,
                b"./copy_source.c\0" as *const u8 as *const libc::c_char,
                100 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        (*dest).nz = (*src).nz;
        return status;
    };
}
