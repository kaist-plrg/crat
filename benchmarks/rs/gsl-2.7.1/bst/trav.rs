use ::libc;
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
    pub table: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub avl_table: gsl_bst_avl_table,
    pub rb_table: gsl_bst_rb_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_bst_trav {
    pub type_0: *const gsl_bst_type,
    pub trav_data: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub avl_trav: gsl_bst_avl_traverser,
    pub rb_trav: gsl_bst_rb_traverser,
}
pub unsafe extern "C" fn gsl_bst_trav_init(
    mut trav: *mut gsl_bst_trav,
    mut w: *const gsl_bst_workspace,
) -> libc::c_int {
    let mut status: libc::c_int = ((*(*w).type_0).trav_init)
        .unwrap()(
        &mut (*trav).trav_data as *mut C2RustUnnamed_0 as *mut libc::c_void,
        &(*w).table as *const C2RustUnnamed as *const libc::c_void,
    );
    (*trav).type_0 = (*w).type_0;
    return status;
}
pub unsafe extern "C" fn gsl_bst_trav_first(
    mut trav: *mut gsl_bst_trav,
    mut w: *const gsl_bst_workspace,
) -> *mut libc::c_void {
    (*trav).type_0 = (*w).type_0;
    return ((*(*w).type_0).trav_first)
        .unwrap()(
        &mut (*trav).trav_data as *mut C2RustUnnamed_0 as *mut libc::c_void,
        &(*w).table as *const C2RustUnnamed as *const libc::c_void,
    );
}
pub unsafe extern "C" fn gsl_bst_trav_last(
    mut trav: *mut gsl_bst_trav,
    mut w: *const gsl_bst_workspace,
) -> *mut libc::c_void {
    (*trav).type_0 = (*w).type_0;
    return ((*(*w).type_0).trav_last)
        .unwrap()(
        &mut (*trav).trav_data as *mut C2RustUnnamed_0 as *mut libc::c_void,
        &(*w).table as *const C2RustUnnamed as *const libc::c_void,
    );
}
pub unsafe extern "C" fn gsl_bst_trav_find(
    mut item: *const libc::c_void,
    mut trav: *mut gsl_bst_trav,
    mut w: *const gsl_bst_workspace,
) -> *mut libc::c_void {
    (*trav).type_0 = (*w).type_0;
    return ((*(*w).type_0).trav_find)
        .unwrap()(
        item,
        &mut (*trav).trav_data as *mut C2RustUnnamed_0 as *mut libc::c_void,
        &(*w).table as *const C2RustUnnamed as *const libc::c_void,
    );
}
pub unsafe extern "C" fn gsl_bst_trav_insert(
    mut item: *mut libc::c_void,
    mut trav: *mut gsl_bst_trav,
    mut w: *mut gsl_bst_workspace,
) -> *mut libc::c_void {
    (*trav).type_0 = (*w).type_0;
    return ((*(*w).type_0).trav_insert)
        .unwrap()(
        item,
        &mut (*trav).trav_data as *mut C2RustUnnamed_0 as *mut libc::c_void,
        &mut (*w).table as *mut C2RustUnnamed as *mut libc::c_void,
    );
}
pub unsafe extern "C" fn gsl_bst_trav_copy(
    mut dest: *mut gsl_bst_trav,
    mut src: *const gsl_bst_trav,
) -> *mut libc::c_void {
    (*dest).type_0 = (*src).type_0;
    return ((*(*src).type_0).trav_copy)
        .unwrap()(
        &mut (*dest).trav_data as *mut C2RustUnnamed_0 as *mut libc::c_void,
        &(*src).trav_data as *const C2RustUnnamed_0 as *const libc::c_void,
    );
}
pub unsafe extern "C" fn gsl_bst_trav_next(
    mut trav: *mut gsl_bst_trav,
) -> *mut libc::c_void {
    return ((*(*trav).type_0).trav_next)
        .unwrap()(&mut (*trav).trav_data as *mut C2RustUnnamed_0 as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_bst_trav_prev(
    mut trav: *mut gsl_bst_trav,
) -> *mut libc::c_void {
    return ((*(*trav).type_0).trav_prev)
        .unwrap()(&mut (*trav).trav_data as *mut C2RustUnnamed_0 as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_bst_trav_cur(
    mut trav: *const gsl_bst_trav,
) -> *mut libc::c_void {
    return ((*(*trav).type_0).trav_cur)
        .unwrap()(&(*trav).trav_data as *const C2RustUnnamed_0 as *const libc::c_void);
}
pub unsafe extern "C" fn gsl_bst_trav_replace(
    mut trav: *mut gsl_bst_trav,
    mut new_item: *mut libc::c_void,
) -> *mut libc::c_void {
    return ((*(*trav).type_0).trav_replace)
        .unwrap()(
        &mut (*trav).trav_data as *mut C2RustUnnamed_0 as *mut libc::c_void,
        new_item,
    );
}
