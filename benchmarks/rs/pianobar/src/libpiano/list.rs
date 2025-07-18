use ::libc;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoListHead {
    pub next: *mut PianoListHead,
}
pub type PianoListHead_t = PianoListHead;
pub unsafe extern "C" fn PianoListAppend(
    l: *mut PianoListHead_t,
    e: *mut PianoListHead_t,
) -> *mut libc::c_void {
    if l.is_null() {
        return e as *mut libc::c_void
    } else {
        let mut curr: *mut PianoListHead_t = l;
        while !((*curr).next).is_null() {
            curr = (*curr).next;
        }
        (*curr).next = e;
        return l as *mut libc::c_void;
    };
}
pub unsafe extern "C" fn PianoListPrepend(
    l: *mut PianoListHead_t,
    e: *mut PianoListHead_t,
) -> *mut libc::c_void {
    (*e).next = l;
    return e as *mut libc::c_void;
}
pub unsafe extern "C" fn PianoListDelete(
    l: *mut PianoListHead_t,
    e: *mut PianoListHead_t,
) -> *mut libc::c_void {
    let mut first: *mut PianoListHead_t = l;
    let mut curr: *mut PianoListHead_t = l;
    let mut prev: *mut PianoListHead_t = 0 as *mut PianoListHead_t;
    while !curr.is_null() {
        if curr == e {
            if !prev.is_null() {
                (*prev).next = (*curr).next;
            } else {
                first = (*curr).next;
            }
            break;
        } else {
            prev = curr;
            curr = (*curr).next as *mut libc::c_void as *mut PianoListHead_t;
        }
    }
    return first as *mut libc::c_void;
}
pub unsafe extern "C" fn PianoListGet(
    l: *mut PianoListHead_t,
    n: size_t,
) -> *mut libc::c_void {
    let mut curr: *mut PianoListHead_t = l;
    let mut i: size_t = n;
    while !curr.is_null() {
        if i == 0 as libc::c_int as libc::c_ulong {
            return curr as *mut libc::c_void;
        }
        i = i.wrapping_sub(1);
        i;
        curr = (*curr).next as *mut libc::c_void as *mut PianoListHead_t;
    }
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn PianoListCount(l: *const PianoListHead_t) -> size_t {
    let mut count: size_t = 0 as libc::c_int as size_t;
    let mut curr: *const PianoListHead_t = l;
    while !curr.is_null() {
        count = count.wrapping_add(1);
        count;
        curr = (*curr).next as *mut libc::c_void as *const PianoListHead_t;
    }
    return count;
}
