use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct slist {
    pub next: *mut slist,
    pub userdata: *const libc::c_void,
}
pub type SList = slist;
pub type SListCallback = unsafe extern "C" fn(
    *mut SList,
    *mut libc::c_void,
) -> *mut libc::c_void;
pub type SListCompare = unsafe extern "C" fn(
    *const SList,
    *const SList,
    *mut libc::c_void,
) -> libc::c_int;
pub unsafe extern "C" fn lt__slist_delete(
    mut head: *mut SList,
    mut delete_fct: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
) -> *mut SList {
    if delete_fct.is_some() {} else {
        __assert_fail(
            b"delete_fct\0" as *const u8 as *const libc::c_char,
            b"libltdl/slist.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 51],
                &[libc::c_char; 51],
            >(b"SList *lt__slist_delete(SList *, void (*)(void *))\0"))
                .as_ptr(),
        );
    }
    'c_1791: {
        if delete_fct.is_some() {} else {
            __assert_fail(
                b"delete_fct\0" as *const u8 as *const libc::c_char,
                b"libltdl/slist.c\0" as *const u8 as *const libc::c_char,
                56 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 51],
                    &[libc::c_char; 51],
                >(b"SList *lt__slist_delete(SList *, void (*)(void *))\0"))
                    .as_ptr(),
            );
        }
    };
    while !head.is_null() {
        let mut next: *mut SList = (*head).next;
        (Some(delete_fct.unwrap())).unwrap()(head as *mut libc::c_void);
        head = next;
    }
    return 0 as *mut SList;
}
pub unsafe extern "C" fn lt__slist_remove(
    mut phead: *mut *mut SList,
    mut find: Option::<SListCallback>,
    mut matchdata: *mut libc::c_void,
) -> *mut SList {
    let mut stale: *mut SList = 0 as *mut SList;
    let mut result: *mut libc::c_void = 0 as *mut libc::c_void;
    if find.is_some() {} else {
        __assert_fail(
            b"find\0" as *const u8 as *const libc::c_char,
            b"libltdl/slist.c\0" as *const u8 as *const libc::c_char,
            83 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 59],
                &[libc::c_char; 59],
            >(b"SList *lt__slist_remove(SList **, SListCallback *, void *)\0"))
                .as_ptr(),
        );
    }
    'c_1953: {
        if find.is_some() {} else {
            __assert_fail(
                b"find\0" as *const u8 as *const libc::c_char,
                b"libltdl/slist.c\0" as *const u8 as *const libc::c_char,
                83 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 59],
                    &[libc::c_char; 59],
                >(b"SList *lt__slist_remove(SList **, SListCallback *, void *)\0"))
                    .as_ptr(),
            );
        }
    };
    if phead.is_null() || (*phead).is_null() {
        return 0 as *mut SList;
    }
    result = (Some(find.unwrap())).unwrap()(*phead, matchdata);
    if !result.is_null() {
        stale = *phead;
        *phead = (*stale).next;
    } else {
        let mut head: *mut SList = 0 as *mut SList;
        head = *phead;
        while !((*head).next).is_null() {
            result = (Some(find.unwrap())).unwrap()((*head).next, matchdata);
            if !result.is_null() {
                stale = (*head).next;
                (*head).next = (*stale).next;
                break;
            } else {
                head = (*head).next;
            }
        }
    }
    return result as *mut SList;
}
pub unsafe extern "C" fn lt__slist_find(
    mut slist: *mut SList,
    mut find: Option::<SListCallback>,
    mut matchdata: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut result: *mut libc::c_void = 0 as *mut libc::c_void;
    if find.is_some() {} else {
        __assert_fail(
            b"find\0" as *const u8 as *const libc::c_char,
            b"libltdl/slist.c\0" as *const u8 as *const libc::c_char,
            122 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"void *lt__slist_find(SList *, SListCallback *, void *)\0"))
                .as_ptr(),
        );
    }
    'c_2312: {
        if find.is_some() {} else {
            __assert_fail(
                b"find\0" as *const u8 as *const libc::c_char,
                b"libltdl/slist.c\0" as *const u8 as *const libc::c_char,
                122 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"void *lt__slist_find(SList *, SListCallback *, void *)\0"))
                    .as_ptr(),
            );
        }
    };
    while !slist.is_null() {
        result = (Some(find.unwrap())).unwrap()(slist, matchdata);
        if !result.is_null() {
            break;
        }
        slist = (*slist).next;
    }
    return result;
}
pub unsafe extern "C" fn lt__slist_concat(
    mut head: *mut SList,
    mut tail: *mut SList,
) -> *mut SList {
    let mut last: *mut SList = 0 as *mut SList;
    if head.is_null() {
        return tail;
    }
    last = head;
    while !((*last).next).is_null() {
        last = (*last).next;
    }
    (*last).next = tail;
    return head;
}
pub unsafe extern "C" fn lt__slist_cons(
    mut item: *mut SList,
    mut slist: *mut SList,
) -> *mut SList {
    if item.is_null() {
        return slist;
    }
    if ((*item).next).is_null() {} else {
        __assert_fail(
            b"!item->next\0" as *const u8 as *const libc::c_char,
            b"libltdl/slist.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 40],
                &[libc::c_char; 40],
            >(b"SList *lt__slist_cons(SList *, SList *)\0"))
                .as_ptr(),
        );
    }
    'c_1705: {
        if ((*item).next).is_null() {} else {
            __assert_fail(
                b"!item->next\0" as *const u8 as *const libc::c_char,
                b"libltdl/slist.c\0" as *const u8 as *const libc::c_char,
                175 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_char; 40],
                >(b"SList *lt__slist_cons(SList *, SList *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*item).next = slist;
    return item;
}
pub unsafe extern "C" fn lt__slist_tail(mut slist: *mut SList) -> *mut SList {
    return if !slist.is_null() { (*slist).next } else { 0 as *mut slist };
}
pub unsafe extern "C" fn lt__slist_nth(
    mut slist: *mut SList,
    mut n: size_t,
) -> *mut SList {
    while n > 1 as libc::c_int as libc::c_ulong && !slist.is_null() {
        slist = (*slist).next;
        n = n.wrapping_sub(1);
        n;
    }
    return slist;
}
pub unsafe extern "C" fn lt__slist_length(mut slist: *mut SList) -> size_t {
    let mut n: size_t = 0;
    n = 0 as libc::c_int as size_t;
    while !slist.is_null() {
        slist = (*slist).next;
        n = n.wrapping_add(1);
        n;
    }
    return n;
}
pub unsafe extern "C" fn lt__slist_reverse(mut slist: *mut SList) -> *mut SList {
    let mut result: *mut SList = 0 as *mut SList;
    let mut next: *mut SList = 0 as *mut SList;
    while !slist.is_null() {
        next = (*slist).next;
        (*slist).next = result;
        result = slist;
        slist = next;
    }
    return result;
}
pub unsafe extern "C" fn lt__slist_foreach(
    mut slist: *mut SList,
    mut foreach: Option::<SListCallback>,
    mut userdata: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut result: *mut libc::c_void = 0 as *mut libc::c_void;
    if foreach.is_some() {} else {
        __assert_fail(
            b"foreach\0" as *const u8 as *const libc::c_char,
            b"libltdl/slist.c\0" as *const u8 as *const libc::c_char,
            246 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 58],
                &[libc::c_char; 58],
            >(b"void *lt__slist_foreach(SList *, SListCallback *, void *)\0"))
                .as_ptr(),
        );
    }
    'c_2416: {
        if foreach.is_some() {} else {
            __assert_fail(
                b"foreach\0" as *const u8 as *const libc::c_char,
                b"libltdl/slist.c\0" as *const u8 as *const libc::c_char,
                246 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 58],
                    &[libc::c_char; 58],
                >(b"void *lt__slist_foreach(SList *, SListCallback *, void *)\0"))
                    .as_ptr(),
            );
        }
    };
    while !slist.is_null() {
        let mut next: *mut SList = (*slist).next;
        result = (Some(foreach.unwrap())).unwrap()(slist, userdata);
        if !result.is_null() {
            break;
        }
        slist = next;
    }
    return result;
}
unsafe extern "C" fn slist_sort_merge(
    mut left: *mut SList,
    mut right: *mut SList,
    mut compare: Option::<SListCompare>,
    mut userdata: *mut libc::c_void,
) -> *mut SList {
    let mut merged: SList = SList {
        next: 0 as *mut slist,
        userdata: 0 as *const libc::c_void,
    };
    let mut insert: *mut SList = 0 as *mut SList;
    insert = &mut merged;
    while !left.is_null() && !right.is_null() {
        if (Some(compare.unwrap())).unwrap()(left, right, userdata) <= 0 as libc::c_int {
            (*insert).next = left;
            insert = (*insert).next;
            left = (*left).next;
        } else {
            (*insert).next = right;
            insert = (*insert).next;
            right = (*right).next;
        }
    }
    (*insert).next = if !left.is_null() { left } else { right };
    return merged.next;
}
pub unsafe extern "C" fn lt__slist_sort(
    mut slist: *mut SList,
    mut compare: Option::<SListCompare>,
    mut userdata: *mut libc::c_void,
) -> *mut SList {
    let mut left: *mut SList = 0 as *mut SList;
    let mut right: *mut SList = 0 as *mut SList;
    if slist.is_null() {
        return slist;
    }
    left = slist;
    right = (*slist).next;
    if right.is_null() {
        return left;
    }
    while !right.is_null()
        && {
            right = (*right).next;
            !right.is_null()
        }
    {
        if right.is_null()
            || {
                right = (*right).next;
                right.is_null()
            }
        {
            break;
        }
        slist = (*slist).next;
    }
    right = (*slist).next;
    (*slist).next = 0 as *mut slist;
    return slist_sort_merge(
        lt__slist_sort(left, compare, userdata),
        lt__slist_sort(right, compare, userdata),
        compare,
        userdata,
    );
}
pub unsafe extern "C" fn lt__slist_box(mut userdata: *const libc::c_void) -> *mut SList {
    let mut item: *mut SList = malloc(::std::mem::size_of::<SList>() as libc::c_ulong)
        as *mut SList;
    if !item.is_null() {
        (*item).next = 0 as *mut slist;
        (*item).userdata = userdata;
    }
    return item;
}
pub unsafe extern "C" fn lt__slist_unbox(mut item: *mut SList) -> *mut libc::c_void {
    let mut userdata: *mut libc::c_void = 0 as *mut libc::c_void;
    if !item.is_null() {
        userdata = (*item).userdata as *mut libc::c_void;
        free(item as *mut libc::c_void);
    }
    return userdata;
}
