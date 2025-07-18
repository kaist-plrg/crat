use ::libc;
extern "C" {
    pub type gl_list_impl;
    pub type gl_list_node_impl;
    fn xalloc_die();
}
pub type size_t = libc::c_ulong;
pub type gl_listelement_equals_fn = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
>;
pub type gl_listelement_hashcode_fn = Option::<
    unsafe extern "C" fn(*const libc::c_void) -> size_t,
>;
pub type gl_listelement_dispose_fn = Option::<
    unsafe extern "C" fn(*const libc::c_void) -> (),
>;
pub type gl_list_t = *mut gl_list_impl;
pub type gl_list_node_t = *mut gl_list_node_impl;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gl_list_implementation {
    pub nx_create_empty: Option::<
        unsafe extern "C" fn(
            gl_list_implementation_t,
            gl_listelement_equals_fn,
            gl_listelement_hashcode_fn,
            gl_listelement_dispose_fn,
            bool,
        ) -> gl_list_t,
    >,
    pub nx_create: Option::<
        unsafe extern "C" fn(
            gl_list_implementation_t,
            gl_listelement_equals_fn,
            gl_listelement_hashcode_fn,
            gl_listelement_dispose_fn,
            bool,
            size_t,
            *mut *const libc::c_void,
        ) -> gl_list_t,
    >,
    pub size: Option::<unsafe extern "C" fn(gl_list_t) -> size_t>,
    pub node_value: Option::<
        unsafe extern "C" fn(gl_list_t, gl_list_node_t) -> *const libc::c_void,
    >,
    pub node_nx_set_value: Option::<
        unsafe extern "C" fn(
            gl_list_t,
            gl_list_node_t,
            *const libc::c_void,
        ) -> libc::c_int,
    >,
    pub next_node: Option::<
        unsafe extern "C" fn(gl_list_t, gl_list_node_t) -> gl_list_node_t,
    >,
    pub previous_node: Option::<
        unsafe extern "C" fn(gl_list_t, gl_list_node_t) -> gl_list_node_t,
    >,
    pub get_at: Option::<unsafe extern "C" fn(gl_list_t, size_t) -> *const libc::c_void>,
    pub nx_set_at: Option::<
        unsafe extern "C" fn(gl_list_t, size_t, *const libc::c_void) -> gl_list_node_t,
    >,
    pub search_from_to: Option::<
        unsafe extern "C" fn(
            gl_list_t,
            size_t,
            size_t,
            *const libc::c_void,
        ) -> gl_list_node_t,
    >,
    pub indexof_from_to: Option::<
        unsafe extern "C" fn(gl_list_t, size_t, size_t, *const libc::c_void) -> size_t,
    >,
    pub nx_add_first: Option::<
        unsafe extern "C" fn(gl_list_t, *const libc::c_void) -> gl_list_node_t,
    >,
    pub nx_add_last: Option::<
        unsafe extern "C" fn(gl_list_t, *const libc::c_void) -> gl_list_node_t,
    >,
    pub nx_add_before: Option::<
        unsafe extern "C" fn(
            gl_list_t,
            gl_list_node_t,
            *const libc::c_void,
        ) -> gl_list_node_t,
    >,
    pub nx_add_after: Option::<
        unsafe extern "C" fn(
            gl_list_t,
            gl_list_node_t,
            *const libc::c_void,
        ) -> gl_list_node_t,
    >,
    pub nx_add_at: Option::<
        unsafe extern "C" fn(gl_list_t, size_t, *const libc::c_void) -> gl_list_node_t,
    >,
    pub remove_node: Option::<unsafe extern "C" fn(gl_list_t, gl_list_node_t) -> bool>,
    pub remove_at: Option::<unsafe extern "C" fn(gl_list_t, size_t) -> bool>,
    pub remove_elt: Option::<
        unsafe extern "C" fn(gl_list_t, *const libc::c_void) -> bool,
    >,
    pub list_free: Option::<unsafe extern "C" fn(gl_list_t) -> ()>,
    pub iterator: Option::<unsafe extern "C" fn(gl_list_t) -> gl_list_iterator_t>,
    pub iterator_from_to: Option::<
        unsafe extern "C" fn(gl_list_t, size_t, size_t) -> gl_list_iterator_t,
    >,
    pub iterator_next: Option::<
        unsafe extern "C" fn(
            *mut gl_list_iterator_t,
            *mut *const libc::c_void,
            *mut gl_list_node_t,
        ) -> bool,
    >,
    pub iterator_free: Option::<unsafe extern "C" fn(*mut gl_list_iterator_t) -> ()>,
    pub sortedlist_search: Option::<
        unsafe extern "C" fn(
            gl_list_t,
            gl_listelement_compar_fn,
            *const libc::c_void,
        ) -> gl_list_node_t,
    >,
    pub sortedlist_search_from_to: Option::<
        unsafe extern "C" fn(
            gl_list_t,
            gl_listelement_compar_fn,
            size_t,
            size_t,
            *const libc::c_void,
        ) -> gl_list_node_t,
    >,
    pub sortedlist_indexof: Option::<
        unsafe extern "C" fn(
            gl_list_t,
            gl_listelement_compar_fn,
            *const libc::c_void,
        ) -> size_t,
    >,
    pub sortedlist_indexof_from_to: Option::<
        unsafe extern "C" fn(
            gl_list_t,
            gl_listelement_compar_fn,
            size_t,
            size_t,
            *const libc::c_void,
        ) -> size_t,
    >,
    pub sortedlist_nx_add: Option::<
        unsafe extern "C" fn(
            gl_list_t,
            gl_listelement_compar_fn,
            *const libc::c_void,
        ) -> gl_list_node_t,
    >,
    pub sortedlist_remove: Option::<
        unsafe extern "C" fn(
            gl_list_t,
            gl_listelement_compar_fn,
            *const libc::c_void,
        ) -> bool,
    >,
}
pub type gl_listelement_compar_fn = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gl_list_iterator_t {
    pub vtable: *const gl_list_implementation,
    pub list: gl_list_t,
    pub count: size_t,
    pub p: *mut libc::c_void,
    pub q: *mut libc::c_void,
    pub i: size_t,
    pub j: size_t,
}
pub type gl_list_implementation_t = *const gl_list_implementation;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gl_list_impl_base {
    pub vtable: *const gl_list_implementation,
    pub equals_fn: gl_listelement_equals_fn,
    pub hashcode_fn: gl_listelement_hashcode_fn,
    pub dispose_fn: gl_listelement_dispose_fn,
    pub allow_duplicates: bool,
}
#[inline]
unsafe extern "C" fn gl_list_node_nx_set_value(
    mut list: gl_list_t,
    mut node: gl_list_node_t,
    mut elt: *const libc::c_void,
) -> libc::c_int {
    return ((*(*(list as *const gl_list_impl_base)).vtable).node_nx_set_value)
        .unwrap()(list, node, elt);
}
#[inline]
unsafe extern "C" fn gl_list_nx_create(
    mut implementation: gl_list_implementation_t,
    mut equals_fn: gl_listelement_equals_fn,
    mut hashcode_fn: gl_listelement_hashcode_fn,
    mut dispose_fn: gl_listelement_dispose_fn,
    mut allow_duplicates: bool,
    mut count: size_t,
    mut contents: *mut *const libc::c_void,
) -> gl_list_t {
    return ((*implementation).nx_create)
        .unwrap()(
        implementation,
        equals_fn,
        hashcode_fn,
        dispose_fn,
        allow_duplicates,
        count,
        contents,
    );
}
#[inline]
unsafe extern "C" fn gl_list_nx_create_empty(
    mut implementation: gl_list_implementation_t,
    mut equals_fn: gl_listelement_equals_fn,
    mut hashcode_fn: gl_listelement_hashcode_fn,
    mut dispose_fn: gl_listelement_dispose_fn,
    mut allow_duplicates: bool,
) -> gl_list_t {
    return ((*implementation).nx_create_empty)
        .unwrap()(implementation, equals_fn, hashcode_fn, dispose_fn, allow_duplicates);
}
#[inline]
unsafe extern "C" fn gl_list_nx_add_first(
    mut list: gl_list_t,
    mut elt: *const libc::c_void,
) -> gl_list_node_t {
    return ((*(*(list as *const gl_list_impl_base)).vtable).nx_add_first)
        .unwrap()(list, elt);
}
#[inline]
unsafe extern "C" fn gl_list_nx_add_at(
    mut list: gl_list_t,
    mut position: size_t,
    mut elt: *const libc::c_void,
) -> gl_list_node_t {
    return ((*(*(list as *const gl_list_impl_base)).vtable).nx_add_at)
        .unwrap()(list, position, elt);
}
#[inline]
unsafe extern "C" fn gl_list_nx_add_after(
    mut list: gl_list_t,
    mut node: gl_list_node_t,
    mut elt: *const libc::c_void,
) -> gl_list_node_t {
    return ((*(*(list as *const gl_list_impl_base)).vtable).nx_add_after)
        .unwrap()(list, node, elt);
}
#[inline]
unsafe extern "C" fn gl_list_nx_add_before(
    mut list: gl_list_t,
    mut node: gl_list_node_t,
    mut elt: *const libc::c_void,
) -> gl_list_node_t {
    return ((*(*(list as *const gl_list_impl_base)).vtable).nx_add_before)
        .unwrap()(list, node, elt);
}
#[inline]
unsafe extern "C" fn gl_list_nx_add_last(
    mut list: gl_list_t,
    mut elt: *const libc::c_void,
) -> gl_list_node_t {
    return ((*(*(list as *const gl_list_impl_base)).vtable).nx_add_last)
        .unwrap()(list, elt);
}
#[inline]
unsafe extern "C" fn gl_list_nx_set_at(
    mut list: gl_list_t,
    mut position: size_t,
    mut elt: *const libc::c_void,
) -> gl_list_node_t {
    return ((*(*(list as *const gl_list_impl_base)).vtable).nx_set_at)
        .unwrap()(list, position, elt);
}
#[inline]
unsafe extern "C" fn gl_sortedlist_nx_add(
    mut list: gl_list_t,
    mut compar: gl_listelement_compar_fn,
    mut elt: *const libc::c_void,
) -> gl_list_node_t {
    return ((*(*(list as *const gl_list_impl_base)).vtable).sortedlist_nx_add)
        .unwrap()(list, compar, elt);
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn gl_sortedlist_add(
    mut list: gl_list_t,
    mut compar: gl_listelement_compar_fn,
    mut elt: *const libc::c_void,
) -> gl_list_node_t {
    let mut result: gl_list_node_t = gl_sortedlist_nx_add(list, compar, elt);
    if result.is_null() {
        xalloc_die();
    }
    return result;
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn gl_list_add_at(
    mut list: gl_list_t,
    mut position: size_t,
    mut elt: *const libc::c_void,
) -> gl_list_node_t {
    let mut result: gl_list_node_t = gl_list_nx_add_at(list, position, elt);
    if result.is_null() {
        xalloc_die();
    }
    return result;
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn gl_list_add_after(
    mut list: gl_list_t,
    mut node: gl_list_node_t,
    mut elt: *const libc::c_void,
) -> gl_list_node_t {
    let mut result: gl_list_node_t = gl_list_nx_add_after(list, node, elt);
    if result.is_null() {
        xalloc_die();
    }
    return result;
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn gl_list_add_before(
    mut list: gl_list_t,
    mut node: gl_list_node_t,
    mut elt: *const libc::c_void,
) -> gl_list_node_t {
    let mut result: gl_list_node_t = gl_list_nx_add_before(list, node, elt);
    if result.is_null() {
        xalloc_die();
    }
    return result;
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn gl_list_add_last(
    mut list: gl_list_t,
    mut elt: *const libc::c_void,
) -> gl_list_node_t {
    let mut result: gl_list_node_t = gl_list_nx_add_last(list, elt);
    if result.is_null() {
        xalloc_die();
    }
    return result;
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn gl_list_add_first(
    mut list: gl_list_t,
    mut elt: *const libc::c_void,
) -> gl_list_node_t {
    let mut result: gl_list_node_t = gl_list_nx_add_first(list, elt);
    if result.is_null() {
        xalloc_die();
    }
    return result;
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn gl_list_set_at(
    mut list: gl_list_t,
    mut position: size_t,
    mut elt: *const libc::c_void,
) -> gl_list_node_t {
    let mut result: gl_list_node_t = gl_list_nx_set_at(list, position, elt);
    if result.is_null() {
        xalloc_die();
    }
    return result;
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn gl_list_node_set_value(
    mut list: gl_list_t,
    mut node: gl_list_node_t,
    mut elt: *const libc::c_void,
) {
    let mut result: libc::c_int = gl_list_node_nx_set_value(list, node, elt);
    if result < 0 as libc::c_int {
        xalloc_die();
    }
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn gl_list_create(
    mut implementation: gl_list_implementation_t,
    mut equals_fn: gl_listelement_equals_fn,
    mut hashcode_fn: gl_listelement_hashcode_fn,
    mut dispose_fn: gl_listelement_dispose_fn,
    mut allow_duplicates: bool,
    mut count: size_t,
    mut contents: *mut *const libc::c_void,
) -> gl_list_t {
    let mut result: gl_list_t = gl_list_nx_create(
        implementation,
        equals_fn,
        hashcode_fn,
        dispose_fn,
        allow_duplicates,
        count,
        contents,
    );
    if result.is_null() {
        xalloc_die();
    }
    return result;
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn gl_list_create_empty(
    mut implementation: gl_list_implementation_t,
    mut equals_fn: gl_listelement_equals_fn,
    mut hashcode_fn: gl_listelement_hashcode_fn,
    mut dispose_fn: gl_listelement_dispose_fn,
    mut allow_duplicates: bool,
) -> gl_list_t {
    let mut result: gl_list_t = gl_list_nx_create_empty(
        implementation,
        equals_fn,
        hashcode_fn,
        dispose_fn,
        allow_duplicates,
    );
    if result.is_null() {
        xalloc_die();
    }
    return result;
}
