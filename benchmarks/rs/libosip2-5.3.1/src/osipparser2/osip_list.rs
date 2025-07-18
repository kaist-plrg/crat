use ::libc;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    static mut osip_free_func: Option::<osip_free_func_t>;
    static mut osip_malloc_func: Option::<osip_malloc_func_t>;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __node {
    pub next: *mut __node_t,
    pub element: *mut libc::c_void,
}
pub type __node_t = __node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_list {
    pub nb_elt: libc::c_int,
    pub node: *mut __node_t,
}
pub type osip_list_t = osip_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_list_iterator {
    pub actual: *mut __node_t,
    pub prev: *mut *mut __node_t,
    pub li: *mut osip_list_t,
    pub pos: libc::c_int,
}
pub type osip_list_iterator_t = osip_list_iterator;
pub type osip_free_func_t = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type osip_malloc_func_t = unsafe extern "C" fn(size_t) -> *mut libc::c_void;
pub unsafe extern "C" fn osip_list_init(mut li: *mut osip_list_t) -> libc::c_int {
    if li.is_null() {
        return -(2 as libc::c_int);
    }
    memset(
        li as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<osip_list_t>() as libc::c_ulong,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_list_clone(
    mut src: *const osip_list_t,
    mut dst: *mut osip_list_t,
    mut clone_func: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut *mut libc::c_void) -> libc::c_int,
    >,
) -> libc::c_int {
    let mut data: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut data2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: libc::c_int = 0;
    let mut iterator: osip_list_iterator_t = osip_list_iterator_t {
        actual: 0 as *mut __node_t,
        prev: 0 as *mut *mut __node_t,
        li: 0 as *mut osip_list_t,
        pos: 0,
    };
    data = osip_list_get_first(src as *mut osip_list_t, &mut iterator);
    while !(iterator.actual).is_null() && iterator.pos < (*iterator.li).nb_elt {
        i = clone_func.unwrap()(data, &mut data2);
        if i != 0 as libc::c_int {
            return i;
        }
        osip_list_add(dst, data2, -(1 as libc::c_int));
        data = osip_list_get_next(&mut iterator);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_list_special_free(
    mut li: *mut osip_list_t,
    mut free_func: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
) {
    let mut element: *mut libc::c_void = 0 as *mut libc::c_void;
    if li.is_null() {
        return;
    }
    while osip_list_eol(li, 0 as libc::c_int) == 0 {
        element = osip_list_get(li, 0 as libc::c_int);
        osip_list_remove(li, 0 as libc::c_int);
        if free_func.is_some() {
            free_func.unwrap()(element);
        }
    }
}
pub unsafe extern "C" fn osip_list_ofchar_free(mut li: *mut osip_list_t) {
    let mut chain: *mut libc::c_char = 0 as *mut libc::c_char;
    if li.is_null() {
        return;
    }
    while osip_list_eol(li, 0 as libc::c_int) == 0 {
        chain = osip_list_get(li, 0 as libc::c_int) as *mut libc::c_char;
        osip_list_remove(li, 0 as libc::c_int);
        if !chain.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(chain as *mut libc::c_void);
            } else {
                free(chain as *mut libc::c_void);
            }
        }
    }
}
pub unsafe extern "C" fn osip_list_size(mut li: *const osip_list_t) -> libc::c_int {
    if li.is_null() {
        return -(2 as libc::c_int);
    }
    return (*li).nb_elt;
}
pub unsafe extern "C" fn osip_list_eol(
    mut li: *const osip_list_t,
    mut i: libc::c_int,
) -> libc::c_int {
    if li.is_null() {
        return -(2 as libc::c_int);
    }
    if i < (*li).nb_elt {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn osip_list_add(
    mut li: *mut osip_list_t,
    mut el: *mut libc::c_void,
    mut pos: libc::c_int,
) -> libc::c_int {
    let mut ntmp: *mut __node_t = 0 as *mut __node_t;
    let mut i: libc::c_int = 0 as libc::c_int;
    if li.is_null() {
        return -(2 as libc::c_int);
    }
    if (*li).nb_elt == 0 as libc::c_int {
        (*li)
            .node = (if osip_malloc_func.is_some() {
            osip_malloc_func.unwrap()(::std::mem::size_of::<__node_t>() as libc::c_ulong)
        } else {
            malloc(::std::mem::size_of::<__node_t>() as libc::c_ulong)
        }) as *mut __node_t;
        if ((*li).node).is_null() {
            return -(4 as libc::c_int);
        }
        (*(*li).node).element = el;
        (*(*li).node).next = 0 as *mut __node_t;
        (*li).nb_elt += 1;
        (*li).nb_elt;
        return (*li).nb_elt;
    }
    if pos == -(1 as libc::c_int) || pos >= (*li).nb_elt {
        pos = (*li).nb_elt;
    }
    ntmp = (*li).node;
    if pos == 0 as libc::c_int {
        (*li)
            .node = (if osip_malloc_func.is_some() {
            osip_malloc_func.unwrap()(::std::mem::size_of::<__node_t>() as libc::c_ulong)
        } else {
            malloc(::std::mem::size_of::<__node_t>() as libc::c_ulong)
        }) as *mut __node_t;
        if ((*li).node).is_null() {
            (*li).node = ntmp;
            return -(4 as libc::c_int);
        }
        (*(*li).node).element = el;
        (*(*li).node).next = ntmp;
        (*li).nb_elt += 1;
        (*li).nb_elt;
        return (*li).nb_elt;
    }
    while pos > i + 1 as libc::c_int {
        i += 1;
        i;
        ntmp = (*ntmp).next;
    }
    if pos == (*li).nb_elt {
        (*ntmp)
            .next = (if osip_malloc_func.is_some() {
            osip_malloc_func.unwrap()(::std::mem::size_of::<__node_t>() as libc::c_ulong)
        } else {
            malloc(::std::mem::size_of::<__node_t>() as libc::c_ulong)
        }) as *mut __node_t;
        if ((*ntmp).next).is_null() {
            return -(4 as libc::c_int);
        }
        ntmp = (*ntmp).next;
        (*ntmp).element = el;
        (*ntmp).next = 0 as *mut __node_t;
        (*li).nb_elt += 1;
        (*li).nb_elt;
        return (*li).nb_elt;
    }
    let mut nextnode: *mut __node_t = (*ntmp).next;
    (*ntmp)
        .next = (if osip_malloc_func.is_some() {
        osip_malloc_func.unwrap()(::std::mem::size_of::<__node_t>() as libc::c_ulong)
    } else {
        malloc(::std::mem::size_of::<__node_t>() as libc::c_ulong)
    }) as *mut __node_t;
    if ((*ntmp).next).is_null() {
        (*ntmp).next = nextnode;
        return -(4 as libc::c_int);
    }
    ntmp = (*ntmp).next;
    (*ntmp).element = el;
    (*ntmp).next = nextnode;
    (*li).nb_elt += 1;
    (*li).nb_elt;
    return (*li).nb_elt;
}
pub unsafe extern "C" fn osip_list_get(
    mut li: *const osip_list_t,
    mut pos: libc::c_int,
) -> *mut libc::c_void {
    let mut ntmp: *mut __node_t = 0 as *mut __node_t;
    let mut i: libc::c_int = 0 as libc::c_int;
    if li.is_null() {
        return 0 as *mut libc::c_void;
    }
    if pos < 0 as libc::c_int || pos >= (*li).nb_elt {
        return 0 as *mut libc::c_void;
    }
    ntmp = (*li).node;
    while pos > i {
        i += 1;
        i;
        ntmp = (*ntmp).next;
    }
    return (*ntmp).element;
}
pub unsafe extern "C" fn osip_list_get_first(
    mut li: *const osip_list_t,
    mut iterator: *mut osip_list_iterator_t,
) -> *mut libc::c_void {
    if li.is_null() || 0 as libc::c_int >= (*li).nb_elt {
        (*iterator).actual = 0 as *mut __node_t;
        return 0 as *mut libc::c_void;
    }
    (*iterator).actual = (*li).node;
    (*iterator).prev = &(*li).node as *const *mut __node_t as *mut *mut __node_t;
    (*iterator).li = li as *mut osip_list_t;
    (*iterator).pos = 0 as libc::c_int;
    return (*(*li).node).element;
}
pub unsafe extern "C" fn osip_list_get_next(
    mut iterator: *mut osip_list_iterator_t,
) -> *mut libc::c_void {
    if ((*iterator).actual).is_null() {
        return 0 as *mut libc::c_void;
    }
    (*iterator).prev = &mut (*(*iterator).actual).next;
    (*iterator).actual = (*(*iterator).actual).next;
    (*iterator).pos += 1;
    (*iterator).pos;
    if !((*iterator).actual).is_null() && (*iterator).pos < (*(*iterator).li).nb_elt {
        return (*(*iterator).actual).element;
    }
    (*iterator).actual = 0 as *mut __node_t;
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn osip_list_iterator_remove(
    mut iterator: *mut osip_list_iterator_t,
) -> *mut libc::c_void {
    if !((*iterator).actual).is_null() && (*iterator).pos < (*(*iterator).li).nb_elt {
        (*(*iterator).li).nb_elt -= 1;
        (*(*iterator).li).nb_elt;
        *(*iterator).prev = (*(*iterator).actual).next;
        if !((*iterator).actual).is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()((*iterator).actual as *mut libc::c_void);
            } else {
                free((*iterator).actual as *mut libc::c_void);
            }
        }
        (*iterator).actual = *(*iterator).prev;
    }
    if !((*iterator).actual).is_null() && (*iterator).pos < (*(*iterator).li).nb_elt {
        return (*(*iterator).actual).element;
    }
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn osip_list_remove(
    mut li: *mut osip_list_t,
    mut pos: libc::c_int,
) -> libc::c_int {
    let mut ntmp: *mut __node_t = 0 as *mut __node_t;
    let mut i: libc::c_int = 0 as libc::c_int;
    if li.is_null() {
        return -(2 as libc::c_int);
    }
    if pos < 0 as libc::c_int || pos >= (*li).nb_elt {
        return -(1 as libc::c_int);
    }
    ntmp = (*li).node;
    if pos == 0 as libc::c_int {
        (*li).node = (*ntmp).next;
        (*li).nb_elt -= 1;
        (*li).nb_elt;
        if !ntmp.is_null() {
            if osip_free_func.is_some() {
                osip_free_func.unwrap()(ntmp as *mut libc::c_void);
            } else {
                free(ntmp as *mut libc::c_void);
            }
        }
        return (*li).nb_elt;
    }
    while pos > i + 1 as libc::c_int {
        i += 1;
        i;
        ntmp = (*ntmp).next;
    }
    let mut remnode: *mut __node_t = 0 as *mut __node_t;
    remnode = (*ntmp).next;
    (*ntmp).next = (*(*ntmp).next).next;
    if !remnode.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(remnode as *mut libc::c_void);
        } else {
            free(remnode as *mut libc::c_void);
        }
    }
    (*li).nb_elt -= 1;
    (*li).nb_elt;
    return (*li).nb_elt;
}
