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
    let mut els: [el; 26] = [el {
        id: 0,
        next: 0 as *mut el,
        prev: 0 as *mut el,
    }; 26];
    let mut e: *mut el = 0 as *mut el;
    let mut tmp: *mut el = 0 as *mut el;
    let mut headA: *mut el = 0 as *mut el;
    let mut headB: *mut el = 0 as *mut el;
    i = 0 as libc::c_int;
    while i < 25 as libc::c_int {
        els[i as usize].id = 'a' as i32 + i;
        i += 1;
        i;
    }
    printf(b"DL prepend elem\n\0" as *const u8 as *const libc::c_char);
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
    if !headA.is_null() {
        els[3 as libc::c_int as usize].prev = (*headA).prev;
        (*(*headA).prev)
            .next = &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el;
        (*headA)
            .prev = &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el;
        els[3 as libc::c_int as usize].next = 0 as *mut el;
    } else {
        headA = &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el;
        (*headA).prev = headA;
        (*headA).next = 0 as *mut el;
    }
    e = headA;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if !(&mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el).is_null() {
        if !headA.is_null() {} else {
            __assert_fail(
                b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                b"test69.c\0" as *const u8 as *const libc::c_char,
                32 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_4468: {
            if !headA.is_null() {} else {
                __assert_fail(
                    b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test69.c\0" as *const u8 as *const libc::c_char,
                    32 as libc::c_int as libc::c_uint,
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
                b"test69.c\0" as *const u8 as *const libc::c_char,
                32 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_4418: {
            if !(&mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el)
                .is_null()
            {} else {
                __assert_fail(
                    b"(&els[4]) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test69.c\0" as *const u8 as *const libc::c_char,
                    32 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        els[4 as libc::c_int as usize]
            .next = &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el;
        els[4 as libc::c_int as usize].prev = els[0 as libc::c_int as usize].prev;
        els[0 as libc::c_int as usize]
            .prev = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el;
        if headA == &mut *els.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut el {
            headA = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el;
        } else {
            (*els[4 as libc::c_int as usize].prev)
                .next = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize)
                as *mut el;
        }
    } else if !headA.is_null() {
        els[4 as libc::c_int as usize].prev = (*headA).prev;
        (*(*headA).prev)
            .next = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el;
        (*headA)
            .prev = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el;
        els[4 as libc::c_int as usize].next = 0 as *mut el;
    } else {
        headA = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el;
        (*headA).prev = headA;
        (*headA).next = 0 as *mut el;
    }
    e = headA;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if !(&mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el).is_null() {
        if !headA.is_null() {} else {
            __assert_fail(
                b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                b"test69.c\0" as *const u8 as *const libc::c_char,
                37 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_4152: {
            if !headA.is_null() {} else {
                __assert_fail(
                    b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test69.c\0" as *const u8 as *const libc::c_char,
                    37 as libc::c_int as libc::c_uint,
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
                b"test69.c\0" as *const u8 as *const libc::c_char,
                37 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_4102: {
            if !(&mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el)
                .is_null()
            {} else {
                __assert_fail(
                    b"(&els[5]) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test69.c\0" as *const u8 as *const libc::c_char,
                    37 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        els[5 as libc::c_int as usize]
            .next = &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el;
        els[5 as libc::c_int as usize].prev = els[4 as libc::c_int as usize].prev;
        els[4 as libc::c_int as usize]
            .prev = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el;
        if headA == &mut *els.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut el {
            headA = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el;
        } else {
            (*els[5 as libc::c_int as usize].prev)
                .next = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize)
                as *mut el;
        }
    } else if !headA.is_null() {
        els[5 as libc::c_int as usize].prev = (*headA).prev;
        (*(*headA).prev)
            .next = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el;
        (*headA)
            .prev = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el;
        els[5 as libc::c_int as usize].next = 0 as *mut el;
    } else {
        headA = &mut *els.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut el;
        (*headA).prev = headA;
        (*headA).next = 0 as *mut el;
    }
    e = headA;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if !(&mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el).is_null() {
        if !headA.is_null() {} else {
            __assert_fail(
                b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                b"test69.c\0" as *const u8 as *const libc::c_char,
                44 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_3836: {
            if !headA.is_null() {} else {
                __assert_fail(
                    b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test69.c\0" as *const u8 as *const libc::c_char,
                    44 as libc::c_int as libc::c_uint,
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
                b"test69.c\0" as *const u8 as *const libc::c_char,
                44 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_3786: {
            if !(&mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el)
                .is_null()
            {} else {
                __assert_fail(
                    b"(&els[6]) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test69.c\0" as *const u8 as *const libc::c_char,
                    44 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        els[6 as libc::c_int as usize]
            .next = &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el;
        els[6 as libc::c_int as usize].prev = els[3 as libc::c_int as usize].prev;
        els[3 as libc::c_int as usize]
            .prev = &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el;
        if headA == &mut *els.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut el {
            headA = &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el;
        } else {
            (*els[6 as libc::c_int as usize].prev)
                .next = &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize)
                as *mut el;
        }
    } else if !headA.is_null() {
        els[6 as libc::c_int as usize].prev = (*headA).prev;
        (*(*headA).prev)
            .next = &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el;
        (*headA)
            .prev = &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el;
        els[6 as libc::c_int as usize].next = 0 as *mut el;
    } else {
        headA = &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el;
        (*headA).prev = headA;
        (*headA).next = 0 as *mut el;
    }
    e = headA;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if !(&mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el).is_null() {
        if !headA.is_null() {} else {
            __assert_fail(
                b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                b"test69.c\0" as *const u8 as *const libc::c_char,
                49 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_3520: {
            if !headA.is_null() {} else {
                __assert_fail(
                    b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test69.c\0" as *const u8 as *const libc::c_char,
                    49 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        if !(&mut *els.as_mut_ptr().offset(7 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[7]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test69.c\0" as *const u8 as *const libc::c_char,
                49 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_3470: {
            if !(&mut *els.as_mut_ptr().offset(7 as libc::c_int as isize) as *mut el)
                .is_null()
            {} else {
                __assert_fail(
                    b"(&els[7]) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test69.c\0" as *const u8 as *const libc::c_char,
                    49 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        els[7 as libc::c_int as usize]
            .next = &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el;
        els[7 as libc::c_int as usize].prev = els[6 as libc::c_int as usize].prev;
        els[6 as libc::c_int as usize]
            .prev = &mut *els.as_mut_ptr().offset(7 as libc::c_int as isize) as *mut el;
        if headA == &mut *els.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut el {
            headA = &mut *els.as_mut_ptr().offset(7 as libc::c_int as isize) as *mut el;
        } else {
            (*els[7 as libc::c_int as usize].prev)
                .next = &mut *els.as_mut_ptr().offset(7 as libc::c_int as isize)
                as *mut el;
        }
    } else if !headA.is_null() {
        els[7 as libc::c_int as usize].prev = (*headA).prev;
        (*(*headA).prev)
            .next = &mut *els.as_mut_ptr().offset(7 as libc::c_int as isize) as *mut el;
        (*headA)
            .prev = &mut *els.as_mut_ptr().offset(7 as libc::c_int as isize) as *mut el;
        els[7 as libc::c_int as usize].next = 0 as *mut el;
    } else {
        headA = &mut *els.as_mut_ptr().offset(7 as libc::c_int as isize) as *mut el;
        (*headA).prev = headA;
        (*headA).next = 0 as *mut el;
    }
    e = headA;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if !(&mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el).is_null() {
        if !headA.is_null() {} else {
            __assert_fail(
                b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                b"test69.c\0" as *const u8 as *const libc::c_char,
                56 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_3204: {
            if !headA.is_null() {} else {
                __assert_fail(
                    b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test69.c\0" as *const u8 as *const libc::c_char,
                    56 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        if !(&mut *els.as_mut_ptr().offset(8 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[8]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test69.c\0" as *const u8 as *const libc::c_char,
                56 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_3154: {
            if !(&mut *els.as_mut_ptr().offset(8 as libc::c_int as isize) as *mut el)
                .is_null()
            {} else {
                __assert_fail(
                    b"(&els[8]) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test69.c\0" as *const u8 as *const libc::c_char,
                    56 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        els[8 as libc::c_int as usize]
            .next = &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el;
        els[8 as libc::c_int as usize].prev = els[2 as libc::c_int as usize].prev;
        els[2 as libc::c_int as usize]
            .prev = &mut *els.as_mut_ptr().offset(8 as libc::c_int as isize) as *mut el;
        if headA == &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el {
            headA = &mut *els.as_mut_ptr().offset(8 as libc::c_int as isize) as *mut el;
        } else {
            (*els[8 as libc::c_int as usize].prev)
                .next = &mut *els.as_mut_ptr().offset(8 as libc::c_int as isize)
                as *mut el;
        }
    } else if !headA.is_null() {
        els[8 as libc::c_int as usize].prev = (*headA).prev;
        (*(*headA).prev)
            .next = &mut *els.as_mut_ptr().offset(8 as libc::c_int as isize) as *mut el;
        (*headA)
            .prev = &mut *els.as_mut_ptr().offset(8 as libc::c_int as isize) as *mut el;
        els[8 as libc::c_int as usize].next = 0 as *mut el;
    } else {
        headA = &mut *els.as_mut_ptr().offset(8 as libc::c_int as isize) as *mut el;
        (*headA).prev = headA;
        (*headA).next = 0 as *mut el;
    }
    if !(&mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el).is_null() {
        if !headA.is_null() {} else {
            __assert_fail(
                b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                b"test69.c\0" as *const u8 as *const libc::c_char,
                57 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_2921: {
            if !headA.is_null() {} else {
                __assert_fail(
                    b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test69.c\0" as *const u8 as *const libc::c_char,
                    57 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        if !(&mut *els.as_mut_ptr().offset(9 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[9]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test69.c\0" as *const u8 as *const libc::c_char,
                57 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_2871: {
            if !(&mut *els.as_mut_ptr().offset(9 as libc::c_int as isize) as *mut el)
                .is_null()
            {} else {
                __assert_fail(
                    b"(&els[9]) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test69.c\0" as *const u8 as *const libc::c_char,
                    57 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        els[9 as libc::c_int as usize]
            .next = &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el;
        els[9 as libc::c_int as usize].prev = els[2 as libc::c_int as usize].prev;
        els[2 as libc::c_int as usize]
            .prev = &mut *els.as_mut_ptr().offset(9 as libc::c_int as isize) as *mut el;
        if headA == &mut *els.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut el {
            headA = &mut *els.as_mut_ptr().offset(9 as libc::c_int as isize) as *mut el;
        } else {
            (*els[9 as libc::c_int as usize].prev)
                .next = &mut *els.as_mut_ptr().offset(9 as libc::c_int as isize)
                as *mut el;
        }
    } else if !headA.is_null() {
        els[9 as libc::c_int as usize].prev = (*headA).prev;
        (*(*headA).prev)
            .next = &mut *els.as_mut_ptr().offset(9 as libc::c_int as isize) as *mut el;
        (*headA)
            .prev = &mut *els.as_mut_ptr().offset(9 as libc::c_int as isize) as *mut el;
        els[9 as libc::c_int as usize].next = 0 as *mut el;
    } else {
        headA = &mut *els.as_mut_ptr().offset(9 as libc::c_int as isize) as *mut el;
        (*headA).prev = headA;
        (*headA).next = 0 as *mut el;
    }
    e = headA;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    i = 10 as libc::c_int;
    e = headA;
    while !e.is_null()
        && {
            tmp = (*e).next;
            1 as libc::c_int != 0
        }
    {
        if !e.is_null() {
            if !headA.is_null() {} else {
                __assert_fail(
                    b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test69.c\0" as *const u8 as *const libc::c_char,
                    66 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
            'c_2579: {
                if !headA.is_null() {} else {
                    __assert_fail(
                        b"(headA) != NULL\0" as *const u8 as *const libc::c_char,
                        b"test69.c\0" as *const u8 as *const libc::c_char,
                        66 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 11],
                            &[libc::c_char; 11],
                        >(b"int main()\0"))
                            .as_ptr(),
                    );
                }
            };
            if !(&mut *els.as_mut_ptr().offset(i as isize) as *mut el).is_null()
            {} else {
                __assert_fail(
                    b"(&els[i]) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test69.c\0" as *const u8 as *const libc::c_char,
                    66 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
            'c_2526: {
                if !(&mut *els.as_mut_ptr().offset(i as isize) as *mut el).is_null()
                {} else {
                    __assert_fail(
                        b"(&els[i]) != NULL\0" as *const u8 as *const libc::c_char,
                        b"test69.c\0" as *const u8 as *const libc::c_char,
                        66 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 11],
                            &[libc::c_char; 11],
                        >(b"int main()\0"))
                            .as_ptr(),
                    );
                }
            };
            els[i as usize].next = e;
            els[i as usize].prev = (*e).prev;
            (*e).prev = &mut *els.as_mut_ptr().offset(i as isize) as *mut el;
            if headA == e {
                headA = &mut *els.as_mut_ptr().offset(i as isize) as *mut el;
            } else {
                (*els[i as usize].prev)
                    .next = &mut *els.as_mut_ptr().offset(i as isize) as *mut el;
            }
        } else if !headA.is_null() {
            els[i as usize].prev = (*headA).prev;
            (*(*headA).prev).next = &mut *els.as_mut_ptr().offset(i as isize) as *mut el;
            (*headA).prev = &mut *els.as_mut_ptr().offset(i as isize) as *mut el;
            els[i as usize].next = 0 as *mut el;
        } else {
            headA = &mut *els.as_mut_ptr().offset(i as isize) as *mut el;
            (*headA).prev = headA;
            (*headA).next = 0 as *mut el;
        }
        i += 1;
        i;
        e = tmp;
    }
    e = headA;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if !headB.is_null() {
        els[20 as libc::c_int as usize].prev = (*headB).prev;
        (*(*headB).prev)
            .next = &mut *els.as_mut_ptr().offset(20 as libc::c_int as isize) as *mut el;
        (*headB)
            .prev = &mut *els.as_mut_ptr().offset(20 as libc::c_int as isize) as *mut el;
        els[20 as libc::c_int as usize].next = 0 as *mut el;
    } else {
        headB = &mut *els.as_mut_ptr().offset(20 as libc::c_int as isize) as *mut el;
        (*headB).prev = headB;
        (*headB).next = 0 as *mut el;
    }
    e = headB;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if !(&mut *els.as_mut_ptr().offset(20 as libc::c_int as isize) as *mut el).is_null()
    {
        if !headB.is_null() {} else {
            __assert_fail(
                b"(headB) != NULL\0" as *const u8 as *const libc::c_char,
                b"test69.c\0" as *const u8 as *const libc::c_char,
                80 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_2125: {
            if !headB.is_null() {} else {
                __assert_fail(
                    b"(headB) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test69.c\0" as *const u8 as *const libc::c_char,
                    80 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        if !(&mut *els.as_mut_ptr().offset(21 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[21]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test69.c\0" as *const u8 as *const libc::c_char,
                80 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_2075: {
            if !(&mut *els.as_mut_ptr().offset(21 as libc::c_int as isize) as *mut el)
                .is_null()
            {} else {
                __assert_fail(
                    b"(&els[21]) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test69.c\0" as *const u8 as *const libc::c_char,
                    80 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        els[21 as libc::c_int as usize]
            .next = &mut *els.as_mut_ptr().offset(20 as libc::c_int as isize) as *mut el;
        els[21 as libc::c_int as usize].prev = els[20 as libc::c_int as usize].prev;
        els[20 as libc::c_int as usize]
            .prev = &mut *els.as_mut_ptr().offset(21 as libc::c_int as isize) as *mut el;
        if headB == &mut *els.as_mut_ptr().offset(20 as libc::c_int as isize) as *mut el
        {
            headB = &mut *els.as_mut_ptr().offset(21 as libc::c_int as isize) as *mut el;
        } else {
            (*els[21 as libc::c_int as usize].prev)
                .next = &mut *els.as_mut_ptr().offset(21 as libc::c_int as isize)
                as *mut el;
        }
    } else if !headB.is_null() {
        els[21 as libc::c_int as usize].prev = (*headB).prev;
        (*(*headB).prev)
            .next = &mut *els.as_mut_ptr().offset(21 as libc::c_int as isize) as *mut el;
        (*headB)
            .prev = &mut *els.as_mut_ptr().offset(21 as libc::c_int as isize) as *mut el;
        els[21 as libc::c_int as usize].next = 0 as *mut el;
    } else {
        headB = &mut *els.as_mut_ptr().offset(21 as libc::c_int as isize) as *mut el;
        (*headB).prev = headB;
        (*headB).next = 0 as *mut el;
    }
    e = headB;
    while !e.is_null() {
        printf(b"%c \0" as *const u8 as *const libc::c_char, (*e).id);
        e = (*e).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if !(&mut *els.as_mut_ptr().offset(21 as libc::c_int as isize) as *mut el).is_null()
    {
        if !headB.is_null() {} else {
            __assert_fail(
                b"(headB) != NULL\0" as *const u8 as *const libc::c_char,
                b"test69.c\0" as *const u8 as *const libc::c_char,
                85 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_1808: {
            if !headB.is_null() {} else {
                __assert_fail(
                    b"(headB) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test69.c\0" as *const u8 as *const libc::c_char,
                    85 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        if !(&mut *els.as_mut_ptr().offset(22 as libc::c_int as isize) as *mut el)
            .is_null()
        {} else {
            __assert_fail(
                b"(&els[22]) != NULL\0" as *const u8 as *const libc::c_char,
                b"test69.c\0" as *const u8 as *const libc::c_char,
                85 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_1753: {
            if !(&mut *els.as_mut_ptr().offset(22 as libc::c_int as isize) as *mut el)
                .is_null()
            {} else {
                __assert_fail(
                    b"(&els[22]) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test69.c\0" as *const u8 as *const libc::c_char,
                    85 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 11],
                        &[libc::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        els[22 as libc::c_int as usize]
            .next = &mut *els.as_mut_ptr().offset(21 as libc::c_int as isize) as *mut el;
        els[22 as libc::c_int as usize].prev = els[21 as libc::c_int as usize].prev;
        els[21 as libc::c_int as usize]
            .prev = &mut *els.as_mut_ptr().offset(22 as libc::c_int as isize) as *mut el;
        if headB == &mut *els.as_mut_ptr().offset(21 as libc::c_int as isize) as *mut el
        {
            headB = &mut *els.as_mut_ptr().offset(22 as libc::c_int as isize) as *mut el;
        } else {
            (*els[22 as libc::c_int as usize].prev)
                .next = &mut *els.as_mut_ptr().offset(22 as libc::c_int as isize)
                as *mut el;
        }
    } else if !headB.is_null() {
        els[22 as libc::c_int as usize].prev = (*headB).prev;
        (*(*headB).prev)
            .next = &mut *els.as_mut_ptr().offset(22 as libc::c_int as isize) as *mut el;
        (*headB)
            .prev = &mut *els.as_mut_ptr().offset(22 as libc::c_int as isize) as *mut el;
        els[22 as libc::c_int as usize].next = 0 as *mut el;
    } else {
        headB = &mut *els.as_mut_ptr().offset(22 as libc::c_int as isize) as *mut el;
        (*headB).prev = headB;
        (*headB).next = 0 as *mut el;
    }
    e = headB;
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
