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
}
pub type size_t = libc::c_ulong;
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
    pub table: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub avl_table: gsl_bst_avl_table,
    pub rb_table: gsl_bst_rb_table,
}
pub const GSL_EFAILED: C2RustUnnamed_0 = 5;
pub const GSL_ENOMEM: C2RustUnnamed_0 = 8;
pub type C2RustUnnamed_0 = libc::c_int;
pub const GSL_EOF: C2RustUnnamed_0 = 32;
pub const GSL_ETOLG: C2RustUnnamed_0 = 31;
pub const GSL_ETOLX: C2RustUnnamed_0 = 30;
pub const GSL_ETOLF: C2RustUnnamed_0 = 29;
pub const GSL_ENOPROGJ: C2RustUnnamed_0 = 28;
pub const GSL_ENOPROG: C2RustUnnamed_0 = 27;
pub const GSL_ETABLE: C2RustUnnamed_0 = 26;
pub const GSL_ECACHE: C2RustUnnamed_0 = 25;
pub const GSL_EUNIMPL: C2RustUnnamed_0 = 24;
pub const GSL_EUNSUP: C2RustUnnamed_0 = 23;
pub const GSL_EDIVERGE: C2RustUnnamed_0 = 22;
pub const GSL_ESING: C2RustUnnamed_0 = 21;
pub const GSL_ENOTSQR: C2RustUnnamed_0 = 20;
pub const GSL_EBADLEN: C2RustUnnamed_0 = 19;
pub const GSL_EROUND: C2RustUnnamed_0 = 18;
pub const GSL_ELOSS: C2RustUnnamed_0 = 17;
pub const GSL_EOVRFLW: C2RustUnnamed_0 = 16;
pub const GSL_EUNDRFLW: C2RustUnnamed_0 = 15;
pub const GSL_ETOL: C2RustUnnamed_0 = 14;
pub const GSL_EBADTOL: C2RustUnnamed_0 = 13;
pub const GSL_EZERODIV: C2RustUnnamed_0 = 12;
pub const GSL_EMAXITER: C2RustUnnamed_0 = 11;
pub const GSL_ERUNAWAY: C2RustUnnamed_0 = 10;
pub const GSL_EBADFUNC: C2RustUnnamed_0 = 9;
pub const GSL_ESANITY: C2RustUnnamed_0 = 7;
pub const GSL_EFACTOR: C2RustUnnamed_0 = 6;
pub const GSL_EINVAL: C2RustUnnamed_0 = 4;
pub const GSL_EFAULT: C2RustUnnamed_0 = 3;
pub const GSL_ERANGE: C2RustUnnamed_0 = 2;
pub const GSL_EDOM: C2RustUnnamed_0 = 1;
pub const GSL_CONTINUE: C2RustUnnamed_0 = -2;
pub const GSL_FAILURE: C2RustUnnamed_0 = -1;
pub const GSL_SUCCESS: C2RustUnnamed_0 = 0;
static mut bst_default_allocator: gsl_bst_allocator = unsafe {
    {
        let mut init = gsl_bst_allocator {
            alloc: Some(
                bst_malloc
                    as unsafe extern "C" fn(
                        size_t,
                        *mut libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            free: Some(
                bst_free
                    as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
            ),
        };
        init
    }
};
pub unsafe extern "C" fn gsl_bst_alloc(
    mut T: *const gsl_bst_type,
    mut allocator: *const gsl_bst_allocator,
    mut compare: Option::<gsl_bst_cmp_function>,
    mut params: *mut libc::c_void,
) -> *mut gsl_bst_workspace {
    let mut status: libc::c_int = 0;
    let mut w: *mut gsl_bst_workspace = 0 as *mut gsl_bst_workspace;
    w = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gsl_bst_workspace>() as libc::c_ulong,
    ) as *mut gsl_bst_workspace;
    if w.is_null() {
        gsl_error(
            b"failed to allocate bst workspace\0" as *const u8 as *const libc::c_char,
            b"bst.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_bst_workspace;
    }
    (*w).type_0 = T;
    status = ((*(*w).type_0).init)
        .unwrap()(
        if !allocator.is_null() { allocator } else { &bst_default_allocator },
        compare,
        params,
        &mut (*w).table as *mut C2RustUnnamed as *mut libc::c_void,
    );
    if status != 0 {
        gsl_bst_free(w);
        gsl_error(
            b"failed to initialize bst\0" as *const u8 as *const libc::c_char,
            b"bst.c\0" as *const u8 as *const libc::c_char,
            66 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return 0 as *mut gsl_bst_workspace;
    }
    return w;
}
pub unsafe extern "C" fn gsl_bst_free(mut w: *mut gsl_bst_workspace) {
    gsl_bst_empty(w);
    free(w as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_bst_empty(mut w: *mut gsl_bst_workspace) -> libc::c_int {
    return ((*(*w).type_0).empty)
        .unwrap()(&mut (*w).table as *mut C2RustUnnamed as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_bst_insert(
    mut item: *mut libc::c_void,
    mut w: *mut gsl_bst_workspace,
) -> *mut libc::c_void {
    return ((*(*w).type_0).insert)
        .unwrap()(item, &mut (*w).table as *mut C2RustUnnamed as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_bst_find(
    mut item: *const libc::c_void,
    mut w: *const gsl_bst_workspace,
) -> *mut libc::c_void {
    return ((*(*w).type_0).find)
        .unwrap()(item, &(*w).table as *const C2RustUnnamed as *const libc::c_void);
}
pub unsafe extern "C" fn gsl_bst_remove(
    mut item: *const libc::c_void,
    mut w: *mut gsl_bst_workspace,
) -> *mut libc::c_void {
    return ((*(*w).type_0).remove)
        .unwrap()(item, &mut (*w).table as *mut C2RustUnnamed as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_bst_nodes(mut w: *const gsl_bst_workspace) -> size_t {
    return ((*(*w).type_0).nodes)
        .unwrap()(&(*w).table as *const C2RustUnnamed as *const libc::c_void);
}
pub unsafe extern "C" fn gsl_bst_node_size(mut w: *const gsl_bst_workspace) -> size_t {
    return (*(*w).type_0).node_size;
}
pub unsafe extern "C" fn gsl_bst_name(
    mut w: *const gsl_bst_workspace,
) -> *const libc::c_char {
    return (*(*w).type_0).name;
}
unsafe extern "C" fn bst_malloc(
    mut size: size_t,
    mut params: *mut libc::c_void,
) -> *mut libc::c_void {
    return malloc(size);
}
unsafe extern "C" fn bst_free(
    mut block: *mut libc::c_void,
    mut params: *mut libc::c_void,
) {
    free(block);
}
