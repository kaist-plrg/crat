use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gl_list_impl {
    pub base: gl_list_impl_base,
    pub root: gl_list_node_impl,
    pub count: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gl_list_node_impl {
    pub next: *mut gl_list_node_impl,
    pub prev: *mut gl_list_node_impl,
    pub value: *const libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gl_list_impl_base {
    pub vtable: *const gl_list_implementation,
    pub equals_fn: gl_listelement_equals_fn,
    pub hashcode_fn: gl_listelement_hashcode_fn,
    pub dispose_fn: gl_listelement_dispose_fn,
    pub allow_duplicates: bool,
}
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
pub type gl_list_t = *mut gl_list_impl;
pub type gl_list_node_t = *mut gl_list_node_impl;
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
unsafe extern "C" fn gl_linked_nx_create_empty(
    mut implementation: gl_list_implementation_t,
    mut equals_fn: gl_listelement_equals_fn,
    mut hashcode_fn: gl_listelement_hashcode_fn,
    mut dispose_fn: gl_listelement_dispose_fn,
    mut allow_duplicates: bool,
) -> gl_list_t {
    let mut list: *mut gl_list_impl = malloc(
        ::std::mem::size_of::<gl_list_impl>() as libc::c_ulong,
    ) as *mut gl_list_impl;
    if list.is_null() {
        return 0 as gl_list_t;
    }
    (*list).base.vtable = implementation;
    (*list).base.equals_fn = equals_fn;
    (*list).base.hashcode_fn = hashcode_fn;
    (*list).base.dispose_fn = dispose_fn;
    (*list).base.allow_duplicates = allow_duplicates;
    (*list).root.next = &mut (*list).root;
    (*list).root.prev = &mut (*list).root;
    (*list).count = 0 as libc::c_int as size_t;
    return list;
}
unsafe extern "C" fn gl_linked_nx_create(
    mut implementation: gl_list_implementation_t,
    mut equals_fn: gl_listelement_equals_fn,
    mut hashcode_fn: gl_listelement_hashcode_fn,
    mut dispose_fn: gl_listelement_dispose_fn,
    mut allow_duplicates: bool,
    mut count: size_t,
    mut contents: *mut *const libc::c_void,
) -> gl_list_t {
    let mut current_block: u64;
    let mut list: *mut gl_list_impl = malloc(
        ::std::mem::size_of::<gl_list_impl>() as libc::c_ulong,
    ) as *mut gl_list_impl;
    let mut tail: gl_list_node_t = 0 as *mut gl_list_node_impl;
    if list.is_null() {
        return 0 as gl_list_t;
    }
    (*list).base.vtable = implementation;
    (*list).base.equals_fn = equals_fn;
    (*list).base.hashcode_fn = hashcode_fn;
    (*list).base.dispose_fn = dispose_fn;
    (*list).base.allow_duplicates = allow_duplicates;
    (*list).count = count;
    tail = &mut (*list).root;
    loop {
        if !(count > 0 as libc::c_int as libc::c_ulong) {
            current_block = 7651349459974463963;
            break;
        }
        let mut node: gl_list_node_t = malloc(
            ::std::mem::size_of::<gl_list_node_impl>() as libc::c_ulong,
        ) as *mut gl_list_node_impl;
        if node.is_null() {
            current_block = 17483768228468089392;
            break;
        }
        (*node).value = *contents;
        (*node).prev = tail;
        (*tail).next = node;
        tail = node;
        contents = contents.offset(1);
        contents;
        count = count.wrapping_sub(1);
        count;
    }
    match current_block {
        17483768228468089392 => {
            let mut node_0: gl_list_node_t = 0 as *mut gl_list_node_impl;
            node_0 = tail;
            while node_0 != &mut (*list).root as *mut gl_list_node_impl {
                let mut prev: gl_list_node_t = (*node_0).prev;
                free(node_0 as *mut libc::c_void);
                node_0 = prev;
            }
            free(list as *mut libc::c_void);
            return 0 as gl_list_t;
        }
        _ => {
            (*tail).next = &mut (*list).root;
            (*list).root.prev = tail;
            return list;
        }
    };
}
unsafe extern "C" fn gl_linked_size(mut list: gl_list_t) -> size_t {
    return (*list).count;
}
unsafe extern "C" fn gl_linked_node_value(
    mut list: gl_list_t,
    mut node: gl_list_node_t,
) -> *const libc::c_void {
    return (*node).value;
}
unsafe extern "C" fn gl_linked_node_nx_set_value(
    mut list: gl_list_t,
    mut node: gl_list_node_t,
    mut elt: *const libc::c_void,
) -> libc::c_int {
    (*node).value = elt;
    return 0 as libc::c_int;
}
unsafe extern "C" fn gl_linked_next_node(
    mut list: gl_list_t,
    mut node: gl_list_node_t,
) -> gl_list_node_t {
    return if (*node).next != &mut (*list).root as *mut gl_list_node_impl {
        (*node).next
    } else {
        0 as *mut gl_list_node_impl
    };
}
unsafe extern "C" fn gl_linked_previous_node(
    mut list: gl_list_t,
    mut node: gl_list_node_t,
) -> gl_list_node_t {
    return if (*node).prev != &mut (*list).root as *mut gl_list_node_impl {
        (*node).prev
    } else {
        0 as *mut gl_list_node_impl
    };
}
unsafe extern "C" fn gl_linked_get_at(
    mut list: gl_list_t,
    mut position: size_t,
) -> *const libc::c_void {
    let mut count: size_t = (*list).count;
    let mut node: gl_list_node_t = 0 as *mut gl_list_node_impl;
    if !(position < count) {
        abort();
    }
    if position
        <= count
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
    {
        node = (*list).root.next;
        while position > 0 as libc::c_int as libc::c_ulong {
            node = (*node).next;
            position = position.wrapping_sub(1);
            position;
        }
    } else {
        position = count
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_sub(position);
        node = (*list).root.prev;
        while position > 0 as libc::c_int as libc::c_ulong {
            node = (*node).prev;
            position = position.wrapping_sub(1);
            position;
        }
    }
    return (*node).value;
}
unsafe extern "C" fn gl_linked_nx_set_at(
    mut list: gl_list_t,
    mut position: size_t,
    mut elt: *const libc::c_void,
) -> gl_list_node_t {
    let mut count: size_t = (*list).count;
    let mut node: gl_list_node_t = 0 as *mut gl_list_node_impl;
    if !(position < count) {
        abort();
    }
    if position
        <= count
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
    {
        node = (*list).root.next;
        while position > 0 as libc::c_int as libc::c_ulong {
            node = (*node).next;
            position = position.wrapping_sub(1);
            position;
        }
    } else {
        position = count
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_sub(position);
        node = (*list).root.prev;
        while position > 0 as libc::c_int as libc::c_ulong {
            node = (*node).prev;
            position = position.wrapping_sub(1);
            position;
        }
    }
    (*node).value = elt;
    return node;
}
unsafe extern "C" fn gl_linked_search_from_to(
    mut list: gl_list_t,
    mut start_index: size_t,
    mut end_index: size_t,
    mut elt: *const libc::c_void,
) -> gl_list_node_t {
    let mut count: size_t = (*list).count;
    if !(start_index <= end_index && end_index <= count) {
        abort();
    }
    let mut equals: gl_listelement_equals_fn = (*list).base.equals_fn;
    let mut node: gl_list_node_t = (*list).root.next;
    end_index = (end_index as libc::c_ulong).wrapping_sub(start_index) as size_t
        as size_t;
    while start_index > 0 as libc::c_int as libc::c_ulong {
        node = (*node).next;
        start_index = start_index.wrapping_sub(1);
        start_index;
    }
    if equals.is_some() {
        while end_index > 0 as libc::c_int as libc::c_ulong {
            if equals.unwrap()(elt, (*node).value) {
                return node;
            }
            node = (*node).next;
            end_index = end_index.wrapping_sub(1);
            end_index;
        }
    } else {
        while end_index > 0 as libc::c_int as libc::c_ulong {
            if elt == (*node).value {
                return node;
            }
            node = (*node).next;
            end_index = end_index.wrapping_sub(1);
            end_index;
        }
    }
    return 0 as gl_list_node_t;
}
unsafe extern "C" fn gl_linked_indexof_from_to(
    mut list: gl_list_t,
    mut start_index: size_t,
    mut end_index: size_t,
    mut elt: *const libc::c_void,
) -> size_t {
    let mut count: size_t = (*list).count;
    if !(start_index <= end_index && end_index <= count) {
        abort();
    }
    let mut equals: gl_listelement_equals_fn = (*list).base.equals_fn;
    let mut index: size_t = start_index;
    let mut node: gl_list_node_t = (*list).root.next;
    while start_index > 0 as libc::c_int as libc::c_ulong {
        node = (*node).next;
        start_index = start_index.wrapping_sub(1);
        start_index;
    }
    if equals.is_some() {
        while index < end_index {
            if equals.unwrap()(elt, (*node).value) {
                return index;
            }
            node = (*node).next;
            index = index.wrapping_add(1);
            index;
        }
    } else {
        while index < end_index {
            if elt == (*node).value {
                return index;
            }
            node = (*node).next;
            index = index.wrapping_add(1);
            index;
        }
    }
    return -(1 as libc::c_int) as size_t;
}
unsafe extern "C" fn gl_linked_nx_add_first(
    mut list: gl_list_t,
    mut elt: *const libc::c_void,
) -> gl_list_node_t {
    let mut node: gl_list_node_t = malloc(
        ::std::mem::size_of::<gl_list_node_impl>() as libc::c_ulong,
    ) as *mut gl_list_node_impl;
    if node.is_null() {
        return 0 as gl_list_node_t;
    }
    (*node).value = elt;
    (*node).prev = &mut (*list).root;
    (*node).next = (*list).root.next;
    (*(*node).next).prev = node;
    (*list).root.next = node;
    (*list).count = ((*list).count).wrapping_add(1);
    (*list).count;
    return node;
}
unsafe extern "C" fn gl_linked_nx_add_last(
    mut list: gl_list_t,
    mut elt: *const libc::c_void,
) -> gl_list_node_t {
    let mut node: gl_list_node_t = malloc(
        ::std::mem::size_of::<gl_list_node_impl>() as libc::c_ulong,
    ) as *mut gl_list_node_impl;
    if node.is_null() {
        return 0 as gl_list_node_t;
    }
    (*node).value = elt;
    (*node).next = &mut (*list).root;
    (*node).prev = (*list).root.prev;
    (*(*node).prev).next = node;
    (*list).root.prev = node;
    (*list).count = ((*list).count).wrapping_add(1);
    (*list).count;
    return node;
}
unsafe extern "C" fn gl_linked_nx_add_before(
    mut list: gl_list_t,
    mut node: gl_list_node_t,
    mut elt: *const libc::c_void,
) -> gl_list_node_t {
    let mut new_node: gl_list_node_t = malloc(
        ::std::mem::size_of::<gl_list_node_impl>() as libc::c_ulong,
    ) as *mut gl_list_node_impl;
    if new_node.is_null() {
        return 0 as gl_list_node_t;
    }
    (*new_node).value = elt;
    (*new_node).next = node;
    (*new_node).prev = (*node).prev;
    (*(*new_node).prev).next = new_node;
    (*node).prev = new_node;
    (*list).count = ((*list).count).wrapping_add(1);
    (*list).count;
    return new_node;
}
unsafe extern "C" fn gl_linked_nx_add_after(
    mut list: gl_list_t,
    mut node: gl_list_node_t,
    mut elt: *const libc::c_void,
) -> gl_list_node_t {
    let mut new_node: gl_list_node_t = malloc(
        ::std::mem::size_of::<gl_list_node_impl>() as libc::c_ulong,
    ) as *mut gl_list_node_impl;
    if new_node.is_null() {
        return 0 as gl_list_node_t;
    }
    (*new_node).value = elt;
    (*new_node).prev = node;
    (*new_node).next = (*node).next;
    (*(*new_node).next).prev = new_node;
    (*node).next = new_node;
    (*list).count = ((*list).count).wrapping_add(1);
    (*list).count;
    return new_node;
}
unsafe extern "C" fn gl_linked_nx_add_at(
    mut list: gl_list_t,
    mut position: size_t,
    mut elt: *const libc::c_void,
) -> gl_list_node_t {
    let mut count: size_t = (*list).count;
    let mut new_node: gl_list_node_t = 0 as *mut gl_list_node_impl;
    if !(position <= count) {
        abort();
    }
    new_node = malloc(::std::mem::size_of::<gl_list_node_impl>() as libc::c_ulong)
        as *mut gl_list_node_impl;
    if new_node.is_null() {
        return 0 as gl_list_node_t;
    }
    (*new_node).value = elt;
    if position <= count.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut node: gl_list_node_t = 0 as *mut gl_list_node_impl;
        node = &mut (*list).root;
        while position > 0 as libc::c_int as libc::c_ulong {
            node = (*node).next;
            position = position.wrapping_sub(1);
            position;
        }
        (*new_node).prev = node;
        (*new_node).next = (*node).next;
        (*(*new_node).next).prev = new_node;
        (*node).next = new_node;
    } else {
        let mut node_0: gl_list_node_t = 0 as *mut gl_list_node_impl;
        position = count.wrapping_sub(position);
        node_0 = &mut (*list).root;
        while position > 0 as libc::c_int as libc::c_ulong {
            node_0 = (*node_0).prev;
            position = position.wrapping_sub(1);
            position;
        }
        (*new_node).next = node_0;
        (*new_node).prev = (*node_0).prev;
        (*(*new_node).prev).next = new_node;
        (*node_0).prev = new_node;
    }
    (*list).count = ((*list).count).wrapping_add(1);
    (*list).count;
    return new_node;
}
unsafe extern "C" fn gl_linked_remove_node(
    mut list: gl_list_t,
    mut node: gl_list_node_t,
) -> bool {
    let mut prev: gl_list_node_t = 0 as *mut gl_list_node_impl;
    let mut next: gl_list_node_t = 0 as *mut gl_list_node_impl;
    prev = (*node).prev;
    next = (*node).next;
    (*prev).next = next;
    (*next).prev = prev;
    (*list).count = ((*list).count).wrapping_sub(1);
    (*list).count;
    if ((*list).base.dispose_fn).is_some() {
        ((*list).base.dispose_fn).unwrap()((*node).value);
    }
    free(node as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn gl_linked_remove_at(
    mut list: gl_list_t,
    mut position: size_t,
) -> bool {
    let mut count: size_t = (*list).count;
    let mut removed_node: gl_list_node_t = 0 as *mut gl_list_node_impl;
    if !(position < count) {
        abort();
    }
    if position
        <= count
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
    {
        let mut node: gl_list_node_t = 0 as *mut gl_list_node_impl;
        let mut after_removed: gl_list_node_t = 0 as *mut gl_list_node_impl;
        node = &mut (*list).root;
        while position > 0 as libc::c_int as libc::c_ulong {
            node = (*node).next;
            position = position.wrapping_sub(1);
            position;
        }
        removed_node = (*node).next;
        after_removed = (*(*node).next).next;
        (*node).next = after_removed;
        (*after_removed).prev = node;
    } else {
        let mut node_0: gl_list_node_t = 0 as *mut gl_list_node_impl;
        let mut before_removed: gl_list_node_t = 0 as *mut gl_list_node_impl;
        position = count
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_sub(position);
        node_0 = &mut (*list).root;
        while position > 0 as libc::c_int as libc::c_ulong {
            node_0 = (*node_0).prev;
            position = position.wrapping_sub(1);
            position;
        }
        removed_node = (*node_0).prev;
        before_removed = (*(*node_0).prev).prev;
        (*node_0).prev = before_removed;
        (*before_removed).next = node_0;
    }
    (*list).count = ((*list).count).wrapping_sub(1);
    (*list).count;
    if ((*list).base.dispose_fn).is_some() {
        ((*list).base.dispose_fn).unwrap()((*removed_node).value);
    }
    free(removed_node as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn gl_linked_remove(
    mut list: gl_list_t,
    mut elt: *const libc::c_void,
) -> bool {
    let mut node: gl_list_node_t = gl_linked_search_from_to(
        list,
        0 as libc::c_int as size_t,
        (*list).count,
        elt,
    );
    if !node.is_null() {
        return gl_linked_remove_node(list, node)
    } else {
        return 0 as libc::c_int != 0
    };
}
unsafe extern "C" fn gl_linked_list_free(mut list: gl_list_t) {
    let mut dispose: gl_listelement_dispose_fn = (*list).base.dispose_fn;
    let mut node: gl_list_node_t = 0 as *mut gl_list_node_impl;
    node = (*list).root.next;
    while node != &mut (*list).root as *mut gl_list_node_impl {
        let mut next: gl_list_node_t = (*node).next;
        if dispose.is_some() {
            dispose.unwrap()((*node).value);
        }
        free(node as *mut libc::c_void);
        node = next;
    }
    free(list as *mut libc::c_void);
}
unsafe extern "C" fn gl_linked_iterator(mut list: gl_list_t) -> gl_list_iterator_t {
    let mut result: gl_list_iterator_t = gl_list_iterator_t {
        vtable: 0 as *const gl_list_implementation,
        list: 0 as *mut gl_list_impl,
        count: 0,
        p: 0 as *mut libc::c_void,
        q: 0 as *mut libc::c_void,
        i: 0,
        j: 0,
    };
    result.vtable = (*list).base.vtable;
    result.list = list;
    result.p = (*list).root.next as *mut libc::c_void;
    result.q = &mut (*list).root as *mut gl_list_node_impl as *mut libc::c_void;
    return result;
}
unsafe extern "C" fn gl_linked_iterator_from_to(
    mut list: gl_list_t,
    mut start_index: size_t,
    mut end_index: size_t,
) -> gl_list_iterator_t {
    let mut result: gl_list_iterator_t = gl_list_iterator_t {
        vtable: 0 as *const gl_list_implementation,
        list: 0 as *mut gl_list_impl,
        count: 0,
        p: 0 as *mut libc::c_void,
        q: 0 as *mut libc::c_void,
        i: 0,
        j: 0,
    };
    let mut n1: size_t = 0;
    let mut n2: size_t = 0;
    let mut n3: size_t = 0;
    if !(start_index <= end_index && end_index <= (*list).count) {
        abort();
    }
    result.vtable = (*list).base.vtable;
    result.list = list;
    n1 = start_index;
    n2 = end_index.wrapping_sub(start_index);
    n3 = ((*list).count).wrapping_sub(end_index);
    if n1 > n2 && n1 > n3 {
        let mut node: gl_list_node_t = 0 as *mut gl_list_node_impl;
        let mut i: size_t = 0;
        node = &mut (*list).root;
        i = n3;
        while i > 0 as libc::c_int as libc::c_ulong {
            node = (*node).prev;
            i = i.wrapping_sub(1);
            i;
        }
        result.q = node as *mut libc::c_void;
        i = n2;
        while i > 0 as libc::c_int as libc::c_ulong {
            node = (*node).prev;
            i = i.wrapping_sub(1);
            i;
        }
        result.p = node as *mut libc::c_void;
    } else if n2 > n3 {
        let mut node_0: gl_list_node_t = 0 as *mut gl_list_node_impl;
        let mut i_0: size_t = 0;
        node_0 = (*list).root.next;
        i_0 = n1;
        while i_0 > 0 as libc::c_int as libc::c_ulong {
            node_0 = (*node_0).next;
            i_0 = i_0.wrapping_sub(1);
            i_0;
        }
        result.p = node_0 as *mut libc::c_void;
        node_0 = &mut (*list).root;
        i_0 = n3;
        while i_0 > 0 as libc::c_int as libc::c_ulong {
            node_0 = (*node_0).prev;
            i_0 = i_0.wrapping_sub(1);
            i_0;
        }
        result.q = node_0 as *mut libc::c_void;
    } else {
        let mut node_1: gl_list_node_t = 0 as *mut gl_list_node_impl;
        let mut i_1: size_t = 0;
        node_1 = (*list).root.next;
        i_1 = n1;
        while i_1 > 0 as libc::c_int as libc::c_ulong {
            node_1 = (*node_1).next;
            i_1 = i_1.wrapping_sub(1);
            i_1;
        }
        result.p = node_1 as *mut libc::c_void;
        i_1 = n2;
        while i_1 > 0 as libc::c_int as libc::c_ulong {
            node_1 = (*node_1).next;
            i_1 = i_1.wrapping_sub(1);
            i_1;
        }
        result.q = node_1 as *mut libc::c_void;
    }
    return result;
}
unsafe extern "C" fn gl_linked_iterator_next(
    mut iterator: *mut gl_list_iterator_t,
    mut eltp: *mut *const libc::c_void,
    mut nodep: *mut gl_list_node_t,
) -> bool {
    if (*iterator).p != (*iterator).q {
        let mut node: gl_list_node_t = (*iterator).p as gl_list_node_t;
        *eltp = (*node).value;
        if !nodep.is_null() {
            *nodep = node;
        }
        (*iterator).p = (*node).next as *mut libc::c_void;
        return 1 as libc::c_int != 0;
    } else {
        return 0 as libc::c_int != 0
    };
}
unsafe extern "C" fn gl_linked_iterator_free(mut iterator: *mut gl_list_iterator_t) {}
unsafe extern "C" fn gl_linked_sortedlist_search(
    mut list: gl_list_t,
    mut compar: gl_listelement_compar_fn,
    mut elt: *const libc::c_void,
) -> gl_list_node_t {
    let mut node: gl_list_node_t = 0 as *mut gl_list_node_impl;
    node = (*list).root.next;
    while node != &mut (*list).root as *mut gl_list_node_impl {
        let mut cmp: libc::c_int = compar.unwrap()((*node).value, elt);
        if cmp > 0 as libc::c_int {
            break;
        }
        if cmp == 0 as libc::c_int {
            return node;
        }
        node = (*node).next;
    }
    return 0 as gl_list_node_t;
}
unsafe extern "C" fn gl_linked_sortedlist_search_from_to(
    mut list: gl_list_t,
    mut compar: gl_listelement_compar_fn,
    mut low: size_t,
    mut high: size_t,
    mut elt: *const libc::c_void,
) -> gl_list_node_t {
    let mut count: size_t = (*list).count;
    if !(low <= high && high <= (*list).count) {
        abort();
    }
    high = (high as libc::c_ulong).wrapping_sub(low) as size_t as size_t;
    if high > 0 as libc::c_int as libc::c_ulong {
        let mut position: size_t = low;
        let mut node: gl_list_node_t = 0 as *mut gl_list_node_impl;
        if position
            <= count
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_div(2 as libc::c_int as libc::c_ulong)
        {
            node = (*list).root.next;
            while position > 0 as libc::c_int as libc::c_ulong {
                node = (*node).next;
                position = position.wrapping_sub(1);
                position;
            }
        } else {
            position = count
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_sub(position);
            node = (*list).root.prev;
            while position > 0 as libc::c_int as libc::c_ulong {
                node = (*node).prev;
                position = position.wrapping_sub(1);
                position;
            }
        }
        loop {
            let mut cmp: libc::c_int = compar.unwrap()((*node).value, elt);
            if cmp > 0 as libc::c_int {
                break;
            }
            if cmp == 0 as libc::c_int {
                return node;
            }
            node = (*node).next;
            high = high.wrapping_sub(1);
            if !(high > 0 as libc::c_int as libc::c_ulong) {
                break;
            }
        }
    }
    return 0 as gl_list_node_t;
}
unsafe extern "C" fn gl_linked_sortedlist_indexof(
    mut list: gl_list_t,
    mut compar: gl_listelement_compar_fn,
    mut elt: *const libc::c_void,
) -> size_t {
    let mut node: gl_list_node_t = 0 as *mut gl_list_node_impl;
    let mut index: size_t = 0;
    node = (*list).root.next;
    index = 0 as libc::c_int as size_t;
    while node != &mut (*list).root as *mut gl_list_node_impl {
        let mut cmp: libc::c_int = compar.unwrap()((*node).value, elt);
        if cmp > 0 as libc::c_int {
            break;
        }
        if cmp == 0 as libc::c_int {
            return index;
        }
        node = (*node).next;
        index = index.wrapping_add(1);
        index;
    }
    return -(1 as libc::c_int) as size_t;
}
unsafe extern "C" fn gl_linked_sortedlist_indexof_from_to(
    mut list: gl_list_t,
    mut compar: gl_listelement_compar_fn,
    mut low: size_t,
    mut high: size_t,
    mut elt: *const libc::c_void,
) -> size_t {
    let mut count: size_t = (*list).count;
    if !(low <= high && high <= (*list).count) {
        abort();
    }
    high = (high as libc::c_ulong).wrapping_sub(low) as size_t as size_t;
    if high > 0 as libc::c_int as libc::c_ulong {
        let mut index: size_t = low;
        let mut position: size_t = low;
        let mut node: gl_list_node_t = 0 as *mut gl_list_node_impl;
        if position
            <= count
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_div(2 as libc::c_int as libc::c_ulong)
        {
            node = (*list).root.next;
            while position > 0 as libc::c_int as libc::c_ulong {
                node = (*node).next;
                position = position.wrapping_sub(1);
                position;
            }
        } else {
            position = count
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_sub(position);
            node = (*list).root.prev;
            while position > 0 as libc::c_int as libc::c_ulong {
                node = (*node).prev;
                position = position.wrapping_sub(1);
                position;
            }
        }
        loop {
            let mut cmp: libc::c_int = compar.unwrap()((*node).value, elt);
            if cmp > 0 as libc::c_int {
                break;
            }
            if cmp == 0 as libc::c_int {
                return index;
            }
            node = (*node).next;
            index = index.wrapping_add(1);
            index;
            high = high.wrapping_sub(1);
            if !(high > 0 as libc::c_int as libc::c_ulong) {
                break;
            }
        }
    }
    return -(1 as libc::c_int) as size_t;
}
unsafe extern "C" fn gl_linked_sortedlist_nx_add(
    mut list: gl_list_t,
    mut compar: gl_listelement_compar_fn,
    mut elt: *const libc::c_void,
) -> gl_list_node_t {
    let mut node: gl_list_node_t = 0 as *mut gl_list_node_impl;
    node = (*list).root.next;
    while node != &mut (*list).root as *mut gl_list_node_impl {
        if compar.unwrap()((*node).value, elt) >= 0 as libc::c_int {
            return gl_linked_nx_add_before(list, node, elt);
        }
        node = (*node).next;
    }
    return gl_linked_nx_add_last(list, elt);
}
unsafe extern "C" fn gl_linked_sortedlist_remove(
    mut list: gl_list_t,
    mut compar: gl_listelement_compar_fn,
    mut elt: *const libc::c_void,
) -> bool {
    let mut node: gl_list_node_t = 0 as *mut gl_list_node_impl;
    node = (*list).root.next;
    while node != &mut (*list).root as *mut gl_list_node_impl {
        let mut cmp: libc::c_int = compar.unwrap()((*node).value, elt);
        if cmp > 0 as libc::c_int {
            break;
        }
        if cmp == 0 as libc::c_int {
            return gl_linked_remove_node(list, node);
        }
        node = (*node).next;
    }
    return 0 as libc::c_int != 0;
}
pub static mut gl_linked_list_implementation: gl_list_implementation = unsafe {
    {
        let mut init = gl_list_implementation {
            nx_create_empty: Some(
                gl_linked_nx_create_empty
                    as unsafe extern "C" fn(
                        gl_list_implementation_t,
                        gl_listelement_equals_fn,
                        gl_listelement_hashcode_fn,
                        gl_listelement_dispose_fn,
                        bool,
                    ) -> gl_list_t,
            ),
            nx_create: Some(
                gl_linked_nx_create
                    as unsafe extern "C" fn(
                        gl_list_implementation_t,
                        gl_listelement_equals_fn,
                        gl_listelement_hashcode_fn,
                        gl_listelement_dispose_fn,
                        bool,
                        size_t,
                        *mut *const libc::c_void,
                    ) -> gl_list_t,
            ),
            size: Some(gl_linked_size as unsafe extern "C" fn(gl_list_t) -> size_t),
            node_value: Some(
                gl_linked_node_value
                    as unsafe extern "C" fn(
                        gl_list_t,
                        gl_list_node_t,
                    ) -> *const libc::c_void,
            ),
            node_nx_set_value: Some(
                gl_linked_node_nx_set_value
                    as unsafe extern "C" fn(
                        gl_list_t,
                        gl_list_node_t,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            next_node: Some(
                gl_linked_next_node
                    as unsafe extern "C" fn(gl_list_t, gl_list_node_t) -> gl_list_node_t,
            ),
            previous_node: Some(
                gl_linked_previous_node
                    as unsafe extern "C" fn(gl_list_t, gl_list_node_t) -> gl_list_node_t,
            ),
            get_at: Some(
                gl_linked_get_at
                    as unsafe extern "C" fn(gl_list_t, size_t) -> *const libc::c_void,
            ),
            nx_set_at: Some(
                gl_linked_nx_set_at
                    as unsafe extern "C" fn(
                        gl_list_t,
                        size_t,
                        *const libc::c_void,
                    ) -> gl_list_node_t,
            ),
            search_from_to: Some(
                gl_linked_search_from_to
                    as unsafe extern "C" fn(
                        gl_list_t,
                        size_t,
                        size_t,
                        *const libc::c_void,
                    ) -> gl_list_node_t,
            ),
            indexof_from_to: Some(
                gl_linked_indexof_from_to
                    as unsafe extern "C" fn(
                        gl_list_t,
                        size_t,
                        size_t,
                        *const libc::c_void,
                    ) -> size_t,
            ),
            nx_add_first: Some(
                gl_linked_nx_add_first
                    as unsafe extern "C" fn(
                        gl_list_t,
                        *const libc::c_void,
                    ) -> gl_list_node_t,
            ),
            nx_add_last: Some(
                gl_linked_nx_add_last
                    as unsafe extern "C" fn(
                        gl_list_t,
                        *const libc::c_void,
                    ) -> gl_list_node_t,
            ),
            nx_add_before: Some(
                gl_linked_nx_add_before
                    as unsafe extern "C" fn(
                        gl_list_t,
                        gl_list_node_t,
                        *const libc::c_void,
                    ) -> gl_list_node_t,
            ),
            nx_add_after: Some(
                gl_linked_nx_add_after
                    as unsafe extern "C" fn(
                        gl_list_t,
                        gl_list_node_t,
                        *const libc::c_void,
                    ) -> gl_list_node_t,
            ),
            nx_add_at: Some(
                gl_linked_nx_add_at
                    as unsafe extern "C" fn(
                        gl_list_t,
                        size_t,
                        *const libc::c_void,
                    ) -> gl_list_node_t,
            ),
            remove_node: Some(
                gl_linked_remove_node
                    as unsafe extern "C" fn(gl_list_t, gl_list_node_t) -> bool,
            ),
            remove_at: Some(
                gl_linked_remove_at as unsafe extern "C" fn(gl_list_t, size_t) -> bool,
            ),
            remove_elt: Some(
                gl_linked_remove
                    as unsafe extern "C" fn(gl_list_t, *const libc::c_void) -> bool,
            ),
            list_free: Some(
                gl_linked_list_free as unsafe extern "C" fn(gl_list_t) -> (),
            ),
            iterator: Some(
                gl_linked_iterator
                    as unsafe extern "C" fn(gl_list_t) -> gl_list_iterator_t,
            ),
            iterator_from_to: Some(
                gl_linked_iterator_from_to
                    as unsafe extern "C" fn(
                        gl_list_t,
                        size_t,
                        size_t,
                    ) -> gl_list_iterator_t,
            ),
            iterator_next: Some(
                gl_linked_iterator_next
                    as unsafe extern "C" fn(
                        *mut gl_list_iterator_t,
                        *mut *const libc::c_void,
                        *mut gl_list_node_t,
                    ) -> bool,
            ),
            iterator_free: Some(
                gl_linked_iterator_free
                    as unsafe extern "C" fn(*mut gl_list_iterator_t) -> (),
            ),
            sortedlist_search: Some(
                gl_linked_sortedlist_search
                    as unsafe extern "C" fn(
                        gl_list_t,
                        gl_listelement_compar_fn,
                        *const libc::c_void,
                    ) -> gl_list_node_t,
            ),
            sortedlist_search_from_to: Some(
                gl_linked_sortedlist_search_from_to
                    as unsafe extern "C" fn(
                        gl_list_t,
                        gl_listelement_compar_fn,
                        size_t,
                        size_t,
                        *const libc::c_void,
                    ) -> gl_list_node_t,
            ),
            sortedlist_indexof: Some(
                gl_linked_sortedlist_indexof
                    as unsafe extern "C" fn(
                        gl_list_t,
                        gl_listelement_compar_fn,
                        *const libc::c_void,
                    ) -> size_t,
            ),
            sortedlist_indexof_from_to: Some(
                gl_linked_sortedlist_indexof_from_to
                    as unsafe extern "C" fn(
                        gl_list_t,
                        gl_listelement_compar_fn,
                        size_t,
                        size_t,
                        *const libc::c_void,
                    ) -> size_t,
            ),
            sortedlist_nx_add: Some(
                gl_linked_sortedlist_nx_add
                    as unsafe extern "C" fn(
                        gl_list_t,
                        gl_listelement_compar_fn,
                        *const libc::c_void,
                    ) -> gl_list_node_t,
            ),
            sortedlist_remove: Some(
                gl_linked_sortedlist_remove
                    as unsafe extern "C" fn(
                        gl_list_t,
                        gl_listelement_compar_fn,
                        *const libc::c_void,
                    ) -> bool,
            ),
        };
        init
    }
};
