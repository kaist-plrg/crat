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
    let mut count: libc::c_int = 0;
    let mut els: [el; 10] = [el {
        id: 0,
        next: 0 as *mut el,
        prev: 0 as *mut el,
    }; 10];
    let mut e: *mut el = 0 as *mut el;
    let mut head: *mut el = 0 as *mut el;
    let mut zeroptr: *mut el = 0 as *mut el;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        els[i as usize].id = 'a' as i32 + i;
        i += 1;
        i;
    }
    printf(b"CDL appends\n\0" as *const u8 as *const libc::c_char);
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
        head = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
    }
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
        head = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
    }
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
        head = &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el;
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = if (*e).next == head { 0 as *mut el } else { (*e).next };
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    count = 0 as libc::c_int;
    e = head;
    while !e.is_null() {
        count += 1;
        count;
        e = if (*e).next == head { 0 as *mut el } else { (*e).next };
    }
    printf(b"count = %d\n\0" as *const u8 as *const libc::c_char, count);
    printf(
        b"Test CDL_PREPEND_ELEM %c with elt NULL\n\0" as *const u8
            as *const libc::c_char,
        els[3 as libc::c_int as usize].id,
    );
    if !zeroptr.is_null() {
        if !head.is_null() {} else {
            __assert_fail(
                b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                33 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_8977: {
            if !head.is_null() {} else {
                __assert_fail(
                    b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test86.c\0" as *const u8 as *const libc::c_char,
                    33 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        if !(&mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[3]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                33 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_8927: {
            if !(&mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el)
                .is_null()
            {} else {
                __assert_fail(
                    b"(&els[3]) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test86.c\0" as *const u8 as *const libc::c_char,
                    33 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        els[3 as libc::c_int as usize].next = zeroptr;
        els[3 as libc::c_int as usize].prev = (*zeroptr).prev;
        (*zeroptr)
            .prev = &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el;
        (*els[3 as libc::c_int as usize].prev)
            .next = &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el;
        if head == zeroptr {
            head = &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el;
        }
    } else if !head.is_null() {
        els[3 as libc::c_int as usize].prev = (*head).prev;
        els[3 as libc::c_int as usize].next = head;
        (*head)
            .prev = &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el;
        (*els[3 as libc::c_int as usize].prev)
            .next = &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el;
    } else {
        els[3 as libc::c_int as usize]
            .prev = &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el;
        els[3 as libc::c_int as usize]
            .next = &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el;
        head = &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el;
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = if (*e).next == head { 0 as *mut el } else { (*e).next };
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Test CDL_PREPEND_ELEM %c before item %c\n\0" as *const u8
            as *const libc::c_char,
        els[4 as libc::c_int as usize].id,
        els[1 as libc::c_int as usize].id,
    );
    if !(&mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el).is_null() {
        if !head.is_null() {} else {
            __assert_fail(
                b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                40 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_8627: {
            if !head.is_null() {} else {
                __assert_fail(
                    b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test86.c\0" as *const u8 as *const libc::c_char,
                    40 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        if !(&mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[4]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                40 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_8577: {
            if !(&mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el)
                .is_null()
            {} else {
                __assert_fail(
                    b"(&els[4]) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test86.c\0" as *const u8 as *const libc::c_char,
                    40 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        els[4 as libc::c_int as usize]
            .next = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
        els[4 as libc::c_int as usize].prev = els[1 as libc::c_int as usize].prev;
        els[1 as libc::c_int as usize]
            .prev = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el;
        (*els[4 as libc::c_int as usize].prev)
            .next = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el;
        if head == &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el {
            head = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el;
        }
    } else if !head.is_null() {
        els[4 as libc::c_int as usize].prev = (*head).prev;
        els[4 as libc::c_int as usize].next = head;
        (*head)
            .prev = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el;
        (*els[4 as libc::c_int as usize].prev)
            .next = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el;
    } else {
        els[4 as libc::c_int as usize]
            .prev = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el;
        els[4 as libc::c_int as usize]
            .next = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el;
        head = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el;
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = if (*e).next == head { 0 as *mut el } else { (*e).next };
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Test CDL_APPEND_ELEM %c with elt NULL\n\0" as *const u8 as *const libc::c_char,
        els[5 as libc::c_int as usize].id,
    );
    if !zeroptr.is_null() {
        if !head.is_null() {} else {
            __assert_fail(
                b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                47 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_8275: {
            if !head.is_null() {} else {
                __assert_fail(
                    b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test86.c\0" as *const u8 as *const libc::c_char,
                    47 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        if !(&mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[5]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                47 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_8225: {
            if !(&mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el)
                .is_null()
            {} else {
                __assert_fail(
                    b"(&els[5]) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test86.c\0" as *const u8 as *const libc::c_char,
                    47 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        els[5 as libc::c_int as usize].next = (*zeroptr).next;
        els[5 as libc::c_int as usize].prev = zeroptr;
        (*zeroptr)
            .next = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el;
        (*els[5 as libc::c_int as usize].next)
            .prev = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el;
    } else {
        if !head.is_null() {
            els[5 as libc::c_int as usize].prev = (*head).prev;
            els[5 as libc::c_int as usize].next = head;
            (*head)
                .prev = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize)
                as *mut el;
            (*els[5 as libc::c_int as usize].prev)
                .next = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize)
                as *mut el;
        } else {
            els[5 as libc::c_int as usize]
                .prev = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize)
                as *mut el;
            els[5 as libc::c_int as usize]
                .next = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize)
                as *mut el;
        }
        head = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el;
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = if (*e).next == head { 0 as *mut el } else { (*e).next };
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Test CDL_APPEND_ELEM %c after item %c\n\0" as *const u8 as *const libc::c_char,
        els[6 as libc::c_int as usize].id,
        els[1 as libc::c_int as usize].id,
    );
    if !(&mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el).is_null() {
        if !head.is_null() {} else {
            __assert_fail(
                b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                54 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_7945: {
            if !head.is_null() {} else {
                __assert_fail(
                    b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test86.c\0" as *const u8 as *const libc::c_char,
                    54 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        if !(&mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[6]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                54 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_7895: {
            if !(&mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el)
                .is_null()
            {} else {
                __assert_fail(
                    b"(&els[6]) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test86.c\0" as *const u8 as *const libc::c_char,
                    54 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        els[6 as libc::c_int as usize].next = els[1 as libc::c_int as usize].next;
        els[6 as libc::c_int as usize]
            .prev = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
        els[1 as libc::c_int as usize]
            .next = &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el;
        (*els[6 as libc::c_int as usize].next)
            .prev = &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el;
    } else {
        if !head.is_null() {
            els[6 as libc::c_int as usize].prev = (*head).prev;
            els[6 as libc::c_int as usize].next = head;
            (*head)
                .prev = &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize)
                as *mut el;
            (*els[6 as libc::c_int as usize].prev)
                .next = &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize)
                as *mut el;
        } else {
            els[6 as libc::c_int as usize]
                .prev = &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize)
                as *mut el;
            els[6 as libc::c_int as usize]
                .next = &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize)
                as *mut el;
        }
        head = &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el;
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = if (*e).next == head { 0 as *mut el } else { (*e).next };
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    count = 0 as libc::c_int;
    e = head;
    while !e.is_null() {
        count += 1;
        count;
        e = if (*e).next == head { 0 as *mut el } else { (*e).next };
    }
    printf(b"count = %d\n\0" as *const u8 as *const libc::c_char, count);
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
    while !e.is_null() && i < 20 as libc::c_int {
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
    printf(b"deleting (b)\n\0" as *const u8 as *const libc::c_char);
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
    printf(b"deleting (a)\n\0" as *const u8 as *const libc::c_char);
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
    printf(b"deleting (c)\n\0" as *const u8 as *const libc::c_char);
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
    printf(b"deleting (g)\n\0" as *const u8 as *const libc::c_char);
    if head == &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el
        && (*head).next == head
    {
        head = 0 as *mut el;
    } else {
        (*els[6 as libc::c_int as usize].next)
            .prev = els[6 as libc::c_int as usize].prev;
        (*els[6 as libc::c_int as usize].prev)
            .next = els[6 as libc::c_int as usize].next;
        if &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el == head {
            head = els[6 as libc::c_int as usize].next;
        }
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = if (*e).next == head { 0 as *mut el } else { (*e).next };
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"deleting (e)\n\0" as *const u8 as *const libc::c_char);
    if head == &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el
        && (*head).next == head
    {
        head = 0 as *mut el;
    } else {
        (*els[4 as libc::c_int as usize].next)
            .prev = els[4 as libc::c_int as usize].prev;
        (*els[4 as libc::c_int as usize].prev)
            .next = els[4 as libc::c_int as usize].next;
        if &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el == head {
            head = els[4 as libc::c_int as usize].next;
        }
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = if (*e).next == head { 0 as *mut el } else { (*e).next };
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"deleting (d)\n\0" as *const u8 as *const libc::c_char);
    if head == &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el
        && (*head).next == head
    {
        head = 0 as *mut el;
    } else {
        (*els[3 as libc::c_int as usize].next)
            .prev = els[3 as libc::c_int as usize].prev;
        (*els[3 as libc::c_int as usize].prev)
            .next = els[3 as libc::c_int as usize].next;
        if &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el == head {
            head = els[3 as libc::c_int as usize].next;
        }
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = if (*e).next == head { 0 as *mut el } else { (*e).next };
    }
    printf(b"deleting (f)\n\0" as *const u8 as *const libc::c_char);
    if head == &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el
        && (*head).next == head
    {
        head = 0 as *mut el;
    } else {
        (*els[5 as libc::c_int as usize].next)
            .prev = els[5 as libc::c_int as usize].prev;
        (*els[5 as libc::c_int as usize].prev)
            .next = els[5 as libc::c_int as usize].next;
        if &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el == head {
            head = els[5 as libc::c_int as usize].next;
        }
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = if (*e).next == head { 0 as *mut el } else { (*e).next };
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"DL appends\n\0" as *const u8 as *const libc::c_char);
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
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    count = 0 as libc::c_int;
    e = head;
    while !e.is_null() {
        count += 1;
        count;
        e = (*e).next;
    }
    printf(b"count = %d\n\0" as *const u8 as *const libc::c_char, count);
    printf(
        b"Test DL_PREPEND_ELEM %c with elt NULL\n\0" as *const u8 as *const libc::c_char,
        els[3 as libc::c_int as usize].id,
    );
    if !zeroptr.is_null() {
        if !head.is_null() {} else {
            __assert_fail(
                b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                138 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_6032: {
            if !head.is_null() {} else {
                __assert_fail(
                    b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test86.c\0" as *const u8 as *const libc::c_char,
                    138 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        if !(&mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[3]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                138 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_5982: {
            if !(&mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el)
                .is_null()
            {} else {
                __assert_fail(
                    b"(&els[3]) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test86.c\0" as *const u8 as *const libc::c_char,
                    138 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        els[3 as libc::c_int as usize].next = zeroptr;
        els[3 as libc::c_int as usize].prev = (*zeroptr).prev;
        (*zeroptr)
            .prev = &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el;
        if head == zeroptr {
            head = &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el;
        } else {
            (*els[3 as libc::c_int as usize].prev)
                .next = &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize)
                as *mut el;
        }
    } else if !head.is_null() {
        els[3 as libc::c_int as usize].prev = (*head).prev;
        (*(*head).prev)
            .next = &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el;
        (*head)
            .prev = &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el;
        els[3 as libc::c_int as usize].next = 0 as *mut el;
    } else {
        head = &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el;
        (*head).prev = head;
        (*head).next = 0 as *mut el;
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Test DL_PREPEND_ELEM %c before item %c\n\0" as *const u8
            as *const libc::c_char,
        els[4 as libc::c_int as usize].id,
        els[1 as libc::c_int as usize].id,
    );
    if !(&mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el).is_null() {
        if !head.is_null() {} else {
            __assert_fail(
                b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                145 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_5711: {
            if !head.is_null() {} else {
                __assert_fail(
                    b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test86.c\0" as *const u8 as *const libc::c_char,
                    145 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        if !(&mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[4]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                145 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_5661: {
            if !(&mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el)
                .is_null()
            {} else {
                __assert_fail(
                    b"(&els[4]) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test86.c\0" as *const u8 as *const libc::c_char,
                    145 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        els[4 as libc::c_int as usize]
            .next = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
        els[4 as libc::c_int as usize].prev = els[1 as libc::c_int as usize].prev;
        els[1 as libc::c_int as usize]
            .prev = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el;
        if head == &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el {
            head = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el;
        } else {
            (*els[4 as libc::c_int as usize].prev)
                .next = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize)
                as *mut el;
        }
    } else if !head.is_null() {
        els[4 as libc::c_int as usize].prev = (*head).prev;
        (*(*head).prev)
            .next = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el;
        (*head)
            .prev = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el;
        els[4 as libc::c_int as usize].next = 0 as *mut el;
    } else {
        head = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el;
        (*head).prev = head;
        (*head).next = 0 as *mut el;
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Test DL_APPEND_ELEM %c with elt NULL\n\0" as *const u8 as *const libc::c_char,
        els[5 as libc::c_int as usize].id,
    );
    if !zeroptr.is_null() {
        if !head.is_null() {} else {
            __assert_fail(
                b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                152 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_5387: {
            if !head.is_null() {} else {
                __assert_fail(
                    b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test86.c\0" as *const u8 as *const libc::c_char,
                    152 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        if !(&mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[5]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                152 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_5337: {
            if !(&mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el)
                .is_null()
            {} else {
                __assert_fail(
                    b"(&els[5]) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test86.c\0" as *const u8 as *const libc::c_char,
                    152 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        els[5 as libc::c_int as usize].next = (*zeroptr).next;
        els[5 as libc::c_int as usize].prev = zeroptr;
        (*zeroptr)
            .next = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el;
        if !(els[5 as libc::c_int as usize].next).is_null() {
            (*els[5 as libc::c_int as usize].next)
                .prev = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize)
                as *mut el;
        } else {
            (*head)
                .prev = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize)
                as *mut el;
        }
    } else {
        els[5 as libc::c_int as usize].next = head;
        if !head.is_null() {
            els[5 as libc::c_int as usize].prev = (*head).prev;
            (*head)
                .prev = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize)
                as *mut el;
        } else {
            els[5 as libc::c_int as usize]
                .prev = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize)
                as *mut el;
        }
        head = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el;
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Test DL_APPEND_ELEM %c after item %c\n\0" as *const u8 as *const libc::c_char,
        els[6 as libc::c_int as usize].id,
        els[1 as libc::c_int as usize].id,
    );
    if !(&mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el).is_null() {
        if !head.is_null() {} else {
            __assert_fail(
                b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                159 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_5082: {
            if !head.is_null() {} else {
                __assert_fail(
                    b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test86.c\0" as *const u8 as *const libc::c_char,
                    159 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        if !(&mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[6]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                159 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_5032: {
            if !(&mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el)
                .is_null()
            {} else {
                __assert_fail(
                    b"(&els[6]) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test86.c\0" as *const u8 as *const libc::c_char,
                    159 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        els[6 as libc::c_int as usize].next = els[1 as libc::c_int as usize].next;
        els[6 as libc::c_int as usize]
            .prev = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
        els[1 as libc::c_int as usize]
            .next = &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el;
        if !(els[6 as libc::c_int as usize].next).is_null() {
            (*els[6 as libc::c_int as usize].next)
                .prev = &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize)
                as *mut el;
        } else {
            (*head)
                .prev = &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize)
                as *mut el;
        }
    } else {
        els[6 as libc::c_int as usize].next = head;
        if !head.is_null() {
            els[6 as libc::c_int as usize].prev = (*head).prev;
            (*head)
                .prev = &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize)
                as *mut el;
        } else {
            els[6 as libc::c_int as usize]
                .prev = &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize)
                as *mut el;
        }
        head = &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el;
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    count = 0 as libc::c_int;
    e = head;
    while !e.is_null() {
        count += 1;
        count;
        e = (*e).next;
    }
    printf(b"count = %d\n\0" as *const u8 as *const libc::c_char, count);
    printf(b"deleting (b)\n\0" as *const u8 as *const libc::c_char);
    if !head.is_null() {} else {
        __assert_fail(
            b"(head) != NULL\0" as *const u8 as *const libc::c_char,
            b"test86.c\0" as *const u8 as *const libc::c_char,
            168 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_4747: {
        if !head.is_null() {} else {
            __assert_fail(
                b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                168 as libc::c_int as libc::c_uint,
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
            b"test86.c\0" as *const u8 as *const libc::c_char,
            168 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_4693: {
        if !(els[1 as libc::c_int as usize].prev).is_null() {} else {
            __assert_fail(
                b"(&els[1])->prev != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                168 as libc::c_int as libc::c_uint,
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
    printf(b"deleting (a)\n\0" as *const u8 as *const libc::c_char);
    if !head.is_null() {} else {
        __assert_fail(
            b"(head) != NULL\0" as *const u8 as *const libc::c_char,
            b"test86.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_4475: {
        if !head.is_null() {} else {
            __assert_fail(
                b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                174 as libc::c_int as libc::c_uint,
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
            b"test86.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_4421: {
        if !(els[0 as libc::c_int as usize].prev).is_null() {} else {
            __assert_fail(
                b"(&els[0])->prev != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                174 as libc::c_int as libc::c_uint,
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
    printf(b"deleting (c)\n\0" as *const u8 as *const libc::c_char);
    if !head.is_null() {} else {
        __assert_fail(
            b"(head) != NULL\0" as *const u8 as *const libc::c_char,
            b"test86.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_4203: {
        if !head.is_null() {} else {
            __assert_fail(
                b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                180 as libc::c_int as libc::c_uint,
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
            b"test86.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_4149: {
        if !(els[2 as libc::c_int as usize].prev).is_null() {} else {
            __assert_fail(
                b"(&els[2])->prev != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                180 as libc::c_int as libc::c_uint,
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
    printf(b"deleting (g)\n\0" as *const u8 as *const libc::c_char);
    if !head.is_null() {} else {
        __assert_fail(
            b"(head) != NULL\0" as *const u8 as *const libc::c_char,
            b"test86.c\0" as *const u8 as *const libc::c_char,
            186 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_3931: {
        if !head.is_null() {} else {
            __assert_fail(
                b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                186 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if !(els[6 as libc::c_int as usize].prev).is_null() {} else {
        __assert_fail(
            b"(&els[6])->prev != NULL\0" as *const u8 as *const libc::c_char,
            b"test86.c\0" as *const u8 as *const libc::c_char,
            186 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_3877: {
        if !(els[6 as libc::c_int as usize].prev).is_null() {} else {
            __assert_fail(
                b"(&els[6])->prev != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                186 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if els[6 as libc::c_int as usize].prev
        == &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el
    {
        head = 0 as *mut el;
    } else if &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el == head
    {
        (*els[6 as libc::c_int as usize].next)
            .prev = els[6 as libc::c_int as usize].prev;
        head = els[6 as libc::c_int as usize].next;
    } else {
        (*els[6 as libc::c_int as usize].prev)
            .next = els[6 as libc::c_int as usize].next;
        if !(els[6 as libc::c_int as usize].next).is_null() {
            (*els[6 as libc::c_int as usize].next)
                .prev = els[6 as libc::c_int as usize].prev;
        } else {
            (*head).prev = els[6 as libc::c_int as usize].prev;
        }
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"deleting (e)\n\0" as *const u8 as *const libc::c_char);
    if !head.is_null() {} else {
        __assert_fail(
            b"(head) != NULL\0" as *const u8 as *const libc::c_char,
            b"test86.c\0" as *const u8 as *const libc::c_char,
            192 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_3659: {
        if !head.is_null() {} else {
            __assert_fail(
                b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                192 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if !(els[4 as libc::c_int as usize].prev).is_null() {} else {
        __assert_fail(
            b"(&els[4])->prev != NULL\0" as *const u8 as *const libc::c_char,
            b"test86.c\0" as *const u8 as *const libc::c_char,
            192 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_3605: {
        if !(els[4 as libc::c_int as usize].prev).is_null() {} else {
            __assert_fail(
                b"(&els[4])->prev != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                192 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if els[4 as libc::c_int as usize].prev
        == &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el
    {
        head = 0 as *mut el;
    } else if &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el == head
    {
        (*els[4 as libc::c_int as usize].next)
            .prev = els[4 as libc::c_int as usize].prev;
        head = els[4 as libc::c_int as usize].next;
    } else {
        (*els[4 as libc::c_int as usize].prev)
            .next = els[4 as libc::c_int as usize].next;
        if !(els[4 as libc::c_int as usize].next).is_null() {
            (*els[4 as libc::c_int as usize].next)
                .prev = els[4 as libc::c_int as usize].prev;
        } else {
            (*head).prev = els[4 as libc::c_int as usize].prev;
        }
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"deleting (d)\n\0" as *const u8 as *const libc::c_char);
    if !head.is_null() {} else {
        __assert_fail(
            b"(head) != NULL\0" as *const u8 as *const libc::c_char,
            b"test86.c\0" as *const u8 as *const libc::c_char,
            198 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_3387: {
        if !head.is_null() {} else {
            __assert_fail(
                b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                198 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if !(els[3 as libc::c_int as usize].prev).is_null() {} else {
        __assert_fail(
            b"(&els[3])->prev != NULL\0" as *const u8 as *const libc::c_char,
            b"test86.c\0" as *const u8 as *const libc::c_char,
            198 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_3333: {
        if !(els[3 as libc::c_int as usize].prev).is_null() {} else {
            __assert_fail(
                b"(&els[3])->prev != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                198 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if els[3 as libc::c_int as usize].prev
        == &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el
    {
        head = 0 as *mut el;
    } else if &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el == head
    {
        (*els[3 as libc::c_int as usize].next)
            .prev = els[3 as libc::c_int as usize].prev;
        head = els[3 as libc::c_int as usize].next;
    } else {
        (*els[3 as libc::c_int as usize].prev)
            .next = els[3 as libc::c_int as usize].next;
        if !(els[3 as libc::c_int as usize].next).is_null() {
            (*els[3 as libc::c_int as usize].next)
                .prev = els[3 as libc::c_int as usize].prev;
        } else {
            (*head).prev = els[3 as libc::c_int as usize].prev;
        }
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"deleting (f)\n\0" as *const u8 as *const libc::c_char);
    if !head.is_null() {} else {
        __assert_fail(
            b"(head) != NULL\0" as *const u8 as *const libc::c_char,
            b"test86.c\0" as *const u8 as *const libc::c_char,
            203 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_3120: {
        if !head.is_null() {} else {
            __assert_fail(
                b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                203 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if !(els[5 as libc::c_int as usize].prev).is_null() {} else {
        __assert_fail(
            b"(&els[5])->prev != NULL\0" as *const u8 as *const libc::c_char,
            b"test86.c\0" as *const u8 as *const libc::c_char,
            203 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_3065: {
        if !(els[5 as libc::c_int as usize].prev).is_null() {} else {
            __assert_fail(
                b"(&els[5])->prev != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                203 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if els[5 as libc::c_int as usize].prev
        == &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el
    {
        head = 0 as *mut el;
    } else if &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el == head
    {
        (*els[5 as libc::c_int as usize].next)
            .prev = els[5 as libc::c_int as usize].prev;
        head = els[5 as libc::c_int as usize].next;
    } else {
        (*els[5 as libc::c_int as usize].prev)
            .next = els[5 as libc::c_int as usize].next;
        if !(els[5 as libc::c_int as usize].next).is_null() {
            (*els[5 as libc::c_int as usize].next)
                .prev = els[5 as libc::c_int as usize].prev;
        } else {
            (*head).prev = els[5 as libc::c_int as usize].prev;
        }
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"LL appends\n\0" as *const u8 as *const libc::c_char);
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
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    count = 0 as libc::c_int;
    e = head;
    while !e.is_null() {
        count += 1;
        count;
        e = (*e).next;
    }
    printf(b"count = %d\n\0" as *const u8 as *const libc::c_char, count);
    printf(
        b"Test LL_PREPEND_ELEM %c with elt NULL\n\0" as *const u8 as *const libc::c_char,
        els[3 as libc::c_int as usize].id,
    );
    if !zeroptr.is_null() {
        let mut _tmp_2: *mut el = 0 as *mut el;
        if !head.is_null() {} else {
            __assert_fail(
                b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                224 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_2575: {
            if !head.is_null() {} else {
                __assert_fail(
                    b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test86.c\0" as *const u8 as *const libc::c_char,
                    224 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        if !(&mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[3]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                224 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_2525: {
            if !(&mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el)
                .is_null()
            {} else {
                __assert_fail(
                    b"(&els[3]) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test86.c\0" as *const u8 as *const libc::c_char,
                    224 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        els[3 as libc::c_int as usize].next = zeroptr;
        if head == zeroptr {
            head = &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el;
        } else {
            _tmp_2 = head;
            while !((*_tmp_2).next).is_null() && (*_tmp_2).next != zeroptr {
                _tmp_2 = (*_tmp_2).next;
            }
            if !((*_tmp_2).next).is_null() {
                (*_tmp_2)
                    .next = &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize)
                    as *mut el;
            }
        }
    } else {
        let mut _tmp_3: *mut el = 0 as *mut el;
        els[3 as libc::c_int as usize].next = 0 as *mut el;
        if !head.is_null() {
            _tmp_3 = head;
            while !((*_tmp_3).next).is_null() {
                _tmp_3 = (*_tmp_3).next;
            }
            (*_tmp_3)
                .next = &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize)
                as *mut el;
        } else {
            head = &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el;
        }
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Test LL_PREPEND_ELEM %c before item %c\n\0" as *const u8
            as *const libc::c_char,
        els[4 as libc::c_int as usize].id,
        els[1 as libc::c_int as usize].id,
    );
    if !(&mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el).is_null() {
        let mut _tmp_4: *mut el = 0 as *mut el;
        if !head.is_null() {} else {
            __assert_fail(
                b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                231 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_2273: {
            if !head.is_null() {} else {
                __assert_fail(
                    b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test86.c\0" as *const u8 as *const libc::c_char,
                    231 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        if !(&mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[4]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                231 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_2223: {
            if !(&mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el)
                .is_null()
            {} else {
                __assert_fail(
                    b"(&els[4]) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test86.c\0" as *const u8 as *const libc::c_char,
                    231 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        els[4 as libc::c_int as usize]
            .next = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
        if head == &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el {
            head = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el;
        } else {
            _tmp_4 = head;
            while !((*_tmp_4).next).is_null()
                && (*_tmp_4).next
                    != &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize)
                        as *mut el
            {
                _tmp_4 = (*_tmp_4).next;
            }
            if !((*_tmp_4).next).is_null() {
                (*_tmp_4)
                    .next = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize)
                    as *mut el;
            }
        }
    } else {
        let mut _tmp_5: *mut el = 0 as *mut el;
        els[4 as libc::c_int as usize].next = 0 as *mut el;
        if !head.is_null() {
            _tmp_5 = head;
            while !((*_tmp_5).next).is_null() {
                _tmp_5 = (*_tmp_5).next;
            }
            (*_tmp_5)
                .next = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize)
                as *mut el;
        } else {
            head = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el;
        }
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Test LL_APPEND_ELEM %c with elt NULL\n\0" as *const u8 as *const libc::c_char,
        els[5 as libc::c_int as usize].id,
    );
    if !zeroptr.is_null() {
        if !head.is_null() {} else {
            __assert_fail(
                b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                238 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_1972: {
            if !head.is_null() {} else {
                __assert_fail(
                    b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test86.c\0" as *const u8 as *const libc::c_char,
                    238 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        if !(&mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[5]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                238 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_1922: {
            if !(&mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el)
                .is_null()
            {} else {
                __assert_fail(
                    b"(&els[5]) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test86.c\0" as *const u8 as *const libc::c_char,
                    238 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        els[5 as libc::c_int as usize].next = (*zeroptr).next;
        (*zeroptr)
            .next = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el;
    } else {
        els[5 as libc::c_int as usize].next = head;
        head = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el;
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Test LL_APPEND_ELEM %c after item %c\n\0" as *const u8 as *const libc::c_char,
        els[6 as libc::c_int as usize].id,
        els[1 as libc::c_int as usize].id,
    );
    if !(&mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el).is_null() {
        if !head.is_null() {} else {
            __assert_fail(
                b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                245 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_1758: {
            if !head.is_null() {} else {
                __assert_fail(
                    b"(head) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test86.c\0" as *const u8 as *const libc::c_char,
                    245 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        if !(&mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[6]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test86.c\0" as *const u8 as *const libc::c_char,
                245 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_1703: {
            if !(&mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el)
                .is_null()
            {} else {
                __assert_fail(
                    b"(&els[6]) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test86.c\0" as *const u8 as *const libc::c_char,
                    245 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        els[6 as libc::c_int as usize].next = els[1 as libc::c_int as usize].next;
        els[1 as libc::c_int as usize]
            .next = &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el;
    } else {
        els[6 as libc::c_int as usize].next = head;
        head = &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el;
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    count = 0 as libc::c_int;
    e = head;
    while !e.is_null() {
        count += 1;
        count;
        e = (*e).next;
    }
    printf(b"count = %d\n\0" as *const u8 as *const libc::c_char, count);
    printf(b"deleting (b)\n\0" as *const u8 as *const libc::c_char);
    let mut _tmp_6: *mut el = 0 as *mut el;
    if head == &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el {
        head = (*head).next;
    } else {
        _tmp_6 = head;
        while !((*_tmp_6).next).is_null()
            && (*_tmp_6).next
                != &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el
        {
            _tmp_6 = (*_tmp_6).next;
        }
        if !((*_tmp_6).next).is_null() {
            (*_tmp_6).next = els[1 as libc::c_int as usize].next;
        }
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"deleting (a)\n\0" as *const u8 as *const libc::c_char);
    let mut _tmp_7: *mut el = 0 as *mut el;
    if head == &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el {
        head = (*head).next;
    } else {
        _tmp_7 = head;
        while !((*_tmp_7).next).is_null()
            && (*_tmp_7).next
                != &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el
        {
            _tmp_7 = (*_tmp_7).next;
        }
        if !((*_tmp_7).next).is_null() {
            (*_tmp_7).next = els[0 as libc::c_int as usize].next;
        }
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"deleting (c)\n\0" as *const u8 as *const libc::c_char);
    let mut _tmp_8: *mut el = 0 as *mut el;
    if head == &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el {
        head = (*head).next;
    } else {
        _tmp_8 = head;
        while !((*_tmp_8).next).is_null()
            && (*_tmp_8).next
                != &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el
        {
            _tmp_8 = (*_tmp_8).next;
        }
        if !((*_tmp_8).next).is_null() {
            (*_tmp_8).next = els[2 as libc::c_int as usize].next;
        }
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"deleting (g)\n\0" as *const u8 as *const libc::c_char);
    let mut _tmp_9: *mut el = 0 as *mut el;
    if head == &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el {
        head = (*head).next;
    } else {
        _tmp_9 = head;
        while !((*_tmp_9).next).is_null()
            && (*_tmp_9).next
                != &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el
        {
            _tmp_9 = (*_tmp_9).next;
        }
        if !((*_tmp_9).next).is_null() {
            (*_tmp_9).next = els[6 as libc::c_int as usize].next;
        }
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"deleting (e)\n\0" as *const u8 as *const libc::c_char);
    let mut _tmp_10: *mut el = 0 as *mut el;
    if head == &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el {
        head = (*head).next;
    } else {
        _tmp_10 = head;
        while !((*_tmp_10).next).is_null()
            && (*_tmp_10).next
                != &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el
        {
            _tmp_10 = (*_tmp_10).next;
        }
        if !((*_tmp_10).next).is_null() {
            (*_tmp_10).next = els[4 as libc::c_int as usize].next;
        }
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"deleting (d)\n\0" as *const u8 as *const libc::c_char);
    let mut _tmp_11: *mut el = 0 as *mut el;
    if head == &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el {
        head = (*head).next;
    } else {
        _tmp_11 = head;
        while !((*_tmp_11).next).is_null()
            && (*_tmp_11).next
                != &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el
        {
            _tmp_11 = (*_tmp_11).next;
        }
        if !((*_tmp_11).next).is_null() {
            (*_tmp_11).next = els[3 as libc::c_int as usize].next;
        }
    }
    e = head;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"deleting (f)\n\0" as *const u8 as *const libc::c_char);
    let mut _tmp_12: *mut el = 0 as *mut el;
    if head == &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el {
        head = (*head).next;
    } else {
        _tmp_12 = head;
        while !((*_tmp_12).next).is_null()
            && (*_tmp_12).next
                != &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el
        {
            _tmp_12 = (*_tmp_12).next;
        }
        if !((*_tmp_12).next).is_null() {
            (*_tmp_12).next = els[5 as libc::c_int as usize].next;
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
