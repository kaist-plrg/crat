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
    let mut els: [el; 20] = [el {
        id: 0,
        next: 0 as *mut el,
        prev: 0 as *mut el,
    }; 20];
    let mut e: *mut el = 0 as *mut el;
    let mut tmp: *mut el = 0 as *mut el;
    let mut tmp2: *mut el = 0 as *mut el;
    let mut headA: *mut el = 0 as *mut el;
    let mut headB: *mut el = 0 as *mut el;
    i = 0 as libc::c_int;
    while i < 20 as libc::c_int {
        els[i as usize].id = 'a' as i32 + i;
        i += 1;
        i;
    }
    printf(b"CDL replace elem\n\0" as *const u8 as *const libc::c_char);
    if !headA.is_null() {
        els[3 as libc::c_int as usize].prev = (*headA).prev;
        els[3 as libc::c_int as usize].next = headA;
        (*headA)
            .prev = &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el;
        (*els[3 as libc::c_int as usize].prev)
            .next = &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el;
    } else {
        els[3 as libc::c_int as usize]
            .prev = &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el;
        els[3 as libc::c_int as usize]
            .next = &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el;
    }
    headA = &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el;
    if !headA.is_null() {
        els[2 as libc::c_int as usize].prev = (*headA).prev;
        els[2 as libc::c_int as usize].next = headA;
        (*headA)
            .prev = &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el;
        (*els[2 as libc::c_int as usize].prev)
            .next = &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el;
    } else {
        els[2 as libc::c_int as usize]
            .prev = &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el;
        els[2 as libc::c_int as usize]
            .next = &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el;
    }
    headA = &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el;
    if !headA.is_null() {
        els[1 as libc::c_int as usize].prev = (*headA).prev;
        els[1 as libc::c_int as usize].next = headA;
        (*headA)
            .prev = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
        (*els[1 as libc::c_int as usize].prev)
            .next = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
    } else {
        els[1 as libc::c_int as usize]
            .prev = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
        els[1 as libc::c_int as usize]
            .next = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
    }
    headA = &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el;
    if !headA.is_null() {
        els[0 as libc::c_int as usize].prev = (*headA).prev;
        els[0 as libc::c_int as usize].next = headA;
        (*headA)
            .prev = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
        (*els[0 as libc::c_int as usize].prev)
            .next = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
    } else {
        els[0 as libc::c_int as usize]
            .prev = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
        els[0 as libc::c_int as usize]
            .next = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
    }
    headA = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
    e = headA;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = if (*e).next == headA { 0 as *mut el } else { (*e).next };
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if !headA.is_null() {} else {
        __assert_fail(
            b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
            b"test72.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_4433: {
        if !headA.is_null() {} else {
            __assert_fail(
                b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                b"test72.c\0" as *const u8 as *const libc::c_char,
                32 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if !(&mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el).is_null()
    {} else {
        __assert_fail(
            b"(&els[0]) != NULL\0" as *const u8 as *const libc::c_char,
            b"test72.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_4383: {
        if !(&mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[0]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test72.c\0" as *const u8 as *const libc::c_char,
                32 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if !(&mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el).is_null()
    {} else {
        __assert_fail(
            b"(&els[4]) != NULL\0" as *const u8 as *const libc::c_char,
            b"test72.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_4333: {
        if !(&mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[4]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test72.c\0" as *const u8 as *const libc::c_char,
                32 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if els[0 as libc::c_int as usize].next
        == &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el
    {
        els[4 as libc::c_int as usize]
            .next = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el;
        els[4 as libc::c_int as usize]
            .prev = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el;
        headA = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el;
    } else {
        els[4 as libc::c_int as usize].next = els[0 as libc::c_int as usize].next;
        els[4 as libc::c_int as usize].prev = els[0 as libc::c_int as usize].prev;
        (*els[4 as libc::c_int as usize].next)
            .prev = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el;
        (*els[4 as libc::c_int as usize].prev)
            .next = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el;
        if headA == &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el {
            headA = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el;
        }
    }
    e = headA;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = if (*e).next == headA { 0 as *mut el } else { (*e).next };
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if !headA.is_null() {} else {
        __assert_fail(
            b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
            b"test72.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_4087: {
        if !headA.is_null() {} else {
            __assert_fail(
                b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                b"test72.c\0" as *const u8 as *const libc::c_char,
                37 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if !(&mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el).is_null()
    {} else {
        __assert_fail(
            b"(&els[4]) != NULL\0" as *const u8 as *const libc::c_char,
            b"test72.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_4037: {
        if !(&mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[4]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test72.c\0" as *const u8 as *const libc::c_char,
                37 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if !(&mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el).is_null()
    {} else {
        __assert_fail(
            b"(&els[5]) != NULL\0" as *const u8 as *const libc::c_char,
            b"test72.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_3987: {
        if !(&mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[5]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test72.c\0" as *const u8 as *const libc::c_char,
                37 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if els[4 as libc::c_int as usize].next
        == &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el
    {
        els[5 as libc::c_int as usize]
            .next = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el;
        els[5 as libc::c_int as usize]
            .prev = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el;
        headA = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el;
    } else {
        els[5 as libc::c_int as usize].next = els[4 as libc::c_int as usize].next;
        els[5 as libc::c_int as usize].prev = els[4 as libc::c_int as usize].prev;
        (*els[5 as libc::c_int as usize].next)
            .prev = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el;
        (*els[5 as libc::c_int as usize].prev)
            .next = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el;
        if headA == &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el {
            headA = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el;
        }
    }
    e = headA;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = if (*e).next == headA { 0 as *mut el } else { (*e).next };
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if !headA.is_null() {} else {
        __assert_fail(
            b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
            b"test72.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_3741: {
        if !headA.is_null() {} else {
            __assert_fail(
                b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                b"test72.c\0" as *const u8 as *const libc::c_char,
                44 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if !(&mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el).is_null()
    {} else {
        __assert_fail(
            b"(&els[3]) != NULL\0" as *const u8 as *const libc::c_char,
            b"test72.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_3691: {
        if !(&mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[3]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test72.c\0" as *const u8 as *const libc::c_char,
                44 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if !(&mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el).is_null()
    {} else {
        __assert_fail(
            b"(&els[6]) != NULL\0" as *const u8 as *const libc::c_char,
            b"test72.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_3641: {
        if !(&mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[6]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test72.c\0" as *const u8 as *const libc::c_char,
                44 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if els[3 as libc::c_int as usize].next
        == &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el
    {
        els[6 as libc::c_int as usize]
            .next = &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el;
        els[6 as libc::c_int as usize]
            .prev = &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el;
        headA = &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el;
    } else {
        els[6 as libc::c_int as usize].next = els[3 as libc::c_int as usize].next;
        els[6 as libc::c_int as usize].prev = els[3 as libc::c_int as usize].prev;
        (*els[6 as libc::c_int as usize].next)
            .prev = &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el;
        (*els[6 as libc::c_int as usize].prev)
            .next = &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el;
        if headA == &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el {
            headA = &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el;
        }
    }
    e = headA;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = if (*e).next == headA { 0 as *mut el } else { (*e).next };
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if !headA.is_null() {} else {
        __assert_fail(
            b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
            b"test72.c\0" as *const u8 as *const libc::c_char,
            49 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_3395: {
        if !headA.is_null() {} else {
            __assert_fail(
                b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                b"test72.c\0" as *const u8 as *const libc::c_char,
                49 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if !(&mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el).is_null()
    {} else {
        __assert_fail(
            b"(&els[6]) != NULL\0" as *const u8 as *const libc::c_char,
            b"test72.c\0" as *const u8 as *const libc::c_char,
            49 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_3345: {
        if !(&mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[6]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test72.c\0" as *const u8 as *const libc::c_char,
                49 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if !(&mut *els.as_mut_ptr().offset(7 as libc::c_int as isize) as *mut el).is_null()
    {} else {
        __assert_fail(
            b"(&els[7]) != NULL\0" as *const u8 as *const libc::c_char,
            b"test72.c\0" as *const u8 as *const libc::c_char,
            49 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_3295: {
        if !(&mut *els.as_mut_ptr().offset(7 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[7]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test72.c\0" as *const u8 as *const libc::c_char,
                49 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if els[6 as libc::c_int as usize].next
        == &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el
    {
        els[7 as libc::c_int as usize]
            .next = &mut *els.as_mut_ptr().offset(7 as libc::c_int as isize) as *mut el;
        els[7 as libc::c_int as usize]
            .prev = &mut *els.as_mut_ptr().offset(7 as libc::c_int as isize) as *mut el;
        headA = &mut *els.as_mut_ptr().offset(7 as libc::c_int as isize) as *mut el;
    } else {
        els[7 as libc::c_int as usize].next = els[6 as libc::c_int as usize].next;
        els[7 as libc::c_int as usize].prev = els[6 as libc::c_int as usize].prev;
        (*els[7 as libc::c_int as usize].next)
            .prev = &mut *els.as_mut_ptr().offset(7 as libc::c_int as isize) as *mut el;
        (*els[7 as libc::c_int as usize].prev)
            .next = &mut *els.as_mut_ptr().offset(7 as libc::c_int as isize) as *mut el;
        if headA == &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el {
            headA = &mut *els.as_mut_ptr().offset(7 as libc::c_int as isize) as *mut el;
        }
    }
    e = headA;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = if (*e).next == headA { 0 as *mut el } else { (*e).next };
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if !headA.is_null() {} else {
        __assert_fail(
            b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
            b"test72.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_3049: {
        if !headA.is_null() {} else {
            __assert_fail(
                b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                b"test72.c\0" as *const u8 as *const libc::c_char,
                56 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if !(&mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el).is_null()
    {} else {
        __assert_fail(
            b"(&els[1]) != NULL\0" as *const u8 as *const libc::c_char,
            b"test72.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_2999: {
        if !(&mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[1]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test72.c\0" as *const u8 as *const libc::c_char,
                56 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if !(&mut *els.as_mut_ptr().offset(8 as libc::c_int as isize) as *mut el).is_null()
    {} else {
        __assert_fail(
            b"(&els[8]) != NULL\0" as *const u8 as *const libc::c_char,
            b"test72.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_2949: {
        if !(&mut *els.as_mut_ptr().offset(8 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[8]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test72.c\0" as *const u8 as *const libc::c_char,
                56 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if els[1 as libc::c_int as usize].next
        == &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el
    {
        els[8 as libc::c_int as usize]
            .next = &mut *els.as_mut_ptr().offset(8 as libc::c_int as isize) as *mut el;
        els[8 as libc::c_int as usize]
            .prev = &mut *els.as_mut_ptr().offset(8 as libc::c_int as isize) as *mut el;
        headA = &mut *els.as_mut_ptr().offset(8 as libc::c_int as isize) as *mut el;
    } else {
        els[8 as libc::c_int as usize].next = els[1 as libc::c_int as usize].next;
        els[8 as libc::c_int as usize].prev = els[1 as libc::c_int as usize].prev;
        (*els[8 as libc::c_int as usize].next)
            .prev = &mut *els.as_mut_ptr().offset(8 as libc::c_int as isize) as *mut el;
        (*els[8 as libc::c_int as usize].prev)
            .next = &mut *els.as_mut_ptr().offset(8 as libc::c_int as isize) as *mut el;
        if headA == &mut *els.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut el {
            headA = &mut *els.as_mut_ptr().offset(8 as libc::c_int as isize) as *mut el;
        }
    }
    if !headA.is_null() {} else {
        __assert_fail(
            b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
            b"test72.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_2752: {
        if !headA.is_null() {} else {
            __assert_fail(
                b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                b"test72.c\0" as *const u8 as *const libc::c_char,
                57 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if !(&mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el).is_null()
    {} else {
        __assert_fail(
            b"(&els[2]) != NULL\0" as *const u8 as *const libc::c_char,
            b"test72.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_2702: {
        if !(&mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[2]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test72.c\0" as *const u8 as *const libc::c_char,
                57 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if !(&mut *els.as_mut_ptr().offset(9 as libc::c_int as isize) as *mut el).is_null()
    {} else {
        __assert_fail(
            b"(&els[9]) != NULL\0" as *const u8 as *const libc::c_char,
            b"test72.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_2652: {
        if !(&mut *els.as_mut_ptr().offset(9 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[9]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test72.c\0" as *const u8 as *const libc::c_char,
                57 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if els[2 as libc::c_int as usize].next
        == &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el
    {
        els[9 as libc::c_int as usize]
            .next = &mut *els.as_mut_ptr().offset(9 as libc::c_int as isize) as *mut el;
        els[9 as libc::c_int as usize]
            .prev = &mut *els.as_mut_ptr().offset(9 as libc::c_int as isize) as *mut el;
        headA = &mut *els.as_mut_ptr().offset(9 as libc::c_int as isize) as *mut el;
    } else {
        els[9 as libc::c_int as usize].next = els[2 as libc::c_int as usize].next;
        els[9 as libc::c_int as usize].prev = els[2 as libc::c_int as usize].prev;
        (*els[9 as libc::c_int as usize].next)
            .prev = &mut *els.as_mut_ptr().offset(9 as libc::c_int as isize) as *mut el;
        (*els[9 as libc::c_int as usize].prev)
            .next = &mut *els.as_mut_ptr().offset(9 as libc::c_int as isize) as *mut el;
        if headA == &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el {
            headA = &mut *els.as_mut_ptr().offset(9 as libc::c_int as isize) as *mut el;
        }
    }
    e = headA;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = if (*e).next == headA { 0 as *mut el } else { (*e).next };
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    i = 10 as libc::c_int;
    e = headA;
    tmp = (if !headA.is_null() { (*headA).prev } else { 0 as *mut el });
    while !e.is_null()
        && {
            tmp2 = (*e).next;
            1 as libc::c_int != 0
        }
    {
        if !headA.is_null() {} else {
            __assert_fail(
                b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                b"test72.c\0" as *const u8 as *const libc::c_char,
                66 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_2346: {
            if !headA.is_null() {} else {
                __assert_fail(
                    b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test72.c\0" as *const u8 as *const libc::c_char,
                    66 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        if !e.is_null() {} else {
            __assert_fail(
                b"(e) != NULL\0" as *const u8 as *const libc::c_char,
                b"test72.c\0" as *const u8 as *const libc::c_char,
                66 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_2301: {
            if !e.is_null() {} else {
                __assert_fail(
                    b"(e) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test72.c\0" as *const u8 as *const libc::c_char,
                    66 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        if !(&mut *els.as_mut_ptr().offset(i as isize) as *mut el).is_null() {} else {
            __assert_fail(
                b"(&els[i]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test72.c\0" as *const u8 as *const libc::c_char,
                66 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_2248: {
            if !(&mut *els.as_mut_ptr().offset(i as isize) as *mut el).is_null()
            {} else {
                __assert_fail(
                    b"(&els[i]) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test72.c\0" as *const u8 as *const libc::c_char,
                    66 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        if (*e).next == e {
            els[i as usize].next = &mut *els.as_mut_ptr().offset(i as isize) as *mut el;
            els[i as usize].prev = &mut *els.as_mut_ptr().offset(i as isize) as *mut el;
            headA = &mut *els.as_mut_ptr().offset(i as isize) as *mut el;
        } else {
            els[i as usize].next = (*e).next;
            els[i as usize].prev = (*e).prev;
            (*els[i as usize].next)
                .prev = &mut *els.as_mut_ptr().offset(i as isize) as *mut el;
            (*els[i as usize].prev)
                .next = &mut *els.as_mut_ptr().offset(i as isize) as *mut el;
            if headA == e {
                headA = &mut *els.as_mut_ptr().offset(i as isize) as *mut el;
            }
        }
        i += 1;
        i;
        e = if e == tmp { 0 as *mut el } else { tmp2 };
    }
    e = headA;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = if (*e).next == headA { 0 as *mut el } else { (*e).next };
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if !headB.is_null() {
        els[18 as libc::c_int as usize].prev = (*headB).prev;
        els[18 as libc::c_int as usize].next = headB;
        (*headB)
            .prev = &mut *els.as_mut_ptr().offset(18 as libc::c_int as isize) as *mut el;
        (*els[18 as libc::c_int as usize].prev)
            .next = &mut *els.as_mut_ptr().offset(18 as libc::c_int as isize) as *mut el;
    } else {
        els[18 as libc::c_int as usize]
            .prev = &mut *els.as_mut_ptr().offset(18 as libc::c_int as isize) as *mut el;
        els[18 as libc::c_int as usize]
            .next = &mut *els.as_mut_ptr().offset(18 as libc::c_int as isize) as *mut el;
    }
    headB = &mut *els.as_mut_ptr().offset(18 as libc::c_int as isize) as *mut el;
    e = headB;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = if (*e).next == headB { 0 as *mut el } else { (*e).next };
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if !headB.is_null() {} else {
        __assert_fail(
            b"(headB) != NULL\0" as *const u8 as *const libc::c_char,
            b"test72.c\0" as *const u8 as *const libc::c_char,
            80 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_1839: {
        if !headB.is_null() {} else {
            __assert_fail(
                b"(headB) != NULL\0" as *const u8 as *const libc::c_char,
                b"test72.c\0" as *const u8 as *const libc::c_char,
                80 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if !(&mut *els.as_mut_ptr().offset(18 as libc::c_int as isize) as *mut el).is_null()
    {} else {
        __assert_fail(
            b"(&els[18]) != NULL\0" as *const u8 as *const libc::c_char,
            b"test72.c\0" as *const u8 as *const libc::c_char,
            80 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_1789: {
        if !(&mut *els.as_mut_ptr().offset(18 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[18]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test72.c\0" as *const u8 as *const libc::c_char,
                80 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if !(&mut *els.as_mut_ptr().offset(19 as libc::c_int as isize) as *mut el).is_null()
    {} else {
        __assert_fail(
            b"(&els[19]) != NULL\0" as *const u8 as *const libc::c_char,
            b"test72.c\0" as *const u8 as *const libc::c_char,
            80 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    }
    'c_1734: {
        if !(&mut *els.as_mut_ptr().offset(19 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[19]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test72.c\0" as *const u8 as *const libc::c_char,
                80 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
    };
    if els[18 as libc::c_int as usize].next
        == &mut *els.as_mut_ptr().offset(18 as libc::c_int as isize) as *mut el
    {
        els[19 as libc::c_int as usize]
            .next = &mut *els.as_mut_ptr().offset(19 as libc::c_int as isize) as *mut el;
        els[19 as libc::c_int as usize]
            .prev = &mut *els.as_mut_ptr().offset(19 as libc::c_int as isize) as *mut el;
        headB = &mut *els.as_mut_ptr().offset(19 as libc::c_int as isize) as *mut el;
    } else {
        els[19 as libc::c_int as usize].next = els[18 as libc::c_int as usize].next;
        els[19 as libc::c_int as usize].prev = els[18 as libc::c_int as usize].prev;
        (*els[19 as libc::c_int as usize].next)
            .prev = &mut *els.as_mut_ptr().offset(19 as libc::c_int as isize) as *mut el;
        (*els[19 as libc::c_int as usize].prev)
            .next = &mut *els.as_mut_ptr().offset(19 as libc::c_int as isize) as *mut el;
        if headB == &mut *els.as_mut_ptr().offset(18 as libc::c_int as isize) as *mut el
        {
            headB = &mut *els.as_mut_ptr().offset(19 as libc::c_int as isize) as *mut el;
        }
    }
    e = headB;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = if (*e).next == headB { 0 as *mut el } else { (*e).next };
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
