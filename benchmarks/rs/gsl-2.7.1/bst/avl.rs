use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
pub struct gsl_bst_avl_traverser {
    pub avl_table: *const gsl_bst_avl_table,
    pub avl_node: *mut gsl_bst_avl_node,
    pub avl_stack: [*mut gsl_bst_avl_node; 32],
    pub avl_height: size_t,
    pub avl_generation: libc::c_ulong,
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
pub type avl_traverser = gsl_bst_avl_traverser;
pub type avl_node = gsl_bst_avl_node;
pub const GSL_ETABLE: C2RustUnnamed = 26;
pub type avl_table = gsl_bst_avl_table;
pub const GSL_SUCCESS: C2RustUnnamed = 0;
pub type C2RustUnnamed = libc::c_int;
pub const GSL_EOF: C2RustUnnamed = 32;
pub const GSL_ETOLG: C2RustUnnamed = 31;
pub const GSL_ETOLX: C2RustUnnamed = 30;
pub const GSL_ETOLF: C2RustUnnamed = 29;
pub const GSL_ENOPROGJ: C2RustUnnamed = 28;
pub const GSL_ENOPROG: C2RustUnnamed = 27;
pub const GSL_ECACHE: C2RustUnnamed = 25;
pub const GSL_EUNIMPL: C2RustUnnamed = 24;
pub const GSL_EUNSUP: C2RustUnnamed = 23;
pub const GSL_EDIVERGE: C2RustUnnamed = 22;
pub const GSL_ESING: C2RustUnnamed = 21;
pub const GSL_ENOTSQR: C2RustUnnamed = 20;
pub const GSL_EBADLEN: C2RustUnnamed = 19;
pub const GSL_EROUND: C2RustUnnamed = 18;
pub const GSL_ELOSS: C2RustUnnamed = 17;
pub const GSL_EOVRFLW: C2RustUnnamed = 16;
pub const GSL_EUNDRFLW: C2RustUnnamed = 15;
pub const GSL_ETOL: C2RustUnnamed = 14;
pub const GSL_EBADTOL: C2RustUnnamed = 13;
pub const GSL_EZERODIV: C2RustUnnamed = 12;
pub const GSL_EMAXITER: C2RustUnnamed = 11;
pub const GSL_ERUNAWAY: C2RustUnnamed = 10;
pub const GSL_EBADFUNC: C2RustUnnamed = 9;
pub const GSL_ENOMEM: C2RustUnnamed = 8;
pub const GSL_ESANITY: C2RustUnnamed = 7;
pub const GSL_EFACTOR: C2RustUnnamed = 6;
pub const GSL_EFAILED: C2RustUnnamed = 5;
pub const GSL_EINVAL: C2RustUnnamed = 4;
pub const GSL_EFAULT: C2RustUnnamed = 3;
pub const GSL_ERANGE: C2RustUnnamed = 2;
pub const GSL_EDOM: C2RustUnnamed = 1;
pub const GSL_CONTINUE: C2RustUnnamed = -2;
pub const GSL_FAILURE: C2RustUnnamed = -1;
unsafe extern "C" fn avl_init(
    mut allocator: *const gsl_bst_allocator,
    mut compare: Option::<gsl_bst_cmp_function>,
    mut params: *mut libc::c_void,
    mut vtable: *mut libc::c_void,
) -> libc::c_int {
    let mut table: *mut avl_table = vtable as *mut avl_table;
    (*table).avl_alloc = allocator;
    (*table).avl_compare = compare;
    (*table).avl_param = params;
    (*table).avl_root = 0 as *mut gsl_bst_avl_node;
    (*table).avl_count = 0 as libc::c_int as size_t;
    (*table).avl_generation = 0 as libc::c_int as libc::c_ulong;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn avl_nodes(mut vtable: *const libc::c_void) -> size_t {
    let mut table: *const avl_table = vtable as *const avl_table;
    return (*table).avl_count;
}
unsafe extern "C" fn avl_empty(mut vtable: *mut libc::c_void) -> libc::c_int {
    let mut table: *mut avl_table = vtable as *mut avl_table;
    let mut p: *mut avl_node = 0 as *mut avl_node;
    let mut q: *mut avl_node = 0 as *mut avl_node;
    p = (*table).avl_root;
    while !p.is_null() {
        if ((*p).avl_link[0 as libc::c_int as usize]).is_null() {
            q = (*p).avl_link[1 as libc::c_int as usize];
            ((*(*table).avl_alloc).free)
                .unwrap()(p as *mut libc::c_void, (*table).avl_param);
        } else {
            q = (*p).avl_link[0 as libc::c_int as usize];
            (*p)
                .avl_link[0 as libc::c_int
                as usize] = (*q).avl_link[1 as libc::c_int as usize];
            (*q).avl_link[1 as libc::c_int as usize] = p;
        }
        p = q;
    }
    (*table).avl_root = 0 as *mut gsl_bst_avl_node;
    (*table).avl_count = 0 as libc::c_int as size_t;
    (*table).avl_generation = 0 as libc::c_int as libc::c_ulong;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn avl_probe(
    mut item: *mut libc::c_void,
    mut table: *mut avl_table,
) -> *mut *mut libc::c_void {
    let mut y: *mut avl_node = 0 as *mut avl_node;
    let mut z: *mut avl_node = 0 as *mut avl_node;
    let mut p: *mut avl_node = 0 as *mut avl_node;
    let mut q: *mut avl_node = 0 as *mut avl_node;
    let mut n: *mut avl_node = 0 as *mut avl_node;
    let mut w: *mut avl_node = 0 as *mut avl_node;
    let mut dir: libc::c_int = 0;
    let mut da: [libc::c_uchar; 32] = [0; 32];
    let mut k: libc::c_int = 0 as libc::c_int;
    z = &mut (*table).avl_root as *mut *mut gsl_bst_avl_node as *mut avl_node;
    y = (*table).avl_root;
    dir = 0 as libc::c_int;
    q = z;
    p = y;
    while !p.is_null() {
        let mut cmp: libc::c_int = ((*table).avl_compare)
            .unwrap()(item, (*p).avl_data, (*table).avl_param);
        if cmp == 0 as libc::c_int {
            return &mut (*p).avl_data;
        }
        if (*p).avl_balance as libc::c_int != 0 as libc::c_int {
            z = q;
            y = p;
            k = 0 as libc::c_int;
        }
        dir = (cmp > 0 as libc::c_int) as libc::c_int;
        let fresh0 = k;
        k = k + 1;
        da[fresh0 as usize] = dir as libc::c_uchar;
        q = p;
        p = (*p).avl_link[dir as usize];
    }
    (*q)
        .avl_link[dir
        as usize] = ((*(*table).avl_alloc).alloc)
        .unwrap()(::std::mem::size_of::<avl_node>() as libc::c_ulong, (*table).avl_param)
        as *mut gsl_bst_avl_node;
    n = (*q).avl_link[dir as usize];
    if n.is_null() {
        return 0 as *mut *mut libc::c_void;
    }
    (*table).avl_count = ((*table).avl_count).wrapping_add(1);
    (*table).avl_count;
    (*n).avl_data = item;
    (*n).avl_link[1 as libc::c_int as usize] = 0 as *mut gsl_bst_avl_node;
    (*n).avl_link[0 as libc::c_int as usize] = (*n).avl_link[1 as libc::c_int as usize];
    (*n).avl_balance = 0 as libc::c_int as libc::c_schar;
    if y.is_null() {
        return &mut (*n).avl_data;
    }
    p = y;
    k = 0 as libc::c_int;
    while p != n {
        if da[k as usize] as libc::c_int == 0 as libc::c_int {
            (*p).avl_balance -= 1;
            (*p).avl_balance;
        } else {
            (*p).avl_balance += 1;
            (*p).avl_balance;
        }
        p = (*p).avl_link[da[k as usize] as usize];
        k += 1;
        k;
    }
    if (*y).avl_balance as libc::c_int == -(2 as libc::c_int) {
        let mut x: *mut avl_node = (*y).avl_link[0 as libc::c_int as usize];
        if (*x).avl_balance as libc::c_int == -(1 as libc::c_int) {
            w = x;
            (*y)
                .avl_link[0 as libc::c_int
                as usize] = (*x).avl_link[1 as libc::c_int as usize];
            (*x).avl_link[1 as libc::c_int as usize] = y;
            (*y).avl_balance = 0 as libc::c_int as libc::c_schar;
            (*x).avl_balance = (*y).avl_balance;
        } else {
            w = (*x).avl_link[1 as libc::c_int as usize];
            (*x)
                .avl_link[1 as libc::c_int
                as usize] = (*w).avl_link[0 as libc::c_int as usize];
            (*w).avl_link[0 as libc::c_int as usize] = x;
            (*y)
                .avl_link[0 as libc::c_int
                as usize] = (*w).avl_link[1 as libc::c_int as usize];
            (*w).avl_link[1 as libc::c_int as usize] = y;
            if (*w).avl_balance as libc::c_int == -(1 as libc::c_int) {
                (*x).avl_balance = 0 as libc::c_int as libc::c_schar;
                (*y).avl_balance = 1 as libc::c_int as libc::c_schar;
            } else if (*w).avl_balance as libc::c_int == 0 as libc::c_int {
                (*y).avl_balance = 0 as libc::c_int as libc::c_schar;
                (*x).avl_balance = (*y).avl_balance;
            } else {
                (*x).avl_balance = -(1 as libc::c_int) as libc::c_schar;
                (*y).avl_balance = 0 as libc::c_int as libc::c_schar;
            }
            (*w).avl_balance = 0 as libc::c_int as libc::c_schar;
        }
    } else if (*y).avl_balance as libc::c_int == 2 as libc::c_int {
        let mut x_0: *mut avl_node = (*y).avl_link[1 as libc::c_int as usize];
        if (*x_0).avl_balance as libc::c_int == 1 as libc::c_int {
            w = x_0;
            (*y)
                .avl_link[1 as libc::c_int
                as usize] = (*x_0).avl_link[0 as libc::c_int as usize];
            (*x_0).avl_link[0 as libc::c_int as usize] = y;
            (*y).avl_balance = 0 as libc::c_int as libc::c_schar;
            (*x_0).avl_balance = (*y).avl_balance;
        } else {
            w = (*x_0).avl_link[0 as libc::c_int as usize];
            (*x_0)
                .avl_link[0 as libc::c_int
                as usize] = (*w).avl_link[1 as libc::c_int as usize];
            (*w).avl_link[1 as libc::c_int as usize] = x_0;
            (*y)
                .avl_link[1 as libc::c_int
                as usize] = (*w).avl_link[0 as libc::c_int as usize];
            (*w).avl_link[0 as libc::c_int as usize] = y;
            if (*w).avl_balance as libc::c_int == 1 as libc::c_int {
                (*x_0).avl_balance = 0 as libc::c_int as libc::c_schar;
                (*y).avl_balance = -(1 as libc::c_int) as libc::c_schar;
            } else if (*w).avl_balance as libc::c_int == 0 as libc::c_int {
                (*y).avl_balance = 0 as libc::c_int as libc::c_schar;
                (*x_0).avl_balance = (*y).avl_balance;
            } else {
                (*x_0).avl_balance = 1 as libc::c_int as libc::c_schar;
                (*y).avl_balance = 0 as libc::c_int as libc::c_schar;
            }
            (*w).avl_balance = 0 as libc::c_int as libc::c_schar;
        }
    } else {
        return &mut (*n).avl_data
    }
    (*z)
        .avl_link[(y != (*z).avl_link[0 as libc::c_int as usize]) as libc::c_int
        as usize] = w;
    (*table).avl_generation = ((*table).avl_generation).wrapping_add(1);
    (*table).avl_generation;
    return &mut (*n).avl_data;
}
unsafe extern "C" fn avl_insert(
    mut item: *mut libc::c_void,
    mut vtable: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut p: *mut *mut libc::c_void = avl_probe(item, vtable as *mut avl_table);
    return if p.is_null() || *p == item { 0 as *mut libc::c_void } else { *p };
}
unsafe extern "C" fn avl_find(
    mut item: *const libc::c_void,
    mut vtable: *const libc::c_void,
) -> *mut libc::c_void {
    let mut table: *const avl_table = vtable as *const avl_table;
    let mut p: *mut avl_node = 0 as *mut avl_node;
    p = (*table).avl_root;
    while !p.is_null() {
        let mut cmp: libc::c_int = ((*table).avl_compare)
            .unwrap()(item, (*p).avl_data, (*table).avl_param);
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
unsafe extern "C" fn avl_remove(
    mut item: *const libc::c_void,
    mut vtable: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut table: *mut avl_table = vtable as *mut avl_table;
    let mut pa: [*mut avl_node; 32] = [0 as *mut avl_node; 32];
    let mut da: [libc::c_uchar; 32] = [0; 32];
    let mut k: libc::c_int = 0;
    let mut p: *mut avl_node = 0 as *mut avl_node;
    let mut cmp: libc::c_int = 0;
    k = 0 as libc::c_int;
    p = &mut (*table).avl_root as *mut *mut gsl_bst_avl_node as *mut avl_node;
    cmp = -(1 as libc::c_int);
    while cmp != 0 as libc::c_int {
        let mut dir: libc::c_int = (cmp > 0 as libc::c_int) as libc::c_int;
        pa[k as usize] = p;
        let fresh1 = k;
        k = k + 1;
        da[fresh1 as usize] = dir as libc::c_uchar;
        p = (*p).avl_link[dir as usize];
        if p.is_null() {
            return 0 as *mut libc::c_void;
        }
        cmp = ((*table).avl_compare).unwrap()(item, (*p).avl_data, (*table).avl_param);
    }
    item = (*p).avl_data;
    if ((*p).avl_link[1 as libc::c_int as usize]).is_null() {
        (*pa[(k - 1 as libc::c_int) as usize])
            .avl_link[da[(k - 1 as libc::c_int) as usize]
            as usize] = (*p).avl_link[0 as libc::c_int as usize];
    } else {
        let mut r: *mut avl_node = (*p).avl_link[1 as libc::c_int as usize];
        if ((*r).avl_link[0 as libc::c_int as usize]).is_null() {
            (*r)
                .avl_link[0 as libc::c_int
                as usize] = (*p).avl_link[0 as libc::c_int as usize];
            (*r).avl_balance = (*p).avl_balance;
            (*pa[(k - 1 as libc::c_int) as usize])
                .avl_link[da[(k - 1 as libc::c_int) as usize] as usize] = r;
            da[k as usize] = 1 as libc::c_int as libc::c_uchar;
            let fresh2 = k;
            k = k + 1;
            pa[fresh2 as usize] = r;
        } else {
            let mut s: *mut avl_node = 0 as *mut avl_node;
            let fresh3 = k;
            k = k + 1;
            let mut j: libc::c_int = fresh3;
            loop {
                da[k as usize] = 0 as libc::c_int as libc::c_uchar;
                let fresh4 = k;
                k = k + 1;
                pa[fresh4 as usize] = r;
                s = (*r).avl_link[0 as libc::c_int as usize];
                if ((*s).avl_link[0 as libc::c_int as usize]).is_null() {
                    break;
                }
                r = s;
            }
            (*s)
                .avl_link[0 as libc::c_int
                as usize] = (*p).avl_link[0 as libc::c_int as usize];
            (*r)
                .avl_link[0 as libc::c_int
                as usize] = (*s).avl_link[1 as libc::c_int as usize];
            (*s)
                .avl_link[1 as libc::c_int
                as usize] = (*p).avl_link[1 as libc::c_int as usize];
            (*s).avl_balance = (*p).avl_balance;
            (*pa[(j - 1 as libc::c_int) as usize])
                .avl_link[da[(j - 1 as libc::c_int) as usize] as usize] = s;
            da[j as usize] = 1 as libc::c_int as libc::c_uchar;
            pa[j as usize] = s;
        }
    }
    ((*(*table).avl_alloc).free).unwrap()(p as *mut libc::c_void, (*table).avl_param);
    loop {
        k -= 1;
        if !(k > 0 as libc::c_int) {
            break;
        }
        let mut y: *mut avl_node = pa[k as usize];
        if da[k as usize] as libc::c_int == 0 as libc::c_int {
            (*y).avl_balance += 1;
            (*y).avl_balance;
            if (*y).avl_balance as libc::c_int == 1 as libc::c_int {
                break;
            }
            if !((*y).avl_balance as libc::c_int == 2 as libc::c_int) {
                continue;
            }
            let mut x: *mut avl_node = (*y).avl_link[1 as libc::c_int as usize];
            if (*x).avl_balance as libc::c_int == -(1 as libc::c_int) {
                let mut w: *mut avl_node = 0 as *mut avl_node;
                w = (*x).avl_link[0 as libc::c_int as usize];
                (*x)
                    .avl_link[0 as libc::c_int
                    as usize] = (*w).avl_link[1 as libc::c_int as usize];
                (*w).avl_link[1 as libc::c_int as usize] = x;
                (*y)
                    .avl_link[1 as libc::c_int
                    as usize] = (*w).avl_link[0 as libc::c_int as usize];
                (*w).avl_link[0 as libc::c_int as usize] = y;
                if (*w).avl_balance as libc::c_int == 1 as libc::c_int {
                    (*x).avl_balance = 0 as libc::c_int as libc::c_schar;
                    (*y).avl_balance = -(1 as libc::c_int) as libc::c_schar;
                } else if (*w).avl_balance as libc::c_int == 0 as libc::c_int {
                    (*y).avl_balance = 0 as libc::c_int as libc::c_schar;
                    (*x).avl_balance = (*y).avl_balance;
                } else {
                    (*x).avl_balance = 1 as libc::c_int as libc::c_schar;
                    (*y).avl_balance = 0 as libc::c_int as libc::c_schar;
                }
                (*w).avl_balance = 0 as libc::c_int as libc::c_schar;
                (*pa[(k - 1 as libc::c_int) as usize])
                    .avl_link[da[(k - 1 as libc::c_int) as usize] as usize] = w;
            } else {
                (*y)
                    .avl_link[1 as libc::c_int
                    as usize] = (*x).avl_link[0 as libc::c_int as usize];
                (*x).avl_link[0 as libc::c_int as usize] = y;
                (*pa[(k - 1 as libc::c_int) as usize])
                    .avl_link[da[(k - 1 as libc::c_int) as usize] as usize] = x;
                if (*x).avl_balance as libc::c_int == 0 as libc::c_int {
                    (*x).avl_balance = -(1 as libc::c_int) as libc::c_schar;
                    (*y).avl_balance = 1 as libc::c_int as libc::c_schar;
                    break;
                } else {
                    (*y).avl_balance = 0 as libc::c_int as libc::c_schar;
                    (*x).avl_balance = (*y).avl_balance;
                }
            }
        } else {
            (*y).avl_balance -= 1;
            (*y).avl_balance;
            if (*y).avl_balance as libc::c_int == -(1 as libc::c_int) {
                break;
            }
            if !((*y).avl_balance as libc::c_int == -(2 as libc::c_int)) {
                continue;
            }
            let mut x_0: *mut avl_node = (*y).avl_link[0 as libc::c_int as usize];
            if (*x_0).avl_balance as libc::c_int == 1 as libc::c_int {
                let mut w_0: *mut avl_node = 0 as *mut avl_node;
                w_0 = (*x_0).avl_link[1 as libc::c_int as usize];
                (*x_0)
                    .avl_link[1 as libc::c_int
                    as usize] = (*w_0).avl_link[0 as libc::c_int as usize];
                (*w_0).avl_link[0 as libc::c_int as usize] = x_0;
                (*y)
                    .avl_link[0 as libc::c_int
                    as usize] = (*w_0).avl_link[1 as libc::c_int as usize];
                (*w_0).avl_link[1 as libc::c_int as usize] = y;
                if (*w_0).avl_balance as libc::c_int == -(1 as libc::c_int) {
                    (*x_0).avl_balance = 0 as libc::c_int as libc::c_schar;
                    (*y).avl_balance = 1 as libc::c_int as libc::c_schar;
                } else if (*w_0).avl_balance as libc::c_int == 0 as libc::c_int {
                    (*y).avl_balance = 0 as libc::c_int as libc::c_schar;
                    (*x_0).avl_balance = (*y).avl_balance;
                } else {
                    (*x_0).avl_balance = -(1 as libc::c_int) as libc::c_schar;
                    (*y).avl_balance = 0 as libc::c_int as libc::c_schar;
                }
                (*w_0).avl_balance = 0 as libc::c_int as libc::c_schar;
                (*pa[(k - 1 as libc::c_int) as usize])
                    .avl_link[da[(k - 1 as libc::c_int) as usize] as usize] = w_0;
            } else {
                (*y)
                    .avl_link[0 as libc::c_int
                    as usize] = (*x_0).avl_link[1 as libc::c_int as usize];
                (*x_0).avl_link[1 as libc::c_int as usize] = y;
                (*pa[(k - 1 as libc::c_int) as usize])
                    .avl_link[da[(k - 1 as libc::c_int) as usize] as usize] = x_0;
                if (*x_0).avl_balance as libc::c_int == 0 as libc::c_int {
                    (*x_0).avl_balance = 1 as libc::c_int as libc::c_schar;
                    (*y).avl_balance = -(1 as libc::c_int) as libc::c_schar;
                    break;
                } else {
                    (*y).avl_balance = 0 as libc::c_int as libc::c_schar;
                    (*x_0).avl_balance = (*y).avl_balance;
                }
            }
        }
    }
    (*table).avl_count = ((*table).avl_count).wrapping_sub(1);
    (*table).avl_count;
    (*table).avl_generation = ((*table).avl_generation).wrapping_add(1);
    (*table).avl_generation;
    return item as *mut libc::c_void;
}
unsafe extern "C" fn avl_t_init(
    mut vtrav: *mut libc::c_void,
    mut vtable: *const libc::c_void,
) -> libc::c_int {
    let mut trav: *mut avl_traverser = vtrav as *mut avl_traverser;
    let mut table: *const avl_table = vtable as *const avl_table;
    (*trav).avl_table = table;
    (*trav).avl_node = 0 as *mut gsl_bst_avl_node;
    (*trav).avl_height = 0 as libc::c_int as size_t;
    (*trav).avl_generation = (*table).avl_generation;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn avl_t_first(
    mut vtrav: *mut libc::c_void,
    mut vtable: *const libc::c_void,
) -> *mut libc::c_void {
    let mut table: *const avl_table = vtable as *const avl_table;
    let mut trav: *mut avl_traverser = vtrav as *mut avl_traverser;
    let mut x: *mut avl_node = 0 as *mut avl_node;
    (*trav).avl_table = table;
    (*trav).avl_height = 0 as libc::c_int as size_t;
    (*trav).avl_generation = (*table).avl_generation;
    x = (*table).avl_root;
    if !x.is_null() {
        while !((*x).avl_link[0 as libc::c_int as usize]).is_null() {
            if (*trav).avl_height >= 32 as libc::c_int as libc::c_ulong {
                gsl_error(
                    b"traverser height exceeds maximum\0" as *const u8
                        as *const libc::c_char,
                    b"avl.c\0" as *const u8 as *const libc::c_char,
                    632 as libc::c_int,
                    GSL_ETABLE as libc::c_int,
                );
                return 0 as *mut libc::c_void;
            }
            let fresh5 = (*trav).avl_height;
            (*trav).avl_height = ((*trav).avl_height).wrapping_add(1);
            (*trav).avl_stack[fresh5 as usize] = x;
            x = (*x).avl_link[0 as libc::c_int as usize];
        }
    }
    (*trav).avl_node = x;
    return if !x.is_null() { (*x).avl_data } else { 0 as *mut libc::c_void };
}
unsafe extern "C" fn avl_t_last(
    mut vtrav: *mut libc::c_void,
    mut vtable: *const libc::c_void,
) -> *mut libc::c_void {
    let mut table: *const avl_table = vtable as *const avl_table;
    let mut trav: *mut avl_traverser = vtrav as *mut avl_traverser;
    let mut x: *mut avl_node = 0 as *mut avl_node;
    (*trav).avl_table = table;
    (*trav).avl_height = 0 as libc::c_int as size_t;
    (*trav).avl_generation = (*table).avl_generation;
    x = (*table).avl_root;
    if !x.is_null() {
        while !((*x).avl_link[1 as libc::c_int as usize]).is_null() {
            if (*trav).avl_height >= 32 as libc::c_int as libc::c_ulong {
                gsl_error(
                    b"traverser height exceeds maximum\0" as *const u8
                        as *const libc::c_char,
                    b"avl.c\0" as *const u8 as *const libc::c_char,
                    669 as libc::c_int,
                    GSL_ETABLE as libc::c_int,
                );
                return 0 as *mut libc::c_void;
            }
            let fresh6 = (*trav).avl_height;
            (*trav).avl_height = ((*trav).avl_height).wrapping_add(1);
            (*trav).avl_stack[fresh6 as usize] = x;
            x = (*x).avl_link[1 as libc::c_int as usize];
        }
    }
    (*trav).avl_node = x;
    return if !x.is_null() { (*x).avl_data } else { 0 as *mut libc::c_void };
}
unsafe extern "C" fn avl_t_find(
    mut item: *const libc::c_void,
    mut vtrav: *mut libc::c_void,
    mut vtable: *const libc::c_void,
) -> *mut libc::c_void {
    let mut table: *const avl_table = vtable as *const avl_table;
    let mut trav: *mut avl_traverser = vtrav as *mut avl_traverser;
    let mut p: *mut avl_node = 0 as *mut avl_node;
    let mut q: *mut avl_node = 0 as *mut avl_node;
    (*trav).avl_table = table;
    (*trav).avl_height = 0 as libc::c_int as size_t;
    (*trav).avl_generation = (*table).avl_generation;
    p = (*table).avl_root;
    while !p.is_null() {
        let mut cmp: libc::c_int = ((*table).avl_compare)
            .unwrap()(item, (*p).avl_data, (*table).avl_param);
        if cmp < 0 as libc::c_int {
            q = (*p).avl_link[0 as libc::c_int as usize];
        } else if cmp > 0 as libc::c_int {
            q = (*p).avl_link[1 as libc::c_int as usize];
        } else {
            (*trav).avl_node = p;
            return (*p).avl_data;
        }
        if (*trav).avl_height >= 32 as libc::c_int as libc::c_ulong {
            gsl_error(
                b"traverser height exceeds maximum\0" as *const u8
                    as *const libc::c_char,
                b"avl.c\0" as *const u8 as *const libc::c_char,
                716 as libc::c_int,
                GSL_ETABLE as libc::c_int,
            );
            return 0 as *mut libc::c_void;
        }
        let fresh7 = (*trav).avl_height;
        (*trav).avl_height = ((*trav).avl_height).wrapping_add(1);
        (*trav).avl_stack[fresh7 as usize] = p;
        p = q;
    }
    (*trav).avl_height = 0 as libc::c_int as size_t;
    (*trav).avl_node = 0 as *mut gsl_bst_avl_node;
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn avl_t_insert(
    mut item: *mut libc::c_void,
    mut vtrav: *mut libc::c_void,
    mut vtable: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut table: *mut avl_table = vtable as *mut avl_table;
    let mut trav: *mut avl_traverser = vtrav as *mut avl_traverser;
    let mut p: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    p = avl_probe(item, table);
    if !p.is_null() {
        (*trav).avl_table = table;
        (*trav)
            .avl_node = (p as *mut libc::c_char).offset(-(16 as libc::c_ulong as isize))
            as *mut avl_node;
        (*trav)
            .avl_generation = ((*table).avl_generation)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        return *p;
    } else {
        avl_t_init(vtrav, vtable);
        return 0 as *mut libc::c_void;
    };
}
unsafe extern "C" fn avl_t_copy(
    mut vtrav: *mut libc::c_void,
    mut vsrc: *const libc::c_void,
) -> *mut libc::c_void {
    let mut src: *const avl_traverser = vsrc as *const avl_traverser;
    let mut trav: *mut avl_traverser = vtrav as *mut avl_traverser;
    if trav != src as *mut avl_traverser {
        (*trav).avl_table = (*src).avl_table;
        (*trav).avl_node = (*src).avl_node;
        (*trav).avl_generation = (*src).avl_generation;
        if (*trav).avl_generation == (*(*trav).avl_table).avl_generation {
            (*trav).avl_height = (*src).avl_height;
            memcpy(
                ((*trav).avl_stack).as_mut_ptr() as *mut libc::c_void,
                ((*src).avl_stack).as_ptr() as *const libc::c_void,
                (::std::mem::size_of::<*mut gsl_bst_avl_node>() as libc::c_ulong)
                    .wrapping_mul((*trav).avl_height),
            );
        }
    }
    return if !((*trav).avl_node).is_null() {
        (*(*trav).avl_node).avl_data
    } else {
        0 as *mut libc::c_void
    };
}
unsafe extern "C" fn avl_t_next(mut vtrav: *mut libc::c_void) -> *mut libc::c_void {
    let mut trav: *mut avl_traverser = vtrav as *mut avl_traverser;
    let mut x: *mut avl_node = 0 as *mut avl_node;
    if (*trav).avl_generation != (*(*trav).avl_table).avl_generation {
        avl_trav_refresh(trav);
    }
    x = (*trav).avl_node;
    if x.is_null() {
        return avl_t_first(vtrav, (*trav).avl_table as *const libc::c_void)
    } else if !((*x).avl_link[1 as libc::c_int as usize]).is_null() {
        if (*trav).avl_height >= 32 as libc::c_int as libc::c_ulong {
            gsl_error(
                b"traverser height exceeds maximum\0" as *const u8
                    as *const libc::c_char,
                b"avl.c\0" as *const u8 as *const libc::c_char,
                811 as libc::c_int,
                GSL_ETABLE as libc::c_int,
            );
            return 0 as *mut libc::c_void;
        }
        let fresh8 = (*trav).avl_height;
        (*trav).avl_height = ((*trav).avl_height).wrapping_add(1);
        (*trav).avl_stack[fresh8 as usize] = x;
        x = (*x).avl_link[1 as libc::c_int as usize];
        while !((*x).avl_link[0 as libc::c_int as usize]).is_null() {
            if (*trav).avl_height >= 32 as libc::c_int as libc::c_ulong {
                gsl_error(
                    b"traverser height exceeds maximum\0" as *const u8
                        as *const libc::c_char,
                    b"avl.c\0" as *const u8 as *const libc::c_char,
                    821 as libc::c_int,
                    GSL_ETABLE as libc::c_int,
                );
                return 0 as *mut libc::c_void;
            }
            let fresh9 = (*trav).avl_height;
            (*trav).avl_height = ((*trav).avl_height).wrapping_add(1);
            (*trav).avl_stack[fresh9 as usize] = x;
            x = (*x).avl_link[0 as libc::c_int as usize];
        }
    } else {
        let mut y: *mut avl_node = 0 as *mut avl_node;
        loop {
            if (*trav).avl_height == 0 as libc::c_int as libc::c_ulong {
                (*trav).avl_node = 0 as *mut gsl_bst_avl_node;
                return 0 as *mut libc::c_void;
            }
            y = x;
            (*trav).avl_height = ((*trav).avl_height).wrapping_sub(1);
            x = (*trav).avl_stack[(*trav).avl_height as usize];
            if !(y == (*x).avl_link[1 as libc::c_int as usize]) {
                break;
            }
        }
    }
    (*trav).avl_node = x;
    return (*x).avl_data;
}
unsafe extern "C" fn avl_t_prev(mut vtrav: *mut libc::c_void) -> *mut libc::c_void {
    let mut trav: *mut avl_traverser = vtrav as *mut avl_traverser;
    let mut x: *mut avl_node = 0 as *mut avl_node;
    if (*trav).avl_generation != (*(*trav).avl_table).avl_generation {
        avl_trav_refresh(trav);
    }
    x = (*trav).avl_node;
    if x.is_null() {
        return avl_t_last(vtrav, (*trav).avl_table as *const libc::c_void)
    } else if !((*x).avl_link[0 as libc::c_int as usize]).is_null() {
        if (*trav).avl_height >= 32 as libc::c_int as libc::c_ulong {
            gsl_error(
                b"traverser height exceeds maximum\0" as *const u8
                    as *const libc::c_char,
                b"avl.c\0" as *const u8 as *const libc::c_char,
                875 as libc::c_int,
                GSL_ETABLE as libc::c_int,
            );
            return 0 as *mut libc::c_void;
        }
        let fresh10 = (*trav).avl_height;
        (*trav).avl_height = ((*trav).avl_height).wrapping_add(1);
        (*trav).avl_stack[fresh10 as usize] = x;
        x = (*x).avl_link[0 as libc::c_int as usize];
        while !((*x).avl_link[1 as libc::c_int as usize]).is_null() {
            if (*trav).avl_height >= 32 as libc::c_int as libc::c_ulong {
                gsl_error(
                    b"traverser height exceeds maximum\0" as *const u8
                        as *const libc::c_char,
                    b"avl.c\0" as *const u8 as *const libc::c_char,
                    885 as libc::c_int,
                    GSL_ETABLE as libc::c_int,
                );
                return 0 as *mut libc::c_void;
            }
            let fresh11 = (*trav).avl_height;
            (*trav).avl_height = ((*trav).avl_height).wrapping_add(1);
            (*trav).avl_stack[fresh11 as usize] = x;
            x = (*x).avl_link[1 as libc::c_int as usize];
        }
    } else {
        let mut y: *mut avl_node = 0 as *mut avl_node;
        loop {
            if (*trav).avl_height == 0 as libc::c_int as libc::c_ulong {
                (*trav).avl_node = 0 as *mut gsl_bst_avl_node;
                return 0 as *mut libc::c_void;
            }
            y = x;
            (*trav).avl_height = ((*trav).avl_height).wrapping_sub(1);
            x = (*trav).avl_stack[(*trav).avl_height as usize];
            if !(y == (*x).avl_link[0 as libc::c_int as usize]) {
                break;
            }
        }
    }
    (*trav).avl_node = x;
    return (*x).avl_data;
}
unsafe extern "C" fn avl_t_cur(mut vtrav: *const libc::c_void) -> *mut libc::c_void {
    let mut trav: *const avl_traverser = vtrav as *const avl_traverser;
    return if !((*trav).avl_node).is_null() {
        (*(*trav).avl_node).avl_data
    } else {
        0 as *mut libc::c_void
    };
}
unsafe extern "C" fn avl_t_replace(
    mut vtrav: *mut libc::c_void,
    mut new_item: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut trav: *mut avl_traverser = vtrav as *mut avl_traverser;
    let mut old: *mut libc::c_void = 0 as *mut libc::c_void;
    old = (*(*trav).avl_node).avl_data;
    (*(*trav).avl_node).avl_data = new_item;
    return old;
}
unsafe extern "C" fn avl_trav_refresh(mut trav: *mut avl_traverser) {
    (*trav).avl_generation = (*(*trav).avl_table).avl_generation;
    if !((*trav).avl_node).is_null() {
        let mut cmp: Option::<gsl_bst_cmp_function> = (*(*trav).avl_table).avl_compare;
        let mut param: *mut libc::c_void = (*(*trav).avl_table).avl_param;
        let mut node: *mut avl_node = (*trav).avl_node;
        let mut i: *mut avl_node = 0 as *mut avl_node;
        (*trav).avl_height = 0 as libc::c_int as size_t;
        i = (*(*trav).avl_table).avl_root;
        while i != node {
            if (*trav).avl_height >= 32 as libc::c_int as libc::c_ulong {
                gsl_error(
                    b"traverser height exceeds maximum\0" as *const u8
                        as *const libc::c_char,
                    b"avl.c\0" as *const u8 as *const libc::c_char,
                    963 as libc::c_int,
                    GSL_ETABLE as libc::c_int,
                );
                return;
            }
            let fresh12 = (*trav).avl_height;
            (*trav).avl_height = ((*trav).avl_height).wrapping_add(1);
            (*trav).avl_stack[fresh12 as usize] = i;
            i = (*i)
                .avl_link[(cmp.unwrap()((*node).avl_data, (*i).avl_data, param)
                > 0 as libc::c_int) as libc::c_int as usize];
        }
    }
}
static mut avl_tree_type: gsl_bst_type = unsafe {
    {
        let mut init = gsl_bst_type {
            name: b"AVL\0" as *const u8 as *const libc::c_char,
            node_size: ::std::mem::size_of::<avl_node>() as libc::c_ulong,
            init: Some(
                avl_init
                    as unsafe extern "C" fn(
                        *const gsl_bst_allocator,
                        Option::<gsl_bst_cmp_function>,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            nodes: Some(
                avl_nodes as unsafe extern "C" fn(*const libc::c_void) -> size_t,
            ),
            insert: Some(
                avl_insert
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            find: Some(
                avl_find
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            remove: Some(
                avl_remove
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            empty: Some(
                avl_empty as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
            ),
            trav_init: Some(
                avl_t_init
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            trav_first: Some(
                avl_t_first
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            trav_last: Some(
                avl_t_last
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            trav_find: Some(
                avl_t_find
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *mut libc::c_void,
                        *const libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            trav_insert: Some(
                avl_t_insert
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            trav_copy: Some(
                avl_t_copy
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            trav_next: Some(
                avl_t_next
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            trav_prev: Some(
                avl_t_prev
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            trav_cur: Some(
                avl_t_cur
                    as unsafe extern "C" fn(*const libc::c_void) -> *mut libc::c_void,
            ),
            trav_replace: Some(
                avl_t_replace
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> *mut libc::c_void,
            ),
        };
        init
    }
};
pub static mut gsl_bst_avl: *const gsl_bst_type = unsafe {
    &avl_tree_type as *const gsl_bst_type
};
