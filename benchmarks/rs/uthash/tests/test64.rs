use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct el {
    pub id: libc::c_int,
    pub next: *mut el,
    pub prev: *mut el,
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut els: [el; 10] = [el {
        id: 0,
        next: 0 as *mut el,
        prev: 0 as *mut el,
    }; 10];
    let mut e: *mut el = 0 as *mut el;
    let mut headA: *mut el = 0 as *mut el;
    let mut headB: *mut el = 0 as *mut el;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        els[i as usize].id = 'a' as i32 + i;
        i += 1;
        i;
    }
    printf(b"DL macros\n\0" as *const u8 as *const libc::c_char);
    if !headA.is_null() {
        els[0 as libc::c_int as usize].prev = (*headA).prev;
        (*(*headA).prev)
            .next = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
        (*headA)
            .prev = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
        els[0 as libc::c_int as usize].next = 0 as *mut el;
    } else {
        headA = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
        (*headA).prev = headA;
        (*headA).next = 0 as *mut el;
    }
    if !headA.is_null() {
        els[1 as libc::c_int as usize].prev = (*headA).prev;
        (*(*headA).prev)
            .next = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
        (*headA)
            .prev = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
        els[1 as libc::c_int as usize].next = 0 as *mut el;
    } else {
        headA = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
        (*headA).prev = headA;
        (*headA).next = 0 as *mut el;
    }
    if !headA.is_null() {
        els[2 as libc::c_int as usize].prev = (*headA).prev;
        (*(*headA).prev)
            .next = &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el;
        (*headA)
            .prev = &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el;
        els[2 as libc::c_int as usize].next = 0 as *mut el;
    } else {
        headA = &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el;
        (*headA).prev = headA;
        (*headA).next = 0 as *mut el;
    }
    e = headA;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if !headB.is_null() {
        els[3 as libc::c_int as usize].prev = (*headB).prev;
        (*(*headB).prev)
            .next = &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el;
        (*headB)
            .prev = &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el;
        els[3 as libc::c_int as usize].next = 0 as *mut el;
    } else {
        headB = &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el;
        (*headB).prev = headB;
        (*headB).next = 0 as *mut el;
    }
    if !headB.is_null() {
        els[4 as libc::c_int as usize].prev = (*headB).prev;
        (*(*headB).prev)
            .next = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el;
        (*headB)
            .prev = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el;
        els[4 as libc::c_int as usize].next = 0 as *mut el;
    } else {
        headB = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el;
        (*headB).prev = headB;
        (*headB).next = 0 as *mut el;
    }
    if !headB.is_null() {
        els[5 as libc::c_int as usize].prev = (*headB).prev;
        (*(*headB).prev)
            .next = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el;
        (*headB)
            .prev = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el;
        els[5 as libc::c_int as usize].next = 0 as *mut el;
    } else {
        headB = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el;
        (*headB).prev = headB;
        (*headB).next = 0 as *mut el;
    }
    e = headB;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    let mut _tmp: *mut el = 0 as *mut el;
    if !headB.is_null() {
        if !headA.is_null() {
            _tmp = (*headB).prev;
            (*headB).prev = (*headA).prev;
            (*(*headA).prev).next = headB;
            (*headA).prev = _tmp;
        } else {
            headA = headB;
        }
    }
    e = headA;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    headA = 0 as *mut el;
    let mut _tmp_0: *mut el = 0 as *mut el;
    if !headB.is_null() {
        if !headA.is_null() {
            _tmp_0 = (*headB).prev;
            (*headB).prev = (*headA).prev;
            (*(*headA).prev).next = headB;
            (*headA).prev = _tmp_0;
        } else {
            headA = headB;
        }
    }
    e = headA;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    headB = 0 as *mut el;
    let mut _tmp_1: *mut el = 0 as *mut el;
    if !headB.is_null() {
        if !headA.is_null() {
            _tmp_1 = (*headB).prev;
            (*headB).prev = (*headA).prev;
            (*(*headA).prev).next = headB;
            (*headA).prev = _tmp_1;
        } else {
            headA = headB;
        }
    }
    e = headA;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    headA = 0 as *mut el;
    headB = 0 as *mut el;
    if !headA.is_null() {
        els[0 as libc::c_int as usize].prev = (*headA).prev;
        (*(*headA).prev)
            .next = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
        (*headA)
            .prev = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
        els[0 as libc::c_int as usize].next = 0 as *mut el;
    } else {
        headA = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
        (*headA).prev = headA;
        (*headA).next = 0 as *mut el;
    }
    if !headB.is_null() {
        els[1 as libc::c_int as usize].prev = (*headB).prev;
        (*(*headB).prev)
            .next = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
        (*headB)
            .prev = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
        els[1 as libc::c_int as usize].next = 0 as *mut el;
    } else {
        headB = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
        (*headB).prev = headB;
        (*headB).next = 0 as *mut el;
    }
    let mut _tmp_2: *mut el = 0 as *mut el;
    if !headB.is_null() {
        if !headA.is_null() {
            _tmp_2 = (*headB).prev;
            (*headB).prev = (*headA).prev;
            (*(*headA).prev).next = headB;
            (*headA).prev = _tmp_2;
        } else {
            headA = headB;
        }
    }
    e = headA;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
