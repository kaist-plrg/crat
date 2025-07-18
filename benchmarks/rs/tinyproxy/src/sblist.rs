use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sblist {
    pub itemsize: size_t,
    pub blockitems: size_t,
    pub count: size_t,
    pub capa: size_t,
    pub items: *mut libc::c_char,
}
pub unsafe extern "C" fn sblist_new(
    mut itemsize: size_t,
    mut blockitems: size_t,
) -> *mut sblist {
    let mut ret: *mut sblist = malloc(::std::mem::size_of::<sblist>() as libc::c_ulong)
        as *mut sblist;
    sblist_init(ret, itemsize, blockitems);
    return ret;
}
unsafe extern "C" fn sblist_clear(mut l: *mut sblist) {
    (*l).items = 0 as *mut libc::c_char;
    (*l).capa = 0 as libc::c_int as size_t;
    (*l).count = 0 as libc::c_int as size_t;
}
pub unsafe extern "C" fn sblist_init(
    mut l: *mut sblist,
    mut itemsize: size_t,
    mut blockitems: size_t,
) {
    if !l.is_null() {
        (*l)
            .blockitems = if blockitems != 0 {
            blockitems
        } else {
            (4096 as libc::c_int as libc::c_ulong).wrapping_div(itemsize)
        };
        (*l).itemsize = itemsize;
        sblist_clear(l);
    }
}
pub unsafe extern "C" fn sblist_free_items(mut l: *mut sblist) {
    if !l.is_null() {
        if !((*l).items).is_null() {
            free((*l).items as *mut libc::c_void);
        }
        sblist_clear(l);
    }
}
pub unsafe extern "C" fn sblist_free(mut l: *mut sblist) {
    if !l.is_null() {
        sblist_free_items(l);
        free(l as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn sblist_item_from_index(
    mut l: *mut sblist,
    mut idx: size_t,
) -> *mut libc::c_char {
    return ((*l).items).offset(idx.wrapping_mul((*l).itemsize) as isize);
}
pub unsafe extern "C" fn sblist_get(
    mut l: *mut sblist,
    mut item: size_t,
) -> *mut libc::c_void {
    if item < (*l).count {
        return sblist_item_from_index(l, item) as *mut libc::c_void;
    }
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn sblist_set(
    mut l: *mut sblist,
    mut item: *mut libc::c_void,
    mut pos: size_t,
) -> libc::c_int {
    if pos >= (*l).count {
        return 0 as libc::c_int;
    }
    memcpy(sblist_item_from_index(l, pos) as *mut libc::c_void, item, (*l).itemsize);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn sblist_grow_if_needed(mut l: *mut sblist) -> libc::c_int {
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*l).count == (*l).capa {
        temp = realloc(
            (*l).items as *mut libc::c_void,
            ((*l).capa).wrapping_add((*l).blockitems).wrapping_mul((*l).itemsize),
        ) as *mut libc::c_char;
        if temp.is_null() {
            return 0 as libc::c_int;
        }
        (*l)
            .capa = ((*l).capa as libc::c_ulong).wrapping_add((*l).blockitems) as size_t
            as size_t;
        (*l).items = temp;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn sblist_add(
    mut l: *mut sblist,
    mut item: *mut libc::c_void,
) -> libc::c_int {
    if sblist_grow_if_needed(l) == 0 {
        return 0 as libc::c_int;
    }
    (*l).count = ((*l).count).wrapping_add(1);
    (*l).count;
    return sblist_set(
        l,
        item,
        ((*l).count).wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
}
pub unsafe extern "C" fn sblist_delete(mut l: *mut sblist, mut item: size_t) {
    if (*l).count != 0 && item < (*l).count {
        memmove(
            sblist_item_from_index(l, item) as *mut libc::c_void,
            sblist_item_from_index(
                l,
                item.wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *const libc::c_void,
            ((*l).count)
                .wrapping_sub(item.wrapping_add(1 as libc::c_int as libc::c_ulong))
                .wrapping_mul((*l).itemsize),
        );
        (*l).count = ((*l).count).wrapping_sub(1);
        (*l).count;
    }
}
