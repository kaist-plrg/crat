use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
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
    let mut head: *mut el = 0 as *mut el;
    let mut els: [el; 10] = [el {
        id: 0,
        next: 0 as *mut el,
        prev: 0 as *mut el,
    }; 10];
    let mut e: *mut el = 0 as *mut el;
    let mut tmp: *mut el = 0 as *mut el;
    let mut tmp2: *mut el = 0 as *mut el;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        els[i as usize].id = 'a' as i32 + i;
        i += 1;
        i;
    }
    printf(b"CDL macros\n\0" as *const u8 as *const libc::c_char);
    if !head.is_null() {
        els[0 as libc::c_int as usize].prev = (*head).prev;
        els[0 as libc::c_int as usize].next = head;
        (*head)
            .prev = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
        (*els[0 as libc::c_int as usize].prev)
            .next = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
    } else {
        els[0 as libc::c_int as usize]
            .prev = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
        els[0 as libc::c_int as usize]
            .next = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
    }
    head = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
    if !head.is_null() {
        els[1 as libc::c_int as usize].prev = (*head).prev;
        els[1 as libc::c_int as usize].next = head;
        (*head)
            .prev = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
        (*els[1 as libc::c_int as usize].prev)
            .next = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
    } else {
        els[1 as libc::c_int as usize]
            .prev = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
        els[1 as libc::c_int as usize]
            .next = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
    }
    head = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
    if !head.is_null() {
        els[2 as libc::c_int as usize].prev = (*head).prev;
        els[2 as libc::c_int as usize].next = head;
        (*head)
            .prev = &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el;
        (*els[2 as libc::c_int as usize].prev)
            .next = &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el;
    } else {
        els[2 as libc::c_int as usize]
            .prev = &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el;
        els[2 as libc::c_int as usize]
            .next = &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el;
    }
    head = &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el;
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = if (*e).next == head { 0 as *mut el } else { (*e).next };
    }
    e = head;
    tmp = (if !head.is_null() { (*head).prev } else { 0 as *mut el });
    while !e.is_null()
        && {
            tmp2 = (*e).next;
            1 as libc::c_int != 0
        }
    {
        printf(b"deleting %c \0" as *const u8 as *const libc::c_char, (*e).id);
        if head == e && (*head).next == head {
            head = 0 as *mut el;
        } else {
            (*(*e).next).prev = (*e).prev;
            (*(*e).prev).next = (*e).next;
            if e == head {
                head = (*e).next;
            }
        }
        e = if e == tmp { 0 as *mut el } else { tmp2 };
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if !head.is_null() {
        printf(b"non-null head\n\0" as *const u8 as *const libc::c_char);
    }
    printf(b"DL macros\n\0" as *const u8 as *const libc::c_char);
    if !head.is_null() {
        els[0 as libc::c_int as usize].prev = (*head).prev;
        (*(*head).prev)
            .next = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
        (*head)
            .prev = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
        els[0 as libc::c_int as usize].next = 0 as *mut el;
    } else {
        head = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
        (*head).prev = head;
        (*head).next = 0 as *mut el;
    }
    if !head.is_null() {
        els[1 as libc::c_int as usize].prev = (*head).prev;
        (*(*head).prev)
            .next = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
        (*head)
            .prev = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
        els[1 as libc::c_int as usize].next = 0 as *mut el;
    } else {
        head = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
        (*head).prev = head;
        (*head).next = 0 as *mut el;
    }
    if !head.is_null() {
        els[2 as libc::c_int as usize].prev = (*head).prev;
        (*(*head).prev)
            .next = &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el;
        (*head)
            .prev = &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el;
        els[2 as libc::c_int as usize].next = 0 as *mut el;
    } else {
        head = &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el;
        (*head).prev = head;
        (*head).next = 0 as *mut el;
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    e = head;
    while !e.is_null()
        && {
            tmp = (*e).next;
            1 as libc::c_int != 0
        }
    {
        printf(b"deleting %c \0" as *const u8 as *const libc::c_char, (*e).id);
        if !head.is_null() {} else {
            __assert_fail(
                b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                b"test41.c\0" as *const u8 as *const libc::c_char,
                47 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_1144: {
            if !head.is_null() {} else {
                __assert_fail(
                    b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test41.c\0" as *const u8 as *const libc::c_char,
                    47 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        if !((*e).prev).is_null() {} else {
            __assert_fail(
                b"(e)->prev != NULL\0" as *const u8 as *const libc::c_char,
                b"test41.c\0" as *const u8 as *const libc::c_char,
                47 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_1092: {
            if !((*e).prev).is_null() {} else {
                __assert_fail(
                    b"(e)->prev != NULL\0" as *const u8 as *const libc::c_char,
                    b"test41.c\0" as *const u8 as *const libc::c_char,
                    47 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        if (*e).prev == e {
            head = 0 as *mut el;
        } else if e == head {
            (*(*e).next).prev = (*e).prev;
            head = (*e).next;
        } else {
            (*(*e).prev).next = (*e).next;
            if !((*e).next).is_null() {
                (*(*e).next).prev = (*e).prev;
            } else {
                (*head).prev = (*e).prev;
            }
        }
        e = tmp;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if !head.is_null() {
        printf(b"non-null head\n\0" as *const u8 as *const libc::c_char);
    }
    printf(b"LL macros\n\0" as *const u8 as *const libc::c_char);
    let mut _tmp: *mut el = 0 as *mut el;
    els[0 as libc::c_int as usize].next = 0 as *mut el;
    if !head.is_null() {
        _tmp = head;
        while !((*_tmp).next).is_null() {
            _tmp = (*_tmp).next;
        }
        (*_tmp)
            .next = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
    } else {
        head = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
    }
    let mut _tmp_0: *mut el = 0 as *mut el;
    els[1 as libc::c_int as usize].next = 0 as *mut el;
    if !head.is_null() {
        _tmp_0 = head;
        while !((*_tmp_0).next).is_null() {
            _tmp_0 = (*_tmp_0).next;
        }
        (*_tmp_0)
            .next = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
    } else {
        head = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
    }
    let mut _tmp_1: *mut el = 0 as *mut el;
    els[2 as libc::c_int as usize].next = 0 as *mut el;
    if !head.is_null() {
        _tmp_1 = head;
        while !((*_tmp_1).next).is_null() {
            _tmp_1 = (*_tmp_1).next;
        }
        (*_tmp_1)
            .next = &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el;
    } else {
        head = &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el;
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    e = head;
    while !e.is_null()
        && {
            tmp = (*e).next;
            1 as libc::c_int != 0
        }
    {
        printf(b"deleting %c \0" as *const u8 as *const libc::c_char, (*e).id);
        let mut _tmp_2: *mut el = 0 as *mut el;
        if head == e {
            head = (*head).next;
        } else {
            _tmp_2 = head;
            while !((*_tmp_2).next).is_null() && (*_tmp_2).next != e {
                _tmp_2 = (*_tmp_2).next;
            }
            if !((*_tmp_2).next).is_null() {
                (*_tmp_2).next = (*e).next;
            }
        }
        e = tmp;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if !head.is_null() {
        printf(b"non-null head\n\0" as *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
