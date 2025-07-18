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
    let mut els: [el; 10] = [el {
        id: 0,
        next: 0 as *mut el,
        prev: 0 as *mut el,
    }; 10];
    let mut e: *mut el = 0 as *mut el;
    let mut head: *mut el = 0 as *mut el;
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
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"advancing head pointer\n\0" as *const u8 as *const libc::c_char);
    head = (*head).next;
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = if (*e).next == head { 0 as *mut el } else { (*e).next };
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    e = head;
    while !e.is_null() && i < 10 as libc::c_int {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        i += 1;
        i;
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    e = head;
    while !e.is_null() && i < 10 as libc::c_int {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        i += 1;
        i;
        e = (*e).prev;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"deleting b\n\0" as *const u8 as *const libc::c_char);
    if head == &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el
        && (*head).next == head
    {
        head = 0 as *mut el;
    } else {
        (*els[1 as libc::c_int as usize].next)
            .prev = els[1 as libc::c_int as usize].prev;
        (*els[1 as libc::c_int as usize].prev)
            .next = els[1 as libc::c_int as usize].next;
        if &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el == head {
            head = els[1 as libc::c_int as usize].next;
        }
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = if (*e).next == head { 0 as *mut el } else { (*e).next };
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"deleting head (a)\n\0" as *const u8 as *const libc::c_char);
    if head == &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el
        && (*head).next == head
    {
        head = 0 as *mut el;
    } else {
        (*els[0 as libc::c_int as usize].next)
            .prev = els[0 as libc::c_int as usize].prev;
        (*els[0 as libc::c_int as usize].prev)
            .next = els[0 as libc::c_int as usize].next;
        if &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el == head {
            head = els[0 as libc::c_int as usize].next;
        }
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = if (*e).next == head { 0 as *mut el } else { (*e).next };
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"deleting new head (c)\n\0" as *const u8 as *const libc::c_char);
    if head == &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el
        && (*head).next == head
    {
        head = 0 as *mut el;
    } else {
        (*els[2 as libc::c_int as usize].next)
            .prev = els[2 as libc::c_int as usize].prev;
        (*els[2 as libc::c_int as usize].prev)
            .next = els[2 as libc::c_int as usize].next;
        if &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el == head {
            head = els[2 as libc::c_int as usize].next;
        }
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = if (*e).next == head { 0 as *mut el } else { (*e).next };
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"DL macros\n\0" as *const u8 as *const libc::c_char);
    els[0 as libc::c_int as usize].next = head;
    if !head.is_null() {
        els[0 as libc::c_int as usize].prev = (*head).prev;
        (*head)
            .prev = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
    } else {
        els[0 as libc::c_int as usize]
            .prev = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
    }
    head = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
    els[1 as libc::c_int as usize].next = head;
    if !head.is_null() {
        els[1 as libc::c_int as usize].prev = (*head).prev;
        (*head)
            .prev = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
    } else {
        els[1 as libc::c_int as usize]
            .prev = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
    }
    head = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
    els[2 as libc::c_int as usize].next = head;
    if !head.is_null() {
        els[2 as libc::c_int as usize].prev = (*head).prev;
        (*head)
            .prev = &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el;
    } else {
        els[2 as libc::c_int as usize]
            .prev = &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el;
    }
    head = &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el;
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"deleting c\n\0" as *const u8 as *const libc::c_char);
    if !head.is_null() {} else {
        __assert_fail(
            b"(head) != NULL\0" as *const u8 as *const libc::c_char,
            b"test27.c\0" as *const u8 as *const libc::c_char,
            78 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_1883: {
        if !head.is_null() {} else {
            __assert_fail(
                b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                b"test27.c\0" as *const u8 as *const libc::c_char,
                78 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if !(els[2 as libc::c_int as usize].prev).is_null() {} else {
        __assert_fail(
            b"(&els[2])->prev != NULL\0" as *const u8 as *const libc::c_char,
            b"test27.c\0" as *const u8 as *const libc::c_char,
            78 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_1829: {
        if !(els[2 as libc::c_int as usize].prev).is_null() {} else {
            __assert_fail(
                b"(&els[2])->prev != NULL\0" as *const u8 as *const libc::c_char,
                b"test27.c\0" as *const u8 as *const libc::c_char,
                78 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if els[2 as libc::c_int as usize].prev
        == &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el
    {
        head = 0 as *mut el;
    } else if &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el == head
    {
        (*els[2 as libc::c_int as usize].next)
            .prev = els[2 as libc::c_int as usize].prev;
        head = els[2 as libc::c_int as usize].next;
    } else {
        (*els[2 as libc::c_int as usize].prev)
            .next = els[2 as libc::c_int as usize].next;
        if !(els[2 as libc::c_int as usize].next).is_null() {
            (*els[2 as libc::c_int as usize].next)
                .prev = els[2 as libc::c_int as usize].prev;
        } else {
            (*head).prev = els[2 as libc::c_int as usize].prev;
        }
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"deleting a\n\0" as *const u8 as *const libc::c_char);
    if !head.is_null() {} else {
        __assert_fail(
            b"(head) != NULL\0" as *const u8 as *const libc::c_char,
            b"test27.c\0" as *const u8 as *const libc::c_char,
            85 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_1611: {
        if !head.is_null() {} else {
            __assert_fail(
                b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                b"test27.c\0" as *const u8 as *const libc::c_char,
                85 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if !(els[0 as libc::c_int as usize].prev).is_null() {} else {
        __assert_fail(
            b"(&els[0])->prev != NULL\0" as *const u8 as *const libc::c_char,
            b"test27.c\0" as *const u8 as *const libc::c_char,
            85 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_1557: {
        if !(els[0 as libc::c_int as usize].prev).is_null() {} else {
            __assert_fail(
                b"(&els[0])->prev != NULL\0" as *const u8 as *const libc::c_char,
                b"test27.c\0" as *const u8 as *const libc::c_char,
                85 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if els[0 as libc::c_int as usize].prev
        == &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el
    {
        head = 0 as *mut el;
    } else if &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el == head
    {
        (*els[0 as libc::c_int as usize].next)
            .prev = els[0 as libc::c_int as usize].prev;
        head = els[0 as libc::c_int as usize].next;
    } else {
        (*els[0 as libc::c_int as usize].prev)
            .next = els[0 as libc::c_int as usize].next;
        if !(els[0 as libc::c_int as usize].next).is_null() {
            (*els[0 as libc::c_int as usize].next)
                .prev = els[0 as libc::c_int as usize].prev;
        } else {
            (*head).prev = els[0 as libc::c_int as usize].prev;
        }
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"deleting b\n\0" as *const u8 as *const libc::c_char);
    if !head.is_null() {} else {
        __assert_fail(
            b"(head) != NULL\0" as *const u8 as *const libc::c_char,
            b"test27.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_1338: {
        if !head.is_null() {} else {
            __assert_fail(
                b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                b"test27.c\0" as *const u8 as *const libc::c_char,
                92 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if !(els[1 as libc::c_int as usize].prev).is_null() {} else {
        __assert_fail(
            b"(&els[1])->prev != NULL\0" as *const u8 as *const libc::c_char,
            b"test27.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_1280: {
        if !(els[1 as libc::c_int as usize].prev).is_null() {} else {
            __assert_fail(
                b"(&els[1])->prev != NULL\0" as *const u8 as *const libc::c_char,
                b"test27.c\0" as *const u8 as *const libc::c_char,
                92 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if els[1 as libc::c_int as usize].prev
        == &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el
    {
        head = 0 as *mut el;
    } else if &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el == head
    {
        (*els[1 as libc::c_int as usize].next)
            .prev = els[1 as libc::c_int as usize].prev;
        head = els[1 as libc::c_int as usize].next;
    } else {
        (*els[1 as libc::c_int as usize].prev)
            .next = els[1 as libc::c_int as usize].next;
        if !(els[1 as libc::c_int as usize].next).is_null() {
            (*els[1 as libc::c_int as usize].next)
                .prev = els[1 as libc::c_int as usize].prev;
        } else {
            (*head).prev = els[1 as libc::c_int as usize].prev;
        }
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"LL macros\n\0" as *const u8 as *const libc::c_char);
    els[0 as libc::c_int as usize].next = head;
    head = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
    els[1 as libc::c_int as usize].next = head;
    head = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
    els[2 as libc::c_int as usize].next = head;
    head = &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el;
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"deleting c\n\0" as *const u8 as *const libc::c_char);
    let mut _tmp: *mut el = 0 as *mut el;
    if head == &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el {
        head = (*head).next;
    } else {
        _tmp = head;
        while !((*_tmp).next).is_null()
            && (*_tmp).next
                != &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el
        {
            _tmp = (*_tmp).next;
        }
        if !((*_tmp).next).is_null() {
            (*_tmp).next = els[2 as libc::c_int as usize].next;
        }
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"deleting a\n\0" as *const u8 as *const libc::c_char);
    let mut _tmp_0: *mut el = 0 as *mut el;
    if head == &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el {
        head = (*head).next;
    } else {
        _tmp_0 = head;
        while !((*_tmp_0).next).is_null()
            && (*_tmp_0).next
                != &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el
        {
            _tmp_0 = (*_tmp_0).next;
        }
        if !((*_tmp_0).next).is_null() {
            (*_tmp_0).next = els[0 as libc::c_int as usize].next;
        }
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"deleting b\n\0" as *const u8 as *const libc::c_char);
    let mut _tmp_1: *mut el = 0 as *mut el;
    if head == &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el {
        head = (*head).next;
    } else {
        _tmp_1 = head;
        while !((*_tmp_1).next).is_null()
            && (*_tmp_1).next
                != &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el
        {
            _tmp_1 = (*_tmp_1).next;
        }
        if !((*_tmp_1).next).is_null() {
            (*_tmp_1).next = els[1 as libc::c_int as usize].next;
        }
    }
    e = head;
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
